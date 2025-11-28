use alloy::primitives::Bytes;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct Input {
    pub message: String,
}

#[derive(Serialize)]
pub struct Output {
    pub result: String,
}

#[derive(Debug)]
pub struct Header {
    pub version: Bytes,
    pub network_code: Bytes,
    pub protocol: u32,
    pub chain_id: u32,
    pub command: u32,
}
