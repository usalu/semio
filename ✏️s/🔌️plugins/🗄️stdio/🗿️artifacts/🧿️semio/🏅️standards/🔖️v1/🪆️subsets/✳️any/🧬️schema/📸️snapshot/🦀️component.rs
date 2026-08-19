//! 🧬️ SemioSnapshot — the envelope union over all 13 domain subsets — every semio artifact round-trips through this shape.
//! W2b closer: the 13 imports below now resolve to each subset's REAL, W2a/W2b-completed
//! snapshot type (brep/mesh/model/value/cad/drawing landed in W2a; document/image/video/audio/
//! animation/presentation/flow landed in W2b) — this file's own shape (an untagged-by-us
//! `SemioSubsetSnapshot` enum + the thin `SemioSnapshot{schema, subset}` wrapper) needed no
//! structural change from the W1b scaffold to pick that up, since only the referenced types'
//! internals grew, not their names/paths.

use crate::artifacts::semio::standards::v1::subsets::animation::schema::snapshot::SemioAnimationSnapshot;
use crate::artifacts::semio::standards::v1::subsets::audio::schema::snapshot::SemioAudioSnapshot;
use crate::artifacts::semio::standards::v1::subsets::brep::schema::snapshot::SemioBrepSnapshot;
use crate::artifacts::semio::standards::v1::subsets::cad::schema::snapshot::SemioCadSnapshot;
use crate::artifacts::semio::standards::v1::subsets::document::schema::snapshot::SemioDocumentSnapshot;
use crate::artifacts::semio::standards::v1::subsets::drawing::schema::snapshot::SemioDrawingSnapshot;
use crate::artifacts::semio::standards::v1::subsets::flow::schema::snapshot::SemioFlowSnapshot;
use crate::artifacts::semio::standards::v1::subsets::graph::schema::snapshot::SemioGraphSnapshot;
use crate::artifacts::semio::standards::v1::subsets::image::schema::snapshot::SemioImageSnapshot;
use crate::artifacts::semio::standards::v1::subsets::kit::schema::snapshot::SemioKitSnapshot;
use crate::artifacts::semio::standards::v1::subsets::mesh::schema::snapshot::SemioMeshSnapshot;
use crate::artifacts::semio::standards::v1::subsets::model::schema::snapshot::SemioModelSnapshot;
use crate::artifacts::semio::standards::v1::subsets::object::schema::snapshot::SemioObjectSnapshot;
use crate::artifacts::semio::standards::v1::subsets::presentation::schema::snapshot::SemioPresentationSnapshot;
use crate::artifacts::semio::standards::v1::subsets::table::schema::snapshot::SemioTableSnapshot;
use crate::artifacts::semio::standards::v1::subsets::text::schema::snapshot::SemioTextSnapshot;
use crate::artifacts::semio::standards::v1::subsets::value::schema::snapshot::SemioValueSnapshot;
use crate::artifacts::semio::standards::v1::subsets::video::schema::snapshot::SemioVideoSnapshot;

/// 🌐️ The envelope union of all 18 semio subset snapshot types (master plan: "SemioSnapshot =
/// tagged union of the 18" — `text` (UNIFIED-COMPOSABLE-ARTIFACT-SYSTEM W2a) is the 14th arm,
/// `table`/`graph` (W2b) the 15th/16th, `object`/`kit` (W2c, the two COMPOSITE subsets) the
/// 17th/18th. Wrapped by `SemioSnapshot` below (a struct, not the enum
/// directly — keeps `#[derive(ArtifactSchema)]` on a proven struct shape; see the W1b manifest for
/// why).
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(tag = "subset", rename_all = "camelCase")]
pub enum SemioSubsetSnapshot {
    Brep(SemioBrepSnapshot),
    Mesh(SemioMeshSnapshot),
    Model(SemioModelSnapshot),
    Value(SemioValueSnapshot),
    Document(SemioDocumentSnapshot),
    Cad(SemioCadSnapshot),
    Drawing(SemioDrawingSnapshot),
    Image(SemioImageSnapshot),
    Video(SemioVideoSnapshot),
    Audio(SemioAudioSnapshot),
    Animation(SemioAnimationSnapshot),
    Presentation(SemioPresentationSnapshot),
    Flow(SemioFlowSnapshot),
    Text(SemioTextSnapshot),
    Table(SemioTableSnapshot),
    Graph(SemioGraphSnapshot),
    Object(SemioObjectSnapshot),
    Kit(SemioKitSnapshot),
}

