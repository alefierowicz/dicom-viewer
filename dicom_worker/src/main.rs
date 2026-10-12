mod command;
mod dtype;
mod handler;
mod volume;

use crate::{command::Command, handler::handle_command, volume::Volume};
use serde_json::Value;
use std::io::{self, Write};

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
