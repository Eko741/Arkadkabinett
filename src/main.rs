use std::{
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
};

use hello::ThreadPool;

pub mod HTML_helpers;
use HTML_helpers::*;

pub mod server_API;
use server_API::*;


fn main() {
    // Opens socket for TCP connection. Over is for localhost and under is for production 
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    //let listener = TcpListener::bind("0.0.0.0:80").unwrap();

    let pool = ThreadPool::new(4); // Create a threadpool of 4 workers. 
    drop(pool);
    let pool = ThreadPool::new(4);

    for stream in listener.incoming() { // Loops through all incoming TCP connections wait when there are none left
        if let Ok(stream) = stream { // If the connection is ok process the stream
            pool.execute(|| -> Result<(),()>{ 
                handle_connection(stream) 
            });
        }
    }
}

fn handle_connection(mut stream: TcpStream)  -> Result<(), ()>{

    let mut request_header: Vec<String> = Vec::new(); // Create vector for all header data inefficient but easy and clean to handle
    let buf_reader = BufReader::new(&stream).lines(); // Get lines from the buffer

    // Push all the data from the buffer reader to the more convinient vector
    for line in buf_reader{ 
        match line {
            Ok(string) => 
                if string == "" { // Check if the string is empty. If it is the buffer is empty and if we don't break it will wait indefinetly  
                    break;
                } else {
                request_header.push(string);
                },
            
            Err(_) => continue
        } 
    }

    if request_header.is_empty(){
        return Ok(()); 
    }

    // Gets the URL that the client requested
    let mut request_parts = request_header[0] // First line of header which contains the URL 
        .split(" ") 
        .nth(1)
        .unwrap_or("/")
        .split("/");


    request_parts.next(); // Skip the domain name/ip adress

    let request_type = request_parts.next().unwrap_or("404"); // Gets the first string after "/" 

    println!("{request_type}");
    // Sorts the types of requests. If no spcific page was requested return the homepage
    let response: String = match request_type {
        "API" => api_request(request_parts.next().unwrap_or(""), &request_header),
        "" => content_from_file("hello.html"),
        _ =>  content_from_file(request_type)
        
    };

    // Writes the output to the TCP socket
    stream.write_all(response.as_bytes()).unwrap();
    
    //Returns an empty Ok
    Ok(())
}

fn api_request (api: &str, request_header: &Vec<String>) -> String{

    // Look for a key from the user
    let mut key: Option<String> = None;
    
    for part in request_header { 
        if part.starts_with("Key")  {
            // If key is found 
            key = Some(part.split_at(5).1.to_string());
            break;
        } else {
            continue; 
        };
    }

    
    match api {
        "start" => start_machine(key),
        "stop" => stop_machine(key),
        _ => error_header("Invalid API call"),
    }
}

fn check_key(key: Option<String>, correct_key: &str) -> Result<(), String>{
    // Checks key and returns correct response
    match key {
        Some(key) => 
            if key != correct_key{
                return Err(unauthorized_header("Wrong key"));
            }, 
        None => return Err(unauthorized_header("No key"))
    }

    Ok(())
}
