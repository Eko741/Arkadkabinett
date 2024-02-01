use sha256::digest;
use std::{
    collections::HashMap,
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
    time::{SystemTime, UNIX_EPOCH},
};

use arkadkabinett::util::find_cookie_val;
use arkadkabinett::HTML_helpers::*;
use arkadkabinett::SharedMem;
use arkadkabinett::ThreadPool;
use arkadkabinett::{server_API::*, util::find_header_val};
use dotenv_codegen::dotenv;

use rsa::{
    pkcs8::{EncodePublicKey, LineEnding},
    RsaPrivateKey, RsaPublicKey,
};

const ADMIN_KEY: &str = dotenv!("ADMIN_KEY");

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
    let url = request_header[0] // First line of header which contains the URL
        .split(' ')
        .nth(1)
        .unwrap_or("/");

    let mut request_parts = url.split('?').nth(0).unwrap_or("/").split('/');

    let mut form_data: HashMap<&str, &str> = HashMap::new();
    for d in url.split('?').nth(1).unwrap_or("").split('&') {
        let mut data = d.split('=');

        let key = data.next();
        let value = data.next();

        if (key.is_none()) || (value.is_none()) {
            continue;
        }

        form_data.insert(key.unwrap(), value.unwrap());
    }

    request_parts.next(); // Skip the domain name/ip adress

    let first_part = request_parts.next().unwrap_or("404.html"); // Gets the first string after "/"
    let second_part = request_parts.next().unwrap_or(""); // Gets the second string after "/"
    println!("{} {}", first_part, second_part);

    // Sorts the types of requests. If no spcific page was requested return the homepage
    let response: String = match first_part {
        "API" => api_request(second_part, &request_header, &form_data, shared_mem),
        "admin" => protected_content_from_file(second_part, &request_header),
        "" => content_from_file("index.html"),
        _ => content_from_file(first_part),
    };

    // Writes the output to the TCP socket
    stream.write_all(response.as_bytes()).unwrap();

    //Returns an empty Ok
    Ok(())
}

fn api_request(
    api: &str,
    request_header: &Vec<String>,
    form_data: &HashMap<&str, &str>,
    shared_mem: std::sync::Arc<SharedMem>,
) -> String {
    // Non password secured api calls
    match api {
        "test" => return error_header("No Testing Underway"),
        "RSA_Key" => return ok_header(shared_mem.public_key_encoded.as_str()),
        "login" => return login(form_data),
        _ => (),
    }

    let cookies = match find_header_val(&request_header, "Cookie") {
        Some(s) => s,
        None => {
            println!("No cookies");
            return redirect_header("/login");
        }
    }
    .split("; ")
    .map(String::from)
    .collect();

    let session = match find_cookie_val(&cookies, "session") {
        Some(s) => s,
        None => {
            println!("No session");
            return redirect_header("/login");
        }
    };
    let session_created: u64 = match find_cookie_val(&cookies, "session-created") {
        Some(s) => s.parse().unwrap(),
        None => {
            println!("No session created");
            return redirect_header("/login");
        }
    };

    let session_active = (session_created + 3600)
        > SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards")
            .as_secs();
    let correct_hash = session == digest(format!("{}{}", ADMIN_KEY, session_created));

    if !session_active || !correct_hash {
        return redirect_header("/login");
    }

    // Password secured API calls
    match api {
        "start" => start_machine(),
        "stop" => stop_machine(),
        _ => error_header("Invalid API call"),
    }
}
