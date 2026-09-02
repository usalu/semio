//! 🧬️ Note snapshot schema — artifact-lane fields only.

use crate::artifacts::note::{NoteBlockNode, NoteImageAsset, NOTE_DOCUMENT_SCHEMA};
use schema::ArtifactSchema;
use serde::{Deserialize, Serialize};
use semio_framework_value_derive::{FromValue, ToValue};
use std::collections::BTreeMap;

//#region 🔖️Snapshot
/// 📸️ Persisted note document snapshot (persistent fields of the artifact).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ToValue, FromValue, dsl::DslRecord, ArtifactSchema)]
#[value(rename_all = "camelCase")]
#[serde(rename_all = "camelCase")]
#[dsl(id = "note.note", layout = "lines")]
#[artifact_schema(id = "s.note.note")]
pub struct NoteSnapshot {
    #[state(artifact)]
    pub schema: String,
    #[state(artifact)]
    pub id: String,
    #[state(artifact)]
    #[value(skip_serializing_if = "Option::is_none")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[state(artifact)]
    #[value(default)]
    #[serde(default)]
    #[dsl(statements, block)]
    pub blocks: Vec<NoteBlockNode>,
    #[state(artifact)]
    #[value(skip_serializing_if = "Option::is_none")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub grid_visible: Option<bool>,
    #[state(artifact)]
    #[value(skip_serializing_if = "Option::is_none")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub grid_spacing: Option<f64>,
    #[state(artifact)]
    #[value(skip_serializing_if = "Option::is_none")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub grid_subdivisions: Option<f64>,
    #[state(artifact)]
    #[value(skip_serializing_if = "Option::is_none")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub grid_opacity: Option<f64>,
    #[state(artifact)]
    #[value(skip_serializing_if = "Option::is_none")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub snap_enabled: Option<bool>,
    #[state(artifact)]
    #[value(skip_serializing_if = "Option::is_none")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub snap_grid_spacing: Option<f64>,
    #[state(artifact)]
    #[value(skip_serializing_if = "Option::is_none")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pencil_width: Option<f64>,
    #[state(artifact)]
    #[value(skip_serializing_if = "Option::is_none")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub eraser_radius: Option<f64>,
    #[state(artifact)]
    #[value(default, skip_serializing_if = "BTreeMap::is_empty")]
    #[serde(default, skip_serializing_if = "BTreeMap::is_empty")]
    pub assets: BTreeMap<String, NoteImageAsset>,
    /// 🔗️ Forward reference slot — ticket `26/08/12/UNIFIED-COMPOSABLE-ARTIFACT-SYSTEM` (`note→R:any`):
    /// a note may point at any other artifact (not a specific composed type), matching layout's
    /// `referenced_model` precedent. Schema/codec-complete, deliberately left inert beyond that (no
    /// mutation dispatch, no resolver read path) — genuinely new capability with no existing UI/
    /// converter to preserve, same honest scope layout's own report used for its analogous slot.
    #[state(artifact)]
    #[link_slot(roles("any"))]
    #[value(default, skip_serializing_if = "Option::is_none")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub linked_artifact: Option<store::ArtifactLink>,
}
//#region 🔖️HandcraftedArtifactCodecs
/// ✉️ P6 handcrafted ArtifactDsl/ArtifactPack (derive no longer emits these traits).
impl store::ArtifactDsl for NoteSnapshot {
    const EXTENSION: &'static str = "note";
    async fn envelope_id() -> &'static str {
        "note.note"
    }
    async fn parse_dsl(text: &str) -> Result<Self, store::TextError> {
        let body = match store::semio_format::split_text_preamble(text) {
            Ok((_, rest)) => rest,
            Err(_) => text,
        };
        let record = dsl::parse(body, &Self::__dsl_spec(), &dsl::ParseOptions { limits: dsl::Limits::default(), mode: dsl::SourceMode::Document })?;
        Self::__dsl_from_record(&record)
    }
    async fn print_dsl(&self) -> String {
        let body = dsl::print(&self.__dsl_to_record(), &Self::__dsl_spec(), dsl::JoinMode::Document);
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(<Self as store::ArtifactDsl>::envelope_id(), store::semio_format::Component::Dsl, 1).expect("valid envelope_id");
        store::semio_format::wrap_text(&envelope, &body)
    }
}

