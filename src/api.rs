use std::collections::HashMap;

// GET REQUEST TO STREAM KEY API
#[tokio::main]
pub async fn get(streamkey: &str) -> Result<(), Box<dyn std::error::Error>> { 
    let client = reqwest::Client::new();
    let apikey = "uMGNMK323G1zOgyD2xBsuMGNMK323G13OgyD2xBsuMGNMK323G1zOgsd2xBs";
    let mut map = HashMap::new();
    map.insert("apiKey", apikey);
    map.insert("streamKey", streamkey);
    let res = client.post("http://throwdown.tv/api/stream-key")
        .json(&map)
        .send()
        .await?;
    println!("BODY: {:?}", res);
    Ok(())
}
