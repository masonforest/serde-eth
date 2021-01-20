pub mod eip_712;

mod error;
mod helpers;
mod ser;
mod types;

pub use crate::{
    error::{Error, Result},
    ser::{to_vec_packed, Serializer},
    types::*,
};
