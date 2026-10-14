//! 📝️ Text representation codec surface for `stdio.semio` (snapshot) — grammar include.
pub const COMPONENT_GRAMMAR_SEMIO: &str = include_str!("📖️.grammar.semio");
pub const COMPONENT_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️.grammar.semio");

#[allow(unused_imports)]
mod snapshot_codec {
use super::*;
use crate::schema::snapshot::*;
use semio_s_artifact_stdio_semio_animation::schema::snapshot::SemioAnimationSnapshot;
use semio_s_artifact_stdio_semio_audio_model::schema::snapshot::SemioAudioSnapshot;
use semio_s_artifact_stdio_semio_brep::schema::snapshot::SemioBrepSnapshot;
use semio_s_artifact_stdio_semio_cad::schema::snapshot::SemioCadSnapshot;
use semio_s_artifact_stdio_semio_document::schema::snapshot::SemioDocumentSnapshot;
use semio_s_artifact_stdio_semio_drawing::schema::snapshot::SemioDrawingSnapshot;
use semio_s_artifact_stdio_semio_flow::schema::snapshot::SemioFlowSnapshot;
use semio_s_artifact_stdio_semio_graph::schema::snapshot::SemioGraphSnapshot;
use semio_s_artifact_stdio_semio_image::schema::snapshot::SemioImageSnapshot;
use semio_s_artifact_stdio_semio_kit::schema::snapshot::SemioKitSnapshot;
use semio_s_artifact_stdio_semio_mesh::schema::snapshot::SemioMeshSnapshot;
use semio_s_artifact_stdio_semio_model::schema::snapshot::SemioModelSnapshot;
use semio_s_artifact_stdio_semio_object::schema::snapshot::SemioObjectSnapshot;
use semio_s_artifact_stdio_semio_presentation::schema::snapshot::SemioPresentationSnapshot;
use semio_s_artifact_stdio_semio_table_model::schema::snapshot::SemioTableSnapshot;
use semio_s_artifact_stdio_semio_text_model::schema::snapshot::SemioTextSnapshot;
use semio_s_artifact_stdio_semio_value_model::schema::snapshot::SemioValueSnapshot;
use semio_s_artifact_stdio_semio_video_model::schema::snapshot::SemioVideoSnapshot;
use framework_schema::ArtifactSchema;



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
        SemioSubsetSnapshot::Brep(s) => <SemioBrepSnapshot as pack::snapshot::ArtifactDsl>::print_dsl(s),
        SemioSubsetSnapshot::Mesh(s) => <SemioMeshSnapshot as pack::snapshot::ArtifactDsl>::print_dsl(s),
        SemioSubsetSnapshot::Model(s) => <SemioModelSnapshot as pack::snapshot::ArtifactDsl>::print_dsl(s),
        SemioSubsetSnapshot::Value(s) => <SemioValueSnapshot as pack::snapshot::ArtifactDsl>::print_dsl(s),
        SemioSubsetSnapshot::Document(s) => <SemioDocumentSnapshot as pack::snapshot::ArtifactDsl>::print_dsl(s),
        SemioSubsetSnapshot::Cad(s) => <SemioCadSnapshot as pack::snapshot::ArtifactDsl>::print_dsl(s),
        SemioSubsetSnapshot::Drawing(s) => <SemioDrawingSnapshot as pack::snapshot::ArtifactDsl>::print_dsl(s),
        SemioSubsetSnapshot::Image(s) => <SemioImageSnapshot as pack::snapshot::ArtifactDsl>::print_dsl(s),
        SemioSubsetSnapshot::Video(s) => <SemioVideoSnapshot as pack::snapshot::ArtifactDsl>::print_dsl(s),
        SemioSubsetSnapshot::Audio(s) => <SemioAudioSnapshot as pack::snapshot::ArtifactDsl>::print_dsl(s),
        SemioSubsetSnapshot::Animation(s) => <SemioAnimationSnapshot as pack::snapshot::ArtifactDsl>::print_dsl(s),
        SemioSubsetSnapshot::Presentation(s) => <SemioPresentationSnapshot as pack::snapshot::ArtifactDsl>::print_dsl(s),
        SemioSubsetSnapshot::Flow(s) => <SemioFlowSnapshot as pack::snapshot::ArtifactDsl>::print_dsl(s),
        SemioSubsetSnapshot::Text(s) => <SemioTextSnapshot as pack::snapshot::ArtifactDsl>::print_dsl(s),
        SemioSubsetSnapshot::Table(s) => <SemioTableSnapshot as pack::snapshot::ArtifactDsl>::print_dsl(s),
        SemioSubsetSnapshot::Graph(s) => <SemioGraphSnapshot as pack::snapshot::ArtifactDsl>::print_dsl(s),
        SemioSubsetSnapshot::Object(s) => <SemioObjectSnapshot as pack::snapshot::ArtifactDsl>::print_dsl(s),
        SemioSubsetSnapshot::Kit(s) => <SemioKitSnapshot as pack::snapshot::ArtifactDsl>::print_dsl(s),
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
        "brep" => SemioSubsetSnapshot::Brep(<SemioBrepSnapshot as pack::snapshot::ArtifactDsl>::parse_dsl(inner_body).map_err(|e| e.to_string())?),
        "mesh" => SemioSubsetSnapshot::Mesh(<SemioMeshSnapshot as pack::snapshot::ArtifactDsl>::parse_dsl(inner_body).map_err(|e| e.to_string())?),
        "model" => SemioSubsetSnapshot::Model(<SemioModelSnapshot as pack::snapshot::ArtifactDsl>::parse_dsl(inner_body).map_err(|e| e.to_string())?),
        "value" => SemioSubsetSnapshot::Value(<SemioValueSnapshot as pack::snapshot::ArtifactDsl>::parse_dsl(inner_body).map_err(|e| e.to_string())?),
        "document" => SemioSubsetSnapshot::Document(<SemioDocumentSnapshot as pack::snapshot::ArtifactDsl>::parse_dsl(inner_body).map_err(|e| e.to_string())?),
        "cad" => SemioSubsetSnapshot::Cad(<SemioCadSnapshot as pack::snapshot::ArtifactDsl>::parse_dsl(inner_body).map_err(|e| e.to_string())?),
        "drawing" => SemioSubsetSnapshot::Drawing(<SemioDrawingSnapshot as pack::snapshot::ArtifactDsl>::parse_dsl(inner_body).map_err(|e| e.to_string())?),
        "image" => SemioSubsetSnapshot::Image(<SemioImageSnapshot as pack::snapshot::ArtifactDsl>::parse_dsl(inner_body).map_err(|e| e.to_string())?),
        "video" => SemioSubsetSnapshot::Video(<SemioVideoSnapshot as pack::snapshot::ArtifactDsl>::parse_dsl(inner_body).map_err(|e| e.to_string())?),
        "audio" => SemioSubsetSnapshot::Audio(<SemioAudioSnapshot as pack::snapshot::ArtifactDsl>::parse_dsl(inner_body).map_err(|e| e.to_string())?),
        "animation" => SemioSubsetSnapshot::Animation(<SemioAnimationSnapshot as pack::snapshot::ArtifactDsl>::parse_dsl(inner_body).map_err(|e| e.to_string())?),
        "presentation" => SemioSubsetSnapshot::Presentation(<SemioPresentationSnapshot as pack::snapshot::ArtifactDsl>::parse_dsl(inner_body).map_err(|e| e.to_string())?),
        "flow" => SemioSubsetSnapshot::Flow(<SemioFlowSnapshot as pack::snapshot::ArtifactDsl>::parse_dsl(inner_body).map_err(|e| e.to_string())?),
        "text" => SemioSubsetSnapshot::Text(<SemioTextSnapshot as pack::snapshot::ArtifactDsl>::parse_dsl(inner_body).map_err(|e| e.to_string())?),
        "table" => SemioSubsetSnapshot::Table(<SemioTableSnapshot as pack::snapshot::ArtifactDsl>::parse_dsl(inner_body).map_err(|e| e.to_string())?),
        "graph" => SemioSubsetSnapshot::Graph(<SemioGraphSnapshot as pack::snapshot::ArtifactDsl>::parse_dsl(inner_body).map_err(|e| e.to_string())?),
        "object" => SemioSubsetSnapshot::Object(<SemioObjectSnapshot as pack::snapshot::ArtifactDsl>::parse_dsl(inner_body).map_err(|e| e.to_string())?),
        "kit" => SemioSubsetSnapshot::Kit(<SemioKitSnapshot as pack::snapshot::ArtifactDsl>::parse_dsl(inner_body).map_err(|e| e.to_string())?),
        other => return Err(format!("semio snapshot: unknown subset tag {other:?}")),
    };
    Ok(SemioSnapshot { schema, subset })
}

