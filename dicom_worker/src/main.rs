use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::io::{self, Write};

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
    fn default_slope() -> f64 {
        // sadly needed for default value when deserialzing enums
        1.0
    }
}

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

pub struct Volume {
    dtype: DType,
    n: usize,
    h: usize,
    w: usize,
    slope: f64,
    intercept: f64,
}

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

fn main() -> io::Result<()> {
    let mut out_handle = io::stdout().lock();
    let mut state: Option<Volume> = None;

    for line in io::stdin().lines() {
        let line = line?;

        if line.trim().is_empty() {
            continue;
        }
        let result: Result<Value, String> = match serde_json::from_str::<Command>(&line) {
            Err(e) => Err(e.to_string()),
            Ok(Command::Quit) => break,
            Ok(cmd) => handle_command(cmd, &mut state),
        };

        let reply = match result {
            Ok(value) => value,
            Err(msg) => serde_json::json!({"ok": false, "error": msg}),
        };

        let message = reply.to_string() + "\n";

        out_handle.write_all(message.as_bytes())?;
        out_handle.flush()?;
    }
    Ok(())
}
