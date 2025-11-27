use bip32::secp256k1::ecdsa::{
    signature::{Signer, Verifier},
    Signature,
};
use bip32::{Prefix, Seed, XPrv};
use k256::ecdsa::{SigningKey, VerifyingKey};
use k256::elliptic_curve::sec1::ToEncodedPoint;
use sha3::{Digest, Keccak256};
use std::env;
use std::io::{self, Read, Write};

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
