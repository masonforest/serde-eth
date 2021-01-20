use num_bigint::BigUint;
use serde::Serialize;

const ADDRESS_SIZE: usize = 20;
#[derive(Debug, Clone)]
pub struct Address(pub [u8; ADDRESS_SIZE]);
#[derive(Debug, Clone)]
pub struct Bytes32(pub [u8; 32]);
#[derive(Debug, Clone, PartialEq)]
pub struct U256(pub BigUint);

impl From<Vec<u8>> for U256 {
    fn from(value: Vec<u8>) -> Self {
        U256(BigUint::from_bytes_be(&value))
    }
}

impl Default for U256 {
    fn default() -> Self {
        0.into()
    }
}

impl From<u64> for U256 {
    fn from(value: u64) -> Self {
        U256(BigUint::from(value))
    }
}

impl From<i32> for U256 {
    fn from(value: i32) -> Self {
        U256(BigUint::from(value as u64))
    }
}

impl From<usize> for U256 {
    fn from(value: usize) -> Self {
        U256(BigUint::from(value))
    }
}

impl Serialize for U256 {
    fn serialize<S>(&self, serializer: S) -> core::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut bytes = self.0.to_bytes_le();
        bytes.resize(32, 0);
        bytes.reverse();

        serializer.serialize_bytes(&bytes)
    }
}

impl Serialize for Address {
    fn serialize<S>(&self, serializer: S) -> core::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_bytes(&self.0.to_vec())
    }
}

impl Serialize for Bytes32 {
    fn serialize<S>(&self, serializer: S) -> core::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_bytes(&self.0.to_vec())
    }
}
