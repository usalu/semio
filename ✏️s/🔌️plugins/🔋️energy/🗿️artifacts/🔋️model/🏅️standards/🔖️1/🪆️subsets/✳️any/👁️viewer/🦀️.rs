//! 👁️ Energy model viewer — the read-only counterpart of `✏️editor` for this subset (ticket
//! 26/08/16/ARTIFACT-VIEWERS-AND-EDITORS-PER-SUBSET contract §2.2). `EnergyModelViewer` implements
//! `ArtifactViewer`, never `ArtifactEditor`/`ArtifactApp` — `ViewerApp<EnergyModelViewer>` (framework
//! SDK) is the sole runtime adapter, so this file can never structurally emit an artifact mutation.
//! Must not import anything from the sibling mutation-capable surface (`policyViewerPurityBreaches`).

use crate::viewer::model::modes::view;
use crate::viewer::model::modes::view::windows::{model as model_window, simulation, structure, zones};
use crate::{EnergyModelMutation, EnergyModelSnapshot, ENERGY_MODEL_DOCUMENT_SCHEMA, MODEL_DIALECT};
use semio_framework::kernel::UiDirtyScope;
use semio_framework_plugin::retained_command::{ArtifactRetainedCommandJob, ArtifactRetainedCommandPayload, BoundedArtifactCommandWork};
use semio_framework_plugin::{
    AppOperationContext, ArtifactOwnedToolJobRequest, ArtifactToolFactoryRegistry, ArtifactToolPublicationContract, ArtifactToolPublicationLane, ArtifactView, ArtifactViewer, ComponentTree, ConfigView, Dialect, Emit, Fault, FaultCode, FaultOrigin,
    HistoryView, InteractiveJobClassification, Label, NoConfig, NoConfigMutation, NoDraftMutation, NoPresence, NoPresenceMutation, NoTransient, NoTransientMutation, ToolExecutionContract,
    ToolFactoryKey, ToolJobFactoryError, UiAssemblyResult, ViewEmit, Viewer, ViewerApp,
};
use semio_framework_value_derive::{FromValue as FromValueDerive, ToValue as ToValueDerive};

//#region 🏷️ActionIds
/// 🏷️ Every dispatchable verb of this VIEWER, exactly once. One row: the 3d window's orbit pose.
///
/// 🔒️ The whole list must stay config-only. A viewer's publication contracts may never name
/// `ArtifactToolPublicationLane::Artifact`, and `setCamera` names `WindowConfig` — the lane that
/// writes ONE window instance's own retained state and nothing else.
const ENERGY_MODEL_VIEW_TOOL_IDS: &[&str] = &[model_window::SET_CAMERA_ACTION_ID];
const ENERGY_MODEL_VIEW_PAYLOAD_SCHEMA: &str = "energy.model.view-command.v1";
/// 🎒️ Real bound for one viewer gesture's wire payload: a pose is two coordinate triples plus a
/// zoom, so 8 KiB is a real ceiling rather than a rubber stamp.
const ENERGY_MODEL_VIEW_RAW_BYTES: usize = 8_192;
/// 🧮️ One window-config write per gesture.
const ENERGY_MODEL_VIEW_WORK_ITEMS: usize = 1;
//#endregion 🏷️ActionIds

//#region 🔖️Command
/// 👁️ The viewer's typed command channel. It has exactly one row, and that row is the reason the
/// channel is no longer inert: the react `World3dHost` dispatches `setCamera` after every orbit of
/// ANY World3d window, read-only or not, and a window kind that does not declare and reduce it drops
/// every gesture (`dropped action "setCamera" … no window kind declares it`).
///
/// 🔒️ Row order is the binary variant ordinal: appending is safe, reordering is a wire break.
#[derive(Clone, Debug, PartialEq, ToValueDerive, FromValueDerive, dsl::DslOps)]
pub enum EnergyModelViewCommand {
    /// 🎥️ The orbit pose as the canonical `{position,target,zoom,up?}` JSON the host sends. One text
    /// field because a `dsl::DslOps` variant binds scalars only — and because that string IS what
    /// `World3dScene::camera_json` consumes.
    #[dsl(key = "setCamera")]
    SetCamera { camera: String },
}

impl Default for EnergyModelViewCommand {
    /// 🌱️ An empty pose, which `camera_emit` refuses explicitly rather than applying — there is no
    /// "do nothing" verb to default to any more, and inventing a pose here would let a malformed
    /// dispatch write a camera nobody asked for.
    fn default() -> Self {
        Self::SetCamera { camera: String::new() }
    }
}

