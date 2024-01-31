use std::{
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
};

use arkadkabinett::security::*;
use arkadkabinett::server_API::*;
use arkadkabinett::HTML_helpers::*;
use arkadkabinett::SharedMem;
use arkadkabinett::ThreadPool;

use rsa::{
    pkcs8::{EncodePublicKey, LineEnding},
    RsaPrivateKey, RsaPublicKey,
};

fn main() {
    // Generates a private and public RSA key. Stores them and a PEM representation of the public key
    // in memory that's safely shared across threads
    let mut rng = rand::thread_rng();
    let priv_key = RsaPrivateKey::new(&mut rng, 2048).expect("failed to generate a key");
    let pub_key = RsaPublicKey::from(&priv_key);

    let pub_key_encoded = pub_key
        .to_public_key_pem(LineEnding::CRLF)
        .expect("Failed to encode public key");

    let shared_mem_arc = std::sync::Arc::new(SharedMem {
        public_key: pub_key,
        public_key_encoded: pub_key_encoded,
        private_key: priv_key,
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

    if request_header.is_empty() {
        return Ok(());
    }

    // Gets the URL that the client requested
    let mut request_parts = request_header[0] // First line of header which contains the URL
        .split(' ')
        .nth(1)
        .unwrap_or("/")
        .split('/');

    request_parts.next(); // Skip the domain name/ip adress

    let request_type = request_parts.next().unwrap_or("404.html"); // Gets the first string after "/"

    println!("{request_type}");
    // Sorts the types of requests. If no spcific page was requested return the homepage
    let response: String = match request_type {
        "API" => api_request(
            request_parts.next().unwrap_or(""),
            &request_header,
            shared_mem,
        ),
        "" => content_from_file("arkadkabinett.html"),
        _ => content_from_file(request_type),
    };
    //println!("{}", response);

    // Writes the output to the TCP socket
    stream.write_all(response.as_bytes()).unwrap();

    //Returns an empty Ok
    Ok(())
}

fn api_request(
    api: &str,
    request_header: &Vec<String>,
    shared_mem: std::sync::Arc<SharedMem>,
) -> String {
    // Non password secured api calls
    match api {
        "test" => return error_header("No Testing Underway"),
        "RSA_Key" => return ok_header(shared_mem.public_key_encoded.as_str()),
        _ => (),
    }

    // Gets password returns error if no key is found
    let key = decrypt_header(request_header, &shared_mem, "Key_Encrypted");

    let key = match key {
        Ok(k) => k,
        Err(err) => return unauthorized_header(&err),
    };

    // Password secured API calls
    match api {
        "start" => start_machine(key),
        "stop" => stop_machine(key),
        _ => error_header("Invalid API call"),
    }
}