impl Default for SemioSubsetSnapshot {
    fn default() -> Self {
        SemioSubsetSnapshot::Brep(SemioBrepSnapshot::default())
    }
}

use schema::ArtifactSchema;
use serde::{Deserialize, Serialize};

//#region 🔖️Ids
pub const STDIO_SEMIO_DOCUMENT_SCHEMA: &str = "stdio.semio";
//#endregion 🔖️Ids

//#region 🔖️Snapshot
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ArtifactSchema)]
#[serde(rename_all = "camelCase")]
#[artifact_schema(id = "s.stdio.semio")]
pub struct SemioSnapshot {
    #[state(artifact)]
    pub schema: String,
    #[state(artifact)]
    pub subset: SemioSubsetSnapshot,
}

impl Default for SemioSnapshot {
    fn default() -> Self {
        Self { schema: STDIO_SEMIO_DOCUMENT_SCHEMA.into(), subset: Default::default() }
    }
}
//#endregion 🔖️Snapshot

//#region 🔖️SubsetDispatch
/// 🏷️ The wire tag naming which of the 13 domain subsets is carried — shared by the text DSL's
/// `subset=<tag>` header line and used to select which subset's own REAL codec to delegate to.
/// `pub(crate)` (not private) since `💡️inferences/🏷️kind/🦀️component.rs`
/// (ticket 26/08/12/INTRODUCE-INFERENCE-SCHEMA-FAMILY-WITH-DEPENDENCY-AWARE-CACHING) is a sibling
/// module, not a descendant, and needs this same dispatch as its own honest derivation.
pub(crate) async fn subset_tag(s: &SemioSubsetSnapshot) -> &'static str {
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

/// 🔢️ The binary sibling of [`subset_tag`] — a real, individually protocol-walkable `u8` ordinal
/// (0-13, enum declaration order), used by the binary pack header instead of a length-prefixed name.
pub(crate) async fn subset_ordinal(s: &SemioSubsetSnapshot) -> u8 {
    match s {
        SemioSubsetSnapshot::Brep(_) => 0,
        SemioSubsetSnapshot::Mesh(_) => 1,
        SemioSubsetSnapshot::Model(_) => 2,
        SemioSubsetSnapshot::Value(_) => 3,
        SemioSubsetSnapshot::Document(_) => 4,
        SemioSubsetSnapshot::Cad(_) => 5,
        SemioSubsetSnapshot::Drawing(_) => 6,
        SemioSubsetSnapshot::Image(_) => 7,
        SemioSubsetSnapshot::Video(_) => 8,
        SemioSubsetSnapshot::Audio(_) => 9,
        SemioSubsetSnapshot::Animation(_) => 10,
        SemioSubsetSnapshot::Presentation(_) => 11,
        SemioSubsetSnapshot::Flow(_) => 12,
        SemioSubsetSnapshot::Text(_) => 13,
        SemioSubsetSnapshot::Table(_) => 14,
        SemioSubsetSnapshot::Graph(_) => 15,
        SemioSubsetSnapshot::Object(_) => 16,
        SemioSubsetSnapshot::Kit(_) => 17,
    }
}
//#endregion 🔖️SubsetDispatch

