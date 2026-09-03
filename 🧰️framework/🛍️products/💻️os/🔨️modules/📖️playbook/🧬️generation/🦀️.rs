//! 🧬️ Immutable shared generation ownership and exact final-owner JSON retirement.

use super::GenerationPlayState;
use crate::os_store as store;
use dsl::{DslValue, FromValue, ToValue};
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::collections::LinkedList;
use std::mem::ManuallyDrop;
use std::sync::Arc;

//#region 🪪️ImmutableRoot
#[derive(Clone, Debug, PartialEq)]
pub struct GenerationPlayRoot(ManuallyDrop<Option<Arc<GenerationPlayState>>>);

impl Default for GenerationPlayRoot { fn default() -> Self { Self::from(GenerationPlayState::default()) } }
impl From<GenerationPlayState> for GenerationPlayRoot { fn from(value: GenerationPlayState) -> Self { Self(ManuallyDrop::new(Some(Arc::new(value)))) } }
impl std::ops::Deref for GenerationPlayRoot {
    type Target = GenerationPlayState;
    fn deref(&self) -> &Self::Target { self.as_state() }
}
impl Serialize for GenerationPlayRoot {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> { self.as_state().serialize(serializer) }
}
impl<'de> Deserialize<'de> for GenerationPlayRoot {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> { GenerationPlayState::deserialize(deserializer).map(Self::from) }
}
/// 🌉️ Hand-written, mirroring the `Serialize`/`Deserialize` impls just above — `GenerationPlayRoot`
/// itself is a `ManuallyDrop<Option<Arc<GenerationPlayState>>>` newtype, not a wire shape in its own
/// right, so `#[derive(ToValue, FromValue)]` is not applicable here (there is no sensible field to
/// forward to but `as_state()`, which is a method, not a field). Delegates straight to the inner
/// `GenerationPlayState`'s own derived `ToValue`/`FromValue`.
impl ToValue for GenerationPlayRoot {
    fn to_value(&self) -> DslValue { self.as_state().to_value() }
}
impl FromValue for GenerationPlayRoot {
    fn from_value(value: DslValue) -> Result<Self, ::semio_framework_os_kernel::ValueError> { GenerationPlayState::from_value(value).map(Self::from) }
}
impl GenerationPlayRoot {
    pub fn as_state(&self) -> &GenerationPlayState { self.0.as_ref().expect("generation root transferred").as_ref() }
    pub fn same_allocation(&self, other: &Self) -> bool { Arc::ptr_eq(self.0.as_ref().expect("generation root transferred"), other.0.as_ref().expect("generation root transferred")) }
    pub fn cold_builder_mut(&mut self) -> Result<&mut GenerationPlayState, &'static str> { Arc::get_mut(self.0.as_mut().expect("generation root transferred")).ok_or("playbook.generation-root-shared") }
    pub fn into_retirement(mut self) -> GenerationRootRetirement {
        GenerationRootRetirement { owned: ManuallyDrop::new(GenerationRetirementState { root: self.0.take(), state: None, owners: LinkedList::new(), bytes: None }) }
    }
    pub fn retire_cold(self) {
        use store::ErasedSnapshotRetirement;
        let mut retirement = self.into_retirement();
        while !matches!(retirement.close_step(1, 4096).expect("cold generation retirement"), store::SnapshotRetirementStep::Complete) {}
    }
}

impl Drop for GenerationPlayRoot {
    fn drop(&mut self) {
        let Some(root) = self.0.take() else { return };
        let Some(state) = Arc::into_inner(root) else { return };
        let state = ManuallyDrop::new(state);
        if state.generations.is_empty() && state.selected_generation_id.is_none() && state.preview_text.is_none() {
            drop(ManuallyDrop::into_inner(state));
        } else if !std::thread::panicking() {
            panic!("nonempty generation root must be explicitly retired before drop");
        }
    }
}
//#endregion 🪪️ImmutableRoot

