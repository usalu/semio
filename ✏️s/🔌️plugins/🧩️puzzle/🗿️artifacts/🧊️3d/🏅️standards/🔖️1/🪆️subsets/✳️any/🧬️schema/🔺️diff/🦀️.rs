//! 🧬️ Puzzle3d diff schema — sparse typed delta over the artifact: per-field entity patches and id-keyed collection deltas.

use crate::standards::v1::subsets::any::schema::Puzzle3dArtifact;
use crate::{Puzzle3dAttraction, Puzzle3dCompatSpecificity, Puzzle3dKindCatalogs, Puzzle3dKindCompatibility, Puzzle3dMeta, Puzzle3dObject, Puzzle3dObjectAnchor, Puzzle3dReference, Puzzle3dReferenceSource, Puzzle3dScale, Puzzle3dTargetVolume, Puzzle3dVortex};
use crate::Puzzle3dSnapshot;
use ::semio_framework_schema::ArtifactSchema;
use protocol::{DiffAlgebra, MutationDiff};
use protocol::list_delta::RowPatch;

//#region 🔖️Diff
/// 🔺️ Sparse typed delta for the puzzle3d artifact: per-field entity patches and id-keyed collection deltas.
#[derive(Clone, Debug, Default, PartialEq, ArtifactSchema, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord)]
#[value(rename_all = "camelCase", default)]
#[artifact_schema(id = "s.puzzle.puzzle3d")]
pub struct Puzzle3dDiff {
    #[state(artifact)]
    pub artifact: Option<Box<Puzzle3dArtifact>>,
    #[state(artifact)]
    pub schema: Option<String>,
    #[state(artifact)]
    pub domain: Option<String>,
    #[state(artifact)]
    pub meta: Option<Puzzle3dMetaPatch>,
    #[state(artifact)]
    pub objects: Option<Puzzle3dObjectsDelta>,
    #[state(artifact)]
    pub attractions: Option<Puzzle3dAttractionsDelta>,
    #[state(artifact)]
    pub target_volumes: Option<Puzzle3dTargetVolumesDelta>,
    #[state(artifact)]
    pub references: Option<Puzzle3dReferencesDelta>,
}
//#endregion 🔖️Diff

//#region 🔖️Patches
/// 🔑️ The identity of one kind-compatibility row: the pair of kinds it links.
#[derive(Clone, Debug, PartialEq, Eq, Hash, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord)]
#[value(rename_all = "camelCase")]
pub struct Puzzle3dKindCompatibilityKey {
    pub source: String,
    pub target: String,
}

/// 🩹 Sparse per-field patch over one `Puzzle3dVortex` — only the named fields change.
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord)]
#[value(rename_all = "camelCase", default)]
pub struct Puzzle3dVortexPatch {
    #[value(default, skip_serializing_if = "Option::is_none", deserialize_with = "deserialize_double_option")]
    pub vortex_kind: Option<Option<String>>,
    #[value(default, skip_serializing_if = "Option::is_none", deserialize_with = "deserialize_double_option")]
    pub label: Option<Option<String>>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub position: Option<[f64; 3]>,
    #[value(default, skip_serializing_if = "Option::is_none", deserialize_with = "deserialize_double_option")]
    pub direction: Option<Option<[f64; 3]>>,
    #[value(default, skip_serializing_if = "Option::is_none", deserialize_with = "deserialize_double_option")]
    pub radius: Option<Option<f64>>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub hidden: Option<bool>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub locked: Option<bool>,
}

