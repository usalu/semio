//! 👁️ Drawing viewer — the read-only counterpart of `✏️editor` for this subset (ticket
//! 26/08/16/ARTIFACT-VIEWERS-AND-EDITORS-PER-SUBSET contract §2.2). `DrawingViewer` implements
//! `ArtifactViewer`, never `ArtifactEditor`/`ArtifactApp` — `ViewerApp<DrawingViewer>` (framework SDK) is
//! the sole runtime adapter, so this file can never structurally emit an artifact or draft mutation.
//! MUST NOT import anything from the sibling editor module (`policyViewerPurityBreaches`).

#![allow(clippy::result_large_err)]
#![allow(unexpected_cfgs)]
extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_schema as framework_schema;

use semio_s_artifact_draw_drawing_model::default_drawing_document;
use crate::modes::view;
use crate::modes::view::windows::canvas;
use semio_s_artifact_draw_drawing_model::snapshot::DrawingSnapshot;
use semio_s_artifact_draw_drawing_model::{DRAWING_DIALECT, DRAWING_DOCUMENT_SCHEMA};
use semio_framework_plugin::app::InteractionView;
use semio_framework_plugin::ArtifactView;
use semio_framework_plugin::ArtifactViewer;
use semio_framework_plugin::ConfigView;
use semio_framework_plugin::Fault;
use semio_framework_ui_locale::Label;
use semio_framework_plugin::NoConfig;
use semio_framework_plugin::NoConfigMutation;
use semio_framework_plugin::NoPresence;
use semio_framework_plugin::NoPresenceMutation;
use semio_framework_plugin::NoTransient;
use semio_framework_plugin::NoTransientMutation;
use semio_framework_plugin::ViewEmit;
use semio_framework_plugin::Viewer;
use semio_framework_2d_compute::EngineHandles;
use semio_framework::kernel::UiDirtyScope;
use semio_framework_plugin::retained_command::{ArtifactRetainedCommandJob, ArtifactRetainedCommandPayload, BoundedArtifactCommandWork};
use semio_framework_plugin::{AppOperationContext, ArtifactOwnedToolJobRequest, ArtifactToolFactoryRegistry, ArtifactToolPublicationContract, ArtifactToolPublicationLane, Emit, FaultCode, FaultOrigin, HistoryView, InteractiveJobClassification, NoDraftMutation, ToolExecutionContract, ToolFactoryKey, ToolJobFactoryError, ViewerApp};
use semio_framework_value_derive::{FromValue as FromValueDerive, ToValue as ToValueDerive};


const DRAWING_VIEW_TOOL_IDS: &[&str] = &[canvas::SET_CAMERA_ACTION_ID];
const DRAWING_VIEW_PAYLOAD_SCHEMA: &str = "drawing.view-command.v1";
const DRAWING_VIEW_RAW_BYTES: usize = 8_192;
const DRAWING_VIEW_WORK_ITEMS: usize = 1;

#[derive(Clone, Debug, PartialEq, semio_framework_value::RetireOwned, ToValueDerive, FromValueDerive, semio_framework_dsl_record_derive::DslEnum)]
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

/// 📷️ Validates a viewer camera and publishes only the addressed local window configuration.
fn camera_emit(command: &DrawingViewCommand, view: Option<&semio_framework_plugin::ViewModel>) -> Result<Emit<semio_s_artifact_draw_drawing_mutations::DrawingMutation, NoConfigMutation, NoDraftMutation>, Fault> {
    let DrawingViewCommand::SetCamera { camera } = command;
    let camera: store::Viewport2d = semio_framework_pack_json::from_json_str(camera, semio_framework_pack_json::JsonMemberPolicy::Reject).map_err(|error| Fault::from(error.to_string()))?;
    camera.validate().map_err(|error| Fault::from(error.to_string()))?;
    let view = view.ok_or_else(|| Fault::from("drawing-viewer-window-required"))?;
    let config = canvas::config::DrawingViewerCanvasWindowConfig { viewport: camera,framed: true };
    Ok(Emit { window_config_mutations: vec![canvas::config::addressed(view,config)?],ui_scope: UiDirtyScope::Partial { window_bodies: Vec::new(),panel_bodies: Vec::new(),utilities: false,tools: false,engagements: false,measures: false,labels: false },..Default::default() })
}

