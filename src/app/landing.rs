use eframe::egui;

use crate::theme;

use super::{OpenRequest, ViewerApp};

impl ViewerApp {
    pub(super) fn show_landing(&mut self, ui: &mut egui::Ui) {
        egui::CentralPanel::default().show(ui, |ui| {
            ui.vertical_centered(|ui| {
                ui.add_space(ui.available_height() / 3.0);
                ui.label(
                    egui::RichText::new("DICOM viewer").size(44.0).family(egui::FontFamily::Name(theme::BOLD.into())),
                );
                ui.add_space(24.0);

                let button_size = egui::vec2(180.0, 44.0);
                let mut request = None;

                ui.horizontal(|ui| {
                    // horizontal layouts start at the left edge, so pad by hand to center the two buttons
                    let total = button_size.x * 2.0 + ui.spacing().item_spacing.x;
                    ui.add_space((ui.available_width() - total) / 2.0);

                    let file = egui::Button::new(egui::RichText::new("open file").size(20.0)).min_size(button_size);
                    if ui.add(file).clicked() {
                        request = Some(OpenRequest::File);
                    }
                    let folder = egui::Button::new(egui::RichText::new("open folder").size(20.0)).min_size(button_size);
                    if ui.add(folder).clicked() {
                        request = Some(OpenRequest::Folder);
                    }
                });

                if let Some(request) = request {
                    self.open(request);
                }

                if let Some(msg) = &self.error {
                    ui.add_space(12.0);
                    let red = ui.visuals().error_fg_color;
                    ui.colored_label(red, msg);
                }
            });
        });
    }
}
