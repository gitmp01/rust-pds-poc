use serde::{Deserialize, Serialize};
use std::io::{self, Read, Write};

mod init;
mod process_deposit;

#[derive(Deserialize, Serialize)]
struct Input {
    command: String,
}

#[derive(Serialize)]
struct Output {
    result: String,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Read plain text input from stdin
    let mut message = String::new();
    io::stdin().read_to_string(&mut message)?;
    let message = message.trim();

    // Parse JSON from message
    let json_value: serde_json::Value = serde_json::from_str(&message)?;

    let command = json_value["command"]
        .as_str()
        .ok_or("Missing 'command' field")?;

    let result = match command {
        "init" => init::init(),
        "processDeposit" => process_deposit::process_deposit(message),
        _ => {
            return Err(format!("Unknown command: {}", command).into());
        }
    };

    let output = Output {
        result: result.unwrap(),
    };

    let json = serde_json::to_string(&output)?;
    print!("{}", json);
    io::stdout().flush()?;

    Ok(())
}
