mod app;
mod dicom;
mod dtype;
mod pixel;
mod volume;
fn main() -> eframe::Result {
    eframe::run_native(
        "DICOM Viewer",
        eframe::NativeOptions::default(),
        Box::new(|_cc| Ok(Box::new(app::ViewerApp::default()))),
    )
}
