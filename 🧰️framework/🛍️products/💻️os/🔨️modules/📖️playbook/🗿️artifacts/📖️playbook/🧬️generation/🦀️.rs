//! 🧬️ Immutable shared generation ownership and exact final-owner JSON retirement.

use super::GenerationPlayState;
use crate::os_store as store;
use semio_framework_value::{DslValue, FromValue, RetirementDemand, ToValue, ValueError, ValueRefusalKind, retained_clone::{RetainedCloneGrant, RetainedCloneProgress, RetainedCloneStep}};
use std::mem::ManuallyDrop;
use semio_framework_value::{retained_clone::{RetainedCloneGrant,RetainedCloneStep},retirement::{RetireOwned,shared::SharedControlledRetirement}};
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
        assert!(GenerationPlayState::controlled_retirement_supported(), "generation fields require complete controlled retirement");
        GenerationRootRetirement { owned: SharedControlledRetirement::lease(self.0.take().expect("original generation lease")) }
    }
    pub fn retire_cold(self) {
        let mut retirement = self.into_retirement();
        loop {
            let copy=4096.max(retirement.next_copy_byte_demand().expect("cold generation copy demand"));
            let grant=RetainedCloneGrant {maximum_items:1,maximum_copy_bytes:copy,maximum_capacity_bytes:retirement.next_capacity_byte_demand(copy).expect("cold generation capacity demand"),maximum_release_bytes:retirement.next_release_byte_demand().expect("cold generation release demand"),maximum_depth:retirement.next_depth_demand().expect("cold generation depth demand")};
            let step=retirement.close_step(grant).expect("cold generation retirement");
            assert!(step.progress().fits(grant),"cold generation receipt exceeds original grant");
            if matches!(step,RetainedCloneStep::Complete(_)){assert!(retirement.terminal_is_empty());break;}
            assert_ne!(step.progress(),Default::default(),"cold generation requires returned observer leases");
        }
    }
}

impl Drop for GenerationPlayRoot {
    fn drop(&mut self) {
        let Some(root)=self.0.as_ref() else {return};
        if root.generations.capacity()==0&&root.selected_generation_id.is_none()&&root.preview_text.is_none(){drop(self.0.take());}
        else if !std::thread::panicking(){panic!("nonempty generation root must be explicitly retired before drop");}
    }
}
//#endregion 🪪️ImmutableRoot

//#region 🧹️FinalOwnerRetirement
/// 🔗️ Closes one original generation lease and funds the complete final-owner backing separately.
pub struct GenerationRootRetirement {owned:SharedControlledRetirement<GenerationPlayState>}
impl store::ErasedSnapshotRetirement for GenerationRootRetirement {
    fn close_step(&mut self,grant:RetainedCloneGrant)->Result<RetainedCloneStep,semio_framework_value::ValueError>{self.owned.step(grant)}
    fn next_copy_byte_demand(&self)->Result<usize,semio_framework_value::ValueError>{self.owned.next_copy_byte_demand()}
    fn next_capacity_byte_demand(&self,copy:usize)->Result<usize,semio_framework_value::ValueError>{self.owned.next_capacity_byte_demand(copy)}
    fn next_release_byte_demand(&self)->Result<usize,semio_framework_value::ValueError>{self.owned.next_release_byte_demand()}
    fn next_depth_demand(&self)->Result<usize,semio_framework_value::ValueError>{self.owned.next_depth_demand()}
    fn terminal_is_empty(&self)->bool{self.owned.terminal_is_empty()}
}
//#endregion 🧹️FinalOwnerRetirement

//#region 🧪️RootLaws
#[cfg(test)]
#[path = "🧪️tests/🧬️generation/🦀️.rs"]
mod tests;
//#endregion 🧪️RootLaws
