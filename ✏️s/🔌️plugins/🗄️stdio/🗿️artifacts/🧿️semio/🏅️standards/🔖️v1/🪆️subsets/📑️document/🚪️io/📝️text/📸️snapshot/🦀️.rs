//! 📝️ Text representation codec surface for `s.stdio.semio.document` (snapshot) — grammar include.
pub const COMPONENT_GRAMMAR_SEMIO: &str = include_str!("📖️.grammar.semio");
pub const COMPONENT_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️.grammar.semio");

#[allow(unused_imports)]
mod snapshot_codec {
use super::*;
use crate::standards::v1::subsets::document::schema::snapshot::*;
use crate::standards::v1::subsets::base::io::text::snapshot::{split_top_level, strip_brackets};
use crate::standards::v1::subsets::document::io::text::diff::{dec_image};
use crate::standards::v1::subsets::document::io::text::diff::{enc_image};
use crate::standards::v1::subsets::document::io::text::diff::{dec_style};
use crate::standards::v1::subsets::document::io::text::diff::{enc_style};
use crate::standards::v1::subsets::document::io::text::diff::{dec_block};
use crate::standards::v1::subsets::document::io::text::diff::{enc_block};
use crate::standards::v1::subsets::drawing::io::text::snapshot::{dec_str};
use crate::standards::v1::subsets::drawing::io::text::snapshot::{enc_str};
use framework_schema::ArtifactSchema;

/// 🧪️ ARTIFACT-SYSTEM-OVERHAUL-REAL-CODECS-RUNTIME-REUSE-EVOLUTION document wave: real structured
/// text body — `schema=<hex>` / `styles=[<style>,...]` / `images=[<image>,...]` /
/// `blocks=[<block>,...]`, one line per top-level field, matching the flow/model/brep pilots'
/// own `print_*_snapshot_body` shape. Reuses `🔺️diff`'s ALREADY-real, already-tested
/// `enc_str`/`enc_style`/`enc_image`/`enc_block` value codecs (established there pre-wave) rather
/// than duplicating a third independent copy — this subset's own established convention (its
/// `🧬️mutations` facet already does the same). Replaces the old hex-of-`serde_json` passthrough.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn print_document_snapshot_body(s: &SemioDocumentSnapshot) -> String {
    format!(
        "schema={}\nstyles=[{}]\nimages=[{}]\nblocks=[{}]",
        enc_str(&s.schema),
        s.styles.iter().map(enc_style).collect::<Vec<_>>().join(","),
        s.images.iter().map(enc_image).collect::<Vec<_>>().join(","),
        s.blocks.iter().map(enc_block).collect::<Vec<_>>().join(",")
    )
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn parse_document_snapshot_body(body: &str) -> Result<SemioDocumentSnapshot, String> {
    let mut schema = None;
    let mut styles = Vec::new();
    let mut images = Vec::new();
    let mut blocks = Vec::new();
    for line in body.lines() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }
        if let Some(rest) = line.strip_prefix("schema=") {
            schema = Some(dec_str(rest)?);
        } else if let Some(rest) = line.strip_prefix("styles=") {
            let inner = strip_brackets(rest)?;
            styles = split_top_level(inner, ',').into_iter().filter(|s| !s.is_empty()).map(dec_style).collect::<Result<Vec<_>, String>>()?;
        } else if let Some(rest) = line.strip_prefix("images=") {
            let inner = strip_brackets(rest)?;
            images = split_top_level(inner, ',').into_iter().filter(|s| !s.is_empty()).map(dec_image).collect::<Result<Vec<_>, String>>()?;
        } else if let Some(rest) = line.strip_prefix("blocks=") {
            let inner = strip_brackets(rest)?;
            blocks = split_top_level(inner, ',').into_iter().filter(|s| !s.is_empty()).map(dec_block).collect::<Result<Vec<_>, String>>()?;
        } else {
            return Err(format!("document snapshot: unknown line {line:?}"));
        }
    }
    let schema = schema.ok_or_else(|| "document snapshot: missing schema line".to_string())?;
    Ok(SemioDocumentSnapshot { schema, styles, images, blocks })
}

