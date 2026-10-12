use serde_json::Value;

use crate::{command::Command, volume::Volume};

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
            // this gets returned out of order in the JSON since its a sorted tree map
            // doesnt matter when reading it via serializers though so whatever
            *state = Some(Volume {
                dtype,
                n: shape[0],
                h: shape[1],
                w: shape[2],
                slope,
                intercept,
            });

            Ok(serde_json::json!({
                "ok": true,
                "path": path,
                "dtype": dtype,
                "shape": shape,
                "intercept": intercept,
                "slope": slope
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