/// 🩹 Sparse per-field patch over one `Puzzle3dObject` — only the named fields change.
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord)]
#[value(rename_all = "camelCase", default)]
pub struct Puzzle3dObjectPatch {
    #[value(default, skip_serializing_if = "Option::is_none", deserialize_with = "deserialize_double_option")]
    pub label: Option<Option<String>>,
    #[value(default, skip_serializing_if = "Option::is_none", deserialize_with = "deserialize_double_option")]
    pub object_kind: Option<Option<String>>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub anchor: Option<Puzzle3dObjectAnchor>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub origin: Option<[f64; 3]>,
    #[value(default, skip_serializing_if = "Option::is_none", deserialize_with = "deserialize_double_option")]
    pub orientation: Option<Option<[f64; 4]>>,
    #[value(default, skip_serializing_if = "Option::is_none", deserialize_with = "deserialize_double_option")]
    pub scale: Option<Option<Puzzle3dScale>>,
    #[value(default, skip_serializing_if = "Option::is_none", deserialize_with = "deserialize_double_option")]
    pub mesh_url: Option<Option<String>>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub vortices: Option<Puzzle3dVorticesDelta>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub hidden: Option<bool>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub locked: Option<bool>,
}

/// 🩹 Sparse per-field patch over one `Puzzle3dAttraction` — only the named fields change.
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord)]
#[value(rename_all = "camelCase", default)]
pub struct Puzzle3dAttractionPatch {
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub attracting: Option<String>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub attracted: Option<String>,
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

/// 🩹 Sparse per-field patch over one `Puzzle3dTargetVolume` — only the named fields change.
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord)]
#[value(rename_all = "camelCase", default)]
pub struct Puzzle3dTargetVolumePatch {
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub origin: Option<[f64; 3]>,
    #[value(default, skip_serializing_if = "Option::is_none", deserialize_with = "deserialize_double_option")]
    pub orientation: Option<Option<[f64; 4]>>,
    #[value(default, skip_serializing_if = "Option::is_none", deserialize_with = "deserialize_double_option")]
    pub scale: Option<Option<Puzzle3dScale>>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub hidden: Option<bool>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub locked: Option<bool>,
}

/// 🩹 Sparse per-field patch over one `Puzzle3dReference` — only the named fields change.
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord)]
#[value(rename_all = "camelCase", default)]
pub struct Puzzle3dReferencePatch {
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub source: Option<Puzzle3dReferenceSource>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub origin: Option<[f64; 3]>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub width_world: Option<f64>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub locked: Option<bool>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub hidden: Option<bool>,
}

/// 🩹 Sparse per-field patch over one `Puzzle3dKindCompatibility` — only the named fields change.
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord)]
#[value(rename_all = "camelCase", default)]
pub struct Puzzle3dKindCompatibilityPatch {
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub bidirectional: Option<bool>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub important: Option<bool>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub specificity: Option<Puzzle3dCompatSpecificity>,
}

/// 🩹 Sparse per-field patch over one `Puzzle3dMeta` — only the named fields change.
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord)]
#[value(rename_all = "camelCase", default)]
pub struct Puzzle3dMetaPatch {
    #[value(default, skip_serializing_if = "Option::is_none", deserialize_with = "deserialize_double_option")]
    pub kind_catalogs: Option<Option<Puzzle3dKindCatalogs>>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub kind_compatibility: Option<Puzzle3dKindCompatibilityDelta>,
}

//#endregion 🔖️Patches

