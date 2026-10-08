//! 🔺️ Note composite-window diffs — sparse per-field deltas over the window configuration and the window transient.

use super::{NoteCompositeWindowConfig, NoteCompositeWindowTransient};
use crate::editor::note::commands::ink_apply_events::NoteInkToolState;
use crate::schema::diff::NoteAssigned;
use crate::NoteCamera;
use protocol::{ApplyCapability, DiffAlgebra, MutationApplyResult, MutationDiff};
use semio_framework_value_derive::{FromValue, ToValue};

/// 🔺️ Sparse field delta for the composite-window configuration; an absent field is untouched.
#[derive(Clone, Debug, Default, PartialEq, ToValue, FromValue)]
#[value(rename_all = "camelCase", default)]
pub struct NoteCompositeWindowConfigDiff {
    pub camera: Option<NoteCamera>,
}

impl MutationDiff<NoteCompositeWindowConfig> for NoteCompositeWindowConfigDiff {
    fn apply(&self, base: &NoteCompositeWindowConfig, _capability: ApplyCapability) -> MutationApplyResult<NoteCompositeWindowConfig> {
        let mut next = base.clone();
        if let Some(camera) = &self.camera {
            next.camera = camera.clone();
        }
        Ok(next)
    }

    fn absorb(&mut self, other: Self) {
        if other.camera.is_some() {
            self.camera = other.camera;
        }
    }
}

impl DiffAlgebra<NoteCompositeWindowConfig> for NoteCompositeWindowConfigDiff {
    fn inverse(&self, base: &NoteCompositeWindowConfig) -> Self {
        Self { camera: self.camera.as_ref().map(|_| base.camera.clone()) }
    }

    fn is_empty(&self) -> bool {
        self == &Self::default()
    }
}

/// 🔺️ Sparse field delta for the composite-window transient; an absent field is untouched.
#[derive(Clone, Debug, Default, PartialEq, ToValue, FromValue)]
#[value(rename_all = "camelCase", default)]
pub struct NoteCompositeWindowTransientDiff {
    pub engagement_input: Option<String>,
    pub ink_tool: Option<NoteAssigned<Option<NoteInkToolState>>>,
}

impl MutationDiff<NoteCompositeWindowTransient> for NoteCompositeWindowTransientDiff {
    fn apply(&self, base: &NoteCompositeWindowTransient, _capability: ApplyCapability) -> MutationApplyResult<NoteCompositeWindowTransient> {
        let mut next = base.clone();
        if let Some(engagement_input) = &self.engagement_input {
            next.engagement_input = engagement_input.clone();
        }
        if let Some(ink_tool) = &self.ink_tool {
            next.ink_tool = ink_tool.value.clone();
        }
        Ok(next)
    }

    fn absorb(&mut self, other: Self) {
        if other.engagement_input.is_some() {
            self.engagement_input = other.engagement_input;
        }
        if other.ink_tool.is_some() {
            self.ink_tool = other.ink_tool;
        }
    }
}

impl DiffAlgebra<NoteCompositeWindowTransient> for NoteCompositeWindowTransientDiff {
    fn inverse(&self, base: &NoteCompositeWindowTransient) -> Self {
        Self { engagement_input: self.engagement_input.as_ref().map(|_| base.engagement_input.clone()), ink_tool: self.ink_tool.as_ref().map(|_| NoteAssigned::new(base.ink_tool.clone())) }
    }

    fn is_empty(&self) -> bool {
        self == &Self::default()
    }
}
