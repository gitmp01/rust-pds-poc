use bip32::secp256k1::ecdsa::{
    signature::{Signer, Verifier},
    Signature,
};
use bip32::{Prefix, Seed, XPrv};
use sha3::{Digest, Keccak256};
use std::env;
use std::io::{self, Write};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // // Read plain text input from stdin
    // let mut message = String::new();
    // io::stdin().read_to_string(&mut message)?;
    // let message = message.trim();

    let protected_seed = env::var("PROTECTED_SEED").expect("PROTECTED_SEED not set");
    let seed_length = hex::decode(&protected_seed).unwrap().len();

    println!("seed_len {}", seed_length);

    let seed_u8_64 = hex::decode(&protected_seed)
        .unwrap()
        .try_into()
        .expect("Vector length must be 64");
    let seed = Seed::new(seed_u8_64);

    println!("seed {}", hex::encode(seed.as_bytes()));

    // Derive the root `XPrv` from the `seed` value
    let root_xprv = XPrv::new(&seed)?;
    assert_eq!(root_xprv, XPrv::derive_from_path(&seed, &"m".parse()?)?);

    // Derive a child `XPrv` using the provided BIP32 derivation path
    let child_path = "m/0/0/10";
    let child_xprv = XPrv::derive_from_path(&seed, &child_path.parse()?)?;

    // Get the `XPub` associated with `child_xprv`.
    let child_xpub = child_xprv.public_key();

    // Serialize `child_xprv` as a string with the `xprv` prefix.
    let child_xprv_str = child_xprv.to_string(Prefix::XPRV);
    assert!(child_xprv_str.starts_with("xprv"));

    // Serialize `child_xpub` as a string with the `xpub` prefix.
    let child_xpub_str = child_xpub.to_string(Prefix::XPUB);
    assert!(child_xpub_str.starts_with("xpub"));

    // Get the ECDSA/secp256k1 signing and verification keys for the xprv and xpub
    let signing_key = child_xprv.private_key();
    let verification_key = child_xpub.public_key();

    let example_msg = b"Hello, world!";
    let signature: Signature = signing_key.sign(example_msg);
    assert!(verification_key.verify(example_msg, &signature).is_ok());

    let public_key_bytes = verification_key.to_encoded_point(false);
    let public_key_bytes_no_prefix = &public_key_bytes.as_bytes()[1..];
    // 3. Hash with Keccak256
    let mut hasher = Keccak256::new();
    hasher.update(public_key_bytes_no_prefix);
    let hash = hasher.finalize();
    // 4. Take last 20 bytes
    let eth_address = &hash[12..];

    println!("ext    priv key: {:?}", root_xprv.to_string(Prefix::XPRV));
    println!("child  priv key  {:?}", child_xprv_str);
    println!("child  pub  key  {:?}", child_xpub_str);
    println!("priv      0x{}", hex::encode(signing_key.to_bytes()));
    println!("address   0x{}", hex::encode(eth_address));
    println!("signature 0x{}", signature);
    io::stdout().flush()?;

    Ok(())
}

pub fn hex_to_bip32_path(hex_string: &str) -> Result<String, String> {
    // Remove '0x' prefix if present
    let clean_hex = if hex_string.starts_with("0x") || hex_string.starts_with("0X") {
        &hex_string[2..]
    } else {
        hex_string
    };

    // Validate input length (32 bytes = 64 hex characters)
    if clean_hex.len() != 64 {
        return Err(format!(
            "Expected 64 hex characters (32 bytes), got {}",
            clean_hex.len()
        ));
    }

    // Validate hex characters
    if !clean_hex.chars().all(|c| c.is_ascii_hexdigit()) {
        return Err("Invalid hex string: contains non-hexadecimal characters".to_string());
    }

    let mut path = vec!["m".to_string()];

    // Divide into 8 chunks of 4 bytes (8 hex characters) each
    for i in (0..64).step_by(8) {
        let chunk = &clean_hex[i..i + 8];
        // Convert 4-byte chunk to integer (big-endian)
        let value = u32::from_str_radix(chunk, 16)
            .map_err(|e| format!("Failed to parse hex chunk '{}': {}", chunk, e))?;
        path.push(value.to_string());
    }

    Ok(path.join("/"))
}