/// 🧩️ ARTIFACT-SYSTEM-OVERHAUL-REAL-CODECS-RUNTIME-REUSE-EVOLUTION document wave: real structured
/// text/binary codecs, replacing the old hex-of-`serde_json` passthrough (both `parse_dsl`'s and
/// `encode_pack_with`'s old bodies called straight into `serde_json::{to_vec,from_slice}`). The
/// derive path (`#[derive(dsl::DslArtifact)]`) was tried first per the ticket's brief and hits the
/// same wall every hand-rolled-tagged-enum semio subset (model/brep) already hit: `DocBlock` is a
/// `#[value(tag = "kind")]` data-carrying enum with heterogeneous per-variant field sets — no
/// `dsl::DslField`/`DslEnum` impl exists for it, and `IndexedTripleDiff<DocBlockDiff, DocBlock>`
/// compounds the gap. Hand-rolled instead, matching `🔺️diff`'s already-hand-rolled convention.
impl store::ArtifactDsl for SemioDocumentSnapshot {
    const EXTENSION: &'static str = "semio";
    fn envelope_id() -> &'static str {
        STDIO_SEMIODOCUMENT_DOCUMENT_SCHEMA
    }

    fn parse_dsl(text: &str) -> Result<Self, semio_framework_diagnostic::TextError> {
        let body = match store::semio_format::split_text_preamble(text) {
            Ok((_, rest)) => rest,
            Err(_) => text,
        };
        parse_document_snapshot_body(body).map_err(|e| semio_framework_diagnostic::TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue, e, semio_framework_diagnostic::TextSpan::at(1, 1)))
    }

    fn print_dsl(&self) -> String {
        let body = print_document_snapshot_body(self);
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(<Self as store::ArtifactDsl>::envelope_id(), store::semio_format::Component::Dsl, 1).expect("valid envelope_id");
        store::semio_format::wrap_text(&envelope, &body)
    }
}

/// 📥️ Parses this subset's own committed `.dsl.semio` text into a real [`SemioDocumentSnapshot`] — a thin
/// wrapper over `store::ArtifactDsl::parse_dsl` so external Rust callers that cannot name this
/// crate's private `store` extern-crate item (the `📃️mutate-semio-document` test adapter, which reads the
/// REAL committed example artifact rather than a hand-transcribed Rust literal of it) can still
/// drive the same codec production does. Same rationale as `🧰️kit`'s `decode_kit_snapshot_json`.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn parse_semio_document_dsl(text: &str) -> Result<SemioDocumentSnapshot, String> {
    <SemioDocumentSnapshot as store::ArtifactDsl>::parse_dsl(text).map_err(|error| error.to_string())
}

/// 📤️ The `store::ArtifactDsl::print_dsl` inverse of [`parse_semio_document_dsl`] — same rationale.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn print_semio_document_dsl(snapshot: &SemioDocumentSnapshot) -> String {
    <SemioDocumentSnapshot as store::ArtifactDsl>::print_dsl(snapshot)
}

/// 📤️ This subset's own `#[value(rename_all = "camelCase")]` structural JSON projection of
/// `s.stdio.semio.document` — the shape the `📃️mutate-semio-document` case compares under `ordered-json-v1`. A thin
/// `serde_json` wrapper (already a direct dependency of this crate, used behind this interface per
/// CLAUDE.md's "external libraries behind an interface" rule, never a new one), so a projection is
/// derived from the snapshot type itself rather than hand-written a second time in the adapter,
/// where it could drift.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn encode_semio_document_snapshot_json(snapshot: &SemioDocumentSnapshot) -> String {
    semio_framework_pack_json::to_json_string(snapshot)
}

/// 📥️ The `serde_json` inverse of [`encode_semio_document_snapshot_json`] — decodes the
/// `before`/`after` halves of `📃️mutate-semio-document`'s committed specification vectors
/// (`../../../../../🧪️tests/📃️mutate-semio-document/🧫️fixtures/🦠️<kind>.json`) into real [`SemioDocumentSnapshot`]
/// values, so the adapter never hand-transcribes a fixture into a Rust literal that could silently
/// drift away from the JSON it claims to mirror.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn decode_semio_document_snapshot_json(text: &str) -> Result<SemioDocumentSnapshot, String> {
    semio_framework_pack_json::from_json_str(text, semio_framework_pack_json::JsonMemberPolicy::Reject).map_err(|error| error.to_string())
}
}
pub use snapshot_codec::*;

#[allow(unused_imports)]
mod snapshot_wire_codec {
use super::*;
use crate::standards::v1::subsets::document::schema::snapshot::*;
use crate::standards::v1::subsets::audio::io::text::diff::{strip_brackets};
use crate::standards::v1::subsets::audio::io::text::diff::{split_top_level};
use crate::standards::v1::subsets::document::io::text::diff::{dec_image};
use crate::standards::v1::subsets::document::io::text::diff::{enc_image};
use crate::standards::v1::subsets::document::io::text::diff::{dec_style};
use crate::standards::v1::subsets::document::io::text::diff::{enc_style};
use crate::standards::v1::subsets::document::io::text::diff::{dec_block};
use crate::standards::v1::subsets::document::io::text::diff::{enc_block};
use crate::standards::v1::subsets::drawing::io::text::snapshot::{dec_str};
use crate::standards::v1::subsets::drawing::io::text::snapshot::{enc_str};
use framework_schema::ArtifactSchema;




}
pub use snapshot_wire_codec::*;