fn command_from_action(action: &str,args: Option<&semio_framework_value::DslValue>) -> Result<DrawingViewCommand,Fault> {
    if action != canvas::SET_CAMERA_ACTION_ID { return Err(Fault::from("drawing-viewer-command-unsupported")); }
    let value = args.and_then(|args| args.get("camera")).ok_or_else(|| Fault::from("drawing-viewer-camera-required"))?;
    let pose = <store::Viewport2d as semio_framework_value::FromValue>::from_value(value.clone()).map_err(|error| Fault::from(error.to_string()))?;
    pose.validate().map_err(|error| Fault::from(error.to_string()))?;
    Ok(DrawingViewCommand::SetCamera { camera: semio_framework_pack_json::to_json_string(&pose) })
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
) -> Result<Emit<semio_s_artifact_draw_drawing_mutations::DrawingMutation, NoConfigMutation, NoDraftMutation>, Fault> {
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

    semio_framework_plugin::retained_command::retained_command_factory_admission!(maximum_raw_bytes: DRAWING_VIEW_RAW_BYTES, checkpoint_resume: false);

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
    type Mutation = semio_s_artifact_draw_drawing_mutations::DrawingMutation;
    type Config = NoConfig;
    type ConfigMutation = NoConfigMutation;
    type Presence = NoPresence;
    type PresenceMutation = NoPresenceMutation;
    type Transient = NoTransient;
    type TransientMutation = NoTransientMutation;
    type Command = DrawingViewCommand;

    const DIALECT: semio_framework_artifact_reference::Dialect = DRAWING_DIALECT;
    const DOCUMENT_SCHEMA: &'static str = DRAWING_DOCUMENT_SCHEMA;

    fn build_envelope_decode_owner_bundle() -> Option<store::os_store::owned_schema_decode::ArtifactEnvelopeDecodeOwnerBundle<Self::Snapshot, Self::Mutation>> {
        Some(semio_s_artifact_draw_drawing_host_arena::drawing_envelope_decode_owner_bundle())
    }

    fn build_document_store_initialization_job(
        envelope: store::os_store::store_schema::ArtifactEnvelope<Self::Snapshot, Self::Mutation>,
        operation: semio_framework_job::OperationId,
        generation: semio_framework_job::Generation,
        actor: protocol::ActorId,
    ) -> Result<semio_framework_plugin::ArtifactStoreInitializationJob<Self::Snapshot, Self::Mutation>, store::os_store::store_schema::ArtifactEnvelope<Self::Snapshot, Self::Mutation>> {
        Ok(semio_s_artifact_draw_drawing_host::owned::drawing_document_store_initialization_job(envelope, operation, generation, actor))
    }

    /// 🧬️ The loaded-parent child projection, read off the snapshot's own derived composition fields;
    /// without it every live envelope load faults `viewer did not declare a loaded-parent child
    /// projection` before the decoded document can replace the store.
    fn child_restore_projection(snapshot: &Self::Snapshot) -> Result<semio_framework_composition_admission::ChildRestoreProjection<'_>, Fault> {
        semio_framework_composition_admission::ChildRestoreProjection::from_snapshot(snapshot).map_err(|error| Fault::new(FaultOrigin::App, FaultCode::new("drawing.child-projection"), error.to_string()))
    }

    fn build_document_store_disposer() -> Option<Box<dyn semio_framework_plugin::ArtifactOwnedDisposer<store::ArtifactStore<Self::Snapshot,Self::Mutation>>>> {
        Some(Box::new(semio_framework_plugin::ArtifactDocumentStoreDisposer::<Self::Snapshot,Self::Mutation>::new()))
    }

    fn build_config_store_disposer() -> Option<Box<dyn semio_framework_plugin::ArtifactOwnedDisposer<store::ConfigStore<Self::Config,Self::ConfigMutation>>>> { Some(semio_framework_plugin::no_config_store_disposer()) }
    fn build_presence_store_disposer() -> Option<Box<dyn semio_framework_plugin::ArtifactOwnedDisposer<store::PresenceStore<Self::Presence,Self::PresenceMutation>>>> { Some(semio_framework_plugin::no_presence_store_disposer()) }
    fn build_transient_store_disposer() -> Option<Box<dyn semio_framework_plugin::ArtifactOwnedDisposer<store::TransientStore<Self::Transient,Self::TransientMutation>>>> { Some(semio_framework_plugin::no_transient_store_disposer()) }
    fn build_presence_peer_retirement_factory() -> Option<std::sync::Arc<dyn store::SnapshotRetirementFactory<Self::Presence>>> { Some(semio_framework_plugin::no_presence_peer_retirement_factory()) }
    fn build_presence_local_root_retirement_factory() -> Option<std::sync::Arc<dyn store::SnapshotRetirementFactory<Self::Presence>>> { Some(semio_framework_plugin::no_presence_local_root_retirement_factory()) }
    fn build_transient_local_root_retirement_factory() -> Option<std::sync::Arc<dyn store::SnapshotRetirementFactory<Self::Transient>>> { Some(semio_framework_plugin::no_transient_local_root_retirement_factory()) }

    fn reference_birth()->Option<fn()->DrawingSnapshot>{Some(||{
        default_drawing_document("empty", None)
    })}

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

    fn command_from_action(action: &str, args: &mut Option<semio_framework_value::DslValue>, original_fault: &mut Option<Fault>) -> Result<Self::Command, Fault> {
        if let Some(fault) = original_fault.take() { return Err(fault); }
        let args = args.as_ref();
        command_from_action(action, args)
    }

    fn register_window_config_owners(registry: &mut semio_framework_plugin::WindowConfigOwnerRegistry) -> Result<(), Fault> {
        registry.register::<canvas::config::DrawingViewerCanvasWindowConfigOwner>()
    }

    fn register_tool_job_factories(registry: &mut ArtifactToolFactoryRegistry<'_, ViewerApp<Self>>) -> Result<(), Fault> {
        let controller = registry.controller_id().to_string();
        registry.register(DrawingViewCommandJobFactory::new(&controller))
    }

    fn build_tool_job(request: &mut Option<ArtifactOwnedToolJobRequest<ViewerApp<Self>>>, output: &mut Option<semio_framework_plugin::ErasedToolJob>, cx: &mut semio_framework_plugin::__job::StepContext<'_>) -> Result<bool, semio_framework_plugin::__job::OriginalTurnFailure> {
        let original = request.as_ref().ok_or_else(|| semio_framework_plugin::__value::ValueError::literal(semio_framework_plugin::__value::ValueRefusalKind::InvariantViolated, "original retained producer request is absent"))?;
        if !DRAWING_VIEW_TOOL_IDS.contains(&original.tool_id.as_str()) {
            return Ok(false);
        }
        if original.command.action_id() != original.tool_id {
            return Err(semio_framework_plugin::__value::ValueError::literal(semio_framework_plugin::__value::ValueRefusalKind::InvariantViolated, "the Drawing view command does not match its exact registered tool").into());
        }
        let tool_id = original.command.action_id();
        semio_framework_plugin::retained_command::producer::admit_retained_tool_producer(request, output, cx, DrawingViewCommand::action_id, DRAWING_VIEW_RAW_BYTES, 1, false, |request| BoundedArtifactCommandWork::new(tool_id, drawing_view_reduce, drawing_view_extent))
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

//#region 🪢️TaxonomyMounts
#[path = "."]
pub mod config {
    #[path = "🎚️config/🧬️schema/🦀️.rs"]
    pub mod schema;
}
#[path = "."]
pub mod modes {
    #[path = "🎭️modes/👁️view/🦀️.rs"]
    pub mod view;
}
#[path = "."]
pub mod presence {
    #[path = "👥️presence/🧬️schema/🦀️.rs"]
    pub mod schema;
}
//#endregion 🪢️TaxonomyMounts
