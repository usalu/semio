//! 📝️ Text representation codec surface for `stdio.ifc` (mutations).

/// 📖️ Grammar include.
pub const COMPONENT_GRAMMAR_SEMIO: &str = include_str!("📖️.grammar.semio");
pub const COMPONENT_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️.grammar.semio");

#[allow(unused_imports)]
mod mutations_codec {
use super::*;
use crate::standards::v4::subsets::any::schema::mutations::*;
use crate::schema::diff::{self, IfcDiff};
use crate::standards::v4::subsets::any::io::text::diff::{dec_entity};
use crate::standards::v4::subsets::any::io::text::diff::{enc_entity};
use crate::standards::v4::subsets::any::io::text::diff::{dec_ifc_value};
use crate::standards::v4::subsets::any::io::text::diff::{enc_ifc_value};
use crate::standards::v4::subsets::any::io::binary::diff::{dec_entity_bin};
use crate::standards::v4::subsets::any::io::binary::diff::{enc_entity_bin};
use crate::standards::v4::subsets::any::io::binary::diff::{dec_ifc_value_bin};
use crate::standards::v4::subsets::any::io::binary::diff::{enc_ifc_value_bin};
use crate::standards::v4::subsets::any::io::text::diff::{dec_ifc_value_list};
use crate::standards::v4::subsets::any::io::text::diff::{enc_ifc_value_list};
use crate::standards::v4::subsets::any::io::binary::diff::{dec_ifc_value_list_bin};
use crate::standards::v4::subsets::any::io::binary::diff::{enc_ifc_value_list_bin};
use crate::standards::v2x3::subsets::base::io::text::diff::{strip_brackets};
use crate::standards::v2x3::subsets::base::io::text::diff::{split_top_level};
use crate::standards::v2x3::subsets::base::io::text::diff::{dec_str};
use crate::standards::v2x3::subsets::base::io::text::diff::{enc_str};
use crate::standards::v2x3::subsets::base::io::binary::diff::{read_str_bin};
use crate::standards::v2x3::subsets::base::io::binary::diff::{write_str_bin};
use crate::schema::snapshot::{IfcEntity, IfcHeader, IfcValue};
use crate::IfcSnapshot;
use protocol::OpBinary;
use protocol::{Mutation, OpText};

/// 🧪️ F6: **hand-rolled** `OpText`/`OpBinary` for `IfcMutation` (`#[derive(dsl::DslOps)]` confirmed
/// rejected above) — reuses `IfcDiff`'s `pub(crate)` grammar primitives
/// (`enc_str`/`enc_ifc_value`/`enc_entity`/`split_top_level`/`encode_option`/...) rather than
/// duplicating them a second time in this file. Grammar: `keyword arg=value ...` (space-separated,
/// same shape the derive's own handcrafted-wrapper convention uses), one match arm per variant (no
/// `DslVariants` scaffolding available since nothing here derives it).
pub(crate) fn enc_ifc_header(h: &IfcHeader) -> String {
    format!("[{},{},{}]", enc_ifc_value_list(&h.file_description), enc_ifc_value_list(&h.file_name), enc_ifc_value_list(&h.file_schema))
}

pub(crate) fn dec_ifc_header(s: &str) -> Result<IfcHeader, String> {
    let parts = split_top_level(strip_brackets(s)?, ',');
    let [fd, fname, fs] = parts.as_slice() else { return Err(format!("ifc header: expected 3 fields, got {}", parts.len())) };
    Ok(IfcHeader { file_description: dec_ifc_value_list(fd)?, file_name: dec_ifc_value_list(fname)?, file_schema: dec_ifc_value_list(fs)? })
}

pub(crate) fn print_ifc_mutation(m: &IfcMutation) -> String {
    match m {
        IfcMutation::SetFileDescription(set_file_description::SetFileDescription { values }) => format!("set-file-description values={}", enc_ifc_value_list(values)),
        IfcMutation::SetFileName(set_file_name::SetFileName { values }) => format!("set-file-name values={}", enc_ifc_value_list(values)),
        IfcMutation::SetFileSchema(set_file_schema::SetFileSchema { values }) => format!("set-file-schema values={}", enc_ifc_value_list(values)),
        IfcMutation::InsertEntity(insert_entity::InsertEntity { index, entity }) => format!("insert-entity index={index} entity={}", enc_entity(entity)),
        IfcMutation::RemoveEntity(remove_entity::RemoveEntity { id }) => format!("remove-entity id={id}"),
        IfcMutation::SetEntityName(set_entity_name::SetEntityName { id, name }) => format!("set-entity-name id={id} name={}", enc_str(name)),
        IfcMutation::SetEntityArg(set_entity_arg::SetEntityArg { id, index, value }) => format!("set-entity-arg id={id} index={index} value={}", enc_ifc_value(value)),
        IfcMutation::InsertEntityArg(insert_entity_arg::InsertEntityArg { id, index, value }) => format!("insert-entity-arg id={id} index={index} value={}", enc_ifc_value(value)),
        IfcMutation::RemoveEntityArg(remove_entity_arg::RemoveEntityArg { id, index }) => format!("remove-entity-arg id={id} index={index}"),
    }
}

pub(crate) fn parse_ifc_mutation(line: &str) -> Result<IfcMutation, String> {
    let (keyword, rest) = line.split_once(' ').unwrap_or((line, ""));
    let args: std::collections::BTreeMap<&str, &str> = rest.split(' ').filter(|s| !s.is_empty()).map(|tok| tok.split_once('=').ok_or_else(|| format!("ifc mutation: bad arg token {tok:?}"))).collect::<Result<Vec<_>, String>>()?.into_iter().collect();
    let arg = |k: &str| args.get(k).copied().ok_or_else(|| format!("ifc mutation: missing arg '{k}' for '{keyword}'"));
    let usize_arg = |k: &str| -> Result<usize, String> { arg(k)?.parse().map_err(|e: std::num::ParseIntError| e.to_string()) };
    let u64_arg = |k: &str| -> Result<u64, String> { arg(k)?.parse().map_err(|e: std::num::ParseIntError| e.to_string()) };
    match keyword {
        "set-file-description" => Ok(IfcMutation::SetFileDescription(set_file_description::SetFileDescription { values: dec_ifc_value_list(arg("values")?)? })),
        "set-file-name" => Ok(IfcMutation::SetFileName(set_file_name::SetFileName { values: dec_ifc_value_list(arg("values")?)? })),
        "set-file-schema" => Ok(IfcMutation::SetFileSchema(set_file_schema::SetFileSchema { values: dec_ifc_value_list(arg("values")?)? })),
        "insert-entity" => Ok(IfcMutation::InsertEntity(insert_entity::InsertEntity { index: usize_arg("index")?, entity: dec_entity(arg("entity")?)? })),
        "remove-entity" => Ok(IfcMutation::RemoveEntity(remove_entity::RemoveEntity { id: u64_arg("id")? })),
        "set-entity-name" => Ok(IfcMutation::SetEntityName(set_entity_name::SetEntityName { id: u64_arg("id")?, name: dec_str(arg("name")?)? })),
        "set-entity-arg" => Ok(IfcMutation::SetEntityArg(set_entity_arg::SetEntityArg { id: u64_arg("id")?, index: usize_arg("index")?, value: dec_ifc_value(arg("value")?)? })),
        "insert-entity-arg" => Ok(IfcMutation::InsertEntityArg(insert_entity_arg::InsertEntityArg { id: u64_arg("id")?, index: usize_arg("index")?, value: dec_ifc_value(arg("value")?)? })),
        "remove-entity-arg" => Ok(IfcMutation::RemoveEntityArg(remove_entity_arg::RemoveEntityArg { id: u64_arg("id")?, index: usize_arg("index")? })),
        other => Err(format!("ifc mutation: unknown keyword {other:?}")),
    }
}

impl OpText for IfcMutation {
    fn print_op(&self) -> String {
        print_ifc_mutation(self)
    }
    fn parse_op(line: &str) -> Result<Self, semio_framework_diagnostic::TextError> {
        parse_ifc_mutation(line).map_err(|e| semio_framework_diagnostic::TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue, e, semio_framework_diagnostic::TextSpan::at(1, 1)))
    }
}
}
pub use mutations_codec::*;
