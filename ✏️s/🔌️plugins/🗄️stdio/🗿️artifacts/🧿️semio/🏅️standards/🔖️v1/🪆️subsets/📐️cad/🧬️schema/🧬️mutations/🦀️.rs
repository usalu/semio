//! 🧬️ SemioCadMutation — document mutation dispatch. Every variant's `diff()` is handcrafted
//! (never apply-and-capture) via the diff module's `wrap_*_diff` helpers; every variant's
//! `inverse()` looks up prior state from `base` and constructs the exact undoing mutation
//! (name/handle-aware, matching bcf/docx precedent).

use crate::standards::v1::subsets::base::schema::geometry::SemioPoint2;
use crate::standards::v1::subsets::base::schema::triples::{NamedTripleDiff};


use crate::standards::v1::subsets::cad::schema::diff::{diff_set_snapshot, wrap_block_diff, wrap_block_entity_diff, wrap_entity_diff, wrap_layer_diff, CadBlockDiff, CadEntityRecordDiff, CadLayerDiff, SemioCadDiff};
















use crate::standards::v1::subsets::cad::schema::snapshot::{CadBlock, CadEntity, CadEntityRecord, CadLayer, SemioCadSnapshot};

use protocol::{Mutation};

//#region 🔖️Mutations
#[path = "🧱add-block/🦀️.rs"]
pub mod add_block;
#[path = "🧩add-block-entity/🦀️.rs"]
pub mod add_block_entity;
#[path = "🔷add-entity/🦀️.rs"]
pub mod add_entity;
#[path = "🗂️add-layer/🦀️.rs"]
pub mod add_layer;
#[path = "🚫remove-block/🦀️.rs"]
pub mod remove_block;
#[path = "✂️remove-block-entity/🦀️.rs"]
pub mod remove_block_entity;
#[path = "🗑️remove-entity/🦀️.rs"]
pub mod remove_entity;
#[path = "🧹remove-layer/🦀️.rs"]
pub mod remove_layer;
#[path = "📍set-block-base-point/🦀️.rs"]
pub mod set_block_base_point;
#[path = "🔺set-block-entity-geometry/🦀️.rs"]
pub mod set_block_entity_geometry;
#[path = "🧷️set-block-entity-layer/🦀️.rs"]
pub mod set_block_entity_layer;
#[path = "📐set-entity-geometry/🦀️.rs"]
pub mod set_entity_geometry;
#[path = "🏳️set-entity-layer/🦀️.rs"]
pub mod set_entity_layer;
#[path = "🎚️set-layer/🦀️.rs"]
pub mod set_layer;
/// 📐️ Typed document mutation for `stdio.semio.cad`. Every variant addresses one facet of the CAD
/// document; `NoMutation` was dropped: `#[derive(dsl::Mutations)]` requires every variant to wrap
/// exactly one leaf payload and a unit variant wraps none (same consequence tiff's baseline
/// migration reached — see `🖼️tiff/🏅️standards/🔖️6.0/🪆️subsets/🧱️baseline/🧬️schema/🧬️mutations/🦀️.rs`).
//#region 🔖️Leaves
#[path = "🩹️patch-snapshot/🦀️.rs"]
pub mod patch_snapshot;
#[path = "📸️set-snapshot/🦀️.rs"]
pub mod set_snapshot;
//#endregion 🔖️Leaves

