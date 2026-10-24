use eframe::egui::{self, Color32, CornerRadius, FontData, FontDefinitions, FontFamily, Stroke, Theme};
use std::sync::Arc;

pub const BG: Color32 = Color32::from_rgb(11, 18, 32); // #0B1220
pub const SURFACE: Color32 = Color32::from_rgb(20, 30, 52); // #141E34 buttons
pub const SURFACE_HI: Color32 = Color32::from_rgb(30, 44, 74); // #1E2C4A hover
pub const DEEP: Color32 = Color32::from_rgb(6, 10, 20); // #060A14
pub const TEXT: Color32 = Color32::from_rgb(220, 227, 240);
pub const ACCENT: Color32 = Color32::from_rgb(242, 140, 40); // #F28C28

pub const BOLD: &str = "bold";

pub fn apply(ctx: &egui::Context) {
    // load_fonts(ctx);

    ctx.set_theme(Theme::Dark);
    ctx.style_mut_of(Theme::Dark, |style| {
        let v = &mut style.visuals;
        v.panel_fill = BG;
        v.window_fill = SURFACE;

        v.extreme_bg_color = DEEP;
        v.faint_bg_color = SURFACE;
        v.hyperlink_color = ACCENT;
        v.selection.bg_fill = ACCENT;
        v.selection.stroke = Stroke::new(1.0, BG);

        let w = &mut v.widgets;
        w.noninteractive.bg_fill = BG;
        w.noninteractive.bg_stroke = Stroke::new(1.0, SURFACE_HI);
        w.noninteractive.fg_stroke = Stroke::new(1.0, TEXT);

        w.inactive.weak_bg_fill = SURFACE;
        w.inactive.bg_fill = SURFACE_HI;
        w.inactive.fg_stroke = Stroke::new(1.0, TEXT);

        w.hovered.weak_bg_fill = SURFACE_HI;
        w.hovered.bg_fill = SURFACE_HI;
        w.hovered.bg_stroke = Stroke::new(1.0, ACCENT);
        w.hovered.fg_stroke = Stroke::new(1.5, Color32::WHITE);

        w.active.weak_bg_fill = ACCENT;
        w.active.bg_fill = ACCENT;
        w.active.bg_stroke = Stroke::new(1.0, ACCENT);
        w.active.fg_stroke = Stroke::new(2.0, Color32::WHITE);

        for state in [&mut w.inactive, &mut w.hovered, &mut w.active] {
            state.corner_radius = CornerRadius::same(6);
        }
    })
}

fn load_fonts(ctx: &egui::Context) {
    let mut fonts = FontDefinitions::default();
    fonts
        .font_data
        .insert(BOLD.to_owned(), Arc::new(FontData::from_static(include_bytes!("../assets/MartianMono-SemiBold.ttf"))));
    fonts.families.insert(FontFamily::Name(BOLD.into()), vec![BOLD.to_owned()]);
    ctx.set_fonts(fonts);
}
