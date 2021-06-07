use reqwest::Error;

// GET REQUEST TO STREAM KEY API
pub fn get_stream_key(streamkey: &str) -> Result<String, Error> {
    let apikey = "uMGNMK323G1zOgyD2xBsuMGNMK323G13OgyD2xBsuMGNMK323G1zOgsd2xBs";
    let res = reqwest::blocking::get(format!("https://throwdown.tv/api/stream-key/{}/{}",apikey, streamkey))?.text()?;
    Ok(res)
}