#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::Mutations)]
#[mutations(snapshot = SemioCadSnapshot, diff = SemioCadDiff, schema = "SemioCadMutation")]
#[value(tag = "mutation", rename_all = "camelCase")]
pub enum SemioCadMutation {
    SetSnapshot(set_snapshot::SetSnapshot),
    PatchSnapshot(patch_snapshot::PatchSnapshot),
    AddLayer(add_layer::AddLayer),
    RemoveLayer(remove_layer::RemoveLayer),
    SetLayer(set_layer::SetLayer),
    AddBlock(add_block::AddBlock),
    RemoveBlock(remove_block::RemoveBlock),
    SetBlockBasePoint(set_block_base_point::SetBlockBasePoint),
    AddEntity(add_entity::AddEntity),
    RemoveEntity(remove_entity::RemoveEntity),
    SetEntityLayer(set_entity_layer::SetEntityLayer),
    SetEntityGeometry(set_entity_geometry::SetEntityGeometry),
    AddBlockEntity(add_block_entity::AddBlockEntity),
    RemoveBlockEntity(remove_block_entity::RemoveBlockEntity),
    SetBlockEntityLayer(set_block_entity_layer::SetBlockEntityLayer),
    SetBlockEntityGeometry(set_block_entity_geometry::SetBlockEntityGeometry),
}

/// 🏷️ The declared mutation vocabulary of `s.stdio.semio.cad`, in `SemioCadMutation`'s own
/// declaration order and kebab-case spelling — the single source of truth for the binary op frame's
/// `tag` ordinal (see [`wire_tag`]), for `parse_cad_mutation`'s keyword match, and for the
/// `semio-v1-cad` catalog in `../../🔣️oracle.json`. The framework never parses Rust, so
/// `kinds_match_the_enum_and_the_catalog` below is what keeps all three honest.
pub const KINDS: &[&str] = &[
    "set-snapshot", "add-layer",
    "remove-layer",
    "set-layer",
    "add-block",
    "remove-block",
    "set-block-base-point",
    "add-entity",
    "remove-entity",
    "set-entity-layer",
    "set-entity-geometry",
    "add-block-entity",
    "remove-block-entity",
    "set-block-entity-layer",
    "set-block-entity-geometry", "patch-snapshot",
];
//#endregion 🔖️Mutations

//#region 🔖️Apply
/// ▶️ Applies `mutation` to `snapshot`. Single semantics source: the returned diff IS what gets
/// applied.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn apply_semio_cad_mutation(snapshot: &mut SemioCadSnapshot, mutation: &SemioCadMutation) -> protocol::MutationOutcome<SemioCadDiff> {
    let outcome = <SemioCadMutation as Mutation<SemioCadSnapshot>>::diff(mutation, snapshot);
    outcome.apply_to(snapshot)
}

/// ↩️ Computes `mutation`'s own inverse against `base` — a thin wrapper around
/// `protocol::Mutation::inverse` so external Rust callers that cannot name this crate's private
/// `protocol` extern-crate item (the `📐️mutate-semio-cad` test adapter, whose `inverse-<kind>`
/// scenarios need a mutation's own computed inverse) can still reach the inverse law that
/// [`apply_semio_cad_mutation`] alone cannot. Same shape as `🧰️kit`'s `inverse_semio_kit_mutation`.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn inverse_semio_cad_mutation(mutation: &SemioCadMutation, base: &SemioCadSnapshot) -> Result<Vec<SemioCadMutation>, semio_framework_value::ValueError> {
    Ok({
    <SemioCadMutation as Mutation<SemioCadSnapshot>>::inverse(mutation, base)?

    })
}


//#endregion 🔖️Apply

