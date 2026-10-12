use crate::dtype::DType;
use serde::Deserialize;

#[derive(Deserialize)]
#[serde(tag = "cmd")]
#[serde(rename_all = "lowercase")]
pub enum Command {
    Ping,
    Quit,
    Load {
        path: String,
        dtype: DType,
        shape: [usize; 3],
        #[serde(default)] // defaults to 0.0
        intercept: f64,
        #[serde(default = "Command::default_slope")]
        slope: f64,
    },
    Status,
}
impl Command {
    pub fn default_slope() -> f64 {
        // sadly needed for default value when deserialzing enums
        1.0
    }
}
