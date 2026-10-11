//! 🧬️ Puzzle2d diff schema — sparse typed delta over the artifact: per-field entity patches and id-keyed collection deltas.

use crate::standards::v1::subsets::any::schema::Puzzle2dArtifact;
use crate::{Puzzle2dCamera, Puzzle2dCompatSpecificity, Puzzle2dEdge, Puzzle2dHandle, Puzzle2dKindCatalogs, Puzzle2dKindCompatibility, Puzzle2dMeta, Puzzle2dNode, Puzzle2dNodeAnchor, Puzzle2dTargetRegion};
use crate::Puzzle2dSnapshot;
use ::semio_framework_schema::ArtifactSchema;
use protocol::{DiffAlgebra, MutationDiff};
use protocol::list_delta::RowPatch;
use semio_framework_value::{list::PagedList, paged::PagedUtf8};

//#region 🔖️Diff
/// 🔺️ Sparse typed delta for the puzzle2d artifact: per-field entity patches and id-keyed collection deltas.
#[derive(Clone, Debug, Default, PartialEq, ArtifactSchema, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned)]
#[value(rename_all = "camelCase", default)]
#[artifact_schema(id = "s.puzzle.puzzle2d")]
pub struct Puzzle2dDiff {
    #[state(artifact)]
    pub artifact: Option<Box<Puzzle2dArtifact>>,
    #[state(artifact)]
    pub schema: Option<PagedUtf8<{ usize::MAX }>>,
    #[state(artifact)]
    pub camera: Option<Puzzle2dCamera>,
    #[state(artifact)]
    pub nodes: Option<Puzzle2dNodesDelta>,
    #[state(artifact)]
    pub edges: Option<Puzzle2dEdgesDelta>,
    #[state(artifact)]
    pub target_regions: Option<Puzzle2dTargetRegionsDelta>,
    #[state(artifact)]
    pub meta: Option<Puzzle2dMetaPatch>,
}
//#endregion 🔖️Diff

//#region 🔖️Patches
/// 🔑️ The identity of one kind-compatibility row: the pair of kinds it links.
#[derive(Clone, Debug, PartialEq, Eq, Hash, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned)]
#[value(rename_all = "camelCase")]
pub struct Puzzle2dKindCompatibilityKey {
    pub source: PagedUtf8<{ usize::MAX }>,
    pub target: PagedUtf8<{ usize::MAX }>,
}

/// 🩹 Sparse per-field patch over one `Puzzle2dHandle` — only the named fields change.
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned)]
#[value(rename_all = "camelCase", default)]
pub struct Puzzle2dHandlePatch {
    #[value(default, skip_serializing_if = "Option::is_none", deserialize_with = "deserialize_double_option")]
    pub handle_kind: Option<Option<PagedUtf8<{ usize::MAX }>>>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub angle: Option<f64>,
    #[value(default, skip_serializing_if = "Option::is_none", deserialize_with = "deserialize_double_option")]
    pub radius: Option<Option<f64>>,
    #[value(default, skip_serializing_if = "Option::is_none", deserialize_with = "deserialize_double_option")]
    pub color: Option<Option<PagedUtf8<{ usize::MAX }>>>,
    #[value(default, skip_serializing_if = "Option::is_none", deserialize_with = "deserialize_double_option")]
    pub icon_kind: Option<Option<PagedUtf8<{ usize::MAX }>>>,
    #[value(default, skip_serializing_if = "Option::is_none", deserialize_with = "deserialize_double_option")]
    pub scale: Option<Option<f64>>,
    #[value(default, skip_serializing_if = "Option::is_none", deserialize_with = "deserialize_double_option")]
    pub visible: Option<Option<bool>>,
    #[value(default, skip_serializing_if = "Option::is_none", deserialize_with = "deserialize_double_option")]
    pub locked: Option<Option<bool>>,
}

