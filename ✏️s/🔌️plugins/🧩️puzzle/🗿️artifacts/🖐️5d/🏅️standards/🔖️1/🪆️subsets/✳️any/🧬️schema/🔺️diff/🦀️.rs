//! 🧬️ Puzzle5d diff schema — sparse typed delta over the artifact: per-field entity patches and id-keyed collection deltas.

use crate::standards::v1::subsets::any::schema::Puzzle5dArtifact;
use crate::{Puzzle5dCompatSpecificity, Puzzle5dFastener, Puzzle5dGrip, Puzzle5dKindCatalogsExtra, Puzzle5dKindCompatibility, Puzzle5dMeta, Puzzle5dPart, Puzzle5dPart2d, Puzzle5dPart3d, Puzzle5dPartAnchor, Puzzle5dGrip2d, Puzzle5dGrip3d, Puzzle5dScale, Puzzle5dTargetVolume};
use crate::Puzzle5dSnapshot;
use ::semio_framework_schema::ArtifactSchema;
use protocol::{DiffAlgebra, MutationDiff};
use protocol::list_delta::RowPatch;
use semio_s_artifact_stdio_semio::standards::v1::subsets::kit::schema::snapshot::SemioKitSnapshot;

//#region 🔖️Diff
/// 🔺️ Sparse typed delta for the puzzle5d artifact: per-field entity patches and id-keyed collection deltas.
#[derive(Clone, Debug, Default, PartialEq, ArtifactSchema, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned)]
#[value(rename_all = "camelCase", default)]
#[artifact_schema(id = "s.puzzle.puzzle5d")]
pub struct Puzzle5dDiff {
    #[state(artifact)]
    pub artifact: Option<Box<Puzzle5dArtifact>>,
    #[state(artifact)]
    pub schema: Option<String>,
    #[state(artifact)]
    pub domain: Option<String>,
    #[value(default, skip_serializing_if = "Option::is_none", deserialize_with = "deserialize_double_option")]
    #[state(artifact)]
    pub label: Option<Option<String>>,
    #[state(artifact)]
    pub meta: Option<Puzzle5dMetaPatch>,
    #[child(kind = "s.stdio.semio")]
    #[value(default, skip_serializing_if = "Option::is_none", deserialize_with = "deserialize_double_option")]
    #[state(artifact)]
    pub kind_catalogs: Option<Option<store::ArtifactChild<SemioKitSnapshot>>>,
    #[value(default, skip_serializing_if = "Option::is_none", deserialize_with = "deserialize_double_option")]
    #[state(artifact)]
    pub kind_catalogs_extra: Option<Option<Puzzle5dKindCatalogsExtra>>,
    #[state(artifact)]
    pub kind_compatibility: Option<Puzzle5dKindCompatibilityDelta>,
    #[state(artifact)]
    pub parts: Option<Puzzle5dPartsDelta>,
    #[state(artifact)]
    pub fasteners: Option<Puzzle5dFastenersDelta>,
    #[state(artifact)]
    pub target_volumes: Option<Puzzle5dTargetVolumesDelta>,
}
//#endregion 🔖️Diff

//#region 🔖️Patches
/// 🔑️ The identity of one kind-compatibility row: the pair of kinds it links.
#[derive(Clone, Debug, PartialEq, Eq, Hash, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned)]
#[value(rename_all = "camelCase")]
pub struct Puzzle5dKindCompatibilityKey {
    pub source: String,
    pub target: String,
}

/// 🩹 Sparse per-field patch over one `Puzzle5dPart2d` — only the named fields change.
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned)]
#[value(rename_all = "camelCase", default)]
pub struct Puzzle5dPart2dPatch {
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub x: Option<f64>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub y: Option<f64>,
    #[value(default, skip_serializing_if = "Option::is_none", deserialize_with = "deserialize_double_option")]
    pub shape: Option<Option<String>>,
    #[value(default, skip_serializing_if = "Option::is_none", deserialize_with = "deserialize_double_option")]
    pub radius: Option<Option<f64>>,
    #[value(default, skip_serializing_if = "Option::is_none", deserialize_with = "deserialize_double_option")]
    pub width: Option<Option<f64>>,
    #[value(default, skip_serializing_if = "Option::is_none", deserialize_with = "deserialize_double_option")]
    pub height: Option<Option<f64>>,
    #[value(default, skip_serializing_if = "Option::is_none", deserialize_with = "deserialize_double_option")]
    pub text: Option<Option<String>>,
    #[value(default, skip_serializing_if = "Option::is_none", deserialize_with = "deserialize_double_option")]
    pub icon_kind: Option<Option<String>>,
    #[value(default, skip_serializing_if = "Option::is_none", deserialize_with = "deserialize_double_option")]
    pub hidden: Option<Option<bool>>,
    #[value(default, skip_serializing_if = "Option::is_none", deserialize_with = "deserialize_double_option")]
    pub locked: Option<Option<bool>>,
}

