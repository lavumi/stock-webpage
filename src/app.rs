use crate::modals::*;
use egui::scroll_area::ScrollBarVisibility;
use egui::{Context, RichText};
use sha2::Digest;

#[derive(serde::Deserialize, serde::Serialize)]
enum AppState {
    BeforeLogin = 0,
    LoggedIn = 1,
}

/// We derive Deserialize/Serialize. so we can persist app state on shutdown.
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
pub struct WrapApp {
    pie_chart: PieChart,
    daily_table: DailyTable,
    line_chart: LineChart,

    app_state: AppState,
    input_password: String,
    stored_hash: String,
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
            app_state: AppState::BeforeLogin,
            input_password: "".to_string(),
            stored_hash: "4ecdc4ec6c0e98bea7165bcb88f79d3a0a95461874705be912fa1d22abaa67ea"
                .to_string(),
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

    fn mobile_view(&mut self, ui: &mut egui::Ui) {
        egui::ScrollArea::vertical()
            .scroll_bar_visibility(ScrollBarVisibility::AlwaysHidden)
            .show(ui, |ui| {
                self.pie_chart.show(ui);
                self.daily_table.show(ui);
            });
    }

    fn desktop_view(&mut self, ctx: &Context, _: &mut egui::Ui) {
        egui::Window::new("Positions")
            .collapsible(false)
            .vscroll(false)
            .hscroll(false)
            .show(ctx, |ui| {
                self.pie_chart.show(ui);
            });

        egui::Window::new("Daily")
            .collapsible(false)
            .vscroll(false)
            .hscroll(false)
            .show(ctx, |ui| {
                self.daily_table.show(ui);
            });
    }

    fn login_view(&mut self, ctx: &Context, _: &mut egui::Ui) {
        egui::Window::new("Login")
            .collapsible(false)
            .vscroll(false)
            .hscroll(false)
            .resizable(false)
            .show(ctx, |ui| {
                ui.label(RichText::new("Enter Password").monospace());
                let text_edit =
                    ui.add(egui::TextEdit::singleline(&mut self.input_password).password(true));
                if text_edit.lost_focus() && ui.input(|i| i.key_pressed(egui::Key::Enter)) {

                    let mut hasher = sha2::Sha256::new();
                    hasher.update(self.input_password.as_bytes());
                    let result = hasher.finalize()  ;
                    let hash_hex: String = result.iter().map(|byte| format!("{:02x}", byte)).collect();
                    if hash_hex == self.stored_hash {
                        self.app_state = AppState::LoggedIn;
                    } else {
                        self.input_password.clear();
                    }
                }
            });
    }
}

impl eframe::App for WrapApp {
    /// Called each time the UI needs repainting, which may be many times per second.
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let screen_size = ctx.screen_rect().size();
        let is_mobile = screen_size.x < screen_size.y;
        // Put your widgets into a `SidePanel`, `TopBottomPanel`, `CentralPanel`, `Window` or `Area`.
        // For inspiration and more examples, go to https://emilk.github.io/egui
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            egui::menu::bar(ui, |menu_ui| {
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
                if !is_mobile {
                    egui::widgets::global_dark_light_mode_switch(menu_ui);
                    menu_ui.separator();
                    if menu_ui.button("Organize windows").clicked() {
                        menu_ui.ctx().memory_mut(|mem| mem.reset_areas());
                    }
                }
            });
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            match self.app_state {
                AppState::BeforeLogin => {
                    self.login_view(ctx, ui);
                }
                AppState::LoggedIn => {
                    if is_mobile {
                        self.mobile_view(ui);
                    } else {
                        self.desktop_view(ctx, ui);
                    }
                }
            }

            // egui::Window::new("Dollar")
            //     .title_bar(false)
            //     .collapsible(false)
            //     .vscroll(false)
            //     .hscroll(false)
            //     .resizable(false)
            //     .show(ctx, |ui| {
            //         self.line_chart.show(ui);
            //     });
        });
    }

    /// Called by the framework to save state before shutdown.
    fn save(&mut self, _storage: &mut dyn eframe::Storage) {
        // eframe::set_value(storage, eframe::APP_KEY, self);
    }
}
