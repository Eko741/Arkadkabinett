use std::collections::HashMap;

use tokio::io::AsyncBufReadExt;

use util::find_url_method_from_header;

pub mod security;

#[allow(non_snake_case)]
pub mod server_API;

#[allow(non_snake_case)]
pub mod HTML_helpers;

pub mod util;

type Reader = tokio::io::ReadHalf<tokio_rustls::server::TlsStream<tokio::net::TcpStream>>;

pub async fn produce_request_form_stream(stream: Reader) -> Result<Request, Box<dyn std::error::Error>>{
    let mut buffered_stream = tokio::io::BufReader::new(stream).lines();
    let mut request_header: HashMap<String, String> = HashMap::new();
    
    let mut i = 0;

    let url_line = match buffered_stream.next_line().await?{
        Some(line) => line,
        None => return Err(Box::new(HeaderError))
    };

    let url_method = match find_url_method_from_header(url_line.as_str()){
        Some(url) => url,
        None => return Err(Box::new(HeaderError))
    };


 
    while let Some(line) = buffered_stream.next_line().await? {
        if line.is_empty(){
            break;
        }

        let line_parts = match line.split_once(": "){
            Some(lp) => lp,
            None => continue
        }; // Split the line into two parts

        // insert the two parts into the hashmap
        request_header.insert(
            if i == 0 {
                "Location".to_string()
            } else {
                line_parts.0.to_string()
            },
            line_parts.1.to_string(),
        );
        i += 1;
    }
    

    Ok(Request{
        url: url_method.1.to_string(),
        method: url_method.0.to_string(),
        request_header: request_header
    })
}

pub struct SharedMem {
    pub placeholder: () 
}

pub struct Request{
    pub url: String,
    pub method: String, // Could be an enum.
    pub request_header: HashMap<String, String>,
}

#[derive(Debug)]
struct HeaderError;
impl std::error::Error for HeaderError{}
impl std::fmt::Display for HeaderError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Header Error")
    }
}

