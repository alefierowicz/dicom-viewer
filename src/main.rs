mod app;
mod dicom;
mod dtype;
mod pixel;
mod theme;
mod volume;
fn main() -> eframe::Result {
    eframe::run_native(
        "DICOM Viewer",
        eframe::NativeOptions::default(),
        Box::new(|cc| {
            theme::apply(&cc.egui_ctx);
            Ok(Box::new(app::ViewerApp::default()))
        }),
    )
}
