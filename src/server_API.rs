use crate::HTML_helpers::*;
use std::{
    collections::HashMap,
    fs,
    process::Command,
};

use crate::security::*;

pub fn start_machine() -> String {
    // Execute shell script to turn on machine input and output
    match Command::new("sh")
        .arg("-c")
        .arg("bash /home/pi/bashScripts/start.sh")
        .spawn()
    {
        Ok(_) => response_header(HttpResponse::OK, "", "Started machine succesfully"),

        Err(_) => response_header(HttpResponse::INTERNALSERVERERROR, "", "Something went wrong"),
    }
}

pub fn stop_machine() -> String {
    // Execute shell script to turn off machine input and output
    match Command::new("sh")
        .arg("-c")
        .arg("bash /home/pi/bashScripts/stop.sh")
        .output()
    {
        Ok(_) => response_header(HttpResponse::OK, "", "Stopped machine succesfully"),

        Err(_) => response_header(HttpResponse::INTERNALSERVERERROR, "", "Something went wrong"),
    }
}

pub fn login(request_header: &HashMap<String, String>) -> String{
    let privilage = match check_privilege(request_header) {
        Ok(privilage) => privilage,
        Err(err) => return response_header(HttpResponse::UNAUTHORIZED, "",err)
    };

    match privilage {
        Privilege::Admin => response_header(HttpResponse::NOCONTENT, "\r\nRedirect: /secure/admin", ""),
        Privilege::User => response_header(HttpResponse::NOCONTENT, "\r\nRedirect: /secure/user", ""),
        Privilege::ElJoelpe => response_header(HttpResponse::NOCONTENT, "\r\nRedirect: /secure/joel", ""),
    }
}

pub fn protected_content_from_file(filename: &str, request_header: &HashMap<String, String>) -> String {
    
    match crate::security::check_privilege(request_header) {
        Ok(_privilage) => (),
        Err(err) => return htpp_response_from_file("/login.html", Some(vec!(("LoginError", err))))
    }

    htpp_response_from_file(filename, None)
}

// Generates an HTML response from a source file. If the passed source file does exist returns 404
pub fn htpp_response_from_file(filename: &str, headers: Option<Vec<(&str, &str)>>) -> String {
    // !!
    //Should check for security issues in the filename. Sigge varsegod.
    // !!

    let mut header_string: String = String::from("");
    
    if let Some(headers) = headers {

        for header in headers{
            header_string.push_str(
        format!("\r\n{}: {}", 
                    header.0, 
                    header.1
                )
                .as_str()
            );
        }
    }

    // Finds postion of last '/' to extract to content type from the filename
    let pos = match filename.rfind('/') {
        Some(pos) => pos,
        None => {
            return response_header(
                HttpResponse::NOTFOUND,
                "",
                fs::read_to_string("files/404.html")
                        .expect("No 404 file")
                        .as_str(),
            )
        }
    };

    // Extracts the content type from the filename.
    // If there is no content type in the filename assume it's an HTML file
    let content_type = if pos == 0 || filename.starts_with("secure") {
        "text/html"
    } else {
        &filename[0..pos]
    };

    header_string.push_str(format!("\r\nContent-Type: {}", content_type).as_str());
    
    match fs::read_to_string(format!("files/{filename}")) {
        Ok(content) => response_header(HttpResponse::OK, header_string.as_str(), content.as_str()),
        Err(_) => response_header(
            HttpResponse::NOTFOUND,
            "",
            fs::read_to_string("files/404.html")
                    .expect("No 404 file")
                    .as_str(),
        ),
    }
}
