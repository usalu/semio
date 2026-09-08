//! 🎪 Energy model artifact — headless BEM document surface over `crate::Model`.

#![allow(clippy::too_many_arguments)]

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_schema as framework_schema;

//#region ⚡️SimulationEngine
#[path = "../../🔨️modules/⚡️simulation/⚙️engine/🔄️air_exchange/🦀️.rs"]
pub mod air_exchange;
#[path = "../../🔨️modules/⚡️simulation/⚙️engine/🌬️air_system/🦀️.rs"]
pub mod air_system;
#[path = "../../🔨️modules/⚡️simulation/⚙️engine/🫧️airflow_network/🦀️.rs"]
pub mod airflow_network;
#[path = "../../🔨️modules/⚡️simulation/⚙️engine/🏛️bestest/🦀️.rs"]
pub mod bestest;
#[path = "../../🔨️modules/⚡️simulation/⚙️engine/📅️calendar/🦀️.rs"]
pub mod calendar;
#[path = "../../🔨️modules/⚡️simulation/⚙️engine/🌀️coils/🦀️.rs"]
pub mod coils;
#[path = "../../🔨️modules/⚡️simulation/⚙️engine/🛋️comfort/🦀️.rs"]
pub mod comfort;
#[path = "../../🔨️modules/⚡️simulation/⚙️engine/🎛️controls/🦀️.rs"]
pub mod controls;
#[path = "../../🔨️modules/⚡️simulation/⚙️engine/📉️curves/🦀️.rs"]
pub mod curves;
#[path = "../../🔨️modules/⚡️simulation/⚙️engine/🌞️daylight/🦀️.rs"]
pub mod daylight;
#[path = "../../🔨️modules/⚡️simulation/⚙️engine/🚦️dispatch/🦀️.rs"]
pub mod dispatch;
#[path = "../../🔨️modules/⚡️simulation/⚙️engine/💰️economics/🦀️.rs"]
pub mod economics;
#[path = "../../🔨️modules/⚡️simulation/⚙️engine/⚡️electrical/🦀️.rs"]
pub mod electrical;
#[path = "../../🔨️modules/⚡️simulation/⚙️engine/🏢️envelope/🦀️.rs"]
pub mod envelope;
#[path = "../../🔨️modules/⚡️simulation/⚙️engine/🚨️error/🦀️.rs"]
pub mod error;
#[path = "../../🔨️modules/⚡️simulation/⚙️engine/🌫️evaporative/🦀️.rs"]
pub mod evaporative;
#[path = "../../🔨️modules/⚡️simulation/⚙️engine/🪭️fans/🦀️.rs"]
pub mod fans;
#[path = "../../🔨️modules/⚡️simulation/⚙️engine/⚠️faults/🦀️.rs"]
pub mod faults;
#[path = "../../🔨️modules/⚡️simulation/⚙️engine/🪟️fenestration/🦀️.rs"]
pub mod fenestration;
#[path = "../../🔨️modules/⚡️simulation/⚙️engine/📈️gains/🦀️.rs"]
pub mod gains;
#[path = "../../🔨️modules/⚡️simulation/⚙️engine/📐️geometry/🦀️.rs"]
pub mod geometry;
#[path = "../../🔨️modules/⚡️simulation/⚙️engine/♻️heat_recovery/🦀️.rs"]
pub mod heat_recovery;
#[path = "../../🔨️modules/⚡️simulation/⚙️engine/💦️humidity_eq/🦀️.rs"]
pub mod humidity_eq;
#[path = "../../🔨️modules/⚡️simulation/⚙️engine/🕸️hvac_topo/🦀️.rs"]
pub mod hvac_topo;
#[path = "../../🔨️modules/⚡️simulation/⚙️engine/🫁️iaq/🦀️.rs"]
pub mod iaq;
#[path = "../../🔨️modules/⚡️simulation/⚙️engine/💯️ideal_hvac/🦀️.rs"]
pub mod ideal_hvac;
#[path = "../../🔨️modules/⚡️simulation/⚙️engine/🌰️kernel/🦀️.rs"]
pub mod kernel;
#[path = "../../🔨️modules/⚡️simulation/⚙️engine/🧱️material/🦀️.rs"]
pub mod material;
#[path = "../../🔨️modules/⚡️simulation/⚙️engine/🧮️meters/🦀️.rs"]
pub mod meters;
#[path = "../../🔨️modules/⚡️simulation/⚙️engine/📊️metrics/🦀️.rs"]
pub mod metrics;
#[path = "../../🔨️modules/⚡️simulation/⚙️engine/🔋️model/🦀️.rs"]
pub mod model;
#[path = "../../🔨️modules/⚡️simulation/⚙️engine/🔢️num/🦀️.rs"]
pub mod num;
#[path = "../../🔨️modules/⚡️simulation/⚙️engine/📤️output/🦀️.rs"]
pub mod output;
#[path = "../../🔨️modules/⚡️simulation/⚙️engine/🏭️plant/🦀️.rs"]
pub mod plant;
#[path = "../../🔨️modules/⚡️simulation/⚙️engine/🧠️precompute/🦀️.rs"]
pub mod precompute;
#[path = "../../🔨️modules/⚡️simulation/⚙️engine/🧰️props/🦀️.rs"]
pub mod props;
#[path = "../../🔨️modules/⚡️simulation/⚙️engine/❄️refrigeration/🦀️.rs"]
pub mod refrigeration;
#[path = "../../🔨️modules/⚡️simulation/⚙️engine/🧾️results/🦀️.rs"]
pub mod results;
#[path = "../../🔨️modules/⚡️simulation/⚙️engine/🛏️room_air/🦀️.rs"]
pub mod room_air;
#[path = "../../🔨️modules/⚡️simulation/⚙️engine/🗓️schedule/🦀️.rs"]
pub mod schedule;
#[path = "../../🔨️modules/⚡️simulation/⚙️engine/🚿️shw/🦀️.rs"]
pub mod shw;
#[path = "../../🔨️modules/⚡️simulation/⚙️engine/🧪️sim/🦀️.rs"]
pub mod sim;
#[path = "../../🔨️modules/⚡️simulation/⚙️engine/📍️site/🦀️.rs"]
pub mod site;
#[path = "../../🔨️modules/⚡️simulation/⚙️engine/📏️sizing/🦀️.rs"]
pub mod sizing;
#[path = "../../🔨️modules/⚡️simulation/⚙️engine/☀️solar/🦀️.rs"]
pub mod solar;
#[path = "../../🔨️modules/⚡️simulation/⚙️engine/🔥️solar_thermal/🦀️.rs"]
pub mod solar_thermal;
#[path = "../../🔨️modules/⚡️simulation/⚙️engine/🔚️terminal/🦀️.rs"]
pub mod terminal;
#[path = "../../🔨️modules/⚡️simulation/⚙️engine/⚖️units/🦀️.rs"]
pub mod units;
#[path = "../../🔨️modules/⚡️simulation/⚙️engine/💧️water/🦀️.rs"]
pub mod water;
#[path = "../../🔨️modules/⚡️simulation/⚙️engine/🏠️zone_air/🦀️.rs"]
pub mod zone_air;
#[path = "../../🔨️modules/⚡️simulation/⚙️engine/🌡️zone_hvac/🦀️.rs"]
pub mod zone_hvac;
//#endregion ⚡️SimulationEngine

//#region 🔖️FlatReExports
pub use air_exchange::*;
pub use air_system::*;
pub use airflow_network::*;
pub use calendar::*;
pub use coils::*;
pub use comfort::*;
pub use controls::*;
pub use curves::*;
pub use daylight::*;
pub use dispatch::*;
pub use economics::*;
pub use electrical::*;
pub use envelope::*;
pub use error::*;
pub use evaporative::*;
pub use fans::*;
pub use faults::*;
pub use fenestration::*;
pub use gains::*;
pub use geometry::*;
pub use heat_recovery::*;
pub use humidity_eq::*;
pub use hvac_topo::*;
pub use iaq::*;
pub use ideal_hvac::{ideal_loads_deliver, ideal_loads_deliver_with_controls, EconomizerControl, HumidityControl, IdealLoadsConfig, IdealLoadsInput, IdealLoadsOutput, IdealLoadsRequest};
pub use kernel::*;
pub use material::*;
pub use meters::*;
pub use metrics::*;
pub use model::*;
pub use num::*;
pub use output::*;
pub use plant::*;
pub use precompute::*;
pub use props::*;
pub use refrigeration::*;
pub use results::*;
pub use room_air::*;
pub use schedule::*;
pub use shw::*;
pub use sim::*;
pub use site::*;
pub use sizing::*;
pub use solar::*;
pub use solar_thermal::*;
pub use terminal::*;
pub use units::*;
pub use water::*;
pub use zone_air::*;
pub use zone_hvac::*;
//#endregion 🔖️FlatReExports

/// ⚡️ Mounted simulation session shared by the artifact's editor and viewer.
#[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧵️simulation-session/🦀️.rs"]
pub mod energy_simulation_session;

use semio_framework_plugin::{ArtifactKindSpec, Dialect, MediaClass, MediaForm, MediaType, OsMediaCapability, StandardId, SubsetId};

pub use crate::schema::EnergyModelArtifact;

/// @emoji 🔖️ Document schema / DSL envelope id.
pub const ENERGY_MODEL_DOCUMENT_SCHEMA: &str = "energy.model";

