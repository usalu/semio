//! 🧊️ Explicit numerical reference EnergyModel snapshot — the persisted artifact-lane fields, the composed `structure`/`zones` children and the
//! value/table bridge they are built from, snapshot construction and whole-document load, and the snapshot's own text,
//! binary and sqlite codecs.

#![allow(async_fn_in_trait)]
#![allow(clippy::too_many_arguments)]
extern crate semio_framework_os_kernel as store;
extern crate protocol;
extern crate semio_framework_schema as framework_schema;
extern crate semio_s_plugin_energy_engine_air_exchange as air_exchange;
extern crate semio_s_plugin_energy_engine_calendar as calendar;
extern crate semio_s_plugin_energy_engine_model as model;
extern crate semio_s_plugin_energy_engine_schedule as schedule;

use {semio_framework_manifest_media::ArtifactKindSpec,semio_framework_artifact_reference::Dialect,semio_framework_manifest_media::MediaClass,semio_framework_manifest_media::MediaForm,semio_framework_manifest_media::MediaType,semio_framework_manifest_media::OsMediaCapability,semio_framework_artifact_reference::StandardId,semio_framework_artifact_reference::SubsetId};
use crate::model::{EntityId, Model};
use crate::state;
use framework_schema::ArtifactSchema;
use semio_framework_value::DslValue;
use semio_framework_value::FromValue;
use semio_framework_value::ToValue;
use semio_framework_value::ValueError;


//#region 🔖️Snapshot
/// 📸️ Persisted energy-model document snapshot (persistent fields of the artifact). Ticket
/// 26/08/12/UNIFIED-COMPOSABLE-ARTIFACT-SYSTEM (`energy→C:value,table R:model`): the old
/// `model_json: String` opaque-JSON field is replaced by two fixed composed CHILD slots — this
/// artifact no longer defines its own persisted-value/table content model, it composes stdio's
/// `value`/`table` subsets instead (see the artifact root's `🔖️Composition` region for the full
/// before/after and the honest exception carve-out for `Surface.vertices_m`). `referenced_model` is
/// a new forward `ArtifactLink` slot. `#[child(...)]`/`#[link_slot(...)]` drive
/// `#[derive(ArtifactSchema)]`'s slot-table emission; never hand-written. Text and pack both encode
/// the derived `EnergyModelPackRecord` below.
#[derive(Clone, Debug, PartialEq, ArtifactSchema, semio_framework_value::RetireOwned)]
#[artifact_schema(id = "s.energy.model")]
pub struct EnergyModelSnapshot {
    #[state(artifact)]
    pub schema: String,
    #[state(artifact)]
    pub model: crate::model::Model,
    #[state(artifact)]
    #[child(kind = "s.stdio.semio")]
    pub structure: EnergyStructureChild,
    #[state(artifact)]
    #[child(kind = "s.stdio.semio")]
    pub zones: EnergyZonesChild,
    #[state(artifact)]
    #[link_slot(roles("model"))]
    pub referenced_model: Option<store::ArtifactLink>,
    /// 🌦️ Forward link to the `🌦️epw` stdio artifact this model is simulated against — a link slot
    /// exactly like `referenced_model`, never an inlined `WeatherData` (ticket
    /// 26/09/06/ENERGY-PLUGIN-END-TO-END).
    #[state(artifact)]
    #[link_slot(roles("weather"))]
    pub weather_link: Option<store::ArtifactLink>,
}

impl Default for EnergyModelSnapshot {
    fn default() -> Self {
        energy_snapshot_with_state(ENERGY_MODEL_DOCUMENT_SCHEMA, &crate::model::Model::default(), None)
    }
}

