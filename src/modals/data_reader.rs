use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Stock {
    pub symbol: String,
    pub owned: u32,
    pub cost: f64,
    pub company_name: String,
    pub close_price: f64,
    pub highest_price: f64,
    pub open_price: f64,
    pub lowest_price: f64,
    pub yesterday_price: f64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Record {
    pub date: String,
    pub balance: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PortfolioRawData {
    pub holdings: Vec<Stock>,
    pub history: Vec<Record>,
}

impl Default for PortfolioRawData {
    fn default() -> Self {
        let json_file: &[u8] = include_bytes!("../../assets/dummy_data.json");
        let data_str = std::str::from_utf8(json_file).unwrap();

        serde_json::from_str(data_str).unwrap()
    }
}

impl PortfolioRawData {
    pub fn new() -> Self {
        let json_file: &[u8] = include_bytes!("../../assets/data.json");
        let data_str = std::str::from_utf8(json_file).unwrap();

        serde_json::from_str(data_str).unwrap()
    }
}
