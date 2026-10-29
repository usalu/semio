//! 🎮️ Energy model editor commands: the action ids every window and panel dispatches, `EnergyModelEditorCommand`
//! with its action bridge and op codecs, the inspector's action definitions and the UI id helpers.

#![allow(async_fn_in_trait)]
#![allow(clippy::too_many_arguments)]

extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_s_plugin_energy_engine_model as model;

use semio_framework_diagnostic::{Fault, FaultCode, FaultOrigin};
use semio_framework_ui_locale::LocalizedLabel;
use semio_framework_value_derive::{FromValue as FromValueDerive, ToValue as ToValueDerive};
use crate::model::SurfaceClass;
use semio_framework_plugin_interaction::UiAssemblyResult;

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
pub const SET_LOAD_PROPERTY_ACTION_ID: &str = "set-load-property";
pub const SET_SHADING_PROPERTY_ACTION_ID: &str = "set-shading-property";
pub const SET_HVAC_PROPERTY_ACTION_ID: &str = "set-hvac-property";
pub const SET_SPACE_PROPERTY_ACTION_ID: &str = "set-space-property";
pub const SET_GAS_MATERIAL_PROPERTY_ACTION_ID: &str = "set-gas-material-property";
pub const SET_SIMULATION_SETTINGS_ACTION_ID: &str = "set-simulation-settings";
pub const SET_RESULT_FIELD_ACTION_ID: &str = "set-result-field";
pub const SET_CAMERA_ACTION_ID: &str = "setCamera";
/// 🧱️ The construction verb: `{construction, property, value}` where `property` is either `name` or
/// one of the LIST edits `addLayer`/`removeLayer`/`moveLayerUp`/`moveLayerDown`/`replaceLayer`, whose
/// operand travels in `value` (a material id, or an index, or `<index>:<materialId>`). One verb,
/// because the inspector's layer controls all carry a single merged `value`.
pub const SET_CONSTRUCTION_PROPERTY_ACTION_ID: &str = "set-construction-property";
pub const SET_THERMOSTAT_SETPOINTS_ACTION_ID: &str = "set-thermostat-setpoints";
pub const SET_SITE_ACTION_ID: &str = "set-site";
pub const SET_RUN_PERIOD_ACTION_ID: &str = "set-run-period";
pub const SELECT_WEATHER_FILE_ACTION_ID: &str = "select-weather-file";
pub const CLEAR_WEATHER_FILE_ACTION_ID: &str = "clear-weather-file";
/// 📚️ Framework-fixed id: the react shell's example picker and the wgpu shell both dispatch exactly
/// `setActiveExample` with `{ exampleId }` (`🏛️ShellHost/🟦️.tsx` `dispatchActiveExample`,
/// `🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs:6481`). It is camelCase, unlike this editor's own kebab verbs,
/// because it is a shell contract rather than an authored name.
pub const SET_ACTIVE_EXAMPLE_ACTION_ID: &str = "setActiveExample";

/// 🧵️ The complete retained-tool roster. Order is the declaration order of
/// [`EnergyModelEditorCommand`] and is asserted equal to it, to the manifest's classification list
/// and to the proof rows by `retained_roster_is_exact_and_exhaustive`.
pub const ENERGY_MODEL_AUTHORED_TOOL_IDS: &[&str] = &[
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
    SET_SIMULATION_SETTINGS_ACTION_ID,
    SET_RESULT_FIELD_ACTION_ID,
    SET_SURFACE_PROPERTY_ACTION_ID,
    SET_FENESTRATION_PROPERTY_ACTION_ID,
    SET_ZONE_PROPERTY_ACTION_ID,
    SET_GLAZING_MATERIAL_PROPERTY_ACTION_ID,
    SET_LOAD_PROPERTY_ACTION_ID,
    SET_SHADING_PROPERTY_ACTION_ID,
    SET_HVAC_PROPERTY_ACTION_ID,
    SET_SPACE_PROPERTY_ACTION_ID,
    SET_GAS_MATERIAL_PROPERTY_ACTION_ID,
    SET_CAMERA_ACTION_ID,
    SET_CONSTRUCTION_PROPERTY_ACTION_ID,
    SELECT_WEATHER_FILE_ACTION_ID,
    CLEAR_WEATHER_FILE_ACTION_ID,
];

