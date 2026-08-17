//! @emoji 🔀️ Serde serializer/deserializer over `DslValue` trees (no JSON text).

use super::DslValue;
use serde::de::{self, DeserializeSeed, EnumAccess, MapAccess, SeqAccess, VariantAccess, Visitor};
use serde::ser::{self, Serialize};
use std::fmt;

#[derive(Debug)]
pub struct SerdeError(String);

impl fmt::Display for SerdeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl std::error::Error for SerdeError {}

impl ser::Error for SerdeError {
    fn custom<T: fmt::Display>(msg: T) -> Self {
        SerdeError(msg.to_string())
    }
}

impl de::Error for SerdeError {
    fn custom<T: fmt::Display>(msg: T) -> Self {
        SerdeError(msg.to_string())
    }
}

pub struct ValueSerializer;

impl ser::Serializer for ValueSerializer {
    type Ok = DslValue;
    type Error = SerdeError;
    type SerializeSeq = SeqSerializer;
    type SerializeTuple = SeqSerializer;
    type SerializeTupleStruct = SeqSerializer;
    type SerializeTupleVariant = TupleVariantSerializer;
    type SerializeMap = MapSerializer;
    type SerializeStruct = MapSerializer;
    type SerializeStructVariant = StructVariantSerializer;

    fn serialize_bool(self, v: bool) -> Result<Self::Ok, Self::Error> {
        Ok(DslValue::Bool(v))
    }

    fn serialize_i8(self, v: i8) -> Result<Self::Ok, Self::Error> {
        Ok(DslValue::Number(f64::from(v)))
    }

    fn serialize_i16(self, v: i16) -> Result<Self::Ok, Self::Error> {
        Ok(DslValue::Number(f64::from(v)))
    }

    fn serialize_i32(self, v: i32) -> Result<Self::Ok, Self::Error> {
        Ok(DslValue::Number(f64::from(v)))
    }

    fn serialize_i64(self, v: i64) -> Result<Self::Ok, Self::Error> {
        Ok(DslValue::Number(v as f64))
    }

    fn serialize_u8(self, v: u8) -> Result<Self::Ok, Self::Error> {
        Ok(DslValue::Number(f64::from(v)))
    }

    fn serialize_u16(self, v: u16) -> Result<Self::Ok, Self::Error> {
        Ok(DslValue::Number(f64::from(v)))
    }

    fn serialize_u32(self, v: u32) -> Result<Self::Ok, Self::Error> {
        Ok(DslValue::Number(f64::from(v)))
    }

    fn serialize_u64(self, v: u64) -> Result<Self::Ok, Self::Error> {
        Ok(DslValue::Number(v as f64))
    }

    fn serialize_f32(self, v: f32) -> Result<Self::Ok, Self::Error> {
        Ok(DslValue::Number(f64::from(v)))
    }

    fn serialize_f64(self, v: f64) -> Result<Self::Ok, Self::Error> {
        Ok(DslValue::Number(v))
    }

    fn serialize_char(self, v: char) -> Result<Self::Ok, Self::Error> {
        Ok(DslValue::String(v.to_string()))
    }

    fn serialize_str(self, v: &str) -> Result<Self::Ok, Self::Error> {
        Ok(DslValue::String(v.to_owned()))
    }

    fn serialize_bytes(self, v: &[u8]) -> Result<Self::Ok, Self::Error> {
        Ok(DslValue::Array(v.iter().map(|b| DslValue::Number(f64::from(*b))).collect()))
    }

    fn serialize_none(self) -> Result<Self::Ok, Self::Error> {
        Ok(DslValue::Null)
    }

    fn serialize_some<T: ?Sized + Serialize>(self, value: &T) -> Result<Self::Ok, Self::Error> {
        value.serialize(self)
    }

    fn serialize_unit(self) -> Result<Self::Ok, Self::Error> {
        Ok(DslValue::Null)
    }

    fn serialize_unit_struct(self, _name: &'static str) -> Result<Self::Ok, Self::Error> {
        Ok(DslValue::Null)
    }

