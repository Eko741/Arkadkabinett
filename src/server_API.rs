const ADMIN_KEY: &str = "3-0_tetris_attack";

use crate::check_key;
use crate::HTML_helpers::*;
use std::{
    process::Command,
    fs,
};

pub fn start_machine(key: Option<String>) -> String{
    let r = crate::check_key(key, ADMIN_KEY);
    
    if r.is_err() {
        return r.unwrap_err();
    }

    match Command::new("sh")
        .arg("-c")
        .arg("bash /home/pi/bashScripts/start.sh")
        .spawn()
    {
        Ok(_) => ok_header("Started machine succesfully"),

        Err(_) => internal_server_error_header("Something went wrong")
    }
}

pub fn stop_machine(key: Option<String>) -> String{

    let r = check_key(key, ADMIN_KEY);
    
    if r.is_err() {
        return r.unwrap_err();
    }
    
    match Command::new("sh")
        .arg("-c")
        .arg("bash /home/pi/bashScripts/stop.sh")
        .output()
    {
        Ok(_) => ok_header("Stopped machine succesfully"),

        Err(_) => internal_server_error_header("Something went wrong")
    }
    
}

pub fn content_from_file(filename: &str) -> String{
    //println!("{filename}");


    match fs::read_to_string(filename) {

        Ok(content) => ok_header(&content),

        Err(_) => {
            let content = fs::read_to_string("404.html").expect("No 404 file");
            error_header(&content)
        },

    }

}