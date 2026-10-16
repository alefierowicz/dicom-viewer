use eframe::egui;

use super::ViewerApp;

impl ViewerApp {
    pub(super) fn show_landing(&mut self, ui: &mut egui::Ui) {
        egui::CentralPanel::default().show(ui, |ui| {
            ui.vertical_centered(|ui| {
                ui.add_space(ui.available_height() / 3.0);
                ui.heading("DICOM viewer");
                ui.add_space(12.0);

                if ui.button("open file").clicked() {
                    self.open_dialog();
                }
                if let Some(msg) = &self.error {
                    ui.add_space(8.0);
                    ui.colored_label(egui::Color32::DARK_RED, msg);
                }
            });
        });
    }
}
