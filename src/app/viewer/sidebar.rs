use eframe::egui;

use crate::app::{OpenRequest, ViewerApp};

impl ViewerApp {
    /// Left panel with the file name and metadata. Returns which open button was clicked, if any.
    pub(super) fn show_sidebar(&mut self, ui: &mut egui::Ui) -> Option<OpenRequest> {
        let Some(file) = &self.loaded else { return None };
        let mut request = None;

        egui::Panel::left("metadata").resizable(false).exact_size(500.0).show(ui, |ui| {
            ui.horizontal(|ui| {
                if ui.button("open file").clicked() {
                    request = Some(OpenRequest::File);
                }
                if ui.button("open folder").clicked() {
                    request = Some(OpenRequest::Folder);
                }
            });
            if let Some(msg) = &self.error {
                let red = ui.visuals().error_fg_color;
                ui.colored_label(red, msg);
            }

            ui.separator();
            ui.heading(&file.name);
            ui.checkbox(&mut self.hide_unknown, "hide unknown tags");

            let rows: Vec<_> =
                file.metadata.iter().filter(|row| !(self.hide_unknown && row.name == "Unknown")).collect();

            ui.label(format!("{} of {} fields", rows.len(), file.metadata.len()));
            ui.separator();

            egui::ScrollArea::both().show(ui, |ui| {
                egui::Grid::new("metadata_grid").num_columns(3).striped(true).show(ui, |ui| {
                    for row in rows {
                        ui.monospace(&row.tag);
                        ui.label(&row.name);
                        ui.label(&row.value);
                        ui.end_row();
                    }
                });
            });
        });

        request
    }
}
