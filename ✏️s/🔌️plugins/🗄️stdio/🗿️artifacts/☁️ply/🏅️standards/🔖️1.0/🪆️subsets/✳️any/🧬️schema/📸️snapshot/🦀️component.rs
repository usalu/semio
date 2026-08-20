//! 🧬️ PlySnapshot schema — complete per-FORMAT-SPEC model of PLY's generic element/property
//! system (Ticket 26/08/10/ARTIFACT-SYSTEM-OVERHAUL…: replaces the old hardcoded
//! `vertices: Vec<MeshVertex>` / `faces: Vec<MeshTriangle>` mesh-only model — those types were
//! shared VERBATIM with stl, exactly the copy-pasted-shared-type anti-pattern the recipe bans).
//! PLY's real structure is: a wire `format`, an ordered list of `comments`, and a name-keyed
//! list of `elements`, each with its own typed `properties` (scalar or list) and typed `rows`
//! of `PlyValue` cells. `vertices`/`faces`-shaped meshes are just the common case that falls out
//! of elements literally named `"vertex"`/`"face"` — nothing about the model hardcodes them.

use crate::artifacts::ply::STDIO_PLY_DOCUMENT_SCHEMA;
use schema::ArtifactSchema;
use serde::{Deserialize, Serialize};

//#region 🔖️Format
/// 📦 The three `format` lines a PLY header may declare.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub enum PlyFormat {
    #[default]
    Ascii,
    BinaryLittleEndian,
    BinaryBigEndian,
}
//#endregion 🔖️Format

//#region 🔖️ScalarType
/// 🔢 The eight PLY scalar property types (long spelling is canonical on output; both long and
/// short — `int8`, `uint32`, … — spellings are accepted on input, see the engine's parser).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum PlyScalarType {
    Char,
    UChar,
    Short,
    UShort,
    Int,
    UInt,
    Float,
    Double,
}
//#endregion 🔖️ScalarType

//#region 🔖️Property
/// 🧩 One `property` declaration inside an `element` block: a plain scalar column, or a
/// variable-length list column (e.g. `property list uchar int vertex_indices` for face indices).
/// `form` (the serde tag) distinguishes the two shapes; it is a separate key from `kind`
/// (the scalar type of a `Scalar` property) to avoid a tag/field name collision.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "form", rename_all = "camelCase")]
pub enum PlyProperty {
    Scalar { name: String, kind: PlyScalarType },
    List { name: String, count_kind: PlyScalarType, value_kind: PlyScalarType },
}

impl PlyProperty {
    /// 🏷️ The property's declared name, regardless of shape.
    pub async fn name(&self) -> &str {
        match self {
            PlyProperty::Scalar { name, .. } => name,
            PlyProperty::List { name, .. } => name,
        }
    }
}
//#endregion 🔖️Property

//#region 🔖️Value
/// 🔣 One typed cell value. `List` holds a variable-length run of same-`value_kind` scalars
/// (e.g. a face's vertex-index list) — adjacently tagged (`kind`/`value`) rather than internally
/// tagged so newtype variants (all of these) serialize cleanly.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "kind", content = "value", rename_all = "camelCase")]
pub enum PlyValue {
    Char(i8),
    UChar(u8),
    Short(i16),
    UShort(u16),
    Int(i32),
    UInt(u32),
    Float(f32),
    Double(f64),
    List(Vec<PlyValue>),
}
//#endregion 🔖️Value

//#region 🔖️Row
/// 📏 One element instance's data — one [`PlyValue`] per declared property, in the same order
/// as the owning [`PlyElement::properties`].
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct PlyRow {
    pub values: Vec<PlyValue>,
}
//#endregion 🔖️Row

//#region 🔖️Element
/// 🧱 One `element <name> <count>` block: its ordered property declarations plus every decoded
/// row. `count` mirrors `rows.len()` for a well-formed document (codecs keep it in sync — see
/// `apply_element_diff`); it is not independently diffable, matching the recipe's rule that
/// collection sizes are never their own diff field (c.f. zip's `entries` — no `entryCount`).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct PlyElement {
    pub name: String,
    pub count: usize,
    pub properties: Vec<PlyProperty>,
    pub rows: Vec<PlyRow>,
}
//#endregion 🔖️Element

