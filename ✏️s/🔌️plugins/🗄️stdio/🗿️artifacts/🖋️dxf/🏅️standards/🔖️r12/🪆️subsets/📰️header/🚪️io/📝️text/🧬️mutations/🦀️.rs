//! 📝️ Text representation codec surface for `stdio.dxf` (mutations).

/// 📖️ Grammar include.
pub const COMPONENT_GRAMMAR_SEMIO: &str = include_str!("📖️.grammar.semio");
pub const COMPONENT_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️.grammar.semio");

#[allow(unused_imports)]
mod mutations_codec {
use super::*;
use crate::standards::v_r12::subsets::any::io::binary::diff::dec_block_bin;
use crate::standards::v_r12::subsets::any::schema::mutations::*;
use crate::schema::diff::{block_diff_between, // 🧪️ P2-FG1: real recursive binary twins backing the upgraded `OpBinary` impl below (see
    // `🔺️diff/🦀️.rs`'s `#region 🔖️ItemBinaryCodecs`/`#region 🔖️BinaryPrimitives`).
    diff_insert_block, diff_insert_entity, diff_insert_layer, diff_insert_linetype, diff_insert_style, diff_remove_block, diff_remove_entity, diff_remove_header_var, diff_remove_layer, diff_remove_linetype, diff_remove_style, diff_set_block, diff_set_entity, diff_set_header_var, diff_set_layer, diff_set_linetype, diff_set_snapshot, diff_set_style, entity_diff_between_pub, layer_diff_between, linetype_diff_between, style_diff_between, DxfDiff};
use crate::standards::v_r12::subsets::any::io::binary::snapshot::{dec_dxf_snapshot_bin};
use crate::standards::v_r12::subsets::any::io::binary::snapshot::{enc_dxf_snapshot_bin};
use crate::standards::v_r12::subsets::any::io::text::snapshot::{dec_dxf_snapshot};
use crate::standards::v_r12::subsets::any::io::text::snapshot::{enc_dxf_snapshot};
use crate::standards::v_r12::subsets::any::io::text::diff::{dec_linetype};
use crate::standards::v_r12::subsets::any::io::text::diff::{enc_linetype};
use crate::standards::v_r12::subsets::any::io::text::diff::{dec_style};
use crate::standards::v_r12::subsets::any::io::text::diff::{enc_style};
use crate::standards::v_r12::subsets::any::io::text::diff::{dec_layer};
use crate::standards::v_r12::subsets::any::io::text::diff::{enc_layer};
use crate::standards::v_r12::subsets::any::io::text::diff::{dec_str};
use crate::standards::v_r12::subsets::any::io::text::diff::{enc_str};
use crate::standards::v_r12::subsets::any::io::binary::diff::{dec_linetype_bin};
use crate::standards::v_r12::subsets::any::io::binary::diff::{enc_linetype_bin};
use crate::standards::v_r12::subsets::any::io::binary::diff::{dec_style_bin};
use crate::standards::v_r12::subsets::any::io::binary::diff::{enc_style_bin};
use crate::standards::v_r12::subsets::any::io::binary::diff::{dec_layer_bin};
use crate::standards::v_r12::subsets::any::io::binary::diff::{enc_layer_bin};
use crate::standards::v_r12::subsets::any::io::binary::diff::{read_str_lp};
use crate::standards::v_r12::subsets::any::io::binary::diff::{write_str_lp};
use crate::standards::v_r12::subsets::any::io::text::diff::{dec_block};
use crate::standards::v_r12::subsets::any::io::text::diff::{enc_block};
use crate::standards::v_r12::subsets::any::io::text::diff::{dec_header_var};
use crate::standards::v_r12::subsets::any::io::text::diff::{enc_header_var};
use crate::standards::v_r12::subsets::any::io::text::diff::{dec_dxf_entity};
use crate::standards::v_r12::subsets::any::io::text::diff::{enc_dxf_entity};
use crate::standards::v_r12::subsets::any::io::binary::diff::{enc_block_bin};
use crate::standards::v_r12::subsets::any::io::binary::diff::{dec_header_var_bin};
use crate::standards::v_r12::subsets::any::io::binary::diff::{enc_header_var_bin};
use crate::standards::v_r12::subsets::any::io::binary::diff::{dec_dxf_entity_bin};
use crate::standards::v_r12::subsets::any::io::binary::diff::{enc_dxf_entity_bin};
use crate::schema::snapshot::{DxfBlock, DxfEntity, DxfHeaderVar, DxfLayer, DxfLinetype, DxfStyle};
use crate::DxfSnapshot;
use protocol::OpBinary;
use protocol::{Mutation, MutationDiff, OpText};

/// 🎙️ Handcrafted `print_op`/`parse_op` — one match arm per variant (no `DslVariants` scaffolding
/// available since nothing here derives it, see module doc). Every arg value is either hex
/// (strings), decimal (indices), or a `🔺️diff` positional/tagged payload — never a literal
/// space or `=`, so top-level tokenizing is a trivial `line.split(' ')` / `tok.split_once('=')`.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn print_dxf_mutation(m: &DxfMutation) -> String {
    match m {
        DxfMutation::SetSnapshot(set_snapshot::SetSnapshot { snapshot }) => format!("set-snapshot snapshot={}", enc_dxf_snapshot(snapshot)),
        DxfMutation::PatchSnapshot(patch_snapshot::PatchSnapshot { patch }) => semio_s_artifact_stdio_contract::editing::snapshot_patch_text(patch),

        DxfMutation::SetHeaderVar(set_header_var::SetHeaderVar { name, header_var }) => format!("set-header-var name={} header-var={}", enc_str(name), enc_header_var(header_var)),
        DxfMutation::RemoveHeaderVar(remove_header_var::RemoveHeaderVar { name }) => format!("remove-header-var name={}", enc_str(name)),

        DxfMutation::InsertLayer(insert_layer::InsertLayer { index, layer }) => format!("insert-layer index={index} layer={}", enc_layer(layer)),
        DxfMutation::RemoveLayer(remove_layer::RemoveLayer { name }) => format!("remove-layer name={}", enc_str(name)),
        DxfMutation::SetLayer(set_layer::SetLayer { name, layer }) => format!("set-layer name={} layer={}", enc_str(name), enc_layer(layer)),

        DxfMutation::InsertStyle(insert_style::InsertStyle { index, style }) => format!("insert-style index={index} style={}", enc_style(style)),
        DxfMutation::RemoveStyle(remove_style::RemoveStyle { name }) => format!("remove-style name={}", enc_str(name)),
        DxfMutation::SetStyle(set_style::SetStyle { name, style }) => format!("set-style name={} style={}", enc_str(name), enc_style(style)),

        DxfMutation::InsertLinetype(insert_linetype::InsertLinetype { index, linetype }) => format!("insert-linetype index={index} linetype={}", enc_linetype(linetype)),
        DxfMutation::RemoveLinetype(remove_linetype::RemoveLinetype { name }) => format!("remove-linetype name={}", enc_str(name)),
        DxfMutation::SetLinetype(set_linetype::SetLinetype { name, linetype }) => format!("set-linetype name={} linetype={}", enc_str(name), enc_linetype(linetype)),

        DxfMutation::InsertEntity(insert_entity::InsertEntity { index, entity }) => format!("insert-entity index={index} entity={}", enc_dxf_entity(entity)),
        DxfMutation::RemoveEntity(remove_entity::RemoveEntity { index }) => format!("remove-entity index={index}"),
        DxfMutation::SetEntity(set_entity::SetEntity { index, entity }) => format!("set-entity index={index} entity={}", enc_dxf_entity(entity)),

        DxfMutation::InsertBlock(insert_block::InsertBlock { index, block }) => format!("insert-block index={index} block={}", enc_block(block)),
        DxfMutation::RemoveBlock(remove_block::RemoveBlock { index }) => format!("remove-block index={index}"),
        DxfMutation::SetBlock(set_block::SetBlock { index, block }) => format!("set-block index={index} block={}", enc_block(block)),
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn parse_dxf_mutation(line: &str) -> Result<DxfMutation, String> {
    let (keyword, rest) = line.split_once(' ').unwrap_or((line, ""));
    let args: std::collections::BTreeMap<&str, &str> = rest.split(' ').filter(|s| !s.is_empty()).map(|tok| tok.split_once('=').ok_or_else(|| format!("dxf mutation: bad arg token {tok:?}"))).collect::<Result<Vec<_>, String>>()?.into_iter().collect();
    let arg = |k: &str| args.get(k).copied().ok_or_else(|| format!("dxf mutation: missing arg '{k}' for '{keyword}'"));
    let usize_arg = |k: &str| -> Result<usize, String> { arg(k)?.parse().map_err(|e: std::num::ParseIntError| e.to_string()) };
    match keyword {
        "patch-snapshot" => semio_s_artifact_stdio_contract::editing::snapshot_patch_from_text(line).map(|patch| DxfMutation::PatchSnapshot(patch_snapshot::PatchSnapshot { patch })),
        "set-snapshot" => Ok(DxfMutation::SetSnapshot(set_snapshot::SetSnapshot { snapshot: dec_dxf_snapshot(arg("snapshot")?)? })),

        "set-header-var" => Ok(DxfMutation::SetHeaderVar(set_header_var::SetHeaderVar { name: dec_str(arg("name")?)?, header_var: dec_header_var(arg("header-var")?)? })),
        "remove-header-var" => Ok(DxfMutation::RemoveHeaderVar(remove_header_var::RemoveHeaderVar { name: dec_str(arg("name")?)? })),

        "insert-layer" => Ok(DxfMutation::InsertLayer(insert_layer::InsertLayer { index: usize_arg("index")?, layer: dec_layer(arg("layer")?)? })),
        "remove-layer" => Ok(DxfMutation::RemoveLayer(remove_layer::RemoveLayer { name: dec_str(arg("name")?)? })),
        "set-layer" => Ok(DxfMutation::SetLayer(set_layer::SetLayer { name: dec_str(arg("name")?)?, layer: dec_layer(arg("layer")?)? })),

        "insert-style" => Ok(DxfMutation::InsertStyle(insert_style::InsertStyle { index: usize_arg("index")?, style: dec_style(arg("style")?)? })),
        "remove-style" => Ok(DxfMutation::RemoveStyle(remove_style::RemoveStyle { name: dec_str(arg("name")?)? })),
        "set-style" => Ok(DxfMutation::SetStyle(set_style::SetStyle { name: dec_str(arg("name")?)?, style: dec_style(arg("style")?)? })),

        "insert-linetype" => Ok(DxfMutation::InsertLinetype(insert_linetype::InsertLinetype { index: usize_arg("index")?, linetype: dec_linetype(arg("linetype")?)? })),
        "remove-linetype" => Ok(DxfMutation::RemoveLinetype(remove_linetype::RemoveLinetype { name: dec_str(arg("name")?)? })),
        "set-linetype" => Ok(DxfMutation::SetLinetype(set_linetype::SetLinetype { name: dec_str(arg("name")?)?, linetype: dec_linetype(arg("linetype")?)? })),

        "insert-entity" => Ok(DxfMutation::InsertEntity(insert_entity::InsertEntity { index: usize_arg("index")?, entity: dec_dxf_entity(arg("entity")?)? })),
        "remove-entity" => Ok(DxfMutation::RemoveEntity(remove_entity::RemoveEntity { index: usize_arg("index")? })),
        "set-entity" => Ok(DxfMutation::SetEntity(set_entity::SetEntity { index: usize_arg("index")?, entity: dec_dxf_entity(arg("entity")?)? })),

        "insert-block" => Ok(DxfMutation::InsertBlock(insert_block::InsertBlock { index: usize_arg("index")?, block: dec_block(arg("block")?)? })),
        "remove-block" => Ok(DxfMutation::RemoveBlock(remove_block::RemoveBlock { index: usize_arg("index")? })),
        "set-block" => Ok(DxfMutation::SetBlock(set_block::SetBlock { index: usize_arg("index")?, block: dec_block(arg("block")?)? })),

        other => Err(format!("dxf mutation: unknown keyword {other:?}")),
    }
}

impl OpText for DxfMutation {
    fn print_op(&self) -> String {
        print_dxf_mutation(self)
    }
    fn parse_op(line: &str) -> Result<Self, semio_framework_diagnostic::TextError> {
        parse_dxf_mutation(line).map_err(|e| semio_framework_diagnostic::TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue, e, semio_framework_diagnostic::TextSpan::at(1, 1)))
    }
}
}
pub use mutations_codec::*;
