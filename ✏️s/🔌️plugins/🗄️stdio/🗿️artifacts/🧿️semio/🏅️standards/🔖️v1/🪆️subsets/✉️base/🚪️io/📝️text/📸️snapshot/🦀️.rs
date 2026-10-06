//! 📝️ Text representation codec surface for `stdio.semio` (snapshot) — grammar include.
pub const COMPONENT_GRAMMAR_SEMIO: &str = include_str!("📖️.grammar.semio");
pub const COMPONENT_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️.grammar.semio");

#[allow(unused_imports)]
mod snapshot_codec {
use super::*;
use crate::standards::v1::subsets::base::schema::snapshot::*;
use crate::standards::v1::subsets::animation::schema::snapshot::SemioAnimationSnapshot;
use crate::standards::v1::subsets::audio::schema::snapshot::SemioAudioSnapshot;
use crate::standards::v1::subsets::brep::schema::snapshot::SemioBrepSnapshot;
use crate::standards::v1::subsets::cad::schema::snapshot::SemioCadSnapshot;
use crate::standards::v1::subsets::document::schema::snapshot::SemioDocumentSnapshot;
use crate::standards::v1::subsets::drawing::schema::snapshot::SemioDrawingSnapshot;
use crate::standards::v1::subsets::flow::schema::snapshot::SemioFlowSnapshot;
use crate::standards::v1::subsets::graph::schema::snapshot::SemioGraphSnapshot;
use crate::standards::v1::subsets::image::schema::snapshot::SemioImageSnapshot;
use crate::standards::v1::subsets::kit::schema::snapshot::SemioKitSnapshot;
use crate::standards::v1::subsets::mesh::schema::snapshot::SemioMeshSnapshot;
use crate::standards::v1::subsets::model::schema::snapshot::SemioModelSnapshot;
use crate::standards::v1::subsets::object::schema::snapshot::SemioObjectSnapshot;
use crate::standards::v1::subsets::presentation::schema::snapshot::SemioPresentationSnapshot;
use crate::standards::v1::subsets::table::schema::snapshot::SemioTableSnapshot;
use crate::standards::v1::subsets::text::schema::snapshot::SemioTextSnapshot;
use crate::standards::v1::subsets::value::schema::snapshot::SemioValueSnapshot;
use crate::standards::v1::subsets::video::schema::snapshot::SemioVideoSnapshot;
use framework_schema::ArtifactSchema;

/// 🏷️ The wire tag naming which of the 13 domain subsets is carried — shared by the text DSL's
/// `subset=<tag>` header line and used to select which subset's own REAL codec to delegate to.
/// `pub(crate)` (not private) since `💡️inferences/🏷️kind/🦀️.rs`
/// (ticket 26/08/12/INTRODUCE-INFERENCE-SCHEMA-FAMILY-WITH-DEPENDENCY-AWARE-CACHING) is a sibling
/// module, not a descendant, and needs this same dispatch as its own honest derivation.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn subset_tag(s: &SemioSubsetSnapshot) -> &'static str {
    match s {
        SemioSubsetSnapshot::Brep(_) => "brep",
        SemioSubsetSnapshot::Mesh(_) => "mesh",
        SemioSubsetSnapshot::Model(_) => "model",
        SemioSubsetSnapshot::Value(_) => "value",
        SemioSubsetSnapshot::Document(_) => "document",
        SemioSubsetSnapshot::Cad(_) => "cad",
        SemioSubsetSnapshot::Drawing(_) => "drawing",
        SemioSubsetSnapshot::Image(_) => "image",
        SemioSubsetSnapshot::Video(_) => "video",
        SemioSubsetSnapshot::Audio(_) => "audio",
        SemioSubsetSnapshot::Animation(_) => "animation",
        SemioSubsetSnapshot::Presentation(_) => "presentation",
        SemioSubsetSnapshot::Flow(_) => "flow",
        SemioSubsetSnapshot::Text(_) => "text",
        SemioSubsetSnapshot::Table(_) => "table",
        SemioSubsetSnapshot::Graph(_) => "graph",
        SemioSubsetSnapshot::Object(_) => "object",
        SemioSubsetSnapshot::Kit(_) => "kit",
    }
}

