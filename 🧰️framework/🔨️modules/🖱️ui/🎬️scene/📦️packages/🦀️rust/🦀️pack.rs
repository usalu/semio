//! 🧵️ Self-contained binary `serde` codec for `SceneDoc` payloads. Every framework "pack" encoder
//! already in the repo (`🎒️pack::encode_record_body`/`encode_json_value`) is schema/`DslValue`-typed
//! and lives in the os-product layer — depending on any of them would pull `dsl`/os-kernel into this
//! wasm-safe, `ui_contract`-and-`serde`-only crate, inverting the framework → os-product layering the
//! same way `semio_framework_actor::pack`'s own header explicitly refuses to. This module follows
//! that exact precedent: a hand-rolled, self-contained binary format, `serde`-generic instead of
//! per-type hand-written `pack_encode`/`pack_decode` pairs (those 15 structs have ~140 fields between
//! them; one generic codec beats fifteen hand-written ones).
//!
//! Deliberately NOT a general-purpose serde format: no `serialize_map`/enum-variant support, because
//! no [`crate::SceneDoc`] struct (after the `_json` opaque-string treatment described in
//! `🦀️scenes.rs`'s header) needs them. Both directions return [`PackError`] rather than panicking on
//! anything unsupported or truncated.
//!
//! 🚫️async: E6 sync payload encoding — no `async fn` anywhere in this module.

use serde::de::{self, DeserializeSeed, MapAccess, SeqAccess, Visitor};
use serde::ser::{self, SerializeSeq, SerializeStruct, SerializeStructVariant, SerializeTuple, SerializeTupleStruct, SerializeTupleVariant};
use serde::{Deserialize, Serialize};
use std::fmt;

//#region 🔖️PackError
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PackError {
    Truncated,
    InvalidTag(u8),
    InvalidUtf8,
    Unsupported(&'static str),
    Message(String),
}

impl fmt::Display for PackError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Truncated => write!(f, "pack: truncated input"),
            Self::InvalidTag(tag) => write!(f, "pack: invalid tag {tag}"),
            Self::InvalidUtf8 => write!(f, "pack: invalid utf8"),
            Self::Unsupported(what) => write!(f, "pack: unsupported for this codec: {what}"),
            Self::Message(message) => write!(f, "pack: {message}"),
        }
    }
}

impl std::error::Error for PackError {}
impl de::Error for PackError {
    fn custom<T: fmt::Display>(message: T) -> Self {
        Self::Message(message.to_string())
    }
}
impl ser::Error for PackError {
    fn custom<T: fmt::Display>(message: T) -> Self {
        Self::Message(message.to_string())
    }
}
//#endregion 🔖️PackError

//#region 🔖️Primitives
const TAG_UNIT: u8 = 0;
const TAG_FALSE: u8 = 1;
const TAG_TRUE: u8 = 2;
const TAG_U64: u8 = 3;
const TAG_I64: u8 = 4;
const TAG_F64: u8 = 5;
const TAG_STR: u8 = 6;
const TAG_BYTES: u8 = 7;
const TAG_NONE: u8 = 8;
const TAG_SOME: u8 = 9;
const TAG_SEQ: u8 = 10;
const TAG_CHAR: u8 = 11;
const TAG_VARIANT: u8 = 12;
const TAG_MAP: u8 = 13;

fn write_varint(out: &mut Vec<u8>, value: u64) {
    let mut remaining = value;
    loop {
        let byte = (remaining & 0x7F) as u8;
        remaining >>= 7;
        if remaining == 0 {
            out.push(byte);
            break;
        }
        out.push(byte | 0x80);
    }
}

fn read_varint(bytes: &[u8], pos: &mut usize) -> Result<u64, PackError> {
    let mut result: u64 = 0;
    for shift in (0..70).step_by(7) {
        let byte = *bytes.get(*pos).ok_or(PackError::Truncated)?;
        *pos += 1;
        result |= ((byte & 0x7F) as u64) << shift;
        if byte & 0x80 == 0 {
            return Ok(result);
        }
    }
    Err(PackError::Truncated)
}

fn zigzag_encode(value: i64) -> u64 {
    ((value << 1) ^ (value >> 63)) as u64
}
fn zigzag_decode(value: u64) -> i64 {
    ((value >> 1) as i64) ^ -((value & 1) as i64)
}

