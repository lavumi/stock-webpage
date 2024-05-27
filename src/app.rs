use crate::daily_table::DailyTable;
use crate::pie_chart::PieChart;

/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
pub struct WrapApp {
    // Example stuff:
    label: String,

    #[serde(skip)] // This how you opt-out of serialization of a field
    value: f32,
    pie_chart: PieChart,
    daily_table: DailyTable,
}

impl Default for WrapApp {
    fn default() -> Self {
        let pie_chart = PieChart::new(
            "Percents",
            &[
                (14.58 + 48.13, "Nvidia"),
                (23.42, "Apple"),
                (22.28, "Amazon"),
                (20.62 + 18.26, "Microsoft"),
                (33.97, "TSMC"),
            ],
        );
        let daily_table = DailyTable::new();
        Self {
            // Example stuff:
            label: "Hello World!".to_owned(),
            value: 2.7,
            pie_chart,
            daily_table,
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
            egui::menu::bar(ui, |ui| {
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

                egui::widgets::global_dark_light_mode_switch(ui);
                ui.separator();
            });
        });

        egui::CentralPanel::default().show(ctx, |_ui| {
            egui::Window::new("chart")
                .open(&mut true)
                .collapsible(false)
                // .interactable(false)
                // .title_bar(false)
                .vscroll(false)
                .hscroll(false)
                .resizable(false)
                .default_size([200.0, 350.0])
                .show(ctx, |ui| {
                    self.pie_chart.show(ui);
                });
            egui::Window::new("Daily")
                .open(&mut true)
                .collapsible(false)
                // .interactable(false)
                // .title_bar(false)
                .vscroll(false)
                .hscroll(false)
                .resizable(false)
                .show(ctx, |ui| {
                    self.daily_table.show(ui);
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
