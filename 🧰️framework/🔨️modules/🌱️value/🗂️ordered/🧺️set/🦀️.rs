//! 🧺️ Immutable ordered string membership over the codebase-owned retained map.

use super::{Grant, Iter, LookupCursor, OrderedMap, Retirement, RetirementStep, UpdateCursor};
use super::super::{DslValue, FromValue, ToValue, ValueError};

//#region 🧺️OrderedSet
#[derive(Clone, Debug, Default, PartialEq, Eq)]
#[must_use = "ordered set ownership must be explicitly retired"]
pub struct OrderedSet { values: OrderedMap<()> }

impl OrderedSet {
    pub fn new() -> Self { Self::default() }
    pub fn from_map(values: OrderedMap<()>) -> Self { Self { values } }
    pub fn len(&self) -> usize { self.values.len() }
    pub fn is_empty(&self) -> bool { self.values.is_empty() }
    pub fn iter(&self) -> SetIter<'_> { SetIter(self.values.iter()) }
    /// 🧊️ Cold synchronous lookup; retained membership uses begin_lookup.
    pub fn contains(&self, key: &str) -> bool { self.values.contains_key(key) }
    /// 🧊️ Cold synchronous insertion; retained callers use begin_insert.
    pub fn insert(&mut self, key: String) -> bool { self.values.insert(key, ()).is_none() }
    /// 🧊️ Cold synchronous removal; retained callers use begin_remove.
    pub fn remove(&mut self, key: &str) -> bool { self.values.remove(key).is_some() }
    pub fn begin_insert(&self, key: String) -> UpdateCursor<()> { self.values.begin_set(key, ()) }
    pub fn begin_remove(&self, key: String) -> UpdateCursor<()> { self.values.begin_remove(key) }
    pub fn begin_lookup(&self, key: String) -> LookupCursor<()> { self.values.begin_lookup(key) }
    pub fn retire(self) -> Retirement<()> { self.values.retire() }
    /// 🧊️ Explicit cold cleanup; never called by retained advance or Drop.
    pub fn retire_cold(self) {
        let mut retirement = self.retire();
        while !matches!(retirement.advance(Grant { maximum_items: 1, maximum_bytes: 4096 }), RetirementStep::Complete) {}
    }
}

pub struct SetIter<'a>(Iter<'a, ()>);
impl<'a> Iterator for SetIter<'a> {
    type Item = &'a String;
    fn next(&mut self) -> Option<Self::Item> { self.0.next().map(|(key, ())| key) }
    fn size_hint(&self) -> (usize, Option<usize>) { self.0.size_hint() }
}
impl DoubleEndedIterator for SetIter<'_> { fn next_back(&mut self) -> Option<Self::Item> { self.0.next_back().map(|(key, ())| key) } }
impl ExactSizeIterator for SetIter<'_> {}
impl<'a> IntoIterator for &'a OrderedSet { type Item = &'a String; type IntoIter = SetIter<'a>; fn into_iter(self) -> Self::IntoIter { self.iter() } }
impl FromIterator<String> for OrderedSet {
    fn from_iter<T: IntoIterator<Item = String>>(values: T) -> Self { let mut set = Self::new(); for value in values { set.insert(value); } set }
}
impl<const N: usize> From<[String; N]> for OrderedSet { fn from(values: [String; N]) -> Self { values.into_iter().collect() } }
//#endregion 🧺️OrderedSet

//#region 🔁️ValueCodec
/// 🔁️ Mirrors the hand-written `Serialize`/`Deserialize` below: a plain string array.
impl ToValue for OrderedSet {
    fn to_value(&self) -> DslValue {
        DslValue::Array(self.iter().map(|value| DslValue::String(value.clone())).collect())
    }
}

impl FromValue for OrderedSet {
    fn from_value(value: DslValue) -> Result<Self, ValueError> {
        let DslValue::Array(items) = value else { return Err(ValueError::new("expected an array for OrderedSet")) };
        items.into_iter().map(String::from_value).collect()
    }
}
//#endregion 🔁️ValueCodec

//#region 🔀️ArrayWire
/// 🧊️ Gated (RUNTIME-DEPENDENCY-ELIMINATION-FOR-S-PLUGINS-AND-ARTIFACTS, 26/09/02): `ToValue`/
/// `FromValue` above is the real wire codec every non-test caller uses. Plain `#[cfg(test)]`
/// alone would only cover THIS crate's own `🧪️tests/🦀️.rs` differential proof against real
/// `serde_json` — a downstream crate's `#[cfg(test)]` build (e.g. `os-flow`'s test target) never
/// activates a DEPENDENCY crate's own `#[cfg(test)]` code, yet `os-flow`'s `Widget::OutputPreview`/
/// `FlowPreviewGui` (`💻️os/🔨️modules/🌊️flow/📄️artifact/🦀️.rs`) carry
/// `#[cfg_attr(test, derive(Serialize, Deserialize))]` over a field of this type: the derive macro
/// requires the `OrderedSet: Serialize`/`Deserialize` bound to exist at THEIR compile time, even
/// though it's never called at runtime (`ToValue`/`FromValue` is). So this also honors the
/// `ordered-set-serde` feature (see this crate's `Cargo.toml`), which `os-flow` enables on its
/// `semio-framework-replication` dependency for exactly that cross-crate derive to type-check.
#[cfg(any(test, feature = "ordered-set-serde"))]
impl serde::Serialize for OrderedSet {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        use serde::ser::SerializeSeq;
        let mut sequence = serializer.serialize_seq(Some(self.len()))?;
        for value in self { sequence.serialize_element(value)?; }
        sequence.end()
    }
}
#[cfg(any(test, feature = "ordered-set-serde"))]
impl<'de> serde::Deserialize<'de> for OrderedSet {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = OrderedSet;
            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { formatter.write_str("an ordered string array") }
            fn visit_seq<A: serde::de::SeqAccess<'de>>(self, mut access: A) -> Result<Self::Value, A::Error> {
                let mut set = OrderedSet::new();
                loop {
                    match access.next_element::<String>() {
                        Ok(Some(value)) => { set.insert(value); }
                        Ok(None) => return Ok(set),
                        Err(error) => { set.retire_cold(); return Err(error); }
                    }
                }
            }
        }
        deserializer.deserialize_seq(Visitor)
    }
}
//#endregion 🔀️ArrayWire

//#region 🧪️SetLaws
#[cfg(test)]
#[path = "🧪️tests/🦀️.rs"]
mod tests;
//#endregion 🧪️SetLaws
