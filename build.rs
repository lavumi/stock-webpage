use dotenv::dotenv;
use reqwest::header::{HeaderMap, HeaderValue, AUTHORIZATION};
use std::env;
use std::error::Error;
use std::fs;
use std::path::Path;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    dotenv().ok();
    let token = env::var("SUPABASE_TOKEN")?;
    let mut headers = HeaderMap::new();
    headers.insert("apikey", HeaderValue::from_str(&token)?);
    headers.insert(
        AUTHORIZATION,
        HeaderValue::from_str(&format!("Bearer {}", token))?,
    );

    let client = reqwest::Client::new();
    let res = client
        .get("https://tpbeztjisfdyqmwrugpt.supabase.co/functions/v1/portfolio")
        .headers(headers)
        .send()
        .await?;

    let body = res.text().await?;

    let out_dir = match std::env::var("OUT_DIR") {
        Ok(dir) => dir,
        Err(e) => {
            eprintln!("Error getting OUT_DIR: {}", e);
            std::process::exit(1);
        }
    };
    let dest_path = Path::new(&out_dir).join("data.json");
    fs::write(dest_path, body)?;

    println!("Data saved to assets/data.json");
    Ok(())
}
