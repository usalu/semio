//! 🔋️ Energy model editor — the authored `ArtifactEditor` surface for `s.energy.model@1/*`
//! (tickets 26/08/16/ARTIFACT-VIEWERS-AND-EDITORS-PER-SUBSET, 26/09/06/ENERGY-PLUGIN-END-TO-END).
//! Three windows: `structure` (framework `TreeWindowKit` over the whole `crate::model::Model`),
//! `zones` (framework `TableWindowKit` over `Model::zones`) and `simulation` (run settings plus the
//! framework-reported state of the `energySimulation` tool run, whose job is `🧵️simulation-session`).
//!
//! 🧵️ Dispatch: every action this editor declares is `InteractiveJobClassification::Migrated` AND
//! carries an exact app-owned bounded-first-step proof. Both halves are load-bearing —
//! `VcsArtifactApp::dispatch_action` runs `require_complete_tool_operation_pipeline`, which refuses
//! anything but a `QualifiedToolProof::AppOwned` with `interactive-job.missing-owned-reducer`, and
//! `AppActionRegistry::validate_tool_job_rows` demands set EQUALITY between
//! `<EnergyModelEditorCommand as protocol::OpBinary>::TOOL_JOB_IDS ∩ migrated-action-ids` and the
//! `bounded_first_step_tool_proofs!` rows (else `interactive-job.catalog-incomplete`). The two
//! framework-kit actions `set-node`/`set-cell` are stamped `Migrated` by `window_kind_definition`
//! itself, so they belong to that set too and appear in [`ENERGY_MODEL_RETAINED_TOOL_IDS`] with
//! everything else.
//!
//! 📬️ Publication lanes: the twelve document verbs declare `ArtifactToolPublicationLane::Artifact`
//! (`setActiveExample` does NOT — it publishes a `kernel::Effect::LoadDocument`, never a mutation)
//! and are dispatchable only because [`EnergyModelEditor::build_artifact_store_one_item_preparation_factory`]
//! supplies the document lane's one-item retained preparation; `set-simulation-settings` publishes to the
//! config lane through [`EnergyModelEditor::build_config_store_one_item_preparation_factory`].
//!
//! ⏯️ The energy simulation is the `energySimulation` tool's framework `ToolRun`
//! (`📋️tool-run-contract.md`): the framework-reserved `toolRun*` actions and chords start, pause, step,
//! abort and finalize it, and [`EnergyModelEditor::build_tool_run_job`] supplies its run job.

use crate::editor::model::modes::edit;
use crate::editor::model::config::{ChangeResultField, ChangeSimulationSettings, EnergyModelConfig, EnergyModelConfigMutation};
use crate::editor::model::modes::edit::tools;
use crate::editor::model::interaction::{EnergyModelInteractionSnapshot, ENERGY_MODEL_INTERACTION_DOMAIN};
use crate::editor::model::panels::artifact as artifact_panel;
use crate::editor::model::panels::inspection as inspection_panel;
use crate::editor::model::modes::edit::windows::{model as model_window, simulation, structure, zones};
use crate::energy_simulation_session::EnergySimulationRunJob;
use crate::model::{EntityId, Material, OutsideBoundary, ScheduleId, Site, Surface, SurfaceClass, Thermostat, Zone};
use crate::mutations;
use crate::{EnergyModelMutation, EnergyModelSnapshot, ENERGY_MODEL_DOCUMENT_SCHEMA, MODEL_DIALECT};
use semio_framework::kernel::UiDirtyScope;
use semio_framework_plugin::retained_command::{ArtifactRetainedCommandJob, ArtifactRetainedCommandPayload, BoundedArtifactCommandWork};
use semio_framework_plugin::{
    AppOperationContext, ArtifactEditor, ArtifactOwnedToolJobRequest, ArtifactToolFactoryRegistry, ArtifactToolPublicationContract, ArtifactToolPublicationLane, ArtifactView, ComponentTree, ConfigView, Dialect, DraftView, Editor, EditorApp, Emit,
    ExampleSource, Fault, FaultCode, FaultOrigin, HistoryView, InteractiveJobClassification, Label, LocalizedLabel, NoDraft, NoDraftMutation, NoPresence, NoPresenceMutation, NoTransient, NoTransientMutation, ToolRunJob, ToolRunJobPurpose,
    ToolRunJobRequest, UiAssemblyResult,
};
use semio_framework_plugin::{ToolExecutionContract, ToolFactoryKey, ToolJobFactoryError};
use semio_framework_value_derive::{FromValue as FromValueDerive, ToValue as ToValueDerive};
use store::EngineHandles;

//#region 🏷️ActionIds
/// 🏷️ Every dispatchable verb of this editor, exactly once. `set-node`/`set-cell` come from the two
/// framework window kits; the rest are authored here. The DSL wire key of each command variant is
/// deliberately IDENTICAL to its manifest action id, so `command_id`, the proof rows and the
/// classification list can never drift apart by a rename.
pub const SET_NODE_ACTION_ID: &str = "set-node";
pub const SET_CELL_ACTION_ID: &str = "set-cell";
pub const CREATE_ZONE_ACTION_ID: &str = "create-zone";
pub const RENAME_ZONE_ACTION_ID: &str = "rename-zone";
pub const DELETE_ZONE_ACTION_ID: &str = "delete-zone";
pub const CREATE_SURFACE_ACTION_ID: &str = "create-surface";
pub const DELETE_SURFACE_ACTION_ID: &str = "delete-surface";
pub const ASSIGN_SURFACE_CONSTRUCTION_ACTION_ID: &str = "assign-surface-construction";
pub const SET_MATERIAL_PROPERTY_ACTION_ID: &str = "set-material-property";
/// 🔍️ The three generic inspector verbs, one per addressable entity family: `{<entity>, property,
/// value}` with `value` carried as TEXT, because one verb has to cover a name, an enum spelling, a
/// flag and a scalar alike — the shape a rendered control's own value arrives in.
pub const SET_SURFACE_PROPERTY_ACTION_ID: &str = "set-surface-property";
pub const SET_FENESTRATION_PROPERTY_ACTION_ID: &str = "set-fenestration-property";
pub const SET_ZONE_PROPERTY_ACTION_ID: &str = "set-zone-property";
pub const SET_GLAZING_MATERIAL_PROPERTY_ACTION_ID: &str = "set-glazing-material-property";
pub const SET_GAS_MATERIAL_PROPERTY_ACTION_ID: &str = "set-gas-material-property";
/// 🧱️ The construction verb: `{construction, property, value}` where `property` is either `name` or
/// one of the LIST edits `addLayer`/`removeLayer`/`moveLayerUp`/`moveLayerDown`/`replaceLayer`, whose
/// operand travels in `value` (a material id, or an index, or `<index>:<materialId>`). One verb,
/// because the inspector's layer controls all carry a single merged `value`.
pub const SET_CONSTRUCTION_PROPERTY_ACTION_ID: &str = "set-construction-property";
pub const SET_THERMOSTAT_SETPOINTS_ACTION_ID: &str = "set-thermostat-setpoints";
pub const SET_SITE_ACTION_ID: &str = "set-site";
pub const SET_RUN_PERIOD_ACTION_ID: &str = "set-run-period";
/// 📚️ Framework-fixed id: the react shell's example picker and the wgpu shell both dispatch exactly
/// `setActiveExample` with `{ exampleId }` (`🏛️ShellHost/🟦️.tsx` `dispatchActiveExample`,
/// `🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs:6481`). It is camelCase, unlike this editor's own kebab verbs,
/// because it is a shell contract rather than an authored name.
pub const SET_ACTIVE_EXAMPLE_ACTION_ID: &str = "setActiveExample";

/// 🧵️ The complete retained-tool roster. Order is the declaration order of
/// [`EnergyModelEditorCommand`] and is asserted equal to it, to the manifest's classification list
/// and to the proof rows by `retained_roster_is_exact_and_exhaustive`.
pub const ENERGY_MODEL_RETAINED_TOOL_IDS: &[&str] = &[
    SET_NODE_ACTION_ID,
    SET_CELL_ACTION_ID,
    CREATE_ZONE_ACTION_ID,
    RENAME_ZONE_ACTION_ID,
    DELETE_ZONE_ACTION_ID,
    CREATE_SURFACE_ACTION_ID,
    DELETE_SURFACE_ACTION_ID,
    ASSIGN_SURFACE_CONSTRUCTION_ACTION_ID,
    SET_MATERIAL_PROPERTY_ACTION_ID,
    SET_THERMOSTAT_SETPOINTS_ACTION_ID,
    SET_SITE_ACTION_ID,
    SET_RUN_PERIOD_ACTION_ID,
    SET_ACTIVE_EXAMPLE_ACTION_ID,
    simulation::SET_SETTINGS_ACTION_ID,
    simulation::SET_RESULT_FIELD_ACTION_ID,
    SET_SURFACE_PROPERTY_ACTION_ID,
    SET_FENESTRATION_PROPERTY_ACTION_ID,
    SET_ZONE_PROPERTY_ACTION_ID,
    SET_GLAZING_MATERIAL_PROPERTY_ACTION_ID,
    SET_GAS_MATERIAL_PROPERTY_ACTION_ID,
    // 🎥️ Declared last, matching the command enum's own declaration order: the 3d window's
    // orbit pose. Retained like every other verb of this editor — `Migrated` is the only
    // UI-dispatchable classification, and a `Migrated` verb without an owned reducer is refused
    // with `interactive-job.missing-owned-reducer`.
    model_window::SET_CAMERA_ACTION_ID,
    SET_CONSTRUCTION_PROPERTY_ACTION_ID,
];

/// ⚠️ The verbs that discard user content no later verb reconstructs — the two inspector deletes and
/// the whole-document example swap. `AppBuilder::action_destructive` reads this roster in
/// [`create_energy_model_editor`], which is what raises `ApprovalMode::WhenDestructive` so the MCP
/// gateway asks a human before an agent commits one.
pub const ENERGY_MODEL_DESTRUCTIVE_ACTION_IDS: &[&str] = &[DELETE_ZONE_ACTION_ID, DELETE_SURFACE_ACTION_ID, SET_ACTIVE_EXAMPLE_ACTION_ID];

/// 📬️ The twelve verbs that publish a semantic mutation into the document store. `setActiveExample`
/// is deliberately NOT one of them — it swaps the whole document through `kernel::Effect::LoadDocument`
/// (outside history), so it publishes to no store lane and declares `HostOnly`.
pub const ENERGY_MODEL_DOCUMENT_TOOL_IDS: &[&str] = &[
    SET_NODE_ACTION_ID,
    SET_CELL_ACTION_ID,
    CREATE_ZONE_ACTION_ID,
    RENAME_ZONE_ACTION_ID,
    DELETE_ZONE_ACTION_ID,
    CREATE_SURFACE_ACTION_ID,
    DELETE_SURFACE_ACTION_ID,
    ASSIGN_SURFACE_CONSTRUCTION_ACTION_ID,
    SET_MATERIAL_PROPERTY_ACTION_ID,
    SET_THERMOSTAT_SETPOINTS_ACTION_ID,
    SET_SITE_ACTION_ID,
    SET_RUN_PERIOD_ACTION_ID,
    SET_SURFACE_PROPERTY_ACTION_ID,
    SET_FENESTRATION_PROPERTY_ACTION_ID,
    SET_ZONE_PROPERTY_ACTION_ID,
    SET_GLAZING_MATERIAL_PROPERTY_ACTION_ID,
    SET_GAS_MATERIAL_PROPERTY_ACTION_ID,
    SET_CONSTRUCTION_PROPERTY_ACTION_ID,
];
//#endregion 🏷️ActionIds

//#region 🔖️Command
/// ✏️ The editor's typed command channel. `SetStructureField`/`SetZoneCell` are the two generic
/// window-kit edit targets; the ten authored document verbs address `crate::model::Model` entities
/// by their own `EntityId`; `SetSimulationSettings` edits the config store's run settings.
#[derive(Clone, Debug, PartialEq, ToValueDerive, FromValueDerive, dsl::DslOps)]
pub enum EnergyModelEditorCommand {
    #[dsl(key = "set-node")]
    SetStructureField { field: String, value: String },
    #[dsl(key = "set-cell")]
    SetZoneCell { row: u32, column: String, value: String },
    #[dsl(key = "create-zone")]
    CreateZone { name: String, volume_m3: f64, multiplier: u32, conditioned: bool },
    #[dsl(key = "rename-zone")]
    RenameZone { zone: u32, new_name: String },
    #[dsl(key = "delete-zone")]
    DeleteZone { zone: u32 },
    #[dsl(key = "create-surface")]
    CreateSurface { name: String, zone: u32, construction: u32, class: String },
    #[dsl(key = "delete-surface")]
    DeleteSurface { surface: u32 },
    #[dsl(key = "assign-surface-construction")]
    AssignSurfaceConstruction { surface: u32, construction: u32 },
    /// 🧱️ One material field by name. `value` is TEXT, like every other inspector verb: a material
    /// carries a NAME and a ROUGHNESS enum beside its seven SI scalars, and one verb has to spell all
    /// three. Numeric properties parse the text and refuse a non-finite or out-of-range reading.
    #[dsl(key = "set-material-property")]
    SetMaterialProperty { material: u32, property: String, value: String },
    #[dsl(key = "set-thermostat-setpoints")]
    SetThermostatSetpoints { thermostat: u32, heating_schedule: u32, cooling_schedule: u32, heating_throttle_range_k: f64, cooling_throttle_range_k: f64 },
    #[dsl(key = "set-site")]
    SetSite { latitude_deg: f64, longitude_deg: f64, elevation_m: f64, time_zone_hours: f64, north_axis_deg: f64 },
    #[dsl(key = "set-run-period")]
    SetRunPeriod { start_month: u32, start_day: u32, end_month: u32, end_day: u32 },
    #[dsl(key = "setActiveExample")]
    SetActiveExample { example_id: String },
    #[dsl(key = "set-simulation-settings")]
    SetSimulationSettings { zone_timestep_minutes: u32, system_timestep_minutes: u32, warmup_days: u32 },
    /// 🎨️ Which published per-surface field the 3d model window colours by — a config-store verb, like
    /// `set-simulation-settings`, but one the simulation run does not read, so it never restarts a run.
    #[dsl(key = "set-result-field")]
    SetResultField { field: String },
    /// 🟫️ One surface field by name. `partner_surface` is the `Interzone` boundary's other half and
    /// is `0` (= none) for every other property and boundary kind — the union's two payload halves
    /// travel as two flat fields for the same reason `change-surface-boundary-condition` splits
    /// them: `dsl::DslScalar` binds unit variants only.
    #[dsl(key = "set-surface-property")]
    SetSurfaceProperty { surface: u32, property: String, value: String, partner_surface: u32 },
    /// 🪟️ One fenestration field by name — every scalar of the record plus its optional glazing
    /// construction (an empty `value` clears the binding).
    #[dsl(key = "set-fenestration-property")]
    SetFenestrationProperty { fenestration: u32, property: String, value: String },
    /// 🏘️ One zone field by name — the id-addressed twin of the `zones` table's positional
    /// `set-cell`, which the inspector cannot use because it addresses a ROW, not an entity.
    #[dsl(key = "set-zone-property")]
    SetZoneProperty { zone: u32, property: String, value: String },
    /// 🧊️ One glazing-material field by name. Only the five optical/thermal scalars this artifact's
    /// vocabulary names (`change-glazing-material-*`) plus the record's own name are addressable —
    /// the reflectance and infrared-transmittance fields still have no mutation kind and are refused
    /// rather than silently written.
    #[dsl(key = "set-glazing-material-property")]
    SetGlazingMaterialProperty { material: u32, property: String, value: String },
    /// 💨️ One gas-gap field by name: its thickness, its fill gas, or its name.
    #[dsl(key = "set-gas-material-property")]
    SetGasMaterialProperty { material: u32, property: String, value: String },
    /// 🎥️ The 3d window's orbit pose, as the canonical `{position,target,zoom,up?}` JSON the react
    /// `World3dHost` sends after every gesture. Carried as one text field because a `dsl::DslOps`
    /// variant binds scalars only — and because that string IS what `World3dScene::camera_json` wants.
    #[dsl(key = "setCamera")]
    SetCamera { camera: String },
    /// 🧱️ One construction field by name — its name, or one LIST edit of its layer stack. Appended
    /// LAST so no existing `OpBinary` ordinal moves.
    #[dsl(key = "set-construction-property")]
    SetConstructionProperty { construction: u32, property: String, value: String },
}

impl EnergyModelEditorCommand {
    /// 🏷️ The manifest action id this variant was declared under — identical to its DSL wire key.
    pub fn action_id(&self) -> &'static str {
        match self {
            Self::SetStructureField { .. } => SET_NODE_ACTION_ID,
            Self::SetZoneCell { .. } => SET_CELL_ACTION_ID,
            Self::CreateZone { .. } => CREATE_ZONE_ACTION_ID,
            Self::RenameZone { .. } => RENAME_ZONE_ACTION_ID,
            Self::DeleteZone { .. } => DELETE_ZONE_ACTION_ID,
            Self::CreateSurface { .. } => CREATE_SURFACE_ACTION_ID,
            Self::DeleteSurface { .. } => DELETE_SURFACE_ACTION_ID,
            Self::AssignSurfaceConstruction { .. } => ASSIGN_SURFACE_CONSTRUCTION_ACTION_ID,
            Self::SetMaterialProperty { .. } => SET_MATERIAL_PROPERTY_ACTION_ID,
            Self::SetThermostatSetpoints { .. } => SET_THERMOSTAT_SETPOINTS_ACTION_ID,
            Self::SetSite { .. } => SET_SITE_ACTION_ID,
            Self::SetRunPeriod { .. } => SET_RUN_PERIOD_ACTION_ID,
            Self::SetActiveExample { .. } => SET_ACTIVE_EXAMPLE_ACTION_ID,
            Self::SetSimulationSettings { .. } => simulation::SET_SETTINGS_ACTION_ID,
            Self::SetResultField { .. } => simulation::SET_RESULT_FIELD_ACTION_ID,
            Self::SetSurfaceProperty { .. } => SET_SURFACE_PROPERTY_ACTION_ID,
            Self::SetFenestrationProperty { .. } => SET_FENESTRATION_PROPERTY_ACTION_ID,
            Self::SetZoneProperty { .. } => SET_ZONE_PROPERTY_ACTION_ID,
            Self::SetGlazingMaterialProperty { .. } => SET_GLAZING_MATERIAL_PROPERTY_ACTION_ID,
            Self::SetGasMaterialProperty { .. } => SET_GAS_MATERIAL_PROPERTY_ACTION_ID,
            Self::SetCamera { .. } => model_window::SET_CAMERA_ACTION_ID,
            Self::SetConstructionProperty { .. } => SET_CONSTRUCTION_PROPERTY_ACTION_ID,
        }
    }
}

