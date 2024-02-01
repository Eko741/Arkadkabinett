pub fn error_header(content: &str) -> String {
    format!(
        "HTTP/1.1 404 NOT FOUND\r\nContent-Length: {}\r\n\r\n{content}",
        content.len()
    )
}

pub fn ok_header(content: &str) -> String {
    ok_header_with_headers(content, "")
}
pub fn ok_header_with_headers(content: &str, headers: &str) -> String {
    format!(
        "HTTP/1.1 200 OK\r\nContent-Length: {}\r\n{}\r\n{content}",
        content.len(),
        headers
    )
}

pub fn redirect_header(location: &str) -> String {
    redirect_header_with_headers(location, "")
}
pub fn redirect_header_with_headers(location: &str, headers: &str) -> String {
    format!(
        "HTTP/1.1 302 FOUND\r\nLocation: {}\r\n{}",
        location, headers
    )
}

pub fn unauthorized_header(content: &str) -> String {
    format!(
        "HTTP/1.1 401 UNAUTHORIZED\r\nContent-Length: {}\r\n\r\n{content}",
        content.len()
    )
}

pub fn internal_server_error_header(content: &str) -> String {
    format!(
        "HTTP/1.1 500 INTERNAL SERVER ERROR\r\nContent-Length: {}\r\n\r\n{content}",
        content.len()
    )
}

pub fn ok_header_content_type(content: &str, content_type: &str) -> String {
    format!(
        "HTTP/1.1 200 OK\r\nContent-Length: {}\r\nContent-Type: {content_type}\r\n\r\n{content}",
        content.len()
    )
}

pub fn error_header_content_type(content: &str, content_type: &str) -> String {
    format!(
        "HTTP/1.1 404 NOT FOUND\r\nContent-Length: {}\r\nContent-Type: {content_type}\r\n\r\n{content}",
        content.len()
    )
}
