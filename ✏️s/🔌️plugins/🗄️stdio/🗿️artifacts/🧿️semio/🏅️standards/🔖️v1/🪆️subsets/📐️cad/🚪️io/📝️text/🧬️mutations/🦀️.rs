//! 📝️ Text representation grammar surface for `s.stdio.semio.cad.mutations`.

pub const COMPONENT_GRAMMAR_SEMIO: &str = include_str!("📖️.grammar.semio");
pub const COMPONENT_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️.grammar.semio");

#[allow(unused_imports)]
mod mutations_codec {
use super::*;
use crate::standards::v1::subsets::cad::schema::mutations::*;
use crate::standards::v1::subsets::base::schema::geometry::SemioPoint2;
use crate::standards::v1::subsets::base::schema::triples::{NamedTripleDiff};
use crate::standards::v1::subsets::base::io::text::snapshot::{split_top_level, strip_brackets};
use crate::standards::v1::subsets::cad::schema::diff::{diff_set_snapshot, wrap_block_diff, wrap_block_entity_diff, wrap_entity_diff, wrap_layer_diff, CadBlockDiff, CadEntityRecordDiff, CadLayerDiff, SemioCadDiff};
use crate::standards::v1::subsets::cad::io::text::snapshot::{dec_entity_record};
use crate::standards::v1::subsets::cad::io::text::snapshot::{enc_entity_record};
use crate::standards::v1::subsets::cad::io::text::snapshot::{dec_layer};
use crate::standards::v1::subsets::cad::io::text::snapshot::{enc_layer};
use crate::standards::v1::subsets::cad::io::text::snapshot::{dec_entity};
use crate::standards::v1::subsets::cad::io::text::snapshot::{enc_entity};
use crate::standards::v1::subsets::cad::io::text::snapshot::{dec_point2};
use crate::standards::v1::subsets::cad::io::text::snapshot::{enc_point2};
use crate::standards::v1::subsets::cad::io::text::snapshot::{dec_block};
use crate::standards::v1::subsets::cad::io::text::snapshot::{enc_block};
use crate::standards::v1::subsets::cad::io::text::snapshot::{dec_list};
use crate::standards::v1::subsets::cad::io::text::snapshot::{enc_list};
use crate::standards::v1::subsets::drawing::io::text::snapshot::{decode_option};
use crate::standards::v1::subsets::drawing::io::text::snapshot::{encode_option};
use crate::standards::v1::subsets::cad::io::text::snapshot::{dec_str};
use crate::standards::v1::subsets::cad::io::text::snapshot::{enc_str};
use crate::standards::v1::subsets::cad::schema::snapshot::{CadBlock, CadEntity, CadEntityRecord, CadLayer, SemioCadSnapshot};
use protocol::OpBinary;
use protocol::{Mutation, OpText};

/// 📥️ Decodes this facet's own internally-tagged (`{"mutation": "<camelCaseVariant>", ...}`) JSON
/// projection — the shape `📐️mutate-semio-cad`'s committed specification vectors carry in their
/// `mutation` member — into a real [`SemioCadMutation`]. A thin `pack::from_json_str` wrapper (over
/// `ToValue`/`FromValue`, first-party, per this ticket's serde→value conversion), so the test adapter reads
/// the committed vector instead of re-declaring it as a Rust literal beside it.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn decode_semio_cad_mutation_json(text: &str) -> Result<SemioCadMutation, String> {
    semio_framework_pack_json::from_json_str(text, semio_framework_pack_json::JsonMemberPolicy::Reject).map_err(|error| error.to_string())
}

/// 🎙️ Hand-rolled `OpText`/`OpBinary` — reuses the diff module's `pub(crate)` grammar primitives
/// (`enc_str`/`enc_layer`/`enc_block`/`enc_entity`/`encode_option`/...) rather than duplicating
/// them, same pattern `BcfMutation` established. Grammar: `keyword arg=value ...`
/// (space-separated), one match arm per variant.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_cad_snapshot(s: &SemioCadSnapshot) -> String {
    format!("[{},{},{},{}]", enc_str(&s.schema), enc_list(&s.layers, enc_layer), enc_list(&s.blocks, enc_block), enc_list(&s.entities, enc_entity_record))
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_cad_snapshot(s: &str) -> Result<SemioCadSnapshot, String> {
    let parts = split_top_level(strip_brackets(s)?, ',');
    let [schema, layers, blocks, entities] = parts.as_slice() else { return Err(format!("cad snapshot: expected 4 fields, got {}", parts.len())) };
    Ok(SemioCadSnapshot { schema: dec_str(schema)?, layers: dec_list(layers, dec_layer)?, blocks: dec_list(blocks, dec_block)?, entities: dec_list(entities, dec_entity_record)? })
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn print_cad_mutation(m: &SemioCadMutation) -> String {
    match m {
        SemioCadMutation::SetSnapshot(set_snapshot::SetSnapshot { snapshot }) => format!("set-snapshot snapshot={}", enc_cad_snapshot(snapshot)),
        SemioCadMutation::PatchSnapshot(patch_snapshot::PatchSnapshot { patch }) => semio_s_artifact_stdio_contract::editing::snapshot_patch_text(patch),
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
        SemioCadMutation::SetBlockEntityGeometry(set_block_entity_geometry::SetBlockEntityGeometry { block_name, handle, entity }) => {
            format!("set-block-entity-geometry block-name={} handle={} entity={}", enc_str(block_name), enc_str(handle), enc_entity(entity))
        }
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn parse_cad_mutation(line: &str) -> Result<SemioCadMutation, String> {
    if let Some(source) = line.strip_prefix("patch-snapshot patch=") {
        let patch = semio_s_artifact_stdio_contract::editing::snapshot_patch_from_hex(source)?;
        return Ok(SemioCadMutation::PatchSnapshot(crate::standards::v1::subsets::cad::schema::mutations::patch_snapshot::PatchSnapshot { patch }));
    }
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
        "set-block-entity-geometry" => {
            Ok(SemioCadMutation::SetBlockEntityGeometry(set_block_entity_geometry::SetBlockEntityGeometry { block_name: dec_str(arg("block-name")?)?, handle: dec_str(arg("handle")?)?, entity: dec_entity(arg("entity")?)? }))
        }
        other => Err(format!("cad mutation: unknown keyword {other:?}")),
    }
}

impl OpText for SemioCadMutation {
    fn print_op(&self) -> String {
        print_cad_mutation(self)
    }
    fn parse_op(line: &str) -> Result<Self, semio_framework_diagnostic::TextError> {
        parse_cad_mutation(line).map_err(|e| semio_framework_diagnostic::TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue, e, semio_framework_diagnostic::TextSpan::at(1, 1)))
    }
}
}
pub use mutations_codec::*;
