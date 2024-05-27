#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)]
pub struct DailyTable {
    stocks: Vec<Stock>,
}

impl Default for DailyTable {
    fn default() -> Self {
        let default_stock = Stock::default();
        let stocks = vec![default_stock];
        Self { stocks }
    }
}

impl DailyTable {
    pub fn new() -> Self {
        let default_stock = Stock::default();
        let mut stocks = vec![];
        for _ in 0..10 {
            stocks.push(default_stock.clone());
        }
        // let stocks = vec![default_stock,default_stock,default_stock,default_stock,default_stock,default_stock,default_stock,default_stock,default_stock,default_stock];
        log::info!("{}", stocks.len());
        Self { stocks }
    }
    pub fn show(&mut self, ui: &mut egui::Ui) {
        for stock in &self.stocks {
            ui.columns(2, |columns| {
                columns[0].horizontal(|ui| {
                    ui.vertical(|ui| {
                        ui.label(egui::RichText::new(stock.market_name.clone()).strong());
                        ui.label(egui::RichText::new(stock.index_name.clone()).small());
                    });
                });
                columns[1].horizontal_wrapped(|ui| {
                    ui.vertical(|ui| {
                        ui.with_layout(egui::Layout::right_to_left(egui::Align::TOP), |ui| {
                            ui.label(egui::RichText::new(stock.index_value.clone()).strong());
                        });
                        ui.with_layout(egui::Layout::right_to_left(egui::Align::TOP), |ui| {
                            ui.label(
                                egui::RichText::new(stock.index_change.clone())
                                    .small()
                                    .color(egui::Color32::GREEN),
                            );
                        });
                    });
                });
            });

            ui.add_space(10.0);
        }
        // log::info!("{}" ,self.stocks.len());
    }
}

#[derive(serde::Deserialize, serde::Serialize, Clone)]
#[serde(default)]
pub struct Stock {
    market_name: String,
    index_name: String,
    index_value: String,
    index_change: String,
}

impl Default for Stock {
    fn default() -> Self {
        Stock::new("NASDAQ", "NASDAQ Composite", 16_920.80, 11.0)
    }
}

impl Stock {
    pub fn new<S: AsRef<str>>(
        market_name: S,
        index_name: S,
        index_value: f64,
        index_change: f64,
    ) -> Self {
        Self {
            market_name: market_name.as_ref().to_string(),
            index_name: index_name.as_ref().to_string(),
            index_value: index_value.to_string(),
            index_change: format!("+{}%", index_change),
        }
    }
}
