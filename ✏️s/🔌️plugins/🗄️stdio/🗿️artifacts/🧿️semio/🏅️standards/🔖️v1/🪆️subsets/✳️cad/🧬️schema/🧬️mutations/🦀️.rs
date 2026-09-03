//! 🧬️ SemioCadMutation — document mutation dispatch. Every variant's `diff()` is handcrafted
//! (never apply-and-capture) via the diff module's `wrap_*_diff` helpers; every variant's
//! `inverse()` looks up prior state from `base` and constructs the exact undoing mutation
//! (name/handle-aware, matching bcf/docx precedent).

use crate::artifacts::semio::standards::v1::subsets::base::schema::geometry::SemioPoint2;
use crate::artifacts::semio::standards::v1::subsets::base::schema::triples::{split_top_level, strip_brackets, NamedTripleDiff};
use crate::artifacts::semio::standards::v1::subsets::cad::schema::diff::{
    dec_block, dec_entity, dec_entity_record, dec_layer, dec_list, dec_point2, dec_str, decode_option, diff_set_snapshot, enc_block, enc_entity, enc_entity_record, enc_layer, enc_list, enc_point2, enc_str, encode_option, wrap_block_diff,
    wrap_block_entity_diff, wrap_entity_diff, wrap_layer_diff, CadBlockDiff, CadEntityRecordDiff, CadLayerDiff, SemioCadDiff,
};
use crate::artifacts::semio::standards::v1::subsets::cad::schema::snapshot::{CadBlock, CadEntity, CadEntityRecord, CadLayer, SemioCadSnapshot};
use protocol::OpBinary;
use protocol::{Mutation, OpText};

//#region 🔖️Mutations
/// 📐️ Typed document mutation for `stdio.semio.cad`. Every variant addresses one facet of the CAD
/// document; `NoMutation` was dropped: `#[derive(dsl::Mutations)]` requires every variant to wrap
/// exactly one leaf payload and a unit variant wraps none (same consequence tiff's baseline
/// migration reached — see `🖼️tiff/🏅️standards/🔖️6.0/🪆️subsets/✳️baseline/🧬️schema/🧬️mutations/🦀️.rs`).
//#region 🔖️Leaves
#[path = "📄set-snapshot/🦀️.rs"]
pub mod set_snapshot;
#[path = "🗂add-layer/🦀️.rs"]
pub mod add_layer;
#[path = "🧹remove-layer/🦀️.rs"]
pub mod remove_layer;
#[path = "🎚set-layer/🦀️.rs"]
pub mod set_layer;
#[path = "🧱add-block/🦀️.rs"]
pub mod add_block;
#[path = "🚫remove-block/🦀️.rs"]
pub mod remove_block;
#[path = "📍set-block-base-point/🦀️.rs"]
pub mod set_block_base_point;
#[path = "🔷add-entity/🦀️.rs"]
pub mod add_entity;
#[path = "🗑remove-entity/🦀️.rs"]
pub mod remove_entity;
#[path = "🏳set-entity-layer/🦀️.rs"]
pub mod set_entity_layer;
#[path = "📐set-entity-geometry/🦀️.rs"]
pub mod set_entity_geometry;
#[path = "🧩add-block-entity/🦀️.rs"]
pub mod add_block_entity;
#[path = "✂remove-block-entity/🦀️.rs"]
pub mod remove_block_entity;
#[path = "🏌set-block-entity-layer/🦀️.rs"]
pub mod set_block_entity_layer;
#[path = "🔺set-block-entity-geometry/🦀️.rs"]
pub mod set_block_entity_geometry;
//#endregion 🔖️Leaves

