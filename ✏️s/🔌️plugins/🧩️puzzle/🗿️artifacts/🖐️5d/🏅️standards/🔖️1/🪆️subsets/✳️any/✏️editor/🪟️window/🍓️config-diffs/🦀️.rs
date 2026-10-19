//! 🍓️ Config diffs: the board and world window config diff records, their cold diff algebra and mutation diff impls. See [`super`].

use super::*;

/// 🔺️ Sparse typed delta of one Puzzle 5D board window's persisted-local options: names only the fields a mutation changes.
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[value(rename_all = "camelCase", default)]
pub struct Puzzle5dBoardWindowConfigDiff {
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub camera2d: Option<Puzzle5dCamera2d>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub lod_mode: Option<String>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub suggestion_offset: Option<f64>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub grid_snap_enabled: Option<bool>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub grid_factor: Option<f64>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub grid_visible: Option<bool>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub selectable_kinds: Option<Puzzle5dSelectableKinds>,
}

impl Puzzle5dBoardWindowConfigDiff {
    /// 🎯️ Every field set to `state`'s value.
    pub fn of(state: &Puzzle5dBoardWindowConfig) -> Self {
        Self { camera2d: Some(state.camera2d.clone()), lod_mode: Some(state.lod_mode.clone()), suggestion_offset: Some(state.suggestion_offset), grid_snap_enabled: Some(state.grid_snap_enabled), grid_factor: Some(state.grid_factor), grid_visible: Some(state.grid_visible), selectable_kinds: Some(state.selectable_kinds.clone()) }
    }
    /// ✂️ The named fields that differ from `base`.
    pub fn changed(&self, base: &Puzzle5dBoardWindowConfig) -> Self {
        Self { camera2d: self.camera2d.as_ref().filter(|value| **value != base.camera2d).cloned(), lod_mode: self.lod_mode.as_ref().filter(|value| **value != base.lod_mode).cloned(), suggestion_offset: self.suggestion_offset.as_ref().filter(|value| **value != base.suggestion_offset).cloned(), grid_snap_enabled: self.grid_snap_enabled.as_ref().filter(|value| **value != base.grid_snap_enabled).cloned(), grid_factor: self.grid_factor.as_ref().filter(|value| **value != base.grid_factor).cloned(), grid_visible: self.grid_visible.as_ref().filter(|value| **value != base.grid_visible).cloned(), selectable_kinds: self.selectable_kinds.as_ref().filter(|value| **value != base.selectable_kinds).cloned() }
    }
    /// ↩️ The named fields at the values `base` holds.
    pub fn restoring(&self, base: &Puzzle5dBoardWindowConfig) -> Self {
        Self { camera2d: self.camera2d.as_ref().map(|_| base.camera2d.clone()), lod_mode: self.lod_mode.as_ref().map(|_| base.lod_mode.clone()), suggestion_offset: self.suggestion_offset.as_ref().map(|_| base.suggestion_offset), grid_snap_enabled: self.grid_snap_enabled.as_ref().map(|_| base.grid_snap_enabled), grid_factor: self.grid_factor.as_ref().map(|_| base.grid_factor), grid_visible: self.grid_visible.as_ref().map(|_| base.grid_visible), selectable_kinds: self.selectable_kinds.as_ref().map(|_| base.selectable_kinds.clone()) }
    }
}

impl protocol::MutationDiff<Puzzle5dBoardWindowConfig> for Puzzle5dBoardWindowConfigDiff {


}

impl protocol::ColdMutationDiff<Puzzle5dBoardWindowConfig> for Puzzle5dBoardWindowConfigDiff {
    fn absorb(&mut self, other: Self) {
        if other.camera2d.is_some() {
            self.camera2d = other.camera2d;
        }
        if other.lod_mode.is_some() {
            self.lod_mode = other.lod_mode;
        }
        if other.suggestion_offset.is_some() {
            self.suggestion_offset = other.suggestion_offset;
        }
        if other.grid_snap_enabled.is_some() {
            self.grid_snap_enabled = other.grid_snap_enabled;
        }
        if other.grid_factor.is_some() {
            self.grid_factor = other.grid_factor;
        }
        if other.grid_visible.is_some() {
            self.grid_visible = other.grid_visible;
        }
        if other.selectable_kinds.is_some() {
            self.selectable_kinds = other.selectable_kinds;
        }
    }

    fn apply(&self, next: &mut Puzzle5dBoardWindowConfig, _capability: protocol::ApplyCapability) -> protocol::MutationApplyResult<()> {
        if let Some(value) = &self.camera2d {
            next.camera2d = value.clone();
        }
        if let Some(value) = &self.lod_mode {
            next.lod_mode = value.clone();
        }
        if let Some(value) = &self.suggestion_offset {
            next.suggestion_offset = *value;
        }
        if let Some(value) = &self.grid_snap_enabled {
            next.grid_snap_enabled = *value;
        }
        if let Some(value) = &self.grid_factor {
            next.grid_factor = *value;
        }
        if let Some(value) = &self.grid_visible {
            next.grid_visible = *value;
        }
        if let Some(value) = &self.selectable_kinds {
            next.selectable_kinds = value.clone();
        }
        Ok(())
    }
}

