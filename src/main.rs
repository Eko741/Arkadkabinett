use rustls::{server::NoClientAuth, ServerConfig};
use tokio::net::TcpListener;
use tokio::io::{AsyncWriteExt, AsyncReadExt};
use tokio_rustls::{Accept, TlsAcceptor};

use arkadkabinett::HTML_helpers::*;
use arkadkabinett::server_API::*;
use arkadkabinett::util::*;
use arkadkabinett::SharedMem;
use tokio::io::BufReader;
use std::collections::HashMap;
use std::io::Read;
use std::time::SystemTime;
use std::time::UNIX_EPOCH;
use sha256::digest;

const ADMIN_KEY: &str = "test";

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Load server certificates
    let certs = rustls_pemfile::certs(&mut std::io::Cursor::new(include_str!("../localhost.crt"))).flatten().collect();
    let keys = rustls_pemfile::private_key(&mut std::io::Cursor::new(include_str!("../localhost.key"))).unwrap().unwrap();
    // Create server configuration
    let config = ServerConfig::builder().with_no_client_auth().with_single_cert(certs, keys).unwrap();

    // Create TCP listener and TLS acceptor
    let acceptor = TlsAcceptor::from(std::sync::Arc::new(config));
    let listener = TcpListener::bind("127.0.0.1:8080").await?;
    
    loop {
        let (stream, _) = listener.accept().await?;
        tokio::spawn(handle_connection(stream, acceptor.clone()));
    }
}

async fn handle_connection(
    stream: tokio::net::TcpStream,
    acceptor: TlsAcceptor,
    //shared_mem: std::sync::Arc<SharedMem>,
) -> Result<(), ()> {
    let mut stream = match acceptor.accept(stream).await {
        Ok(stream) => stream,
        Err(err) => 
        {
            println!("{}", err);
            return Err(());
        } 
    };
    let mut buffer = [0; 1000];
    let n = stream.read(&mut buffer[..]).await.unwrap();
    println!("{n}");
    println!("{:?}", buffer);
   
   let vec  = &buffer[..n].to_vec();
   let content = String::from_utf8(vec.to_vec()).unwrap();

    println!("{}", content);
    let buf_reader = content.lines(); // Get lines from the buffer

    // Push all the data from the buffer reader to the more convinient vector
    let mut request_header: HashMap<String, String> = HashMap::new();

    for (i, line) in buf_reader.enumerate() {
        if line.is_empty() {
            // If it is the buffer is empty and if we don't break it will wait indefinetly
            break;
        }

        let mut line_parts = line.split(": "); // Split the line into two parts

        // insert the two parts into the hashmap
        request_header.insert(
            if i == 0 {
                "Location".to_string()
            } else {
                line_parts.next().unwrap_or(&"").to_string()
            },
            line_parts.next().unwrap_or(&"").to_string(),
        );
    } // Create vector for all header data inefficient but easy and clean to handle

    println!("Connection established");

    // Check that the request header isn't empty
    if request_header.is_empty() {
        return Ok(());
    }

    let url = find_url_from_header(request_header.get("Location").unwrap())
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
        //api_request(second_part, &request_header, &shared_mem)
        "asda".to_string()
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
    stream.write_all(response.as_bytes()).await.unwrap();
    
    
    //Returns an empty Ok
    Ok(())
}

fn api_request(
    api_name: &str,
    request_header: &HashMap<String, String>,
    shared_mem: &std::sync::Arc<SharedMem>,
) -> String {
    // Non password secured api calls
    match api_name {
        "test" => return error_header("No Testing Underway"),
        _ => (),
    }

    let cookies = request_header
        .get("Cookie")
        .unwrap_or(&"".to_string())
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