use eframe::egui;

use crate::app::ViewerApp;

impl ViewerApp {
    /// floating box in the top-right corner: slice slider and zoom/offset readout
    pub(super) fn show_controls(&mut self, ui: &mut egui::Ui) {
        let Some(file) = self.files.get(self.current) else { return };
        let frames = file.volume.frames();

        egui::Area::new(egui::Id::unique("view_controls"))
            .anchor(egui::Align2::RIGHT_TOP, egui::vec2(-12.0, 12.0))
            .show(ui.ctx(), |ui| {
                egui::Frame::popup(ui.style()).show(ui, |ui| {
                    ui.style_mut().wrap_mode = Some(egui::TextWrapMode::Extend);
                    ui.set_min_width(240.0);

                    if frames > 1 {
                        ui.add(egui::Slider::new(&mut self.frame, 0..=frames - 1).text("slice"));
                    }
                    if let Some(v) = &self.view {
                        ui.monospace(format!("zoom {:.2}x   offset ({:.0}, {:.0})", v.zoom, v.offset.x, v.offset.y));
                    }
                });
            });
    }
}