//#region 🌉️ActionBridge
/// 🌉️ Host action arguments (`DslValue`) → typed command. Numbers arriving as numeric strings (the
/// shape a `<select>`-sourced argument takes) are accepted for every numeric field, so the same
/// action works from the command palette, a keybinding and a rendered form alike.
mod args_bridge {
    use super::{EnergyModelEditorCommand as Command, Fault, FaultCode, FaultOrigin};

    fn field<'a>(args: Option<&'a dsl::DslValue>, key: &str) -> Option<&'a dsl::DslValue> {
        args?.get(key)
    }

    fn text(args: Option<&dsl::DslValue>, key: &str) -> Option<String> {
        let value = field(args, key)?;
        value.as_str().map(str::to_owned).or_else(|| value.as_u64().map(|number| number.to_string())).or_else(|| value.as_i64().map(|number| number.to_string())).or_else(|| value.as_f64().map(|number| number.to_string()))
    }

    fn number(args: Option<&dsl::DslValue>, key: &str) -> Option<f64> {
        let value = field(args, key)?;
        value.as_f64().or_else(|| value.as_str()?.parse().ok())
    }

    fn flag(args: Option<&dsl::DslValue>, key: &str) -> Option<bool> {
        let value = field(args, key)?;
        value.as_bool().or_else(|| value.as_str()?.parse().ok())
    }

    /// 🎥️ The `{position,target,zoom,up?}` object the react `World3dHost` sends under `camera`,
    /// validated as a real pose and canonicalized back to the exact JSON string
    /// `World3dScene::camera_json` consumes. A malformed or non-finite pose answers `None`, which the
    /// bridge turns into an explicit refusal rather than a silently ignored gesture.
    fn camera_pose_json(args: Option<&dsl::DslValue>) -> Option<String> {
        let value = field(args, "camera")?;
        let pose = <store::Viewport3dOrbit as dsl::FromValue>::from_value(value.clone()).ok()?;
        pose.validate().ok()?;
        Some(dsl::json::to_json_string(&dsl::ToValue::to_value(&pose)))
    }

    fn unknown(action: &str) -> Fault {
        Fault::new(FaultOrigin::App, FaultCode::new("app.command.unsupported"), format!("the energy model editor has no command for action '{action}'"))
    }

    pub fn command_from_action(action: &str, args: Option<&dsl::DslValue>) -> Result<Command, Fault> {
        let text_or = |key: &str, fallback: &str| text(args, key).unwrap_or_else(|| fallback.to_string());
        let f64_or = |key: &str, fallback: f64| number(args, key).unwrap_or(fallback);
        let u32_or = |key: &str, fallback: u32| number(args, key).map_or(fallback, |value| value as u32);
        let bool_or = |key: &str, fallback: bool| flag(args, key).unwrap_or(fallback);
        Ok(match action {
            // 🧱️ `nodeId` is the argument name `TreeWindowKit::editable_window_kind()` declares for
            // its own `set-node` row; `id` is what this editor's structure tree sent before the kit
            // named the argument at all, and both reach the same field.
            super::SET_NODE_ACTION_ID => Command::SetStructureField { field: text(args, "nodeId").unwrap_or_else(|| text_or("id", "")), value: text_or("value", "") },
            super::SET_CELL_ACTION_ID => Command::SetZoneCell { row: u32_or("row", 0), column: text_or("column", ""), value: text_or("value", "") },
            super::CREATE_ZONE_ACTION_ID => Command::CreateZone { name: text_or("name", "Zone"), volume_m3: f64_or("volumeM3", 100.0), multiplier: u32_or("multiplier", 1), conditioned: bool_or("conditioned", true) },
            super::RENAME_ZONE_ACTION_ID => Command::RenameZone { zone: u32_or("zone", 0), new_name: text_or("newName", "") },
            super::DELETE_ZONE_ACTION_ID => Command::DeleteZone { zone: u32_or("zone", 0) },
            super::CREATE_SURFACE_ACTION_ID => Command::CreateSurface { name: text_or("name", "Surface"), zone: u32_or("zone", 0), construction: u32_or("construction", 0), class: text_or("class", "exteriorWall") },
            super::DELETE_SURFACE_ACTION_ID => Command::DeleteSurface { surface: u32_or("surface", 0) },
            super::ASSIGN_SURFACE_CONSTRUCTION_ACTION_ID => Command::AssignSurfaceConstruction { surface: u32_or("surface", 0), construction: u32_or("construction", 0) },
            super::SET_MATERIAL_PROPERTY_ACTION_ID => {
                Command::SetMaterialProperty { material: u32_or("material", u32_or("id", 0)), property: text(args, "property").unwrap_or_else(|| text_or("field", "")), value: text_or("value", "") }
            }
            super::SET_CONSTRUCTION_PROPERTY_ACTION_ID => {
                Command::SetConstructionProperty { construction: u32_or("construction", u32_or("id", 0)), property: text(args, "property").unwrap_or_else(|| text_or("field", "")), value: text_or("value", "") }
            }
            // 🌡️ A WHOLE-RECORD verb reached from a PER-FIELD control: the inspector authors all five
            // slots at their current values plus `field`, naming the one the host then overwrites
            // under `value`. Without that indirection the merged `value` would be read by nobody and
            // the edited slot would silently fall back to the default below (the review's blocker:
            // every throttle-range edit wrote 2.0 and every site edit wrote 0.0). A palette or
            // keybinding invocation carries no `field`, so `edited` is `None` and every slot is read
            // by its own name exactly as before.
            super::SET_THERMOSTAT_SETPOINTS_ACTION_ID => {
                let edited = text(args, "field");
                let merged = |key: &str, fallback: f64| if edited.as_deref() == Some(key) { number(args, "value").unwrap_or(fallback) } else { f64_or(key, fallback) };
                Command::SetThermostatSetpoints {
                    thermostat: u32_or("thermostat", u32_or("id", 0)),
                    heating_schedule: merged("heatingSchedule", 0.0) as u32,
                    cooling_schedule: merged("coolingSchedule", 0.0) as u32,
                    heating_throttle_range_k: merged("heatingThrottleRangeK", 2.0),
                    cooling_throttle_range_k: merged("coolingThrottleRangeK", 2.0),
                }
            }
            // 📍️ Same `{field, value}` indirection as the thermostat above — the site is a singleton
            // record edited one scalar at a time.
            super::SET_SITE_ACTION_ID => {
                let edited = text(args, "field");
                let merged = |key: &str, fallback: f64| if edited.as_deref() == Some(key) { number(args, "value").unwrap_or(fallback) } else { f64_or(key, fallback) };
                Command::SetSite {
                    latitude_deg: merged("latitudeDeg", 0.0),
                    longitude_deg: merged("longitudeDeg", 0.0),
                    elevation_m: merged("elevationM", 0.0),
                    time_zone_hours: merged("timeZoneHours", 0.0),
                    north_axis_deg: merged("northAxisDeg", 0.0),
                }
            }
            super::SET_RUN_PERIOD_ACTION_ID => Command::SetRunPeriod { start_month: u32_or("startMonth", 1), start_day: u32_or("startDay", 1), end_month: u32_or("endMonth", 12), end_day: u32_or("endDay", 31) },
            super::SET_ACTIVE_EXAMPLE_ACTION_ID => Command::SetActiveExample { example_id: text_or("exampleId", "") },
            // 🔍️ The inspector's controls author only `{field, id}` — the host merges the control's
            // own current value under `value`, so `field` is read back as `property` here.
            super::SET_SURFACE_PROPERTY_ACTION_ID => Command::SetSurfaceProperty {
                surface: u32_or("surface", u32_or("id", 0)),
                property: text(args, "property").unwrap_or_else(|| text_or("field", "")),
                value: text_or("value", ""),
                partner_surface: u32_or("partnerSurface", 0),
            },
            super::SET_FENESTRATION_PROPERTY_ACTION_ID => {
                Command::SetFenestrationProperty { fenestration: u32_or("fenestration", u32_or("id", 0)), property: text(args, "property").unwrap_or_else(|| text_or("field", "")), value: text_or("value", "") }
            }
            super::SET_ZONE_PROPERTY_ACTION_ID => Command::SetZoneProperty { zone: u32_or("zone", u32_or("id", 0)), property: text(args, "property").unwrap_or_else(|| text_or("field", "")), value: text_or("value", "") },
            super::SET_GLAZING_MATERIAL_PROPERTY_ACTION_ID => {
                Command::SetGlazingMaterialProperty { material: u32_or("material", u32_or("id", 0)), property: text(args, "property").unwrap_or_else(|| text_or("field", "")), value: text_or("value", "") }
            }
            super::SET_GAS_MATERIAL_PROPERTY_ACTION_ID => {
                Command::SetGasMaterialProperty { material: u32_or("material", u32_or("id", 0)), property: text(args, "property").unwrap_or_else(|| text_or("field", "")), value: text_or("value", "") }
            }
            super::simulation::SET_SETTINGS_ACTION_ID => {
                let defaults = super::EnergyModelConfig::default();
                Command::SetSimulationSettings { zone_timestep_minutes: u32_or("zoneTimestepMinutes", defaults.zone_timestep_minutes), system_timestep_minutes: u32_or("systemTimestepMinutes", defaults.system_timestep_minutes), warmup_days: u32_or("warmupDays", defaults.warmup_days) }
            }
            // 🎨️ A select control merges its own scalar under `value`, so `field` is read from either
            // spelling — the same `{field, value}` convention the inspector's controls use above.
            super::simulation::SET_RESULT_FIELD_ACTION_ID => Command::SetResultField { field: text(args, "field").unwrap_or_else(|| text_or("value", super::EnergyModelConfig::default().result_field.as_str())) },
            // 🎥️ `command_from_action` must be TOTAL over the roster (`every_declared_action_is_
            // classified_and_resolves_to_a_command` calls it with no args at all), so an absent or
            // malformed pose becomes the empty string here and `camera_emit` refuses it there — the
            // one place that can also tell WHICH window the gesture addressed.
            super::model_window::SET_CAMERA_ACTION_ID => Command::SetCamera { camera: camera_pose_json(args).unwrap_or_default() },
            _ => return Err(unknown(action)),
        })
    }
}
//#endregion 🌉️ActionBridge

//#region 🔖️OpCodec
/// 🎯️ Handcrafted (P6: `#[derive(dsl::DslOps)]` emits `DslVariants` only — `OpText`/`OpBinary` are
/// handcrafted per artifact). Same shape as `📕️norm`'s `NormConfigMutation`/`🔱️trinity`'s
/// `TrinityJackCommand`.
impl protocol::OpText for EnergyModelEditorCommand {
    fn parse_op(line: &str) -> Result<Self, store::TextError> {
        dsl::variants_text::parse_op(line)
    }
    fn print_op(&self) -> String {
        dsl::variants_text::print_op(self)
    }
}

impl protocol::OpBinary for EnergyModelEditorCommand {
    /// 🧵️ The exact manifest ids this typed command schema owns — the left-hand side of
    /// `validate_tool_job_rows`' `expected = TOOL_JOB_IDS ∩ migrated` set equality. Omitting it
    /// silently falls back to the trait default `["typed-command"]`, which makes `expected` empty
    /// and rejects every proof row this editor declares.
    const TOOL_JOB_IDS: &'static [&'static str] = ENERGY_MODEL_RETAINED_TOOL_IDS;

    fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        const OP_BINARY_FORMAT: u8 = 1;
        let (keyword, record) = <Self as dsl::DslVariants>::to_named_record(self);
        let variants = <Self as dsl::DslVariants>::variants();
        let ordinal = variants.iter().position(|(k, _)| *k == keyword).ok_or(protocol::ProtocolError::Malformed { what: "op variant", offset: 0, detail: format!("keyword {keyword:?} is not a declared variant") })?;
        let spec = (variants[ordinal].1)();
        let body = store::pack_rt::encode_record_body(&spec, &record, &store::PackEncodeOptions::default()).map_err(protocol::ProtocolError::from)?;
        let mut out = Vec::with_capacity(body.len() + 3);
        out.push(OP_BINARY_FORMAT);
        store::pack_rt::write_varint_u64(&mut out, ordinal as u64);
        out.extend_from_slice(&body);
        Ok(out)
    }
    fn decode_op(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        const OP_BINARY_FORMAT: u8 = 1;
        let mut reader = store::pack_rt::ByteReader::new(bytes);
        let format = reader.read_u8()?;
        if format != OP_BINARY_FORMAT {
            return Err(protocol::ProtocolError::Malformed { what: "op format", offset: 0, detail: format!("unsupported op format {format}") });
        }
        let ordinal = reader.read_varint_u64()?;
        let variants = <Self as dsl::DslVariants>::variants();
        let (keyword, spec_fn) = variants.get(ordinal as usize).ok_or(protocol::ProtocolError::Malformed { what: "op variant", offset: 1, detail: format!("ordinal {ordinal} out of range for {} declared variants", variants.len()) })?;
        let spec = spec_fn();
        let body = &bytes[reader.position()..];
        let (record, _report) = store::pack_rt::decode_record_body(body, &spec, &store::PackDecodeOptions::default()).map_err(protocol::ProtocolError::from)?;
        <Self as dsl::DslVariants>::from_named_record(keyword, &record).map_err(|error| protocol::ProtocolError::Malformed { what: "op record", offset: reader.position() as u64, detail: error.to_string() })
    }
}
//#endregion 🔖️OpCodec
//#endregion 🔖️Command

//#region 🧬️MutationSeam
/// 🧬️ THE single seam between an editor command and this artifact's semantic mutation vocabulary.
/// `kind` is the ledger name from the ticket's `📓️mutation-tag-ledger.md`, kept for the description
/// and for the fault a not-yet-landed group raises. The seam translates the edited `Model` into the
/// granular semantic steps that produced it — there is no whole-document replace to fall back on
/// (`📓️derivation-rules.md` rule 6). Every field this vocabulary does not yet name is caught by the
/// exhaustive `probe` comparison below and refused LOUDLY, so a group's missing kind can never be
/// swallowed as a silent no-op.
fn model_edit(kind: &'static str, base: &crate::model::Model, model: &crate::model::Model, description: String) -> Result<Emit<EnergyModelMutation, EnergyModelConfigMutation>, Fault> {
    let mut steps = Vec::new();
    if base.name != model.name {
        steps.push(mutations::rename_model(model.name.clone()));
    }
    if base.version != model.version {
        steps.push(mutations::change_model_version(model.version.clone()));
    }
    if base.site != model.site {
        steps.push(mutations::update_site(model.site.latitude_deg, model.site.longitude_deg, model.site.elevation_m, model.site.time_zone_hours, model.site.north_axis_deg));
    }
    if base.run_period != model.run_period {
        steps.push(mutations::update_run_period(model.run_period.start_month, model.run_period.start_day, model.run_period.end_month, model.run_period.end_day, model.run_period.year));
    }
    if base.ground_temperature != model.ground_temperature {
        steps.push(mutations::update_ground_temperature(model.ground_temperature.building_surface_c.to_vec(), model.ground_temperature.shallow_c.to_vec(), model.ground_temperature.deep_c));
    }
    if base.airflow_network != model.airflow_network {
        steps.push(match &model.airflow_network {
            Some(network) => mutations::replace_airflow_network(true, network.zone_node_ids.iter().map(|(zone, _)| zone.0).collect(), network.zone_node_ids.iter().map(|(_, node)| *node).collect(), network.outdoor_node_id, network.link_ids.clone()),
            None => mutations::replace_airflow_network(false, Vec::new(), Vec::new(), 0, Vec::new()),
        });
    }
    for was in &base.output_variables {
        if !model.output_variables.iter().any(|now| now.name == was.name && now.key == was.key) {
            steps.push(mutations::remove_output_variable(was.name.clone(), was.key.clone()));
        }
    }
    for now in &model.output_variables {
        if !base.output_variables.iter().any(|was| was.name == now.name && was.key == now.key) {
            steps.push(mutations::add_output_variable(now.name.clone(), now.key.clone(), now.reporting_frequency));
        }
    }
    let projected_zones = diff_zones(base, model, &mut steps);
    let projected_surfaces = diff_surfaces(base, model, &mut steps);
    let projected_fenestrations = diff_fenestrations(base, model, &mut steps);
    let projected_materials = diff_materials(kind, base, model, &mut steps)?;
    let projected_glazing = diff_glazing_materials(kind, base, model, &mut steps)?;
    let projected_gases = diff_gas_materials(kind, base, model, &mut steps)?;
    let projected_constructions = diff_constructions(kind, base, model, &mut steps)?;
    let projected_thermostats = diff_thermostats(kind, base, model, &mut steps)?;
    let mut probe = base.clone();
    probe.name = model.name.clone();
    probe.version = model.version.clone();
    probe.site = model.site;
    probe.run_period = model.run_period;
    probe.ground_temperature = model.ground_temperature.clone();
    probe.airflow_network = model.airflow_network.clone();
    probe.output_variables = model.output_variables.clone();
    // 🔬️ Projections, never clones: every collection below reports back exactly the fields its diff
    // emitted a step for, so an undiffed field makes `probe != *model` and faults LOUDLY through
    // `kind_unavailable` instead of being swallowed as a silent no-op.
    probe.zones = projected_zones;
    probe.surfaces = projected_surfaces;
    // 🪟️ NOT `model.fenestrations.clone()`: the projection [`diff_fenestrations`] returns carries
    // exactly the fields it emitted a mutation for, so a Fenestration field no diff step names makes
    // `probe != *model` and faults LOUDLY through `kind_unavailable` instead of vanishing.
    probe.fenestrations = projected_fenestrations;
    probe.adjacency_pairs = model.adjacency_pairs.clone();
    probe.materials = projected_materials;
    probe.glazing_materials = projected_glazing;
    probe.gas_materials = projected_gases;
    probe.constructions = projected_constructions;
    probe.thermostats = projected_thermostats;
    if probe != *model {
        return Err(kind_unavailable(kind, kind));
    }
    Ok(Emit { artifact_mutations: steps, description: Some(description), ..Default::default() })
}