/// 🩹 Sparse per-field patch over one `Puzzle2dNode` — only the named fields change.
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned)]
#[value(rename_all = "camelCase", default)]
pub struct Puzzle2dNodePatch {
    #[value(default, skip_serializing_if = "Option::is_none", deserialize_with = "deserialize_double_option")]
    pub node_kind: Option<Option<PagedUtf8<{ usize::MAX }>>>,
    #[value(default, skip_serializing_if = "Option::is_none", deserialize_with = "deserialize_double_option")]
    pub shape: Option<Option<PagedUtf8<{ usize::MAX }>>>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub x: Option<f64>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub y: Option<f64>,
    #[value(default, skip_serializing_if = "Option::is_none", deserialize_with = "deserialize_double_option")]
    pub radius: Option<Option<f64>>,
    #[value(default, skip_serializing_if = "Option::is_none", deserialize_with = "deserialize_double_option")]
    pub width: Option<Option<f64>>,
    #[value(default, skip_serializing_if = "Option::is_none", deserialize_with = "deserialize_double_option")]
    pub height: Option<Option<f64>>,
    #[value(default, skip_serializing_if = "Option::is_none", deserialize_with = "deserialize_double_option")]
    pub text: Option<Option<PagedUtf8<{ usize::MAX }>>>,
    #[value(default, skip_serializing_if = "Option::is_none", deserialize_with = "deserialize_double_option")]
    pub icon_kind: Option<Option<PagedUtf8<{ usize::MAX }>>>,
    #[value(default, skip_serializing_if = "Option::is_none", deserialize_with = "deserialize_double_option")]
    pub root: Option<Option<bool>>,
    #[value(default, skip_serializing_if = "Option::is_none", deserialize_with = "deserialize_double_option")]
    pub scale: Option<Option<f64>>,
    #[value(default, skip_serializing_if = "Option::is_none", deserialize_with = "deserialize_double_option")]
    pub visible: Option<Option<bool>>,
    #[value(default, skip_serializing_if = "Option::is_none", deserialize_with = "deserialize_double_option")]
    pub locked: Option<Option<bool>>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub anchor: Option<Puzzle2dNodeAnchor>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub handles: Option<Puzzle2dHandlesDelta>,
}

/// 🩹 Sparse per-field patch over one `Puzzle2dEdge` — only the named fields change.
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned)]
#[value(rename_all = "camelCase", default)]
pub struct Puzzle2dEdgePatch {
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub source: Option<PagedUtf8<{ usize::MAX }>>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub target: Option<PagedUtf8<{ usize::MAX }>>,
    #[value(default, skip_serializing_if = "Option::is_none", deserialize_with = "deserialize_double_option")]
    pub edge_kind: Option<Option<PagedUtf8<{ usize::MAX }>>>,
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
    #[value(default, skip_serializing_if = "Option::is_none", deserialize_with = "deserialize_double_option")]
    pub source_tip: Option<Option<PagedUtf8<{ usize::MAX }>>>,
    #[value(default, skip_serializing_if = "Option::is_none", deserialize_with = "deserialize_double_option")]
    pub target_tip: Option<Option<PagedUtf8<{ usize::MAX }>>>,
    #[value(default, skip_serializing_if = "Option::is_none", deserialize_with = "deserialize_double_option")]
    pub visible: Option<Option<bool>>,
    #[value(default, skip_serializing_if = "Option::is_none", deserialize_with = "deserialize_double_option")]
    pub locked: Option<Option<bool>>,
}

/// 🩹 Sparse per-field patch over one `Puzzle2dTargetRegion` — only the named fields change.
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned)]
#[value(rename_all = "camelCase", default)]
pub struct Puzzle2dTargetRegionPatch {
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub x: Option<f64>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub y: Option<f64>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub width: Option<f64>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub height: Option<f64>,
    #[value(default, skip_serializing_if = "Option::is_none", deserialize_with = "deserialize_double_option")]
    pub label: Option<Option<PagedUtf8<{ usize::MAX }>>>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub hidden: Option<bool>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub locked: Option<bool>,
}

/// 🩹 Sparse per-field patch over one `Puzzle2dKindCompatibility` — only the named fields change.
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned)]
#[value(rename_all = "camelCase", default)]
pub struct Puzzle2dKindCompatibilityPatch {
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub bidirectional: Option<bool>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub important: Option<bool>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub specificity: Option<Puzzle2dCompatSpecificity>,
}

