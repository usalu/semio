//! 🔋️ Energy model editor — the authored `ArtifactEditor` surface for `s.energy.model@1/*`
//! (tickets 26/08/16/ARTIFACT-VIEWERS-AND-EDITORS-PER-SUBSET, 26/09/06/ENERGY-PLUGIN-END-TO-END).
//! Three windows: `structure` (framework `TreeWindowKit` over the whole `crate::model::Model`),
//! `zones` (framework `TableWindowKit` over `Model::zones`) and `simulation` (a hand-rolled window
//! projecting the mounted `🧵️simulation-session` worker).
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
//! supplies the document lane's one-item retained preparation; the six session verbs publish nothing
//! to a store and declare `HostOnly`.

use crate::artifacts::model::mutations;
use crate::artifacts::model::{EnergyModelMutation, EnergyModelSnapshot, ENERGY_MODEL_DOCUMENT_SCHEMA, MODEL_DIALECT};
use crate::editor::model::modes::edit;
use crate::editor::model::modes::edit::windows::{simulation, structure, zones};
use crate::energy_simulation_session::{self as simulation_session, EnergySimulationConfigProjection, EnergySimulationEventKind, EnergySimulationRequestIdentity};
use crate::model::{EntityId, Material, OutsideBoundary, ScheduleId, Site, Surface, SurfaceClass, Thermostat, Zone};
use semio_framework_plugin::{ToolExecutionContract, ToolFactoryKey, ToolJobFactoryError};
use semio_framework_plugin::retained_command::{ArtifactRetainedCommandJob, ArtifactRetainedCommandPayload, BoundedArtifactCommandWork};
use semio_framework_plugin::{
    AppOperationContext, ArtifactEditor, ArtifactOwnedToolJobRequest, ArtifactToolFactoryRegistry, ArtifactToolPublicationContract, ArtifactToolPublicationLane, ArtifactView, ComponentTree, ConfigView,
    Dialect, DraftView, Editor, EditorApp, Emit, ExampleSource, Fault, FaultCode, FaultOrigin, HistoryView, InteractiveJobClassification, Label, LocalizedLabel, NoConfig, NoConfigMutation, NoDraft, NoDraftMutation, NoPresence, NoPresenceMutation,
    NoTransient, NoTransientMutation, UiAssemblyResult,
};
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
    simulation::START_ACTION_ID,
    simulation::CANCEL_ACTION_ID,
    simulation::RETRY_ACTION_ID,
    simulation::DISCARD_ACTION_ID,
    simulation::ADOPT_ACTION_ID,
    simulation::CONFIGURE_ACTION_ID,
];

/// 📬️ The twelve verbs that publish a semantic mutation into the document store. `setActiveExample`
/// is deliberately NOT one of them — it swaps the whole document through `kernel::Effect::LoadDocument`
/// (outside history), so it publishes to no store lane and declares `HostOnly` like the six session
/// verbs do.
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
];
//#endregion 🏷️ActionIds

