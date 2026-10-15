use core::error;

use dicom_object::DefaultDicomObject;
use eframe::egui;
#[derive(Default)]
struct ViewerApp {
    file: Option<DefaultDicomObject>,
    error_msg: Option<String>,
}

impl eframe::App for ViewerApp {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ui, |ui| {
            ui.heading("DICOM Viewer");

            if self.file.is_some() {
                ui.label("DICOM file loaded");
            }
            if self.error_msg.is_some() {
                ui.label(format!("{:?}", self.error_msg));
            }
            if ui.button("Open file").clicked() {
                if let Some(path) = rfd::FileDialog::new()
                    .add_filter("DICOM", &["dcm"])
                    .pick_file()
                {
                    match dicom_object::open_file(&path) {
                        Ok(obj) => {
                            self.file = Some(obj);
                            self.error_msg = None
                        }
                        Err(e) => self.error_msg = Some(e.to_string()),
                    }
                }
            }
        });
    }
}

fn main() -> eframe::Result {
    eframe::run_native(
        "DICOM Viewer",
        eframe::NativeOptions::default(),
        Box::new(|_cc| Ok(Box::new(ViewerApp::default()))),
    )
}
