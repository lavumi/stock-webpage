use eframe::emath::Vec2;
use egui_plot::{Line, Plot};

#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)]
pub struct LineChart {
    name: String,
    data: Vec<f64>,
}

impl Default for LineChart {
    fn default() -> Self {
        let data = vec![
            0.1, 0.2, 0.5, 0.3, 0.2, 0.8, 1.2, 0.3, 1.4, 2.6, 1.3, 2.2, 2.3, 2.4,
        ];
        Self {
            name: "default".to_string(),
            data,
        }
    }
}

impl LineChart {
    pub fn new<S: AsRef<str>>(name: S, data: Vec<f64>) -> Self {
        return Self {
            name: name.as_ref().to_string(),
            data: data.clone(),
        };
    }

    #[allow(unused)]
    pub fn show(&mut self, ui: &mut egui::Ui) {
        Plot::new(self.name.clone())
            .show_grid(false)
            .show_background(false)
            .set_margin_fraction(Vec2 { x: 0.1, y: 0.1 })
            .show(ui, |plot_ui| {
                let points: Vec<_> = self
                    .data
                    .iter()
                    .enumerate()
                    .map(|(i, x)| [i as f64, *x])
                    .collect();
                let line = Line::new(points);
                plot_ui.line(line);
            });
    }
}
