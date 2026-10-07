//! 🔣️ Independent JSON carrier traits stream paged native arrays and text through their semantic shapes.

use super::{PagedBytes, PagedMap, PagedUtf8};
use crate::list::PagedList;
use serde::{de::{Error, MapAccess, SeqAccess, Visitor}, ser::{SerializeMap, SerializeSeq}, Deserialize, Deserializer, Serialize, Serializer};
use std::{fmt::{Formatter, Result as FormatResult}, marker::PhantomData};

impl<T: Serialize, const N: usize> Serialize for PagedList<T, N> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let mut sequence = serializer.serialize_seq(Some(self.len()))?;
        for value in self { sequence.serialize_element(value)?; }
        sequence.end()
    }
}

struct ListVisitor<T, const N: usize>(PhantomData<T>);

impl<'de, T: Deserialize<'de>, const N: usize> Visitor<'de> for ListVisitor<T, N> {
    type Value = PagedList<T, N>;
    fn expecting(&self, formatter: &mut Formatter<'_>) -> FormatResult { formatter.write_str("a semantic array") }
    fn visit_seq<A: SeqAccess<'de>>(self, mut sequence: A) -> Result<Self::Value, A::Error> {
        let mut output = PagedList::default();
        while let Some(value) = sequence.next_element()? {
            while !output.has_reserved_slot() {
                let bytes = output.next_allocation_bytes().map_err(A::Error::custom)?;
                output.reserve_one(bytes).map_err(|error| A::Error::custom(error.refusal()))?;
            }
            output.push_reserved(value).map_err(|_| A::Error::custom("paged array rejected its admitted slot"))?;
        }
        Ok(output)
    }
}

impl<'de, T: Deserialize<'de>, const N: usize> Deserialize<'de> for PagedList<T, N> {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> { deserializer.deserialize_seq(ListVisitor::<T, N>(PhantomData)) }
}

impl<const N: usize> Serialize for PagedUtf8<N> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> { serializer.collect_str(self) }
}

struct TextVisitor<const N: usize>;

impl<'de, const N: usize> Visitor<'de> for TextVisitor<N> {
    type Value = PagedUtf8<N>;
    fn expecting(&self, formatter: &mut Formatter<'_>) -> FormatResult { formatter.write_str("semantic UTF-8 text") }
    fn visit_str<E: Error>(self, value: &str) -> Result<Self::Value, E> { PagedUtf8::try_from_str(value).map_err(E::custom) }
}

impl<'de, const N: usize> Deserialize<'de> for PagedUtf8<N> {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> { deserializer.deserialize_str(TextVisitor::<N>) }
}

impl<const N: usize> Serialize for PagedBytes<N> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let mut sequence = serializer.serialize_seq(Some(self.len()))?;
        for value in self.iter() { sequence.serialize_element(&value)?; }
        sequence.end()
    }
}

impl<V: Serialize, const N: usize> Serialize for PagedMap<V, N> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let mut map = serializer.serialize_map(Some(self.len()))?;
        for (key, value) in self.iter() { map.serialize_entry(key, value)?; }
        map.end()
    }
}

struct ObjectVisitor<V, const N: usize>(PhantomData<V>);

impl<'de, V: Deserialize<'de>, const N: usize> Visitor<'de> for ObjectVisitor<V, N> {
    type Value = PagedMap<V, N>;
    fn expecting(&self, formatter: &mut Formatter<'_>) -> FormatResult { formatter.write_str("a semantic object with unique UTF-8 keys") }
    fn visit_map<A: MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
        let mut output = Self::Value::default();
        while let Some((key, value)) = map.next_entry::<PagedUtf8<{usize::MAX}>, V>()? {
            if output.entries.iter().any(|entry| entry.0 == key) { return Err(A::Error::custom("duplicate paged object key")); }
            while !output.entries.has_reserved_slot() {
                let bytes = output.entries.next_allocation_bytes().map_err(A::Error::custom)?;
                output.entries.reserve_one(bytes).map_err(|error| A::Error::custom(error.refusal()))?;
            }
            output.entries.push_reserved((key, value)).map_err(|_| A::Error::custom("paged object rejected its admitted slot"))?;
        }
        Ok(output)
    }
}

impl<'de, V: Deserialize<'de>, const N: usize> Deserialize<'de> for PagedMap<V, N> {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> { deserializer.deserialize_map(ObjectVisitor::<V, N>(PhantomData)) }
}