//#region 🔖️Snapshot
/// 📸️ Persisted `stdio.ply` snapshot — complete per the PLY spec: wire `format`, in-order
/// `comments` (position matters, see `POLICY_GRAMMAR_HONESTY`'s retention rule), and the
/// name-keyed `elements` list (each a strong-like entity with its own per-field diff).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ArtifactSchema)]
#[serde(rename_all = "camelCase")]
#[artifact_schema(id = "s.stdio.ply")]
pub struct PlySnapshot {
    #[state(artifact)]
    pub schema: String,
    #[state(artifact)]
    #[serde(default)]
    pub format: PlyFormat,
    #[state(artifact)]
    #[serde(default)]
    pub comments: Vec<String>,
    #[state(artifact)]
    #[serde(default)]
    pub elements: Vec<PlyElement>,
}

impl Default for PlySnapshot {
    fn default() -> Self {
        Self { schema: STDIO_PLY_DOCUMENT_SCHEMA.into(), format: PlyFormat::default(), comments: Vec::new(), elements: Vec::new() }
    }
}
//#endregion 🔖️Snapshot

//#region 🔖️HandcraftedArtifactCodecs
impl store::ArtifactDsl for PlySnapshot {
    const EXTENSION: &'static str = "ply";
    async fn envelope_id() -> &'static str {
        "stdio.ply"
    }

    async fn parse_dsl(text: &str) -> Result<Self, store::TextError> {
        let body = match store::semio_format::split_text_preamble(text) {
            Ok((_, rest)) => rest,
            Err(_) => text,
        };
        crate::artifacts::ply::engine::decode_ply(body.as_bytes()).await.map_err(|e| store::TextError::new(format!("ply parse: {e}"), dsl::TextSpan::at(1, 1)))
    }
    async fn print_dsl(&self) -> String {
        let bytes = crate::artifacts::ply::engine::encode_ply(self).await.unwrap_or_default();
        let body = String::from_utf8(bytes).unwrap_or_default();
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(<Self as store::ArtifactDsl>::envelope_id().await, store::semio_format::Component::Dsl, 1).expect("valid envelope_id");
        store::semio_format::wrap_text(&envelope, &body)
    }
}

impl store::ArtifactPack for PlySnapshot {
    async fn encode_pack_with(&self, options: &store::PackEncodeOptions) -> Result<Vec<u8>, store::PackError> {
        let _ = options;
        // 🐛️ P2-FG3 bugfix: the Pack facet is the artifact's REAL on-disk byte-exact
        // representation and must respect the snapshot's own persisted `format` (ascii /
        // binary_little_endian / binary_big_endian) — unlike `print_dsl` below, which
        // deliberately NORMALIZES to ascii so the DSL/text facet stays legible UTF-8 regardless
        // of `format` (see this artifact's own P2-FG3 report). Previously this called the
        // ascii-forcing `encode_ply(self)` unconditionally, silently discarding `self.format` on
        // every Pack round-trip for a binary-format snapshot (`decode_pack(encode_pack(snap))`
        // would come back with `format: Ascii` regardless of what was persisted) — a real,
        // pre-existing correctness bug, fixed here.
        let raw = crate::artifacts::ply::engine::encode_ply_with_format(self, self.format).await.map_err(|e| store::PackError::Schema(e))?;
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(<Self as store::ArtifactDsl>::envelope_id().await, store::semio_format::Component::Pack, 1).map_err(|e| store::PackError::Schema(e.to_string()))?;
        Ok(store::semio_format::wrap_binary(&envelope, &raw))
    }
    async fn decode_pack_with(bytes: &[u8], options: &store::PackDecodeOptions) -> Result<Self, store::PackError> {
        let (envelope, inner) = store::semio_format::unwrap_binary(bytes).map_err(|e| store::PackError::Schema(e.to_string()))?;
        if envelope.envelope_id() != <Self as store::ArtifactDsl>::envelope_id() {
            return Err(store::PackError::Schema(format!("pack envelope mismatch: expected {}, got {}", <Self as store::ArtifactDsl>::envelope_id(), envelope.envelope_id())));
        }
        let _ = options;
        crate::artifacts::ply::engine::decode_ply(&inner).await.map_err(|e| store::PackError::Schema(e))
    }
}
//#endregion 🔖️HandcraftedArtifactCodecs