/// 🧪️ W-S closer (`any`): real delegating text DSL — NOT a re-derivation of any of the 13
/// wrapped subsets' own hex/bracket/recursive grammars. The body is exactly 2 header lines
/// (`subset=<tag>`, `schema=<hex>`) followed by the WRAPPED subset's own real `print_dsl()`
/// output with ITS OWN preamble line stripped (this envelope already carries its own `semio
/// stdio.semio.dsl v1` preamble via `store::semio_format::wrap_text` below — embedding a second
/// preamble line would double up). `parse_dsl` hands the un-prefixed remainder straight to the
/// matching subset's own real `parse_dsl` — every one of those already tolerates a missing
/// preamble (falls back to treating the whole text as body), the same convention this envelope's
/// own `parse_dsl` itself relies on one level up.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn hex_encode(bytes: &[u8]) -> String {
    bytes.iter().map(|b| format!("{b:02x}")).collect()
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn hex_decode(s: &str) -> Result<Vec<u8>, String> {
    if !s.len().is_multiple_of(2) {
        return Err(format!("odd hex length: {s:?}"));
    }
    (0..s.len()).step_by(2).map(|i| u8::from_str_radix(&s[i..i + 2], 16).map_err(|e| e.to_string())).collect()
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn strip_inner_preamble(text: &str) -> &str {
    match store::semio_format::split_text_preamble(text) {
        Ok((_, rest)) => rest,
        Err(_) => text,
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_semio_snapshot_body(snap: &SemioSnapshot) -> String {
    let tag = subset_tag(&snap.subset);
    let inner_printed = match &snap.subset {
        SemioSubsetSnapshot::Brep(s) => <SemioBrepSnapshot as store::ArtifactDsl>::print_dsl(s),
        SemioSubsetSnapshot::Mesh(s) => <SemioMeshSnapshot as store::ArtifactDsl>::print_dsl(s),
        SemioSubsetSnapshot::Model(s) => <SemioModelSnapshot as store::ArtifactDsl>::print_dsl(s),
        SemioSubsetSnapshot::Value(s) => <SemioValueSnapshot as store::ArtifactDsl>::print_dsl(s),
        SemioSubsetSnapshot::Document(s) => <SemioDocumentSnapshot as store::ArtifactDsl>::print_dsl(s),
        SemioSubsetSnapshot::Cad(s) => <SemioCadSnapshot as store::ArtifactDsl>::print_dsl(s),
        SemioSubsetSnapshot::Drawing(s) => <SemioDrawingSnapshot as store::ArtifactDsl>::print_dsl(s),
        SemioSubsetSnapshot::Image(s) => <SemioImageSnapshot as store::ArtifactDsl>::print_dsl(s),
        SemioSubsetSnapshot::Video(s) => <SemioVideoSnapshot as store::ArtifactDsl>::print_dsl(s),
        SemioSubsetSnapshot::Audio(s) => <SemioAudioSnapshot as store::ArtifactDsl>::print_dsl(s),
        SemioSubsetSnapshot::Animation(s) => <SemioAnimationSnapshot as store::ArtifactDsl>::print_dsl(s),
        SemioSubsetSnapshot::Presentation(s) => <SemioPresentationSnapshot as store::ArtifactDsl>::print_dsl(s),
        SemioSubsetSnapshot::Flow(s) => <SemioFlowSnapshot as store::ArtifactDsl>::print_dsl(s),
        SemioSubsetSnapshot::Text(s) => <SemioTextSnapshot as store::ArtifactDsl>::print_dsl(s),
        SemioSubsetSnapshot::Table(s) => <SemioTableSnapshot as store::ArtifactDsl>::print_dsl(s),
        SemioSubsetSnapshot::Graph(s) => <SemioGraphSnapshot as store::ArtifactDsl>::print_dsl(s),
        SemioSubsetSnapshot::Object(s) => <SemioObjectSnapshot as store::ArtifactDsl>::print_dsl(s),
        SemioSubsetSnapshot::Kit(s) => <SemioKitSnapshot as store::ArtifactDsl>::print_dsl(s),
    };
    let inner_body = strip_inner_preamble(&inner_printed);
    format!("subset={tag}\nschema={}\n{inner_body}", hex_encode(snap.schema.as_bytes()))
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_semio_snapshot_body(body: &str) -> Result<SemioSnapshot, String> {
    let mut parts = body.splitn(3, '\n');
    let subset_line = parts.next().ok_or_else(|| "semio snapshot: missing subset line".to_string())?.trim();
    let tag = subset_line.strip_prefix("subset=").ok_or_else(|| format!("semio snapshot: expected subset= line, got {subset_line:?}"))?;
    let schema_line = parts.next().ok_or_else(|| "semio snapshot: missing schema line".to_string())?.trim();
    let schema_hex = schema_line.strip_prefix("schema=").ok_or_else(|| format!("semio snapshot: expected schema= line, got {schema_line:?}"))?;
    let schema = String::from_utf8(hex_decode(schema_hex)?).map_err(|e| e.to_string())?;
    let inner_body = parts.next().unwrap_or("");
    let subset = match tag {
        "brep" => SemioSubsetSnapshot::Brep(<SemioBrepSnapshot as store::ArtifactDsl>::parse_dsl(inner_body).map_err(|e| e.to_string())?),
        "mesh" => SemioSubsetSnapshot::Mesh(<SemioMeshSnapshot as store::ArtifactDsl>::parse_dsl(inner_body).map_err(|e| e.to_string())?),
        "model" => SemioSubsetSnapshot::Model(<SemioModelSnapshot as store::ArtifactDsl>::parse_dsl(inner_body).map_err(|e| e.to_string())?),
        "value" => SemioSubsetSnapshot::Value(<SemioValueSnapshot as store::ArtifactDsl>::parse_dsl(inner_body).map_err(|e| e.to_string())?),
        "document" => SemioSubsetSnapshot::Document(<SemioDocumentSnapshot as store::ArtifactDsl>::parse_dsl(inner_body).map_err(|e| e.to_string())?),
        "cad" => SemioSubsetSnapshot::Cad(<SemioCadSnapshot as store::ArtifactDsl>::parse_dsl(inner_body).map_err(|e| e.to_string())?),
        "drawing" => SemioSubsetSnapshot::Drawing(<SemioDrawingSnapshot as store::ArtifactDsl>::parse_dsl(inner_body).map_err(|e| e.to_string())?),
        "image" => SemioSubsetSnapshot::Image(<SemioImageSnapshot as store::ArtifactDsl>::parse_dsl(inner_body).map_err(|e| e.to_string())?),
        "video" => SemioSubsetSnapshot::Video(<SemioVideoSnapshot as store::ArtifactDsl>::parse_dsl(inner_body).map_err(|e| e.to_string())?),
        "audio" => SemioSubsetSnapshot::Audio(<SemioAudioSnapshot as store::ArtifactDsl>::parse_dsl(inner_body).map_err(|e| e.to_string())?),
        "animation" => SemioSubsetSnapshot::Animation(<SemioAnimationSnapshot as store::ArtifactDsl>::parse_dsl(inner_body).map_err(|e| e.to_string())?),
        "presentation" => SemioSubsetSnapshot::Presentation(<SemioPresentationSnapshot as store::ArtifactDsl>::parse_dsl(inner_body).map_err(|e| e.to_string())?),
        "flow" => SemioSubsetSnapshot::Flow(<SemioFlowSnapshot as store::ArtifactDsl>::parse_dsl(inner_body).map_err(|e| e.to_string())?),
        "text" => SemioSubsetSnapshot::Text(<SemioTextSnapshot as store::ArtifactDsl>::parse_dsl(inner_body).map_err(|e| e.to_string())?),
        "table" => SemioSubsetSnapshot::Table(<SemioTableSnapshot as store::ArtifactDsl>::parse_dsl(inner_body).map_err(|e| e.to_string())?),
        "graph" => SemioSubsetSnapshot::Graph(<SemioGraphSnapshot as store::ArtifactDsl>::parse_dsl(inner_body).map_err(|e| e.to_string())?),
        "object" => SemioSubsetSnapshot::Object(<SemioObjectSnapshot as store::ArtifactDsl>::parse_dsl(inner_body).map_err(|e| e.to_string())?),
        "kit" => SemioSubsetSnapshot::Kit(<SemioKitSnapshot as store::ArtifactDsl>::parse_dsl(inner_body).map_err(|e| e.to_string())?),
        other => return Err(format!("semio snapshot: unknown subset tag {other:?}")),
    };
    Ok(SemioSnapshot { schema, subset })
}

/// 🎁 Real delegating text/binary codecs — replaces the old hex-of-`serde_json` scaffold. Every
/// byte of the WRAPPED subset's own payload is produced/consumed by THAT subset's own real,
/// already-conformance-tested `ArtifactDsl`/`ArtifactPack` impl; this envelope only adds the
/// `subset`/`schema` header and the outer `store::semio_format` wrapping every stdio artifact uses.
impl store::ArtifactDsl for SemioSnapshot {
    const EXTENSION: &'static str = "semio";
    fn envelope_id() -> &'static str {
        STDIO_SEMIO_DOCUMENT_SCHEMA
    }

    fn parse_dsl(text: &str) -> Result<Self, semio_framework_diagnostic::TextError> {
        let body = match store::semio_format::split_text_preamble(text) {
            Ok((_, rest)) => rest,
            Err(_) => text,
        };
        dec_semio_snapshot_body(body).map_err(|e| semio_framework_diagnostic::TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue, e, semio_framework_diagnostic::TextSpan::at(1, 1)))
    }

    fn print_dsl(&self) -> String {
        let body = enc_semio_snapshot_body(self);
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(<Self as store::ArtifactDsl>::envelope_id(), store::semio_format::Component::Dsl, 1).expect("valid envelope_id");
        store::semio_format::wrap_text(&envelope, &body)
    }
}

/// 📥️ Parses the ENVELOPE's own committed `.dsl.semio` text into a real [`SemioSnapshot`] — a thin
/// wrapper over `store::ArtifactDsl::parse_dsl` so external Rust callers that cannot name this
/// crate's private `store` extern-crate item (the `✉️mutate-semio-base` test adapter, whose
/// `identity-round-trip` scenario reads the REAL committed `📚️examples/🌐️envelope` artifact) can still
/// drive the same codec production does. Same shape and same rationale as `🌊️flow`'s own bridge.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn parse_semio_envelope_dsl(text: &str) -> Result<SemioSnapshot, String> {
    <SemioSnapshot as store::ArtifactDsl>::parse_dsl(text).map_err(|error| error.to_string())
}

/// 📤️ The `store::ArtifactDsl::print_dsl` inverse of [`parse_semio_envelope_dsl`] — same rationale.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn print_semio_envelope_dsl(snapshot: &SemioSnapshot) -> String {
    <SemioSnapshot as store::ArtifactDsl>::print_dsl(snapshot)
}

