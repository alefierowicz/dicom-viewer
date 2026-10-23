use eframe::egui;

use super::ViewerApp;
use crate::volume::Volume;
pub(super) struct View {
    zoom: f32,
    offset: egui::Vec2,
}

const MIN_LABEL_ZOOM: f32 = 30.0;
impl ViewerApp {
    pub(super) fn show_main(&mut self, ui: &mut egui::Ui) {
        let Some(file) = &self.loaded else { return };
        let frames = file.volume.frames();
        let previous_frame = self.frame;

        ui.input(|i| {
            if i.key_pressed(egui::Key::ArrowUp) {
                self.frame = (self.frame + 1).min(frames - 1);
            }
            if i.key_pressed(egui::Key::ArrowDown) {
                self.frame = self.frame.saturating_sub(1);
            }
        });

        if self.texture.is_none() {
            let (lo, hi) = file.volume.value_range();
            if let Some(pixels) = file.volume.render_frame(self.frame, lo, hi) {
                let size = [file.volume.width(), file.volume.height()];
                let image = if file.volume.is_color() {
                    egui::ColorImage::from_rgb(size, &pixels)
                } else {
                    egui::ColorImage::from_gray(size, &pixels)
                };
                self.texture = Some(ui.ctx().load_texture("frame", image, egui::TextureOptions::NEAREST))
            }
        }
        egui::Panel::left("metadata").resizable(false).exact_size(500.0).show(ui, |ui| {
            ui.heading(&file.name);
            if frames > 1 {
                ui.add(egui::Slider::new(&mut self.frame, 0..=frames - 1).text("slice"));
            }
            ui.label(format!("{} fields", file.metadata.len()));

            // DEBUG: current offset & zoom level
            if let Some(v) = &self.view {
                ui.label(format!("current zoom level: {}", v.zoom));
                ui.label(format!("current offset: {}", v.offset));
            }
            ui.separator();

            egui::ScrollArea::both().show(ui, |ui| {
                egui::Grid::new("metadata_grid").num_columns(3).striped(true).show(ui, |ui| {
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

        if self.texture.is_none() || self.frame != previous_frame {
            let (lo, hi) = file.volume.value_range();
            if let Some(pixels) = file.volume.render_frame(self.frame, lo, hi) {
                let size = [file.volume.width(), file.volume.height()];
                let image = egui::ColorImage::from_gray(size, &pixels);

                if let Some(texture) = &mut self.texture {
                    texture.set(image, egui::TextureOptions::NEAREST);
                } else {
                    self.texture = Some(ui.ctx().load_texture("frame", image, egui::TextureOptions::NEAREST));
                }
            }
        }
        egui::CentralPanel::default().show(ui, |ui| {
            let Some(texture) = &self.texture else {
                ui.label("cannot display this image");
                return;
            };
            Self::show_image(ui, self.frame, texture, &mut self.view, &file.volume);
        });
    }

    fn show_image(
        ui: &mut egui::Ui,
        frame: usize,
        texture: &egui::TextureHandle,
        view: &mut Option<View>,
        volume: &Volume,
    ) {
        // reserving space to paint image
        // rect is the size of the free space, response - what we did just now (ie. click drag zoom whatever)
        let (rect, response) = ui.allocate_exact_size(ui.available_size(), egui::Sense::click_and_drag());
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

        let painter = ui.painter_at(rect);
        painter.image(texture.id(), image_rect, uv, egui::Color32::WHITE);

        if view.zoom >= MIN_LABEL_ZOOM {
            Self::draw_pixel_values(&painter, rect, image_rect, view.zoom, image_size, volume, frame);
        }
    }

    fn draw_pixel_values(
        painter: &egui::Painter,
        rect: egui::Rect,
        image_rect: egui::Rect,
        zoom: f32,
        image_size: egui::Vec2,
        volume: &Volume,
        frame: usize,
    ) {
        let first = ((rect.min - image_rect.min) / zoom).floor().max(egui::Vec2::ZERO);

        let last = ((rect.max - image_rect.min) / zoom).ceil().min(image_size);
        let font = if volume.is_color() { egui::FontId::monospace(10.0) } else { egui::FontId::monospace(12.0) };

        for y in first.y as usize..last.y as usize {
            for x in first.x as usize..last.x as usize {
                let Some(val) = volume.pixel_value(frame, x, y) else {
                    continue;
                };

                let text = format!("{val}");
                let center = image_rect.min + egui::vec2(x as f32 + 0.5, y as f32 + 0.5) * zoom;

                painter.text(
                    center + egui::vec2(1.0, 1.0),
                    egui::Align2::CENTER_CENTER,
                    &text,
                    font.clone(),
                    egui::Color32::BLACK,
                );
                painter.text(center, egui::Align2::CENTER_CENTER, &text, font.clone(), egui::Color32::WHITE);
            }
        }
    }
}
