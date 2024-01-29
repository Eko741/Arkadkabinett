
// Finds a value in a request header vec
pub fn find_val(vector: &Vec<String>, pattern: &str) -> Option<String>{
    for part in vector { 
        if part.starts_with(pattern)  {
            return Some(part.split_at(pattern.len() + 2).1.to_string());
        }
    }
    None
}

// Checks key and returns correct response
pub fn check_key(key: String, correct_key: &str) -> Result<(), String>{
    if key != correct_key{
        return Err(crate::HTML_helpers::unauthorized_header("Wrong key"));
    }

    Ok(())
}