    fn serialize_unit_variant(self, _name: &'static str, _variant_index: u32, variant: &'static str) -> Result<Self::Ok, Self::Error> {
        Ok(DslValue::String(variant.to_owned()))
    }

    fn serialize_newtype_struct<T: ?Sized + Serialize>(self, _name: &'static str, value: &T) -> Result<Self::Ok, Self::Error> {
        value.serialize(self)
    }

    fn serialize_newtype_variant<T: ?Sized + Serialize>(self, _name: &'static str, _variant_index: u32, variant: &'static str, value: &T) -> Result<Self::Ok, Self::Error> {
        Ok(DslValue::object([("kind".to_owned(), DslValue::String(variant.to_owned())), ("value".to_owned(), value.serialize(ValueSerializer)?)]))
    }

    fn serialize_seq(self, len: Option<usize>) -> Result<Self::SerializeSeq, Self::Error> {
        Ok(SeqSerializer { vec: Vec::with_capacity(len.unwrap_or(0)) })
    }

    fn serialize_tuple(self, len: usize) -> Result<Self::SerializeTuple, Self::Error> {
        Ok(SeqSerializer { vec: Vec::with_capacity(len) })
    }

    fn serialize_tuple_struct(self, _name: &'static str, len: usize) -> Result<Self::SerializeTuple, Self::Error> {
        Ok(SeqSerializer { vec: Vec::with_capacity(len) })
    }

    fn serialize_tuple_variant(self, _name: &'static str, _variant_index: u32, variant: &'static str, len: usize) -> Result<Self::SerializeTupleVariant, Self::Error> {
        Ok(TupleVariantSerializer { variant, vec: Vec::with_capacity(len) })
    }

    fn serialize_map(self, len: Option<usize>) -> Result<Self::SerializeMap, Self::Error> {
        Ok(MapSerializer { entries: Vec::with_capacity(len.unwrap_or(0)), pending_key: None })
    }

    fn serialize_struct(self, _name: &'static str, len: usize) -> Result<Self::SerializeStruct, Self::Error> {
        Ok(MapSerializer { entries: Vec::with_capacity(len), pending_key: None })
    }

    fn serialize_struct_variant(self, _name: &'static str, _variant_index: u32, variant: &'static str, len: usize) -> Result<Self::SerializeStructVariant, Self::Error> {
        Ok(StructVariantSerializer { variant, entries: Vec::with_capacity(len) })
    }
}

pub struct SeqSerializer {
    vec: Vec<DslValue>,
}

impl ser::SerializeSeq for SeqSerializer {
    type Ok = DslValue;
    type Error = SerdeError;

    fn serialize_element<T: ?Sized + Serialize>(&mut self, value: &T) -> Result<(), Self::Error> {
        self.vec.push(value.serialize(ValueSerializer)?);
        Ok(())
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(DslValue::Array(self.vec))
    }
}

impl ser::SerializeTuple for SeqSerializer {
    type Ok = DslValue;
    type Error = SerdeError;

    fn serialize_element<T: ?Sized + Serialize>(&mut self, value: &T) -> Result<(), Self::Error> {
        ser::SerializeSeq::serialize_element(self, value)
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        ser::SerializeSeq::end(self)
    }
}

impl ser::SerializeTupleStruct for SeqSerializer {
    type Ok = DslValue;
    type Error = SerdeError;

    fn serialize_field<T: ?Sized + Serialize>(&mut self, value: &T) -> Result<(), Self::Error> {
        ser::SerializeSeq::serialize_element(self, value)
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        ser::SerializeSeq::end(self)
    }
}

pub struct TupleVariantSerializer {
    variant: &'static str,
    vec: Vec<DslValue>,
}

impl ser::SerializeTupleVariant for TupleVariantSerializer {
    type Ok = DslValue;
    type Error = SerdeError;

    fn serialize_field<T: ?Sized + Serialize>(&mut self, value: &T) -> Result<(), Self::Error> {
        self.vec.push(value.serialize(ValueSerializer)?);
        Ok(())
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        // 🏷️ Same `{kind, value}` tagging as newtype/struct variants (see
        // `EnumAccessTagged::variant_seed` below, which only ever looks for "value") — a
        // variant-specific "fields" key here would silently starve `tuple_variant` on decode.
        Ok(DslValue::object([("kind".to_owned(), DslValue::String(self.variant.to_owned())), ("value".to_owned(), DslValue::Array(self.vec))]))
    }
}

