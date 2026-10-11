//! 🧬️ S Space index snapshot schema — artifact-lane fields only. Ticket
//! 26/08/16/HUB-SPACES-LIVE-PRESENCE-AND-COLLABORATIVE-STUDIOS §C4.

use crate::S_SPACE_INDEX_DOCUMENT_SCHEMA;
use ::semio_framework_schema::ArtifactSchema;
use semio_s_space_core::time::utc_minute_text;

//#region 🔖️Dialect
/// 🪪️ One artifact's coordinate inside a space's index — mirrors the freeze's
/// `dialect { artifactKind, standard, subset }` shape.
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[value(rename_all = "camelCase", default)]
pub struct SpaceArtifactDialect {
    pub artifact_kind: String,
    pub standard: String,
    pub subset: String,
}
//#endregion 🔖️Dialect

//#region 🔖️Row
/// 📇️ One row of a space's artifact index — persisted metadata only, never the artifact's own
/// document bytes (those live in their own backbone document, addressed by `id`).
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[value(rename_all = "camelCase", default)]
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
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, ArtifactSchema, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned)]
#[value(rename_all = "camelCase")]
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



//#endregion 🔖️HandcraftedArtifactCodecs

impl Default for SSpaceSnapshot {
    fn default() -> Self {
        Self { schema: S_SPACE_INDEX_DOCUMENT_SCHEMA.into(), space_id: String::new(), artifacts: Vec::new() }
    }
}

//#region 🔖️DocumentHelpers
/// 🆕️ A fresh, empty index for a newly created hub space.
pub fn empty_space_index_snapshot(space_id: &str) -> SSpaceSnapshot {
    SSpaceSnapshot { schema: S_SPACE_INDEX_DOCUMENT_SCHEMA.into(), space_id: space_id.into(), artifacts: Vec::new() }
}

/// 🆔️ Mints a fresh artifact id for `create-artifact` — `handle()` is pure/no-IO, so this derives
/// uniqueness from the mutation's own inputs (creation instant + a collision-probed counter) rather
/// than a random/host-global source. Unique within `existing` (one space's index).
pub fn mint_artifact_id(existing: &[SpaceArtifactRow], now_ms: u64) -> String {
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
semio_framework_ui_locale::app_labels! {
    /// 🗣️ The space app table's strings (worker-brief task 1: name · id · kind · subset · updated · updated-by ·
    /// presence) — the single source both the editor's and the viewer's `main` window render from (neutral
    /// schema-layer helper so the viewer never has to import from `✏️editor`, `policyViewerPurityBreaches`). The
    /// NAME leads: the table grid names every row and every row button by its first cell ("Öffnen: Werkstattplan"),
    /// and a leading artifact id read "Open: artifact-5bd9…" to a screen reader (ticket 26/09/23 S16); the row's
    /// identity is its key (`artifact:<id>`), never a cell.
    pub struct SpaceIndexTableLabels {
        table_name: native_en "Artifacts", native_de "Artefakte", reuse_en "Artifacts", reuse_de "Artefakte";
        column_id: native_en "ID", native_de "ID", reuse_en "ID", reuse_de "ID";
        column_name: native_en "Name", native_de "Name", reuse_en "Name", reuse_de "Name";
        column_kind: native_en "Kind", native_de "Art", reuse_en "Kind", reuse_de "Art";
        column_subset: native_en "Subset", native_de "Teilmenge", reuse_en "Subset", reuse_de "Teilmenge";
        column_updated: native_en "Updated", native_de "Aktualisiert", reuse_en "Updated", reuse_de "Aktualisiert";
        column_updated_by: native_en "Updated By", native_de "Aktualisiert von", reuse_en "Updated By", reuse_de "Aktualisiert von";
        column_presence: native_en "Presence", native_de "Anwesenheit", reuse_en "Presence", reuse_de "Anwesenheit";
        column_actions: native_en "Actions", native_de "Aktionen", reuse_en "Actions", reuse_de "Aktionen";
        action_open: native_en "Open", native_de "Öffnen", reuse_en "Open", reuse_de "Öffnen";
        create_artifact: native_en "Create Artifact", native_de "Artefakt erstellen", reuse_en "Create Artifact", reuse_de "Artefakt erstellen";
    }
}

impl SpaceIndexTableLabels {
    /// 📊️ The seven column headers, in cell order.
    pub fn columns(&self) -> [&str; 7] {
        [self.column_name.as_str(), self.column_id.as_str(), self.column_kind.as_str(), self.column_subset.as_str(), self.column_updated.as_str(), self.column_updated_by.as_str(), self.column_presence.as_str()]
    }

    /// 📊️ One table row for `row` in these labels' language; `presence` is a display-ready summary (empty
    /// when the caller has no live presence data, e.g. the viewer, which folds no presence of its own).
    pub fn row(&self, row: &SpaceArtifactRow, presence: &str) -> [String; 7] {
        [row.name.clone(), row.id.clone(), row.kind_id.clone(), row.dialect.subset.clone(), utc_minute_text(row.updated_at_ms, self.locale), row.updated_by.clone(), presence.into()]
    }
}

//#endregion 🔖️TableProjection

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests

//#region 🌉️IdentityBridge

//#endregion 🌉️IdentityBridge

