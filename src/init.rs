use serde::Serialize;

use bip32::secp256k1::ecdsa::{
    signature::{Signer, Verifier},
    Signature,
};
use bip32::{Prefix, Seed, XPrv, XPub};
use sha3::{Digest, Keccak256};
use std::env;
use std::io::{self, Read, Write};

// Returns:
//
// | version |     network code     | Compressed public key (signing) | Public Key child code |
// |         | protocol  | chain id |                                 |                       |
// |---------+-----------+----------+---------------------------------+-----------------------|
// |    1B   |    4B     |   4B     |                33B              |          32B          |
//
pub fn init() -> Result<String, Box<dyn std::error::Error>> {
    let protected_seed =
        env::var("PROTECTED_SEED").expect("PROTECTED_SEED env variable is undefined");

    let seed_u8_64: [u8; 64] = hex::decode(&protected_seed)
        .unwrap()
        .try_into()
        .expect("Vector length must be 64");

    let seed = Seed::new(seed_u8_64);

    // Root private key
    let root_xprv = XPrv::new(&seed)?;
    // assert_eq!(root_xprv, XPrv::derive_from_path(&seed, &"m".parse()?)?);

    // Normal child from which we derive deposit addresses
    let root_derivation_path = "m/44'/60'/0'/0/0";
    let child_xprv = XPrv::derive_from_path(&seed, &root_derivation_path.parse()?)?;
    let child_xpub = child_xprv.public_key();

    let public_key = child_xpub.public_key();

    let public_key_bytes = public_key.to_encoded_point(true);

    let version = "00";
    let network_code = "0000000100002105"; // evm, base
    let result = format!(
        "0x{}{}{}{}",
        version,
        network_code,
        hex::encode(public_key_bytes),
        hex::encode(child_xpub.attrs().chain_code),
    );

    Ok(result)
}