pub struct MapSerializer {
    entries: Vec<(String, DslValue)>,
    pending_key: Option<String>,
}

impl ser::SerializeMap for MapSerializer {
    type Ok = DslValue;
    type Error = SerdeError;

    fn serialize_key<T: ?Sized + Serialize>(&mut self, key: &T) -> Result<(), Self::Error> {
        let key = key.serialize(ValueSerializer)?;
        let key = key.as_str().ok_or_else(|| SerdeError("map keys must be strings".to_string()))?.to_owned();
        self.pending_key = Some(key);
        Ok(())
    }

    fn serialize_value<T: ?Sized + Serialize>(&mut self, value: &T) -> Result<(), Self::Error> {
        let key = self.pending_key.take().ok_or_else(|| SerdeError("serialize_value without key".to_string()))?;
        self.entries.push((key, value.serialize(ValueSerializer)?));
        Ok(())
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(DslValue::Object(self.entries))
    }
}

impl ser::SerializeStruct for MapSerializer {
    type Ok = DslValue;
    type Error = SerdeError;

    fn serialize_field<T: ?Sized + Serialize>(&mut self, key: &'static str, value: &T) -> Result<(), Self::Error> {
        self.entries.push((key.to_owned(), value.serialize(ValueSerializer)?));
        Ok(())
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        ser::SerializeMap::end(self)
    }
}

pub struct StructVariantSerializer {
    variant: &'static str,
    entries: Vec<(String, DslValue)>,
}

impl ser::SerializeStructVariant for StructVariantSerializer {
    type Ok = DslValue;
    type Error = SerdeError;

    fn serialize_field<T: ?Sized + Serialize>(&mut self, key: &'static str, value: &T) -> Result<(), Self::Error> {
        self.entries.push((key.to_owned(), value.serialize(ValueSerializer)?));
        Ok(())
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(DslValue::object([("kind".to_owned(), DslValue::String(self.variant.to_owned())), ("value".to_owned(), DslValue::Object(self.entries))]))
    }
}

pub struct ValueDeserializer {
    value: DslValue,
}

impl ValueDeserializer {
    pub fn new(value: DslValue) -> Self {
        Self { value }
    }

    fn take(&mut self) -> DslValue {
        std::mem::replace(&mut self.value, DslValue::Null)
    }
}

impl<'de> de::Deserializer<'de> for &mut ValueDeserializer {
    type Error = SerdeError;

    fn deserialize_any<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value, Self::Error> {
        match self.take() {
            DslValue::Null => visitor.visit_unit(),
            DslValue::Bool(b) => visitor.visit_bool(b),
            DslValue::Number(n) => {
                if n.fract() == 0.0 {
                    if n >= 0.0 && n <= u64::MAX as f64 {
                        visitor.visit_u64(n as u64)
                    } else if n >= i64::MIN as f64 && n <= i64::MAX as f64 {
                        visitor.visit_i64(n as i64)
                    } else {
                        visitor.visit_f64(n)
                    }
                } else {
                    visitor.visit_f64(n)
                }
            }
            DslValue::String(s) => visitor.visit_string(s),
            DslValue::Array(items) => {
                let mut seq = SeqAccessDeserializer::new(items);
                visitor.visit_seq(&mut seq)
            }
            DslValue::Object(entries) => {
                let mut map = MapAccessDeserializer::new(entries);
                visitor.visit_map(&mut map)
            }
        }
    }

    fn deserialize_option<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value, Self::Error> {
        if matches!(self.value, DslValue::Null) {
            visitor.visit_none()
        } else {
            visitor.visit_some(&mut *self)
        }
    }

