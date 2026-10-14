//! 📝️ Text representation codec surface for `stdio.semio.presentation` (snapshot).

/// 📖️ Grammar include.
pub const COMPONENT_GRAMMAR_SEMIO: &str = include_str!("📖️.grammar.semio");
pub const COMPONENT_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️.grammar.semio");

#[allow(unused_imports)]
mod snapshot_codec {
use super::*;
use crate::io::text::diff::{enc_list, dec_list};
use crate::schema::snapshot::*;
use semio_s_artifact_stdio_semio_base::schema::geometry::SemioPoint2;
use semio_s_artifact_stdio_semio_base::io::text::codec::{split_top_level, strip_brackets};
use semio_s_artifact_stdio_semio_document::schema::snapshot::DocBlock;
/// 🧱️ REUSE, don't reinvent — the sibling `🔺️diff` facet re-exports document's own real, already-
/// tested `DocBlock` codec (`enc_block`/`dec_block`) plus the entity value-codecs it owns
/// (`enc_master`/`enc_layout`/`enc_slide`, `enc_str`, `enc_list`) — this facet imports them rather
/// than duplicating a third independent copy (ARTIFACT-SYSTEM-OVERHAUL-REAL-CODECS-RUNTIME-REUSE-
/// EVOLUTION presentation wave, following `document`'s own snapshot-imports-from-diff convention).
use crate::io::text::diff::dec_slide;
use crate::io::text::diff::enc_slide;
use crate::io::text::diff::dec_layout;
use crate::io::text::diff::enc_layout;
use crate::io::text::diff::dec_master;
use crate::io::text::diff::enc_master;
use semio_s_artifact_stdio_semio_document::io::text::diff::dec_block;
use semio_s_artifact_stdio_semio_document::io::text::diff::enc_block;
use semio_s_artifact_stdio_semio_base::io::text::codec::dec_str;
use semio_s_artifact_stdio_semio_base::io::text::codec::enc_str;
use framework_schema::ArtifactSchema;

/// 🧪️ ARTIFACT-SYSTEM-OVERHAUL-REAL-CODECS-RUNTIME-REUSE-EVOLUTION presentation wave: real
/// structured text body — `schema=<hex>` / `masters=[<master>,...]` / `layouts=[<layout>,...]` /
/// `slides=[<slide>,...]`, one line per top-level field, matching the flow/model/brep/document
/// pilots' own `print_*_snapshot_body` shape. Reuses `🔺️diff`'s ALREADY-real, already-tested
/// `enc_str`/`enc_master`/`enc_layout`/`enc_slide` value codecs (which themselves reuse document's
/// real `enc_block` for every `blocks`/`notes` leaf) rather than duplicating a third independent
/// copy. Replaces the old hex-of-`serde_json` passthrough.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn print_presentation_snapshot_body(s: &SemioPresentationSnapshot) -> String {
    format!(
        "schema={}\nmasters=[{}]\nlayouts=[{}]\nslides=[{}]",
        enc_str(&s.schema),
        s.masters.iter().map(enc_master).collect::<Vec<_>>().join(","),
        s.layouts.iter().map(enc_layout).collect::<Vec<_>>().join(","),
        s.slides.iter().map(enc_slide).collect::<Vec<_>>().join(",")
    )
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn parse_presentation_snapshot_body(body: &str) -> Result<SemioPresentationSnapshot, String> {
    let mut schema = None;
    let mut masters = Vec::new();
    let mut layouts = Vec::new();
    let mut slides = Vec::new();
    for line in body.lines() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }
        if let Some(rest) = line.strip_prefix("schema=") {
            schema = Some(dec_str(rest)?);
        } else if let Some(rest) = line.strip_prefix("masters=") {
            let inner = strip_brackets(rest)?;
            masters = split_top_level(inner, ',').into_iter().filter(|s| !s.is_empty()).map(dec_master).collect::<Result<Vec<_>, String>>()?;
        } else if let Some(rest) = line.strip_prefix("layouts=") {
            let inner = strip_brackets(rest)?;
            layouts = split_top_level(inner, ',').into_iter().filter(|s| !s.is_empty()).map(dec_layout).collect::<Result<Vec<_>, String>>()?;
        } else if let Some(rest) = line.strip_prefix("slides=") {
            let inner = strip_brackets(rest)?;
            slides = split_top_level(inner, ',').into_iter().filter(|s| !s.is_empty()).map(dec_slide).collect::<Result<Vec<_>, String>>()?;
        } else {
            return Err(format!("presentation snapshot: unknown line {line:?}"));
        }
    }
    let schema = schema.ok_or_else(|| "presentation snapshot: missing schema line".to_string())?;
    Ok(SemioPresentationSnapshot { schema, masters, layouts, slides })
}

