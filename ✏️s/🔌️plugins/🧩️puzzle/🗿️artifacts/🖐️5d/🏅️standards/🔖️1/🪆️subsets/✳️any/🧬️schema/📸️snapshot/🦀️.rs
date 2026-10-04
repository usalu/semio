//! 🧬️ Puzzle5d snapshot schema — artifact-lane fields only.

use crate::{Puzzle5dFastener, Puzzle5dKindCompatibility, Puzzle5dMeta, Puzzle5dPart, Puzzle5dTargetVolume, PUZZLE_5D_SCHEMA};
use ::semio_framework_schema::ArtifactSchema;
use semio_s_artifact_stdio_semio::standards::v1::subsets::kit::schema::snapshot::SemioKitSnapshot;

#[path="🪶️sqlite/🦀️.rs"]
mod sqlite;

/// 🚪️ Owns the codec used by the actual subset's native declaration.
pub(crate) fn native_codec()->store::ArtifactCodec {
    store::ArtifactCodec::bare::<Puzzle5dSnapshot,crate::Puzzle5dMutation>(PUZZLE_5D_SCHEMA)
}

//#region 🔖️Snapshot
/// 📸️ Persisted puzzle5d document snapshot (persistent fields of the artifact).
///
/// 🔣️ Artifact JSON uses the first-party value codec, including the composed kit child.
#[derive(Clone, Debug, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue, semio_framework_dsl_record_derive::DslRecord, ArtifactSchema)]
#[value(rename_all = "camelCase")]
#[dsl(id = "puzzle.puzzle5d", layout = "lines")]
#[artifact_schema(id = "s.puzzle.puzzle5d")]
pub struct Puzzle5dSnapshot {
    #[state(artifact)]
    pub schema: String,
    #[value(default)]
    #[state(artifact)]
    pub domain: String,
    #[value(default, skip_serializing_if = "Option::is_none")]
    #[state(artifact)]
    pub label: Option<String>,
    #[value(default)]
    #[dsl(block)]
    #[state(artifact)]
    pub meta: Puzzle5dMeta,
    /// 🧩️ Ticket 26/08/12/UNIFIED-COMPOSABLE-ARTIFACT-SYSTEM W4d: composed `s.stdio.semio.kit`
    /// child handle — the shared (`SemioKitType` id/name/category) half of what was the inline
    /// `Puzzle5dKindCatalogs` field. See `🗿️artifacts/🖐️5d/🦀️.rs`'s `🔖️KindCatalogComposition`
    /// region for the split/join contract and `kind_catalogs_of` accessor.
    #[value(default, skip_serializing_if = "Option::is_none")]
    #[child(kind = "s.stdio.semio")]
    #[state(artifact)]
    pub kind_catalogs: Option<store::ArtifactChild<SemioKitSnapshot>>,
    /// 🧩️ The puzzle5d-owned overflow half `SemioKitType` cannot represent — sibling to
    /// `kind_catalogs`, id-joined back together by `kind_catalogs_of`.
    #[value(default, skip_serializing_if = "Option::is_none")]
    #[state(artifact)]
    pub kind_catalogs_extra: Option<Puzzle5dKindCatalogsExtra>,
    #[value(default)]
    #[dsl(table)]
    #[state(artifact)]
    pub kind_compatibility: Vec<Puzzle5dKindCompatibility>,
    #[value(default)]
    #[dsl(table)]
    #[state(artifact)]
    pub parts: Vec<Puzzle5dPart>,
    #[value(default)]
    #[dsl(table)]
    #[state(artifact)]
    pub fasteners: Vec<Puzzle5dFastener>,
    /// 🧊️ Oriented boxes constraining where the fill planner may place — the 3d-projection half of
    /// the document; the board pane paints their derived flat rectangles.
    #[value(default, skip_serializing_if = "Vec::is_empty")]
    #[dsl(table)]
    #[state(artifact)]
    pub target_volumes: Vec<Puzzle5dTargetVolume>,
}
//#endregion 🔖️Snapshot