//#region 🔖️Deltas
protocol::list_delta! { pub Puzzle3dVorticesDelta { removal: Puzzle3dVortexRemoval, insertion: Puzzle3dVortexInsertion, relocation: Puzzle3dVortexRelocation, modification: Puzzle3dVortexModification, row: Puzzle3dVortex, patch: Puzzle3dVortexPatch, list: Vec<Puzzle3dVortex>, key: String = |item| item.id.clone() } }
protocol::list_delta! { pub Puzzle3dObjectsDelta { removal: Puzzle3dObjectRemoval, insertion: Puzzle3dObjectInsertion, relocation: Puzzle3dObjectRelocation, modification: Puzzle3dObjectModification, row: Puzzle3dObject, patch: Puzzle3dObjectPatch, list: Vec<Puzzle3dObject>, key: String = |item| item.id.clone() } }
protocol::list_delta! { pub Puzzle3dAttractionsDelta { removal: Puzzle3dAttractionRemoval, insertion: Puzzle3dAttractionInsertion, relocation: Puzzle3dAttractionRelocation, modification: Puzzle3dAttractionModification, row: Puzzle3dAttraction, patch: Puzzle3dAttractionPatch, list: Vec<Puzzle3dAttraction>, key: String = |item| item.id.clone() } }
protocol::list_delta! { pub Puzzle3dTargetVolumesDelta { removal: Puzzle3dTargetVolumeRemoval, insertion: Puzzle3dTargetVolumeInsertion, relocation: Puzzle3dTargetVolumeRelocation, modification: Puzzle3dTargetVolumeModification, row: Puzzle3dTargetVolume, patch: Puzzle3dTargetVolumePatch, list: Vec<Puzzle3dTargetVolume>, key: String = |item| item.id.clone() } }
protocol::list_delta! { pub Puzzle3dReferencesDelta { removal: Puzzle3dReferenceRemoval, insertion: Puzzle3dReferenceInsertion, relocation: Puzzle3dReferenceRelocation, modification: Puzzle3dReferenceModification, row: Puzzle3dReference, patch: Puzzle3dReferencePatch, list: Vec<Puzzle3dReference>, key: String = |item| item.id.clone() } }
protocol::list_delta! { pub Puzzle3dKindCompatibilityDelta { removal: Puzzle3dKindCompatibilityRemoval, insertion: Puzzle3dKindCompatibilityInsertion, relocation: Puzzle3dKindCompatibilityRelocation, modification: Puzzle3dKindCompatibilityModification, row: Puzzle3dKindCompatibility, patch: Puzzle3dKindCompatibilityPatch, list: Vec<Puzzle3dKindCompatibility>, key: Puzzle3dKindCompatibilityKey = |item| Puzzle3dKindCompatibilityKey { source: item.source.clone(), target: item.target.clone() } } }
//#endregion 🔖️Deltas

//#region 🔖️Algebra
/// 🕳️ Tri-state decode of every `Option<Option<T>>` patch slot: a missing key is the unchanged slot (`None`) and a PRESENT
/// `null` is the clear `Some(None)`, never the unchanged slot the blanket `Option<T>` decode would fold it into.
fn deserialize_double_option<T: semio_framework_value::FromValue>(value: semio_framework_value::DslValue) -> Result<Option<Option<T>>, semio_framework_value::ValueError> {
    <Option<T> as semio_framework_value::FromValue>::from_value(value).map(Some)
}
//#endregion 🔖️Algebra


