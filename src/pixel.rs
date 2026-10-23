pub enum PixelValue {
    Gray(f64),
    Rgb(u8, u8, u8),
}
impl std::fmt::Display for PixelValue {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match &self {
            PixelValue::Gray(v) => write!(f, "{v:.0}"),
            PixelValue::Rgb(r, g, b) => write!(f, "{r}\n{g}\n{b}"),
        }
    }
}