//#region 🔖️HandcraftedArtifactCodecs
/// ✉️ P6 handcrafted ArtifactDsl/ArtifactPack (derive no longer emits these traits).
impl store::ArtifactDsl for Puzzle5dSnapshot {
    const EXTENSION: &'static str = "puzzle5d";
    fn envelope_id() -> &'static str {
        "puzzle.puzzle5d"
    }
    fn parse_dsl(text: &str) -> Result<Self, semio_framework_diagnostic::TextError> {
        let body = match store::semio_format::split_text_preamble(text) {
            Ok((_, rest)) => rest,
            Err(_) => text,
        };
        let record = semio_framework_dsl_record::parse(body, &Self::__dsl_spec(), &semio_framework_dsl_record::ParseOptions { limits: semio_framework_diagnostic::Limits::default(), mode: semio_framework_dsl_record::SourceMode::Document })?;
        Self::__dsl_from_record(&record)
    }
    fn print_dsl(&self) -> String {
        let body = semio_framework_dsl_record::print(&self.__dsl_to_record(), &Self::__dsl_spec(), semio_framework_dsl_record::JoinMode::Document);
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(<Self as store::ArtifactDsl>::envelope_id(), store::semio_format::Component::Dsl, 1).expect("valid envelope_id");
        store::semio_format::wrap_text(&envelope, &body)
    }
}

impl store::ArtifactPack for Puzzle5dSnapshot {
    fn encode_pack_with(&self, options: &store::PackEncodeOptions) -> Result<Vec<u8>, store::PackError> {
        let inner = store::pack_rt::encode_document(&Self::__dsl_spec(), &self.__dsl_to_record(), options)?;
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(<Self as store::ArtifactDsl>::envelope_id(), store::semio_format::Component::Pack, 1).map_err(|e| store::PackError::from(e.into_value_error()))?;
        Ok(store::semio_format::wrap_binary(&envelope, &inner))
    }
    fn decode_pack_with(bytes: &[u8], options: &store::PackDecodeOptions) -> Result<Self, store::PackError> {
        let (envelope, inner) = store::semio_format::unwrap_binary(bytes).map_err(|e| store::PackError::from(e.into_value_error()))?;
        if !envelope.matches_identity(<Self as store::ArtifactDsl>::envelope_id(), store::semio_format::Component::Pack, 1) {
            return Err(store::PackError::from(semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, format!("pack envelope mismatch: expected {}.pack v1, got {}", <Self as store::ArtifactDsl>::envelope_id(), envelope.binary_token()))));
        }
        let (record, _report) = store::pack_rt::decode_document(&inner, &Self::__dsl_spec(), options)?;
        Self::__dsl_from_record(&record).map_err(store::text_error_to_pack_error)
    }
    fn record_spec() -> Option<semio_framework_dsl_record::RecordSpec> {
        Some(Self::__dsl_spec())
    }
    fn sqlite_snapshot_codec() -> Option<store::ArtifactSqliteSnapshotCodec> {
        Some(<Self as store::ArtifactSqliteSnapshot>::sqlite_codec())
    }
}
//#endregion 🔖️HandcraftedArtifactCodecs

impl Default for Puzzle5dSnapshot {
    fn default() -> Self {
        Self { schema: PUZZLE_5D_SCHEMA.to_string(), domain: "architecture".to_string(), label: None, meta: Default::default(), kind_catalogs: None, kind_catalogs_extra: None, kind_compatibility: Vec::new(), parts: Vec::new(), fasteners: Vec::new(), target_volumes: Vec::new() }
    }
}

//#region 🔁️Re-exports
/// 🔁️ Entities this module's schema exports and its crate declares elsewhere.
pub use crate::Puzzle5dKindCatalogsExtra;
//#endregion 🔁️Re-exports

#[cfg(test)]
#[path = "🧪️tests/🪶️sqlite/🦀️.rs"]
mod sqlite_snapshot_tests;