//#region 🔖️Command
/// ✏️ The editor's typed command channel. `SetStructureField`/`SetZoneCell` are the two generic
/// window-kit edit targets; the ten authored document verbs address `crate::model::Model` entities
/// by their own `EntityId`; the six session verbs drive the mounted simulation worker.
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
    #[dsl(key = "set-material-property")]
    SetMaterialProperty { material: u32, property: String, value: f64 },
    #[dsl(key = "set-thermostat-setpoints")]
    SetThermostatSetpoints { thermostat: u32, heating_schedule: u32, cooling_schedule: u32, heating_throttle_range_k: f64, cooling_throttle_range_k: f64 },
    #[dsl(key = "set-site")]
    SetSite { latitude_deg: f64, longitude_deg: f64, elevation_m: f64, time_zone_hours: f64, north_axis_deg: f64 },
    #[dsl(key = "set-run-period")]
    SetRunPeriod { start_month: u32, start_day: u32, end_month: u32, end_day: u32 },
    #[dsl(key = "setActiveExample")]
    SetActiveExample { example_id: String },
    #[dsl(key = "start-energy-simulation")]
    StartSimulation { request: u64 },
    #[dsl(key = "cancel-energy-simulation")]
    CancelSimulation { request: u64, operation: u64, generation: u64, config_digest: u64 },
    #[dsl(key = "retry-energy-simulation")]
    RetrySimulation { request: u64, operation: u64, generation: u64, config_digest: u64 },
    #[dsl(key = "discard-energy-simulation")]
    DiscardSimulation { request: u64, operation: u64, generation: u64, config_digest: u64 },
    #[dsl(key = "adopt-energy-simulation")]
    AdoptSimulation { request: u64, operation: u64, generation: u64, config_digest: u64 },
    #[dsl(key = "configure-energy-simulation")]
    ConfigureSimulation { locale: String, zone_timestep_minutes: u32, system_timestep_minutes: u32, warmup_days: u32 },
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
            Self::StartSimulation { .. } => simulation::START_ACTION_ID,
            Self::CancelSimulation { .. } => simulation::CANCEL_ACTION_ID,
            Self::RetrySimulation { .. } => simulation::RETRY_ACTION_ID,
            Self::DiscardSimulation { .. } => simulation::DISCARD_ACTION_ID,
            Self::AdoptSimulation { .. } => simulation::ADOPT_ACTION_ID,
            Self::ConfigureSimulation { .. } => simulation::CONFIGURE_ACTION_ID,
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

    fn unknown(action: &str) -> Fault {
        Fault::new(FaultOrigin::App, FaultCode::new("app.command.unsupported"), format!("the energy model editor has no command for action '{action}'"))
    }

    pub fn command_from_action(action: &str, args: Option<&dsl::DslValue>) -> Result<Command, Fault> {
        let text_or = |key: &str, fallback: &str| text(args, key).unwrap_or_else(|| fallback.to_string());
        let f64_or = |key: &str, fallback: f64| number(args, key).unwrap_or(fallback);
        let u32_or = |key: &str, fallback: u32| number(args, key).map_or(fallback, |value| value as u32);
        let u64_or = |key: &str, fallback: u64| number(args, key).map_or(fallback, |value| value as u64);
        let bool_or = |key: &str, fallback: bool| flag(args, key).unwrap_or(fallback);
        let identity = |command: fn(u64, u64, u64, u64) -> Command| command(u64_or("request", 0), u64_or("operation", 0), u64_or("generation", 0), u64_or("configDigest", 0));
        Ok(match action {
            super::SET_NODE_ACTION_ID => Command::SetStructureField { field: text_or("id", ""), value: text_or("value", "") },
            super::SET_CELL_ACTION_ID => Command::SetZoneCell { row: u32_or("row", 0), column: text_or("column", ""), value: text_or("value", "") },
            super::CREATE_ZONE_ACTION_ID => Command::CreateZone { name: text_or("name", "Zone"), volume_m3: f64_or("volumeM3", 100.0), multiplier: u32_or("multiplier", 1), conditioned: bool_or("conditioned", true) },
            super::RENAME_ZONE_ACTION_ID => Command::RenameZone { zone: u32_or("zone", 0), new_name: text_or("newName", "") },
            super::DELETE_ZONE_ACTION_ID => Command::DeleteZone { zone: u32_or("zone", 0) },
            super::CREATE_SURFACE_ACTION_ID => Command::CreateSurface { name: text_or("name", "Surface"), zone: u32_or("zone", 0), construction: u32_or("construction", 0), class: text_or("class", "exteriorWall") },
            super::DELETE_SURFACE_ACTION_ID => Command::DeleteSurface { surface: u32_or("surface", 0) },
            super::ASSIGN_SURFACE_CONSTRUCTION_ACTION_ID => Command::AssignSurfaceConstruction { surface: u32_or("surface", 0), construction: u32_or("construction", 0) },
            super::SET_MATERIAL_PROPERTY_ACTION_ID => Command::SetMaterialProperty { material: u32_or("material", 0), property: text_or("property", "conductivity"), value: f64_or("value", 0.0) },
            super::SET_THERMOSTAT_SETPOINTS_ACTION_ID => Command::SetThermostatSetpoints {
                thermostat: u32_or("thermostat", 0),
                heating_schedule: u32_or("heatingSchedule", 0),
                cooling_schedule: u32_or("coolingSchedule", 0),
                heating_throttle_range_k: f64_or("heatingThrottleRangeK", 2.0),
                cooling_throttle_range_k: f64_or("coolingThrottleRangeK", 2.0),
            },
            super::SET_SITE_ACTION_ID => Command::SetSite {
                latitude_deg: f64_or("latitudeDeg", 0.0),
                longitude_deg: f64_or("longitudeDeg", 0.0),
                elevation_m: f64_or("elevationM", 0.0),
                time_zone_hours: f64_or("timeZoneHours", 0.0),
                north_axis_deg: f64_or("northAxisDeg", 0.0),
            },
            super::SET_RUN_PERIOD_ACTION_ID => Command::SetRunPeriod { start_month: u32_or("startMonth", 1), start_day: u32_or("startDay", 1), end_month: u32_or("endMonth", 12), end_day: u32_or("endDay", 31) },
            super::SET_ACTIVE_EXAMPLE_ACTION_ID => Command::SetActiveExample { example_id: text_or("exampleId", "") },
            super::simulation::START_ACTION_ID => Command::StartSimulation { request: u64_or("request", 0) },
            super::simulation::CANCEL_ACTION_ID => identity(|request, operation, generation, config_digest| Command::CancelSimulation { request, operation, generation, config_digest }),
            super::simulation::RETRY_ACTION_ID => identity(|request, operation, generation, config_digest| Command::RetrySimulation { request, operation, generation, config_digest }),
            super::simulation::DISCARD_ACTION_ID => identity(|request, operation, generation, config_digest| Command::DiscardSimulation { request, operation, generation, config_digest }),
            super::simulation::ADOPT_ACTION_ID => identity(|request, operation, generation, config_digest| Command::AdoptSimulation { request, operation, generation, config_digest }),
            super::simulation::CONFIGURE_ACTION_ID => Command::ConfigureSimulation {
                locale: text_or("locale", "en"),
                zone_timestep_minutes: u32_or("zoneTimestepMinutes", 60),
                system_timestep_minutes: u32_or("systemTimestepMinutes", 60),
                warmup_days: u32_or("warmupDays", 7),
            },
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
        let variants = <Self as dsl::DslVariants>::variants();
        for (keyword, spec_fn) in &variants {
            let probe = format!("{} ", keyword);
            if line == keyword.as_str() || line.starts_with(&probe) {
                let record = dsl::parse(line, &spec_fn(), &dsl::ParseOptions { limits: dsl::Limits::default(), mode: dsl::SourceMode::Inline })?;
                return <Self as dsl::DslVariants>::from_named_record(keyword, &record);
            }
        }
        Err(dsl::__rt::field_error(format!("unknown operation line '{line}'")))
    }
    fn print_op(&self) -> String {
        let (keyword, record) = <Self as dsl::DslVariants>::to_named_record(self);
        let variants = <Self as dsl::DslVariants>::variants();
        let spec_fn = variants.iter().find(|(k, _)| k == &keyword).map(|(_, s)| *s).expect("variant spec must exist for its own keyword");
        dsl::print(&record, &spec_fn(), dsl::JoinMode::Inline)
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
fn model_edit(kind: &'static str, base: &crate::model::Model, model: &crate::model::Model, description: String) -> Result<Emit<EnergyModelMutation>, Fault> {
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
    diff_zones(base, model, &mut steps);
    diff_surfaces(base, model, &mut steps);
    diff_materials(kind, base, model, &mut steps)?;
    diff_thermostats(kind, base, model, &mut steps)?;
    let mut probe = base.clone();
    probe.name = model.name.clone();
    probe.version = model.version.clone();
    probe.site = model.site;
    probe.run_period = model.run_period;
    probe.ground_temperature = model.ground_temperature.clone();
    probe.airflow_network = model.airflow_network.clone();
    probe.output_variables = model.output_variables.clone();
    probe.zones = model.zones.clone();
    probe.surfaces = model.surfaces.clone();
    probe.fenestrations = model.fenestrations.clone();
    probe.adjacency_pairs = model.adjacency_pairs.clone();
    probe.materials = model.materials.clone();
    probe.thermostats = model.thermostats.clone();
    if probe != *model {
        return Err(kind_unavailable(kind, kind));
    }
    Ok(Emit { artifact_mutations: steps, description: Some(description), ..Default::default() })
}

/// 🏘️ Zones: a create/delete of the whole row plus the five per-field kinds. `create-zone`/
/// `delete-zone` landed with the 100s group, so an identity change is no longer a refusal.
fn diff_zones(base: &crate::model::Model, model: &crate::model::Model, steps: &mut Vec<EnergyModelMutation>) {
    for was in &base.zones {
        if !model.zones.iter().any(|now| now.id == was.id) {
            steps.push(mutations::delete_zone(was.id));
        }
    }
    for now in &model.zones {
        let Some(was) = base.zones.iter().find(|was| was.id == now.id) else {
            steps.push(mutations::create_zone(now.id, now.name.clone(), now.volume_m3, now.multiplier, now.conditioned, now.part_of_total_floor_area));
            continue;
        };
        if was.name != now.name {
            steps.push(mutations::rename_zone(now.id, now.name.clone()));
        }
        if was.volume_m3 != now.volume_m3 {
            steps.push(mutations::change_zone_volume(now.id, now.volume_m3));
        }
        if was.multiplier != now.multiplier {
            steps.push(mutations::change_zone_multiplier(now.id, now.multiplier));
        }
        if was.conditioned != now.conditioned {
            steps.push(mutations::change_zone_conditioned(now.id, now.conditioned));
        }
        if was.part_of_total_floor_area != now.part_of_total_floor_area {
            steps.push(mutations::change_zone_floor_area_participation(now.id, now.part_of_total_floor_area));
        }
    }
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
fn diff_surfaces(base: &crate::model::Model, model: &crate::model::Model, steps: &mut Vec<EnergyModelMutation>) {
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
            continue;
        };
        if was.name != now.name {
            steps.push(mutations::rename_surface(now.id, now.name.clone()));
        }
        if was.zone_id != now.zone_id {
            steps.push(mutations::change_surface_zone(now.id, now.zone_id));
        }
        if was.class != now.class {
            steps.push(mutations::change_surface_class(now.id, now.class));
        }
        if was.vertices_m != now.vertices_m {
            steps.push(mutations::replace_surface_vertices(now.id, now.vertices_m.clone()));
        }
        if was.construction_id != now.construction_id {
            steps.push(mutations::change_surface_construction(now.id, now.construction_id));
        }
        if was.outside_boundary_condition != now.outside_boundary_condition {
            steps.push(mutations::change_surface_boundary_condition(now.id, now.outside_boundary_condition.kind(), interzone_partner(now.outside_boundary_condition)));
        }
        if was.sun_exposed != now.sun_exposed {
            steps.push(mutations::change_surface_sun_exposed(now.id, now.sun_exposed));
        }
        if was.wind_exposed != now.wind_exposed {
            steps.push(mutations::change_surface_wind_exposed(now.id, now.wind_exposed));
        }
        if was.multiplier != now.multiplier {
            steps.push(mutations::change_surface_multiplier(now.id, now.multiplier));
        }
    }
}

/// 🧱️ The seven material scalars `set-material-property` addresses. No editor verb creates or
/// deletes a material, so an identity change is still refused LOUDLY rather than masked by the
/// probe below.
fn diff_materials(kind: &'static str, base: &crate::model::Model, model: &crate::model::Model, steps: &mut Vec<EnergyModelMutation>) -> Result<(), Fault> {
    if base.materials.iter().map(|material| material.id).ne(model.materials.iter().map(|material| material.id)) {
        return Err(kind_unavailable(kind, "create-material / delete-material"));
    }
    for (was, now) in base.materials.iter().zip(&model.materials) {
        if was.thickness_m != now.thickness_m {
            steps.push(mutations::change_material_thickness(now.id, now.thickness_m));
        }
        if was.conductivity_w_m_k != now.conductivity_w_m_k {
            steps.push(mutations::change_material_conductivity(now.id, now.conductivity_w_m_k));
        }
        if was.density_kg_m3 != now.density_kg_m3 {
            steps.push(mutations::change_material_density(now.id, now.density_kg_m3));
        }
        if was.specific_heat_j_kg_k != now.specific_heat_j_kg_k {
            steps.push(mutations::change_material_specific_heat(now.id, now.specific_heat_j_kg_k));
        }
        if was.thermal_absorptance != now.thermal_absorptance {
            steps.push(mutations::change_material_thermal_absorptance(now.id, now.thermal_absorptance));
        }
        if was.solar_absorptance != now.solar_absorptance {
            steps.push(mutations::change_material_solar_absorptance(now.id, now.solar_absorptance));
        }
        if was.visible_absorptance != now.visible_absorptance {
            steps.push(mutations::change_material_visible_absorptance(now.id, now.visible_absorptance));
        }
    }
    Ok(())
}

/// 🌡️ The four thermostat fields `set-thermostat-setpoints` addresses. Like materials, no editor
/// verb adds or removes a thermostat, so an identity change is refused rather than masked.
fn diff_thermostats(kind: &'static str, base: &crate::model::Model, model: &crate::model::Model, steps: &mut Vec<EnergyModelMutation>) -> Result<(), Fault> {
    if base.thermostats.iter().map(|thermostat| thermostat.id).ne(model.thermostats.iter().map(|thermostat| thermostat.id)) {
        return Err(kind_unavailable(kind, "create-thermostat / delete-thermostat"));
    }
    for (was, now) in base.thermostats.iter().zip(&model.thermostats) {
        if was.heating_setpoint_schedule_id != now.heating_setpoint_schedule_id {
            steps.push(mutations::change_thermostat_heating_setpoint_schedule(now.id, now.heating_setpoint_schedule_id));
        }
        if was.cooling_setpoint_schedule_id != now.cooling_setpoint_schedule_id {
            steps.push(mutations::change_thermostat_cooling_setpoint_schedule(now.id, now.cooling_setpoint_schedule_id));
        }
        if was.heating_throttle_range_k != now.heating_throttle_range_k {
            steps.push(mutations::change_thermostat_heating_throttle_range(now.id, now.heating_throttle_range_k));
        }
        if was.cooling_throttle_range_k != now.cooling_throttle_range_k {
            steps.push(mutations::change_thermostat_cooling_throttle_range(now.id, now.cooling_throttle_range_k));
        }
    }
    Ok(())
}

/// 📂️ The sanctioned whole-document load: a `kernel::Effect::LoadDocument` carrying a genesis
/// pack+spr the host swaps into the live store through `ArtifactStore::reset`, OUTSIDE undo history.
/// This is why `📚️examples` need no `replace-model` kind — whole-document replace has no mutation
/// representative in this artifact's vocabulary at all (`📓️derivation-rules.md` rule 6), exactly as
/// in `📐️cad`'s `reset_document_effect` and `🔱️trinity`'s. A freshly minted envelope has no edits,
/// so its spr encode is infallible.
fn load_document_effect(model: &crate::model::Model) -> semio_framework_plugin::kernel::Effect {
    let snapshot = crate::artifacts::model::energy_snapshot_with_state(ENERGY_MODEL_DOCUMENT_SCHEMA, model, None);
    let pack = <EnergyModelSnapshot as store::ArtifactPack>::encode_pack(&snapshot);
    let envelope = store::create_document_envelope::<EnergyModelSnapshot, EnergyModelMutation>(ENERGY_MODEL_DOCUMENT_SCHEMA, "model", snapshot, None);
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
fn reduce(command: &EnergyModelEditorCommand, doc: &ArtifactView<'_, EnergyModelSnapshot>) -> Result<Emit<EnergyModelMutation>, Fault> {
    if is_session_command(command) {
        let operation = doc.operation()?;
        let render = simulation_session::render_identity_of(operation).ok_or_else(|| Fault::from("energy simulation command lacks a canonical document revision"))?;
        let event = session_event(command, render)?.ok_or_else(|| Fault::from("energy session command lost its event on the way to the session"))?;
        simulation_session::record_event(render, event).map_err(Fault::from)?;
        return Ok(Emit { description: Some(command.action_id().into()), ..Default::default() });
    }
    let mut model = crate::artifacts::model::energy_model(doc.snapshot);
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
            let kind = set_material_property(target, property, *value)?;
            (kind, format!("Set material {material} {property}"))
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
        _ => unreachable!("session events returned before document mutation dispatch"),
    };
    model_edit(kind, &crate::artifacts::model::energy_model(doc.snapshot), &model, description)
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

fn set_material_property(material: &mut Material, property: &str, value: f64) -> Result<&'static str, Fault> {
    let positive = |value: f64| value > 0.0;
    let fraction = |value: f64| (0.0..=1.0).contains(&value);
    let invalid = || Fault::new(FaultOrigin::App, FaultCode::new("mutation.invalid-payload"), format!("'{value}' is outside the SI range of material property '{property}'"));
    Ok(match property {
        "thicknessM" if positive(value) => {
            material.thickness_m = value;
            "change-material-thickness"
        }
        "conductivityWMK" if positive(value) => {
            material.conductivity_w_m_k = value;
            "change-material-conductivity"
        }
        "densityKgM3" if positive(value) => {
            material.density_kg_m3 = value;
            "change-material-density"
        }
        "specificHeatJKgK" if positive(value) => {
            material.specific_heat_j_kg_k = value;
            "change-material-specific-heat"
        }
        "thermalAbsorptance" if fraction(value) => {
            material.thermal_absorptance = value;
            "change-material-thermal-absorptance"
        }
        "solarAbsorptance" if fraction(value) => {
            material.solar_absorptance = value;
            "change-material-solar-absorptance"
        }
        "visibleAbsorptance" if fraction(value) => {
            material.visible_absorptance = value;
            "change-material-visible-absorptance"
        }
        "thicknessM" | "conductivityWMK" | "densityKgM3" | "specificHeatJKgK" | "thermalAbsorptance" | "solarAbsorptance" | "visibleAbsorptance" => return Err(invalid()),
        _ => return Err(Fault::new(FaultOrigin::App, FaultCode::new("mutation.invalid-payload"), format!("'{property}' is not a material property"))),
    })
}

/// ⚡️ Whether this verb drives the mounted simulation session instead of the document.
fn is_session_command(command: &EnergyModelEditorCommand) -> bool {
    matches!(
        command,
        EnergyModelEditorCommand::StartSimulation { .. }
            | EnergyModelEditorCommand::CancelSimulation { .. }
            | EnergyModelEditorCommand::RetrySimulation { .. }
            | EnergyModelEditorCommand::DiscardSimulation { .. }
            | EnergyModelEditorCommand::AdoptSimulation { .. }
            | EnergyModelEditorCommand::ConfigureSimulation { .. }
    )
}

/// ⚡️ Maps the six session verbs onto their `EnergySimulationEventKind`; every document verb maps to
/// `None` and falls through to the model reducer. `start` reads the run settings a previous
/// `configure-energy-simulation` admitted for this exact app instance rather than carrying them in
/// its own payload, so the window and the run can never disagree about what is about to run.
fn session_event(command: &EnergyModelEditorCommand, render: semio_framework_plugin::AppRenderOperationContext) -> Result<Option<EnergySimulationEventKind>, Fault> {
    let identity = |request: &u64, operation: &u64, generation: &u64, config_digest: &u64| EnergySimulationRequestIdentity { request: *request, operation: *operation, generation: *generation, config_digest: *config_digest };
    Ok(Some(match command {
        EnergyModelEditorCommand::StartSimulation { request } => EnergySimulationEventKind::Start { request: *request, config: simulation_session::session_settings(Some(render)) },
        EnergyModelEditorCommand::CancelSimulation { request, operation, generation, config_digest } => EnergySimulationEventKind::Cancel(identity(request, operation, generation, config_digest)),
        EnergyModelEditorCommand::RetrySimulation { request, operation, generation, config_digest } => EnergySimulationEventKind::Retry(identity(request, operation, generation, config_digest)),
        EnergyModelEditorCommand::DiscardSimulation { request, operation, generation, config_digest } => EnergySimulationEventKind::Discard(identity(request, operation, generation, config_digest)),
        EnergyModelEditorCommand::AdoptSimulation { request, operation, generation, config_digest } => EnergySimulationEventKind::Adopt(identity(request, operation, generation, config_digest)),
        EnergyModelEditorCommand::ConfigureSimulation { locale, zone_timestep_minutes, system_timestep_minutes, warmup_days } => {
            if locale != "en" && locale != "de" {
                return Err(Fault::new(FaultOrigin::App, FaultCode::new("app.command.locale"), "the energy simulation requires an explicit locale, en or de"));
            }
            EnergySimulationEventKind::Configure {
                config: EnergySimulationConfigProjection {
                    locale_de: locale == "de",
                    checkpoint_token: 0,
                    zone_timestep_minutes: *zone_timestep_minutes,
                    system_timestep_minutes: *system_timestep_minutes,
                    warmup_days: *warmup_days,
                },
            }
        }
        _ => return Ok(None),
    }))
}
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
    _config: &NoConfig,
    history: &HistoryView,
    _interaction: &protocol::InteractionState,
    _hover: &semio_framework_plugin::app::InteractionHoverState,
    operation: &AppOperationContext,
) -> Result<Emit<EnergyModelMutation, NoConfigMutation, NoDraftMutation>, Fault> {
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
        ArtifactToolPublicationContract { tool_id: simulation::START_ACTION_ID, lanes: &[ArtifactToolPublicationLane::HostOnly] },
        ArtifactToolPublicationContract { tool_id: simulation::CANCEL_ACTION_ID, lanes: &[ArtifactToolPublicationLane::HostOnly] },
        ArtifactToolPublicationContract { tool_id: simulation::RETRY_ACTION_ID, lanes: &[ArtifactToolPublicationLane::HostOnly] },
        ArtifactToolPublicationContract { tool_id: simulation::DISCARD_ACTION_ID, lanes: &[ArtifactToolPublicationLane::HostOnly] },
        ArtifactToolPublicationContract { tool_id: simulation::ADOPT_ACTION_ID, lanes: &[ArtifactToolPublicationLane::HostOnly] },
        ArtifactToolPublicationContract { tool_id: simulation::CONFIGURE_ACTION_ID, lanes: &[ArtifactToolPublicationLane::HostOnly] },
    ];
}
//#endregion 🧵️RetainedCommands

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
    protocol::Edit {
        id: id.clone(),
        actor: Some(authority.actor().to_string()),
        forwards,
        inverse,
        mutation_meta: vec![protocol::MutationMeta {
            mutation_id: Some(protocol::MutationId(format!("{id}#0"))),
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
        Ok(store::ArtifactStoreOneItemFootprint { work_items: 1, retained_bytes: store::ARTIFACT_STORE_ONE_ITEM_MAXIMUM_BYTES })
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
        use protocol::{Mutation as _, MutationDiff as _};
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
    type Config = NoConfig;
    type ConfigMutation = NoConfigMutation;
    type Draft = NoDraft;
    type DraftMutation = NoDraftMutation;
    type Presence = NoPresence;
    type PresenceMutation = NoPresenceMutation;
    type Transient = NoTransient;
    type TransientMutation = NoTransientMutation;
    type Command = EnergyModelEditorCommand;

    const DIALECT: Dialect = MODEL_DIALECT;
    const DOCUMENT_SCHEMA: &'static str = ENERGY_MODEL_DOCUMENT_SCHEMA;

    semio_framework_plugin::bounded_first_step_tool_proofs! {
        owner: EditorApp<EnergyModelEditor>,
        owner_file: "✏️s/🔌️plugins/🔋️energy/🗿️artifacts/🔋️model/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs",
        controller: "s.energy.model@1/*#editor",
        document_schema: "energy.model",
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
            "start-energy-simulation",
            "cancel-energy-simulation",
            "retry-energy-simulation",
            "discard-energy-simulation",
            "adopt-energy-simulation",
            "configure-energy-simulation"
        ]
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
        let payload = ArtifactRetainedCommandPayload::try_new_with_context(
            *request.command,
            request.snapshot,
            request.config,
            request.history,
            request.interaction_state,
            request.interaction_hover,
            request.context,
            operation_context,
            request.completion,
            EnergyModelEditorCommand::action_id,
            ENERGY_MODEL_RETAINED_RAW_BYTES,
            1,
            work,
        )?;
        Ok(Some(semio_framework_plugin::ToolOperationSpec::new(request.controller_id, request.tool_id, request.payload_schema_id, payload, request.operation)))
    }

    fn build_artifact_store_one_item_preparation_factory() -> Option<std::sync::Arc<dyn store::ArtifactStoreOneItemPreparationFactory<Self::Snapshot, Self::Mutation>>> {
        Some(std::sync::Arc::new(EnergyModelStorePreparationFactory))
    }

    fn initial_snapshot() -> EnergyModelSnapshot {
        EnergyModelSnapshot::default()
    }

    fn mounted_job_maintenance_step(instance_id: u32, maximum_items: usize, maximum_bytes: usize) -> Result<semio_framework_plugin::PluginCloseStep, Fault> {
        Ok(simulation_session::maintenance_step(instance_id, maximum_items, maximum_bytes))
    }

    fn mounted_job_close_step(instance_id: u32, maximum_items: usize, maximum_bytes: usize) -> Result<semio_framework_plugin::PluginCloseStep, Fault> {
        Ok(simulation_session::close_step(instance_id, maximum_items, maximum_bytes))
    }

    fn mounted_jobs_terminal_is_empty(instance_id: u32) -> bool {
        simulation_session::terminal_is_empty(instance_id)
    }

    fn mounted_job_prepare_snapshot_read(operation: semio_framework_plugin::AppRenderOperationContext, snapshot: &Self::Snapshot) -> bool {
        simulation_session::prepare_snapshot_read(operation, snapshot)
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
        _draft: &DraftView<'_, Self::Draft>,
        _engines: &EngineHandles,
    ) -> Result<Emit<Self::Mutation, Self::ConfigMutation, Self::DraftMutation>, Fault> {
        reduce(command, doc)
    }

    fn pending_effects(doc: &ArtifactView<'_, EnergyModelSnapshot>, _cfg: &ConfigView<'_, NoConfig>) -> Vec<semio_framework_plugin::kernel::Effect> {
        simulation_session::reconcile(doc)
    }

    fn render(body_key: &str, doc: &ArtifactView<'_, Self::Snapshot>, _cfg: &ConfigView<'_, Self::Config>) -> UiAssemblyResult<ComponentTree> {
        let render = doc.render_operation();
        let node = match body_key {
            structure::BODY_KEY => structure::render(doc.snapshot)?,
            zones::BODY_KEY => zones::render(doc.snapshot)?,
            simulation::BODY_KEY => {
                let settings = simulation_session::session_settings(render);
                simulation_session::with_projection(render, |projection| simulation::render(projection, settings, &doc.snapshot.model, settings.locale_de))
            }
            _ => semio_framework_plugin::built_text_node(Label::data(format!("Unknown body: {body_key}"))).map_err(|_| semio_framework_plugin::PluginAssemblyError::new("energy.model.render", "the unknown-body label could not be assembled"))?,
        };
        Ok(semio_framework_plugin::built_to_component_tree(node))
    }
}
//#endregion 🔖️Editor

//#region 📚️Examples
/// 📚️ The bundled `📚️examples/🎬️<slug>` fixtures, table-driven — a new example is ONE row here plus
/// its `#[path]` mount in `📦️packages/🦀️rust/🦀️.rs`. Every row names a module that exports the same
/// three items (`ID`, `LABEL_EN`, `model()`), so the table cannot go out of step with a leaf.
/// The ANSI/ASHRAE 140 case ids are proper nouns and read identically in both authored languages;
/// `Demo` likewise. That is a real translation decision, not a missing one.
fn example_rows() -> Vec<(&'static str, &'static str, crate::model::Model)> {
    use crate::examples::{
        art_model_bestest_600, art_model_bestest_600ff, art_model_bestest_610, art_model_bestest_620, art_model_bestest_630, art_model_bestest_640, art_model_bestest_650, art_model_bestest_900, art_model_bestest_900ff,
        art_model_bestest_910, art_model_bestest_920, art_model_bestest_930, art_model_bestest_940, art_model_bestest_950, art_model_demo,
    };
    vec![
        (art_model_demo::ID, art_model_demo::LABEL_EN, art_model_demo::model()),
        (art_model_bestest_600::ID, art_model_bestest_600::LABEL_EN, art_model_bestest_600::model()),
        (art_model_bestest_600ff::ID, art_model_bestest_600ff::LABEL_EN, art_model_bestest_600ff::model()),
        (art_model_bestest_610::ID, art_model_bestest_610::LABEL_EN, art_model_bestest_610::model()),
        (art_model_bestest_620::ID, art_model_bestest_620::LABEL_EN, art_model_bestest_620::model()),
        (art_model_bestest_630::ID, art_model_bestest_630::LABEL_EN, art_model_bestest_630::model()),
        (art_model_bestest_640::ID, art_model_bestest_640::LABEL_EN, art_model_bestest_640::model()),
        (art_model_bestest_650::ID, art_model_bestest_650::LABEL_EN, art_model_bestest_650::model()),
        (art_model_bestest_900::ID, art_model_bestest_900::LABEL_EN, art_model_bestest_900::model()),
        (art_model_bestest_900ff::ID, art_model_bestest_900ff::LABEL_EN, art_model_bestest_900ff::model()),
        (art_model_bestest_910::ID, art_model_bestest_910::LABEL_EN, art_model_bestest_910::model()),
        (art_model_bestest_920::ID, art_model_bestest_920::LABEL_EN, art_model_bestest_920::model()),
        (art_model_bestest_930::ID, art_model_bestest_930::LABEL_EN, art_model_bestest_930::model()),
        (art_model_bestest_940::ID, art_model_bestest_940::LABEL_EN, art_model_bestest_940::model()),
        (art_model_bestest_950::ID, art_model_bestest_950::LABEL_EN, art_model_bestest_950::model()),
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
            let snapshot = crate::artifacts::model::energy_snapshot_with_state(ENERGY_MODEL_DOCUMENT_SCHEMA, &model, None);
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

//#region 🔖️Manifest
/// 🧱️ The editor's `AppDefinition`. Every id in [`ENERGY_MODEL_RETAINED_TOOL_IDS`] is classified
/// `Migrated` here — `set-node`/`set-cell` are already stamped by their kits, the other sixteen are
/// classified explicitly, and `EditorBuilder::try_build_definition` panics on any `Unclassified` id.
/// Each window owns its own action list (`structure::actions`/`zones::actions`/the simulation
/// window's `definition`), so this function never restates one.
pub fn create_energy_model_editor() -> semio_framework_plugin::AppDefinition {
    let mut builder = Editor::builder(MODEL_DIALECT)
        .document(["semio", "energy", "model"])
        .icon_id("battery")
        .mode_def(edit::definition())
        .default_mode_id(edit::ENERGY_MODEL_EDIT_MODE_ID)
        .window_kind_def(structure::definition())
        .window_kind_def(zones::definition())
        .window_kind_def(simulation::definition())
        .mutation(SET_ACTIVE_EXAMPLE_ACTION_ID, LocalizedLabel::native("Load example", "Beispiel laden"))
        .action_args(SET_ACTIVE_EXAMPLE_ACTION_ID, vec![semio_framework_plugin::ActionArgDef::select("exampleId", LocalizedLabel::native("Example", "Beispiel"), example_options()).required()])
        .keybinding("mod+shift+n", CREATE_ZONE_ACTION_ID)
        .keybinding("mod+shift+s", CREATE_SURFACE_ACTION_ID)
        .keybinding("mod+shift+g", SET_SITE_ACTION_ID)
        .keybinding("mod+enter", simulation::START_ACTION_ID)
        .keybinding("mod+period", simulation::CANCEL_ACTION_ID)
        .keybinding("mod+shift+enter", simulation::ADOPT_ACTION_ID)
        .default_layout(edit::layout());
    for tool_id in ENERGY_MODEL_RETAINED_TOOL_IDS {
        builder = builder.action_interactive_job(*tool_id, InteractiveJobClassification::Migrated);
    }
    builder.build_definition()
}
//#endregion 🔖️Manifest

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::BTreeSet;

    fn definition() -> semio_framework_plugin::AppDefinition {
        create_energy_model_editor()
    }

    #[semio_framework_async_macros::async_test]
    async fn create_energy_model_editor_builds_a_definition_for_the_editor_role() {
        let def = definition();
        assert_eq!(def.role, semio_framework_plugin::AppRole::Editor);
        assert_eq!(def.dialect, MODEL_DIALECT.into());
    }

    #[semio_framework_async_macros::async_test]
    async fn editor_dialect_matches_the_artifact_coordinate() {
        assert_eq!(<EnergyModelEditor as ArtifactEditor>::DIALECT, MODEL_DIALECT);
    }

    #[semio_framework_async_macros::async_test]
    async fn editor_declares_all_three_windows() {
        let def = definition();
        for id in [structure::WINDOW_KIND_ID, zones::WINDOW_KIND_ID, simulation::WINDOW_KIND_ID] {
            assert!(def.window_kinds.iter().any(|window| window.id == id), "missing window kind {id}");
        }
    }

    /// 🧵️ The framework demands set equality between `TOOL_JOB_IDS`, the `Migrated` action ids, the
    /// factory's `TOOL_IDS`, its per-tool publication contracts and the proof rows. Any drift is
    /// `interactive-job.catalog-incomplete`/`catalog-authority` at app construction, so it is checked
    /// here rather than discovered at dispatch.
    #[semio_framework_async_macros::async_test]
    async fn retained_roster_is_exact_and_exhaustive() {
        use semio_framework_plugin::ArtifactOwnedToolJobFactory;
        let roster = ENERGY_MODEL_RETAINED_TOOL_IDS.iter().copied().collect::<BTreeSet<_>>();
        assert_eq!(roster.len(), ENERGY_MODEL_RETAINED_TOOL_IDS.len(), "the retained roster repeats a tool id");
        assert_eq!(roster, <EnergyModelEditorCommand as protocol::OpBinary>::TOOL_JOB_IDS.iter().copied().collect::<BTreeSet<_>>());
        assert_eq!(roster, <EnergyModelCommandJobFactory as ArtifactOwnedToolJobFactory>::TOOL_IDS.iter().copied().collect::<BTreeSet<_>>());
        assert_eq!(roster, <EnergyModelCommandJobFactory as ArtifactOwnedToolJobFactory>::PUBLICATION_CONTRACTS.iter().map(|contract| contract.tool_id).collect::<BTreeSet<_>>());
        assert_eq!(roster, <EnergyModelEditor as ArtifactEditor>::bounded_first_step_tool_proofs().iter().map(|proof| proof.tool_id).collect::<BTreeSet<_>>());
        let def = definition();
        let migrated = def
            .window_kinds
            .iter()
            .flat_map(|window| window.actions.iter())
            .filter(|action| action.semantics.execution.interactive_job == InteractiveJobClassification::Migrated)
            .map(|action| action.id.as_str())
            .collect::<BTreeSet<_>>();
        assert!(roster.is_subset(&migrated), "unclassified retained tools: {:?}", roster.difference(&migrated).collect::<Vec<_>>());
    }

    #[semio_framework_async_macros::async_test]
    async fn every_declared_action_is_classified_and_resolves_to_a_command() {
        for action in definition().window_kinds.iter().flat_map(|window| window.actions.iter()) {
            assert_ne!(action.semantics.execution.interactive_job, InteractiveJobClassification::Unclassified, "action {} is unclassified", action.id);
        }
        for tool_id in ENERGY_MODEL_RETAINED_TOOL_IDS {
            let command = <EnergyModelEditor as ArtifactEditor>::command_from_action(tool_id, None).unwrap_or_else(|error| panic!("action {tool_id} has no command: {error:?}"));
            assert_eq!(command.action_id(), *tool_id);
        }
    }

    #[semio_framework_async_macros::async_test]
    async fn document_verbs_publish_to_the_artifact_lane_and_session_verbs_do_not() {
        use semio_framework_plugin::ArtifactOwnedToolJobFactory;
        for contract in <EnergyModelCommandJobFactory as ArtifactOwnedToolJobFactory>::PUBLICATION_CONTRACTS {
            let expected = if ENERGY_MODEL_DOCUMENT_TOOL_IDS.contains(&contract.tool_id) { ArtifactToolPublicationLane::Artifact } else { ArtifactToolPublicationLane::HostOnly };
            assert_eq!(contract.lanes, &[expected], "wrong publication lane for {}", contract.tool_id);
        }
        assert!(<EnergyModelEditor as ArtifactEditor>::build_artifact_store_one_item_preparation_factory().is_some(), "the artifact lane needs its one-item preparation factory");
    }

    #[semio_framework_async_macros::async_test]
    async fn examples_are_registered_for_the_shell_picker() {
        let sources = examples();
        assert_eq!(sources.len(), example_rows().len());
        assert!(sources.len() >= 15, "the demo plus the fifteen ANSI/ASHRAE 140 cases must all reach the picker");
        let mut seen = BTreeSet::new();
        for source in &sources {
            assert!(seen.insert(source.id().to_string()), "duplicate example id {}", source.id());
            assert!(!source.document_json().trim().is_empty(), "example {} carries no document", source.id());
            assert!(example_model(source.id()).is_some(), "example {} has no model behind its picker row", source.id());
        }
        let options = example_options().into_iter().map(|option| option.value).collect::<Vec<_>>();
        assert_eq!(options, sources.iter().map(|source| source.id().to_string()).collect::<Vec<_>>(), "the picker's option list must be the example list");
        let definition = create_energy_model_editor();
        let picker = definition
            .window_kinds
            .iter()
            .flat_map(|window| window.actions.iter())
            .find(|action| action.id == SET_ACTIVE_EXAMPLE_ACTION_ID)
            .expect("the shell dispatches setActiveExample, so the app must declare it on a window");
        assert_eq!(picker.args.len(), 1);
        assert_eq!(picker.args[0].id, "exampleId");
    }

    /// 📂️ Loading an example is a whole-document swap, and this artifact's vocabulary has NO
    /// whole-document-replace kind on purpose. So the verb publishes a `kernel::Effect::LoadDocument`
    /// (the host's `ArtifactStore::reset` route, outside undo history) and NOT a single mutation —
    /// asserted for every bundled example, so a new row can never silently fall back on the seam.
    #[semio_framework_async_macros::async_test]
    async fn loading_an_example_swaps_the_document_through_an_effect_and_never_a_mutation() {
        let snapshot = EnergyModelSnapshot::default();
        let history = HistoryView::default();
        let doc = ArtifactView::new(&snapshot, &history);
        for (id, _, model) in example_rows() {
            let emit = reduce(&EnergyModelEditorCommand::SetActiveExample { example_id: id.to_string() }, &doc).unwrap_or_else(|error| panic!("example {id} must load: {error:?}"));
            assert!(emit.artifact_mutations.is_empty(), "example {id} must not reach the mutation seam");
            let [semio_framework_plugin::kernel::Effect::LoadDocument { pack, spr }] = emit.effects.as_slice() else { panic!("example {id} must emit exactly one LoadDocument effect") };
            assert!(!pack.is_empty() && !spr.is_empty(), "example {id} emitted an empty document");
            let loaded = <EnergyModelSnapshot as store::ArtifactPack>::decode_pack(pack).expect("the emitted pack decodes");
            assert_eq!(loaded.model, model, "example {id} loaded a different model than its own leaf declares");
        }
        let fault = reduce(&EnergyModelEditorCommand::SetActiveExample { example_id: "nonsense".into() }, &doc).expect_err("an unknown example id is refused");
        assert_eq!(fault.code.as_str(), "mutation.target-missing");
    }

    fn model_with_one_zone() -> crate::model::Model {
        let mut model = crate::model::Model::default();
        model.zones.push(Zone { id: EntityId(1), name: "Zone 1".into(), volume_m3: 100.0, multiplier: 1, conditioned: true, part_of_total_floor_area: true });
        model
    }

    /// 🧫️ One zone, two constructions, one material, two constant schedules, one surface and one
    /// thermostat — the smallest document in which every authored document verb has a real target.
    fn populated_model() -> crate::model::Model {
        let mut model = model_with_one_zone();
        model.constructions.push(crate::model::Construction { id: EntityId(1), name: "Wall".into(), layer_material_ids: vec![EntityId(1)] });
        model.constructions.push(crate::model::Construction { id: EntityId(2), name: "Roof".into(), layer_material_ids: vec![EntityId(1)] });
        model.materials.push(Material { id: EntityId(1), name: "Concrete".into(), thickness_m: 0.1, conductivity_w_m_k: 1.0, density_kg_m3: 2000.0, specific_heat_j_kg_k: 900.0, thermal_absorptance: 0.9, solar_absorptance: 0.6, visible_absorptance: 0.6 });
        model.surfaces.push(Surface {
            id: EntityId(1),
            name: "South".into(),
            zone_id: EntityId(1),
            class: SurfaceClass::ExteriorWall,
            vertices_m: vec![[0.0, 0.0, 0.0], [1.0, 0.0, 0.0], [1.0, 0.0, 1.0]],
            construction_id: EntityId(1),
            outside_boundary_condition: OutsideBoundary::OutdoorAir,
            sun_exposed: true,
            wind_exposed: true,
            multiplier: 1,
        });
        model.schedules.constants.push(crate::schedule::ConstantSchedule { id: ScheduleId(1), value: 20.0 });
        model.schedules.constants.push(crate::schedule::ConstantSchedule { id: ScheduleId(2), value: 27.0 });
        model.thermostats.push(Thermostat { id: EntityId(1), zone_id: EntityId(1), heating_setpoint_schedule_id: ScheduleId(1), cooling_setpoint_schedule_id: ScheduleId(2), heating_throttle_range_k: 2.0, cooling_throttle_range_k: 2.0 });
        model
    }

    fn snapshot_of(model: &crate::model::Model) -> EnergyModelSnapshot {
        crate::artifacts::model::energy_snapshot_with_state(ENERGY_MODEL_DOCUMENT_SCHEMA, model, None)
    }

    fn applied(snapshot: &EnergyModelSnapshot, command: &EnergyModelEditorCommand) -> crate::model::Model {
        use protocol::{Mutation as _, MutationDiff as _};
        let history = HistoryView::default();
        let doc = ArtifactView::new(snapshot, &history);
        let emit = reduce(command, &doc).expect("document verb reduces");
        let mut next = snapshot.clone();
        for mutation in &emit.artifact_mutations {
            next = protocol::MutationDiff::apply(mutation.diff(&next).diff(), &next).expect("diff applies");
        }
        next.model
    }

    /// 🧬️ The verbs whose semantic mutation kinds W-D0's model-root group has already landed —
    /// these must produce real granular steps, never a refusal.
    #[semio_framework_async_macros::async_test]
    async fn landed_semantic_kinds_produce_granular_mutations() {
        let snapshot = snapshot_of(&model_with_one_zone());
        let renamed = applied(&snapshot, &EnergyModelEditorCommand::RenameZone { zone: 1, new_name: "Loft".into() });
        assert_eq!(renamed.zones[0].name, "Loft");
        let sited = applied(&snapshot, &EnergyModelEditorCommand::SetSite { latitude_deg: 39.74, longitude_deg: -105.18, elevation_m: 1609.0, time_zone_hours: -7.0, north_axis_deg: 0.0 });
        assert!((sited.site.latitude_deg - 39.74).abs() < 1e-9);
        let period = applied(&snapshot, &EnergyModelEditorCommand::SetRunPeriod { start_month: 2, start_day: 1, end_month: 2, end_day: 28 });
        assert_eq!((period.run_period.start_month, period.run_period.end_day), (2, 28));
        let field = applied(&snapshot, &EnergyModelEditorCommand::SetStructureField { field: "name".into(), value: "BESTEST 600".into() });
        assert_eq!(field.name, "BESTEST 600");
        let cell = applied(&snapshot, &EnergyModelEditorCommand::SetZoneCell { row: 0, column: "volumeM3".into(), value: "129.6".into() });
        assert!((cell.zones[0].volume_m3 - 129.6).abs() < 1e-9);
    }

    /// 🧬️ The verbs that were waiting on the 100s–900s mutation groups. Those leaves have landed
    /// (`create-zone`, `delete-zone`, `create-surface`, `delete-surface`, `change-surface-*`,
    /// `change-material-*`, `change-thermostat-*`), so this is now a strict ROUND TRIP: each verb
    /// must produce granular semantic steps whose application reproduces exactly the edit the user
    /// asked for. A `mutation.kind-unavailable` here is a real regression, not a pending group.
    #[semio_framework_async_macros::async_test]
    async fn every_document_verb_round_trips_through_the_granular_vocabulary() {
        let snapshot = snapshot_of(&populated_model());

        let created = applied(&snapshot, &EnergyModelEditorCommand::CreateZone { name: "Attic".into(), volume_m3: 40.0, multiplier: 1, conditioned: false });
        assert_eq!(created.zones.len(), 2);
        assert_eq!(created.zones[1].name, "Attic");
        assert!(!created.zones[1].conditioned);

        let with_surface = applied(&snapshot, &EnergyModelEditorCommand::CreateSurface { name: "North".into(), zone: 1, construction: 1, class: "roof".into() });
        assert_eq!(with_surface.surfaces.len(), 2);
        assert_eq!(with_surface.surfaces[1].name, "North");
        assert_eq!(with_surface.surfaces[1].class, SurfaceClass::Roof);

        let without_surface = applied(&snapshot, &EnergyModelEditorCommand::DeleteSurface { surface: 1 });
        assert!(without_surface.surfaces.is_empty());

        let reconstructed = applied(&snapshot, &EnergyModelEditorCommand::AssignSurfaceConstruction { surface: 1, construction: 2 });
        assert_eq!(reconstructed.surfaces[0].construction_id, EntityId(2));

        let insulated = applied(&snapshot, &EnergyModelEditorCommand::SetMaterialProperty { material: 1, property: "conductivityWMK".into(), value: 0.04 });
        assert!((insulated.materials[0].conductivity_w_m_k - 0.04).abs() < 1e-12);

        let retuned = applied(
            &snapshot,
            &EnergyModelEditorCommand::SetThermostatSetpoints { thermostat: 1, heating_schedule: 2, cooling_schedule: 1, heating_throttle_range_k: 1.0, cooling_throttle_range_k: 1.5 },
        );
        assert_eq!(retuned.thermostats[0].heating_setpoint_schedule_id, ScheduleId(2));
        assert!((retuned.thermostats[0].cooling_throttle_range_k - 1.5).abs() < 1e-12);

        let mut freed = populated_model();
        freed.surfaces.clear();
        freed.thermostats.clear();
        let dropped = applied(&snapshot_of(&freed), &EnergyModelEditorCommand::DeleteZone { zone: 1 });
        assert!(dropped.zones.is_empty());
    }

    #[semio_framework_async_macros::async_test]
    async fn a_zone_still_referenced_by_a_surface_cannot_be_deleted() {
        let mut model = model_with_one_zone();
        model.constructions.push(crate::model::Construction { id: EntityId(1), name: "Wall".into(), layer_material_ids: Vec::new() });
        model.surfaces.push(Surface {
            id: EntityId(1),
            name: "South".into(),
            zone_id: EntityId(1),
            class: SurfaceClass::ExteriorWall,
            vertices_m: vec![[0.0, 0.0, 0.0], [1.0, 0.0, 0.0], [1.0, 0.0, 1.0]],
            construction_id: EntityId(1),
            outside_boundary_condition: OutsideBoundary::OutdoorAir,
            sun_exposed: true,
            wind_exposed: true,
            multiplier: 1,
        });
        let snapshot = snapshot_of(&model);
        let history = HistoryView::default();
        let doc = ArtifactView::new(&snapshot, &history);
        let fault = reduce(&EnergyModelEditorCommand::DeleteZone { zone: 1 }, &doc).expect_err("a referenced zone is refused");
        assert_eq!(fault.code.as_str(), "mutation.target-in-use");
    }

    /// 🧱️ The SI guards run BEFORE the seam, so they are observable whatever the vocabulary state.
    #[semio_framework_async_macros::async_test]
    async fn out_of_range_payloads_are_refused_before_they_reach_the_vocabulary() {
        let mut model = model_with_one_zone();
        model.materials.push(Material { id: EntityId(1), name: "Concrete".into(), thickness_m: 0.1, conductivity_w_m_k: 1.0, density_kg_m3: 2000.0, specific_heat_j_kg_k: 900.0, thermal_absorptance: 0.9, solar_absorptance: 0.6, visible_absorptance: 0.6 });
        let snapshot = snapshot_of(&model);
        let history = HistoryView::default();
        let doc = ArtifactView::new(&snapshot, &history);
        for command in [
            EnergyModelEditorCommand::SetMaterialProperty { material: 1, property: "conductivityWMK".into(), value: -1.0 },
            EnergyModelEditorCommand::SetSite { latitude_deg: 120.0, longitude_deg: 0.0, elevation_m: 0.0, time_zone_hours: 0.0, north_axis_deg: 0.0 },
            EnergyModelEditorCommand::SetRunPeriod { start_month: 13, start_day: 1, end_month: 12, end_day: 31 },
            EnergyModelEditorCommand::CreateZone { name: "Void".into(), volume_m3: 0.0, multiplier: 1, conditioned: true },
        ] {
            let fault = reduce(&command, &doc).expect_err("an out-of-range payload is refused");
            assert_eq!(fault.code.as_str(), "mutation.invalid-payload", "{} failed for the wrong reason", command.action_id());
        }
        let fault = reduce(&EnergyModelEditorCommand::SetMaterialProperty { material: 9, property: "conductivityWMK".into(), value: 0.04 }, &doc).expect_err("an unknown material is refused");
        assert_eq!(fault.code.as_str(), "mutation.target-missing");
    }

    //#region 🧵️DispatchLaw
    /// 🧬️ `testkit::new_app_with_registry` needs the `App { definition, examples }` shape, which is
    /// also the exact shape the plugin root registers — so a drift between the two would be caught
    /// here rather than at boot.
    pub(crate) fn energy_model_manifest_for_testkit() -> semio_framework_plugin::App {
        semio_framework_plugin::App { definition: create_energy_model_editor(), examples: examples().into_iter().map(Into::into).collect() }
    }

    /// 🧵️ A registry-backed app bound to the live runtime instance `meta("local")` addresses. The
    /// registry-LESS `testkit::new_app` cannot be used here: it builds an app with no
    /// `AppActionRegistry`, so `migrated_tool_ids()` is empty and `validate_tool_job_rows` fails
    /// closed with `interactive-job.catalog-authority` the moment any proof is declared.
    async fn dispatchable_app() -> semio_framework_plugin::VcsArtifactApp<EditorApp<EnergyModelEditor>> {
        use semio_framework_plugin::PluginApp as _;
        let mut app = semio_framework_plugin::testkit::new_app_with_registry::<EditorApp<EnergyModelEditor>>(energy_model_manifest_for_testkit).await;
        app.bind_instance_id(semio_framework_plugin::testkit::meta("local").instance_id).await;
        app
    }

    /// 🧵️ THE dispatch law: every declared verb must reach this editor's own reducer. A domain fault
    /// (`mutation.target-missing` for an id absent from the empty default document,
    /// `mutation.kind-unavailable` for a vocabulary group still landing, or an `energy.session.*`
    /// rejection for a session verb with no live run) is a PASS — it proves the command was
    /// constructed, admitted and reduced. Any `interactive-job.*` code is a FAIL: it means the verb
    /// never reached the app at all (missing factory, missing proof, unsupported publication lane,
    /// unclassified action).
    #[semio_framework_async_macros::async_test]
    async fn every_declared_verb_dispatches_without_an_interactive_job_fault() {
        use semio_framework_plugin::{effective_action_args, DslValue};
        use semio_framework_plugin::PluginApp as _;
        let definition = create_energy_model_editor();
        let mut app = dispatchable_app().await;
        let mut reached = 0;
        for tool_id in ENERGY_MODEL_RETAINED_TOOL_IDS {
            let action = definition
                .window_kinds
                .iter()
                .flat_map(|window| window.actions.iter())
                .find(|action| action.id == *tool_id)
                .unwrap_or_else(|| panic!("action {tool_id} is declared on no window kind"));
            let staged = effective_action_args(&action.args, &DslValue::Object(Vec::new()), None);
            match app.handle_action(tool_id, Some(&staged), &semio_framework_plugin::testkit::meta("local")).await {
                Ok(_) => reached += 1,
                Err(fault) => {
                    assert!(!fault.code.as_str().starts_with("interactive-job."), "action {tool_id} never reached the app: {} — {}", fault.code.as_str(), fault.message);
                    reached += 1;
                }
            }
        }
        assert_eq!(reached, ENERGY_MODEL_RETAINED_TOOL_IDS.len());
    }

    /// 🧵️ The positive half of the law: a verb addressing real document state dispatches with NO
    /// fault at all, and the operation is genuinely admitted — either the reduction published its
    /// mutations inline or the retained tool operation is now live on the app.
    #[semio_framework_async_macros::async_test]
    async fn renaming_the_model_dispatches_cleanly_through_the_real_action_route() {
        use semio_framework_plugin::DslValue;
        use semio_framework_plugin::PluginApp as _;
        let mut app = dispatchable_app().await;
        let args = DslValue::Object(vec![("id".to_string(), DslValue::String("name".to_string())), ("value".to_string(), DslValue::String("BESTEST 600".to_string()))]);
        let result = app.handle_action(SET_NODE_ACTION_ID, Some(&args), &semio_framework_plugin::testkit::meta("local")).await.expect("set-node dispatches without a fault");
        assert!(!result.mutations.is_empty() || app.has_pending_typed_operations(), "the rename neither published nor retained an operation");
    }
    //#endregion 🧵️DispatchLaw

    #[semio_framework_async_macros::async_test]
    async fn a_command_round_trips_through_its_own_text_and_binary_codec() {
        use protocol::{OpBinary as _, OpText as _};
        let command = EnergyModelEditorCommand::RenameZone { zone: 7, new_name: "Kitchen".into() };
        assert_eq!(EnergyModelEditorCommand::parse_op(&command.print_op()).expect("text round trip"), command);
        assert_eq!(EnergyModelEditorCommand::decode_op(&command.encode_op().expect("encode")).expect("binary round trip"), command);
    }
}
//#endregion 🧪️Tests
