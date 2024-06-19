pub fn url_decode(url: String) -> Result<String, Box<dyn std::error::Error>> {
    let decoded = percent_encoding::percent_decode_str(&url)
        .decode_utf8()?
        .to_string()
        .replace('+', " ");
    
    //check multiple encoding
    if regex::Regex::new(r"%([A-Za-z0-9]{2,3})").unwrap().find_iter(&decoded).count() != 0 {
        return url_decode(decoded);
    }

    return Ok(decoded);
}