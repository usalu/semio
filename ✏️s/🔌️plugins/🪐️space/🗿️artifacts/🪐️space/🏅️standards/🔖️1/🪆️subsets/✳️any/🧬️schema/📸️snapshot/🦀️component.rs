//! 🧬️ S Space index snapshot schema — artifact-lane fields only. Ticket
//! 26/08/16/HUB-SPACES-LIVE-PRESENCE-AND-COLLABORATIVE-STUDIOS §C4.

use crate::artifacts::space::S_SPACE_INDEX_DOCUMENT_SCHEMA;
use schema::ArtifactSchema;
use serde::{Deserialize, Serialize};

//#region 🔖️Dialect
/// 🪪️ One artifact's coordinate inside a space's index — mirrors the freeze's
/// `dialect { artifactKind, standard, subset }` shape.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase", default)]
pub struct SpaceArtifactDialect {
    pub artifact_kind: String,
    pub standard: String,
    pub subset: String,
}
//#endregion 🔖️Dialect

//#region 🔖️Row
/// 📇️ One row of a space's artifact index — persisted metadata only, never the artifact's own
/// document bytes (those live in their own backbone document, addressed by `id`).
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase", default)]
pub struct SpaceArtifactRow {
    pub id: String,
    pub name: String,
    pub kind_id: String,
    pub schema: String,
    #[dsl(block)]
    pub dialect: SpaceArtifactDialect,
    pub created_at_ms: u64,
    pub created_by: String,
    pub updated_at_ms: u64,
    pub updated_by: String,
}
//#endregion 🔖️Row

//#region 🔖️Snapshot
/// 📸️ Persisted S Space index document snapshot — one per hub space, document id `index`.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ArtifactSchema, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
#[artifact_schema(id = "s.space.space")]
#[dsl(extension = "sspace")]
#[dsl(layout = "lines")]
pub struct SSpaceSnapshot {
    #[state(artifact)]
    pub schema: String,
    #[state(artifact)]
    pub space_id: String,
    #[dsl(table)]
    #[state(artifact)]
    pub artifacts: Vec<SpaceArtifactRow>,
}
//#endregion 🔖️Snapshot

//#region 🔖️HandcraftedArtifactCodecs
/// ✉️ Handcrafted `ArtifactDsl`/`ArtifactPack` — mirrors `SHomeSnapshot`'s own handcrafted pair.
impl store::ArtifactDsl for SSpaceSnapshot {
    const EXTENSION: &'static str = "sspace";
    async fn envelope_id() -> &'static str {
        "s.space"
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

impl store::ArtifactPack for SSpaceSnapshot {
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

impl Default for SSpaceSnapshot {
    async fn default() -> Self {
        Self { schema: S_SPACE_INDEX_DOCUMENT_SCHEMA.into(), space_id: String::new(), artifacts: Vec::new() }
    }
}

//#region 🔖️DocumentHelpers
/// 🆕️ A fresh, empty index for a newly created hub space.
pub async fn empty_space_index_snapshot(space_id: &str) -> SSpaceSnapshot {
    SSpaceSnapshot { schema: S_SPACE_INDEX_DOCUMENT_SCHEMA.into(), space_id: space_id.into(), artifacts: Vec::new() }
}

/// 🆔️ Mints a fresh artifact id for `create-artifact` — `handle()` is pure/no-IO, so this derives
/// uniqueness from the mutation's own inputs (creation instant + a collision-probed counter) rather
/// than a random/host-global source. Unique within `existing` (one space's index).
pub async fn mint_artifact_id(existing: &[SpaceArtifactRow], now_ms: u64) -> String {
    let mut n = existing.len() as u64;
    loop {
        let candidate = format!("artifact-{now_ms}-{n}");
        if !existing.iter().any(|row| row.id == candidate) {
            return candidate;
        }
        n += 1;
    }
}
//#endregion 🔖️DocumentHelpers

//#region 🔖️TableProjection
/// 📊️ The space app table's columns (worker-brief task 1: name · kind · subset · updated ·
/// updated-by · presence, `id` first as the row's own identity cell) — the single source of truth
/// both the editor's and the viewer's `main` window render from (neutral schema-layer helper so the
/// viewer never has to import from `✏️editor`, `policyViewerPurityBreaches`).
pub const SPACE_INDEX_TABLE_COLUMNS: [&str; 7] = ["ID", "Name", "Kind", "Subset", "Updated", "Updated By", "Presence"];

/// 📊️ One table row for `row`; `presence` is a display-ready summary (empty string when the caller
/// has no live presence data, e.g. the viewer, which folds no `fold-directory-events`/
/// `presence-heartbeat` commands of its own).
pub async fn space_index_table_row(row: &SpaceArtifactRow, presence: &str) -> Vec<String> {
    vec![row.id.clone(), row.name.clone(), row.kind_id.clone(), row.dialect.subset.clone(), row.updated_at_ms.to_string(), row.updated_by.clone(), presence.into()]
}
//#endregion 🔖️TableProjection

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    async fn empty_snapshot_uses_the_space_index_schema() {
        let snapshot = empty_space_index_snapshot("space-1");
        assert_eq!(snapshot.schema, S_SPACE_INDEX_DOCUMENT_SCHEMA);
        assert_eq!(snapshot.space_id, "space-1");
        assert!(snapshot.artifacts.is_empty());
    }

    #[test]
    async fn mint_artifact_id_probes_past_a_collision() {
        let existing = vec![SpaceArtifactRow { id: "artifact-1-0".into(), ..Default::default() }];
        assert_eq!(mint_artifact_id(&existing, 1), "artifact-1-1");
        assert_eq!(mint_artifact_id(&[], 1), "artifact-1-0");
    }

    #[test]
    async fn table_row_projects_the_seven_worker_brief_columns() {
        assert_eq!(SPACE_INDEX_TABLE_COLUMNS.len(), 7);
        let row = SpaceArtifactRow {
            id: "artifact-1".into(),
            name: "First".into(),
            kind_id: "s.draw.draw".into(),
            schema: "draw.document".into(),
            dialect: SpaceArtifactDialect { artifact_kind: "s.draw.draw".into(), standard: "1".into(), subset: "*".into() },
            created_at_ms: 1,
            created_by: "user:1".into(),
            updated_at_ms: 42,
            updated_by: "user:2".into(),
        };
        assert_eq!(space_index_table_row(&row, "user:9"), vec!["artifact-1", "First", "s.draw.draw", "*", "42", "user:2", "user:9"]);
    }

    #[test]
    async fn dsl_round_trips_default_and_populated_documents() {
        store::os_store::test_support::assert_dsl_round_trip(&SSpaceSnapshot::default());
        let mut populated = empty_space_index_snapshot("space-2");
        populated.artifacts.push(SpaceArtifactRow {
            id: "artifact-1".into(),
            name: "First".into(),
            kind_id: "space.sdraw".into(),
            schema: "s.draw".into(),
            dialect: SpaceArtifactDialect { artifact_kind: "s.draw".into(), standard: "1".into(), subset: "*".into() },
            created_at_ms: 1,
            created_by: "user:1".into(),
            updated_at_ms: 1,
            updated_by: "user:1".into(),
        });
        store::os_store::test_support::assert_dsl_round_trip(&populated);
    }
}
//#endregion 🧪️Tests
