use std::fs;
use std::env;
use std::process::Command;
// GET REQUEST TO STREAM KEY API AND TRANSCODE IT WITH FFMPEG

use serde_derive::{Deserialize, Serialize};
#[derive(Serialize, Deserialize, Debug)]
struct StreamInfo {
    username: String,
    verified: bool,
    isSuspended: bool,
    isVip: bool
}

// Start Stream
#[tokio::main]
pub async fn start(streamkey: &str) -> Result<(), Box<dyn std::error::Error>> { 
    let apikey = "uMGNMK323G1zOgyD2xBsuMGNMK323G13OgyD2xBsuMGNMK323G1zOgsd2xBs";
    let res_body = reqwest::get(format!("http://localhost:8080/api/stream-key/{}/{}", apikey, streamkey))
        .await?
        .text()
        .await?;

    let res: StreamInfo = serde_json::from_str(&res_body)?;
    println!("CREATING HLS DIRECTORY FOR USERNAME: {}", res.username);

    let mut directory: String = env::current_dir().unwrap().to_str().unwrap().to_string();
    directory.push_str(&format!("/public/{}", res.username));

    // Create Directory if not exists
    fs::create_dir_all(&directory).ok();

    Command::new("ffmpeg")
        .arg("-v")
        .arg("verbose")
        .arg("-y")
        .arg("-i")
        .arg(format!("rtmp://127.0.0.1:1935/live/{}", streamkey))
        .arg("-c:v")
        .arg("libx264")
        .arg("-c:a")
        .arg("aac")
        .arg("-ac")
        .arg("1")
        .arg("-strict")
        .arg("-2")
        .arg("-crf")
        .arg("18")
        .arg("-profile:v")
        .arg("baseline")
        .arg("-maxrate")
        .arg("400k")
        .arg("-bufsize")
        .arg("1835k")
        .arg("-pix_fmt")
        .arg("yuv420p")
        .arg("-flags")
        .arg("-global_header")
        .arg("-hls_time")
        .arg("10")
        .arg("-hls_list_size")
        .arg("6")
        .arg("-hls_wrap")
        .arg("10")
        .arg("-start_number")
        .arg("1")
        .arg(format!("{}/index.m3u8", directory))
        .spawn()
        .expect("ffmpeg does not exist");
    Ok(())
}

#[tokio::main]
pub async fn stop(streamkey: &str) -> Result<(), Box<dyn std::error::Error>> { 
    let apikey = "uMGNMK323G1zOgyD2xBsuMGNMK323G13OgyD2xBsuMGNMK323G1zOgsd2xBs";
    let res_body = reqwest::get(format!("https://throwdown.tv/api/stream-key/{}/{}", apikey, streamkey))
        .await?
        .text()
        .await?;

    let res: StreamInfo = serde_json::from_str(&res_body)?;
    println!("REMOVING HLS DIRECTORY FOR USERNAME: {}", res.username);

    let mut directory: String = env::current_dir().unwrap().to_str().unwrap().to_string();
    directory.push_str(&format!("/public/{}", res.username));

    // Remove Directory if not exists
    fs::remove_dir_all(&directory)?;
    Ok(())
}
