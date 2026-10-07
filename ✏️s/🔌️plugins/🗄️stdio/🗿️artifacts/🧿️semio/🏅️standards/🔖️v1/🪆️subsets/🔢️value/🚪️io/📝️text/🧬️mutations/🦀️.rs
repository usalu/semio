//! 📝️ Text representation codec surface for `stdio.semio.value` (mutations).

pub const COMPONENT_GRAMMAR_SEMIO: &str = include_str!("📖️.grammar.semio");
pub const COMPONENT_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️.grammar.semio");

#[allow(unused_imports)]
mod mutations_codec {
use super::*;
use crate::standards::v1::subsets::value::schema::mutations::*;
use crate::standards::v1::subsets::base::schema::triples::{IndexAdded, NamedModified, NamedTripleDiff};
use crate::standards::v1::subsets::base::io::text::snapshot::{split_top_level, strip_brackets};
use crate::standards::v1::subsets::value::schema::diff::diff_set_snapshot;
use crate::standards::v1::subsets::value::schema::diff::{value_diff_between, NamedAdded, SemioValueDiff, SemioValueTreeDiff};
use crate::standards::v1::subsets::value::io::text::diff::{dec_semio_value};
use crate::standards::v1::subsets::value::io::text::diff::{enc_semio_value};
use crate::standards::v1::subsets::value::io::binary::diff::{dec_semio_value_node_bin};
use crate::standards::v1::subsets::value::io::binary::diff::{enc_semio_value_node_bin};
use crate::standards::v1::subsets::value::io::binary::diff::{dec_semio_value_bin};
use crate::standards::v1::subsets::value::io::binary::diff::{enc_semio_value_bin};
use crate::standards::v1::subsets::drawing::io::binary::snapshot::{read_str_lp};
use crate::standards::v1::subsets::drawing::io::binary::snapshot::{write_str_lp};
use crate::standards::v1::subsets::value::io::text::diff::{dec_value_id};
use crate::standards::v1::subsets::value::io::text::diff::{enc_value_id};
use crate::standards::v1::subsets::drawing::io::text::snapshot::{dec_str};
use crate::standards::v1::subsets::drawing::io::text::snapshot::{enc_str};
use crate::standards::v1::subsets::value::schema::snapshot::{SemioValue, SemioValueEntry, SemioValueNode, SemioValueSnapshot, ValueId};
use crate::standards::v1::subsets::value::io::text::snapshot::{dec_semio_value_snapshot};
use crate::standards::v1::subsets::value::io::text::snapshot::{enc_semio_value_snapshot};
#[cfg(test)]
use protocol::command::DiffAlgebra;
use protocol::{Mutation, OpText};

/// 🧪️ Hand-rolled `OpText`/`OpBinary` for `SemioValueMutation` (`#[derive(dsl::DslOps)]` blocked,
/// see the enum doc comment above) — reuses `SemioValueTreeDiff`'s `pub(crate)` grammar primitives
/// rather than duplicating them a second time in this file. Grammar: `keyword arg=value ...`
/// (space-separated), one match arm per variant — same shape `JsonMutation`'s hand-rolled codec
/// uses.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_path_segment(seg: &SemioValuePathSegment) -> String {
    match seg {
        SemioValuePathSegment::Key { key } => format!("K[{}]", enc_str(key)),
        SemioValuePathSegment::Index { index } => format!("I[{index}]"),
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_path_segment(s: &str) -> Result<SemioValuePathSegment, String> {
    let (tag, rest) = s.split_at(1);
    match tag {
        "K" => Ok(SemioValuePathSegment::Key { key: dec_str(strip_brackets(rest)?)? }),
        "I" => Ok(SemioValuePathSegment::Index { index: strip_brackets(rest)?.parse().map_err(|e: std::num::ParseIntError| e.to_string())? }),
        other => Err(format!("semio value path segment: unknown tag {other:?}")),
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_path(p: &SemioValuePath) -> String {
    format!("[{}]", p.iter().map(enc_path_segment).collect::<Vec<_>>().join(","))
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_path(s: &str) -> Result<SemioValuePath, String> {
    split_top_level(strip_brackets(s)?, ',').into_iter().filter(|s| !s.is_empty()).map(dec_path_segment).collect()
}

/// 🧭️ `enc_semio_snapshot`/`dec_semio_snapshot` — thin aliases for the single-source-of-truth
/// `SemioValueSnapshot` text codec now owned by the sibling `📸️snapshot/🦀️.rs` (also
/// reused there by `ArtifactDsl`/`ArtifactPack`), rather than a second independent copy.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_semio_snapshot(s: &SemioValueSnapshot) -> String {
    enc_semio_value_snapshot(s)
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_semio_snapshot(s: &str) -> Result<SemioValueSnapshot, String> {
    dec_semio_value_snapshot(s)
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn print_value_mutation(m: &SemioValueMutation) -> String {
    match m {
        SemioValueMutation::SetSnapshot(set_snapshot::SetSnapshot { snapshot }) => format!("set-snapshot snapshot={}", enc_semio_snapshot(snapshot)),
        SemioValueMutation::PatchSnapshot(patch_snapshot::PatchSnapshot { patch }) => semio_s_artifact_stdio_contract::editing::snapshot_patch_text(patch),
        SemioValueMutation::SetValue(set_value::SetValue { path, value }) => format!("set-value path={} value={}", enc_path(path), enc_semio_value(value)),
        SemioValueMutation::SetMapEntry(set_map_entry::SetMapEntry { path, key, value }) => {
            format!("set-map-entry path={} key={} value={}", enc_path(path), enc_str(key), enc_semio_value(value))
        }
        SemioValueMutation::RemoveMapEntry(remove_map_entry::RemoveMapEntry { path, key }) => format!("remove-map-entry path={} key={}", enc_path(path), enc_str(key)),
        SemioValueMutation::InsertListItem(insert_list_item::InsertListItem { path, index, value }) => {
            format!("insert-list-item path={} index={index} value={}", enc_path(path), enc_semio_value(value))
        }
        SemioValueMutation::RemoveListItem(remove_list_item::RemoveListItem { path, index }) => format!("remove-list-item path={} index={index}", enc_path(path)),
        SemioValueMutation::SetNode(set_node::SetNode { id, value }) => format!("set-node id={} value={}", enc_value_id(id), enc_semio_value(value)),
        SemioValueMutation::RemoveNode(remove_node::RemoveNode { id }) => format!("remove-node id={}", enc_value_id(id)),
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn parse_value_mutation(line: &str) -> Result<SemioValueMutation, String> {
    if let Some(source) = line.strip_prefix("patch-snapshot patch=") {
        let patch = semio_s_artifact_stdio_contract::editing::snapshot_patch_from_hex(source)?;
        return Ok(SemioValueMutation::PatchSnapshot(crate::standards::v1::subsets::value::schema::mutations::patch_snapshot::PatchSnapshot { patch }));
    }
    let (keyword, rest) = line.split_once(' ').unwrap_or((line, ""));
    let args: std::collections::BTreeMap<&str, &str> =
        rest.split(' ').filter(|s| !s.is_empty()).map(|tok| tok.split_once('=').ok_or_else(|| format!("semio value mutation: bad arg token {tok:?}"))).collect::<Result<Vec<_>, String>>()?.into_iter().collect();
    let arg = |k: &str| args.get(k).copied().ok_or_else(|| format!("semio value mutation: missing arg '{k}' for '{keyword}'"));
    let usize_arg = |k: &str| -> Result<usize, String> { arg(k)?.parse().map_err(|e: std::num::ParseIntError| e.to_string()) };
    match keyword {
        "patch-snapshot" => semio_s_artifact_stdio_contract::editing::snapshot_patch_from_text(line).map(|patch| SemioValueMutation::PatchSnapshot(patch_snapshot::PatchSnapshot { patch })),
        "set-snapshot" => Ok(SemioValueMutation::SetSnapshot(set_snapshot::SetSnapshot { snapshot: dec_semio_snapshot(arg("snapshot")?)? })),
        "set-value" => Ok(SemioValueMutation::SetValue(set_value::SetValue { path: dec_path(arg("path")?)?, value: dec_semio_value(arg("value")?)? })),
        "set-map-entry" => Ok(SemioValueMutation::SetMapEntry(set_map_entry::SetMapEntry { path: dec_path(arg("path")?)?, key: dec_str(arg("key")?)?, value: dec_semio_value(arg("value")?)? })),
        "remove-map-entry" => Ok(SemioValueMutation::RemoveMapEntry(remove_map_entry::RemoveMapEntry { path: dec_path(arg("path")?)?, key: dec_str(arg("key")?)? })),
        "insert-list-item" => Ok(SemioValueMutation::InsertListItem(insert_list_item::InsertListItem { path: dec_path(arg("path")?)?, index: usize_arg("index")?, value: dec_semio_value(arg("value")?)? })),
        "remove-list-item" => Ok(SemioValueMutation::RemoveListItem(remove_list_item::RemoveListItem { path: dec_path(arg("path")?)?, index: usize_arg("index")? })),
        "set-node" => Ok(SemioValueMutation::SetNode(set_node::SetNode { id: dec_value_id(arg("id")?)?, value: dec_semio_value(arg("value")?)? })),
        "remove-node" => Ok(SemioValueMutation::RemoveNode(remove_node::RemoveNode { id: dec_value_id(arg("id")?)? })),
        other => Err(format!("semio value mutation: unknown keyword {other:?}")),
    }
}

impl OpText for SemioValueMutation {
    fn print_op(&self) -> String {
        print_value_mutation(self)
    }
    fn parse_op(line: &str) -> Result<Self, semio_framework_diagnostic::TextError> {
        parse_value_mutation(line).map_err(|e| semio_framework_diagnostic::TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue, e, semio_framework_diagnostic::TextSpan::at(1, 1)))
    }
}
}
pub use mutations_codec::*;
