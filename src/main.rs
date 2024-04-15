use std::fs;
use std::io::{self, Write};
use std::time::{SystemTime, UNIX_EPOCH};

use base64::prelude::*;
use serde_json::Value;
use urlencoding::encode;

mod ecb_utils;
mod web_utils;

#[tokio::main]
async fn main(){

    let result = upload_data().await.expect("Failed to upload data.");

    println!("Result: {}", result)

}

async fn upload_data() -> io::Result<String>{

    println!("Reading save_file.json...");
    let file_content = fs::read_to_string(r"C:\Users\Mako\Documents\Decompiles\Win\save_file.json").expect("Failed to read file.");
    println!("Compressing json...");
    let compressed_json:Value = serde_json::from_str(&*file_content)?;
    serde_json::to_string(&compressed_json)?;

    let key = "FuckUpAndroidStudioWithoutReason";

    println!("Encrypting save data...");
    let encrypted_data = ecb_utils::encrypt_ecb(key.as_bytes(), file_content.as_bytes()).expect("Failed to encrypt json.");

    println!("Encoding save data with base64...");
    let encoded = BASE64_STANDARD.encode(&encrypted_data);

    let result = request(encoded.as_str()).await.expect("Failed to send web request.");

    Ok(result)
}

async fn request(game_save_base64: &str) -> Result<String, Box<dyn std::error::Error>>{

    let mut timestamp: u64 = 0;
    let key = "FuckUpAndroidStudioWithoutReason";

    println!("Getting timestamp...");
    match SystemTime::now().duration_since(UNIX_EPOCH){
        Ok(duration) => {
            timestamp = duration.as_secs() * 1000 + duration.subsec_millis() as u64;
        }
        Err(e) => println!("Failed to get timestamp: {:?}", e),
    }

    println!("Timestamp: {}", timestamp);

    let encrypt_timestamp = ecb_utils::encrypt_ecb(key.as_bytes(), timestamp.to_string().as_bytes())?;
    let encoded_timestamp = BASE64_STANDARD.encode(&encrypt_timestamp);

    let data_json = format!("{}{}{}", r#"{"saveVersion":106,"savePart0":""#, game_save_base64, r#"","savePart1":"","savePart2":""}"#);
    let data_json_urlencoded = encode(data_json.as_str());
    let urlencoded_timestamp = encode(&*encoded_timestamp);

    let body = format!("{}{}{}{}", r#"data="#, data_json_urlencoded,r#"&key="#, urlencoded_timestamp);

    println!("Sending POST request...");
    let result = web_utils::send_orzmic_request(body.to_string()).await?;

    Ok(result)

}