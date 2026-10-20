use crate::volume::Volume;

impl Volume {
    fn to_display(value: f64, lo: f64, hi: f64) -> u8 {
        let t = (value - lo) / (hi - lo);
        (t.clamp(0.0, 1.0) * 255.0).round() as u8
    }

    pub fn render_frame(&self, frame: usize, lo: f64, hi: f64) -> Option<Vec<u8>> {
        if frame >= self.n || hi <= lo {
            return None;
        }
        let frame_len = self.w * self.h * self.dtype.size();
        let start = frame * frame_len;

        let bytes = &self.data[start..start + frame_len];
        let mut out = Vec::with_capacity(self.w * self.h);

        Self::visit(self.dtype, bytes, self.slope, self.intercept, |v| {
            out.push(Self::to_display(v, lo, hi))
        });
        Some(out)
    }
}