impl store::ArtifactPack for NoteSnapshot {
    async fn encode_pack_with(&self, options: &store::PackEncodeOptions) -> Result<Vec<u8>, store::PackError> {
        let inner = store::pack_rt::encode_document(&Self::__dsl_spec(), &self.__dsl_to_record(), options)?;
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(<Self as store::ArtifactDsl>::envelope_id(), store::semio_format::Component::Pack, 1).map_err(|e| store::PackError::Schema(e.to_string()))?;
        Ok(store::semio_format::wrap_binary(&envelope, &inner))
    }
    async fn decode_pack_with(bytes: &[u8], options: &store::PackDecodeOptions) -> Result<Self, store::PackError> {
        let (envelope, inner) = store::semio_format::unwrap_binary(bytes).map_err(|e| store::PackError::Schema(e.to_string()))?;
        if envelope.envelope_id() != <Self as store::ArtifactDsl>::envelope_id() {
            return Err(store::PackError::Schema(format!("pack envelope mismatch: expected {}, got {}", <Self as store::ArtifactDsl>::envelope_id(), envelope.envelope_id())));
        }
        let (record, _report) = store::pack_rt::decode_document(&inner, &Self::__dsl_spec(), options)?;
        Self::__dsl_from_record(&record).map_err(store::text_error_to_pack_error)
    }
    async fn record_spec() -> Option<dsl::RecordSpec> {
        Some(Self::__dsl_spec())
    }
}
//#endregion 🔖️HandcraftedArtifactCodecs

impl Default for NoteSnapshot {
    fn default() -> Self {
        Self {
            schema: NOTE_DOCUMENT_SCHEMA.into(),
            id: String::new(),
            title: None,
            blocks: Vec::new(),
            grid_visible: Some(true),
            grid_spacing: Some(32.0),
            grid_subdivisions: Some(4.0),
            grid_opacity: Some(0.35),
            snap_enabled: Some(false),
            snap_grid_spacing: Some(8.0),
            pencil_width: Some(3.0),
            eraser_radius: Some(12.0),
            assets: BTreeMap::new(),
            linked_artifact: None,
        }
    }
}
//#endregion 🔖️Snapshot

//#region 🧪️Tests
#[cfg(test)]
mod round_trip_tests {
    use super::*;
    use crate::artifacts::note::NoteBlockNode;

    /// 🧪️ `linked_artifact` (the new `R:any` forward reference slot) and a text block's composed
    /// `content` child handle must both survive the hand-rolled text/binary codecs — codec
    /// completeness is not caught by `cargo check`, only a real round trip proves it.
    #[semio_framework_async_macros::async_test]
    async fn linked_artifact_and_text_content_round_trip_through_text_and_binary() {
        let mut snapshot = NoteSnapshot::default();
        snapshot.id = "doc-composed".into();
        snapshot.linked_artifact = Some(store::ArtifactLink { target: store::os_io::ArtifactRef::parse_uri("doc-2!s.writer.writer@1/any").expect("valid link ref uri"), pin: store::LinkPin::Head, role: "any".into() });
        snapshot.blocks.push(NoteBlockNode::Text {
            id: "text-1".into(),
            name: "Text".into(),
            x: 0.0,
            y: 0.0,
            width: 200.0,
            height: 80.0,
            rotation: 0.0,
            visible: true,
            locked: false,
            content: crate::artifacts::note::note_text_child_handle("text-1", &[]),
            font_size: 16.0,
            font_weight: "normal".into(),
            align: "left".into(),
        });

        let text = store::ArtifactDsl::print_dsl(&snapshot);
        let from_text = <NoteSnapshot as store::ArtifactDsl>::parse_dsl(&text).expect("parse round-tripped text");
        assert_eq!(from_text, snapshot);

        let bytes = store::ArtifactPack::encode_pack(&snapshot);
        let from_binary = <NoteSnapshot as store::ArtifactPack>::decode_pack(&bytes).expect("decode round-tripped binary");
        assert_eq!(from_binary, snapshot);
    }

    #[semio_framework_async_macros::async_test]
    async fn absent_linked_artifact_round_trips_as_none() {
        let snapshot = NoteSnapshot::default();
        let text = store::ArtifactDsl::print_dsl(&snapshot);
        assert_eq!(<NoteSnapshot as store::ArtifactDsl>::parse_dsl(&text).expect("parse"), snapshot);
        let bytes = store::ArtifactPack::encode_pack(&snapshot);
        assert_eq!(<NoteSnapshot as store::ArtifactPack>::decode_pack(&bytes).expect("decode"), snapshot);
    }
}
//#endregion 🧪️Tests
