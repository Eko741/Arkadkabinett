use rustls::{server::NoClientAuth, ServerConfig};
use tokio::net::TcpListener;
use tokio::io::{split, AsyncReadExt, AsyncWriteExt};
use tokio_rustls::{Accept, TlsAcceptor};

use arkadkabinett::HTML_helpers::*;
use arkadkabinett::server_API::*;
use arkadkabinett::util::*;
use arkadkabinett::SharedMem;
use arkadkabinett::produce_request_form_stream;
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
        tokio::spawn(handle_client(stream, acceptor.clone()));
    }
}

async fn handle_client(stream: tokio::net::TcpStream, acceptor: TlsAcceptor) -> Result<(),()> {
    let stream = match acceptor.accept(stream).await {
        Ok(stream) => stream,
        Err(err) => 
        {
            eprintln!("{}", err);
            return Err(());
        } 
    };

    let (reader, mut writer) = split(stream);

    let mut request_header =  match produce_request_form_stream(reader).await{
        Ok(rqh) => rqh,
        Err(err) => {
            eprintln!("{}", err);
            return Err(());
        }
    };

    if !request_header.url.contains('.') {
        request_header.url.push_str("/index.html");
    }

    let (first_part, second_part) = match request_header.url.split_once('/'){
        Some(parts) => parts,
        None => ("", "")
    };
    
    println!("{}", request_header.url);
    // Sorts the types of requests. If no spcific page was requested return the homepage
    let response: String = 
        if first_part == "API" {
            // If it's an API call
            api_request(second_part, &request_header.request_header)
        } else if first_part == "admin" {
            // If it's an admin page
            protected_content_from_file(&request_header.url, &request_header.request_header)
        } else {
            // Get content from the file
            htpp_response_from_file(&request_header.url)
        };
    
    // Writes the output to the TCP socket
    // Should handle error better.
    writer.write_all(response.as_bytes()).await.unwrap();
    
    //Returns an empty Ok
    Ok(())
}

fn api_request(
    api_name: &str,
    request_header: &HashMap<String, String>,
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