/// 📤️ The envelope's JSON carrier, derived from the schema types themselves: `SemioSnapshot`'s own
/// `ToValue` (`camelCase` fields, `SemioSubsetSnapshot` internally tagged on `subset`) printed by the
/// first-party `pack` JSON codec — the shape `🧬️schema/📸️snapshot/🔣️.json` publishes and the committed
/// `🧫️fixtures/🧬️mutations/` vectors carry. See <../🔣️.json>.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn encode_semio_snapshot_json(snapshot: &SemioSnapshot) -> String {
    semio_framework_pack_json::to_json_string(snapshot)
}

/// 📥️ The inverse of [`encode_semio_snapshot_json`], through `SemioSnapshot`'s own `FromValue`.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn decode_semio_snapshot_json(text: &str) -> Result<SemioSnapshot, String> {
    semio_framework_pack_json::from_json_str(text, semio_framework_pack_json::JsonMemberPolicy::Reject).map_err(|error| error.to_string())
}
}
pub use snapshot_codec::*;

#[allow(unused_imports)]
mod snapshot_wire_codec {
use super::*;
use crate::standards::v1::subsets::base::schema::snapshot::*;
use crate::standards::v1::subsets::animation::schema::snapshot::SemioAnimationSnapshot;
use crate::standards::v1::subsets::audio::schema::snapshot::SemioAudioSnapshot;
use crate::standards::v1::subsets::brep::schema::snapshot::SemioBrepSnapshot;
use crate::standards::v1::subsets::cad::schema::snapshot::SemioCadSnapshot;
use crate::standards::v1::subsets::document::schema::snapshot::SemioDocumentSnapshot;
use crate::standards::v1::subsets::drawing::schema::snapshot::SemioDrawingSnapshot;
use crate::standards::v1::subsets::flow::schema::snapshot::SemioFlowSnapshot;
use crate::standards::v1::subsets::graph::schema::snapshot::SemioGraphSnapshot;
use crate::standards::v1::subsets::image::schema::snapshot::SemioImageSnapshot;
use crate::standards::v1::subsets::kit::schema::snapshot::SemioKitSnapshot;
use crate::standards::v1::subsets::mesh::schema::snapshot::SemioMeshSnapshot;
use crate::standards::v1::subsets::model::schema::snapshot::SemioModelSnapshot;
use crate::standards::v1::subsets::object::schema::snapshot::SemioObjectSnapshot;
use crate::standards::v1::subsets::presentation::schema::snapshot::SemioPresentationSnapshot;
use crate::standards::v1::subsets::table::schema::snapshot::SemioTableSnapshot;
use crate::standards::v1::subsets::text::schema::snapshot::SemioTextSnapshot;
use crate::standards::v1::subsets::value::schema::snapshot::SemioValueSnapshot;
use crate::standards::v1::subsets::video::schema::snapshot::SemioVideoSnapshot;
use framework_schema::ArtifactSchema;




}
pub use snapshot_wire_codec::*;

