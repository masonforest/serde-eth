use crate::{
    constants::WORD_SIZE,
    error::{Error, Result},
    helpers::{pad_left, pad_right},
};
use serde::ser::{self, Serialize};

pub enum Value {
    StaticType(Vec<u8>),
    DynamicType(Vec<u8>),
}
pub struct Serializer {
    packed: bool,
    outputs: Vec<Value>,
}

pub fn to_vec<T>(value: &T) -> Result<Vec<u8>>
where
    T: Serialize,
{
    let mut serializer = Serializer {
        packed: false,
        outputs: vec![],
    };
    value.serialize(&mut serializer)?;
    collect_output(serializer)
}

pub fn to_vec_packed<T>(value: &T) -> Result<Vec<u8>>
where
    T: Serialize,
{
    let mut serializer = Serializer {
        packed: true,
        outputs: vec![],
    };
    value.serialize(&mut serializer)?;
    collect_output(serializer)
}

fn collect_output(serializer: Serializer) -> Result<Vec<u8>> {
    let mut output: Vec<u8> = vec![];
    let mut dynamic_types_output: Vec<u8> = vec![];
    for value in &serializer.outputs {
        match value {
            Value::StaticType(bytes) => {
                output.extend(bytes);
            }
            Value::DynamicType(bytes) => {
                let index =
                    (serializer.outputs.len() * WORD_SIZE + dynamic_types_output.len()) as u64;
                output.extend(to_vec(&index)?);

                dynamic_types_output
                    .extend([to_vec(&bytes.len())?, pad_right(&bytes.to_vec())].concat());
            }
        }
    }
    Ok([output, dynamic_types_output].concat())
}

impl<'a> ser::Serializer for &'a mut Serializer {
    type Ok = ();
    type Error = Error;
    type SerializeSeq = Self;
    type SerializeTuple = Self;
    type SerializeTupleStruct = Self;
    type SerializeTupleVariant = Self;
    type SerializeMap = Self;
    type SerializeStruct = Self;
    type SerializeStructVariant = Self;
    fn serialize_bool(self, _v: bool) -> Result<()> {
        Ok(())
    }
    fn serialize_i8(self, v: i8) -> Result<()> {
        self.serialize_i64(i64::from(v))
    }

    fn serialize_i16(self, v: i16) -> Result<()> {
        self.serialize_i64(i64::from(v))
    }
    fn serialize_i32(self, v: i32) -> Result<()> {
        self.serialize_i64(i64::from(v))
    }

    fn serialize_i64(self, v: i64) -> Result<()> {
        let bytes = if self.packed {
            v.to_be_bytes().to_vec()
        } else {
            pad_left(&v.to_be_bytes())
        };
        self.outputs.push(Value::StaticType(bytes));
        Ok(())
    }

    fn serialize_u8(self, v: u8) -> Result<()> {
        self.serialize_u64(u64::from(v))
    }

    fn serialize_u16(self, v: u16) -> Result<()> {
        self.serialize_u64(u64::from(v))
    }

    fn serialize_u32(self, v: u32) -> Result<()> {
        self.serialize_u64(u64::from(v))
    }

    fn serialize_u64(self, v: u64) -> Result<()> {
        let bytes = if self.packed {
            v.to_be_bytes().to_vec()
        } else {
            pad_left(&v.to_be_bytes())
        };
        self.outputs.push(Value::StaticType(bytes));
        Ok(())
    }

    fn serialize_f32(self, v: f32) -> Result<()> {
        self.serialize_f64(f64::from(v))
    }

    fn serialize_f64(self, _v: f64) -> Result<()> {
        Ok(())
    }

    fn serialize_char(self, v: char) -> Result<()> {
        self.serialize_str(&v.to_string())
    }

    fn serialize_str(self, v: &str) -> Result<()> {
        let bytes = v.as_bytes().to_vec();
        self.outputs.push(Value::DynamicType(bytes.clone()));

        Ok(())
    }

    fn serialize_bytes(self, v: &[u8]) -> Result<()> {
        self.outputs.push(Value::StaticType(v.to_vec()));
        Ok(())
    }

    fn serialize_none(self) -> Result<()> {
        self.serialize_unit()
    }

    fn serialize_some<T>(self, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        value.serialize(self)
    }

    fn serialize_unit(self) -> Result<()> {
        Ok(())
    }

    fn serialize_unit_struct(self, _name: &'static str) -> Result<()> {
        self.serialize_unit()
    }

