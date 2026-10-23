use crate::{
    dtype::{DType, Sample},
    volume::Volume,
};

// decluttering code by passing closures instead of repeating this massive match expression
impl Volume {
    pub fn visit(dtype: DType, bytes: &[u8], slope: f64, intercept: f64, f: impl FnMut(f64)) {
        match dtype {
            DType::I8 => Self::visit_typed::<i8>(bytes, slope, intercept, f),
            DType::U8 => Self::visit_typed::<u8>(bytes, slope, intercept, f),
            DType::I16 => Self::visit_typed::<i16>(bytes, slope, intercept, f),
            DType::U16 => Self::visit_typed::<u16>(bytes, slope, intercept, f),
            DType::I32 => Self::visit_typed::<i32>(bytes, slope, intercept, f),
            DType::U32 => Self::visit_typed::<u32>(bytes, slope, intercept, f),
        }
    }

    fn visit_typed<T: Sample>(bytes: &[u8], slope: f64, intercept: f64, mut f: impl FnMut(f64)) {
        for chunk in bytes.chunks_exact(std::mem::size_of::<T>()) {
            f(T::read(chunk) * slope + intercept); // https://www.kitware.com/dicom-rescale-intercept-rescale-slope-and-itk/
        }
    }
}