impl protocol::DiffAlgebra<Puzzle5dBoardWindowConfig> for Puzzle5dBoardWindowConfigDiff {

    fn is_empty(&self) -> bool {
        self.camera2d.is_none() && self.lod_mode.is_none() && self.suggestion_offset.is_none() && self.grid_snap_enabled.is_none() && self.grid_factor.is_none() && self.grid_visible.is_none() && self.selectable_kinds.is_none()
    }
}

impl protocol::ColdDiffAlgebra<Puzzle5dBoardWindowConfig> for Puzzle5dBoardWindowConfigDiff {
    fn inverse(&self, base: &Puzzle5dBoardWindowConfig) -> Self {
        self.restoring(base)
    }
}

/// 🔺️ Sparse typed delta of one Puzzle 5D world window's persisted-local options: names only the fields a mutation changes.
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[value(rename_all = "camelCase", default)]
pub struct Puzzle5dWorldWindowConfigDiff {
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub camera3d: Option<Puzzle5dCamera3d>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub sun: Option<WorldSunConfig>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub grid_visible: Option<bool>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub grid_snap_enabled: Option<bool>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub grid_spacing: Option<f64>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub lod_automatic: Option<bool>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub lod_depth_variable: Option<bool>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub lod_manual: Option<f64>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub selectable_kinds: Option<Puzzle5dSelectableKinds>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub grip_show: Option<String>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub grip_direction: Option<String>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub transform_move: Option<bool>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub transform_rotate: Option<bool>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub voxel_dims: Option<[u32; 3]>,
}

impl Puzzle5dWorldWindowConfigDiff {
    /// 🎯️ Every field set to `state`'s value.
    pub fn of(state: &Puzzle5dWorldWindowConfig) -> Self {
        Self { camera3d: Some(state.camera3d.clone()), sun: Some(state.sun.clone()), grid_visible: Some(state.grid_visible), grid_snap_enabled: Some(state.grid_snap_enabled), grid_spacing: Some(state.grid_spacing), lod_automatic: Some(state.lod_automatic), lod_depth_variable: Some(state.lod_depth_variable), lod_manual: Some(state.lod_manual), selectable_kinds: Some(state.selectable_kinds.clone()), grip_show: Some(state.grip_show.clone()), grip_direction: Some(state.grip_direction.clone()), transform_move: Some(state.transform_move), transform_rotate: Some(state.transform_rotate), voxel_dims: Some(state.voxel_dims) }
    }
    /// ✂️ The named fields that differ from `base`.
    pub fn changed(&self, base: &Puzzle5dWorldWindowConfig) -> Self {
        Self { camera3d: self.camera3d.as_ref().filter(|value| **value != base.camera3d).cloned(), sun: self.sun.as_ref().filter(|value| **value != base.sun).cloned(), grid_visible: self.grid_visible.as_ref().filter(|value| **value != base.grid_visible).cloned(), grid_snap_enabled: self.grid_snap_enabled.as_ref().filter(|value| **value != base.grid_snap_enabled).cloned(), grid_spacing: self.grid_spacing.as_ref().filter(|value| **value != base.grid_spacing).cloned(), lod_automatic: self.lod_automatic.as_ref().filter(|value| **value != base.lod_automatic).cloned(), lod_depth_variable: self.lod_depth_variable.as_ref().filter(|value| **value != base.lod_depth_variable).cloned(), lod_manual: self.lod_manual.as_ref().filter(|value| **value != base.lod_manual).cloned(), selectable_kinds: self.selectable_kinds.as_ref().filter(|value| **value != base.selectable_kinds).cloned(), grip_show: self.grip_show.as_ref().filter(|value| **value != base.grip_show).cloned(), grip_direction: self.grip_direction.as_ref().filter(|value| **value != base.grip_direction).cloned(), transform_move: self.transform_move.as_ref().filter(|value| **value != base.transform_move).cloned(), transform_rotate: self.transform_rotate.as_ref().filter(|value| **value != base.transform_rotate).cloned(), voxel_dims: self.voxel_dims.as_ref().filter(|value| **value != base.voxel_dims).cloned() }
    }
    /// ↩️ The named fields at the values `base` holds.
    pub fn restoring(&self, base: &Puzzle5dWorldWindowConfig) -> Self {
        Self { camera3d: self.camera3d.as_ref().map(|_| base.camera3d.clone()), sun: self.sun.as_ref().map(|_| base.sun.clone()), grid_visible: self.grid_visible.as_ref().map(|_| base.grid_visible), grid_snap_enabled: self.grid_snap_enabled.as_ref().map(|_| base.grid_snap_enabled), grid_spacing: self.grid_spacing.as_ref().map(|_| base.grid_spacing), lod_automatic: self.lod_automatic.as_ref().map(|_| base.lod_automatic), lod_depth_variable: self.lod_depth_variable.as_ref().map(|_| base.lod_depth_variable), lod_manual: self.lod_manual.as_ref().map(|_| base.lod_manual), selectable_kinds: self.selectable_kinds.as_ref().map(|_| base.selectable_kinds.clone()), grip_show: self.grip_show.as_ref().map(|_| base.grip_show.clone()), grip_direction: self.grip_direction.as_ref().map(|_| base.grip_direction.clone()), transform_move: self.transform_move.as_ref().map(|_| base.transform_move), transform_rotate: self.transform_rotate.as_ref().map(|_| base.transform_rotate), voxel_dims: self.voxel_dims.as_ref().map(|_| base.voxel_dims) }
    }
}

