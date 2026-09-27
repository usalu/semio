//! 👁️ Drawing viewer — the read-only counterpart of `✏️editor` for this subset (ticket
//! 26/08/16/ARTIFACT-VIEWERS-AND-EDITORS-PER-SUBSET contract §2.2). `DrawingViewer` implements
//! `ArtifactViewer`, never `ArtifactEditor`/`ArtifactApp` — `ViewerApp<DrawingViewer>` (framework SDK) is
//! the sole runtime adapter, so this file can never structurally emit an artifact or draft mutation.
//! MUST NOT import anything from the sibling editor module (`policyViewerPurityBreaches`).

use crate::schema::default_drawing_document;
use crate::viewer::drawing::modes::view;
use crate::viewer::drawing::modes::view::windows::canvas;
use crate::{DrawingSnapshot, DRAWING_DIALECT, DRAWING_DOCUMENT_SCHEMA};
use semio_framework_plugin::app::InteractionView;
use semio_framework_plugin::{ArtifactView, ArtifactViewer, ConfigView, Fault, Label, NoConfig, NoConfigMutation, NoPresence, NoPresenceMutation, NoTransient, NoTransientMutation, ViewEmit, Viewer};
use store::EngineHandles;
use semio_framework::kernel::UiDirtyScope;
use semio_framework_plugin::retained_command::{ArtifactRetainedCommandJob, ArtifactRetainedCommandPayload, BoundedArtifactCommandWork};
use semio_framework_plugin::{AppOperationContext, ArtifactOwnedToolJobRequest, ArtifactToolFactoryRegistry, ArtifactToolPublicationContract, ArtifactToolPublicationLane, Emit, FaultCode, FaultOrigin, HistoryView, InteractiveJobClassification, NoDraftMutation, ToolExecutionContract, ToolFactoryKey, ToolJobFactoryError, ViewerApp};
use semio_framework_value_derive::{FromValue as FromValueDerive, ToValue as ToValueDerive};


const DRAWING_VIEW_TOOL_IDS: &[&str] = &[canvas::SET_CAMERA_ACTION_ID];
const DRAWING_VIEW_PAYLOAD_SCHEMA: &str = "drawing.view-command.v1";
const DRAWING_VIEW_RAW_BYTES: usize = 8_192;
const DRAWING_VIEW_WORK_ITEMS: usize = 1;

#[derive(Clone, Debug, PartialEq, ToValueDerive, FromValueDerive, dsl::DslOps)]
pub enum DrawingViewCommand {
    #[dsl(key = "setCamera")]
    SetCamera { camera: String },
}

impl Default for DrawingViewCommand {
    fn default() -> Self {
        Self::SetCamera { camera: String::new() }
    }
}

impl DrawingViewCommand {
    pub fn action_id(&self) -> &'static str {
        match self {
            Self::SetCamera { .. } => canvas::SET_CAMERA_ACTION_ID,
        }
    }
}

impl protocol::OpBinary for DrawingViewCommand {
    const TOOL_JOB_IDS: &'static [&'static str] = DRAWING_VIEW_TOOL_IDS;

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

/// 📷️ Validates a viewer camera and publishes only the addressed local window configuration.
fn camera_emit(command: &DrawingViewCommand, view: Option<&semio_framework_plugin::ViewModel>) -> Result<Emit<crate::op::DrawingMutation, NoConfigMutation, NoDraftMutation>, Fault> {
    let DrawingViewCommand::SetCamera { camera } = command;
    let camera: store::Viewport2d = dsl::json::from_json_str(camera).map_err(|error| Fault::from(error.to_string()))?;
    camera.validate().map_err(|error| Fault::from(error.to_string()))?;
    let view = view.ok_or_else(|| Fault::from("drawing-viewer-window-required"))?;
    let config = canvas::config::DrawingViewerCanvasWindowConfig { viewport: camera,framed: true };
    Ok(Emit { window_config_mutations: vec![canvas::config::addressed(view,config)?],ui_scope: UiDirtyScope::Partial { window_bodies: Vec::new(),panel_bodies: Vec::new(),utilities: false,tools: false,engagements: false,measures: false,labels: false },coalesce_key: Some(format!("drawing.viewer.camera:{}",view.window_id.as_deref().unwrap_or_default())),..Default::default() })
}

