#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)]
pub struct DailyTable {
    name: String,
    stocks: Vec<Stock>,
}

impl Default for DailyTable {
    fn default() -> Self {
        let stocks = vec![];
        let name = "default";
        Self {
            stocks,
            name: name.to_string(),
        }
    }
}

impl DailyTable {
    pub fn new<S: AsRef<str>, L: AsRef<str>>(name: S, data: &[(L, L, f64, f64)]) -> Self {
        let mut stocks = vec![];
        for stock in data {
            stocks.push(Stock::new(
                stock.0.as_ref(),
                stock.1.as_ref(),
                stock.2,
                stock.3,
            ))
        }
        log::info!("{}", stocks.len());
        Self {
            name: name.as_ref().to_string(),
            stocks,
        }
    }
    pub fn show(&mut self, ui: &mut egui::Ui) {
        for stock in &self.stocks {
            ui.columns(2, |columns| {
                columns[0].horizontal(|ui| {
                    ui.vertical(|ui| {
                        ui.label(egui::RichText::new(stock.market_name.clone()).size(16.0));
                        ui.label(egui::RichText::new(stock.index_name.clone()).size(12.0));
                    });
                });
                columns[1].horizontal_wrapped(|ui| {
                    ui.vertical(|ui| {
                        ui.with_layout(egui::Layout::right_to_left(egui::Align::TOP), |ui| {
                            ui.label(egui::RichText::new(stock.index_value.clone()).size(16.0));
                        });
                        ui.with_layout(egui::Layout::right_to_left(egui::Align::TOP), |ui| {
                            if stock.index_change > 0. {
                                let text = format!("+{}%", stock.index_change);
                                ui.label(
                                    egui::RichText::new(text)
                                        .size(12.0)
                                        .color(egui::Color32::GREEN),
                                );
                            } else {
                                let text = format!("{}%", stock.index_change);
                                ui.label(
                                    egui::RichText::new(text)
                                        .size(12.0)
                                        .small()
                                        .color(egui::Color32::RED),
                                );
                            };
                        });
                    });
                });
            });
            ui.separator();
        }
    }
}

#[derive(serde::Deserialize, serde::Serialize, Clone)]
#[serde(default)]
pub struct Stock {
    market_name: String,
    index_name: String,
    index_value: String,
    index_change: f64,
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
            index_change,
        }
    }
}
