use serde_json::Value;

use crate::{command::Command, dtype::DType, volume::Volume};

pub fn handle_command(cmd: Command, state: &mut Option<Volume>) -> Result<Value, String> {
    match cmd {
        Command::Ping => Ok(serde_json::json!({"ok": true})),
        Command::Load {
            path,
            dtype,
            shape,
            intercept,
            slope,
        } => {
            let vol = Volume::open(&path, dtype, shape, slope, intercept)?;
            let (min, max) = match vol.dtype {
                DType::I8 => vol.edge_values::<i8>(),
                DType::U8 => vol.edge_values::<u8>(),
                DType::I16 => vol.edge_values::<i16>(),
                DType::U16 => vol.edge_values::<u16>(),
                DType::I32 => vol.edge_values::<i32>(),
                DType::U32 => vol.edge_values::<u32>(),
                DType::F32 => vol.edge_values::<f32>(),
                DType::F64 => vol.edge_values::<f64>(),
            };

            *state = Some(vol);

            Ok(serde_json::json!({
                "ok": true,
                "min": min,
                "max": max
            }))
        }
        Command::Status => match &state {
            None => Err("volume not loaded.".into()),
            Some(vol) => Ok(serde_json::json!({
                "ok": true,
                "parameters": {
                    "dtype": vol.dtype,
                    "shape": [vol.n, vol.h, vol.w],
                    "intercept": vol.intercept,
                    "slope": vol.slope
                }
            })),
        },
        Command::Quit => unreachable!("handled via main"),
    }
}