fn command_from_action(action: &str,args: Option<&dsl::DslValue>) -> Result<DrawingViewCommand,Fault> {
    if action != canvas::SET_CAMERA_ACTION_ID { return Err(Fault::from("drawing-viewer-command-unsupported")); }
    let value = args.and_then(|args| args.get("camera")).ok_or_else(|| Fault::from("drawing-viewer-camera-required"))?;
    let pose = <store::Viewport2d as dsl::FromValue>::from_value(value.clone()).map_err(|error| Fault::from(error.to_string()))?;
    pose.validate().map_err(|error| Fault::from(error.to_string()))?;
    Ok(DrawingViewCommand::SetCamera { camera: dsl::json::to_json_string(&pose) })
}

fn drawing_view_contract() -> ToolExecutionContract {
    ToolExecutionContract::bounded_first_step(DRAWING_VIEW_RAW_BYTES, DRAWING_VIEW_WORK_ITEMS, 1, 16_384, 7_500)
}

#[expect(clippy::unnecessary_wraps, reason = "The retained work contract takes an optional extent callback, and one camera is always one work item.")]
fn drawing_view_extent(_command: &DrawingViewCommand, _snapshot: &DrawingSnapshot, _interaction: &protocol::InteractionState) -> Option<usize> {
    Some(1)
}

#[expect(clippy::too_many_arguments, reason = "The retained command reducer implements the framework's eight-argument callback contract.")]
#[allow(clippy::needless_pass_by_value)]
fn drawing_view_reduce(
    command: &DrawingViewCommand,
    _snapshot: &DrawingSnapshot,
    _config: &NoConfig,
    _history: &HistoryView,
    _interaction: &protocol::InteractionState,
    _hover: &semio_framework_plugin::app::InteractionHoverState,
    context: Option<&semio_framework_plugin::app::ArtifactOwnedToolJobContext<ViewerApp<DrawingViewer>>>,
    _operation: &AppOperationContext,
) -> Result<Emit<crate::op::DrawingMutation, NoConfigMutation, NoDraftMutation>, Fault> {
    camera_emit(command, context.and_then(|context| context.view_state.as_ref()))
}

struct DrawingViewCommandJobFactory {
    keys: Vec<ToolFactoryKey>,
}

impl DrawingViewCommandJobFactory {
    fn new(controller_id: &str) -> Self {
        Self { keys: DRAWING_VIEW_TOOL_IDS.iter().map(|tool_id| ToolFactoryKey::new(controller_id, *tool_id)).collect() }
    }
}

impl semio_framework_plugin::ToolJobFactory for DrawingViewCommandJobFactory {
    type Payload = ArtifactRetainedCommandPayload<ViewerApp<DrawingViewer>>;
    type Job = ArtifactRetainedCommandJob<ViewerApp<DrawingViewer>>;

    fn keys(&self) -> &[ToolFactoryKey] {
        &self.keys
    }

    fn payload_schema_id(&self) -> &str {
        DRAWING_VIEW_PAYLOAD_SCHEMA
    }

    fn classification(&self) -> InteractiveJobClassification {
        InteractiveJobClassification::Migrated
    }

    fn execution_contract(&self) -> ToolExecutionContract {
        drawing_view_contract()
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
        if input.declared_bytes() > DRAWING_VIEW_RAW_BYTES || checkpoint.is_some() {
            return Err((ToolJobFactoryError::new("the bounded Drawing view command rejects an oversized wire or checkpoint owner"), input, checkpoint));
        }
        Ok(ArtifactRetainedCommandJob::from_wire(payload, input))
    }
}

impl semio_framework_plugin::ArtifactOwnedToolJobFactory for DrawingViewCommandJobFactory {
    type Owner = ViewerApp<DrawingViewer>;
    const TOOL_IDS: &'static [&'static str] = DRAWING_VIEW_TOOL_IDS;
    const DOCUMENT_SCHEMA: &'static str = DRAWING_DOCUMENT_SCHEMA;
    const PUBLICATION_CONTRACTS: &'static [ArtifactToolPublicationContract] = &[ArtifactToolPublicationContract { tool_id: canvas::SET_CAMERA_ACTION_ID, lanes: &[ArtifactToolPublicationLane::WindowConfig] }];
}


//#region 🔖️Viewer
#[derive(Default, Clone, Copy)]
pub struct DrawingViewer;

