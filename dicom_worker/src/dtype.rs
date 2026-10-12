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