impl RowPatch<Puzzle3dVortex> for Puzzle3dVortexPatch {
    fn commit_into(&self, item: &mut Puzzle3dVortex, capability: protocol::ApplyCapability) -> protocol::MutationApplyResult<()> {
        if let Some(value) = &self.vortex_kind {
            item.vortex_kind = value.clone();
        }
        if let Some(value) = &self.label {
            item.label = value.clone();
        }
        if let Some(value) = &self.position {
            item.position = *value;
        }
        if let Some(value) = &self.direction {
            item.direction = *value;
        }
        if let Some(value) = &self.radius {
            item.radius = *value;
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
        if later.vortex_kind.is_some() {
            self.vortex_kind = later.vortex_kind;
        }
        if later.label.is_some() {
            self.label = later.label;
        }
        if later.position.is_some() {
            self.position = later.position;
        }
        if later.direction.is_some() {
            self.direction = later.direction;
        }
        if later.radius.is_some() {
            self.radius = later.radius;
        }
        if later.hidden.is_some() {
            self.hidden = later.hidden;
        }
        if later.locked.is_some() {
            self.locked = later.locked;
        }
    }
    fn inverse(&self, base: &Puzzle3dVortex) -> Self {
        Self {
            vortex_kind: self.vortex_kind.as_ref().map(|_| base.vortex_kind.clone()),
            label: self.label.as_ref().map(|_| base.label.clone()),
            position: self.position.as_ref().map(|_| base.position),
            direction: self.direction.as_ref().map(|_| base.direction),
            radius: self.radius.as_ref().map(|_| base.radius),
            hidden: self.hidden.as_ref().map(|_| base.hidden),
            locked: self.locked.as_ref().map(|_| base.locked),
        }
    }
    fn is_empty(&self) -> bool {
        self.vortex_kind.is_none() && self.label.is_none() && self.position.is_none() && self.direction.is_none() && self.radius.is_none() && self.hidden.is_none() && self.locked.is_none()
    }
}

impl RowPatch<Puzzle3dObject> for Puzzle3dObjectPatch {
    fn commit_into(&self, item: &mut Puzzle3dObject, capability: protocol::ApplyCapability) -> protocol::MutationApplyResult<()> {
        if let Some(value) = &self.label {
            item.label = value.clone();
        }
        if let Some(value) = &self.object_kind {
            item.object_kind = value.clone();
        }
        if let Some(value) = &self.anchor {
            item.anchor = *value;
        }
        if let Some(value) = &self.origin {
            item.origin = *value;
        }
        if let Some(value) = &self.orientation {
            item.orientation = *value;
        }
        if let Some(value) = &self.scale {
            item.scale = *value;
        }
        if let Some(value) = &self.mesh_url {
            item.mesh_url = value.clone();
        }
        if let Some(delta) = &self.vortices {
            item.vortices = delta.commit_onto(&item.vortices, capability).map_err(|error| error.under(["vortices"]))?;
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
        if later.label.is_some() {
            self.label = later.label;
        }
        if later.object_kind.is_some() {
            self.object_kind = later.object_kind;
        }
        if later.anchor.is_some() {
            self.anchor = later.anchor;
        }
        if later.origin.is_some() {
            self.origin = later.origin;
        }
        if later.orientation.is_some() {
            self.orientation = later.orientation;
        }
        if later.scale.is_some() {
            self.scale = later.scale;
        }
        if later.mesh_url.is_some() {
            self.mesh_url = later.mesh_url;
        }
        match (&mut self.vortices, later.vortices) {
            (Some(earlier), Some(next)) => earlier.absorb(next),
            (slot @ None, next) => *slot = next,
            (Some(_), None) => {}
        }
        if later.hidden.is_some() {
            self.hidden = later.hidden;
        }
        if later.locked.is_some() {
            self.locked = later.locked;
        }
    }
    fn inverse(&self, base: &Puzzle3dObject) -> Self {
        Self {
            label: self.label.as_ref().map(|_| base.label.clone()),
            object_kind: self.object_kind.as_ref().map(|_| base.object_kind.clone()),
            anchor: self.anchor.as_ref().map(|_| base.anchor),
            origin: self.origin.as_ref().map(|_| base.origin),
            orientation: self.orientation.as_ref().map(|_| base.orientation),
            scale: self.scale.as_ref().map(|_| base.scale),
            mesh_url: self.mesh_url.as_ref().map(|_| base.mesh_url.clone()),
            vortices: self.vortices.as_ref().map(|delta| delta.inverse(&base.vortices)),
            hidden: self.hidden.as_ref().map(|_| base.hidden),
            locked: self.locked.as_ref().map(|_| base.locked),
        }
    }
    fn is_empty(&self) -> bool {
        self.label.is_none() && self.object_kind.is_none() && self.anchor.is_none() && self.origin.is_none() && self.orientation.is_none() && self.scale.is_none() && self.mesh_url.is_none() && self.vortices.as_ref().is_none_or(Puzzle3dVorticesDelta::is_empty) && self.hidden.is_none() && self.locked.is_none()
    }
}

impl RowPatch<Puzzle3dAttraction> for Puzzle3dAttractionPatch {
    fn commit_into(&self, item: &mut Puzzle3dAttraction, capability: protocol::ApplyCapability) -> protocol::MutationApplyResult<()> {
        if let Some(value) = &self.attracting {
            item.attracting = value.clone();
        }
        if let Some(value) = &self.attracted {
            item.attracted = value.clone();
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
        if later.attracting.is_some() {
            self.attracting = later.attracting;
        }
        if later.attracted.is_some() {
            self.attracted = later.attracted;
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
    fn inverse(&self, base: &Puzzle3dAttraction) -> Self {
        Self {
            attracting: self.attracting.as_ref().map(|_| base.attracting.clone()),
            attracted: self.attracted.as_ref().map(|_| base.attracted.clone()),
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
        self.attracting.is_none() && self.attracted.is_none() && self.gap.is_none() && self.shift.is_none() && self.rise.is_none() && self.rotation.is_none() && self.turn.is_none() && self.tilt.is_none() && self.x.is_none() && self.y.is_none()
    }
}

impl RowPatch<Puzzle3dTargetVolume> for Puzzle3dTargetVolumePatch {
    fn commit_into(&self, item: &mut Puzzle3dTargetVolume, capability: protocol::ApplyCapability) -> protocol::MutationApplyResult<()> {
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
    fn inverse(&self, base: &Puzzle3dTargetVolume) -> Self {
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

impl RowPatch<Puzzle3dReference> for Puzzle3dReferencePatch {
    fn commit_into(&self, item: &mut Puzzle3dReference, capability: protocol::ApplyCapability) -> protocol::MutationApplyResult<()> {
        if let Some(value) = &self.source {
            item.source = value.clone();
        }
        if let Some(value) = &self.origin {
            item.origin = *value;
        }
        if let Some(value) = &self.width_world {
            item.width_world = *value;
        }
        if let Some(value) = &self.locked {
            item.locked = *value;
        }
        if let Some(value) = &self.hidden {
            item.hidden = *value;
        }
        Ok(())
    }
    fn absorb(&mut self, later: Self) {
        if later.source.is_some() {
            self.source = later.source;
        }
        if later.origin.is_some() {
            self.origin = later.origin;
        }
        if later.width_world.is_some() {
            self.width_world = later.width_world;
        }
        if later.locked.is_some() {
            self.locked = later.locked;
        }
        if later.hidden.is_some() {
            self.hidden = later.hidden;
        }
    }
    fn inverse(&self, base: &Puzzle3dReference) -> Self {
        Self {
            source: self.source.as_ref().map(|_| base.source.clone()),
            origin: self.origin.as_ref().map(|_| base.origin),
            width_world: self.width_world.as_ref().map(|_| base.width_world),
            locked: self.locked.as_ref().map(|_| base.locked),
            hidden: self.hidden.as_ref().map(|_| base.hidden),
        }
    }
    fn is_empty(&self) -> bool {
        self.source.is_none() && self.origin.is_none() && self.width_world.is_none() && self.locked.is_none() && self.hidden.is_none()
    }
}

impl RowPatch<Puzzle3dKindCompatibility> for Puzzle3dKindCompatibilityPatch {
    fn commit_into(&self, item: &mut Puzzle3dKindCompatibility, capability: protocol::ApplyCapability) -> protocol::MutationApplyResult<()> {
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
    fn inverse(&self, base: &Puzzle3dKindCompatibility) -> Self {
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

impl RowPatch<Puzzle3dMeta> for Puzzle3dMetaPatch {
    fn commit_into(&self, item: &mut Puzzle3dMeta, capability: protocol::ApplyCapability) -> protocol::MutationApplyResult<()> {
        if let Some(value) = &self.kind_catalogs {
            item.kind_catalogs = value.clone();
        }
        if let Some(delta) = &self.kind_compatibility {
            item.kind_compatibility = delta.commit_onto(&item.kind_compatibility, capability).map_err(|error| error.under(["kindCompatibility"]))?;
        }
        Ok(())
    }
    fn absorb(&mut self, later: Self) {
        if later.kind_catalogs.is_some() {
            self.kind_catalogs = later.kind_catalogs;
        }
        match (&mut self.kind_compatibility, later.kind_compatibility) {
            (Some(earlier), Some(next)) => earlier.absorb(next),
            (slot @ None, next) => *slot = next,
            (Some(_), None) => {}
        }
    }
    fn inverse(&self, base: &Puzzle3dMeta) -> Self {
        Self {
            kind_catalogs: self.kind_catalogs.as_ref().map(|_| base.kind_catalogs.clone()),
            kind_compatibility: self.kind_compatibility.as_ref().map(|delta| delta.inverse(&base.kind_compatibility)),
        }
    }
    fn is_empty(&self) -> bool {
        self.kind_catalogs.is_none() && self.kind_compatibility.as_ref().is_none_or(Puzzle3dKindCompatibilityDelta::is_empty)
    }
}

impl MutationDiff<Puzzle3dSnapshot> for Puzzle3dDiff {
    fn apply(&self, base: &Puzzle3dSnapshot, capability: protocol::ApplyCapability) -> protocol::MutationApplyResult<Puzzle3dSnapshot> {
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
        if let Some(patch) = &self.meta {
            patch.commit_into(&mut item.meta, capability).map_err(|error| error.under(["meta"]))?;
        }
        if let Some(delta) = &self.objects {
            item.objects = delta.commit_onto(&item.objects, capability).map_err(|error| error.under(["objects"]))?;
        }
        if let Some(delta) = &self.attractions {
            item.attractions = delta.commit_onto(&item.attractions, capability).map_err(|error| error.under(["attractions"]))?;
        }
        if let Some(delta) = &self.target_volumes {
            item.target_volumes = delta.commit_onto(&item.target_volumes, capability).map_err(|error| error.under(["targetVolumes"]))?;
        }
        if let Some(delta) = &self.references {
            item.references = delta.commit_onto(&item.references, capability).map_err(|error| error.under(["references"]))?;
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
        match (&mut self.meta, later.meta) {
            (Some(earlier), Some(next)) => earlier.absorb(next),
            (slot @ None, next) => *slot = next,
            (Some(_), None) => {}
        }
        match (&mut self.objects, later.objects) {
            (Some(earlier), Some(next)) => earlier.absorb(next),
            (slot @ None, next) => *slot = next,
            (Some(_), None) => {}
        }
        match (&mut self.attractions, later.attractions) {
            (Some(earlier), Some(next)) => earlier.absorb(next),
            (slot @ None, next) => *slot = next,
            (Some(_), None) => {}
        }
        match (&mut self.target_volumes, later.target_volumes) {
            (Some(earlier), Some(next)) => earlier.absorb(next),
            (slot @ None, next) => *slot = next,
            (Some(_), None) => {}
        }
        match (&mut self.references, later.references) {
            (Some(earlier), Some(next)) => earlier.absorb(next),
            (slot @ None, next) => *slot = next,
            (Some(_), None) => {}
        }
    }
}

impl DiffAlgebra<Puzzle3dSnapshot> for Puzzle3dDiff {
    fn inverse(&self, base: &Puzzle3dSnapshot) -> Self {
        if self.artifact.is_some() {
            return Self { artifact: Some(Box::new(Puzzle3dArtifact::from_snapshot(base.clone()))), ..Default::default() };
        }
        Self {
            artifact: None,
            schema: self.schema.as_ref().map(|_| base.schema.clone()),
            domain: self.domain.as_ref().map(|_| base.domain.clone()),
            meta: self.meta.as_ref().map(|patch| patch.inverse(&base.meta)),
            objects: self.objects.as_ref().map(|delta| delta.inverse(&base.objects)),
            attractions: self.attractions.as_ref().map(|delta| delta.inverse(&base.attractions)),
            target_volumes: self.target_volumes.as_ref().map(|delta| delta.inverse(&base.target_volumes)),
            references: self.references.as_ref().map(|delta| delta.inverse(&base.references)),
        }
    }
    fn is_empty(&self) -> bool {
        self.artifact.is_none() && self.schema.is_none() && self.domain.is_none() && self.meta.as_ref().is_none_or(|patch| patch.is_empty()) && self.objects.as_ref().is_none_or(Puzzle3dObjectsDelta::is_empty) && self.attractions.as_ref().is_none_or(Puzzle3dAttractionsDelta::is_empty) && self.target_volumes.as_ref().is_none_or(Puzzle3dTargetVolumesDelta::is_empty) && self.references.as_ref().is_none_or(Puzzle3dReferencesDelta::is_empty)
    }
}
