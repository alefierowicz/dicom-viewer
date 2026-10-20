use crate::volume::Volume;

impl Volume {
    // getter for dicom.rs
    pub fn value_range(&self) -> (f64, f64) {
        (self.min, self.max)
    }

    pub fn width(&self) -> usize {
        self.w
    }
    pub fn height(&self) -> usize {
        self.h
    }
}
