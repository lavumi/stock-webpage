use dotenv::dotenv;
use reqwest::header::{HeaderMap, HeaderValue, AUTHORIZATION};
use std::env;
use std::error::Error;
use std::fs;

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

    fs::create_dir_all("assets")?;
    fs::write("assets/data.json", body)?;

    println!("Data saved to assets/data.json");
    Ok(())
}
