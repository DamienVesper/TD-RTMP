use reqwest::Error;

// GET REQUEST TO STREAM KEY API
pub fn get_stream_key(streamkey: &str) -> Result<reqwest::blocking::Response, Error> {
    let body = reqwest::blocking::get(format!("https://throwdown.tv/api/stream-key/{}", streamkey))?;
    Ok(body)
}