    fn deserialize_enum<V: Visitor<'de>>(self, _name: &'static str, _variants: &'static [&'static str], visitor: V) -> Result<V::Value, Self::Error> {
        let value = self.take();
        match value {
            DslValue::String(variant) => visitor.visit_enum(EnumAccessUnit { variant }),
            DslValue::Object(entries) => visitor.visit_enum(EnumAccessTagged { entries }),
            other => visitor.visit_enum(EnumAccessTagged { entries: vec![("value".to_owned(), other)] }),
        }
    }

    /// @emoji 🎁️ Newtype structs are transparent single-field wrappers (`serialize_newtype_struct`
    /// never wraps the inner value in any envelope), so the inverse must hand the visitor the very
    /// same `DslValue` back via `visit_newtype_struct`, not re-dispatch on its runtime shape through
    /// `deserialize_any` — the derived `Visitor` for a newtype struct only implements
    /// `visit_newtype_struct`/`visit_seq`, never the scalar `visit_*` methods, so blanket-forwarding
    /// this one broke every newtype wrapping a scalar (bool/number/string).
    fn deserialize_newtype_struct<V: Visitor<'de>>(self, _name: &'static str, visitor: V) -> Result<V::Value, Self::Error> {
        visitor.visit_newtype_struct(self)
    }

    serde::forward_to_deserialize_any! {
        bool i8 i16 i32 i64 u8 u16 u32 u64 f32 f64 char str string
        bytes byte_buf unit unit_struct seq tuple
        tuple_struct map struct identifier ignored_any
    }
}

struct SeqAccessDeserializer {
    iter: std::vec::IntoIter<DslValue>,
}

impl SeqAccessDeserializer {
    fn new(vec: Vec<DslValue>) -> Self {
        Self { iter: vec.into_iter() }
    }
}

impl<'de> SeqAccess<'de> for SeqAccessDeserializer {
    type Error = SerdeError;

    fn next_element_seed<T: DeserializeSeed<'de>>(&mut self, seed: T) -> Result<Option<T::Value>, Self::Error> {
        Ok(self.iter.next().map(|value| seed.deserialize(&mut ValueDeserializer::new(value))).transpose()?)
    }
}

struct MapAccessDeserializer {
    iter: std::vec::IntoIter<(String, DslValue)>,
    value: Option<DslValue>,
}

impl MapAccessDeserializer {
    fn new(entries: Vec<(String, DslValue)>) -> Self {
        Self { iter: entries.into_iter(), value: None }
    }
}

impl<'de> MapAccess<'de> for MapAccessDeserializer {
    type Error = SerdeError;

    fn next_key_seed<K: DeserializeSeed<'de>>(&mut self, seed: K) -> Result<Option<K::Value>, Self::Error> {
        self.value = None;
        match self.iter.next() {
            Some((key, value)) => {
                self.value = Some(value);
                seed.deserialize(MapKeyDeserializer { key }).map(Some)
            }
            None => Ok(None),
        }
    }

    fn next_value_seed<V: DeserializeSeed<'de>>(&mut self, seed: V) -> Result<V::Value, Self::Error> {
        let value = self.value.take().ok_or_else(|| SerdeError("value before key".to_string()))?;
        seed.deserialize(&mut ValueDeserializer::new(value))
    }
}

struct MapKeyDeserializer {
    key: String,
}

impl<'de> de::Deserializer<'de> for MapKeyDeserializer {
    type Error = SerdeError;

    fn deserialize_any<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value, Self::Error> {
        visitor.visit_string(self.key)
    }

    serde::forward_to_deserialize_any! {
        bool i8 i16 i32 i64 u8 u16 u32 u64 f32 f64 char str string
        bytes byte_buf option unit unit_struct newtype_struct seq tuple
        tuple_struct map struct enum identifier ignored_any
    }
}

struct EnumAccessUnit {
    variant: String,
}

impl<'de> EnumAccess<'de> for EnumAccessUnit {
    type Error = SerdeError;
    type Variant = VariantAccessUnit;

    fn variant_seed<V: DeserializeSeed<'de>>(self, seed: V) -> Result<(V::Value, Self::Variant), Self::Error> {
        Ok((seed.deserialize(VariantNameDeserializer { variant: self.variant })?, VariantAccessUnit))
    }
}

