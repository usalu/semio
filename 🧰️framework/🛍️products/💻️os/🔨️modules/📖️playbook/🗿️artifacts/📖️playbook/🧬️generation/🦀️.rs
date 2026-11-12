//! 🧬️ Immutable shared generation ownership and exact final-owner JSON retirement.

use super::GenerationPlayState;
use semio_framework_value::{DslValue, FromValue, ToValue};
#[cfg(test)] use semio_framework_value::{ValueRefusalKind};
use std::mem::ManuallyDrop;
use semio_framework_value::retirement::{RetireOwned,arc_custody::ArcLeaseRetirement};
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
        GenerationRootRetirement::lease(self.0.take().expect("original generation lease"))
    }
    pub fn retire_cold(mut self) {
        if let Some(state) = Arc::into_inner(self.0.take().expect("original generation lease")) { GenerationPlayState::retire_decoded(state); }
    }
}

impl Drop for GenerationPlayRoot {
    fn drop(&mut self) {
        let Some(root)=self.0.as_ref() else {return};
        if root.generations.capacity()==0&&root.selected_generation_id.is_none()&&root.preview_text.is_none(){drop(self.0.take());}
        else if !std::thread::panicking(){panic!("nonempty generation root must be explicitly retired before drop");}
    }
}
impl RetireOwned for GenerationPlayRoot {
    type RetirementCursor = GenerationRootRetirement;
    fn retirement_seed(self) -> Self::RetirementCursor {
        self.into_retirement()
    }
    fn retirement_birth_bytes(&self) -> Option<usize> {
        Some(size_of::<GenerationRootRetirement>())
    }
    fn controlled_retirement_supported() -> bool {
        true
    }
}
//#endregion 🪪️ImmutableRoot

//#region 🧹️FinalOwnerRetirement
/// 🔗️ Closes one original generation lease and funds the complete final-owner backing separately.
pub type GenerationRootRetirement = ArcLeaseRetirement<GenerationPlayState>;
//#endregion 🧹️FinalOwnerRetirement

//#region 🧪️RootLaws
#[cfg(test)]
#[path = "🧪️tests/🧬️generation/🦀️.rs"]
mod tests;
//#endregion 🧪️RootLaws
