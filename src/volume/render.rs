use crate::volume::{PixelValue, Volume};

impl Volume {
    fn to_display(value: f64, lo: f64, hi: f64) -> u8 {
        let t = (value - lo) / (hi - lo);
        (t.clamp(0.0, 1.0) * 255.0).round() as u8
    }

    pub fn render_frame(&self, frame: usize, lo: f64, hi: f64) -> Option<Vec<u8>> {
        if frame >= self.n {
            return None;
        }
        let frame_len = self.w * self.h * self.dtype.size() * self.samples;
        let start = frame * frame_len;
        let bytes = &self.data[start..start + frame_len];
        if self.is_color() {
            return Some(bytes.to_vec());
        }

        if hi <= lo {
            return None;
        }

        let mut out = Vec::with_capacity(self.w * self.h);

        Self::visit(self.dtype, bytes, self.slope, self.intercept, |v| {
            let b = Self::to_display(v, lo, hi);
            out.push(if self.inverted { 255 - b } else { b });
        });

        Some(out)
    }

    pub fn pixel_value(&self, frame: usize, x: usize, y: usize) -> Option<PixelValue> {
        if frame >= self.n || x >= self.w || y >= self.h {
            return None;
        }

        let size = self.dtype.size();
        let idx = frame * self.h * self.w + y * self.w + x;

        let start = size * idx * self.samples;
        let pixel = self.data.get(start..start + self.samples * size)?;
        let mut value: Option<PixelValue> = None;

        if self.is_color() {
            value = Some(PixelValue::Rgb(pixel[0], pixel[1], pixel[2]))
        } else {
            Self::visit(self.dtype, pixel, 1.0, 0.0, |v| value = Some(PixelValue::Gray(v)));
        }

        value
    }
}
