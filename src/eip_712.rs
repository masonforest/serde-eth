use crate::helpers::keccak256;
use serde::ser::Serialize;
// https://eips.ethereum.org/EIPS/eip-712#specification
const HEADER_BYTES: [u8; 2] = [25, 1];

pub fn hash<D: Serialize + Typed, V: Serialize + Typed>(domain_separator: D, value: V) -> Vec<u8> {
    [HEADER_BYTES.to_vec(), domain_separator.hash(), value.hash()].concat()
}

pub trait Typed {
    fn encode_type() -> String;
    fn hash(self) -> Vec<u8>;

    fn type_hash() -> Vec<u8>
    where
        Self: std::marker::Sized,
    {
        keccak256(Self::encode_type().as_bytes().to_vec())
    }
}