/// @emoji 🧬️ Artifact schema descriptor id.
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
pub type EnergyStructureChild = store::ArtifactChild<semio_s_artifact_stdio_semio::standards::v1::subsets::value::schema::snapshot::SemioValueSnapshot>;
pub type EnergyZonesChild = store::ArtifactChild<semio_s_artifact_stdio_semio::standards::v1::subsets::table::schema::snapshot::SemioTableSnapshot>;
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
fn semio_value_from_json(value: &serde_json::Value) -> semio_s_artifact_stdio_semio::standards::v1::subsets::value::schema::snapshot::SemioValue {
    use semio_s_artifact_stdio_semio::standards::v1::subsets::value::schema::snapshot::{SemioValue, SemioValueEntry};
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
fn json_from_semio_value(value: &semio_s_artifact_stdio_semio::standards::v1::subsets::value::schema::snapshot::SemioValue) -> serde_json::Value {
    use semio_s_artifact_stdio_semio::standards::v1::subsets::value::schema::snapshot::SemioValue;
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
pub fn energy_structure_from_model(model: &Model) -> semio_s_artifact_stdio_semio::standards::v1::subsets::value::schema::snapshot::SemioValueSnapshot {
    use semio_s_artifact_stdio_semio::standards::v1::subsets::value::schema::snapshot::{SemioValueSnapshot, STDIO_SEMIOVALUE_DOCUMENT_SCHEMA};
    let value = serde_json::to_value(model).unwrap_or(serde_json::Value::Null);
    SemioValueSnapshot { schema: STDIO_SEMIOVALUE_DOCUMENT_SCHEMA.into(), root: semio_value_from_json(&value), nodes: Vec::new() }
}

/// 🌉 Inverse of [`energy_structure_from_model`]. Falls back to `Model::default()` if `structure`'s
/// root doesn't decode into a full `Model` (e.g. a foreign composer wrote a partial/foreign tree) —
/// documented, honest degradation, never a panic, matching the recipe's converter-honesty rule.
pub fn energy_model_from_structure(structure: &semio_s_artifact_stdio_semio::standards::v1::subsets::value::schema::snapshot::SemioValueSnapshot) -> Model {
    serde_json::from_value(json_from_semio_value(&structure.root)).unwrap_or_default()
}

/// 🌉 REAL, DERIVED converter: one `s.stdio.semio.table` row per `Zone` (`id`/`name`/`volumeM3`/
/// `multiplier`/`conditioned`/`partOfTotalFloorArea`) — a non-authoritative convenience projection,
/// always regenerated alongside `structure` from the SAME model (never an independent source, so
/// the two never diverge). `energy_model_from_structure` alone is authoritative on read; this table
/// is never consulted for reconstruction, mirroring `forms`'s own `results` table exactly.
pub fn energy_zones_table_from_model(model: &Model) -> semio_s_artifact_stdio_semio::standards::v1::subsets::table::schema::snapshot::SemioTableSnapshot {
    use semio_s_artifact_stdio_semio::standards::v1::subsets::table::schema::snapshot::{SemioTableCellKind, SemioTableColumn, SemioTableRow, SemioTableSnapshot, STDIO_SEMIOTABLE_DOCUMENT_SCHEMA};
    use semio_s_artifact_stdio_semio::standards::v1::subsets::value::schema::snapshot::SemioValue;
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
pub fn energy_structure_content(snapshot: &EnergyModelSnapshot) -> semio_s_artifact_stdio_semio::standards::v1::subsets::value::schema::snapshot::SemioValueSnapshot {
    energy_structure_from_model(&snapshot.model)
}

/// 🔎️ Twin of [`energy_structure_content`] for the `zones` child.
pub fn energy_zones_content(snapshot: &EnergyModelSnapshot) -> semio_s_artifact_stdio_semio::standards::v1::subsets::table::schema::snapshot::SemioTableSnapshot {
    energy_zones_table_from_model(&snapshot.model)
}
//#endregion 🔖️Converters

//#region 🔖️RetainedArtifactState
/// 🧵️ Exact immutable store owner used by mounted Energy capture. It exposes only a borrowed Model,
/// revalidates the store generation immediately before every read, and returns the precise snapshot
/// lease witness to the store retirement pump on close.
pub struct EnergyModelReadLease {
    snapshot: Option<store::SnapshotRead<EnergyModelSnapshot>>,
    returned: Option<store::SnapshotReadReturn>,
    generation: u64,
    revision: [u8; 32],
}

impl EnergyModelReadLease {
    pub fn new(snapshot: store::SnapshotRead<EnergyModelSnapshot>, generation: u64, revision: [u8; 32]) -> Self {
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
pub fn energy_children_from_model(_model: &Model) -> (EnergyStructureChild, EnergyZonesChild) {
    let scene_id = "energy-model".to_string();
    let dialect_for = |subset: &str| store::os_io::ArtifactDialect { artifact_kind: "s.stdio.semio".into(), standard: "v1".into(), subset: subset.into() };
    let target_for = |subset: &str| store::os_io::ArtifactRef { artifact_id: format!("energy-{subset}"), dialect: dialect_for(subset) };
    (store::ArtifactChild::new(scene_id.clone(), target_for("value")), store::ArtifactChild::new(scene_id, target_for("table")))
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
/// The effect carries a genesis `(pack, spr)` pair; the host loops it back as
/// `AppCommand::LoadDocument`, which is what actually calls [`store::ArtifactStore::reset`]
/// (`🔌️plugin/🦀️.rs`'s `load_document_pack`) — a guest plugin never holds the store itself. History
/// therefore starts empty by construction: loading a second document is no more undoable back into
/// the first than opening a second file is. Same shape as `📐️cad`'s `reset_document_effect` and
/// `🔱️trinity`'s; a freshly minted, edit-free envelope makes the spr encode infallible.
pub fn energy_model_load_document_effect(document_id: &str, model: &Model, links: &EnergyModelLinkSlots) -> semio_framework_plugin::kernel::Effect {
    let snapshot = energy_snapshot_with_links(model, links);
    let pack = <EnergyModelSnapshot as store::ArtifactPack>::encode_pack(&snapshot);
    let envelope = store::create_document_envelope::<EnergyModelSnapshot, EnergyModelMutation>(ENERGY_MODEL_DOCUMENT_SCHEMA, document_id, snapshot, None);
    let spr = semio_framework_plugin::resolve_ready(store::print_document_spr(&envelope)).expect("energy model document spr encode is infallible for a fresh, edit-free envelope");
    semio_framework_plugin::kernel::Effect::LoadDocument { pack, spr }
}
//#endregion ♻️WholeDocumentLoad

//#region 🔖️ArtifactKind
/// 🗂️ This artifact's `ArtifactKindSpec` — Data × Value per owner-table (`data.model`).
pub fn artifact_kind() -> ArtifactKindSpec {
    ArtifactKindSpec {
        id: "data.model".into(),
        name: "Energy Model".into(),
        source_format: ENERGY_MODEL_DOCUMENT_SCHEMA.into(),
        component_kind: "energy".into(),
        dimension: "data".into(),
        media_capability: OsMediaCapability::MeshOnly,
        media_type: MediaType { class: MediaClass::Data, form: MediaForm::Value },
        schema: ENERGY_MODEL_DOCUMENT_SCHEMA.into(),
        export_formats: vec![],
        import_formats: vec![],
        export_stdio_kinds: vec!["stdio.csv".into(), "stdio.json".into(), "stdio.xlsx".into(), "stdio.zip".into()],
        import_stdio_kinds: vec!["stdio.csv".into(), "stdio.json".into(), "stdio.xlsx".into(), "stdio.zip".into()],
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
        ("s.energy.model.composer.format-2", "composer", "s.stdio.csv@rfc4180/*", &[("dialect", "s.stdio.csv@rfc4180/*")], None),
        ("s.energy.model.composer.format-3", "composer", "s.stdio.xlsx@ecma-376/*", &[("dialect", "s.stdio.xlsx@ecma-376/*")], None),
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
        .document_codec_bare::<EnergyModelSnapshot, EnergyModelMutation>(ENERGY_MODEL_DOCUMENT_SCHEMA)
        .try_build()
}

/// 📌️ Handcrafted facet grammars (text) and protocols (binary) for in-process execution — built once
/// and leaked to a `&'static` slice since `dsl::passthrough_hooks` isn't `const fn`, mirroring the
/// `🗒️note` exemplar's helper of the same shape.
fn pilot_languages() -> &'static [dsl::LanguageSpec] {
    static LANGUAGES: std::sync::OnceLock<Vec<dsl::LanguageSpec>> = std::sync::OnceLock::new();
    LANGUAGES
        .get_or_init(|| {
            vec![
                dsl::LanguageSpec {
                    id: "energy.model",
                    extension: Some("energy"),
                    role: dsl::LanguageRole::Document,
                    grammar: Some(document_dsl::COMPONENT_GRAMMAR_SEMIO),
                    grammar_path: Some(document_dsl::COMPONENT_GRAMMAR_PATH),
                    protocol: Some(snapshot::pack::COMPONENT_PROTOCOL_SEMIO),
                    protocol_path: Some(snapshot::pack::COMPONENT_PROTOCOL_PATH),
                    hooks: dsl::passthrough_hooks("energy.model"),
                },
                dsl::LanguageSpec {
                    id: "energy.model.op",
                    extension: None,
                    role: dsl::LanguageRole::Ops,
                    grammar: Some(op::COMPONENT_GRAMMAR_SEMIO),
                    grammar_path: Some(op::COMPONENT_GRAMMAR_PATH),
                    protocol: Some(spr::COMPONENT_PROTOCOL_SEMIO),
                    protocol_path: Some(spr::COMPONENT_PROTOCOL_PATH),
                    hooks: dsl::passthrough_hooks("energy.model.op"),
                },
                dsl::LanguageSpec {
                    id: "energy.model.diff",
                    extension: None,
                    role: dsl::LanguageRole::Diff,
                    grammar: Some(diff::COMPONENT_GRAMMAR_SEMIO),
                    grammar_path: Some(diff::COMPONENT_GRAMMAR_PATH),
                    protocol: None,
                    protocol_path: None,
                    hooks: dsl::passthrough_hooks("energy.model.diff"),
                },
                dsl::LanguageSpec {
                    id: "energy.model.pack",
                    extension: None,
                    role: dsl::LanguageRole::Pack,
                    grammar: None,
                    grammar_path: None,
                    protocol: Some(snapshot::pack::COMPONENT_PROTOCOL_SEMIO),
                    protocol_path: Some(snapshot::pack::COMPONENT_PROTOCOL_PATH),
                    hooks: dsl::passthrough_hooks("energy.model.pack"),
                },
                dsl::LanguageSpec {
                    id: "energy.model.spr",
                    extension: None,
                    role: dsl::LanguageRole::Spr,
                    grammar: None,
                    grammar_path: None,
                    protocol: Some(spr::COMPONENT_PROTOCOL_SEMIO),
                    protocol_path: Some(spr::COMPONENT_PROTOCOL_PATH),
                    hooks: dsl::passthrough_hooks("energy.model.spr"),
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
                                #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/💾️binary/🦀️.rs"]
                                pub mod binary;
                                #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/📝️text/🦀️.rs"]
                                pub mod text;
                            }
                            #[path = "."]
                            pub mod inferences {
                                #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/🦀️.rs"]
                                mod component;
                                pub use component::*;
                                #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/💾️binary/🦀️.rs"]
                                pub mod binary;
                                #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/📝️text/🦀️.rs"]
                                pub mod text;
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
                                #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔺️diff/📝️text/🦀️.rs"]
                                pub mod text;
                                pub use text::*;
                                #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔺️diff/💾️binary/🦀️.rs"]
                                pub mod binary;
                            }
                            #[path = "."]
                            pub mod mutations {
                                #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️.rs"]
                                mod component;
                                pub use component::*;
                                #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💾️binary/🦀️.rs"]
                                pub mod binary;
                                #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📝️text/🦀️.rs"]
                                pub mod text;
                                #[path = "."]
                                pub mod rename_model {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏷️rename-model/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏷️rename-model/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏷️rename-model/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏷️rename-model/🧪️tests/✅️renames-the-model/🦀️.rs"]
                                    mod tests_renames_the_model;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏷️rename-model/🧪️tests/⛔️refuses-a-blank-name/🦀️.rs"]
                                    mod tests_refuses_a_blank_name;
                                }
                                #[path = "."]
                                pub mod change_model_version {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔢️change-model-version/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔢️change-model-version/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔢️change-model-version/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔢️change-model-version/🧪️tests/✅️bumps-the-version/🦀️.rs"]
                                    mod tests_bumps_the_version;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔢️change-model-version/🧪️tests/⛔️refuses-a-blank-version/🦀️.rs"]
                                    mod tests_refuses_a_blank_version;
                                }
                                #[path = "."]
                                pub mod update_site {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌍️update-site/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌍️update-site/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌍️update-site/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌍️update-site/🧪️tests/✅️relocates-to-denver/🦀️.rs"]
                                    mod tests_relocates_to_denver;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌍️update-site/🧪️tests/⛔️refuses-a-bad-latitude/🦀️.rs"]
                                    mod tests_refuses_a_bad_latitude;
                                }
                                #[path = "."]
                                pub mod update_ground_temperature {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌡️update-ground-temperature/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌡️update-ground-temperature/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌡️update-ground-temperature/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌡️update-ground-temperature/🧪️tests/✅️sets-denver-ground/🦀️.rs"]
                                    mod tests_sets_denver_ground;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌡️update-ground-temperature/🧪️tests/⛔️refuses-a-short-year/🦀️.rs"]
                                    mod tests_refuses_a_short_year;
                                }
                                #[path = "."]
                                pub mod update_run_period {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📅️update-run-period/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📅️update-run-period/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📅️update-run-period/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📅️update-run-period/🧪️tests/✅️shortens-to-january/🦀️.rs"]
                                    mod tests_shortens_to_january;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📅️update-run-period/🧪️tests/⛔️refuses-month-13/🦀️.rs"]
                                    mod tests_refuses_month_13;
                                }
                                #[path = "."]
                                pub mod replace_airflow_network {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🫧️replace-airflow-network/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🫧️replace-airflow-network/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🫧️replace-airflow-network/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🫧️replace-airflow-network/🧪️tests/✅️attaches-a-network/🦀️.rs"]
                                    mod tests_attaches_a_network;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🫧️replace-airflow-network/🧪️tests/⛔️refuses-unpaired-nodes/🦀️.rs"]
                                    mod tests_refuses_unpaired_nodes;
                                }
                                #[path = "."]
                                pub mod add_output_variable {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📊️add-output-variable/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📊️add-output-variable/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📊️add-output-variable/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📊️add-output-variable/🧪️tests/✅️adds-zone-air-temp/🦀️.rs"]
                                    mod tests_adds_zone_air_temp;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📊️add-output-variable/🧪️tests/⛔️refuses-a-duplicate/🦀️.rs"]
                                    mod tests_refuses_a_duplicate;
                                }
                                #[path = "."]
                                pub mod remove_output_variable {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📉️remove-output-variable/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📉️remove-output-variable/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📉️remove-output-variable/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📉️remove-output-variable/🧪️tests/✅️drops-zone-air-temp/🦀️.rs"]
                                    mod tests_drops_zone_air_temp;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📉️remove-output-variable/🧪️tests/⛔️refuses-an-absent-one/🦀️.rs"]
                                    mod tests_refuses_an_absent_one;
                                }
                                #[path = "."]
                                pub mod bind_weather_file {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌦️bind-weather-file/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌦️bind-weather-file/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌦️bind-weather-file/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌦️bind-weather-file/🧪️tests/✅️binds-hannover-epw/🦀️.rs"]
                                    mod tests_binds_hannover_epw;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌦️bind-weather-file/🧪️tests/⛔️refuses-a-bad-uri/🦀️.rs"]
                                    mod tests_refuses_a_bad_uri;
                                }
                                #[path = "."]
                                pub mod unbind_weather_file {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌤️unbind-weather-file/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌤️unbind-weather-file/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌤️unbind-weather-file/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌤️unbind-weather-file/🧪️tests/✅️unbinds-the-weather/🦀️.rs"]
                                    mod tests_unbinds_the_weather;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌤️unbind-weather-file/🧪️tests/⛔️refuses-when-unbound/🦀️.rs"]
                                    mod tests_refuses_when_unbound;
                                }
                                #[path = "."]
                                pub mod connect_referenced_model {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪢️connect-referenced-model/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪢️connect-referenced-model/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪢️connect-referenced-model/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪢️connect-referenced-model/🧪️tests/✅️connects-the-geometry/🦀️.rs"]
                                    mod tests_connects_the_geometry;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪢️connect-referenced-model/🧪️tests/⛔️refuses-a-bad-uri/🦀️.rs"]
                                    mod tests_refuses_a_bad_uri;
                                }
                                #[path = "."]
                                pub mod disconnect_referenced_model {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/✂️disconnect-referenced-model/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/✂️disconnect-referenced-model/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/✂️disconnect-referenced-model/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/✂️disconnect-referenced-model/🧪️tests/✅️disconnects-the-geometry/🦀️.rs"]
                                    mod tests_disconnects_the_geometry;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/✂️disconnect-referenced-model/🧪️tests/⛔️refuses-when-absent/🦀️.rs"]
                                    mod tests_refuses_when_absent;
                                }
                                #[path = "."]
                                pub mod rename_zone {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏠️rename-zone/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏠️rename-zone/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏠️rename-zone/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏠️rename-zone/🧪️tests/✅️renames-zone-one/🦀️.rs"]
                                    mod tests_renames_zone_one;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏠️rename-zone/🧪️tests/⛔️refuses-a-missing-zone/🦀️.rs"]
                                    mod tests_refuses_a_missing_zone;
                                }
                                #[path = "."]
                                pub mod change_zone_volume {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📦️change-zone-volume/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📦️change-zone-volume/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📦️change-zone-volume/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📦️change-zone-volume/🧪️tests/✅️resizes-zone-one/🦀️.rs"]
                                    mod tests_resizes_zone_one;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📦️change-zone-volume/🧪️tests/⛔️refuses-zero-volume/🦀️.rs"]
                                    mod tests_refuses_zero_volume;
                                }
                                #[path = "."]
                                pub mod change_zone_multiplier {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/✖️change-zone-multiplier/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/✖️change-zone-multiplier/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/✖️change-zone-multiplier/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/✖️change-zone-multiplier/🧪️tests/✅️stacks-four-storeys/🦀️.rs"]
                                    mod tests_stacks_four_storeys;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/✖️change-zone-multiplier/🧪️tests/⛔️refuses-zero-instances/🦀️.rs"]
                                    mod tests_refuses_zero_instances;
                                }
                                #[path = "."]
                                pub mod change_zone_conditioned {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌬️change-zone-conditioned/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌬️change-zone-conditioned/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌬️change-zone-conditioned/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌬️change-zone-conditioned/🧪️tests/✅️frees-the-zone/🦀️.rs"]
                                    mod tests_frees_the_zone;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌬️change-zone-conditioned/🧪️tests/⛔️refuses-a-missing-zone/🦀️.rs"]
                                    mod tests_refuses_a_missing_zone;
                                }
                                #[path = "."]
                                pub mod change_zone_floor_area_participation {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📐️change-zone-floor-area-participation/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📐️change-zone-floor-area-participation/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📐️change-zone-floor-area-participation/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📐️change-zone-floor-area-participation/🧪️tests/✅️excludes-the-zone/🦀️.rs"]
                                    mod tests_excludes_the_zone;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📐️change-zone-floor-area-participation/🧪️tests/⛔️refuses-a-missing-zone/🦀️.rs"]
                                    mod tests_refuses_a_missing_zone;
                                }
                                #[path = "."]
                                pub mod create_zone {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏘️create-zone/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏘️create-zone/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏘️create-zone/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏘️create-zone/🧪️tests/✅️adds-a-second-zone/🦀️.rs"]
                                    mod tests_adds_a_second_zone;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏘️create-zone/🧪️tests/⛔️refuses-a-taken-id/🦀️.rs"]
                                    mod tests_refuses_a_taken_id;
                                }
                                #[path = "."]
                                pub mod delete_zone {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏚️delete-zone/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏚️delete-zone/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏚️delete-zone/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏚️delete-zone/🧪️tests/✅️deletes-a-free-zone/🦀️.rs"]
                                    mod tests_deletes_a_free_zone;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏚️delete-zone/🧪️tests/⛔️refuses-a-used-zone/🦀️.rs"]
                                    mod tests_refuses_a_used_zone;
                                }
                                #[path = "."]
                                pub mod create_space {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪑️create-space/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪑️create-space/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪑️create-space/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪑️create-space/🧪️tests/✅️adds-a-space/🦀️.rs"]
                                    mod tests_adds_a_space;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪑️create-space/🧪️tests/⛔️refuses-a-missing-zone/🦀️.rs"]
                                    mod tests_refuses_a_missing_zone;
                                }
                                #[path = "."]
                                pub mod delete_space {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧹️delete-space/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧹️delete-space/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧹️delete-space/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧹️delete-space/🧪️tests/✅️deletes-a-free-space/🦀️.rs"]
                                    mod tests_deletes_a_free_space;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧹️delete-space/🧪️tests/⛔️refuses-a-missing-space/🦀️.rs"]
                                    mod tests_refuses_a_missing_space;
                                }
                                #[path = "."]
                                pub mod rename_space {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔤️rename-space/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔤️rename-space/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔤️rename-space/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔤️rename-space/🧪️tests/✅️renames-a-space/🦀️.rs"]
                                    mod tests_renames_a_space;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔤️rename-space/🧪️tests/⛔️refuses-a-blank-name/🦀️.rs"]
                                    mod tests_refuses_a_blank_name;
                                }
                                #[path = "."]
                                pub mod change_space_floor_area {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧮️change-space-floor-area/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧮️change-space-floor-area/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧮️change-space-floor-area/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧮️change-space-floor-area/🧪️tests/✅️resizes-a-space/🦀️.rs"]
                                    mod tests_resizes_a_space;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧮️change-space-floor-area/🧪️tests/⛔️refuses-negative-area/🦀️.rs"]
                                    mod tests_refuses_negative_area;
                                }
                                #[path = "."]
                                pub mod change_space_zone {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🚚️change-space-zone/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🚚️change-space-zone/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🚚️change-space-zone/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🚚️change-space-zone/🧪️tests/✅️moves-a-space/🦀️.rs"]
                                    mod tests_moves_a_space;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🚚️change-space-zone/🧪️tests/⛔️refuses-a-missing-zone/🦀️.rs"]
                                    mod tests_refuses_a_missing_zone;
                                }
                                #[path = "."]
                                pub mod create_surface {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🟫️create-surface/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🟫️create-surface/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🟫️create-surface/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🟫️create-surface/🧪️tests/✅️adds-a-south-wall/🦀️.rs"]
                                    mod tests_adds_a_south_wall;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🟫️create-surface/🧪️tests/⛔️refuses-two-vertices/🦀️.rs"]
                                    mod tests_refuses_two_vertices;
                                }
                                #[path = "."]
                                pub mod delete_surface {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪚️delete-surface/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪚️delete-surface/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪚️delete-surface/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪚️delete-surface/🧪️tests/✅️cascades-a-window/🦀️.rs"]
                                    mod tests_cascades_a_window;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪚️delete-surface/🧪️tests/⛔️refuses-a-partner/🦀️.rs"]
                                    mod tests_refuses_a_partner;
                                }
                                #[path = "."]
                                pub mod rename_surface {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏳️rename-surface/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏳️rename-surface/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏳️rename-surface/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏳️rename-surface/🧪️tests/✅️renames-a-wall/🦀️.rs"]
                                    mod tests_renames_a_wall;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏳️rename-surface/🧪️tests/⛔️refuses-a-taken-name/🦀️.rs"]
                                    mod tests_refuses_a_taken_name;
                                }
                                #[path = "."]
                                pub mod change_surface_zone {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🗜️change-surface-zone/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🗜️change-surface-zone/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🗜️change-surface-zone/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🗜️change-surface-zone/🧪️tests/✅️moves-a-wall/🦀️.rs"]
                                    mod tests_moves_a_wall;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🗜️change-surface-zone/🧪️tests/⛔️refuses-a-missing-zone/🦀️.rs"]
                                    mod tests_refuses_a_missing_zone;
                                }
                                #[path = "."]
                                pub mod change_surface_class {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧩️change-surface-class/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧩️change-surface-class/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧩️change-surface-class/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧩️change-surface-class/🧪️tests/✅️turns-a-wall-to-roof/🦀️.rs"]
                                    mod tests_turns_a_wall_to_roof;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧩️change-surface-class/🧪️tests/⛔️refuses-a-missing-one/🦀️.rs"]
                                    mod tests_refuses_a_missing_one;
                                }
                                #[path = "."]
                                pub mod replace_surface_vertices {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔺️replace-surface-vertices/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔺️replace-surface-vertices/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔺️replace-surface-vertices/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔺️replace-surface-vertices/🧪️tests/✅️narrows-a-wall/🦀️.rs"]
                                    mod tests_narrows_a_wall;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔺️replace-surface-vertices/🧪️tests/⛔️refuses-a-line/🦀️.rs"]
                                    mod tests_refuses_a_line;
                                }
                                #[path = "."]
                                pub mod change_surface_construction {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧰️change-surface-construction/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧰️change-surface-construction/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧰️change-surface-construction/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧰️change-surface-construction/🧪️tests/✅️swaps-the-wall-stack/🦀️.rs"]
                                    mod tests_swaps_the_wall_stack;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧰️change-surface-construction/🧪️tests/⛔️refuses-a-missing-one/🦀️.rs"]
                                    mod tests_refuses_a_missing_one;
                                }
                                #[path = "."]
                                pub mod change_surface_boundary_condition {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🚧️change-surface-boundary-condition/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🚧️change-surface-boundary-condition/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🚧️change-surface-boundary-condition/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🚧️change-surface-boundary-condition/🧪️tests/✅️grounds-a-floor/🦀️.rs"]
                                    mod tests_grounds_a_floor;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🚧️change-surface-boundary-condition/🧪️tests/⛔️refuses-half-a-union/🦀️.rs"]
                                    mod tests_refuses_half_a_union;
                                }
                                #[path = "."]
                                pub mod change_surface_sun_exposed {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌅️change-surface-sun-exposed/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌅️change-surface-sun-exposed/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌅️change-surface-sun-exposed/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌅️change-surface-sun-exposed/🧪️tests/✅️shades-a-wall/🦀️.rs"]
                                    mod tests_shades_a_wall;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌅️change-surface-sun-exposed/🧪️tests/⛔️refuses-a-missing-one/🦀️.rs"]
                                    mod tests_refuses_a_missing_one;
                                }
                                #[path = "."]
                                pub mod change_surface_wind_exposed {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🍃️change-surface-wind-exposed/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🍃️change-surface-wind-exposed/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🍃️change-surface-wind-exposed/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🍃️change-surface-wind-exposed/🧪️tests/✅️shelters-a-wall/🦀️.rs"]
                                    mod tests_shelters_a_wall;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🍃️change-surface-wind-exposed/🧪️tests/⛔️refuses-a-missing-one/🦀️.rs"]
                                    mod tests_refuses_a_missing_one;
                                }
                                #[path = "."]
                                pub mod change_surface_multiplier {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔁️change-surface-multiplier/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔁️change-surface-multiplier/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔁️change-surface-multiplier/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔁️change-surface-multiplier/🧪️tests/✅️repeats-a-wall/🦀️.rs"]
                                    mod tests_repeats_a_wall;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔁️change-surface-multiplier/🧪️tests/⛔️refuses-zero/🦀️.rs"]
                                    mod tests_refuses_zero;
                                }
                                #[path = "."]
                                pub mod create_fenestration {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪟️create-fenestration/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪟️create-fenestration/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪟️create-fenestration/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪟️create-fenestration/🧪️tests/✅️adds-a-south-window/🦀️.rs"]
                                    mod tests_adds_a_south_window;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪟️create-fenestration/🧪️tests/⛔️refuses-a-missing-host/🦀️.rs"]
                                    mod tests_refuses_a_missing_host;
                                }
                                #[path = "."]
                                pub mod delete_fenestration {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🚪️delete-fenestration/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🚪️delete-fenestration/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🚪️delete-fenestration/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🚪️delete-fenestration/🧪️tests/✅️removes-a-window/🦀️.rs"]
                                    mod tests_removes_a_window;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🚪️delete-fenestration/🧪️tests/⛔️refuses-a-missing-one/🦀️.rs"]
                                    mod tests_refuses_a_missing_one;
                                }
                                #[path = "."]
                                pub mod rename_fenestration {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏁️rename-fenestration/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏁️rename-fenestration/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏁️rename-fenestration/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏁️rename-fenestration/🧪️tests/✅️renames-a-window/🦀️.rs"]
                                    mod tests_renames_a_window;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏁️rename-fenestration/🧪️tests/⛔️refuses-a-blank-name/🦀️.rs"]
                                    mod tests_refuses_a_blank_name;
                                }
                                #[path = "."]
                                pub mod change_fenestration_surface {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧲️change-fenestration-surface/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧲️change-fenestration-surface/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧲️change-fenestration-surface/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧲️change-fenestration-surface/🧪️tests/✅️rehosts-a-window/🦀️.rs"]
                                    mod tests_rehosts_a_window;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧲️change-fenestration-surface/🧪️tests/⛔️refuses-a-missing-host/🦀️.rs"]
                                    mod tests_refuses_a_missing_host;
                                }
                                #[path = "."]
                                pub mod change_fenestration_u_value {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌐️change-fenestration-u-value/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌐️change-fenestration-u-value/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌐️change-fenestration-u-value/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌐️change-fenestration-u-value/🧪️tests/✅️swaps-the-glazing/🦀️.rs"]
                                    mod tests_swaps_the_glazing;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌐️change-fenestration-u-value/🧪️tests/⛔️refuses-zero-u/🦀️.rs"]
                                    mod tests_refuses_zero_u;
                                }
                                #[path = "."]
                                pub mod change_fenestration_shgc {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌇️change-fenestration-shgc/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌇️change-fenestration-shgc/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌇️change-fenestration-shgc/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌇️change-fenestration-shgc/🧪️tests/✅️dims-the-solar-gain/🦀️.rs"]
                                    mod tests_dims_the_solar_gain;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌇️change-fenestration-shgc/🧪️tests/⛔️refuses-shgc-above-one/🦀️.rs"]
                                    mod tests_refuses_shgc_above_one;
                                }
                                #[path = "."]
                                pub mod change_fenestration_vlt {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌈️change-fenestration-vlt/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌈️change-fenestration-vlt/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌈️change-fenestration-vlt/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌈️change-fenestration-vlt/🧪️tests/✅️dims-the-daylight/🦀️.rs"]
                                    mod tests_dims_the_daylight;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌈️change-fenestration-vlt/🧪️tests/⛔️refuses-negative-vlt/🦀️.rs"]
                                    mod tests_refuses_negative_vlt;
                                }
                                #[path = "."]
                                pub mod change_fenestration_area {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🟥️change-fenestration-area/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🟥️change-fenestration-area/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🟥️change-fenestration-area/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🟥️change-fenestration-area/🧪️tests/✅️doubles-the-glazing/🦀️.rs"]
                                    mod tests_doubles_the_glazing;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🟥️change-fenestration-area/🧪️tests/⛔️refuses-zero-area/🦀️.rs"]
                                    mod tests_refuses_zero_area;
                                }
                                #[path = "."]
                                pub mod change_fenestration_frame_conductance {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🖼️change-fenestration-frame-conductance/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🖼️change-fenestration-frame-conductance/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🖼️change-fenestration-frame-conductance/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🖼️change-fenestration-frame-conductance/🧪️tests/✅️adds-a-frame/🦀️.rs"]
                                    mod tests_adds_a_frame;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🖼️change-fenestration-frame-conductance/🧪️tests/⛔️refuses-negative-frame/🦀️.rs"]
                                    mod tests_refuses_negative_frame;
                                }
                                #[path = "."]
                                pub mod change_fenestration_divider_conductance {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧷️change-fenestration-divider-conductance/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧷️change-fenestration-divider-conductance/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧷️change-fenestration-divider-conductance/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧷️change-fenestration-divider-conductance/🧪️tests/✅️adds-dividers/🦀️.rs"]
                                    mod tests_adds_dividers;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧷️change-fenestration-divider-conductance/🧪️tests/⛔️refuses-a-negative/🦀️.rs"]
                                    mod tests_refuses_a_negative;
                                }
                                #[path = "."]
                                pub mod create_shading_surface {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌳️create-shading-surface/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌳️create-shading-surface/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌳️create-shading-surface/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌳️create-shading-surface/🧪️tests/✅️adds-an-awning/🦀️.rs"]
                                    mod tests_adds_an_awning;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌳️create-shading-surface/🧪️tests/⛔️refuses-a-line/🦀️.rs"]
                                    mod tests_refuses_a_line;
                                }
                                #[path = "."]
                                pub mod delete_shading_surface {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪵️delete-shading-surface/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪵️delete-shading-surface/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪵️delete-shading-surface/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪵️delete-shading-surface/🧪️tests/✅️removes-an-awning/🦀️.rs"]
                                    mod tests_removes_an_awning;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪵️delete-shading-surface/🧪️tests/⛔️refuses-a-missing-one/🦀️.rs"]
                                    mod tests_refuses_a_missing_one;
                                }
                                #[path = "."]
                                pub mod rename_shading_surface {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏕️rename-shading-surface/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏕️rename-shading-surface/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏕️rename-shading-surface/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏕️rename-shading-surface/🧪️tests/✅️renames-an-awning/🦀️.rs"]
                                    mod tests_renames_an_awning;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏕️rename-shading-surface/🧪️tests/⛔️refuses-a-blank-name/🦀️.rs"]
                                    mod tests_refuses_a_blank_name;
                                }
                                #[path = "."]
                                pub mod replace_shading_surface_vertices {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🗺️replace-shading-surface-vertices/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🗺️replace-shading-surface-vertices/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🗺️replace-shading-surface-vertices/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🗺️replace-shading-surface-vertices/🧪️tests/✅️deepens-an-awning/🦀️.rs"]
                                    mod tests_deepens_an_awning;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🗺️replace-shading-surface-vertices/🧪️tests/⛔️refuses-a-line/🦀️.rs"]
                                    mod tests_refuses_a_line;
                                }
                                #[path = "."]
                                pub mod change_shading_surface_transmittance_schedule {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/⛱️change-shading-surface-transmittance-schedule/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/⛱️change-shading-surface-transmittance-schedule/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/⛱️change-shading-surface-transmittance-schedule/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/⛱️change-shading-surface-transmittance-schedule/🧪️tests/✅️lets-light-in/🦀️.rs"]
                                    mod tests_lets_light_in;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/⛱️change-shading-surface-transmittance-schedule/🧪️tests/⛔️refuses-a-ghost/🦀️.rs"]
                                    mod tests_refuses_a_ghost;
                                }
                                #[path = "."]
                                pub mod connect_surfaces {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🤝️connect-surfaces/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🤝️connect-surfaces/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🤝️connect-surfaces/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🤝️connect-surfaces/🧪️tests/✅️joins-two-walls/🦀️.rs"]
                                    mod tests_joins_two_walls;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🤝️connect-surfaces/🧪️tests/⛔️refuses-a-self-pair/🦀️.rs"]
                                    mod tests_refuses_a_self_pair;
                                }
                                #[path = "."]
                                pub mod disconnect_surfaces {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💔️disconnect-surfaces/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💔️disconnect-surfaces/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💔️disconnect-surfaces/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💔️disconnect-surfaces/🧪️tests/✅️parts-two-walls/🦀️.rs"]
                                    mod tests_parts_two_walls;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💔️disconnect-surfaces/🧪️tests/⛔️refuses-a-missing-pair/🦀️.rs"]
                                    mod tests_refuses_a_missing_pair;
                                }
                                #[path = "."]
                                pub mod bind_fenestration_glazing_construction {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧊️bind-fenestration-glazing-construction/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧊️bind-fenestration-glazing-construction/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧊️bind-fenestration-glazing-construction/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧊️bind-fenestration-glazing-construction/🧪️tests/✅️glazes-a-window/🦀️.rs"]
                                    mod tests_glazes_a_window;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧊️bind-fenestration-glazing-construction/🧪️tests/⛔️refuses-no-stack/🦀️.rs"]
                                    mod tests_refuses_no_stack;
                                }
                                #[path = "."]
                                pub mod clear_fenestration_glazing_construction {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🫗️clear-fenestration-glazing-construction/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🫗️clear-fenestration-glazing-construction/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🫗️clear-fenestration-glazing-construction/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🫗️clear-fenestration-glazing-construction/🧪️tests/✅️ungazes-a-window/🦀️.rs"]
                                    mod tests_ungazes_a_window;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🫗️clear-fenestration-glazing-construction/🧪️tests/⛔️refuses-empty/🦀️.rs"]
                                    mod tests_refuses_empty;
                                }
                                #[path = "."]
                                pub mod change_fenestration_height {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/⬆️change-fenestration-height/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/⬆️change-fenestration-height/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/⬆️change-fenestration-height/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/⬆️change-fenestration-height/🧪️tests/✅️raises-the-head/🦀️.rs"]
                                    mod tests_raises_the_head;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/⬆️change-fenestration-height/🧪️tests/⛔️refuses-zero-height/🦀️.rs"]
                                    mod tests_refuses_zero_height;
                                }
                                #[path = "."]
                                pub mod change_fenestration_sill_height {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/⬇️change-fenestration-sill-height/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/⬇️change-fenestration-sill-height/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/⬇️change-fenestration-sill-height/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/⬇️change-fenestration-sill-height/🧪️tests/✅️raises-the-sill/🦀️.rs"]
                                    mod tests_raises_the_sill;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/⬇️change-fenestration-sill-height/🧪️tests/⛔️refuses-negative-sill/🦀️.rs"]
                                    mod tests_refuses_negative_sill;
                                }
                                #[path = "."]
                                pub mod change_fenestration_overhang_depth {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧢️change-fenestration-overhang-depth/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧢️change-fenestration-overhang-depth/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧢️change-fenestration-overhang-depth/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧢️change-fenestration-overhang-depth/🧪️tests/✅️adds-case-610-shade/🦀️.rs"]
                                    mod tests_adds_case_610_shade;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧢️change-fenestration-overhang-depth/🧪️tests/⛔️refuses-negative-depth/🦀️.rs"]
                                    mod tests_refuses_negative_depth;
                                }
                                #[path = "."]
                                pub mod change_fenestration_overhang_offset {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🎩️change-fenestration-overhang-offset/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🎩️change-fenestration-overhang-offset/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🎩️change-fenestration-overhang-offset/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🎩️change-fenestration-overhang-offset/🧪️tests/✅️lifts-the-overhang/🦀️.rs"]
                                    mod tests_lifts_the_overhang;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🎩️change-fenestration-overhang-offset/🧪️tests/⛔️refuses-negative-offset/🦀️.rs"]
                                    mod tests_refuses_negative_offset;
                                }
                                #[path = "."]
                                pub mod change_fenestration_fin_depth {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🐬️change-fenestration-fin-depth/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🐬️change-fenestration-fin-depth/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🐬️change-fenestration-fin-depth/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🐬️change-fenestration-fin-depth/🧪️tests/✅️adds-case-630-fins/🦀️.rs"]
                                    mod tests_adds_case_630_fins;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🐬️change-fenestration-fin-depth/🧪️tests/⛔️refuses-negative-fin/🦀️.rs"]
                                    mod tests_refuses_negative_fin;
                                }
                                #[path = "."]
                                pub mod change_fenestration_fin_offset {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🐋️change-fenestration-fin-offset/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🐋️change-fenestration-fin-offset/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🐋️change-fenestration-fin-offset/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🐋️change-fenestration-fin-offset/🧪️tests/✅️spreads-the-fins/🦀️.rs"]
                                    mod tests_spreads_the_fins;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🐋️change-fenestration-fin-offset/🧪️tests/⛔️refuses-negative-offset/🦀️.rs"]
                                    mod tests_refuses_negative_offset;
                                }
                                #[path = "."]
                                pub mod create_material {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧱️create-material/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧱️create-material/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧱️create-material/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧱️create-material/🧪️tests/✅️applies/🦀️.rs"]
                                    mod tests_applies;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧱️create-material/🧪️tests/⛔️refuses/🦀️.rs"]
                                    mod tests_refuses;
                                }
                                #[path = "."]
                                pub mod delete_material {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪨️delete-material/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪨️delete-material/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪨️delete-material/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪨️delete-material/🧪️tests/✅️applies/🦀️.rs"]
                                    mod tests_applies;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪨️delete-material/🧪️tests/⛔️refuses/🦀️.rs"]
                                    mod tests_refuses;
                                }
                                #[path = "."]
                                pub mod rename_material {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪧️rename-material/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪧️rename-material/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪧️rename-material/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪧️rename-material/🧪️tests/✅️applies/🦀️.rs"]
                                    mod tests_applies;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪧️rename-material/🧪️tests/⛔️refuses/🦀️.rs"]
                                    mod tests_refuses;
                                }
                                #[path = "."]
                                pub mod change_material_thickness {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📏️change-material-thickness/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📏️change-material-thickness/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📏️change-material-thickness/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📏️change-material-thickness/🧪️tests/✅️applies/🦀️.rs"]
                                    mod tests_applies;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📏️change-material-thickness/🧪️tests/⛔️refuses/🦀️.rs"]
                                    mod tests_refuses;
                                }
                                #[path = "."]
                                pub mod change_material_conductivity {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔥️change-material-conductivity/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔥️change-material-conductivity/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔥️change-material-conductivity/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔥️change-material-conductivity/🧪️tests/✅️applies/🦀️.rs"]
                                    mod tests_applies;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔥️change-material-conductivity/🧪️tests/⛔️refuses/🦀️.rs"]
                                    mod tests_refuses;
                                }
                                #[path = "."]
                                pub mod change_material_density {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/⚖️change-material-density/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/⚖️change-material-density/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/⚖️change-material-density/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/⚖️change-material-density/🧪️tests/✅️applies/🦀️.rs"]
                                    mod tests_applies;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/⚖️change-material-density/🧪️tests/⛔️refuses/🦀️.rs"]
                                    mod tests_refuses;
                                }
                                #[path = "."]
                                pub mod change_material_specific_heat {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/♨️change-material-specific-heat/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/♨️change-material-specific-heat/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/♨️change-material-specific-heat/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/♨️change-material-specific-heat/🧪️tests/✅️applies/🦀️.rs"]
                                    mod tests_applies;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/♨️change-material-specific-heat/🧪️tests/⛔️refuses/🦀️.rs"]
                                    mod tests_refuses;
                                }
                                #[path = "."]
                                pub mod change_material_thermal_absorptance {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔆️change-material-thermal-absorptance/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔆️change-material-thermal-absorptance/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔆️change-material-thermal-absorptance/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔆️change-material-thermal-absorptance/🧪️tests/✅️applies/🦀️.rs"]
                                    mod tests_applies;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔆️change-material-thermal-absorptance/🧪️tests/⛔️refuses/🦀️.rs"]
                                    mod tests_refuses;
                                }
                                #[path = "."]
                                pub mod change_material_solar_absorptance {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/☀️change-material-solar-absorptance/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/☀️change-material-solar-absorptance/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/☀️change-material-solar-absorptance/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/☀️change-material-solar-absorptance/🧪️tests/✅️applies/🦀️.rs"]
                                    mod tests_applies;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/☀️change-material-solar-absorptance/🧪️tests/⛔️refuses/🦀️.rs"]
                                    mod tests_refuses;
                                }
                                #[path = "."]
                                pub mod change_material_visible_absorptance {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/👁️change-material-visible-absorptance/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/👁️change-material-visible-absorptance/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/👁️change-material-visible-absorptance/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/👁️change-material-visible-absorptance/🧪️tests/✅️applies/🦀️.rs"]
                                    mod tests_applies;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/👁️change-material-visible-absorptance/🧪️tests/⛔️refuses/🦀️.rs"]
                                    mod tests_refuses;
                                }
                                #[path = "."]
                                pub mod create_construction {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏗️create-construction/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏗️create-construction/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏗️create-construction/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏗️create-construction/🧪️tests/✅️applies/🦀️.rs"]
                                    mod tests_applies;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏗️create-construction/🧪️tests/⛔️refuses/🦀️.rs"]
                                    mod tests_refuses;
                                }
                                #[path = "."]
                                pub mod delete_construction {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧨️delete-construction/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧨️delete-construction/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧨️delete-construction/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧨️delete-construction/🧪️tests/✅️applies/🦀️.rs"]
                                    mod tests_applies;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧨️delete-construction/🧪️tests/⛔️refuses/🦀️.rs"]
                                    mod tests_refuses;
                                }
                                #[path = "."]
                                pub mod rename_construction {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪪️rename-construction/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪪️rename-construction/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪪️rename-construction/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪪️rename-construction/🧪️tests/✅️applies/🦀️.rs"]
                                    mod tests_applies;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪪️rename-construction/🧪️tests/⛔️refuses/🦀️.rs"]
                                    mod tests_refuses;
                                }
                                #[path = "."]
                                pub mod add_construction_layer {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/➕️add-construction-layer/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/➕️add-construction-layer/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/➕️add-construction-layer/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/➕️add-construction-layer/🧪️tests/✅️applies/🦀️.rs"]
                                    mod tests_applies;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/➕️add-construction-layer/🧪️tests/⛔️refuses/🦀️.rs"]
                                    mod tests_refuses;
                                }
                                #[path = "."]
                                pub mod remove_construction_layer {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/➖️remove-construction-layer/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/➖️remove-construction-layer/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/➖️remove-construction-layer/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/➖️remove-construction-layer/🧪️tests/✅️applies/🦀️.rs"]
                                    mod tests_applies;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/➖️remove-construction-layer/🧪️tests/⛔️refuses/🦀️.rs"]
                                    mod tests_refuses;
                                }
                                #[path = "."]
                                pub mod reorder_construction_layers {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔀️reorder-construction-layers/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔀️reorder-construction-layers/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔀️reorder-construction-layers/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔀️reorder-construction-layers/🧪️tests/✅️applies/🦀️.rs"]
                                    mod tests_applies;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔀️reorder-construction-layers/🧪️tests/⛔️refuses/🦀️.rs"]
                                    mod tests_refuses;
                                }
                                #[path = "."]
                                pub mod create_people_gain {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/👤️create-people-gain/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/👤️create-people-gain/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/👤️create-people-gain/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/👤️create-people-gain/🧪️tests/✅️applies/🦀️.rs"]
                                    mod tests_applies;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/👤️create-people-gain/🧪️tests/⛔️refuses/🦀️.rs"]
                                    mod tests_refuses;
                                }
                                #[path = "."]
                                pub mod delete_people_gain {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🚷️delete-people-gain/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🚷️delete-people-gain/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🚷️delete-people-gain/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🚷️delete-people-gain/🧪️tests/✅️applies/🦀️.rs"]
                                    mod tests_applies;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🚷️delete-people-gain/🧪️tests/⛔️refuses/🦀️.rs"]
                                    mod tests_refuses;
                                }
                                #[path = "."]
                                pub mod change_people_gain_zone {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🚶️change-people-gain-zone/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🚶️change-people-gain-zone/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🚶️change-people-gain-zone/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🚶️change-people-gain-zone/🧪️tests/✅️applies/🦀️.rs"]
                                    mod tests_applies;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🚶️change-people-gain-zone/🧪️tests/⛔️refuses/🦀️.rs"]
                                    mod tests_refuses;
                                }
                                #[path = "."]
                                pub mod change_people_gain_schedule {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/⏰️change-people-gain-schedule/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/⏰️change-people-gain-schedule/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/⏰️change-people-gain-schedule/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/⏰️change-people-gain-schedule/🧪️tests/✅️applies/🦀️.rs"]
                                    mod tests_applies;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/⏰️change-people-gain-schedule/🧪️tests/⛔️refuses/🦀️.rs"]
                                    mod tests_refuses;
                                }
                                #[path = "."]
                                pub mod change_people_gain_activity_schedule {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏃️change-people-gain-activity-schedule/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏃️change-people-gain-activity-schedule/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏃️change-people-gain-activity-schedule/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏃️change-people-gain-activity-schedule/🧪️tests/✅️applies/🦀️.rs"]
                                    mod tests_applies;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏃️change-people-gain-activity-schedule/🧪️tests/⛔️refuses/🦀️.rs"]
                                    mod tests_refuses;
                                }
                                #[path = "."]
                                pub mod change_people_gain_people_per_area {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/👥️change-people-gain-people-per-area/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/👥️change-people-gain-people-per-area/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/👥️change-people-gain-people-per-area/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/👥️change-people-gain-people-per-area/🧪️tests/✅️applies/🦀️.rs"]
                                    mod tests_applies;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/👥️change-people-gain-people-per-area/🧪️tests/⛔️refuses/🦀️.rs"]
                                    mod tests_refuses;
                                }
                                #[path = "."]
                                pub mod change_people_gain_sensible_fraction {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌞️change-people-gain-sensible-fraction/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌞️change-people-gain-sensible-fraction/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌞️change-people-gain-sensible-fraction/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌞️change-people-gain-sensible-fraction/🧪️tests/✅️applies/🦀️.rs"]
                                    mod tests_applies;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌞️change-people-gain-sensible-fraction/🧪️tests/⛔️refuses/🦀️.rs"]
                                    mod tests_refuses;
                                }
                                #[path = "."]
                                pub mod change_people_gain_latent_fraction {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💧️change-people-gain-latent-fraction/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💧️change-people-gain-latent-fraction/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💧️change-people-gain-latent-fraction/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💧️change-people-gain-latent-fraction/🧪️tests/✅️applies/🦀️.rs"]
                                    mod tests_applies;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💧️change-people-gain-latent-fraction/🧪️tests/⛔️refuses/🦀️.rs"]
                                    mod tests_refuses;
                                }
                                #[path = "."]
                                pub mod change_people_gain_radiant_fraction {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📡️change-people-gain-radiant-fraction/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📡️change-people-gain-radiant-fraction/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📡️change-people-gain-radiant-fraction/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📡️change-people-gain-radiant-fraction/🧪️tests/✅️applies/🦀️.rs"]
                                    mod tests_applies;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📡️change-people-gain-radiant-fraction/🧪️tests/⛔️refuses/🦀️.rs"]
                                    mod tests_refuses;
                                }
                                #[path = "."]
                                pub mod create_lighting_gain {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💡️create-lighting-gain/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💡️create-lighting-gain/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💡️create-lighting-gain/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💡️create-lighting-gain/🧪️tests/✅️applies/🦀️.rs"]
                                    mod tests_applies;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💡️create-lighting-gain/🧪️tests/⛔️refuses/🦀️.rs"]
                                    mod tests_refuses;
                                }
                                #[path = "."]
                                pub mod delete_lighting_gain {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🕯️delete-lighting-gain/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🕯️delete-lighting-gain/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🕯️delete-lighting-gain/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🕯️delete-lighting-gain/🧪️tests/✅️applies/🦀️.rs"]
                                    mod tests_applies;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🕯️delete-lighting-gain/🧪️tests/⛔️refuses/🦀️.rs"]
                                    mod tests_refuses;
                                }
                                #[path = "."]
                                pub mod change_lighting_gain_zone {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔦️change-lighting-gain-zone/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔦️change-lighting-gain-zone/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔦️change-lighting-gain-zone/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔦️change-lighting-gain-zone/🧪️tests/✅️applies/🦀️.rs"]
                                    mod tests_applies;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔦️change-lighting-gain-zone/🧪️tests/⛔️refuses/🦀️.rs"]
                                    mod tests_refuses;
                                }
                                #[path = "."]
                                pub mod change_lighting_gain_schedule {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/⏱️change-lighting-gain-schedule/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/⏱️change-lighting-gain-schedule/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/⏱️change-lighting-gain-schedule/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/⏱️change-lighting-gain-schedule/🧪️tests/✅️applies/🦀️.rs"]
                                    mod tests_applies;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/⏱️change-lighting-gain-schedule/🧪️tests/⛔️refuses/🦀️.rs"]
                                    mod tests_refuses;
                                }
                                #[path = "."]
                                pub mod change_lighting_gain_watts_per_area {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔌️change-lighting-gain-watts-per-area/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔌️change-lighting-gain-watts-per-area/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔌️change-lighting-gain-watts-per-area/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔌️change-lighting-gain-watts-per-area/🧪️tests/✅️applies/🦀️.rs"]
                                    mod tests_applies;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔌️change-lighting-gain-watts-per-area/🧪️tests/⛔️refuses/🦀️.rs"]
                                    mod tests_refuses;
                                }
                                #[path = "."]
                                pub mod change_lighting_gain_radiant_fraction {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌟️change-lighting-gain-radiant-fraction/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌟️change-lighting-gain-radiant-fraction/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌟️change-lighting-gain-radiant-fraction/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌟️change-lighting-gain-radiant-fraction/🧪️tests/✅️applies/🦀️.rs"]
                                    mod tests_applies;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌟️change-lighting-gain-radiant-fraction/🧪️tests/⛔️refuses/🦀️.rs"]
                                    mod tests_refuses;
                                }
                                #[path = "."]
                                pub mod change_lighting_gain_visible_fraction {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔅️change-lighting-gain-visible-fraction/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔅️change-lighting-gain-visible-fraction/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔅️change-lighting-gain-visible-fraction/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔅️change-lighting-gain-visible-fraction/🧪️tests/✅️applies/🦀️.rs"]
                                    mod tests_applies;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔅️change-lighting-gain-visible-fraction/🧪️tests/⛔️refuses/🦀️.rs"]
                                    mod tests_refuses;
                                }
                                #[path = "."]
                                pub mod change_lighting_gain_return_air_fraction {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🎐️change-lighting-gain-return-air-fraction/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🎐️change-lighting-gain-return-air-fraction/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🎐️change-lighting-gain-return-air-fraction/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🎐️change-lighting-gain-return-air-fraction/🧪️tests/✅️applies/🦀️.rs"]
                                    mod tests_applies;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🎐️change-lighting-gain-return-air-fraction/🧪️tests/⛔️refuses/🦀️.rs"]
                                    mod tests_refuses;
                                }
                                #[path = "."]
                                pub mod create_equipment_gain {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🖥️create-equipment-gain/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🖥️create-equipment-gain/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🖥️create-equipment-gain/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🖥️create-equipment-gain/🧪️tests/✅️applies/🦀️.rs"]
                                    mod tests_applies;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🖥️create-equipment-gain/🧪️tests/⛔️refuses/🦀️.rs"]
                                    mod tests_refuses;
                                }
                                #[path = "."]
                                pub mod delete_equipment_gain {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧯️delete-equipment-gain/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧯️delete-equipment-gain/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧯️delete-equipment-gain/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧯️delete-equipment-gain/🧪️tests/✅️applies/🦀️.rs"]
                                    mod tests_applies;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧯️delete-equipment-gain/🧪️tests/⛔️refuses/🦀️.rs"]
                                    mod tests_refuses;
                                }
                                #[path = "."]
                                pub mod change_equipment_gain_zone {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🖨️change-equipment-gain-zone/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🖨️change-equipment-gain-zone/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🖨️change-equipment-gain-zone/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🖨️change-equipment-gain-zone/🧪️tests/✅️applies/🦀️.rs"]
                                    mod tests_applies;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🖨️change-equipment-gain-zone/🧪️tests/⛔️refuses/🦀️.rs"]
                                    mod tests_refuses;
                                }
                                #[path = "."]
                                pub mod change_equipment_gain_schedule {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/⌛️change-equipment-gain-schedule/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/⌛️change-equipment-gain-schedule/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/⌛️change-equipment-gain-schedule/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/⌛️change-equipment-gain-schedule/🧪️tests/✅️applies/🦀️.rs"]
                                    mod tests_applies;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/⌛️change-equipment-gain-schedule/🧪️tests/⛔️refuses/🦀️.rs"]
                                    mod tests_refuses;
                                }
                                #[path = "."]
                                pub mod change_equipment_gain_watts_per_area {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/⚡️change-equipment-gain-watts-per-area/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/⚡️change-equipment-gain-watts-per-area/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/⚡️change-equipment-gain-watts-per-area/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/⚡️change-equipment-gain-watts-per-area/🧪️tests/✅️applies/🦀️.rs"]
                                    mod tests_applies;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/⚡️change-equipment-gain-watts-per-area/🧪️tests/⛔️refuses/🦀️.rs"]
                                    mod tests_refuses;
                                }
                                #[path = "."]
                                pub mod change_equipment_gain_radiant_fraction {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌠️change-equipment-gain-radiant-fraction/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌠️change-equipment-gain-radiant-fraction/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌠️change-equipment-gain-radiant-fraction/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌠️change-equipment-gain-radiant-fraction/🧪️tests/✅️applies/🦀️.rs"]
                                    mod tests_applies;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌠️change-equipment-gain-radiant-fraction/🧪️tests/⛔️refuses/🦀️.rs"]
                                    mod tests_refuses;
                                }
                                #[path = "."]
                                pub mod change_equipment_gain_latent_fraction {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💦️change-equipment-gain-latent-fraction/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💦️change-equipment-gain-latent-fraction/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💦️change-equipment-gain-latent-fraction/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💦️change-equipment-gain-latent-fraction/🧪️tests/✅️applies/🦀️.rs"]
                                    mod tests_applies;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💦️change-equipment-gain-latent-fraction/🧪️tests/⛔️refuses/🦀️.rs"]
                                    mod tests_refuses;
                                }
                                #[path = "."]
                                pub mod create_infiltration {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💨️create-infiltration/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💨️create-infiltration/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💨️create-infiltration/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💨️create-infiltration/🧪️tests/✅️applies/🦀️.rs"]
                                    mod tests_applies;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💨️create-infiltration/🧪️tests/⛔️refuses/🦀️.rs"]
                                    mod tests_refuses;
                                }
                                #[path = "."]
                                pub mod delete_infiltration {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧽️delete-infiltration/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧽️delete-infiltration/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧽️delete-infiltration/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧽️delete-infiltration/🧪️tests/✅️applies/🦀️.rs"]
                                    mod tests_applies;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧽️delete-infiltration/🧪️tests/⛔️refuses/🦀️.rs"]
                                    mod tests_refuses;
                                }
                                #[path = "."]
                                pub mod change_infiltration_zone {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌀️change-infiltration-zone/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌀️change-infiltration-zone/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌀️change-infiltration-zone/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌀️change-infiltration-zone/🧪️tests/✅️applies/🦀️.rs"]
                                    mod tests_applies;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌀️change-infiltration-zone/🧪️tests/⛔️refuses/🦀️.rs"]
                                    mod tests_refuses;
                                }
                                #[path = "."]
                                pub mod change_infiltration_schedule {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/⏳️change-infiltration-schedule/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/⏳️change-infiltration-schedule/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/⏳️change-infiltration-schedule/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/⏳️change-infiltration-schedule/🧪️tests/✅️applies/🦀️.rs"]
                                    mod tests_applies;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/⏳️change-infiltration-schedule/🧪️tests/⛔️refuses/🦀️.rs"]
                                    mod tests_refuses;
                                }
                                #[path = "."]
                                pub mod change_infiltration_flow_per_exterior_area {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌫️change-infiltration-flow-per-exterior-area/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌫️change-infiltration-flow-per-exterior-area/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌫️change-infiltration-flow-per-exterior-area/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌫️change-infiltration-flow-per-exterior-area/🧪️tests/✅️applies/🦀️.rs"]
                                    mod tests_applies;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌫️change-infiltration-flow-per-exterior-area/🧪️tests/⛔️refuses/🦀️.rs"]
                                    mod tests_refuses;
                                }
                                #[path = "."]
                                pub mod change_infiltration_constant_term_coefficient {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🅰️change-infiltration-constant-term-coefficient/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🅰️change-infiltration-constant-term-coefficient/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🅰️change-infiltration-constant-term-coefficient/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🅰️change-infiltration-constant-term-coefficient/🧪️tests/✅️applies/🦀️.rs"]
                                    mod tests_applies;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🅰️change-infiltration-constant-term-coefficient/🧪️tests/⛔️refuses/🦀️.rs"]
                                    mod tests_refuses;
                                }
                                #[path = "."]
                                pub mod change_infiltration_temperature_term_coefficient {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🅱️change-infiltration-temperature-term-coefficient/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🅱️change-infiltration-temperature-term-coefficient/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🅱️change-infiltration-temperature-term-coefficient/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🅱️change-infiltration-temperature-term-coefficient/🧪️tests/✅️applies/🦀️.rs"]
                                    mod tests_applies;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🅱️change-infiltration-temperature-term-coefficient/🧪️tests/⛔️refuses/🦀️.rs"]
                                    mod tests_refuses;
                                }
                                #[path = "."]
                                pub mod change_infiltration_velocity_term_coefficient {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🆎️change-infiltration-velocity-term-coefficient/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🆎️change-infiltration-velocity-term-coefficient/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🆎️change-infiltration-velocity-term-coefficient/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🆎️change-infiltration-velocity-term-coefficient/🧪️tests/✅️applies/🦀️.rs"]
                                    mod tests_applies;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🆎️change-infiltration-velocity-term-coefficient/🧪️tests/⛔️refuses/🦀️.rs"]
                                    mod tests_refuses;
                                }
                                #[path = "."]
                                pub mod change_infiltration_velocity_squared_term_coefficient {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🆑️change-infiltration-velocity-squared-term-coefficient/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🆑️change-infiltration-velocity-squared-term-coefficient/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🆑️change-infiltration-velocity-squared-term-coefficient/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🆑️change-infiltration-velocity-squared-term-coefficient/🧪️tests/✅️applies/🦀️.rs"]
                                    mod tests_applies;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🆑️change-infiltration-velocity-squared-term-coefficient/🧪️tests/⛔️refuses/🦀️.rs"]
                                    mod tests_refuses;
                                }
                                #[path = "."]
                                pub mod create_mechanical_ventilation {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌪️create-mechanical-ventilation/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌪️create-mechanical-ventilation/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌪️create-mechanical-ventilation/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌪️create-mechanical-ventilation/🧪️tests/✅️applies/🦀️.rs"]
                                    mod tests_applies;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌪️create-mechanical-ventilation/🧪️tests/⛔️refuses/🦀️.rs"]
                                    mod tests_refuses;
                                }
                                #[path = "."]
                                pub mod delete_mechanical_ventilation {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🚫️delete-mechanical-ventilation/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🚫️delete-mechanical-ventilation/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🚫️delete-mechanical-ventilation/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🚫️delete-mechanical-ventilation/🧪️tests/✅️applies/🦀️.rs"]
                                    mod tests_applies;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🚫️delete-mechanical-ventilation/🧪️tests/⛔️refuses/🦀️.rs"]
                                    mod tests_refuses;
                                }
                                #[path = "."]
                                pub mod change_mechanical_ventilation_zone {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧭️change-mechanical-ventilation-zone/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧭️change-mechanical-ventilation-zone/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧭️change-mechanical-ventilation-zone/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧭️change-mechanical-ventilation-zone/🧪️tests/✅️applies/🦀️.rs"]
                                    mod tests_applies;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧭️change-mechanical-ventilation-zone/🧪️tests/⛔️refuses/🦀️.rs"]
                                    mod tests_refuses;
                                }
                                #[path = "."]
                                pub mod change_mechanical_ventilation_schedule {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📆️change-mechanical-ventilation-schedule/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📆️change-mechanical-ventilation-schedule/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📆️change-mechanical-ventilation-schedule/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📆️change-mechanical-ventilation-schedule/🧪️tests/✅️applies/🦀️.rs"]
                                    mod tests_applies;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📆️change-mechanical-ventilation-schedule/🧪️tests/⛔️refuses/🦀️.rs"]
                                    mod tests_refuses;
                                }
                                #[path = "."]
                                pub mod change_mechanical_ventilation_design_flow {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🚿️change-mechanical-ventilation-design-flow/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🚿️change-mechanical-ventilation-design-flow/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🚿️change-mechanical-ventilation-design-flow/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🚿️change-mechanical-ventilation-design-flow/🧪️tests/✅️applies/🦀️.rs"]
                                    mod tests_applies;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🚿️change-mechanical-ventilation-design-flow/🧪️tests/⛔️refuses/🦀️.rs"]
                                    mod tests_refuses;
                                }
                                #[path = "."]
                                pub mod change_mechanical_ventilation_fan_total_efficiency {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💠️change-mechanical-ventilation-fan-total-efficiency/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💠️change-mechanical-ventilation-fan-total-efficiency/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💠️change-mechanical-ventilation-fan-total-efficiency/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💠️change-mechanical-ventilation-fan-total-efficiency/🧪️tests/✅️applies/🦀️.rs"]
                                    mod tests_applies;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💠️change-mechanical-ventilation-fan-total-efficiency/🧪️tests/⛔️refuses/🦀️.rs"]
                                    mod tests_refuses;
                                }
                                #[path = "."]
                                pub mod change_mechanical_ventilation_fan_delta_pressure {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🎈️change-mechanical-ventilation-fan-delta-pressure/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🎈️change-mechanical-ventilation-fan-delta-pressure/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🎈️change-mechanical-ventilation-fan-delta-pressure/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🎈️change-mechanical-ventilation-fan-delta-pressure/🧪️tests/✅️applies/🦀️.rs"]
                                    mod tests_applies;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🎈️change-mechanical-ventilation-fan-delta-pressure/🧪️tests/⛔️refuses/🦀️.rs"]
                                    mod tests_refuses;
                                }
                                #[path = "."]
                                pub mod change_infiltration_method {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔬️change-infiltration-method/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔬️change-infiltration-method/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔬️change-infiltration-method/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔬️change-infiltration-method/🧪️tests/✅️applies/🦀️.rs"]
                                    mod tests_applies;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔬️change-infiltration-method/🧪️tests/⛔️refuses/🦀️.rs"]
                                    mod tests_refuses;
                                }
                                #[path = "."]
                                pub mod change_infiltration_design_flow_ach {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔄️change-infiltration-design-flow-ach/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔄️change-infiltration-design-flow-ach/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔄️change-infiltration-design-flow-ach/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔄️change-infiltration-design-flow-ach/🧪️tests/✅️applies/🦀️.rs"]
                                    mod tests_applies;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔄️change-infiltration-design-flow-ach/🧪️tests/⛔️refuses/🦀️.rs"]
                                    mod tests_refuses;
                                }
                                #[path = "."]
                                pub mod change_infiltration_effective_leakage_area {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🕳️change-infiltration-effective-leakage-area/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🕳️change-infiltration-effective-leakage-area/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🕳️change-infiltration-effective-leakage-area/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🕳️change-infiltration-effective-leakage-area/🧪️tests/✅️applies/🦀️.rs"]
                                    mod tests_applies;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🕳️change-infiltration-effective-leakage-area/🧪️tests/⛔️refuses/🦀️.rs"]
                                    mod tests_refuses;
                                }
                                #[path = "."]
                                pub mod change_infiltration_discharge_coefficient {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🚰️change-infiltration-discharge-coefficient/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🚰️change-infiltration-discharge-coefficient/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🚰️change-infiltration-discharge-coefficient/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🚰️change-infiltration-discharge-coefficient/🧪️tests/✅️applies/🦀️.rs"]
                                    mod tests_applies;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🚰️change-infiltration-discharge-coefficient/🧪️tests/⛔️refuses/🦀️.rs"]
                                    mod tests_refuses;
                                }
                                #[path = "."]
                                pub mod change_infiltration_stack_height {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏭️change-infiltration-stack-height/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏭️change-infiltration-stack-height/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏭️change-infiltration-stack-height/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏭️change-infiltration-stack-height/🧪️tests/✅️applies/🦀️.rs"]
                                    mod tests_applies;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏭️change-infiltration-stack-height/🧪️tests/⛔️refuses/🦀️.rs"]
                                    mod tests_refuses;
                                }
                                #[path = "."]
                                pub mod create_thermostat {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🩺️create-thermostat/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🩺️create-thermostat/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🩺️create-thermostat/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🩺️create-thermostat/🧪️tests/✅️controls-zone-one/🦀️.rs"]
                                    mod tests_controls_zone_one;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🩺️create-thermostat/🧪️tests/⛔️refuses-an-absent-zone/🦀️.rs"]
                                    mod tests_refuses_an_absent_zone;
                                }
                                #[path = "."]
                                pub mod delete_thermostat {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🛑️delete-thermostat/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🛑️delete-thermostat/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🛑️delete-thermostat/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🛑️delete-thermostat/🧪️tests/✅️frees-zone-one/🦀️.rs"]
                                    mod tests_frees_zone_one;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🛑️delete-thermostat/🧪️tests/⛔️refuses-an-absent-one/🦀️.rs"]
                                    mod tests_refuses_an_absent_one;
                                }
                                #[path = "."]
                                pub mod change_thermostat_zone {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🛖️change-thermostat-zone/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🛖️change-thermostat-zone/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🛖️change-thermostat-zone/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🛖️change-thermostat-zone/🧪️tests/✅️moves-to-zone-two/🦀️.rs"]
                                    mod tests_moves_to_zone_two;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🛖️change-thermostat-zone/🧪️tests/⛔️refuses-an-absent-zone/🦀️.rs"]
                                    mod tests_refuses_an_absent_zone;
                                }
                                #[path = "."]
                                pub mod change_thermostat_heating_setpoint_schedule {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🥵️change-thermostat-heating-setpoint-schedule/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🥵️change-thermostat-heating-setpoint-schedule/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🥵️change-thermostat-heating-setpoint-schedule/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🥵️change-thermostat-heating-setpoint-schedule/🧪️tests/✅️repoints-heating/🦀️.rs"]
                                    mod tests_repoints_heating;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🥵️change-thermostat-heating-setpoint-schedule/🧪️tests/⛔️refuses-an-absent-one/🦀️.rs"]
                                    mod tests_refuses_an_absent_one;
                                }
                                #[path = "."]
                                pub mod change_thermostat_cooling_setpoint_schedule {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🐧️change-thermostat-cooling-setpoint-schedule/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🐧️change-thermostat-cooling-setpoint-schedule/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🐧️change-thermostat-cooling-setpoint-schedule/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🐧️change-thermostat-cooling-setpoint-schedule/🧪️tests/✅️repoints-cooling/🦀️.rs"]
                                    mod tests_repoints_cooling;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🐧️change-thermostat-cooling-setpoint-schedule/🧪️tests/⛔️refuses-an-absent-one/🦀️.rs"]
                                    mod tests_refuses_an_absent_one;
                                }
                                #[path = "."]
                                pub mod change_thermostat_heating_throttle_range {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🎚️change-thermostat-heating-throttle-range/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🎚️change-thermostat-heating-throttle-range/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🎚️change-thermostat-heating-throttle-range/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🎚️change-thermostat-heating-throttle-range/🧪️tests/✅️widens-heating-band/🦀️.rs"]
                                    mod tests_widens_heating_band;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🎚️change-thermostat-heating-throttle-range/🧪️tests/⛔️refuses-a-zero-band/🦀️.rs"]
                                    mod tests_refuses_a_zero_band;
                                }
                                #[path = "."]
                                pub mod change_thermostat_cooling_throttle_range {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🎛️change-thermostat-cooling-throttle-range/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🎛️change-thermostat-cooling-throttle-range/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🎛️change-thermostat-cooling-throttle-range/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🎛️change-thermostat-cooling-throttle-range/🧪️tests/✅️widens-cooling-band/🦀️.rs"]
                                    mod tests_widens_cooling_band;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🎛️change-thermostat-cooling-throttle-range/🧪️tests/⛔️refuses-a-negative-band/🦀️.rs"]
                                    mod tests_refuses_a_negative_band;
                                }
                                #[path = "."]
                                pub mod create_humidistat {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌂️create-humidistat/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌂️create-humidistat/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌂️create-humidistat/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌂️create-humidistat/🧪️tests/✅️controls-zone-one/🦀️.rs"]
                                    mod tests_controls_zone_one;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌂️create-humidistat/🧪️tests/⛔️refuses-an-absent-one/🦀️.rs"]
                                    mod tests_refuses_an_absent_one;
                                }
                                #[path = "."]
                                pub mod delete_humidistat {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏜️delete-humidistat/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏜️delete-humidistat/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏜️delete-humidistat/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏜️delete-humidistat/🧪️tests/✅️drops-the-control/🦀️.rs"]
                                    mod tests_drops_the_control;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏜️delete-humidistat/🧪️tests/⛔️refuses-an-absent-one/🦀️.rs"]
                                    mod tests_refuses_an_absent_one;
                                }
                                #[path = "."]
                                pub mod change_humidistat_zone {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏙️change-humidistat-zone/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏙️change-humidistat-zone/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏙️change-humidistat-zone/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏙️change-humidistat-zone/🧪️tests/✅️moves-to-zone-two/🦀️.rs"]
                                    mod tests_moves_to_zone_two;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏙️change-humidistat-zone/🧪️tests/⛔️refuses-an-absent-zone/🦀️.rs"]
                                    mod tests_refuses_an_absent_zone;
                                }
                                #[path = "."]
                                pub mod change_humidistat_humidifying_setpoint_schedule {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/☔️change-humidistat-humidifying-setpoint-schedule/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/☔️change-humidistat-humidifying-setpoint-schedule/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/☔️change-humidistat-humidifying-setpoint-schedule/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/☔️change-humidistat-humidifying-setpoint-schedule/🧪️tests/✅️repoints-humidifying/🦀️.rs"]
                                    mod tests_repoints_humidifying;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/☔️change-humidistat-humidifying-setpoint-schedule/🧪️tests/⛔️refuses-an-absent-one/🦀️.rs"]
                                    mod tests_refuses_an_absent_one;
                                }
                                #[path = "."]
                                pub mod change_humidistat_dehumidifying_setpoint_schedule {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏝️change-humidistat-dehumidifying-setpoint-schedule/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏝️change-humidistat-dehumidifying-setpoint-schedule/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏝️change-humidistat-dehumidifying-setpoint-schedule/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏝️change-humidistat-dehumidifying-setpoint-schedule/🧪️tests/✅️repoints-drying/🦀️.rs"]
                                    mod tests_repoints_drying;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏝️change-humidistat-dehumidifying-setpoint-schedule/🧪️tests/⛔️refuses-an-absent-one/🦀️.rs"]
                                    mod tests_refuses_an_absent_one;
                                }
                                #[path = "."]
                                pub mod change_humidistat_humidifying_throttle_range {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌧️change-humidistat-humidifying-throttle-range/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌧️change-humidistat-humidifying-throttle-range/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌧️change-humidistat-humidifying-throttle-range/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌧️change-humidistat-humidifying-throttle-range/🧪️tests/✅️widens-the-band/🦀️.rs"]
                                    mod tests_widens_the_band;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌧️change-humidistat-humidifying-throttle-range/🧪️tests/⛔️refuses-a-zero-band/🦀️.rs"]
                                    mod tests_refuses_a_zero_band;
                                }
                                #[path = "."]
                                pub mod change_humidistat_dehumidifying_throttle_range {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧻️change-humidistat-dehumidifying-throttle-range/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧻️change-humidistat-dehumidifying-throttle-range/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧻️change-humidistat-dehumidifying-throttle-range/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧻️change-humidistat-dehumidifying-throttle-range/🧪️tests/✅️widens-the-band/🦀️.rs"]
                                    mod tests_widens_the_band;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧻️change-humidistat-dehumidifying-throttle-range/🧪️tests/⛔️refuses-a-negative-band/🦀️.rs"]
                                    mod tests_refuses_a_negative_band;
                                }
                                #[path = "."]
                                pub mod create_ideal_loads_system {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🫁️create-ideal-loads-system/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🫁️create-ideal-loads-system/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🫁️create-ideal-loads-system/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🫁️create-ideal-loads-system/🧪️tests/✅️serves-zone-one/🦀️.rs"]
                                    mod tests_serves_zone_one;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🫁️create-ideal-loads-system/🧪️tests/⛔️refuses-an-absent-zone/🦀️.rs"]
                                    mod tests_refuses_an_absent_zone;
                                }
                                #[path = "."]
                                pub mod delete_ideal_loads_system {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🫥️delete-ideal-loads-system/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🫥️delete-ideal-loads-system/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🫥️delete-ideal-loads-system/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🫥️delete-ideal-loads-system/🧪️tests/✅️drops-the-system/🦀️.rs"]
                                    mod tests_drops_the_system;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🫥️delete-ideal-loads-system/🧪️tests/⛔️refuses-an-absent-one/🦀️.rs"]
                                    mod tests_refuses_an_absent_one;
                                }
                                #[path = "."]
                                pub mod change_ideal_loads_system_zone {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏢️change-ideal-loads-system-zone/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏢️change-ideal-loads-system-zone/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏢️change-ideal-loads-system-zone/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏢️change-ideal-loads-system-zone/🧪️tests/✅️moves-to-zone-two/🦀️.rs"]
                                    mod tests_moves_to_zone_two;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏢️change-ideal-loads-system-zone/🧪️tests/⛔️refuses-an-absent-zone/🦀️.rs"]
                                    mod tests_refuses_an_absent_zone;
                                }
                                #[path = "."]
                                pub mod change_ideal_loads_system_max_heating_supply_air_temp {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔴️change-ideal-loads-system-max-heating-supply-air-temp/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔴️change-ideal-loads-system-max-heating-supply-air-temp/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔴️change-ideal-loads-system-max-heating-supply-air-temp/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔴️change-ideal-loads-system-max-heating-supply-air-temp/🧪️tests/✅️cools-the-supply/🦀️.rs"]
                                    mod tests_cools_the_supply;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔴️change-ideal-loads-system-max-heating-supply-air-temp/🧪️tests/⛔️refuses-a-hot-supply/🦀️.rs"]
                                    mod tests_refuses_a_hot_supply;
                                }
                                #[path = "."]
                                pub mod change_ideal_loads_system_min_cooling_supply_air_temp {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔵️change-ideal-loads-system-min-cooling-supply-air-temp/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔵️change-ideal-loads-system-min-cooling-supply-air-temp/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔵️change-ideal-loads-system-min-cooling-supply-air-temp/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔵️change-ideal-loads-system-min-cooling-supply-air-temp/🧪️tests/✅️lowers-the-supply/🦀️.rs"]
                                    mod tests_lowers_the_supply;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔵️change-ideal-loads-system-min-cooling-supply-air-temp/🧪️tests/⛔️refuses-a-cold-supply/🦀️.rs"]
                                    mod tests_refuses_a_cold_supply;
                                }
                                #[path = "."]
                                pub mod change_ideal_loads_system_max_heating_capacity {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/⛽️change-ideal-loads-system-max-heating-capacity/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/⛽️change-ideal-loads-system-max-heating-capacity/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/⛽️change-ideal-loads-system-max-heating-capacity/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/⛽️change-ideal-loads-system-max-heating-capacity/🧪️tests/✅️caps-the-heating/🦀️.rs"]
                                    mod tests_caps_the_heating;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/⛽️change-ideal-loads-system-max-heating-capacity/🧪️tests/⛔️refuses-a-stray-value/🦀️.rs"]
                                    mod tests_refuses_a_stray_value;
                                }
                                #[path = "."]
                                pub mod change_ideal_loads_system_max_cooling_capacity {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🟧️change-ideal-loads-system-max-cooling-capacity/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🟧️change-ideal-loads-system-max-cooling-capacity/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🟧️change-ideal-loads-system-max-cooling-capacity/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🟧️change-ideal-loads-system-max-cooling-capacity/🧪️tests/✅️caps-the-cooling/🦀️.rs"]
                                    mod tests_caps_the_cooling;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🟧️change-ideal-loads-system-max-cooling-capacity/🧪️tests/⛔️refuses-a-stray-value/🦀️.rs"]
                                    mod tests_refuses_a_stray_value;
                                }
                                #[path = "."]
                                pub mod change_ideal_loads_system_outdoor_air_per_person {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧍️change-ideal-loads-system-outdoor-air-per-person/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧍️change-ideal-loads-system-outdoor-air-per-person/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧍️change-ideal-loads-system-outdoor-air-per-person/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧍️change-ideal-loads-system-outdoor-air-per-person/🧪️tests/✅️ventilates-per-head/🦀️.rs"]
                                    mod tests_ventilates_per_head;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧍️change-ideal-loads-system-outdoor-air-per-person/🧪️tests/⛔️refuses-a-negative/🦀️.rs"]
                                    mod tests_refuses_a_negative;
                                }
                                #[path = "."]
                                pub mod change_ideal_loads_system_outdoor_air_per_area {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔳️change-ideal-loads-system-outdoor-air-per-area/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔳️change-ideal-loads-system-outdoor-air-per-area/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔳️change-ideal-loads-system-outdoor-air-per-area/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔳️change-ideal-loads-system-outdoor-air-per-area/🧪️tests/✅️ventilates-per-area/🦀️.rs"]
                                    mod tests_ventilates_per_area;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔳️change-ideal-loads-system-outdoor-air-per-area/🧪️tests/⛔️refuses-a-negative/🦀️.rs"]
                                    mod tests_refuses_a_negative;
                                }
                                #[path = "."]
                                pub mod create_zone_equipment {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🛠️create-zone-equipment/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🛠️create-zone-equipment/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🛠️create-zone-equipment/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🛠️create-zone-equipment/🧪️tests/✅️adds-a-baseboard/🦀️.rs"]
                                    mod tests_adds_a_baseboard;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🛠️create-zone-equipment/🧪️tests/⛔️refuses-rank-zero/🦀️.rs"]
                                    mod tests_refuses_rank_zero;
                                }
                                #[path = "."]
                                pub mod delete_zone_equipment {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🗑️delete-zone-equipment/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🗑️delete-zone-equipment/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🗑️delete-zone-equipment/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🗑️delete-zone-equipment/🧪️tests/✅️drops-the-baseboard/🦀️.rs"]
                                    mod tests_drops_the_baseboard;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🗑️delete-zone-equipment/🧪️tests/⛔️refuses-an-absent-one/🦀️.rs"]
                                    mod tests_refuses_an_absent_one;
                                }
                                #[path = "."]
                                pub mod change_zone_equipment_zone {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏬️change-zone-equipment-zone/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏬️change-zone-equipment-zone/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏬️change-zone-equipment-zone/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏬️change-zone-equipment-zone/🧪️tests/✅️moves-to-zone-two/🦀️.rs"]
                                    mod tests_moves_to_zone_two;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏬️change-zone-equipment-zone/🧪️tests/⛔️refuses-an-absent-zone/🦀️.rs"]
                                    mod tests_refuses_an_absent_zone;
                                }
                                #[path = "."]
                                pub mod change_zone_equipment_type {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔧️change-zone-equipment-type/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔧️change-zone-equipment-type/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔧️change-zone-equipment-type/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔧️change-zone-equipment-type/🧪️tests/✅️swaps-to-a-fan-coil/🦀️.rs"]
                                    mod tests_swaps_to_a_fan_coil;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔧️change-zone-equipment-type/🧪️tests/⛔️refuses-an-absent-row/🦀️.rs"]
                                    mod tests_refuses_an_absent_row;
                                }
                                #[path = "."]
                                pub mod change_zone_equipment_priority {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🎗️change-zone-equipment-priority/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🎗️change-zone-equipment-priority/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🎗️change-zone-equipment-priority/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🎗️change-zone-equipment-priority/🧪️tests/✅️demotes-it/🦀️.rs"]
                                    mod tests_demotes_it;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🎗️change-zone-equipment-priority/🧪️tests/⛔️refuses-rank-zero/🦀️.rs"]
                                    mod tests_refuses_rank_zero;
                                }
                                #[path = "."]
                                pub mod change_zone_equipment_heating_capacity {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧇️change-zone-equipment-heating-capacity/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧇️change-zone-equipment-heating-capacity/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧇️change-zone-equipment-heating-capacity/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧇️change-zone-equipment-heating-capacity/🧪️tests/✅️uprates-heating/🦀️.rs"]
                                    mod tests_uprates_heating;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧇️change-zone-equipment-heating-capacity/🧪️tests/⛔️refuses-a-negative/🦀️.rs"]
                                    mod tests_refuses_a_negative;
                                }
                                #[path = "."]
                                pub mod change_zone_equipment_cooling_capacity {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🍧️change-zone-equipment-cooling-capacity/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🍧️change-zone-equipment-cooling-capacity/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🍧️change-zone-equipment-cooling-capacity/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🍧️change-zone-equipment-cooling-capacity/🧪️tests/✅️uprates-cooling/🦀️.rs"]
                                    mod tests_uprates_cooling;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🍧️change-zone-equipment-cooling-capacity/🧪️tests/⛔️refuses-a-negative/🦀️.rs"]
                                    mod tests_refuses_a_negative;
                                }
                                #[path = "."]
                                pub mod create_daylight_zone {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔭️create-daylight-zone/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔭️create-daylight-zone/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔭️create-daylight-zone/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔭️create-daylight-zone/🧪️tests/✅️lights-zone-one/🦀️.rs"]
                                    mod tests_lights_zone_one;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔭️create-daylight-zone/🧪️tests/⛔️refuses-a-bad-tau/🦀️.rs"]
                                    mod tests_refuses_a_bad_tau;
                                }
                                #[path = "."]
                                pub mod delete_daylight_zone {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌗️delete-daylight-zone/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌗️delete-daylight-zone/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌗️delete-daylight-zone/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌗️delete-daylight-zone/🧪️tests/✅️darkens-the-zone/🦀️.rs"]
                                    mod tests_darkens_the_zone;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌗️delete-daylight-zone/🧪️tests/⛔️refuses-an-absent-one/🦀️.rs"]
                                    mod tests_refuses_an_absent_one;
                                }
                                #[path = "."]
                                pub mod change_daylight_zone_zone {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏫️change-daylight-zone-zone/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏫️change-daylight-zone-zone/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏫️change-daylight-zone-zone/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏫️change-daylight-zone-zone/🧪️tests/✅️moves-to-zone-two/🦀️.rs"]
                                    mod tests_moves_to_zone_two;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏫️change-daylight-zone-zone/🧪️tests/⛔️refuses-an-absent-zone/🦀️.rs"]
                                    mod tests_refuses_an_absent_zone;
                                }
                                #[path = "."]
                                pub mod change_daylight_zone_illuminance_target {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪔️change-daylight-zone-illuminance-target/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪔️change-daylight-zone-illuminance-target/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪔️change-daylight-zone-illuminance-target/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪔️change-daylight-zone-illuminance-target/🧪️tests/✅️dims-the-target/🦀️.rs"]
                                    mod tests_dims_the_target;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪔️change-daylight-zone-illuminance-target/🧪️tests/⛔️refuses-a-dark-target/🦀️.rs"]
                                    mod tests_refuses_a_dark_target;
                                }
                                #[path = "."]
                                pub mod change_daylight_zone_glare_limit {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🕶️change-daylight-zone-glare-limit/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🕶️change-daylight-zone-glare-limit/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🕶️change-daylight-zone-glare-limit/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🕶️change-daylight-zone-glare-limit/🧪️tests/✅️tightens-glare/🦀️.rs"]
                                    mod tests_tightens_glare;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🕶️change-daylight-zone-glare-limit/🧪️tests/⛔️refuses-a-negative/🦀️.rs"]
                                    mod tests_refuses_a_negative;
                                }
                                #[path = "."]
                                pub mod change_daylight_zone_window_transmittance {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🥃️change-daylight-zone-window-transmittance/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🥃️change-daylight-zone-window-transmittance/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🥃️change-daylight-zone-window-transmittance/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🥃️change-daylight-zone-window-transmittance/🧪️tests/✅️darkens-the-glass/🦀️.rs"]
                                    mod tests_darkens_the_glass;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🥃️change-daylight-zone-window-transmittance/🧪️tests/⛔️refuses-a-bad-tau/🦀️.rs"]
                                    mod tests_refuses_a_bad_tau;
                                }
                                #[path = "."]
                                pub mod create_sizing_object {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📶️create-sizing-object/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📶️create-sizing-object/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📶️create-sizing-object/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📶️create-sizing-object/🧪️tests/✅️sizes-zone-one/🦀️.rs"]
                                    mod tests_sizes_zone_one;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📶️create-sizing-object/🧪️tests/⛔️refuses-an-absent-zone/🦀️.rs"]
                                    mod tests_refuses_an_absent_zone;
                                }
                                #[path = "."]
                                pub mod delete_sizing_object {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪒️delete-sizing-object/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪒️delete-sizing-object/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪒️delete-sizing-object/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪒️delete-sizing-object/🧪️tests/✅️drops-the-sizing/🦀️.rs"]
                                    mod tests_drops_the_sizing;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪒️delete-sizing-object/🧪️tests/⛔️refuses-an-absent-one/🦀️.rs"]
                                    mod tests_refuses_an_absent_one;
                                }
                                #[path = "."]
                                pub mod change_sizing_object_zone {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏨️change-sizing-object-zone/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏨️change-sizing-object-zone/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏨️change-sizing-object-zone/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏨️change-sizing-object-zone/🧪️tests/✅️moves-to-zone-two/🦀️.rs"]
                                    mod tests_moves_to_zone_two;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏨️change-sizing-object-zone/🧪️tests/⛔️refuses-an-absent-zone/🦀️.rs"]
                                    mod tests_refuses_an_absent_zone;
                                }
                                #[path = "."]
                                pub mod change_sizing_object_sizing_type {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧾️change-sizing-object-sizing-type/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧾️change-sizing-object-sizing-type/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧾️change-sizing-object-sizing-type/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧾️change-sizing-object-sizing-type/🧪️tests/✅️sizes-for-cooling/🦀️.rs"]
                                    mod tests_sizes_for_cooling;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧾️change-sizing-object-sizing-type/🧪️tests/⛔️refuses-an-absent-row/🦀️.rs"]
                                    mod tests_refuses_an_absent_row;
                                }
                                #[path = "."]
                                pub mod change_sizing_object_design_day_type {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌥️change-sizing-object-design-day-type/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌥️change-sizing-object-design-day-type/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌥️change-sizing-object-design-day-type/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌥️change-sizing-object-design-day-type/🧪️tests/✅️reads-a-hot-day/🦀️.rs"]
                                    mod tests_reads_a_hot_day;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌥️change-sizing-object-design-day-type/🧪️tests/⛔️refuses-an-absent-row/🦀️.rs"]
                                    mod tests_refuses_an_absent_row;
                                }
                                #[path = "."]
                                pub mod create_room_air_model_assignment {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🛏️create-room-air-model-assignment/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🛏️create-room-air-model-assignment/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🛏️create-room-air-model-assignment/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🛏️create-room-air-model-assignment/🧪️tests/✅️stratifies-zone-two/🦀️.rs"]
                                    mod tests_stratifies_zone_two;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🛏️create-room-air-model-assignment/🧪️tests/⛔️refuses-a-second-one/🦀️.rs"]
                                    mod tests_refuses_a_second_one;
                                }
                                #[path = "."]
                                pub mod delete_room_air_model_assignment {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧺️delete-room-air-model-assignment/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧺️delete-room-air-model-assignment/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧺️delete-room-air-model-assignment/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧺️delete-room-air-model-assignment/🧪️tests/✅️falls-back/🦀️.rs"]
                                    mod tests_falls_back;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧺️delete-room-air-model-assignment/🧪️tests/⛔️refuses-an-absent-one/🦀️.rs"]
                                    mod tests_refuses_an_absent_one;
                                }
                                #[path = "."]
                                pub mod change_room_air_model {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪭️change-room-air-model/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪭️change-room-air-model/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪭️change-room-air-model/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪭️change-room-air-model/🧪️tests/✅️stratifies-the-air/🦀️.rs"]
                                    mod tests_stratifies_the_air;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪭️change-room-air-model/🧪️tests/⛔️refuses-an-absent-row/🦀️.rs"]
                                    mod tests_refuses_an_absent_row;
                                }
                                #[path = "."]
                                pub mod create_setpoint_manager {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📌️create-setpoint-manager/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📌️create-setpoint-manager/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📌️create-setpoint-manager/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📌️create-setpoint-manager/🧪️tests/✅️adds-a-scheduled-spm/🦀️.rs"]
                                    mod tests_adds_a_scheduled_spm;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📌️create-setpoint-manager/🧪️tests/⛔️refuses-a-bad-kind/🦀️.rs"]
                                    mod tests_refuses_a_bad_kind;
                                }
                                #[path = "."]
                                pub mod delete_setpoint_manager {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🍄️delete-setpoint-manager/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🍄️delete-setpoint-manager/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🍄️delete-setpoint-manager/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🍄️delete-setpoint-manager/🧪️tests/✅️drops-the-spm/🦀️.rs"]
                                    mod tests_drops_the_spm;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🍄️delete-setpoint-manager/🧪️tests/⛔️refuses-an-absent-one/🦀️.rs"]
                                    mod tests_refuses_an_absent_one;
                                }
                                #[path = "."]
                                pub mod rename_setpoint_manager {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🖇️rename-setpoint-manager/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🖇️rename-setpoint-manager/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🖇️rename-setpoint-manager/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🖇️rename-setpoint-manager/🧪️tests/✅️renames-the-spm/🦀️.rs"]
                                    mod tests_renames_the_spm;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🖇️rename-setpoint-manager/🧪️tests/⛔️refuses-a-blank-name/🦀️.rs"]
                                    mod tests_refuses_a_blank_name;
                                }
                                #[path = "."]
                                pub mod replace_setpoint_manager_kind {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔃️replace-setpoint-manager-kind/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔃️replace-setpoint-manager-kind/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔃️replace-setpoint-manager-kind/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔃️replace-setpoint-manager-kind/🧪️tests/✅️resets-on-outdoor-air/🦀️.rs"]
                                    mod tests_resets_on_outdoor_air;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔃️replace-setpoint-manager-kind/🧪️tests/⛔️refuses-stray-limits/🦀️.rs"]
                                    mod tests_refuses_stray_limits;
                                }
                                #[path = "."]
                                pub mod change_setpoint_manager_schedule {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🎼️change-setpoint-manager-schedule/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🎼️change-setpoint-manager-schedule/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🎼️change-setpoint-manager-schedule/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🎼️change-setpoint-manager-schedule/🧪️tests/✅️repoints-the-spm/🦀️.rs"]
                                    mod tests_repoints_the_spm;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🎼️change-setpoint-manager-schedule/🧪️tests/⛔️refuses-an-absent-one/🦀️.rs"]
                                    mod tests_refuses_an_absent_one;
                                }
                                #[path = "."]
                                pub mod create_air_loop {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🛞️create-air-loop/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🛞️create-air-loop/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🛞️create-air-loop/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🛞️create-air-loop/🧪️tests/✅️adds-a-main-loop/🦀️.rs"]
                                    mod tests_adds_a_main_loop;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🛞️create-air-loop/🧪️tests/⛔️refuses-a-jumbled-list/🦀️.rs"]
                                    mod tests_refuses_a_jumbled_list;
                                }
                                #[path = "."]
                                pub mod delete_air_loop {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🥀️delete-air-loop/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🥀️delete-air-loop/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🥀️delete-air-loop/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🥀️delete-air-loop/🧪️tests/✅️drops-the-loop/🦀️.rs"]
                                    mod tests_drops_the_loop;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🥀️delete-air-loop/🧪️tests/⛔️refuses-a-served-loop/🦀️.rs"]
                                    mod tests_refuses_a_served_loop;
                                }
                                #[path = "."]
                                pub mod rename_air_loop {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📇️rename-air-loop/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📇️rename-air-loop/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📇️rename-air-loop/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📇️rename-air-loop/🧪️tests/✅️renames-the-loop/🦀️.rs"]
                                    mod tests_renames_the_loop;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📇️rename-air-loop/🧪️tests/⛔️refuses-a-blank-name/🦀️.rs"]
                                    mod tests_refuses_a_blank_name;
                                }
                                #[path = "."]
                                pub mod change_air_loop_supply_node {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/↗️change-air-loop-supply-node/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/↗️change-air-loop-supply-node/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/↗️change-air-loop-supply-node/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/↗️change-air-loop-supply-node/🧪️tests/✅️repoints-supply/🦀️.rs"]
                                    mod tests_repoints_supply;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/↗️change-air-loop-supply-node/🧪️tests/⛔️refuses-node-zero/🦀️.rs"]
                                    mod tests_refuses_node_zero;
                                }
                                #[path = "."]
                                pub mod change_air_loop_return_node {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/↘️change-air-loop-return-node/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/↘️change-air-loop-return-node/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/↘️change-air-loop-return-node/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/↘️change-air-loop-return-node/🧪️tests/✅️repoints-return/🦀️.rs"]
                                    mod tests_repoints_return;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/↘️change-air-loop-return-node/🧪️tests/⛔️refuses-node-zero/🦀️.rs"]
                                    mod tests_refuses_node_zero;
                                }
                                #[path = "."]
                                pub mod change_air_loop_design_supply_air_flow {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🍥️change-air-loop-design-supply-air-flow/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🍥️change-air-loop-design-supply-air-flow/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🍥️change-air-loop-design-supply-air-flow/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🍥️change-air-loop-design-supply-air-flow/🧪️tests/✅️uprates-the-flow/🦀️.rs"]
                                    mod tests_uprates_the_flow;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🍥️change-air-loop-design-supply-air-flow/🧪️tests/⛔️refuses-no-flow/🦀️.rs"]
                                    mod tests_refuses_no_flow;
                                }
                                #[path = "."]
                                pub mod add_air_loop_terminal_zone {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪺️add-air-loop-terminal-zone/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪺️add-air-loop-terminal-zone/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪺️add-air-loop-terminal-zone/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪺️add-air-loop-terminal-zone/🧪️tests/✅️serves-zone-two/🦀️.rs"]
                                    mod tests_serves_zone_two;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪺️add-air-loop-terminal-zone/🧪️tests/⛔️refuses-an-absent-zone/🦀️.rs"]
                                    mod tests_refuses_an_absent_zone;
                                }
                                #[path = "."]
                                pub mod remove_air_loop_terminal_zone {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪹️remove-air-loop-terminal-zone/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪹️remove-air-loop-terminal-zone/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪹️remove-air-loop-terminal-zone/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪹️remove-air-loop-terminal-zone/🧪️tests/✅️stops-serving-one/🦀️.rs"]
                                    mod tests_stops_serving_one;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪹️remove-air-loop-terminal-zone/🧪️tests/⛔️refuses-an-unserved/🦀️.rs"]
                                    mod tests_refuses_an_unserved;
                                }
                                #[path = "."]
                                pub mod create_plant_loop {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/⚗️create-plant-loop/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/⚗️create-plant-loop/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/⚗️create-plant-loop/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/⚗️create-plant-loop/🧪️tests/✅️adds-a-hot-loop/🦀️.rs"]
                                    mod tests_adds_a_hot_loop;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/⚗️create-plant-loop/🧪️tests/⛔️refuses-a-jumbled-list/🦀️.rs"]
                                    mod tests_refuses_a_jumbled_list;
                                }
                                #[path = "."]
                                pub mod delete_plant_loop {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💣️delete-plant-loop/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💣️delete-plant-loop/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💣️delete-plant-loop/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💣️delete-plant-loop/🧪️tests/✅️drops-the-loop/🦀️.rs"]
                                    mod tests_drops_the_loop;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💣️delete-plant-loop/🧪️tests/⛔️refuses-an-absent-one/🦀️.rs"]
                                    mod tests_refuses_an_absent_one;
                                }
                                #[path = "."]
                                pub mod rename_plant_loop {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📛️rename-plant-loop/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📛️rename-plant-loop/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📛️rename-plant-loop/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📛️rename-plant-loop/🧪️tests/✅️renames-the-loop/🦀️.rs"]
                                    mod tests_renames_the_loop;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📛️rename-plant-loop/🧪️tests/⛔️refuses-a-blank-name/🦀️.rs"]
                                    mod tests_refuses_a_blank_name;
                                }
                                #[path = "."]
                                pub mod change_plant_loop_type {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/♻️change-plant-loop-type/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/♻️change-plant-loop-type/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/♻️change-plant-loop-type/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/♻️change-plant-loop-type/🧪️tests/✅️turns-it-chilled/🦀️.rs"]
                                    mod tests_turns_it_chilled;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/♻️change-plant-loop-type/🧪️tests/⛔️refuses-an-absent-row/🦀️.rs"]
                                    mod tests_refuses_an_absent_row;
                                }
                                #[path = "."]
                                pub mod change_plant_loop_supply_temperature {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/☕️change-plant-loop-supply-temperature/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/☕️change-plant-loop-supply-temperature/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/☕️change-plant-loop-supply-temperature/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/☕️change-plant-loop-supply-temperature/🧪️tests/✅️cools-the-supply/🦀️.rs"]
                                    mod tests_cools_the_supply;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/☕️change-plant-loop-supply-temperature/🧪️tests/⛔️refuses-a-hot-supply/🦀️.rs"]
                                    mod tests_refuses_a_hot_supply;
                                }
                                #[path = "."]
                                pub mod change_plant_loop_return_temperature {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🫖️change-plant-loop-return-temperature/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🫖️change-plant-loop-return-temperature/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🫖️change-plant-loop-return-temperature/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🫖️change-plant-loop-return-temperature/🧪️tests/✅️cools-the-return/🦀️.rs"]
                                    mod tests_cools_the_return;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🫖️change-plant-loop-return-temperature/🧪️tests/⛔️refuses-a-cold-return/🦀️.rs"]
                                    mod tests_refuses_a_cold_return;
                                }
                                #[path = "."]
                                pub mod change_plant_loop_design_flow {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🚤️change-plant-loop-design-flow/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🚤️change-plant-loop-design-flow/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🚤️change-plant-loop-design-flow/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🚤️change-plant-loop-design-flow/🧪️tests/✅️uprates-the-flow/🦀️.rs"]
                                    mod tests_uprates_the_flow;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🚤️change-plant-loop-design-flow/🧪️tests/⛔️refuses-no-flow/🦀️.rs"]
                                    mod tests_refuses_no_flow;
                                }
                                #[path = "."]
                                pub mod add_plant_loop_equipment {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔩️add-plant-loop-equipment/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔩️add-plant-loop-equipment/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔩️add-plant-loop-equipment/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔩️add-plant-loop-equipment/🧪️tests/✅️names-equipment/🦀️.rs"]
                                    mod tests_names_equipment;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔩️add-plant-loop-equipment/🧪️tests/⛔️refuses-the-unset-id/🦀️.rs"]
                                    mod tests_refuses_the_unset_id;
                                }
                                #[path = "."]
                                pub mod remove_plant_loop_equipment {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/⚙️remove-plant-loop-equipment/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/⚙️remove-plant-loop-equipment/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/⚙️remove-plant-loop-equipment/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/⚙️remove-plant-loop-equipment/🧪️tests/✅️drops-equipment/🦀️.rs"]
                                    mod tests_drops_equipment;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/⚙️remove-plant-loop-equipment/🧪️tests/⛔️refuses-an-unlisted/🦀️.rs"]
                                    mod tests_refuses_an_unlisted;
                                }
                                #[path = "."]
                                pub mod create_outdoor_air_system {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌲️create-outdoor-air-system/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌲️create-outdoor-air-system/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌲️create-outdoor-air-system/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌲️create-outdoor-air-system/🧪️tests/✅️ventilates-the-loop/🦀️.rs"]
                                    mod tests_ventilates_the_loop;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌲️create-outdoor-air-system/🧪️tests/⛔️refuses-an-absent-loop/🦀️.rs"]
                                    mod tests_refuses_an_absent_loop;
                                }
                                #[path = "."]
                                pub mod delete_outdoor_air_system {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🍂️delete-outdoor-air-system/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🍂️delete-outdoor-air-system/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🍂️delete-outdoor-air-system/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🍂️delete-outdoor-air-system/🧪️tests/✅️drops-the-system/🦀️.rs"]
                                    mod tests_drops_the_system;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🍂️delete-outdoor-air-system/🧪️tests/⛔️refuses-an-absent-one/🦀️.rs"]
                                    mod tests_refuses_an_absent_one;
                                }
                                #[path = "."]
                                pub mod change_outdoor_air_system_air_loop {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/⛓️change-outdoor-air-system-air-loop/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/⛓️change-outdoor-air-system-air-loop/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/⛓️change-outdoor-air-system-air-loop/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/⛓️change-outdoor-air-system-air-loop/🧪️tests/✅️moves-to-the-spare/🦀️.rs"]
                                    mod tests_moves_to_the_spare;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/⛓️change-outdoor-air-system-air-loop/🧪️tests/⛔️refuses-an-absent-loop/🦀️.rs"]
                                    mod tests_refuses_an_absent_loop;
                                }
                                #[path = "."]
                                pub mod change_outdoor_air_system_min_oa_flow {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦋️change-outdoor-air-system-min-oa-flow/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦋️change-outdoor-air-system-min-oa-flow/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦋️change-outdoor-air-system-min-oa-flow/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦋️change-outdoor-air-system-min-oa-flow/🧪️tests/✅️raises-the-minimum/🦀️.rs"]
                                    mod tests_raises_the_minimum;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦋️change-outdoor-air-system-min-oa-flow/🧪️tests/⛔️refuses-a-negative/🦀️.rs"]
                                    mod tests_refuses_a_negative;
                                }
                                #[path = "."]
                                pub mod change_outdoor_air_system_economizer_enabled {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💰️change-outdoor-air-system-economizer-enabled/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💰️change-outdoor-air-system-economizer-enabled/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💰️change-outdoor-air-system-economizer-enabled/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💰️change-outdoor-air-system-economizer-enabled/🧪️tests/✅️frees-the-cooling/🦀️.rs"]
                                    mod tests_frees_the_cooling;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💰️change-outdoor-air-system-economizer-enabled/🧪️tests/⛔️refuses-an-absent-row/🦀️.rs"]
                                    mod tests_refuses_an_absent_row;
                                }
                                #[path = "."]
                                pub mod create_electrical_load_center {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏦️create-electrical-load-center/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏦️create-electrical-load-center/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏦️create-electrical-load-center/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏦️create-electrical-load-center/🧪️tests/✅️applies/🦀️.rs"]
                                    mod tests_applies;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏦️create-electrical-load-center/🧪️tests/⛔️refuses/🦀️.rs"]
                                    mod tests_refuses;
                                }
                                #[path = "."]
                                pub mod delete_electrical_load_center {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔻️delete-electrical-load-center/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔻️delete-electrical-load-center/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔻️delete-electrical-load-center/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔻️delete-electrical-load-center/🧪️tests/✅️applies/🦀️.rs"]
                                    mod tests_applies;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔻️delete-electrical-load-center/🧪️tests/⛔️refuses/🦀️.rs"]
                                    mod tests_refuses;
                                }
                                #[path = "."]
                                pub mod rename_electrical_load_center {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🖊️rename-electrical-load-center/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🖊️rename-electrical-load-center/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🖊️rename-electrical-load-center/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🖊️rename-electrical-load-center/🧪️tests/✅️applies/🦀️.rs"]
                                    mod tests_applies;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🖊️rename-electrical-load-center/🧪️tests/⛔️refuses/🦀️.rs"]
                                    mod tests_refuses;
                                }
                                #[path = "."]
                                pub mod add_electrical_load_center_pv {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/☄️add-electrical-load-center-pv/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/☄️add-electrical-load-center-pv/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/☄️add-electrical-load-center-pv/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/☄️add-electrical-load-center-pv/🧪️tests/✅️applies/🦀️.rs"]
                                    mod tests_applies;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/☄️add-electrical-load-center-pv/🧪️tests/⛔️refuses/🦀️.rs"]
                                    mod tests_refuses;
                                }
                                #[path = "."]
                                pub mod remove_electrical_load_center_pv {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌘️remove-electrical-load-center-pv/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌘️remove-electrical-load-center-pv/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌘️remove-electrical-load-center-pv/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌘️remove-electrical-load-center-pv/🧪️tests/✅️applies/🦀️.rs"]
                                    mod tests_applies;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌘️remove-electrical-load-center-pv/🧪️tests/⛔️refuses/🦀️.rs"]
                                    mod tests_refuses;
                                }
                                #[path = "."]
                                pub mod add_electrical_load_center_battery {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔋️add-electrical-load-center-battery/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔋️add-electrical-load-center-battery/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔋️add-electrical-load-center-battery/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔋️add-electrical-load-center-battery/🧪️tests/✅️applies/🦀️.rs"]
                                    mod tests_applies;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔋️add-electrical-load-center-battery/🧪️tests/⛔️refuses/🦀️.rs"]
                                    mod tests_refuses;
                                }
                                #[path = "."]
                                pub mod remove_electrical_load_center_battery {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪝️remove-electrical-load-center-battery/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪝️remove-electrical-load-center-battery/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪝️remove-electrical-load-center-battery/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪝️remove-electrical-load-center-battery/🧪️tests/✅️applies/🦀️.rs"]
                                    mod tests_applies;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪝️remove-electrical-load-center-battery/🧪️tests/⛔️refuses/🦀️.rs"]
                                    mod tests_refuses;
                                }
                                #[path = "."]
                                pub mod create_pv_system {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/✨️create-pv-system/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/✨️create-pv-system/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/✨️create-pv-system/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/✨️create-pv-system/🧪️tests/✅️applies/🦀️.rs"]
                                    mod tests_applies;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/✨️create-pv-system/🧪️tests/⛔️refuses/🦀️.rs"]
                                    mod tests_refuses;
                                }
                                #[path = "."]
                                pub mod delete_pv_system {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌒️delete-pv-system/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌒️delete-pv-system/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌒️delete-pv-system/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌒️delete-pv-system/🧪️tests/✅️applies/🦀️.rs"]
                                    mod tests_applies;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌒️delete-pv-system/🧪️tests/⛔️refuses/🦀️.rs"]
                                    mod tests_refuses;
                                }
                                #[path = "."]
                                pub mod change_pv_system_dc_capacity {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/⚛️change-pv-system-dc-capacity/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/⚛️change-pv-system-dc-capacity/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/⚛️change-pv-system-dc-capacity/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/⚛️change-pv-system-dc-capacity/🧪️tests/✅️applies/🦀️.rs"]
                                    mod tests_applies;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/⚛️change-pv-system-dc-capacity/🧪️tests/⛔️refuses/🦀️.rs"]
                                    mod tests_refuses;
                                }
                                #[path = "."]
                                pub mod change_pv_system_area {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🟨️change-pv-system-area/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🟨️change-pv-system-area/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🟨️change-pv-system-area/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🟨️change-pv-system-area/🧪️tests/✅️applies/🦀️.rs"]
                                    mod tests_applies;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🟨️change-pv-system-area/🧪️tests/⛔️refuses/🦀️.rs"]
                                    mod tests_refuses;
                                }
                                #[path = "."]
                                pub mod change_pv_system_tilt {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📈️change-pv-system-tilt/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📈️change-pv-system-tilt/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📈️change-pv-system-tilt/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📈️change-pv-system-tilt/🧪️tests/✅️applies/🦀️.rs"]
                                    mod tests_applies;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📈️change-pv-system-tilt/🧪️tests/⛔️refuses/🦀️.rs"]
                                    mod tests_refuses;
                                }
                                #[path = "."]
                                pub mod change_pv_system_azimuth {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧿️change-pv-system-azimuth/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧿️change-pv-system-azimuth/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧿️change-pv-system-azimuth/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧿️change-pv-system-azimuth/🧪️tests/✅️applies/🦀️.rs"]
                                    mod tests_applies;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧿️change-pv-system-azimuth/🧪️tests/⛔️refuses/🦀️.rs"]
                                    mod tests_refuses;
                                }
                                #[path = "."]
                                pub mod change_pv_system_module_efficiency {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🎖️change-pv-system-module-efficiency/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🎖️change-pv-system-module-efficiency/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🎖️change-pv-system-module-efficiency/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🎖️change-pv-system-module-efficiency/🧪️tests/✅️applies/🦀️.rs"]
                                    mod tests_applies;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🎖️change-pv-system-module-efficiency/🧪️tests/⛔️refuses/🦀️.rs"]
                                    mod tests_refuses;
                                }
                                #[path = "."]
                                pub mod change_pv_system_inverter_efficiency {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/♌️change-pv-system-inverter-efficiency/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/♌️change-pv-system-inverter-efficiency/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/♌️change-pv-system-inverter-efficiency/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/♌️change-pv-system-inverter-efficiency/🧪️tests/✅️applies/🦀️.rs"]
                                    mod tests_applies;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/♌️change-pv-system-inverter-efficiency/🧪️tests/⛔️refuses/🦀️.rs"]
                                    mod tests_refuses;
                                }
                                #[path = "."]
                                pub mod create_battery {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪙️create-battery/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪙️create-battery/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪙️create-battery/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪙️create-battery/🧪️tests/✅️applies/🦀️.rs"]
                                    mod tests_applies;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪙️create-battery/🧪️tests/⛔️refuses/🦀️.rs"]
                                    mod tests_refuses;
                                }
                                #[path = "."]
                                pub mod delete_battery {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/♒️delete-battery/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/♒️delete-battery/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/♒️delete-battery/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/♒️delete-battery/🧪️tests/✅️applies/🦀️.rs"]
                                    mod tests_applies;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/♒️delete-battery/🧪️tests/⛔️refuses/🦀️.rs"]
                                    mod tests_refuses;
                                }
                                #[path = "."]
                                pub mod change_battery_capacity {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🥫️change-battery-capacity/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🥫️change-battery-capacity/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🥫️change-battery-capacity/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🥫️change-battery-capacity/🧪️tests/✅️applies/🦀️.rs"]
                                    mod tests_applies;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🥫️change-battery-capacity/🧪️tests/⛔️refuses/🦀️.rs"]
                                    mod tests_refuses;
                                }
                                #[path = "."]
                                pub mod change_battery_max_charge {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/⏫️change-battery-max-charge/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/⏫️change-battery-max-charge/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/⏫️change-battery-max-charge/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/⏫️change-battery-max-charge/🧪️tests/✅️applies/🦀️.rs"]
                                    mod tests_applies;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/⏫️change-battery-max-charge/🧪️tests/⛔️refuses/🦀️.rs"]
                                    mod tests_refuses;
                                }
                                #[path = "."]
                                pub mod change_battery_max_discharge {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/⏬️change-battery-max-discharge/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/⏬️change-battery-max-discharge/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/⏬️change-battery-max-discharge/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/⏬️change-battery-max-discharge/🧪️tests/✅️applies/🦀️.rs"]
                                    mod tests_applies;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/⏬️change-battery-max-discharge/🧪️tests/⛔️refuses/🦀️.rs"]
                                    mod tests_refuses;
                                }
                                #[path = "."]
                                pub mod change_battery_round_trip_efficiency {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🥉️change-battery-round-trip-efficiency/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🥉️change-battery-round-trip-efficiency/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🥉️change-battery-round-trip-efficiency/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🥉️change-battery-round-trip-efficiency/🧪️tests/✅️applies/🦀️.rs"]
                                    mod tests_applies;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🥉️change-battery-round-trip-efficiency/🧪️tests/⛔️refuses/🦀️.rs"]
                                    mod tests_refuses;
                                }
                                #[path = "."]
                                pub mod create_shw_system {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🛀️create-shw-system/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🛀️create-shw-system/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🛀️create-shw-system/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🛀️create-shw-system/🧪️tests/✅️applies/🦀️.rs"]
                                    mod tests_applies;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🛀️create-shw-system/🧪️tests/⛔️refuses/🦀️.rs"]
                                    mod tests_refuses;
                                }
                                #[path = "."]
                                pub mod delete_shw_system {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🚱️delete-shw-system/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🚱️delete-shw-system/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🚱️delete-shw-system/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🚱️delete-shw-system/🧪️tests/✅️applies/🦀️.rs"]
                                    mod tests_applies;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🚱️delete-shw-system/🧪️tests/⛔️refuses/🦀️.rs"]
                                    mod tests_refuses;
                                }
                                #[path = "."]
                                pub mod change_shw_system_heater_capacity {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🍵️change-shw-system-heater-capacity/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🍵️change-shw-system-heater-capacity/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🍵️change-shw-system-heater-capacity/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🍵️change-shw-system-heater-capacity/🧪️tests/✅️applies/🦀️.rs"]
                                    mod tests_applies;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🍵️change-shw-system-heater-capacity/🧪️tests/⛔️refuses/🦀️.rs"]
                                    mod tests_refuses;
                                }
                                #[path = "."]
                                pub mod change_shw_system_storage_volume {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🛢️change-shw-system-storage-volume/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🛢️change-shw-system-storage-volume/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🛢️change-shw-system-storage-volume/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🛢️change-shw-system-storage-volume/🧪️tests/✅️applies/🦀️.rs"]
                                    mod tests_applies;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🛢️change-shw-system-storage-volume/🧪️tests/⛔️refuses/🦀️.rs"]
                                    mod tests_refuses;
                                }
                                #[path = "."]
                                pub mod change_shw_system_setpoint {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏹️change-shw-system-setpoint/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏹️change-shw-system-setpoint/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏹️change-shw-system-setpoint/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏹️change-shw-system-setpoint/🧪️tests/✅️applies/🦀️.rs"]
                                    mod tests_applies;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏹️change-shw-system-setpoint/🧪️tests/⛔️refuses/🦀️.rs"]
                                    mod tests_refuses;
                                }
                                #[path = "."]
                                pub mod change_shw_system_schedule {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🕐️change-shw-system-schedule/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🕐️change-shw-system-schedule/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🕐️change-shw-system-schedule/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🕐️change-shw-system-schedule/🧪️tests/✅️applies/🦀️.rs"]
                                    mod tests_applies;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🕐️change-shw-system-schedule/🧪️tests/⛔️refuses/🦀️.rs"]
                                    mod tests_refuses;
                                }
                                #[path = "."]
                                pub mod create_solar_thermal_system {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌄️create-solar-thermal-system/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌄️create-solar-thermal-system/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌄️create-solar-thermal-system/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌄️create-solar-thermal-system/🧪️tests/✅️applies/🦀️.rs"]
                                    mod tests_applies;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌄️create-solar-thermal-system/🧪️tests/⛔️refuses/🦀️.rs"]
                                    mod tests_refuses;
                                }
                                #[path = "."]
                                pub mod delete_solar_thermal_system {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌆️delete-solar-thermal-system/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌆️delete-solar-thermal-system/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌆️delete-solar-thermal-system/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌆️delete-solar-thermal-system/🧪️tests/✅️applies/🦀️.rs"]
                                    mod tests_applies;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌆️delete-solar-thermal-system/🧪️tests/⛔️refuses/🦀️.rs"]
                                    mod tests_refuses;
                                }
                                #[path = "."]
                                pub mod change_solar_thermal_system_collector_area {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🟩️change-solar-thermal-system-collector-area/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🟩️change-solar-thermal-system-collector-area/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🟩️change-solar-thermal-system-collector-area/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🟩️change-solar-thermal-system-collector-area/🧪️tests/✅️applies/🦀️.rs"]
                                    mod tests_applies;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🟩️change-solar-thermal-system-collector-area/🧪️tests/⛔️refuses/🦀️.rs"]
                                    mod tests_refuses;
                                }
                                #[path = "."]
                                pub mod change_solar_thermal_system_efficiency {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏅️change-solar-thermal-system-efficiency/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏅️change-solar-thermal-system-efficiency/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏅️change-solar-thermal-system-efficiency/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏅️change-solar-thermal-system-efficiency/🧪️tests/✅️applies/🦀️.rs"]
                                    mod tests_applies;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏅️change-solar-thermal-system-efficiency/🧪️tests/⛔️refuses/🦀️.rs"]
                                    mod tests_refuses;
                                }
                                #[path = "."]
                                pub mod change_solar_thermal_system_storage_volume {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧃️change-solar-thermal-system-storage-volume/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧃️change-solar-thermal-system-storage-volume/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧃️change-solar-thermal-system-storage-volume/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧃️change-solar-thermal-system-storage-volume/🧪️tests/✅️applies/🦀️.rs"]
                                    mod tests_applies;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧃️change-solar-thermal-system-storage-volume/🧪️tests/⛔️refuses/🦀️.rs"]
                                    mod tests_refuses;
                                }
                                #[path = "."]
                                pub mod change_solar_thermal_system_tilt {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔼️change-solar-thermal-system-tilt/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔼️change-solar-thermal-system-tilt/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔼️change-solar-thermal-system-tilt/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔼️change-solar-thermal-system-tilt/🧪️tests/✅️applies/🦀️.rs"]
                                    mod tests_applies;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔼️change-solar-thermal-system-tilt/🧪️tests/⛔️refuses/🦀️.rs"]
                                    mod tests_refuses;
                                }
                                #[path = "."]
                                pub mod change_solar_thermal_system_azimuth {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/⛵️change-solar-thermal-system-azimuth/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/⛵️change-solar-thermal-system-azimuth/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/⛵️change-solar-thermal-system-azimuth/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/⛵️change-solar-thermal-system-azimuth/🧪️tests/✅️applies/🦀️.rs"]
                                    mod tests_applies;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/⛵️change-solar-thermal-system-azimuth/🧪️tests/⛔️refuses/🦀️.rs"]
                                    mod tests_refuses;
                                }
                                #[path = "."]
                                pub mod create_refrigeration_system {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/❄️create-refrigeration-system/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/❄️create-refrigeration-system/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/❄️create-refrigeration-system/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/❄️create-refrigeration-system/🧪️tests/✅️applies/🦀️.rs"]
                                    mod tests_applies;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/❄️create-refrigeration-system/🧪️tests/⛔️refuses/🦀️.rs"]
                                    mod tests_refuses;
                                }
                                #[path = "."]
                                pub mod delete_refrigeration_system {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🫠️delete-refrigeration-system/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🫠️delete-refrigeration-system/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🫠️delete-refrigeration-system/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🫠️delete-refrigeration-system/🧪️tests/✅️applies/🦀️.rs"]
                                    mod tests_applies;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🫠️delete-refrigeration-system/🧪️tests/⛔️refuses/🦀️.rs"]
                                    mod tests_refuses;
                                }
                                #[path = "."]
                                pub mod change_refrigeration_system_case_count {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🗄️change-refrigeration-system-case-count/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🗄️change-refrigeration-system-case-count/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🗄️change-refrigeration-system-case-count/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🗄️change-refrigeration-system-case-count/🧪️tests/✅️applies/🦀️.rs"]
                                    mod tests_applies;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🗄️change-refrigeration-system-case-count/🧪️tests/⛔️refuses/🦀️.rs"]
                                    mod tests_refuses;
                                }
                                #[path = "."]
                                pub mod change_refrigeration_system_design_load {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏋️change-refrigeration-system-design-load/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏋️change-refrigeration-system-design-load/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏋️change-refrigeration-system-design-load/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏋️change-refrigeration-system-design-load/🧪️tests/✅️applies/🦀️.rs"]
                                    mod tests_applies;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏋️change-refrigeration-system-design-load/🧪️tests/⛔️refuses/🦀️.rs"]
                                    mod tests_refuses;
                                }
                                #[path = "."]
                                pub mod change_refrigeration_system_defrost_schedule {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🕑️change-refrigeration-system-defrost-schedule/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🕑️change-refrigeration-system-defrost-schedule/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🕑️change-refrigeration-system-defrost-schedule/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🕑️change-refrigeration-system-defrost-schedule/🧪️tests/✅️applies/🦀️.rs"]
                                    mod tests_applies;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🕑️change-refrigeration-system-defrost-schedule/🧪️tests/⛔️refuses/🦀️.rs"]
                                    mod tests_refuses;
                                }
                                #[path = "."]
                                pub mod create_water_system {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🚽️create-water-system/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🚽️create-water-system/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🚽️create-water-system/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🚽️create-water-system/🧪️tests/✅️applies/🦀️.rs"]
                                    mod tests_applies;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🚽️create-water-system/🧪️tests/⛔️refuses/🦀️.rs"]
                                    mod tests_refuses;
                                }
                                #[path = "."]
                                pub mod delete_water_system {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧼️delete-water-system/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧼️delete-water-system/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧼️delete-water-system/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧼️delete-water-system/🧪️tests/✅️applies/🦀️.rs"]
                                    mod tests_applies;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧼️delete-water-system/🧪️tests/⛔️refuses/🦀️.rs"]
                                    mod tests_refuses;
                                }
                                #[path = "."]
                                pub mod change_water_system_fixture_count {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪣️change-water-system-fixture-count/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪣️change-water-system-fixture-count/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪣️change-water-system-fixture-count/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪣️change-water-system-fixture-count/🧪️tests/✅️applies/🦀️.rs"]
                                    mod tests_applies;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪣️change-water-system-fixture-count/🧪️tests/⛔️refuses/🦀️.rs"]
                                    mod tests_refuses;
                                }
                                #[path = "."]
                                pub mod change_water_system_peak_flow {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🚾️change-water-system-peak-flow/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🚾️change-water-system-peak-flow/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🚾️change-water-system-peak-flow/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🚾️change-water-system-peak-flow/🧪️tests/✅️applies/🦀️.rs"]
                                    mod tests_applies;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🚾️change-water-system-peak-flow/🧪️tests/⛔️refuses/🦀️.rs"]
                                    mod tests_refuses;
                                }
                                #[path = "."]
                                pub mod change_water_system_schedule {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🕒️change-water-system-schedule/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🕒️change-water-system-schedule/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🕒️change-water-system-schedule/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🕒️change-water-system-schedule/🧪️tests/✅️applies/🦀️.rs"]
                                    mod tests_applies;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🕒️change-water-system-schedule/🧪️tests/⛔️refuses/🦀️.rs"]
                                    mod tests_refuses;
                                }
                                #[path = "."]
                                pub mod create_fault {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/⚠️create-fault/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/⚠️create-fault/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/⚠️create-fault/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/⚠️create-fault/🧪️tests/✅️applies/🦀️.rs"]
                                    mod tests_applies;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/⚠️create-fault/🧪️tests/⛔️refuses/🦀️.rs"]
                                    mod tests_refuses;
                                }
                                #[path = "."]
                                pub mod delete_fault {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🩹️delete-fault/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🩹️delete-fault/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🩹️delete-fault/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🩹️delete-fault/🧪️tests/✅️applies/🦀️.rs"]
                                    mod tests_applies;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🩹️delete-fault/🧪️tests/⛔️refuses/🦀️.rs"]
                                    mod tests_refuses;
                                }
                                #[path = "."]
                                pub mod change_fault_target_equipment {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🎣️change-fault-target-equipment/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🎣️change-fault-target-equipment/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🎣️change-fault-target-equipment/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🎣️change-fault-target-equipment/🧪️tests/✅️applies/🦀️.rs"]
                                    mod tests_applies;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🎣️change-fault-target-equipment/🧪️tests/⛔️refuses/🦀️.rs"]
                                    mod tests_refuses;
                                }
                                #[path = "."]
                                pub mod change_fault_type {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🐛️change-fault-type/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🐛️change-fault-type/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🐛️change-fault-type/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🐛️change-fault-type/🧪️tests/✅️applies/🦀️.rs"]
                                    mod tests_applies;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🐛️change-fault-type/🧪️tests/⛔️refuses/🦀️.rs"]
                                    mod tests_refuses;
                                }
                                #[path = "."]
                                pub mod change_fault_severity {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌶️change-fault-severity/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌶️change-fault-severity/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌶️change-fault-severity/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌶️change-fault-severity/🧪️tests/✅️applies/🦀️.rs"]
                                    mod tests_applies;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌶️change-fault-severity/🧪️tests/⛔️refuses/🦀️.rs"]
                                    mod tests_refuses;
                                }
                                #[path = "."]
                                pub mod change_fault_start_schedule {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🕓️change-fault-start-schedule/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🕓️change-fault-start-schedule/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🕓️change-fault-start-schedule/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🕓️change-fault-start-schedule/🧪️tests/✅️applies/🦀️.rs"]
                                    mod tests_applies;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🕓️change-fault-start-schedule/🧪️tests/⛔️refuses/🦀️.rs"]
                                    mod tests_refuses;
                                }
                                #[path = "."]
                                pub mod create_space_list {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📋️create-space-list/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📋️create-space-list/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📋️create-space-list/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📋️create-space-list/🧪️tests/✅️applies/🦀️.rs"]
                                    mod tests_applies;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📋️create-space-list/🧪️tests/⛔️refuses/🦀️.rs"]
                                    mod tests_refuses;
                                }
                                #[path = "."]
                                pub mod delete_space_list {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🗒️delete-space-list/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🗒️delete-space-list/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🗒️delete-space-list/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🗒️delete-space-list/🧪️tests/✅️applies/🦀️.rs"]
                                    mod tests_applies;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🗒️delete-space-list/🧪️tests/⛔️refuses/🦀️.rs"]
                                    mod tests_refuses;
                                }
                                #[path = "."]
                                pub mod rename_space_list {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪶️rename-space-list/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪶️rename-space-list/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪶️rename-space-list/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪶️rename-space-list/🧪️tests/✅️applies/🦀️.rs"]
                                    mod tests_applies;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪶️rename-space-list/🧪️tests/⛔️refuses/🦀️.rs"]
                                    mod tests_refuses;
                                }
                                #[path = "."]
                                pub mod add_space_list_member {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/➡️add-space-list-member/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/➡️add-space-list-member/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/➡️add-space-list-member/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/➡️add-space-list-member/🧪️tests/✅️applies/🦀️.rs"]
                                    mod tests_applies;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/➡️add-space-list-member/🧪️tests/⛔️refuses/🦀️.rs"]
                                    mod tests_refuses;
                                }
                                #[path = "."]
                                pub mod remove_space_list_member {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪤️remove-space-list-member/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪤️remove-space-list-member/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪤️remove-space-list-member/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪤️remove-space-list-member/🧪️tests/✅️applies/🦀️.rs"]
                                    mod tests_applies;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪤️remove-space-list-member/🧪️tests/⛔️refuses/🦀️.rs"]
                                    mod tests_refuses;
                                }
                                #[path = "."]
                                pub mod create_thermal_enclosure {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏟️create-thermal-enclosure/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏟️create-thermal-enclosure/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏟️create-thermal-enclosure/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏟️create-thermal-enclosure/🧪️tests/✅️applies/🦀️.rs"]
                                    mod tests_applies;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏟️create-thermal-enclosure/🧪️tests/⛔️refuses/🦀️.rs"]
                                    mod tests_refuses;
                                }
                                #[path = "."]
                                pub mod delete_thermal_enclosure {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏯️delete-thermal-enclosure/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏯️delete-thermal-enclosure/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏯️delete-thermal-enclosure/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏯️delete-thermal-enclosure/🧪️tests/✅️applies/🦀️.rs"]
                                    mod tests_applies;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏯️delete-thermal-enclosure/🧪️tests/⛔️refuses/🦀️.rs"]
                                    mod tests_refuses;
                                }
                                #[path = "."]
                                pub mod rename_thermal_enclosure {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🖋️rename-thermal-enclosure/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🖋️rename-thermal-enclosure/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🖋️rename-thermal-enclosure/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🖋️rename-thermal-enclosure/🧪️tests/✅️applies/🦀️.rs"]
                                    mod tests_applies;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🖋️rename-thermal-enclosure/🧪️tests/⛔️refuses/🦀️.rs"]
                                    mod tests_refuses;
                                }
                                #[path = "."]
                                pub mod add_thermal_enclosure_zone {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔒️add-thermal-enclosure-zone/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔒️add-thermal-enclosure-zone/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔒️add-thermal-enclosure-zone/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔒️add-thermal-enclosure-zone/🧪️tests/✅️applies/🦀️.rs"]
                                    mod tests_applies;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔒️add-thermal-enclosure-zone/🧪️tests/⛔️refuses/🦀️.rs"]
                                    mod tests_refuses;
                                }
                                #[path = "."]
                                pub mod remove_thermal_enclosure_zone {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔓️remove-thermal-enclosure-zone/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔓️remove-thermal-enclosure-zone/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔓️remove-thermal-enclosure-zone/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔓️remove-thermal-enclosure-zone/🧪️tests/✅️applies/🦀️.rs"]
                                    mod tests_applies;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔓️remove-thermal-enclosure-zone/🧪️tests/⛔️refuses/🦀️.rs"]
                                    mod tests_refuses;
                                }
                                #[path = "."]
                                pub mod create_constant_schedule {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🕜️create-constant-schedule/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🕜️create-constant-schedule/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🕜️create-constant-schedule/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🕜️create-constant-schedule/🧪️tests/✅️applies/🦀️.rs"]
                                    mod tests_applies;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🕜️create-constant-schedule/🧪️tests/⛔️refuses/🦀️.rs"]
                                    mod tests_refuses;
                                }
                                #[path = "."]
                                pub mod delete_constant_schedule {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📍️delete-constant-schedule/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📍️delete-constant-schedule/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📍️delete-constant-schedule/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📍️delete-constant-schedule/🧪️tests/✅️applies/🦀️.rs"]
                                    mod tests_applies;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📍️delete-constant-schedule/🧪️tests/⛔️refuses/🦀️.rs"]
                                    mod tests_refuses;
                                }
                                #[path = "."]
                                pub mod change_constant_schedule_value {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🕝️change-constant-schedule-value/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🕝️change-constant-schedule-value/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🕝️change-constant-schedule-value/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🕝️change-constant-schedule-value/🧪️tests/✅️applies/🦀️.rs"]
                                    mod tests_applies;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🕝️change-constant-schedule-value/🧪️tests/⛔️refuses/🦀️.rs"]
                                    mod tests_refuses;
                                }
                                #[path = "."]
                                pub mod create_daily_schedule {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🕞️create-daily-schedule/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🕞️create-daily-schedule/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🕞️create-daily-schedule/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🕞️create-daily-schedule/🧪️tests/✅️applies/🦀️.rs"]
                                    mod tests_applies;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🕞️create-daily-schedule/🧪️tests/⛔️refuses/🦀️.rs"]
                                    mod tests_refuses;
                                }
                                #[path = "."]
                                pub mod delete_daily_schedule {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌓️delete-daily-schedule/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌓️delete-daily-schedule/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌓️delete-daily-schedule/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌓️delete-daily-schedule/🧪️tests/✅️applies/🦀️.rs"]
                                    mod tests_applies;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌓️delete-daily-schedule/🧪️tests/⛔️refuses/🦀️.rs"]
                                    mod tests_refuses;
                                }
                                #[path = "."]
                                pub mod replace_daily_schedule_hourly_values {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🕔️replace-daily-schedule-hourly-values/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🕔️replace-daily-schedule-hourly-values/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🕔️replace-daily-schedule-hourly-values/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🕔️replace-daily-schedule-hourly-values/🧪️tests/✅️applies/🦀️.rs"]
                                    mod tests_applies;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🕔️replace-daily-schedule-hourly-values/🧪️tests/⛔️refuses/🦀️.rs"]
                                    mod tests_refuses;
                                }
                                #[path = "."]
                                pub mod change_daily_schedule_interpolation {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🕕️change-daily-schedule-interpolation/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🕕️change-daily-schedule-interpolation/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🕕️change-daily-schedule-interpolation/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🕕️change-daily-schedule-interpolation/🧪️tests/✅️applies/🦀️.rs"]
                                    mod tests_applies;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🕕️change-daily-schedule-interpolation/🧪️tests/⛔️refuses/🦀️.rs"]
                                    mod tests_refuses;
                                }
                                #[path = "."]
                                pub mod change_daily_schedule_limits {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🕟️change-daily-schedule-limits/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🕟️change-daily-schedule-limits/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🕟️change-daily-schedule-limits/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🕟️change-daily-schedule-limits/🧪️tests/✅️applies/🦀️.rs"]
                                    mod tests_applies;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🕟️change-daily-schedule-limits/🧪️tests/⛔️refuses/🦀️.rs"]
                                    mod tests_refuses;
                                }
                                #[path = "."]
                                pub mod create_weekly_schedule {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🗓️create-weekly-schedule/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🗓️create-weekly-schedule/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🗓️create-weekly-schedule/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🗓️create-weekly-schedule/🧪️tests/✅️applies/🦀️.rs"]
                                    mod tests_applies;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🗓️create-weekly-schedule/🧪️tests/⛔️refuses/🦀️.rs"]
                                    mod tests_refuses;
                                }
                                #[path = "."]
                                pub mod delete_weekly_schedule {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🕖️delete-weekly-schedule/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🕖️delete-weekly-schedule/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🕖️delete-weekly-schedule/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🕖️delete-weekly-schedule/🧪️tests/✅️applies/🦀️.rs"]
                                    mod tests_applies;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🕖️delete-weekly-schedule/🧪️tests/⛔️refuses/🦀️.rs"]
                                    mod tests_refuses;
                                }
                                #[path = "."]
                                pub mod change_weekly_schedule_day {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🕗️change-weekly-schedule-day/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🕗️change-weekly-schedule-day/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🕗️change-weekly-schedule-day/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🕗️change-weekly-schedule-day/🧪️tests/✅️applies/🦀️.rs"]
                                    mod tests_applies;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🕗️change-weekly-schedule-day/🧪️tests/⛔️refuses/🦀️.rs"]
                                    mod tests_refuses;
                                }
                                #[path = "."]
                                pub mod create_annual_schedule {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📚️create-annual-schedule/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📚️create-annual-schedule/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📚️create-annual-schedule/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📚️create-annual-schedule/🧪️tests/✅️applies/🦀️.rs"]
                                    mod tests_applies;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📚️create-annual-schedule/🧪️tests/⛔️refuses/🦀️.rs"]
                                    mod tests_refuses;
                                }
                                #[path = "."]
                                pub mod delete_annual_schedule {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📕️delete-annual-schedule/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📕️delete-annual-schedule/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📕️delete-annual-schedule/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📕️delete-annual-schedule/🧪️tests/✅️applies/🦀️.rs"]
                                    mod tests_applies;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📕️delete-annual-schedule/🧪️tests/⛔️refuses/🦀️.rs"]
                                    mod tests_refuses;
                                }
                                #[path = "."]
                                pub mod insert_annual_schedule_rule {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📗️insert-annual-schedule-rule/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📗️insert-annual-schedule-rule/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📗️insert-annual-schedule-rule/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📗️insert-annual-schedule-rule/🧪️tests/✅️applies/🦀️.rs"]
                                    mod tests_applies;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📗️insert-annual-schedule-rule/🧪️tests/⛔️refuses/🦀️.rs"]
                                    mod tests_refuses;
                                }
                                #[path = "."]
                                pub mod remove_annual_schedule_rule {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📙️remove-annual-schedule-rule/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📙️remove-annual-schedule-rule/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📙️remove-annual-schedule-rule/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📙️remove-annual-schedule-rule/🧪️tests/✅️applies/🦀️.rs"]
                                    mod tests_applies;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📙️remove-annual-schedule-rule/🧪️tests/⛔️refuses/🦀️.rs"]
                                    mod tests_refuses;
                                }
                                #[path = "."]
                                pub mod reorder_annual_schedule_rules {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🗂️reorder-annual-schedule-rules/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🗂️reorder-annual-schedule-rules/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🗂️reorder-annual-schedule-rules/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🗂️reorder-annual-schedule-rules/🧪️tests/✅️applies/🦀️.rs"]
                                    mod tests_applies;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🗂️reorder-annual-schedule-rules/🧪️tests/⛔️refuses/🦀️.rs"]
                                    mod tests_refuses;
                                }
                                #[path = "."]
                                pub mod change_annual_schedule_default_daily_schedule {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🎌️change-annual-schedule-default-daily-schedule/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🎌️change-annual-schedule-default-daily-schedule/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🎌️change-annual-schedule-default-daily-schedule/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🎌️change-annual-schedule-default-daily-schedule/🧪️tests/✅️applies/🦀️.rs"]
                                    mod tests_applies;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🎌️change-annual-schedule-default-daily-schedule/🧪️tests/⛔️refuses/🦀️.rs"]
                                    mod tests_refuses;
                                }
                                #[path = "."]
                                pub mod change_annual_schedule_holiday_daily_schedule {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🎄️change-annual-schedule-holiday-daily-schedule/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🎄️change-annual-schedule-holiday-daily-schedule/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🎄️change-annual-schedule-holiday-daily-schedule/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🎄️change-annual-schedule-holiday-daily-schedule/🧪️tests/✅️applies/🦀️.rs"]
                                    mod tests_applies;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🎄️change-annual-schedule-holiday-daily-schedule/🧪️tests/⛔️refuses/🦀️.rs"]
                                    mod tests_refuses;
                                }
                                #[path = "."]
                                pub mod add_annual_schedule_holiday {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🎉️add-annual-schedule-holiday/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🎉️add-annual-schedule-holiday/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🎉️add-annual-schedule-holiday/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🎉️add-annual-schedule-holiday/🧪️tests/✅️applies/🦀️.rs"]
                                    mod tests_applies;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🎉️add-annual-schedule-holiday/🧪️tests/⛔️refuses/🦀️.rs"]
                                    mod tests_refuses;
                                }
                                #[path = "."]
                                pub mod remove_annual_schedule_holiday {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🎊️remove-annual-schedule-holiday/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🎊️remove-annual-schedule-holiday/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🎊️remove-annual-schedule-holiday/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🎊️remove-annual-schedule-holiday/🧪️tests/✅️applies/🦀️.rs"]
                                    mod tests_applies;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🎊️remove-annual-schedule-holiday/🧪️tests/⛔️refuses/🦀️.rs"]
                                    mod tests_refuses;
                                }
                                #[path = "."]
                                pub mod create_time_series_schedule {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪗️create-time-series-schedule/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪗️create-time-series-schedule/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪗️create-time-series-schedule/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪗️create-time-series-schedule/🧪️tests/✅️applies/🦀️.rs"]
                                    mod tests_applies;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪗️create-time-series-schedule/🧪️tests/⛔️refuses/🦀️.rs"]
                                    mod tests_refuses;
                                }
                                #[path = "."]
                                pub mod delete_time_series_schedule {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🎞️delete-time-series-schedule/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🎞️delete-time-series-schedule/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🎞️delete-time-series-schedule/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🎞️delete-time-series-schedule/🧪️tests/✅️applies/🦀️.rs"]
                                    mod tests_applies;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🎞️delete-time-series-schedule/🧪️tests/⛔️refuses/🦀️.rs"]
                                    mod tests_refuses;
                                }
                                #[path = "."]
                                pub mod replace_time_series_schedule_values {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🕘️replace-time-series-schedule-values/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🕘️replace-time-series-schedule-values/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🕘️replace-time-series-schedule-values/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🕘️replace-time-series-schedule-values/🧪️tests/✅️applies/🦀️.rs"]
                                    mod tests_applies;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🕘️replace-time-series-schedule-values/🧪️tests/⛔️refuses/🦀️.rs"]
                                    mod tests_refuses;
                                }
                                #[path = "."]
                                pub mod change_time_series_schedule_timestep {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🕙️change-time-series-schedule-timestep/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🕙️change-time-series-schedule-timestep/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🕙️change-time-series-schedule-timestep/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🕙️change-time-series-schedule-timestep/🧪️tests/✅️applies/🦀️.rs"]
                                    mod tests_applies;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🕙️change-time-series-schedule-timestep/🧪️tests/⛔️refuses/🦀️.rs"]
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
                                        pub mod csv {
                                            #[path = "."]
                                            pub mod v_rfc4180 {
                                                #[path = "."]
                                                pub mod any {
                                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/📊️csv/🔖️rfc4180/✳️any/🦀️.rs"]
                                                    mod component;
                                                    pub use component::*;
                                                }
                                            }
                                        }
                                        #[path = "."]
                                        pub mod xlsx {
                                            #[path = "."]
                                            pub mod v_ecma_376 {
                                                #[path = "."]
                                                pub mod any {
                                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/📕️xlsx/🔖️ecma-376/✳️any/🦀️.rs"]
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
                                        pub mod csv {
                                            #[path = "."]
                                            pub mod v_rfc4180 {
                                                #[path = "."]
                                                pub mod any {
                                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📤️export/🧵️serializers/🗿️artifacts/📊️csv/🔖️rfc4180/✳️any/🦀️.rs"]
                                                    mod component;
                                                    pub use component::*;
                                                }
                                            }
                                        }
                                        #[path = "."]
                                        pub mod xlsx {
                                            #[path = "."]
                                            pub mod v_ecma_376 {
                                                #[path = "."]
                                                pub mod any {
                                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📤️export/🧵️serializers/🗿️artifacts/📕️xlsx/🔖️ecma-376/✳️any/🦀️.rs"]
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
        pub mod io {
            pub use super::standards::v1::subsets::any::io::*;
        }
        pub mod op {
            pub use crate::standards::v1::subsets::any::schema::mutations::text::*;
        }
        pub mod document_dsl {
            pub use crate::standards::v1::subsets::any::schema::snapshot::text::*;
        }
        pub mod spr {
            pub use crate::standards::v1::subsets::any::schema::mutations::binary::*;
        }
        pub mod diff {
            pub use crate::standards::v1::subsets::any::schema::diff::*;
            pub mod schema {
                pub use crate::standards::v1::subsets::any::schema::diff::*;
            }
            pub mod text {
                pub use crate::standards::v1::subsets::any::schema::diff::text::*;
            }
            pub mod pack {
                pub use crate::standards::v1::subsets::any::schema::diff::binary::*;
            }
            pub mod binary {
                pub use crate::standards::v1::subsets::any::schema::diff::binary::*;
            }
        }
        pub mod mutations {
            pub use crate::standards::v1::subsets::any::schema::mutations::*;
            pub mod schema {
                pub use crate::standards::v1::subsets::any::schema::mutations::*;
            }
            pub mod text {
                pub use crate::standards::v1::subsets::any::schema::mutations::text::*;
            }
            pub mod pack {
                pub use crate::standards::v1::subsets::any::schema::mutations::binary::*;
            }
            pub mod binary {
                pub use crate::standards::v1::subsets::any::schema::mutations::binary::*;
            }
        }
        pub mod snapshot {
            pub use crate::standards::v1::subsets::any::schema::snapshot::*;
            pub mod schema {
                pub use crate::standards::v1::subsets::any::schema::snapshot::*;
            }
            pub mod text {
                pub use crate::standards::v1::subsets::any::schema::snapshot::text::*;
            }
            pub mod pack {
                pub use crate::standards::v1::subsets::any::schema::snapshot::binary::*;
            }
            pub mod binary {
                pub use crate::standards::v1::subsets::any::schema::snapshot::binary::*;
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
                #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🏛️bestest-600FF/🦀️.rs"]
                mod component;
                pub use component::*;
                #[cfg(test)]
                #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🏛️bestest-600FF/🧪️tests/🧩️example/🦀️.rs"]
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
                #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🏛️bestest-900FF/🦀️.rs"]
                mod component;
                pub use component::*;
                #[cfg(test)]
                #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🏛️bestest-900FF/🧪️tests/🧩️example/🦀️.rs"]
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
            }
        }

// 🎭️ Ticket 26/08/16/ARTIFACT-VIEWERS-AND-EDITORS-PER-SUBSET: energy's FIRST authored editor+viewer
// surfaces for `s.energy.model@1/*` — energy had zero document apps, so there was no app tree to
// migrate here (contrast the pilot's `📐️cad`, which moved an existing `🎛️apps/📐️cad/` tree). Two
// independent `#[path = "."]` trees, mirroring `🔖️Artifacts` above: `editor` mounts real
// mutation-capable content, `viewer` mounts an independently-authored read-only twin that never
// imports through `editor` (`policyViewerPurityBreaches`). Facet dirs that hold only
// `📌️.empty.md` (`🎚️config`/`🎮️commands`/`👥️presence`/`🫧️transient` at every surface/mode level) need
// no mount — nothing real lives there yet (`Config`/`Presence`/`Transient` = `NoConfig`/`NoPresence`/
// `NoTransient`).
#[path = "."]
pub mod editor {
    #[path = "."]
    pub mod model {
        #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs"]
        mod component;
        pub use component::*;

        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod edit {
                #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🦀️.rs"]
                mod component;
                pub use component::*;

                #[path = "."]
                pub mod windows {
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
        pub mod modes {
            #[path = "."]
            pub mod view {
                #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/👁️viewer/🎭️modes/👁️view/🦀️.rs"]
                mod component;
                pub use component::*;

                #[path = "."]
                pub mod windows {
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