impl ArtifactViewer for DrawingViewer {
    type Snapshot = DrawingSnapshot;
    type Mutation = crate::op::DrawingMutation;
    type Config = NoConfig;
    type ConfigMutation = NoConfigMutation;
    type Presence = NoPresence;
    type PresenceMutation = NoPresenceMutation;
    type Transient = NoTransient;
    type TransientMutation = NoTransientMutation;
    type Command = DrawingViewCommand;

    const DIALECT: semio_framework::Dialect = DRAWING_DIALECT;
    const DOCUMENT_SCHEMA: &'static str = DRAWING_DOCUMENT_SCHEMA;

    fn build_envelope_decode_owner_bundle() -> Option<store::ArtifactEnvelopeDecodeOwnerBundle<Self::Snapshot, Self::Mutation>> {
        Some(crate::spr::drawing_envelope_decode_owner_bundle())
    }

    fn build_document_store_initialization_job(
        envelope: store::ArtifactEnvelope<Self::Snapshot, Self::Mutation>,
        operation: semio_framework_job::OperationId,
        generation: semio_framework_job::Generation,
    ) -> Result<semio_framework_plugin::ArtifactStoreInitializationJob<Self::Snapshot, Self::Mutation>, store::ArtifactEnvelope<Self::Snapshot, Self::Mutation>> {
        Ok(crate::spr::drawing_document_store_initialization_job(envelope, operation, generation))
    }