fn write_string(out: &mut Vec<u8>, value: &str) {
    out.push(TAG_STR);
    write_varint(out, value.len() as u64);
    out.extend_from_slice(value.as_bytes());
}
//#endregion 🔖️Primitives

//#region 🔖️Serializer
pub struct PackSerializer<'a> {
    out: &'a mut Vec<u8>,
}

pub fn to_bytes<T: Serialize + ?Sized>(value: &T) -> Result<Vec<u8>, PackError> {
    let mut out = Vec::new();
    value.serialize(&mut PackSerializer { out: &mut out })?;
    Ok(out)
}

impl<'a> ser::Serializer for &mut PackSerializer<'a> {
    type Ok = ();
    type Error = PackError;
    type SerializeSeq = Self;
    type SerializeTuple = Self;
    type SerializeTupleStruct = Self;
    type SerializeTupleVariant = Self;
    type SerializeMap = Self;
    type SerializeStruct = Self;
    type SerializeStructVariant = Self;

    fn serialize_bool(self, v: bool) -> Result<(), PackError> {
        self.out.push(if v { TAG_TRUE } else { TAG_FALSE });
        Ok(())
    }
    fn serialize_i8(self, v: i8) -> Result<(), PackError> {
        self.serialize_i64(v as i64)
    }
    fn serialize_i16(self, v: i16) -> Result<(), PackError> {
        self.serialize_i64(v as i64)
    }
    fn serialize_i32(self, v: i32) -> Result<(), PackError> {
        self.serialize_i64(v as i64)
    }
    fn serialize_i64(self, v: i64) -> Result<(), PackError> {
        self.out.push(TAG_I64);
        write_varint(self.out, zigzag_encode(v));
        Ok(())
    }
    fn serialize_u8(self, v: u8) -> Result<(), PackError> {
        self.serialize_u64(v as u64)
    }
    fn serialize_u16(self, v: u16) -> Result<(), PackError> {
        self.serialize_u64(v as u64)
    }
    fn serialize_u32(self, v: u32) -> Result<(), PackError> {
        self.serialize_u64(v as u64)
    }
    fn serialize_u64(self, v: u64) -> Result<(), PackError> {
        self.out.push(TAG_U64);
        write_varint(self.out, v);
        Ok(())
    }
    fn serialize_f32(self, v: f32) -> Result<(), PackError> {
        self.serialize_f64(v as f64)
    }
    fn serialize_f64(self, v: f64) -> Result<(), PackError> {
        self.out.push(TAG_F64);
        self.out.extend_from_slice(&v.to_le_bytes());
        Ok(())
    }
    fn serialize_char(self, v: char) -> Result<(), PackError> {
        self.out.push(TAG_CHAR);
        write_varint(self.out, v as u64);
        Ok(())
    }
    fn serialize_str(self, v: &str) -> Result<(), PackError> {
        write_string(self.out, v);
        Ok(())
    }
    fn serialize_bytes(self, v: &[u8]) -> Result<(), PackError> {
        self.out.push(TAG_BYTES);
        write_varint(self.out, v.len() as u64);
        self.out.extend_from_slice(v);
        Ok(())
    }
    fn serialize_none(self) -> Result<(), PackError> {
        self.out.push(TAG_NONE);
        Ok(())
    }
    fn serialize_some<T: Serialize + ?Sized>(self, value: &T) -> Result<(), PackError> {
        self.out.push(TAG_SOME);
        value.serialize(self)
    }
    fn serialize_unit(self) -> Result<(), PackError> {
        self.out.push(TAG_UNIT);
        Ok(())
    }
    fn serialize_unit_struct(self, _name: &'static str) -> Result<(), PackError> {
        self.serialize_unit()
    }
    fn serialize_unit_variant(self, _name: &'static str, _variant_index: u32, variant: &'static str) -> Result<(), PackError> {
        self.out.push(TAG_VARIANT);
        write_string(self.out, variant);
        self.out.push(TAG_UNIT);
        Ok(())
    }
    fn serialize_newtype_struct<T: Serialize + ?Sized>(self, _name: &'static str, value: &T) -> Result<(), PackError> {
        value.serialize(self)
    }
    fn serialize_newtype_variant<T: Serialize + ?Sized>(self, _name: &'static str, _variant_index: u32, variant: &'static str, value: &T) -> Result<(), PackError> {
        self.out.push(TAG_VARIANT);
        write_string(self.out, variant);
        value.serialize(self)
    }
    fn serialize_seq(self, len: Option<usize>) -> Result<Self, PackError> {
        self.out.push(TAG_SEQ);
        write_varint(self.out, len.ok_or(PackError::Unsupported("seq with unknown length"))? as u64);
        Ok(self)
    }
    fn serialize_tuple(self, len: usize) -> Result<Self, PackError> {
        self.out.push(TAG_SEQ);
        write_varint(self.out, len as u64);
        Ok(self)
    }
    fn serialize_tuple_struct(self, _name: &'static str, len: usize) -> Result<Self, PackError> {
        self.out.push(TAG_SEQ);
        write_varint(self.out, len as u64);
        Ok(self)
    }
    fn serialize_tuple_variant(self, _name: &'static str, _variant_index: u32, variant: &'static str, len: usize) -> Result<Self, PackError> {
        self.out.push(TAG_VARIANT);
        write_string(self.out, variant);
        self.out.push(TAG_SEQ);
        write_varint(self.out, len as u64);
        Ok(self)
    }
    fn serialize_map(self, _len: Option<usize>) -> Result<Self, PackError> {
        Err(PackError::Unsupported("map"))
    }
    fn serialize_struct(self, _name: &'static str, len: usize) -> Result<Self, PackError> {
        self.out.push(TAG_MAP);
        write_varint(self.out, len as u64);
        Ok(self)
    }
    fn serialize_struct_variant(self, _name: &'static str, _variant_index: u32, variant: &'static str, len: usize) -> Result<Self, PackError> {
        self.out.push(TAG_VARIANT);
        write_string(self.out, variant);
        self.out.push(TAG_MAP);
        write_varint(self.out, len as u64);
        Ok(self)
    }
}

