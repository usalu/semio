//! 🧬️ SemioKitSnapshot — semio's own type/design domain: TYPES (with representation references)
//! and DESIGNS (pieces and connections). SECOND COMPOSITE subset (UNIFIED-COMPOSABLE-ARTIFACT-
//! SYSTEM, W2c) — carries both CHILD slots (`objects`/`models`/`properties`, owned) AND a LINK
//! slot (`representations`, independent-lifecycle). Absorbs the duplicated `kit.catalog` artifact
//! kind puzzle/three-block apps currently declare separately (that dissolution — repointing those
//! apps' `AppSchema::artifact_kind()` registrations at this subset — is a later wave's concern,
//! same "later wave" scoping `🔤️text`'s report used for its own absorbed `LocalizedText` dissolve).
//!
//! Composes `object`/`model` (owned pieces/example instances) and `value` (shared property set),
//! per `📌️important.md`'s suggested shape. `representations` is the one LINK slot: a TYPE's visual
//! representation is often reused across many kits/catalogs (a shared library item), so it is
//! referenced, never owned — each `ArtifactLink.role` carries the owning `SemioKitType.id` (the
//! join key between the flat link pool and the type that displays it; a type may have zero, one,
//! or many representations sharing its id as `role`).

use crate::standards::v1::subsets::base::schema::geometry::SemioTransform;
use crate::standards::v1::subsets::model::schema::snapshot::SemioModelSnapshot;
use crate::standards::v1::subsets::object::schema::snapshot::SemioObjectSnapshot;
use crate::standards::v1::subsets::value::schema::snapshot::SemioValueSnapshot;
use framework_schema::ArtifactSchema;
#[cfg(test)]
use serde::{Deserialize, Serialize};

//#region 🔖️Ids
pub const STDIO_SEMIOKIT_DOCUMENT_SCHEMA: &str = "stdio.semio.kit";
//#endregion 🔖️Ids

//#region 🔖️Type
/// 🏷️ One TYPE in the kit's catalog — a name/category, its representations living in the sibling
/// `representations` LINK pool (joined by `role == id`, see module doc comment). Id-keyed (no
/// positional meaning — `add-type`/`remove-type`/`rename-type` all address by `id`).
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, Default, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(test, derive(Serialize, Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct SemioKitType {
    pub id: String,
    pub name: String,
    pub category: String,
}
//#endregion 🔖️Type

//#region 🔖️Design
/// 📐️ One PIECE inside a design: an instance of a TYPE (`type_id`, joins `SemioKitType.id`) at a
/// local `transform`.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, Default, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(test, derive(Serialize, Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct SemioKitPiece {
    pub id: String,
    pub type_id: String,
    pub transform: SemioTransform,
}

/// 🔌️ One CONNECTION between two pieces' named ports.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, Default, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(test, derive(Serialize, Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct SemioKitConnection {
    pub id: String,
    pub connecting_piece_id: String,
    pub connecting_port: String,
    pub connected_piece_id: String,
    pub connected_port: String,
}

/// 📋️ One DESIGN — a named arrangement of pieces and their connections. Id-keyed.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, Default, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(test, derive(Serialize, Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct SemioKitDesign {
    pub id: String,
    pub name: String,
    pub pieces: Vec<SemioKitPiece>,
    pub connections: Vec<SemioKitConnection>,
}
//#endregion 🔖️Design

//#region 🔖️Snapshot
#[derive(Clone, Debug, PartialEq, ArtifactSchema, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[artifact_schema(id = "s.stdio.semio.kit")]
pub struct SemioKitSnapshot {
    #[state(artifact)]
    pub schema: String,
    #[state(artifact)]
    pub types: Vec<SemioKitType>,
    #[state(artifact)]
    pub designs: Vec<SemioKitDesign>,
    #[state(artifact)]
    #[child(kind = "s.stdio.semio")]
    pub objects: Vec<store::ArtifactChild<SemioObjectSnapshot>>,
    #[state(artifact)]
    #[child(kind = "s.stdio.semio")]
    pub models: Vec<store::ArtifactChild<SemioModelSnapshot>>,
    #[state(artifact)]
    #[child(kind = "s.stdio.semio")]
    pub properties: Option<store::ArtifactChild<SemioValueSnapshot>>,
    #[state(artifact)]
    #[link_slot(roles("representation"))]
    pub representations: Vec<store::ArtifactLink>,
}

impl Default for SemioKitSnapshot {
    fn default() -> Self {
        Self { schema: STDIO_SEMIOKIT_DOCUMENT_SCHEMA.into(), types: Vec::new(), designs: Vec::new(), objects: Vec::new(), models: Vec::new(), properties: None, representations: Vec::new() }
    }
}
//#endregion 🔖️Snapshot