impl EnergyModelViewCommand {
    /// 🪪️ The manifest action id this command stands for.
    pub fn action_id(&self) -> &'static str {
        match self {
            Self::SetCamera { .. } => model_window::SET_CAMERA_ACTION_ID,
        }
    }
}

//#region 🔖️OpCodec
impl protocol::OpBinary for EnergyModelViewCommand {
    /// 🧵️ The exact manifest ids this typed command schema owns — the left-hand side of
    /// `validate_tool_job_rows`' set equality against the `bounded_first_step_tool_proofs!` rows.
    const TOOL_JOB_IDS: &'static [&'static str] = ENERGY_MODEL_VIEW_TOOL_IDS;

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

//#region 🎥️Camera
/// 🎥️ Turns one `setCamera` gesture into the addressed window-config write, or into an explicit
/// refusal. This is the WHOLE behavior of the viewer's command channel.
///
/// 🔒️ The emitted `Emit` carries `window_config_mutations` and NOTHING else — no artifact mutation,
/// no config mutation, no effect — which is the runtime half of the read-only guarantee whose
/// compile-time half is `ViewEmit`. `reduce`'s own signature could carry a document mutation; this
/// function is the one place that decides, and it never builds one.
fn camera_emit(command: &EnergyModelViewCommand, view_state: Option<&semio_framework_plugin::ViewModel>) -> Result<Emit<EnergyModelMutation, NoConfigMutation, NoDraftMutation>, Fault> {
    let EnergyModelViewCommand::SetCamera { camera } = command;
    let view = view_state.ok_or_else(|| Fault::new(FaultOrigin::App, FaultCode::new("energy.model.3d.viewer.window-required"), "a camera change requires a concrete 3d model window"))?;
    if camera.is_empty() {
        return Err(Fault::new(FaultOrigin::App, FaultCode::new("app.command.invalid-payload"), "setCamera carries no {position,target,zoom} pose"));
    }
    let value = dsl::json::from_json_str::<dsl::DslValue>(camera).map_err(|error| Fault::new(FaultOrigin::App, FaultCode::new("app.command.invalid-payload"), format!("the camera pose is not a value: {error}")))?;
    let pose = <model_window::config::EnergyModelViewerCameraPose as dsl::FromValue>::from_value(value).map_err(|error| Fault::new(FaultOrigin::App, FaultCode::new("app.command.invalid-payload"), format!("the camera pose is malformed: {error}")))?;
    if !pose.is_valid() {
        return Err(Fault::new(FaultOrigin::App, FaultCode::new("app.command.invalid-payload"), "the camera pose is not finite, or its zoom is not positive"));
    }
    let mutation = model_window::config::EnergyModelViewerWindowConfigMutation::SetCamera(model_window::config::SetCamera { camera: pose });
    Ok(Emit {
        window_config_mutations: vec![model_window::config::addressed(view, mutation)?],
        description: Some("Set camera".into()),
        // 🐢️ Nothing to repaint: the pane that sent the pose already holds it.
        ui_scope: UiDirtyScope::Partial { window_bodies: Vec::new(), panel_bodies: Vec::new(), utilities: false, tools: false, engagements: false, measures: false, labels: false },
        // 🧲️ One coalesce key per window instance, so a burst of debounced orbit ticks collapses.
        coalesce_key: Some(format!("energy.model.3d.viewer.camera:{}", view.window_id.as_deref().unwrap_or_default())),
        ..Default::default()
    })
}

/// 🪪️ The bridge from a dispatched action's args to the typed command. The pose is validated and
/// canonicalized to the exact JSON string `World3dScene::camera_json` consumes, so a malformed
/// gesture is refused here rather than silently ignored.
fn command_from_action(action: &str, args: Option<&dsl::DslValue>) -> Result<EnergyModelViewCommand, Fault> {
    if action != model_window::SET_CAMERA_ACTION_ID {
        return Err(Fault::new(FaultOrigin::App, FaultCode::new("app.command.unsupported"), format!("the energy model viewer has no command for action '{action}'")));
    }
    let camera = args
        .and_then(|args| args.get("camera"))
        .and_then(|value| {
            let pose = <store::Viewport3dOrbit as dsl::FromValue>::from_value(value.clone()).ok()?;
            pose.validate().ok()?;
            Some(dsl::json::to_json_string(&dsl::ToValue::to_value(&pose)))
        })
        .unwrap_or_default();
    Ok(EnergyModelViewCommand::SetCamera { camera })
}
//#endregion 🎥️Camera

