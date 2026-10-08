//! 📝️ Text representation codec surface for `stdio.ply` (mutations).

/// 📖️ Grammar include.
pub const COMPONENT_GRAMMAR_SEMIO: &str = include_str!("📖️.grammar.semio");
pub const COMPONENT_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️.grammar.semio");

#[allow(unused_imports)]
mod mutations_codec {
use super::*;
use crate::standards::v1_0::subsets::any::schema::mutations::*;

use crate::standards::v1_0::subsets::any::io::text::diff::{dec_value};
use crate::standards::v1_0::subsets::any::io::text::diff::{enc_value};
use crate::standards::v1_0::subsets::any::io::binary::diff::{read_bin_value};
use crate::standards::v1_0::subsets::any::io::binary::diff::{write_bin_value};
use crate::standards::v1_0::subsets::any::io::text::diff::{dec_row};
use crate::standards::v1_0::subsets::any::io::text::diff::{enc_row};
use crate::standards::v1_0::subsets::any::io::binary::diff::{read_bin_row};
use crate::standards::v1_0::subsets::any::io::binary::diff::{write_bin_row};
use crate::standards::v1_0::subsets::any::io::text::diff::{dec_element};
use crate::standards::v1_0::subsets::any::io::text::diff::{enc_element};
use crate::standards::v1_0::subsets::any::io::binary::diff::{read_bin_element};
use crate::standards::v1_0::subsets::any::io::binary::diff::{write_bin_element};
use crate::standards::v1_0::subsets::any::io::text::diff::{dec_format};
use crate::standards::v1_0::subsets::any::io::text::diff::{enc_format};
use crate::standards::v1_0::subsets::any::io::text::diff::{dec_str};
use crate::standards::v1_0::subsets::any::io::text::diff::{enc_str};
use crate::standards::v1_0::subsets::any::io::text::diff::{strip_brackets};
use crate::standards::v1_0::subsets::any::io::text::diff::{split_top_level};
use crate::standards::v1_0::subsets::any::io::binary::diff::{read_bin_str};
use crate::standards::v1_0::subsets::any::io::binary::diff::{write_bin_str};
use crate::schema::snapshot::{PlyElement, PlyFormat, PlyRow, PlyValue};
use crate::PlySnapshot;
use protocol::Mutation;
use protocol::OpBinary;
use protocol::OpText;

/// 🧪️ F6: **hand-rolled** `OpText`/`OpBinary` for `PlyMutation` (`#[derive(dsl::DslOps)]`
/// confirmed rejected above) — reuses `PlyDiff`'s `pub(crate)` grammar primitives
/// (`hex_encode`/`enc_element`/`enc_row`/`enc_value`/`split_top_level`/`encode_option`/...)
/// rather than duplicating them a second time in this file. Grammar: `keyword arg=value ...`
/// (space-separated, same shape the derive's own handcrafted-wrapper convention uses, and the
/// same shape svg's hand-rolled `OpText` uses), one match arm per variant (no `DslVariants`
/// scaffolding available since nothing here derives it).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn print_ply_mutation(m: &PlyMutation) -> String {
    match m {
        PlyMutation::SetFormat(set_format::SetFormat { format }) => format!("set-format format={}", enc_format(*format)),
        PlyMutation::InsertComment(insert_comment::InsertComment { index, comment }) => format!("insert-comment index={index} comment={}", enc_str(comment)),
        PlyMutation::RemoveComment(remove_comment::RemoveComment { index }) => format!("remove-comment index={index}"),
        PlyMutation::AddElement(add_element::AddElement { index, element }) => format!("add-element index={index} element={}", enc_element(element)),
        PlyMutation::RemoveElement(remove_element::RemoveElement { name }) => format!("remove-element name={}", enc_str(name)),
        PlyMutation::InsertRow(insert_row::InsertRow { element_name, index, row }) => format!("insert-row element-name={} index={index} row={}", enc_str(element_name), enc_row(row)),
        PlyMutation::RemoveRow(remove_row::RemoveRow { element_name, index }) => format!("remove-row element-name={} index={index}", enc_str(element_name)),
        PlyMutation::SetRowProperty(set_row_property::SetRowProperty { element_name, row_index, property_name, value }) => {
            format!("set-row-property element-name={} row-index={row_index} property-name={} value={}", enc_str(element_name), enc_str(property_name), enc_value(value),)
        }
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn parse_ply_mutation(line: &str) -> Result<PlyMutation, String> {
    let (keyword, rest) = line.split_once(' ').unwrap_or((line, ""));
    let args: std::collections::BTreeMap<&str, &str> = rest.split(' ').filter(|s| !s.is_empty()).map(|tok| tok.split_once('=').ok_or_else(|| format!("ply mutation: bad arg token {tok:?}"))).collect::<Result<Vec<_>, String>>()?.into_iter().collect();
    let arg = |k: &str| args.get(k).copied().ok_or_else(|| format!("ply mutation: missing arg '{k}' for '{keyword}'"));
    let usize_arg = |k: &str| -> Result<usize, String> { arg(k)?.parse().map_err(|e: std::num::ParseIntError| e.to_string()) };
    match keyword {
        "set-format" => Ok(PlyMutation::SetFormat(set_format::SetFormat { format: dec_format(arg("format")?)? })),
        "insert-comment" => Ok(PlyMutation::InsertComment(insert_comment::InsertComment { index: usize_arg("index")?, comment: dec_str(arg("comment")?)? })),
        "remove-comment" => Ok(PlyMutation::RemoveComment(remove_comment::RemoveComment { index: usize_arg("index")? })),
        "add-element" => Ok(PlyMutation::AddElement(add_element::AddElement { index: usize_arg("index")?, element: dec_element(arg("element")?)? })),
        "remove-element" => Ok(PlyMutation::RemoveElement(remove_element::RemoveElement { name: dec_str(arg("name")?)? })),
        "insert-row" => Ok(PlyMutation::InsertRow(insert_row::InsertRow { element_name: dec_str(arg("element-name")?)?, index: usize_arg("index")?, row: dec_row(arg("row")?)? })),
        "remove-row" => Ok(PlyMutation::RemoveRow(remove_row::RemoveRow { element_name: dec_str(arg("element-name")?)?, index: usize_arg("index")? })),
        "set-row-property" => {
            Ok(PlyMutation::SetRowProperty(set_row_property::SetRowProperty { element_name: dec_str(arg("element-name")?)?, row_index: usize_arg("row-index")?, property_name: dec_str(arg("property-name")?)?, value: dec_value(arg("value")?)? }))
        }
        other => Err(format!("ply mutation: unknown keyword {other:?}")),
    }
}

impl OpText for PlyMutation {
    fn print_op(&self) -> String {
        print_ply_mutation(self)
    }
    fn parse_op(line: &str) -> Result<Self, semio_framework_diagnostic::TextError> {
        parse_ply_mutation(line).map_err(|e| semio_framework_diagnostic::TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue, e, semio_framework_diagnostic::TextSpan::at(1, 1)))
    }
}
}
pub use mutations_codec::*;