/// 🩹 Sparse per-field patch over one `Puzzle2dMeta` — only the named fields change.
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned)]
#[value(rename_all = "camelCase", default)]
pub struct Puzzle2dMetaPatch {
    #[value(default, skip_serializing_if = "Option::is_none", deserialize_with = "deserialize_double_option")]
    pub manifest_id: Option<Option<PagedUtf8<{ usize::MAX }>>>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub kind_compatibility: Option<Puzzle2dKindCompatibilityDelta>,
    #[value(default, skip_serializing_if = "Option::is_none", deserialize_with = "deserialize_double_option")]
    pub kind_catalogs: Option<Option<Puzzle2dKindCatalogs>>,
}

//#endregion 🔖️Patches

//#region 🔖️Deltas
protocol::list_delta! { pub Puzzle2dHandlesDelta { removal: Puzzle2dHandleRemoval, insertion: Puzzle2dHandleInsertion, relocation: Puzzle2dHandleRelocation, modification: Puzzle2dHandleModification, row: Puzzle2dHandle, patch: Puzzle2dHandlePatch, list: PagedList<Puzzle2dHandle, { usize::MAX }>, key: PagedUtf8<{ usize::MAX }> = |item| item.id.clone() } }
protocol::list_delta! { pub Puzzle2dNodesDelta { removal: Puzzle2dNodeRemoval, insertion: Puzzle2dNodeInsertion, relocation: Puzzle2dNodeRelocation, modification: Puzzle2dNodeModification, row: Puzzle2dNode, patch: Puzzle2dNodePatch, list: PagedList<Puzzle2dNode, { usize::MAX }>, key: PagedUtf8<{ usize::MAX }> = |item| item.id.clone() } }
protocol::list_delta! { pub Puzzle2dEdgesDelta { removal: Puzzle2dEdgeRemoval, insertion: Puzzle2dEdgeInsertion, relocation: Puzzle2dEdgeRelocation, modification: Puzzle2dEdgeModification, row: Puzzle2dEdge, patch: Puzzle2dEdgePatch, list: PagedList<Puzzle2dEdge, { usize::MAX }>, key: PagedUtf8<{ usize::MAX }> = |item| item.id.clone() } }
protocol::list_delta! { pub Puzzle2dTargetRegionsDelta { removal: Puzzle2dTargetRegionRemoval, insertion: Puzzle2dTargetRegionInsertion, relocation: Puzzle2dTargetRegionRelocation, modification: Puzzle2dTargetRegionModification, row: Puzzle2dTargetRegion, patch: Puzzle2dTargetRegionPatch, list: PagedList<Puzzle2dTargetRegion, { usize::MAX }>, key: PagedUtf8<{ usize::MAX }> = |item| item.id.clone() } }
protocol::list_delta! { pub Puzzle2dKindCompatibilityDelta { removal: Puzzle2dKindCompatibilityRemoval, insertion: Puzzle2dKindCompatibilityInsertion, relocation: Puzzle2dKindCompatibilityRelocation, modification: Puzzle2dKindCompatibilityModification, row: Puzzle2dKindCompatibility, patch: Puzzle2dKindCompatibilityPatch, list: PagedList<Puzzle2dKindCompatibility, { usize::MAX }>, key: Puzzle2dKindCompatibilityKey = |item| Puzzle2dKindCompatibilityKey { source: item.source.clone(), target: item.target.clone() } } }
//#endregion 🔖️Deltas

//#region 🔖️Algebra
/// 🕳️ Tri-state decode of every `Option<Option<T>>` patch slot: a missing key is the unchanged slot (`None`) and a PRESENT
/// `null` is the clear `Some(None)`, never the unchanged slot the blanket `Option<T>` decode would fold it into.
fn deserialize_double_option<T: semio_framework_value::FromValue>(value: semio_framework_value::DslValue) -> Result<Option<Option<T>>, semio_framework_value::ValueError> {
    <Option<T> as semio_framework_value::FromValue>::from_value(value).map(Some)
}
//#endregion 🔖️Algebra