/// ⚠️ The verbs that discard user content no later verb reconstructs — the two inspector deletes and
/// the whole-document example swap. `AppBuilder::action_destructive` reads this roster in
/// [`create_energy_model_editor`], which is what raises `ApprovalMode::WhenDestructive` so the MCP
/// gateway asks a human before an agent commits one.
pub const ENERGY_MODEL_DESTRUCTIVE_ACTION_IDS: &[&str] = &[DELETE_ZONE_ACTION_ID, DELETE_SURFACE_ACTION_ID, SET_ACTIVE_EXAMPLE_ACTION_ID];

/// 📬️ The twelve verbs that publish a semantic mutation into the document store. `setActiveExample`
/// is deliberately NOT one of them — it swaps the whole document through `kernel::Effect::LoadDocument`
/// (outside history), so it publishes to no store lane and declares `HostOnly`.
pub const ENERGY_MODEL_AUTHORED_DOCUMENT_TOOL_IDS: &[&str] = &[
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
    SET_LOAD_PROPERTY_ACTION_ID,
    SET_SHADING_PROPERTY_ACTION_ID,
    SET_HVAC_PROPERTY_ACTION_ID,
    SET_SPACE_PROPERTY_ACTION_ID,
    SET_GAS_MATERIAL_PROPERTY_ACTION_ID,
    SET_CONSTRUCTION_PROPERTY_ACTION_ID,
    SELECT_WEATHER_FILE_ACTION_ID,
    CLEAR_WEATHER_FILE_ACTION_ID,
];
pub const ENERGY_MODEL_NATIVE_DESCRIPTORS: &[protocol::MutationLeafDescriptor] = <semio_s_artifact_energy_model_mutations::EnergyModelMutation as protocol::Mutation<semio_s_artifact_energy_model_snapshot::EnergyModelSnapshot>>::DESCRIPTORS;
const fn with_native_tools<const N: usize>(authored: &[&'static str]) -> [&'static str; N] {
    let mut tools = [""; N];
    let mut index = 0;
    while index < authored.len() { tools[index] = authored[index]; index += 1; }
    let mut native = 0;
    while native < ENERGY_MODEL_NATIVE_DESCRIPTORS.len() { tools[index] = ENERGY_MODEL_NATIVE_DESCRIPTORS[native].aggregate_variant; native += 1; index += 1; }
    tools
}
pub const ENERGY_MODEL_RETAINED_TOOL_IDS: &[&str] = &with_native_tools::<{ ENERGY_MODEL_AUTHORED_TOOL_IDS.len() + ENERGY_MODEL_NATIVE_DESCRIPTORS.len() }>(ENERGY_MODEL_AUTHORED_TOOL_IDS);
pub const ENERGY_MODEL_DOCUMENT_TOOL_IDS: &[&str] = &with_native_tools::<{ ENERGY_MODEL_AUTHORED_DOCUMENT_TOOL_IDS.len() + ENERGY_MODEL_NATIVE_DESCRIPTORS.len() }>(ENERGY_MODEL_AUTHORED_DOCUMENT_TOOL_IDS);
//#endregion 🏷️ActionIds

//#region 🔖️Command
/// ✏️ The editor's typed command channel. `SetStructureField`/`SetZoneCell` are the two generic
/// window-kit edit targets; the ten authored document verbs address `crate::model::Model` entities
/// by their own `EntityId`; `SetSimulationSettings` edits the config store's run settings.
#[derive(Clone, Debug, PartialEq, ToValueDerive, FromValueDerive, semio_framework_dsl_record_derive::DslEnum, semio_framework_value::RetireOwned)]
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
    SetThermostatSetpoints { thermostat: u32, zone: u32, heating_schedule: u32, cooling_schedule: u32, heating_throttle_range_k: f64, cooling_throttle_range_k: f64 },
    #[dsl(key = "set-site")]
    SetSite { latitude_deg: Option<f64>, longitude_deg: Option<f64>, elevation_m: Option<f64>, time_zone_hours: Option<f64>, north_axis_deg: Option<f64> },
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
    #[dsl(key = "set-load-property")]
    SetLoadProperty { load: u32, property: String, value: String },
    #[dsl(key = "set-shading-property")]
    SetShadingProperty { shading: u32, property: String, value: String },
    #[dsl(key = "apply-native-mutation")]
    ApplyNativeMutation { kind: String, #[dsl(statements)] mutation: Option<semio_s_artifact_energy_model_mutations::EnergyModelMutation> },
    #[dsl(key = "set-hvac-property")]
    SetHvacProperty { system: u32, property: String, value: String },
    #[dsl(key = "set-space-property")]
    SetSpaceProperty { space: u32, property: String, value: String },
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
    #[dsl(key = "select-weather-file")]
    SelectWeatherFile { value: String },
    #[dsl(key = "clear-weather-file")]
    ClearWeatherFile,
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
            Self::SetSimulationSettings { .. } => SET_SIMULATION_SETTINGS_ACTION_ID,
            Self::SetResultField { .. } => SET_RESULT_FIELD_ACTION_ID,
            Self::SetSurfaceProperty { .. } => SET_SURFACE_PROPERTY_ACTION_ID,
            Self::SetFenestrationProperty { .. } => SET_FENESTRATION_PROPERTY_ACTION_ID,
            Self::SetZoneProperty { .. } => SET_ZONE_PROPERTY_ACTION_ID,
            Self::SetGlazingMaterialProperty { .. } => SET_GLAZING_MATERIAL_PROPERTY_ACTION_ID,
            Self::SetLoadProperty { .. } => SET_LOAD_PROPERTY_ACTION_ID,
            Self::SetShadingProperty { .. } => SET_SHADING_PROPERTY_ACTION_ID,
            Self::ApplyNativeMutation { kind, .. } => ENERGY_MODEL_NATIVE_DESCRIPTORS.iter().find(|descriptor| descriptor.semantic_kind == kind).map_or("native-mutation.invalid", |descriptor| descriptor.aggregate_variant),
            Self::SetHvacProperty { .. } => SET_HVAC_PROPERTY_ACTION_ID,
            Self::SetSpaceProperty { .. } => SET_SPACE_PROPERTY_ACTION_ID,
            Self::SetGasMaterialProperty { .. } => SET_GAS_MATERIAL_PROPERTY_ACTION_ID,
            Self::SetCamera { .. } => SET_CAMERA_ACTION_ID,
            Self::SetConstructionProperty { .. } => SET_CONSTRUCTION_PROPERTY_ACTION_ID,
            Self::SelectWeatherFile { .. } => SELECT_WEATHER_FILE_ACTION_ID,
            Self::ClearWeatherFile => CLEAR_WEATHER_FILE_ACTION_ID,
        }
    }
}

