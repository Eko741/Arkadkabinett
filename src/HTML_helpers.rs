pub fn error_header(content: &str) -> String{
    format!("HTTP/1.1 404 NOT FOUND\r\nContent-Length: {}\r\n\r\n{content}", content.len())
}

pub fn ok_header(content: &str) -> String{
    format!("HTTP/1.1 200 OK\r\nContent-Length: {}\r\n\r\n{content}", content.len())
}

pub fn unauthorized_header(content: &str) -> String{
    format!("HTTP/1.1 401 UNAUTHORIZED\r\nContent-Length: {}\r\n\r\n{content}", content.len())
}

pub fn internal_server_error_header(content: &str) -> String{
    format!("HTTP/1.1 500 INTERNAL SERVER ERROR\r\nContent-Length: {}\r\n\r\n{content}", content.len())
}