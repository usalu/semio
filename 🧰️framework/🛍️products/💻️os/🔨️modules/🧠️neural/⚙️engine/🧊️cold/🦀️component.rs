//! 🧊️ Explicit batch ownership scopes; these helpers perform synchronous domain cleanup and earn no retained credit.

use super::*;
use std::ops::{Deref, DerefMut};

//#region 🧊️Scope
/// 🧊️ Consumes an explicitly cold owner, including nested domain values and error-path cleanup.
pub trait ColdRetire { fn retire_cold(self); }

/// 🧊️ Visible synchronous scope for a batch-only domain owner; retained jobs must use typed retirement cursors.
pub struct ColdOwner<T: ColdRetire> { value: Option<T> }
impl<T: ColdRetire> ColdOwner<T> {
    pub fn new(value: T) -> Self { Self { value: Some(value) } }
    pub fn into_inner(mut self) -> T { self.value.take().unwrap() }
}
impl<T: ColdRetire> Deref for ColdOwner<T> { type Target = T; fn deref(&self) -> &T { self.value.as_ref().unwrap() } }
impl<T: ColdRetire> DerefMut for ColdOwner<T> { fn deref_mut(&mut self) -> &mut T { self.value.as_mut().unwrap() } }
impl<T: ColdRetire> Drop for ColdOwner<T> { fn drop(&mut self) { if let Some(value) = self.value.take() { value.retire_cold(); } } }
impl<T: ColdRetire + std::fmt::Debug> std::fmt::Debug for ColdOwner<T> { fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { self.deref().fmt(formatter) } }
impl<T: ColdRetire + PartialEq> PartialEq for ColdOwner<T> { fn eq(&self, other: &Self) -> bool { self.deref() == other.deref() } }
impl<T: ColdRetire + Serialize> Serialize for ColdOwner<T> { fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> { self.deref().serialize(serializer) } }
//#endregion 🧊️Scope

//#region 🧬️DomainOwners
impl ColdRetire for String { fn retire_cold(self) { drop(self); } }
impl<A: ColdRetire, B: ColdRetire, C: ColdRetire> ColdRetire for (A, B, C) { fn retire_cold(self) { self.0.retire_cold(); self.1.retire_cold(); self.2.retire_cold(); } }
impl ColdRetire for Dictionary { fn retire_cold(self) { retirement::retire_value_cold(self.into_retirement()); } }
impl ColdRetire for Value { fn retire_cold(self) { retirement::retire_value_cold(ValueRetirement::from_value(self)); } }
impl<T: ColdRetire> ColdRetire for Vec<T> { fn retire_cold(self) { for value in self { value.retire_cold(); } } }
impl<T: ColdRetire> ColdRetire for Option<T> { fn retire_cold(self) { if let Some(value) = self { value.retire_cold(); } } }
impl<K, V: ColdRetire> ColdRetire for HashMap<K, V> { fn retire_cold(self) { for (_, value) in self { value.retire_cold(); } } }
impl<K, V: ColdRetire> ColdRetire for BTreeMap<K, V> { fn retire_cold(self) { for (_, value) in self { value.retire_cold(); } } }
impl ColdRetire for FieldSpec { fn retire_cold(self) { self.default.retire_cold(); } }
impl ColdRetire for Schema { fn retire_cold(self) { self.fields.retire_cold(); } }
impl ColdRetire for ChannelSpec { fn retire_cold(self) { self.default.retire_cold(); } }
impl ColdRetire for OperatorInfo { fn retire_cold(self) { self.inputs.retire_cold(); self.outputs.retire_cold(); } }
impl ColdRetire for OperatorRecord {
    fn retire_cold(self) { self.info.retire_cold(); for implementation in self.implementations { implementation.operator.retire_cold(); } }
}
impl ColdRetire for Registry { fn retire_cold(self) { self.schemas.retire_cold(); self.operators.retire_cold(); } }
impl ColdRetire for Neuron { fn retire_cold(self) { self.params.retire_cold(); if let Some(tree) = self.tree { (*tree).retire_cold(); } } }
impl ColdRetire for Tree { fn retire_cold(self) { self.neurons.retire_cold(); } }
impl ColdRetire for EvalChannels { fn retire_cold(self) { self.outputs.retire_cold(); self.inputs.retire_cold(); } }
impl ColdRetire for BudgetedEval { fn retire_cold(self) { self.channels.retire_cold(); } }
impl ColdRetire for NeuralCache {
    fn retire_cold(self) {
        let mut retirement = NeuralCacheRetirement::new(std::sync::Arc::new(self));
        while !matches!(retirement.close_step(1, 4096), ValueRetirementStep::Complete) {}
    }
}
//#endregion 🧬️DomainOwners
