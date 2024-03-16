pub fn find_cookie_val<'a>(cookies: &Vec<(& 'a str, & 'a str)>, pattern: &str) -> Option<&'a str>{
    for (key, value) in cookies {
        if *key == pattern {
            return Some(value);
        }
    }
    None
}

// Returns the URL found in a request header
// Example: GET /text/javascript/script.js HTTP/1.1
// First removes: "GET " then: " HTTP/1.1" and returns: /text/javascript/script.js
pub fn find_url_from_header(header: &str) -> Option<&str> {
    let last_part = &header[header.find('/')?..];
    Some(&last_part[..last_part.find(' ')?])
}

// Returns the URL and method found in a request header
// Example: GET /text/javascript/script.js HTTP/1.1
// First removes: "GET " then: " HTTP/1.1" and returns: GET, /text/javascript/script.js
pub fn find_url_method_from_header(header: &str) -> Option<(&str, &str)> {
    let parts = header.split_once('/')?; 
    Some((parts.0, &(parts.1)[..parts.1.find(' ')?]))
}
