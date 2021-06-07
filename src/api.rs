use reqwest::Error;

// GET REQUEST TO STREAM KEY API
pub fn get_stream_key(streamkey: &str) -> Result<reqwest::blocking::Response, Error> {
    let res = reqwest::blocking::get(format!("https://throwdown.tv/api/stream-key/{}", streamkey))?.json();
    Ok(res.username)
}
