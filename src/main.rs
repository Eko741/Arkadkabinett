use std::{
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
};

use arkadkabinett::{security::*, util::find_url_from_header};
use arkadkabinett::server_API::*;
use arkadkabinett::HTML_helpers::*;
use arkadkabinett::SharedMem;
use arkadkabinett::ThreadPool;

fn main() {
    // Shared memory that's safely shared across threads. Read only
    let shared_mem_arc = std::sync::Arc::new( 
        SharedMem{
            rsa_key : generate_key_pair()
            // More read only data can be added through the SharedMem struct 
        }
    );

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

    let url = find_url_from_header(request_header[0].as_str()).unwrap_or("/");

    let response: String = 
        if url.starts_with("/API/") {
            // If it's an API call
            api_request(url, &request_header, shared_mem)
        } else if url == "/" {
            // If no spcific page was requested return the homepage
            htpp_response_from_file("/home.html")
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
    shared_mem: std::sync::Arc<SharedMem>,
) -> String {
    
    // Remove "/API/" from the start
    let api_name = &api_name[5..];

    // Non password secured api calls
    match api_name {
        "test" => return error_header("No Testing Underway"),
        "RSA_Key" => return ok_header(shared_mem.rsa_key.public_key_encoded.as_str()),
        _ => (),
    }

    // Gets password returns error if no key is found
    let key = decrypt_header(request_header, &shared_mem, "Key_Encrypted");

    let key = match key {
        Ok(k) => k,
        Err(err) => return unauthorized_header(err.as_str()),
    };

    // Password secured API calls
    match api_name {
        "start" => start_machine(key),
        "stop" => stop_machine(key),
        _ => error_header("Invalid API call"),
    }
}
