use std::time::Duration;

/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)]
pub struct App {
    label: String,

    value: f32,
}

impl Default for App {
    fn default() -> Self {
        Self {
            label: "Hello World!".to_owned(),
            value: 2.7,
        }
    }
}

impl App {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        if let Some(storage) = cc.storage {
            return eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
        }

        Default::default()
    }
}

impl eframe::App for App {
    fn auto_save_interval(&self) -> Duration {
        Duration::from_secs(60)
    }

    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }

    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            egui::menu::bar(ui, |ui| {
                ui.menu_button("File", |ui| {
                    #[cfg(not(target_arch = "wasm32"))]
                    if ui.button("Quit").clicked() {
                        frame.close();
                    }
                    if ui.button("Save").clicked() {
                        self.save(frame.storage_mut().unwrap());
                    }
                });

                ui.horizontal(|ui| {
                    egui::warn_if_debug_build(ui);
                });
            });
        });

        egui::SidePanel::left("side_panel").show(ctx, |ui| {
            ui.heading("Side Panel");

            ui.add(egui::Slider::new(&mut self.value, 0.0..=10.0).text("value"));

            ui.with_layout(egui::Layout::bottom_up(egui::Align::LEFT), |_ui| {
                // Add stuff to the bottom of the sidepanel
            });
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Viktor Kasimir wasm stuff");
            ui.hyperlink("https://github.com/vikkasswe");
        });
    }
}
