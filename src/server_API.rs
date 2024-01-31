use crate::util::check_key;
use crate::HTML_helpers::*;
use dotenv_codegen::dotenv;
use std::{fs, process::Command};

const ADMIN_KEY: &str = dotenv!("ADMIN_KEY");

pub fn start_machine(key: String) -> String {
    // Check if key is correct. Else return error header
    let is_key_correct = check_key(key, ADMIN_KEY);

    if let Err(err) = is_key_correct {
        return err;
    }

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

pub fn stop_machine(key: String) -> String {
    // Check if key is correct. Else return error header
    let is_key_correct = check_key(key, ADMIN_KEY);

    if let Err(err) = is_key_correct {
        return err;
    }

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

// Generates an HTML response from a source file. If the passed source file does exist returns 404
pub fn content_from_file(filename: &str) -> String {
    //Should check for security issues in the filename
    match fs::read_to_string(format!("files/{filename}")) {
        Ok(content) => match content.split_once("\r\n") {
            Some((content_type, page)) => ok_header_content_type(page, content_type),
            None => internal_server_error_header("Internal server error"),
        },

        Err(_) => match fs::read_to_string(format!("files/{filename}.html")) {
            Ok(content) => match content.split_once("\r\n") {
                Some((content_type, page)) => ok_header_content_type(page, content_type),
                None => internal_server_error_header("Internal server error"),
            },

            Err(_) => {
                let content = fs::read_to_string("files/404.html").expect("No 404 file");
                match content.split_once("\r\n") {
                    Some((content_type, page)) => error_header_content_type(page, content_type),
                    None => internal_server_error_header("Internal server error"),
                }
            }
        },
    }
}