#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::Mutations)]
#[mutations(snapshot = SemioCadSnapshot, diff = SemioCadDiff, schema = "SemioCadMutation")]
#[value(tag = "mutation", rename_all = "camelCase")]
pub enum SemioCadMutation {
    SetSnapshot(set_snapshot::SetSnapshot),
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
/// `tag` ordinal (see [`variant_ordinal`]), for `parse_cad_mutation`'s keyword match, and for the
/// `semio-v1-cad` catalog in `../../🔣️oracle.json`. The framework never parses Rust, so
/// `kinds_match_the_enum_and_the_catalog` below is what keeps all three honest.
pub const KINDS: &[&str] = &[
    "set-snapshot",
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
    "set-block-entity-geometry",
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
/// `protocol` extern-crate item (the `mutate-semio-cad` test adapter, whose `inverse-<kind>`
/// scenarios need a mutation's own computed inverse) can still reach the inverse law that
/// [`apply_semio_cad_mutation`] alone cannot. Same shape as `✳️kit`'s `inverse_semio_kit_mutation`.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn inverse_semio_cad_mutation(mutation: &SemioCadMutation, base: &SemioCadSnapshot) -> Vec<SemioCadMutation> {
    <SemioCadMutation as Mutation<SemioCadSnapshot>>::inverse(mutation, base)
}

/// 📥️ Decodes this facet's own internally-tagged (`{"mutation": "<camelCaseVariant>", ...}`) JSON
/// projection — the shape `mutate-semio-cad`'s committed specification vectors carry in their
/// `mutation` member — into a real [`SemioCadMutation`]. A thin `pack::from_json_str` wrapper (over
/// `ToValue`/`FromValue`, first-party, per this ticket's serde→value conversion), so the test adapter reads
/// the committed vector instead of re-declaring it as a Rust literal beside it.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn decode_semio_cad_mutation_json(text: &str) -> Result<SemioCadMutation, String> {
    pack::from_json_str(text).map_err(|error| error.to_string())
}
//#endregion 🔖️Apply

//#region 🔖️MutationTrait
// 🚫️async: E1 pure codec/computation helper — lifted verbatim from the former `impl Mutation`.
pub(crate) fn agg_diff(this: &SemioCadMutation, base: &SemioCadSnapshot) -> protocol::MutationOutcome<SemioCadDiff> {
    protocol::MutationOutcome::new(match this {
        SemioCadMutation::SetSnapshot(set_snapshot::SetSnapshot { snapshot }) => diff_set_snapshot(base, snapshot),
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
        SemioCadMutation::AddBlockEntity(add_block_entity::AddBlockEntity { block_name, entity }) => wrap_block_diff(block_name, CadBlockDiff { base_point: None, entities: Some(NamedTripleDiff { removed: Vec::new(), modified: Vec::new(), added: vec![entity.clone()] }) }),
        SemioCadMutation::RemoveBlockEntity(remove_block_entity::RemoveBlockEntity { block_name, handle }) => wrap_block_diff(block_name, CadBlockDiff { base_point: None, entities: Some(NamedTripleDiff { removed: vec![handle.clone()], modified: Vec::new(), added: Vec::new() }) }),
        SemioCadMutation::SetBlockEntityLayer(set_block_entity_layer::SetBlockEntityLayer { block_name, handle, layer }) => wrap_block_entity_diff(block_name, handle, CadEntityRecordDiff { layer: Some(layer.clone()), entity: None }),
        SemioCadMutation::SetBlockEntityGeometry(set_block_entity_geometry::SetBlockEntityGeometry { block_name, handle, entity }) => wrap_block_entity_diff(block_name, handle, CadEntityRecordDiff { layer: None, entity: Some(entity.clone()) }),
    })
}

