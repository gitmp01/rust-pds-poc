// Returns a deterministic derivation path from a 32 byte array
//
//   [1, 23, 241, ...] -> '1021029/543985/34985/...'
//
// This is meant to be appended to the standard derivation paths
// pertinent to each blockchain in use.
//
// Example:
//
//   "m/44'/60'/0'/0/0/" (ethereum) + "1021029/543985/34985/..."
//
pub fn get_derivation_path_from_hash(hash: &[u8]) -> Result<String, String> {
    // Validate input length (32 bytes)
    if hash.len() != 32 {
        return Err(format!("Expected 32 bytes, got {}", hash.len()));
    }

    let mut path = vec![];

    // Divide into 8 chunks of 4 bytes each
    let non_hardened_limit = 2u32.pow(31) - 1;
    for i in (0..32).step_by(4) {
        let chunk = &hash[i..i + 4];
        // Convert 4-byte chunk to u32 (big-endian)
        let value = u32::from_be_bytes([chunk[0], chunk[1], chunk[2], chunk[3]]);
        let normalized_value = value % non_hardened_limit;
        path.push(normalized_value.to_string());
    }

    Ok(path.join("/"))
}