//#region 🔖️TextPrimitives
/// 🧪️ W-S closer (`any`): real delegating text DSL — NOT a re-derivation of any of the 13
/// wrapped subsets' own hex/bracket/recursive grammars. The body is exactly 2 header lines
/// (`subset=<tag>`, `schema=<hex>`) followed by the WRAPPED subset's own real `print_dsl()`
/// output with ITS OWN preamble line stripped (this envelope already carries its own `semio
/// stdio.semio.dsl v1` preamble via `store::semio_format::wrap_text` below — embedding a second
/// preamble line would double up). `parse_dsl` hands the un-prefixed remainder straight to the
/// matching subset's own real `parse_dsl` — every one of those already tolerates a missing
/// preamble (falls back to treating the whole text as body), the same convention this envelope's
/// own `parse_dsl` itself relies on one level up.
async fn hex_encode(bytes: &[u8]) -> String {
    bytes.iter().map(|b| format!("{b:02x}")).collect()
}
async fn hex_decode(s: &str) -> Result<Vec<u8>, String> {
    if s.len() % 2 != 0 {
        return Err(format!("odd hex length: {s:?}"));
    }
    (0..s.len()).step_by(2).map(|i| u8::from_str_radix(&s[i..i + 2], 16).map_err(|e| e.to_string())).collect()
}

async fn strip_inner_preamble(text: &str) -> &str {
    match store::semio_format::split_text_preamble(text) {
        Ok((_, rest)) => rest,
        Err(_) => text,
    }
}