/// 🩹 Sparse per-field patch over one `Puzzle5dPart3d` — only the named fields change.
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned)]
#[value(rename_all = "camelCase", default)]
pub struct Puzzle5dPart3dPatch {
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub origin: Option<[f64; 3]>,
    #[value(default, skip_serializing_if = "Option::is_none", deserialize_with = "deserialize_double_option")]
    pub mesh_url: Option<Option<String>>,
    #[value(default, skip_serializing_if = "Option::is_none", deserialize_with = "deserialize_double_option")]
    pub orientation: Option<Option<[f64; 4]>>,
    #[value(default, skip_serializing_if = "Option::is_none", deserialize_with = "deserialize_double_option")]
    pub scale: Option<Option<Puzzle5dScale>>,
    #[value(default, skip_serializing_if = "Option::is_none", deserialize_with = "deserialize_double_option")]
    pub label: Option<Option<String>>,
}

/// 🩹 Sparse per-field patch over one `Puzzle5dGrip2d` — only the named fields change.
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned)]
#[value(rename_all = "camelCase", default)]
pub struct Puzzle5dGrip2dPatch {
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub angle: Option<f64>,
    #[value(default, skip_serializing_if = "Option::is_none", deserialize_with = "deserialize_double_option")]
    pub grip_kind: Option<Option<String>>,
    #[value(default, skip_serializing_if = "Option::is_none", deserialize_with = "deserialize_double_option")]
    pub radius: Option<Option<f64>>,
}

/// 🩹 Sparse per-field patch over one `Puzzle5dGrip3d` — only the named fields change.
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned)]
#[value(rename_all = "camelCase", default)]
pub struct Puzzle5dGrip3dPatch {
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub position: Option<[f64; 3]>,
    #[value(default, skip_serializing_if = "Option::is_none", deserialize_with = "deserialize_double_option")]
    pub direction: Option<Option<[f64; 3]>>,
    #[value(default, skip_serializing_if = "Option::is_none", deserialize_with = "deserialize_double_option")]
    pub radius: Option<Option<f64>>,
    #[value(default, skip_serializing_if = "Option::is_none", deserialize_with = "deserialize_double_option")]
    pub label: Option<Option<String>>,
}

/// 🩹 Sparse per-field patch over one `Puzzle5dGrip` — only the named fields change.
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned)]
#[value(rename_all = "camelCase", default)]
pub struct Puzzle5dGripPatch {
    #[value(default, skip_serializing_if = "Option::is_none", deserialize_with = "deserialize_double_option")]
    pub grip_kind: Option<Option<String>>,
    #[value(default, skip_serializing_if = "Option::is_none", rename = "2d")]
    pub grip_2d: Option<Puzzle5dGrip2dPatch>,
    #[value(default, skip_serializing_if = "Option::is_none", rename = "3d")]
    pub grip_3d: Option<Puzzle5dGrip3dPatch>,
}

/// 🩹 Sparse per-field patch over one `Puzzle5dPart` — only the named fields change.
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned)]
#[value(rename_all = "camelCase", default)]
pub struct Puzzle5dPartPatch {
    #[value(default, skip_serializing_if = "Option::is_none", deserialize_with = "deserialize_double_option")]
    pub part_kind: Option<Option<String>>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub anchor: Option<Puzzle5dPartAnchor>,
    #[value(default, skip_serializing_if = "Option::is_none", rename = "2d")]
    pub part_2d: Option<Puzzle5dPart2dPatch>,
    #[value(default, skip_serializing_if = "Option::is_none", rename = "3d")]
    pub part_3d: Option<Puzzle5dPart3dPatch>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub grips: Option<Puzzle5dGripsDelta>,
}

/// 🩹 Sparse per-field patch over one `Puzzle5dFastener` — only the named fields change.
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned)]
#[value(rename_all = "camelCase", default)]
pub struct Puzzle5dFastenerPatch {
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub source: Option<String>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub target: Option<String>,
    #[value(default, skip_serializing_if = "Option::is_none", deserialize_with = "deserialize_double_option")]
    pub fastener_kind: Option<Option<String>>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub gap: Option<f64>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub shift: Option<f64>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub rise: Option<f64>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub rotation: Option<f64>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub turn: Option<f64>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub tilt: Option<f64>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub x: Option<f64>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub y: Option<f64>,
}

