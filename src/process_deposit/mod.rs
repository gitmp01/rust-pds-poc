use super::data;
use alloy::primitives::Bytes;

mod bip32_ext;
mod evm;

pub fn process_deposit(
    header: data::Header,
    message: Bytes,
) -> Result<String, Box<dyn std::error::Error>> {
    let commitment_bytes = message.slice(0..);
    let commitment: [&str; 7] = minicbor::decode(&commitment_bytes)?;
    // FIXME: inefficient
    let commitment_vec: Vec<String> = commitment.iter().map(|s| s.to_string()).collect();

    match header.protocol {
        1 => evm::handle_deposit(commitment_vec),
        _ => {
            return Err(format!("Unsupported protocol: {}", header.protocol).into());
        }
    }
}
