//! 📝️ Text representation codec surface for `s.stdio.semio.document` (snapshot) — grammar include.
pub const COMPONENT_GRAMMAR_SEMIO: &str = include_str!("📖️.grammar.semio");
pub const COMPONENT_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️.grammar.semio");

#[allow(unused_imports)]
mod snapshot_codec {
use super::*;
use crate::schema::snapshot::*;
use semio_s_artifact_stdio_semio_base::io::text::codec::split_top_level;
use semio_s_artifact_stdio_semio_base::io::text::codec::strip_brackets;
use crate::io::text::snapshot::dec_image;
use crate::io::text::snapshot::enc_image;
use crate::io::text::snapshot::dec_style;
use crate::io::text::snapshot::enc_style;
use crate::io::text::snapshot::dec_block;
use crate::io::text::snapshot::enc_block;
use semio_s_artifact_stdio_semio_base::io::text::codec::dec_str;
use semio_s_artifact_stdio_semio_base::io::text::codec::enc_str;
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
impl pack::snapshot::ArtifactDsl for SemioDocumentSnapshot {
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
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(<Self as pack::snapshot::ArtifactDsl>::envelope_id(), store::semio_format::Component::Dsl, 1).expect("valid envelope_id");
        store::semio_format::wrap_text(&envelope, &body)
    }
}

/// 📥️ Parses this subset's own committed `.dsl.semio` text into a real [`SemioDocumentSnapshot`] — a thin
/// wrapper over `pack::snapshot::ArtifactDsl::parse_dsl` so external Rust callers that cannot name this
/// crate's private `store` extern-crate item (the `📃️mutate-semio-document` test adapter, which reads the
/// REAL committed example artifact rather than a hand-transcribed Rust literal of it) can still
/// drive the same codec production does. Same rationale as `🧰️kit`'s `decode_kit_snapshot_json`.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn parse_semio_document_dsl(text: &str) -> Result<SemioDocumentSnapshot, String> {
    <SemioDocumentSnapshot as pack::snapshot::ArtifactDsl>::parse_dsl(text).map_err(|error| error.to_string())
}

/// 📤️ The `pack::snapshot::ArtifactDsl::print_dsl` inverse of [`parse_semio_document_dsl`] — same rationale.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn print_semio_document_dsl(snapshot: &SemioDocumentSnapshot) -> String {
    <SemioDocumentSnapshot as pack::snapshot::ArtifactDsl>::print_dsl(snapshot)
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
use crate::schema::snapshot::*;
use semio_s_artifact_stdio_semio_base::io::text::codec::strip_brackets;
use semio_s_artifact_stdio_semio_base::io::text::codec::split_top_level;
use semio_s_artifact_stdio_semio_base::io::text::codec::dec_str;
use semio_s_artifact_stdio_semio_base::io::text::codec::enc_str;
use framework_schema::ArtifactSchema;
use semio_s_artifact_stdio_semio_base::io::text::codec::{hex_encode, hex_decode};
use semio_s_artifact_stdio_semio_base::io::text::codec::{encode_option, decode_option};




// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn enc_bool(b: &bool) -> String {
    if *b {
        "1".to_string()
    } else {
        "0".to_string()
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn dec_bool(s: &str) -> Result<bool, String> {
    match s {
        "1" => Ok(true),
        "0" => Ok(false),
        other => Err(format!("bool: bad value {other:?}")),
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn enc_u8(v: &u8) -> String {
    v.to_string()
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn dec_u8(s: &str) -> Result<u8, String> {
    s.parse().map_err(|e: std::num::ParseIntError| e.to_string())
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn enc_f64(v: &f64) -> String {
    v.to_bits().to_string()
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn dec_f64(s: &str) -> Result<f64, String> {
    s.parse::<u64>().map(f64::from_bits).map_err(|e| e.to_string())
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn enc_list<T>(items: &[T], enc: impl Fn(&T) -> String) -> String {
    format!("[{}]", items.iter().map(enc).collect::<Vec<_>>().join(","))
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn dec_list<T>(s: &str, dec: impl Fn(&str) -> Result<T, String>) -> Result<Vec<T>, String> {
    split_top_level(strip_brackets(s)?, ',').into_iter().filter(|s| !s.is_empty()).map(dec).collect()
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn enc_run_style(s: &RunStyle) -> String {
    format!(
        "[{},{},{},{},{},{},{}]",
        enc_bool(&s.bold),
        enc_bool(&s.italic),
        enc_bool(&s.underline),
        encode_option(&s.size, enc_f64),
        encode_option(&s.font, |v| enc_str(v)),
        encode_option(&s.color, |v| enc_str(v)),
        encode_option(&s.link, |v| enc_str(v))
    )
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn dec_run_style(s: &str) -> Result<RunStyle, String> {
    let inner = strip_brackets(s)?;
    let parts = split_top_level(inner, ',');
    let [bold, italic, underline, size, font, color, link] = parts.as_slice() else { return Err(format!("run style: expected 7 fields, got {}", parts.len())) };
    Ok(RunStyle {
        bold: dec_bool(bold)?,
        italic: dec_bool(italic)?,
        underline: dec_bool(underline)?,
        size: decode_option(size, dec_f64)?,
        font: decode_option(font, dec_str)?,
        color: decode_option(color, dec_str)?,
        link: decode_option(link, dec_str)?,
    })
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn enc_run(r: &DocRun) -> String {
    format!("[{},{}]", enc_str(&r.text), enc_run_style(&r.style))
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn dec_run(s: &str) -> Result<DocRun, String> {
    let inner = strip_brackets(s)?;
    let parts = split_top_level(inner, ',');
    let [text, style] = parts.as_slice() else { return Err(format!("run: expected 2 fields, got {}", parts.len())) };
    Ok(DocRun { text: dec_str(text)?, style: dec_run_style(style)? })
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn enc_block(b: &DocBlock) -> String {
    match b {
        DocBlock::Paragraph { style_id, runs } => format!("P[{},{}]", encode_option(style_id, |v| enc_str(v)), enc_list(runs, enc_run)),
        DocBlock::Heading { level, style_id, runs } => format!("H[{},{},{}]", enc_u8(level), encode_option(style_id, |v| enc_str(v)), enc_list(runs, enc_run)),
        DocBlock::List { ordered, items } => format!("L[{},{}]", enc_bool(ordered), enc_list(items, enc_list_item)),
        DocBlock::Table { rows } => format!("T[{}]", enc_list(rows, enc_row)),
        DocBlock::Code { language, text } => format!("C[{},{}]", encode_option(language, |v| enc_str(v)), enc_str(text)),
        DocBlock::Quote { blocks } => format!("Q[{}]", enc_list(blocks, enc_block)),
        DocBlock::Image { image_id, alt, width, height } => format!("I[{},{},{},{}]", enc_str(image_id), enc_str(alt), encode_option(width, enc_f64), encode_option(height, enc_f64)),
        DocBlock::PageBreak => "B[]".to_string(),
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn dec_block(s: &str) -> Result<DocBlock, String> {
    let (tag, rest) = s.split_at(1);
    let inner = strip_brackets(rest)?;
    match tag {
        "P" => {
            let parts = split_top_level(inner, ',');
            let [style_id, runs] = parts.as_slice() else { return Err(format!("paragraph: expected 2 fields, got {}", parts.len())) };
            Ok(DocBlock::Paragraph { style_id: decode_option(style_id, dec_str)?, runs: dec_list(runs, dec_run)? })
        }
        "H" => {
            let parts = split_top_level(inner, ',');
            let [level, style_id, runs] = parts.as_slice() else { return Err(format!("heading: expected 3 fields, got {}", parts.len())) };
            Ok(DocBlock::Heading { level: dec_u8(level)?, style_id: decode_option(style_id, dec_str)?, runs: dec_list(runs, dec_run)? })
        }
        "L" => {
            let parts = split_top_level(inner, ',');
            let [ordered, items] = parts.as_slice() else { return Err(format!("list: expected 2 fields, got {}", parts.len())) };
            Ok(DocBlock::List { ordered: dec_bool(ordered)?, items: dec_list(items, dec_list_item)? })
        }
        "T" => Ok(DocBlock::Table { rows: dec_list(inner, dec_row)? }),
        "C" => {
            let parts = split_top_level(inner, ',');
            let [language, text] = parts.as_slice() else { return Err(format!("code: expected 2 fields, got {}", parts.len())) };
            Ok(DocBlock::Code { language: decode_option(language, dec_str)?, text: dec_str(text)? })
        }
        "Q" => Ok(DocBlock::Quote { blocks: dec_list(inner, dec_block)? }),
        "I" => {
            let parts = split_top_level(inner, ',');
            let [image_id, alt, width, height] = parts.as_slice() else { return Err(format!("image: expected 4 fields, got {}", parts.len())) };
            Ok(DocBlock::Image { image_id: dec_str(image_id)?, alt: dec_str(alt)?, width: decode_option(width, dec_f64)?, height: decode_option(height, dec_f64)? })
        }
        "B" => Ok(DocBlock::PageBreak),
        other => Err(format!("block: unknown tag {other:?}")),
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn enc_list_item(i: &DocListItem) -> String {
    format!("[{}]", enc_list(&i.blocks, enc_block))
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn dec_list_item(s: &str) -> Result<DocListItem, String> {
    let inner = strip_brackets(s)?;
    Ok(DocListItem { blocks: dec_list(inner, dec_block)? })
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn enc_cell(c: &DocTableCell) -> String {
    format!("[{}]", enc_list(&c.blocks, enc_block))
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn dec_cell(s: &str) -> Result<DocTableCell, String> {
    let inner = strip_brackets(s)?;
    Ok(DocTableCell { blocks: dec_list(inner, dec_block)? })
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn enc_row(r: &DocTableRow) -> String {
    format!("[{}]", enc_list(&r.cells, enc_cell))
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn dec_row(s: &str) -> Result<DocTableRow, String> {
    let inner = strip_brackets(s)?;
    Ok(DocTableRow { cells: dec_list(inner, dec_cell)? })
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn enc_style(s: &DocStyle) -> String {
    format!("[{},{},{}]", enc_str(&s.id), enc_str(&s.name), encode_option(&s.based_on, |v| enc_str(v)))
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn dec_style(s: &str) -> Result<DocStyle, String> {
    let inner = strip_brackets(s)?;
    let parts = split_top_level(inner, ',');
    let [id, name, based_on] = parts.as_slice() else { return Err(format!("style: expected 3 fields, got {}", parts.len())) };
    Ok(DocStyle { id: dec_str(id)?, name: dec_str(name)?, based_on: decode_option(based_on, dec_str)? })
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn enc_image(i: &DocImage) -> String {
    format!("[{},{},{}]", enc_str(&i.id), enc_str(&i.mime), hex_encode(&i.bytes))
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn dec_image(s: &str) -> Result<DocImage, String> {
    let inner = strip_brackets(s)?;
    let parts = split_top_level(inner, ',');
    let [id, mime, bytes] = parts.as_slice() else { return Err(format!("image: expected 3 fields, got {}", parts.len())) };
    Ok(DocImage { id: dec_str(id)?, mime: dec_str(mime)?, bytes: hex_decode(bytes)? })
}

}
pub use snapshot_wire_codec::*;