// 🌱️ Hand-written, not derived — `structure`/`zones` are `store::os_store::artifact_child::ArtifactChild<S>` and
// `referenced_model` is `Option<store::ArtifactLink>`, neither of which has a
// `#[derive(ToValue, FromValue)]`-reachable impl (fan-out playbook trap #3; `ArtifactLink` mirrors
// the same framework-exempt shape). `model: crate::model::Model` goes through `ToValue`/`FromValue`
// directly — `Model` now derives both.
impl ToValue for EnergyModelSnapshot {
    fn to_value(&self) -> DslValue {
        DslValue::object([
            ("schema".to_string(), self.schema.to_value()),
            ("model".to_string(), self.model.to_value()),
            ("structure".to_string(), ToValue::to_value(&self.structure)),
            ("zones".to_string(), ToValue::to_value(&self.zones)),
            ("referencedModel".to_string(), ToValue::to_value(&self.referenced_model)),
            ("weatherLink".to_string(), ToValue::to_value(&self.weather_link)),
        ])
    }
}
impl FromValue for EnergyModelSnapshot {
    fn from_value(value: DslValue) -> Result<Self, ValueError> {
        let entries = DslValue::into_object(value)?;
        let field = |key: &str| entries.iter().find(|(k, _)| k == key).map_or(DslValue::Null, |(_, v)| v.clone());
        Ok(Self {
            schema: String::from_value(field("schema"))?,
            model: crate::model::Model::from_value(field("model"))?,
            structure: FromValue::from_value(field("structure"))?,
            zones: FromValue::from_value(field("zones"))?,
            referenced_model: FromValue::from_value(field("referencedModel"))?,
            weather_link: FromValue::from_value(field("weatherLink"))?,
        })
    }
}
//#endregion 🔖️Snapshot

//#region 🔖️PackRecord







//#endregion 🔖️PackRecord

//#region 🔖️HandcraftedArtifactCodecs



//#endregion 🔖️HandcraftedArtifactCodecs

//#region 🧪️Tests

//#endregion 🧪️Tests

//#region 🌉️IdentityBridge

//#endregion 🌉️IdentityBridge

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



//#region 🔖️RetainedArtifactState
/// 🧵️ Exact immutable store owner used by mounted Energy capture. It exposes only a borrowed Model,
/// revalidates the store generation immediately before every read, and returns the precise snapshot
/// lease witness to the store retirement pump on close.
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
pub fn energy_child_restore_projection(snapshot: &EnergyModelSnapshot) -> Result<semio_framework_composition_admission::ChildRestoreProjection<'_>, semio_framework_diagnostic::Fault> {
    semio_framework_composition_admission::ChildRestoreProjection::from_snapshot(snapshot).map_err(|error| semio_framework_diagnostic::Fault::from(format!("energy model child projection failed: {error}")))
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
//#endregion ♻️WholeDocumentLoad


//#region 🔖️ArtifactKind
/// 🗂️ This artifact's `ArtifactKindSpec` — Data × Value per owner-table (`data.model`).
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
//#endregion 🔖️ArtifactKind

//#region 🧬️Facet
/// 🧬️ The snapshot facet's five handcrafted language leaves, owned here beside the snapshot they describe; the
/// artifact schema descriptor (`semio_s_artifact_energy_model_schema`) assembles them.
pub const FACET: semio_framework_schema_registry::FacetLeaves = semio_framework_schema_registry::FacetLeaves {
    rust: include_str!("../🦀️.rs"),
    typescript: include_str!("../🟦️.ts"),
    graphql: include_str!("../🔗️.graphql"),
    json_schema: include_str!("../🔣️.json"),
    proto: include_str!("../🛰️.proto"),
};
//#endregion 🧬️Facet

//#region 🚪️Representations
#[path = "../../../🚪️io/📝️text/📸️snapshot/🦀️.rs"]
pub mod text;
#[path = "../../../🚪️io/💾️binary/📸️snapshot/🦀️.rs"]
pub mod binary;
#[path = "../../../🚪️io/🪶️sqlite/📸️snapshot/🦀️.rs"]
pub mod sqlite;
//#endregion 🚪️Representations
