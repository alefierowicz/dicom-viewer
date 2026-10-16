mod app;
mod dicom;

fn main() -> eframe::Result {
    eframe::run_native(
        "DICOM Viewer",
        eframe::NativeOptions::default(),
        Box::new(|_cc| Ok(Box::new(app::ViewerApp::default()))),
    )
}
