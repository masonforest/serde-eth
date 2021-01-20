use sha3::{Digest, Keccak256};

pub fn pad_right(mut value: Vec<u8>, byte_size: usize) -> Vec<u8> {
    let padding = byte_size - value.len() % byte_size;
    value.resize(value.len() + padding, 0);
    value
}

pub fn _pad_left(value: Vec<u8>, byte_size: usize) -> Vec<u8> {
    let padding = byte_size - value.len() % byte_size;
    let mut new_vec = vec![0; padding];

    new_vec.splice(new_vec.len()..new_vec.len(), value.iter().cloned());
    new_vec
}

pub fn keccak256(bytes: Vec<u8>) -> Vec<u8> {
    let mut hasher = Keccak256::new();
    hasher.update(bytes);
    hasher.finalize().to_vec()
}
