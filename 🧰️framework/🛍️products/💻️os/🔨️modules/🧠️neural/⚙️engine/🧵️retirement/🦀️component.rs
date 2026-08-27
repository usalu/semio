//! 🧹️ Exact nested-value retirement and explicitly synchronous construction owners.

use super::{Atom, ChannelSpec, Dictionary, EvalChannels, FieldSpec, NeuronSnapshot, OperatorInfo, Schema, TreeSnapshot, Value, ValueType};
use protocol::value::ordered::{Grant, OrderedMap, Retirement, RetirementStep};
use std::collections::{BTreeMap, LinkedList};
use std::mem::ManuallyDrop;
use std::sync::Arc;

//#region 🧵️DomainRetirement
enum Owner {
    Map(Retirement<Value>), Value(Value), Shared(Arc<Value>), Bytes(Vec<u8>), Strings(Vec<String>),
    Dictionaries(BTreeMap<String, Dictionary>), Snapshot(TreeSnapshot), Neurons(BTreeMap<String, NeuronSnapshot>), Seeds(BTreeMap<String, u64>),
    Operator(OperatorInfo), Channels(Vec<ChannelSpec>), Schema(Schema), Fields(Vec<FieldSpec>), Type(ValueType),
}
/// 🎟️ Exact released payload bytes and one retained structural ownership operation.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ValueRetirementStep { Blocked, Pending { released_items: usize, released_bytes: usize }, Complete }

/// 🔒️ Owns all nested dictionary and string frontiers until explicit terminal-empty close.
#[must_use = "nested value retirement must reach terminal-empty before drop"]
pub struct ValueRetirement { owners: ManuallyDrop<LinkedList<Owner>> }
impl Default for ValueRetirement { fn default() -> Self { Self { owners: ManuallyDrop::new(LinkedList::new()) } } }
impl ValueRetirement {
    pub fn from_value(value: Value) -> Self { let mut owner = Self::default(); owner.push_value(value); owner }
    pub fn from_dictionary(value: Dictionary) -> Self { let mut owner = Self::default(); owner.push_dictionary(value); owner }
    pub fn push_value(&mut self, value: Value) { self.owners.push_back(Owner::Value(value)); }
    pub fn push_shared(&mut self, value: Arc<Value>) { self.owners.push_back(Owner::Shared(value)); }
    pub fn push_dictionary(&mut self, mut dictionary: Dictionary) { self.push_map(std::mem::take(&mut dictionary.pairs)); }
    pub fn text(&mut self, text: String) { self.owners.push_back(Owner::Bytes(text.into_bytes())); }
    pub fn push_dictionaries(&mut self, values: BTreeMap<String, Dictionary>) { self.owners.push_back(Owner::Dictionaries(values)); }
    pub fn push_channels(&mut self, channels: EvalChannels) { self.push_dictionaries(channels.outputs); self.push_dictionaries(channels.inputs); }
    pub fn push_snapshot(&mut self, snapshot: TreeSnapshot) { self.owners.push_back(Owner::Snapshot(snapshot)); }
    pub fn push_operator(&mut self, operator: OperatorInfo) { self.owners.push_back(Owner::Operator(operator)); }
    pub fn push_schema(&mut self, schema: Schema) { self.owners.push_back(Owner::Schema(schema)); }
    pub fn push_strings(&mut self, strings: Vec<String>) { self.owners.push_back(Owner::Strings(strings)); }
    pub fn terminal_is_empty(&self) -> bool { self.owners.is_empty() }
    pub(crate) fn push_map(&mut self, map: OrderedMap<Value>) { let retirement = map.retire(); if !retirement.is_empty() { self.owners.push_back(Owner::Map(retirement)); } }
    pub fn close_step(&mut self, maximum_items: usize, maximum_bytes: usize) -> ValueRetirementStep {
        if maximum_items == 0 || maximum_bytes == 0 { return ValueRetirementStep::Blocked; }
        let Some(owner) = self.owners.pop_front() else { return ValueRetirementStep::Complete; };
        let mut released_bytes = 0;
        match owner {
            Owner::Map(mut map) => {
                let step = map.advance(Grant { maximum_items, maximum_bytes });
                if !map.is_empty() { self.owners.push_front(Owner::Map(map)); }
                match step {
                    RetirementStep::OwnedValue(value) => self.owners.push_front(Owner::Value(value)),
                    RetirementStep::Progress { released_bytes: bytes, .. } => released_bytes = bytes,
                    RetirementStep::Blocked => unreachable!("positive domain retirement grant"),
                    RetirementStep::Complete => {}
                }
            }
            Owner::Shared(value) => if let Some(value) = Arc::into_inner(value) { self.owners.push_front(Owner::Value(value)); },
            Owner::Value(Value::Dictionary(dictionary)) => self.push_dictionary(dictionary),
            Owner::Value(Value::Atom(Atom::String(text))) => self.owners.push_front(Owner::Bytes(text.into_bytes())),
            Owner::Value(Value::Atom(_)) => {}
            Owner::Strings(mut values) => {
                if let Some(value) = values.pop() { self.text(value); }
                if !values.is_empty() { self.owners.push_front(Owner::Strings(values)); }
            }
            Owner::Dictionaries(mut values) => {
                if let Some((key, value)) = values.pop_first() { self.text(key); self.push_dictionary(value); }
                if !values.is_empty() { self.owners.push_front(Owner::Dictionaries(values)); }
            }
            Owner::Snapshot(value) => { self.owners.push_front(Owner::Neurons(value.neurons)); self.owners.push_front(Owner::Seeds(value.seed_keys)); }
            Owner::Neurons(mut values) => {
                if let Some((key, value)) = values.pop_first() { self.text(key); self.owners.push_back(Owner::Strings(value.dependents)); }
                if !values.is_empty() { self.owners.push_front(Owner::Neurons(values)); }
            }
            Owner::Seeds(mut values) => {
                if let Some((key, _)) = values.pop_first() { self.text(key); }
                if !values.is_empty() { self.owners.push_front(Owner::Seeds(values)); }
            }
            Owner::Operator(value) => {
                self.text(value.id); self.text(value.extension); self.text(value.name); self.text(value.abbreviation); self.text(value.icon); self.text(value.summary);
                self.owners.push_back(Owner::Channels(value.inputs)); self.owners.push_back(Owner::Channels(value.outputs)); self.owners.push_back(Owner::Strings(value.group));
                if let Some(value) = value.variadic_input { self.text(value.slot_key); }
                if let Some(value) = value.variadic_output { self.text(value.slot_key); }
            }
            Owner::Channels(mut values) => {
                if let Some(value) = values.pop() {
                    self.text(value.code); self.text(value.abbreviation); self.text(value.name); self.text(value.full_name);
                    if let Some(label) = value.label { self.text(label); }
                    if let Some(default) = value.default { self.push_value(default); }
                    self.owners.push_back(Owner::Strings(value.operators));
                }
                if !values.is_empty() { self.owners.push_front(Owner::Channels(values)); }
            }
            Owner::Schema(value) => {
                self.text(value.id); self.text(value.module); self.text(value.name); self.text(value.icon); self.text(value.summary);
                self.owners.push_back(Owner::Fields(value.fields));
            }
            Owner::Fields(mut values) => {
                if let Some(value) = values.pop() {
                    self.text(value.key);
                    if let Some(label) = value.label { self.text(label); }
                    if let Some(default) = value.default { self.push_value(default); }
                    self.owners.push_back(Owner::Type(value.value));
                }
                if !values.is_empty() { self.owners.push_front(Owner::Fields(values)); }
            }
            Owner::Type(ValueType::Schema(id)) => self.text(id),
            Owner::Type(ValueType::List(inner)) => self.owners.push_front(Owner::Type(*inner)),
            Owner::Type(_) => {}
            Owner::Bytes(mut bytes) => {
                released_bytes = maximum_bytes.min(bytes.len()); bytes.truncate(bytes.len() - released_bytes);
                if !bytes.is_empty() { self.owners.push_front(Owner::Bytes(bytes)); }
            }
        }
        ValueRetirementStep::Pending { released_items: 1, released_bytes }
    }
}
impl Drop for ValueRetirement {
    fn drop(&mut self) { if !std::thread::panicking() { assert!(self.terminal_is_empty(), "neural values must finish explicit domain retirement before drop"); } }
}
//#endregion 🧵️DomainRetirement