//#region 🔖️ValueCodec
/// 🔀️ Encodes composite child and link fields through their first-party value contracts.
impl semio_framework_value::ToValue for SemioKitSnapshot {
    fn to_value(&self) -> semio_framework_value::DslValue {
        let mut entries = vec![
            ("schema".to_string(), semio_framework_value::ToValue::to_value(&self.schema)),
            ("types".to_string(), semio_framework_value::ToValue::to_value(&self.types)),
            ("designs".to_string(), semio_framework_value::ToValue::to_value(&self.designs)),
            ("objects".to_string(), semio_framework_value::ToValue::to_value(&self.objects)),
            ("models".to_string(), semio_framework_value::ToValue::to_value(&self.models)),
            ("representations".to_string(), semio_framework_value::ToValue::to_value(&self.representations)),
        ];
        if let Some(properties) = &self.properties {
            entries.push(("properties".to_string(), semio_framework_value::ToValue::to_value(properties)));
        }
        semio_framework_value::DslValue::object(entries)
    }
}
impl semio_framework_value::FromValue for SemioKitSnapshot {
    fn edit_value_at_path(&mut self, path: &[&str], edit: semio_framework_value::ValueEdit) -> Result<(), semio_framework_value::ValueError> {
        semio_framework_value::edit_through_value(self, path, edit)
    }

    fn from_value(value: semio_framework_value::DslValue) -> Result<Self, semio_framework_value::ValueError> {
        let entries = semio_framework_value::DslValue::into_object(value)?;
        let get = |key: &str| entries.iter().find(|(k, _)| k == key).map(|(_, v)| v.clone());
        let field = |key: &str| get(key).ok_or_else(|| semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, format!("missing field `{key}`")));
        Ok(Self {
            schema: semio_framework_value::FromValue::from_value(field("schema")?)?,
            types: get("types").map(semio_framework_value::FromValue::from_value).transpose()?.unwrap_or_default(),
            designs: get("designs").map(semio_framework_value::FromValue::from_value).transpose()?.unwrap_or_default(),
            objects: get("objects").map(semio_framework_value::FromValue::from_value).transpose()?.unwrap_or_default(),
            models: get("models").map(semio_framework_value::FromValue::from_value).transpose()?.unwrap_or_default(),
            properties: get("properties").map(semio_framework_value::FromValue::from_value).transpose()?,
            representations: get("representations").map(semio_framework_value::FromValue::from_value).transpose()?.unwrap_or_default(),
        })
    }
}
//#endregion 🔖️ValueCodec

//#region 🔖️CodecPrimitives









































//#endregion 🔖️CodecPrimitives

//#region 🔖️TextPrimitives


//#endregion 🔖️TextPrimitives

//#region 🔖️BinaryPrimitives









































//#endregion 🔖️BinaryPrimitives

//#region 🔖️HandcraftedArtifactCodecs



//#endregion 🔖️HandcraftedArtifactCodecs

//#region 🔖️JsonBridge



//#endregion 🔖️JsonBridge

//#region 🔖️Wire







//#endregion 🔖️Wire

//#region 🔖️Demo
/// 🌱 The demo `s.stdio.semio.kit` — one type ("chair") with one representation link, one design
/// ("living-room") with two pieces and one connection, one owned object child, one owned model
/// child, and a properties child. Exercises every field shape at least once.
#[cfg(test)]
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn demo_kit_snapshot() -> SemioKitSnapshot {
    let dialect = |subset: &str| semio_framework_artifact_reference::ArtifactDialect { artifact_kind: "s.stdio.semio".into(), standard: "v1".into(), subset: subset.into() };
    SemioKitSnapshot {
        schema: STDIO_SEMIOKIT_DOCUMENT_SCHEMA.into(),
        types: vec![SemioKitType { id: "chair".into(), name: "Chair".into(), category: "furniture".into() }],
        designs: vec![SemioKitDesign {
            id: "living-room".into(),
            name: "Living Room".into(),
            pieces: vec![SemioKitPiece { id: "piece-1".into(), type_id: "chair".into(), transform: SemioTransform::identity() }, SemioKitPiece { id: "piece-2".into(), type_id: "chair".into(), transform: SemioTransform::identity() }],
            connections: vec![SemioKitConnection { id: "conn-1".into(), connecting_piece_id: "piece-1".into(), connecting_port: "left".into(), connected_piece_id: "piece-2".into(), connected_port: "right".into() }],
        }],
        objects: vec![store::ArtifactChild::new("obj-01".into(), semio_framework_artifact_reference::ArtifactRef { artifact_id: "obj-01".into(), dialect: dialect("object") })],
        models: vec![store::ArtifactChild::new("model-01".into(), semio_framework_artifact_reference::ArtifactRef { artifact_id: "model-01".into(), dialect: dialect("model") })],
        properties: Some(store::ArtifactChild::new("props-01".into(), semio_framework_artifact_reference::ArtifactRef { artifact_id: "props-01".into(), dialect: dialect("value") })),
        representations: vec![store::ArtifactLink { target: semio_framework_artifact_reference::ArtifactRef { artifact_id: "chair-repr".into(), dialect: dialect("mesh") }, pin: store::LinkPin::Head, role: "chair".into() }],
    }
}
//#endregion 🔖️Demo

//#region 🔖️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🔖️Tests








/// 🪆️ This typed kit owner's actual native dialect boundary, independent of child identity.
impl SemioKitSnapshot {
 pub fn admits_dialect_parts(kind:&str,standard:&str,subset:&str)->bool{kind=="s.stdio.semio"&&standard=="v1"&&matches!(subset,"kit"|"*")}
}