impl<'a> SerializeSeq for &mut PackSerializer<'a> {
    type Ok = ();
    type Error = PackError;
    fn serialize_element<T: Serialize + ?Sized>(&mut self, value: &T) -> Result<(), PackError> {
        value.serialize(&mut **self)
    }
    fn end(self) -> Result<(), PackError> {
        Ok(())
    }
}
impl<'a> SerializeTuple for &mut PackSerializer<'a> {
    type Ok = ();
    type Error = PackError;
    fn serialize_element<T: Serialize + ?Sized>(&mut self, value: &T) -> Result<(), PackError> {
        value.serialize(&mut **self)
    }
    fn end(self) -> Result<(), PackError> {
        Ok(())
    }
}
impl<'a> SerializeTupleStruct for &mut PackSerializer<'a> {
    type Ok = ();
    type Error = PackError;
    fn serialize_field<T: Serialize + ?Sized>(&mut self, value: &T) -> Result<(), PackError> {
        value.serialize(&mut **self)
    }
    fn end(self) -> Result<(), PackError> {
        Ok(())
    }
}
impl<'a> SerializeTupleVariant for &mut PackSerializer<'a> {
    type Ok = ();
    type Error = PackError;
    fn serialize_field<T: Serialize + ?Sized>(&mut self, value: &T) -> Result<(), PackError> {
        value.serialize(&mut **self)
    }
    fn end(self) -> Result<(), PackError> {
        Ok(())
    }
}
impl<'a> ser::SerializeMap for &mut PackSerializer<'a> {
    type Ok = ();
    type Error = PackError;
    fn serialize_key<T: Serialize + ?Sized>(&mut self, _key: &T) -> Result<(), PackError> {
        Err(PackError::Unsupported("map"))
    }
    fn serialize_value<T: Serialize + ?Sized>(&mut self, _value: &T) -> Result<(), PackError> {
        Err(PackError::Unsupported("map"))
    }
    fn end(self) -> Result<(), PackError> {
        Err(PackError::Unsupported("map"))
    }
}
impl<'a> SerializeStruct for &mut PackSerializer<'a> {
    type Ok = ();
    type Error = PackError;
    fn serialize_field<T: Serialize + ?Sized>(&mut self, key: &'static str, value: &T) -> Result<(), PackError> {
        write_string(self.out, key);
        value.serialize(&mut **self)
    }
    fn end(self) -> Result<(), PackError> {
        Ok(())
    }
}
impl<'a> SerializeStructVariant for &mut PackSerializer<'a> {
    type Ok = ();
    type Error = PackError;
    fn serialize_field<T: Serialize + ?Sized>(&mut self, key: &'static str, value: &T) -> Result<(), PackError> {
        write_string(self.out, key);
        value.serialize(&mut **self)
    }
    fn end(self) -> Result<(), PackError> {
        Ok(())
    }
}
//#endregion 🔖️Serializer