//#region 🔖️MutationTrait
// 🚫️async: E1 pure codec/computation helper — lifted verbatim from the former `impl Mutation`.
pub(crate) fn agg_diff(this: &SemioCadMutation, base: &SemioCadSnapshot) -> protocol::MutationOutcome<SemioCadDiff> {
    protocol::MutationOutcome::new(match this {
        SemioCadMutation::SetSnapshot(set_snapshot::SetSnapshot { snapshot }) => diff_set_snapshot(base, snapshot),
        SemioCadMutation::PatchSnapshot(patch) => return <patch_snapshot::PatchSnapshot as protocol::MutationKind<SemioCadSnapshot, SemioCadMutation>>::diff(patch, base),
        SemioCadMutation::AddLayer(add_layer::AddLayer { layer }) => SemioCadDiff { layers: Some(NamedTripleDiff { removed: Vec::new(), modified: Vec::new(), added: vec![layer.clone()] }), blocks: None, entities: None },
        SemioCadMutation::RemoveLayer(remove_layer::RemoveLayer { name }) => SemioCadDiff { layers: Some(NamedTripleDiff { removed: vec![name.clone()], modified: Vec::new(), added: Vec::new() }), blocks: None, entities: None },
        SemioCadMutation::SetLayer(set_layer::SetLayer { name, color_index, line_type, visible }) => wrap_layer_diff(name, CadLayerDiff { color_index: *color_index, line_type: line_type.clone(), visible: *visible }),
        SemioCadMutation::AddBlock(add_block::AddBlock { block }) => SemioCadDiff { layers: None, blocks: Some(NamedTripleDiff { removed: Vec::new(), modified: Vec::new(), added: vec![block.clone()] }), entities: None },
        SemioCadMutation::RemoveBlock(remove_block::RemoveBlock { name }) => SemioCadDiff { layers: None, blocks: Some(NamedTripleDiff { removed: vec![name.clone()], modified: Vec::new(), added: Vec::new() }), entities: None },
        SemioCadMutation::SetBlockBasePoint(set_block_base_point::SetBlockBasePoint { name, base_point }) => wrap_block_diff(name, CadBlockDiff { base_point: Some(*base_point), entities: None }),
        SemioCadMutation::AddEntity(add_entity::AddEntity { entity }) => SemioCadDiff { layers: None, blocks: None, entities: Some(NamedTripleDiff { removed: Vec::new(), modified: Vec::new(), added: vec![entity.clone()] }) },
        SemioCadMutation::RemoveEntity(remove_entity::RemoveEntity { handle }) => SemioCadDiff { layers: None, blocks: None, entities: Some(NamedTripleDiff { removed: vec![handle.clone()], modified: Vec::new(), added: Vec::new() }) },
        SemioCadMutation::SetEntityLayer(set_entity_layer::SetEntityLayer { handle, layer }) => wrap_entity_diff(handle, CadEntityRecordDiff { layer: Some(layer.clone()), entity: None }),
        SemioCadMutation::SetEntityGeometry(set_entity_geometry::SetEntityGeometry { handle, entity }) => wrap_entity_diff(handle, CadEntityRecordDiff { layer: None, entity: Some(entity.clone()) }),
        SemioCadMutation::AddBlockEntity(add_block_entity::AddBlockEntity { block_name, entity }) => {
            wrap_block_diff(block_name, CadBlockDiff { base_point: None, entities: Some(NamedTripleDiff { removed: Vec::new(), modified: Vec::new(), added: vec![entity.clone()] }) })
        }
        SemioCadMutation::RemoveBlockEntity(remove_block_entity::RemoveBlockEntity { block_name, handle }) => {
            wrap_block_diff(block_name, CadBlockDiff { base_point: None, entities: Some(NamedTripleDiff { removed: vec![handle.clone()], modified: Vec::new(), added: Vec::new() }) })
        }
        SemioCadMutation::SetBlockEntityLayer(set_block_entity_layer::SetBlockEntityLayer { block_name, handle, layer }) => wrap_block_entity_diff(block_name, handle, CadEntityRecordDiff { layer: Some(layer.clone()), entity: None }),
        SemioCadMutation::SetBlockEntityGeometry(set_block_entity_geometry::SetBlockEntityGeometry { block_name, handle, entity }) => wrap_block_entity_diff(block_name, handle, CadEntityRecordDiff { layer: None, entity: Some(entity.clone()) }),
    })
}