/// 🧩️ ARTIFACT-SYSTEM-OVERHAUL-REAL-CODECS-RUNTIME-REUSE-EVOLUTION presentation wave: real
/// structured text/binary codecs, replacing the old hex-of-`serde_json` passthrough (both
/// `parse_dsl`'s and `encode_pack_with`'s old bodies called straight into
/// `serde_json::{to_vec,from_slice}`). The derive path (`#[derive(dsl::DslArtifact)]`) hits the
/// same wall every hand-rolled-tagged-enum semio subset (model/brep/drawing/document) already hit:
/// `SlideShape`/`PlaceholderKind` are `#[value(tag = ...)]` data-carrying enums with heterogeneous
/// per-variant field sets (and `SlideShape::TextBox`/`Table` transitively embed `DocBlock`, itself
/// the same shape) — no `dsl::DslField`/`DslEnum` impl exists for either. Hand-rolled instead,
/// matching `🔺️diff`'s already-hand-rolled convention.
impl pack::snapshot::ArtifactDsl for SemioPresentationSnapshot {
    const EXTENSION: &'static str = "semio";
    fn envelope_id() -> &'static str {
        STDIO_SEMIOPRESENTATION_DOCUMENT_SCHEMA
    }

    fn parse_dsl(text: &str) -> Result<Self, semio_framework_diagnostic::TextError> {
        let body = match store::semio_format::split_text_preamble(text) {
            Ok((_, rest)) => rest,
            Err(_) => text,
        };
        parse_presentation_snapshot_body(body).map_err(|e| semio_framework_diagnostic::TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue, e, semio_framework_diagnostic::TextSpan::at(1, 1)))
    }

    fn print_dsl(&self) -> String {
        let body = print_presentation_snapshot_body(self);
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(<Self as pack::snapshot::ArtifactDsl>::envelope_id(), store::semio_format::Component::Dsl, 1).expect("valid envelope_id");
        store::semio_format::wrap_text(&envelope, &body)
    }
}

/// 🚪️ The four codec entry points above, reachable from OUTSIDE this crate. `store` is a private
/// `extern crate semio_framework_os_kernel as store` alias in `🦀️.rs`, so an external caller —
/// an owner-root test adapter is exactly that — can neither bring `pack::snapshot::ArtifactDsl`/
/// `pack::snapshot::ArtifactPack` into scope nor name `store::TextError`/`store::PackError` in a signature.
/// These four wrappers carry the error across as a plain `String` so the subset's own text and
/// binary envelopes stay drivable end to end (`kit`'s precedent for the same structural gap).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn parse_semio_presentation_dsl(text: &str) -> Result<SemioPresentationSnapshot, String> {
    <SemioPresentationSnapshot as pack::snapshot::ArtifactDsl>::parse_dsl(text).map_err(|e| e.to_string())
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn print_semio_presentation_dsl(snapshot: &SemioPresentationSnapshot) -> String {
    <SemioPresentationSnapshot as pack::snapshot::ArtifactDsl>::print_dsl(snapshot)
}
}
pub use snapshot_codec::*;

#[allow(unused_imports)]
mod snapshot_wire_codec {
use super::*;
use crate::schema::snapshot::*;
use semio_s_artifact_stdio_semio_base::schema::geometry::SemioPoint2;
use semio_s_artifact_stdio_semio_base::io::text::codec::strip_brackets;
use semio_s_artifact_stdio_semio_base::io::text::codec::split_top_level;
use semio_s_artifact_stdio_semio_document::schema::snapshot::DocBlock;
/// 🧱️ REUSE, don't reinvent — the sibling `🔺️diff` facet re-exports document's own real, already-
/// tested `DocBlock` codec (`enc_block`/`dec_block`) plus the entity value-codecs it owns
/// (`enc_master`/`enc_layout`/`enc_slide`, `enc_str`, `enc_list`) — this facet imports them rather
/// than duplicating a third independent copy (ARTIFACT-SYSTEM-OVERHAUL-REAL-CODECS-RUNTIME-REUSE-
/// EVOLUTION presentation wave, following `document`'s own snapshot-imports-from-diff convention).
use crate::io::text::diff::dec_slide;
use crate::io::text::diff::enc_slide;
use crate::io::text::diff::dec_layout;
use crate::io::text::diff::enc_layout;
use crate::io::text::diff::dec_master;
use crate::io::text::diff::enc_master;
use semio_s_artifact_stdio_semio_document::io::text::diff::dec_block;
use semio_s_artifact_stdio_semio_document::io::text::diff::enc_block;
use semio_s_artifact_stdio_semio_base::io::text::codec::dec_str;
use semio_s_artifact_stdio_semio_base::io::text::codec::enc_str;
use framework_schema::ArtifactSchema;




}

#[allow(unused_imports)]
mod diff_codec {
use super::*;
use crate::io::text::diff::{enc_list, dec_list, enc_master, dec_master, enc_layout, dec_layout, enc_slide, dec_slide};
use semio_s_artifact_stdio_semio_base::io::text::codec::{enc_str, dec_str};
use crate::schema::diff::*;
use semio_s_artifact_stdio_semio_base::schema::geometry::SemioPoint2;
use semio_s_artifact_stdio_semio_base::schema::triples::{IndexAdded, IndexModified, IndexedTripleDiff, NamedModified, NamedTripleDiff};
use semio_s_artifact_stdio_semio_base::io::text::codec::strip_brackets;
use semio_s_artifact_stdio_semio_base::io::text::codec::split_top_level;
/// 🧱️ REUSE, don't reinvent — `document::DocBlock`'s own real, already-tested text codec
/// (`ws-codec-document-report.md`), re-exported here so both this file's own leaf encoders AND
/// the sibling `🧬️mutations`/`📸️snapshot` facets can import `{enc_block, dec_block}` from THIS
/// module (matching the pre-existing convention where this file is the one place that owns every
/// value codec presentation's other facets import from).
use semio_s_artifact_stdio_semio_document::io::text::diff::dec_block;
use semio_s_artifact_stdio_semio_document::io::text::diff::enc_block;
use semio_s_artifact_stdio_semio_document::schema::snapshot::DocBlock;
use crate::schema::snapshot::SemioPresentationSnapshot;
use crate::schema::snapshot::{PlaceholderKind, Slide, SlideFrame, SlideLayout, SlideMaster, SlidePictureImage, SlideShape, SlideTableCell, SlideTableRow};
use framework_schema::ArtifactSchema;
use protocol::command::DiffAlgebra;
use protocol::MutationDiff;

}
