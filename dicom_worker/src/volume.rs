use crate::dtype::{DType, Sample};
use memmap2::Mmap;
use std::fs::File;
use std::io::Error;

pub struct Volume {
    pub dtype: DType,
    pub n: usize,
    pub h: usize,
    pub w: usize,
    pub slope: f64,
    pub intercept: f64,
    pub map: Mmap,
}

impl Volume {
    pub fn open(
        path: &str,
        dtype: DType,
        shape: [usize; 3],
        slope: f64,
        intercept: f64,
    ) -> Result<Volume, String> {
        if shape.contains(&0) {
            Err(String::from("invalid shape"))
        } else {
            let file = File::open(&path)
                .map_err(|e: Error| -> String { format!("cannot open file {path}: {e}") })?;

            // the file isnt modified whatsoever by the worker.
            // trust me bro
            let map = unsafe { Mmap::map(&file) }.map_err(|e| -> String {
                format! {"failure mapping file to memory: {e}"}
            })?;

            let expected = shape.iter().product::<usize>() * dtype.size();
            if map.len() != expected {
                Err(format!(
                    "invalid file size - expected {} bytes, got {} bytes",
                    expected,
                    map.len()
                ))
            } else {
                Ok(Volume {
                    dtype,
                    n: shape[0],
                    h: shape[1],
                    w: shape[2],
                    slope,
                    intercept,
                    map,
                })
            }
        }
    }

    pub fn edge_values<T: Sample>(&self) -> (f64, f64) {
        let mut min: f64 = f64::INFINITY;
        let mut max: f64 = f64::NEG_INFINITY;

        for chunk in self.map.chunks_exact(T::SIZE) {
            let raw = T::read(chunk);
            let value = raw * self.slope + self.intercept;

            min = min.min(value);
            max = max.max(value);
        }

        (min, max)
    }
}