// 🚫️async: E1 pure codec/computation helper — lifted verbatim from the former `impl Mutation`.
pub(crate) fn agg_inverse(this: &SemioCadMutation, base: &SemioCadSnapshot) -> Result<Vec<SemioCadMutation>, semio_framework_value::ValueError> {
    Ok({
    match this {
        SemioCadMutation::SetSnapshot(_) => vec![SemioCadMutation::SetSnapshot(set_snapshot::SetSnapshot { snapshot: base.clone() })],
        SemioCadMutation::PatchSnapshot(patch) => return Ok(<patch_snapshot::PatchSnapshot as protocol::MutationKind<SemioCadSnapshot, SemioCadMutation>>::inverse(patch, base)?),
        SemioCadMutation::AddLayer(add_layer::AddLayer { layer }) => vec![SemioCadMutation::RemoveLayer(remove_layer::RemoveLayer { name: layer.name.clone() })],
        SemioCadMutation::RemoveLayer(remove_layer::RemoveLayer { name }) => match find_layer(base, name) {
            Some(l) => vec![SemioCadMutation::AddLayer(add_layer::AddLayer { layer: l.clone() })],
            None => Vec::new(),
        },
        SemioCadMutation::SetLayer(set_layer::SetLayer { name, color_index, line_type, visible }) => match find_layer(base, name) {
            Some(l) => vec![SemioCadMutation::SetLayer(set_layer::SetLayer {
                name: name.clone(),
                color_index: color_index.as_ref().map(|_| l.color_index),
                line_type: line_type.as_ref().map(|_| l.line_type.clone()),
                visible: visible.as_ref().map(|_| l.visible),
            })],
            None => Vec::new(),
        },
        SemioCadMutation::AddBlock(add_block::AddBlock { block }) => vec![SemioCadMutation::RemoveBlock(remove_block::RemoveBlock { name: block.name.clone() })],
        SemioCadMutation::RemoveBlock(remove_block::RemoveBlock { name }) => match find_block(base, name) {
            Some(b) => vec![SemioCadMutation::AddBlock(add_block::AddBlock { block: b.clone() })],
            None => Vec::new(),
        },
        SemioCadMutation::SetBlockBasePoint(set_block_base_point::SetBlockBasePoint { name, .. }) => match find_block(base, name) {
            Some(b) => vec![SemioCadMutation::SetBlockBasePoint(set_block_base_point::SetBlockBasePoint { name: name.clone(), base_point: b.base_point })],
            None => Vec::new(),
        },
        SemioCadMutation::AddEntity(add_entity::AddEntity { entity }) => vec![SemioCadMutation::RemoveEntity(remove_entity::RemoveEntity { handle: entity.handle.clone() })],
        SemioCadMutation::RemoveEntity(remove_entity::RemoveEntity { handle }) => match find_entity(base, handle) {
            Some(e) => vec![SemioCadMutation::AddEntity(add_entity::AddEntity { entity: e.clone() })],
            None => Vec::new(),
        },
        SemioCadMutation::SetEntityLayer(set_entity_layer::SetEntityLayer { handle, .. }) => match find_entity(base, handle) {
            Some(e) => vec![SemioCadMutation::SetEntityLayer(set_entity_layer::SetEntityLayer { handle: handle.clone(), layer: e.layer.clone() })],
            None => Vec::new(),
        },
        SemioCadMutation::SetEntityGeometry(set_entity_geometry::SetEntityGeometry { handle, .. }) => match find_entity(base, handle) {
            Some(e) => vec![SemioCadMutation::SetEntityGeometry(set_entity_geometry::SetEntityGeometry { handle: handle.clone(), entity: e.entity.clone() })],
            None => Vec::new(),
        },
        SemioCadMutation::AddBlockEntity(add_block_entity::AddBlockEntity { block_name, entity }) => vec![SemioCadMutation::RemoveBlockEntity(remove_block_entity::RemoveBlockEntity { block_name: block_name.clone(), handle: entity.handle.clone() })],
        SemioCadMutation::RemoveBlockEntity(remove_block_entity::RemoveBlockEntity { block_name, handle }) => match find_block_entity(base, block_name, handle) {
            Some(e) => vec![SemioCadMutation::AddBlockEntity(add_block_entity::AddBlockEntity { block_name: block_name.clone(), entity: e.clone() })],
            None => Vec::new(),
        },
        SemioCadMutation::SetBlockEntityLayer(set_block_entity_layer::SetBlockEntityLayer { block_name, handle, .. }) => match find_block_entity(base, block_name, handle) {
            Some(e) => vec![SemioCadMutation::SetBlockEntityLayer(set_block_entity_layer::SetBlockEntityLayer { block_name: block_name.clone(), handle: handle.clone(), layer: e.layer.clone() })],
            None => Vec::new(),
        },
        SemioCadMutation::SetBlockEntityGeometry(set_block_entity_geometry::SetBlockEntityGeometry { block_name, handle, .. }) => match find_block_entity(base, block_name, handle) {
            Some(e) => vec![SemioCadMutation::SetBlockEntityGeometry(set_block_entity_geometry::SetBlockEntityGeometry { block_name: block_name.clone(), handle: handle.clone(), entity: e.entity.clone() })],
            None => Vec::new(),
        },
    }

    })
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn find_layer<'a>(base: &'a SemioCadSnapshot, name: &str) -> Option<&'a CadLayer> {
    base.layers.iter().find(|l| l.name == name)
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn find_block<'a>(base: &'a SemioCadSnapshot, name: &str) -> Option<&'a CadBlock> {
    base.blocks.iter().find(|b| b.name == name)
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn find_entity<'a>(base: &'a SemioCadSnapshot, handle: &str) -> Option<&'a CadEntityRecord> {
    base.entities.iter().find(|e| e.handle == handle)
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn find_block_entity<'a>(base: &'a SemioCadSnapshot, block_name: &str, handle: &str) -> Option<&'a CadEntityRecord> {
    find_block(base, block_name)?.entities.iter().find(|e| e.handle == handle)
}
//#endregion 🔖️MutationTrait

