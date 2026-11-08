//! 🔋️ The public Energy document owns its sole native persistent model and original semantic sources.
#![allow(async_fn_in_trait)]
#![allow(clippy::too_many_arguments)]
extern crate semio_os_kernel as store;
extern crate protocol;
extern crate semio_framework_schema as framework_schema;
extern crate semio_s_plugin_energy_engine_air_exchange as air_exchange;
extern crate semio_s_plugin_energy_engine_calendar as calendar;
extern crate semio_s_plugin_energy_engine_model as model;
extern crate semio_s_plugin_energy_engine_schedule as schedule;

use semio_framework_artifact_reference::{Dialect,StandardId,SubsetId};
use crate::model::EntityId;



#[path="🧊️reference/🦀️.rs"]pub mod reference;
pub use original::EnergyModelSnapshot;
impl store::os_store::serialization::ArtifactStoreSnapshot for EnergyModelSnapshot{fn native_pack_source(&self)->Option<store::os_store::serialization::OriginalSnapshotPackView<'_>>{let(source,spec)=original::pack::source(self);Some(store::os_store::serialization::OriginalSnapshotPackView{source,spec,envelope:<Self as pack::snapshot::ArtifactDsl>::envelope_id()})}}
pub const ENERGY_MODEL_DOCUMENT_SCHEMA: &str = "energy.model";

/// 🧬️ Artifact schema descriptor id.
pub const ENERGY_MODEL_ARTIFACT_SCHEMA_ID: &str = "s.energy.model";

//#region 🔖️Dialect
/// 🪪️ Ticket 26/08/16/ARTIFACT-VIEWERS-AND-EDITORS-PER-SUBSET contract §1: the canonical surface-id
/// coordinate for this artifact's ONE subset (`✳️any`) — `s.energy.model@1/*`. Lives at the ARTIFACT
/// root (not under `✏️editor`/`👁️viewer`) so a viewer file can read it without ever importing
/// through the sibling editor module. `artifact_kind` matches this file's own snapshot schema id
/// (`EnergyModelSnapshot`'s `#[artifact_schema(id = "s.energy.model")]`, same as
/// `ENERGY_MODEL_ARTIFACT_SCHEMA_ID` above); `standard`/`subset` match this file's own
/// `🏅️standards/🔖️1/🪆️subsets/✳️any` location on disk.
pub const MODEL_DIALECT: Dialect = Dialect { artifact_kind: ENERGY_MODEL_ARTIFACT_SCHEMA_ID, standard: StandardId("1"), subset: SubsetId::ANY };
//#endregion 🔖️Dialect

pub fn energy_zones_table_from_model(model: &state::EnergyModelView<'_>) -> semio_s_artifact_stdio_semio_table_model::schema::snapshot::SemioTableSnapshot {
    use semio_s_artifact_stdio_semio_table_model::schema::snapshot::SemioTableCellKind;
    use semio_s_artifact_stdio_semio_table_model::schema::snapshot::SemioTableColumn;
    use semio_s_artifact_stdio_semio_table_model::schema::snapshot::SemioTableRow;
    use semio_s_artifact_stdio_semio_table_model::schema::snapshot::SemioTableSnapshot;
    use semio_s_artifact_stdio_semio_table_model::schema::snapshot::STDIO_SEMIOTABLE_DOCUMENT_SCHEMA;
    use semio_s_artifact_stdio_semio_value_model::schema::snapshot::SemioValue;
    SemioTableSnapshot {
        schema: STDIO_SEMIOTABLE_DOCUMENT_SCHEMA.into(),
        columns: vec![
            SemioTableColumn { name: "id".into(), kind: SemioTableCellKind::Int },
            SemioTableColumn { name: "name".into(), kind: SemioTableCellKind::Str },
            SemioTableColumn { name: "volumeM3".into(), kind: SemioTableCellKind::Float },
            SemioTableColumn { name: "multiplier".into(), kind: SemioTableCellKind::Int },
            SemioTableColumn { name: "conditioned".into(), kind: SemioTableCellKind::Bool },
            SemioTableColumn { name: "partOfTotalFloorArea".into(), kind: SemioTableCellKind::Bool },
        ],
        rows: model
            .zones
            .iter()
            .map(|zone| SemioTableRow {
                cells: vec![
                    SemioValue::Int { lexeme: zone.id.0.to_string() },
                    SemioValue::Str { value: zone.name.get().clone() },
                    SemioValue::Float { lexeme: format!("{}", zone.volume_m3) },
                    SemioValue::Int { lexeme: zone.multiplier.to_string() },
                    SemioValue::Bool { value: zone.conditioned },
                    SemioValue::Bool { value: zone.part_of_total_floor_area },
                ],
            })
            .collect(),
    }
}

pub struct EnergyModelReadLease {
    snapshot: Option<store::os_store::snapshot_read::SnapshotRead<EnergyModelSnapshot>>,
    returned: Option<store::os_store::snapshot_read::SnapshotReadReturn>,
    generation: u64,
    revision: [u8; 32],
}

impl EnergyModelReadLease {
    pub fn new(snapshot: store::os_store::snapshot_read::SnapshotRead<EnergyModelSnapshot>, generation: u64, revision: [u8; 32]) -> Self {
        Self { snapshot: Some(snapshot), returned: None, generation, revision }
    }

