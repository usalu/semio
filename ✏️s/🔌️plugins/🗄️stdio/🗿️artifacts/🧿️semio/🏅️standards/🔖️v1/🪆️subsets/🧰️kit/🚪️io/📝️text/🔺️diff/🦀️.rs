//! 📝️ Text representation codec surface for `s.stdio.semio.kit.diff` — grammar include.
pub const COMPONENT_GRAMMAR_SEMIO: &str = include_str!("📖️.grammar.semio");
pub const COMPONENT_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️.grammar.semio");

#[allow(unused_imports)]
mod diff_codec {
use super::*;
use crate::standards::v1::subsets::kit::schema::diff::*;
use protocol::{DiffText,DiffBinary};
use crate::standards::v1::subsets::kit::schema::snapshot::{SemioKitDesign, SemioKitSnapshot, SemioKitType};
use crate::standards::v1::subsets::model::schema::snapshot::SemioModelSnapshot;
use crate::standards::v1::subsets::object::schema::snapshot::SemioObjectSnapshot;
use crate::standards::v1::subsets::value::schema::snapshot::SemioValueSnapshot;
use framework_schema::ArtifactSchema;
use protocol::MutationDiff;
use crate::standards::v1::subsets::base::io::text::snapshot::{dec_indexed_triple, dec_opt, enc_indexed_triple, enc_opt, split_top_level, strip_brackets};
use crate::standards::v1::subsets::base::schema::triples::Replace;
use crate::standards::v1::subsets::kit::io::text::snapshot::{dec_child, dec_connection, dec_design, dec_link, dec_pin, dec_piece, dec_str, dec_type, enc_child, enc_connection, enc_design, enc_link, enc_pin, enc_piece, enc_str, enc_type};
use crate::standards::v1::subsets::kit::io::text::snapshot::{dec_design_list};
use crate::standards::v1::subsets::kit::io::text::snapshot::{enc_design_list};
use crate::standards::v1::subsets::kit::io::text::snapshot::{dec_type_list};
use crate::standards::v1::subsets::kit::io::text::snapshot::{enc_type_list};
use crate::standards::v1::subsets::kit::io::text::snapshot::{dec_link_list};
use crate::standards::v1::subsets::kit::io::text::snapshot::{enc_link_list};
use crate::standards::v1::subsets::kit::io::text::snapshot::{dec_child_opt};
use crate::standards::v1::subsets::kit::io::text::snapshot::{enc_child_opt};
use crate::standards::v1::subsets::kit::io::text::snapshot::{dec_child_list};
use crate::standards::v1::subsets::kit::io::text::snapshot::{enc_child_list};

/// 🏷️ Field tags in presence-bit order: types, designs, objects, models, properties, representations.
pub(crate) const KIT_DIFF_TAGS: [&str; 6] = ["t", "d", "o", "m", "p", "r"];

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn enc_opt_str(value: Option<&String>) -> String {
    enc_opt(value, |v| enc_str(v))
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn enc_type_diff(d: &SemioKitTypeDiff) -> String {
    format!("[{},{}]", enc_opt_str(d.name.as_ref()), enc_opt_str(d.category.as_ref()))
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn dec_type_diff(s: &str) -> Result<SemioKitTypeDiff, String> {
    let parts = split_top_level(strip_brackets(s)?, ',');
    let [name, category] = parts.as_slice() else { return Err(format!("type diff: expected 2 fields, got {}", parts.len())) };
    Ok(SemioKitTypeDiff { name: dec_opt(name, dec_str)?, category: dec_opt(category, dec_str)? })
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn enc_design_diff(d: &SemioKitDesignDiff) -> String {
    let pieces = enc_opt(d.pieces.as_ref(), |pieces| format!("[{}]", pieces.iter().map(enc_piece).collect::<Vec<_>>().join(",")));
    let connections = enc_opt(d.connections.as_ref(), |connections| format!("[{}]", connections.iter().map(enc_connection).collect::<Vec<_>>().join(",")));
    format!("[{},{},{}]", enc_opt_str(d.name.as_ref()), pieces, connections)
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn dec_design_diff(s: &str) -> Result<SemioKitDesignDiff, String> {
    let parts = split_top_level(strip_brackets(s)?, ',');
    let [name, pieces, connections] = parts.as_slice() else { return Err(format!("design diff: expected 3 fields, got {}", parts.len())) };
    Ok(SemioKitDesignDiff {
        name: dec_opt(name, dec_str)?,
        pieces: dec_opt(pieces, |list| split_top_level(strip_brackets(list)?, ',').into_iter().filter(|s| !s.is_empty()).map(dec_piece).collect::<Result<Vec<_>, String>>())?,
        connections: dec_opt(connections, |list| split_top_level(strip_brackets(list)?, ',').into_iter().filter(|s| !s.is_empty()).map(dec_connection).collect::<Result<Vec<_>, String>>())?,
    })
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn enc_link_diff(d: &SemioKitLinkDiff) -> String {
    format!("[{}]", enc_opt(d.pin.as_ref(), enc_pin))
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn dec_link_diff(s: &str) -> Result<SemioKitLinkDiff, String> {
    let parts = split_top_level(strip_brackets(s)?, ',');
    let [pin] = parts.as_slice() else { return Err(format!("link diff: expected 1 field, got {}", parts.len())) };
    Ok(SemioKitLinkDiff { pin: dec_opt(pin, dec_pin)? })
}

/// 🧩 The `[triple]`-bracketed text of every present field, in presence-bit order — the shared payload of the text and binary codecs.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn kit_diff_sections(d: &SemioKitDiff) -> [Option<String>; 6] {
    let triple = |enc: String| format!("[{enc}]");
    [
        d.types.as_ref().map(|t| triple(enc_indexed_triple(t, enc_type_diff, enc_type))),
        d.designs.as_ref().map(|t| triple(enc_indexed_triple(t, enc_design_diff, enc_design))),
        d.objects.as_ref().map(|t| triple(enc_indexed_triple(t, |r| enc_child(&r.value), enc_child))),
        d.models.as_ref().map(|t| triple(enc_indexed_triple(t, |r| enc_child(&r.value), enc_child))),
        d.properties.as_ref().map(enc_child_opt),
        d.representations.as_ref().map(|t| triple(enc_indexed_triple(t, enc_link_diff, enc_link))),
    ]
}

/// 🧩 Decodes one field's text into `d`; `index` is the presence-bit order of [`KIT_DIFF_TAGS`].
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_kit_section(d: &mut SemioKitDiff, index: usize, text: &str) -> Result<(), String> {
    match index {
        0 => d.types = Some(dec_indexed_triple(strip_brackets(text)?, dec_type_diff, dec_type)?),
        1 => d.designs = Some(dec_indexed_triple(strip_brackets(text)?, dec_design_diff, dec_design)?),
        2 => d.objects = Some(dec_indexed_triple(strip_brackets(text)?, |c| dec_child(c).map(|value| Replace { value }), dec_child)?),
        3 => d.models = Some(dec_indexed_triple(strip_brackets(text)?, |c| dec_child(c).map(|value| Replace { value }), dec_child)?),
        4 => d.properties = Some(dec_child_opt(text)?),
        5 => d.representations = Some(dec_indexed_triple(strip_brackets(text)?, dec_link_diff, dec_link)?),
        other => return Err(format!("kit diff: unknown field index {other}")),
    }
    Ok(())
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn print_kit_diff(d: &SemioKitDiff) -> String {
    KIT_DIFF_TAGS.iter().zip(kit_diff_sections(d)).filter_map(|(tag, section)| section.map(|section| format!("{tag}={section}"))).collect::<Vec<_>>().join(";")
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn parse_kit_diff(line: &str) -> Result<SemioKitDiff, String> {
    let mut d = SemioKitDiff::default();
    if line.is_empty() {
        return Ok(d);
    }
    for field in split_top_level(line, ';') {
        let (tag, rest) = field.split_once('=').ok_or_else(|| format!("kit diff: missing '=' in {field:?}"))?;
        let index = KIT_DIFF_TAGS.iter().position(|known| *known == tag).ok_or_else(|| format!("kit diff: unknown field tag {tag:?}"))?;
        dec_kit_section(&mut d, index, rest)?;
    }
    Ok(d)
}

impl protocol::DiffText for SemioKitDiff {
fn print_diff(&self) -> String {
    print_kit_diff(self)
}
fn parse_diff(line: &str) -> Result<Self, semio_framework_diagnostic::TextError> {
    parse_kit_diff(line).map_err(|e| semio_framework_diagnostic::TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue, e, semio_framework_diagnostic::TextSpan::at(1, 1)))
}
}
}
pub use diff_codec::*;