impl RowPatch<Puzzle2dHandle> for Puzzle2dHandlePatch {
    fn commit_into(&self, item: &mut Puzzle2dHandle, capability: protocol::ApplyCapability) -> protocol::MutationApplyResult<()> {
        if let Some(value) = &self.handle_kind {
            item.handle_kind = value.clone();
        }
        if let Some(value) = &self.angle {
            item.angle = *value;
        }
        if let Some(value) = &self.radius {
            item.radius = *value;
        }
        if let Some(value) = &self.color {
            item.color = value.clone();
        }
        if let Some(value) = &self.icon_kind {
            item.icon_kind = value.clone();
        }
        if let Some(value) = &self.scale {
            item.scale = *value;
        }
        if let Some(value) = &self.visible {
            item.visible = *value;
        }
        if let Some(value) = &self.locked {
            item.locked = *value;
        }
        Ok(())
    }
    fn absorb(&mut self, later: Self) {
        if later.handle_kind.is_some() {
            self.handle_kind = later.handle_kind;
        }
        if later.angle.is_some() {
            self.angle = later.angle;
        }
        if later.radius.is_some() {
            self.radius = later.radius;
        }
        if later.color.is_some() {
            self.color = later.color;
        }
        if later.icon_kind.is_some() {
            self.icon_kind = later.icon_kind;
        }
        if later.scale.is_some() {
            self.scale = later.scale;
        }
        if later.visible.is_some() {
            self.visible = later.visible;
        }
        if later.locked.is_some() {
            self.locked = later.locked;
        }
    }
    fn inverse(&self, base: &Puzzle2dHandle) -> Self {
        Self {
            handle_kind: self.handle_kind.as_ref().map(|_| base.handle_kind.clone()),
            angle: self.angle.as_ref().map(|_| base.angle),
            radius: self.radius.as_ref().map(|_| base.radius),
            color: self.color.as_ref().map(|_| base.color.clone()),
            icon_kind: self.icon_kind.as_ref().map(|_| base.icon_kind.clone()),
            scale: self.scale.as_ref().map(|_| base.scale),
            visible: self.visible.as_ref().map(|_| base.visible),
            locked: self.locked.as_ref().map(|_| base.locked),
        }
    }
    fn is_empty(&self) -> bool {
        self.handle_kind.is_none() && self.angle.is_none() && self.radius.is_none() && self.color.is_none() && self.icon_kind.is_none() && self.scale.is_none() && self.visible.is_none() && self.locked.is_none()
    }
}

