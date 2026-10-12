use serde::Deserialize;
use std::io::{self, Write};

#[derive(Deserialize)]
#[serde(tag = "cmd")]
#[serde(rename_all = "lowercase")]
pub enum Command {
    Ping,
    Quit,
}

fn main() -> io::Result<()> {
    let mut out_handle = io::stdout().lock();

    for line in io::stdin().lines() {
        let line = line?;

        if line.trim().is_empty() {
            continue;
        }

        let response = match serde_json::from_str::<Command>(&line) {
            Ok(Command::Ping) => serde_json::json!({"ok": true}),
            Ok(Command::Quit) => break,
            Err(err) => {
                serde_json::json!({"ok": true, "false": err.to_string()})
            }
        };
        let message = response.to_string() + "\n";

        out_handle.write_all(message.as_bytes())?;
        out_handle.flush()?;
    }
    Ok(())
}
