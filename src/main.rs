use std::env;
use std::io::{self, Read, Write};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Read plain text input from stdin
    let mut message = String::new();
    io::stdin().read_to_string(&mut message)?;
    let message = message.trim();

    // Get NEAR environment variables
    let sender_id = env::var("NEAR_SENDER_ID").unwrap_or_else(|_| "unknown".to_string());
    let block_height = env::var("NEAR_BLOCK_HEIGHT").unwrap_or_else(|_| "unknown".to_string());

    // Create echo message with context
    let echo_message = format!(
        "{} said \"{}\" at block {}",
        sender_id, message, block_height
    );

    print!("{}", echo_message);
    io::stdout().flush()?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_output_serialization() {
        let output = Output {
            echo: "user.near said \"test\" at block 12345".to_string(),
        };
        let json = serde_json::to_string(&output).unwrap();
        assert!(json.contains("user.near"));
        assert!(json.contains("test"));
        assert!(json.contains("12345"));
    }
}
