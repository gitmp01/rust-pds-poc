use serde::{Deserialize, Serialize};
mod bip32_ext;
mod evm;

#[derive(Deserialize, Serialize)]
struct InputParams {
    chain: String,
    #[serde(rename = "depositAddress")]
    deposit_address: String,
    #[serde(rename = "commitmentParams")]
    commitment_params: Vec<String>,
}

#[derive(Deserialize, Serialize)]
struct Input {
    command: String,
    params: InputParams,
}

pub fn process_deposit(message: &str) -> Result<String, Box<dyn std::error::Error>> {
    println!("Processing deposit with message: {}", message);
    let input: Input = serde_json::from_str(&message)?;

    match input.params.chain.as_str() {
        "evm" => evm::handle_deposit(input.params.deposit_address, input.params.commitment_params),
        _ => {
            return Err(format!("Unsupported chain: {}", input.params.chain).into());
        }
    }
}
