//! 🧬️ PlySnapshot schema — complete per-FORMAT-SPEC model of PLY's generic element/property
//! system (Ticket 26/08/10/ARTIFACT-SYSTEM-OVERHAUL…: replaces the old hardcoded
//! `vertices: Vec<MeshVertex>` / `faces: Vec<MeshTriangle>` mesh-only model — those types were
//! shared VERBATIM with stl, exactly the copy-pasted-shared-type anti-pattern the recipe bans).
//! PLY's real structure is: a wire `format`, an ordered list of `comments`, and a name-keyed
//! list of `elements`, each with its own typed `properties` (scalar or list) and typed `rows`
//! of `PlyValue` cells. `vertices`/`faces`-shaped meshes are just the common case that falls out
//! of elements literally named `"vertex"`/`"face"` — nothing about the model hardcodes them.

use crate::STDIO_PLY_DOCUMENT_SCHEMA;
use framework_schema::ArtifactSchema;

//#region 🔖️Format
/// 📦 The three `format` lines a PLY header may declare.
#[derive(Clone, Copy, Debug, PartialEq, Eq, value_derive::ToValue, value_derive::FromValue, Default, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[value(rename_all = "camelCase")]
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
#[derive(Clone, Copy, Debug, PartialEq, Eq, value_derive::ToValue, value_derive::FromValue, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[value(rename_all = "camelCase")]
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
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[value(tag = "form", rename_all = "camelCase", rename_all_fields = "camelCase")]
pub enum PlyProperty {
    Scalar { name: String, kind: PlyScalarType },
    List { name: String, count_kind: PlyScalarType, value_kind: PlyScalarType },
}

impl PlyProperty {
    /// 🏷️ The property's declared name, regardless of shape.
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn name(&self) -> &str {
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
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[value(tag = "kind", content = "value", rename_all = "camelCase")]
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
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, Default, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[value(rename_all = "camelCase")]
pub struct PlyRow {
    pub values: Vec<PlyValue>,
}
//#endregion 🔖️Row

//#region 🔖️Element
/// 🧱 One element declaration owns its unsigned count independently from retained occurrence rows.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, Default, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[value(rename_all = "camelCase")]
pub struct PlyElement {
    pub name: String,
    pub count: u64,
    pub properties: Vec<PlyProperty>,
    pub rows: Vec<PlyRow>,
}
//#endregion 🔖️Element

//#region 🔖️Snapshot
/// 📸️ Persisted `stdio.ply` snapshot — complete per the PLY spec: wire `format`, in-order
/// `comments` (position matters, see `POLICY_GRAMMAR_HONESTY`'s retention rule), and the
/// name-keyed `elements` list (each a strong-like entity with its own per-field diff).
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, ArtifactSchema, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone)]
#[value(rename_all = "camelCase")]
#[artifact_schema(id = "s.stdio.ply")]
pub struct PlySnapshot {
    #[state(artifact)]
    pub schema: String,
    #[state(artifact)]
    #[value(default)]
    pub format: PlyFormat,
    #[state(artifact)]
    #[value(default)]
    pub comments: Vec<String>,
    #[state(artifact)]
    #[value(default)]
    pub elements: Vec<PlyElement>,
}

impl Default for PlySnapshot {
    fn default() -> Self {
        Self { schema: STDIO_PLY_DOCUMENT_SCHEMA.into(), format: PlyFormat::default(), comments: Vec::new(), elements: Vec::new() }
    }
}
//#endregion 🔖️Snapshot

//#region 🔖️HandcraftedArtifactCodecs

//#endregion 🔖️HandcraftedArtifactCodecs


