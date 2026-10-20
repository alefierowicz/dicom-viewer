mod landing;
mod viewer;

use eframe::egui;

use crate::dicom::{LoadedFile, load_file};

#[derive(Default)]
pub struct ViewerApp {
    loaded: Option<LoadedFile>,
    texture: Option<egui::TextureHandle>,
    error: Option<String>,
    view: Option<viewer::View>,
}

impl ViewerApp {
    fn open_dialog(&mut self) {
        let Some(path) = rfd::FileDialog::new()
            .add_filter("DICOM", &["dcm"])
            .pick_file()
        else {
            return;
        };

        match load_file(&path) {
            Ok(file) => {
                self.loaded = Some(file);
                self.texture = None;
                self.error = None;
                self.view = None;
            }
            Err(msg) => self.error = Some(msg),
        }
    }
}

impl eframe::App for ViewerApp {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        if self.loaded.is_none() {
            self.show_landing(ui);
        } else {
            self.show_main(ui);
        }
    }
}
