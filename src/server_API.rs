use crate::util::{find_cookie_val, find_header_val};
use crate::HTML_helpers::*;
use dotenv_codegen::dotenv;
use sha256::digest;
use std::{
    collections::HashMap,
    fs,
    process::Command,
    time::{SystemTime, UNIX_EPOCH},
};

const ADMIN_KEY: &str = dotenv!("ADMIN_KEY");

pub fn start_machine() -> String {
    // Execute shell script to turn on machine input and output
    match Command::new("sh")
        .arg("-c")
        .arg("bash /home/pi/bashScripts/start.sh")
        .spawn()
    {
        Ok(_) => ok_header("Started machine succesfully"),

        Err(_) => internal_server_error_header("Something went wrong"),
    }
}

pub fn stop_machine() -> String {
    // Execute shell script to turn off machine input and output
    match Command::new("sh")
        .arg("-c")
        .arg("bash /home/pi/bashScripts/stop.sh")
        .output()
    {
        Ok(_) => ok_header("Stopped machine succesfully"),

        Err(_) => internal_server_error_header("Something went wrong"),
    }
}

pub fn login(form_data: &HashMap<&str, &str>) -> String {
    let password = match form_data.get("password") {
        Some(p) => p,
        None => return error_header("No password"),
    };

    let time = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards")
        .as_secs();
    let session = digest(format!("{}{}", password, time));

    redirect_header_with_headers(
        "/admin",
        format!(
            "Set-Cookie: session={}; Path=/; Max-Age={}\r\nSet-Cookie: session-created={:?}; Path=/; Max-Age={}\r\n",
            session,
            3600,
            time,
            3600
        )
        .as_str(),
    )
}

pub fn protected_content_from_file(filename: &str, header: &Vec<String>) -> String {
    let cookies = match find_header_val(&header, "Cookie") {
        Some(s) => s,
        None => return redirect_header("/login"),
    }
    .split("; ")
    .map(String::from)
    .collect();

    let session = match find_cookie_val(&cookies, "session") {
        Some(s) => s,
        None => return redirect_header("/login"),
    };
    let session_created: u64 = match find_cookie_val(&cookies, "session-created") {
        Some(s) => s.parse().unwrap(),
        None => return redirect_header("/login"),
    };

    let session_active = (session_created + 3600)
        > SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards")
            .as_secs();
    let correct_hash = session == digest(format!("{}{}", ADMIN_KEY, session_created));

    if !session_active || !correct_hash {
        return redirect_header("/login");
    }

    match filename {
        "/admin" => htpp_response_from_file("/admin/index.html"),
        _ => htpp_response_from_file(filename),
    }
}

// Generates an HTML response from a source file. If the passed source file does exist returns 404
pub fn htpp_response_from_file(filename: &str) -> String {
    // !!
    //Should check for security issues in the filename. Sigge varsegod.
    // !!

    // If no file extension was given assume it's HTML
    let filename = if !filename.contains('.') {
        format!("{filename}.html")
    } else {
        filename.to_string()
    };

    // Finds postion of last '/' to extract to content type from the filename
    let pos = match filename.rfind('/') {
        Some(pos) => pos,
        None => {
            return error_header(
                fs::read_to_string("files/404.html")
                    .expect("No 404 file")
                    .as_str(),
            )
        }
    };

    // Extracts the content type from the filename.
    // If there is no content type in the filename assume it's an HTML file
    let content_type = if pos == 0 {
        "text/html"
    } else {
        &filename[1..pos]
    };

    match fs::read_to_string(format!("files/{filename}")) {
        Ok(content) => ok_header_content_type(content.as_str(), content_type),
        Err(_) => error_header(
            fs::read_to_string("files/404.html")
                .expect("No 404 file")
                .as_str(),
        ),
    }
}