/// 🎁 Real delegating text/binary codecs — replaces the old hex-of-`serde_json` scaffold. Every
/// byte of the WRAPPED subset's own payload is produced/consumed by THAT subset's own real,
/// already-conformance-tested `ArtifactDsl`/`ArtifactPack` impl; this envelope only adds the
/// `subset`/`schema` header and the outer `store::semio_format` wrapping every stdio artifact uses.
impl pack::snapshot::ArtifactDsl for SemioSnapshot {
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
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(<Self as pack::snapshot::ArtifactDsl>::envelope_id(), store::semio_format::Component::Dsl, 1).expect("valid envelope_id");
        store::semio_format::wrap_text(&envelope, &body)
    }
}

/// 📥️ Parses the ENVELOPE's own committed `.dsl.semio` text into a real [`SemioSnapshot`] — a thin
/// wrapper over `pack::snapshot::ArtifactDsl::parse_dsl` so external Rust callers that cannot name this
/// crate's private `store` extern-crate item (the `✉️mutate-semio-base` test adapter, whose
/// `identity-round-trip` scenario reads the REAL committed `📚️examples/🌐️envelope` artifact) can still
/// drive the same codec production does. Same shape and same rationale as `🌊️flow`'s own bridge.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn parse_semio_envelope_dsl(text: &str) -> Result<SemioSnapshot, String> {
    <SemioSnapshot as pack::snapshot::ArtifactDsl>::parse_dsl(text).map_err(|error| error.to_string())
}

