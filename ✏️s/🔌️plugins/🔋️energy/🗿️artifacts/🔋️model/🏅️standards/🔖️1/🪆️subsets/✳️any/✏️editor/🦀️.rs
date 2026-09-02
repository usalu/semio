//! 🔋️ Energy model editor — the FIRST authored `ArtifactEditor` surface for `s.energy.model@1/*`
//! (ticket 26/08/16/ARTIFACT-VIEWERS-AND-EDITORS-PER-SUBSET). Energy had zero document apps, so there
//! is no app tree to migrate — this is authored fresh straight against `EnergyModelSnapshot`'s own
//! composed shape (`structure`: the whole `crate::model::Model` folded into one `s.stdio.semio.value`
//! tree; `zones`: a derived per-zone table, see the artifact root's `🔖️Composition` region). The
//! structure and zone windows edit composed children; the mounted simulation window projects the
//! retained worker session without materializing numerical state on the UI thread. Edit windows'
//! commands funnel through the single mutation this artifact declares, `EnergyModelMutation::
//! ReplaceModel` (decode the working `Model`, apply the edit, re-encode — the same "swap `structure`+
//! `zones` together" shape every other `ReplaceModel` caller uses).

use crate::artifacts::model::mutations::replace_model::ReplaceModel;
use crate::artifacts::model::{EnergyModelMutation, EnergyModelSnapshot, ENERGY_MODEL_DOCUMENT_SCHEMA, MODEL_DIALECT};
use crate::editor::model::modes::edit;
use crate::editor::model::modes::edit::windows::{simulation, structure, zones};
use crate::energy_simulation_session::{self as simulation_session, EnergySimulationConfigProjection, EnergySimulationEventKind, EnergySimulationRequestIdentity};
use semio_framework_plugin::{
    ArtifactEditor, ArtifactView, ComponentTree, ConfigView, Dialect, DraftView, Editor, Emit, Fault, Label, NoConfig, NoConfigMutation, NoDraft, NoDraftMutation, NoPresence, NoPresenceMutation, NoTransient, NoTransientMutation,
};
use semio_framework_value_derive::{FromValue as FromValueDerive, ToValue as ToValueDerive};
use store::EngineHandles;

