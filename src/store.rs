/*
use cdk::runtime;


pub const KEY_TOKEN: &str = "_token";
pub const KEY_SUPPORT_REQUIRED_PCT: &str = "_supportRequiredPct";
pub const KEY_MIN_ACCEPT_QUORUM_PCT: &str = "_minAcceptQuorumPct";
pub const KEY_VOTE_TIME: &str = "_voteTime";
pub const KEY_VOTE_LENGTH: &str = "_vote_length";

pub fn set_param(key: &str, value: &[u8]) {
    runtime::make_dependencies()
        .storage
        .set(key.as_bytes(), value)
}

pub fn get_param(key: &str) -> std::vec::Vec<u8> {
    let val = runtime::make_dependencies()
        .storage
        .get(key.as_bytes())
        .unwrap();
    //String::from_utf8(val).unwrap()
    return val
}

pub fn convert_nibbles_to_u64(values: &[u8]) -> u64 {
    let mut out = 0;
    for &i in values {
        out = out << 4 | i as u64;
    }
    out
}

pub fn convert_nibbles_to_u128(values: &[u8]) -> u128 {
    let mut out = 0;
    for &i in values {
        out = out << 4 | i as u128;
    }
    out
}

pub fn get_block_number() -> u64 {
    return 2
}

*/