struct EnumAccessTagged {
    entries: Vec<(String, DslValue)>,
}

impl<'de> EnumAccess<'de> for EnumAccessTagged {
    type Error = SerdeError;
    type Variant = VariantAccessNewtype;

    fn variant_seed<V: DeserializeSeed<'de>>(self, seed: V) -> Result<(V::Value, Self::Variant), Self::Error> {
        if let Some((_kind, DslValue::String(variant))) = self.entries.iter().find(|(k, _)| k == "kind") {
            let payload = self.entries.iter().find(|(k, _)| k == "value").map(|(_, v)| v.clone()).unwrap_or(DslValue::Null);
            return Ok((seed.deserialize(VariantNameDeserializer { variant: variant.clone() })?, VariantAccessNewtype(payload)));
        }
        let (variant, payload) = self.entries.into_iter().next().ok_or_else(|| SerdeError("empty enum object".to_string()))?;
        Ok((seed.deserialize(VariantNameDeserializer { variant })?, VariantAccessNewtype(payload)))
    }
}

struct VariantNameDeserializer {
    variant: String,
}

impl<'de> de::Deserializer<'de> for VariantNameDeserializer {
    type Error = SerdeError;

    fn deserialize_any<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value, Self::Error> {
        visitor.visit_string(self.variant)
    }

    serde::forward_to_deserialize_any! {
        bool i8 i16 i32 i64 u8 u16 u32 u64 f32 f64 char str string
        bytes byte_buf option unit unit_struct newtype_struct seq tuple
        tuple_struct map struct enum identifier ignored_any
    }
}

struct VariantAccessUnit;

impl<'de> VariantAccess<'de> for VariantAccessUnit {
    type Error = SerdeError;

    fn unit_variant(self) -> Result<(), Self::Error> {
        Ok(())
    }

    fn newtype_variant_seed<T: DeserializeSeed<'de>>(self, _seed: T) -> Result<T::Value, Self::Error> {
        Err(SerdeError("expected newtype variant".to_string()))
    }

    fn tuple_variant<V: Visitor<'de>>(self, _len: usize, _visitor: V) -> Result<V::Value, Self::Error> {
        Err(SerdeError("expected tuple variant".to_string()))
    }

    fn struct_variant<V: Visitor<'de>>(self, _fields: &'static [&'static str], _visitor: V) -> Result<V::Value, Self::Error> {
        Err(SerdeError("expected struct variant".to_string()))
    }
}

struct VariantAccessNewtype(DslValue);

impl<'de> VariantAccess<'de> for VariantAccessNewtype {
    type Error = SerdeError;

    fn unit_variant(self) -> Result<(), Self::Error> {
        Err(SerdeError("expected unit variant".to_string()))
    }

    fn newtype_variant_seed<T: DeserializeSeed<'de>>(self, seed: T) -> Result<T::Value, Self::Error> {
        seed.deserialize(&mut ValueDeserializer::new(self.0))
    }

    fn tuple_variant<V: Visitor<'de>>(self, _len: usize, visitor: V) -> Result<V::Value, Self::Error> {
        match self.0 {
            DslValue::Array(items) => {
                let mut seq = SeqAccessDeserializer::new(items);
                visitor.visit_seq(&mut seq)
            }
            other => {
                let _ = other;
                visitor.visit_seq(&mut SeqAccessDeserializer::new(vec![]))
            }
        }
    }

    fn struct_variant<V: Visitor<'de>>(self, _fields: &'static [&'static str], visitor: V) -> Result<V::Value, Self::Error> {
        match self.0 {
            DslValue::Object(entries) => {
                let mut map = MapAccessDeserializer::new(entries);
                visitor.visit_map(&mut map)
            }
            other => {
                let mut map = MapAccessDeserializer::new(vec![]);
                let _ = other;
                visitor.visit_map(&mut map)
            }
        }
    }
}

impl fmt::Display for ValueDeserializer {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "DslValue deserializer")
    }
}

