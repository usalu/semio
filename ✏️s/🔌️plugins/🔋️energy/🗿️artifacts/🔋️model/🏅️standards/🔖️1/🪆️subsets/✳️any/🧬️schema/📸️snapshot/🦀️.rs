//! 🔋️ The public Energy document owns its sole native persistent model and original semantic sources.
#![allow(async_fn_in_trait)]
#![allow(clippy::too_many_arguments)]
extern crate semio_framework_os_kernel as store;
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

//#region 🗂️StructureOverview
/// 🗂️ The framework tree kit's sibling ceiling: `TreeWindowKit::render` collects each node's
/// children into a `UiFixedList<BuiltNode>` (`UI_FIXED_LIST_ITEMS` = 32) and the 33rd sibling
/// faults the whole assembly with `tree-window.siblings`, so the window renders EMPTY at runtime.
/// `Model` carries 35 collections — [`energy_structure_overview`] folds them into groups that stay
/// under this ceiling, and both the editor's and the viewer's `structure` windows read that one
/// grouping (the viewer may never import from the editor surface).
pub const STRUCTURE_TREE_SIBLING_CEILING: usize = 32;

/// 🗂️ One overview group of the structure tree: node id, English label, and the collections it
/// folds as `(node id, live element count)`.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct StructureOverviewGroup {
    pub id: &'static str,
    pub label: &'static str,
    pub counts: Vec<(&'static str, usize)>,
}

/// 🗂️ Every collection on [`Model`] exactly once, folded into seven domain groups, each with its
/// live element counts — the read overview both `structure` windows draw beneath `name`/`version`.
pub fn energy_structure_overview(model: &state::EnergyModelView<'_>) -> Vec<StructureOverviewGroup> {
    vec![
        StructureOverviewGroup {
            id: "geometry",
            label: "Geometry",
            counts: vec![
                ("zones", model.zones.len()),
                ("spaces", model.spaces.len()),
                ("surfaces", model.surfaces.len()),
                ("fenestrations", model.fenestrations.len()),
                ("shadingSurfaces", model.shading_surfaces.len()),
                ("spaceLists", model.space_lists.len()),
                ("thermalEnclosures", model.thermal_enclosures.len()),
                ("adjacencyPairs", model.adjacency_pairs.len()),
            ],
        },
        StructureOverviewGroup { id: "envelope", label: "Envelope", counts: vec![("materials", model.materials.len()), ("constructions", model.constructions.len())] },
        StructureOverviewGroup {
            id: "loads",
            label: "Internal loads & air exchange",
            counts: vec![
                ("people", model.people.len()),
                ("lighting", model.lighting.len()),
                ("equipment", model.equipment.len()),
                ("infiltrations", model.infiltrations.len()),
                ("mechanicalVentilations", model.mechanical_ventilations.len()),
            ],
        },
        StructureOverviewGroup { id: "controls", label: "Controls", counts: vec![("thermostats", model.thermostats.len()), ("humidistats", model.humidistats.len()), ("setpointManagers", model.setpoint_managers.len())] },
        StructureOverviewGroup {
            id: "hvac",
            label: "HVAC",
            counts: vec![
                ("idealLoads", model.ideal_loads.len()),
                ("zoneEquipment", model.zone_equipment.len()),
                ("airLoops", model.air_loops.len()),
                ("plantLoops", model.plant_loops.len()),
                ("outdoorAirSystems", model.outdoor_air_systems.len()),
            ],
        },
        StructureOverviewGroup {
            id: "systems",
            label: "Electrical, water & refrigeration",
            counts: vec![
                ("electricalLoadCenters", model.electrical_load_centers.len()),
                ("pvSystems", model.pv_systems.len()),
                ("batteryStorage", model.battery_storage.len()),
                ("shwSystems", model.shw_systems.len()),
                ("solarThermalSystems", model.solar_thermal_systems.len()),
                ("refrigerationSystems", model.refrigeration_systems.len()),
                ("waterSystems", model.water_systems.len()),
            ],
        },
        StructureOverviewGroup {
            id: "analysis",
            label: "Analysis & output",
            counts: vec![("faults", model.faults.len()), ("outputVariables", model.output_variables.len()), ("sizingObjects", model.sizing_objects.len()), ("daylightZones", model.daylight_zones.len()), ("roomAirModels", model.room_air_models.len())],
        },
    ]
}

/// 🌳️ The shared `structure` tree both surfaces render: `name`/`version` (the editor's two
/// `set-node` targets), the site line, then one node per overview group labeled with its total.
pub fn energy_structure_tree(model: &state::EnergyModelView<'_>) -> semio_framework_plugin_window_kits_rich::tree_kit::TreeView {
    use semio_framework_plugin_window_kits_rich::tree_kit::{TreeNodeView, TreeView};
    fn leaf(id: &str, label: String) -> TreeNodeView {
        TreeNodeView { id: id.into(), label, children: Vec::new() }
    }
    let mut children = vec![
        leaf("name", format!("Name: {}", model.name)),
        leaf("version", format!("Version: {}", model.version)),
        leaf("site", format!("Site: lat {:.2}°, lon {:.2}°, elev {:.1} m", model.site.latitude_deg, model.site.longitude_deg, model.site.elevation_m)),
    ];
    children.extend(energy_structure_overview(model).into_iter().map(|group| {
        let total: usize = group.counts.iter().map(|(_, count)| count).sum();
        TreeNodeView { id: group.id.into(), label: format!("{}: {total}", group.label), children: group.counts.iter().map(|(name, count)| leaf(name, format!("{name}: {count}"))).collect() }
    }));
    TreeView { roots: vec![TreeNodeView { id: "model".into(), label: format!("{} (v{})", model.name, model.version), children }] }
}
//#endregion 🗂️StructureOverview
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
        label: semio_framework_ui_locale::LocalizedLabel::native("Energy Model", "Energiemodell"),
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
