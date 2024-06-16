use std::f64::consts::TAU;

use egui::{Align2, Color32, RichText, Stroke};
use egui_plot::{Plot, PlotPoint, PlotPoints, Polygon, Text};

const FULL_CIRCLE_VERTICES: f64 = 120.0;
const RADIUS: f64 = 1.0;
const R_TAU: f64 = 1.0 / TAU;

#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)]
pub struct PieChart {
    name: String,
    sectors: Vec<Sector>,
    color_preset: Vec<Color32>,
}

impl Default for PieChart {
    fn default() -> Self {
        PieChart::new("Default", &[(14.0, "100%")])
    }
}
impl PieChart {
    pub fn new<S: AsRef<str>, L: AsRef<str>>(name: S, data: &[(f64, L)]) -> Self {
        let sum: f64 = data.iter().map(|(f, _)| f).sum();

        let slices: Vec<_> = data.iter().map(|(f, n)| (f / sum, n)).collect();

        let step = TAU / FULL_CIRCLE_VERTICES;

        let mut offset = 0.0_f64;

        let sectors = slices
            .iter()
            .map(|(p, n)| {
                let vertices = (FULL_CIRCLE_VERTICES * p).round() as usize;

                let start = TAU * offset;
                let end = TAU * (offset + p);

                let sector = Sector::new(n, start, end, vertices, step);

                offset += p;

                sector
            })
            .collect();

        let color_preset = vec![
            Color32::LIGHT_BLUE,
            Color32::KHAKI,
            Color32::LIGHT_RED,
            Color32::LIGHT_YELLOW,
            Color32::LIGHT_GREEN,
            Color32::LIGHT_GRAY,
        ];
        Self {
            name: name.as_ref().to_string(),
            sectors,
            color_preset,
        }
    }

    pub fn show(&mut self, ui: &mut egui::Ui) {
        let sectors = self.sectors.clone();
        Plot::new(&self.name)
            .show_background(false)
            .show_axes([false; 2])
            .allow_boxed_zoom(false)
            .allow_drag(false)
            .allow_zoom(false)
            .allow_scroll(false)
            .show_x(false)
            .show_y(false)
            .view_aspect(1.0)
            .data_aspect(1.0)
            .show_grid([false; 2])
            .show(ui, |plot_ui| {
                for (index, sector) in sectors.into_iter().enumerate() {
                    let Sector {
                        name,
                        points,
                        center,
                        percent,
                        ..
                    } = sector;

                    let stroke = Stroke::new(1.0, self.color_preset[index]);
                    plot_ui.polygon(
                        Polygon::new(PlotPoints::new(points))
                            .name(&name)
                            .stroke(stroke),
                    );
                    let text = RichText::new(format!("{}\n{:.2}%", &name, percent))
                        .monospace()
                        .color(self.color_preset[index]);
                    plot_ui.text(
                        Text::new(PlotPoint::new(center[0], center[1]), text)
                            .anchor(Align2::CENTER_CENTER),
                    );
                }
            });
    }
}

#[derive(serde::Deserialize, serde::Serialize, Clone)]
#[serde(default)]
struct Sector {
    name: String,
    start: f64,
    end: f64,
    points: Vec<[f64; 2]>,
    center: [f64; 2],
    percent: f64,
}

impl Default for Sector {
    fn default() -> Self {
        Sector::new("default", 0., 1., FULL_CIRCLE_VERTICES as usize, 0.)
    }
}
impl Sector {
    pub fn new<S: AsRef<str>>(name: S, start: f64, end: f64, vertices: usize, step: f64) -> Self {
        let mut points = vec![];

        if end - TAU != start {
            points.push([0.0, 0.0]);
        }

        points.push([RADIUS * start.sin(), RADIUS * start.cos()]);

        for v in 1..vertices {
            let t = start + step * v as f64;
            points.push([RADIUS * t.sin(), RADIUS * t.cos()]);
        }

        points.push([RADIUS * end.sin(), RADIUS * end.cos()]);

        let center = [
            points[vertices / 2][0] * 0.66,
            points[vertices / 2][1] * 0.66,
        ];

        let percent = (end - start) * R_TAU * 100.0;
        Self {
            name: name.as_ref().to_string(),
            start,
            end,
            points,
            center,
            percent,
        }
    }

    #[allow(unused)]
    pub fn contains(&self, &PlotPoint { x, y }: &PlotPoint) -> bool {
        let r = y.hypot(x);
        let mut theta = x.atan2(y);

        if theta < 0.0 {
            theta += TAU;
        }

        r < RADIUS && theta > self.start && theta < self.end
    }
}
