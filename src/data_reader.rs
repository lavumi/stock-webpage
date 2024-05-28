use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::BufReader;

#[derive(Debug, Serialize, Deserialize)]
pub struct PieChart {
    pub symbol: String,
    pub owned: u32,
    pub price: f64,
    pub cost: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DailyTable {
    pub symbol: String,
    pub index: String,
    pub price: f64,
    pub change: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PortfolioRawData {
    pub pie_chart: Vec<PieChart>,
    pub daily: Vec<DailyTable>,
}

impl PortfolioRawData {
    pub fn new() -> Self {
        let file = File::open("assets/data.json").unwrap();
        let reader = BufReader::new(file);
        let data = serde_json::from_reader(reader).unwrap();
        return data;
    }
}
