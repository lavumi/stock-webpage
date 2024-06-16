use crate::modals::data_reader::Record;
use chrono::{Datelike, Local, NaiveDate};
use eframe::emath::Vec2;
use egui_plot::{Bar, BarChart, Plot};

pub enum Period {
    Daily,
    Monthly,
    Yearly,
}

fn format_balance(value: f64) -> String {
    if value.abs() >= 1_000_000_000_000.0 {
        format!("{:.2}T$", value / 1_000_000_000_000.0)
    } else if value.abs() >= 1_000_000_000.0 {
        format!("{:.2}B$", value / 1_000_000_000.0)
    } else if value.abs() >= 1_000_000.0 {
        format!("{:.2}M$", value / 1_000_000.0)
    } else if value.abs() >= 1_000.0 {
        format!("{:.2}k$", value / 1_000.0)
    } else {
        format!("{:.2}$", value)
    }
}

#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)]
pub struct PeriodChart {
    name: String,
    data: Vec<Record>,
    view_data: Vec<f64>,
    x_axis: Vec<String>,
}

impl Default for PeriodChart {
    fn default() -> Self {
        let data = vec![];
        Self {
            name: "default".to_string(),
            data,
            view_data: vec![],
            x_axis: vec![],
        }
    }
}

impl PeriodChart {
    pub fn new<S: AsRef<str>>(name: S, data: Vec<Record>) -> Self {
        let mut chart = PeriodChart {
            name: name.as_ref().to_string(),
            data: data.clone(),
            view_data: vec![],
            x_axis: vec![],
        };
        chart.set_type(Period::Daily);
        chart
    }

    pub fn set_type(&mut self, period: Period) {
        let today = Local::now().date_naive();
        let mut target_date = vec![];
        target_date.push(today.to_string());
        for i in 0..6 {
            let day = match period {
                Period::Daily => {
                    NaiveDate::from_ymd_opt(today.year(), today.month(), today.day() - i)
                }
                Period::Monthly => NaiveDate::from_ymd_opt(today.year(), today.month() - i, 1),
                Period::Yearly => NaiveDate::from_ymd_opt(today.year() - i as i32, 1, 1),
            };
            target_date.push(day.unwrap().to_string());
        }
        target_date.sort();
        let filtered: Vec<_> = self
            .data
            .iter()
            .filter(|r| target_date.contains(&r.date))
            .collect();

        self.view_data = filtered.iter().map(|r| r.balance).collect();
        self.x_axis = filtered.iter().skip(1).map(|r| r.date.clone()).collect();
    }

    pub fn show(&mut self, ui: &mut egui::Ui) {
        ui.horizontal(|button_ui| {
            button_ui.add_space(6.0);
            if button_ui.button("Daily").clicked() {
                self.set_type(Period::Daily)
            }
            button_ui.add_space(6.0);
            if button_ui.button("Monthly").clicked() {
                self.set_type(Period::Monthly)
            }
            button_ui.add_space(6.0);
            if button_ui.button("Yearly").clicked() {
                self.set_type(Period::Yearly)
            }
        });

        ui.separator();

        let x_axis = self.x_axis.clone();
        Plot::new(self.name.clone())
            .show_background(false)
            .show_grid(false)
            .allow_scroll(false)
            .set_margin_fraction(Vec2 { x: 0.15, y: 0.07 })
            .y_axis_formatter(|value, _, _| format_balance(value.value))
            .x_axis_formatter(move |value, _, _| {
                x_axis
                    .get(value.value as usize)
                    .unwrap_or(&"".to_string())
                    .clone()
            })
            .view_aspect(1.4)
            .show(ui, |plot_ui| {
                let bars: Vec<_> = self
                    .view_data
                    .windows(2)
                    .enumerate()
                    .map(|(i, window)| Bar::new(i as f64, window[1] - window[0]))
                    .collect();
                let chart = BarChart::new(bars);
                plot_ui.bar_chart(chart);
            });
    }
}