//#region 🧹️FinalOwnerRetirement
enum JsonOwner {
    Value(DslValue),
    Array(std::vec::IntoIter<DslValue>),
    Object(std::vec::IntoIter<(String, DslValue)>),
}

struct GenerationRetirementState {
    root: Option<Arc<GenerationPlayState>>,
    state: Option<GenerationPlayState>,
    owners: LinkedList<JsonOwner>,
    bytes: Option<Vec<u8>>,
}

pub struct GenerationRootRetirement { owned: ManuallyDrop<GenerationRetirementState> }

impl GenerationRetirementState {
    fn close_step(&mut self, items: usize, bytes: usize) -> Result<store::SnapshotRetirementStep, String> {
        use store::SnapshotRetirementStep as Step;
        if items == 0 || bytes == 0 { return Ok(Step::Blocked); }
        if let Some(value) = self.bytes.as_mut() {
            let released_bytes = bytes.min(value.len());
            value.truncate(value.len() - released_bytes);
            if value.is_empty() { self.bytes = None; }
            return Ok(Step::Pending { released_items: 0, released_bytes });
        }
        if let Some(owner) = self.owners.pop_front() {
            match owner {
                JsonOwner::Value(DslValue::String(value)) => self.bytes = Some(value.into_bytes()),
                JsonOwner::Value(DslValue::Array(value)) => self.owners.push_front(JsonOwner::Array(value.into_iter())),
                JsonOwner::Value(DslValue::Object(value)) => self.owners.push_front(JsonOwner::Object(value.into_iter())),
                JsonOwner::Value(_) => {}
                JsonOwner::Array(mut values) => if let Some(value) = values.next() { self.owners.push_front(JsonOwner::Array(values)); self.owners.push_front(JsonOwner::Value(value)); },
                JsonOwner::Object(mut values) => if let Some((key, value)) = values.next() {
                    self.owners.push_front(JsonOwner::Object(values));
                    self.owners.push_front(JsonOwner::Value(value));
                    self.bytes = Some(key.into_bytes());
                },
            }
            return Ok(Step::Pending { released_items: 1, released_bytes: 0 });
        }
        if let Some(state) = self.state.as_mut() {
            if let Some(generation) = state.generations.pop() {
                self.owners.push_front(JsonOwner::Object(generation.values.into_iter().collect::<Vec<_>>().into_iter()));
                self.owners.push_front(JsonOwner::Value(DslValue::String(generation.name)));
                self.bytes = Some(generation.id.into_bytes());
            } else if let Some(value) = state.selected_generation_id.take().or_else(|| state.preview_text.take()) { self.bytes = Some(value.into_bytes()); }
            else { self.state = None; }
            return Ok(Step::Pending { released_items: 1, released_bytes: 0 });
        }
        if let Some(root) = self.root.take() {
            self.state = Arc::into_inner(root);
            return Ok(Step::Pending { released_items: 1, released_bytes: 0 });
        }
        Ok(Step::Complete)
    }
    fn terminal_is_empty(&self) -> bool { self.root.is_none() && self.state.is_none() && self.owners.is_empty() && self.bytes.is_none() }
}

impl store::ErasedSnapshotRetirement for GenerationRootRetirement {
    fn close_step(&mut self, items: usize, bytes: usize) -> Result<store::SnapshotRetirementStep, String> { self.owned.close_step(items, bytes) }
    fn terminal_is_empty(&self) -> bool { self.owned.terminal_is_empty() }
}

impl Drop for GenerationRootRetirement {
    fn drop(&mut self) {
        if !self.owned.terminal_is_empty() {
            if !std::thread::panicking() { panic!("generation root dropped before bounded retirement"); }
            return;
        }
        unsafe { ManuallyDrop::drop(&mut self.owned); }
    }
}
//#endregion 🧹️FinalOwnerRetirement

//#region 🧪️RootLaws
#[cfg(test)]
#[path = "🧪️tests/🦀️.rs"]
mod tests;
//#endregion 🧪️RootLaws
