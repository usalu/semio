//! 🎪 Energy model artifact — headless BEM document surface over `crate::model::Model`.

#![allow(clippy::too_many_arguments)]

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_schema as framework_schema;

extern crate semio_s_plugin_energy_epw as epw;

//#region ⚡️SimulationEngine
extern crate semio_s_plugin_energy_engine_air_exchange as air_exchange;
extern crate semio_s_plugin_energy_engine_air_system as air_system;
extern crate semio_s_plugin_energy_engine_airflow_network as airflow_network;
extern crate semio_s_plugin_energy_engine_bestest as bestest;
extern crate semio_s_plugin_energy_engine_calendar as calendar;
extern crate semio_s_plugin_energy_engine_coils as coils;
extern crate semio_s_plugin_energy_engine_comfort as comfort;
extern crate semio_s_plugin_energy_engine_controls as controls;
extern crate semio_s_plugin_energy_engine_curves as curves;
extern crate semio_s_plugin_energy_engine_daylight as daylight;
extern crate semio_s_plugin_energy_engine_dispatch as dispatch;
extern crate semio_s_plugin_energy_engine_economics as economics;
extern crate semio_s_plugin_energy_engine_electrical as electrical;
extern crate semio_s_plugin_energy_engine_envelope as envelope;
extern crate semio_s_plugin_energy_engine_error as error;
extern crate semio_s_plugin_energy_engine_evaporative as evaporative;
extern crate semio_s_plugin_energy_engine_fans as fans;
extern crate semio_s_plugin_energy_engine_faults as faults;
extern crate semio_s_plugin_energy_engine_fenestration as fenestration;
extern crate semio_s_plugin_energy_engine_gains as gains;
extern crate semio_s_plugin_energy_engine_geometry as geometry;
extern crate semio_s_plugin_energy_engine_heat_recovery as heat_recovery;
extern crate semio_s_plugin_energy_engine_humidity_eq as humidity_eq;
extern crate semio_s_plugin_energy_engine_hvac_topo as hvac_topo;
extern crate semio_s_plugin_energy_engine_iaq as iaq;
extern crate semio_s_plugin_energy_engine_ideal_hvac as ideal_hvac;
extern crate semio_s_plugin_energy_engine_kernel as kernel;
extern crate semio_s_plugin_energy_engine_material as material;
extern crate semio_s_plugin_energy_engine_meters as meters;
extern crate semio_s_plugin_energy_engine_metrics as metrics;
extern crate semio_s_plugin_energy_engine_model as model;
extern crate semio_s_plugin_energy_engine_num as num;
extern crate semio_s_plugin_energy_engine_output as output;
extern crate semio_s_plugin_energy_engine_plant as plant;
extern crate semio_s_plugin_energy_engine_precompute as precompute;
extern crate semio_s_plugin_energy_engine_props as props;
extern crate semio_s_plugin_energy_engine_refrigeration as refrigeration;
extern crate semio_s_plugin_energy_engine_results as results;
extern crate semio_s_plugin_energy_engine_room_air as room_air;
extern crate semio_s_plugin_energy_engine_scene as scene;
extern crate semio_s_plugin_energy_engine_schedule as schedule;
extern crate semio_s_plugin_energy_engine_shw as shw;
extern crate semio_s_plugin_energy_engine_sim as sim;
extern crate semio_s_plugin_energy_engine_site as site;
extern crate semio_s_plugin_energy_engine_sizing as sizing;
extern crate semio_s_plugin_energy_engine_solar as solar;
extern crate semio_s_plugin_energy_engine_solar_thermal as solar_thermal;
extern crate semio_s_plugin_energy_engine_terminal as terminal;
extern crate semio_s_plugin_energy_engine_units as units;
extern crate semio_s_plugin_energy_engine_water as water;
extern crate semio_s_plugin_energy_engine_zone_air as zone_air;
extern crate semio_s_plugin_energy_engine_zone_hvac as zone_hvac;
//#endregion ⚡️SimulationEngine

/// ⚡️ Energy simulation tool run: definition vocabulary and run job.
#[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧵️simulation-session/🦀️.rs"]
pub mod energy_simulation_session;

use {semio_framework_plugin::ArtifactKindSpec,semio_framework_artifact_reference::Dialect,semio_framework_plugin::MediaClass,semio_framework_plugin::MediaForm,semio_framework_plugin::MediaType,semio_framework_plugin::OsMediaCapability,semio_framework_artifact_reference::StandardId,semio_framework_artifact_reference::SubsetId};

pub use crate::schema::EnergyModelArtifact;

/// 🔖️ Document schema / DSL envelope id.
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

//#region 🔖️Composition
/// 🧩️ Ticket 26/08/12/UNIFIED-COMPOSABLE-ARTIFACT-SYSTEM (`energy→C:value,table R:model`): the
/// old `model_json: String` opaque-JSON field (a hand-rolled duplicate of exactly what
/// `s.stdio.semio.value` already generalizes — a typed, structured value tree) is replaced by two
/// fixed composed CHILD slots on `EnergyModelSnapshot`: `structure` (the SOLE lossless source of
/// truth for the whole `crate::model::Model`, folded into one `SemioValue::Map` via a generic
/// JSON bridge — `Model` already derives `Serialize`/`Deserialize`, so `serde_json::Value` is a
/// real, lossless intermediate) and `zones` (a DERIVED, non-authoritative tabular projection —
/// one row per `Zone`, always regenerated alongside `structure` from the SAME model, never an
/// independent source, so the two never diverge — same "one lossless source + one derived
/// convenience projection" split `forms`'s `structure`/`results` and `mathematical`'s
/// `notation`/`results` established). `referenced_model` is a new forward `ArtifactLink` slot
/// (role `"model"`) for the building/spatial model this energy model analyzes — per the design
/// map's `R:model` half. Grepped before writing this: energy has NO existing dependency on any
/// external artifact (`ArtifactLink`/`link_slot` appear nowhere in this plugin today) — like
/// `layout`'s own `referenced_model` finding, this is genuinely NEW forward capability, not a
/// duplication removal, so it is schema/codec-complete but left inert (no mutation dispatch, no
/// resolver read path) — documented honestly rather than wired to a fictional consumer.
///
/// Genuine exception, NOT composed: `crate::model::Model`'s `Surface.vertices_m: Vec<[f64; 3]>`
/// (and `ShadingSurface.vertices_m`) is raw 3D geometry embedded inline, which in principle
/// duplicates what an external spatial model would own — but folding it out into `referenced_model`
/// would mean rewriting how every one of the ~40 engine modules under `🔨️modules/⚡️simulation/
/// ⚙️engine/` (envelope/daylight/solar/geometry/…) resolves surface geometry, from a plain struct
/// field read to a link-resolution round trip through a not-yet-wired `LinkResolver` seam (per the
/// migration recipe's §3 finding: `VcsArtifactApp.children` has zero live content behind it for
/// any plugin yet) — a kernel-dissolution-scale change (DKM's own ticket), not a schema migration.
/// `vertices_m` stays inside `structure`'s lossless `Model` tree, same as every other field.
//#region 🔖️ChildTypes
pub type EnergyStructureChild = store::os_store::artifact_child::ArtifactChild<semio_s_artifact_stdio_semio_value_model::schema::snapshot::SemioValueSnapshot>;
pub type EnergyZonesChild = store::os_store::artifact_child::ArtifactChild<semio_s_artifact_stdio_semio_table_model::schema::snapshot::SemioTableSnapshot>;
//#endregion 🔖️ChildTypes

//#region 🔖️Converters
/// 🌉 Generic, lossless `serde_json::Value` <-> `SemioValue` bridge — the SAME "both JSON-
/// equivalent" trade `forms`'s `semio_value_from_dsl`/`dsl_from_semio_value` makes for its own
/// JSON-shaped source. `crate::model::Model` has no byte-array or cross-artifact-ref fields, so
/// `SemioValue::Bytes`/`::Ref` are never PRODUCED by `energy_structure_from_model` below — they are
/// still handled (never a panic) for the theoretical case of a foreign composer writing one into
/// this artifact's own `structure` child.
///
/// 🌱️ NOT routed through `ToValue`/`DslValue` (ticket
/// `26/09/01/RUNTIME-DEPENDENCY-ELIMINATION-FOR-S-PLUGINS-AND-ARTIFACTS`), even though `Model`
/// itself now derives `ToValue`/`FromValue` (`🔨️modules/⚡️simulation/⚙️engine/🔋️model/🦀️.rs`):
/// `DslValue::Number` now carries the same `UInt`/`Int`/`Float` fidelity `serde_json::Value::Number`
/// does, so this bridge's original reason for staying on `serde_json` (`Model -> DslValue` losing
/// whether a leaf was an integer or a float field — the distinction `SemioValue::Int{lexeme}` vs
/// `SemioValue::Float{lexeme}` exists to carry, so a UI table cell or a JSON-schema-typed consumer
/// doesn't see `5.0` where the model held an `i32` `5`) no longer applies at the `DslValue` layer
/// itself; this bridge stays on `serde_json` regardless, out of scope for the ticket that gave
/// `DslValue::Number` that fidelity (`26/09/01/RUNTIME-DEPENDENCY-ELIMINATION-FOR-S-PLUGINS-AND-ARTIFACTS`).
fn semio_value_from_json(value: &serde_json::Value) -> semio_s_artifact_stdio_semio_value_model::schema::snapshot::SemioValue {
    use semio_s_artifact_stdio_semio_value_model::schema::snapshot::SemioValue;
    use semio_s_artifact_stdio_semio_value_model::schema::snapshot::SemioValueEntry;
    match value {
        serde_json::Value::Null => SemioValue::Null,
        serde_json::Value::Bool(value) => SemioValue::Bool { value: *value },
        serde_json::Value::Number(n) => {
            if n.is_i64() || n.is_u64() {
                SemioValue::Int { lexeme: n.to_string() }
            } else {
                SemioValue::Float { lexeme: n.to_string() }
            }
        }
        serde_json::Value::String(value) => SemioValue::Str { value: value.clone() },
        serde_json::Value::Array(items) => SemioValue::List { items: items.iter().map(semio_value_from_json).collect() },
        serde_json::Value::Object(entries) => SemioValue::Map { entries: entries.iter().map(|(key, value)| SemioValueEntry { key: key.clone(), value: semio_value_from_json(value) }).collect() },
    }
}

/// 🌉 Inverse of [`semio_value_from_json`] — real reconstruction, not a stub. `Bytes`/`Ref` degrade
/// honestly (documented above) since `Model` never produces either.
fn json_from_semio_value(value: &semio_s_artifact_stdio_semio_value_model::schema::snapshot::SemioValue) -> serde_json::Value {
    use semio_s_artifact_stdio_semio_value_model::schema::snapshot::SemioValue;
    match value {
        SemioValue::Null => serde_json::Value::Null,
        SemioValue::Bool { value } => serde_json::Value::Bool(*value),
        SemioValue::Int { lexeme } => lexeme.parse::<i64>().map_or_else(|_| serde_json::Value::String(lexeme.clone()), serde_json::Value::from),
        SemioValue::Float { lexeme } => lexeme.parse::<f64>().ok().and_then(serde_json::Number::from_f64).map_or_else(|| serde_json::Value::String(lexeme.clone()), serde_json::Value::Number),
        SemioValue::Str { value } => serde_json::Value::String(value.clone()),
        SemioValue::Bytes { value } => serde_json::Value::Array(value.iter().map(|b| serde_json::Value::from(*b)).collect()),
        SemioValue::List { items } => serde_json::Value::Array(items.iter().map(json_from_semio_value).collect()),
        SemioValue::Map { entries } => serde_json::Value::Object(entries.iter().map(|entry| (entry.key.clone(), json_from_semio_value(&entry.value))).collect()),
        SemioValue::Ref { .. } => serde_json::Value::Null,
    }
}

/// 🌉 REAL bidirectional converter: the whole `Model` <-> one `s.stdio.semio.value` tree — the SOLE
/// lossless source of truth for this artifact's persisted content.
pub fn energy_structure_from_model(model: &Model) -> semio_s_artifact_stdio_semio_value_model::schema::snapshot::SemioValueSnapshot {
    use semio_s_artifact_stdio_semio_value_model::schema::snapshot::SemioValueSnapshot;
    use semio_s_artifact_stdio_semio_value_model::schema::snapshot::STDIO_SEMIOVALUE_DOCUMENT_SCHEMA;
    let value = serde_json::to_value(model).unwrap_or(serde_json::Value::Null);
    SemioValueSnapshot { schema: STDIO_SEMIOVALUE_DOCUMENT_SCHEMA.into(), root: semio_value_from_json(&value), nodes: Vec::new() }
}

/// 🌉 Inverse of [`energy_structure_from_model`]. Falls back to `Model::default()` if `structure`'s
/// root doesn't decode into a full `Model` (e.g. a foreign composer wrote a partial/foreign tree) —
/// documented, honest degradation, never a panic, matching the recipe's converter-honesty rule.
pub fn energy_model_from_structure(structure: &semio_s_artifact_stdio_semio_value_model::schema::snapshot::SemioValueSnapshot) -> Model {
    serde_json::from_value(json_from_semio_value(&structure.root)).unwrap_or_default()
}

/// 🌉 REAL, DERIVED converter: one `s.stdio.semio.table` row per `Zone` (`id`/`name`/`volumeM3`/
/// `multiplier`/`conditioned`/`partOfTotalFloorArea`) — a non-authoritative convenience projection,
/// always regenerated alongside `structure` from the SAME model (never an independent source, so
/// the two never diverge). `energy_model_from_structure` alone is authoritative on read; this table
/// is never consulted for reconstruction, mirroring `forms`'s own `results` table exactly.
pub fn energy_zones_table_from_model(model: &Model) -> semio_s_artifact_stdio_semio_table_model::schema::snapshot::SemioTableSnapshot {
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
                    SemioValue::Str { value: zone.name.clone() },
                    SemioValue::Float { lexeme: format!("{}", zone.volume_m3) },
                    SemioValue::Int { lexeme: zone.multiplier.to_string() },
                    SemioValue::Bool { value: zone.conditioned },
                    SemioValue::Bool { value: zone.part_of_total_floor_area },
                ],
            })
            .collect(),
    }
}

/// 🔎️ Real read accessor for the composed `structure` child's actual content — derives it fresh
/// from the working scene's cached `Model` every call (never stored separately, so it cannot drift
/// from `structure`'s handle). No render/export call site consumes this yet (energy is a headless
/// engine with no document app — see `🦀️.rs`'s own "Shape note"), matching `layout`'s honest
/// framing for its own inert `referenced_model` slot: real, tested, not yet wired to a consumer.
pub fn energy_structure_content(snapshot: &EnergyModelSnapshot) -> semio_s_artifact_stdio_semio_value_model::schema::snapshot::SemioValueSnapshot {
    energy_structure_from_model(&snapshot.model)
}

