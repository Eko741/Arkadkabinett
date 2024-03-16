use rustls::ServerConfig;
use tokio::net::TcpListener;
use tokio::io::{split, AsyncWriteExt};
use tokio_rustls::TlsAcceptor;

use arkadkabinett::{security, HTML_helpers::*};
use arkadkabinett::server_API::*;
use arkadkabinett::produce_request_form_stream;
use std::collections::HashMap;


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

    if request_header.url.ends_with('/'){
        request_header.url.pop();
    }
    
    if !request_header.url.contains('.') {
        request_header.url.push_str("/index.html");
    }

    let (first_part, second_part) = match request_header.url.split_once('/'){
        Some(parts) => parts,
        None => ("", "index.html")
    };
    
    // Sorts the types of requests. If no spcific page was requested return the homepage
    let response: String = match first_part {
        "API" => api_request(second_part, &request_header.request_header),
        "secure" => protected_content_from_file(&request_header.url, &request_header.request_header),
        _ => htpp_response_from_file(&request_header.url, None)        
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
        "test.api" => return response_header(HttpResponse::NOTFOUND, "", "No Testing Underway"),
        "login.api" => return login(request_header),
        _ => (),
    }

    match crate::security::check_privilege(request_header) {
        Ok(_privilage) => (),
        Err(err) => return response_header(HttpResponse::UNAUTHORIZED, "", err)
    }

    // Password secured API calls
    match api_name {
        "start.api" => start_machine(),
        "stop.api" => stop_machine(),
        _ => response_header(HttpResponse::NOTFOUND, "", "Invalid API call"),
    }
}