#[allow(unused_imports)]
mod snapshot_wire2_codec {
use crate::standards::v1::subsets::base::schema::triples::*;


/// 📐️ Bracket-depth-aware split — a `sep` inside `[...]` never splits (so a modified/added
/// entry's own nested `[...]` payload survives the outer `;`/`,` split intact).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn split_top_level(s: &str, sep: char) -> Vec<&str> {
    if s.is_empty() {
        return Vec::new();
    }
    let mut out = Vec::new();
    let mut depth = 0i32;
    let mut start = 0usize;
    for (i, c) in s.char_indices() {
        match c {
            '[' => depth += 1,
            ']' => depth -= 1,
            c if c == sep && depth == 0 => {
                out.push(&s[start..i]);
                start = i + c.len_utf8();
            }
            _ => {}
        }
    }
    out.push(&s[start..]);
    out
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn strip_brackets(s: &str) -> Result<&str, String> {
    s.strip_prefix('[').and_then(|s| s.strip_suffix(']')).ok_or_else(|| format!("expected [...], got {s:?}"))
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn enc_indexed_triple<D, T>(diff: &IndexedTripleDiff<D, T>, enc_d: impl Fn(&D) -> String, enc_t: impl Fn(&T) -> String) -> String {
    let removed = diff.removed.iter().map(|i| i.to_string()).collect::<Vec<_>>().join(",");
    let modified = diff.modified.iter().map(|m| format!("{}:{}", m.index, enc_d(&m.diff))).collect::<Vec<_>>().join(",");
    let added = diff.added.iter().map(|a| format!("{}:{}", a.index, enc_t(&a.item))).collect::<Vec<_>>().join(",");
    format!("[{removed}];[{modified}];[{added}]")
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn dec_indexed_triple<D, T>(body: &str, dec_d: impl Fn(&str) -> Result<D, String>, dec_t: impl Fn(&str) -> Result<T, String>) -> Result<IndexedTripleDiff<D, T>, String> {
    let three = split_top_level(body, ';');
    let [removed_s, modified_s, added_s] = three.as_slice() else { return Err(format!("indexed triple: expected 3 sections, got {}", three.len())) };
    let removed = split_top_level(strip_brackets(removed_s)?, ',').into_iter().filter(|s| !s.is_empty()).map(|s| s.parse::<usize>().map_err(|e: std::num::ParseIntError| e.to_string())).collect::<Result<Vec<_>, String>>()?;
    let modified = split_top_level(strip_brackets(modified_s)?, ',')
        .into_iter()
        .filter(|s| !s.is_empty())
        .map(|entry| {
            let (idx, rest) = entry.split_once(':').ok_or_else(|| format!("indexed modified: bad entry {entry:?}"))?;
            Ok(IndexModified { index: idx.parse::<usize>().map_err(|e: std::num::ParseIntError| e.to_string())?, diff: dec_d(rest)? })
        })
        .collect::<Result<Vec<_>, String>>()?;
    let added = split_top_level(strip_brackets(added_s)?, ',')
        .into_iter()
        .filter(|s| !s.is_empty())
        .map(|entry| {
            let (idx, rest) = entry.split_once(':').ok_or_else(|| format!("indexed added: bad entry {entry:?}"))?;
            Ok(IndexAdded { index: idx.parse::<usize>().map_err(|e: std::num::ParseIntError| e.to_string())?, item: dec_t(rest)? })
        })
        .collect::<Result<Vec<_>, String>>()?;
    Ok(IndexedTripleDiff { removed, modified, added })
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn enc_named_triple<K, D, T>(triple: &NamedTripleDiff<K, D, T>, enc_k: impl Fn(&K) -> String, enc_d: impl Fn(&D) -> String, enc_t: impl Fn(&T) -> String) -> String {
    let removed = triple.removed.iter().map(&enc_k).collect::<Vec<_>>().join(",");
    let modified = triple.modified.iter().map(|m| format!("{}:{}", enc_k(&m.key), enc_d(&m.diff))).collect::<Vec<_>>().join(",");
    let added = triple.added.iter().map(enc_t).collect::<Vec<_>>().join(",");
    format!("[{removed}];[{modified}];[{added}]")
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn dec_named_triple<K, D, T>(s: &str, dec_k: impl Fn(&str) -> Result<K, String>, dec_d: impl Fn(&str) -> Result<D, String>, dec_t: impl Fn(&str) -> Result<T, String>) -> Result<NamedTripleDiff<K, D, T>, String> {
    let three = split_top_level(s, ';');
    let [removed_s, modified_s, added_s] = three.as_slice() else { return Err(format!("named triple: expected 3 sections, got {}", three.len())) };
    let removed = split_top_level(strip_brackets(removed_s)?, ',').into_iter().filter(|s| !s.is_empty()).map(&dec_k).collect::<Result<Vec<_>, String>>()?;
    let modified = split_top_level(strip_brackets(modified_s)?, ',')
        .into_iter()
        .filter(|s| !s.is_empty())
        .map(|entry| {
            let (k, rest) = entry.split_once(':').ok_or_else(|| format!("named triple modified: bad entry {entry:?}"))?;
            Ok(NamedModified { key: dec_k(k)?, diff: dec_d(rest)? })
        })
        .collect::<Result<Vec<_>, String>>()?;
    let added = split_top_level(strip_brackets(added_s)?, ',').into_iter().filter(|s| !s.is_empty()).map(dec_t).collect::<Result<Vec<_>, String>>()?;
    Ok(NamedTripleDiff { removed, modified, added })
}

/// 🧷 `NamedAdded<T>`-wrapping encode/decode pair (`index:item`, same `idx:rest` shape
/// `enc_indexed_triple`/`dec_indexed_triple` already use for `IndexAdded<T>` above) — for a
/// consumer instantiating `NamedTripleDiff<K, D, NamedAdded<T>>`, pass
/// `|a| enc_named_added(a, enc_t)`/`|s| dec_named_added(s, dec_t)` as `enc_named_triple`'s/
/// `dec_named_triple`'s own `enc_t`/`dec_t` argument.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn enc_named_added<T>(a: &NamedAdded<T>, enc_t: impl Fn(&T) -> String) -> String {
    format!("{}:{}", a.index, enc_t(&a.item))
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn dec_named_added<T>(s: &str, dec_t: impl Fn(&str) -> Result<T, String>) -> Result<NamedAdded<T>, String> {
    let (idx, rest) = s.split_once(':').ok_or_else(|| format!("named added: bad entry {s:?}"))?;
    Ok(NamedAdded { index: idx.parse::<usize>().map_err(|e: std::num::ParseIntError| e.to_string())?, item: dec_t(rest)? })
}
}
pub use snapshot_wire2_codec::*;
