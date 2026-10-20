pub enum DType {
    I8,
    U8,
    I16,
    U16,
    I32,
    U32,
}

pub trait Sample {
    fn read(chunk: &[u8]) -> f64;
}

macro_rules! impl_sample {
    ($t: ty) => {
        impl Sample for $t {
            fn read(bytes: &[u8]) -> f64 {
                <$t>::from_ne_bytes(bytes.try_into().unwrap()) as f64
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
