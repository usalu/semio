//! 🗂️ Product-neutral map deltas compose partial changes without losing presence requirements.
use std::collections::BTreeMap;
use crate::value::{DslValue, FromValue, ToValue, ValueError};
use super::{MutationApplyError, MutationApplyResult};

/// 🧭️ Accepted presence of one key in the original base.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MapPresence { Any, Present, Absent, Never }

impl MapPresence {
    fn accepts(self, present: bool) -> bool { self == Self::Any || self == if present { Self::Present } else { Self::Absent } }
    fn name(self) -> &'static str { match self { Self::Any => "any", Self::Present => "present", Self::Absent => "absent", Self::Never => "never" } }
}

/// ✍️ Final value action after the original-base precondition succeeds.
#[derive(Clone, Debug, PartialEq)]
pub enum MapEntryOperation<V> { Set(V), Remove, Reject }

/// 🎯️ A single-key partial transformation.
#[derive(Clone, Debug, PartialEq)]
pub struct MapEntryDelta<V> { precondition: MapPresence, operation: MapEntryOperation<V> }

impl<V> MapEntryDelta<V> {
    /// 🧭️ The entry's original-base presence requirement.
    pub fn precondition(&self) -> MapPresence { self.precondition }
    /// ✍️ The entry's final value operation.
    pub fn operation(&self) -> &MapEntryOperation<V> { &self.operation }
    /// ♻️ Transfers owned fields to an incremental retirement provider.
    pub fn into_parts(self) -> (MapPresence, MapEntryOperation<V>) { (self.precondition, self.operation) }
    fn absorb(&mut self, next: Self) {
        let compatible = self.precondition != MapPresence::Never && next.precondition.accepts(matches!(self.operation, MapEntryOperation::Set(_)));
        if compatible { self.operation = next.operation; }
        else { self.precondition = MapPresence::Never; self.operation = MapEntryOperation::Reject; }
    }
}

/// 🗃️ Sorted, duplicate-free changed entries with one compact transformation per key.
#[derive(Clone, Debug, PartialEq)]
pub struct MapDelta<V> { entries: BTreeMap<String, MapEntryDelta<V>> }

impl<V> Default for MapDelta<V> {
    fn default() -> Self { Self { entries: BTreeMap::new() } }
}

impl<V> MapDelta<V> {
    /// ➕️ A set accepts either original presence and preserves null as a value.
    pub fn set(key: String, value: V) -> Self {
        Self { entries: BTreeMap::from([(key, MapEntryDelta { precondition: MapPresence::Any, operation: MapEntryOperation::Set(value) })]) }
    }
    /// ➖️ A direct removal requires an existing key.
    pub fn remove(key: String) -> Self {
        Self { entries: BTreeMap::from([(key, MapEntryDelta { precondition: MapPresence::Present, operation: MapEntryOperation::Remove })]) }
    }
    /// 📖️ Read-only view of compact entries in canonical key order.
    pub fn entries(&self) -> &BTreeMap<String, MapEntryDelta<V>> { &self.entries }
    /// ♻️ Transfers the entry tree to its caller's incremental retirement provider.
    pub fn into_entries(self) -> BTreeMap<String, MapEntryDelta<V>> { self.entries }
    /// 🕳️ An empty map delta is the identity transformation.
    pub fn is_empty(&self) -> bool { self.entries.is_empty() }
    /// 🪢️ Composes known sequential changes, retaining every original-base requirement.
    pub fn absorb(&mut self, other: Self) {
        for (key, next) in other.entries {
            match self.entries.entry(key) {
                std::collections::btree_map::Entry::Occupied(mut entry) => entry.get_mut().absorb(next),
                std::collections::btree_map::Entry::Vacant(entry) => { entry.insert(next); }
            }
        }
    }
    /// 🧵️ Composes an optional field delta without a product-local map helper.
    pub fn absorb_optional(target: &mut Option<Self>, other: Option<Self>) {
        if let Some(other) = other {
            if let Some(target) = target { target.absorb(other); }
            else { *target = Some(other); }
        }
    }
}

