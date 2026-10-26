mod landing;
mod viewer;

use crate::dicom::{LoadedFile, load_file};
use eframe::egui;
use rayon::prelude::*;
use std::path::PathBuf;

#[derive(Default)]
pub struct ViewerApp {
    loaded: Option<LoadedFile>,
    texture: Option<egui::TextureHandle>,
    error: Option<String>,
    view: Option<viewer::View>,
    frame: usize,
    hide_unknown: bool,
}

/// which open button was clicked
#[derive(Clone, Copy)]
enum OpenRequest {
    File,
    Folder,
}

impl ViewerApp {
    fn open(&mut self, request: OpenRequest) {
        match request {
            OpenRequest::File => self.open_dialog(),
            OpenRequest::Folder => self.open_folder_dialog(),
        }
    }

    fn open_folder_dialog(&mut self) {
        let Some(folder) = rfd::FileDialog::new().pick_folder() else {
            return;
        };
        let list = match std::fs::read_dir(&folder) {
            Ok(list) => list,
            Err(e) => {
                self.error = Some(e.to_string());
                return;
            }
        };

        let paths: Vec<PathBuf> = list
            .filter_map(|e| e.ok())
            .map(|e| e.path())
            .filter(|p| p.extension().is_some_and(|ext| ext.eq_ignore_ascii_case("dcm")))
            .collect();

        let results: Vec<Result<LoadedFile, String>> = paths.par_iter().map(|p| load_file(p)).collect();
        let mut good_files: Vec<LoadedFile> = Vec::new();
        let mut errors: Vec<String> = Vec::new();

        for file in results {
            match file {
                Ok(f) => good_files.push(f),
                Err(msg) => errors.push(msg),
            }
        }

        println!("{} files loaded, {} failed", good_files.len(), errors.len());
    }

    fn open_dialog(&mut self) {
        let Some(path) = rfd::FileDialog::new().add_filter("DICOM", &["dcm"]).pick_file() else {
            return;
        };

        match load_file(&path) {
            Ok(file) => {
                self.loaded = Some(file);
                self.texture = None;
                self.error = None;
                self.view = None;
                self.frame = 0;
                self.hide_unknown = false;
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
