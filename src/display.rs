use eframe::{self, egui};

pub struct Display {}

impl Display {
    pub fn new(height: f32, width: f32, title: String) -> Self {
        let native_options = eframe::NativeOptions {
            viewport: egui::ViewportBuilder::default()
                .with_inner_size([width, height])
                .with_resizable(false),
            ..Default::default()
        };

        let _ = eframe::run_native(
            title.as_str(),
            native_options,
            Box::new(|cc| Ok(Box::new(Window::new(cc)))),
        );

        Display {}
    }
}

pub struct Window;

impl Window {
    fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        Self {}
    }
}

impl eframe::App for Window {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        egui::Panel::top("top_panel").show_inside(ui, |ui| {
            egui::MenuBar::new().ui(ui, |ui| {
                // NOTE: no File->Quit on web pages!
                let is_web = cfg!(target_arch = "wasm32");
                if !is_web {
                    ui.menu_button("File", |ui| {
                        if ui.button("Quit").clicked() {
                            ui.send_viewport_cmd(egui::ViewportCommand::Close);
                        }
                    });
                    ui.add_space(16.0);
                }

                egui::widgets::global_theme_preference_buttons(ui);
            });
        });

        egui::CentralPanel::default().show_inside(ui, |ui| {
            ui.heading("eframe template");

            ui.separator();
        });
    }
}
