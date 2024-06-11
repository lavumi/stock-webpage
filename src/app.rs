use crate::modals::*;

/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
pub struct WrapApp {
    pie_chart: PieChart,
    daily_table: DailyTable,
    line_chart: LineChart,
}

impl Default for WrapApp {
    fn default() -> Self {
        let pf_data = PortfolioRawData::new();

        let pie_chart_data: Vec<(f64, String)> = pf_data
            .holdings
            .iter()
            .map(|data| {
                (
                    data.owned as f64 * data.close_price,
                    data.symbol.to_string(),
                )
            })
            .collect();

        let pie_chart = PieChart::new("Percents", &pie_chart_data);

        let daily_table_data: Vec<(String, String, f64, f64)> = pf_data
            .holdings
            .iter()
            .map(|data| {
                let change =
                    (data.close_price - data.yesterday_price) / data.yesterday_price * 100.0;
                (
                    data.symbol.to_string(),
                    data.company_name.to_string(),
                    data.close_price,
                    change,
                )
            })
            .collect();

        let daily_table = DailyTable::new("daily", &daily_table_data);

        let line_chart = LineChart::new("dollar", pf_data.usd_krw);
        Self {
            pie_chart,
            daily_table,
            line_chart,
        }
    }
}

impl WrapApp {
    /// Called once before the first frame.
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        // This is also where you can customize the look and feel of egui using
        // `cc.egui_ctx.set_visuals` and `cc.egui_ctx.set_fonts`.

        // Load previous app state (if any).
        // Note that you must enable the `persistence` feature for this to work.
        // if let Some(storage) = cc.storage {
        //     return eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
        // }

        Default::default()
    }
}

impl eframe::App for WrapApp {
    /// Called each time the UI needs repainting, which may be many times per second.
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Put your widgets into a `SidePanel`, `TopBottomPanel`, `CentralPanel`, `Window` or `Area`.
        // For inspiration and more examples, go to https://emilk.github.io/egui
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            egui::menu::bar(ui, |_ui| {
                // NOTE: no File->Quit on web pages!
                // let is_web = cfg!(target_arch = "wasm32");
                // if !is_web {
                //     ui.menu_button("File", |ui| {
                //         if ui.button("Quit").clicked() {
                //             ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                //         }
                //     });
                //     ui.add_space(16.0);
                // }
                // egui::widgets::global_dark_light_mode_switch(ui);
                // ui.separator();

                // if ui.button("Organize windows").clicked() {
                //     ui.ctx().memory_mut(|mem| mem.reset_areas());
                // }
            });
        });

        egui::CentralPanel::default().show(ctx, |_| {
            egui::Window::new("positions")
                .title_bar(false)
                // .frame(frame)
                // .open(&mut true)
                .collapsible(false)
                .vscroll(false)
                .hscroll(false)
                .resizable(false)
                .fixed_size([400.0, 400.0])
                .fixed_pos([20., 40.])
                .show(ctx, |ui| {
                    self.pie_chart.show(ui);
                });

            egui::Window::new("Daily")
                // .open(&mut true)
                .title_bar(false)
                .collapsible(false)
                .vscroll(false)
                .hscroll(false)
                .fixed_pos([20., 470.])
                .fixed_size([400.0, 400.0])
                .show(ctx, |ui| {
                    self.daily_table.show(ui);
                });
            egui::Window::new("Dollar")
                .title_bar(false)
                .open(&mut true)
                .collapsible(false)
                .vscroll(false)
                .hscroll(false)
                .resizable(false)
                // .fixed_size([400.0, 170.0])
                .show(ctx, |ui| {
                    self.line_chart.show(ui);
                });
        });
    }

    /// Called by the frame work to save state before shutdown.
    fn save(&mut self, _storage: &mut dyn eframe::Storage) {
        // eframe::set_value(storage, eframe::APP_KEY, self);
    }
}

// fn powered_by_egui_and_eframe(ui: &mut egui::Ui) {
//     ui.horizontal(|ui| {
//         ui.spacing_mut().item_spacing.x = 0.0;
//         ui.label("Powered by ");
//         ui.hyperlink_to("egui", "https://github.com/emilk/egui");
//         ui.label(" and ");
//         ui.hyperlink_to(
//             "eframe",
//             "https://github.com/emilk/egui/tree/master/crates/eframe",
//         );
//         ui.label(".");
//     });
// }
