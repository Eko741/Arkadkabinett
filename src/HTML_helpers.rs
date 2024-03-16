pub enum HttpResponse {
    OK,
    NOTFOUND,
    UNAUTHORIZED,
    FOUND,
    INTERNALSERVERERROR,
    NOCONTENT
}

pub fn response_header(response_type: HttpResponse, headers: &str, content: &str) -> String{
    format!(
        "HTTP/1.1 {}{}{}",
        response_type_to_str(response_type),
        headers,
        if !content.is_empty(){
            format!("\r\nContent-Length: {}\r\n\r\n{}", content.len(), content)
        } else {
            String::from("\r\n\r\n")
        }
    )
}
#[inline]
fn response_type_to_str(response_type: HttpResponse) -> & 'static str{
    match response_type {
        HttpResponse::OK => "200 OK",
        HttpResponse::NOTFOUND => "404 NOT FOUND",
        HttpResponse::UNAUTHORIZED => "401 UNAUTHORIZED",
        HttpResponse::FOUND => "302 FOUND",
        HttpResponse::INTERNALSERVERERROR => "500 INTERNAL SERVER ERROR",
        HttpResponse::NOCONTENT => "204 NO CONTENT"
    } 
}