impl<V: Clone> MapDelta<V> {
    /// 🛡️ Validates every original key before changing the first entry.
    pub fn apply_to(&self, target: &mut BTreeMap<String, V>) -> MutationApplyResult<()> {
        for (key, entry) in &self.entries {
            if !entry.precondition.accepts(target.contains_key(key)) {
                let (code, message) = match entry.precondition {
                    MapPresence::Present => ("mutation.apply.missing-target", "required map entry does not exist"),
                    MapPresence::Absent => ("mutation.apply.target-precondition", "map entry must be absent"),
                    _ => ("mutation.apply.unsatisfiable-precondition", "composed map entry has no accepted base"),
                };
                return Err(MutationApplyError::new(code, message).at([key.as_str()]));
            }
        }
        for (key, entry) in &self.entries {
            match &entry.operation {
                MapEntryOperation::Set(value) => { target.insert(key.clone(), value.clone()); }
                MapEntryOperation::Remove => { target.remove(key); }
                MapEntryOperation::Reject => unreachable!("rejection is checked before any entry is applied"),
            }
        }
        Ok(())
    }
}

fn record(value: DslValue, fields: &[&str]) -> Result<BTreeMap<String, DslValue>, ValueError> {
    let DslValue::Object(values) = value else { return Err(ValueError::new("map delta value must be a record")); };
    let mut result = BTreeMap::new();
    for (key, value) in values {
        if result.insert(key, value).is_some() { return Err(ValueError::new("duplicate map delta record field")); }
    }
    if result.len() != fields.len() || fields.iter().any(|key| !result.contains_key(*key)) { return Err(ValueError::new("map delta record has incorrect fields")); }
    Ok(result)
}

impl<V: ToValue> ToValue for MapDelta<V> {
    fn to_value(&self) -> DslValue {
        let entries = self.entries.iter().map(|(key, entry)| {
            let operation = match &entry.operation {
                MapEntryOperation::Set(value) => DslValue::object([("kind".into(), "set".to_value()), ("value".into(), value.to_value())]),
                MapEntryOperation::Remove => DslValue::object([("kind".into(), "remove".to_value())]),
                MapEntryOperation::Reject => DslValue::object([("kind".into(), "reject".to_value())]),
            };
            DslValue::object([("key".into(), key.to_value()), ("precondition".into(), entry.precondition.name().to_value()), ("operation".into(), operation)])
        }).collect();
        DslValue::object([("entries".into(), DslValue::Array(entries))])
    }
}

impl<V: FromValue> FromValue for MapDelta<V> {
    fn from_value(value: DslValue) -> Result<Self, ValueError> {
        let mut root = record(value, &["entries"])?;
        let DslValue::Array(entries) = root.remove("entries").unwrap() else { return Err(ValueError::new("map delta entries must be a list")); };
        let mut result = Self::default();
        for value in entries {
            let mut entry = record(value, &["key", "precondition", "operation"])?;
            let key = String::from_value(entry.remove("key").unwrap())?;
            let precondition = match String::from_value(entry.remove("precondition").unwrap())?.as_str() {
                "any" => MapPresence::Any, "present" => MapPresence::Present, "absent" => MapPresence::Absent, "never" => MapPresence::Never,
                _ => return Err(ValueError::new("unknown map presence requirement")),
            };
            let raw = entry.remove("operation").unwrap();
            let set = raw.as_object().and_then(|fields| fields.iter().find(|(key, _)| key == "kind").map(|(_, value)| value)).and_then(DslValue::as_str) == Some("set");
            let mut operation = record(raw, if set { &["kind", "value"] } else { &["kind"] })?;
            let kind = String::from_value(operation.remove("kind").unwrap())?;
            let operation = match kind.as_str() {
                "set" => MapEntryOperation::Set(V::from_value(operation.remove("value").unwrap())?),
                "remove" => MapEntryOperation::Remove,
                "reject" => MapEntryOperation::Reject,
                _ => return Err(ValueError::new("unknown map operation")),
            };
            if (precondition == MapPresence::Never) != matches!(operation, MapEntryOperation::Reject) { return Err(ValueError::new("unsatisfiable map changes must be explicit rejections")); }
            if result.entries.insert(key, MapEntryDelta { precondition, operation }).is_some() { return Err(ValueError::new("duplicate map delta key")); }
        }
        Ok(result)
    }
}

#[cfg(test)]
#[path = "🧪️tests/🦀️.rs"]
mod tests;