//#region 🔖️Deserializer
pub struct PackDeserializer<'de> {
    bytes: &'de [u8],
    pos: usize,
}

pub fn from_bytes<'de, T: Deserialize<'de>>(bytes: &'de [u8]) -> Result<T, PackError> {
    let mut deserializer = PackDeserializer { bytes, pos: 0 };
    T::deserialize(&mut deserializer)
}

impl<'de> PackDeserializer<'de> {
    fn read_tag(&mut self) -> Result<u8, PackError> {
        let tag = *self.bytes.get(self.pos).ok_or(PackError::Truncated)?;
        self.pos += 1;
        Ok(tag)
    }
    fn read_varint(&mut self) -> Result<u64, PackError> {
        read_varint(self.bytes, &mut self.pos)
    }
    fn read_bytes_exact(&mut self, len: usize) -> Result<&'de [u8], PackError> {
        let end = self.pos + len;
        let slice = self.bytes.get(self.pos..end).ok_or(PackError::Truncated)?;
        self.pos = end;
        Ok(slice)
    }
}

struct PackSeqAccess<'a, 'de> {
    de: &'a mut PackDeserializer<'de>,
    remaining: u64,
}

struct PackMapAccess<'a, 'de> {
    de: &'a mut PackDeserializer<'de>,
    remaining: u64,
}
impl<'a, 'de> MapAccess<'de> for PackMapAccess<'a, 'de> {
    type Error = PackError;
    fn next_key_seed<K: DeserializeSeed<'de>>(&mut self, seed: K) -> Result<Option<K::Value>, PackError> {
        if self.remaining == 0 {
            return Ok(None);
        }
        seed.deserialize(&mut *self.de).map(Some)
    }
    fn next_value_seed<V: DeserializeSeed<'de>>(&mut self, seed: V) -> Result<V::Value, PackError> {
        self.remaining -= 1;
        seed.deserialize(&mut *self.de)
    }
    fn size_hint(&self) -> Option<usize> {
        Some(self.remaining as usize)
    }
}
impl<'a, 'de> SeqAccess<'de> for PackSeqAccess<'a, 'de> {
    type Error = PackError;
    fn next_element_seed<S: DeserializeSeed<'de>>(&mut self, seed: S) -> Result<Option<S::Value>, PackError> {
        if self.remaining == 0 {
            return Ok(None);
        }
        self.remaining -= 1;
        seed.deserialize(&mut *self.de).map(Some)
    }
    fn size_hint(&self) -> Option<usize> {
        Some(self.remaining as usize)
    }
}

macro_rules! forward_to_deserialize_num {
    ($($method:ident => $visit:ident : $ty:ty),* $(,)?) => {
        $(
            fn $method<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value, PackError> {
                let tag = self.read_tag()?;
                match tag {
                    TAG_U64 => { let v = self.read_varint()?; visitor.$visit(v as $ty) }
                    TAG_I64 => { let raw = self.read_varint()?; visitor.$visit(zigzag_decode(raw) as $ty) }
                    other => Err(PackError::InvalidTag(other)),
                }
            }
        )*
    };
}