//#region 🧵️RetainedCommands
fn energy_model_view_contract() -> ToolExecutionContract {
    ToolExecutionContract::bounded_first_step(ENERGY_MODEL_VIEW_RAW_BYTES, ENERGY_MODEL_VIEW_WORK_ITEMS, 1, 16_384, 7_500)
}

#[expect(clippy::unnecessary_wraps, reason = "The retained work contract takes an optional extent callback, and one camera is always one work item.")]
fn energy_model_view_extent(_command: &EnergyModelViewCommand, _snapshot: &EnergyModelSnapshot, _interaction: &protocol::InteractionState) -> Option<usize> {
    Some(1)
}

/// 👁️ The one reducer the viewer's single tool runs. `InteractiveJobClassification::Migrated` is the
/// only UI-dispatchable classification and a `Migrated` verb without an exact app-owned reducer is
/// refused with `interactive-job.missing-owned-reducer` — so this function existing is precisely what
/// makes the read-only window's orbit survive.
#[expect(clippy::too_many_arguments, reason = "The retained command reducer implements the framework's eight-argument callback contract.")]
#[allow(clippy::needless_pass_by_value)]
fn energy_model_view_reduce(
    command: &EnergyModelViewCommand,
    _snapshot: &EnergyModelSnapshot,
    _config: &NoConfig,
    _history: &HistoryView,
    _interaction: &protocol::InteractionState,
    _hover: &semio_framework_plugin::app::InteractionHoverState,
    context: Option<&semio_framework_plugin::app::ArtifactOwnedToolJobContext<ViewerApp<EnergyModelViewer>>>,
    _operation: &AppOperationContext,
) -> Result<Emit<EnergyModelMutation, NoConfigMutation, NoDraftMutation>, Fault> {
    camera_emit(command, context.and_then(|context| context.view_state.as_ref()))
}

struct EnergyModelViewCommandJobFactory {
    keys: Vec<ToolFactoryKey>,
}

impl EnergyModelViewCommandJobFactory {
    fn new(controller_id: &str) -> Self {
        Self { keys: ENERGY_MODEL_VIEW_TOOL_IDS.iter().map(|tool_id| ToolFactoryKey::new(controller_id, *tool_id)).collect() }
    }
}

impl semio_framework_plugin::ToolJobFactory for EnergyModelViewCommandJobFactory {
    type Payload = ArtifactRetainedCommandPayload<ViewerApp<EnergyModelViewer>>;
    type Job = ArtifactRetainedCommandJob<ViewerApp<EnergyModelViewer>>;

    fn keys(&self) -> &[ToolFactoryKey] {
        &self.keys
    }

    fn payload_schema_id(&self) -> &str {
        ENERGY_MODEL_VIEW_PAYLOAD_SCHEMA
    }

    fn classification(&self) -> InteractiveJobClassification {
        InteractiveJobClassification::Migrated
    }

    fn execution_contract(&self) -> ToolExecutionContract {
        energy_model_view_contract()
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
        if input.declared_bytes() > ENERGY_MODEL_VIEW_RAW_BYTES || checkpoint.is_some() {
            return Err((ToolJobFactoryError::new("the bounded energy model view command rejects an oversized wire or checkpoint owner"), input, checkpoint));
        }
        Ok(ArtifactRetainedCommandJob::from_wire(payload, input))
    }
}

impl semio_framework_plugin::ArtifactOwnedToolJobFactory for EnergyModelViewCommandJobFactory {
    type Owner = ViewerApp<EnergyModelViewer>;
    const TOOL_IDS: &'static [&'static str] = ENERGY_MODEL_VIEW_TOOL_IDS;
    const DOCUMENT_SCHEMA: &'static str = ENERGY_MODEL_DOCUMENT_SCHEMA;
    /// 🔒️ WINDOW-CONFIG ONLY. This one row is the runtime half of the read-only guarantee: naming
    /// `ArtifactToolPublicationLane::Artifact` here would be rejected against a viewer's own emit.
    const PUBLICATION_CONTRACTS: &'static [ArtifactToolPublicationContract] = &[ArtifactToolPublicationContract { tool_id: model_window::SET_CAMERA_ACTION_ID, lanes: &[ArtifactToolPublicationLane::WindowConfig] }];
}
//#endregion 🧵️RetainedCommands

//#region 🔖️Viewer
#[derive(Default, Clone, Copy)]
pub struct EnergyModelViewer;