    /// 🧬️ The loaded-parent child projection, read off the snapshot's own derived composition fields;
    /// without it every live envelope load faults `viewer did not declare a loaded-parent child
    /// projection` before the decoded document can replace the store.
    fn child_restore_projection(snapshot: &Self::Snapshot) -> Result<store::ChildRestoreProjection<'_>, Fault> {
        store::ChildRestoreProjection::from_snapshot(snapshot).map_err(|error| Fault::new(semio_framework_plugin::FaultOrigin::App, semio_framework_plugin::FaultCode::new("drawing.child-projection"), error.to_string()))
    }

    /// 🔐️ The artifact's own document-store owner catalogue, identical to the sibling editor's: a viewer holds the same
    /// snapshot and must retire its owned values the same way, never through the framework's generic bounded owners.
    fn build_document_store_owners() -> Option<store::DocumentStoreOwners<Self::Snapshot, Self::Mutation>> {
        Some(crate::spr::drawing_document_store_owners())
    }

    fn build_document_store_disposer() -> Option<Box<dyn semio_framework_plugin::ArtifactOwnedDisposer<store::ArtifactStore<Self::Snapshot,Self::Mutation>>>> {
        Some(Box::new(semio_framework_plugin::ArtifactDocumentStoreDisposer::<Self::Snapshot,Self::Mutation>::new()))
    }

    fn build_config_store_owners() -> Option<store::DocumentStoreOwners<Self::Config,Self::ConfigMutation>> { Some(semio_framework_plugin::no_config_store_owners()) }
    fn build_config_store_disposer() -> Option<Box<dyn semio_framework_plugin::ArtifactOwnedDisposer<store::ConfigStore<Self::Config,Self::ConfigMutation>>>> { Some(semio_framework_plugin::no_config_store_disposer()) }
    fn build_presence_store_disposer() -> Option<Box<dyn semio_framework_plugin::ArtifactOwnedDisposer<store::PresenceStore<Self::Presence,Self::PresenceMutation>>>> { Some(semio_framework_plugin::no_presence_store_disposer()) }
    fn build_transient_store_disposer() -> Option<Box<dyn semio_framework_plugin::ArtifactOwnedDisposer<store::TransientStore<Self::Transient,Self::TransientMutation>>>> { Some(semio_framework_plugin::no_transient_store_disposer()) }
    fn build_presence_peer_retirement_factory() -> Option<std::sync::Arc<dyn store::SnapshotRetirementFactory<Self::Presence>>> { Some(semio_framework_plugin::no_presence_peer_retirement_factory()) }
    fn build_presence_local_root_retirement_factory() -> Option<std::sync::Arc<dyn store::SnapshotRetirementFactory<Self::Presence>>> { Some(semio_framework_plugin::no_presence_local_root_retirement_factory()) }
    fn build_transient_local_root_retirement_factory() -> Option<std::sync::Arc<dyn store::SnapshotRetirementFactory<Self::Transient>>> { Some(semio_framework_plugin::no_transient_local_root_retirement_factory()) }

    fn initial_snapshot() -> DrawingSnapshot {
        default_drawing_document("empty", None)
    }

    /// 🔒️ Camera publication requires the retained window-config route.
    fn handle(
        _command: &Self::Command,
        _doc: &ArtifactView<'_, Self::Snapshot>,
        _cfg: &ConfigView<'_, Self::Config>,
        _interaction: &InteractionView<'_>,
        _view_state: Option<&semio_framework_plugin::ViewModel>,
        _engines: &EngineHandles,
    ) -> Result<ViewEmit<Self::ConfigMutation>, Fault> {
        Err(Fault::from("drawing-viewer-retained-route-required"))
    }

    fn command_id(command: &Self::Command) -> &'static str {
        command.action_id()
    }

    fn command_from_action(action: &str, args: Option<&dsl::DslValue>) -> Result<Self::Command, Fault> {
        command_from_action(action, args)
    }

    fn register_window_config_owners(registry: &mut semio_framework_plugin::WindowConfigOwnerRegistry) -> Result<(), Fault> {
        registry.register::<canvas::config::DrawingViewerCanvasWindowConfigOwner>()
    }

    fn register_tool_job_factories(registry: &mut ArtifactToolFactoryRegistry<'_, ViewerApp<Self>>) -> Result<(), Fault> {
        let controller = registry.controller_id().to_string();
        registry.register(DrawingViewCommandJobFactory::new(&controller))
    }

    fn build_tool_job(request: ArtifactOwnedToolJobRequest<ViewerApp<Self>>) -> Result<Option<semio_framework_plugin::ToolOperationSpec>, Fault> {
        if !DRAWING_VIEW_TOOL_IDS.contains(&request.tool_id.as_str()) {
            return Ok(None);
        }
        if request.command.action_id() != request.tool_id {
            return Err(Fault::new(FaultOrigin::App, FaultCode::new("drawing.viewer.retained.tool-mismatch"), "the Drawing view command does not match its exact registered tool"));
        }
        let tool_id = request.command.action_id();
        let work = Box::new(BoundedArtifactCommandWork::new(tool_id, drawing_view_reduce, drawing_view_extent));
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
            DrawingViewCommand::action_id,
            DRAWING_VIEW_RAW_BYTES,
            1,
            work,
        )?;
        Ok(Some(semio_framework_plugin::ToolOperationSpec::new(request.controller_id, request.tool_id, request.payload_schema_id, payload, request.operation)))
    }

    semio_framework_plugin::bounded_first_step_tool_proofs! {
        owner: ViewerApp<DrawingViewer>,
        owner_file: "✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/🏅️standards/🔖️1/🪆️subsets/✳️any/👁️viewer/🦀️.rs",
        controller: "s.draw.drawing@1/*#viewer",
        artifact_schema: "drawing.document",
        factory: "DrawingViewCommandJobFactory",
        factory_type: DrawingViewCommandJobFactory,
        contract: ToolExecutionContract::bounded_first_step(8_192, 1, 1, 16_384, 7_500),
        tools: ["setCamera"]
    }


    fn render(body_key: &str, doc: &ArtifactView<'_, Self::Snapshot>, cfg: &ConfigView<'_, Self::Config>, _view_state: &semio_framework_plugin::ViewModel) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::ComponentTree> {
        let root = match body_key {
            canvas::BODY_KEY => canvas::render_with_camera(doc.snapshot,&canvas::config::current(cfg)),
            _ => semio_framework_plugin::built_text_node(Label::data(format!("Unknown body: {body_key}")))
                .map_err(|_| semio_framework_plugin::PluginAssemblyError::new("drawing.viewer.body.label", "the fixed Drawing viewer label exceeds its UI bound")),
        }?;
        Ok(semio_framework_plugin::built_to_component_tree(root))
    }
}
//#endregion 🔖️Viewer

//#region 🔖️Manifest
pub fn create_drawing_viewer() -> semio_framework_plugin::AppDefinition {
    Viewer::builder(DRAWING_DIALECT).document(["semio", "drawing"]).icon_id("drawing").mode_def(view::definition()).default_mode_id(view::DRAWING_VIEW_MODE_VIEW).window_kind_def(canvas::definition()).default_layout(view::layout()).action_audience("setCamera", semio_framework_plugin::CapabilityAudience::Chrome).build_definition()
}
//#endregion 🔖️Manifest

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