impl<'de> serde::Deserializer<'de> for &mut PackDeserializer<'de> {
    type Error = PackError;

    fn deserialize_any<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value, PackError> {
        let tag = self.read_tag()?;
        match tag {
            TAG_UNIT => visitor.visit_unit(),
            TAG_FALSE => visitor.visit_bool(false),
            TAG_TRUE => visitor.visit_bool(true),
            TAG_U64 => visitor.visit_u64(self.read_varint()?),
            TAG_I64 => {
                let raw = self.read_varint()?;
                visitor.visit_i64(zigzag_decode(raw))
            }
            TAG_F64 => {
                let bytes = self.read_bytes_exact(8)?;
                visitor.visit_f64(f64::from_le_bytes(bytes.try_into().expect("8 bytes")))
            }
            TAG_STR => {
                let len = self.read_varint()? as usize;
                let raw = self.read_bytes_exact(len)?;
                let s = std::str::from_utf8(raw).map_err(|_| PackError::InvalidUtf8)?;
                visitor.visit_str(s)
            }
            TAG_BYTES => {
                let len = self.read_varint()? as usize;
                let raw = self.read_bytes_exact(len)?;
                visitor.visit_bytes(raw)
            }
            TAG_NONE => visitor.visit_none(),
            TAG_SOME => visitor.visit_some(self),
            TAG_CHAR => {
                let code = self.read_varint()? as u32;
                let c = char::from_u32(code).ok_or(PackError::InvalidTag(TAG_CHAR))?;
                visitor.visit_char(c)
            }
            TAG_SEQ => {
                let len = self.read_varint()?;
                visitor.visit_seq(PackSeqAccess { de: self, remaining: len })
            }
            TAG_MAP => {
                let len = self.read_varint()?;
                visitor.visit_map(PackMapAccess { de: self, remaining: len })
            }
            other => Err(PackError::InvalidTag(other)),
        }
    }

    fn deserialize_option<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value, PackError> {
        let tag = self.read_tag()?;
        match tag {
            TAG_NONE => visitor.visit_none(),
            TAG_SOME => visitor.visit_some(self),
            other => Err(PackError::InvalidTag(other)),
        }
    }

    fn deserialize_bool<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value, PackError> {
        let tag = self.read_tag()?;
        match tag {
            TAG_FALSE => visitor.visit_bool(false),
            TAG_TRUE => visitor.visit_bool(true),
            other => Err(PackError::InvalidTag(other)),
        }
    }

    fn deserialize_str<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value, PackError> {
        let tag = self.read_tag()?;
        if tag != TAG_STR {
            return Err(PackError::InvalidTag(tag));
        }
        let len = self.read_varint()? as usize;
        let raw = self.read_bytes_exact(len)?;
        let s = std::str::from_utf8(raw).map_err(|_| PackError::InvalidUtf8)?;
        visitor.visit_borrowed_str(s)
    }
    fn deserialize_string<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value, PackError> {
        self.deserialize_str(visitor)
    }

    fn deserialize_bytes<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value, PackError> {
        let tag = self.read_tag()?;
        if tag != TAG_BYTES {
            return Err(PackError::InvalidTag(tag));
        }
        let len = self.read_varint()? as usize;
        let raw = self.read_bytes_exact(len)?;
        visitor.visit_borrowed_bytes(raw)
    }
    fn deserialize_byte_buf<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value, PackError> {
        self.deserialize_bytes(visitor)
    }

    fn deserialize_f32<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value, PackError> {
        let tag = self.read_tag()?;
        if tag != TAG_F64 {
            return Err(PackError::InvalidTag(tag));
        }
        let bytes = self.read_bytes_exact(8)?;
        visitor.visit_f32(f64::from_le_bytes(bytes.try_into().expect("8 bytes")) as f32)
    }
    fn deserialize_f64<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value, PackError> {
        let tag = self.read_tag()?;
        if tag != TAG_F64 {
            return Err(PackError::InvalidTag(tag));
        }
        let bytes = self.read_bytes_exact(8)?;
        visitor.visit_f64(f64::from_le_bytes(bytes.try_into().expect("8 bytes")))
    }

    fn deserialize_char<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value, PackError> {
        let tag = self.read_tag()?;
        if tag != TAG_CHAR {
            return Err(PackError::InvalidTag(tag));
        }
        let code = self.read_varint()? as u32;
        let c = char::from_u32(code).ok_or(PackError::InvalidTag(TAG_CHAR))?;
        visitor.visit_char(c)
    }

    fn deserialize_unit<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value, PackError> {
        let tag = self.read_tag()?;
        if tag != TAG_UNIT {
            return Err(PackError::InvalidTag(tag));
        }
        visitor.visit_unit()
    }
    fn deserialize_unit_struct<V: Visitor<'de>>(self, _name: &'static str, visitor: V) -> Result<V::Value, PackError> {
        self.deserialize_unit(visitor)
    }
    fn deserialize_newtype_struct<V: Visitor<'de>>(self, _name: &'static str, visitor: V) -> Result<V::Value, PackError> {
        visitor.visit_newtype_struct(self)
    }

    fn deserialize_seq<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value, PackError> {
        let tag = self.read_tag()?;
        if tag != TAG_SEQ {
            return Err(PackError::InvalidTag(tag));
        }
        let len = self.read_varint()?;
        visitor.visit_seq(PackSeqAccess { de: self, remaining: len })
    }
    fn deserialize_tuple<V: Visitor<'de>>(self, _len: usize, visitor: V) -> Result<V::Value, PackError> {
        self.deserialize_seq(visitor)
    }
    fn deserialize_tuple_struct<V: Visitor<'de>>(self, _name: &'static str, _len: usize, visitor: V) -> Result<V::Value, PackError> {
        self.deserialize_seq(visitor)
    }
    fn deserialize_struct<V: Visitor<'de>>(self, _name: &'static str, _fields: &'static [&'static str], visitor: V) -> Result<V::Value, PackError> {
        let tag = self.read_tag()?;
        if tag != TAG_MAP {
            return Err(PackError::InvalidTag(tag));
        }
        let len = self.read_varint()?;
        visitor.visit_map(PackMapAccess { de: self, remaining: len })
    }

    fn deserialize_map<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value, PackError> {
        let tag = self.read_tag()?;
        if tag != TAG_MAP {
            return Err(PackError::InvalidTag(tag));
        }
        let len = self.read_varint()?;
        visitor.visit_map(PackMapAccess { de: self, remaining: len })
    }
    fn deserialize_enum<V: Visitor<'de>>(self, _name: &'static str, _variants: &'static [&'static str], visitor: V) -> Result<V::Value, PackError> {
        let tag = self.read_tag()?;
        if tag != TAG_VARIANT {
            return Err(PackError::InvalidTag(tag));
        }
        visitor.visit_enum(PackEnumAccess { de: self })
    }
    fn deserialize_identifier<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value, PackError> {
        self.deserialize_str(visitor)
    }
    fn deserialize_ignored_any<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value, PackError> {
        self.deserialize_any(visitor)
    }

    forward_to_deserialize_num! {
        deserialize_i8 => visit_i8: i8,
        deserialize_i16 => visit_i16: i16,
        deserialize_i32 => visit_i32: i32,
        deserialize_i64 => visit_i64: i64,
        deserialize_u8 => visit_u8: u8,
        deserialize_u16 => visit_u16: u16,
        deserialize_u32 => visit_u32: u32,
        deserialize_u64 => visit_u64: u64,
    }
}

