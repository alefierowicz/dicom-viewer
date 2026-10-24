mod getters;
mod render;
mod visit;

use crate::dtype::DType;
use dicom_object::{FileDicomObject, InMemDicomObject};
use dicom_pixeldata::{
    PhotometricInterpretation::{Monochrome1, Monochrome2, Rgb},
    PixelDecoder,
    PixelRepresentation::{Signed, Unsigned},
    PlanarConfiguration,
};

pub struct Volume {
    dtype: DType, // data type
    n: usize,     // number of frames
    h: usize,
    w: usize,
    slope: f64, // see visit.rs
    intercept: f64,
    min: f64,       // minimum pixel val
    max: f64,       // maximum -||-
    data: Vec<u8>,  // raw bytes
    samples: usize, // how many samples per pixel
    inverted: bool, // mostly for photometric Monochrome1
}

impl Volume {
    // builder
    pub fn build(obj: &FileDicomObject<InMemDicomObject>) -> Result<Volume, String> {
        let decoded = obj.decode_pixel_data().map_err(|e| -> String { format!("failure decoding pixel data: {e}") })?;

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

        let data = decoded.data().to_vec();
        let is_u8 = matches!(dtype, DType::U8);

        let (samples, inverted) = match (decoded.photometric_interpretation(), decoded.samples_per_pixel()) {
            (Monochrome2, 1) => (1, false),
            (Monochrome1, 1) => (1, true),
            (Rgb, 3) if is_u8 && decoded.planar_configuration() == PlanarConfiguration::Standard => (3, false), // only expecting rgb for u8

            (photometric, spp) => {
                return Err(format!("unsupported image type: {photometric:?} with {spp} samples"));
            }
        };

        let n: usize = decoded.number_of_frames() as usize;
        let h: usize = decoded.rows() as usize;
        let w: usize = decoded.columns() as usize;

        let rescale = decoded.rescale().map_err(|e| -> String { format!("failed to fetch rescale values: {e}") })?;

        let first_row = match rescale.first() {
            Some(res) => res,
            None => return Err("failed to read rescale".into()),
        };

        let intercept = first_row.intercept;
        let slope: f64 = first_row.slope;

        let (mut min, mut max) = (f64::INFINITY, f64::NEG_INFINITY);
        Self::visit(dtype, &data, slope, intercept, |v| {
            min = min.min(v);
            max = max.max(v);
        });

        Ok(Volume { n, h, w, dtype, slope, intercept, min, max, data, samples, inverted })
    }
}