    pub fn model(&self) -> Option<&state::EnergyModelState> {
        let snapshot = self.snapshot.as_ref()?;
        snapshot.commit_authority_matches(self.generation, self.revision).then_some(&snapshot.model)
    }

    pub fn close_step(&mut self) -> bool {
        if let Some(snapshot) = self.snapshot.take() {
            let Some(witness) = snapshot.return_to_registry_witness() else { return false };
            self.returned = Some(witness);
            return false;
        }
        if self.returned.as_ref().is_some_and(|witness| !witness.terminal_is_empty()) {
            return false;
        }
        self.returned = None;
        true
    }

    pub fn terminal_is_empty(&self) -> bool {
        self.snapshot.is_none() && self.returned.is_none()
    }
}

pub fn next_entity_id(existing: impl Iterator<Item = EntityId>) -> EntityId {
    EntityId(existing.map(|id| id.0).max().unwrap_or(0).saturating_add(1))
}
pub fn artifact_kind() -> semio_framework_manifest_media::ArtifactKindSpec {
    use semio_framework_manifest_media::{MediaClass, MediaForm, MediaType, OsMediaCapability};
    semio_framework_manifest_media::ArtifactKindSpec {
        id: "data.model".into(),
        label: semio_ui_locale::LocalizedLabel::native("Energy Model", "Energiemodell"),
        source_format: ENERGY_MODEL_DOCUMENT_SCHEMA.into(),
        component_kind: "energy".into(),
        dimension: "data".into(),
        media_capability: OsMediaCapability::MeshOnly,
        media_type: MediaType { class: MediaClass::Data, form: MediaForm::Value },
        schema: ENERGY_MODEL_DOCUMENT_SCHEMA.into(),
        export_formats: vec![],
        import_formats: vec![],
        export_stdio_kinds: vec!["stdio.json".into(), "stdio.zip".into()],
        import_stdio_kinds: vec!["stdio.json".into(), "stdio.zip".into()],
    }
}
pub const FACET: semio_framework_schema_registry::FacetLeaves = semio_framework_schema_registry::FacetLeaves {
    rust: include_str!("🦀️.rs"),
    typescript: include_str!("🟦️.ts"),
    graphql: include_str!("🔗️.graphql"),
    json_schema: include_str!("🔣️.json"),
    proto: include_str!("🛰️.proto"),
};
#[path="🧳️state/🦀️.rs"]
pub mod state;

#[path="🧬️original/🦀️.rs"]
pub mod original;



#[path="🧬️original/📦️pack/🫴️receiving/🦀️.rs"]mod native_pack_receiving;
