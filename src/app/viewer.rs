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
                ui.label(format!("{} fields", file.metadata.len()));
                ui.separator();

                egui::ScrollArea::both().show(ui, |ui| {
                    egui::Grid::new("metadata_grid")
                        .num_columns(3)
                        .striped(true)
                        .show(ui, |ui| {
                            for row in &file.metadata
                            // todo: make this into a checkmark for future use so that its not swamped with UNKNOWNS
                            /* .iter().filter(|val| val.name != "Unknown") */
                            {
                                ui.monospace(&row.tag);
                                ui.label(&row.name);
                                ui.label(&row.value);
                                ui.end_row();
                            }
                        });
                });
            });

        egui::CentralPanel::default().show(ui, |ui| {
            ui.label("image viewer will go here");
        });
    }
}
