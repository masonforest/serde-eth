pub mod eip_712;

mod constants;
mod error;
mod helpers;
mod ser;
mod types;

pub use crate::{
    error::{Error, Result},
    ser::{to_vec, to_vec_packed, Serializer},
    types::*,
};