/// 🏘️ Zones: a create/delete of the whole row plus the five per-field kinds. `create-zone`/
/// `delete-zone` landed with the 100s group, so an identity change is no longer a refusal.
fn diff_zones(base: &crate::model::Model, model: &crate::model::Model, steps: &mut Vec<EnergyModelMutation>) -> Vec<Zone> {
    for was in &base.zones {
        if !model.zones.iter().any(|now| now.id == was.id) {
            steps.push(mutations::delete_zone(was.id));
        }
    }
    let mut projected = Vec::with_capacity(model.zones.len());
    for now in &model.zones {
        let Some(was) = base.zones.iter().find(|was| was.id == now.id) else {
            steps.push(mutations::create_zone(now.id, now.name.clone(), now.volume_m3, now.multiplier, now.conditioned, now.part_of_total_floor_area));
            projected.push(now.clone());
            continue;
        };
        let mut carried = was.clone();
        if was.name != now.name {
            steps.push(mutations::rename_zone(now.id, now.name.clone()));
            carried.name = now.name.clone();
        }
        if was.volume_m3 != now.volume_m3 {
            steps.push(mutations::change_zone_volume(now.id, now.volume_m3));
            carried.volume_m3 = now.volume_m3;
        }
        if was.multiplier != now.multiplier {
            steps.push(mutations::change_zone_multiplier(now.id, now.multiplier));
            carried.multiplier = now.multiplier;
        }
        if was.conditioned != now.conditioned {
            steps.push(mutations::change_zone_conditioned(now.id, now.conditioned));
            carried.conditioned = now.conditioned;
        }
        if was.part_of_total_floor_area != now.part_of_total_floor_area {
            steps.push(mutations::change_zone_floor_area_participation(now.id, now.part_of_total_floor_area));
            carried.part_of_total_floor_area = now.part_of_total_floor_area;
        }
        projected.push(carried);
    }
    projected
}

/// 🚧️ The interzone partner an `OutsideBoundary` carries — the boundary mutation names the tag
/// through `OutsideBoundaryKind` and the partner through this separate optional id, because
/// `dsl::DslScalar` binds unit variants only.
fn interzone_partner(boundary: OutsideBoundary) -> Option<EntityId> {
    match boundary {
        OutsideBoundary::Interzone(partner) => Some(partner),
        _ => None,
    }
}

/// 🟫️ Surfaces and the two collections a surface delete cascades into. Order is load-bearing:
/// every dependent (fenestration, adjacency pair) is disconnected BEFORE its surface disappears,
/// and every create runs after every delete, so no intermediate document ever dangles a reference.
fn diff_surfaces(base: &crate::model::Model, model: &crate::model::Model, steps: &mut Vec<EnergyModelMutation>) -> Vec<Surface> {
    for was in &base.fenestrations {
        if !model.fenestrations.iter().any(|now| now.id == was.id) {
            steps.push(mutations::delete_fenestration(was.id));
        }
    }
    for was in &base.adjacency_pairs {
        if !model.adjacency_pairs.iter().any(|now| now.surface_a_id == was.surface_a_id && now.surface_b_id == was.surface_b_id) {
            steps.push(mutations::disconnect_surfaces(was.surface_a_id, was.surface_b_id));
        }
    }
    for was in &base.surfaces {
        if !model.surfaces.iter().any(|now| now.id == was.id) {
            steps.push(mutations::delete_surface(was.id));
        }
    }
    let mut projected = Vec::with_capacity(model.surfaces.len());
    for now in &model.surfaces {
        let Some(was) = base.surfaces.iter().find(|was| was.id == now.id) else {
            steps.push(mutations::create_surface(
                now.id,
                now.name.clone(),
                now.zone_id,
                now.class,
                now.vertices_m.clone(),
                now.construction_id,
                now.outside_boundary_condition.kind(),
                interzone_partner(now.outside_boundary_condition),
                now.sun_exposed,
                now.wind_exposed,
                now.multiplier,
            ));
            projected.push(now.clone());
            continue;
        };
        let mut carried = was.clone();
        if was.name != now.name {
            steps.push(mutations::rename_surface(now.id, now.name.clone()));
            carried.name = now.name.clone();
        }
        if was.zone_id != now.zone_id {
            steps.push(mutations::change_surface_zone(now.id, now.zone_id));
            carried.zone_id = now.zone_id;
        }
        if was.class != now.class {
            steps.push(mutations::change_surface_class(now.id, now.class));
            carried.class = now.class;
        }
        if was.vertices_m != now.vertices_m {
            steps.push(mutations::replace_surface_vertices(now.id, now.vertices_m.clone()));
            carried.vertices_m = now.vertices_m.clone();
        }
        if was.construction_id != now.construction_id {
            steps.push(mutations::change_surface_construction(now.id, now.construction_id));
            carried.construction_id = now.construction_id;
        }
        if was.outside_boundary_condition != now.outside_boundary_condition {
            steps.push(mutations::change_surface_boundary_condition(now.id, now.outside_boundary_condition.kind(), interzone_partner(now.outside_boundary_condition)));
            carried.outside_boundary_condition = now.outside_boundary_condition;
        }
        if was.sun_exposed != now.sun_exposed {
            steps.push(mutations::change_surface_sun_exposed(now.id, now.sun_exposed));
            carried.sun_exposed = now.sun_exposed;
        }
        if was.wind_exposed != now.wind_exposed {
            steps.push(mutations::change_surface_wind_exposed(now.id, now.wind_exposed));
            carried.wind_exposed = now.wind_exposed;
        }
        if was.multiplier != now.multiplier {
            steps.push(mutations::change_surface_multiplier(now.id, now.multiplier));
            carried.multiplier = now.multiplier;
        }
        projected.push(carried);
    }
    projected
}

/// 🪟️ Fenestrations: every scalar of the record, plus the optional glazing-construction binding.
/// Deletion is already cascaded by [`diff_surfaces`] (a window is disconnected before its host
/// surface disappears), so this function only creates and diffs.
///
/// 🔬️ It returns the PROJECTION of `base`'s fenestrations through exactly the steps it emitted —
/// `model_edit`'s probe uses that instead of `model.fenestrations.clone()`, so a field this function
/// forgets to name is caught by the probe comparison and refused loudly rather than silently
/// dropped. That masking is the bug this ticket's exploration found: before this step a window
/// u-value edit reduced cleanly and emitted NO mutation at all.
fn diff_fenestrations(base: &crate::model::Model, model: &crate::model::Model, steps: &mut Vec<EnergyModelMutation>) -> Vec<crate::model::Fenestration> {
    let mut projected = Vec::with_capacity(model.fenestrations.len());
    for now in &model.fenestrations {
        let Some(was) = base.fenestrations.iter().find(|was| was.id == now.id) else {
            steps.push(mutations::create_fenestration(
                now.id,
                now.name.clone(),
                now.surface_id,
                now.u_value_w_m2k,
                now.shgc,
                now.vlt,
                now.area_m2,
                now.height_m,
                now.sill_height_m,
                now.frame_conductance_w_k,
                now.divider_conductance_w_k,
                now.overhang_depth_m,
                now.overhang_offset_m,
                now.fin_depth_m,
                now.fin_offset_m,
                now.glazing_construction_id,
            ));
            // 🔶️ `create-fenestration` carries no polygon — a created window starts rectangular and
            // its real corners, when it has any, follow as their own `replace-fenestration-vertices`.
            let mut created = now.clone();
            created.vertices_m = Vec::new();
            if !now.vertices_m.is_empty() {
                steps.push(mutations::replace_fenestration_vertices(now.id, now.vertices_m.clone()));
                created.vertices_m = now.vertices_m.clone();
            }
            projected.push(created);
            continue;
        };
        let mut carried = was.clone();
        if was.name != now.name {
            steps.push(mutations::rename_fenestration(now.id, now.name.clone()));
            carried.name = now.name.clone();
        }
        if was.surface_id != now.surface_id {
            steps.push(mutations::change_fenestration_surface(now.id, now.surface_id));
            carried.surface_id = now.surface_id;
        }
        if was.u_value_w_m2k != now.u_value_w_m2k {
            steps.push(mutations::change_fenestration_u_value(now.id, now.u_value_w_m2k));
            carried.u_value_w_m2k = now.u_value_w_m2k;
        }
        if was.shgc != now.shgc {
            steps.push(mutations::change_fenestration_shgc(now.id, now.shgc));
            carried.shgc = now.shgc;
        }
        if was.vlt != now.vlt {
            steps.push(mutations::change_fenestration_vlt(now.id, now.vlt));
            carried.vlt = now.vlt;
        }
        if was.area_m2 != now.area_m2 {
            steps.push(mutations::change_fenestration_area(now.id, now.area_m2));
            carried.area_m2 = now.area_m2;
        }
        if was.height_m != now.height_m {
            steps.push(mutations::change_fenestration_height(now.id, now.height_m));
            carried.height_m = now.height_m;
        }
        if was.sill_height_m != now.sill_height_m {
            steps.push(mutations::change_fenestration_sill_height(now.id, now.sill_height_m));
            carried.sill_height_m = now.sill_height_m;
        }
        if was.frame_conductance_w_k != now.frame_conductance_w_k {
            steps.push(mutations::change_fenestration_frame_conductance(now.id, now.frame_conductance_w_k));
            carried.frame_conductance_w_k = now.frame_conductance_w_k;
        }
        if was.divider_conductance_w_k != now.divider_conductance_w_k {
            steps.push(mutations::change_fenestration_divider_conductance(now.id, now.divider_conductance_w_k));
            carried.divider_conductance_w_k = now.divider_conductance_w_k;
        }
        if was.overhang_depth_m != now.overhang_depth_m {
            steps.push(mutations::change_fenestration_overhang_depth(now.id, now.overhang_depth_m));
            carried.overhang_depth_m = now.overhang_depth_m;
        }
        if was.overhang_offset_m != now.overhang_offset_m {
            steps.push(mutations::change_fenestration_overhang_offset(now.id, now.overhang_offset_m));
            carried.overhang_offset_m = now.overhang_offset_m;
        }
        if was.fin_depth_m != now.fin_depth_m {
            steps.push(mutations::change_fenestration_fin_depth(now.id, now.fin_depth_m));
            carried.fin_depth_m = now.fin_depth_m;
        }
        if was.fin_offset_m != now.fin_offset_m {
            steps.push(mutations::change_fenestration_fin_offset(now.id, now.fin_offset_m));
            carried.fin_offset_m = now.fin_offset_m;
        }
        if was.vertices_m != now.vertices_m {
            steps.push(mutations::replace_fenestration_vertices(now.id, now.vertices_m.clone()));
            carried.vertices_m = now.vertices_m.clone();
        }
        if was.glazing_construction_id != now.glazing_construction_id {
            steps.push(match now.glazing_construction_id {
                Some(construction) => mutations::bind_fenestration_glazing_construction(now.id, construction),
                None => mutations::clear_fenestration_glazing_construction(now.id),
            });
            carried.glazing_construction_id = now.glazing_construction_id;
        }
        projected.push(carried);
    }
    projected
}

/// 🧱️ The seven material scalars `set-material-property` addresses. No editor verb creates or
/// deletes a material, so an identity change is still refused LOUDLY rather than masked by the
/// probe below.
fn diff_materials(kind: &'static str, base: &crate::model::Model, model: &crate::model::Model, steps: &mut Vec<EnergyModelMutation>) -> Result<Vec<Material>, Fault> {
    if base.materials.iter().map(|material| material.id).ne(model.materials.iter().map(|material| material.id)) {
        return Err(kind_unavailable(kind, "create-material / delete-material"));
    }
    let mut projected = Vec::with_capacity(model.materials.len());
    for (was, now) in base.materials.iter().zip(&model.materials) {
        let mut carried = was.clone();
        if was.name != now.name {
            steps.push(mutations::rename_material(now.id, now.name.clone()));
            carried.name = now.name.clone();
        }
        if was.roughness != now.roughness {
            steps.push(mutations::change_material_roughness(now.id, now.roughness));
            carried.roughness = now.roughness;
        }
        if was.thickness_m != now.thickness_m {
            steps.push(mutations::change_material_thickness(now.id, now.thickness_m));
            carried.thickness_m = now.thickness_m;
        }
        if was.conductivity_w_m_k != now.conductivity_w_m_k {
            steps.push(mutations::change_material_conductivity(now.id, now.conductivity_w_m_k));
            carried.conductivity_w_m_k = now.conductivity_w_m_k;
        }
        if was.density_kg_m3 != now.density_kg_m3 {
            steps.push(mutations::change_material_density(now.id, now.density_kg_m3));
            carried.density_kg_m3 = now.density_kg_m3;
        }
        if was.specific_heat_j_kg_k != now.specific_heat_j_kg_k {
            steps.push(mutations::change_material_specific_heat(now.id, now.specific_heat_j_kg_k));
            carried.specific_heat_j_kg_k = now.specific_heat_j_kg_k;
        }
        if was.thermal_absorptance != now.thermal_absorptance {
            steps.push(mutations::change_material_thermal_absorptance(now.id, now.thermal_absorptance));
            carried.thermal_absorptance = now.thermal_absorptance;
        }
        if was.solar_absorptance != now.solar_absorptance {
            steps.push(mutations::change_material_solar_absorptance(now.id, now.solar_absorptance));
            carried.solar_absorptance = now.solar_absorptance;
        }
        if was.visible_absorptance != now.visible_absorptance {
            steps.push(mutations::change_material_visible_absorptance(now.id, now.visible_absorptance));
            carried.visible_absorptance = now.visible_absorptance;
        }
        projected.push(carried);
    }
    Ok(projected)
}

/// 🧊️ Glazing materials. `change-glazing-material-*` names only five of the record's twelve optical
/// scalars, so — like [`diff_fenestrations`] — this returns the PROJECTION of `base` through the
/// steps it emitted and `model_edit`'s probe compares against that: a reflectance edit nobody has a
/// mutation for is refused LOUDLY instead of vanishing.
fn diff_glazing_materials(kind: &'static str, base: &crate::model::Model, model: &crate::model::Model, steps: &mut Vec<EnergyModelMutation>) -> Result<Vec<crate::model::GlazingMaterial>, Fault> {
    if base.glazing_materials.iter().map(|material| material.id).ne(model.glazing_materials.iter().map(|material| material.id)) {
        return Err(kind_unavailable(kind, "create-glazing-material / delete-glazing-material"));
    }
    let mut projected = Vec::with_capacity(model.glazing_materials.len());
    for (was, now) in base.glazing_materials.iter().zip(&model.glazing_materials) {
        let mut carried = was.clone();
        if was.name != now.name {
            steps.push(mutations::rename_glazing_material(now.id, now.name.clone()));
            carried.name = now.name.clone();
        }
        if was.thickness_m != now.thickness_m {
            steps.push(mutations::change_glazing_material_thickness(now.id, now.thickness_m));
            carried.thickness_m = now.thickness_m;
        }
        if was.conductivity_w_m_k != now.conductivity_w_m_k {
            steps.push(mutations::change_glazing_material_conductivity(now.id, now.conductivity_w_m_k));
            carried.conductivity_w_m_k = now.conductivity_w_m_k;
        }
        if was.solar_transmittance != now.solar_transmittance {
            steps.push(mutations::change_glazing_material_solar_transmittance(now.id, now.solar_transmittance));
            carried.solar_transmittance = now.solar_transmittance;
        }
        if was.visible_transmittance != now.visible_transmittance {
            steps.push(mutations::change_glazing_material_visible_transmittance(now.id, now.visible_transmittance));
            carried.visible_transmittance = now.visible_transmittance;
        }
        if was.infrared_emissivity_front != now.infrared_emissivity_front || was.infrared_emissivity_back != now.infrared_emissivity_back {
            steps.push(mutations::change_glazing_material_infrared_emissivity(now.id, now.infrared_emissivity_front, now.infrared_emissivity_back));
            carried.infrared_emissivity_front = now.infrared_emissivity_front;
            carried.infrared_emissivity_back = now.infrared_emissivity_back;
        }
        projected.push(carried);
    }
    Ok(projected)
}

/// 💨️ Gas gaps: thickness, fill gas and name — the record's whole addressable surface.
fn diff_gas_materials(kind: &'static str, base: &crate::model::Model, model: &crate::model::Model, steps: &mut Vec<EnergyModelMutation>) -> Result<Vec<crate::model::GasMaterial>, Fault> {
    if base.gas_materials.iter().map(|material| material.id).ne(model.gas_materials.iter().map(|material| material.id)) {
        return Err(kind_unavailable(kind, "create-gas-material / delete-gas-material"));
    }
    let mut projected = Vec::with_capacity(model.gas_materials.len());
    for (was, now) in base.gas_materials.iter().zip(&model.gas_materials) {
        let mut carried = was.clone();
        if was.name != now.name {
            steps.push(mutations::rename_gas_material(now.id, now.name.clone()));
            carried.name = now.name.clone();
        }
        if was.thickness_m != now.thickness_m {
            steps.push(mutations::change_gas_material_thickness(now.id, now.thickness_m));
            carried.thickness_m = now.thickness_m;
        }
        if was.gas != now.gas {
            steps.push(mutations::change_gas_material_gas(now.id, now.gas));
            carried.gas = now.gas;
        }
        projected.push(carried);
    }
    Ok(projected)
}

/// 🧱️ Constructions: the name and the layer stack. The vocabulary names no whole-stack replace, so
/// the stack's change is classified into the THREE list kinds it actually declares —
/// `add-construction-layer` (one insert), `remove-construction-layer` (one delete),
/// `reorder-construction-layers` (a permutation) — and a one-slot exchange is the remove+insert pair
/// at the same index. Anything else (two independent edits in one revision, a create/delete of a
/// construction) is refused LOUDLY rather than masked, exactly like the other catalogues.
///
/// 🔬️ Returns the projection of `base` through the steps it emitted, so an unnamed field faults.
fn diff_constructions(kind: &'static str, base: &crate::model::Model, model: &crate::model::Model, steps: &mut Vec<EnergyModelMutation>) -> Result<Vec<crate::model::Construction>, Fault> {
    if base.constructions.iter().map(|construction| construction.id).ne(model.constructions.iter().map(|construction| construction.id)) {
        return Err(kind_unavailable(kind, "create-construction / delete-construction"));
    }
    let mut projected = Vec::with_capacity(model.constructions.len());
    for (was, now) in base.constructions.iter().zip(&model.constructions) {
        let mut carried = was.clone();
        if was.name != now.name {
            steps.push(mutations::rename_construction(now.id, now.name.clone()));
            carried.name = now.name.clone();
        }
        if was.layer_material_ids != now.layer_material_ids {
            for step in construction_layer_steps(kind, now.id, &was.layer_material_ids, &now.layer_material_ids)? {
                steps.push(step);
            }
            carried.layer_material_ids = now.layer_material_ids.clone();
        }
        projected.push(carried);
    }
    Ok(projected)
}

