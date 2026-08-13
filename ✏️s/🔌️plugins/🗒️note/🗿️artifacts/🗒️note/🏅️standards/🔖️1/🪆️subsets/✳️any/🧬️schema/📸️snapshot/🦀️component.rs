//! 🧬️ Note snapshot schema — persistent fields only.

use crate::artifacts::note::{NoteBlockNode, NoteImageAsset, NOTE_DOCUMENT_SCHEMA};
use schema::ArtifactSchema;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

//#region 🔖️Snapshot
/// 📸️ Persisted note document snapshot (persistent fields of the artifact).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord, ArtifactSchema)]
#[serde(rename_all = "camelCase")]
#[dsl(id = "note.note", layout = "lines")]
#[artifact_schema(id = "s.note.note")]
pub struct NoteSnapshot {
    #[state(artifact)]
    pub schema: String,
    #[state(artifact)]
    pub id: String,
    #[state(artifact)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[state(artifact)]
    #[serde(default)]
    #[dsl(statements, block)]
    pub blocks: Vec<NoteBlockNode>,
    #[state(artifact)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub grid_visible: Option<bool>,
    #[state(artifact)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub grid_spacing: Option<f64>,
    #[state(artifact)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub grid_subdivisions: Option<f64>,
    #[state(artifact)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub grid_opacity: Option<f64>,
    #[state(artifact)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub snap_enabled: Option<bool>,
    #[state(artifact)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub snap_grid_spacing: Option<f64>,
    #[state(artifact)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pencil_width: Option<f64>,
    #[state(artifact)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub eraser_radius: Option<f64>,
    #[state(artifact)]
    #[serde(default, skip_serializing_if = "BTreeMap::is_empty")]
    pub assets: BTreeMap<String, NoteImageAsset>,
}
//#region 🔖️HandcraftedArtifactCodecs
/// ✉️ P6 handcrafted ArtifactDsl/ArtifactPack (derive no longer emits these traits).
impl store::ArtifactDsl for NoteSnapshot {
    const EXTENSION: &'static str = "note";
    fn envelope_id() -> &'static str {
        "note.note"
    }
    fn parse_dsl(text: &str) -> Result<Self, store::TextError> {
        let body = match store::semio_format::split_text_preamble(text) {
            Ok((_, rest)) => rest,
            Err(_) => text,
        };
        let record = dsl::parse(
            body,
            &Self::__dsl_spec(),
            &dsl::ParseOptions {
                limits: dsl::Limits::default(),
                mode: dsl::SourceMode::Document,
            },
        )?;
        Self::__dsl_from_record(&record)
    }
    fn print_dsl(&self) -> String {
        let body = dsl::print(&self.__dsl_to_record(), &Self::__dsl_spec(), dsl::JoinMode::Document);
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(
            <Self as store::ArtifactDsl>::envelope_id(),
            store::semio_format::Component::Dsl,
            1,
        )
        .expect("valid envelope_id");
        store::semio_format::wrap_text(&envelope, &body)
    }
}

impl store::ArtifactPack for NoteSnapshot {
    fn encode_pack_with(&self, options: &store::PackEncodeOptions) -> Result<Vec<u8>, store::PackError> {
        let inner = store::pack_rt::encode_document(&Self::__dsl_spec(), &self.__dsl_to_record(), options)?;
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(
            <Self as store::ArtifactDsl>::envelope_id(),
            store::semio_format::Component::Pack,
            1,
        )
        .map_err(|e| store::PackError::Schema(e.to_string()))?;
        Ok(store::semio_format::wrap_binary(&envelope, &inner))
    }
    fn decode_pack_with(bytes: &[u8], options: &store::PackDecodeOptions) -> Result<Self, store::PackError> {
        let (envelope, inner) = store::semio_format::unwrap_binary(bytes).map_err(|e| store::PackError::Schema(e.to_string()))?;
        if envelope.envelope_id() != <Self as store::ArtifactDsl>::envelope_id() {
            return Err(store::PackError::Schema(format!(
                "pack envelope mismatch: expected {}, got {}",
                <Self as store::ArtifactDsl>::envelope_id(),
                envelope.envelope_id()
            )));
        }
        let (record, _report) = store::pack_rt::decode_document(&inner, &Self::__dsl_spec(), options)?;
        Self::__dsl_from_record(&record).map_err(store::text_error_to_pack_error)
    }
    fn record_spec() -> Option<dsl::RecordSpec> {
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
        }
    }
}
//#endregion 🔖️Snapshot
