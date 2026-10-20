use crate::dtype::{DType, Sample};
use dicom_object::{FileDicomObject, InMemDicomObject};
use dicom_pixeldata::{
    PixelDecoder,
    PixelRepresentation::{Signed, Unsigned},
};

pub struct Volume {
    dtype: DType,
    n: u32,
    h: u32,
    w: u32,
    slope: f64,
    intercept: f64,
    min: f64,
    max: f64,
    data: Vec<u8>,
}

impl Volume {
    pub fn edge_values<T: Sample>(data: &[u8], slope: &f64, intercept: &f64) -> (f64, f64) {
        let mut min: f64 = f64::INFINITY;
        let mut max: f64 = f64::NEG_INFINITY;

        for chunk in data.chunks_exact(std::mem::size_of::<T>()) {
            let raw = T::read(chunk);
            let value = raw * slope + intercept;

            min = min.min(value);
            max = max.max(value);
        }

        (min, max)
    }
    pub fn value_range(&self) -> (f64, f64) {
        (self.min, self.max)
    }
    pub fn build(obj: &FileDicomObject<InMemDicomObject>) -> Result<Volume, String> {
        let decoded = obj.decode_pixel_data().map_err(|e| -> String {
            format! {"failure decoding pixel data: {e}"}
        })?;

        let data = decoded.data().to_vec();
        if decoded.samples_per_pixel() != 1 {
            return Err("more samples per pixel than 1 is not currently supported.".into());
        }

        let n: u32 = decoded.number_of_frames();
        let h: u32 = decoded.rows();
        let w: u32 = decoded.columns();

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
        let min_max: (f64, f64) = match dtype {
            DType::I8 => Volume::edge_values::<i8>(&data, &slope, &intercept),
            DType::U8 => Volume::edge_values::<u8>(&data, &slope, &intercept),
            DType::I16 => Volume::edge_values::<i16>(&data, &slope, &intercept),
            DType::U16 => Volume::edge_values::<u16>(&data, &slope, &intercept),
            DType::I32 => Volume::edge_values::<i32>(&data, &slope, &intercept),
            DType::U32 => Volume::edge_values::<u32>(&data, &slope, &intercept),
        };

        println!(
            "[INFO]shape: [{}x{}x{}], intercept: {}, slope: {}, min: {}, max: {}",
            n, h, w, intercept, slope, min_max.0, min_max.1
        );

        Ok(Volume {
            n,
            h,
            w,
            dtype,
            slope,
            intercept,
            min: min_max.0,
            max: min_max.1,
            data,
        })
    }
}