/// 🧱️ The list kinds that carry one layer-stack edit. `was`/`now` differ by construction.
fn construction_layer_steps(kind: &'static str, id: EntityId, was: &[EntityId], now: &[EntityId]) -> Result<Vec<EnergyModelMutation>, Fault> {
    // ➕️ One insert: dropping the inserted slot from `now` restores `was`.
    if now.len() == was.len() + 1 {
        for index in 0..now.len() {
            let without: Vec<EntityId> = now.iter().enumerate().filter(|(at, _)| *at != index).map(|(_, id)| *id).collect();
            if without == was {
                return Ok(vec![mutations::add_construction_layer(id, index as u32, now[index])]);
            }
        }
    }
    // ➖️ One delete: dropping the removed slot from `was` reaches `now`.
    if was.len() == now.len() + 1 {
        for index in 0..was.len() {
            let without: Vec<EntityId> = was.iter().enumerate().filter(|(at, _)| *at != index).map(|(_, id)| *id).collect();
            if without == now {
                return Ok(vec![mutations::remove_construction_layer(id, index as u32)]);
            }
        }
    }
    if was.len() == now.len() {
        let differing: Vec<usize> = (0..was.len()).filter(|index| was[*index] != now[*index]).collect();
        // 🔁️ One slot exchanged: the vocabulary has no `replace-construction-layer`, so it travels as
        // the remove/insert pair at that same index — order load-bearing, the delete first.
        if differing.len() == 1 {
            let index = differing[0] as u32;
            return Ok(vec![mutations::remove_construction_layer(id, index), mutations::add_construction_layer(id, index, now[differing[0]])]);
        }
        // 🔀️ A permutation of the same multiset: one reorder.
        let (mut sorted_was, mut sorted_now): (Vec<u32>, Vec<u32>) = (was.iter().map(|id| id.0).collect(), now.iter().map(|id| id.0).collect());
        sorted_was.sort_unstable();
        sorted_now.sort_unstable();
        if sorted_was == sorted_now {
            return Ok(vec![mutations::reorder_construction_layers(id, now.to_vec())]);
        }
    }
    Err(kind_unavailable(kind, "replace-construction-layers"))
}

/// 🌡️ The four thermostat fields `set-thermostat-setpoints` addresses. Like materials, no editor
/// verb adds or removes a thermostat, so an identity change is refused rather than masked.
fn diff_thermostats(kind: &'static str, base: &crate::model::Model, model: &crate::model::Model, steps: &mut Vec<EnergyModelMutation>) -> Result<Vec<Thermostat>, Fault> {
    if base.thermostats.iter().map(|thermostat| thermostat.id).ne(model.thermostats.iter().map(|thermostat| thermostat.id)) {
        return Err(kind_unavailable(kind, "create-thermostat / delete-thermostat"));
    }
    let mut projected = Vec::with_capacity(model.thermostats.len());
    for (was, now) in base.thermostats.iter().zip(&model.thermostats) {
        let mut carried = was.clone();
        if was.zone_id != now.zone_id {
            steps.push(mutations::change_thermostat_zone(now.id, now.zone_id));
            carried.zone_id = now.zone_id;
        }
        if was.heating_setpoint_schedule_id != now.heating_setpoint_schedule_id {
            steps.push(mutations::change_thermostat_heating_setpoint_schedule(now.id, now.heating_setpoint_schedule_id));
            carried.heating_setpoint_schedule_id = now.heating_setpoint_schedule_id;
        }
        if was.cooling_setpoint_schedule_id != now.cooling_setpoint_schedule_id {
            steps.push(mutations::change_thermostat_cooling_setpoint_schedule(now.id, now.cooling_setpoint_schedule_id));
            carried.cooling_setpoint_schedule_id = now.cooling_setpoint_schedule_id;
        }
        if was.heating_throttle_range_k != now.heating_throttle_range_k {
            steps.push(mutations::change_thermostat_heating_throttle_range(now.id, now.heating_throttle_range_k));
            carried.heating_throttle_range_k = now.heating_throttle_range_k;
        }
        if was.cooling_throttle_range_k != now.cooling_throttle_range_k {
            steps.push(mutations::change_thermostat_cooling_throttle_range(now.id, now.cooling_throttle_range_k));
            carried.cooling_throttle_range_k = now.cooling_throttle_range_k;
        }
        projected.push(carried);
    }
    Ok(projected)
}

/// 📂️ The sanctioned whole-document load: a `kernel::Effect::LoadDocument` carrying a genesis
/// pack+spr the host swaps into the live store through `ArtifactStore::reset`, OUTSIDE undo history.
/// This is why `📚️examples` need no `replace-model` kind — whole-document replace has no mutation
/// representative in this artifact's vocabulary at all (`📓️derivation-rules.md` rule 6), exactly as
/// in `📐️cad`'s `reset_document_effect` and `🔱️trinity`'s. A freshly minted envelope has no edits,
/// so its spr encode is infallible.
fn load_document_effect(model: &crate::model::Model) -> semio_framework_plugin::kernel::Effect {
    let snapshot = crate::energy_snapshot_with_state(ENERGY_MODEL_DOCUMENT_SCHEMA, model, None);
    let pack = <EnergyModelSnapshot as store::ArtifactPack>::encode_pack(&snapshot);
    let envelope = store::create_document_envelope::<EnergyModelSnapshot, EnergyModelMutation>(ENERGY_MODEL_DOCUMENT_SCHEMA, "model", snapshot, None).into_owners();
    let spr = semio_framework_plugin::resolve_ready(store::print_document_spr(&envelope)).expect("energy model document spr encode is infallible for a fresh, edit-free envelope");
    semio_framework_plugin::kernel::Effect::LoadDocument { pack, spr }
}

/// ⛔️ The refusal a not-yet-landed mutation group raises, naming itself instead of degrading to a
/// silent no-op or a banned whole-document replace.
fn kind_unavailable(kind: &'static str, missing: &str) -> Fault {
    Fault::new(FaultOrigin::App, FaultCode::new("mutation.kind-unavailable"), format!("the editor command {kind:?} needs the semantic mutation {missing:?}, which this artifact's vocabulary does not declare yet"))
}

/// 🆔️ Mints the next free `EntityId` for a collection addressed by its own ids — `max + 1`, so an
/// id is never reused after a delete and every reference in the document stays unambiguous.
fn next_entity_id(existing: impl Iterator<Item = EntityId>) -> EntityId {
    EntityId(existing.map(|id| id.0).max().unwrap_or(0).saturating_add(1))
}

fn surface_class_from_id(id: &str) -> Option<SurfaceClass> {
    Some(match id {
        "exteriorWall" => SurfaceClass::ExteriorWall,
        "interiorWall" => SurfaceClass::InteriorWall,
        "roof" => SurfaceClass::Roof,
        "ceiling" => SurfaceClass::Ceiling,
        "floor" => SurfaceClass::Floor,
        "interzone" => SurfaceClass::Interzone,
        "adiabatic" => SurfaceClass::Adiabatic,
        "ground" => SurfaceClass::Ground,
        _ => return None,
    })
}

fn target_missing(entity: &str, id: u32) -> Fault {
    Fault::new(FaultOrigin::App, FaultCode::new("mutation.target-missing"), format!("the energy model has no {entity} with id {id}"))
}

fn target_in_use(entity: &str, id: u32, blocker: &str) -> Fault {
    Fault::new(FaultOrigin::App, FaultCode::new("mutation.target-in-use"), format!("{entity} {id} is still referenced by at least one {blocker}"))
}
//#endregion 🧬️MutationSeam

//#region 🔖️Reduce
/// 🧩️ The one pure reducer both `ArtifactEditor::handle` and the retained bounded work step run —
/// identical semantics on the interactive path and on the retained path by construction.
fn reduce(command: &EnergyModelEditorCommand, doc: &ArtifactView<'_, EnergyModelSnapshot>) -> Result<Emit<EnergyModelMutation, EnergyModelConfigMutation>, Fault> {
    let mut model = crate::energy_model(doc.snapshot);
    let (kind, description) = match command {
        EnergyModelEditorCommand::SetStructureField { field, value } => {
            match field.as_str() {
                "name" => model.name = value.clone(),
                "version" => model.version = value.clone(),
                _ => return Ok(Emit::default()),
            }
            ("rename-model", format!("Set {field}"))
        }
        EnergyModelEditorCommand::SetZoneCell { row, column, value } => {
            let Some(zone) = model.zones.get_mut(*row as usize) else { return Ok(Emit::default()) };
            match column.as_str() {
                "name" => zone.name = value.clone(),
                "volumeM3" => match value.parse::<f64>() {
                    Ok(parsed) => zone.volume_m3 = parsed,
                    Err(_) => return Ok(Emit::default()),
                },
                "multiplier" => match value.parse::<u32>() {
                    Ok(parsed) => zone.multiplier = parsed,
                    Err(_) => return Ok(Emit::default()),
                },
                "conditioned" => match value.parse::<bool>() {
                    Ok(parsed) => zone.conditioned = parsed,
                    Err(_) => return Ok(Emit::default()),
                },
                "partOfTotalFloorArea" => match value.parse::<bool>() {
                    Ok(parsed) => zone.part_of_total_floor_area = parsed,
                    Err(_) => return Ok(Emit::default()),
                },
                _ => return Ok(Emit::default()),
            }
            ("change-zone-volume", format!("Set zone {row} {column}"))
        }
        EnergyModelEditorCommand::CreateZone { name, volume_m3, multiplier, conditioned } => {
            if *volume_m3 <= 0.0 {
                return Err(Fault::new(FaultOrigin::App, FaultCode::new("mutation.invalid-payload"), "a zone volume must be strictly positive"));
            }
            let id = next_entity_id(model.zones.iter().map(|zone| zone.id));
            model.zones.push(Zone { id, name: name.clone(), volume_m3: *volume_m3, multiplier: (*multiplier).max(1), conditioned: *conditioned, part_of_total_floor_area: true });
            ("create-zone", format!("Create zone {name}"))
        }
        EnergyModelEditorCommand::RenameZone { zone, new_name } => {
            let target = model.zones.iter_mut().find(|entry| entry.id.0 == *zone).ok_or_else(|| target_missing("zone", *zone))?;
            target.name = new_name.clone();
            ("rename-zone", format!("Rename zone {zone}"))
        }
        EnergyModelEditorCommand::DeleteZone { zone } => {
            let id = EntityId(*zone);
            if !model.zones.iter().any(|entry| entry.id == id) {
                return Err(target_missing("zone", *zone));
            }
            if model.spaces.iter().any(|space| space.zone_id == id) {
                return Err(target_in_use("zone", *zone, "space"));
            }
            if model.surfaces.iter().any(|surface| surface.zone_id == id) {
                return Err(target_in_use("zone", *zone, "surface"));
            }
            if model.thermostats.iter().any(|thermostat| thermostat.zone_id == id) {
                return Err(target_in_use("zone", *zone, "thermostat"));
            }
            model.zones.retain(|entry| entry.id != id);
            ("delete-zone", format!("Delete zone {zone}"))
        }
        EnergyModelEditorCommand::CreateSurface { name, zone, construction, class } => {
            let zone_id = EntityId(*zone);
            let construction_id = EntityId(*construction);
            if !model.zones.iter().any(|entry| entry.id == zone_id) {
                return Err(target_missing("zone", *zone));
            }
            if !model.constructions.iter().any(|entry| entry.id == construction_id) {
                return Err(target_missing("construction", *construction));
            }
            let class = surface_class_from_id(class).ok_or_else(|| Fault::new(FaultOrigin::App, FaultCode::new("mutation.invalid-payload"), format!("'{class}' is not a surface class")))?;
            let id = next_entity_id(model.surfaces.iter().map(|surface| surface.id));
            model.surfaces.push(Surface {
                id,
                name: name.clone(),
                zone_id,
                class,
                vertices_m: vec![[0.0, 0.0, 0.0], [1.0, 0.0, 0.0], [1.0, 0.0, 1.0], [0.0, 0.0, 1.0]],
                construction_id,
                outside_boundary_condition: OutsideBoundary::OutdoorAir,
                sun_exposed: true,
                wind_exposed: true,
                multiplier: 1,
            });
            ("create-surface", format!("Create surface {name}"))
        }
        EnergyModelEditorCommand::DeleteSurface { surface } => {
            let id = EntityId(*surface);
            if !model.surfaces.iter().any(|entry| entry.id == id) {
                return Err(target_missing("surface", *surface));
            }
            model.fenestrations.retain(|fenestration| fenestration.surface_id != id);
            model.adjacency_pairs.retain(|pair| pair.surface_a_id != id && pair.surface_b_id != id);
            model.surfaces.retain(|entry| entry.id != id);
            ("delete-surface", format!("Delete surface {surface}"))
        }
        EnergyModelEditorCommand::AssignSurfaceConstruction { surface, construction } => {
            let construction_id = EntityId(*construction);
            if !model.constructions.iter().any(|entry| entry.id == construction_id) {
                return Err(target_missing("construction", *construction));
            }
            let target = model.surfaces.iter_mut().find(|entry| entry.id.0 == *surface).ok_or_else(|| target_missing("surface", *surface))?;
            target.construction_id = construction_id;
            ("change-surface-construction", format!("Assign construction {construction} to surface {surface}"))
        }
        EnergyModelEditorCommand::SetMaterialProperty { material, property, value } => {
            let target = model.materials.iter_mut().find(|entry| entry.id.0 == *material).ok_or_else(|| target_missing("material", *material))?;
            let kind = set_material_property(target, property, value)?;
            (kind, format!("Set material {material} {property}"))
        }
        EnergyModelEditorCommand::SetConstructionProperty { construction, property, value } => {
            let kind = set_construction_property(&mut model, *construction, property, value)?;
            (kind, format!("Set construction {construction} {property}"))
        }
        EnergyModelEditorCommand::SetSurfaceProperty { surface, property, value, partner_surface } => {
            let kind = set_surface_property(&mut model, *surface, property, value, *partner_surface)?;
            (kind, format!("Set surface {surface} {property}"))
        }
        EnergyModelEditorCommand::SetFenestrationProperty { fenestration, property, value } => {
            let kind = set_fenestration_property(&mut model, *fenestration, property, value)?;
            (kind, format!("Set window {fenestration} {property}"))
        }
        EnergyModelEditorCommand::SetGlazingMaterialProperty { material, property, value } => {
            let target = model.glazing_materials.iter_mut().find(|entry| entry.id.0 == *material).ok_or_else(|| target_missing("glazing material", *material))?;
            let kind = set_glazing_material_property(target, property, value)?;
            (kind, format!("Set glazing material {material} {property}"))
        }
        EnergyModelEditorCommand::SetGasMaterialProperty { material, property, value } => {
            let target = model.gas_materials.iter_mut().find(|entry| entry.id.0 == *material).ok_or_else(|| target_missing("gas material", *material))?;
            let kind = set_gas_material_property(target, property, value)?;
            (kind, format!("Set gas material {material} {property}"))
        }
        EnergyModelEditorCommand::SetZoneProperty { zone, property, value } => {
            let target = model.zones.iter_mut().find(|entry| entry.id.0 == *zone).ok_or_else(|| target_missing("zone", *zone))?;
            let kind = set_zone_property(target, property, value)?;
            (kind, format!("Set zone {zone} {property}"))
        }
        EnergyModelEditorCommand::SetThermostatSetpoints { thermostat, heating_schedule, cooling_schedule, heating_throttle_range_k, cooling_throttle_range_k } => {
            let schedules = &model.schedules;
            if !schedule_exists(schedules, *heating_schedule) {
                return Err(target_missing("schedule", *heating_schedule));
            }
            if !schedule_exists(schedules, *cooling_schedule) {
                return Err(target_missing("schedule", *cooling_schedule));
            }
            let target: &mut Thermostat = model.thermostats.iter_mut().find(|entry| entry.id.0 == *thermostat).ok_or_else(|| target_missing("thermostat", *thermostat))?;
            target.heating_setpoint_schedule_id = ScheduleId(*heating_schedule);
            target.cooling_setpoint_schedule_id = ScheduleId(*cooling_schedule);
            target.heating_throttle_range_k = *heating_throttle_range_k;
            target.cooling_throttle_range_k = *cooling_throttle_range_k;
            ("change-thermostat-heating-setpoint-schedule", format!("Set thermostat {thermostat} setpoints"))
        }
        EnergyModelEditorCommand::SetSite { latitude_deg, longitude_deg, elevation_m, time_zone_hours, north_axis_deg } => {
            if !(-90.0..=90.0).contains(latitude_deg) || !(-180.0..=180.0).contains(longitude_deg) {
                return Err(Fault::new(FaultOrigin::App, FaultCode::new("mutation.invalid-payload"), "the site latitude/longitude are outside their SI ranges"));
            }
            model.site = Site { latitude_deg: *latitude_deg, longitude_deg: *longitude_deg, elevation_m: *elevation_m, time_zone_hours: *time_zone_hours, north_axis_deg: *north_axis_deg };
            ("update-site", "Set site".to_string())
        }
        EnergyModelEditorCommand::SetRunPeriod { start_month, start_day, end_month, end_day } => {
            let valid = (1..=12).contains(start_month) && (1..=12).contains(end_month) && (1..=31).contains(start_day) && (1..=31).contains(end_day);
            if !valid {
                return Err(Fault::new(FaultOrigin::App, FaultCode::new("mutation.invalid-payload"), "the run period must address real calendar months and days"));
            }
            model.run_period.start_month = *start_month as u8;
            model.run_period.start_day = *start_day as u8;
            model.run_period.end_month = *end_month as u8;
            model.run_period.end_day = *end_day as u8;
            ("update-run-period", "Set run period".to_string())
        }
        EnergyModelEditorCommand::SetActiveExample { example_id } => {
            let loaded = example_model(example_id).ok_or_else(|| Fault::new(FaultOrigin::App, FaultCode::new("mutation.target-missing"), format!("this artifact bundles no example {example_id:?}")))?;
            return Ok(Emit { effects: vec![load_document_effect(&loaded)], description: Some(format!("Load example {example_id}")), ..Default::default() });
        }
        EnergyModelEditorCommand::SetSimulationSettings { zone_timestep_minutes, system_timestep_minutes, warmup_days } => {
            let settings = ChangeSimulationSettings { zone_timestep_minutes: *zone_timestep_minutes, system_timestep_minutes: *system_timestep_minutes, warmup_days: *warmup_days };
            if !settings.config().is_valid() {
                return Err(Fault::new(FaultOrigin::App, FaultCode::new("mutation.invalid-payload"), "the simulation settings are outside the engine's admissible timestep and warmup ranges"));
            }
            return Ok(Emit { config_mutations: vec![EnergyModelConfigMutation::ChangeSimulationSettings(settings)], description: Some("Set simulation settings".into()), ..Default::default() });
        }
        EnergyModelEditorCommand::SetResultField { field } => {
            let Some(selected) = crate::editor::model::results::ResultField::from_id(field) else {
                return Err(Fault::new(FaultOrigin::App, FaultCode::new("mutation.invalid-payload"), format!("'{field}' is not a published per-surface result field")));
            };
            // 🎨️ Only the 3d model window's body re-renders: the map, the ramp and the legend are all
            // derived inside its own `render`, and nothing else in the editor reads `resultField`.
            return Ok(Emit {
                config_mutations: vec![EnergyModelConfigMutation::ChangeResultField(ChangeResultField { field: selected.id().to_string() })],
                description: Some(format!("Colour surfaces by {}", selected.id())),
                ui_scope: UiDirtyScope::Partial {
                    window_bodies: vec![crate::energy_simulation_session::ENERGY_MODEL_3D_WINDOW_KIND_ID.to_string(), simulation::BODY_KEY.to_string()],
                    panel_bodies: Vec::new(),
                    utilities: false,
                    tools: false,
                    engagements: false,
                    measures: false,
                    labels: false,
                },
                ..Default::default()
            });
        }
        // 🎥️ A camera is addressed at ONE window instance, and `reduce` sees no `ViewModel` — both
        // dispatch routes intercept `SetCamera` through `camera_emit` before they ever get here, so
        // reaching this arm means the gesture arrived without its window context.
        EnergyModelEditorCommand::SetCamera { .. } => {
            return Err(Fault::new(FaultOrigin::App, FaultCode::new("energy.model.3d.window-required"), "a camera change requires a concrete 3d model window"));
        }
    };
    model_edit(kind, &crate::energy_model(doc.snapshot), &model, description)
}