struct PackEnumAccess<'a, 'de> {
    de: &'a mut PackDeserializer<'de>,
}
impl<'a, 'de> de::EnumAccess<'de> for PackEnumAccess<'a, 'de> {
    type Error = PackError;
    type Variant = PackVariantAccess<'a, 'de>;
    fn variant_seed<S: DeserializeSeed<'de>>(self, seed: S) -> Result<(S::Value, Self::Variant), PackError> {
        let variant = String::deserialize(&mut *self.de)?;
        let value = seed.deserialize(variant.into_deserializer())?;
        Ok((value, PackVariantAccess { de: self.de }))
    }
}
use serde::de::IntoDeserializer;

struct PackVariantAccess<'a, 'de> {
    de: &'a mut PackDeserializer<'de>,
}
impl<'a, 'de> de::VariantAccess<'de> for PackVariantAccess<'a, 'de> {
    type Error = PackError;
    fn unit_variant(self) -> Result<(), PackError> {
        <()>::deserialize(self.de)
    }
    fn newtype_variant_seed<S: DeserializeSeed<'de>>(self, seed: S) -> Result<S::Value, PackError> {
        seed.deserialize(self.de)
    }
    fn tuple_variant<V: Visitor<'de>>(self, _len: usize, visitor: V) -> Result<V::Value, PackError> {
        serde::Deserializer::deserialize_seq(self.de, visitor)
    }
    fn struct_variant<V: Visitor<'de>>(self, _fields: &'static [&'static str], visitor: V) -> Result<V::Value, PackError> {
        serde::Deserializer::deserialize_map(self.de, visitor)
    }
}
//#endregion 🔖️Deserializer

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    struct Nested {
        label: String,
        weight: f64,
    }

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    struct Sample {
        name: String,
        count: u32,
        active: bool,
        tag: Option<String>,
        items: Vec<Nested>,
    }

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    struct Sparse {
        first_value: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        middle_value: Option<String>,
        last_value: String,
    }

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    enum Choice {
        Unit,
        Newtype(String),
        Record { value: String },
    }

    #[test]
    fn round_trips_struct_with_option_and_seq() {
        let value = Sample { name: "hello".into(), count: 42, active: true, tag: Some("v1".into()), items: vec![Nested { label: "a".into(), weight: 1.5 }, Nested { label: "b".into(), weight: -2.25 }] };
        let bytes = to_bytes(&value).expect("encode");
        let back: Sample = from_bytes(&bytes).expect("decode");
        assert_eq!(value, back);
    }

    #[test]
    fn round_trips_none_option() {
        let value = Sample { name: String::new(), count: 0, active: false, tag: None, items: vec![] };
        let bytes = to_bytes(&value).expect("encode");
        let back: Sample = from_bytes(&bytes).expect("decode");
        assert_eq!(value, back);
    }

    #[test]
    fn self_describing_struct_preserves_a_field_after_a_skipped_middle_field() {
        let value = Sparse { first_value: "first".into(), middle_value: None, last_value: "last".into() };
        let bytes = to_bytes(&value).expect("encode");
        let back: Sparse = from_bytes(&bytes).expect("decode");
        assert_eq!(value, back);
        assert!(bytes.windows("lastValue".len()).any(|window| window == b"lastValue"));
    }

    #[test]
    fn self_describing_enum_round_trips_every_payload_shape() {
        for value in [Choice::Unit, Choice::Newtype("newtype".into()), Choice::Record { value: "record".into() }] {
            let bytes = to_bytes(&value).expect("encode");
            let back: Choice = from_bytes(&bytes).expect("decode");
            assert_eq!(value, back);
        }
    }

    #[test]
    fn truncated_input_errs_not_panics() {
        let result: Result<Sample, PackError> = from_bytes(&[TAG_SEQ]);
        assert!(result.is_err());
    }

    //#region 🎬️RetainedSceneOracle
    #[test]
    fn owned_scene_neutral_vectors_match_native_serde_packet() {
        #[derive(Serialize)]
        enum FixtureVariant {
            Idle,
            Scale(u64),
        }
        struct Bytes<'a>(&'a [u8]);
        impl Serialize for Bytes<'_> {
            fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
                serializer.serialize_bytes(self.0)
            }
        }
        let fixture: serde_json::Value = serde_json::from_str(include_str!("../../../🧬️contract/🧵️retained/🧪️fixtures/🔣️owned-scene.json")).unwrap();
        let cases = fixture["cases"].as_array().unwrap();
        assert_eq!(cases.len(), 19);
        for case in cases {
            let name = case["name"].as_str().unwrap();
            let expected: Vec<u8> = case["hex"].as_str().unwrap().as_bytes().chunks_exact(2).map(|digits| u8::from_str_radix(std::str::from_utf8(digits).unwrap(), 16).unwrap()).collect();
            let actual = match name {
                "unit" => to_bytes(&()).unwrap(),
                "false" => to_bytes(&false).unwrap(),
                "true" => to_bytes(&true).unwrap(),
                "unsigned" => to_bytes(&300u64).unwrap(),
                "negative" => to_bytes(&-3i64).unwrap(),
                "double" => to_bytes(&1.5f64).unwrap(),
                "unicode" | "bom-preserved" => to_bytes(case["value"].as_str().unwrap()).unwrap(),
                "bytes" => to_bytes(&Bytes(&[0, 128, 255])).unwrap(),
                "none" => to_bytes(&Option::<bool>::None).unwrap(),
                "some" => to_bytes(&Some(true)).unwrap(),
                "char" => to_bytes(&'🧹').unwrap(),
                "unit-variant" => to_bytes(&FixtureVariant::Idle).unwrap(),
                "data-variant" => to_bytes(&FixtureVariant::Scale(2)).unwrap(),
                "sequence" | "nested-map" | "prototype-key" | "empty-containers" | "fnv-collision-exact-keys" => to_bytes(&case["value"]).unwrap(),
                _ => panic!("Unmatched neutral scene case: {name}"),
            };
            assert_eq!(actual, expected, "{name}");
        }
    }
    //#endregion 🎬️RetainedSceneOracle
}
