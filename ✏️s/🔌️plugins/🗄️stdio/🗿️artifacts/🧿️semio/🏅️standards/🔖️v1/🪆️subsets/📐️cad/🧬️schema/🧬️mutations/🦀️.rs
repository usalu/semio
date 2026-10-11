//! 🧬️ SemioCadMutation — document mutation dispatch. Every variant's `diff()` is handcrafted
//! (never apply-and-capture) via the diff module's `wrap_*_diff` helpers; every variant's
//! `inverse()` looks up prior state from `base` and constructs the exact undoing mutation
//! (name/handle-aware, matching bcf/docx precedent).

use crate::standards::v1::subsets::base::schema::geometry::SemioPoint2;
use crate::standards::v1::subsets::base::schema::triples::{NamedTripleDiff};


use crate::standards::v1::subsets::cad::schema::diff::{wrap_block_diff, wrap_block_entity_diff, wrap_entity_diff, wrap_layer_diff, CadBlockDiff, CadEntityRecordDiff, CadLayerDiff, SemioCadDiff};
















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
//#endregion 🔖️Leaves

#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::Mutations, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutations(snapshot = SemioCadSnapshot, diff = SemioCadDiff, schema = "SemioCadMutation")]
#[value(tag = "mutation", rename_all = "camelCase")]
pub enum SemioCadMutation {
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
    "add-layer",
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
    "set-block-entity-geometry", ];
//#endregion 🔖️Mutations

/// 🧮️ Pure diff face of [`Mutation::diff`], named only in this subset's own reachable types (`protocol` is a private
/// `extern crate` alias, so an owner-root test adapter cannot bring the `Mutation` trait into scope).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn diff_semio_cad_mutation(mutation: &SemioCadMutation, base: &SemioCadSnapshot) -> protocol::MutationOutcome<SemioCadDiff> {
    <SemioCadMutation as protocol::Mutation<SemioCadSnapshot>>::diff(mutation, base)
}


/// ↩️ Computes `mutation`'s own inverse against `base` — a thin wrapper around
/// `protocol::Mutation::inverse` so external Rust callers that cannot name this crate's private
/// `protocol` extern-crate item (the `📐️mutate-semio-cad` test adapter, whose `inverse-<kind>`
/// scenarios need a mutation's own computed inverse) can still reach the inverse law that
/// `diff_semio_*_mutation` alone cannot. Same shape as `🧰️kit`'s `inverse_semio_kit_mutation`.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn inverse_semio_cad_mutation(mutation: &SemioCadMutation, base: &SemioCadSnapshot) -> Result<Vec<SemioCadMutation>, semio_framework_value::ValueError> {
    Ok({
    <SemioCadMutation as Mutation<SemioCadSnapshot>>::inverse(mutation, base)?

    })
}


//#endregion 🔖️Apply





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
        SemioCadMutation::AddLayer(add_layer::AddLayer { layer: CadLayer { name: "fresh".into(), color_index: 3, line_type: "CONTINUOUS".into(), visible: true }, at: None }),
        SemioCadMutation::RemoveLayer(remove_layer::RemoveLayer { name: "dim".into() }),
        SemioCadMutation::SetLayer(set_layer::SetLayer { name: "0".into(), color_index: Some(3), line_type: None, visible: Some(false) }),
        SemioCadMutation::AddBlock(add_block::AddBlock { block: CadBlock { name: "window".into(), base_point: SemioPoint2 { x: 2.0, y: 2.0 }, entities: Vec::new() }, at: None }),
        SemioCadMutation::RemoveBlock(remove_block::RemoveBlock { name: "door".into() }),
        SemioCadMutation::SetBlockBasePoint(set_block_base_point::SetBlockBasePoint { name: "door".into(), base_point: SemioPoint2 { x: 5.0, y: 5.0 } }),
        SemioCadMutation::AddEntity(add_entity::AddEntity { entity: CadEntityRecord { handle: "h2".into(), layer: "0".into(), entity: CadEntity::Circle { center: SemioPoint2 { x: 1.0, y: 1.0 }, radius: 2.0 } }, at: None }),
        SemioCadMutation::RemoveEntity(remove_entity::RemoveEntity { handle: "h1".into() }),
        SemioCadMutation::SetEntityLayer(set_entity_layer::SetEntityLayer { handle: "h1".into(), layer: "dim".into() }),
        SemioCadMutation::SetEntityGeometry(set_entity_geometry::SetEntityGeometry {
            handle: "h1".into(),
            entity: CadEntity::Ellipse { center: SemioPoint2 { x: 0.0, y: 0.0 }, major_axis_end: SemioPoint2 { x: 1.0, y: 0.0 }, ratio: 0.5, start_param: 0.0, end_param: 6.28 },
        }),
        SemioCadMutation::AddBlockEntity(add_block_entity::AddBlockEntity {
            block_name: "door".into(),
            entity: CadEntityRecord { handle: "be2".into(), layer: "0".into(), entity: CadEntity::Text { position: SemioPoint2 { x: 0.0, y: 0.0 }, height: 2.5, rotation: 0.0, content: "label".into() } }, at: None,
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

#[cfg(test)]
use protocol::{OpBinary,OpText};