impl RowPatch<Puzzle2dNode> for Puzzle2dNodePatch {
    fn commit_into(&self, item: &mut Puzzle2dNode, capability: protocol::ApplyCapability) -> protocol::MutationApplyResult<()> {
        if let Some(value) = &self.node_kind {
            item.node_kind = value.clone();
        }
        if let Some(value) = &self.shape {
            item.shape = value.clone();
        }
        if let Some(value) = &self.x {
            item.x = *value;
        }
        if let Some(value) = &self.y {
            item.y = *value;
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
        if let Some(value) = &self.root {
            item.root = *value;
        }
        if let Some(value) = &self.scale {
            item.scale = *value;
        }
        if let Some(value) = &self.visible {
            item.visible = *value;
        }
        if let Some(value) = &self.locked {
            item.locked = *value;
        }
        if let Some(value) = &self.anchor {
            item.anchor = *value;
        }
        if let Some(delta) = &self.handles {
            item.handles = delta.commit_onto(&item.handles, capability).map_err(|error| error.under(["handles"]))?;
        }
        Ok(())
    }
    fn absorb(&mut self, later: Self) {
        if later.node_kind.is_some() {
            self.node_kind = later.node_kind;
        }
        if later.shape.is_some() {
            self.shape = later.shape;
        }
        if later.x.is_some() {
            self.x = later.x;
        }
        if later.y.is_some() {
            self.y = later.y;
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
        if later.root.is_some() {
            self.root = later.root;
        }
        if later.scale.is_some() {
            self.scale = later.scale;
        }
        if later.visible.is_some() {
            self.visible = later.visible;
        }
        if later.locked.is_some() {
            self.locked = later.locked;
        }
        if later.anchor.is_some() {
            self.anchor = later.anchor;
        }
        match (&mut self.handles, later.handles) {
            (Some(earlier), Some(next)) => earlier.absorb(next),
            (slot @ None, next) => *slot = next,
            (Some(_), None) => {}
        }
    }
    fn inverse(&self, base: &Puzzle2dNode) -> Self {
        Self {
            node_kind: self.node_kind.as_ref().map(|_| base.node_kind.clone()),
            shape: self.shape.as_ref().map(|_| base.shape.clone()),
            x: self.x.as_ref().map(|_| base.x),
            y: self.y.as_ref().map(|_| base.y),
            radius: self.radius.as_ref().map(|_| base.radius),
            width: self.width.as_ref().map(|_| base.width),
            height: self.height.as_ref().map(|_| base.height),
            text: self.text.as_ref().map(|_| base.text.clone()),
            icon_kind: self.icon_kind.as_ref().map(|_| base.icon_kind.clone()),
            root: self.root.as_ref().map(|_| base.root),
            scale: self.scale.as_ref().map(|_| base.scale),
            visible: self.visible.as_ref().map(|_| base.visible),
            locked: self.locked.as_ref().map(|_| base.locked),
            anchor: self.anchor.as_ref().map(|_| base.anchor),
            handles: self.handles.as_ref().map(|delta| delta.inverse(&base.handles)),
        }
    }
    fn is_empty(&self) -> bool {
        self.node_kind.is_none() && self.shape.is_none() && self.x.is_none() && self.y.is_none() && self.radius.is_none() && self.width.is_none() && self.height.is_none() && self.text.is_none() && self.icon_kind.is_none() && self.root.is_none() && self.scale.is_none() && self.visible.is_none() && self.locked.is_none() && self.anchor.is_none() && self.handles.as_ref().is_none_or(Puzzle2dHandlesDelta::is_empty)
    }
}

impl RowPatch<Puzzle2dEdge> for Puzzle2dEdgePatch {
    fn commit_into(&self, item: &mut Puzzle2dEdge, capability: protocol::ApplyCapability) -> protocol::MutationApplyResult<()> {
        if let Some(value) = &self.source {
            item.source = value.clone();
        }
        if let Some(value) = &self.target {
            item.target = value.clone();
        }
        if let Some(value) = &self.edge_kind {
            item.edge_kind = value.clone();
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
        if let Some(value) = &self.source_tip {
            item.source_tip = value.clone();
        }
        if let Some(value) = &self.target_tip {
            item.target_tip = value.clone();
        }
        if let Some(value) = &self.visible {
            item.visible = *value;
        }
        if let Some(value) = &self.locked {
            item.locked = *value;
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
        if later.edge_kind.is_some() {
            self.edge_kind = later.edge_kind;
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
        if later.source_tip.is_some() {
            self.source_tip = later.source_tip;
        }
        if later.target_tip.is_some() {
            self.target_tip = later.target_tip;
        }
        if later.visible.is_some() {
            self.visible = later.visible;
        }
        if later.locked.is_some() {
            self.locked = later.locked;
        }
    }
    fn inverse(&self, base: &Puzzle2dEdge) -> Self {
        Self {
            source: self.source.as_ref().map(|_| base.source.clone()),
            target: self.target.as_ref().map(|_| base.target.clone()),
            edge_kind: self.edge_kind.as_ref().map(|_| base.edge_kind.clone()),
            gap: self.gap.as_ref().map(|_| base.gap),
            shift: self.shift.as_ref().map(|_| base.shift),
            rise: self.rise.as_ref().map(|_| base.rise),
            rotation: self.rotation.as_ref().map(|_| base.rotation),
            turn: self.turn.as_ref().map(|_| base.turn),
            tilt: self.tilt.as_ref().map(|_| base.tilt),
            x: self.x.as_ref().map(|_| base.x),
            y: self.y.as_ref().map(|_| base.y),
            source_tip: self.source_tip.as_ref().map(|_| base.source_tip.clone()),
            target_tip: self.target_tip.as_ref().map(|_| base.target_tip.clone()),
            visible: self.visible.as_ref().map(|_| base.visible),
            locked: self.locked.as_ref().map(|_| base.locked),
        }
    }
    fn is_empty(&self) -> bool {
        self.source.is_none() && self.target.is_none() && self.edge_kind.is_none() && self.gap.is_none() && self.shift.is_none() && self.rise.is_none() && self.rotation.is_none() && self.turn.is_none() && self.tilt.is_none() && self.x.is_none() && self.y.is_none() && self.source_tip.is_none() && self.target_tip.is_none() && self.visible.is_none() && self.locked.is_none()
    }
}

impl RowPatch<Puzzle2dTargetRegion> for Puzzle2dTargetRegionPatch {
    fn commit_into(&self, item: &mut Puzzle2dTargetRegion, capability: protocol::ApplyCapability) -> protocol::MutationApplyResult<()> {
        if let Some(value) = &self.x {
            item.x = *value;
        }
        if let Some(value) = &self.y {
            item.y = *value;
        }
        if let Some(value) = &self.width {
            item.width = *value;
        }
        if let Some(value) = &self.height {
            item.height = *value;
        }
        if let Some(value) = &self.label {
            item.label = value.clone();
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
        if later.width.is_some() {
            self.width = later.width;
        }
        if later.height.is_some() {
            self.height = later.height;
        }
        if later.label.is_some() {
            self.label = later.label;
        }
        if later.hidden.is_some() {
            self.hidden = later.hidden;
        }
        if later.locked.is_some() {
            self.locked = later.locked;
        }
    }
    fn inverse(&self, base: &Puzzle2dTargetRegion) -> Self {
        Self {
            x: self.x.as_ref().map(|_| base.x),
            y: self.y.as_ref().map(|_| base.y),
            width: self.width.as_ref().map(|_| base.width),
            height: self.height.as_ref().map(|_| base.height),
            label: self.label.as_ref().map(|_| base.label.clone()),
            hidden: self.hidden.as_ref().map(|_| base.hidden),
            locked: self.locked.as_ref().map(|_| base.locked),
        }
    }
    fn is_empty(&self) -> bool {
        self.x.is_none() && self.y.is_none() && self.width.is_none() && self.height.is_none() && self.label.is_none() && self.hidden.is_none() && self.locked.is_none()
    }
}

impl RowPatch<Puzzle2dKindCompatibility> for Puzzle2dKindCompatibilityPatch {
    fn commit_into(&self, item: &mut Puzzle2dKindCompatibility, capability: protocol::ApplyCapability) -> protocol::MutationApplyResult<()> {
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
    fn inverse(&self, base: &Puzzle2dKindCompatibility) -> Self {
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

impl RowPatch<Puzzle2dMeta> for Puzzle2dMetaPatch {
    fn commit_into(&self, item: &mut Puzzle2dMeta, capability: protocol::ApplyCapability) -> protocol::MutationApplyResult<()> {
        if let Some(value) = &self.manifest_id {
            item.manifest_id = value.clone();
        }
        if let Some(delta) = &self.kind_compatibility {
            item.kind_compatibility = delta.commit_onto(&item.kind_compatibility, capability).map_err(|error| error.under(["kindCompatibility"]))?;
        }
        if let Some(value) = &self.kind_catalogs {
            item.kind_catalogs = value.clone();
        }
        Ok(())
    }
    fn absorb(&mut self, later: Self) {
        if later.manifest_id.is_some() {
            self.manifest_id = later.manifest_id;
        }
        match (&mut self.kind_compatibility, later.kind_compatibility) {
            (Some(earlier), Some(next)) => earlier.absorb(next),
            (slot @ None, next) => *slot = next,
            (Some(_), None) => {}
        }
        if later.kind_catalogs.is_some() {
            self.kind_catalogs = later.kind_catalogs;
        }
    }
    fn inverse(&self, base: &Puzzle2dMeta) -> Self {
        Self {
            manifest_id: self.manifest_id.as_ref().map(|_| base.manifest_id.clone()),
            kind_compatibility: self.kind_compatibility.as_ref().map(|delta| delta.inverse(&base.kind_compatibility)),
            kind_catalogs: self.kind_catalogs.as_ref().map(|_| base.kind_catalogs.clone()),
        }
    }
    fn is_empty(&self) -> bool {
        self.manifest_id.is_none() && self.kind_compatibility.as_ref().is_none_or(Puzzle2dKindCompatibilityDelta::is_empty) && self.kind_catalogs.is_none()
    }
}

impl MutationDiff<Puzzle2dSnapshot> for Puzzle2dDiff {
    fn apply(&self, base: &Puzzle2dSnapshot, capability: protocol::ApplyCapability) -> protocol::MutationApplyResult<Puzzle2dSnapshot> {
        let mut item = match &self.artifact {
            Some(artifact) => artifact.to_snapshot(),
            None => base.clone(),
        };
        if let Some(value) = &self.schema {
            item.schema = value.clone();
        }
        if let Some(value) = &self.camera {
            item.camera = value.clone();
        }
        if let Some(delta) = &self.nodes {
            item.nodes = delta.commit_onto(&item.nodes, capability).map_err(|error| error.under(["nodes"]))?;
        }
        if let Some(delta) = &self.edges {
            item.edges = delta.commit_onto(&item.edges, capability).map_err(|error| error.under(["edges"]))?;
        }
        if let Some(delta) = &self.target_regions {
            item.target_regions = delta.commit_onto(&item.target_regions, capability).map_err(|error| error.under(["targetRegions"]))?;
        }
        if let Some(patch) = &self.meta {
            patch.commit_into(&mut item.meta, capability).map_err(|error| error.under(["meta"]))?;
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
        if later.camera.is_some() {
            self.camera = later.camera;
        }
        match (&mut self.nodes, later.nodes) {
            (Some(earlier), Some(next)) => earlier.absorb(next),
            (slot @ None, next) => *slot = next,
            (Some(_), None) => {}
        }
        match (&mut self.edges, later.edges) {
            (Some(earlier), Some(next)) => earlier.absorb(next),
            (slot @ None, next) => *slot = next,
            (Some(_), None) => {}
        }
        match (&mut self.target_regions, later.target_regions) {
            (Some(earlier), Some(next)) => earlier.absorb(next),
            (slot @ None, next) => *slot = next,
            (Some(_), None) => {}
        }
        match (&mut self.meta, later.meta) {
            (Some(earlier), Some(next)) => earlier.absorb(next),
            (slot @ None, next) => *slot = next,
            (Some(_), None) => {}
        }
    }
}

impl DiffAlgebra<Puzzle2dSnapshot> for Puzzle2dDiff {
    fn inverse(&self, base: &Puzzle2dSnapshot) -> Self {
        if self.artifact.is_some() {
            return Self { artifact: Some(Box::new(Puzzle2dArtifact::from_snapshot(base.clone()))), ..Default::default() };
        }
        Self {
            artifact: None,
            schema: self.schema.as_ref().map(|_| base.schema.clone()),
            camera: self.camera.as_ref().map(|_| base.camera.clone()),
            nodes: self.nodes.as_ref().map(|delta| delta.inverse(&base.nodes)),
            edges: self.edges.as_ref().map(|delta| delta.inverse(&base.edges)),
            target_regions: self.target_regions.as_ref().map(|delta| delta.inverse(&base.target_regions)),
            meta: self.meta.as_ref().map(|patch| patch.inverse(&base.meta)),
        }
    }
    fn is_empty(&self) -> bool {
        self.artifact.is_none() && self.schema.is_none() && self.camera.is_none() && self.nodes.as_ref().is_none_or(Puzzle2dNodesDelta::is_empty) && self.edges.as_ref().is_none_or(Puzzle2dEdgesDelta::is_empty) && self.target_regions.as_ref().is_none_or(Puzzle2dTargetRegionsDelta::is_empty) && self.meta.as_ref().is_none_or(|patch| patch.is_empty())
    }
}
