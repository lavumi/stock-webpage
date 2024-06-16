use crate::modals::*;
use egui::scroll_area::ScrollBarVisibility;
use egui::{Context, RichText};
use sha2::Digest;

#[derive(PartialEq, serde::Deserialize, serde::Serialize)]
enum Tab {
    Holdings,
    Daily,
    History,
}

/// We derive Deserialize/Serialize so we can persist app state on shutdown.
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
    history_chart: PeriodChart,

    mobile_tab: Tab,

    app_state: AppState,
    input_password: String,
    stored_hash: String,
}

impl Default for WrapApp {
    fn default() -> Self {
        let pf_data = PortfolioRawData::default();
        let (pie_chart, daily_table, history_chart) = WrapApp::initialize_data(pf_data);
        Self {
            pie_chart,
            daily_table,
            history_chart,
            mobile_tab: Tab::Holdings,
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
        Default::default()
    }

    fn load(&mut self) {
        let pf_data = PortfolioRawData::new();
        let (pie_chart, daily_table, history_chart) = WrapApp::initialize_data(pf_data);
        self.pie_chart = pie_chart;
        self.daily_table = daily_table;
        self.history_chart = history_chart;
    }

    fn initialize_data(pf_data: PortfolioRawData) -> (PieChart, DailyTable, PeriodChart) {
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

        let history_chart = PeriodChart::new("period", pf_data.history);

        (pie_chart, daily_table, history_chart)
    }

    fn mobile_view(&mut self, ui: &mut egui::Ui) {
        egui::ScrollArea::vertical()
            .scroll_bar_visibility(ScrollBarVisibility::AlwaysHidden)
            .show(ui, |ui| match self.mobile_tab {
                Tab::Holdings => {
                    self.pie_chart.show(ui);
                }
                Tab::Daily => {
                    self.daily_table.show(ui);
                }
                Tab::History => {
                    self.history_chart.show(ui);
                }
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
            .resizable(false)
            .show(ctx, |ui| {
                self.daily_table.show(ui);
            });

        egui::Window::new("Period")
            .collapsible(false)
            .vscroll(false)
            .hscroll(false)
            .show(ctx, |ui| {
                self.history_chart.show(ui);
            });
    }

    fn login_check(&self) -> bool {
        let mut hasher = sha2::Sha256::new();
        hasher.update(self.input_password.as_bytes());
        let result = hasher.finalize();
        use std::fmt::Write;
        let hex_string: String = result.iter().fold(String::new(), |mut acc, byte| {
            write!(&mut acc, "{:02x}", byte).expect("Unable to write");
            acc
        });

        hex_string == self.stored_hash
    }

    fn login_view(&mut self, ctx: &Context, _: &mut egui::Ui) {
        egui::Window::new("Login")
            .collapsible(false)
            .vscroll(false)
            .hscroll(false)
            .resizable(false)
            .fixed_size([270.0, 100.0])
            // .min_width(300.0)
            .show(ctx, |ui| {
                ui.label(RichText::new("Enter Password").monospace());
                let text_edit =
                    ui.add(egui::TextEdit::singleline(&mut self.input_password).password(true));
                ui.add_space(6.0);
                ui.horizontal(|ui| {
                    if ui.button("Guest").clicked() {
                        self.app_state = AppState::LoggedIn;
                    }
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::TOP), |ui| {
                        if ui.button("Enter").clicked()
                            || (text_edit.lost_focus()
                                && ui.input(|i| i.key_pressed(egui::Key::Enter)))
                        {
                            if self.login_check() {
                                self.load();
                                self.app_state = AppState::LoggedIn;
                            } else {
                                self.input_password.clear();
                            }
                        }
                    });
                });
            });
    }
}

impl eframe::App for WrapApp {
    /// Called each time the UI needs repainting, which may be many times per second.
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let screen_size = ctx.screen_rect().size();
        let is_mobile = screen_size.x < screen_size.y;
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
                } else {
                    menu_ui.radio_value(&mut self.mobile_tab, Tab::Holdings, "PieChart");
                    menu_ui.radio_value(&mut self.mobile_tab, Tab::Daily, "Daily");
                    menu_ui.radio_value(&mut self.mobile_tab, Tab::History, "History");
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
