use super::bip32_ext::get_derivation_path_from_hash;
use alloy::consensus::{EthereumTxEnvelope, SignableTransaction, TxLegacy};
use alloy::network::TxSignerSync;
use alloy::primitives::{Address, Bytes, ChainId, U256};
use alloy::signers::local::PrivateKeySigner;
use bip32::{Seed, XPrv};
use bytes::{Buf, BufMut};
use k256::ecdsa::VerifyingKey;
use k256::sha2::{Digest, Sha256};
use sha3::Keccak256;
use std::env;

fn get_address(public_key: &VerifyingKey) -> String {
    let public_key_bytes = public_key.to_encoded_point(false);
    let public_key_bytes_no_prefix = &public_key_bytes.as_bytes()[1..];
    let mut hasher = Keccak256::new();
    hasher.update(public_key_bytes_no_prefix);
    let hash = hasher.finalize();
    // last 20 bytes
    hex::encode(&hash[12..])
}

pub fn handle_deposit(
    deposit_address: String,
    commitment_params: Vec<String>,
) -> Result<String, Box<dyn std::error::Error>> {
    let protected_seed =
        env::var("PROTECTED_SEED").expect("PROTECTED_SEED env variable is undefined");

    let seed_u8_64: [u8; 64] = hex::decode(&protected_seed)
        .unwrap()
        .try_into()
        .expect("Vector length must be 64");

    let seed = Seed::new(seed_u8_64);

    let mut hasher = Sha256::new();
    for param in &commitment_params {
        hasher.update(param.as_bytes());
    }
    let hash = hasher.finalize();

    let root_derivation_path = "m/44'/60'/0'/0/0/";
    let derivation_path = format!(
        "{}{}",
        root_derivation_path,
        get_derivation_path_from_hash(hash.as_slice()).unwrap()
    );

    let child_xprv = XPrv::derive_from_path(&seed, &derivation_path.parse()?)?;
    let child_xpub = child_xprv.public_key();

    let private_key = child_xprv.private_key();
    let public_key = child_xpub.public_key();

    let address: Address = get_address(public_key).parse()?;
    let deposit_address: Address = deposit_address.parse()?;

    if address != deposit_address {
        let err = format!(
            "Address mismatch: derived address '{}' does not match deposit address '{}'",
            address, deposit_address
        );
        return Err(err.into());
    }

    // Extract to and calldata from commitment_params
    if commitment_params.len() != 7 {
        return Err("commitment_params must contain exactly 7 elements: [chain_id, nonce, gas_price, gas_limit, to, value, calldata]".into());
    }

    let chain_id: Option<ChainId> = Some(commitment_params[0].parse()?);
    let nonce: u64 = commitment_params[1].parse()?;
    let gas_price: u128 = commitment_params[2].parse()?;
    let gas_limit: u64 = commitment_params[3].parse()?;
    // TODO: revise the commitment_params format as we may want to perform multiple calls
    let to_address: Address = commitment_params[4].parse()?;
    let value: U256 = commitment_params[5].parse()?;
    let calldata: Bytes = commitment_params[6].parse()?;

    let signer = PrivateKeySigner::from_signing_key(private_key.clone());

    let mut tx = TxLegacy {
        nonce: nonce,
        gas_price: gas_price,
        gas_limit: gas_limit,
        to: alloy::primitives::TxKind::Call(to_address),
        value: value,
        input: calldata,
        chain_id: chain_id,
    };

    let signature = signer.sign_transaction_sync(&mut tx)?;

    let signed = tx.into_signed(signature);
    let mut buf = vec![];
    signed.rlp_encode(&mut buf);

    Ok(format!("0x{}", hex::encode(buf)))
}
