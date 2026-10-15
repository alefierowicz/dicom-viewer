use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Copy)]
#[serde(rename_all = "lowercase")]
pub enum DType {
    I8,
    U8,
    I16,
    U16,
    I32,
    U32,
    F32,
    F64,
}

impl DType {
    pub fn size(self) -> usize {
        match self {
            DType::I8 | DType::U8 => 1,
            DType::I16 | DType::U16 => 2,
            DType::I32 | DType::U32 | DType::F32 => 4,
            DType::F64 => 8,
        }
    }
}

pub trait Sample {
    const SIZE: usize; // size of one value as X bytes
    fn read(chunk: &[u8]) -> f64;
}

macro_rules! impl_sample {
    ($t: ty) => {
        impl Sample for $t {
            const SIZE: usize = std::mem::size_of::<$t>();

            fn read(bytes: &[u8]) -> f64 {
                <$t>::from_le_bytes(bytes.try_into().unwrap()) as f64
            }
        }
    };
}

impl_sample!(i8);
impl_sample!(u8);
impl_sample!(i16);
impl_sample!(u16);
impl_sample!(i32);
impl_sample!(u32);
impl_sample!(f32);
impl_sample!(f64);
