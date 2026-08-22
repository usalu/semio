//! 🧬️ Puzzle5d snapshot schema — artifact-lane fields only.

use crate::artifacts::puzzle5d::{Puzzle5dFastener, Puzzle5dKindCatalogsExtra, Puzzle5dKindCompatibility, Puzzle5dMeta, Puzzle5dPart, PUZZLE_5D_SCHEMA};
use artifact_schema::ArtifactSchema;
use semio_s_plugin_stdio::artifacts::semio::standards::v1::subsets::kit::schema::snapshot::SemioKitSnapshot;
use serde::{Deserialize, Serialize};

//#region 🔖️Snapshot
/// 📸️ Persisted puzzle5d document snapshot (persistent fields of the artifact).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord, ArtifactSchema)]
#[serde(rename_all = "camelCase")]
#[dsl(id = "puzzle.puzzle5d", layout = "lines")]
#[artifact_schema(id = "s.puzzle.puzzle5d")]
pub struct Puzzle5dSnapshot {
    #[state(artifact)]
    pub schema: String,
    #[serde(default)]
    #[state(artifact)]
    pub domain: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[state(artifact)]
    pub label: Option<String>,
    #[serde(default)]
    #[dsl(block)]
    #[state(artifact)]
    pub meta: Puzzle5dMeta,
    /// 🧩️ Ticket 26/08/12/UNIFIED-COMPOSABLE-ARTIFACT-SYSTEM W4d: composed `s.stdio.semio.kit`
    /// child handle — the shared (`SemioKitType` id/name/category) half of what was the inline
    /// `Puzzle5dKindCatalogs` field. See `🗿️artifacts/🖐️5d/🦀️component.rs`'s `🔖️KindCatalogComposition`
    /// region for the split/join contract and `kind_catalogs_of` accessor.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[child(kind = "s.stdio.semio.kit")]
    #[state(artifact)]
    pub kind_catalogs: Option<store::ArtifactChild<SemioKitSnapshot>>,
    /// 🧩️ The puzzle5d-owned overflow half `SemioKitType` cannot represent — sibling to
    /// `kind_catalogs`, id-joined back together by `kind_catalogs_of`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[state(artifact)]
    pub kind_catalogs_extra: Option<Puzzle5dKindCatalogsExtra>,
    #[serde(default)]
    #[dsl(table)]
    #[state(artifact)]
    pub kind_compatibility: Vec<Puzzle5dKindCompatibility>,
    #[serde(default)]
    #[dsl(table)]
    #[state(artifact)]
    pub parts: Vec<Puzzle5dPart>,
    #[serde(default)]
    #[dsl(table)]
    #[state(artifact)]
    pub fasteners: Vec<Puzzle5dFastener>,
}
//#endregion 🔖️Snapshot

//#region 🔖️HandcraftedArtifactCodecs
/// ✉️ P6 handcrafted ArtifactDsl/ArtifactPack (derive no longer emits these traits).
impl store::ArtifactDsl for Puzzle5dSnapshot {
    const EXTENSION: &'static str = "puzzle5d";
    fn envelope_id() -> &'static str {
        "puzzle.puzzle5d"
    }
    fn parse_dsl(text: &str) -> Result<Self, store::TextError> {
        let body = match store::semio_format::split_text_preamble(text) {
            Ok((_, rest)) => rest,
            Err(_) => text,
        };
        let record = dsl::parse(body, &Self::__dsl_spec(), &dsl::ParseOptions { limits: dsl::Limits::default(), mode: dsl::SourceMode::Document })?;
        Self::__dsl_from_record(&record)
    }
    fn print_dsl(&self) -> String {
        let body = dsl::print(&self.__dsl_to_record(), &Self::__dsl_spec(), dsl::JoinMode::Document);
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(<Self as store::ArtifactDsl>::envelope_id(), store::semio_format::Component::Dsl, 1).expect("valid envelope_id");
        store::semio_format::wrap_text(&envelope, &body)
    }
}

impl store::ArtifactPack for Puzzle5dSnapshot {
    fn encode_pack_with(&self, options: &store::PackEncodeOptions) -> Result<Vec<u8>, store::PackError> {
        let inner = store::pack_rt::encode_document(&Self::__dsl_spec(), &self.__dsl_to_record(), options)?;
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(<Self as store::ArtifactDsl>::envelope_id(), store::semio_format::Component::Pack, 1).map_err(|e| store::PackError::Schema(e.to_string()))?;
        Ok(store::semio_format::wrap_binary(&envelope, &inner))
    }
    fn decode_pack_with(bytes: &[u8], options: &store::PackDecodeOptions) -> Result<Self, store::PackError> {
        let (envelope, inner) = store::semio_format::unwrap_binary(bytes).map_err(|e| store::PackError::Schema(e.to_string()))?;
        if envelope.envelope_id() != <Self as store::ArtifactDsl>::envelope_id() {
            return Err(store::PackError::Schema(format!("pack envelope mismatch: expected {}, got {}", <Self as store::ArtifactDsl>::envelope_id(), envelope.envelope_id())));
        }
        let (record, _report) = store::pack_rt::decode_document(&inner, &Self::__dsl_spec(), options)?;
        Self::__dsl_from_record(&record).map_err(store::text_error_to_pack_error)
    }
    fn record_spec() -> Option<dsl::RecordSpec> {
        Some(Self::__dsl_spec())
    }
}
//#endregion 🔖️HandcraftedArtifactCodecs

impl Default for Puzzle5dSnapshot {
    fn default() -> Self {
        Self { schema: PUZZLE_5D_SCHEMA.to_string(), domain: "architecture".to_string(), label: None, meta: Default::default(), kind_catalogs: None, kind_catalogs_extra: None, kind_compatibility: Vec::new(), parts: Vec::new(), fasteners: Vec::new() }
    }
}