/// 🎥️ The window-addressed half of the command set: `setCamera` writes the addressed
/// `energy.model.3d` window's own retained pose, never the document and never the app config. Both
/// dispatch routes (`ArtifactEditor::handle` and the retained `energy_model_reduce`) call this FIRST
/// and only fall through to [`reduce`] when it answers `None`, so the two can never diverge.
///
/// 🐢️ `UiDirtyScope::Partial` with nothing listed: the pane that sent the pose already holds it, and
/// republishing the 3d body on every debounced orbit tick would be pure churn. A window that opens
/// later reads the stored pose on its own first render.
fn camera_emit(command: &EnergyModelEditorCommand, view_state: Option<&semio_framework_plugin::ViewModel>) -> Option<Result<Emit<EnergyModelMutation, EnergyModelConfigMutation, NoDraftMutation>, Fault>> {
    let EnergyModelEditorCommand::SetCamera { camera } = command else { return None };
    Some((|| {
        let view = view_state.ok_or_else(|| Fault::new(FaultOrigin::App, FaultCode::new("energy.model.3d.window-required"), "a camera change requires a concrete 3d model window"))?;
        if camera.is_empty() {
            return Err(Fault::new(FaultOrigin::App, FaultCode::new("app.command.invalid-payload"), "setCamera carries no {position,target,zoom} pose"));
        }
        let value = dsl::json::from_json_str::<dsl::DslValue>(camera).map_err(|error| Fault::new(FaultOrigin::App, FaultCode::new("app.command.invalid-payload"), format!("the camera pose is not a value: {error}")))?;
        let pose = <model_window::config::EnergyModelCameraPose as dsl::FromValue>::from_value(value).map_err(|error| Fault::new(FaultOrigin::App, FaultCode::new("app.command.invalid-payload"), format!("the camera pose is malformed: {error}")))?;
        if !pose.is_valid() {
            return Err(Fault::new(FaultOrigin::App, FaultCode::new("app.command.invalid-payload"), "the camera pose is not finite, or its zoom is not positive"));
        }
        let mutation = model_window::config::EnergyModelWindowConfigMutation::SetCamera(model_window::config::SetCamera { camera: pose });
        Ok(Emit {
            window_config_mutations: vec![model_window::config::addressed(view, mutation)?],
            description: Some("Set camera".into()),
            coalesce_key: Some(format!("energy.model.3d.camera:{}", view.window_id.as_deref().unwrap_or_default())),
            ui_scope: UiDirtyScope::Partial { window_bodies: Vec::new(), panel_bodies: Vec::new(), utilities: false, tools: false, engagements: false, measures: false, labels: false },
            ..Default::default()
        })
    })())
}

/// 🗓️ A thermostat setpoint reference must resolve inside the model's own `ScheduleSet` — the five
/// schedule families are addressed by one shared `ScheduleId` space, so any of them may own it.
fn schedule_exists(schedules: &crate::schedule::ScheduleSet, id: u32) -> bool {
    let id = ScheduleId(id);
    schedules.constants.iter().any(|schedule| schedule.id == id)
        || schedules.daily.iter().any(|schedule| schedule.id == id)
        || schedules.weekly.iter().any(|schedule| schedule.id == id)
        || schedules.annual.iter().any(|schedule| schedule.id == id)
        || schedules.time_series.iter().any(|schedule| schedule.id == id)
}

//#region 🔍️InspectorProperties
/// ⛔️ `'{property}'` is not a field of `{entity}`.
fn unknown_property(entity: &str, property: &str) -> Fault {
    Fault::new(FaultOrigin::App, FaultCode::new("mutation.invalid-payload"), format!("'{property}' is not a {entity} property"))
}

/// ⛔️ `'{value}'` cannot be read as `{property}`, or lies outside its SI range.
fn invalid_value(property: &str, value: &str) -> Fault {
    Fault::new(FaultOrigin::App, FaultCode::new("mutation.invalid-payload"), format!("'{value}' is outside the admissible range of property '{property}'"))
}

fn as_f64(property: &str, value: &str, admits: impl Fn(f64) -> bool) -> Result<f64, Fault> {
    let parsed: f64 = value.trim().parse().map_err(|_| invalid_value(property, value))?;
    if !parsed.is_finite() || !admits(parsed) {
        return Err(invalid_value(property, value));
    }
    Ok(parsed)
}

fn as_u32(property: &str, value: &str, admits: impl Fn(u32) -> bool) -> Result<u32, Fault> {
    let parsed: u32 = value.trim().parse().map_err(|_| invalid_value(property, value))?;
    if !admits(parsed) {
        return Err(invalid_value(property, value));
    }
    Ok(parsed)
}

fn as_bool(property: &str, value: &str) -> Result<bool, Fault> {
    match value.trim() {
        "true" | "1" | "on" | "yes" => Ok(true),
        "false" | "0" | "off" | "no" | "" => Ok(false),
        _ => Err(invalid_value(property, value)),
    }
}

/// 🚧️ The wire spelling of an [`OutsideBoundaryKind`] — camelCase, the same vocabulary
/// [`surface_class_from_id`] uses for [`SurfaceClass`].
fn outside_boundary_kind_from_id(id: &str) -> Option<crate::model::OutsideBoundaryKind> {
    use crate::model::OutsideBoundaryKind as Kind;
    Some(match id {
        "outdoorAir" => Kind::OutdoorAir,
        "ground" => Kind::Ground,
        "otherSideTemperature" => Kind::OtherSideTemperature,
        "adiabatic" => Kind::Adiabatic,
        "interzone" => Kind::Interzone,
        _ => return None,
    })
}

/// 🟫️ Every `Surface` field the inspector addresses. `construction`/`boundary` validate their
/// references against the live model FIRST, so a dangling id is a refusal and never a document that
/// points at nothing.
fn set_surface_property(model: &mut crate::model::Model, surface: u32, property: &str, value: &str, partner_surface: u32) -> Result<&'static str, Fault> {
    let boundary = match property {
        "boundary" => {
            let kind = outside_boundary_kind_from_id(value.trim()).ok_or_else(|| invalid_value(property, value))?;
            // 🚧️ An interzone boundary needs its other half. A control that carries only the kind can
            // name the partner nowhere, so a partner already on the surface is CARRIED FORWARD rather
            // than dropped — re-picking `interzone` on a surface that is already interzone keeps its
            // neighbour instead of refusing.
            let held = model.surfaces.iter().find(|entry| entry.id.0 == surface).and_then(|entry| interzone_partner(entry.outside_boundary_condition));
            let partner = (partner_surface != 0).then_some(EntityId(partner_surface)).or(held);
            if let Some(partner) = partner {
                if !model.surfaces.iter().any(|entry| entry.id == partner) {
                    return Err(target_missing("surface", partner.0));
                }
            }
            Some(OutsideBoundary::from_parts(kind, partner).ok_or_else(|| invalid_value(property, value))?)
        }
        // 🚧️ The one control that can MAKE a surface interzone: `value` is the partner surface, and the
        // boundary becomes `Interzone(partner)` in the same step. `partner_surface` stays the palette's
        // spelling; the inspector's flat `{field, id, value}` shape has only one slot to carry it in.
        "interzonePartner" => {
            let partner = EntityId(as_u32(property, value, |parsed| parsed != 0)?);
            if partner.0 == surface {
                return Err(invalid_value(property, value));
            }
            if !model.surfaces.iter().any(|entry| entry.id == partner) {
                return Err(target_missing("surface", partner.0));
            }
            Some(OutsideBoundary::Interzone(partner))
        }
        _ => None,
    };
    let construction = match property {
        "construction" => {
            let id = EntityId(as_u32(property, value, |_| true)?);
            if !model.constructions.iter().any(|entry| entry.id == id) {
                return Err(target_missing("construction", id.0));
            }
            Some(id)
        }
        _ => None,
    };
    let target = model.surfaces.iter_mut().find(|entry| entry.id.0 == surface).ok_or_else(|| target_missing("surface", surface))?;
    Ok(match property {
        "name" => {
            target.name = value.to_string();
            "rename-surface"
        }
        "class" => {
            target.class = surface_class_from_id(value.trim()).ok_or_else(|| invalid_value(property, value))?;
            "change-surface-class"
        }
        "boundary" | "interzonePartner" => {
            target.outside_boundary_condition = boundary.expect("the boundary arm parsed its payload above");
            "change-surface-boundary-condition"
        }
        "construction" => {
            target.construction_id = construction.expect("the construction arm parsed its payload above");
            "change-surface-construction"
        }
        "sunExposed" => {
            target.sun_exposed = as_bool(property, value)?;
            "change-surface-sun-exposed"
        }
        "windExposed" => {
            target.wind_exposed = as_bool(property, value)?;
            "change-surface-wind-exposed"
        }
        "multiplier" => {
            target.multiplier = as_u32(property, value, |parsed| parsed >= 1)?;
            "change-surface-multiplier"
        }
        _ => return Err(unknown_property("surface", property)),
    })
}

/// 🪟️ Every `Fenestration` field the inspector addresses — the fifteen scalars plus the optional
/// glazing construction, whose EMPTY value clears the binding rather than naming a construction.
fn set_fenestration_property(model: &mut crate::model::Model, fenestration: u32, property: &str, value: &str) -> Result<&'static str, Fault> {
    let glazing = match property {
        "glazingConstruction" => {
            let trimmed = value.trim();
            if trimmed.is_empty() || trimmed == "none" {
                Some(None)
            } else {
                let id = EntityId(as_u32(property, value, |_| true)?);
                if !model.constructions.iter().any(|entry| entry.id == id) {
                    return Err(target_missing("construction", id.0));
                }
                Some(Some(id))
            }
        }
        _ => None,
    };
    let target = model.fenestrations.iter_mut().find(|entry| entry.id.0 == fenestration).ok_or_else(|| target_missing("fenestration", fenestration))?;
    let positive = |parsed: f64| parsed > 0.0;
    let non_negative = |parsed: f64| parsed >= 0.0;
    let fraction = |parsed: f64| (0.0..=1.0).contains(&parsed);
    Ok(match property {
        "name" => {
            target.name = value.to_string();
            "rename-fenestration"
        }
        "uValueWM2K" => {
            target.u_value_w_m2k = as_f64(property, value, positive)?;
            "change-fenestration-u-value"
        }
        "shgc" => {
            target.shgc = as_f64(property, value, fraction)?;
            "change-fenestration-shgc"
        }
        "vlt" => {
            target.vlt = as_f64(property, value, fraction)?;
            "change-fenestration-vlt"
        }
        "areaM2" => {
            target.area_m2 = as_f64(property, value, positive)?;
            "change-fenestration-area"
        }
        "heightM" => {
            target.height_m = as_f64(property, value, positive)?;
            "change-fenestration-height"
        }
        "sillHeightM" => {
            target.sill_height_m = as_f64(property, value, non_negative)?;
            "change-fenestration-sill-height"
        }
        "frameConductanceWK" => {
            target.frame_conductance_w_k = as_f64(property, value, non_negative)?;
            "change-fenestration-frame-conductance"
        }
        "dividerConductanceWK" => {
            target.divider_conductance_w_k = as_f64(property, value, non_negative)?;
            "change-fenestration-divider-conductance"
        }
        "overhangDepthM" => {
            target.overhang_depth_m = as_f64(property, value, non_negative)?;
            "change-fenestration-overhang-depth"
        }
        "overhangOffsetM" => {
            target.overhang_offset_m = as_f64(property, value, non_negative)?;
            "change-fenestration-overhang-offset"
        }
        "finDepthM" => {
            target.fin_depth_m = as_f64(property, value, non_negative)?;
            "change-fenestration-fin-depth"
        }
        "finOffsetM" => {
            target.fin_offset_m = as_f64(property, value, non_negative)?;
            "change-fenestration-fin-offset"
        }
        "glazingConstruction" => {
            target.glazing_construction_id = glazing.expect("the glazing arm parsed its payload above");
            "bind-fenestration-glazing-construction"
        }
        _ => return Err(unknown_property("fenestration", property)),
    })
}

/// 🧊️ The glazing-material fields this artifact's vocabulary can actually name. The seven other
/// optical scalars (`solar_reflectance_*`, `visible_reflectance_*`, `infrared_transmittance`) have no
/// mutation kind, so they are refused here rather than written and then faulted by the probe.
fn set_glazing_material_property(material: &mut crate::model::GlazingMaterial, property: &str, value: &str) -> Result<&'static str, Fault> {
    let positive = |parsed: f64| parsed > 0.0;
    let fraction = |parsed: f64| (0.0..=1.0).contains(&parsed);
    Ok(match property {
        "name" => {
            material.name = value.to_string();
            "rename-glazing-material"
        }
        "thicknessM" => {
            material.thickness_m = as_f64(property, value, positive)?;
            "change-glazing-material-thickness"
        }
        "conductivityWMK" => {
            material.conductivity_w_m_k = as_f64(property, value, positive)?;
            "change-glazing-material-conductivity"
        }
        "solarTransmittance" => {
            material.solar_transmittance = as_f64(property, value, fraction)?;
            "change-glazing-material-solar-transmittance"
        }
        "visibleTransmittance" => {
            material.visible_transmittance = as_f64(property, value, fraction)?;
            "change-glazing-material-visible-transmittance"
        }
        "infraredEmissivityFront" => {
            material.infrared_emissivity_front = as_f64(property, value, fraction)?;
            "change-glazing-material-infrared-emissivity"
        }
        "infraredEmissivityBack" => {
            material.infrared_emissivity_back = as_f64(property, value, fraction)?;
            "change-glazing-material-infrared-emissivity"
        }
        _ => return Err(unknown_property("glazing material", property)),
    })
}

/// 💨️ The gas-gap fields: thickness, fill gas and name.
fn set_gas_material_property(material: &mut crate::model::GasMaterial, property: &str, value: &str) -> Result<&'static str, Fault> {
    Ok(match property {
        "name" => {
            material.name = value.to_string();
            "rename-gas-material"
        }
        "thicknessM" => {
            material.thickness_m = as_f64(property, value, |parsed| parsed > 0.0)?;
            "change-gas-material-thickness"
        }
        "gas" => {
            material.gas = gas_kind_from_id(value.trim()).ok_or_else(|| invalid_value(property, value))?;
            "change-gas-material-gas"
        }
        _ => return Err(unknown_property("gas material", property)),
    })
}

/// 💨️ Wire spelling of a [`crate::model::GasKind`] — camelCase, like every other enum this editor
/// carries over the action bus.
pub fn gas_kind_id(gas: crate::model::GasKind) -> &'static str {
    use crate::model::GasKind as Kind;
    match gas {
        Kind::Air => "air",
        Kind::Argon => "argon",
        Kind::Krypton => "krypton",
        Kind::Xenon => "xenon",
    }
}

fn gas_kind_from_id(id: &str) -> Option<crate::model::GasKind> {
    use crate::model::GasKind as Kind;
    Some(match id {
        "air" => Kind::Air,
        "argon" => Kind::Argon,
        "krypton" => Kind::Krypton,
        "xenon" => Kind::Xenon,
        _ => return None,
    })
}

/// 💨️ Every fill gas, in declaration order — the inspector's gas select.
pub const GAS_KIND_IDS: &[&str] = &["air", "argon", "krypton", "xenon"];

/// 🏘️ Every `Zone` field, addressed by the zone's own `EntityId` rather than by its table row.
fn set_zone_property(zone: &mut Zone, property: &str, value: &str) -> Result<&'static str, Fault> {
    Ok(match property {
        "name" => {
            zone.name = value.to_string();
            "rename-zone"
        }
        "volumeM3" => {
            zone.volume_m3 = as_f64(property, value, |parsed| parsed > 0.0)?;
            "change-zone-volume"
        }
        "multiplier" => {
            zone.multiplier = as_u32(property, value, |parsed| parsed >= 1)?;
            "change-zone-multiplier"
        }
        "conditioned" => {
            zone.conditioned = as_bool(property, value)?;
            "change-zone-conditioned"
        }
        "partOfTotalFloorArea" => {
            zone.part_of_total_floor_area = as_bool(property, value)?;
            "change-zone-floor-area-participation"
        }
        _ => return Err(unknown_property("zone", property)),
    })
}
//#endregion 🔍️InspectorProperties