async fn enc_semio_snapshot_body(snap: &SemioSnapshot) -> String {
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

async fn dec_semio_snapshot_body(body: &str) -> Result<SemioSnapshot, String> {
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
//#endregion 🔖️TextPrimitives

//#region 🔖️BinaryPrimitives
/// 🧪️ Real varint-length-prefixed binary envelope: `format u8` + `tag u8` (real
/// [`subset_ordinal`]) + varint-length-prefixed `schema` UTF-8, then the WRAPPED subset's own
/// full, already-real `ArtifactPack::encode_pack()` bytes as one opaque trailing payload — that
/// call already applies THAT subset's own `semio_format` envelope internally, so this is a real,
/// honest double-envelope (delegation, not a re-derivation of any subset's own binary layout).
async fn write_bytes_lp(out: &mut Vec<u8>, bytes: &[u8]) {
    store::pack_rt::write_varint_u64(out, bytes.len() as u64);
    out.extend_from_slice(bytes);
}
async fn read_bytes_lp(reader: &mut store::ByteReader<'_>) -> Result<Vec<u8>, String> {
    let len = reader.read_varint_u64().map_err(|e| e.to_string())? as usize;
    Ok(reader.read_bytes(len).map_err(|e| e.to_string())?.to_vec())
}

const PACK_BINARY_FORMAT: u8 = 1;

async fn encode_semio_snapshot_binary(snap: &SemioSnapshot) -> Vec<u8> {
    let mut out = Vec::new();
    out.push(PACK_BINARY_FORMAT);
    out.push(subset_ordinal(&snap.subset));
    write_bytes_lp(&mut out, snap.schema.as_bytes());
    let payload = match &snap.subset {
        SemioSubsetSnapshot::Brep(s) => <SemioBrepSnapshot as store::ArtifactPack>::encode_pack(s),
        SemioSubsetSnapshot::Mesh(s) => <SemioMeshSnapshot as store::ArtifactPack>::encode_pack(s),
        SemioSubsetSnapshot::Model(s) => <SemioModelSnapshot as store::ArtifactPack>::encode_pack(s),
        SemioSubsetSnapshot::Value(s) => <SemioValueSnapshot as store::ArtifactPack>::encode_pack(s),
        SemioSubsetSnapshot::Document(s) => <SemioDocumentSnapshot as store::ArtifactPack>::encode_pack(s),
        SemioSubsetSnapshot::Cad(s) => <SemioCadSnapshot as store::ArtifactPack>::encode_pack(s),
        SemioSubsetSnapshot::Drawing(s) => <SemioDrawingSnapshot as store::ArtifactPack>::encode_pack(s),
        SemioSubsetSnapshot::Image(s) => <SemioImageSnapshot as store::ArtifactPack>::encode_pack(s),
        SemioSubsetSnapshot::Video(s) => <SemioVideoSnapshot as store::ArtifactPack>::encode_pack(s),
        SemioSubsetSnapshot::Audio(s) => <SemioAudioSnapshot as store::ArtifactPack>::encode_pack(s),
        SemioSubsetSnapshot::Animation(s) => <SemioAnimationSnapshot as store::ArtifactPack>::encode_pack(s),
        SemioSubsetSnapshot::Presentation(s) => <SemioPresentationSnapshot as store::ArtifactPack>::encode_pack(s),
        SemioSubsetSnapshot::Flow(s) => <SemioFlowSnapshot as store::ArtifactPack>::encode_pack(s),
        SemioSubsetSnapshot::Text(s) => <SemioTextSnapshot as store::ArtifactPack>::encode_pack(s),
        SemioSubsetSnapshot::Table(s) => <SemioTableSnapshot as store::ArtifactPack>::encode_pack(s),
        SemioSubsetSnapshot::Graph(s) => <SemioGraphSnapshot as store::ArtifactPack>::encode_pack(s),
        SemioSubsetSnapshot::Object(s) => <SemioObjectSnapshot as store::ArtifactPack>::encode_pack(s),
        SemioSubsetSnapshot::Kit(s) => <SemioKitSnapshot as store::ArtifactPack>::encode_pack(s),
    };
    out.extend_from_slice(&payload);
    out
}

async fn decode_semio_snapshot_binary(bytes: &[u8]) -> Result<SemioSnapshot, String> {
    let mut reader = store::ByteReader::new(bytes);
    let format = reader.read_u8().map_err(|e| e.to_string())?;
    if format != PACK_BINARY_FORMAT {
        return Err(format!("unsupported pack format {format}"));
    }
    let tag = reader.read_u8().map_err(|e| e.to_string())?;
    let schema = String::from_utf8(read_bytes_lp(&mut reader)?).map_err(|e| e.to_string())?;
    let payload = reader.read_bytes(reader.remaining()).map_err(|e| e.to_string())?;
    let subset = match tag {
        0 => SemioSubsetSnapshot::Brep(<SemioBrepSnapshot as store::ArtifactPack>::decode_pack(payload).map_err(|e| e.to_string())?),
        1 => SemioSubsetSnapshot::Mesh(<SemioMeshSnapshot as store::ArtifactPack>::decode_pack(payload).map_err(|e| e.to_string())?),
        2 => SemioSubsetSnapshot::Model(<SemioModelSnapshot as store::ArtifactPack>::decode_pack(payload).map_err(|e| e.to_string())?),
        3 => SemioSubsetSnapshot::Value(<SemioValueSnapshot as store::ArtifactPack>::decode_pack(payload).map_err(|e| e.to_string())?),
        4 => SemioSubsetSnapshot::Document(<SemioDocumentSnapshot as store::ArtifactPack>::decode_pack(payload).map_err(|e| e.to_string())?),
        5 => SemioSubsetSnapshot::Cad(<SemioCadSnapshot as store::ArtifactPack>::decode_pack(payload).map_err(|e| e.to_string())?),
        6 => SemioSubsetSnapshot::Drawing(<SemioDrawingSnapshot as store::ArtifactPack>::decode_pack(payload).map_err(|e| e.to_string())?),
        7 => SemioSubsetSnapshot::Image(<SemioImageSnapshot as store::ArtifactPack>::decode_pack(payload).map_err(|e| e.to_string())?),
        8 => SemioSubsetSnapshot::Video(<SemioVideoSnapshot as store::ArtifactPack>::decode_pack(payload).map_err(|e| e.to_string())?),
        9 => SemioSubsetSnapshot::Audio(<SemioAudioSnapshot as store::ArtifactPack>::decode_pack(payload).map_err(|e| e.to_string())?),
        10 => SemioSubsetSnapshot::Animation(<SemioAnimationSnapshot as store::ArtifactPack>::decode_pack(payload).map_err(|e| e.to_string())?),
        11 => SemioSubsetSnapshot::Presentation(<SemioPresentationSnapshot as store::ArtifactPack>::decode_pack(payload).map_err(|e| e.to_string())?),
        12 => SemioSubsetSnapshot::Flow(<SemioFlowSnapshot as store::ArtifactPack>::decode_pack(payload).map_err(|e| e.to_string())?),
        13 => SemioSubsetSnapshot::Text(<SemioTextSnapshot as store::ArtifactPack>::decode_pack(payload).map_err(|e| e.to_string())?),
        14 => SemioSubsetSnapshot::Table(<SemioTableSnapshot as store::ArtifactPack>::decode_pack(payload).map_err(|e| e.to_string())?),
        15 => SemioSubsetSnapshot::Graph(<SemioGraphSnapshot as store::ArtifactPack>::decode_pack(payload).map_err(|e| e.to_string())?),
        16 => SemioSubsetSnapshot::Object(<SemioObjectSnapshot as store::ArtifactPack>::decode_pack(payload).map_err(|e| e.to_string())?),
        17 => SemioSubsetSnapshot::Kit(<SemioKitSnapshot as store::ArtifactPack>::decode_pack(payload).map_err(|e| e.to_string())?),
        other => return Err(format!("semio snapshot: unknown subset tag {other}")),
    };
    Ok(SemioSnapshot { schema, subset })
}
//#endregion 🔖️BinaryPrimitives

//#region 🔖️HandcraftedArtifactCodecs
/// 🎁 Real delegating text/binary codecs — replaces the old hex-of-`serde_json` scaffold. Every
/// byte of the WRAPPED subset's own payload is produced/consumed by THAT subset's own real,
/// already-conformance-tested `ArtifactDsl`/`ArtifactPack` impl; this envelope only adds the
/// `subset`/`schema` header and the outer `store::semio_format` wrapping every stdio artifact uses.
impl store::ArtifactDsl for SemioSnapshot {
    const EXTENSION: &'static str = "semio";
    async fn envelope_id() -> &'static str {
        STDIO_SEMIO_DOCUMENT_SCHEMA
    }

    async fn parse_dsl(text: &str) -> Result<Self, store::TextError> {
        let body = match store::semio_format::split_text_preamble(text) {
            Ok((_, rest)) => rest,
            Err(_) => text,
        };
        dec_semio_snapshot_body(body).map_err(|e| store::TextError::new(e, dsl::TextSpan::at(1, 1)))
    }

    async fn print_dsl(&self) -> String {
        let body = enc_semio_snapshot_body(self);
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(<Self as store::ArtifactDsl>::envelope_id(), store::semio_format::Component::Dsl, 1).expect("valid envelope_id");
        store::semio_format::wrap_text(&envelope, &body)
    }
}

impl store::ArtifactPack for SemioSnapshot {
    async fn encode_pack_with(&self, options: &store::PackEncodeOptions) -> Result<Vec<u8>, store::PackError> {
        let _ = options;
        let raw = encode_semio_snapshot_binary(self);
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(<Self as store::ArtifactDsl>::envelope_id(), store::semio_format::Component::Pack, 1).map_err(|e| store::PackError::Schema(e.to_string()))?;
        Ok(store::semio_format::wrap_binary(&envelope, &raw))
    }

    async fn decode_pack_with(bytes: &[u8], options: &store::PackDecodeOptions) -> Result<Self, store::PackError> {
        let (envelope, inner) = store::semio_format::unwrap_binary(bytes).map_err(|e| store::PackError::Schema(e.to_string()))?;
        if envelope.envelope_id() != <Self as store::ArtifactDsl>::envelope_id() {
            return Err(store::PackError::Schema(format!("pack envelope mismatch: expected {}, got {}", <Self as store::ArtifactDsl>::envelope_id(), envelope.envelope_id())));
        }
        let _ = options;
        decode_semio_snapshot_binary(&inner).map_err(store::PackError::Schema)
    }
}
//#endregion 🔖️HandcraftedArtifactCodecs

//#region 🔖️Demo
/// 🌱 The demo `s.stdio.semio` document — wraps `flow`'s own real demo snapshot (2 nodes, 1
/// edge, incl. a negative coordinate) so this facet's fixtures/conformance tests exercise a real,
/// already-nontrivial nested payload rather than an all-default stub. Single source of truth for
/// `📚️examples/🌐️envelope/🖼️assets/🗣️example.dsl.semio`/`🎒️example.pack.semio` and this facet's
/// own conformance-law tests.
#[cfg(test)]
pub(crate) async fn demo_semio_snapshot() -> SemioSnapshot {
    use crate::artifacts::semio::standards::v1::subsets::flow::schema::snapshot::demo_flow_snapshot;
    SemioSnapshot { schema: STDIO_SEMIO_DOCUMENT_SCHEMA.into(), subset: SemioSubsetSnapshot::Flow(demo_flow_snapshot()) }
}
//#endregion 🔖️Demo

//#region 🔖️Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[semio_framework_async_macros::async_test]
    async fn pack_round_trips_default_subset() {
        let snap = SemioSnapshot::default();
        let bytes = <SemioSnapshot as store::ArtifactPack>::encode_pack(&snap);
        let back = <SemioSnapshot as store::ArtifactPack>::decode_pack(&bytes).expect("decode");
        assert_eq!(snap, back);
    }

    #[semio_framework_async_macros::async_test]
    async fn dsl_text_round_trips_default_subset() {
        let snap = SemioSnapshot::default();
        let text = <SemioSnapshot as store::ArtifactDsl>::print_dsl(&snap);
        let back = <SemioSnapshot as store::ArtifactDsl>::parse_dsl(&text).expect("parse");
        assert_eq!(snap, back);
    }

    /// 🧪️ Real delegation, non-default nested payload: the demo (flow-wrapped) snapshot's
    /// text/binary round trips must both hold, proving this isn't merely round-tripping an
    /// all-zero stub.
    #[semio_framework_async_macros::async_test]
    async fn pack_and_dsl_round_trip_the_demo_snapshot() {
        let snap = demo_semio_snapshot();
        let bytes = <SemioSnapshot as store::ArtifactPack>::encode_pack(&snap);
        assert_eq!(<SemioSnapshot as store::ArtifactPack>::decode_pack(&bytes).expect("decode"), snap);
        let text = <SemioSnapshot as store::ArtifactDsl>::print_dsl(&snap);
        assert_eq!(<SemioSnapshot as store::ArtifactDsl>::parse_dsl(&text).expect("parse"), snap);
    }

    /// 🧪️ Every one of the 18 subset tags real-round-trips through both facets — proves the
    /// dispatch tables (text AND binary) are wired correctly for every wrapped subset, not just
    /// the one exercised by [`demo_semio_snapshot`].
    #[semio_framework_async_macros::async_test]
    async fn all_eighteen_subset_tags_round_trip_text_and_binary() {
        let subsets: Vec<SemioSubsetSnapshot> = vec![
            SemioSubsetSnapshot::Brep(Default::default()),
            SemioSubsetSnapshot::Mesh(Default::default()),
            SemioSubsetSnapshot::Model(Default::default()),
            SemioSubsetSnapshot::Value(Default::default()),
            SemioSubsetSnapshot::Document(Default::default()),
            SemioSubsetSnapshot::Cad(Default::default()),
            SemioSubsetSnapshot::Drawing(Default::default()),
            SemioSubsetSnapshot::Image(Default::default()),
            SemioSubsetSnapshot::Video(Default::default()),
            SemioSubsetSnapshot::Audio(Default::default()),
            SemioSubsetSnapshot::Animation(Default::default()),
            SemioSubsetSnapshot::Presentation(Default::default()),
            SemioSubsetSnapshot::Flow(Default::default()),
            SemioSubsetSnapshot::Text(Default::default()),
            SemioSubsetSnapshot::Table(Default::default()),
            SemioSubsetSnapshot::Graph(Default::default()),
            SemioSubsetSnapshot::Object(Default::default()),
            SemioSubsetSnapshot::Kit(Default::default()),
        ];
        for subset in subsets {
            let snap = SemioSnapshot { schema: STDIO_SEMIO_DOCUMENT_SCHEMA.into(), subset };
            let text = <SemioSnapshot as store::ArtifactDsl>::print_dsl(&snap);
            assert_eq!(<SemioSnapshot as store::ArtifactDsl>::parse_dsl(&text).unwrap_or_else(|e| panic!("parse_dsl failed for {:?}: {e}", subset_tag(&snap.subset))), snap);
            let bytes = <SemioSnapshot as store::ArtifactPack>::encode_pack(&snap);
            assert_eq!(<SemioSnapshot as store::ArtifactPack>::decode_pack(&bytes).unwrap_or_else(|e| panic!("decode_pack failed for {:?}: {e}", subset_tag(&snap.subset))), snap);
        }
    }
}
//#endregion 🔖️Tests
