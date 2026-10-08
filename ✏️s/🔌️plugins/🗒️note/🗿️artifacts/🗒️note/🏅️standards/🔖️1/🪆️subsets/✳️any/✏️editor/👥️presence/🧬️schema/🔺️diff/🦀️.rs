//! 🔺️ Note presence diff — sparse per-field delta over [`NotePresence`].

use super::super::NotePresence;
use protocol::{ApplyCapability, DiffAlgebra, MutationApplyResult, MutationDiff};

/// 🔺️ Sparse field delta for the note presence; an absent field is untouched.
#[derive(Clone, Debug, Default, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue)]
#[value(rename_all = "camelCase", default)]
pub struct NotePresenceDiff {
    pub camera_x: Option<f64>,
    pub camera_y: Option<f64>,
    pub camera_zoom: Option<f64>,
}

impl MutationDiff<NotePresence> for NotePresenceDiff {
    fn apply(&self, base: &NotePresence, _capability: ApplyCapability) -> MutationApplyResult<NotePresence> {
        let mut next = base.clone();
        if let Some(camera_x) = self.camera_x {
            next.camera_x = camera_x;
        }
        if let Some(camera_y) = self.camera_y {
            next.camera_y = camera_y;
        }
        if let Some(camera_zoom) = self.camera_zoom {
            next.camera_zoom = camera_zoom;
        }
        Ok(next)
    }

    fn absorb(&mut self, other: Self) {
        if other.camera_x.is_some() {
            self.camera_x = other.camera_x;
        }
        if other.camera_y.is_some() {
            self.camera_y = other.camera_y;
        }
        if other.camera_zoom.is_some() {
            self.camera_zoom = other.camera_zoom;
        }
    }
}

impl DiffAlgebra<NotePresence> for NotePresenceDiff {
    fn inverse(&self, base: &NotePresence) -> Self {
        Self { camera_x: self.camera_x.map(|_| base.camera_x), camera_y: self.camera_y.map(|_| base.camera_y), camera_zoom: self.camera_zoom.map(|_| base.camera_zoom) }
    }

    fn between(base: &NotePresence, other: &NotePresence) -> Self {
        Self { camera_x: (base.camera_x != other.camera_x).then_some(other.camera_x), camera_y: (base.camera_y != other.camera_y).then_some(other.camera_y), camera_zoom: (base.camera_zoom != other.camera_zoom).then_some(other.camera_zoom) }
    }

    fn is_empty(&self) -> bool {
        self == &Self::default()
    }
}