/// 🧱️ The material record's WHOLE addressable surface: its name, its roughness class and its seven
/// SI scalars. `value` is text like every other inspector verb, so one control shape carries a name,
/// an enum spelling and a number alike; the numeric properties parse it and refuse a non-finite or
/// out-of-range reading rather than writing it.
fn set_material_property(material: &mut Material, property: &str, value: &str) -> Result<&'static str, Fault> {
    let positive = |parsed: f64| parsed > 0.0;
    let fraction = |parsed: f64| (0.0..=1.0).contains(&parsed);
    Ok(match property {
        "name" => {
            material.name = value.to_string();
            "rename-material"
        }
        "roughness" => {
            material.roughness = surface_roughness_from_id(value.trim()).ok_or_else(|| invalid_value(property, value))?;
            "change-material-roughness"
        }
        "thicknessM" => {
            material.thickness_m = as_f64(property, value, positive)?;
            "change-material-thickness"
        }
        "conductivityWMK" => {
            material.conductivity_w_m_k = as_f64(property, value, positive)?;
            "change-material-conductivity"
        }
        "densityKgM3" => {
            material.density_kg_m3 = as_f64(property, value, positive)?;
            "change-material-density"
        }
        "specificHeatJKgK" => {
            material.specific_heat_j_kg_k = as_f64(property, value, positive)?;
            "change-material-specific-heat"
        }
        "thermalAbsorptance" => {
            material.thermal_absorptance = as_f64(property, value, fraction)?;
            "change-material-thermal-absorptance"
        }
        "solarAbsorptance" => {
            material.solar_absorptance = as_f64(property, value, fraction)?;
            "change-material-solar-absorptance"
        }
        "visibleAbsorptance" => {
            material.visible_absorptance = as_f64(property, value, fraction)?;
            "change-material-visible-absorptance"
        }
        _ => return Err(unknown_property("material", property)),
    })
}

/// 🧱️ One construction field by name, or ONE list edit of its layer stack. The layer verbs carry
/// their operand in `value` because a rendered control merges exactly one scalar there:
/// `addLayer` = the material id to append, `removeLayer`/`moveLayerUp`/`moveLayerDown` = the layer
/// index, `replaceLayer:<index>` = the material id that takes that slot (the un-suffixed spelling
/// takes `<index>:<materialId>`, which is what a palette invocation types).
///
/// ⚠️ `add-construction-layer` admits an OPAQUE `Material` only — its own diff refuses a glazing or
/// gas id with `mutation.target-missing`. So an add/replace naming one is refused HERE, loudly and
/// early, instead of reducing cleanly and dying at the store.
fn set_construction_property(model: &mut crate::model::Model, construction: u32, property: &str, value: &str) -> Result<&'static str, Fault> {
    let opaque: Vec<EntityId> = model.materials.iter().map(|material| material.id).collect();
    let index = model.constructions.iter().position(|entry| entry.id.0 == construction).ok_or_else(|| target_missing("construction", construction))?;
    let layer_material = |raw: &str| -> Result<EntityId, Fault> {
        let id = EntityId(as_u32(property, raw, |_| true)?);
        if !opaque.contains(&id) {
            return Err(Fault::new(
                FaultOrigin::App,
                FaultCode::new("mutation.invalid-payload"),
                format!("layer material {} is not an opaque material — 'add-construction-layer' declares no glazing or gas layer", id.0),
            ));
        }
        Ok(id)
    };
    let slot = |raw: &str, layers: &[EntityId]| -> Result<usize, Fault> {
        let at = as_u32(property, raw, |_| true)? as usize;
        if at >= layers.len() {
            return Err(invalid_value(property, raw));
        }
        Ok(at)
    };
    Ok(match property {
        "name" => {
            model.constructions[index].name = value.to_string();
            "rename-construction"
        }
        "addLayer" => {
            let material = layer_material(value.trim())?;
            model.constructions[index].layer_material_ids.push(material);
            "add-construction-layer"
        }
        "removeLayer" => {
            let at = slot(value.trim(), &model.constructions[index].layer_material_ids)?;
            model.constructions[index].layer_material_ids.remove(at);
            "remove-construction-layer"
        }
        "moveLayerUp" | "moveLayerDown" => {
            let at = slot(value.trim(), &model.constructions[index].layer_material_ids)?;
            let other = if property == "moveLayerUp" { at.checked_sub(1) } else { at.checked_add(1) };
            let layers = &mut model.constructions[index].layer_material_ids;
            let Some(other) = other.filter(|other| *other < layers.len()) else { return Err(invalid_value(property, value)) };
            layers.swap(at, other);
            "reorder-construction-layers"
        }
        _ if property == "replaceLayer" || property.starts_with("replaceLayer:") => {
            let (raw_index, raw_material) = match property.strip_prefix("replaceLayer:") {
                Some(at) => (at, value.trim()),
                None => value.trim().split_once(':').ok_or_else(|| invalid_value(property, value))?,
            };
            let at = slot(raw_index.trim(), &model.constructions[index].layer_material_ids)?;
            let material = layer_material(raw_material.trim())?;
            model.constructions[index].layer_material_ids[at] = material;
            "replace-construction-layer"
        }
        _ => return Err(unknown_property("construction", property)),
    })
}

/// 🧱️ Wire spelling of a [`crate::model::SurfaceRoughness`] — camelCase, like every other enum this
/// editor carries over the action bus.
pub fn surface_roughness_id(roughness: crate::model::SurfaceRoughness) -> &'static str {
    use crate::model::SurfaceRoughness as Kind;
    match roughness {
        Kind::VeryRough => "veryRough",
        Kind::Rough => "rough",
        Kind::MediumRough => "mediumRough",
        Kind::MediumSmooth => "mediumSmooth",
        Kind::Smooth => "smooth",
        Kind::VerySmooth => "verySmooth",
    }
}

fn surface_roughness_from_id(id: &str) -> Option<crate::model::SurfaceRoughness> {
    use crate::model::SurfaceRoughness as Kind;
    Some(match id {
        "veryRough" => Kind::VeryRough,
        "rough" => Kind::Rough,
        "mediumRough" => Kind::MediumRough,
        "mediumSmooth" => Kind::MediumSmooth,
        "smooth" => Kind::Smooth,
        "verySmooth" => Kind::VerySmooth,
        _ => return None,
    })
}

/// 🧱️ Every roughness class, in declaration order — the inspector's roughness select.
pub const SURFACE_ROUGHNESS_IDS: &[&str] = &["veryRough", "rough", "mediumRough", "mediumSmooth", "smooth", "verySmooth"];

//#endregion 🔖️Reduce

//#region 🧵️RetainedCommands
const ENERGY_MODEL_RETAINED_PAYLOAD_SCHEMA: &str = "energy.model.tool-command.v1";
const ENERGY_MODEL_RETAINED_RAW_BYTES: usize = 65_536;
/// 🧮️ One retained work item covers one whole-model edit; the extent guard below refuses a document
/// whose addressable entity count would exceed the bounded first step.
const ENERGY_MODEL_RETAINED_WORK_ITEMS: usize = 65_536;

fn energy_model_contract() -> ToolExecutionContract {
    ToolExecutionContract::bounded_first_step(ENERGY_MODEL_RETAINED_RAW_BYTES, ENERGY_MODEL_RETAINED_WORK_ITEMS, 1, 262_144, 7_500)
}

/// 🧮️ Refuses a first step whose document is larger than the declared retained extent, so an
/// oversized model is rejected at admission rather than truncated mid-edit.
fn energy_model_extent(command: &EnergyModelEditorCommand, snapshot: &EnergyModelSnapshot, _interaction: &protocol::InteractionState) -> Option<usize> {
    if !ENERGY_MODEL_RETAINED_TOOL_IDS.contains(&command.action_id()) {
        return None;
    }
    let model = &snapshot.model;
    let items = model.zones.len().saturating_add(model.spaces.len()).saturating_add(model.surfaces.len()).saturating_add(model.fenestrations.len()).saturating_add(model.materials.len()).saturating_add(model.constructions.len());
    (items <= ENERGY_MODEL_RETAINED_WORK_ITEMS).then_some(1)
}

#[allow(clippy::needless_pass_by_value)]
fn energy_model_reduce(
    command: &EnergyModelEditorCommand,
    snapshot: &EnergyModelSnapshot,
    _config: &EnergyModelConfig,
    history: &HistoryView,
    _interaction: &protocol::InteractionState,
    _hover: &semio_framework_plugin::app::InteractionHoverState,
    _context: Option<&semio_framework_plugin::app::ArtifactOwnedToolJobContext<EditorApp<EnergyModelEditor>>>,
    operation: &AppOperationContext,
) -> Result<Emit<EnergyModelMutation, EnergyModelConfigMutation, NoDraftMutation>, Fault> {
    if let Some(camera) = camera_emit(command, _context.and_then(|context| context.view_state.as_ref())) {
        return camera;
    }
    reduce(command, &ArtifactView::with_operation(snapshot, history, operation.clone()))
}

struct EnergyModelCommandJobFactory {
    keys: Vec<ToolFactoryKey>,
}

impl EnergyModelCommandJobFactory {
    fn new(controller_id: &str) -> Self {
        Self { keys: ENERGY_MODEL_RETAINED_TOOL_IDS.iter().map(|tool_id| ToolFactoryKey::new(controller_id, *tool_id)).collect() }
    }
}

impl semio_framework_plugin::ToolJobFactory for EnergyModelCommandJobFactory {
    type Payload = ArtifactRetainedCommandPayload<EditorApp<EnergyModelEditor>>;
    type Job = ArtifactRetainedCommandJob<EditorApp<EnergyModelEditor>>;

    fn keys(&self) -> &[ToolFactoryKey] {
        &self.keys
    }

    fn payload_schema_id(&self) -> &str {
        ENERGY_MODEL_RETAINED_PAYLOAD_SCHEMA
    }

    fn classification(&self) -> InteractiveJobClassification {
        InteractiveJobClassification::Migrated
    }

    fn execution_contract(&self) -> ToolExecutionContract {
        energy_model_contract()
    }

    fn create_job(&mut self, _operation: semio_framework_job::Operation, payload: Self::Payload) -> Result<Self::Job, ToolJobFactoryError> {
        Ok(ArtifactRetainedCommandJob::new(payload))
    }

    fn create_job_from_wire_pages_with_payload(
        &mut self,
        _operation: semio_framework_job::Operation,
        payload: Self::Payload,
        input: semio_framework_plugin::action_bus::RetainedToolWireInput,
        checkpoint: Option<semio_framework_plugin::action_bus::RetainedToolWireInput>,
    ) -> Result<Self::Job, (ToolJobFactoryError, semio_framework_plugin::action_bus::RetainedToolWireInput, Option<semio_framework_plugin::action_bus::RetainedToolWireInput>)> {
        if input.declared_bytes() > ENERGY_MODEL_RETAINED_RAW_BYTES || checkpoint.is_some() {
            return Err((ToolJobFactoryError::new("the bounded energy model command rejects an oversized wire or checkpoint owner"), input, checkpoint));
        }
        Ok(ArtifactRetainedCommandJob::from_wire(payload, input))
    }
}

impl semio_framework_plugin::ArtifactOwnedToolJobFactory for EnergyModelCommandJobFactory {
    type Owner = EditorApp<EnergyModelEditor>;
    const TOOL_IDS: &'static [&'static str] = ENERGY_MODEL_RETAINED_TOOL_IDS;
    const DOCUMENT_SCHEMA: &'static str = ENERGY_MODEL_DOCUMENT_SCHEMA;
    const PUBLICATION_CONTRACTS: &'static [ArtifactToolPublicationContract] = &[
        ArtifactToolPublicationContract { tool_id: SET_NODE_ACTION_ID, lanes: &[ArtifactToolPublicationLane::Artifact] },
        ArtifactToolPublicationContract { tool_id: SET_CELL_ACTION_ID, lanes: &[ArtifactToolPublicationLane::Artifact] },
        ArtifactToolPublicationContract { tool_id: CREATE_ZONE_ACTION_ID, lanes: &[ArtifactToolPublicationLane::Artifact] },
        ArtifactToolPublicationContract { tool_id: RENAME_ZONE_ACTION_ID, lanes: &[ArtifactToolPublicationLane::Artifact] },
        ArtifactToolPublicationContract { tool_id: DELETE_ZONE_ACTION_ID, lanes: &[ArtifactToolPublicationLane::Artifact] },
        ArtifactToolPublicationContract { tool_id: CREATE_SURFACE_ACTION_ID, lanes: &[ArtifactToolPublicationLane::Artifact] },
        ArtifactToolPublicationContract { tool_id: DELETE_SURFACE_ACTION_ID, lanes: &[ArtifactToolPublicationLane::Artifact] },
        ArtifactToolPublicationContract { tool_id: ASSIGN_SURFACE_CONSTRUCTION_ACTION_ID, lanes: &[ArtifactToolPublicationLane::Artifact] },
        ArtifactToolPublicationContract { tool_id: SET_MATERIAL_PROPERTY_ACTION_ID, lanes: &[ArtifactToolPublicationLane::Artifact] },
        ArtifactToolPublicationContract { tool_id: SET_THERMOSTAT_SETPOINTS_ACTION_ID, lanes: &[ArtifactToolPublicationLane::Artifact] },
        ArtifactToolPublicationContract { tool_id: SET_SITE_ACTION_ID, lanes: &[ArtifactToolPublicationLane::Artifact] },
        ArtifactToolPublicationContract { tool_id: SET_RUN_PERIOD_ACTION_ID, lanes: &[ArtifactToolPublicationLane::Artifact] },
        ArtifactToolPublicationContract { tool_id: SET_ACTIVE_EXAMPLE_ACTION_ID, lanes: &[ArtifactToolPublicationLane::HostOnly] },
        ArtifactToolPublicationContract { tool_id: simulation::SET_SETTINGS_ACTION_ID, lanes: &[ArtifactToolPublicationLane::Config] },
        ArtifactToolPublicationContract { tool_id: simulation::SET_RESULT_FIELD_ACTION_ID, lanes: &[ArtifactToolPublicationLane::Config] },
        // 🎥️ The camera writes ONE window instance's own retained state — never the document, never
        // the app config.
        ArtifactToolPublicationContract { tool_id: model_window::SET_CAMERA_ACTION_ID, lanes: &[ArtifactToolPublicationLane::WindowConfig] },
        ArtifactToolPublicationContract { tool_id: SET_SURFACE_PROPERTY_ACTION_ID, lanes: &[ArtifactToolPublicationLane::Artifact] },
        ArtifactToolPublicationContract { tool_id: SET_FENESTRATION_PROPERTY_ACTION_ID, lanes: &[ArtifactToolPublicationLane::Artifact] },
        ArtifactToolPublicationContract { tool_id: SET_ZONE_PROPERTY_ACTION_ID, lanes: &[ArtifactToolPublicationLane::Artifact] },
        ArtifactToolPublicationContract { tool_id: SET_GLAZING_MATERIAL_PROPERTY_ACTION_ID, lanes: &[ArtifactToolPublicationLane::Artifact] },
        ArtifactToolPublicationContract { tool_id: SET_GAS_MATERIAL_PROPERTY_ACTION_ID, lanes: &[ArtifactToolPublicationLane::Artifact] },
        ArtifactToolPublicationContract { tool_id: SET_CONSTRUCTION_PROPERTY_ACTION_ID, lanes: &[ArtifactToolPublicationLane::Artifact] },
    ];
}
//#endregion 🧵️RetainedCommands

/// 📬️ Upper bound of one encoded run-settings config record.
const ENERGY_MODEL_CONFIG_STORE_MAXIMUM_BYTES: usize = 4_096;

//#region 📬️StorePreparation
/// 📬️ The document lane's one-item retained preparation. Without it every verb declaring
/// `ArtifactToolPublicationLane::Artifact` is registered with an unsupported publication contract and
/// stays dispatch-dead, whatever its classification.
struct EnergyModelStorePreparationFactory;

struct EnergyModelStorePreparation {
    base: Option<store::SnapshotRead<EnergyModelSnapshot>>,
    mutation: Option<EnergyModelMutation>,
    description: Option<String>,
    authority: Option<std::sync::Arc<store::ArtifactStoreOneItemLiveAuthority>>,
    prepared: Option<store::ArtifactStoreOneItemPrepared<EnergyModelSnapshot, EnergyModelMutation>>,
    checkpoint: store::ArtifactStoreOneItemCheckpoint,
    cancelled: bool,
    closing: bool,
}

/// 🧾️ One retained document edit, authored exactly as the store's live authority describes it.
fn energy_model_retained_edit<M>(id: String, authority: &store::ArtifactStoreOneItemLiveAuthority, forwards: Vec<M>, inverse: Vec<M>, description: Option<String>) -> protocol::Edit<M> {
    let mutation_id = protocol::MutationId(format!("{id}#0"));
    protocol::Edit {
        id,
        actor: Some(authority.actor().to_string()),
        forwards,
        inverse,
        mutation_meta: vec![protocol::MutationMeta {
            mutation_id: Some(mutation_id),
            dependencies: Vec::new(),
            base_version: authority.base_applied_edit_count() as u64,
            author_id: Some(protocol::ActorId(authority.actor().to_string())),
            timestamp: authority.next_clock(),
            undo_policy: protocol::UndoPolicy::ExactBaseOnly,
            payload_hash: None,
            semantic_kind: None,
            label: None,
            group_id: None,
            origin: Default::default(),
        }],
        description,
        coalesce_key: None,
        sequence_number: authority.next_sequence_number(),
        started_at: String::new(),
        finished_at: None,
    }
}

impl store::ArtifactStoreOneItemPreparationFactory<EnergyModelSnapshot, EnergyModelMutation> for EnergyModelStorePreparationFactory {
    fn preflight(&self, _mutation: &EnergyModelMutation, description: Option<&str>, lane: store::HistoryLane) -> Result<store::ArtifactStoreOneItemFootprint, String> {
        if lane != store::HistoryLane::Document || description.is_some_and(|value| value.len() > store::ARTIFACT_STORE_ONE_ITEM_ID_BYTES) {
            return Err("the energy model store preparation rejected its lane or description envelope".into());
        }
        Ok(store::ArtifactStoreOneItemFootprint { work_items: 2, retained_bytes: store::ARTIFACT_STORE_ONE_ITEM_MAXIMUM_BYTES })
    }

    fn begin(
        &self,
        request: store::ArtifactStoreOneItemPreparationRequest<EnergyModelSnapshot, EnergyModelMutation>,
    ) -> Result<Box<dyn store::ArtifactStoreOneItemPreparation<EnergyModelSnapshot, EnergyModelMutation>>, store::ArtifactStoreOneItemPreparationRequest<EnergyModelSnapshot, EnergyModelMutation>> {
        let model = &request.base.get().model;
        let items = model.zones.len().saturating_add(model.spaces.len()).saturating_add(model.surfaces.len()).saturating_add(model.fenestrations.len()).saturating_add(model.materials.len()).saturating_add(model.constructions.len());
        if request.lane != store::HistoryLane::Document
            || request.operation != request.authority.operation()
            || request.generation != request.authority.generation()
            || request.base_revision != request.authority.base_revision()
            || request.authority.actor().len() > store::ARTIFACT_STORE_ONE_ITEM_ID_BYTES
            || items > ENERGY_MODEL_RETAINED_WORK_ITEMS
        {
            return Err(request);
        }
        Ok(Box::new(EnergyModelStorePreparation {
            base: Some(request.base),
            mutation: Some(request.mutation),
            description: request.description,
            authority: Some(request.authority),
            prepared: None,
            checkpoint: store::ArtifactStoreOneItemCheckpoint::default(),
            cancelled: false,
            closing: false,
        }))
    }
}