// 🚫️async: E1 pure codec/computation helper — lifted verbatim from the former `impl Mutation`.
pub(crate) fn agg_inverse(this: &SemioCadMutation, base: &SemioCadSnapshot) -> Vec<SemioCadMutation> {
    match this {
        SemioCadMutation::SetSnapshot(_) => vec![SemioCadMutation::SetSnapshot(set_snapshot::SetSnapshot { snapshot: base.clone() })],
        SemioCadMutation::AddLayer(add_layer::AddLayer { layer }) => vec![SemioCadMutation::RemoveLayer(remove_layer::RemoveLayer { name: layer.name.clone() })],
        SemioCadMutation::RemoveLayer(remove_layer::RemoveLayer { name }) => match find_layer(base, name) {
            Some(l) => vec![SemioCadMutation::AddLayer(add_layer::AddLayer { layer: l.clone() })],
            None => Vec::new(),
        },
        SemioCadMutation::SetLayer(set_layer::SetLayer { name, color_index, line_type, visible }) => match find_layer(base, name) {
            Some(l) => vec![SemioCadMutation::SetLayer(set_layer::SetLayer { name: name.clone(), color_index: color_index.as_ref().map(|_| l.color_index), line_type: line_type.as_ref().map(|_| l.line_type.clone()), visible: visible.as_ref().map(|_| l.visible) })],
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
/// 🎙️ Hand-rolled `OpText`/`OpBinary` — reuses the diff module's `pub(crate)` grammar primitives
/// (`enc_str`/`enc_layer`/`enc_block`/`enc_entity`/`encode_option`/...) rather than duplicating
/// them, same pattern `BcfMutation` established. Grammar: `keyword arg=value ...`
/// (space-separated), one match arm per variant.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn enc_cad_snapshot(s: &SemioCadSnapshot) -> String {
    format!("[{},{},{},{}]", enc_str(&s.schema), enc_list(&s.layers, enc_layer), enc_list(&s.blocks, enc_block), enc_list(&s.entities, enc_entity_record))
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn dec_cad_snapshot(s: &str) -> Result<SemioCadSnapshot, String> {
    let parts = split_top_level(strip_brackets(s)?, ',');
    let [schema, layers, blocks, entities] = parts.as_slice() else { return Err(format!("cad snapshot: expected 4 fields, got {}", parts.len())) };
    Ok(SemioCadSnapshot { schema: dec_str(schema)?, layers: dec_list(layers, dec_layer)?, blocks: dec_list(blocks, dec_block)?, entities: dec_list(entities, dec_entity_record)? })
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn print_cad_mutation(m: &SemioCadMutation) -> String {
    match m {
        SemioCadMutation::SetSnapshot(set_snapshot::SetSnapshot { snapshot }) => format!("set-snapshot snapshot={}", enc_cad_snapshot(snapshot)),
        SemioCadMutation::AddLayer(add_layer::AddLayer { layer }) => format!("add-layer layer={}", enc_layer(layer)),
        SemioCadMutation::RemoveLayer(remove_layer::RemoveLayer { name }) => format!("remove-layer name={}", enc_str(name)),
        SemioCadMutation::SetLayer(set_layer::SetLayer { name, color_index, line_type, visible }) => format!(
            "set-layer name={} color-index={} line-type={} visible={}",
            enc_str(name),
            encode_option(color_index, |v: &i32| v.to_string()),
            encode_option(line_type, |v: &String| enc_str(v)),
            encode_option(visible, |v: &bool| if *v { "1".to_string() } else { "0".to_string() }),
        ),
        SemioCadMutation::AddBlock(add_block::AddBlock { block }) => format!("add-block block={}", enc_block(block)),
        SemioCadMutation::RemoveBlock(remove_block::RemoveBlock { name }) => format!("remove-block name={}", enc_str(name)),
        SemioCadMutation::SetBlockBasePoint(set_block_base_point::SetBlockBasePoint { name, base_point }) => format!("set-block-base-point name={} base-point={}", enc_str(name), enc_point2(base_point)),
        SemioCadMutation::AddEntity(add_entity::AddEntity { entity }) => format!("add-entity entity={}", enc_entity_record(entity)),
        SemioCadMutation::RemoveEntity(remove_entity::RemoveEntity { handle }) => format!("remove-entity handle={}", enc_str(handle)),
        SemioCadMutation::SetEntityLayer(set_entity_layer::SetEntityLayer { handle, layer }) => format!("set-entity-layer handle={} layer={}", enc_str(handle), enc_str(layer)),
        SemioCadMutation::SetEntityGeometry(set_entity_geometry::SetEntityGeometry { handle, entity }) => format!("set-entity-geometry handle={} entity={}", enc_str(handle), enc_entity(entity)),
        SemioCadMutation::AddBlockEntity(add_block_entity::AddBlockEntity { block_name, entity }) => format!("add-block-entity block-name={} entity={}", enc_str(block_name), enc_entity_record(entity)),
        SemioCadMutation::RemoveBlockEntity(remove_block_entity::RemoveBlockEntity { block_name, handle }) => format!("remove-block-entity block-name={} handle={}", enc_str(block_name), enc_str(handle)),
        SemioCadMutation::SetBlockEntityLayer(set_block_entity_layer::SetBlockEntityLayer { block_name, handle, layer }) => format!("set-block-entity-layer block-name={} handle={} layer={}", enc_str(block_name), enc_str(handle), enc_str(layer)),
        SemioCadMutation::SetBlockEntityGeometry(set_block_entity_geometry::SetBlockEntityGeometry { block_name, handle, entity }) => format!("set-block-entity-geometry block-name={} handle={} entity={}", enc_str(block_name), enc_str(handle), enc_entity(entity)),
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn parse_cad_mutation(line: &str) -> Result<SemioCadMutation, String> {
    let (keyword, rest) = line.split_once(' ').unwrap_or((line, ""));
    let args: std::collections::BTreeMap<&str, &str> = rest.split(' ').filter(|s| !s.is_empty()).map(|tok| tok.split_once('=').ok_or_else(|| format!("cad mutation: bad arg token {tok:?}"))).collect::<Result<Vec<_>, String>>()?.into_iter().collect();
    let arg = |k: &str| args.get(k).copied().ok_or_else(|| format!("cad mutation: missing arg '{k}' for '{keyword}'"));
    match keyword {
        "set-snapshot" => Ok(SemioCadMutation::SetSnapshot(set_snapshot::SetSnapshot { snapshot: dec_cad_snapshot(arg("snapshot")?)? })),
        "add-layer" => Ok(SemioCadMutation::AddLayer(add_layer::AddLayer { layer: dec_layer(arg("layer")?)? })),
        "remove-layer" => Ok(SemioCadMutation::RemoveLayer(remove_layer::RemoveLayer { name: dec_str(arg("name")?)? })),
        "set-layer" => Ok(SemioCadMutation::SetLayer(set_layer::SetLayer {
            name: dec_str(arg("name")?)?,
            color_index: decode_option(arg("color-index")?, |v| v.parse::<i32>().map_err(|e: std::num::ParseIntError| e.to_string()))?,
            line_type: decode_option(arg("line-type")?, dec_str)?,
            visible: decode_option(arg("visible")?, |v| Ok(v == "1"))?,
        })),
        "add-block" => Ok(SemioCadMutation::AddBlock(add_block::AddBlock { block: dec_block(arg("block")?)? })),
        "remove-block" => Ok(SemioCadMutation::RemoveBlock(remove_block::RemoveBlock { name: dec_str(arg("name")?)? })),
        "set-block-base-point" => Ok(SemioCadMutation::SetBlockBasePoint(set_block_base_point::SetBlockBasePoint { name: dec_str(arg("name")?)?, base_point: dec_point2(arg("base-point")?)? })),
        "add-entity" => Ok(SemioCadMutation::AddEntity(add_entity::AddEntity { entity: dec_entity_record(arg("entity")?)? })),
        "remove-entity" => Ok(SemioCadMutation::RemoveEntity(remove_entity::RemoveEntity { handle: dec_str(arg("handle")?)? })),
        "set-entity-layer" => Ok(SemioCadMutation::SetEntityLayer(set_entity_layer::SetEntityLayer { handle: dec_str(arg("handle")?)?, layer: dec_str(arg("layer")?)? })),
        "set-entity-geometry" => Ok(SemioCadMutation::SetEntityGeometry(set_entity_geometry::SetEntityGeometry { handle: dec_str(arg("handle")?)?, entity: dec_entity(arg("entity")?)? })),
        "add-block-entity" => Ok(SemioCadMutation::AddBlockEntity(add_block_entity::AddBlockEntity { block_name: dec_str(arg("block-name")?)?, entity: dec_entity_record(arg("entity")?)? })),
        "remove-block-entity" => Ok(SemioCadMutation::RemoveBlockEntity(remove_block_entity::RemoveBlockEntity { block_name: dec_str(arg("block-name")?)?, handle: dec_str(arg("handle")?)? })),
        "set-block-entity-layer" => Ok(SemioCadMutation::SetBlockEntityLayer(set_block_entity_layer::SetBlockEntityLayer { block_name: dec_str(arg("block-name")?)?, handle: dec_str(arg("handle")?)?, layer: dec_str(arg("layer")?)? })),
        "set-block-entity-geometry" => Ok(SemioCadMutation::SetBlockEntityGeometry(set_block_entity_geometry::SetBlockEntityGeometry { block_name: dec_str(arg("block-name")?)?, handle: dec_str(arg("handle")?)?, entity: dec_entity(arg("entity")?)? })),
        other => Err(format!("cad mutation: unknown keyword {other:?}")),
    }
}

impl OpText for SemioCadMutation {
    fn print_op(&self) -> String {
        print_cad_mutation(self)
    }
    fn parse_op(line: &str) -> Result<Self, store::TextError> {
        parse_cad_mutation(line).map_err(|e| store::TextError::new(e, dsl::TextSpan::at(1, 1)))
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn variant_ordinal(m: &SemioCadMutation) -> u8 {
    match m {
        SemioCadMutation::SetSnapshot(_) => 0,
        SemioCadMutation::AddLayer(_) => 1,
        SemioCadMutation::RemoveLayer(_) => 2,
        SemioCadMutation::SetLayer(_) => 3,
        SemioCadMutation::AddBlock(_) => 4,
        SemioCadMutation::RemoveBlock(_) => 5,
        SemioCadMutation::SetBlockBasePoint(_) => 6,
        SemioCadMutation::AddEntity(_) => 7,
        SemioCadMutation::RemoveEntity(_) => 8,
        SemioCadMutation::SetEntityLayer(_) => 9,
        SemioCadMutation::SetEntityGeometry(_) => 10,
        SemioCadMutation::AddBlockEntity(_) => 11,
        SemioCadMutation::RemoveBlockEntity(_) => 12,
        SemioCadMutation::SetBlockEntityLayer(_) => 13,
        SemioCadMutation::SetBlockEntityGeometry(_) => 14,
    }
}
/// ✂️ Just the `key=value ...` argument tail of `print_cad_mutation` — the binary frame's `tag`
/// byte already carries the keyword, so the text keyword itself is redundant in the binary payload.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn print_cad_mutation_args(m: &SemioCadMutation) -> String {
    match print_cad_mutation(m).split_once(' ') {
        Some((_, rest)) => rest.to_string(),
        None => String::new(),
    }
}

/// ⚡️ Real binary op frame, replacing the old `print_op().into_bytes()` text-as-binary shortcut.
/// `format u8` (`OP_BINARY_FORMAT` convention) + `tag u8` (the variant ordinal, see
/// [`KINDS`]) are two REAL fixed fields; the variant's own `key=value ...` argument payload
/// follows as one opaque trailing `bytes` chain — reusing the already-real, already-tested
/// `print_cad_mutation`/`parse_cad_mutation` text codec rather than re-deriving a second
/// independent encoding.
impl OpBinary for SemioCadMutation {
    fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        const OP_BINARY_FORMAT: u8 = 1;
        let mut out = vec![OP_BINARY_FORMAT, variant_ordinal(self)];
        out.extend_from_slice(print_cad_mutation_args(self).as_bytes());
        Ok(out)
    }
    fn decode_op(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        const OP_BINARY_FORMAT: u8 = 1;
        if bytes.len() < 2 {
            return Err(protocol::ProtocolError::Malformed { what: "op header", offset: 0, detail: "truncated (need format+tag)".to_string() });
        }
        if bytes[0] != OP_BINARY_FORMAT {
            return Err(protocol::ProtocolError::Malformed { what: "op format", offset: 0, detail: format!("unsupported op format {}", bytes[0]) });
        }
        let tag = bytes[1];
        let keyword = KINDS.get(tag as usize).ok_or_else(|| protocol::ProtocolError::Malformed { what: "op tag", offset: 1, detail: format!("tag {tag} out of range for {} declared variants", KINDS.len()) })?;
        let args = std::str::from_utf8(&bytes[2..]).map_err(|e| protocol::ProtocolError::Malformed { what: "op utf8", offset: 2, detail: e.to_string() })?;
        let line = if args.is_empty() { keyword.to_string() } else { format!("{keyword} {args}") };
        Self::parse_op(&line).map_err(|e| protocol::ProtocolError::Malformed { what: "op text", offset: 2, detail: e.to_string() })
    }
}
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
        schema: crate::artifacts::semio::standards::v1::subsets::cad::schema::snapshot::STDIO_SEMIOCAD_DOCUMENT_SCHEMA.into(),
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
        SemioCadMutation::SetEntityGeometry(set_entity_geometry::SetEntityGeometry { handle: "h1".into(), entity: CadEntity::Ellipse { center: SemioPoint2 { x: 0.0, y: 0.0 }, major_axis_end: SemioPoint2 { x: 1.0, y: 0.0 }, ratio: 0.5, start_param: 0.0, end_param: 6.28 } }),
        SemioCadMutation::AddBlockEntity(add_block_entity::AddBlockEntity {
            block_name: "door".into(),
            entity: CadEntityRecord { handle: "be2".into(), layer: "0".into(), entity: CadEntity::Text { position: SemioPoint2 { x: 0.0, y: 0.0 }, height: 2.5, rotation: 0.0, content: "label".into() } },
        }),
        SemioCadMutation::RemoveBlockEntity(remove_block_entity::RemoveBlockEntity { block_name: "door".into(), handle: "be1".into() }),
        SemioCadMutation::SetBlockEntityLayer(set_block_entity_layer::SetBlockEntityLayer { block_name: "door".into(), handle: "be1".into(), layer: "dim".into() }),
        SemioCadMutation::SetBlockEntityGeometry(set_block_entity_geometry::SetBlockEntityGeometry { block_name: "door".into(), handle: "be1".into(), entity: CadEntity::Arc { center: SemioPoint2 { x: 0.0, y: 0.0 }, radius: 1.0, start_angle: 0.0, end_angle: 90.0 } }),
    ]
}
//#endregion 🔖️Demo

//#region 🔖️Tests
#[cfg(test)]
mod tests {
    use super::*;

    //#region 🧪️KindsCatalog
    /// 🏷️ [`KINDS`] must name every declared variant, in the exact order and spelling the binary op
    /// frame's `tag` ordinal and the text grammar's keyword both use, and every one of those
    /// spellings must also appear in the committed oracle manifest's catalog. The framework never
    /// parses Rust, so this is what makes the declaration honest.
    #[test]
    fn kinds_match_the_enum_and_the_catalog() {
        assert_eq!(KINDS.len(), 15, "KINDS must name exactly one entry per declared SemioCadMutation variant");
        let mut seen = vec![false; KINDS.len()];
        for m in demo_mutation_cases() {
            let keyword = print_cad_mutation(&m).split(' ').next().expect("printed op is never empty").to_string();
            let ordinal = variant_ordinal(&m) as usize;
            assert_eq!(KINDS[ordinal], keyword, "KINDS must match the declaration order and spelling for {m:?}");
            seen[ordinal] = true;
        }
        assert!(seen.iter().all(|hit| *hit), "demo_mutation_cases must reach every KINDS entry, missing {:?}", KINDS.iter().zip(seen.iter()).filter(|(_, hit)| !**hit).map(|(kind, _)| *kind).collect::<Vec<_>>());
        let manifest = include_str!("../../🧪️oracle/🔣️.json");
        for kind in KINDS {
            assert!(manifest.contains(&format!("\"{kind}\"")), "KINDS entry {kind:?} must also appear in the committed oracle manifest's catalog");
        }
    }
    //#endregion 🧪️KindsCatalog

    //#region 🧪️Law1_MutationDiffLaw
    /// ⚖️ Law 1 — `mutation_diff_law`: for every variant, `apply_semio_cad_mutation`'s returned
    /// diff equals `m.diff(base)`, and applying it matches `diff.diff().apply(base)`.
    #[semio_framework_async_macros::async_test]
    async fn mutation_diff_law() {
        let base = fixture();
        for m in demo_mutation_cases() {
            let mut snap = base.clone();
            let returned = apply_semio_cad_mutation(&mut snap, &m);
            let expected_diff = m.diff(&base);
            assert_eq!(returned, expected_diff, "returned diff mismatch for {m:?}");
            assert_eq!(snap, protocol::MutationDiff::apply(expected_diff.diff(), &base).expect("apply must succeed for a well-formed fixture"), "apply mismatch for {m:?}");
        }
    }
    //#endregion

    //#region 🧪️Law2_InverseLaw
    /// ⚖️ Law 2 — `inverse_law`: every mutation round-trips (mutation-level) and every diff
    /// round-trips (diff-level `d.diff().inverse(base).apply(&d.diff().apply(base)) == base`).
    #[semio_framework_async_macros::async_test]
    async fn inverse_law() {
        use protocol::command::DiffAlgebra;
        let base = fixture();
        for m in demo_mutation_cases() {
            let mut snap = base.clone();
            apply_semio_cad_mutation(&mut snap, &m);
            for inv in m.inverse(&base) {
                let mut undone = snap.clone();
                apply_semio_cad_mutation(&mut undone, &inv);
                assert_eq!(undone, base, "mutation-level inverse mismatch for {m:?}");
            }

            let d = m.diff(&base);
            let after = protocol::MutationDiff::apply(d.diff(), &base).expect("apply must succeed for a well-formed fixture");
            let d_inv = d.diff().inverse(&base);
            assert_eq!(protocol::MutationDiff::apply(&d_inv, &after).expect("apply must succeed for a well-formed fixture"), base, "diff-level inverse mismatch for {m:?}");
        }
    }
    //#endregion

    //#region 🧪️Law7_OpTextBinaryRoundtripLaw
    /// ⚖️ Law 7 — `op_text_binary_roundtrip_law`: `OpText`/`OpBinary` round-trip for the
    /// hand-rolled `SemioCadMutation` grammar, covering every variant via [`demo_mutation_cases`].
    #[semio_framework_async_macros::async_test]
    async fn op_text_binary_roundtrip_law() {
        for m in demo_mutation_cases() {
            let printed = m.print_op();
            assert!(!printed.contains('\n'), "print_op must be one line, got {printed:?}");
            let parsed = SemioCadMutation::parse_op(&printed).unwrap_or_else(|e| panic!("parse_op({printed:?}) failed: {e}"));
            assert_eq!(parsed, m, "print_op/parse_op round-trip mismatch for {m:?} (printed {printed:?})");

            let encoded = m.encode_op().unwrap_or_else(|e| panic!("encode_op({m:?}) failed: {e}"));
            let decoded = SemioCadMutation::decode_op(&encoded).unwrap_or_else(|e| panic!("decode_op failed: {e}"));
            assert_eq!(decoded, m, "encode_op/decode_op round-trip mismatch for {m:?}");
        }
    }
    //#endregion
}
//#endregion 🔖️Tests

//#region 🧪️FixtureCases
/// 🧪️ Handcrafted `📄set-snapshot` fixture cases, wired from this tree's own mutations root so
/// `🦀️.rs` stays untouched (`#[path]` on a non-inline module resolves against this file's own
/// directory).
#[cfg(test)]
#[path = "📄set-snapshot/🧪️tests/dims-the-walls-layer-and-widens-the-circle/🦀️.rs"]
mod set_snapshot_dims_the_walls_layer_and_widens_the_circle;
//#endregion 🧪️FixtureCases