//#region 🌉️ActionBridge
/// 🌉️ Host action arguments (`DslValue`) → typed command. Numbers arriving as numeric strings (the
/// shape a `<select>`-sourced argument takes) are accepted for every numeric field, so the same
/// action works from the command palette, a keybinding and a rendered form alike.
pub mod args_bridge {
    use super::{EnergyModelEditorCommand as Command, Fault, FaultCode, FaultOrigin};

    fn field<'a>(args: Option<&'a semio_framework_value::DslValue>, key: &str) -> Option<&'a semio_framework_value::DslValue> {
        args?.get(key)
    }

    fn text(args: Option<&semio_framework_value::DslValue>, key: &str) -> Option<String> {
        let value = field(args, key)?;
        value.as_str().map(str::to_owned).or_else(|| value.as_u64().map(|number| number.to_string())).or_else(|| value.as_i64().map(|number| number.to_string())).or_else(|| value.as_f64().map(|number| number.to_string()))
    }

    fn number(args: Option<&semio_framework_value::DslValue>, key: &str) -> Option<f64> {
        let value = field(args, key)?;
        value.as_f64().or_else(|| value.as_str()?.parse().ok())
    }

    fn flag(args: Option<&semio_framework_value::DslValue>, key: &str) -> Option<bool> {
        let value = field(args, key)?;
        value.as_bool().or_else(|| value.as_str()?.parse().ok())
    }

    /// 🎥️ The `{position,target,zoom,up?}` object the react `World3dHost` sends under `camera`,
    /// validated as a real pose and canonicalized back to the exact JSON string
    /// `World3dScene::camera_json` consumes. A malformed or non-finite pose answers `None`, which the
    /// bridge turns into an explicit refusal rather than a silently ignored gesture.
    fn camera_pose_json(args: Option<&semio_framework_value::DslValue>) -> Option<String> {
        let value = field(args, "camera")?;
        let pose = <store::Viewport3dOrbit as semio_framework_value::FromValue>::from_value(value.clone()).ok()?;
        pose.validate().ok()?;
        Some(semio_framework_pack_json::to_json_string(&semio_framework_value::ToValue::to_value(&pose)))
    }

    fn unknown(action: &str) -> Fault {
        Fault::new(FaultOrigin::App, FaultCode::new("app.command.unsupported"), format!("the energy model editor has no command for action '{action}'"))
    }

    pub fn command_from_action(action: &str, args: Option<&semio_framework_value::DslValue>) -> Result<Command, Fault> {
        let text_or = |key: &str, fallback: &str| text(args, key).unwrap_or_else(|| fallback.to_string());
        let f64_or = |key: &str, fallback: f64| number(args, key).unwrap_or(fallback);
        let u32_or = |key: &str, fallback: u32| number(args, key).map_or(fallback, |value| value as u32);
        let bool_or = |key: &str, fallback: bool| flag(args, key).unwrap_or(fallback);
        if let Some(descriptor) = super::ENERGY_MODEL_NATIVE_DESCRIPTORS.iter().find(|descriptor| descriptor.aggregate_variant == action) {
            let _ = descriptor;
            return Err(Fault::from("app.command.original-admission-required"));
        }
        Ok(match action {
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
            super::SET_THERMOSTAT_SETPOINTS_ACTION_ID => {
                let edited = text(args, "field");
                let merged = |key: &str, fallback: f64| if edited.as_deref() == Some(key) { number(args, "value").unwrap_or(fallback) } else { f64_or(key, fallback) };
                Command::SetThermostatSetpoints {
                    thermostat: u32_or("thermostat", u32_or("id", 0)),
                    zone: merged("zone", 0.0) as u32,
                    heating_schedule: merged("heatingSchedule", 0.0) as u32,
                    cooling_schedule: merged("coolingSchedule", 0.0) as u32,
                    heating_throttle_range_k: merged("heatingThrottleRangeK", 2.0),
                    cooling_throttle_range_k: merged("coolingThrottleRangeK", 2.0),
                }
            }
            super::SET_SITE_ACTION_ID => {
                let edited = text(args, "field");
                let field = |key: &str| match edited.as_deref() {
                    Some(edited) if edited == key => number(args, "value"),
                    Some(_) => None,
                    None => number(args, key),
                };
                Command::SetSite { latitude_deg: field("latitudeDeg"), longitude_deg: field("longitudeDeg"), elevation_m: field("elevationM"), time_zone_hours: field("timeZoneHours"), north_axis_deg: field("northAxisDeg") }
            }
            super::SELECT_WEATHER_FILE_ACTION_ID => Command::SelectWeatherFile { value: text_or("value", "") },
            super::CLEAR_WEATHER_FILE_ACTION_ID => Command::ClearWeatherFile,
            super::SET_RUN_PERIOD_ACTION_ID => Command::SetRunPeriod { start_month: u32_or("startMonth", 1), start_day: u32_or("startDay", 1), end_month: u32_or("endMonth", 12), end_day: u32_or("endDay", 31) },
            super::SET_ACTIVE_EXAMPLE_ACTION_ID => Command::SetActiveExample { example_id: text_or("exampleId", "") },
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
            super::SET_HVAC_PROPERTY_ACTION_ID => {
                Command::SetHvacProperty { system: u32_or("system", u32_or("id", 0)), property: text(args, "property").unwrap_or_else(|| text_or("field", "")), value: text_or("value", "") }
            }
            super::SET_SPACE_PROPERTY_ACTION_ID => {
                Command::SetSpaceProperty { space: u32_or("space", u32_or("id", 0)), property: text(args, "property").unwrap_or_else(|| text_or("field", "")), value: text_or("value", "") }
            }
            super::SET_LOAD_PROPERTY_ACTION_ID => {
                Command::SetLoadProperty { load: u32_or("load", u32_or("id", 0)), property: text(args, "property").unwrap_or_else(|| text_or("field", "")), value: text_or("value", "") }
            }
            super::SET_SHADING_PROPERTY_ACTION_ID => {
                Command::SetShadingProperty { shading: u32_or("shading", u32_or("id", 0)), property: text(args, "property").unwrap_or_else(|| text_or("field", "")), value: text_or("value", "") }
            }
            super::SET_GAS_MATERIAL_PROPERTY_ACTION_ID => {
                Command::SetGasMaterialProperty { material: u32_or("material", u32_or("id", 0)), property: text(args, "property").unwrap_or_else(|| text_or("field", "")), value: text_or("value", "") }
            }
            super::SET_SIMULATION_SETTINGS_ACTION_ID => {
                let defaults = semio_s_artifact_energy_model_editor_config::EnergyModelConfig::default();
                Command::SetSimulationSettings { zone_timestep_minutes: u32_or("zoneTimestepMinutes", defaults.zone_timestep_minutes), system_timestep_minutes: u32_or("systemTimestepMinutes", defaults.system_timestep_minutes), warmup_days: u32_or("warmupDays", defaults.warmup_days) }
            }
            super::SET_RESULT_FIELD_ACTION_ID => Command::SetResultField { field: text(args, "field").unwrap_or_else(|| text_or("value", semio_s_artifact_energy_model_editor_config::EnergyModelConfig::default().result_field.as_str())) },
            super::SET_CAMERA_ACTION_ID => Command::SetCamera { camera: camera_pose_json(args).unwrap_or_default() },
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
    fn parse_op(line: &str) -> Result<Self, semio_framework_diagnostic::TextError> {
        semio_framework_dsl_record::variants_text::parse_op(line)
    }
    fn print_op(&self) -> String {
        semio_framework_dsl_record::variants_text::print_op(self)
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
        let (keyword, record) = <Self as semio_framework_dsl_record::DslVariants>::to_named_record(self);
        let variants = <Self as semio_framework_dsl_record::DslVariants>::variants();
        let ordinal = variants.iter().position(|(k, _)| *k == keyword).ok_or(protocol::ProtocolError::Malformed { what: "op variant", offset: 0, detail: format!("keyword {keyword:?} is not a declared variant") })?;
        let spec = (variants[ordinal].1.ordinary)();
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
        let variants = <Self as semio_framework_dsl_record::DslVariants>::variants();
        let (keyword, spec_fn) = variants.get(ordinal as usize).ok_or(protocol::ProtocolError::Malformed { what: "op variant", offset: 1, detail: format!("ordinal {ordinal} out of range for {} declared variants", variants.len()) })?;
        let spec = (spec_fn.ordinary)();
        let body = &bytes[reader.position()..];
        let (record, _report) = store::pack_rt::decode_record_body(body, &spec, &store::PackDecodeOptions::default()).map_err(protocol::ProtocolError::from)?;
        <Self as semio_framework_dsl_record::DslVariants>::from_named_record(keyword, &record).map_err(|error| protocol::ProtocolError::Malformed { what: "op record", offset: reader.position() as u64, detail: error.to_string() })
    }
}
//#endregion 🔖️OpCodec
//#endregion 🔖️Command

//#region 🔖️UiHelpers
/// 🎛️ The controller id every panel row and inspector control mints its action under — the canonical
/// surface id of this editor (`semio_framework::surface_app_id(MODEL_DIALECT, AppRole::Editor)`),
/// the same spelling `bounded_first_step_tool_proofs!` declares above.
pub const ENERGY_MODEL_EDITOR_CONTROLLER_ID: &str = "s.energy.model@1/*#editor";

/// 🏷️ Admits resolved energy text into the semantic UI contract.
pub fn ui_label(value: impl AsRef<str>) -> UiAssemblyResult<semio_framework_ui_contract::Label> {
    semio_framework_ui_contract::Label::try_from(value.as_ref()).map_err(|_| semio_framework_plugin_ownership::sdk_fault::PluginAssemblyError::new("ui.fixed-capacity", "energy UI label admission failed"))
}

/// 🎛️ Mints one energy-editor action for a panel row or an inspector control binding.
pub fn energy_model_action(action: &str, args: Option<semio_framework_ui_contract::UiValue>) -> UiAssemblyResult<(semio_framework_ui_contract::ActionId, Option<semio_framework_ui_contract::UiValue>)> {
    semio_framework_plugin_window_kits::ActionFactory::new(ENERGY_MODEL_EDITOR_CONTROLLER_ID).action(action, args)
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

//#region 🆔️KindIds
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

/// 💨️ Every fill gas, in declaration order — the inspector's gas select.
pub const GAS_KIND_IDS: &[&str] = &["air", "argon", "krypton", "xenon"];

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

/// 🧱️ Every roughness class, in declaration order — the inspector's roughness select.
pub const SURFACE_ROUGHNESS_IDS: &[&str] = &["veryRough", "rough", "mediumRough", "mediumSmooth", "smooth", "verySmooth"];
//#endregion 🆔️KindIds

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
fn inspector_action(id: &str, en: &str, de: &str, args: Vec<semio_framework::ActionArgDef>) -> semio_framework::ActionDefinition {
    semio_framework::ActionDefinition::bounded_catalog(id, LocalizedLabel::native(en, de), semio_framework::ActionKind::Mutation).with_args(args)
}

/// 🔍️ Every verb the inspection panel can dispatch, in one place. They must be reachable while ANY
/// window kind is active — the structure tree, the zones table, the simulation window and the 3d
/// model window alike.
pub fn inspector_action_definitions() -> Vec<semio_framework::ActionDefinition> {
    use semio_framework::ActionArgDef as Arg;
    let property = || Arg::text("property", LocalizedLabel::native("Property", "Eigenschaft")).required();
    let text_value = || Arg::text("value", LocalizedLabel::native("Value", "Wert")).required();
    let mut actions = vec![
        inspector_action(SELECT_WEATHER_FILE_ACTION_ID,"Select weather artifact","Wetterartefakt auswählen",vec![Arg::text("value",LocalizedLabel::native("EPW artifact address","EPW-Artefaktadresse")).required()]),
        inspector_action(CLEAR_WEATHER_FILE_ACTION_ID,"Clear weather artifact","Wetterartefakt lösen",Vec::new()),
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
        inspector_action(SET_LOAD_PROPERTY_ACTION_ID, "Set load property", "Lasteigenschaft setzen", vec![Arg::number("load", LocalizedLabel::native("Load id", "Last-Id")).required(), property(), text_value()]),
        inspector_action(SET_SHADING_PROPERTY_ACTION_ID, "Set shading property", "Verschattungseigenschaft setzen", vec![Arg::number("shading", LocalizedLabel::native("Shading surface id", "Verschattungsflächen-Id")).required(), property(), text_value()]),
        inspector_action(SET_HVAC_PROPERTY_ACTION_ID, "Set HVAC property", "HLK-Eigenschaft setzen", vec![Arg::number("system", LocalizedLabel::native("System id", "System-Id")).required(), property(), text_value()]),
        inspector_action(SET_SPACE_PROPERTY_ACTION_ID, "Set space property", "Raumeigenschaft setzen", vec![Arg::number("space", LocalizedLabel::native("Space id", "Raum-Id")).required(), property(), text_value()]),
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
            vec![Arg::number("material", LocalizedLabel::native("Material id", "Material-Id")).required(), property(), text_value()],
        ),
        inspector_action(
            SET_THERMOSTAT_SETPOINTS_ACTION_ID,
            "Set thermostat setpoints",
            "Thermostat-Sollwerte setzen",
            vec![
                Arg::number("thermostat", LocalizedLabel::native("Thermostat id", "Thermostat-Id")).required(),
                Arg::number("zone", LocalizedLabel::native("Thermal zone", "Thermische Zone")).required(),
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
                Arg::slider("latitudeDeg", LocalizedLabel::native("Latitude (°)", "Breitengrad (°)"), -90.0, 90.0),
                Arg::slider("longitudeDeg", LocalizedLabel::native("Longitude (°)", "Längengrad (°)"), -180.0, 180.0),
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
    ];
    actions.extend(native_mutation_actions());
    actions
}

//#endregion 🔍️InspectorActions

/// 🧬️ Each native mutation receives one precise schema-derived action form and one retained tool identity.
pub fn native_mutation_actions() -> Vec<semio_framework::ActionDefinition> {
    use protocol::Mutation;
    use semio_s_artifact_energy_model_mutations::EnergyModelMutation;
    use semio_s_artifact_energy_model_snapshot::EnergyModelSnapshot;
    use semio_framework::{ActionArgDef, ArgSchema};
    let schemas = <EnergyModelMutation as Mutation<EnergyModelSnapshot>>::INPUT_SCHEMAS;
    let documents = <EnergyModelMutation as Mutation<EnergyModelSnapshot>>::INPUT_SCHEMA_DOCUMENTS;
    let resolver = |id: &str| documents.iter().flat_map(|row| row.iter()).find_map(|document| {
        let value: semio_framework_value::DslValue = semio_framework_pack_json::from_json_str(document, semio_framework_pack_json::JsonMemberPolicy::Reject).ok()?;
        (value.get("$id").and_then(semio_framework_value::DslValue::as_str) == Some(id)).then_some(value)
    }).or_else(|| semio_framework_manifest_mutation_inputs::registered_input_schema_document(id));
    fn schema_keys(schema: &mut ArgSchema) {
        match schema {
            ArgSchema::Object { fields } => action_keys(fields),
            ArgSchema::Array { items, .. } => schema_keys(items),
            _ => {}
        }
    }
    fn action_keys(fields: &mut [ActionArgDef]) {
        for field in fields {
            field.id = field.id.rsplit('/').next().unwrap_or(&field.id).replace("~1", "/").replace("~0", "~");
            schema_keys(&mut field.schema);
        }
    }
    ENERGY_MODEL_NATIVE_DESCRIPTORS.iter().zip(schemas).map(|(descriptor, schema)| {
        let mut fields = semio_framework_manifest_mutation_inputs::mutation_input_defs(schema, &resolver).unwrap_or_else(|error| panic!("{} input schema refused: {error:?}", descriptor.semantic_kind));
        action_keys(&mut fields);
        let mut action = inspector_action(descriptor.aggregate_variant, descriptor.display_name, descriptor.display_name, fields);
        let verb = match descriptor.semantic_kind.split('-').next().unwrap_or("") {
            "create" => "Erstellen", "delete" | "remove" => "Entfernen", "change" => "Ändern", "rename" => "Umbenennen", "replace" => "Ersetzen", "add" => "Hinzufügen", "insert" => "Einfügen", "reorder" => "Neu ordnen", "bind" => "Zuordnen", "clear" => "Löschen", _ => "Bearbeiten",
        };
        let entity = [
            ("fault", "Fehler"), ("daily-schedule", "Tagesprofil"), ("setpoint-manager", "Sollwertmanager"), ("infiltration", "Infiltration"), ("fenestration", "Fenster"), ("annual-schedule", "Jahresprofil"), ("weekly-schedule", "Wochenprofil"), ("lighting-gain", "Beleuchtungslast"), ("sizing-object", "Auslegungsobjekt"), ("room-air-model", "Raumluftmodell"), ("constant-schedule", "Konstantes Profil"), ("plant-loop", "Anlagenkreislauf"), ("shw-system", "Warmwassersystem"), ("battery", "Batterie"), ("refrigeration-system", "Kältesystem"), ("ideal-loads-system", "Ideales Lastsystem"), ("outdoor-air-system", "Außenluftsystem"), ("mechanical-ventilation", "Mechanische Lüftung"), ("electrical-load-center", "Elektrische Lastzentrale"), ("water-system", "Wassersystem"), ("equipment-gain", "Gerätelast"), ("air-loop", "Luftkreislauf"), ("time-series-schedule", "Zeitreihenprofil"), ("daylight-zone", "Tageslichtzone"), ("humidistat", "Feuchteregler"), ("surface", "Fläche"), ("glazing-material", "Verglasungsmaterial"), ("gas-material", "Gaszwischenraum"), ("material", "Material"), ("thermostat", "Thermostat"), ("shading-surface", "Verschattungsfläche"), ("thermal-enclosure", "Thermische Hülle"), ("pv-system", "PV-System"), ("solar-thermal-system", "Solarthermieanlage"), ("people-gain", "Personenlast"), ("zone-equipment", "Zonenausstattung"), ("zone", "Zone"), ("construction", "Konstruktion"), ("space", "Raum"), ("space-list", "Raumliste"), ("ground", "Erdboden"), ("site", "Standort"), ("run", "Simulationszeitraum"), ("airflow-network", "Luftströmungsnetz"), ("output-variable", "Ausgabegröße"), ("model", "Energiemodell"),
        ].into_iter().filter(|(kind, _)| descriptor.semantic_kind.contains(kind)).max_by_key(|(kind, _)| kind.len()).map_or("Energiemodell", |(_, label)| label);
        action.label = LocalizedLabel::from_fn(|terminology, locale| {
            if locale == semio_framework_ui_locale::Locale::De {
                let field = action.args.iter().find(|field| field.id != "id" && field.id != "index").map(|field| field.label.resolve(terminology, locale));
                field.map_or_else(|| format!("{verb} · {entity}"), |field| format!("{verb} · {entity} · {field}"))
            } else { descriptor.display_name.to_string() }
        });
        action.category = Some("🔋".to_string());
        if descriptor.semantic_kind.starts_with("delete-") { action = action.destructive(); }
        action
    }).collect()
}

#[path="🛬️action/🦀️.rs"]pub mod original_action;
impl original_action::EnergyOriginalCommand<semio_s_artifact_energy_model_mutations::EnergyModelMutation> for EnergyModelEditorCommand{
 fn descriptor(index:usize)->Option<(&'static str,&'static str,semio_s_artifact_energy_model_native_mutations_singletons::action::NativeActionFactory<semio_s_artifact_energy_model_mutations::EnergyModelMutation>)>{let descriptor=ENERGY_MODEL_NATIVE_DESCRIPTORS.get(index)?;let(action,factory)=semio_s_artifact_energy_model_mutations::EnergyModelMutation::ORIGINAL_ACTION_RECEIVERS.get(index)?;if *action!=descriptor.aggregate_variant{return None}Some((action,descriptor.semantic_kind,*factory))}
 fn from_original_mutation(kind:String,mutation:semio_s_artifact_energy_model_mutations::EnergyModelMutation)->Self{Self::ApplyNativeMutation{kind,mutation:Some(mutation)}}
 fn original_wire(&self)->semio_framework_plugin_command::command_admission::original_typed_command::original_wire::ArtifactPreparedOperationSource<'_>{let(_,ordinal,spec)=semio_framework_dsl_record::BorrowedDslVariants::projected_borrowed_variant_identity(self);semio_framework_plugin_command::command_admission::original_typed_command::original_wire::ArtifactPreparedOperationSource::Pack{tag:ordinal as u64,body:self,spec}}
 const MAXIMUM_WIRE_BYTES:usize=262144;
}
impl semio_framework_dsl_record::native_encoding::FieldProjectionSource for EnergyModelEditorCommand{
 fn projection_view(&self,path:&[usize])->Result<semio_framework_dsl_record::native_encoding::FieldProjectionView<'_>,semio_framework_value::ValueError>{semio_framework_dsl_record::BorrowedDslVariants::projected_borrowed_variant_view(self,path)}
 fn projection_key(&self,path:&[usize],index:usize)->Result<&str,semio_framework_value::ValueError>{semio_framework_dsl_record::BorrowedDslVariants::projected_borrowed_variant_key(self,path,index)}
}
pub static ENERGY_ORIGINAL_ACTION_FACTORY:original_action::EnergyActionFactory<EnergyModelEditorCommand,semio_s_artifact_energy_model_mutations::EnergyModelMutation>=original_action::EnergyActionFactory::new();