impl store::ArtifactStoreOneItemPreparation<EnergyModelSnapshot, EnergyModelMutation> for EnergyModelStorePreparation {
    fn advance(&mut self, grant: store::ArtifactStoreOneItemGrant) -> Result<store::ArtifactStoreOneItemPreparationStep, String> {
        use protocol::Mutation as _;
        if !grant.permits_one() || self.cancelled {
            return Ok(store::ArtifactStoreOneItemPreparationStep::Blocked);
        }
        if self.prepared.is_some() {
            return Ok(store::ArtifactStoreOneItemPreparationStep::Prepared(self.checkpoint));
        }
        let base = self.base.as_ref().ok_or_else(|| "the energy model preparation lost its exact base root".to_string())?;
        let mutation = self.mutation.take().ok_or_else(|| "the energy model preparation lost its mutation owner".to_string())?;
        let inverse = mutation.inverse(base.get());
        let post = protocol::MutationDiff::apply(mutation.diff(base.get()).diff(), base.get()).map_err(|error| error.to_string())?;
        let authority = self.authority.as_ref().ok_or_else(|| "the energy model preparation lost its store authority".to_string())?;
        let id = format!("energy-model-retained-{}", authority.next_sequence_number());
        let edit = energy_model_retained_edit(id, authority, vec![mutation], inverse, self.description.take());
        let prepared = authority.prepare_one_item(edit, std::sync::Arc::new(post))?;
        self.checkpoint = store::ArtifactStoreOneItemCheckpoint { cursor: 1, completed_items: 1, completed_bytes: 1, digest: prepared.edit_digest() };
        self.prepared = Some(prepared);
        Ok(store::ArtifactStoreOneItemPreparationStep::Prepared(self.checkpoint))
    }

    fn checkpoint(&self) -> store::ArtifactStoreOneItemCheckpoint {
        self.checkpoint
    }

    fn prepared(&self) -> Option<&store::ArtifactStoreOneItemPrepared<EnergyModelSnapshot, EnergyModelMutation>> {
        self.prepared.as_ref()
    }

    fn take_prepared(&mut self) -> Option<store::ArtifactStoreOneItemPrepared<EnergyModelSnapshot, EnergyModelMutation>> {
        self.prepared.take()
    }

    fn cancel(&mut self) {
        self.cancelled = true;
    }

    fn begin_close(&mut self) {
        self.closing = true;
    }

    fn close_step(&mut self, grant: store::ArtifactStoreOneItemGrant) -> Result<store::SnapshotRetirementStep, String> {
        if !self.closing || grant.maximum_items == 0 {
            return Ok(store::SnapshotRetirementStep::Pending { released_items: 0, released_bytes: 0 });
        }
        if self.prepared.take().is_some() || self.mutation.take().is_some() || self.description.take().is_some() {
            return Ok(store::SnapshotRetirementStep::Pending { released_items: 1, released_bytes: 0 });
        }
        if let Some(base) = self.base.take() {
            if !base.return_to_registry() {
                return Err("the energy model preparation could not return its exact base root".into());
            }
            return Ok(store::SnapshotRetirementStep::Pending { released_items: 1, released_bytes: 0 });
        }
        if let Some(authority) = self.authority.as_ref() {
            if grant.maximum_bytes < authority.actor().len() {
                return Ok(store::SnapshotRetirementStep::Blocked);
            }
            self.authority = None;
            return Ok(store::SnapshotRetirementStep::Pending { released_items: 1, released_bytes: 0 });
        }
        Ok(store::SnapshotRetirementStep::Complete)
    }

    fn terminal_is_empty(&self) -> bool {
        self.closing && self.base.is_none() && self.mutation.is_none() && self.description.is_none() && self.authority.is_none() && self.prepared.is_none()
    }
}
//#endregion 📬️StorePreparation


//#region 🔖️Editor
#[derive(Default, Clone, Copy)]
pub struct EnergyModelEditor;

impl ArtifactEditor for EnergyModelEditor {
    type Snapshot = EnergyModelSnapshot;
    type Mutation = EnergyModelMutation;
    type Config = EnergyModelConfig;
    type ConfigMutation = EnergyModelConfigMutation;
    type Draft = NoDraft;
    type DraftMutation = NoDraftMutation;
    type Presence = NoPresence;
    type PresenceMutation = NoPresenceMutation;
    type Transient = NoTransient;
    type TransientMutation = NoTransientMutation;
    type Command = EnergyModelEditorCommand;
    /// 🧩️ `structure`/`zones` are `s.stdio.semio@v1/{value,table}` members, so the roster that
    /// resolves their genesis dialects is stdio's closed `SemioMembers` (same as gis/sourcing).
    type Members = semio_s_artifact_stdio_semio::SemioMembers;

    const DIALECT: Dialect = MODEL_DIALECT;
    const DOCUMENT_SCHEMA: &'static str = ENERGY_MODEL_DOCUMENT_SCHEMA;

    fn child_restore_projection(snapshot: &Self::Snapshot) -> Result<store::ChildRestoreProjection<'_>, Fault> {
        crate::energy_child_restore_projection(snapshot)
    }

    /// 🌱️ Both composed children derive from the model — see `crate::energy_genesis_child_pack`.
    fn genesis_child_pack(snapshot: &Self::Snapshot, slot: &str, child_id: &str) -> Option<Vec<u8>> {
        crate::energy_genesis_child_pack(snapshot, slot, child_id)
    }

    semio_framework_plugin::bounded_first_step_tool_proofs! {
        owner: EditorApp<EnergyModelEditor>,
        owner_file: "✏️s/🔌️plugins/🔋️energy/🗿️artifacts/🔋️model/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs",
        controller: "s.energy.model@1/*#editor",
        artifact_schema: "energy.model",
        factory: "EnergyModelCommandJobFactory",
        factory_type: EnergyModelCommandJobFactory,
        contract: ToolExecutionContract::bounded_first_step(65_536, 65_536, 1, 262_144, 7_500),
        tools: [
            "set-node",
            "set-cell",
            "create-zone",
            "rename-zone",
            "delete-zone",
            "create-surface",
            "delete-surface",
            "assign-surface-construction",
            "set-material-property",
            "set-thermostat-setpoints",
            "set-site",
            "set-run-period",
            "setActiveExample",
            "set-simulation-settings",
            "set-result-field",
            "set-surface-property",
            "set-fenestration-property",
            "set-zone-property",
            "set-glazing-material-property",
            "set-gas-material-property",
            "setCamera",
            "set-construction-property"
        ]
    }

    /// 🎥️ The 3d window's own retained orbit pose — one bounded window-config store, keyed by window
    /// INSTANCE, so two open 3d panes never share a camera.
    fn register_window_config_owners(registry: &mut semio_framework_plugin::WindowConfigOwnerRegistry) -> Result<(), Fault> {
        registry.register::<model_window::config::EnergyModelWindowConfigOwner>()
    }

    fn register_tool_job_factories(registry: &mut ArtifactToolFactoryRegistry<'_, EditorApp<Self>>) -> Result<(), Fault> {
        let controller = registry.controller_id().to_string();
        registry.register(EnergyModelCommandJobFactory::new(&controller))
    }

    fn build_tool_job(request: ArtifactOwnedToolJobRequest<EditorApp<Self>>) -> Result<Option<semio_framework_plugin::ToolOperationSpec>, Fault> {
        if !ENERGY_MODEL_RETAINED_TOOL_IDS.contains(&request.tool_id.as_str()) {
            return Ok(None);
        }
        if request.command.action_id() != request.tool_id {
            return Err(Fault::new(FaultOrigin::App, FaultCode::new("energy.model.retained.tool-mismatch"), "the energy model command does not match its exact registered tool"));
        }
        if energy_model_extent(&request.command, &request.snapshot, &request.interaction_state).is_none() {
            return Err(Fault::new(FaultOrigin::App, FaultCode::new("energy.model.retained.extent"), "the energy model bounded route exceeded its declared work extent"));
        }
        let tool_id = request.command.action_id();
        let work = Box::new(BoundedArtifactCommandWork::new(tool_id, energy_model_reduce, energy_model_extent));
        let operation_context = AppOperationContext {
            app_instance_id: request.app_instance_id,
            parent_document_id: request.parent_document_id,
            operation_id: request.operation.operation.0,
            generation: request.operation.generation.0,
            canonical_base_revision: request.canonical_base_revision,
        };
        let payload = ArtifactRetainedCommandPayload::try_new(
            semio_framework_plugin::retained_command::ArtifactRetainedCommandInputs {
                command: *request.command,
                snapshot: request.snapshot,
                config: request.config,
                history: request.history,
                interaction_state: request.interaction_state,
                interaction_hover: request.interaction_hover,
                context: Some(request.context),
                operation: operation_context,
                completion: request.completion,
            },
            EnergyModelEditorCommand::action_id,
            ENERGY_MODEL_RETAINED_RAW_BYTES,
            1,
            work,
        )?;
        Ok(Some(semio_framework_plugin::ToolOperationSpec::new(request.controller_id, request.tool_id, request.payload_schema_id, payload, request.operation)))
    }

    fn build_document_store_owners() -> Option<store::DocumentStoreOwners<Self::Snapshot, Self::Mutation>> {
        Some(semio_framework_plugin::bounded_document_store_owners::<Self::Snapshot, Self::Mutation>())
    }

    /// 🌱️ The bounded initialization twin of the bounded owners above (same pairing as fem/sourcing).
    /// Without it `begin_persisted_document_store_replacement` refuses every whole-document load with
    /// `artifact-store.persisted-initializer-refused` — which is exactly how `setActiveExample`
    /// (`Effect::LoadDocument` → the host's `loadDocumentArchive`) failed in the react shell.
    fn build_document_store_initialization_job(
        envelope: store::ArtifactEnvelope<Self::Snapshot, Self::Mutation>,
        operation: semio_framework_job::OperationId,
        generation: semio_framework_job::Generation,
    ) -> Result<semio_framework_plugin::ArtifactStoreInitializationJob<Self::Snapshot, Self::Mutation>, store::ArtifactEnvelope<Self::Snapshot, Self::Mutation>> {
        Ok(semio_framework_plugin::bounded_document_store_initialization_job(envelope, ENERGY_MODEL_DOCUMENT_SCHEMA, operation, generation))
    }

    fn build_document_store_disposer() -> Option<Box<dyn semio_framework_plugin::ArtifactOwnedDisposer<store::ArtifactStore<Self::Snapshot, Self::Mutation>>>> {
        Some(semio_framework_plugin::bounded_document_store_disposer::<Self::Snapshot, Self::Mutation>())
    }

    fn build_config_store_owners() -> Option<store::DocumentStoreOwners<Self::Config, Self::ConfigMutation>> {
        Some(semio_framework_plugin::bounded_config_store_owners::<Self::Config, Self::ConfigMutation>())
    }

    fn build_config_store_disposer() -> Option<Box<dyn semio_framework_plugin::ArtifactOwnedDisposer<store::ConfigStore<Self::Config, Self::ConfigMutation>>>> {
        Some(semio_framework_plugin::bounded_config_store_disposer::<Self::Config, Self::ConfigMutation>())
    }

    fn build_draft_store_owners() -> Option<store::DocumentStoreOwners<Self::Draft, Self::DraftMutation>> {
        Some(semio_framework_plugin::no_draft_store_owners())
    }

    fn build_draft_store_disposer() -> Option<Box<dyn semio_framework_plugin::ArtifactOwnedDisposer<store::DraftStore<Self::Draft, Self::DraftMutation>>>> {
        Some(semio_framework_plugin::no_draft_store_disposer())
    }

    fn build_presence_store_disposer() -> Option<Box<dyn semio_framework_plugin::ArtifactOwnedDisposer<store::PresenceStore<Self::Presence, Self::PresenceMutation>>>> {
        Some(semio_framework_plugin::no_presence_store_disposer())
    }

    fn build_presence_local_root_retirement_factory() -> Option<std::sync::Arc<dyn store::SnapshotRetirementFactory<Self::Presence>>> {
        Some(semio_framework_plugin::no_presence_local_root_retirement_factory())
    }

    fn build_presence_peer_retirement_factory() -> Option<std::sync::Arc<dyn store::SnapshotRetirementFactory<Self::Presence>>> {
        Some(semio_framework_plugin::no_presence_peer_retirement_factory())
    }

    fn build_transient_store_disposer() -> Option<Box<dyn semio_framework_plugin::ArtifactOwnedDisposer<store::TransientStore<Self::Transient, Self::TransientMutation>>>> {
        Some(semio_framework_plugin::no_transient_store_disposer())
    }

    fn build_transient_local_root_retirement_factory() -> Option<std::sync::Arc<dyn store::SnapshotRetirementFactory<Self::Transient>>> {
        Some(semio_framework_plugin::no_transient_local_root_retirement_factory())
    }

    fn build_artifact_store_one_item_preparation_factory() -> Option<std::sync::Arc<dyn store::ArtifactStoreOneItemPreparationFactory<Self::Snapshot, Self::Mutation>>> {
        Some(std::sync::Arc::new(EnergyModelStorePreparationFactory))
    }

    fn build_config_store_one_item_preparation_factory() -> Option<std::sync::Arc<dyn store::ArtifactStoreOneItemPreparationFactory<Self::Config, Self::ConfigMutation>>> {
        Some(semio_framework_plugin::bounded_config_store_one_item_preparation_factory::<Self::Config, Self::ConfigMutation>("energy-model-config-retained", ENERGY_MODEL_CONFIG_STORE_MAXIMUM_BYTES))
    }

    /// ⏯️ The `energySimulation` run job over the run's base snapshot and the config store's settings.
    /// The run is read-only and declares no `revalidateJob`, so only the `Run` purpose builds a job.
    fn build_tool_run_job(request: ToolRunJobRequest<'_, EditorApp<Self>>) -> Result<Option<ToolRunJob>, Fault> {
        if request.tool_id != tools::simulation::TOOL_ID || request.purpose != ToolRunJobPurpose::Run {
            return Ok(None);
        }
        Ok(Some(Box::new(EnergySimulationRunJob::new(request.identity, request.snapshot, request.config.simulation_template()))))
    }

    fn initial_snapshot() -> EnergyModelSnapshot {
        EnergyModelSnapshot::default()
    }

    fn command_id(command: &EnergyModelEditorCommand) -> &'static str {
        command.action_id()
    }

    fn command_from_action(action: &str, args: Option<&dsl::DslValue>) -> Result<Self::Command, Fault> {
        args_bridge::command_from_action(action, args)
    }

    /// ✏️ Delegates to the same pure [`reduce`] the retained bounded work step runs, so the
    /// interactive and retained routes can never diverge.
    fn handle(
        command: &Self::Command,
        doc: &ArtifactView<'_, Self::Snapshot>,
        _cfg: &ConfigView<'_, Self::Config>,
        _interaction: &semio_framework_plugin::app::InteractionView<'_>,
        _view_state: Option<&semio_framework_plugin::ViewModel>,
        _draft: &DraftView<'_, Self::Draft>,
        _engines: &EngineHandles,
    ) -> Result<Emit<Self::Mutation, Self::ConfigMutation, Self::DraftMutation>, Fault> {
        if let Some(camera) = camera_emit(command, _view_state) {
            return camera;
        }
        reduce(command, doc)
    }

    /// 🎨️ The plain render path: no request context, so no live interaction domain to read. The 3d
    /// model window still renders — it simply paints nothing as selected or hovered.
    fn render(body_key: &str, doc: &ArtifactView<'_, Self::Snapshot>, cfg: &ConfigView<'_, Self::Config>, view_state: &semio_framework_plugin::ViewModel) -> UiAssemblyResult<ComponentTree> {
        render_body(body_key, doc, cfg, view_state, &EnergyModelInteractionSnapshot::default())
    }

    /// 🕹️ The request-context render: resolves the framework-owned `energyModel` domain ONCE and
    /// threads its selection/hover into the whole body, so the 3d window's paint and (later) the
    /// inspector read one authority instead of either surface keeping selection of its own.
    fn render_with_request_context(
        _owner: &semio_framework_plugin::ArtifactInstanceOperationOwnerHandle,
        body_key: &str,
        doc: &ArtifactView<'_, Self::Snapshot>,
        cfg: &ConfigView<'_, Self::Config>,
        view_state: &semio_framework_plugin::ViewModel,
        _transient: &semio_framework_plugin::TransientView<'_, Self::Transient>,
        interaction: &semio_framework_plugin::app::InteractionView<'_>,
    ) -> UiAssemblyResult<ComponentTree> {
        render_body(body_key, doc, cfg, view_state, &EnergyModelInteractionSnapshot::from_interaction(interaction))
    }
}

/// 🎨️ The one body dispatch both render entry points share, so the plain and the request-context
/// routes can never diverge.
fn render_body(
    body_key: &str,
    doc: &ArtifactView<'_, EnergyModelSnapshot>,
    cfg: &ConfigView<'_, EnergyModelConfig>,
    view_state: &semio_framework_plugin::ViewModel,
    interaction: &EnergyModelInteractionSnapshot,
) -> UiAssemblyResult<ComponentTree> {
    let node = match body_key {
        structure::BODY_KEY => structure::render(doc.snapshot)?,
        zones::BODY_KEY => zones::render(doc.snapshot)?,
        simulation::BODY_KEY => simulation::render(doc.tool_run(), cfg.snapshot, &doc.snapshot.model, view_state.locale),
        model_window::BODY_KEY => {
            // 🎨️ Results mode: while a finished (or ticking) energy simulation run carries a per-surface
            // payload, its chosen field paints the surfaces and the legend rides as the caption.
            let energy = crate::editor::model::results::surface_energy_from_run(doc.tool_run());
            let field = crate::editor::model::results::result_field(cfg.snapshot);
            let painted = energy.as_ref().map(|map| crate::editor::model::results::surface_colors(map, field));
            let caption = painted.as_ref().map(|(_, min, max)| crate::editor::model::results::legend_caption(field, *min, *max));
            // 🎥️ `config::current` is the addressed window's retained orbit pose, `None` until it has
            // been moved — an unmoved window keeps the model-derived camera and `fit_json`'s framing.
            // 🎨️ The same `(min, max)` the ramp was normalized over also labels the legend strip's two
            // ends, so the caption line and the swatch strip can never round differently.
            model_window::render_with_legend(
                &crate::energy_model(doc.snapshot),
                interaction,
                painted.as_ref().map(|(colors, _, _)| colors),
                caption.as_deref(),
                painted.as_ref().map(|(_, min, max)| (*min, *max)),
                model_window::config::current(cfg),
            )?
        }
        // 🪟️ The host's open/scroll state for THIS body, read once per render — every container of
        // the outliner materialises exactly the slice it names and stamps its own full `total`.
        artifact_panel::BODY_KEY => artifact_panel::render(doc.snapshot, interaction, view_state.locale, &semio_framework_plugin::TreeWindows::for_body(view_state, artifact_panel::BODY_KEY))?,
        // 🎨️ `cfg` reaches the inspector because its Results section renders the CURRENT colour field
        // and binds `set-result-field` — the one control lane D's selector had nowhere to live.
        inspection_panel::BODY_KEY => inspection_panel::render(doc.snapshot, interaction, cfg.snapshot, view_state.locale, &semio_framework_plugin::TreeWindows::for_body(view_state, inspection_panel::BODY_KEY))?,
        _ => semio_framework_plugin::built_text_node(Label::data(format!("Unknown body: {body_key}"))).map_err(|_| semio_framework_plugin::PluginAssemblyError::new("energy.model.render", "the unknown-body label could not be assembled"))?,
    };
    Ok(semio_framework_plugin::built_to_component_tree(node))
}
//#endregion 🔖️Editor