//#region OpCodecs















//#endregion OpCodecs

//#region 🔖️Demo
/// 🌱 Shared fixture + representative `SemioCadMutation` cases (one per variant, plus extra
/// `SetEntityGeometry`/`SetBlockEntityGeometry` cases exercising several of the 9 `CadEntity`
/// kinds) — single source of truth for this facet's own tests AND
/// `ops_grammar_conformance_law`/`protocol_walk_law` in `🎹️composer/🦀️.rs`.
#[cfg(test)]
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn fixture() -> SemioCadSnapshot {
    SemioCadSnapshot {
        schema: crate::standards::v1::subsets::cad::schema::snapshot::STDIO_SEMIOCAD_DOCUMENT_SCHEMA.into(),
        layers: vec![CadLayer { name: "0".into(), color_index: 7, line_type: "CONTINUOUS".into(), visible: true }, CadLayer { name: "dim".into(), color_index: 7, line_type: "CONTINUOUS".into(), visible: true }],
        blocks: vec![CadBlock {
            name: "door".into(),
            base_point: SemioPoint2 { x: 0.0, y: 0.0 },
            entities: vec![CadEntityRecord { handle: "be1".into(), layer: "0".into(), entity: CadEntity::Circle { center: SemioPoint2 { x: 1.0, y: 1.0 }, radius: 2.0 } }],
        }],
        entities: vec![CadEntityRecord { handle: "h1".into(), layer: "0".into(), entity: CadEntity::Circle { center: SemioPoint2 { x: 1.0, y: 1.0 }, radius: 2.0 } }],
    }
}

