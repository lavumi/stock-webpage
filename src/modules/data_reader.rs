use csv::ReaderBuilder;
use serde::{Deserialize, Serialize};

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

    #[serde(skip_deserializing)]
    pub usd_krw: Vec<f64>,
}

impl PortfolioRawData {
    pub fn new() -> Self {
        let json_file: &[u8] = include_bytes!("../../assets/data.json");
        let data_str = std::str::from_utf8(json_file).unwrap();
        let mut data: PortfolioRawData = serde_json::from_str(data_str).unwrap();

        let csv_file = include_str!("../../assets/usd-krw.csv");
        let reader = std::io::Cursor::new(csv_file);
        let mut csv_reader = ReaderBuilder::new().from_reader(reader);

        let mut usd_krw = vec![];

        for result in csv_reader.records() {
            let record = result.unwrap();
            let rate_string_opt = record.get(1);
            match rate_string_opt {
                None => {}
                Some(rate) => {
                    let float_rate = rate.trim().replace(',', "").parse::<f64>();
                    match float_rate {
                        Ok(res) => {
                            usd_krw.push(res);
                        }
                        Err(_) => {
                            println!("{:?}", rate);
                        }
                    }
                }
            }
        }

        data.usd_krw = usd_krw;
        data
    }
}
