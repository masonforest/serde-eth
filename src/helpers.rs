use crate::constants::WORD_SIZE;
use sha3::{Digest, Keccak256};

pub fn pad_right(bytes: &[u8]) -> Vec<u8> {
    let padding_length = WORD_SIZE - bytes.len() % WORD_SIZE;
    let padding = vec![0u8; padding_length];
    [bytes, &padding[..]].concat()
}

pub fn pad_left(bytes: &[u8]) -> Vec<u8> {
    let padding_length = WORD_SIZE - bytes.len() % WORD_SIZE;
    let padding = vec![0u8; padding_length];
    [&padding[..], bytes].concat()
}

pub fn keccak256(bytes: Vec<u8>) -> Vec<u8> {
    let mut hasher = Keccak256::new();
    hasher.update(bytes);
    hasher.finalize().to_vec()
}
