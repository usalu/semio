//! 🧬️ ObjSnapshot schema — persistent fields; real byte codec lives in `⚙️engine`. Complete
//! per FORMAT SPEC (ticket 26/08/10/ARTIFACT-SYSTEM-OVERHAUL-REAL-CODECS-RUNTIME-REUSE-EVOLUTION,
//! `🧬️schema-design.md` obj row): `v`/`vt`/`vn` index-keyed geometry rows (incl. the optional
//! homogeneous/3rd-component `w`), `f` index-keyed n-gon faces, name-keyed `g`/`o` group and
//! object face-membership, range-tagged `usemtl`/`s` (smoothing), `mtllib`, and position-retained
//! `unknown_statements` for every real source line the codec doesn't otherwise model (including
//! comments — nothing on disk is silently dropped).
//!
//! 🧪️ F6: `#[derive(dsl::DslRecord)]` on every struct here (mutation-side classification, ticket
//! `f6-recon-report.md` §3's decision rule) — none of these types carry a data-carrying enum or a
//! tri-state `Option<Option<T>>` field (that restriction lives only in `🔺️diff`'s sparse-patch
//! types, hand-rolled separately), so the whole snapshot tree binds `DslField` cleanly and
//! `#[derive(dsl::DslOps)]` on `ObjMutation` (see `🧬️mutations`) works without a `FlowMutationDsl`
//! -style mirror enum.

use crate::artifacts::obj::STDIO_OBJ_DOCUMENT_SCHEMA;
use schema::ArtifactSchema;
use serde::{Deserialize, Serialize};

//#region 🔖️MeshModel
/// 📍 A `v` position line: `x y z [w]` (spec default `w = 1.0` when omitted — `None` here
/// means the source omitted it; the value itself is never fabricated).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
pub struct ObjVertex {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub w: Option<f64>,
}

/// 🧵 A `vt` texture-coordinate line: `u [v] [w]` (`v` defaults to 0 when omitted per spec but
/// is stored concretely since every real codec path fills it; the rarely-used 3rd component
/// `w` is genuinely optional and tri-stated at the diff level).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
pub struct ObjTexCoord {
    pub u: f64,
    pub v: f64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub w: Option<f64>,
}

