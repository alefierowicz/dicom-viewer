mod getters;
mod render;
mod visit;

use crate::dtype::DType;
use dicom_object::{FileDicomObject, InMemDicomObject};
use dicom_pixeldata::{
    PixelDecoder,
    PixelRepresentation::{Signed, Unsigned},
};

pub struct Volume {
    dtype: DType,
    n: usize,
    h: usize,
    w: usize,
    slope: f64,
    intercept: f64,
    min: f64,
    max: f64,
    data: Vec<u8>,
}

impl Volume {
    // builder
    pub fn build(obj: &FileDicomObject<InMemDicomObject>) -> Result<Volume, String> {
        let decoded = obj.decode_pixel_data().map_err(|e| -> String {
            format! {"failure decoding pixel data: {e}"}
        })?;

        let data = decoded.data().to_vec();
        if decoded.samples_per_pixel() != 1 {
            return Err("more samples per pixel than 1 is not currently supported.".into());
        }

        let n: usize = decoded.number_of_frames() as usize;
        let h: usize = decoded.rows() as usize;
        let w: usize = decoded.columns() as usize;

        let rescale = decoded
            .rescale()
            .map_err(|e| -> String { format!("failed to fetch rescale values: {e}") })?;

        let first_row = rescale[0];
        let intercept = first_row.intercept;
        let slope: f64 = first_row.slope;

        let dtype = match (decoded.bits_allocated(), decoded.pixel_representation()) {
            (8, Unsigned) => DType::U8,
            (8, Signed) => DType::I8,
            (16, Unsigned) => DType::U16,
            (16, Signed) => DType::I16,
            (32, Unsigned) => DType::U32,
            (32, Signed) => DType::I32,
            (bits, repr) => {
                return Err(format!("unsupported pixel format: {bits} bits, {repr:?}"));
            }
        };
        let (mut min, mut max) = (f64::INFINITY, f64::NEG_INFINITY);
        Self::visit(dtype, &data, slope, intercept, |v| {
            min = min.min(v);
            max = max.max(v);
        });

        Ok(Volume {
            n,
            h,
            w,
            dtype,
            slope,
            intercept,
            min,
            max,
            data,
        })
    }
}