impl protocol::MutationDiff<Puzzle5dWorldWindowConfig> for Puzzle5dWorldWindowConfigDiff {


}

impl protocol::ColdMutationDiff<Puzzle5dWorldWindowConfig> for Puzzle5dWorldWindowConfigDiff {
    fn absorb(&mut self, other: Self) {
        if other.camera3d.is_some() {
            self.camera3d = other.camera3d;
        }
        if other.sun.is_some() {
            self.sun = other.sun;
        }
        if other.grid_visible.is_some() {
            self.grid_visible = other.grid_visible;
        }
        if other.grid_snap_enabled.is_some() {
            self.grid_snap_enabled = other.grid_snap_enabled;
        }
        if other.grid_spacing.is_some() {
            self.grid_spacing = other.grid_spacing;
        }
        if other.lod_automatic.is_some() {
            self.lod_automatic = other.lod_automatic;
        }
        if other.lod_depth_variable.is_some() {
            self.lod_depth_variable = other.lod_depth_variable;
        }
        if other.lod_manual.is_some() {
            self.lod_manual = other.lod_manual;
        }
        if other.selectable_kinds.is_some() {
            self.selectable_kinds = other.selectable_kinds;
        }
        if other.grip_show.is_some() {
            self.grip_show = other.grip_show;
        }
        if other.grip_direction.is_some() {
            self.grip_direction = other.grip_direction;
        }
        if other.transform_move.is_some() {
            self.transform_move = other.transform_move;
        }
        if other.transform_rotate.is_some() {
            self.transform_rotate = other.transform_rotate;
        }
        if other.voxel_dims.is_some() {
            self.voxel_dims = other.voxel_dims;
        }
    }

    fn apply(&self, next: &mut Puzzle5dWorldWindowConfig, _capability: protocol::ApplyCapability) -> protocol::MutationApplyResult<()> {
        if let Some(value) = &self.camera3d {
            next.camera3d = value.clone();
        }
        if let Some(value) = &self.sun {
            next.sun = value.clone();
        }
        if let Some(value) = &self.grid_visible {
            next.grid_visible = *value;
        }
        if let Some(value) = &self.grid_snap_enabled {
            next.grid_snap_enabled = *value;
        }
        if let Some(value) = &self.grid_spacing {
            next.grid_spacing = *value;
        }
        if let Some(value) = &self.lod_automatic {
            next.lod_automatic = *value;
        }
        if let Some(value) = &self.lod_depth_variable {
            next.lod_depth_variable = *value;
        }
        if let Some(value) = &self.lod_manual {
            next.lod_manual = *value;
        }
        if let Some(value) = &self.selectable_kinds {
            next.selectable_kinds = value.clone();
        }
        if let Some(value) = &self.grip_show {
            next.grip_show = value.clone();
        }
        if let Some(value) = &self.grip_direction {
            next.grip_direction = value.clone();
        }
        if let Some(value) = &self.transform_move {
            next.transform_move = *value;
        }
        if let Some(value) = &self.transform_rotate {
            next.transform_rotate = *value;
        }
        if let Some(value) = &self.voxel_dims {
            next.voxel_dims = *value;
        }
        Ok(())
    }
}

impl protocol::DiffAlgebra<Puzzle5dWorldWindowConfig> for Puzzle5dWorldWindowConfigDiff {

    fn is_empty(&self) -> bool {
        self.camera3d.is_none() && self.sun.is_none() && self.grid_visible.is_none() && self.grid_snap_enabled.is_none() && self.grid_spacing.is_none() && self.lod_automatic.is_none() && self.lod_depth_variable.is_none() && self.lod_manual.is_none() && self.selectable_kinds.is_none() && self.grip_show.is_none() && self.grip_direction.is_none() && self.transform_move.is_none() && self.transform_rotate.is_none() && self.voxel_dims.is_none()
    }
}

impl protocol::ColdDiffAlgebra<Puzzle5dWorldWindowConfig> for Puzzle5dWorldWindowConfigDiff {
    fn inverse(&self, base: &Puzzle5dWorldWindowConfig) -> Self {
        self.restoring(base)
    }
}