//#region 🔖️Command
/// ✏️ The editor's typed command channel for structure/zone edits and event-sourced mounted
/// Energy simulation start/cancel/retry/discard/adopt actions. The structure first pass
/// `SetStructureField` only reaches the two top-level scalars the tree renders as addressable nodes
/// (`name`/`version`) — the tree's other nodes are a read overview of the model's collection sizes,
/// not yet individually addressable edit targets; documented honestly, not silently incomplete.
#[derive(Clone, Debug, PartialEq, ToValueDerive, FromValueDerive, dsl::DslOps)]
pub enum EnergyModelEditorCommand {
    #[dsl(key = "set-structure-field")]
    SetStructureField { field: String, value: String },
    #[dsl(key = "set-zone-cell")]
    SetZoneCell { row: u32, column: String, value: String },
    #[dsl(key = "start-energy-simulation")]
    StartSimulation {
        request: u64,
        locale: String,
        checkpoint_token: u64,
        zone_timestep_minutes: u32,
        system_timestep_minutes: u32,
        warmup_days: u32,
        run_period_start_month: u8,
        run_period_start_day: u8,
        run_period_end_month: u8,
        run_period_end_day: u8,
    },
    #[dsl(key = "cancel-energy-simulation")]
    CancelSimulation { request: u64, operation: u64, generation: u64, config_digest: u64 },
    #[dsl(key = "retry-energy-simulation")]
    RetrySimulation { request: u64, operation: u64, generation: u64, config_digest: u64 },
    #[dsl(key = "discard-energy-simulation")]
    DiscardSimulation { request: u64, operation: u64, generation: u64, config_digest: u64 },
    #[dsl(key = "adopt-energy-simulation")]
    AdoptSimulation { request: u64, operation: u64, generation: u64, config_digest: u64 },
}

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

    async fn initial_snapshot() -> EnergyModelSnapshot {
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

    async fn command_id(command: &EnergyModelEditorCommand) -> &'static str {
        match command {
            EnergyModelEditorCommand::SetStructureField { .. } => "set-structure-field",
            EnergyModelEditorCommand::SetZoneCell { .. } => "set-zone-cell",
            EnergyModelEditorCommand::StartSimulation { .. } => simulation::START_ACTION_ID,
            EnergyModelEditorCommand::CancelSimulation { .. } => simulation::CANCEL_ACTION_ID,
            EnergyModelEditorCommand::RetrySimulation { .. } => simulation::RETRY_ACTION_ID,
            EnergyModelEditorCommand::DiscardSimulation { .. } => simulation::DISCARD_ACTION_ID,
            EnergyModelEditorCommand::AdoptSimulation { .. } => simulation::ADOPT_ACTION_ID,
        }
    }

    /// ✏️ Decodes the working `crate::model::Model` behind the snapshot's composed children, applies
    /// the one addressed field/cell, then re-encodes the WHOLE model as `ReplaceModel::new_model_json`
    /// — `structure`/`zones` are always regenerated together (rule 6, `🧬️mutations/🦀️.rs`'s
    /// own doc comment), never targeted independently. An out-of-range row or unknown column/field is
    /// a documented no-op (`Emit::default()`), never a panic.
    async fn handle(
        command: &Self::Command,
        doc: &ArtifactView<'_, Self::Snapshot>,
        _cfg: &ConfigView<'_, Self::Config>,
        _interaction: &semio_framework_plugin::app::InteractionView<'_>,
        _draft: &DraftView<'_, Self::Draft>,
        _engines: &EngineHandles,
    ) -> Result<Emit<Self::Mutation>, Fault> {
        let event = match command {
            EnergyModelEditorCommand::StartSimulation { request, locale, checkpoint_token, zone_timestep_minutes, system_timestep_minutes, warmup_days, run_period_start_month, run_period_start_day, run_period_end_month, run_period_end_day } => {
                if locale != "en" && locale != "de" {
                    return Err(Fault::from("energy simulation requires explicit locale en or de"));
                }
                Some(EnergySimulationEventKind::Start {
                    request: *request,
                    config: EnergySimulationConfigProjection {
                        locale_de: locale == "de",
                        checkpoint_token: *checkpoint_token,
                        zone_timestep_minutes: *zone_timestep_minutes,
                        system_timestep_minutes: *system_timestep_minutes,
                        warmup_days: *warmup_days,
                        run_period_start_month: *run_period_start_month,
                        run_period_start_day: *run_period_start_day,
                        run_period_end_month: *run_period_end_month,
                        run_period_end_day: *run_period_end_day,
                    },
                })
            }
            EnergyModelEditorCommand::CancelSimulation { request, operation, generation, config_digest } => {
                Some(EnergySimulationEventKind::Cancel(EnergySimulationRequestIdentity { request: *request, operation: *operation, generation: *generation, config_digest: *config_digest }))
            }
            EnergyModelEditorCommand::RetrySimulation { request, operation, generation, config_digest } => {
                Some(EnergySimulationEventKind::Retry(EnergySimulationRequestIdentity { request: *request, operation: *operation, generation: *generation, config_digest: *config_digest }))
            }
            EnergyModelEditorCommand::DiscardSimulation { request, operation, generation, config_digest } => {
                Some(EnergySimulationEventKind::Discard(EnergySimulationRequestIdentity { request: *request, operation: *operation, generation: *generation, config_digest: *config_digest }))
            }
            EnergyModelEditorCommand::AdoptSimulation { request, operation, generation, config_digest } => {
                Some(EnergySimulationEventKind::Adopt(EnergySimulationRequestIdentity { request: *request, operation: *operation, generation: *generation, config_digest: *config_digest }))
            }
            _ => None,
        };
        if let Some(event) = event {
            let render = doc.render_operation().ok_or_else(|| Fault::from("energy simulation command lacks document operation context"))?;
            simulation_session::record_event(render, event).map_err(Fault::from)?;
            return Ok(Emit { description: Some(Self::command_id(command).await.into()), ..Default::default() });
        }
        let mut model = crate::artifacts::model::energy_model(doc.snapshot);
        let description = match command {
            EnergyModelEditorCommand::SetStructureField { field, value } => {
                match field.as_str() {
                    "name" => model.name = value.clone(),
                    "version" => model.version = value.clone(),
                    _ => return Ok(Emit::default()),
                }
                format!("Set {field}")
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
                format!("Set zone {row} {column}")
            }
            _ => unreachable!("simulation events returned before document mutation dispatch"),
        };
        let new_model_json = pack::json::to_json_string(&model);
        Ok(Emit { artifact_mutations: vec![EnergyModelMutation::ReplaceModel(ReplaceModel { new_model_json })], description: Some(description), ..Default::default() })
    }

    async fn pending_effects(doc: &ArtifactView<'_, EnergyModelSnapshot>, _cfg: &ConfigView<'_, NoConfig>) -> Vec<semio_framework::kernel::Effect> {
        simulation_session::reconcile(doc)
    }

    async fn render(body_key: &str, doc: &ArtifactView<'_, Self::Snapshot>, _cfg: &ConfigView<'_, Self::Config>) -> ComponentTree {
        semio_framework_plugin::built_to_component_tree(match body_key {
            structure::BODY_KEY => structure::render(doc.snapshot),
            zones::BODY_KEY => zones::render(doc.snapshot),
            simulation::BODY_KEY => simulation_session::with_projection(doc.render_operation(), |projection| simulation::render(projection, projection.is_some_and(|projection| projection.locale_de))),
            _ => semio_framework_plugin::built_text_node(Label::data(format!("Unknown body: {body_key}"))),
        })
    }
}
//#endregion 🔖️Editor

//#region 🔖️Manifest
pub fn create_energy_model_editor() -> semio_framework_plugin::AppDefinition {
    Editor::builder(MODEL_DIALECT)
        .document(["semio", "energy", "model"])
        .icon_id("battery")
        .mode_def(edit::definition())
        .default_mode_id(edit::ENERGY_MODEL_EDIT_MODE_ID)
        .window_kind_def(structure::definition())
        .window_kind_def(zones::definition())
        .window_kind_def(simulation::definition())
        .default_layout(edit::layout())
        .build_definition()
}
//#endregion 🔖️Manifest

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[semio_framework_async_macros::async_test]
    async fn create_energy_model_editor_builds_a_definition_for_the_editor_role() {
        let def = create_energy_model_editor();
        assert_eq!(def.role, semio_framework_plugin::AppRole::Editor);
        assert_eq!(def.dialect, MODEL_DIALECT.into());
    }

    #[semio_framework_async_macros::async_test]
    async fn editor_dialect_matches_the_artifact_coordinate() {
        assert_eq!(<EnergyModelEditor as ArtifactEditor>::DIALECT, MODEL_DIALECT);
    }

    #[semio_framework_async_macros::async_test]
    async fn editor_declares_both_windows() {
        let def = create_energy_model_editor();
        assert!(def.window_kinds.iter().any(|w| w.id == structure::WINDOW_KIND_ID));
        assert!(def.window_kinds.iter().any(|w| w.id == zones::WINDOW_KIND_ID));
    }
}
//#endregion 🧪️Tests
