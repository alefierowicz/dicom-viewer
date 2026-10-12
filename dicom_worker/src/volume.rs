use crate::dtype::DType;
pub struct Volume {
    pub dtype: DType,
    pub n: usize,
    pub h: usize,
    pub w: usize,
    pub slope: f64,
    pub intercept: f64,
}
