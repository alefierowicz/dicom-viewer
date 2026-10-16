use eframe::egui;

use super::ViewerApp;

impl ViewerApp {
    pub(super) fn show_main(&mut self, ui: &mut egui::Ui) {
        let Some(file) = &self.loaded else { return };

        egui::Panel::left("metadata")
            .resizable(true)
            .default_size(380.0)
            .show(ui, |ui| {
                ui.heading(&file.name);
                ui.separator();
                ui.label("metadata will go here");
            });

        egui::CentralPanel::default().show(ui, |ui| {
            ui.label("image viewer will go here");
        });
    }
}
