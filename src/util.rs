// Finds a value in a vec
pub fn find_val(
    vector: &Vec<String>,
    pattern: &str,
    key_value_sep_length: usize,
) -> Option<String> {
    for part in vector {
        if part.starts_with(pattern) {
            return Some(
                part.split_at(pattern.len() + key_value_sep_length)
                    .1
                    .to_string(),
            );
        }
    }
    None
}

pub fn find_cookie_val(vector: &Vec<String>, pattern: &str) -> Option<String> {
    find_val(vector, pattern, 1)
}

// Checks key and returns correct response
pub fn check_key(key: String, correct_key: &str) -> Result<(), String> {
    if key != correct_key {
        return Err(crate::HTML_helpers::unauthorized_header("Wrong key"));
    }

    Ok(())
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
