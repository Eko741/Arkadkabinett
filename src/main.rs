use sha256::digest;
use std::{
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
    time::{SystemTime, UNIX_EPOCH},
};

use arkadkabinett::server_API::*;
use arkadkabinett::util::find_cookie_val;
use arkadkabinett::util::find_header_val;
use arkadkabinett::HTML_helpers::*;
use arkadkabinett::SharedMem;
use arkadkabinett::ThreadPool;
use arkadkabinett::{security::*, util::find_url_from_header};
use dotenv_codegen::dotenv;

const ADMIN_KEY: &str = dotenv!("ADMIN_KEY");
fn main() {
    // Shared memory that's safely shared across threads. Read only
    let shared_mem_arc = std::sync::Arc::new(SharedMem {
        rsa_key: generate_key_pair(), // More read only data can be added through the SharedMem struct
    });

    // Opens socket for TCP connection. Over is for localhost and under is for production
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    //let listener = TcpListener::bind("0.0.0.0:80").unwrap();

    let pool = ThreadPool::new(4);

    for stream in listener.incoming().flatten() {
        // Loops through all incoming TCP connections wait when there are none left
        let shared_mem_clone = std::sync::Arc::clone(&shared_mem_arc);
        pool.execute(move || -> Result<(), ()> { handle_connection(stream, shared_mem_clone) });
    }
}

fn handle_connection(
    mut stream: TcpStream,
    shared_mem: std::sync::Arc<SharedMem>,
) -> Result<(), ()> {
    let buf_reader = BufReader::new(&stream).lines(); // Get lines from the buffer

    // Push all the data from the buffer reader to the more convinient vector
    let mut request_header: Vec<String> = Vec::new(); // Create vector for all header data inefficient but easy and clean to handle

    for line in buf_reader.flatten() {
        if line.is_empty() {
            // If it is the buffer is empty and if we don't break it will wait indefinetly
            break;
        } else {
            request_header.push(line);
        }
    }

    // Check that the request header isn't empty
    if request_header.is_empty() {
        return Ok(());
    }

    let url = find_url_from_header(request_header[0].as_str())
        .unwrap_or("/")
        .split('?')
        .nth(0)
        .unwrap_or("/");

    let mut request_parts = url.split('?').nth(0).unwrap_or("/").split('/');
    request_parts.next(); // Skip the domain name/ip adress

    let first_part = request_parts.next().unwrap_or("404.html"); // Gets the first string after "/"
    let second_part = request_parts.next().unwrap_or(""); // Gets the second string after "/"

    // Sorts the types of requests. If no spcific page was requested return the homepage
    let response: String = if first_part == "API" {
        // If it's an API call
        api_request(second_part, &request_header, &shared_mem)
    } else if first_part == "" {
        // If no spcific page was requested return the homepage
        htpp_response_from_file("/index.html")
    } else if first_part == "admin" {
        // If it's an admin page
        protected_content_from_file(url, &request_header)
    } else {
        // Get content from the file
        htpp_response_from_file(url)
    };

    // Writes the output to the TCP socket
    // Should handle error better.
    stream.write_all(response.as_bytes()).unwrap();

    //Returns an empty Ok
    Ok(())
}

fn api_request(
    api_name: &str,
    request_header: &Vec<String>,
    shared_mem: &std::sync::Arc<SharedMem>,
) -> String {
    // Non password secured api calls
    match api_name {
        "test" => return error_header("No Testing Underway"),
        "RSA_Key" => return ok_header(shared_mem.rsa_key.public_key_encoded.as_str()),
        _ => (),
    }

    let cookies = match find_header_val(&request_header, "Cookie") {
        Some(s) => s,
        None => return unauthorized_header("No cookies"),
    }
    .split("; ")
    .map(String::from)
    .collect();

    let session = match find_cookie_val(&cookies, "session") {
        Some(s) => s,
        None => return unauthorized_header("No session"),
    };
    let session_created: u128 = match find_cookie_val(&cookies, "session-created") {
        Some(s) => s.parse().unwrap(),
        None => return unauthorized_header("No session"),
    };

    let session_active = (session_created + 3600)
        > SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards")
            .as_millis();
    let correct_hash = session == digest(format!("{}{}", ADMIN_KEY, session_created));

    if !session_active || !correct_hash {
        return redirect_header("/login");
    }

    // Password secured API calls
    match api_name {
        "start" => start_machine(),
        "stop" => stop_machine(),
        _ => error_header("Invalid API call"),
    }
}
