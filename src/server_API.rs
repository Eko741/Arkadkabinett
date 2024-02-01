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
    let pos = match filename.rfind('/')  {
        Some(pos) => pos,
        None => return error_header(fs::read_to_string("files/404.html").expect("No 404 file").as_str())
    };

    // Extracts the content type from the filename. 
    // If there is no content type in the filename assume it's an HTML file
    let content_type = 
        if pos == 0 {
            "text/html"
        } else {
            &filename[1..pos]
        };

    match fs::read_to_string(format!("files/{filename}")) {
        Ok(content) => ok_header_content_type(content.as_str(), content_type),
        Err(_) => error_header(fs::read_to_string("files/404.html").expect("No 404 file").as_str())
    }
}