/// 📤️ The `pack::snapshot::ArtifactDsl::print_dsl` inverse of [`parse_semio_envelope_dsl`] — same rationale.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn print_semio_envelope_dsl(snapshot: &SemioSnapshot) -> String {
    <SemioSnapshot as pack::snapshot::ArtifactDsl>::print_dsl(snapshot)
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
use crate::schema::snapshot::*;
use semio_s_artifact_stdio_semio_animation::schema::snapshot::SemioAnimationSnapshot;
use semio_s_artifact_stdio_semio_audio_model::schema::snapshot::SemioAudioSnapshot;
use semio_s_artifact_stdio_semio_brep::schema::snapshot::SemioBrepSnapshot;
use semio_s_artifact_stdio_semio_cad::schema::snapshot::SemioCadSnapshot;
use semio_s_artifact_stdio_semio_document::schema::snapshot::SemioDocumentSnapshot;
use semio_s_artifact_stdio_semio_drawing::schema::snapshot::SemioDrawingSnapshot;
use semio_s_artifact_stdio_semio_flow::schema::snapshot::SemioFlowSnapshot;
use semio_s_artifact_stdio_semio_graph::schema::snapshot::SemioGraphSnapshot;
use semio_s_artifact_stdio_semio_image::schema::snapshot::SemioImageSnapshot;
use semio_s_artifact_stdio_semio_kit::schema::snapshot::SemioKitSnapshot;
use semio_s_artifact_stdio_semio_mesh::schema::snapshot::SemioMeshSnapshot;
use semio_s_artifact_stdio_semio_model::schema::snapshot::SemioModelSnapshot;
use semio_s_artifact_stdio_semio_object::schema::snapshot::SemioObjectSnapshot;
use semio_s_artifact_stdio_semio_presentation::schema::snapshot::SemioPresentationSnapshot;
use semio_s_artifact_stdio_semio_table_model::schema::snapshot::SemioTableSnapshot;
use semio_s_artifact_stdio_semio_text_model::schema::snapshot::SemioTextSnapshot;
use semio_s_artifact_stdio_semio_value_model::schema::snapshot::SemioValueSnapshot;
use semio_s_artifact_stdio_semio_video_model::schema::snapshot::SemioVideoSnapshot;
use framework_schema::ArtifactSchema;




}

