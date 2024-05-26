use std::f64::consts::TAU;

use egui_plot::{Legend, Plot, PlotPoint, PlotPoints, Polygon, Text};
use egui::{Align2, RichText};

const FULL_CIRCLE_VERTICES: f64 = 240.0;
const RADIUS: f64 = 1.0;


#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)]
pub struct PieChart {
    name: String,
    sectors: Vec<Sector>,
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

        Self {
            name: name.as_ref().to_string(),
            sectors,
        }
    }

    pub fn show(&mut self, ui: &mut egui::Ui) {
        let sectors = self.sectors.clone();

        Plot::new(&self.name)
            .label_formatter(|_: &str, _: &PlotPoint| String::default())
            .show_background(false)
            .legend(Legend::default())
            .show_axes([false; 2])
            .allow_boxed_zoom(false)
            .allow_drag(false)
            .allow_zoom(false)
            .allow_scroll(false)
            .data_aspect(1.0)
            .show_x(false)
            .show_y(false)
            .show_grid([false;2])
            // .set_margin_fraction([0.7; 2].into()) // this won't prevent the plot from moving
            // `include_*` will lock it into place
            // .include_x(-2.0)
            // .include_x(2.0)
            // .include_y(-2.0)
            // .include_y(2.0)
            .show(ui, |plot_ui| {
                for sector in sectors.into_iter() {
                    let highlight = plot_ui.pointer_coordinate().map(|p| sector.contains(&p)).unwrap_or_default();
                    let Sector { name, points, .. } = sector;

                    plot_ui.polygon(Polygon::new(PlotPoints::new(points)).name(&name).highlight(highlight));

                    if highlight {
                        let p = plot_ui.pointer_coordinate().unwrap();

                        // TODO proper zoom
                        let text = RichText::new(&name).size(15.0).heading();
                        plot_ui.text(Text::new(p, text).name(&name).anchor(Align2::LEFT_BOTTOM));
                    }
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
}

impl Default for Sector {
    fn default() -> Self {
        Sector::new("default", 0., 1. ,FULL_CIRCLE_VERTICES as usize , 0. )
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

        Self {
            name: name.as_ref().to_string(),
            start,
            end,
            points,
        }
    }

    pub fn contains(&self, &PlotPoint { x, y }: &PlotPoint) -> bool {
        let r = y.hypot(x);
        let mut theta = x.atan2(y);

        if theta < 0.0 {
            theta += TAU;
        }

        r < RADIUS && theta > self.start && theta < self.end
    }
}