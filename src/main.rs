use eframe::egui;
#[derive(Default)]
struct ViewerApp {
    file_name: Option<String>,
}

impl ViewerApp {
    fn show_landing(&mut self, ui: &mut egui::Ui) {
        egui::CentralPanel::default().show(ui, |ui| {
            ui.vertical_centered(|ui| {
                ui.add_space(ui.available_height() / 3.0);
                ui.heading("DICOM viewer");
                ui.add_space(12.0);

                if ui.button("open file").clicked() {
                    if let Some(path) = rfd::FileDialog::new()
                        .add_filter("DICOM", &["dcm"])
                        .pick_file()
                    {
                        let name = path
                            .file_name()
                            .map(|n| n.to_string_lossy().to_string())
                            .unwrap_or_default();
                        self.file_name = Some(name);
                    }
                }
            })
        });
    }

    fn show_main(&mut self, ui: &mut egui::Ui) {
        let name = self.file_name.as_deref().unwrap_or("");

        egui::Panel::left("metadata")
            .resizable(true)
            .default_size(380.0)
            .show(ui, |ui| {
                ui.heading(name);
                ui.separator();
                ui.label("metadata will go here");
            });

        egui::CentralPanel::default().show(ui, |ui| {
            ui.label("image viewer will go here");
        });
    }
}

impl eframe::App for ViewerApp {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ui, |ui| {
            ui.heading("DICOM Viewer");

            if self.file_name.is_none() {
                self.show_landing(ui);
            } else {
                self.show_main(ui);
            }
        });
    }
}

fn main() -> eframe::Result {
    eframe::run_native(
        "DICOM Viewer",
        eframe::NativeOptions::default(),
        Box::new(|_cc| Ok(Box::new(ViewerApp::default()))),
    )
}