/// 🔎️ Twin of [`energy_structure_content`] for the `zones` child.
pub fn energy_zones_content(snapshot: &EnergyModelSnapshot) -> semio_s_artifact_stdio_semio_table_model::schema::snapshot::SemioTableSnapshot {
    energy_zones_table_from_model(&snapshot.model)
}
//#endregion 🔖️Converters

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
pub fn energy_structure_overview(model: &Model) -> Vec<StructureOverviewGroup> {
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
pub fn energy_structure_tree(model: &Model) -> semio_framework_plugin::app::TreeView {
    use semio_framework_plugin::app::{TreeNodeView, TreeView};
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

//#region 🔖️RetainedArtifactState
/// 🧵️ Exact immutable store owner used by mounted Energy capture. It exposes only a borrowed Model,
/// revalidates the store generation immediately before every read, and returns the precise snapshot
/// lease witness to the store retirement pump on close.
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

    pub fn model(&self) -> Option<&Model> {
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

/// 🧷️ Mints stable composed-child handles. The authoritative numerical model is owned directly by
/// the event-sourced snapshot and therefore travels through the store's generation-qualified
/// `SnapshotRead` lease; no process-local cache participates in reads or admission.
///
/// 🪪️ Each handle's `child_id` IS its target's `artifact_id` (`energy-value` / `energy-table`), and
/// the two differ: the store's `ChildRestoreProjection` (the first thing `ArtifactApp` runs on its
/// initial snapshot) refuses a handle whose `child_id != artifact_id` with `InvalidReference` and a
/// second handle carrying an already-seen `child_id` with `DuplicateChild` — one shared
/// `"energy-model"` id for both slots faulted every guest at `genesis_child_pack` (ticket
/// 26/09/06/ENERGY-PLUGIN-END-TO-END, 2026-09-16), before a single window could render. Same shape
/// as `forms_children_from_steps`.
pub fn energy_children_from_model(_model: &Model) -> (EnergyStructureChild, EnergyZonesChild) {
    let dialect_for = |subset: &str| semio_framework_artifact_reference::ArtifactDialect { artifact_kind: "s.stdio.semio".into(), standard: "v1".into(), subset: subset.into() };
    let target_for = |subset: &str| semio_framework_artifact_reference::ArtifactRef { artifact_id: format!("energy-{subset}"), dialect: dialect_for(subset) };
    let (structure, zones) = (target_for("value"), target_for("table"));
    (store::os_store::artifact_child::ArtifactChild::new(structure.artifact_id.clone(), structure), store::os_store::artifact_child::ArtifactChild::new(zones.artifact_id.clone(), zones))
}

/// 🔎️ Compatibility value accessor for artifact editing and inference. Mounted simulation never
/// calls this whole-owner clone; it captures incrementally from an exact `SnapshotRead` lease.
pub fn energy_model(snapshot: &EnergyModelSnapshot) -> Model {
    snapshot.model.clone()
}

/// 🏗️ Builds a full `EnergyModelSnapshot` from a literal `Model` — the standard fixture/import
/// constructor replacing the old `model_json: String` struct literal now that `structure`/`zones`
/// are composed child handles, not a plain field.
pub fn energy_snapshot_with_state(schema: impl Into<String>, model: &Model, referenced_model: Option<store::ArtifactLink>) -> EnergyModelSnapshot {
    let (structure, zones) = energy_children_from_model(model);
    EnergyModelSnapshot { schema: schema.into(), model: model.clone(), structure, zones, referenced_model, weather_link: None }
}
//#endregion 🔖️RetainedArtifactState

//#region 🌱️GenesisChildren
/// 🌱️ `ArtifactApp::genesis_child_pack` for both composed children. Each is a pure function of
/// `snapshot.model` (`energy_structure_from_model` / `energy_zones_table_from_model`), so a fresh
/// boot and every whole-document load (`Effect::LoadDocument` → the host's `loadDocumentArchive`
/// with `members: []`, which is how `setActiveExample` lands) re-derive them here instead of
/// shipping them — without this the archive closure is `Incomplete` and every example load faults.
/// Answers only for the slot/child pairs the snapshot itself declares (same shape as
/// `genesis_gis_map_child_pack`).
pub fn energy_genesis_child_pack(snapshot: &EnergyModelSnapshot, slot: &str, child_id: &str) -> Option<Vec<u8>> {
    use pack::snapshot::ArtifactPack;
    if slot == "structure" && child_id == snapshot.structure.child_id {
        return Some(<semio_s_artifact_stdio_semio_value_model::schema::snapshot::SemioValueSnapshot as ArtifactPack>::encode_pack(&energy_structure_from_model(&snapshot.model)));
    }
    if slot == "zones" && child_id == snapshot.zones.child_id {
        return Some(<semio_s_artifact_stdio_semio_table_model::schema::snapshot::SemioTableSnapshot as ArtifactPack>::encode_pack(&energy_zones_table_from_model(&snapshot.model)));
    }
    None
}

/// 🧬️ The bounded projection of this snapshot's two child handles, as both surfaces hand it to the
/// framework (`ArtifactEditor::child_restore_projection` / `ArtifactViewer::child_restore_projection`).
pub fn energy_child_restore_projection(snapshot: &EnergyModelSnapshot) -> Result<store::ChildRestoreProjection<'_>, semio_framework_plugin::Fault> {
    store::ChildRestoreProjection::from_snapshot(snapshot).map_err(|error| semio_framework_plugin::Fault::from(format!("energy model child projection failed: {error}")))
}
//#endregion 🌱️GenesisChildren

//#endregion 🔖️Composition

//#region 🆔️EntityIdMinting
/// 🆔️ Mints the next free [`crate::model::EntityId`] for one id-addressed collection — `max + 1`, so
/// an id is never REUSED after a delete and every dangling reference elsewhere in the document stays
/// unambiguous rather than silently re-pointing at a new entity (`📓️explore-mutation-vocabulary.md`
/// §5.3, §5.4).
///
/// This is the caller's job, not the mutation's: every `create-*` kind takes the id it must use as
/// part of its payload, because a mutation that minted its own id would not be a pure function of
/// `(snapshot, payload)` and its inverse could not name what to delete. One helper, at the artifact
/// root rather than in one surface, so the editor and every `create-*` group share the convention.
///
/// `EntityId(0)` is never minted: an empty collection starts at 1, which keeps 0 available as the
/// "no entity" sentinel every wire form already reads it as.
pub fn next_entity_id(existing: impl Iterator<Item = EntityId>) -> EntityId {
    EntityId(existing.map(|id| id.0).max().unwrap_or(0).saturating_add(1))
}
//#endregion 🆔️EntityIdMinting

//#region ♻️WholeDocumentLoad
/// 🔗️ The two link slots a whole-document load carries forward or clears. A struct rather than two
/// positional `Option<ArtifactLink>` arguments, because both have the same type and swapping them at
/// a call site would type-check in silence.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct EnergyModelLinkSlots {
    pub referenced_model: Option<store::ArtifactLink>,
    pub weather_link: Option<store::ArtifactLink>,
}

impl EnergyModelLinkSlots {
    /// 🔗️ The slots as a snapshot currently holds them — what re-opening the same document keeps.
    pub fn of(snapshot: &EnergyModelSnapshot) -> Self {
        Self { referenced_model: snapshot.referenced_model.clone(), weather_link: snapshot.weather_link.clone() }
    }
}

/// 📸️ A genesis snapshot for a whole-document load: the model plus whichever link slots the caller
/// carries over. `energy_snapshot_with_state` cannot express the weather slot, so every whole-load
/// site goes through this instead of setting the field back up by hand.
pub fn energy_snapshot_with_links(model: &Model, links: &EnergyModelLinkSlots) -> EnergyModelSnapshot {
    let mut snapshot = energy_snapshot_with_state(ENERGY_MODEL_DOCUMENT_SCHEMA, model, links.referenced_model.clone());
    snapshot.weather_link = links.weather_link.clone();
    snapshot
}

/// ♻️ The one sanctioned whole-document LOAD for this artifact — file open, `.energy`/epJSON import
/// and example load all route through here, and none of them is a mutation: ticket
/// `26/08/12/SEMANTIC-MUTATIONS-DIRECT-LEAF-OVERHAUL`'s derivation rule 6 forbids a whole-document
/// `replace` kind, which is why `♻️replace-model` is gone from this vocabulary entirely.
///
/// The effect carries a genesis `(pack, spr)` pair; the host loops it back as a document archive load
/// (`AppCommand::LoadDocumentArchive`), whose commit is what actually replaces the store — a guest plugin
/// never holds the store itself. History
/// therefore starts empty by construction: loading a second document is no more undoable back into
/// the first than opening a second file is. Same shape as `📐️cad`'s `reset_document_effect` and
/// `🔱️trinity`'s; a freshly minted, edit-free envelope makes the spr encode infallible.
pub fn energy_model_load_document_effect(document_id: &str, model: &Model, links: &EnergyModelLinkSlots) -> semio_framework_plugin::kernel::Effect {
    let snapshot = energy_snapshot_with_links(model, links);
    let pack = <EnergyModelSnapshot as pack::snapshot::ArtifactPack>::encode_pack(&snapshot);
    let envelope = store::os_store::document_materialize::create_document_envelope::<EnergyModelSnapshot, EnergyModelMutation>(ENERGY_MODEL_DOCUMENT_SCHEMA, document_id, snapshot, None).into_owners();
    let spr = ::semio_framework_async::poll::resolve_ready(store::os_store::text_format::print_document_spr(&envelope)).expect("energy model document spr encode is infallible for a fresh, edit-free envelope");
    semio_framework_plugin::kernel::Effect::LoadDocument { pack, spr }
}
//#endregion ♻️WholeDocumentLoad

//#region 🔖️ArtifactKind
/// 🗂️ This artifact's `ArtifactKindSpec` — Data × Value per owner-table (`data.model`).
pub fn artifact_kind() -> ArtifactKindSpec {
    ArtifactKindSpec {
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
//#endregion 🔖️ArtifactKind

//#region 🔖️Declaration
/// 🔖️ This artifact's declaration (ticket 26/08/12/ARTIFACTS-ONLY-PLUGIN-ARCHITECTURE M1, relocated
/// off `⚙️engine` to the artifact root — `declaration()` describes the artifact itself, not engine
/// behaviour) — replaces the old side-effecting `register()`, which the plugin root called
/// unconditionally (energy has no document apps, so there was never a `.setup()` narrowing here to
/// begin with). The retired root-level registrar only called `register_composer_entries(v1::entries())`
/// — exactly what `.composers(...)` below now does through `register_all` — so no imperative alias
/// remains. `.composers(...)` reaches `🚪️io/🦀️.rs`'s own `io_registry` module (the one with
/// the actual `ComposerEntry` rows) by its full path —
/// ticket 26/08/12/ENGINELESS-ARTIFACTS-AND-APP-STATE-MACHINES relocated it off the now-deleted
/// `⚙️engine` (an artifact is a schema + io system, never an engine) into `🚪️io/`, updating this
/// qualified reference in lockstep.
/// `register_document_codec()` — folded into this declaration via `.document_codec_bare::<Snapshot,
/// Mutation>(schema)` (ticket 26/08/12/ARTIFACTS-ONLY-PLUGIN-ARCHITECTURE W1d): `.document_codec::<A:
/// ArtifactApp>()` requires an `ArtifactApp` to bind `A::Snapshot`/`A::Mutation`, and this plugin is a
/// headless library with ZERO apps — there is no `ArtifactApp` to name. `document_codec_bare` is the
/// new sibling closing exactly that gap (see its own doc); the old free fn in `⚙️engine` is deleted
/// with this — nothing else called it.
pub fn definition() -> Result<semio_framework_plugin::ArtifactDefinition, semio_framework_plugin::ArtifactDefinitionError> {
    use semio_framework_plugin::{ArtifactCapability, ArtifactCapabilityKind, ArtifactDefinition, ArtifactIdentity, ArtifactIdentityClaim, ArtifactIdentityNamespace, ArtifactLocale, ArtifactLocalization};

    type CapabilityRow<'a> = (&'a str, &'a str, &'a str, &'a [(&'a str, &'a str)], Option<(&'a str, &'a str)>);
    let rows: &[CapabilityRow<'_>] = &[
        ("s.energy.model.standard.v1", "standard", "1", &[], None),
        ("s.energy.model.standard.v1.profile.any", "profile", "any", &[], None),
        ("s.energy.model.schema.artifact", "schema", "s.energy.model", &[("schema", "s.energy.model")], None),
        ("s.energy.model.inference.artifact", "inference", "s.energy.model.inference", &[("schema", "s.energy.model.inference")], None),
        ("s.energy.model.composer.native", "composer", "s.energy.model@1/*", &[("dialect", "s.energy.model@1/*")], None),
        ("s.energy.model.composer.format-1", "composer", "s.stdio.zip@2.0/*", &[("dialect", "s.stdio.zip@2.0/*")], None),
        ("s.energy.model.composer.format-4", "composer", "s.stdio.json@rfc8259/*", &[("dialect", "s.stdio.json@rfc8259/*")], None),
        ("s.energy.model.grammar.1", "grammar", "energy.model", &[("grammar", "energy.model")], None),
        ("s.energy.model.grammar.2", "grammar", "energy.model.op", &[("grammar", "energy.model.op")], None),
        ("s.energy.model.grammar.3", "grammar", "energy.model.diff", &[("grammar", "energy.model.diff")], None),
        ("s.energy.model.grammar.4", "grammar", "energy.model.pack", &[("grammar", "energy.model.pack")], None),
        ("s.energy.model.grammar.5", "grammar", "energy.model.spr", &[("grammar", "energy.model.spr")], None),
        ("s.energy.model.codec.document-1", "codec", "energy.model:energy", &[("codec", "energy.model"), ("codec-extension", "12:energy.model:energy")], None),
        ("s.energy.model.localization.en", "localization", "Energy Model", &[], Some(("en", "Energy Model"))),
        ("s.energy.model.localization.de", "localization", "Energiemodell", &[], Some(("de", "Energiemodell"))),
    ];
    let mut definition = ArtifactDefinition::new(ArtifactIdentity::parse("s.energy.model")?);
    for (identity, kind, descriptor, claims, localization) in rows {
        let mut capability = ArtifactCapability::new(ArtifactIdentity::parse(*identity)?, ArtifactCapabilityKind::parse(*kind)?).descriptor(descriptor.as_bytes())?;
        for (namespace, value) in *claims {
            capability = capability.claim(ArtifactIdentityClaim::new(ArtifactIdentityNamespace::parse(*namespace)?, *value)?)?;
        }
        if let Some((locale, text)) = localization {
            capability = capability.localization(ArtifactLocalization::new(ArtifactLocale::parse(*locale)?, *text)?)?;
        }
        definition = definition.capability(capability)?;
    }
    Ok(definition)
}

pub fn declaration() -> Result<semio_framework_plugin::ArtifactDeclaration, semio_framework_plugin::ArtifactDefinitionError> {
    semio_framework_plugin::ArtifactDeclaration::builder(definition()?)
        .schema(standards::v1::subsets::any::schema::energy_model_artifact_schema_descriptor())
        .inferences([standards::v1::subsets::any::schema::inferences::energy_model_artifact_inference_descriptor()])
        .composers(standards::v1::subsets::any::io::io_registry::entries())
        .languages(pilot_languages())
        .document_codec_bare::<EnergyModelSnapshot, EnergyModelMutation>(ENERGY_MODEL_DOCUMENT_SCHEMA, Dialect { artifact_kind: "s.energy.model", standard: StandardId("1"), subset: SubsetId("*") })
        .try_build()
}

/// 📌️ Handcrafted facet grammars (text) and protocols (binary) for in-process execution — built once
/// and leaked to a `&'static` slice since `dsl::passthrough_hooks` isn't `const fn`, mirroring the
/// `🗒️note` exemplar's helper of the same shape.
fn pilot_languages() -> &'static [semio_framework_dsl::LanguageSpec] {
    static LANGUAGES: std::sync::OnceLock<Vec<semio_framework_dsl::LanguageSpec>> = std::sync::OnceLock::new();
    LANGUAGES
        .get_or_init(|| {
            vec![
                semio_framework_dsl::LanguageSpec {
                    id: "energy.model",
                    extension: Some("energy"),
                    role: semio_framework_dsl::LanguageRole::Document,
                    grammar: Some(standards::v1::subsets::any::io::text::snapshot::COMPONENT_GRAMMAR_SEMIO),
                    grammar_path: Some(standards::v1::subsets::any::io::text::snapshot::COMPONENT_GRAMMAR_PATH),
                    protocol: Some(standards::v1::subsets::any::io::binary::snapshot::COMPONENT_PROTOCOL_SEMIO),
                    protocol_path: Some(standards::v1::subsets::any::io::binary::snapshot::COMPONENT_PROTOCOL_PATH),
                    hooks: semio_framework_dsl::passthrough_hooks("energy.model"),
                },
                semio_framework_dsl::LanguageSpec {
                    id: "energy.model.op",
                    extension: None,
                    role: semio_framework_dsl::LanguageRole::Ops,
                    grammar: Some(standards::v1::subsets::any::io::text::mutations::COMPONENT_GRAMMAR_SEMIO),
                    grammar_path: Some(standards::v1::subsets::any::io::text::mutations::COMPONENT_GRAMMAR_PATH),
                    protocol: Some(standards::v1::subsets::any::io::binary::mutations::COMPONENT_PROTOCOL_SEMIO),
                    protocol_path: Some(standards::v1::subsets::any::io::binary::mutations::COMPONENT_PROTOCOL_PATH),
                    hooks: semio_framework_dsl::passthrough_hooks("energy.model.op"),
                },
                semio_framework_dsl::LanguageSpec {
                    id: "energy.model.diff",
                    extension: None,
                    role: semio_framework_dsl::LanguageRole::Diff,
                    grammar: Some(standards::v1::subsets::any::io::text::diff::COMPONENT_GRAMMAR_SEMIO),
                    grammar_path: Some(standards::v1::subsets::any::io::text::diff::COMPONENT_GRAMMAR_PATH),
                    protocol: None,
                    protocol_path: None,
                    hooks: semio_framework_dsl::passthrough_hooks("energy.model.diff"),
                },
                semio_framework_dsl::LanguageSpec {
                    id: "energy.model.pack",
                    extension: None,
                    role: semio_framework_dsl::LanguageRole::Pack,
                    grammar: None,
                    grammar_path: None,
                    protocol: Some(standards::v1::subsets::any::io::binary::snapshot::COMPONENT_PROTOCOL_SEMIO),
                    protocol_path: Some(standards::v1::subsets::any::io::binary::snapshot::COMPONENT_PROTOCOL_PATH),
                    hooks: semio_framework_dsl::passthrough_hooks("energy.model.pack"),
                },
                semio_framework_dsl::LanguageSpec {
                    id: "energy.model.spr",
                    extension: None,
                    role: semio_framework_dsl::LanguageRole::Spr,
                    grammar: None,
                    grammar_path: None,
                    protocol: Some(standards::v1::subsets::any::io::binary::mutations::COMPONENT_PROTOCOL_SEMIO),
                    protocol_path: Some(standards::v1::subsets::any::io::binary::mutations::COMPONENT_PROTOCOL_PATH),
                    hooks: semio_framework_dsl::passthrough_hooks("energy.model.spr"),
                },
            ]
        })
        .as_slice()
}
//#endregion 🔖️Declaration

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️artifact-root/🦀️.rs"]
mod artifact_root_tests;
//#endregion 🧪️Tests

#[path = "."]
pub mod standards {
    #[path = "."]
    pub mod v1 {
        #[path = "."]
        pub mod subsets {
            #[path = "."]
            pub mod any {
                #[path = "."]
                pub mod schema {
                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🦀️.rs"]
                    mod component;
                    pub use component::*;
                    #[path = "."]
                    pub mod snapshot {
                        #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🦀️.rs"]
                        mod component;
                        pub use component::*;
                    }
                    #[path = "."]
                    pub mod inferences {
                        #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/🦀️.rs"]
                        mod component;
                        pub use component::*;
                        #[path = "."]
                        pub mod entries {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/🗃️entries/🦀️.rs"]
                            mod component;
                            pub use component::*;
                        }
                    }
                    #[path = "."]
                    pub mod diff {
                        #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔺️diff/🦀️.rs"]
                        mod component;
                        pub use component::*;
                    }
                    #[path = "."]
                    pub mod mutations {
                        #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️.rs"]
                        mod component;
                        pub use component::*;
                        #[path = "."]
                        pub mod rename_model {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔋️model/🏷️rename/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔋️model/🏷️rename/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔋️model/🏷️rename/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔋️model/🏷️rename/🧪️tests/⛔️refuses/🦀️.rs"]
                            mod tests_refuses_a_blank_name;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔋️model/🏷️rename/🧪️tests/✅️renames-the-model/🦀️.rs"]
                            mod tests_renames_the_model;
                        }
                        #[path = "."]
                        pub mod change_model_version {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔋️model/🔢️change-version/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔋️model/🔢️change-version/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔋️model/🔢️change-version/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔋️model/🔢️change-version/🧪️tests/✅️bumps/🦀️.rs"]
                            mod tests_bumps_the_version;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔋️model/🔢️change-version/🧪️tests/⛔️refuses/🦀️.rs"]
                            mod tests_refuses_a_blank_version;
                        }
                        #[path = "."]
                        pub mod change_site_latitude {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📍️site/🌍️change-latitude/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📍️site/🌍️change-latitude/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📍️site/🌍️change-latitude/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📍️site/🌍️change-latitude/🧪️tests/✅️sets/🦀️.rs"]
                            mod tests_sets;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📍️site/🌍️change-latitude/🧪️tests/⛔️refuses/🦀️.rs"]
                            mod tests_refuses;
                        }
                        #[path = "."]
                        pub mod change_site_longitude {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📍️site/🌏️change-longitude/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📍️site/🌏️change-longitude/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📍️site/🌏️change-longitude/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📍️site/🌏️change-longitude/🧪️tests/✅️sets/🦀️.rs"]
                            mod tests_sets;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📍️site/🌏️change-longitude/🧪️tests/⛔️refuses/🦀️.rs"]
                            mod tests_refuses;
                        }
                        #[path = "."]
                        pub mod change_site_elevation {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📍️site/⛰️change-elevation/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📍️site/⛰️change-elevation/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📍️site/⛰️change-elevation/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📍️site/⛰️change-elevation/🧪️tests/✅️sets/🦀️.rs"]
                            mod tests_sets;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📍️site/⛰️change-elevation/🧪️tests/⛔️refuses/🦀️.rs"]
                            mod tests_refuses;
                        }
                        #[path = "."]
                        pub mod change_site_time_zone {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📍️site/🕰️change-time-zone/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📍️site/🕰️change-time-zone/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📍️site/🕰️change-time-zone/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📍️site/🕰️change-time-zone/🧪️tests/✅️sets/🦀️.rs"]
                            mod tests_sets;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📍️site/🕰️change-time-zone/🧪️tests/⛔️refuses/🦀️.rs"]
                            mod tests_refuses;
                        }
                        #[path = "."]
                        pub mod change_site_north_axis {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📍️site/🔝️change-north-axis/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📍️site/🔝️change-north-axis/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📍️site/🔝️change-north-axis/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📍️site/🔝️change-north-axis/🧪️tests/✅️sets/🦀️.rs"]
                            mod tests_sets;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📍️site/🔝️change-north-axis/🧪️tests/🟰️same/🦀️.rs"]
                            mod tests_same;
                        }
                        #[path = "."]
                        pub mod change_ground_building {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📍️site/🌡️change-ground-building/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📍️site/🌡️change-ground-building/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📍️site/🌡️change-ground-building/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📍️site/🌡️change-ground-building/🧪️tests/✅️sets/🦀️.rs"]
                            mod tests_sets;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📍️site/🌡️change-ground-building/🧪️tests/⛔️refuses/🦀️.rs"]
                            mod tests_refuses;
                        }
                        #[path = "."]
                        pub mod change_ground_shallow {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📍️site/🌱️change-ground-shallow/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📍️site/🌱️change-ground-shallow/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📍️site/🌱️change-ground-shallow/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📍️site/🌱️change-ground-shallow/🧪️tests/✅️sets/🦀️.rs"]
                            mod tests_sets;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📍️site/🌱️change-ground-shallow/🧪️tests/⛔️refuses/🦀️.rs"]
                            mod tests_refuses;
                        }
                        #[path = "."]
                        pub mod change_ground_deep {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📍️site/⛏️change-ground-deep/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📍️site/⛏️change-ground-deep/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📍️site/⛏️change-ground-deep/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📍️site/⛏️change-ground-deep/🧪️tests/✅️sets/🦀️.rs"]
                            mod tests_sets;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📍️site/⛏️change-ground-deep/🧪️tests/⛔️refuses/🦀️.rs"]
                            mod tests_refuses;
                        }
                        #[path = "."]
                        pub mod change_run_start_month {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/⏯️run/🛫️change-start-month/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/⏯️run/🛫️change-start-month/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/⏯️run/🛫️change-start-month/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/⏯️run/🛫️change-start-month/🧪️tests/✅️sets/🦀️.rs"]
                            mod tests_sets;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/⏯️run/🛫️change-start-month/🧪️tests/⛔️refuses/🦀️.rs"]
                            mod tests_refuses;
                        }
                        #[path = "."]
                        pub mod change_run_start_day {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/⏯️run/▶️change-start-day/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/⏯️run/▶️change-start-day/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/⏯️run/▶️change-start-day/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/⏯️run/▶️change-start-day/🧪️tests/✅️sets/🦀️.rs"]
                            mod tests_sets;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/⏯️run/▶️change-start-day/🧪️tests/⛔️refuses/🦀️.rs"]
                            mod tests_refuses;
                        }
                        #[path = "."]
                        pub mod change_run_end_month {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/⏯️run/🛬️change-end-month/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/⏯️run/🛬️change-end-month/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/⏯️run/🛬️change-end-month/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/⏯️run/🛬️change-end-month/🧪️tests/✅️sets/🦀️.rs"]
                            mod tests_sets;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/⏯️run/🛬️change-end-month/🧪️tests/⛔️refuses/🦀️.rs"]
                            mod tests_refuses;
                        }
                        #[path = "."]
                        pub mod change_run_end_day {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/⏯️run/⏹️change-end-day/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/⏯️run/⏹️change-end-day/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/⏯️run/⏹️change-end-day/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/⏯️run/⏹️change-end-day/🧪️tests/✅️sets/🦀️.rs"]
                            mod tests_sets;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/⏯️run/⏹️change-end-day/🧪️tests/⛔️refuses/🦀️.rs"]
                            mod tests_refuses;
                        }
                        #[path = "."]
                        pub mod change_run_year {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/⏯️run/📅️change-year/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/⏯️run/📅️change-year/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/⏯️run/📅️change-year/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/⏯️run/📅️change-year/🧪️tests/✅️sets/🦀️.rs"]
                            mod tests_sets;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/⏯️run/📅️change-year/🧪️tests/⛔️refuses/🦀️.rs"]
                            mod tests_refuses;
                        }
                        #[path = "."]
                        pub mod replace_airflow_network {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔋️model/🫧️replace-airflow-network/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔋️model/🫧️replace-airflow-network/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔋️model/🫧️replace-airflow-network/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔋️model/🫧️replace-airflow-network/🧪️tests/✅️attaches/🦀️.rs"]
                            mod tests_attaches_a_network;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔋️model/🫧️replace-airflow-network/🧪️tests/⛔️refuses/🦀️.rs"]
                            mod tests_refuses_unpaired_nodes;
                        }
                        #[path = "."]
                        pub mod add_output_variable {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔋️model/📊️add-output-variable/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔋️model/📊️add-output-variable/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔋️model/📊️add-output-variable/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔋️model/📊️add-output-variable/🧪️tests/✅️adds/🦀️.rs"]
                            mod tests_adds_zone_air_temp;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔋️model/📊️add-output-variable/🧪️tests/⛔️refuses/🦀️.rs"]
                            mod tests_refuses_a_duplicate;
                        }
                        #[path = "."]
                        pub mod remove_output_variable {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔋️model/📉️remove-output-variable/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔋️model/📉️remove-output-variable/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔋️model/📉️remove-output-variable/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔋️model/📉️remove-output-variable/🧪️tests/✅️drops/🦀️.rs"]
                            mod tests_drops_zone_air_temp;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔋️model/📉️remove-output-variable/🧪️tests/⛔️refuses/🦀️.rs"]
                            mod tests_refuses_an_absent_one;
                        }
                        #[path = "."]
                        pub mod bind_weather_file {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔋️model/🌦️bind-weather-file/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔋️model/🌦️bind-weather-file/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔋️model/🌦️bind-weather-file/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔋️model/🌦️bind-weather-file/🧪️tests/✅️binds/🦀️.rs"]
                            mod tests_binds_hannover_epw;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔋️model/🌦️bind-weather-file/🧪️tests/⛔️refuses/🦀️.rs"]
                            mod tests_refuses_a_bad_uri;
                        }
                        #[path = "."]
                        pub mod unbind_weather_file {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔋️model/🌤️unbind-weather-file/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔋️model/🌤️unbind-weather-file/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔋️model/🌤️unbind-weather-file/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔋️model/🌤️unbind-weather-file/🧪️tests/⛔️refuses/🦀️.rs"]
                            mod tests_refuses_when_unbound;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔋️model/🌤️unbind-weather-file/🧪️tests/✅️unbinds/🦀️.rs"]
                            mod tests_unbinds_the_weather;
                        }
                        #[path = "."]
                        pub mod connect_referenced_model {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔋️model/🪢️connect-referenced-model/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔋️model/🪢️connect-referenced-model/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔋️model/🪢️connect-referenced-model/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔋️model/🪢️connect-referenced-model/🧪️tests/✅️connects/🦀️.rs"]
                            mod tests_connects_the_geometry;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔋️model/🪢️connect-referenced-model/🧪️tests/⛔️refuses/🦀️.rs"]
                            mod tests_refuses_a_bad_uri;
                        }
                        #[path = "."]
                        pub mod disconnect_referenced_model {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔋️model/✂️disconnect-referenced-model/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔋️model/✂️disconnect-referenced-model/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔋️model/✂️disconnect-referenced-model/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔋️model/✂️disconnect-referenced-model/🧪️tests/✅️disconnects/🦀️.rs"]
                            mod tests_disconnects_the_geometry;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔋️model/✂️disconnect-referenced-model/🧪️tests/⛔️refuses/🦀️.rs"]
                            mod tests_refuses_when_absent;
                        }
                        #[path = "."]
                        pub mod rename_zone {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏡️zone/🏠️rename/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏡️zone/🏠️rename/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏡️zone/🏠️rename/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏡️zone/🏠️rename/🧪️tests/⛔️refuses/🦀️.rs"]
                            mod tests_refuses_a_missing_zone;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏡️zone/🏠️rename/🧪️tests/✅️renames-zone-one/🦀️.rs"]
                            mod tests_renames_zone_one;
                        }
                        #[path = "."]
                        pub mod change_zone_volume {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏡️zone/🧊️change-volume/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏡️zone/🧊️change-volume/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏡️zone/🧊️change-volume/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏡️zone/🧊️change-volume/🧪️tests/⛔️refuses/🦀️.rs"]
                            mod tests_refuses_zero_volume;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏡️zone/🧊️change-volume/🧪️tests/✅️resizes/🦀️.rs"]
                            mod tests_resizes_zone_one;
                        }
                        #[path = "."]
                        pub mod change_zone_multiplier {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏡️zone/✖️change-multiplier/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏡️zone/✖️change-multiplier/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏡️zone/✖️change-multiplier/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏡️zone/✖️change-multiplier/🧪️tests/⛔️refuses/🦀️.rs"]
                            mod tests_refuses_zero_instances;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏡️zone/✖️change-multiplier/🧪️tests/✅️stacks/🦀️.rs"]
                            mod tests_stacks_four_storeys;
                        }
                        #[path = "."]
                        pub mod change_zone_conditioned {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏡️zone/🌬️change-conditioned/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏡️zone/🌬️change-conditioned/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏡️zone/🌬️change-conditioned/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏡️zone/🌬️change-conditioned/🧪️tests/✅️frees/🦀️.rs"]
                            mod tests_frees_the_zone;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏡️zone/🌬️change-conditioned/🧪️tests/⛔️refuses/🦀️.rs"]
                            mod tests_refuses_a_missing_zone;
                        }
                        #[path = "."]
                        pub mod change_zone_floor_area_participation {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏡️zone/📐️change-floor-area-participation/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏡️zone/📐️change-floor-area-participation/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏡️zone/📐️change-floor-area-participation/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏡️zone/📐️change-floor-area-participation/🧪️tests/✅️excludes/🦀️.rs"]
                            mod tests_excludes_the_zone;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏡️zone/📐️change-floor-area-participation/🧪️tests/⛔️refuses/🦀️.rs"]
                            mod tests_refuses_a_missing_zone;
                        }
                        #[path = "."]
                        pub mod create_zone {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏡️zone/🏘️create/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏡️zone/🏘️create/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏡️zone/🏘️create/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏡️zone/🏘️create/🧪️tests/✅️adds-a-second-zone/🦀️.rs"]
                            mod tests_adds_a_second_zone;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏡️zone/🏘️create/🧪️tests/⛔️refuses-a-taken-id/🦀️.rs"]
                            mod tests_refuses_a_taken_id;
                        }
                        #[path = "."]
                        pub mod delete_zone {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏡️zone/🏚️delete/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏡️zone/🏚️delete/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏡️zone/🏚️delete/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏡️zone/🏚️delete/🧪️tests/✅️deletes-a-free-zone/🦀️.rs"]
                            mod tests_deletes_a_free_zone;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏡️zone/🏚️delete/🧪️tests/⛔️refuses-a-used-zone/🦀️.rs"]
                            mod tests_refuses_a_used_zone;
                        }
                        #[path = "."]
                        pub mod create_space {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🛋️space/🪑️create/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🛋️space/🪑️create/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🛋️space/🪑️create/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🛋️space/🪑️create/🧪️tests/✅️adds-a-space/🦀️.rs"]
                            mod tests_adds_a_space;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🛋️space/🪑️create/🧪️tests/⛔️refuses/🦀️.rs"]
                            mod tests_refuses_a_missing_zone;
                        }
                        #[path = "."]
                        pub mod delete_space {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🛋️space/🧹️delete/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🛋️space/🧹️delete/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🛋️space/🧹️delete/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🛋️space/🧹️delete/🧪️tests/✅️deletes-a-free-space/🦀️.rs"]
                            mod tests_deletes_a_free_space;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🛋️space/🧹️delete/🧪️tests/⛔️refuses/🦀️.rs"]
                            mod tests_refuses_a_missing_space;
                        }
                        #[path = "."]
                        pub mod rename_space {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🛋️space/🔤️rename/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🛋️space/🔤️rename/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🛋️space/🔤️rename/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🛋️space/🔤️rename/🧪️tests/⛔️refuses/🦀️.rs"]
                            mod tests_refuses_a_blank_name;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🛋️space/🔤️rename/🧪️tests/✅️renames-a-space/🦀️.rs"]
                            mod tests_renames_a_space;
                        }
                        #[path = "."]
                        pub mod change_space_floor_area {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🛋️space/🧮️change-floor-area/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🛋️space/🧮️change-floor-area/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🛋️space/🧮️change-floor-area/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🛋️space/🧮️change-floor-area/🧪️tests/⛔️refuses/🦀️.rs"]
                            mod tests_refuses_negative_area;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🛋️space/🧮️change-floor-area/🧪️tests/✅️resizes/🦀️.rs"]
                            mod tests_resizes_a_space;
                        }
                        #[path = "."]
                        pub mod change_space_zone {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🛋️space/🚚️change-zone/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🛋️space/🚚️change-zone/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🛋️space/🚚️change-zone/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🛋️space/🚚️change-zone/🧪️tests/✅️moves-a-space/🦀️.rs"]
                            mod tests_moves_a_space;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🛋️space/🚚️change-zone/🧪️tests/⛔️refuses/🦀️.rs"]
                            mod tests_refuses_a_missing_zone;
                        }
                        #[path = "."]
                        pub mod create_surface {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧱️surface/🟫️create/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧱️surface/🟫️create/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧱️surface/🟫️create/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧱️surface/🟫️create/🧪️tests/✅️adds-a-south-wall/🦀️.rs"]
                            mod tests_adds_a_south_wall;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧱️surface/🟫️create/🧪️tests/⛔️refuses/🦀️.rs"]
                            mod tests_refuses_two_vertices;
                        }
                        #[path = "."]
                        pub mod delete_surface {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧱️surface/🪚️delete/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧱️surface/🪚️delete/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧱️surface/🪚️delete/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧱️surface/🪚️delete/🧪️tests/✅️cascades-a-window/🦀️.rs"]
                            mod tests_cascades_a_window;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧱️surface/🪚️delete/🧪️tests/⛔️refuses-a-partner/🦀️.rs"]
                            mod tests_refuses_a_partner;
                        }
                        #[path = "."]
                        pub mod rename_surface {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧱️surface/🏳️rename/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧱️surface/🏳️rename/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧱️surface/🏳️rename/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧱️surface/🏳️rename/🧪️tests/⛔️refuses/🦀️.rs"]
                            mod tests_refuses_a_taken_name;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧱️surface/🏳️rename/🧪️tests/✅️renames-a-wall/🦀️.rs"]
                            mod tests_renames_a_wall;
                        }
                        #[path = "."]
                        pub mod change_surface_zone {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧱️surface/🗜️change-zone/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧱️surface/🗜️change-zone/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧱️surface/🗜️change-zone/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧱️surface/🗜️change-zone/🧪️tests/✅️moves-a-wall/🦀️.rs"]
                            mod tests_moves_a_wall;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧱️surface/🗜️change-zone/🧪️tests/⛔️refuses/🦀️.rs"]
                            mod tests_refuses_a_missing_zone;
                        }
                        #[path = "."]
                        pub mod change_surface_class {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧱️surface/🧩️change-class/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧱️surface/🧩️change-class/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧱️surface/🧩️change-class/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧱️surface/🧩️change-class/🧪️tests/⛔️refuses/🦀️.rs"]
                            mod tests_refuses_a_missing_one;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧱️surface/🧩️change-class/🧪️tests/✅️turns/🦀️.rs"]
                            mod tests_turns_a_wall_to_roof;
                        }
                        #[path = "."]
                        pub mod replace_surface_vertices {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧱️surface/🔺️replace-vertices/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧱️surface/🔺️replace-vertices/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧱️surface/🔺️replace-vertices/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧱️surface/🔺️replace-vertices/🧪️tests/✅️narrows/🦀️.rs"]
                            mod tests_narrows_a_wall;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧱️surface/🔺️replace-vertices/🧪️tests/⛔️refuses/🦀️.rs"]
                            mod tests_refuses_a_line;
                        }
                        #[path = "."]
                        pub mod change_surface_construction {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧱️surface/🧰️change-construction/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧱️surface/🧰️change-construction/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧱️surface/🧰️change-construction/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧱️surface/🧰️change-construction/🧪️tests/⛔️refuses/🦀️.rs"]
                            mod tests_refuses_a_missing_one;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧱️surface/🧰️change-construction/🧪️tests/✅️swaps/🦀️.rs"]
                            mod tests_swaps_the_wall_stack;
                        }
                        #[path = "."]
                        pub mod change_surface_boundary_condition {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔲️boundary/🚧️change-boundary-condition/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔲️boundary/🚧️change-boundary-condition/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔲️boundary/🚧️change-boundary-condition/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔲️boundary/🚧️change-boundary-condition/🧪️tests/✅️grounds/🦀️.rs"]
                            mod tests_grounds_a_floor;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔲️boundary/🚧️change-boundary-condition/🧪️tests/⛔️refuses/🦀️.rs"]
                            mod tests_refuses_half_a_union;
                        }
                        #[path = "."]
                        pub mod change_surface_sun_exposed {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔲️boundary/🌅️change-sun-exposed/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔲️boundary/🌅️change-sun-exposed/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔲️boundary/🌅️change-sun-exposed/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔲️boundary/🌅️change-sun-exposed/🧪️tests/⛔️refuses/🦀️.rs"]
                            mod tests_refuses_a_missing_one;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔲️boundary/🌅️change-sun-exposed/🧪️tests/✅️shades/🦀️.rs"]
                            mod tests_shades_a_wall;
                        }
                        #[path = "."]
                        pub mod change_surface_wind_exposed {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔲️boundary/🍃️change-wind-exposed/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔲️boundary/🍃️change-wind-exposed/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔲️boundary/🍃️change-wind-exposed/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔲️boundary/🍃️change-wind-exposed/🧪️tests/⛔️refuses/🦀️.rs"]
                            mod tests_refuses_a_missing_one;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔲️boundary/🍃️change-wind-exposed/🧪️tests/✅️shelters/🦀️.rs"]
                            mod tests_shelters_a_wall;
                        }
                        #[path = "."]
                        pub mod change_surface_multiplier {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧱️surface/🔁️change-multiplier/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧱️surface/🔁️change-multiplier/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧱️surface/🔁️change-multiplier/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧱️surface/🔁️change-multiplier/🧪️tests/⛔️refuses/🦀️.rs"]
                            mod tests_refuses_zero;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧱️surface/🔁️change-multiplier/🧪️tests/✅️repeats/🦀️.rs"]
                            mod tests_repeats_a_wall;
                        }
                        #[path = "."]
                        pub mod create_fenestration {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪟️fenestration/🪟️create/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪟️fenestration/🪟️create/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪟️fenestration/🪟️create/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪟️fenestration/🪟️create/🧪️tests/✅️adds/🦀️.rs"]
                            mod tests_adds_a_south_window;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪟️fenestration/🪟️create/🧪️tests/⛔️refuses/🦀️.rs"]
                            mod tests_refuses_a_missing_host;
                        }
                        #[path = "."]
                        pub mod delete_fenestration {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪟️fenestration/🚪️delete/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪟️fenestration/🚪️delete/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪟️fenestration/🚪️delete/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪟️fenestration/🚪️delete/🧪️tests/⛔️refuses/🦀️.rs"]
                            mod tests_refuses_a_missing_one;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪟️fenestration/🚪️delete/🧪️tests/✅️removes/🦀️.rs"]
                            mod tests_removes_a_window;
                        }
                        #[path = "."]
                        pub mod rename_fenestration {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪟️fenestration/🏁️rename/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪟️fenestration/🏁️rename/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪟️fenestration/🏁️rename/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪟️fenestration/🏁️rename/🧪️tests/⛔️refuses/🦀️.rs"]
                            mod tests_refuses_a_blank_name;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪟️fenestration/🏁️rename/🧪️tests/✅️renames/🦀️.rs"]
                            mod tests_renames_a_window;
                        }
                        #[path = "."]
                        pub mod change_fenestration_surface {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪟️fenestration/🧲️change-surface/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪟️fenestration/🧲️change-surface/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪟️fenestration/🧲️change-surface/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪟️fenestration/🧲️change-surface/🧪️tests/⛔️refuses/🦀️.rs"]
                            mod tests_refuses_a_missing_host;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪟️fenestration/🧲️change-surface/🧪️tests/✅️rehosts/🦀️.rs"]
                            mod tests_rehosts_a_window;
                        }
                        #[path = "."]
                        pub mod change_fenestration_u_value {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔍️glazing/🌐️change-u-value/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔍️glazing/🌐️change-u-value/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔍️glazing/🌐️change-u-value/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔍️glazing/🌐️change-u-value/🧪️tests/⛔️refuses/🦀️.rs"]
                            mod tests_refuses_zero_u;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔍️glazing/🌐️change-u-value/🧪️tests/✅️swaps/🦀️.rs"]
                            mod tests_swaps_the_glazing;
                        }
                        #[path = "."]
                        pub mod change_fenestration_shgc {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔍️glazing/🌇️change-shgc/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔍️glazing/🌇️change-shgc/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔍️glazing/🌇️change-shgc/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔍️glazing/🌇️change-shgc/🧪️tests/✅️dims/🦀️.rs"]
                            mod tests_dims_the_solar_gain;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔍️glazing/🌇️change-shgc/🧪️tests/⛔️refuses/🦀️.rs"]
                            mod tests_refuses_shgc_above_one;
                        }
                        #[path = "."]
                        pub mod change_fenestration_vlt {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔍️glazing/🌈️change-vlt/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔍️glazing/🌈️change-vlt/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔍️glazing/🌈️change-vlt/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔍️glazing/🌈️change-vlt/🧪️tests/✅️dims/🦀️.rs"]
                            mod tests_dims_the_daylight;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔍️glazing/🌈️change-vlt/🧪️tests/⛔️refuses/🦀️.rs"]
                            mod tests_refuses_negative_vlt;
                        }
                        #[path = "."]
                        pub mod change_fenestration_area {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪟️fenestration/🟥️change-area/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪟️fenestration/🟥️change-area/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪟️fenestration/🟥️change-area/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪟️fenestration/🟥️change-area/🧪️tests/✅️doubles/🦀️.rs"]
                            mod tests_doubles_the_glazing;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪟️fenestration/🟥️change-area/🧪️tests/⛔️refuses/🦀️.rs"]
                            mod tests_refuses_zero_area;
                        }
                        #[path = "."]
                        pub mod change_fenestration_frame_conductance {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔍️glazing/🖼️change-frame-conductance/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔍️glazing/🖼️change-frame-conductance/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔍️glazing/🖼️change-frame-conductance/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔍️glazing/🖼️change-frame-conductance/🧪️tests/✅️adds-a/🦀️.rs"]
                            mod tests_adds_a_frame;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔍️glazing/🖼️change-frame-conductance/🧪️tests/⛔️refuses/🦀️.rs"]
                            mod tests_refuses_negative_frame;
                        }
                        #[path = "."]
                        pub mod change_fenestration_divider_conductance {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔍️glazing/🧷️change-divider-conductance/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔍️glazing/🧷️change-divider-conductance/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔍️glazing/🧷️change-divider-conductance/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔍️glazing/🧷️change-divider-conductance/🧪️tests/✅️adds/🦀️.rs"]
                            mod tests_adds_dividers;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔍️glazing/🧷️change-divider-conductance/🧪️tests/⛔️refuses/🦀️.rs"]
                            mod tests_refuses_a_negative;
                        }
                        #[path = "."]
                        pub mod create_shading_surface {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌳️shading/🌳️create/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌳️shading/🌳️create/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌳️shading/🌳️create/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌳️shading/🌳️create/🧪️tests/✅️adds/🦀️.rs"]
                            mod tests_adds_an_awning;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌳️shading/🌳️create/🧪️tests/⛔️refuses/🦀️.rs"]
                            mod tests_refuses_a_line;
                        }
                        #[path = "."]
                        pub mod delete_shading_surface {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌳️shading/🪵️delete/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌳️shading/🪵️delete/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌳️shading/🪵️delete/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌳️shading/🪵️delete/🧪️tests/⛔️refuses/🦀️.rs"]
                            mod tests_refuses_a_missing_one;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌳️shading/🪵️delete/🧪️tests/✅️removes/🦀️.rs"]
                            mod tests_removes_an_awning;
                        }
                        #[path = "."]
                        pub mod rename_shading_surface {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌳️shading/🏕️rename/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌳️shading/🏕️rename/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌳️shading/🏕️rename/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌳️shading/🏕️rename/🧪️tests/⛔️refuses/🦀️.rs"]
                            mod tests_refuses_a_blank_name;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌳️shading/🏕️rename/🧪️tests/✅️renames/🦀️.rs"]
                            mod tests_renames_an_awning;
                        }
                        #[path = "."]
                        pub mod replace_shading_surface_vertices {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌳️shading/🗺️replace-vertices/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌳️shading/🗺️replace-vertices/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌳️shading/🗺️replace-vertices/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌳️shading/🗺️replace-vertices/🧪️tests/✅️deepens/🦀️.rs"]
                            mod tests_deepens_an_awning;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌳️shading/🗺️replace-vertices/🧪️tests/⛔️refuses/🦀️.rs"]
                            mod tests_refuses_a_line;
                        }
                        #[path = "."]
                        pub mod change_shading_surface_transmittance_schedule {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌳️shading/⛱️change-transmittance-schedule/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌳️shading/⛱️change-transmittance-schedule/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌳️shading/⛱️change-transmittance-schedule/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌳️shading/⛱️change-transmittance-schedule/🧪️tests/✅️lets/🦀️.rs"]
                            mod tests_lets_light_in;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌳️shading/⛱️change-transmittance-schedule/🧪️tests/⛔️refuses/🦀️.rs"]
                            mod tests_refuses_a_ghost;
                        }
                        #[path = "."]
                        pub mod connect_surfaces {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔲️boundary/🤝️connect/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔲️boundary/🤝️connect/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔲️boundary/🤝️connect/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔲️boundary/🤝️connect/🧪️tests/✅️joins-two-walls/🦀️.rs"]
                            mod tests_joins_two_walls;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔲️boundary/🤝️connect/🧪️tests/⛔️refuses/🦀️.rs"]
                            mod tests_refuses_a_self_pair;
                        }
                        #[path = "."]
                        pub mod disconnect_surfaces {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔲️boundary/💔️disconnect/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔲️boundary/💔️disconnect/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔲️boundary/💔️disconnect/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔲️boundary/💔️disconnect/🧪️tests/✅️parts/🦀️.rs"]
                            mod tests_parts_two_walls;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔲️boundary/💔️disconnect/🧪️tests/⛔️refuses/🦀️.rs"]
                            mod tests_refuses_a_missing_pair;
                        }
                        #[path = "."]
                        pub mod bind_fenestration_glazing_construction {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔍️glazing/🧊️bind-glazing-construction/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔍️glazing/🧊️bind-glazing-construction/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔍️glazing/🧊️bind-glazing-construction/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔍️glazing/🧊️bind-glazing-construction/🧪️tests/✅️glazes/🦀️.rs"]
                            mod tests_glazes_a_window;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔍️glazing/🧊️bind-glazing-construction/🧪️tests/⛔️refuses/🦀️.rs"]
                            mod tests_refuses_no_stack;
                        }
                        #[path = "."]
                        pub mod clear_fenestration_glazing_construction {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔍️glazing/🫗️clear-glazing-construction/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔍️glazing/🫗️clear-glazing-construction/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔍️glazing/🫗️clear-glazing-construction/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔍️glazing/🫗️clear-glazing-construction/🧪️tests/⛔️refuses/🦀️.rs"]
                            mod tests_refuses_empty;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔍️glazing/🫗️clear-glazing-construction/🧪️tests/✅️ungazes/🦀️.rs"]
                            mod tests_ungazes_a_window;
                        }
                        #[path = "."]
                        pub mod change_fenestration_height {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪟️fenestration/⬆️change-height/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪟️fenestration/⬆️change-height/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪟️fenestration/⬆️change-height/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪟️fenestration/⬆️change-height/🧪️tests/✅️raises/🦀️.rs"]
                            mod tests_raises_the_head;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪟️fenestration/⬆️change-height/🧪️tests/⛔️refuses/🦀️.rs"]
                            mod tests_refuses_zero_height;
                        }
                        #[path = "."]
                        pub mod change_fenestration_sill_height {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪟️fenestration/⬇️change-sill-height/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪟️fenestration/⬇️change-sill-height/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪟️fenestration/⬇️change-sill-height/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪟️fenestration/⬇️change-sill-height/🧪️tests/✅️raises/🦀️.rs"]
                            mod tests_raises_the_sill;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪟️fenestration/⬇️change-sill-height/🧪️tests/⛔️refuses/🦀️.rs"]
                            mod tests_refuses_negative_sill;
                        }
                        #[path = "."]
                        pub mod change_fenestration_overhang_depth {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/⛱️shading-device/🧢️change-overhang-depth/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/⛱️shading-device/🧢️change-overhang-depth/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/⛱️shading-device/🧢️change-overhang-depth/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/⛱️shading-device/🧢️change-overhang-depth/🧪️tests/✅️adds/🦀️.rs"]
                            mod tests_adds_case_610_shade;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/⛱️shading-device/🧢️change-overhang-depth/🧪️tests/⛔️refuses/🦀️.rs"]
                            mod tests_refuses_negative_depth;
                        }
                        #[path = "."]
                        pub mod change_fenestration_overhang_offset {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/⛱️shading-device/🎩️change-overhang-offset/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/⛱️shading-device/🎩️change-overhang-offset/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/⛱️shading-device/🎩️change-overhang-offset/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/⛱️shading-device/🎩️change-overhang-offset/🧪️tests/✅️lifts/🦀️.rs"]
                            mod tests_lifts_the_overhang;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/⛱️shading-device/🎩️change-overhang-offset/🧪️tests/⛔️refuses/🦀️.rs"]
                            mod tests_refuses_negative_offset;
                        }
                        #[path = "."]
                        pub mod change_fenestration_fin_depth {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/⛱️shading-device/🐬️change-fin-depth/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/⛱️shading-device/🐬️change-fin-depth/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/⛱️shading-device/🐬️change-fin-depth/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/⛱️shading-device/🐬️change-fin-depth/🧪️tests/✅️adds/🦀️.rs"]
                            mod tests_adds_case_630_fins;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/⛱️shading-device/🐬️change-fin-depth/🧪️tests/⛔️refuses/🦀️.rs"]
                            mod tests_refuses_negative_fin;
                        }
                        #[path = "."]
                        pub mod change_fenestration_fin_offset {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/⛱️shading-device/🐋️change-fin-offset/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/⛱️shading-device/🐋️change-fin-offset/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/⛱️shading-device/🐋️change-fin-offset/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/⛱️shading-device/🐋️change-fin-offset/🧪️tests/⛔️refuses/🦀️.rs"]
                            mod tests_refuses_negative_offset;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/⛱️shading-device/🐋️change-fin-offset/🧪️tests/✅️spreads/🦀️.rs"]
                            mod tests_spreads_the_fins;
                        }
                        #[path = "."]
                        pub mod create_material {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪨️material/🧱️create/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪨️material/🧱️create/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪨️material/🧱️create/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪨️material/🧱️create/🧪️tests/✅️applies/🦀️.rs"]
                            mod tests_applies;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪨️material/🧱️create/🧪️tests/⛔️refuses/🦀️.rs"]
                            mod tests_refuses;
                        }
                        #[path = "."]
                        pub mod delete_material {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪨️material/🪨️delete/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪨️material/🪨️delete/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪨️material/🪨️delete/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪨️material/🪨️delete/🧪️tests/✅️applies/🦀️.rs"]
                            mod tests_applies;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪨️material/🪨️delete/🧪️tests/⛔️refuses/🦀️.rs"]
                            mod tests_refuses;
                        }
                        #[path = "."]
                        pub mod rename_material {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪨️material/🪧️rename/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪨️material/🪧️rename/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪨️material/🪧️rename/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪨️material/🪧️rename/🧪️tests/✅️applies/🦀️.rs"]
                            mod tests_applies;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪨️material/🪧️rename/🧪️tests/⛔️refuses/🦀️.rs"]
                            mod tests_refuses;
                        }
                        #[path = "."]
                        pub mod change_material_thickness {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪨️material/📏️change-thickness/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪨️material/📏️change-thickness/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪨️material/📏️change-thickness/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪨️material/📏️change-thickness/🧪️tests/✅️applies/🦀️.rs"]
                            mod tests_applies;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪨️material/📏️change-thickness/🧪️tests/⛔️refuses/🦀️.rs"]
                            mod tests_refuses;
                        }
                        #[path = "."]
                        pub mod change_material_conductivity {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪨️material/🔥️change-conductivity/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪨️material/🔥️change-conductivity/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪨️material/🔥️change-conductivity/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪨️material/🔥️change-conductivity/🧪️tests/✅️applies/🦀️.rs"]
                            mod tests_applies;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪨️material/🔥️change-conductivity/🧪️tests/⛔️refuses/🦀️.rs"]
                            mod tests_refuses;
                        }
                        #[path = "."]
                        pub mod change_material_density {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪨️material/⚖️change-density/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪨️material/⚖️change-density/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪨️material/⚖️change-density/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪨️material/⚖️change-density/🧪️tests/✅️applies/🦀️.rs"]
                            mod tests_applies;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪨️material/⚖️change-density/🧪️tests/⛔️refuses/🦀️.rs"]
                            mod tests_refuses;
                        }
                        #[path = "."]
                        pub mod change_material_specific_heat {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪨️material/♨️change-specific-heat/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪨️material/♨️change-specific-heat/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪨️material/♨️change-specific-heat/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪨️material/♨️change-specific-heat/🧪️tests/✅️applies/🦀️.rs"]
                            mod tests_applies;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪨️material/♨️change-specific-heat/🧪️tests/⛔️refuses/🦀️.rs"]
                            mod tests_refuses;
                        }
                        #[path = "."]
                        pub mod change_material_thermal_absorptance {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🎨️finish/🔆️change-thermal-absorptance/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🎨️finish/🔆️change-thermal-absorptance/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🎨️finish/🔆️change-thermal-absorptance/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🎨️finish/🔆️change-thermal-absorptance/🧪️tests/✅️applies/🦀️.rs"]
                            mod tests_applies;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🎨️finish/🔆️change-thermal-absorptance/🧪️tests/⛔️refuses/🦀️.rs"]
                            mod tests_refuses;
                        }
                        #[path = "."]
                        pub mod change_material_solar_absorptance {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🎨️finish/☀️change-solar-absorptance/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🎨️finish/☀️change-solar-absorptance/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🎨️finish/☀️change-solar-absorptance/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🎨️finish/☀️change-solar-absorptance/🧪️tests/✅️applies/🦀️.rs"]
                            mod tests_applies;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🎨️finish/☀️change-solar-absorptance/🧪️tests/⛔️refuses/🦀️.rs"]
                            mod tests_refuses;
                        }
                        #[path = "."]
                        pub mod change_material_visible_absorptance {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🎨️finish/👁️change-visible-absorptance/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🎨️finish/👁️change-visible-absorptance/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🎨️finish/👁️change-visible-absorptance/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🎨️finish/👁️change-visible-absorptance/🧪️tests/✅️applies/🦀️.rs"]
                            mod tests_applies;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🎨️finish/👁️change-visible-absorptance/🧪️tests/⛔️refuses/🦀️.rs"]
                            mod tests_refuses;
                        }
                        #[path = "."]
                        pub mod create_construction {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏗️construction/🏗️create/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏗️construction/🏗️create/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏗️construction/🏗️create/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏗️construction/🏗️create/🧪️tests/✅️applies/🦀️.rs"]
                            mod tests_applies;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏗️construction/🏗️create/🧪️tests/⛔️refuses/🦀️.rs"]
                            mod tests_refuses;
                        }
                        #[path = "."]
                        pub mod delete_construction {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏗️construction/🧨️delete/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏗️construction/🧨️delete/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏗️construction/🧨️delete/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏗️construction/🧨️delete/🧪️tests/✅️applies/🦀️.rs"]
                            mod tests_applies;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏗️construction/🧨️delete/🧪️tests/⛔️refuses/🦀️.rs"]
                            mod tests_refuses;
                        }
                        #[path = "."]
                        pub mod rename_construction {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏗️construction/🪪️rename/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏗️construction/🪪️rename/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏗️construction/🪪️rename/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏗️construction/🪪️rename/🧪️tests/✅️applies/🦀️.rs"]
                            mod tests_applies;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏗️construction/🪪️rename/🧪️tests/⛔️refuses/🦀️.rs"]
                            mod tests_refuses;
                        }
                        #[path = "."]
                        pub mod add_construction_layer {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏗️construction/➕️add-layer/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏗️construction/➕️add-layer/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏗️construction/➕️add-layer/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏗️construction/➕️add-layer/🧪️tests/✅️applies/🦀️.rs"]
                            mod tests_applies;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏗️construction/➕️add-layer/🧪️tests/⛔️refuses/🦀️.rs"]
                            mod tests_refuses;
                        }
                        #[path = "."]
                        pub mod remove_construction_layer {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏗️construction/➖️remove-layer/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏗️construction/➖️remove-layer/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏗️construction/➖️remove-layer/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏗️construction/➖️remove-layer/🧪️tests/✅️applies/🦀️.rs"]
                            mod tests_applies;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏗️construction/➖️remove-layer/🧪️tests/⛔️refuses/🦀️.rs"]
                            mod tests_refuses;
                        }
                        #[path = "."]
                        pub mod reorder_construction_layers {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏗️construction/🔀️reorder-layers/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏗️construction/🔀️reorder-layers/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏗️construction/🔀️reorder-layers/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏗️construction/🔀️reorder-layers/🧪️tests/✅️applies/🦀️.rs"]
                            mod tests_applies;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏗️construction/🔀️reorder-layers/🧪️tests/⛔️refuses/🦀️.rs"]
                            mod tests_refuses;
                        }
                        #[path = "."]
                        pub mod create_people_gain {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/👥️people-gain/👤️create/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/👥️people-gain/👤️create/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/👥️people-gain/👤️create/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/👥️people-gain/👤️create/🧪️tests/✅️applies/🦀️.rs"]
                            mod tests_applies;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/👥️people-gain/👤️create/🧪️tests/⛔️refuses/🦀️.rs"]
                            mod tests_refuses;
                        }
                        #[path = "."]
                        pub mod delete_people_gain {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/👥️people-gain/🚷️delete/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/👥️people-gain/🚷️delete/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/👥️people-gain/🚷️delete/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/👥️people-gain/🚷️delete/🧪️tests/✅️applies/🦀️.rs"]
                            mod tests_applies;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/👥️people-gain/🚷️delete/🧪️tests/⛔️refuses/🦀️.rs"]
                            mod tests_refuses;
                        }
                        #[path = "."]
                        pub mod change_people_gain_zone {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/👥️people-gain/🚶️change-zone/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/👥️people-gain/🚶️change-zone/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/👥️people-gain/🚶️change-zone/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/👥️people-gain/🚶️change-zone/🧪️tests/✅️applies/🦀️.rs"]
                            mod tests_applies;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/👥️people-gain/🚶️change-zone/🧪️tests/⛔️refuses/🦀️.rs"]
                            mod tests_refuses;
                        }
                        #[path = "."]
                        pub mod change_people_gain_schedule {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/👥️people-gain/⏰️change-schedule/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/👥️people-gain/⏰️change-schedule/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/👥️people-gain/⏰️change-schedule/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/👥️people-gain/⏰️change-schedule/🧪️tests/✅️applies/🦀️.rs"]
                            mod tests_applies;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/👥️people-gain/⏰️change-schedule/🧪️tests/⛔️refuses/🦀️.rs"]
                            mod tests_refuses;
                        }
                        #[path = "."]
                        pub mod change_people_gain_activity_schedule {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/👥️people-gain/🏃️change-activity-schedule/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/👥️people-gain/🏃️change-activity-schedule/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/👥️people-gain/🏃️change-activity-schedule/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/👥️people-gain/🏃️change-activity-schedule/🧪️tests/✅️applies/🦀️.rs"]
                            mod tests_applies;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/👥️people-gain/🏃️change-activity-schedule/🧪️tests/⛔️refuses/🦀️.rs"]
                            mod tests_refuses;
                        }
                        #[path = "."]
                        pub mod change_people_gain_people_per_area {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/👥️people-gain/👥️change-people-per-area/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/👥️people-gain/👥️change-people-per-area/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/👥️people-gain/👥️change-people-per-area/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/👥️people-gain/👥️change-people-per-area/🧪️tests/✅️applies/🦀️.rs"]
                            mod tests_applies;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/👥️people-gain/👥️change-people-per-area/🧪️tests/⛔️refuses/🦀️.rs"]
                            mod tests_refuses;
                        }
                        #[path = "."]
                        pub mod change_people_gain_sensible_fraction {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/👥️people-gain/🌞️change-sensible-fraction/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/👥️people-gain/🌞️change-sensible-fraction/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/👥️people-gain/🌞️change-sensible-fraction/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/👥️people-gain/🌞️change-sensible-fraction/🧪️tests/✅️applies/🦀️.rs"]
                            mod tests_applies;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/👥️people-gain/🌞️change-sensible-fraction/🧪️tests/⛔️refuses/🦀️.rs"]
                            mod tests_refuses;
                        }
                        #[path = "."]
                        pub mod change_people_gain_latent_fraction {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/👥️people-gain/💧️change-latent-fraction/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/👥️people-gain/💧️change-latent-fraction/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/👥️people-gain/💧️change-latent-fraction/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/👥️people-gain/💧️change-latent-fraction/🧪️tests/✅️applies/🦀️.rs"]
                            mod tests_applies;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/👥️people-gain/💧️change-latent-fraction/🧪️tests/⛔️refuses/🦀️.rs"]
                            mod tests_refuses;
                        }
                        #[path = "."]
                        pub mod change_people_gain_radiant_fraction {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/👥️people-gain/📡️change-radiant-fraction/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/👥️people-gain/📡️change-radiant-fraction/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/👥️people-gain/📡️change-radiant-fraction/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/👥️people-gain/📡️change-radiant-fraction/🧪️tests/✅️applies/🦀️.rs"]
                            mod tests_applies;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/👥️people-gain/📡️change-radiant-fraction/🧪️tests/⛔️refuses/🦀️.rs"]
                            mod tests_refuses;
                        }
                        #[path = "."]
                        pub mod create_lighting_gain {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💡️lighting-gain/💡️create/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💡️lighting-gain/💡️create/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💡️lighting-gain/💡️create/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💡️lighting-gain/💡️create/🧪️tests/✅️applies/🦀️.rs"]
                            mod tests_applies;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💡️lighting-gain/💡️create/🧪️tests/⛔️refuses/🦀️.rs"]
                            mod tests_refuses;
                        }
                        #[path = "."]
                        pub mod delete_lighting_gain {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💡️lighting-gain/🕯️delete/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💡️lighting-gain/🕯️delete/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💡️lighting-gain/🕯️delete/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💡️lighting-gain/🕯️delete/🧪️tests/✅️applies/🦀️.rs"]
                            mod tests_applies;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💡️lighting-gain/🕯️delete/🧪️tests/⛔️refuses/🦀️.rs"]
                            mod tests_refuses;
                        }
                        #[path = "."]
                        pub mod change_lighting_gain_zone {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💡️lighting-gain/🔦️change-zone/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💡️lighting-gain/🔦️change-zone/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💡️lighting-gain/🔦️change-zone/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💡️lighting-gain/🔦️change-zone/🧪️tests/✅️applies/🦀️.rs"]
                            mod tests_applies;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💡️lighting-gain/🔦️change-zone/🧪️tests/⛔️refuses/🦀️.rs"]
                            mod tests_refuses;
                        }
                        #[path = "."]
                        pub mod change_lighting_gain_schedule {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💡️lighting-gain/⏱️change-schedule/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💡️lighting-gain/⏱️change-schedule/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💡️lighting-gain/⏱️change-schedule/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💡️lighting-gain/⏱️change-schedule/🧪️tests/✅️applies/🦀️.rs"]
                            mod tests_applies;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💡️lighting-gain/⏱️change-schedule/🧪️tests/⛔️refuses/🦀️.rs"]
                            mod tests_refuses;
                        }
                        #[path = "."]
                        pub mod change_lighting_gain_watts_per_area {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💡️lighting-gain/🔌️change-watts-per-area/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💡️lighting-gain/🔌️change-watts-per-area/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💡️lighting-gain/🔌️change-watts-per-area/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💡️lighting-gain/🔌️change-watts-per-area/🧪️tests/✅️applies/🦀️.rs"]
                            mod tests_applies;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💡️lighting-gain/🔌️change-watts-per-area/🧪️tests/⛔️refuses/🦀️.rs"]
                            mod tests_refuses;
                        }
                        #[path = "."]
                        pub mod change_lighting_gain_radiant_fraction {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💡️lighting-gain/🌟️change-radiant-fraction/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💡️lighting-gain/🌟️change-radiant-fraction/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💡️lighting-gain/🌟️change-radiant-fraction/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💡️lighting-gain/🌟️change-radiant-fraction/🧪️tests/✅️applies/🦀️.rs"]
                            mod tests_applies;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💡️lighting-gain/🌟️change-radiant-fraction/🧪️tests/⛔️refuses/🦀️.rs"]
                            mod tests_refuses;
                        }
                        #[path = "."]
                        pub mod change_lighting_gain_visible_fraction {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💡️lighting-gain/🔅️change-visible-fraction/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💡️lighting-gain/🔅️change-visible-fraction/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💡️lighting-gain/🔅️change-visible-fraction/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💡️lighting-gain/🔅️change-visible-fraction/🧪️tests/✅️applies/🦀️.rs"]
                            mod tests_applies;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💡️lighting-gain/🔅️change-visible-fraction/🧪️tests/⛔️refuses/🦀️.rs"]
                            mod tests_refuses;
                        }
                        #[path = "."]
                        pub mod change_lighting_gain_return_air_fraction {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💡️lighting-gain/🎐️change-return-air-fraction/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💡️lighting-gain/🎐️change-return-air-fraction/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💡️lighting-gain/🎐️change-return-air-fraction/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💡️lighting-gain/🎐️change-return-air-fraction/🧪️tests/✅️applies/🦀️.rs"]
                            mod tests_applies;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💡️lighting-gain/🎐️change-return-air-fraction/🧪️tests/⛔️refuses/🦀️.rs"]
                            mod tests_refuses;
                        }
                        #[path = "."]
                        pub mod create_equipment_gain {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🖥️equipment-gain/🖥️create/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🖥️equipment-gain/🖥️create/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🖥️equipment-gain/🖥️create/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🖥️equipment-gain/🖥️create/🧪️tests/✅️applies/🦀️.rs"]
                            mod tests_applies;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🖥️equipment-gain/🖥️create/🧪️tests/⛔️refuses/🦀️.rs"]
                            mod tests_refuses;
                        }
                        #[path = "."]
                        pub mod delete_equipment_gain {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🖥️equipment-gain/🧯️delete/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🖥️equipment-gain/🧯️delete/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🖥️equipment-gain/🧯️delete/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🖥️equipment-gain/🧯️delete/🧪️tests/✅️applies/🦀️.rs"]
                            mod tests_applies;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🖥️equipment-gain/🧯️delete/🧪️tests/⛔️refuses/🦀️.rs"]
                            mod tests_refuses;
                        }
                        #[path = "."]
                        pub mod change_equipment_gain_zone {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🖥️equipment-gain/🖨️change-zone/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🖥️equipment-gain/🖨️change-zone/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🖥️equipment-gain/🖨️change-zone/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🖥️equipment-gain/🖨️change-zone/🧪️tests/✅️applies/🦀️.rs"]
                            mod tests_applies;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🖥️equipment-gain/🖨️change-zone/🧪️tests/⛔️refuses/🦀️.rs"]
                            mod tests_refuses;
                        }
                        #[path = "."]
                        pub mod change_equipment_gain_schedule {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🖥️equipment-gain/⌛️change-schedule/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🖥️equipment-gain/⌛️change-schedule/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🖥️equipment-gain/⌛️change-schedule/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🖥️equipment-gain/⌛️change-schedule/🧪️tests/✅️applies/🦀️.rs"]
                            mod tests_applies;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🖥️equipment-gain/⌛️change-schedule/🧪️tests/⛔️refuses/🦀️.rs"]
                            mod tests_refuses;
                        }
                        #[path = "."]
                        pub mod change_equipment_gain_watts_per_area {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🖥️equipment-gain/⚡️change-watts-per-area/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🖥️equipment-gain/⚡️change-watts-per-area/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🖥️equipment-gain/⚡️change-watts-per-area/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🖥️equipment-gain/⚡️change-watts-per-area/🧪️tests/✅️applies/🦀️.rs"]
                            mod tests_applies;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🖥️equipment-gain/⚡️change-watts-per-area/🧪️tests/⛔️refuses/🦀️.rs"]
                            mod tests_refuses;
                        }
                        #[path = "."]
                        pub mod change_equipment_gain_radiant_fraction {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🖥️equipment-gain/🌠️change-radiant-fraction/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🖥️equipment-gain/🌠️change-radiant-fraction/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🖥️equipment-gain/🌠️change-radiant-fraction/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🖥️equipment-gain/🌠️change-radiant-fraction/🧪️tests/✅️applies/🦀️.rs"]
                            mod tests_applies;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🖥️equipment-gain/🌠️change-radiant-fraction/🧪️tests/⛔️refuses/🦀️.rs"]
                            mod tests_refuses;
                        }
                        #[path = "."]
                        pub mod change_equipment_gain_latent_fraction {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🖥️equipment-gain/💦️change-latent-fraction/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🖥️equipment-gain/💦️change-latent-fraction/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🖥️equipment-gain/💦️change-latent-fraction/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🖥️equipment-gain/💦️change-latent-fraction/🧪️tests/✅️applies/🦀️.rs"]
                            mod tests_applies;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🖥️equipment-gain/💦️change-latent-fraction/🧪️tests/⛔️refuses/🦀️.rs"]
                            mod tests_refuses;
                        }
                        #[path = "."]
                        pub mod create_infiltration {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💨️infiltration/💨️create/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💨️infiltration/💨️create/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💨️infiltration/💨️create/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💨️infiltration/💨️create/🧪️tests/✅️applies/🦀️.rs"]
                            mod tests_applies;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💨️infiltration/💨️create/🧪️tests/⛔️refuses/🦀️.rs"]
                            mod tests_refuses;
                        }
                        #[path = "."]
                        pub mod delete_infiltration {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💨️infiltration/🧽️delete/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💨️infiltration/🧽️delete/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💨️infiltration/🧽️delete/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💨️infiltration/🧽️delete/🧪️tests/✅️applies/🦀️.rs"]
                            mod tests_applies;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💨️infiltration/🧽️delete/🧪️tests/⛔️refuses/🦀️.rs"]
                            mod tests_refuses;
                        }
                        #[path = "."]
                        pub mod change_infiltration_zone {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💨️infiltration/🌀️change-zone/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💨️infiltration/🌀️change-zone/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💨️infiltration/🌀️change-zone/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💨️infiltration/🌀️change-zone/🧪️tests/✅️applies/🦀️.rs"]
                            mod tests_applies;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💨️infiltration/🌀️change-zone/🧪️tests/⛔️refuses/🦀️.rs"]
                            mod tests_refuses;
                        }
                        #[path = "."]
                        pub mod change_infiltration_schedule {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💨️infiltration/⏳️change-schedule/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💨️infiltration/⏳️change-schedule/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💨️infiltration/⏳️change-schedule/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💨️infiltration/⏳️change-schedule/🧪️tests/✅️applies/🦀️.rs"]
                            mod tests_applies;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💨️infiltration/⏳️change-schedule/🧪️tests/⛔️refuses/🦀️.rs"]
                            mod tests_refuses;
                        }
                        #[path = "."]
                        pub mod change_infiltration_flow_per_exterior_area {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💨️infiltration/🌫️change-flow-per-exterior-area/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💨️infiltration/🌫️change-flow-per-exterior-area/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💨️infiltration/🌫️change-flow-per-exterior-area/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💨️infiltration/🌫️change-flow-per-exterior-area/🧪️tests/✅️applies/🦀️.rs"]
                            mod tests_applies;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💨️infiltration/🌫️change-flow-per-exterior-area/🧪️tests/⛔️refuses/🦀️.rs"]
                            mod tests_refuses;
                        }
                        #[path = "."]
                        pub mod change_infiltration_constant_term_coefficient {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🕳️leakage/🅰️change-constant-term-coefficient/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🕳️leakage/🅰️change-constant-term-coefficient/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🕳️leakage/🅰️change-constant-term-coefficient/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🕳️leakage/🅰️change-constant-term-coefficient/🧪️tests/✅️applies/🦀️.rs"]
                            mod tests_applies;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🕳️leakage/🅰️change-constant-term-coefficient/🧪️tests/⛔️refuses/🦀️.rs"]
                            mod tests_refuses;
                        }
                        #[path = "."]
                        pub mod change_infiltration_temperature_term_coefficient {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🕳️leakage/🅱️change-temperature-term-coefficient/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🕳️leakage/🅱️change-temperature-term-coefficient/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🕳️leakage/🅱️change-temperature-term-coefficient/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🕳️leakage/🅱️change-temperature-term-coefficient/🧪️tests/✅️applies/🦀️.rs"]
                            mod tests_applies;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🕳️leakage/🅱️change-temperature-term-coefficient/🧪️tests/⛔️refuses/🦀️.rs"]
                            mod tests_refuses;
                        }
                        #[path = "."]
                        pub mod change_infiltration_velocity_term_coefficient {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🕳️leakage/🆎️change-velocity-term-coefficient/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🕳️leakage/🆎️change-velocity-term-coefficient/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🕳️leakage/🆎️change-velocity-term-coefficient/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🕳️leakage/🆎️change-velocity-term-coefficient/🧪️tests/✅️applies/🦀️.rs"]
                            mod tests_applies;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🕳️leakage/🆎️change-velocity-term-coefficient/🧪️tests/⛔️refuses/🦀️.rs"]
                            mod tests_refuses;
                        }
                        #[path = "."]
                        pub mod change_infiltration_velocity_squared_term_coefficient {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🕳️leakage/🆑️change-velocity-squared-term-coefficient/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🕳️leakage/🆑️change-velocity-squared-term-coefficient/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🕳️leakage/🆑️change-velocity-squared-term-coefficient/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🕳️leakage/🆑️change-velocity-squared-term-coefficient/🧪️tests/✅️applies/🦀️.rs"]
                            mod tests_applies;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🕳️leakage/🆑️change-velocity-squared-term-coefficient/🧪️tests/⛔️refuses/🦀️.rs"]
                            mod tests_refuses;
                        }
                        #[path = "."]
                        pub mod create_mechanical_ventilation {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌪️ventilation/🌪️create/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌪️ventilation/🌪️create/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌪️ventilation/🌪️create/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌪️ventilation/🌪️create/🧪️tests/✅️applies/🦀️.rs"]
                            mod tests_applies;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌪️ventilation/🌪️create/🧪️tests/⛔️refuses/🦀️.rs"]
                            mod tests_refuses;
                        }
                        #[path = "."]
                        pub mod delete_mechanical_ventilation {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌪️ventilation/🚫️delete/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌪️ventilation/🚫️delete/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌪️ventilation/🚫️delete/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌪️ventilation/🚫️delete/🧪️tests/✅️applies/🦀️.rs"]
                            mod tests_applies;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌪️ventilation/🚫️delete/🧪️tests/⛔️refuses/🦀️.rs"]
                            mod tests_refuses;
                        }
                        #[path = "."]
                        pub mod change_mechanical_ventilation_zone {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌪️ventilation/🧭️change-zone/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌪️ventilation/🧭️change-zone/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌪️ventilation/🧭️change-zone/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌪️ventilation/🧭️change-zone/🧪️tests/✅️applies/🦀️.rs"]
                            mod tests_applies;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌪️ventilation/🧭️change-zone/🧪️tests/⛔️refuses/🦀️.rs"]
                            mod tests_refuses;
                        }
                        #[path = "."]
                        pub mod change_mechanical_ventilation_schedule {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌪️ventilation/📆️change-schedule/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌪️ventilation/📆️change-schedule/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌪️ventilation/📆️change-schedule/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌪️ventilation/📆️change-schedule/🧪️tests/✅️applies/🦀️.rs"]
                            mod tests_applies;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌪️ventilation/📆️change-schedule/🧪️tests/⛔️refuses/🦀️.rs"]
                            mod tests_refuses;
                        }
                        #[path = "."]
                        pub mod change_mechanical_ventilation_design_flow {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌪️ventilation/🚿️change-design-flow/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌪️ventilation/🚿️change-design-flow/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌪️ventilation/🚿️change-design-flow/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌪️ventilation/🚿️change-design-flow/🧪️tests/✅️applies/🦀️.rs"]
                            mod tests_applies;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌪️ventilation/🚿️change-design-flow/🧪️tests/⛔️refuses/🦀️.rs"]
                            mod tests_refuses;
                        }
                        #[path = "."]
                        pub mod change_mechanical_ventilation_fan_total_efficiency {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌪️ventilation/💠️change-fan-total-efficiency/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌪️ventilation/💠️change-fan-total-efficiency/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌪️ventilation/💠️change-fan-total-efficiency/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌪️ventilation/💠️change-fan-total-efficiency/🧪️tests/✅️applies/🦀️.rs"]
                            mod tests_applies;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌪️ventilation/💠️change-fan-total-efficiency/🧪️tests/⛔️refuses/🦀️.rs"]
                            mod tests_refuses;
                        }
                        #[path = "."]
                        pub mod change_mechanical_ventilation_fan_delta_pressure {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌪️ventilation/🎈️change-fan-delta-pressure/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌪️ventilation/🎈️change-fan-delta-pressure/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌪️ventilation/🎈️change-fan-delta-pressure/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌪️ventilation/🎈️change-fan-delta-pressure/🧪️tests/✅️applies/🦀️.rs"]
                            mod tests_applies;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌪️ventilation/🎈️change-fan-delta-pressure/🧪️tests/⛔️refuses/🦀️.rs"]
                            mod tests_refuses;
                        }
                        #[path = "."]
                        pub mod change_infiltration_method {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💨️infiltration/🔬️change-method/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💨️infiltration/🔬️change-method/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💨️infiltration/🔬️change-method/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💨️infiltration/🔬️change-method/🧪️tests/✅️applies/🦀️.rs"]
                            mod tests_applies;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💨️infiltration/🔬️change-method/🧪️tests/⛔️refuses/🦀️.rs"]
                            mod tests_refuses;
                        }
                        #[path = "."]
                        pub mod change_infiltration_design_flow_ach {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💨️infiltration/🔄️change-design-flow-ach/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💨️infiltration/🔄️change-design-flow-ach/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💨️infiltration/🔄️change-design-flow-ach/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💨️infiltration/🔄️change-design-flow-ach/🧪️tests/✅️applies/🦀️.rs"]
                            mod tests_applies;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💨️infiltration/🔄️change-design-flow-ach/🧪️tests/⛔️refuses/🦀️.rs"]
                            mod tests_refuses;
                        }
                        #[path = "."]
                        pub mod change_infiltration_effective_leakage_area {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🕳️leakage/🕳️change-effective-leakage-area/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🕳️leakage/🕳️change-effective-leakage-area/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🕳️leakage/🕳️change-effective-leakage-area/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🕳️leakage/🕳️change-effective-leakage-area/🧪️tests/✅️applies/🦀️.rs"]
                            mod tests_applies;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🕳️leakage/🕳️change-effective-leakage-area/🧪️tests/⛔️refuses/🦀️.rs"]
                            mod tests_refuses;
                        }
                        #[path = "."]
                        pub mod change_infiltration_discharge_coefficient {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🕳️leakage/🚰️change-discharge-coefficient/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🕳️leakage/🚰️change-discharge-coefficient/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🕳️leakage/🚰️change-discharge-coefficient/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🕳️leakage/🚰️change-discharge-coefficient/🧪️tests/✅️applies/🦀️.rs"]
                            mod tests_applies;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🕳️leakage/🚰️change-discharge-coefficient/🧪️tests/⛔️refuses/🦀️.rs"]
                            mod tests_refuses;
                        }
                        #[path = "."]
                        pub mod change_infiltration_stack_height {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🕳️leakage/🏭️change-stack-height/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🕳️leakage/🏭️change-stack-height/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🕳️leakage/🏭️change-stack-height/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🕳️leakage/🏭️change-stack-height/🧪️tests/✅️applies/🦀️.rs"]
                            mod tests_applies;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🕳️leakage/🏭️change-stack-height/🧪️tests/⛔️refuses/🦀️.rs"]
                            mod tests_refuses;
                        }
                        #[path = "."]
                        pub mod create_thermostat {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌡️thermostat/🩺️create/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌡️thermostat/🩺️create/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌡️thermostat/🩺️create/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌡️thermostat/🩺️create/🧪️tests/✅️controls/🦀️.rs"]
                            mod tests_controls_zone_one;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌡️thermostat/🩺️create/🧪️tests/⛔️refuses/🦀️.rs"]
                            mod tests_refuses_an_absent_zone;
                        }
                        #[path = "."]
                        pub mod delete_thermostat {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌡️thermostat/🛑️delete/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌡️thermostat/🛑️delete/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌡️thermostat/🛑️delete/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌡️thermostat/🛑️delete/🧪️tests/✅️frees-zone-one/🦀️.rs"]
                            mod tests_frees_zone_one;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌡️thermostat/🛑️delete/🧪️tests/⛔️refuses/🦀️.rs"]
                            mod tests_refuses_an_absent_one;
                        }
                        #[path = "."]
                        pub mod change_thermostat_zone {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌡️thermostat/🛖️change-zone/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌡️thermostat/🛖️change-zone/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌡️thermostat/🛖️change-zone/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌡️thermostat/🛖️change-zone/🧪️tests/✅️moves/🦀️.rs"]
                            mod tests_moves_to_zone_two;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌡️thermostat/🛖️change-zone/🧪️tests/⛔️refuses/🦀️.rs"]
                            mod tests_refuses_an_absent_zone;
                        }
                        #[path = "."]
                        pub mod change_thermostat_heating_setpoint_schedule {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌡️thermostat/🥵️change-heating-setpoint-schedule/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌡️thermostat/🥵️change-heating-setpoint-schedule/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌡️thermostat/🥵️change-heating-setpoint-schedule/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌡️thermostat/🥵️change-heating-setpoint-schedule/🧪️tests/⛔️refuses/🦀️.rs"]
                            mod tests_refuses_an_absent_one;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌡️thermostat/🥵️change-heating-setpoint-schedule/🧪️tests/✅️repoints/🦀️.rs"]
                            mod tests_repoints_heating;
                        }
                        #[path = "."]
                        pub mod change_thermostat_cooling_setpoint_schedule {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌡️thermostat/🐧️change-cooling-setpoint-schedule/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌡️thermostat/🐧️change-cooling-setpoint-schedule/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌡️thermostat/🐧️change-cooling-setpoint-schedule/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌡️thermostat/🐧️change-cooling-setpoint-schedule/🧪️tests/⛔️refuses/🦀️.rs"]
                            mod tests_refuses_an_absent_one;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌡️thermostat/🐧️change-cooling-setpoint-schedule/🧪️tests/✅️repoints/🦀️.rs"]
                            mod tests_repoints_cooling;
                        }
                        #[path = "."]
                        pub mod change_thermostat_heating_throttle_range {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌡️thermostat/🎚️change-heating-throttle-range/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌡️thermostat/🎚️change-heating-throttle-range/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌡️thermostat/🎚️change-heating-throttle-range/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌡️thermostat/🎚️change-heating-throttle-range/🧪️tests/⛔️refuses/🦀️.rs"]
                            mod tests_refuses_a_zero_band;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌡️thermostat/🎚️change-heating-throttle-range/🧪️tests/✅️widens/🦀️.rs"]
                            mod tests_widens_heating_band;
                        }
                        #[path = "."]
                        pub mod change_thermostat_cooling_throttle_range {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌡️thermostat/🎛️change-cooling-throttle-range/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌡️thermostat/🎛️change-cooling-throttle-range/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌡️thermostat/🎛️change-cooling-throttle-range/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌡️thermostat/🎛️change-cooling-throttle-range/🧪️tests/⛔️refuses/🦀️.rs"]
                            mod tests_refuses_a_negative_band;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌡️thermostat/🎛️change-cooling-throttle-range/🧪️tests/✅️widens/🦀️.rs"]
                            mod tests_widens_cooling_band;
                        }
                        #[path = "."]
                        pub mod create_humidistat {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💧️humidistat/🌂️create/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💧️humidistat/🌂️create/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💧️humidistat/🌂️create/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💧️humidistat/🌂️create/🧪️tests/✅️controls/🦀️.rs"]
                            mod tests_controls_zone_one;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💧️humidistat/🌂️create/🧪️tests/⛔️refuses/🦀️.rs"]
                            mod tests_refuses_an_absent_one;
                        }
                        #[path = "."]
                        pub mod delete_humidistat {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💧️humidistat/🏜️delete/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💧️humidistat/🏜️delete/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💧️humidistat/🏜️delete/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💧️humidistat/🏜️delete/🧪️tests/✅️drops/🦀️.rs"]
                            mod tests_drops_the_control;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💧️humidistat/🏜️delete/🧪️tests/⛔️refuses/🦀️.rs"]
                            mod tests_refuses_an_absent_one;
                        }
                        #[path = "."]
                        pub mod change_humidistat_zone {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💧️humidistat/🏙️change-zone/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💧️humidistat/🏙️change-zone/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💧️humidistat/🏙️change-zone/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💧️humidistat/🏙️change-zone/🧪️tests/✅️moves/🦀️.rs"]
                            mod tests_moves_to_zone_two;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💧️humidistat/🏙️change-zone/🧪️tests/⛔️refuses/🦀️.rs"]
                            mod tests_refuses_an_absent_zone;
                        }
                        #[path = "."]
                        pub mod change_humidistat_humidifying_setpoint_schedule {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💧️humidistat/☔️change-humidifying-setpoint-schedule/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💧️humidistat/☔️change-humidifying-setpoint-schedule/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💧️humidistat/☔️change-humidifying-setpoint-schedule/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💧️humidistat/☔️change-humidifying-setpoint-schedule/🧪️tests/⛔️refuses/🦀️.rs"]
                            mod tests_refuses_an_absent_one;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💧️humidistat/☔️change-humidifying-setpoint-schedule/🧪️tests/✅️repoints/🦀️.rs"]
                            mod tests_repoints_humidifying;
                        }
                        #[path = "."]
                        pub mod change_humidistat_dehumidifying_setpoint_schedule {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💧️humidistat/🏝️change-dehumidifying-setpoint-schedule/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💧️humidistat/🏝️change-dehumidifying-setpoint-schedule/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💧️humidistat/🏝️change-dehumidifying-setpoint-schedule/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💧️humidistat/🏝️change-dehumidifying-setpoint-schedule/🧪️tests/⛔️refuses/🦀️.rs"]
                            mod tests_refuses_an_absent_one;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💧️humidistat/🏝️change-dehumidifying-setpoint-schedule/🧪️tests/✅️repoints/🦀️.rs"]
                            mod tests_repoints_drying;
                        }
                        #[path = "."]
                        pub mod change_humidistat_humidifying_throttle_range {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💧️humidistat/🌧️change-humidifying-throttle-range/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💧️humidistat/🌧️change-humidifying-throttle-range/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💧️humidistat/🌧️change-humidifying-throttle-range/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💧️humidistat/🌧️change-humidifying-throttle-range/🧪️tests/⛔️refuses/🦀️.rs"]
                            mod tests_refuses_a_zero_band;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💧️humidistat/🌧️change-humidifying-throttle-range/🧪️tests/✅️widens/🦀️.rs"]
                            mod tests_widens_the_band;
                        }
                        #[path = "."]
                        pub mod change_humidistat_dehumidifying_throttle_range {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💧️humidistat/🧻️change-dehumidifying-throttle-range/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💧️humidistat/🧻️change-dehumidifying-throttle-range/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💧️humidistat/🧻️change-dehumidifying-throttle-range/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💧️humidistat/🧻️change-dehumidifying-throttle-range/🧪️tests/⛔️refuses/🦀️.rs"]
                            mod tests_refuses_a_negative_band;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💧️humidistat/🧻️change-dehumidifying-throttle-range/🧪️tests/✅️widens/🦀️.rs"]
                            mod tests_widens_the_band;
                        }
                        #[path = "."]
                        pub mod create_ideal_loads_system {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💯️ideal-loads/🫁️create/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💯️ideal-loads/🫁️create/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💯️ideal-loads/🫁️create/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💯️ideal-loads/🫁️create/🧪️tests/⛔️refuses/🦀️.rs"]
                            mod tests_refuses_an_absent_zone;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💯️ideal-loads/🫁️create/🧪️tests/✅️serves/🦀️.rs"]
                            mod tests_serves_zone_one;
                        }
                        #[path = "."]
                        pub mod delete_ideal_loads_system {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💯️ideal-loads/🫥️delete/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💯️ideal-loads/🫥️delete/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💯️ideal-loads/🫥️delete/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💯️ideal-loads/🫥️delete/🧪️tests/✅️drops/🦀️.rs"]
                            mod tests_drops_the_system;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💯️ideal-loads/🫥️delete/🧪️tests/⛔️refuses/🦀️.rs"]
                            mod tests_refuses_an_absent_one;
                        }
                        #[path = "."]
                        pub mod change_ideal_loads_system_zone {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💯️ideal-loads/🏢️change-zone/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💯️ideal-loads/🏢️change-zone/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💯️ideal-loads/🏢️change-zone/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💯️ideal-loads/🏢️change-zone/🧪️tests/✅️moves/🦀️.rs"]
                            mod tests_moves_to_zone_two;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💯️ideal-loads/🏢️change-zone/🧪️tests/⛔️refuses/🦀️.rs"]
                            mod tests_refuses_an_absent_zone;
                        }
                        #[path = "."]
                        pub mod change_ideal_loads_system_max_heating_supply_air_temp {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💯️ideal-loads/🔴️change-max-heating-supply-air-temp/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💯️ideal-loads/🔴️change-max-heating-supply-air-temp/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💯️ideal-loads/🔴️change-max-heating-supply-air-temp/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💯️ideal-loads/🔴️change-max-heating-supply-air-temp/🧪️tests/✅️cools/🦀️.rs"]
                            mod tests_cools_the_supply;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💯️ideal-loads/🔴️change-max-heating-supply-air-temp/🧪️tests/⛔️refuses/🦀️.rs"]
                            mod tests_refuses_a_hot_supply;
                        }
                        #[path = "."]
                        pub mod change_ideal_loads_system_min_cooling_supply_air_temp {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💯️ideal-loads/🔵️change-min-cooling-supply-air-temp/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💯️ideal-loads/🔵️change-min-cooling-supply-air-temp/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💯️ideal-loads/🔵️change-min-cooling-supply-air-temp/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💯️ideal-loads/🔵️change-min-cooling-supply-air-temp/🧪️tests/✅️lowers/🦀️.rs"]
                            mod tests_lowers_the_supply;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💯️ideal-loads/🔵️change-min-cooling-supply-air-temp/🧪️tests/⛔️refuses/🦀️.rs"]
                            mod tests_refuses_a_cold_supply;
                        }
                        #[path = "."]
                        pub mod change_ideal_loads_system_max_heating_capacity {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💯️ideal-loads/⛽️change-max-heating-capacity/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💯️ideal-loads/⛽️change-max-heating-capacity/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💯️ideal-loads/⛽️change-max-heating-capacity/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💯️ideal-loads/⛽️change-max-heating-capacity/🧪️tests/✅️caps/🦀️.rs"]
                            mod tests_caps_the_heating;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💯️ideal-loads/⛽️change-max-heating-capacity/🧪️tests/⛔️refuses/🦀️.rs"]
                            mod tests_refuses_a_stray_value;
                        }
                        #[path = "."]
                        pub mod change_ideal_loads_system_max_cooling_capacity {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💯️ideal-loads/🟧️change-max-cooling-capacity/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💯️ideal-loads/🟧️change-max-cooling-capacity/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💯️ideal-loads/🟧️change-max-cooling-capacity/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💯️ideal-loads/🟧️change-max-cooling-capacity/🧪️tests/✅️caps/🦀️.rs"]
                            mod tests_caps_the_cooling;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💯️ideal-loads/🟧️change-max-cooling-capacity/🧪️tests/⛔️refuses/🦀️.rs"]
                            mod tests_refuses_a_stray_value;
                        }
                        #[path = "."]
                        pub mod change_ideal_loads_system_outdoor_air_per_person {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💯️ideal-loads/🧍️change-outdoor-air-per-person/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💯️ideal-loads/🧍️change-outdoor-air-per-person/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💯️ideal-loads/🧍️change-outdoor-air-per-person/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💯️ideal-loads/🧍️change-outdoor-air-per-person/🧪️tests/⛔️refuses/🦀️.rs"]
                            mod tests_refuses_a_negative;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💯️ideal-loads/🧍️change-outdoor-air-per-person/🧪️tests/✅️ventilates/🦀️.rs"]
                            mod tests_ventilates_per_head;
                        }
                        #[path = "."]
                        pub mod change_ideal_loads_system_outdoor_air_per_area {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💯️ideal-loads/🔳️change-outdoor-air-per-area/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💯️ideal-loads/🔳️change-outdoor-air-per-area/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💯️ideal-loads/🔳️change-outdoor-air-per-area/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💯️ideal-loads/🔳️change-outdoor-air-per-area/🧪️tests/⛔️refuses/🦀️.rs"]
                            mod tests_refuses_a_negative;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💯️ideal-loads/🔳️change-outdoor-air-per-area/🧪️tests/✅️ventilates/🦀️.rs"]
                            mod tests_ventilates_per_area;
                        }
                        #[path = "."]
                        pub mod create_zone_equipment {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔧️zone-equipment/🛠️create/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔧️zone-equipment/🛠️create/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔧️zone-equipment/🛠️create/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔧️zone-equipment/🛠️create/🧪️tests/✅️adds/🦀️.rs"]
                            mod tests_adds_a_baseboard;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔧️zone-equipment/🛠️create/🧪️tests/⛔️refuses/🦀️.rs"]
                            mod tests_refuses_rank_zero;
                        }
                        #[path = "."]
                        pub mod delete_zone_equipment {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔧️zone-equipment/🗑️delete/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔧️zone-equipment/🗑️delete/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔧️zone-equipment/🗑️delete/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔧️zone-equipment/🗑️delete/🧪️tests/✅️drops/🦀️.rs"]
                            mod tests_drops_the_baseboard;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔧️zone-equipment/🗑️delete/🧪️tests/⛔️refuses/🦀️.rs"]
                            mod tests_refuses_an_absent_one;
                        }
                        #[path = "."]
                        pub mod change_zone_equipment_zone {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔧️zone-equipment/🏬️change-zone/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔧️zone-equipment/🏬️change-zone/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔧️zone-equipment/🏬️change-zone/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔧️zone-equipment/🏬️change-zone/🧪️tests/✅️moves/🦀️.rs"]
                            mod tests_moves_to_zone_two;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔧️zone-equipment/🏬️change-zone/🧪️tests/⛔️refuses/🦀️.rs"]
                            mod tests_refuses_an_absent_zone;
                        }
                        #[path = "."]
                        pub mod change_zone_equipment_type {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔧️zone-equipment/🔧️change-type/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔧️zone-equipment/🔧️change-type/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔧️zone-equipment/🔧️change-type/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔧️zone-equipment/🔧️change-type/🧪️tests/⛔️refuses/🦀️.rs"]
                            mod tests_refuses_an_absent_row;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔧️zone-equipment/🔧️change-type/🧪️tests/✅️swaps/🦀️.rs"]
                            mod tests_swaps_to_a_fan_coil;
                        }
                        #[path = "."]
                        pub mod change_zone_equipment_priority {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔧️zone-equipment/🎗️change-priority/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔧️zone-equipment/🎗️change-priority/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔧️zone-equipment/🎗️change-priority/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔧️zone-equipment/🎗️change-priority/🧪️tests/✅️demotes-it/🦀️.rs"]
                            mod tests_demotes_it;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔧️zone-equipment/🎗️change-priority/🧪️tests/⛔️refuses/🦀️.rs"]
                            mod tests_refuses_rank_zero;
                        }
                        #[path = "."]
                        pub mod change_zone_equipment_heating_capacity {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔧️zone-equipment/🧇️change-heating-capacity/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔧️zone-equipment/🧇️change-heating-capacity/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔧️zone-equipment/🧇️change-heating-capacity/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔧️zone-equipment/🧇️change-heating-capacity/🧪️tests/⛔️refuses/🦀️.rs"]
                            mod tests_refuses_a_negative;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔧️zone-equipment/🧇️change-heating-capacity/🧪️tests/✅️uprates/🦀️.rs"]
                            mod tests_uprates_heating;
                        }
                        #[path = "."]
                        pub mod change_zone_equipment_cooling_capacity {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔧️zone-equipment/🍧️change-cooling-capacity/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔧️zone-equipment/🍧️change-cooling-capacity/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔧️zone-equipment/🍧️change-cooling-capacity/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔧️zone-equipment/🍧️change-cooling-capacity/🧪️tests/⛔️refuses/🦀️.rs"]
                            mod tests_refuses_a_negative;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔧️zone-equipment/🍧️change-cooling-capacity/🧪️tests/✅️uprates/🦀️.rs"]
                            mod tests_uprates_cooling;
                        }
                        #[path = "."]
                        pub mod create_daylight_zone {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌞️daylight-zone/🔭️create/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌞️daylight-zone/🔭️create/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌞️daylight-zone/🔭️create/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌞️daylight-zone/🔭️create/🧪️tests/✅️lights/🦀️.rs"]
                            mod tests_lights_zone_one;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌞️daylight-zone/🔭️create/🧪️tests/⛔️refuses/🦀️.rs"]
                            mod tests_refuses_a_bad_tau;
                        }
                        #[path = "."]
                        pub mod delete_daylight_zone {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌞️daylight-zone/🌗️delete/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌞️daylight-zone/🌗️delete/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌞️daylight-zone/🌗️delete/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌞️daylight-zone/🌗️delete/🧪️tests/✅️darkens/🦀️.rs"]
                            mod tests_darkens_the_zone;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌞️daylight-zone/🌗️delete/🧪️tests/⛔️refuses/🦀️.rs"]
                            mod tests_refuses_an_absent_one;
                        }
                        #[path = "."]
                        pub mod change_daylight_zone_zone {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌞️daylight-zone/🏫️change-zone/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌞️daylight-zone/🏫️change-zone/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌞️daylight-zone/🏫️change-zone/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌞️daylight-zone/🏫️change-zone/🧪️tests/✅️moves/🦀️.rs"]
                            mod tests_moves_to_zone_two;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌞️daylight-zone/🏫️change-zone/🧪️tests/⛔️refuses/🦀️.rs"]
                            mod tests_refuses_an_absent_zone;
                        }
                        #[path = "."]
                        pub mod change_daylight_zone_illuminance_target {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌞️daylight-zone/🪔️change-illuminance-target/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌞️daylight-zone/🪔️change-illuminance-target/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌞️daylight-zone/🪔️change-illuminance-target/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌞️daylight-zone/🪔️change-illuminance-target/🧪️tests/✅️dims/🦀️.rs"]
                            mod tests_dims_the_target;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌞️daylight-zone/🪔️change-illuminance-target/🧪️tests/⛔️refuses/🦀️.rs"]
                            mod tests_refuses_a_dark_target;
                        }
                        #[path = "."]
                        pub mod change_daylight_zone_glare_limit {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌞️daylight-zone/🕶️change-glare-limit/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌞️daylight-zone/🕶️change-glare-limit/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌞️daylight-zone/🕶️change-glare-limit/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌞️daylight-zone/🕶️change-glare-limit/🧪️tests/⛔️refuses/🦀️.rs"]
                            mod tests_refuses_a_negative;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌞️daylight-zone/🕶️change-glare-limit/🧪️tests/✅️tightens/🦀️.rs"]
                            mod tests_tightens_glare;
                        }
                        #[path = "."]
                        pub mod change_daylight_zone_window_transmittance {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌞️daylight-zone/🥃️change-window-transmittance/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌞️daylight-zone/🥃️change-window-transmittance/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌞️daylight-zone/🥃️change-window-transmittance/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌞️daylight-zone/🥃️change-window-transmittance/🧪️tests/✅️darkens/🦀️.rs"]
                            mod tests_darkens_the_glass;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌞️daylight-zone/🥃️change-window-transmittance/🧪️tests/⛔️refuses/🦀️.rs"]
                            mod tests_refuses_a_bad_tau;
                        }
                        #[path = "."]
                        pub mod create_sizing_object {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📏️sizing/📶️create/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📏️sizing/📶️create/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📏️sizing/📶️create/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📏️sizing/📶️create/🧪️tests/⛔️refuses/🦀️.rs"]
                            mod tests_refuses_an_absent_zone;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📏️sizing/📶️create/🧪️tests/✅️sizes/🦀️.rs"]
                            mod tests_sizes_zone_one;
                        }
                        #[path = "."]
                        pub mod delete_sizing_object {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📏️sizing/🪒️delete/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📏️sizing/🪒️delete/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📏️sizing/🪒️delete/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📏️sizing/🪒️delete/🧪️tests/✅️drops/🦀️.rs"]
                            mod tests_drops_the_sizing;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📏️sizing/🪒️delete/🧪️tests/⛔️refuses/🦀️.rs"]
                            mod tests_refuses_an_absent_one;
                        }
                        #[path = "."]
                        pub mod change_sizing_object_zone {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📏️sizing/🏨️change-zone/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📏️sizing/🏨️change-zone/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📏️sizing/🏨️change-zone/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📏️sizing/🏨️change-zone/🧪️tests/✅️moves/🦀️.rs"]
                            mod tests_moves_to_zone_two;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📏️sizing/🏨️change-zone/🧪️tests/⛔️refuses/🦀️.rs"]
                            mod tests_refuses_an_absent_zone;
                        }
                        #[path = "."]
                        pub mod change_sizing_object_sizing_type {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📏️sizing/🧾️change-sizing-type/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📏️sizing/🧾️change-sizing-type/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📏️sizing/🧾️change-sizing-type/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📏️sizing/🧾️change-sizing-type/🧪️tests/⛔️refuses/🦀️.rs"]
                            mod tests_refuses_an_absent_row;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📏️sizing/🧾️change-sizing-type/🧪️tests/✅️sizes/🦀️.rs"]
                            mod tests_sizes_for_cooling;
                        }
                        #[path = "."]
                        pub mod change_sizing_object_design_day_type {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📏️sizing/🌥️change-design-day-type/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📏️sizing/🌥️change-design-day-type/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📏️sizing/🌥️change-design-day-type/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📏️sizing/🌥️change-design-day-type/🧪️tests/✅️reads/🦀️.rs"]
                            mod tests_reads_a_hot_day;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📏️sizing/🌥️change-design-day-type/🧪️tests/⛔️refuses/🦀️.rs"]
                            mod tests_refuses_an_absent_row;
                        }
                        #[path = "."]
                        pub mod create_room_air_model_assignment {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🛏️room-air/🛏️create-assignment/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🛏️room-air/🛏️create-assignment/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🛏️room-air/🛏️create-assignment/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🛏️room-air/🛏️create-assignment/🧪️tests/⛔️refuses/🦀️.rs"]
                            mod tests_refuses_a_second_one;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🛏️room-air/🛏️create-assignment/🧪️tests/✅️stratifies/🦀️.rs"]
                            mod tests_stratifies_zone_two;
                        }
                        #[path = "."]
                        pub mod delete_room_air_model_assignment {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🛏️room-air/🧺️delete-assignment/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🛏️room-air/🧺️delete-assignment/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🛏️room-air/🧺️delete-assignment/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🛏️room-air/🧺️delete-assignment/🧪️tests/✅️falls-back/🦀️.rs"]
                            mod tests_falls_back;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🛏️room-air/🧺️delete-assignment/🧪️tests/⛔️refuses/🦀️.rs"]
                            mod tests_refuses_an_absent_one;
                        }
                        #[path = "."]
                        pub mod change_room_air_model {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🛏️room-air/🪭️change/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🛏️room-air/🪭️change/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🛏️room-air/🪭️change/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🛏️room-air/🪭️change/🧪️tests/⛔️refuses/🦀️.rs"]
                            mod tests_refuses_an_absent_row;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🛏️room-air/🪭️change/🧪️tests/✅️stratifies/🦀️.rs"]
                            mod tests_stratifies_the_air;
                        }
                        #[path = "."]
                        pub mod create_setpoint_manager {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🎚️setpoint/📌️create/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🎚️setpoint/📌️create/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🎚️setpoint/📌️create/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🎚️setpoint/📌️create/🧪️tests/✅️adds/🦀️.rs"]
                            mod tests_adds_a_scheduled_spm;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🎚️setpoint/📌️create/🧪️tests/⛔️refuses/🦀️.rs"]
                            mod tests_refuses_a_bad_kind;
                        }
                        #[path = "."]
                        pub mod delete_setpoint_manager {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🎚️setpoint/🍄️delete/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🎚️setpoint/🍄️delete/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🎚️setpoint/🍄️delete/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🎚️setpoint/🍄️delete/🧪️tests/✅️drops/🦀️.rs"]
                            mod tests_drops_the_spm;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🎚️setpoint/🍄️delete/🧪️tests/⛔️refuses/🦀️.rs"]
                            mod tests_refuses_an_absent_one;
                        }
                        #[path = "."]
                        pub mod rename_setpoint_manager {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🎚️setpoint/🖇️rename/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🎚️setpoint/🖇️rename/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🎚️setpoint/🖇️rename/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🎚️setpoint/🖇️rename/🧪️tests/⛔️refuses/🦀️.rs"]
                            mod tests_refuses_a_blank_name;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🎚️setpoint/🖇️rename/🧪️tests/✅️renames/🦀️.rs"]
                            mod tests_renames_the_spm;
                        }
                        #[path = "."]
                        pub mod replace_setpoint_manager_kind {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🎚️setpoint/🔃️replace-kind/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🎚️setpoint/🔃️replace-kind/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🎚️setpoint/🔃️replace-kind/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🎚️setpoint/🔃️replace-kind/🧪️tests/⛔️refuses/🦀️.rs"]
                            mod tests_refuses_stray_limits;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🎚️setpoint/🔃️replace-kind/🧪️tests/✅️resets/🦀️.rs"]
                            mod tests_resets_on_outdoor_air;
                        }
                        #[path = "."]
                        pub mod change_setpoint_manager_schedule {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🎚️setpoint/🎼️change-schedule/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🎚️setpoint/🎼️change-schedule/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🎚️setpoint/🎼️change-schedule/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🎚️setpoint/🎼️change-schedule/🧪️tests/⛔️refuses/🦀️.rs"]
                            mod tests_refuses_an_absent_one;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🎚️setpoint/🎼️change-schedule/🧪️tests/✅️repoints/🦀️.rs"]
                            mod tests_repoints_the_spm;
                        }
                        #[path = "."]
                        pub mod create_air_loop {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌬️air-loop/🛞️create/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌬️air-loop/🛞️create/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌬️air-loop/🛞️create/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌬️air-loop/🛞️create/🧪️tests/✅️adds-a-main-loop/🦀️.rs"]
                            mod tests_adds_a_main_loop;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌬️air-loop/🛞️create/🧪️tests/⛔️refuses/🦀️.rs"]
                            mod tests_refuses_a_jumbled_list;
                        }
                        #[path = "."]
                        pub mod delete_air_loop {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌬️air-loop/🥀️delete/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌬️air-loop/🥀️delete/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌬️air-loop/🥀️delete/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌬️air-loop/🥀️delete/🧪️tests/✅️drops-the-loop/🦀️.rs"]
                            mod tests_drops_the_loop;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌬️air-loop/🥀️delete/🧪️tests/⛔️refuses/🦀️.rs"]
                            mod tests_refuses_a_served_loop;
                        }
                        #[path = "."]
                        pub mod rename_air_loop {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌬️air-loop/📇️rename/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌬️air-loop/📇️rename/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌬️air-loop/📇️rename/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌬️air-loop/📇️rename/🧪️tests/⛔️refuses/🦀️.rs"]
                            mod tests_refuses_a_blank_name;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌬️air-loop/📇️rename/🧪️tests/✅️renames-the-loop/🦀️.rs"]
                            mod tests_renames_the_loop;
                        }
                        #[path = "."]
                        pub mod change_air_loop_supply_node {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌬️air-loop/↗️change-supply-node/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌬️air-loop/↗️change-supply-node/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌬️air-loop/↗️change-supply-node/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌬️air-loop/↗️change-supply-node/🧪️tests/⛔️refuses/🦀️.rs"]
                            mod tests_refuses_node_zero;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌬️air-loop/↗️change-supply-node/🧪️tests/✅️repoints/🦀️.rs"]
                            mod tests_repoints_supply;
                        }
                        #[path = "."]
                        pub mod change_air_loop_return_node {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌬️air-loop/↘️change-return-node/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌬️air-loop/↘️change-return-node/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌬️air-loop/↘️change-return-node/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌬️air-loop/↘️change-return-node/🧪️tests/⛔️refuses/🦀️.rs"]
                            mod tests_refuses_node_zero;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌬️air-loop/↘️change-return-node/🧪️tests/✅️repoints/🦀️.rs"]
                            mod tests_repoints_return;
                        }
                        #[path = "."]
                        pub mod change_air_loop_design_supply_air_flow {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌬️air-loop/🍥️change-design-supply-air-flow/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌬️air-loop/🍥️change-design-supply-air-flow/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌬️air-loop/🍥️change-design-supply-air-flow/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌬️air-loop/🍥️change-design-supply-air-flow/🧪️tests/⛔️refuses/🦀️.rs"]
                            mod tests_refuses_no_flow;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌬️air-loop/🍥️change-design-supply-air-flow/🧪️tests/✅️uprates/🦀️.rs"]
                            mod tests_uprates_the_flow;
                        }
                        #[path = "."]
                        pub mod add_air_loop_terminal_zone {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌬️air-loop/🪺️add-terminal-zone/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌬️air-loop/🪺️add-terminal-zone/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌬️air-loop/🪺️add-terminal-zone/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌬️air-loop/🪺️add-terminal-zone/🧪️tests/⛔️refuses/🦀️.rs"]
                            mod tests_refuses_an_absent_zone;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌬️air-loop/🪺️add-terminal-zone/🧪️tests/✅️serves/🦀️.rs"]
                            mod tests_serves_zone_two;
                        }
                        #[path = "."]
                        pub mod remove_air_loop_terminal_zone {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌬️air-loop/🪹️remove-terminal-zone/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌬️air-loop/🪹️remove-terminal-zone/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌬️air-loop/🪹️remove-terminal-zone/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌬️air-loop/🪹️remove-terminal-zone/🧪️tests/⛔️refuses/🦀️.rs"]
                            mod tests_refuses_an_unserved;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌬️air-loop/🪹️remove-terminal-zone/🧪️tests/✅️stops/🦀️.rs"]
                            mod tests_stops_serving_one;
                        }
                        #[path = "."]
                        pub mod create_plant_loop {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏭️plant-loop/⚗️create/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏭️plant-loop/⚗️create/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏭️plant-loop/⚗️create/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏭️plant-loop/⚗️create/🧪️tests/✅️adds-a-hot-loop/🦀️.rs"]
                            mod tests_adds_a_hot_loop;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏭️plant-loop/⚗️create/🧪️tests/⛔️refuses/🦀️.rs"]
                            mod tests_refuses_a_jumbled_list;
                        }
                        #[path = "."]
                        pub mod delete_plant_loop {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏭️plant-loop/💣️delete/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏭️plant-loop/💣️delete/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏭️plant-loop/💣️delete/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏭️plant-loop/💣️delete/🧪️tests/✅️drops-the-loop/🦀️.rs"]
                            mod tests_drops_the_loop;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏭️plant-loop/💣️delete/🧪️tests/⛔️refuses/🦀️.rs"]
                            mod tests_refuses_an_absent_one;
                        }
                        #[path = "."]
                        pub mod rename_plant_loop {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏭️plant-loop/📛️rename/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏭️plant-loop/📛️rename/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏭️plant-loop/📛️rename/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏭️plant-loop/📛️rename/🧪️tests/⛔️refuses/🦀️.rs"]
                            mod tests_refuses_a_blank_name;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏭️plant-loop/📛️rename/🧪️tests/✅️renames-the-loop/🦀️.rs"]
                            mod tests_renames_the_loop;
                        }
                        #[path = "."]
                        pub mod change_plant_loop_type {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏭️plant-loop/♻️change-type/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏭️plant-loop/♻️change-type/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏭️plant-loop/♻️change-type/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏭️plant-loop/♻️change-type/🧪️tests/⛔️refuses/🦀️.rs"]
                            mod tests_refuses_an_absent_row;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏭️plant-loop/♻️change-type/🧪️tests/✅️turns/🦀️.rs"]
                            mod tests_turns_it_chilled;
                        }
                        #[path = "."]
                        pub mod change_plant_loop_supply_temperature {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏭️plant-loop/☕️change-supply-temperature/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏭️plant-loop/☕️change-supply-temperature/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏭️plant-loop/☕️change-supply-temperature/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏭️plant-loop/☕️change-supply-temperature/🧪️tests/✅️cools/🦀️.rs"]
                            mod tests_cools_the_supply;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏭️plant-loop/☕️change-supply-temperature/🧪️tests/⛔️refuses/🦀️.rs"]
                            mod tests_refuses_a_hot_supply;
                        }
                        #[path = "."]
                        pub mod change_plant_loop_return_temperature {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏭️plant-loop/🫖️change-return-temperature/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏭️plant-loop/🫖️change-return-temperature/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏭️plant-loop/🫖️change-return-temperature/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏭️plant-loop/🫖️change-return-temperature/🧪️tests/✅️cools/🦀️.rs"]
                            mod tests_cools_the_return;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏭️plant-loop/🫖️change-return-temperature/🧪️tests/⛔️refuses/🦀️.rs"]
                            mod tests_refuses_a_cold_return;
                        }
                        #[path = "."]
                        pub mod change_plant_loop_design_flow {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏭️plant-loop/🚤️change-design-flow/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏭️plant-loop/🚤️change-design-flow/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏭️plant-loop/🚤️change-design-flow/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏭️plant-loop/🚤️change-design-flow/🧪️tests/⛔️refuses/🦀️.rs"]
                            mod tests_refuses_no_flow;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏭️plant-loop/🚤️change-design-flow/🧪️tests/✅️uprates/🦀️.rs"]
                            mod tests_uprates_the_flow;
                        }
                        #[path = "."]
                        pub mod add_plant_loop_equipment {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏭️plant-loop/🔩️add-equipment/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏭️plant-loop/🔩️add-equipment/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏭️plant-loop/🔩️add-equipment/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏭️plant-loop/🔩️add-equipment/🧪️tests/✅️names/🦀️.rs"]
                            mod tests_names_equipment;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏭️plant-loop/🔩️add-equipment/🧪️tests/⛔️refuses/🦀️.rs"]
                            mod tests_refuses_the_unset_id;
                        }
                        #[path = "."]
                        pub mod remove_plant_loop_equipment {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏭️plant-loop/⚙️remove-equipment/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏭️plant-loop/⚙️remove-equipment/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏭️plant-loop/⚙️remove-equipment/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏭️plant-loop/⚙️remove-equipment/🧪️tests/✅️drops/🦀️.rs"]
                            mod tests_drops_equipment;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏭️plant-loop/⚙️remove-equipment/🧪️tests/⛔️refuses/🦀️.rs"]
                            mod tests_refuses_an_unlisted;
                        }
                        #[path = "."]
                        pub mod create_outdoor_air_system {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🍃️outdoor-air/🌲️create/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🍃️outdoor-air/🌲️create/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🍃️outdoor-air/🌲️create/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🍃️outdoor-air/🌲️create/🧪️tests/⛔️refuses/🦀️.rs"]
                            mod tests_refuses_an_absent_loop;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🍃️outdoor-air/🌲️create/🧪️tests/✅️ventilates/🦀️.rs"]
                            mod tests_ventilates_the_loop;
                        }
                        #[path = "."]
                        pub mod delete_outdoor_air_system {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🍃️outdoor-air/🍂️delete/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🍃️outdoor-air/🍂️delete/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🍃️outdoor-air/🍂️delete/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🍃️outdoor-air/🍂️delete/🧪️tests/✅️drops/🦀️.rs"]
                            mod tests_drops_the_system;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🍃️outdoor-air/🍂️delete/🧪️tests/⛔️refuses/🦀️.rs"]
                            mod tests_refuses_an_absent_one;
                        }
                        #[path = "."]
                        pub mod change_outdoor_air_system_air_loop {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🍃️outdoor-air/⛓️change-air-loop/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🍃️outdoor-air/⛓️change-air-loop/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🍃️outdoor-air/⛓️change-air-loop/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🍃️outdoor-air/⛓️change-air-loop/🧪️tests/✅️moves/🦀️.rs"]
                            mod tests_moves_to_the_spare;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🍃️outdoor-air/⛓️change-air-loop/🧪️tests/⛔️refuses/🦀️.rs"]
                            mod tests_refuses_an_absent_loop;
                        }
                        #[path = "."]
                        pub mod change_outdoor_air_system_min_oa_flow {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🍃️outdoor-air/🦋️change-min-oa-flow/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🍃️outdoor-air/🦋️change-min-oa-flow/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🍃️outdoor-air/🦋️change-min-oa-flow/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🍃️outdoor-air/🦋️change-min-oa-flow/🧪️tests/✅️raises/🦀️.rs"]
                            mod tests_raises_the_minimum;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🍃️outdoor-air/🦋️change-min-oa-flow/🧪️tests/⛔️refuses/🦀️.rs"]
                            mod tests_refuses_a_negative;
                        }
                        #[path = "."]
                        pub mod change_outdoor_air_system_economizer_enabled {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🍃️outdoor-air/💰️change-economizer-enabled/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🍃️outdoor-air/💰️change-economizer-enabled/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🍃️outdoor-air/💰️change-economizer-enabled/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🍃️outdoor-air/💰️change-economizer-enabled/🧪️tests/✅️frees/🦀️.rs"]
                            mod tests_frees_the_cooling;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🍃️outdoor-air/💰️change-economizer-enabled/🧪️tests/⛔️refuses/🦀️.rs"]
                            mod tests_refuses_an_absent_row;
                        }
                        #[path = "."]
                        pub mod create_electrical_load_center {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔌️load-center/🏦️create/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔌️load-center/🏦️create/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔌️load-center/🏦️create/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔌️load-center/🏦️create/🧪️tests/✅️applies/🦀️.rs"]
                            mod tests_applies;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔌️load-center/🏦️create/🧪️tests/⛔️refuses/🦀️.rs"]
                            mod tests_refuses;
                        }
                        #[path = "."]
                        pub mod delete_electrical_load_center {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔌️load-center/🔻️delete/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔌️load-center/🔻️delete/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔌️load-center/🔻️delete/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔌️load-center/🔻️delete/🧪️tests/✅️applies/🦀️.rs"]
                            mod tests_applies;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔌️load-center/🔻️delete/🧪️tests/⛔️refuses/🦀️.rs"]
                            mod tests_refuses;
                        }
                        #[path = "."]
                        pub mod rename_electrical_load_center {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔌️load-center/🖊️rename/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔌️load-center/🖊️rename/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔌️load-center/🖊️rename/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔌️load-center/🖊️rename/🧪️tests/✅️applies/🦀️.rs"]
                            mod tests_applies;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔌️load-center/🖊️rename/🧪️tests/⛔️refuses/🦀️.rs"]
                            mod tests_refuses;
                        }
                        #[path = "."]
                        pub mod add_electrical_load_center_pv {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔌️load-center/☄️add-pv/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔌️load-center/☄️add-pv/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔌️load-center/☄️add-pv/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔌️load-center/☄️add-pv/🧪️tests/✅️applies/🦀️.rs"]
                            mod tests_applies;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔌️load-center/☄️add-pv/🧪️tests/⛔️refuses/🦀️.rs"]
                            mod tests_refuses;
                        }
                        #[path = "."]
                        pub mod remove_electrical_load_center_pv {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔌️load-center/🌘️remove-pv/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔌️load-center/🌘️remove-pv/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔌️load-center/🌘️remove-pv/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔌️load-center/🌘️remove-pv/🧪️tests/✅️applies/🦀️.rs"]
                            mod tests_applies;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔌️load-center/🌘️remove-pv/🧪️tests/⛔️refuses/🦀️.rs"]
                            mod tests_refuses;
                        }
                        #[path = "."]
                        pub mod add_electrical_load_center_battery {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔌️load-center/🔋️add-battery/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔌️load-center/🔋️add-battery/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔌️load-center/🔋️add-battery/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔌️load-center/🔋️add-battery/🧪️tests/✅️applies/🦀️.rs"]
                            mod tests_applies;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔌️load-center/🔋️add-battery/🧪️tests/⛔️refuses/🦀️.rs"]
                            mod tests_refuses;
                        }
                        #[path = "."]
                        pub mod remove_electrical_load_center_battery {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔌️load-center/🪝️remove-battery/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔌️load-center/🪝️remove-battery/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔌️load-center/🪝️remove-battery/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔌️load-center/🪝️remove-battery/🧪️tests/✅️applies/🦀️.rs"]
                            mod tests_applies;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔌️load-center/🪝️remove-battery/🧪️tests/⛔️refuses/🦀️.rs"]
                            mod tests_refuses;
                        }
                        #[path = "."]
                        pub mod create_pv_system {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/☀️pv-system/✨️create/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/☀️pv-system/✨️create/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/☀️pv-system/✨️create/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/☀️pv-system/✨️create/🧪️tests/✅️applies/🦀️.rs"]
                            mod tests_applies;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/☀️pv-system/✨️create/🧪️tests/⛔️refuses/🦀️.rs"]
                            mod tests_refuses;
                        }
                        #[path = "."]
                        pub mod delete_pv_system {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/☀️pv-system/🌒️delete/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/☀️pv-system/🌒️delete/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/☀️pv-system/🌒️delete/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/☀️pv-system/🌒️delete/🧪️tests/✅️applies/🦀️.rs"]
                            mod tests_applies;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/☀️pv-system/🌒️delete/🧪️tests/⛔️refuses/🦀️.rs"]
                            mod tests_refuses;
                        }
                        #[path = "."]
                        pub mod change_pv_system_dc_capacity {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/☀️pv-system/⚛️change-dc-capacity/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/☀️pv-system/⚛️change-dc-capacity/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/☀️pv-system/⚛️change-dc-capacity/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/☀️pv-system/⚛️change-dc-capacity/🧪️tests/✅️applies/🦀️.rs"]
                            mod tests_applies;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/☀️pv-system/⚛️change-dc-capacity/🧪️tests/⛔️refuses/🦀️.rs"]
                            mod tests_refuses;
                        }
                        #[path = "."]
                        pub mod change_pv_system_area {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/☀️pv-system/🟨️change-area/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/☀️pv-system/🟨️change-area/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/☀️pv-system/🟨️change-area/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/☀️pv-system/🟨️change-area/🧪️tests/✅️applies/🦀️.rs"]
                            mod tests_applies;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/☀️pv-system/🟨️change-area/🧪️tests/⛔️refuses/🦀️.rs"]
                            mod tests_refuses;
                        }
                        #[path = "."]
                        pub mod change_pv_system_tilt {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/☀️pv-system/📈️change-tilt/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/☀️pv-system/📈️change-tilt/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/☀️pv-system/📈️change-tilt/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/☀️pv-system/📈️change-tilt/🧪️tests/✅️applies/🦀️.rs"]
                            mod tests_applies;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/☀️pv-system/📈️change-tilt/🧪️tests/⛔️refuses/🦀️.rs"]
                            mod tests_refuses;
                        }
                        #[path = "."]
                        pub mod change_pv_system_azimuth {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/☀️pv-system/🧿️change-azimuth/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/☀️pv-system/🧿️change-azimuth/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/☀️pv-system/🧿️change-azimuth/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/☀️pv-system/🧿️change-azimuth/🧪️tests/✅️applies/🦀️.rs"]
                            mod tests_applies;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/☀️pv-system/🧿️change-azimuth/🧪️tests/⛔️refuses/🦀️.rs"]
                            mod tests_refuses;
                        }
                        #[path = "."]
                        pub mod change_pv_system_module_efficiency {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/☀️pv-system/🎖️change-module-efficiency/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/☀️pv-system/🎖️change-module-efficiency/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/☀️pv-system/🎖️change-module-efficiency/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/☀️pv-system/🎖️change-module-efficiency/🧪️tests/✅️applies/🦀️.rs"]
                            mod tests_applies;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/☀️pv-system/🎖️change-module-efficiency/🧪️tests/⛔️refuses/🦀️.rs"]
                            mod tests_refuses;
                        }
                        #[path = "."]
                        pub mod change_pv_system_inverter_efficiency {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/☀️pv-system/♌️change-inverter-efficiency/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/☀️pv-system/♌️change-inverter-efficiency/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/☀️pv-system/♌️change-inverter-efficiency/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/☀️pv-system/♌️change-inverter-efficiency/🧪️tests/✅️applies/🦀️.rs"]
                            mod tests_applies;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/☀️pv-system/♌️change-inverter-efficiency/🧪️tests/⛔️refuses/🦀️.rs"]
                            mod tests_refuses;
                        }
                        #[path = "."]
                        pub mod create_battery {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪫️battery/🪙️create/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪫️battery/🪙️create/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪫️battery/🪙️create/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪫️battery/🪙️create/🧪️tests/✅️applies/🦀️.rs"]
                            mod tests_applies;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪫️battery/🪙️create/🧪️tests/⛔️refuses/🦀️.rs"]
                            mod tests_refuses;
                        }
                        #[path = "."]
                        pub mod delete_battery {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪫️battery/♒️delete/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪫️battery/♒️delete/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪫️battery/♒️delete/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪫️battery/♒️delete/🧪️tests/✅️applies/🦀️.rs"]
                            mod tests_applies;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪫️battery/♒️delete/🧪️tests/⛔️refuses/🦀️.rs"]
                            mod tests_refuses;
                        }
                        #[path = "."]
                        pub mod change_battery_capacity {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪫️battery/🥫️change-capacity/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪫️battery/🥫️change-capacity/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪫️battery/🥫️change-capacity/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪫️battery/🥫️change-capacity/🧪️tests/✅️applies/🦀️.rs"]
                            mod tests_applies;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪫️battery/🥫️change-capacity/🧪️tests/⛔️refuses/🦀️.rs"]
                            mod tests_refuses;
                        }
                        #[path = "."]
                        pub mod change_battery_max_charge {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪫️battery/⏫️change-max-charge/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪫️battery/⏫️change-max-charge/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪫️battery/⏫️change-max-charge/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪫️battery/⏫️change-max-charge/🧪️tests/✅️applies/🦀️.rs"]
                            mod tests_applies;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪫️battery/⏫️change-max-charge/🧪️tests/⛔️refuses/🦀️.rs"]
                            mod tests_refuses;
                        }
                        #[path = "."]
                        pub mod change_battery_max_discharge {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪫️battery/⏬️change-max-discharge/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪫️battery/⏬️change-max-discharge/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪫️battery/⏬️change-max-discharge/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪫️battery/⏬️change-max-discharge/🧪️tests/✅️applies/🦀️.rs"]
                            mod tests_applies;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪫️battery/⏬️change-max-discharge/🧪️tests/⛔️refuses/🦀️.rs"]
                            mod tests_refuses;
                        }
                        #[path = "."]
                        pub mod change_battery_round_trip_efficiency {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪫️battery/🥉️change-round-trip-efficiency/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪫️battery/🥉️change-round-trip-efficiency/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪫️battery/🥉️change-round-trip-efficiency/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪫️battery/🥉️change-round-trip-efficiency/🧪️tests/✅️applies/🦀️.rs"]
                            mod tests_applies;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪫️battery/🥉️change-round-trip-efficiency/🧪️tests/⛔️refuses/🦀️.rs"]
                            mod tests_refuses;
                        }
                        #[path = "."]
                        pub mod create_shw_system {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🚿️shw-system/🛀️create/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🚿️shw-system/🛀️create/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🚿️shw-system/🛀️create/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🚿️shw-system/🛀️create/🧪️tests/✅️applies/🦀️.rs"]
                            mod tests_applies;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🚿️shw-system/🛀️create/🧪️tests/⛔️refuses/🦀️.rs"]
                            mod tests_refuses;
                        }
                        #[path = "."]
                        pub mod delete_shw_system {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🚿️shw-system/🚱️delete/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🚿️shw-system/🚱️delete/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🚿️shw-system/🚱️delete/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🚿️shw-system/🚱️delete/🧪️tests/✅️applies/🦀️.rs"]
                            mod tests_applies;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🚿️shw-system/🚱️delete/🧪️tests/⛔️refuses/🦀️.rs"]
                            mod tests_refuses;
                        }
                        #[path = "."]
                        pub mod change_shw_system_heater_capacity {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🚿️shw-system/🍵️change-heater-capacity/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🚿️shw-system/🍵️change-heater-capacity/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🚿️shw-system/🍵️change-heater-capacity/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🚿️shw-system/🍵️change-heater-capacity/🧪️tests/✅️applies/🦀️.rs"]
                            mod tests_applies;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🚿️shw-system/🍵️change-heater-capacity/🧪️tests/⛔️refuses/🦀️.rs"]
                            mod tests_refuses;
                        }
                        #[path = "."]
                        pub mod change_shw_system_storage_volume {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🚿️shw-system/🛢️change-storage-volume/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🚿️shw-system/🛢️change-storage-volume/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🚿️shw-system/🛢️change-storage-volume/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🚿️shw-system/🛢️change-storage-volume/🧪️tests/✅️applies/🦀️.rs"]
                            mod tests_applies;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🚿️shw-system/🛢️change-storage-volume/🧪️tests/⛔️refuses/🦀️.rs"]
                            mod tests_refuses;
                        }
                        #[path = "."]
                        pub mod change_shw_system_setpoint {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🚿️shw-system/🏹️change-setpoint/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🚿️shw-system/🏹️change-setpoint/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🚿️shw-system/🏹️change-setpoint/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🚿️shw-system/🏹️change-setpoint/🧪️tests/✅️applies/🦀️.rs"]
                            mod tests_applies;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🚿️shw-system/🏹️change-setpoint/🧪️tests/⛔️refuses/🦀️.rs"]
                            mod tests_refuses;
                        }
                        #[path = "."]
                        pub mod change_shw_system_schedule {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🚿️shw-system/🕐️change-schedule/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🚿️shw-system/🕐️change-schedule/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🚿️shw-system/🕐️change-schedule/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🚿️shw-system/🕐️change-schedule/🧪️tests/✅️applies/🦀️.rs"]
                            mod tests_applies;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🚿️shw-system/🕐️change-schedule/🧪️tests/⛔️refuses/🦀️.rs"]
                            mod tests_refuses;
                        }
                        #[path = "."]
                        pub mod create_solar_thermal_system {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔥️solar-thermal/🌄️create/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔥️solar-thermal/🌄️create/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔥️solar-thermal/🌄️create/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔥️solar-thermal/🌄️create/🧪️tests/✅️applies/🦀️.rs"]
                            mod tests_applies;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔥️solar-thermal/🌄️create/🧪️tests/⛔️refuses/🦀️.rs"]
                            mod tests_refuses;
                        }
                        #[path = "."]
                        pub mod delete_solar_thermal_system {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔥️solar-thermal/🌆️delete/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔥️solar-thermal/🌆️delete/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔥️solar-thermal/🌆️delete/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔥️solar-thermal/🌆️delete/🧪️tests/✅️applies/🦀️.rs"]
                            mod tests_applies;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔥️solar-thermal/🌆️delete/🧪️tests/⛔️refuses/🦀️.rs"]
                            mod tests_refuses;
                        }
                        #[path = "."]
                        pub mod change_solar_thermal_system_collector_area {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔥️solar-thermal/🟩️change-collector-area/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔥️solar-thermal/🟩️change-collector-area/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔥️solar-thermal/🟩️change-collector-area/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔥️solar-thermal/🟩️change-collector-area/🧪️tests/✅️applies/🦀️.rs"]
                            mod tests_applies;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔥️solar-thermal/🟩️change-collector-area/🧪️tests/⛔️refuses/🦀️.rs"]
                            mod tests_refuses;
                        }
                        #[path = "."]
                        pub mod change_solar_thermal_system_efficiency {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔥️solar-thermal/🏅️change-efficiency/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔥️solar-thermal/🏅️change-efficiency/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔥️solar-thermal/🏅️change-efficiency/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔥️solar-thermal/🏅️change-efficiency/🧪️tests/✅️applies/🦀️.rs"]
                            mod tests_applies;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔥️solar-thermal/🏅️change-efficiency/🧪️tests/⛔️refuses/🦀️.rs"]
                            mod tests_refuses;
                        }
                        #[path = "."]
                        pub mod change_solar_thermal_system_storage_volume {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔥️solar-thermal/🧃️change-storage-volume/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔥️solar-thermal/🧃️change-storage-volume/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔥️solar-thermal/🧃️change-storage-volume/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔥️solar-thermal/🧃️change-storage-volume/🧪️tests/✅️applies/🦀️.rs"]
                            mod tests_applies;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔥️solar-thermal/🧃️change-storage-volume/🧪️tests/⛔️refuses/🦀️.rs"]
                            mod tests_refuses;
                        }
                        #[path = "."]
                        pub mod change_solar_thermal_system_tilt {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔥️solar-thermal/🔼️change-tilt/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔥️solar-thermal/🔼️change-tilt/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔥️solar-thermal/🔼️change-tilt/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔥️solar-thermal/🔼️change-tilt/🧪️tests/✅️applies/🦀️.rs"]
                            mod tests_applies;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔥️solar-thermal/🔼️change-tilt/🧪️tests/⛔️refuses/🦀️.rs"]
                            mod tests_refuses;
                        }
                        #[path = "."]
                        pub mod change_solar_thermal_system_azimuth {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔥️solar-thermal/⛵️change-azimuth/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔥️solar-thermal/⛵️change-azimuth/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔥️solar-thermal/⛵️change-azimuth/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔥️solar-thermal/⛵️change-azimuth/🧪️tests/✅️applies/🦀️.rs"]
                            mod tests_applies;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔥️solar-thermal/⛵️change-azimuth/🧪️tests/⛔️refuses/🦀️.rs"]
                            mod tests_refuses;
                        }
                        #[path = "."]
                        pub mod create_refrigeration_system {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/❄️refrigeration/❄️create/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/❄️refrigeration/❄️create/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/❄️refrigeration/❄️create/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/❄️refrigeration/❄️create/🧪️tests/✅️applies/🦀️.rs"]
                            mod tests_applies;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/❄️refrigeration/❄️create/🧪️tests/⛔️refuses/🦀️.rs"]
                            mod tests_refuses;
                        }
                        #[path = "."]
                        pub mod delete_refrigeration_system {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/❄️refrigeration/🫠️delete/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/❄️refrigeration/🫠️delete/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/❄️refrigeration/🫠️delete/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/❄️refrigeration/🫠️delete/🧪️tests/✅️applies/🦀️.rs"]
                            mod tests_applies;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/❄️refrigeration/🫠️delete/🧪️tests/⛔️refuses/🦀️.rs"]
                            mod tests_refuses;
                        }
                        #[path = "."]
                        pub mod change_refrigeration_system_case_count {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/❄️refrigeration/🗄️change-case-count/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/❄️refrigeration/🗄️change-case-count/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/❄️refrigeration/🗄️change-case-count/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/❄️refrigeration/🗄️change-case-count/🧪️tests/✅️applies/🦀️.rs"]
                            mod tests_applies;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/❄️refrigeration/🗄️change-case-count/🧪️tests/⛔️refuses/🦀️.rs"]
                            mod tests_refuses;
                        }
                        #[path = "."]
                        pub mod change_refrigeration_system_design_load {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/❄️refrigeration/🏋️change-design-load/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/❄️refrigeration/🏋️change-design-load/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/❄️refrigeration/🏋️change-design-load/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/❄️refrigeration/🏋️change-design-load/🧪️tests/✅️applies/🦀️.rs"]
                            mod tests_applies;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/❄️refrigeration/🏋️change-design-load/🧪️tests/⛔️refuses/🦀️.rs"]
                            mod tests_refuses;
                        }
                        #[path = "."]
                        pub mod change_refrigeration_system_defrost_schedule {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/❄️refrigeration/🕑️change-defrost-schedule/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/❄️refrigeration/🕑️change-defrost-schedule/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/❄️refrigeration/🕑️change-defrost-schedule/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/❄️refrigeration/🕑️change-defrost-schedule/🧪️tests/✅️applies/🦀️.rs"]
                            mod tests_applies;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/❄️refrigeration/🕑️change-defrost-schedule/🧪️tests/⛔️refuses/🦀️.rs"]
                            mod tests_refuses;
                        }
                        #[path = "."]
                        pub mod create_water_system {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🚰️water-system/🚽️create/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🚰️water-system/🚽️create/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🚰️water-system/🚽️create/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🚰️water-system/🚽️create/🧪️tests/✅️applies/🦀️.rs"]
                            mod tests_applies;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🚰️water-system/🚽️create/🧪️tests/⛔️refuses/🦀️.rs"]
                            mod tests_refuses;
                        }
                        #[path = "."]
                        pub mod delete_water_system {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🚰️water-system/🧼️delete/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🚰️water-system/🧼️delete/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🚰️water-system/🧼️delete/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🚰️water-system/🧼️delete/🧪️tests/✅️applies/🦀️.rs"]
                            mod tests_applies;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🚰️water-system/🧼️delete/🧪️tests/⛔️refuses/🦀️.rs"]
                            mod tests_refuses;
                        }
                        #[path = "."]
                        pub mod change_water_system_fixture_count {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🚰️water-system/🪣️change-fixture-count/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🚰️water-system/🪣️change-fixture-count/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🚰️water-system/🪣️change-fixture-count/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🚰️water-system/🪣️change-fixture-count/🧪️tests/✅️applies/🦀️.rs"]
                            mod tests_applies;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🚰️water-system/🪣️change-fixture-count/🧪️tests/⛔️refuses/🦀️.rs"]
                            mod tests_refuses;
                        }
                        #[path = "."]
                        pub mod change_water_system_peak_flow {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🚰️water-system/🚾️change-peak-flow/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🚰️water-system/🚾️change-peak-flow/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🚰️water-system/🚾️change-peak-flow/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🚰️water-system/🚾️change-peak-flow/🧪️tests/✅️applies/🦀️.rs"]
                            mod tests_applies;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🚰️water-system/🚾️change-peak-flow/🧪️tests/⛔️refuses/🦀️.rs"]
                            mod tests_refuses;
                        }
                        #[path = "."]
                        pub mod change_water_system_schedule {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🚰️water-system/🕒️change-schedule/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🚰️water-system/🕒️change-schedule/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🚰️water-system/🕒️change-schedule/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🚰️water-system/🕒️change-schedule/🧪️tests/✅️applies/🦀️.rs"]
                            mod tests_applies;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🚰️water-system/🕒️change-schedule/🧪️tests/⛔️refuses/🦀️.rs"]
                            mod tests_refuses;
                        }
                        #[path = "."]
                        pub mod create_fault {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/⚠️fault/⚠️create/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/⚠️fault/⚠️create/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/⚠️fault/⚠️create/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/⚠️fault/⚠️create/🧪️tests/✅️applies/🦀️.rs"]
                            mod tests_applies;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/⚠️fault/⚠️create/🧪️tests/⛔️refuses/🦀️.rs"]
                            mod tests_refuses;
                        }
                        #[path = "."]
                        pub mod delete_fault {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/⚠️fault/🩹️delete/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/⚠️fault/🩹️delete/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/⚠️fault/🩹️delete/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/⚠️fault/🩹️delete/🧪️tests/✅️applies/🦀️.rs"]
                            mod tests_applies;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/⚠️fault/🩹️delete/🧪️tests/⛔️refuses/🦀️.rs"]
                            mod tests_refuses;
                        }
                        #[path = "."]
                        pub mod change_fault_target_equipment {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/⚠️fault/🎣️change-target-equipment/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/⚠️fault/🎣️change-target-equipment/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/⚠️fault/🎣️change-target-equipment/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/⚠️fault/🎣️change-target-equipment/🧪️tests/✅️applies/🦀️.rs"]
                            mod tests_applies;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/⚠️fault/🎣️change-target-equipment/🧪️tests/⛔️refuses/🦀️.rs"]
                            mod tests_refuses;
                        }
                        #[path = "."]
                        pub mod change_fault_type {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/⚠️fault/🐛️change-type/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/⚠️fault/🐛️change-type/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/⚠️fault/🐛️change-type/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/⚠️fault/🐛️change-type/🧪️tests/✅️applies/🦀️.rs"]
                            mod tests_applies;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/⚠️fault/🐛️change-type/🧪️tests/⛔️refuses/🦀️.rs"]
                            mod tests_refuses;
                        }
                        #[path = "."]
                        pub mod change_fault_severity {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/⚠️fault/🌶️change-severity/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/⚠️fault/🌶️change-severity/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/⚠️fault/🌶️change-severity/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/⚠️fault/🌶️change-severity/🧪️tests/✅️applies/🦀️.rs"]
                            mod tests_applies;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/⚠️fault/🌶️change-severity/🧪️tests/⛔️refuses/🦀️.rs"]
                            mod tests_refuses;
                        }
                        #[path = "."]
                        pub mod change_fault_start_schedule {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/⚠️fault/🕓️change-start-schedule/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/⚠️fault/🕓️change-start-schedule/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/⚠️fault/🕓️change-start-schedule/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/⚠️fault/🕓️change-start-schedule/🧪️tests/✅️applies/🦀️.rs"]
                            mod tests_applies;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/⚠️fault/🕓️change-start-schedule/🧪️tests/⛔️refuses/🦀️.rs"]
                            mod tests_refuses;
                        }
                        #[path = "."]
                        pub mod create_space_list {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📋️space-list/📋️create/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📋️space-list/📋️create/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📋️space-list/📋️create/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📋️space-list/📋️create/🧪️tests/✅️applies/🦀️.rs"]
                            mod tests_applies;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📋️space-list/📋️create/🧪️tests/⛔️refuses/🦀️.rs"]
                            mod tests_refuses;
                        }
                        #[path = "."]
                        pub mod delete_space_list {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📋️space-list/🗒️delete/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📋️space-list/🗒️delete/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📋️space-list/🗒️delete/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📋️space-list/🗒️delete/🧪️tests/✅️applies/🦀️.rs"]
                            mod tests_applies;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📋️space-list/🗒️delete/🧪️tests/⛔️refuses/🦀️.rs"]
                            mod tests_refuses;
                        }
                        #[path = "."]
                        pub mod rename_space_list {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📋️space-list/🪶️rename/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📋️space-list/🪶️rename/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📋️space-list/🪶️rename/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📋️space-list/🪶️rename/🧪️tests/✅️applies/🦀️.rs"]
                            mod tests_applies;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📋️space-list/🪶️rename/🧪️tests/⛔️refuses/🦀️.rs"]
                            mod tests_refuses;
                        }
                        #[path = "."]
                        pub mod add_space_list_member {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📋️space-list/➡️add-member/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📋️space-list/➡️add-member/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📋️space-list/➡️add-member/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📋️space-list/➡️add-member/🧪️tests/✅️applies/🦀️.rs"]
                            mod tests_applies;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📋️space-list/➡️add-member/🧪️tests/⛔️refuses/🦀️.rs"]
                            mod tests_refuses;
                        }
                        #[path = "."]
                        pub mod remove_space_list_member {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📋️space-list/🪤️remove-member/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📋️space-list/🪤️remove-member/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📋️space-list/🪤️remove-member/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📋️space-list/🪤️remove-member/🧪️tests/✅️applies/🦀️.rs"]
                            mod tests_applies;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📋️space-list/🪤️remove-member/🧪️tests/⛔️refuses/🦀️.rs"]
                            mod tests_refuses;
                        }
                        #[path = "."]
                        pub mod create_thermal_enclosure {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔒️enclosure/🏟️create/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔒️enclosure/🏟️create/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔒️enclosure/🏟️create/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔒️enclosure/🏟️create/🧪️tests/✅️applies/🦀️.rs"]
                            mod tests_applies;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔒️enclosure/🏟️create/🧪️tests/⛔️refuses/🦀️.rs"]
                            mod tests_refuses;
                        }
                        #[path = "."]
                        pub mod delete_thermal_enclosure {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔒️enclosure/🏯️delete/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔒️enclosure/🏯️delete/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔒️enclosure/🏯️delete/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔒️enclosure/🏯️delete/🧪️tests/✅️applies/🦀️.rs"]
                            mod tests_applies;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔒️enclosure/🏯️delete/🧪️tests/⛔️refuses/🦀️.rs"]
                            mod tests_refuses;
                        }
                        #[path = "."]
                        pub mod rename_thermal_enclosure {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔒️enclosure/🖋️rename/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔒️enclosure/🖋️rename/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔒️enclosure/🖋️rename/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔒️enclosure/🖋️rename/🧪️tests/✅️applies/🦀️.rs"]
                            mod tests_applies;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔒️enclosure/🖋️rename/🧪️tests/⛔️refuses/🦀️.rs"]
                            mod tests_refuses;
                        }
                        #[path = "."]
                        pub mod add_thermal_enclosure_zone {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔒️enclosure/🔒️add-zone/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔒️enclosure/🔒️add-zone/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔒️enclosure/🔒️add-zone/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔒️enclosure/🔒️add-zone/🧪️tests/✅️applies/🦀️.rs"]
                            mod tests_applies;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔒️enclosure/🔒️add-zone/🧪️tests/⛔️refuses/🦀️.rs"]
                            mod tests_refuses;
                        }
                        #[path = "."]
                        pub mod remove_thermal_enclosure_zone {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔒️enclosure/🔓️remove-zone/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔒️enclosure/🔓️remove-zone/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔒️enclosure/🔓️remove-zone/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔒️enclosure/🔓️remove-zone/🧪️tests/✅️applies/🦀️.rs"]
                            mod tests_applies;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔒️enclosure/🔓️remove-zone/🧪️tests/⛔️refuses/🦀️.rs"]
                            mod tests_refuses;
                        }
                        #[path = "."]
                        pub mod create_constant_schedule {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🕐️fixed-schedule/🕜️create/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🕐️fixed-schedule/🕜️create/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🕐️fixed-schedule/🕜️create/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🕐️fixed-schedule/🕜️create/🧪️tests/✅️applies/🦀️.rs"]
                            mod tests_applies;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🕐️fixed-schedule/🕜️create/🧪️tests/⛔️refuses/🦀️.rs"]
                            mod tests_refuses;
                        }
                        #[path = "."]
                        pub mod delete_constant_schedule {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🕐️fixed-schedule/📍️delete/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🕐️fixed-schedule/📍️delete/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🕐️fixed-schedule/📍️delete/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🕐️fixed-schedule/📍️delete/🧪️tests/✅️applies/🦀️.rs"]
                            mod tests_applies;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🕐️fixed-schedule/📍️delete/🧪️tests/⛔️refuses/🦀️.rs"]
                            mod tests_refuses;
                        }
                        #[path = "."]
                        pub mod change_constant_schedule_value {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🕐️fixed-schedule/🕝️change-value/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🕐️fixed-schedule/🕝️change-value/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🕐️fixed-schedule/🕝️change-value/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🕐️fixed-schedule/🕝️change-value/🧪️tests/✅️applies/🦀️.rs"]
                            mod tests_applies;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🕐️fixed-schedule/🕝️change-value/🧪️tests/⛔️refuses/🦀️.rs"]
                            mod tests_refuses;
                        }
                        #[path = "."]
                        pub mod create_daily_schedule {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🕒️day-schedule/🕞️create/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🕒️day-schedule/🕞️create/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🕒️day-schedule/🕞️create/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🕒️day-schedule/🕞️create/🧪️tests/✅️applies/🦀️.rs"]
                            mod tests_applies;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🕒️day-schedule/🕞️create/🧪️tests/⛔️refuses/🦀️.rs"]
                            mod tests_refuses;
                        }
                        #[path = "."]
                        pub mod delete_daily_schedule {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🕒️day-schedule/🌓️delete/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🕒️day-schedule/🌓️delete/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🕒️day-schedule/🌓️delete/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🕒️day-schedule/🌓️delete/🧪️tests/✅️applies/🦀️.rs"]
                            mod tests_applies;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🕒️day-schedule/🌓️delete/🧪️tests/⛔️refuses/🦀️.rs"]
                            mod tests_refuses;
                        }
                        #[path = "."]
                        pub mod replace_daily_schedule_hourly_values {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🕒️day-schedule/🕔️replace-hourly-values/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🕒️day-schedule/🕔️replace-hourly-values/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🕒️day-schedule/🕔️replace-hourly-values/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🕒️day-schedule/🕔️replace-hourly-values/🧪️tests/✅️applies/🦀️.rs"]
                            mod tests_applies;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🕒️day-schedule/🕔️replace-hourly-values/🧪️tests/⛔️refuses/🦀️.rs"]
                            mod tests_refuses;
                        }
                        #[path = "."]
                        pub mod change_daily_schedule_interpolation {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🕒️day-schedule/🕕️change-interpolation/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🕒️day-schedule/🕕️change-interpolation/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🕒️day-schedule/🕕️change-interpolation/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🕒️day-schedule/🕕️change-interpolation/🧪️tests/✅️applies/🦀️.rs"]
                            mod tests_applies;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🕒️day-schedule/🕕️change-interpolation/🧪️tests/⛔️refuses/🦀️.rs"]
                            mod tests_refuses;
                        }
                        #[path = "."]
                        pub mod change_daily_schedule_limits {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🕒️day-schedule/🕟️change-limits/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🕒️day-schedule/🕟️change-limits/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🕒️day-schedule/🕟️change-limits/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🕒️day-schedule/🕟️change-limits/🧪️tests/✅️applies/🦀️.rs"]
                            mod tests_applies;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🕒️day-schedule/🕟️change-limits/🧪️tests/⛔️refuses/🦀️.rs"]
                            mod tests_refuses;
                        }
                        #[path = "."]
                        pub mod create_weekly_schedule {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🗓️week-schedule/🗓️create/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🗓️week-schedule/🗓️create/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🗓️week-schedule/🗓️create/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🗓️week-schedule/🗓️create/🧪️tests/✅️applies/🦀️.rs"]
                            mod tests_applies;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🗓️week-schedule/🗓️create/🧪️tests/⛔️refuses/🦀️.rs"]
                            mod tests_refuses;
                        }
                        #[path = "."]
                        pub mod delete_weekly_schedule {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🗓️week-schedule/🕖️delete/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🗓️week-schedule/🕖️delete/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🗓️week-schedule/🕖️delete/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🗓️week-schedule/🕖️delete/🧪️tests/✅️applies/🦀️.rs"]
                            mod tests_applies;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🗓️week-schedule/🕖️delete/🧪️tests/⛔️refuses/🦀️.rs"]
                            mod tests_refuses;
                        }
                        #[path = "."]
                        pub mod change_weekly_schedule_day {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🗓️week-schedule/🕗️change-day/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🗓️week-schedule/🕗️change-day/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🗓️week-schedule/🕗️change-day/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🗓️week-schedule/🕗️change-day/🧪️tests/✅️applies/🦀️.rs"]
                            mod tests_applies;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🗓️week-schedule/🕗️change-day/🧪️tests/⛔️refuses/🦀️.rs"]
                            mod tests_refuses;
                        }
                        #[path = "."]
                        pub mod create_annual_schedule {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📆️year-schedule/📚️create/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📆️year-schedule/📚️create/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📆️year-schedule/📚️create/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📆️year-schedule/📚️create/🧪️tests/✅️applies/🦀️.rs"]
                            mod tests_applies;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📆️year-schedule/📚️create/🧪️tests/⛔️refuses/🦀️.rs"]
                            mod tests_refuses;
                        }
                        #[path = "."]
                        pub mod delete_annual_schedule {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📆️year-schedule/📕️delete/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📆️year-schedule/📕️delete/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📆️year-schedule/📕️delete/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📆️year-schedule/📕️delete/🧪️tests/✅️applies/🦀️.rs"]
                            mod tests_applies;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📆️year-schedule/📕️delete/🧪️tests/⛔️refuses/🦀️.rs"]
                            mod tests_refuses;
                        }
                        #[path = "."]
                        pub mod insert_annual_schedule_rule {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📆️year-schedule/📗️insert-rule/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📆️year-schedule/📗️insert-rule/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📆️year-schedule/📗️insert-rule/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📆️year-schedule/📗️insert-rule/🧪️tests/✅️applies/🦀️.rs"]
                            mod tests_applies;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📆️year-schedule/📗️insert-rule/🧪️tests/⛔️refuses/🦀️.rs"]
                            mod tests_refuses;
                        }
                        #[path = "."]
                        pub mod remove_annual_schedule_rule {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📆️year-schedule/📙️remove-rule/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📆️year-schedule/📙️remove-rule/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📆️year-schedule/📙️remove-rule/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📆️year-schedule/📙️remove-rule/🧪️tests/✅️applies/🦀️.rs"]
                            mod tests_applies;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📆️year-schedule/📙️remove-rule/🧪️tests/⛔️refuses/🦀️.rs"]
                            mod tests_refuses;
                        }
                        #[path = "."]
                        pub mod reorder_annual_schedule_rules {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📆️year-schedule/🗂️reorder-rules/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📆️year-schedule/🗂️reorder-rules/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📆️year-schedule/🗂️reorder-rules/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📆️year-schedule/🗂️reorder-rules/🧪️tests/✅️applies/🦀️.rs"]
                            mod tests_applies;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📆️year-schedule/🗂️reorder-rules/🧪️tests/⛔️refuses/🦀️.rs"]
                            mod tests_refuses;
                        }
                        #[path = "."]
                        pub mod change_annual_schedule_default_daily_schedule {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📆️year-schedule/🎌️change-default-daily-schedule/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📆️year-schedule/🎌️change-default-daily-schedule/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📆️year-schedule/🎌️change-default-daily-schedule/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📆️year-schedule/🎌️change-default-daily-schedule/🧪️tests/✅️applies/🦀️.rs"]
                            mod tests_applies;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📆️year-schedule/🎌️change-default-daily-schedule/🧪️tests/⛔️refuses/🦀️.rs"]
                            mod tests_refuses;
                        }
                        #[path = "."]
                        pub mod change_annual_schedule_holiday_daily_schedule {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📆️year-schedule/🎄️change-holiday-daily-schedule/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📆️year-schedule/🎄️change-holiday-daily-schedule/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📆️year-schedule/🎄️change-holiday-daily-schedule/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📆️year-schedule/🎄️change-holiday-daily-schedule/🧪️tests/✅️applies/🦀️.rs"]
                            mod tests_applies;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📆️year-schedule/🎄️change-holiday-daily-schedule/🧪️tests/⛔️refuses/🦀️.rs"]
                            mod tests_refuses;
                        }
                        #[path = "."]
                        pub mod add_annual_schedule_holiday {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📆️year-schedule/🎉️add-holiday/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📆️year-schedule/🎉️add-holiday/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📆️year-schedule/🎉️add-holiday/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📆️year-schedule/🎉️add-holiday/🧪️tests/✅️applies/🦀️.rs"]
                            mod tests_applies;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📆️year-schedule/🎉️add-holiday/🧪️tests/⛔️refuses/🦀️.rs"]
                            mod tests_refuses;
                        }
                        #[path = "."]
                        pub mod remove_annual_schedule_holiday {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📆️year-schedule/🎊️remove-holiday/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📆️year-schedule/🎊️remove-holiday/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📆️year-schedule/🎊️remove-holiday/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📆️year-schedule/🎊️remove-holiday/🧪️tests/✅️applies/🦀️.rs"]
                            mod tests_applies;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📆️year-schedule/🎊️remove-holiday/🧪️tests/⛔️refuses/🦀️.rs"]
                            mod tests_refuses;
                        }
                        #[path = "."]
                        pub mod create_time_series_schedule {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📈️time-series/🪗️create/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📈️time-series/🪗️create/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📈️time-series/🪗️create/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📈️time-series/🪗️create/🧪️tests/✅️applies/🦀️.rs"]
                            mod tests_applies;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📈️time-series/🪗️create/🧪️tests/⛔️refuses/🦀️.rs"]
                            mod tests_refuses;
                        }
                        #[path = "."]
                        pub mod delete_time_series_schedule {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📈️time-series/🎞️delete/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📈️time-series/🎞️delete/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📈️time-series/🎞️delete/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📈️time-series/🎞️delete/🧪️tests/✅️applies/🦀️.rs"]
                            mod tests_applies;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📈️time-series/🎞️delete/🧪️tests/⛔️refuses/🦀️.rs"]
                            mod tests_refuses;
                        }
                        #[path = "."]
                        pub mod replace_time_series_schedule_values {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📈️time-series/🕘️replace-values/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📈️time-series/🕘️replace-values/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📈️time-series/🕘️replace-values/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📈️time-series/🕘️replace-values/🧪️tests/✅️applies/🦀️.rs"]
                            mod tests_applies;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📈️time-series/🕘️replace-values/🧪️tests/⛔️refuses/🦀️.rs"]
                            mod tests_refuses;
                        }
                        #[path = "."]
                        pub mod change_time_series_schedule_timestep {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📈️time-series/🕙️change-timestep/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📈️time-series/🕙️change-timestep/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📈️time-series/🕙️change-timestep/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📈️time-series/🕙️change-timestep/🧪️tests/✅️applies/🦀️.rs"]
                            mod tests_applies;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📈️time-series/🕙️change-timestep/🧪️tests/⛔️refuses/🦀️.rs"]
                            mod tests_refuses;
                        }
                        #[path = "."]
                        pub mod replace_fenestration_vertices {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪟️fenestration/🔶️replace-vertices/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪟️fenestration/🔶️replace-vertices/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪟️fenestration/🔶️replace-vertices/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪟️fenestration/🔶️replace-vertices/🧪️tests/✅️shapes/🦀️.rs"]
                            mod tests_shapes_a_gable;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪟️fenestration/🔶️replace-vertices/🧪️tests/⛔️refuses/🦀️.rs"]
                            mod tests_refuses_a_line;
                        }
                        #[path = "."]
                        pub mod change_glazing_material_thickness {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🥂️window-layer/🔷️change-glazing-thickness/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🥂️window-layer/🔷️change-glazing-thickness/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🥂️window-layer/🔷️change-glazing-thickness/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🥂️window-layer/🔷️change-glazing-thickness/🧪️tests/✅️applies/🦀️.rs"]
                            mod tests_applies;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🥂️window-layer/🔷️change-glazing-thickness/🧪️tests/⛔️refuses/🦀️.rs"]
                            mod tests_refuses;
                        }
                        #[path = "."]
                        pub mod change_glazing_material_conductivity {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🥂️window-layer/🟠️change-glazing-conductivity/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🥂️window-layer/🟠️change-glazing-conductivity/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🥂️window-layer/🟠️change-glazing-conductivity/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🥂️window-layer/🟠️change-glazing-conductivity/🧪️tests/✅️applies/🦀️.rs"]
                            mod tests_applies;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🥂️window-layer/🟠️change-glazing-conductivity/🧪️tests/⛔️refuses/🦀️.rs"]
                            mod tests_refuses;
                        }
                        #[path = "."]
                        pub mod change_glazing_material_solar_transmittance {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🥂️window-layer/🟡️change-glazing-solar-transmittance/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🥂️window-layer/🟡️change-glazing-solar-transmittance/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🥂️window-layer/🟡️change-glazing-solar-transmittance/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🥂️window-layer/🟡️change-glazing-solar-transmittance/🧪️tests/✅️applies/🦀️.rs"]
                            mod tests_applies;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🥂️window-layer/🟡️change-glazing-solar-transmittance/🧪️tests/⛔️refuses/🦀️.rs"]
                            mod tests_refuses;
                        }
                        #[path = "."]
                        pub mod change_glazing_material_visible_transmittance {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🥂️window-layer/🥽️change-glazing-visible-transmittance/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🥂️window-layer/🥽️change-glazing-visible-transmittance/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🥂️window-layer/🥽️change-glazing-visible-transmittance/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🥂️window-layer/🥽️change-glazing-visible-transmittance/🧪️tests/✅️applies/🦀️.rs"]
                            mod tests_applies;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🥂️window-layer/🥽️change-glazing-visible-transmittance/🧪️tests/⛔️refuses/🦀️.rs"]
                            mod tests_refuses;
                        }
                        #[path = "."]
                        pub mod change_glazing_material_infrared_emissivity {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🥂️window-layer/🩻️change-glazing-infrared-emissivity/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🥂️window-layer/🩻️change-glazing-infrared-emissivity/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🥂️window-layer/🩻️change-glazing-infrared-emissivity/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🥂️window-layer/🩻️change-glazing-infrared-emissivity/🧪️tests/✅️applies/🦀️.rs"]
                            mod tests_applies;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🥂️window-layer/🩻️change-glazing-infrared-emissivity/🧪️tests/⛔️refuses/🦀️.rs"]
                            mod tests_refuses;
                        }
                        #[path = "."]
                        pub mod rename_glazing_material {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🥂️window-layer/🟢️rename-glazing/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🥂️window-layer/🟢️rename-glazing/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🥂️window-layer/🟢️rename-glazing/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🥂️window-layer/🟢️rename-glazing/🧪️tests/✅️applies/🦀️.rs"]
                            mod tests_applies;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🥂️window-layer/🟢️rename-glazing/🧪️tests/⛔️refuses/🦀️.rs"]
                            mod tests_refuses;
                        }
                        #[path = "."]
                        pub mod change_gas_material_thickness {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🥂️window-layer/🟣️change-gas-thickness/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🥂️window-layer/🟣️change-gas-thickness/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🥂️window-layer/🟣️change-gas-thickness/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🥂️window-layer/🟣️change-gas-thickness/🧪️tests/✅️applies/🦀️.rs"]
                            mod tests_applies;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🥂️window-layer/🟣️change-gas-thickness/🧪️tests/⛔️refuses/🦀️.rs"]
                            mod tests_refuses;
                        }
                        #[path = "."]
                        pub mod change_gas_material_gas {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🥂️window-layer/🟤️change-gas-type/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🥂️window-layer/🟤️change-gas-type/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🥂️window-layer/🟤️change-gas-type/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🥂️window-layer/🟤️change-gas-type/🧪️tests/✅️applies/🦀️.rs"]
                            mod tests_applies;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🥂️window-layer/🟤️change-gas-type/🧪️tests/⛔️refuses/🦀️.rs"]
                            mod tests_refuses;
                        }
                        #[path = "."]
                        pub mod rename_gas_material {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🥂️window-layer/🔘️rename-gas/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🥂️window-layer/🔘️rename-gas/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🥂️window-layer/🔘️rename-gas/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🥂️window-layer/🔘️rename-gas/🧪️tests/✅️applies/🦀️.rs"]
                            mod tests_applies;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🥂️window-layer/🔘️rename-gas/🧪️tests/⛔️refuses/🦀️.rs"]
                            mod tests_refuses;
                        }
                        #[path = "."]
                        pub mod change_material_roughness {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🎨️finish/🗻️change-roughness/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🎨️finish/🗻️change-roughness/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🎨️finish/🗻️change-roughness/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🎨️finish/🗻️change-roughness/🧪️tests/✅️applies/🦀️.rs"]
                            mod tests_applies;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🎨️finish/🗻️change-roughness/🧪️tests/⛔️refuses/🦀️.rs"]
                            mod tests_refuses;
                        }
                    }
                }
                #[path = "."]
                pub mod io {
                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🦀️.rs"]
                    mod component;
                    pub use component::*;
                    #[path = "."]
                    pub mod import {
                        #[path = "."]
                        pub mod deserializers {
                            #[path = "."]
                            pub mod artifacts {
                                #[path = "."]
                                pub mod zip {
                                    #[path = "."]
                                    pub mod v2_0 {
                                        #[path = "."]
                                        pub mod any {
                                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/🎒️zip/🔖️2.0/✳️any/🦀️.rs"]
                                            mod component;
                                            pub use component::*;
                                        }
                                    }
                                }
                                #[path = "."]
                                pub mod txt {
                                    #[path = "."]
                                    pub mod v_utf_8 {
                                        #[path = "."]
                                        pub mod any {
                                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/🔤️txt/🔖️utf-8/✳️any/🦀️.rs"]
                                            mod component;
                                            pub use component::*;
                                        }
                                    }
                                }
                                #[path = "."]
                                pub mod json {
                                    #[path = "."]
                                    pub mod v_rfc8259 {
                                        #[path = "."]
                                        pub mod any {
                                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/🔣️json/🔖️rfc8259/✳️any/🦀️.rs"]
                                            mod component;
                                            pub use component::*;
                                        }
                                    }
                                }
                                #[path = "."]
                                pub mod epjson {
                                    #[path = "."]
                                    pub mod v25_2 {
                                        #[path = "."]
                                        pub mod any {
                                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/⚡️epjson/🔖️25.2/✳️any/🦀️.rs"]
                                            mod component;
                                            pub use component::*;
                                        }
                                    }
                                }
                            }
                        }
                    }
                    #[path = "."]
                    pub mod export {
                        #[path = "."]
                        pub mod serializers {
                            #[path = "."]
                            pub mod artifacts {
                                #[path = "."]
                                pub mod zip {
                                    #[path = "."]
                                    pub mod v2_0 {
                                        #[path = "."]
                                        pub mod any {
                                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📤️export/🧵️serializers/🗿️artifacts/🎒️zip/🔖️2.0/✳️any/🦀️.rs"]
                                            mod component;
                                            pub use component::*;
                                        }
                                    }
                                }
                                #[path = "."]
                                pub mod txt {
                                    #[path = "."]
                                    pub mod v_utf_8 {
                                        #[path = "."]
                                        pub mod any {
                                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📤️export/🧵️serializers/🗿️artifacts/🔤️txt/🔖️utf-8/✳️any/🦀️.rs"]
                                            mod component;
                                            pub use component::*;
                                        }
                                    }
                                }
                                #[path = "."]
                                pub mod json {
                                    #[path = "."]
                                    pub mod v_rfc8259 {
                                        #[path = "."]
                                        pub mod any {
                                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📤️export/🧵️serializers/🗿️artifacts/🔣️json/🔖️rfc8259/✳️any/🦀️.rs"]
                                            mod component;
                                            pub use component::*;
                                        }
                                    }
                                }
                                #[path = "."]
                                pub mod epjson {
                                    #[path = "."]
                                    pub mod v25_2 {
                                        #[path = "."]
                                        pub mod any {
                                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📤️export/🧵️serializers/🗿️artifacts/⚡️epjson/🔖️25.2/✳️any/🦀️.rs"]
                                            mod component;
                                            pub use component::*;
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

// ---- Shims: keep pre-migration module paths resolving for external callers ----
pub mod schema {
    pub use super::standards::v1::subsets::any::schema::*;
}




pub mod diff {
    pub use crate::standards::v1::subsets::any::schema::diff::*;
    pub mod schema {
        pub use crate::standards::v1::subsets::any::schema::diff::*;
    }



}
pub mod mutations {
    pub use crate::standards::v1::subsets::any::schema::mutations::*;
    pub mod schema {
        pub use crate::standards::v1::subsets::any::schema::mutations::*;
    }



}
pub mod snapshot {
    pub use crate::standards::v1::subsets::any::schema::snapshot::*;
    pub mod schema {
        pub use crate::standards::v1::subsets::any::schema::snapshot::*;
    }



}
pub use crate::standards::v1::subsets::any::schema::diff::EnergyModelDiff;
pub use crate::standards::v1::subsets::any::schema::mutations::EnergyModelMutation;
pub use crate::standards::v1::subsets::any::schema::snapshot::EnergyModelSnapshot;

#[path = "."]
pub mod examples {
    #[path = "."]
    pub mod demo {
        #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🎬️demo/🦀️.rs"]
        mod component;
        pub use component::*;
        #[cfg(test)]
        #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🎬️demo/🧪️tests/🧩️example/🦀️.rs"]
        mod tests;
    }
    #[path = "."]
    pub mod bestest_600 {
        #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🏛️bestest-600/🦀️.rs"]
        mod component;
        pub use component::*;
        #[cfg(test)]
        #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🏛️bestest-600/🧪️tests/🧩️example/🦀️.rs"]
        mod tests;
    }
    #[path = "."]
    pub mod bestest_600ff {
        #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🏛️bestest-600ff/🦀️.rs"]
        mod component;
        pub use component::*;
        #[cfg(test)]
        #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🏛️bestest-600ff/🧪️tests/🧩️example/🦀️.rs"]
        mod tests;
    }
    #[path = "."]
    pub mod bestest_610 {
        #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🏛️bestest-610/🦀️.rs"]
        mod component;
        pub use component::*;
        #[cfg(test)]
        #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🏛️bestest-610/🧪️tests/🧩️example/🦀️.rs"]
        mod tests;
    }
    #[path = "."]
    pub mod bestest_620 {
        #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🏛️bestest-620/🦀️.rs"]
        mod component;
        pub use component::*;
        #[cfg(test)]
        #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🏛️bestest-620/🧪️tests/🧩️example/🦀️.rs"]
        mod tests;
    }
    #[path = "."]
    pub mod bestest_630 {
        #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🏛️bestest-630/🦀️.rs"]
        mod component;
        pub use component::*;
        #[cfg(test)]
        #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🏛️bestest-630/🧪️tests/🧩️example/🦀️.rs"]
        mod tests;
    }
    #[path = "."]
    pub mod bestest_640 {
        #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🏛️bestest-640/🦀️.rs"]
        mod component;
        pub use component::*;
        #[cfg(test)]
        #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🏛️bestest-640/🧪️tests/🧩️example/🦀️.rs"]
        mod tests;
    }
    #[path = "."]
    pub mod bestest_650 {
        #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🏛️bestest-650/🦀️.rs"]
        mod component;
        pub use component::*;
        #[cfg(test)]
        #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🏛️bestest-650/🧪️tests/🧩️example/🦀️.rs"]
        mod tests;
    }
    #[path = "."]
    pub mod bestest_900 {
        #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🏛️bestest-900/🦀️.rs"]
        mod component;
        pub use component::*;
        #[cfg(test)]
        #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🏛️bestest-900/🧪️tests/🧩️example/🦀️.rs"]
        mod tests;
    }
    #[path = "."]
    pub mod bestest_900ff {
        #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🏛️bestest-900ff/🦀️.rs"]
        mod component;
        pub use component::*;
        #[cfg(test)]
        #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🏛️bestest-900ff/🧪️tests/🧩️example/🦀️.rs"]
        mod tests;
    }
    #[path = "."]
    pub mod bestest_910 {
        #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🏛️bestest-910/🦀️.rs"]
        mod component;
        pub use component::*;
        #[cfg(test)]
        #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🏛️bestest-910/🧪️tests/🧩️example/🦀️.rs"]
        mod tests;
    }
    #[path = "."]
    pub mod bestest_920 {
        #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🏛️bestest-920/🦀️.rs"]
        mod component;
        pub use component::*;
        #[cfg(test)]
        #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🏛️bestest-920/🧪️tests/🧩️example/🦀️.rs"]
        mod tests;
    }
    #[path = "."]
    pub mod bestest_930 {
        #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🏛️bestest-930/🦀️.rs"]
        mod component;
        pub use component::*;
        #[cfg(test)]
        #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🏛️bestest-930/🧪️tests/🧩️example/🦀️.rs"]
        mod tests;
    }
    #[path = "."]
    pub mod bestest_940 {
        #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🏛️bestest-940/🦀️.rs"]
        mod component;
        pub use component::*;
        #[cfg(test)]
        #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🏛️bestest-940/🧪️tests/🧩️example/🦀️.rs"]
        mod tests;
    }
    #[path = "."]
    pub mod bestest_950 {
        #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🏛️bestest-950/🦀️.rs"]
        mod component;
        pub use component::*;
        #[cfg(test)]
        #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🏛️bestest-950/🧪️tests/🧩️example/🦀️.rs"]
        mod tests;
    }    #[cfg(test)]
    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🧪️tests/🔬️bestest-catalog/🦀️.rs"]
    mod bestest_catalog_tests;
}

// 🎭️ Ticket 26/08/16/ARTIFACT-VIEWERS-AND-EDITORS-PER-SUBSET: energy's FIRST authored editor+viewer
// surfaces for `s.energy.model@1/*` — energy had zero document apps, so there was no app tree to
// migrate here (contrast the pilot's `📐️cad`, which moved an existing `🎛️apps/📐️cad/` tree). Two
// independent `#[path = "."]` trees, mirroring `🔖️Artifacts` above: `editor` mounts real
// mutation-capable content, `viewer` mounts an independently-authored read-only twin that never
// imports through `editor` (`policyViewerPurityBreaches`). The editor's `🎚️config` holds the simulation run
// settings; facet dirs that hold only `📌️.empty.md` (`🎮️commands`/`👥️presence`/`🫧️transient`) need no mount.
#[path = "."]
pub mod editor {
    #[path = "."]
    pub mod model {
        #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs"]
        mod component;
        pub use component::*;
        #[path = "."]
        pub mod presence {
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/👥️presence/🧬️schema/🦀️.rs"]
            pub mod schema;
        }

        #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎚️config/🦀️.rs"]
        pub mod config;
        #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🕹️interaction/🦀️.rs"]
        pub mod interaction;
        /// 📊️ Decoding the simulation run's per-surface tick payload into a colour map + legend.
        #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📊️results/🦀️.rs"]
        pub mod results;

        // 📌️ The two dock panels the editor contributes beside its windows: the artifact tree
        // (outliner) and the inspector, both reading the one `"energyModel"` interaction domain.
        #[path = "."]
        pub mod panels {
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📌️panels/🗿️artifact/🦀️.rs"]
            pub mod artifact;
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📌️panels/🔍️inspection/🦀️.rs"]
            pub mod inspection;
        }

        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod edit {
                #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🦀️.rs"]
                mod component;
                pub use component::*;

                #[path = "."]
                pub mod tools {
                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🛠️tools/⚡️simulation/🦀️.rs"]
                    pub mod simulation;
                }

                #[path = "."]
                pub mod windows {
                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/🧊️model/🦀️.rs"]
                    pub mod model;
                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/⚡️simulation/🦀️.rs"]
                    pub mod simulation;
                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/🌳️structure/🦀️.rs"]
                    pub mod structure;
                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/📊️zones/🦀️.rs"]
                    pub mod zones;
                }
            }
        }
    }
}

#[path = "."]
pub mod viewer {
    #[path = "."]
    pub mod model {
        #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/👁️viewer/🦀️.rs"]
        mod component;
        pub use component::*;
        #[path = "."]
        pub mod presence {
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/👁️viewer/👥️presence/🧬️schema/🦀️.rs"]
            pub mod schema;
        }
        #[path = "."]
        pub mod config {
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/👁️viewer/🎚️config/🧬️schema/🦀️.rs"]
            pub mod schema;
        }

        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod view {
                #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/👁️viewer/🎭️modes/👁️view/🦀️.rs"]
                mod component;
                pub use component::*;

                #[path = "."]
                pub mod windows {
                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/👁️viewer/🎭️modes/👁️view/🪟️windows/🧊️model/🦀️.rs"]
                    pub mod model;
                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/👁️viewer/🎭️modes/👁️view/🪟️windows/⚡️simulation/🦀️.rs"]
                    pub mod simulation;
                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/👁️viewer/🎭️modes/👁️view/🪟️windows/🌳️structure/🦀️.rs"]
                    pub mod structure;
                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/👁️viewer/🎭️modes/👁️view/🪟️windows/📊️zones/🦀️.rs"]
                    pub mod zones;
                }
            }
        }
    }
}

pub use crate::standards::v1::subsets::any::io::{ModelBuilderConstruction, EnergyModelParts, EnergyModelAnalyzerAnalysis, ModelBuilderFacets, ModelBuilder, EnergyModelAnalyzer, EnergyModelComposer};