    fn serialize_unit_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        variant: &'static str,
    ) -> Result<()> {
        self.serialize_str(variant)
    }

    fn serialize_newtype_struct<T>(self, _name: &'static str, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        value.serialize(self)
    }

    fn serialize_newtype_variant<T>(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _value: &T,
    ) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        Ok(())
    }

    fn serialize_seq(self, _len: Option<usize>) -> Result<Self::SerializeSeq> {
        Ok(self)
    }

    fn serialize_tuple(self, len: usize) -> Result<Self::SerializeTuple> {
        self.serialize_seq(Some(len))
    }

    fn serialize_tuple_struct(
        self,
        _name: &'static str,
        len: usize,
    ) -> Result<Self::SerializeTupleStruct> {
        self.serialize_seq(Some(len))
    }

    fn serialize_tuple_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeTupleVariant> {
        Ok(self)
    }

    fn serialize_map(self, _len: Option<usize>) -> Result<Self::SerializeMap> {
        Ok(self)
    }

    fn serialize_struct(self, _name: &'static str, len: usize) -> Result<Self::SerializeStruct> {
        self.serialize_map(Some(len))
    }

    fn serialize_struct_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeStructVariant> {
        Ok(self)
    }
}

impl<'a> ser::SerializeSeq for &'a mut Serializer {
    type Ok = ();
    type Error = Error;

    fn serialize_element<T>(&mut self, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        value.serialize(&mut **self)?;
        Ok(())
    }

    fn end(self) -> Result<()> {
        Ok(())
    }
}

impl<'a> ser::SerializeTuple for &'a mut Serializer {
    type Ok = ();
    type Error = Error;

    fn serialize_element<T>(&mut self, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        value.serialize(&mut **self)?;
        Ok(())
    }

    fn end(self) -> Result<()> {
        Ok(())
    }
}

impl<'a> ser::SerializeTupleStruct for &'a mut Serializer {
    type Ok = ();
    type Error = Error;

    fn serialize_field<T>(&mut self, _value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        Ok(())
    }

    fn end(self) -> Result<()> {
        Ok(())
    }
}

impl<'a> ser::SerializeTupleVariant for &'a mut Serializer {
    type Ok = ();
    type Error = Error;

    fn serialize_field<T>(&mut self, _value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        Ok(())
    }

    fn end(self) -> Result<()> {
        Ok(())
    }
}

impl<'a> ser::SerializeMap for &'a mut Serializer {
    type Ok = ();
    type Error = Error;

    fn serialize_key<T>(&mut self, _key: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        Ok(())
    }

    fn serialize_value<T>(&mut self, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        value.serialize(&mut **self)
    }

    fn end(self) -> Result<()> {
        Ok(())
    }
}

impl<'a> ser::SerializeStruct for &'a mut Serializer {
    type Ok = ();
    type Error = Error;

    fn serialize_field<T>(&mut self, _key: &'static str, _value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        Ok(())
    }

    fn end(self) -> Result<()> {
        Ok(())
    }
}

impl<'a> ser::SerializeStructVariant for &'a mut Serializer {
    type Ok = ();
    type Error = Error;

    fn serialize_field<T>(&mut self, _key: &'static str, _value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        Ok(())
    }

    fn end(self) -> Result<()> {
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::{to_vec, to_vec_packed};
    use crate::{types::U256, Bytes32};
    use hex_literal::hex;
    use num_bigint::BigUint;

    #[test]
    fn test_u256() {
        for (input, output) in [
            (
                256,
                hex!("0000000000000000000000000000000000000000000000000000000000000100"),
            ),
            (
                123456_u64,
                hex!("000000000000000000000000000000000000000000000000000000000001e240"),
            ),
        ]
        .iter()
        {
            assert_eq!(to_vec(&U256(BigUint::from(*input))).unwrap(), output);
        }
    }

    #[test]
    fn test_tuple() {
        for (input, output) in [(
            (1u64),
            hex!("0000000000000000000000000000000000000000000000000000000000000001"),
        )]
        .iter()
        {
            assert_eq!(to_vec(&input).unwrap(), output);
        }
    }

    #[test]
    fn test_bytes32() {
        for (input, output) in [(
            Bytes32([0; 32]),
            hex!("0000000000000000000000000000000000000000000000000000000000000000"),
        )]
        .iter()
        {
            assert_eq!(to_vec(&input).unwrap(), output);
        }
    }

    #[test]
    fn test_big_int() {
        for (input, output) in [(
            U256(BigUint::from(10000u64) * BigUint::from(10u64).pow(14)),
            hex!("0000000000000000000000000000000000000000000000000DE0B6B3A7640000"),
        )]
        .iter()
        {
            assert_eq!(to_vec(&input).unwrap(), output);
        }
    }

    #[test]
    fn test_i64() {
        for (input, output) in [(
            1i64,
            hex!("0000000000000000000000000000000000000000000000000000000000000001"),
        )]
        .iter()
        {
            assert_eq!(to_vec(&input).unwrap(), output);
        }
    }

    #[test]
    fn test_i64_packed() {
        for (input, output) in [(1i64, hex!("0000000000000001"))].iter() {
            assert_eq!(to_vec_packed(&input).unwrap(), output);
        }
    }

    #[test]
    fn test_string() {
        for (input, output) in [(
            "gavofyork",
            hex!(
                "
  0000000000000000000000000000000000000000000000000000000000000020
  0000000000000000000000000000000000000000000000000000000000000009
  6761766f66796f726b0000000000000000000000000000000000000000000000
            "
            ),
        )]
        .iter()
        {
            assert_eq!(to_vec(&input).unwrap(), output);
        }
    }
}