impl ArtifactViewer for EnergyModelViewer {
    type Snapshot = EnergyModelSnapshot;
    type Mutation = EnergyModelMutation;
    type Config = NoConfig;
    type ConfigMutation = NoConfigMutation;
    type Presence = NoPresence;
    type PresenceMutation = NoPresenceMutation;
    type Transient = NoTransient;
    type TransientMutation = NoTransientMutation;
    type Command = EnergyModelViewCommand;
    /// 🧩️ Read-only twin of the editor's roster: `structure`/`zones` are stdio `value`/`table` members.
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

    fn initial_snapshot() -> EnergyModelSnapshot {
        EnergyModelSnapshot::default()
    }

    /// 🛂️ A viewer declares every nontrivial store owner explicitly (the trait defaults fail closed);
    /// the document store is the bounded pair the editor also uses, and every other lane is `No*`.
    /// Without these the two genesis members can never be retired (`interactive-job.close-owned-
    /// disposer-missing`).
    fn build_document_store_owners() -> Option<store::DocumentStoreOwners<Self::Snapshot, Self::Mutation>> {
        Some(semio_framework_plugin::bounded_document_store_owners::<Self::Snapshot, Self::Mutation>())
    }

    fn build_document_store_disposer() -> Option<Box<dyn semio_framework_plugin::ArtifactOwnedDisposer<store::ArtifactStore<Self::Snapshot, Self::Mutation>>>> {
        Some(semio_framework_plugin::bounded_document_store_disposer::<Self::Snapshot, Self::Mutation>())
    }

    fn build_config_store_owners() -> Option<store::DocumentStoreOwners<Self::Config, Self::ConfigMutation>> {
        Some(semio_framework_plugin::no_config_store_owners())
    }

    fn build_config_store_disposer() -> Option<Box<dyn semio_framework_plugin::ArtifactOwnedDisposer<store::ConfigStore<Self::Config, Self::ConfigMutation>>>> {
        Some(semio_framework_plugin::no_config_store_disposer())
    }

    fn build_presence_store_disposer() -> Option<Box<dyn semio_framework_plugin::ArtifactOwnedDisposer<store::PresenceStore<Self::Presence, Self::PresenceMutation>>>> {
        Some(semio_framework_plugin::no_presence_store_disposer())
    }

    fn build_transient_store_disposer() -> Option<Box<dyn semio_framework_plugin::ArtifactOwnedDisposer<store::TransientStore<Self::Transient, Self::TransientMutation>>>> {
        Some(semio_framework_plugin::no_transient_store_disposer())
    }

    fn build_presence_peer_retirement_factory() -> Option<std::sync::Arc<dyn store::SnapshotRetirementFactory<Self::Presence>>> {
        Some(semio_framework_plugin::no_presence_peer_retirement_factory())
    }

    fn build_presence_local_root_retirement_factory() -> Option<std::sync::Arc<dyn store::SnapshotRetirementFactory<Self::Presence>>> {
        Some(semio_framework_plugin::no_presence_local_root_retirement_factory())
    }

    fn build_transient_local_root_retirement_factory() -> Option<std::sync::Arc<dyn store::SnapshotRetirementFactory<Self::Transient>>> {
        Some(semio_framework_plugin::no_transient_local_root_retirement_factory())
    }

