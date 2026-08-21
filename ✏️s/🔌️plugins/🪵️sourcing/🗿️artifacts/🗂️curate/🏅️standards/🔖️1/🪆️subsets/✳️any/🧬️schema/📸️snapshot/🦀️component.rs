//! 🧬️ Curate snapshot schema — artifact-lane fields only.

use crate::artifacts::curate::{CuratedItem, ObjectKindExtra};
use schema::ArtifactSchema;
use semio_s_plugin_stdio::artifacts::semio::standards::v1::subsets::kit::schema::snapshot::SemioKitSnapshot;
use serde::{Deserialize, Serialize};

//#region 🔖️Snapshot
/// 📸️ Persisted curate document snapshot (persistent fields of the artifact). `catalog`/`stock_extra`
/// together replace the former inline `stock: Vec<ObjectKind>` field: `catalog` composes stdio's
/// `s.stdio.semio.kit` subset as an owned child (the shared `id`/`name`/`category` type-registry
/// vocabulary), `stock_extra` carries the sourcing-owned overflow (`typologyPath`/`availability`/
/// `geometry`) that subset can't represent — see `crate::artifacts::curate::stock_of` for the
/// reassembly accessor every reader funnels through.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord, ArtifactSchema)]
#[serde(rename_all = "camelCase")]
#[dsl(id = "curate.curate", layout = "lines")]
#[artifact_schema(id = "s.sourcing.curate")]
pub struct CurateSnapshot {
    #[state(artifact)]
    #[child(kind = "s.stdio.semio.kit")]
    pub catalog: store::ArtifactChild<SemioKitSnapshot>,
    #[state(artifact)]
    #[serde(default)]
    pub stock_extra: Vec<ObjectKindExtra>,
    #[state(artifact)]
    #[serde(default)]
    #[dsl(table)]
    pub curated: Vec<CuratedItem>,
}

impl Default for CurateSnapshot {
    /// 🌱 `ArtifactChild<S>` has no blanket `Default` (its target is content-addressed, never
    /// arbitrary), so this is hand-written rather than derived — mints the same empty-stock handle
    /// `catalog_child_handle(&[])` would, matching an explicitly-built empty document.
    fn default() -> Self {
        Self { catalog: crate::artifacts::curate::catalog_child_handle(&[]), stock_extra: Vec::new(), curated: Vec::new() }
    }
}
//#region 🔖️HandcraftedArtifactCodecs
/// ✉️ P6 handcrafted ArtifactDsl/ArtifactPack (derive no longer emits these traits).
impl store::ArtifactDsl for CurateSnapshot {
    const EXTENSION: &'static str = "curate";
    async fn envelope_id() -> &'static str {
        "curate.curate"
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

impl store::ArtifactPack for CurateSnapshot {
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
//#endregion 🔖️Snapshot