//#region 🧊️ColdOwners
/// 🧊️ Explicit synchronous boundary; never used by retained advance or close operations.
pub fn retire_value_cold(mut owner: ValueRetirement) {
    while !matches!(owner.close_step(1, 4096), ValueRetirementStep::Complete) {}
}

/// 🧊️ Cold construction owns replacement and error cleanup; its name makes unbounded work explicit.
pub struct ColdDictionaryBuilder { dictionary: Option<Dictionary> }
impl Default for ColdDictionaryBuilder { fn default() -> Self { Self { dictionary: Some(Dictionary::new()) } } }
impl ColdDictionaryBuilder {
    pub fn new() -> Self { Self::default() }
    pub fn from_dictionary(dictionary: Dictionary) -> Self { Self { dictionary: Some(dictionary) } }
    pub fn dictionary(&self) -> &Dictionary { self.dictionary.as_ref().unwrap() }
    pub fn insert(&mut self, key: String, value: Value) {
        let dictionary = self.dictionary.as_mut().unwrap();
        let mut update = dictionary.pairs.begin_set(key, value);
        let grant = Grant { maximum_items: 1, maximum_bytes: 4096 };
        while !update.is_complete() { update.advance(grant); }
        let displaced = std::mem::replace(&mut dictionary.pairs, update.take_result().unwrap());
        let mut retirement = ValueRetirement::default(); retirement.push_map(displaced); retire_value_cold(retirement);
        update.begin_close();
        loop {
            match update.close_step(grant) {
                RetirementStep::OwnedValue(value) => retire_value_cold(ValueRetirement::from_value(value)),
                RetirementStep::Complete => break,
                RetirementStep::Blocked => unreachable!("positive cold builder grant"),
                RetirementStep::Progress { .. } => {}
            }
        }
        assert!(update.terminal_is_empty());
    }
    pub fn finish(mut self) -> Dictionary { self.dictionary.take().unwrap() }
}
impl Drop for ColdDictionaryBuilder {
    fn drop(&mut self) { if let Some(dictionary) = self.dictionary.take() { retire_value_cold(ValueRetirement::from_dictionary(dictionary)); } }
}

/// 🧊️ Explicit cold value scope for batch evaluation, decoding, and tests; retained owners use ValueRetirement.
pub struct ColdValueOwner { value: Option<Value> }
impl ColdValueOwner {
    pub fn new(value: Value) -> Self { Self { value: Some(value) } }
    pub fn value(&self) -> &Value { self.value.as_ref().unwrap() }
    pub fn into_value(mut self) -> Value { self.value.take().unwrap() }
}
impl Drop for ColdValueOwner {
    fn drop(&mut self) { if let Some(value) = self.value.take() { retire_value_cold(ValueRetirement::from_value(value)); } }
}
//#endregion 🧊️ColdOwners

#[cfg(test)]
#[path = "🧪️component.rs"]
mod tests;
