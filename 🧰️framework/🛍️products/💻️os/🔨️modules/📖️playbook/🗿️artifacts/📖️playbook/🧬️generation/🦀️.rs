//! 🧬️ Immutable shared generation ownership and exact final-owner JSON retirement.

use super::GenerationPlayState;
use crate::os_store as store;
use semio_framework_value::{DslValue, FromValue, ToValue};
use std::mem::ManuallyDrop;
use std::sync::Arc;

//#region 🪪️ImmutableRoot
#[derive(Clone, Debug, PartialEq)]
pub struct GenerationPlayRoot(ManuallyDrop<Option<Arc<GenerationPlayState>>>);

impl Default for GenerationPlayRoot {
    fn default() -> Self {
        Self::from(GenerationPlayState::default())
    }
}
impl From<GenerationPlayState> for GenerationPlayRoot {
    fn from(value: GenerationPlayState) -> Self {
        Self(ManuallyDrop::new(Some(Arc::new(value))))
    }
}
impl std::ops::Deref for GenerationPlayRoot {
    type Target = GenerationPlayState;
    fn deref(&self) -> &Self::Target {
        self.as_state()
    }
}
/// 🧬️ The shared owner delegates its declared value fields to GenerationPlayState.
impl ToValue for GenerationPlayRoot {
    fn to_value(&self) -> DslValue {
        self.as_state().to_value()
    }
}
impl FromValue for GenerationPlayRoot {
    fn from_value(value: DslValue) -> Result<Self, ::semio_framework_value::ValueError> {
        GenerationPlayState::from_value(value).map(Self::from)
    }
}
impl GenerationPlayRoot {
    pub fn as_state(&self) -> &GenerationPlayState {
        self.0.as_ref().expect("generation root transferred").as_ref()
    }
    pub fn same_allocation(&self, other: &Self) -> bool {
        Arc::ptr_eq(self.0.as_ref().expect("generation root transferred"), other.0.as_ref().expect("generation root transferred"))
    }
    pub fn cold_builder_mut(&mut self) -> Result<&mut GenerationPlayState, &'static str> {
        Arc::get_mut(self.0.as_mut().expect("generation root transferred")).ok_or("playbook.generation-root-shared")
    }
    pub fn into_retirement(mut self) -> GenerationRootRetirement {
        GenerationRootRetirement { owned: ManuallyDrop::new(GenerationRetirementState { root: self.0.take(), state: None, values: None, value: None, bytes: None }) }
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
struct GenerationRetirementState {
    root: Option<Arc<GenerationPlayState>>,
    state: Option<GenerationPlayState>,
    values: Option<semio_framework_value::ordered::Retirement<DslValue>>,
    value: Option<Box<dyn store::ErasedSnapshotRetirement>>,
    bytes: Option<Vec<u8>>,
}

pub struct GenerationRootRetirement {
    owned: ManuallyDrop<GenerationRetirementState>,
}

impl GenerationRetirementState {
    fn close_step(&mut self, items: usize, bytes: usize) -> Result<store::SnapshotRetirementStep, semio_framework_value::ValueError> {
        use store::SnapshotRetirementStep as Step;
        if items == 0 || bytes == 0 {
            return Ok(Step::Blocked);
        }
        if let Some(value) = self.bytes.as_mut() {
            if !value.is_empty() {
                let processed = bytes.min(value.len());
                value.truncate(value.len() - processed);
                return Ok(Step::Pending { released_items: 0, released_bytes: 0 });
            }
            let released_bytes = value.capacity();
            if released_bytes > bytes { return Ok(Step::Blocked); }
            drop(self.bytes.take());
            return Ok(Step::Pending { released_items: 1, released_bytes });
        }
        if let Some(value) = self.value.as_mut() {
            let step = value.close_step(1, bytes)?;
            if matches!(step, Step::Complete) { self.value = None; return Ok(Step::Pending { released_items: 1, released_bytes: 0 }); }
            return Ok(step);
        }
        if let Some(values) = self.values.as_mut() {
            use semio_framework_value::{ordered::RetirementStep, retained_clone::RetainedCloneGrant};
            match values.advance(RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: bytes, maximum_capacity_bytes: 0, maximum_release_bytes: bytes, maximum_depth: values.next_depth_demand() }) {
                RetirementStep::Blocked => return Ok(Step::Blocked),
                RetirementStep::Progress { released_items, released_bytes } => return Ok(Step::Pending { released_items, released_bytes }),
                RetirementStep::OwnedValue(value) => self.value = Some(semio_framework_value::retirement::owned_retirement(value)),
                RetirementStep::Complete => self.values = None,
                RetirementStep::ProcessedBytes(_) => return Ok(Step::Pending { released_items: 0, released_bytes: 0 }),
                RetirementStep::Failure(error) => return Err(error),
            }
            return Ok(Step::Pending { released_items: 1, released_bytes: 0 });
        }
        if let Some(state) = self.state.as_mut() {
            if let Some(generation) = state.generations.pop() {
                self.values = Some(generation.values.retire());
                self.value = Some(semio_framework_value::retirement::owned_retirement(generation.name));
                self.bytes = Some(generation.id.into_bytes());
            } else if let Some(value) = state.selected_generation_id.take().or_else(|| state.preview_text.take()) {
                self.bytes = Some(value.into_bytes());
            } else {
                self.state = None;
            }
            return Ok(Step::Pending { released_items: 1, released_bytes: 0 });
        }
        if let Some(root) = self.root.take() {
            self.state = Arc::into_inner(root);
            return Ok(Step::Pending { released_items: 1, released_bytes: 0 });
        }
        Ok(Step::Complete)
    }
    fn terminal_is_empty(&self) -> bool {
        self.root.is_none() && self.state.is_none() && self.values.is_none() && self.value.is_none() && self.bytes.is_none()
    }
}

impl store::ErasedSnapshotRetirement for GenerationRootRetirement {
    fn close_step(&mut self, items: usize, bytes: usize) -> Result<store::SnapshotRetirementStep, semio_framework_value::ValueError> {
        self.owned.close_step(items, bytes)
    }
    fn terminal_is_empty(&self) -> bool {
        self.owned.terminal_is_empty()
    }
}

impl Drop for GenerationRootRetirement {
    fn drop(&mut self) {
        if !self.owned.terminal_is_empty() {
            if !std::thread::panicking() {
                panic!("generation root dropped before bounded retirement");
            }
            return;
        }
        unsafe {
            ManuallyDrop::drop(&mut self.owned);
        }
    }
}
//#endregion 🧹️FinalOwnerRetirement

//#region 🧪️RootLaws
#[cfg(test)]
#[path = "🧪️tests/🧬️generation/🦀️.rs"]
mod tests;
//#endregion 🧪️RootLaws