impl Serialize for DslValue {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: ser::Serializer,
    {
        match self {
            DslValue::Null => serializer.serialize_unit(),
            DslValue::Bool(value) => serializer.serialize_bool(*value),
            DslValue::Number(value) => serializer.serialize_f64(*value),
            DslValue::String(value) => serializer.serialize_str(value),
            DslValue::Array(items) => {
                use ser::SerializeSeq;
                let mut seq = serializer.serialize_seq(Some(items.len()))?;
                for item in items {
                    seq.serialize_element(item)?;
                }
                seq.end()
            }
            DslValue::Object(entries) => {
                use ser::SerializeMap;
                let mut map = serializer.serialize_map(Some(entries.len()))?;
                for (key, value) in entries {
                    map.serialize_entry(key, value)?;
                }
                map.end()
            }
        }
    }
}

struct DslValueSeed;

impl<'de> Visitor<'de> for DslValueSeed {
    type Value = DslValue;

    fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("a dynamic DSL value")
    }

    fn visit_unit<E>(self) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Ok(DslValue::Null)
    }

    fn visit_bool<E>(self, value: bool) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Ok(DslValue::Bool(value))
    }

    fn visit_i64<E>(self, value: i64) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Ok(DslValue::Number(value as f64))
    }

    fn visit_u64<E>(self, value: u64) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Ok(DslValue::Number(value as f64))
    }

    fn visit_f64<E>(self, value: f64) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Ok(DslValue::Number(value))
    }

    fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Ok(DslValue::String(value.to_string()))
    }

    fn visit_string<E>(self, value: String) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Ok(DslValue::String(value))
    }

    fn visit_seq<A>(self, mut access: A) -> Result<Self::Value, A::Error>
    where
        A: SeqAccess<'de>,
    {
        let mut items = Vec::new();
        while let Some(item) = access.next_element()? {
            items.push(item);
        }
        Ok(DslValue::Array(items))
    }

    fn visit_map<A>(self, mut access: A) -> Result<Self::Value, A::Error>
    where
        A: MapAccess<'de>,
    {
        let mut entries = Vec::new();
        while let Some((key, value)) = access.next_entry()? {
            entries.push((key, value));
        }
        Ok(DslValue::Object(entries))
    }
}

impl<'de> de::Deserialize<'de> for DslValue {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: de::Deserializer<'de>,
    {
        deserializer.deserialize_any(DslValueSeed)
    }
}

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use serde::{Deserialize, Serialize};

    fn round_trip<T>(value: T) -> T
    where
        T: Serialize + serde::de::DeserializeOwned,
    {
        let encoded = value.serialize(ValueSerializer).expect("encode into DslValue");
        T::deserialize(&mut ValueDeserializer::new(encoded)).expect("decode from DslValue")
    }

    /// @emoji 🎁️ A newtype struct wrapping a scalar must round-trip: `deserialize_newtype_struct`
    /// used to blanket-forward to `deserialize_any`, which called `visit_u64`/`visit_i64` directly
    /// on the derived newtype visitor — a visitor that only implements `visit_newtype_struct` — and
    /// panicked with "invalid type: integer …, expected tuple struct …".
    #[test]
    fn newtype_struct_wrapping_scalar_round_trips() {
        #[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
        struct Counter(i64);

        assert_eq!(round_trip(Counter(15)), Counter(15));
        assert_eq!(round_trip(Counter(-3)), Counter(-3));
    }

    #[test]
    fn newtype_struct_wrapping_string_round_trips() {
        #[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
        struct Label(String);

        assert_eq!(round_trip(Label("hub".to_owned())), Label("hub".to_owned()));
    }

    /// @emoji 🏷️ `TupleVariantSerializer` used to tag its payload "fields" while
    /// `EnumAccessTagged::variant_seed` only ever looked for "value" (the key newtype/struct
    /// variants use) — every tuple enum variant with 2+ fields silently decoded to an empty seq,
    /// dropping every field instead of erroring.
    #[test]
    fn tuple_enum_variant_round_trips() {
        #[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
        enum E {
            Pair(i64, i64),
        }
        assert_eq!(round_trip(E::Pair(1, 2)), E::Pair(1, 2));
    }
}
//#endregion 🧪️Tests
