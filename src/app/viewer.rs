mod canvas;
mod controls;
mod sidebar;

use eframe::egui;

use super::ViewerApp;

pub(super) struct View {
    zoom: f32,
    offset: egui::Vec2,
}

impl ViewerApp {
    pub(super) fn show_main(&mut self, ui: &mut egui::Ui) {
        let previous_frame: usize = self.frame;

        self.handle_keys(ui);
        let request = self.show_sidebar(ui);
        self.show_controls(ui); // must stay before update_texture, or slider changes get missed
        self.update_texture(ui, previous_frame);
        self.show_canvas(ui);

        if let Some(request) = request {
            self.open(request);
        }
    }

    /// up/down: frames inside the current file, left/right: switch files
    fn handle_keys(&mut self, ui: &mut egui::Ui) {
        let Some(file) = self.files.get(self.current) else { return };
        let frames = file.volume.frames();
        let mut switch_to = None;

        ui.input(|i| {
            if i.key_pressed(egui::Key::ArrowUp) {
                self.frame = (self.frame + 1).min(frames - 1);
            }
            if i.key_pressed(egui::Key::ArrowDown) {
                self.frame = self.frame.saturating_sub(1);
            }
            if i.key_pressed(egui::Key::ArrowRight) {
                switch_to = Some(self.current + 1);
            }
            if i.key_pressed(egui::Key::ArrowLeft) {
                switch_to = self.current.checked_sub(1);
            }
        });

        if let Some(index) = switch_to {
            self.select_file(index);
        }
    }

    /// shows another loaded file, starting from its first frame, fitted to the window
    fn select_file(&mut self, index: usize) {
        if index == self.current || index >= self.files.len() {
            return;
        }
        self.current = index;
        self.frame = 0;
        self.view = None;
        self.texture = None; // forces update_texture to rebuild even though frame may not have changed
    }

    fn update_texture(&mut self, ui: &mut egui::Ui, previous_frame: usize) {
        let Some(file) = self.files.get(self.current) else { return };
        if self.texture.is_some() && self.frame == previous_frame {
            return;
        }

        let (lo, hi) = file.volume.value_range();
        let Some(pixels) = file.volume.render_frame(self.frame, lo, hi) else { return };

        let size = [file.volume.width(), file.volume.height()];
        let image = if file.volume.is_color() {
            egui::ColorImage::from_rgb(size, &pixels)
        } else {
            egui::ColorImage::from_gray(size, &pixels)
        };

        if let Some(texture) = &mut self.texture {
            texture.set(image, egui::TextureOptions::NEAREST);
        } else {
            self.texture = Some(ui.ctx().load_texture("frame", image, egui::TextureOptions::NEAREST));
        }
    }
}
