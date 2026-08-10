//! 🧬️ Playbook snapshot schema — persistent fields only.

use crate::artifacts::playbook::{PlaybookStep, PLAYBOOK_DOCUMENT_SCHEMA};
use schema::ArtifactSchema;
use serde::{Deserialize, Serialize};

//#region 🔖️Snapshot
/// 📸️ Persisted playbook document snapshot (persistent fields of the artifact).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ArtifactSchema)]
#[serde(rename_all = "camelCase")]
#[artifact_schema(id = "s.playbook.playbook")]
pub struct PlaybookSnapshot {
    #[state(persistent)]
    pub schema: String,
    #[state(persistent)]
    pub id: String,
    #[state(persistent)]
    pub version: String,
    #[state(persistent)]
    pub title: Option<String>,
    #[state(persistent)]
    pub steps: Vec<PlaybookStep>,
}

impl Default for PlaybookSnapshot {
    fn default() -> Self {
        Self::from_kernel(crate::playbook::empty_playbook_snapshot())
    }
}

impl PlaybookSnapshot {
    /// 🌉️ Builds a plugin snapshot from the shared kernel `PlaybookSnapshot`.
    pub fn from_kernel(spec: crate::playbook::PlaybookSpec) -> Self {
        Self {
            schema: spec.schema,
            id: spec.id,
            version: spec.version,
            title: spec.title,
            steps: spec.steps,
        }
    }

    /// 🌉️ Lowers this snapshot into the kernel `PlaybookSnapshot` for shared domain helpers.
    pub fn to_kernel(self) -> crate::playbook::PlaybookSpec {
        crate::playbook::PlaybookSpec {
            schema: self.schema,
            id: self.id,
            version: self.version,
            title: self.title,
            steps: self.steps,
        }
    }

    /// 🌉️ Borrows as kernel spec without consuming `self`.
    pub fn as_kernel(&self) -> crate::playbook::PlaybookSpec {
        self.clone().to_kernel()
    }
}
//#endregion 🔖️Snapshot

//#region 🔖️HandcraftedDocumentCodecs
impl store::DocumentDsl for PlaybookSnapshot {
    const EXTENSION: &'static str = "playbook";
    fn envelope_id() -> &'static str {
        "playbook.playbook"
    }
    fn parse_dsl(text: &str) -> Result<Self, store::TextError> {
        let body = match store::semio_format::split_text_preamble(text) {
            Ok((_, rest)) => rest,
            Err(_) => text,
        };
        Ok(Self::from_kernel(<crate::playbook::PlaybookSpec as store::DocumentDsl>::parse_dsl(body)?))
    }
    fn print_dsl(&self) -> String {
        let kernel = self.as_kernel();
        let body = dsl::print(
            &crate::playbook::PlaybookSpec::__dsl_to_record(&kernel),
            &crate::playbook::PlaybookSpec::__dsl_spec(),
            dsl::JoinMode::Document,
        );
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(
            <Self as store::DocumentDsl>::envelope_id(),
            store::semio_format::Component::Dsl,
            1,
        )
        .expect("valid envelope_id");
        store::semio_format::wrap_text(&envelope, &body)
    }
}

impl store::DocumentPack for PlaybookSnapshot {
    fn encode_pack_with(&self, options: &store::PackEncodeOptions) -> Result<Vec<u8>, store::PackError> {
        let kernel = self.as_kernel();
        let inner = store::pack_rt::encode_document(
            &crate::playbook::PlaybookSpec::__dsl_spec(),
            &crate::playbook::PlaybookSpec::__dsl_to_record(&kernel),
            options,
        )?;
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(
            <Self as store::DocumentDsl>::envelope_id(),
            store::semio_format::Component::Pack,
            1,
        )
        .map_err(|e| store::PackError::Schema(e.to_string()))?;
        Ok(store::semio_format::wrap_binary(&envelope, &inner))
    }
    fn decode_pack_with(bytes: &[u8], options: &store::PackDecodeOptions) -> Result<Self, store::PackError> {
        let (envelope, inner) =
            store::semio_format::unwrap_binary(bytes).map_err(|e| store::PackError::Schema(e.to_string()))?;
        if envelope.envelope_id() != <Self as store::DocumentDsl>::envelope_id() {
            return Err(store::PackError::Schema(format!(
                "pack envelope mismatch: expected {}, got {}",
                <Self as store::DocumentDsl>::envelope_id(),
                envelope.envelope_id()
            )));
        }
        let (record, _report) = store::pack_rt::decode_document(&inner, &crate::playbook::PlaybookSpec::__dsl_spec(), options)?;
        Ok(Self::from_kernel(
            crate::playbook::PlaybookSpec::__dsl_from_record(&record).map_err(store::text_error_to_pack_error)?,
        ))
    }
}
//#endregion 🔖️HandcraftedDocumentCodecs
