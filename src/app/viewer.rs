use eframe::egui;

use super::ViewerApp;

pub(super) struct View {
    zoom: f32,
    offset: egui::Vec2,
}

impl ViewerApp {
    pub(super) fn show_main(&mut self, ui: &mut egui::Ui) {
        let Some(file) = &self.loaded else { return };
        if self.texture.is_none() {
            let (lo, hi) = file.volume.value_range();
            if let Some(pixels) = file.volume.render_frame(0, lo, hi) {
                let size = [file.volume.height(), file.volume.width()];
                let image = egui::ColorImage::from_gray(size, &pixels);

                self.texture = Some(ui.ctx().load_texture(
                    "frame",
                    image,
                    egui::TextureOptions::NEAREST,
                ))
            }
        }
        egui::Panel::left("metadata")
            .resizable(false)
            .exact_size(380.0)
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
                            // .iter().filter(|val| val.name != "Unknown")
                            // todo: make this into a checkmark for future use so that its not swamped with UNKNOWNS
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
            let Some(texture) = &self.texture else {
                ui.label("cannot display this image");
                return;
            };
            Self::show_image(ui, texture, &mut self.view);
        });
    }

    fn show_image(ui: &mut egui::Ui, texture: &egui::TextureHandle, view: &mut Option<View>) {
        // reserving space to paint image
        // rect is the size of the free space, response - what we did just now (ie. click drag zoom whatever)
        let (rect, response) =
            ui.allocate_exact_size(ui.available_size(), egui::Sense::click_and_drag());
        let image_size = texture.size_vec2();

        // fitting the image on screen
        if view.is_none() || response.double_clicked() {
            let zoom = (rect.width() / image_size.x).min(rect.height() / image_size.y);
            let offset = (rect.size() - image_size * zoom) / 2.0;
            // ^ measured from left to right, divided by two so the image is centered

            *view = Some(View { zoom, offset });
        }
        let Some(view) = view.as_mut() else { return };

        // dragging the image
        view.offset += response.drag_delta();

        // zoom
        if let Some(cursor) = response.hover_pos() {
            let scroll = ui.input(|i| i.smooth_scroll_delta.y);

            if scroll != 0.0 {
                let factor = (scroll * 0.002).exp();
                let cursor_in_panel = cursor - rect.min; // all cursor calculations are based off the top-left corner as default
                view.offset = cursor_in_panel - (cursor_in_panel - view.offset) * factor;
                view.zoom *= factor;
            }
        }

        let image_rect = egui::Rect::from_min_size(rect.min + view.offset, image_size * view.zoom);
        let uv = egui::Rect::from_min_max(egui::pos2(0.0, 0.0), egui::pos2(1.0, 1.0));

        ui.painter_at(rect)
            .image(texture.id(), image_rect, uv, egui::Color32::WHITE);
    }
}