#[cfg(test)]
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn demo_mutation_cases() -> Vec<SemioCadMutation> {
    let base = fixture();
    vec![
        SemioCadMutation::PatchSnapshot(patch_snapshot::PatchSnapshot { patch: semio_s_artifact_stdio_contract::editing::SnapshotPatch::Set { path: "/schema".into(), value: semio_framework_value::DslValue::String("stdio.patch-snapshot.witness".into()) } }),
        SemioCadMutation::SetSnapshot(set_snapshot::SetSnapshot { snapshot: base.clone() }),
        SemioCadMutation::AddLayer(add_layer::AddLayer { layer: CadLayer { name: "fresh".into(), color_index: 3, line_type: "CONTINUOUS".into(), visible: true } }),
        SemioCadMutation::RemoveLayer(remove_layer::RemoveLayer { name: "dim".into() }),
        SemioCadMutation::SetLayer(set_layer::SetLayer { name: "0".into(), color_index: Some(3), line_type: None, visible: Some(false) }),
        SemioCadMutation::AddBlock(add_block::AddBlock { block: CadBlock { name: "window".into(), base_point: SemioPoint2 { x: 2.0, y: 2.0 }, entities: Vec::new() } }),
        SemioCadMutation::RemoveBlock(remove_block::RemoveBlock { name: "door".into() }),
        SemioCadMutation::SetBlockBasePoint(set_block_base_point::SetBlockBasePoint { name: "door".into(), base_point: SemioPoint2 { x: 5.0, y: 5.0 } }),
        SemioCadMutation::AddEntity(add_entity::AddEntity { entity: CadEntityRecord { handle: "h2".into(), layer: "0".into(), entity: CadEntity::Circle { center: SemioPoint2 { x: 1.0, y: 1.0 }, radius: 2.0 } } }),
        SemioCadMutation::RemoveEntity(remove_entity::RemoveEntity { handle: "h1".into() }),
        SemioCadMutation::SetEntityLayer(set_entity_layer::SetEntityLayer { handle: "h1".into(), layer: "dim".into() }),
        SemioCadMutation::SetEntityGeometry(set_entity_geometry::SetEntityGeometry {
            handle: "h1".into(),
            entity: CadEntity::Ellipse { center: SemioPoint2 { x: 0.0, y: 0.0 }, major_axis_end: SemioPoint2 { x: 1.0, y: 0.0 }, ratio: 0.5, start_param: 0.0, end_param: 6.28 },
        }),
        SemioCadMutation::AddBlockEntity(add_block_entity::AddBlockEntity {
            block_name: "door".into(),
            entity: CadEntityRecord { handle: "be2".into(), layer: "0".into(), entity: CadEntity::Text { position: SemioPoint2 { x: 0.0, y: 0.0 }, height: 2.5, rotation: 0.0, content: "label".into() } },
        }),
        SemioCadMutation::RemoveBlockEntity(remove_block_entity::RemoveBlockEntity { block_name: "door".into(), handle: "be1".into() }),
        SemioCadMutation::SetBlockEntityLayer(set_block_entity_layer::SetBlockEntityLayer { block_name: "door".into(), handle: "be1".into(), layer: "dim".into() }),
        SemioCadMutation::SetBlockEntityGeometry(set_block_entity_geometry::SetBlockEntityGeometry {
            block_name: "door".into(),
            handle: "be1".into(),
            entity: CadEntity::Arc { center: SemioPoint2 { x: 0.0, y: 0.0 }, radius: 1.0, start_angle: 0.0, end_angle: 90.0 },
        }),
    ]
}
//#endregion 🔖️Demo

//#region 🔖️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🔖️Tests

//#region 🧪️FixtureCases
/// 🧪️ Handcrafted `📸️set-snapshot` fixture cases, wired from this tree's own mutations root so
/// `🦀️.rs` stays untouched (`#[path]` on a non-inline module resolves against this file's own
/// directory).
#[cfg(test)]
#[path = "📸️set-snapshot/🧪️tests/⭕️dims/🦀️.rs"]
mod set_snapshot_dims_the_walls_layer_and_widens_the_circle;
//#endregion 🧪️FixtureCases

#[cfg(test)]
use protocol::{OpBinary,OpText};