/// 📐 A `vn` normal line: always 3 components.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
pub struct ObjNormal {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

/// 🔗 One `v[/vt][/vn]` reference inside an `f` line (0-based, negative indices already
/// resolved at parse time per the OBJ spec's own relative-index rule).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
pub struct ObjFaceVertex {
    pub vertex: u32,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub texcoord: Option<u32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub normal: Option<u32>,
}

/// 🧩 A `f` line, kept as its original n-gon (never eagerly triangulated). Pure geometry —
/// `o`/`g`/`usemtl`/`s` state is tracked separately as face-index membership/ranges on
/// [`ObjSnapshot`] (not duplicated per-face), matching the recipe's index-keyed-collection shape.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
pub struct ObjFace {
    pub vertices: Vec<ObjFaceVertex>,
}

/// 🏷️ A named `g` group — `faces` is the (possibly non-contiguous) set of face indices active
/// while this name was one of the currently-active group names (OBJ's `g a b c` puts every
/// subsequent face into ALL of `a`, `b`, AND `c` simultaneously, so membership is a list, not a
/// single range — a face-index LIST is this artifact's chosen shape for name-keyed membership,
/// documented per the recipe's "your call" latitude).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
pub struct ObjGroup {
    pub name: String,
    #[serde(default)]
    pub faces: Vec<usize>,
}

/// 🏷️ A named `o` object — exactly one object is ever active at a time (unlike groups), so
/// membership sets across different `ObjObject`s never overlap.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
pub struct ObjObject {
    pub name: String,
    #[serde(default)]
    pub faces: Vec<usize>,
}

/// 🎨 One `usemtl` transition: `material` is active for every face from `face_index_from`
/// (inclusive) up to the next range's `face_index_from` (or the end of `faces`). Range-tagged
/// rather than per-face, matching real OBJ's own sequential/single-active semantics.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
pub struct ObjUsemtlRange {
    pub face_index_from: usize,
    pub material: String,
}

/// 🧵 One `s` transition: `group` is the active smoothing group from `face_index_from`
/// onward; `None` represents `s off`.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
pub struct ObjSmoothingRange {
    pub face_index_from: usize,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub group: Option<u32>,
}

/// 🕳️ A real source line the codec doesn't otherwise model — comments (`#`) and any keyword
/// outside `v`/`vt`/`vn`/`f`/`o`/`g`/`usemtl`/`mtllib`/`s` — retained verbatim so nothing on
/// disk is silently dropped. `line_index` is the 0-based line number at the time of the decode
/// that produced this snapshot (informational; re-encoding renumbers on the next decode as part
/// of this codec's documented normal form — see `⚙️engine` module docs).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
pub struct ObjUnknownStatement {
    pub line_index: usize,
    pub raw: String,
}
//#endregion 🔖️MeshModel

//#region 🔖️Snapshot
/// 📸️ Persisted `stdio.obj` snapshot — complete per the Wavefront OBJ 3.0 spec's real,
/// commonly-implemented grammar.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ArtifactSchema, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
#[artifact_schema(id = "s.stdio.obj")]
pub struct ObjSnapshot {
    #[state(artifact)]
    pub schema: String,
    #[state(artifact)]
    #[serde(default)]
    pub vertices: Vec<ObjVertex>,
    #[state(artifact)]
    #[serde(default)]
    pub texcoords: Vec<ObjTexCoord>,
    #[state(artifact)]
    #[serde(default)]
    pub normals: Vec<ObjNormal>,
    #[state(artifact)]
    #[serde(default)]
    pub faces: Vec<ObjFace>,
    #[state(artifact)]
    #[serde(default)]
    pub groups: Vec<ObjGroup>,
    #[state(artifact)]
    #[serde(default)]
    pub objects: Vec<ObjObject>,
    #[state(artifact)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mtllib: Option<String>,
    #[state(artifact)]
    #[serde(default)]
    pub usemtl: Vec<ObjUsemtlRange>,
    #[state(artifact)]
    #[serde(default)]
    pub smoothing_groups: Vec<ObjSmoothingRange>,
    #[state(artifact)]
    #[serde(default)]
    pub unknown_statements: Vec<ObjUnknownStatement>,
}

impl Default for ObjSnapshot {
    fn default() -> Self {
        Self {
            schema: STDIO_OBJ_DOCUMENT_SCHEMA.into(),
            vertices: Vec::new(),
            texcoords: Vec::new(),
            normals: Vec::new(),
            faces: Vec::new(),
            groups: Vec::new(),
            objects: Vec::new(),
            mtllib: None,
            usemtl: Vec::new(),
            smoothing_groups: Vec::new(),
            unknown_statements: Vec::new(),
        }
    }
}
//#endregion 🔖️Snapshot

//#region 🔖️HandcraftedArtifactCodecs
// 🔗 Real grammar lives in `⚙️engine::encode_obj`/`decode_obj` — see
// https://www.fileformat.info/format/wavefrontobj/egff.htm for the grammar this mirrors.
impl store::ArtifactDsl for ObjSnapshot {
    const EXTENSION: &'static str = "obj";
    async fn envelope_id() -> &'static str {
        "stdio.obj"
    }

    async fn parse_dsl(text: &str) -> Result<Self, store::TextError> {
        let body = match store::semio_format::split_text_preamble(text) {
            Ok((_, rest)) => rest,
            Err(_) => text,
        };
        crate::artifacts::obj::engine::decode_obj(body).await.map_err(|e| store::TextError::new(format!("obj parse: {e}"), dsl::TextSpan::at(1, 1)))
    }
    async fn print_dsl(&self) -> String {
        let body = crate::artifacts::obj::engine::encode_obj(self);
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(<Self as store::ArtifactDsl>::envelope_id().await, store::semio_format::Component::Dsl, 1).expect("valid envelope_id");
        store::semio_format::wrap_text(&envelope, &body)
    }
}

impl store::ArtifactPack for ObjSnapshot {
    async fn encode_pack_with(&self, options: &store::PackEncodeOptions) -> Result<Vec<u8>, store::PackError> {
        let _ = options;
        let raw = crate::artifacts::obj::engine::encode_obj(self).await.into_bytes();
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(<Self as store::ArtifactDsl>::envelope_id().await, store::semio_format::Component::Pack, 1).map_err(|e| store::PackError::Schema(e.to_string()))?;
        Ok(store::semio_format::wrap_binary(&envelope, &raw))
    }
    async fn decode_pack_with(bytes: &[u8], options: &store::PackDecodeOptions) -> Result<Self, store::PackError> {
        let (envelope, inner) = store::semio_format::unwrap_binary(bytes).map_err(|e| store::PackError::Schema(e.to_string()))?;
        if envelope.envelope_id() != <Self as store::ArtifactDsl>::envelope_id() {
            return Err(store::PackError::Schema(format!("pack envelope mismatch: expected {}, got {}", <Self as store::ArtifactDsl>::envelope_id(), envelope.envelope_id())));
        }
        let _ = options;
        let text = String::from_utf8(inner).map_err(|e| store::PackError::Schema(e.to_string()))?;
        crate::artifacts::obj::engine::decode_obj(&text).await.map_err(store::PackError::Schema)
    }
}
//#endregion 🔖️HandcraftedArtifactCodecs