/// 🩹 Sparse per-field patch over one `Puzzle5dTargetVolume` — only the named fields change.
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned)]
#[value(rename_all = "camelCase", default)]
pub struct Puzzle5dTargetVolumePatch {
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub origin: Option<[f64; 3]>,
    #[value(default, skip_serializing_if = "Option::is_none", deserialize_with = "deserialize_double_option")]
    pub orientation: Option<Option<[f64; 4]>>,
    #[value(default, skip_serializing_if = "Option::is_none", deserialize_with = "deserialize_double_option")]
    pub scale: Option<Option<Puzzle5dScale>>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub hidden: Option<bool>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub locked: Option<bool>,
}

/// 🩹 Sparse per-field patch over one `Puzzle5dKindCompatibility` — only the named fields change.
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned)]
#[value(rename_all = "camelCase", default)]
pub struct Puzzle5dKindCompatibilityPatch {
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub bidirectional: Option<bool>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub important: Option<bool>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub specificity: Option<Puzzle5dCompatSpecificity>,
}

/// 🩹 Sparse per-field patch over one `Puzzle5dMeta` — only the named fields change.
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned)]
#[value(rename_all = "camelCase", default)]
pub struct Puzzle5dMetaPatch {
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

//#endregion 🔖️Patches

//#region 🔖️Deltas
protocol::list_delta! { pub Puzzle5dGripsDelta { removal: Puzzle5dGripRemoval, insertion: Puzzle5dGripInsertion, relocation: Puzzle5dGripRelocation, modification: Puzzle5dGripModification, row: Puzzle5dGrip, patch: Puzzle5dGripPatch, list: Vec<Puzzle5dGrip>, key: String = |item| item.id.clone() } }
protocol::list_delta! { pub Puzzle5dPartsDelta { removal: Puzzle5dPartRemoval, insertion: Puzzle5dPartInsertion, relocation: Puzzle5dPartRelocation, modification: Puzzle5dPartModification, row: Puzzle5dPart, patch: Puzzle5dPartPatch, list: Vec<Puzzle5dPart>, key: String = |item| item.id.clone() } }
protocol::list_delta! { pub Puzzle5dFastenersDelta { removal: Puzzle5dFastenerRemoval, insertion: Puzzle5dFastenerInsertion, relocation: Puzzle5dFastenerRelocation, modification: Puzzle5dFastenerModification, row: Puzzle5dFastener, patch: Puzzle5dFastenerPatch, list: Vec<Puzzle5dFastener>, key: String = |item| item.id.clone() } }
protocol::list_delta! { pub Puzzle5dTargetVolumesDelta { removal: Puzzle5dTargetVolumeRemoval, insertion: Puzzle5dTargetVolumeInsertion, relocation: Puzzle5dTargetVolumeRelocation, modification: Puzzle5dTargetVolumeModification, row: Puzzle5dTargetVolume, patch: Puzzle5dTargetVolumePatch, list: Vec<Puzzle5dTargetVolume>, key: String = |item| item.id.clone() } }
protocol::list_delta! { pub Puzzle5dKindCompatibilityDelta { removal: Puzzle5dKindCompatibilityRemoval, insertion: Puzzle5dKindCompatibilityInsertion, relocation: Puzzle5dKindCompatibilityRelocation, modification: Puzzle5dKindCompatibilityModification, row: Puzzle5dKindCompatibility, patch: Puzzle5dKindCompatibilityPatch, list: Vec<Puzzle5dKindCompatibility>, key: Puzzle5dKindCompatibilityKey = |item| Puzzle5dKindCompatibilityKey { source: item.source.clone(), target: item.target.clone() } } }
//#endregion 🔖️Deltas

//#region 🔖️Algebra
/// 🕳️ Tri-state decode of every `Option<Option<T>>` patch slot: a missing key is the unchanged slot (`None`) and a PRESENT
/// `null` is the clear `Some(None)`, never the unchanged slot the blanket `Option<T>` decode would fold it into.
fn deserialize_double_option<T: semio_framework_value::FromValue>(value: semio_framework_value::DslValue) -> Result<Option<Option<T>>, semio_framework_value::ValueError> {
    <Option<T> as semio_framework_value::FromValue>::from_value(value).map(Some)
}
//#endregion 🔖️Algebra


impl RowPatch<Puzzle5dPart2d> for Puzzle5dPart2dPatch {
    fn commit_into(&self, item: &mut Puzzle5dPart2d, capability: protocol::ApplyCapability) -> protocol::MutationApplyResult<()> {
        if let Some(value) = &self.x {
            item.x = *value;
        }
        if let Some(value) = &self.y {
            item.y = *value;
        }
        if let Some(value) = &self.shape {
            item.shape = value.clone();
        }
        if let Some(value) = &self.radius {
            item.radius = *value;
        }
        if let Some(value) = &self.width {
            item.width = *value;
        }
        if let Some(value) = &self.height {
            item.height = *value;
        }
        if let Some(value) = &self.text {
            item.text = value.clone();
        }
        if let Some(value) = &self.icon_kind {
            item.icon_kind = value.clone();
        }
        if let Some(value) = &self.hidden {
            item.hidden = *value;
        }
        if let Some(value) = &self.locked {
            item.locked = *value;
        }
        Ok(())
    }
    fn absorb(&mut self, later: Self) {
        if later.x.is_some() {
            self.x = later.x;
        }
        if later.y.is_some() {
            self.y = later.y;
        }
        if later.shape.is_some() {
            self.shape = later.shape;
        }
        if later.radius.is_some() {
            self.radius = later.radius;
        }
        if later.width.is_some() {
            self.width = later.width;
        }
        if later.height.is_some() {
            self.height = later.height;
        }
        if later.text.is_some() {
            self.text = later.text;
        }
        if later.icon_kind.is_some() {
            self.icon_kind = later.icon_kind;
        }
        if later.hidden.is_some() {
            self.hidden = later.hidden;
        }
        if later.locked.is_some() {
            self.locked = later.locked;
        }
    }
    fn inverse(&self, base: &Puzzle5dPart2d) -> Self {
        Self {
            x: self.x.as_ref().map(|_| base.x),
            y: self.y.as_ref().map(|_| base.y),
            shape: self.shape.as_ref().map(|_| base.shape.clone()),
            radius: self.radius.as_ref().map(|_| base.radius),
            width: self.width.as_ref().map(|_| base.width),
            height: self.height.as_ref().map(|_| base.height),
            text: self.text.as_ref().map(|_| base.text.clone()),
            icon_kind: self.icon_kind.as_ref().map(|_| base.icon_kind.clone()),
            hidden: self.hidden.as_ref().map(|_| base.hidden),
            locked: self.locked.as_ref().map(|_| base.locked),
        }
    }
    fn is_empty(&self) -> bool {
        self.x.is_none() && self.y.is_none() && self.shape.is_none() && self.radius.is_none() && self.width.is_none() && self.height.is_none() && self.text.is_none() && self.icon_kind.is_none() && self.hidden.is_none() && self.locked.is_none()
    }
}

impl RowPatch<Puzzle5dPart3d> for Puzzle5dPart3dPatch {
    fn commit_into(&self, item: &mut Puzzle5dPart3d, capability: protocol::ApplyCapability) -> protocol::MutationApplyResult<()> {
        if let Some(value) = &self.origin {
            item.origin = *value;
        }
        if let Some(value) = &self.mesh_url {
            item.mesh_url = value.clone();
        }
        if let Some(value) = &self.orientation {
            item.orientation = *value;
        }
        if let Some(value) = &self.scale {
            item.scale = *value;
        }
        if let Some(value) = &self.label {
            item.label = value.clone();
        }
        Ok(())
    }
    fn absorb(&mut self, later: Self) {
        if later.origin.is_some() {
            self.origin = later.origin;
        }
        if later.mesh_url.is_some() {
            self.mesh_url = later.mesh_url;
        }
        if later.orientation.is_some() {
            self.orientation = later.orientation;
        }
        if later.scale.is_some() {
            self.scale = later.scale;
        }
        if later.label.is_some() {
            self.label = later.label;
        }
    }
    fn inverse(&self, base: &Puzzle5dPart3d) -> Self {
        Self {
            origin: self.origin.as_ref().map(|_| base.origin),
            mesh_url: self.mesh_url.as_ref().map(|_| base.mesh_url.clone()),
            orientation: self.orientation.as_ref().map(|_| base.orientation),
            scale: self.scale.as_ref().map(|_| base.scale),
            label: self.label.as_ref().map(|_| base.label.clone()),
        }
    }
    fn is_empty(&self) -> bool {
        self.origin.is_none() && self.mesh_url.is_none() && self.orientation.is_none() && self.scale.is_none() && self.label.is_none()
    }
}

impl RowPatch<Puzzle5dGrip2d> for Puzzle5dGrip2dPatch {
    fn commit_into(&self, item: &mut Puzzle5dGrip2d, capability: protocol::ApplyCapability) -> protocol::MutationApplyResult<()> {
        if let Some(value) = &self.angle {
            item.angle = *value;
        }
        if let Some(value) = &self.grip_kind {
            item.grip_kind = value.clone();
        }
        if let Some(value) = &self.radius {
            item.radius = *value;
        }
        Ok(())
    }
    fn absorb(&mut self, later: Self) {
        if later.angle.is_some() {
            self.angle = later.angle;
        }
        if later.grip_kind.is_some() {
            self.grip_kind = later.grip_kind;
        }
        if later.radius.is_some() {
            self.radius = later.radius;
        }
    }
    fn inverse(&self, base: &Puzzle5dGrip2d) -> Self {
        Self {
            angle: self.angle.as_ref().map(|_| base.angle),
            grip_kind: self.grip_kind.as_ref().map(|_| base.grip_kind.clone()),
            radius: self.radius.as_ref().map(|_| base.radius),
        }
    }
    fn is_empty(&self) -> bool {
        self.angle.is_none() && self.grip_kind.is_none() && self.radius.is_none()
    }
}

impl RowPatch<Puzzle5dGrip3d> for Puzzle5dGrip3dPatch {
    fn commit_into(&self, item: &mut Puzzle5dGrip3d, capability: protocol::ApplyCapability) -> protocol::MutationApplyResult<()> {
        if let Some(value) = &self.position {
            item.position = *value;
        }
        if let Some(value) = &self.direction {
            item.direction = *value;
        }
        if let Some(value) = &self.radius {
            item.radius = *value;
        }
        if let Some(value) = &self.label {
            item.label = value.clone();
        }
        Ok(())
    }
    fn absorb(&mut self, later: Self) {
        if later.position.is_some() {
            self.position = later.position;
        }
        if later.direction.is_some() {
            self.direction = later.direction;
        }
        if later.radius.is_some() {
            self.radius = later.radius;
        }
        if later.label.is_some() {
            self.label = later.label;
        }
    }
    fn inverse(&self, base: &Puzzle5dGrip3d) -> Self {
        Self {
            position: self.position.as_ref().map(|_| base.position),
            direction: self.direction.as_ref().map(|_| base.direction),
            radius: self.radius.as_ref().map(|_| base.radius),
            label: self.label.as_ref().map(|_| base.label.clone()),
        }
    }
    fn is_empty(&self) -> bool {
        self.position.is_none() && self.direction.is_none() && self.radius.is_none() && self.label.is_none()
    }
}

impl RowPatch<Puzzle5dGrip> for Puzzle5dGripPatch {
    fn commit_into(&self, item: &mut Puzzle5dGrip, capability: protocol::ApplyCapability) -> protocol::MutationApplyResult<()> {
        if let Some(value) = &self.grip_kind {
            item.grip_kind = value.clone();
        }
        if let Some(patch) = &self.grip_2d {
            patch.commit_into(&mut item.grip_2d, capability).map_err(|error| error.under(["2d"]))?;
        }
        if let Some(patch) = &self.grip_3d {
            patch.commit_into(&mut item.grip_3d, capability).map_err(|error| error.under(["3d"]))?;
        }
        Ok(())
    }
    fn absorb(&mut self, later: Self) {
        if later.grip_kind.is_some() {
            self.grip_kind = later.grip_kind;
        }
        match (&mut self.grip_2d, later.grip_2d) {
            (Some(earlier), Some(next)) => earlier.absorb(next),
            (slot @ None, next) => *slot = next,
            (Some(_), None) => {}
        }
        match (&mut self.grip_3d, later.grip_3d) {
            (Some(earlier), Some(next)) => earlier.absorb(next),
            (slot @ None, next) => *slot = next,
            (Some(_), None) => {}
        }
    }
    fn inverse(&self, base: &Puzzle5dGrip) -> Self {
        Self {
            grip_kind: self.grip_kind.as_ref().map(|_| base.grip_kind.clone()),
            grip_2d: self.grip_2d.as_ref().map(|patch| patch.inverse(&base.grip_2d)),
            grip_3d: self.grip_3d.as_ref().map(|patch| patch.inverse(&base.grip_3d)),
        }
    }
    fn is_empty(&self) -> bool {
        self.grip_kind.is_none() && self.grip_2d.as_ref().is_none_or(|patch| patch.is_empty()) && self.grip_3d.as_ref().is_none_or(|patch| patch.is_empty())
    }
}

impl RowPatch<Puzzle5dPart> for Puzzle5dPartPatch {
    fn commit_into(&self, item: &mut Puzzle5dPart, capability: protocol::ApplyCapability) -> protocol::MutationApplyResult<()> {
        if let Some(value) = &self.part_kind {
            item.part_kind = value.clone();
        }
        if let Some(value) = &self.anchor {
            item.anchor = *value;
        }
        if let Some(patch) = &self.part_2d {
            patch.commit_into(&mut item.part_2d, capability).map_err(|error| error.under(["2d"]))?;
        }
        if let Some(patch) = &self.part_3d {
            patch.commit_into(&mut item.part_3d, capability).map_err(|error| error.under(["3d"]))?;
        }
        if let Some(delta) = &self.grips {
            item.grips = delta.commit_onto(&item.grips, capability).map_err(|error| error.under(["grips"]))?;
        }
        Ok(())
    }
    fn absorb(&mut self, later: Self) {
        if later.part_kind.is_some() {
            self.part_kind = later.part_kind;
        }
        if later.anchor.is_some() {
            self.anchor = later.anchor;
        }
        match (&mut self.part_2d, later.part_2d) {
            (Some(earlier), Some(next)) => earlier.absorb(next),
            (slot @ None, next) => *slot = next,
            (Some(_), None) => {}
        }
        match (&mut self.part_3d, later.part_3d) {
            (Some(earlier), Some(next)) => earlier.absorb(next),
            (slot @ None, next) => *slot = next,
            (Some(_), None) => {}
        }
        match (&mut self.grips, later.grips) {
            (Some(earlier), Some(next)) => earlier.absorb(next),
            (slot @ None, next) => *slot = next,
            (Some(_), None) => {}
        }
    }
    fn inverse(&self, base: &Puzzle5dPart) -> Self {
        Self {
            part_kind: self.part_kind.as_ref().map(|_| base.part_kind.clone()),
            anchor: self.anchor.as_ref().map(|_| base.anchor),
            part_2d: self.part_2d.as_ref().map(|patch| patch.inverse(&base.part_2d)),
            part_3d: self.part_3d.as_ref().map(|patch| patch.inverse(&base.part_3d)),
            grips: self.grips.as_ref().map(|delta| delta.inverse(&base.grips)),
        }
    }
    fn is_empty(&self) -> bool {
        self.part_kind.is_none() && self.anchor.is_none() && self.part_2d.as_ref().is_none_or(|patch| patch.is_empty()) && self.part_3d.as_ref().is_none_or(|patch| patch.is_empty()) && self.grips.as_ref().is_none_or(Puzzle5dGripsDelta::is_empty)
    }
}

impl RowPatch<Puzzle5dFastener> for Puzzle5dFastenerPatch {
    fn commit_into(&self, item: &mut Puzzle5dFastener, capability: protocol::ApplyCapability) -> protocol::MutationApplyResult<()> {
        if let Some(value) = &self.source {
            item.source = value.clone();
        }
        if let Some(value) = &self.target {
            item.target = value.clone();
        }
        if let Some(value) = &self.fastener_kind {
            item.fastener_kind = value.clone();
        }
        if let Some(value) = &self.gap {
            item.gap = *value;
        }
        if let Some(value) = &self.shift {
            item.shift = *value;
        }
        if let Some(value) = &self.rise {
            item.rise = *value;
        }
        if let Some(value) = &self.rotation {
            item.rotation = *value;
        }
        if let Some(value) = &self.turn {
            item.turn = *value;
        }
        if let Some(value) = &self.tilt {
            item.tilt = *value;
        }
        if let Some(value) = &self.x {
            item.x = *value;
        }
        if let Some(value) = &self.y {
            item.y = *value;
        }
        Ok(())
    }
    fn absorb(&mut self, later: Self) {
        if later.source.is_some() {
            self.source = later.source;
        }
        if later.target.is_some() {
            self.target = later.target;
        }
        if later.fastener_kind.is_some() {
            self.fastener_kind = later.fastener_kind;
        }
        if later.gap.is_some() {
            self.gap = later.gap;
        }
        if later.shift.is_some() {
            self.shift = later.shift;
        }
        if later.rise.is_some() {
            self.rise = later.rise;
        }
        if later.rotation.is_some() {
            self.rotation = later.rotation;
        }
        if later.turn.is_some() {
            self.turn = later.turn;
        }
        if later.tilt.is_some() {
            self.tilt = later.tilt;
        }
        if later.x.is_some() {
            self.x = later.x;
        }
        if later.y.is_some() {
            self.y = later.y;
        }
    }
    fn inverse(&self, base: &Puzzle5dFastener) -> Self {
        Self {
            source: self.source.as_ref().map(|_| base.source.clone()),
            target: self.target.as_ref().map(|_| base.target.clone()),
            fastener_kind: self.fastener_kind.as_ref().map(|_| base.fastener_kind.clone()),
            gap: self.gap.as_ref().map(|_| base.gap),
            shift: self.shift.as_ref().map(|_| base.shift),
            rise: self.rise.as_ref().map(|_| base.rise),
            rotation: self.rotation.as_ref().map(|_| base.rotation),
            turn: self.turn.as_ref().map(|_| base.turn),
            tilt: self.tilt.as_ref().map(|_| base.tilt),
            x: self.x.as_ref().map(|_| base.x),
            y: self.y.as_ref().map(|_| base.y),
        }
    }
    fn is_empty(&self) -> bool {
        self.source.is_none() && self.target.is_none() && self.fastener_kind.is_none() && self.gap.is_none() && self.shift.is_none() && self.rise.is_none() && self.rotation.is_none() && self.turn.is_none() && self.tilt.is_none() && self.x.is_none() && self.y.is_none()
    }
}

impl RowPatch<Puzzle5dTargetVolume> for Puzzle5dTargetVolumePatch {
    fn commit_into(&self, item: &mut Puzzle5dTargetVolume, capability: protocol::ApplyCapability) -> protocol::MutationApplyResult<()> {
        if let Some(value) = &self.origin {
            item.origin = *value;
        }
        if let Some(value) = &self.orientation {
            item.orientation = *value;
        }
        if let Some(value) = &self.scale {
            item.scale = *value;
        }
        if let Some(value) = &self.hidden {
            item.hidden = *value;
        }
        if let Some(value) = &self.locked {
            item.locked = *value;
        }
        Ok(())
    }
    fn absorb(&mut self, later: Self) {
        if later.origin.is_some() {
            self.origin = later.origin;
        }
        if later.orientation.is_some() {
            self.orientation = later.orientation;
        }
        if later.scale.is_some() {
            self.scale = later.scale;
        }
        if later.hidden.is_some() {
            self.hidden = later.hidden;
        }
        if later.locked.is_some() {
            self.locked = later.locked;
        }
    }
    fn inverse(&self, base: &Puzzle5dTargetVolume) -> Self {
        Self {
            origin: self.origin.as_ref().map(|_| base.origin),
            orientation: self.orientation.as_ref().map(|_| base.orientation),
            scale: self.scale.as_ref().map(|_| base.scale),
            hidden: self.hidden.as_ref().map(|_| base.hidden),
            locked: self.locked.as_ref().map(|_| base.locked),
        }
    }
    fn is_empty(&self) -> bool {
        self.origin.is_none() && self.orientation.is_none() && self.scale.is_none() && self.hidden.is_none() && self.locked.is_none()
    }
}

impl RowPatch<Puzzle5dKindCompatibility> for Puzzle5dKindCompatibilityPatch {
    fn commit_into(&self, item: &mut Puzzle5dKindCompatibility, capability: protocol::ApplyCapability) -> protocol::MutationApplyResult<()> {
        if let Some(value) = &self.bidirectional {
            item.bidirectional = *value;
        }
        if let Some(value) = &self.important {
            item.important = *value;
        }
        if let Some(value) = &self.specificity {
            item.specificity = *value;
        }
        Ok(())
    }
    fn absorb(&mut self, later: Self) {
        if later.bidirectional.is_some() {
            self.bidirectional = later.bidirectional;
        }
        if later.important.is_some() {
            self.important = later.important;
        }
        if later.specificity.is_some() {
            self.specificity = later.specificity;
        }
    }
    fn inverse(&self, base: &Puzzle5dKindCompatibility) -> Self {
        Self {
            bidirectional: self.bidirectional.as_ref().map(|_| base.bidirectional),
            important: self.important.as_ref().map(|_| base.important),
            specificity: self.specificity.as_ref().map(|_| base.specificity),
        }
    }
    fn is_empty(&self) -> bool {
        self.bidirectional.is_none() && self.important.is_none() && self.specificity.is_none()
    }
}

impl RowPatch<Puzzle5dMeta> for Puzzle5dMetaPatch {
    fn commit_into(&self, item: &mut Puzzle5dMeta, capability: protocol::ApplyCapability) -> protocol::MutationApplyResult<()> {
        if let Some(value) = &self.description {
            item.description = value.clone();
        }
        Ok(())
    }
    fn absorb(&mut self, later: Self) {
        if later.description.is_some() {
            self.description = later.description;
        }
    }
    fn inverse(&self, base: &Puzzle5dMeta) -> Self {
        Self {
            description: self.description.as_ref().map(|_| base.description.clone()),
        }
    }
    fn is_empty(&self) -> bool {
        self.description.is_none()
    }
}

impl MutationDiff<Puzzle5dSnapshot> for Puzzle5dDiff {
    fn apply(&self, base: &Puzzle5dSnapshot, capability: protocol::ApplyCapability) -> protocol::MutationApplyResult<Puzzle5dSnapshot> {
        let mut item = match &self.artifact {
            Some(artifact) => artifact.to_snapshot(),
            None => base.clone(),
        };
        if let Some(value) = &self.schema {
            item.schema = value.clone();
        }
        if let Some(value) = &self.domain {
            item.domain = value.clone();
        }
        if let Some(value) = &self.label {
            item.label = value.clone();
        }
        if let Some(patch) = &self.meta {
            patch.commit_into(&mut item.meta, capability).map_err(|error| error.under(["meta"]))?;
        }
        if let Some(value) = &self.kind_catalogs {
            item.kind_catalogs = value.clone();
        }
        if let Some(value) = &self.kind_catalogs_extra {
            item.kind_catalogs_extra = value.clone();
        }
        if let Some(delta) = &self.kind_compatibility {
            item.kind_compatibility = delta.commit_onto(&item.kind_compatibility, capability).map_err(|error| error.under(["kindCompatibility"]))?;
        }
        if let Some(delta) = &self.parts {
            item.parts = delta.commit_onto(&item.parts, capability).map_err(|error| error.under(["parts"]))?;
        }
        if let Some(delta) = &self.fasteners {
            item.fasteners = delta.commit_onto(&item.fasteners, capability).map_err(|error| error.under(["fasteners"]))?;
        }
        if let Some(delta) = &self.target_volumes {
            item.target_volumes = delta.commit_onto(&item.target_volumes, capability).map_err(|error| error.under(["targetVolumes"]))?;
        }
        Ok(item)
    }
    fn absorb(&mut self, later: Self) {
        if later.artifact.is_some() {
            *self = later;
            return;
        }
        if later.schema.is_some() {
            self.schema = later.schema;
        }
        if later.domain.is_some() {
            self.domain = later.domain;
        }
        if later.label.is_some() {
            self.label = later.label;
        }
        match (&mut self.meta, later.meta) {
            (Some(earlier), Some(next)) => earlier.absorb(next),
            (slot @ None, next) => *slot = next,
            (Some(_), None) => {}
        }
        if later.kind_catalogs.is_some() {
            self.kind_catalogs = later.kind_catalogs;
        }
        if later.kind_catalogs_extra.is_some() {
            self.kind_catalogs_extra = later.kind_catalogs_extra;
        }
        match (&mut self.kind_compatibility, later.kind_compatibility) {
            (Some(earlier), Some(next)) => earlier.absorb(next),
            (slot @ None, next) => *slot = next,
            (Some(_), None) => {}
        }
        match (&mut self.parts, later.parts) {
            (Some(earlier), Some(next)) => earlier.absorb(next),
            (slot @ None, next) => *slot = next,
            (Some(_), None) => {}
        }
        match (&mut self.fasteners, later.fasteners) {
            (Some(earlier), Some(next)) => earlier.absorb(next),
            (slot @ None, next) => *slot = next,
            (Some(_), None) => {}
        }
        match (&mut self.target_volumes, later.target_volumes) {
            (Some(earlier), Some(next)) => earlier.absorb(next),
            (slot @ None, next) => *slot = next,
            (Some(_), None) => {}
        }
    }
}

impl DiffAlgebra<Puzzle5dSnapshot> for Puzzle5dDiff {
    fn inverse(&self, base: &Puzzle5dSnapshot) -> Self {
        if self.artifact.is_some() {
            return Self { artifact: Some(Box::new(Puzzle5dArtifact::from_snapshot(base.clone()))), ..Default::default() };
        }
        Self {
            artifact: None,
            schema: self.schema.as_ref().map(|_| base.schema.clone()),
            domain: self.domain.as_ref().map(|_| base.domain.clone()),
            label: self.label.as_ref().map(|_| base.label.clone()),
            meta: self.meta.as_ref().map(|patch| patch.inverse(&base.meta)),
            kind_catalogs: self.kind_catalogs.as_ref().map(|_| base.kind_catalogs.clone()),
            kind_catalogs_extra: self.kind_catalogs_extra.as_ref().map(|_| base.kind_catalogs_extra.clone()),
            kind_compatibility: self.kind_compatibility.as_ref().map(|delta| delta.inverse(&base.kind_compatibility)),
            parts: self.parts.as_ref().map(|delta| delta.inverse(&base.parts)),
            fasteners: self.fasteners.as_ref().map(|delta| delta.inverse(&base.fasteners)),
            target_volumes: self.target_volumes.as_ref().map(|delta| delta.inverse(&base.target_volumes)),
        }
    }
    fn is_empty(&self) -> bool {
        self.artifact.is_none() && self.schema.is_none() && self.domain.is_none() && self.label.is_none() && self.meta.as_ref().is_none_or(|patch| patch.is_empty()) && self.kind_catalogs.is_none() && self.kind_catalogs_extra.is_none() && self.kind_compatibility.as_ref().is_none_or(Puzzle5dKindCompatibilityDelta::is_empty) && self.parts.as_ref().is_none_or(Puzzle5dPartsDelta::is_empty) && self.fasteners.as_ref().is_none_or(Puzzle5dFastenersDelta::is_empty) && self.target_volumes.as_ref().is_none_or(Puzzle5dTargetVolumesDelta::is_empty)
    }
}
