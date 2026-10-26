mod landing;
mod viewer;

use crate::dicom::{LoadedFile, load_file};
use eframe::egui;
use rayon::{iter::Either, prelude::*};
use std::path::PathBuf;

#[derive(Default)]
pub struct ViewerApp {
    files: Vec<LoadedFile>, // a single opened file is just a vec of one
    current: usize,         // index into `files` of the one being shown
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

        let mut paths: Vec<PathBuf> = list
            .filter_map(|e| e.ok())
            .map(|e| e.path())
            .filter(|p| p.extension().is_some_and(|ext| ext.eq_ignore_ascii_case("dcm")))
            .collect();
        paths.sort();

        let (mut good_files, errors): (Vec<LoadedFile>, Vec<String>) =
            paths.par_iter().map(|p| load_file(p)).partition_map(|r| match r {
                Ok(file) => Either::Left(file),
                Err(msg) => Either::Right(msg),
            });

        good_files.sort_by_key(|f| f.instance);
        println!("[DEBUG] {} files loaded, {} failed", good_files.len(), errors.len());

        if good_files.is_empty() {
            self.error = Some(format!("no loadable .dcm files in this folder ({} failed)", errors.len()));
            return;
        }
        self.set_files(good_files);
    }

    fn open_dialog(&mut self) {
        let Some(path) = rfd::FileDialog::new().add_filter("DICOM", &["dcm"]).pick_file() else {
            return;
        };

        match load_file(&path) {
            Ok(file) => self.set_files(vec![file]),
            Err(msg) => self.error = Some(msg),
        }
    }

    /// replaces whatever was loaded and starts from the first file
    fn set_files(&mut self, files: Vec<LoadedFile>) {
        self.files = files;
        self.current = 0;
        self.texture = None;
        self.error = None;
        self.view = None;
        self.frame = 0;
        self.hide_unknown = false;
    }
}

impl eframe::App for ViewerApp {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        if self.files.is_empty() {
            self.show_landing(ui);
        } else {
            self.show_main(ui);
        }
    }
}
