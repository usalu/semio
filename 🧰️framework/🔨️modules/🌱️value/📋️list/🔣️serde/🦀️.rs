//! 🔣️ Original paged-list serde implementations belong to their concrete storage owner.
use crate::PagedList;
use serde::{de::{Error,SeqAccess,Visitor},ser::SerializeSeq,Deserialize,Deserializer,Serialize,Serializer};
use std::{fmt::{Formatter,Result as FormatResult},marker::PhantomData};
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

