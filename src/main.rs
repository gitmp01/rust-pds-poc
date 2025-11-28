use alloy::hex;
use alloy::primitives::Bytes;
use std::io::{self, Read, Write};

mod data;
mod init;
mod process_deposit;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Read plain text input from stdin
    let mut json_str = String::new();
    io::stdin().read_to_string(&mut json_str)?;
    let json_str = json_str.trim();

    // Parse JSON from message
    let json: data::Input = serde_json::from_str(&json_str)?;
    let message_bytes = Bytes::from(hex::decode(&json.message)?);
    let version = message_bytes.slice(0..1);
    let network_code = message_bytes.slice(1..9);
    let protocol = u32::from_be_bytes(message_bytes[1..5].try_into()?);
    let chain_id = u32::from_be_bytes(message_bytes[5..9].try_into()?);
    let command_bytes = message_bytes.slice(9..13);
    let command = u32::from_be_bytes(command_bytes[0..4].try_into()?);
    let message = message_bytes.slice(13..);

    let header = data::Header {
        version,
        network_code,
        protocol,
        chain_id,
        command,
    };

    let result = match command {
        0 => init::init(),
        1 => process_deposit::process_deposit(header, message),
        _ => {
            return Err(format!("Unknown command: {}", command).into());
        }
    };

    let output = data::Output { result: result? };

    let json = serde_json::to_string(&output)?;
    print!("{}", json);
    io::stdout().flush()?;

    Ok(())
}
