//! 🧪️ Independent serde codec oracle of the snapshot JSON value (test and testing feature only).

use crate::standards::v2_0::subsets::any::schema::snapshot::*;
use serde::de::{MapAccess,SeqAccess,Visitor};
use serde::ser::{SerializeMap,SerializeSeq};
use serde::{Deserialize,Deserializer,Serialize,Serializer};
use std::fmt;
impl Serialize for GltfJson {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            GltfJson::Null => serializer.serialize_unit(),
            GltfJson::Bool(b) => serializer.serialize_bool(*b),
            GltfJson::Number(n) => serializer.serialize_f64(*n),
            GltfJson::String(s) => serializer.serialize_str(s),
            GltfJson::Array(items) => {
                let mut seq = serializer.serialize_seq(Some(items.len()))?;
                for item in items {
                    seq.serialize_element(item)?;
                }
                seq.end()
            }
            GltfJson::Object(members) => {
                let mut map = serializer.serialize_map(Some(members.len()))?;
                for (k, v) in members {
                    map.serialize_entry(k, v)?;
                }
                map.end()
            }
        }
    }
}

struct GltfJsonVisitor;
impl<'de> Visitor<'de> for GltfJsonVisitor {
    type Value = GltfJson;
    fn expecting(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("a JSON value (glTF extras/extensions)")
    }
    fn visit_unit<E>(self) -> Result<Self::Value, E> {
        Ok(GltfJson::Null)
    }
    fn visit_none<E>(self) -> Result<Self::Value, E> {
        Ok(GltfJson::Null)
    }
    fn visit_bool<E>(self, v: bool) -> Result<Self::Value, E> {
        Ok(GltfJson::Bool(v))
    }
    fn visit_i64<E>(self, v: i64) -> Result<Self::Value, E> {
        Ok(GltfJson::Number(v as f64))
    }
    fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E> {
        Ok(GltfJson::Number(v as f64))
    }
    fn visit_f64<E>(self, v: f64) -> Result<Self::Value, E> {
        Ok(GltfJson::Number(v))
    }
    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E> {
        Ok(GltfJson::String(v.to_string()))
    }
    fn visit_string<E>(self, v: String) -> Result<Self::Value, E> {
        Ok(GltfJson::String(v))
    }
    fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
    where
        A: SeqAccess<'de>,
    {
        let mut items = Vec::new();
        while let Some(v) = seq.next_element()? {
            items.push(v);
        }
        Ok(GltfJson::Array(items))
    }
    fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
    where
        A: MapAccess<'de>,
    {
        let mut members = Vec::new();
        while let Some((k, v)) = map.next_entry::<String, GltfJson>()? {
            members.push((k, v));
        }
        Ok(GltfJson::Object(members))
    }
}

impl<'de> Deserialize<'de> for GltfJson {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        deserializer.deserialize_any(GltfJsonVisitor)
    }
}
