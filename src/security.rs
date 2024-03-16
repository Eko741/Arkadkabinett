use std::collections::HashMap;
use crate::util::*;
use std::time::SystemTime;
use std::time::UNIX_EPOCH;
use sha256::digest;
use dotenv_codegen::dotenv;

const ADMIN_KEY: &str = dotenv!("ADMIN_KEY");


pub enum Privilege{
    User,
    Admin,
    ElJoelpe
}

pub fn check_privilege(request_header: &HashMap<String, String>) -> Result<Privilege, &str>{
    let cookies: &String = match request_header.get("Cookie"){
        Some(cookies) => cookies,
        None => return Err("No Session Cookie")
    };

    let mut cookie_vector: Vec<(&str, &str)> = Vec::new();

    for cookie in cookies.split("; "){
        match cookie.split_once("=") {
            Some((key, value)) => cookie_vector.push((key, value)),
            None => (),
        }
    };

    let session = match find_cookie_val(&cookie_vector, "session") {
        Some(s) => s,
        None => return Err("No Session Cookie")
    };

    let session_created: u128 = match find_cookie_val(&cookie_vector, "session-created") {
        Some(s) => match s.parse(){
            Ok(num) => num,
            Err(_) => return Err("Incorrect Session Cookie")
        },
        None => return Err("No Session Cookie")
    };

    let session_active = (session_created + 3600000)
        > SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards")
            .as_millis();

    let correct_hash = session == digest(format!("{}{}", ADMIN_KEY, session_created));

    if !session_active || !correct_hash {
        return Err("Incorrect Password or Expired Cookies");
    }

    Ok(Privilege::Admin)
}