use crate::volume::Volume;

impl Volume {
    pub fn frames(&self) -> usize {
        self.n
    }

    pub fn value_range(&self) -> (f64, f64) {
        (self.min, self.max)
    }

    pub fn width(&self) -> usize {
        self.w
    }
    pub fn height(&self) -> usize {
        self.h
    }
    pub fn is_color(&self) -> bool {
        self.samples == 3
    }
}
