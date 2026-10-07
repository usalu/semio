//! 📝️ Text representation codec surface for `stdio.xlsx` (mutations).

/// 📖️ Grammar include.
pub const COMPONENT_GRAMMAR_SEMIO: &str = include_str!("📖️.grammar.semio");
pub const COMPONENT_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️.grammar.semio");

#[allow(unused_imports)]
mod mutations_codec {
use super::*;
use crate::standards::v_ecma_376::subsets::base::schema::mutations::*;
use crate::schema::diff::{diff_set_snapshot, XlsxDiff};
use crate::standards::v_ecma_376::subsets::base::io::binary::diff::{dec_cell_value_bin,dec_sheet_bin,enc_cell_value_bin,enc_sheet_bin,read_str_lp,write_str_lp};
use crate::standards::v_ecma_376::subsets::base::io::text::diff::{dec_cell_value,dec_sheet,dec_str,enc_cell_value,enc_sheet,enc_str};
#[cfg(test)]
use crate::schema::snapshot::XlsxCell;
#[cfg(test)]
use crate::schema::snapshot::XlsxWorkbook;
use crate::schema::snapshot::{XlsxCellValue, XlsxSheet};
use crate::XlsxSnapshot;
use protocol::OpBinary;
use protocol::{Mutation, OpText};
use semio_s_artifact_stdio_xml::schema::snapshot::{XmlAttr, XmlNode};
#[cfg(test)]
use semio_s_artifact_stdio_zip::opc::OpcRelationship;
#[cfg(test)]
use semio_s_artifact_stdio_zip::opc::OpcTargetMode;

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_xlsx_snapshot(s: &XlsxSnapshot) -> String {
    enc_str(&semio_framework_pack_json::to_json_string(s))
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_xlsx_snapshot(s: &str) -> Result<XlsxSnapshot, String> {
    semio_framework_pack_json::from_json_str(&dec_str(s)?, semio_framework_pack_json::JsonMemberPolicy::Reject).map_err(|error| error.to_string())
}

pub(crate) fn enc_cell_address(address: &cell_address::XlsxCellAddress) -> String {
    enc_str(&semio_framework_pack_json::to_json_string(address))
}

pub(crate) fn dec_cell_address(value: &str) -> Result<cell_address::XlsxCellAddress, String> {
    semio_framework_pack_json::from_json_str(&dec_str(value)?, semio_framework_pack_json::JsonMemberPolicy::Reject).map_err(|error| error.to_string())
}

pub(crate) fn enc_cell_vacancy_address(address: &cell_address::XlsxCellVacancyAddress) -> String {
    enc_str(&semio_framework_pack_json::to_json_string(address))
}

pub(crate) fn dec_cell_vacancy_address(value: &str) -> Result<cell_address::XlsxCellVacancyAddress, String> {
    semio_framework_pack_json::from_json_str(&dec_str(value)?, semio_framework_pack_json::JsonMemberPolicy::Reject).map_err(|error| error.to_string())
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn print_xlsx_mutation(m: &XlsxMutation) -> String {
    match m {
        XlsxMutation::SetSnapshot(set_snapshot::SetSnapshot { snapshot }) => format!("set-snapshot snapshot={}", enc_xlsx_snapshot(snapshot)),
        XlsxMutation::PatchSnapshot(patch_snapshot::PatchSnapshot { patch }) => semio_s_artifact_stdio_contract::editing::snapshot_patch_text(patch),
        XlsxMutation::InsertSheet(insert_sheet::InsertSheet { sheet }) => format!("insert-sheet sheet={}", enc_sheet(sheet)),
        XlsxMutation::RemoveSheet(remove_sheet::RemoveSheet { name }) => format!("remove-sheet name={}", enc_str(name)),
        XlsxMutation::RenameSheet(rename_sheet::RenameSheet { name, new_name }) => format!("rename-sheet name={} new-name={}", enc_str(name), enc_str(new_name)),
        XlsxMutation::SetCell(set_cell::SetCell { address, value }) => format!("set-cell address={} value={}", enc_cell_address(address), enc_cell_value(value)),
        XlsxMutation::InsertCell(insert_cell::InsertCell { address, value }) => format!("insert-cell address={} value={}", enc_cell_vacancy_address(address), enc_cell_value(value)),
        XlsxMutation::RemoveCell(remove_cell::RemoveCell { address }) => format!("remove-cell address={}", enc_cell_address(address)),
        XlsxMutation::InsertSharedString(insert_shared_string::InsertSharedString { value }) => format!("insert-shared-string value={}", enc_str(value)),
        XlsxMutation::RemoveSharedString(remove_shared_string::RemoveSharedString { index }) => format!("remove-shared-string index={index}"),
        XlsxMutation::SetSharedString(set_shared_string::SetSharedString { index, value }) => format!("set-shared-string index={index} value={}", enc_str(value)),
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn parse_xlsx_mutation(line: &str) -> Result<XlsxMutation, String> {
    let (keyword, rest) = line.split_once(' ').unwrap_or((line, ""));
    let args: std::collections::BTreeMap<&str, &str> = rest.split(' ').map(|tok| tok.split_once('=').ok_or_else(|| format!("xlsx mutation: bad arg token {tok:?}"))).collect::<Result<Vec<_>, String>>()?.into_iter().collect();
    let arg = |k: &str| args.get(k).copied().ok_or_else(|| format!("xlsx mutation: missing arg '{k}' for '{keyword}'"));
    let usize_arg = |k: &str| -> Result<usize, String> { arg(k)?.parse().map_err(|e: std::num::ParseIntError| e.to_string()) };
    match keyword {
        "patch-snapshot" => semio_s_artifact_stdio_contract::editing::snapshot_patch_from_text(line).map(|patch| XlsxMutation::PatchSnapshot(patch_snapshot::PatchSnapshot { patch })),
        "set-snapshot" => Ok(XlsxMutation::SetSnapshot(set_snapshot::SetSnapshot { snapshot: dec_xlsx_snapshot(arg("snapshot")?)? })),
        "insert-sheet" => Ok(XlsxMutation::InsertSheet(insert_sheet::InsertSheet { sheet: dec_sheet(arg("sheet")?)? })),
        "remove-sheet" => Ok(XlsxMutation::RemoveSheet(remove_sheet::RemoveSheet { name: dec_str(arg("name")?)? })),
        "rename-sheet" => Ok(XlsxMutation::RenameSheet(rename_sheet::RenameSheet { name: dec_str(arg("name")?)?, new_name: dec_str(arg("new-name")?)? })),
        "set-cell" => Ok(XlsxMutation::SetCell(set_cell::SetCell { address: dec_cell_address(arg("address")?)?, value: dec_cell_value(arg("value")?)? })),
        "insert-cell" => Ok(XlsxMutation::InsertCell(insert_cell::InsertCell { address: dec_cell_vacancy_address(arg("address")?)?, value: dec_cell_value(arg("value")?)? })),
        "remove-cell" => Ok(XlsxMutation::RemoveCell(remove_cell::RemoveCell { address: dec_cell_address(arg("address")?)? })),
        "insert-shared-string" => Ok(XlsxMutation::InsertSharedString(insert_shared_string::InsertSharedString { value: dec_str(arg("value")?)? })),
        "remove-shared-string" => Ok(XlsxMutation::RemoveSharedString(remove_shared_string::RemoveSharedString { index: usize_arg("index")? })),
        "set-shared-string" => Ok(XlsxMutation::SetSharedString(set_shared_string::SetSharedString { index: usize_arg("index")?, value: dec_str(arg("value")?)? })),
        other => Err(format!("xlsx mutation: unknown keyword {other:?}")),
    }
}

impl OpText for XlsxMutation {
    fn print_op(&self) -> String {
        print_xlsx_mutation(self)
    }
    fn parse_op(line: &str) -> Result<Self, semio_framework_diagnostic::TextError> {
        parse_xlsx_mutation(line).map_err(|e| semio_framework_diagnostic::TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue, e, semio_framework_diagnostic::TextSpan::at(1, 1)))
    }
}
}
pub use mutations_codec::*;