//#region 📚️Examples
/// 📚️ The bundled `📚️examples/🎬️<slug>` fixtures, table-driven — a new example is ONE row here plus
/// its `#[path]` mount in `📦️packages/🦀️rust/🦀️.rs`. Every row names a module that exports the same
/// three items (`ID`, `LABEL_EN`, `model()`), so the table cannot go out of step with a leaf.
/// The ANSI/ASHRAE 140 case ids are proper nouns and read identically in both authored languages;
/// `Demo` likewise. That is a real translation decision, not a missing one.
fn example_rows() -> Vec<(&'static str, &'static str, crate::model::Model)> {
    use crate::examples::{bestest_600, bestest_600ff, bestest_610, bestest_620, bestest_630, bestest_640, bestest_650, bestest_900, bestest_900ff, bestest_910, bestest_920, bestest_930, bestest_940, bestest_950, demo};
    vec![
        (demo::ID, demo::LABEL_EN, demo::model()),
        (bestest_600::ID, bestest_600::LABEL_EN, bestest_600::model()),
        (bestest_600ff::ID, bestest_600ff::LABEL_EN, bestest_600ff::model()),
        (bestest_610::ID, bestest_610::LABEL_EN, bestest_610::model()),
        (bestest_620::ID, bestest_620::LABEL_EN, bestest_620::model()),
        (bestest_630::ID, bestest_630::LABEL_EN, bestest_630::model()),
        (bestest_640::ID, bestest_640::LABEL_EN, bestest_640::model()),
        (bestest_650::ID, bestest_650::LABEL_EN, bestest_650::model()),
        (bestest_900::ID, bestest_900::LABEL_EN, bestest_900::model()),
        (bestest_900ff::ID, bestest_900ff::LABEL_EN, bestest_900ff::model()),
        (bestest_910::ID, bestest_910::LABEL_EN, bestest_910::model()),
        (bestest_920::ID, bestest_920::LABEL_EN, bestest_920::model()),
        (bestest_930::ID, bestest_930::LABEL_EN, bestest_930::model()),
        (bestest_940::ID, bestest_940::LABEL_EN, bestest_940::model()),
        (bestest_950::ID, bestest_950::LABEL_EN, bestest_950::model()),
    ]
}

/// 📚️ The manifest rows the plugin root registers through
/// `.editor_with_examples::<EnergyModelEditor>(create_energy_model_editor(), examples())`. That
/// second argument is the ONLY thing that fills `PluginManifest.examples`
/// (`PluginBuilder::editor` hardcodes an empty set), and the react shell's picker is hidden while
/// that list is empty.
pub fn examples() -> Vec<ExampleSource> {
    example_rows()
        .into_iter()
        .map(|(id, label, model)| {
            let snapshot = crate::energy_snapshot_with_state(ENERGY_MODEL_DOCUMENT_SCHEMA, &model, None);
            ExampleSource::new(id, LocalizedLabel::native(label, label), pack::json::to_json_string(&snapshot), "file")
        })
        .collect()
}

/// 📚️ The model behind one bundled example id — the read `setActiveExample` reduces against.
fn example_model(example_id: &str) -> Option<crate::model::Model> {
    example_rows().into_iter().find(|(id, _, _)| *id == example_id).map(|(_, _, model)| model)
}

/// 📚️ The picker's option list, in `example_rows` order.
fn example_options() -> Vec<semio_framework_plugin::ActionArgOption> {
    example_rows().into_iter().map(|(id, label, _)| semio_framework_plugin::ActionArgOption { value: id.to_string(), label: LocalizedLabel::native(label, label) }).collect()
}
//#endregion 📚️Examples

//#region 🔖️UiHelpers
/// 🎛️ The controller id every panel row and inspector control mints its action under — the canonical
/// surface id of this editor (`semio_framework::surface_app_id(MODEL_DIALECT, AppRole::Editor)`),
/// the same spelling `bounded_first_step_tool_proofs!` declares above.
pub const ENERGY_MODEL_EDITOR_CONTROLLER_ID: &str = "s.energy.model@1/*#editor";

/// 🏷️ Admits resolved energy text into the semantic UI contract.
pub fn ui_label(value: impl AsRef<str>) -> UiAssemblyResult<semio_framework_plugin::plugin_app_close_prelude::Label> {
    semio_framework_plugin::plugin_app_close_prelude::Label::try_from(value.as_ref()).map_err(|_| semio_framework_plugin::PluginAssemblyError::new("ui.fixed-capacity", "energy UI label admission failed"))
}

/// 🎛️ Mints one energy-editor action for a panel row or an inspector control binding.
pub fn energy_model_action(action: &str, args: Option<semio_framework_plugin::UiValue>) -> UiAssemblyResult<(semio_framework_plugin::ActionId, Option<semio_framework_plugin::UiValue>)> {
    semio_framework_plugin::ActionFactory::new(ENERGY_MODEL_EDITOR_CONTROLLER_ID).action(action, args)
}

/// 🟫️ Wire spelling of a [`SurfaceClass`] — the inverse of [`surface_class_from_id`], so the
/// inspector's class select offers exactly the vocabulary `create-surface`/`set-surface-property`
/// accept.
pub fn surface_class_id(class: SurfaceClass) -> &'static str {
    match class {
        SurfaceClass::ExteriorWall => "exteriorWall",
        SurfaceClass::InteriorWall => "interiorWall",
        SurfaceClass::Roof => "roof",
        SurfaceClass::Ceiling => "ceiling",
        SurfaceClass::Floor => "floor",
        SurfaceClass::Interzone => "interzone",
        SurfaceClass::Adiabatic => "adiabatic",
        SurfaceClass::Ground => "ground",
    }
}

/// 🟫️ Every surface class, in declaration order — the inspector's class select.
pub const SURFACE_CLASS_IDS: &[&str] = &["exteriorWall", "interiorWall", "roof", "ceiling", "floor", "interzone", "adiabatic", "ground"];

/// 🚧️ Wire spelling of an [`crate::model::OutsideBoundaryKind`].
pub fn outside_boundary_kind_id(kind: crate::model::OutsideBoundaryKind) -> &'static str {
    use crate::model::OutsideBoundaryKind as Kind;
    match kind {
        Kind::OutdoorAir => "outdoorAir",
        Kind::Ground => "ground",
        Kind::OtherSideTemperature => "otherSideTemperature",
        Kind::Adiabatic => "adiabatic",
        Kind::Interzone => "interzone",
    }
}

/// 🚧️ Every boundary kind, in declaration order — the inspector's boundary select.
pub const OUTSIDE_BOUNDARY_KIND_IDS: &[&str] = &["outdoorAir", "ground", "otherSideTemperature", "adiabatic", "interzone"];
//#endregion 🔖️UiHelpers

//#region 🔍️InspectorActions
/// 🎬️ One inspector-dispatchable verb, declared APP-level rather than on a window kind.
///
/// ⚠️ This placement is load-bearing, not a style choice. A panel action is dispatched in the
/// ACTIVE WINDOW's context, and `AppBuilder::build_definition` copies an app-level action onto every
/// window kind — but SKIPS any id that some window already declares itself
/// (`explicitly_owned_action_ids`). So the moment one of these verbs is listed in
/// `structure::actions()` or `zones::actions()`, it belongs to that window ALONE and the shell
/// refuses it from anywhere else with `window kind energy.model.3d does not own action
/// set-fenestration-property` — exactly the defect the first browser probe of this ticket hit.
fn inspector_action(id: &str, en: &str, de: &str, args: Vec<semio_framework_plugin::ActionArgDef>) -> semio_framework_plugin::ActionDefinition {
    semio_framework_plugin::ActionDefinition::bounded_catalog(id, LocalizedLabel::native(en, de), semio_framework_plugin::ActionKind::Mutation).with_args(args)
}

/// 🔍️ Every verb the inspection panel can dispatch, in one place. They must be reachable while ANY
/// window kind is active — the structure tree, the zones table, the simulation window and the 3d
/// model window alike.
pub fn inspector_action_definitions() -> Vec<semio_framework_plugin::ActionDefinition> {
    use semio_framework_plugin::ActionArgDef as Arg;
    let property = || Arg::text("property", LocalizedLabel::native("Property", "Eigenschaft")).required();
    let text_value = || Arg::text("value", LocalizedLabel::native("Value", "Wert")).required();
    vec![
        inspector_action(
            SET_SURFACE_PROPERTY_ACTION_ID,
            "Set surface property",
            "Flächeneigenschaft setzen",
            vec![
                Arg::number("surface", LocalizedLabel::native("Surface id", "Flächen-Id")).required(),
                property(),
                text_value(),
                Arg::number("partnerSurface", LocalizedLabel::native("Interzone partner surface", "Nachbarfläche")),
            ],
        ),
        inspector_action(
            SET_FENESTRATION_PROPERTY_ACTION_ID,
            "Set window property",
            "Fenstereigenschaft setzen",
            vec![Arg::number("fenestration", LocalizedLabel::native("Window id", "Fenster-Id")).required(), property(), text_value()],
        ),
        inspector_action(SET_ZONE_PROPERTY_ACTION_ID, "Set zone property", "Zoneneigenschaft setzen", vec![Arg::number("zone", LocalizedLabel::native("Zone id", "Zonen-Id")).required(), property(), text_value()]),
        inspector_action(
            SET_GLAZING_MATERIAL_PROPERTY_ACTION_ID,
            "Set glazing material property",
            "Verglasungsmaterial-Eigenschaft setzen",
            vec![Arg::number("material", LocalizedLabel::native("Glazing material id", "Verglasungsmaterial-Id")).required(), property(), text_value()],
        ),
        inspector_action(
            SET_GAS_MATERIAL_PROPERTY_ACTION_ID,
            "Set gas gap property",
            "Gasfüllungs-Eigenschaft setzen",
            vec![Arg::number("material", LocalizedLabel::native("Gas gap id", "Gasfüllungs-Id")).required(), property(), text_value()],
        ),
        inspector_action(
            SET_MATERIAL_PROPERTY_ACTION_ID,
            "Set material property",
            "Materialeigenschaft setzen",
            vec![Arg::number("material", LocalizedLabel::native("Material id", "Material-Id")).required(), property(), Arg::number("value", LocalizedLabel::native("Value", "Wert")).required()],
        ),
        inspector_action(
            SET_THERMOSTAT_SETPOINTS_ACTION_ID,
            "Set thermostat setpoints",
            "Thermostat-Sollwerte setzen",
            vec![
                Arg::number("thermostat", LocalizedLabel::native("Thermostat id", "Thermostat-Id")).required(),
                Arg::number("heatingSchedule", LocalizedLabel::native("Heating setpoint schedule", "Heiz-Sollwertprofil")).required(),
                Arg::number("coolingSchedule", LocalizedLabel::native("Cooling setpoint schedule", "Kühl-Sollwertprofil")).required(),
                Arg::number("heatingThrottleRangeK", LocalizedLabel::native("Heating throttle range (K)", "Heiz-Regelbereich (K)")),
                Arg::number("coolingThrottleRangeK", LocalizedLabel::native("Cooling throttle range (K)", "Kühl-Regelbereich (K)")),
            ],
        ),
        inspector_action(
            SET_SITE_ACTION_ID,
            "Set site",
            "Standort setzen",
            vec![
                Arg::slider("latitudeDeg", LocalizedLabel::native("Latitude (°)", "Breitengrad (°)"), -90.0, 90.0).required(),
                Arg::slider("longitudeDeg", LocalizedLabel::native("Longitude (°)", "Längengrad (°)"), -180.0, 180.0).required(),
                Arg::number("elevationM", LocalizedLabel::native("Elevation (m)", "Höhe (m)")),
                Arg::number("timeZoneHours", LocalizedLabel::native("Time zone (h)", "Zeitzone (h)")),
                Arg::number("northAxisDeg", LocalizedLabel::native("North axis (°)", "Nordachse (°)")),
            ],
        ),
        inspector_action(DELETE_ZONE_ACTION_ID, "Delete zone", "Zone löschen", vec![Arg::number("zone", LocalizedLabel::native("Zone id", "Zonen-Id")).required()]),
        inspector_action(DELETE_SURFACE_ACTION_ID, "Delete surface", "Fläche löschen", vec![Arg::number("surface", LocalizedLabel::native("Surface id", "Flächen-Id")).required()]),
        inspector_action(
            SET_CONSTRUCTION_PROPERTY_ACTION_ID,
            "Set construction property",
            "Konstruktionseigenschaft setzen",
            vec![Arg::number("construction", LocalizedLabel::native("Construction id", "Konstruktions-Id")).required(), property(), text_value()],
        ),
    ]
}

/// 🪟️ The verbs a WINDOW renders but any window may have to dispatch — the simulation window's three
/// config/document verbs (its settings tree and the inspector's own result-field select both reach
/// `set-result-field`), plus the two creation verbs the app keybindings `mod+shift+n`/`mod+shift+s`
/// fire. They are declared app-level for exactly the reason [`inspector_action_definitions`] is: an
/// action a window kind lists in its own `actions` is copied onto NO other window, so the chord is
/// refused with `window kind … does not own action …` whenever another window holds focus.
pub fn window_shared_action_definitions() -> Vec<semio_framework_plugin::ActionDefinition> {
    vec![simulation::settings_action(), simulation::result_field_action(), simulation::run_period_action(), structure::create_surface_action(), zones::create_zone_action(), zones::rename_zone_action()]
}

/// 🎬️ Every action this editor declares APP-level, in one roster: `build_definition` copies each of
/// them onto every window kind, which is what makes a panel control and a keybinding dispatchable
/// whatever window is active. `setActiveExample` is app-level too, through `.mutation(…)`.
pub fn app_level_action_definitions() -> Vec<semio_framework_plugin::ActionDefinition> {
    let mut actions = inspector_action_definitions();
    actions.extend(window_shared_action_definitions());
    actions
}
//#endregion 🔍️InspectorActions

//#region 🔖️Manifest
/// 🧱️ The editor's `AppDefinition`. Every id in [`ENERGY_MODEL_RETAINED_TOOL_IDS`] is classified
/// `Migrated` here — `set-node`/`set-cell` are already stamped by their kits, the other twelve are
/// classified explicitly, and `EditorBuilder::try_build_definition` panics on any `Unclassified` id.
/// The `energySimulation` tool declares its run, so the framework injects the reserved `toolRun*`
/// actions and binds their chords; the editor binds none of its own for the simulation.
/// Each window owns its own action list (`structure::actions`/`zones::actions`/the simulation
/// window's `definition`), so this function never restates one.
pub fn create_energy_model_editor() -> semio_framework_plugin::AppDefinition {
    let mut builder = Editor::builder(MODEL_DIALECT)
        .document(["semio", "energy", "model"])
        .artifact_kind(crate::artifact_kind())
        .terminology("reuse")
        .terminology_document("reuse", ["Entwerfen mit Bestand", "Energie"])
        .icon_id("battery")
        .mode_def(edit::definition())
        .default_mode_id(edit::ENERGY_MODEL_EDIT_MODE_ID)
        .window_kind_def(structure::definition())
        .window_kind_def(zones::definition())
        .window_kind_def(simulation::definition())
        .window_kind_def(model_window::definition())
        // 🕹️ The one framework-owned selection/hover domain the tree panel, the inspector and the 3d
        // model window share (`✏️editor/🕹️interaction/🦀️.rs`).
        .interaction(crate::editor::model::interaction::energy_model_interaction_definition())
        // 🧊️ Binding the 3d window to that domain is what lets the react `World3dHost` dispatch the
        // framework-reserved `interactionSelect`/`interactionHover` on its own pick — no plugin
        // pointer command exists, and none is needed.
        .window_kind_interactions(model_window::WINDOW_KIND_ID, vec![semio_framework_plugin::InteractionRef::new(ENERGY_MODEL_INTERACTION_DOMAIN)])
        // 📌️ The two dock panels: the artifact tree marks and picks into that same domain, the
        // inspector edits whatever it resolves to.
        .panel_tab_def(artifact_panel::definition())
        .panel_tab_def(inspection_panel::definition())
        .tool(tools::simulation::definition())
        .mutation(SET_ACTIVE_EXAMPLE_ACTION_ID, LocalizedLabel::native("Load example", "Beispiel laden"))
        .action_args(SET_ACTIVE_EXAMPLE_ACTION_ID, vec![semio_framework_plugin::ActionArgDef::select("exampleId", LocalizedLabel::native("Example", "Beispiel"), example_options()).required()])
        .keybinding("mod+shift+n", CREATE_ZONE_ACTION_ID)
        .keybinding("mod+shift+s", CREATE_SURFACE_ACTION_ID)
        .keybinding("mod+shift+g", SET_SITE_ACTION_ID)
        .default_layout(edit::layout());
    // 🔍️ App-level, NOT per-window: `build_definition` copies these onto every window kind, which is
    // what makes an inspector control dispatchable while the 3d window (or any other) is active.
    for action in app_level_action_definitions() {
        builder = builder.action_with(action);
    }
    for tool_id in ENERGY_MODEL_RETAINED_TOOL_IDS {
        builder = builder.action_interactive_job(*tool_id, InteractiveJobClassification::Migrated);
    }
    // ⚠️ Declared after every action is on the builder: `action_destructive` rewrites an
    // already-declared id, and the two inspector deletes only arrive with the loop above.
    for destructive_id in ENERGY_MODEL_DESTRUCTIVE_ACTION_IDS {
        builder = builder.action_destructive(*destructive_id);
    }
    builder.build_definition()
}
//#endregion 🔖️Manifest

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