    fn command_id(command: &Self::Command) -> &'static str {
        command.action_id()
    }

    fn command_from_action(action: &str, args: Option<&dsl::DslValue>) -> Result<Self::Command, Fault> {
        command_from_action(action, args)
    }

    /// 👁️ Structurally read-only. `ViewEmit` has no window-config lane at all, so the ONE verb this
    /// viewer owns cannot be served here — `setCamera` is `InteractiveJobClassification::Migrated`
    /// and therefore travels the RETAINED route ([`energy_model_view_reduce`]), whose `Emit` does
    /// carry `window_config_mutations`. This arm exists so that route's absence is a loud refusal
    /// rather than a silently dropped orbit.
    fn handle(
        _command: &Self::Command,
        _doc: &ArtifactView<'_, Self::Snapshot>,
        _cfg: &ConfigView<'_, Self::Config>,
        _interaction: &semio_framework_plugin::app::InteractionView<'_>,
        _view_state: Option<&semio_framework_plugin::ViewModel>,
        _engines: &store::EngineHandles,
    ) -> Result<ViewEmit<Self::ConfigMutation>, Fault> {
        Err(Fault::new(
            FaultOrigin::App,
            FaultCode::new("energy.model.3d.viewer.retained-route-required"),
            "a viewer camera is a window-config write, which ViewEmit cannot carry — it must reach the retained reducer",
        ))
    }

    /// 🎥️ The 3d viewer window's own retained orbit pose — one bounded window-config store, keyed by
    /// window INSTANCE, so two open read-only panes never share a camera. Its state and its verb are
    /// authored under `👁️viewer`, never imported from the editor's twin.
    fn register_window_config_owners(registry: &mut semio_framework_plugin::WindowConfigOwnerRegistry) -> Result<(), Fault> {
        registry.register::<model_window::config::EnergyModelViewerWindowConfigOwner>()
    }

    fn register_tool_job_factories(registry: &mut ArtifactToolFactoryRegistry<'_, ViewerApp<Self>>) -> Result<(), Fault> {
        let controller = registry.controller_id().to_string();
        registry.register(EnergyModelViewCommandJobFactory::new(&controller))
    }

    fn build_tool_job(request: ArtifactOwnedToolJobRequest<ViewerApp<Self>>) -> Result<Option<semio_framework_plugin::ToolOperationSpec>, Fault> {
        if !ENERGY_MODEL_VIEW_TOOL_IDS.contains(&request.tool_id.as_str()) {
            return Ok(None);
        }
        if request.command.action_id() != request.tool_id {
            return Err(Fault::new(FaultOrigin::App, FaultCode::new("energy.model.viewer.retained.tool-mismatch"), "the energy model view command does not match its exact registered tool"));
        }
        let tool_id = request.command.action_id();
        let work = Box::new(BoundedArtifactCommandWork::new(tool_id, energy_model_view_reduce, energy_model_view_extent));
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
            EnergyModelViewCommand::action_id,
            ENERGY_MODEL_VIEW_RAW_BYTES,
            1,
            work,
        )?;
        Ok(Some(semio_framework_plugin::ToolOperationSpec::new(request.controller_id, request.tool_id, request.payload_schema_id, payload, request.operation)))
    }

    semio_framework_plugin::bounded_first_step_tool_proofs! {
        owner: ViewerApp<EnergyModelViewer>,
        owner_file: "✏️s/🔌️plugins/🔋️energy/🗿️artifacts/🔋️model/🏅️standards/🔖️1/🪆️subsets/✳️any/👁️viewer/🦀️.rs",
        controller: "s.energy.model@1/*#viewer",
        artifact_schema: "energy.model",
        factory: "EnergyModelViewCommandJobFactory",
        factory_type: EnergyModelViewCommandJobFactory,
        contract: ToolExecutionContract::bounded_first_step(8_192, 1, 1, 16_384, 7_500),
        tools: ["setCamera"]
    }

    fn render(body_key: &str, doc: &ArtifactView<'_, Self::Snapshot>, cfg: &ConfigView<'_, Self::Config>, _view_state: &semio_framework_plugin::ViewModel) -> UiAssemblyResult<ComponentTree> {
        let node = match body_key {
            structure::BODY_KEY => structure::render(doc.snapshot)?,
            zones::BODY_KEY => zones::render(doc.snapshot)?,
            simulation::BODY_KEY => simulation::render(&doc.snapshot.model),
            // 🎥️ `config::current` is the addressed window's retained orbit pose, `None` until it has
            // been moved — an unmoved window keeps the model-derived camera and `fit_json`'s framing.
            model_window::BODY_KEY => model_window::render_with_camera(&doc.snapshot.model, model_window::config::current(cfg))?,
            _ => {
                semio_framework_plugin::built_text_node(Label::data(format!("Unknown body: {body_key}"))).map_err(|_| semio_framework_plugin::PluginAssemblyError::new("energy.model.viewer.render", "the unknown-body label could not be assembled"))?
            }
        };
        Ok(semio_framework_plugin::built_to_component_tree(node))
    }
}
//#endregion 🔖️Viewer

//#region 🔖️Manifest
pub fn create_energy_model_viewer() -> semio_framework_plugin::AppDefinition {
    Viewer::builder(MODEL_DIALECT)
        .document(["semio", "energy", "model"])
        .icon_id("battery")
        .mode_def(view::definition())
        .default_mode_id(view::ENERGY_MODEL_VIEW_MODE_ID)
        .window_kind_def(structure::definition())
        .window_kind_def(zones::definition())
        .window_kind_def(simulation::definition())
        .window_kind_def(model_window::definition())
        .default_layout(view::layout())
        .build_definition()
}
//#endregion 🔖️Manifest

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
