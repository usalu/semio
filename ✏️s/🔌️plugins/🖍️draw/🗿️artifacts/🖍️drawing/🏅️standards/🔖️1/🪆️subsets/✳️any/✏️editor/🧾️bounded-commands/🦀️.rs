//! 🧾️ Drawing bounded commands: the bounded, export, image and path tool ids, their publication contracts and the bounded command job factory.

use super::*;

//#region 🧵️BoundedCommands
/// 🧵️ Every non-gesture `DrawingCommand` row, each reducing in one bounded first step through
/// [`DrawingBoundedCommandJobFactory`]. Together with [`DRAWING_GESTURE_TOOL_IDS`] this covers the
/// command enum exactly (pinned by `retained_route_dispositions_are_exact_and_exhaustive`): an id
/// missing from both lists is unreachable from the client, because `validate_tool_job_rows` demands
/// one proof row per `Migrated` generated id and `validate_ui_dispatch_classification` rejects
/// anything that is not `Migrated`.
pub const DRAWING_BOUNDED_TOOL_IDS: &[&str] = &[
    "deleteSelection",
    "nudgeSelectionLeft",
    "nudgeSelectionLeftFast",
    "nudgeSelectionRight",
    "nudgeSelectionRightFast",
    "nudgeSelectionUp",
    "nudgeSelectionUpFast",
    "nudgeSelectionDown",
    "nudgeSelectionDownFast",
    "setSnapshot",
    "commitDocument",
    "loadDocumentJson",
    "setActiveExample",
    "setSelectedOpacity",
    "engagementSubmit",
    "addLayer",
    "dropLayerKind",
    "moveLayer",
    "deleteLayer",
    "duplicateLayer",
    "toggleLayerVisible",
    "combineBoolean",
    "patchLayer",
    "patchLayers",
    "setCamera",
    "setCameraZoom",
    "engagementInput",
    "editSelection",
    "editFill",
];

pub(super) const DRAWING_EXPORT_TOOL_IDS: &[&str] = &["exportDocument"];

pub(super) const DRAWING_CLIPBOARD_RESERVED_IDS: &[&str] = &["copy","cut","paste"];

pub(super) const DRAWING_EXPORT_PUBLICATION_CONTRACTS: &[semio_framework_plugin::ArtifactToolPublicationContract] = &[
    semio_framework_plugin::ArtifactToolPublicationContract { tool_id: "exportDocument", lanes: &[semio_framework_plugin::ArtifactToolPublicationLane::HostOnly] },
];

pub const DRAWING_IMAGE_RAW_BYTES:usize=89_481_584;

pub const DRAWING_IMAGE_TOOL_IDS: &[&str] = &["importImage"];

pub const DRAWING_IMAGE_PUBLICATION_CONTRACTS: &[semio_framework_plugin::ArtifactToolPublicationContract] = &[
    semio_framework_plugin::ArtifactToolPublicationContract { tool_id: "importImage", lanes: &[semio_framework_plugin::ArtifactToolPublicationLane::Artifact, semio_framework_plugin::ArtifactToolPublicationLane::Interaction] },
];

pub const DRAWING_PATH_TOOL_IDS: &[&str] = &["editPath"];

pub const DRAWING_PATH_PUBLICATION_CONTRACTS: &[semio_framework_plugin::ArtifactToolPublicationContract] = &[
    semio_framework_plugin::ArtifactToolPublicationContract { tool_id: "editPath", lanes: &[semio_framework_plugin::ArtifactToolPublicationLane::Artifact] },
];

pub const DRAWING_BOUNDED_PAYLOAD_SCHEMA: &str = "drawing.tool-command.v1";

const DRAWING_BOUNDED_RAW_BYTES: usize = 65_536;

pub(super) const DRAWING_BOUNDED_WORK_ITEMS: usize = 4_096;

/// 🛣️ One publication lane row per bounded route, read off each handler's real `Emit` construction
/// (`🎮️commands/*/🦀️.rs`). The four whole-document routes emit nothing but an `Effect::LoadDocument`
/// — effects are not a store lane, so those are honestly `HostOnly`; the layer routes emit
/// `DrawingMutation`s; the view routes emit exact Canvas window mutations.
const DRAWING_BOUNDED_PUBLICATION_CONTRACTS: &[semio_framework_plugin::ArtifactToolPublicationContract] = &[
    semio_framework_plugin::ArtifactToolPublicationContract { tool_id: "deleteSelection", lanes: &[semio_framework_plugin::ArtifactToolPublicationLane::Artifact] },
    semio_framework_plugin::ArtifactToolPublicationContract { tool_id: "nudgeSelectionLeft", lanes: &[semio_framework_plugin::ArtifactToolPublicationLane::Artifact] },
    semio_framework_plugin::ArtifactToolPublicationContract { tool_id: "nudgeSelectionLeftFast", lanes: &[semio_framework_plugin::ArtifactToolPublicationLane::Artifact] },
    semio_framework_plugin::ArtifactToolPublicationContract { tool_id: "nudgeSelectionRight", lanes: &[semio_framework_plugin::ArtifactToolPublicationLane::Artifact] },
    semio_framework_plugin::ArtifactToolPublicationContract { tool_id: "nudgeSelectionRightFast", lanes: &[semio_framework_plugin::ArtifactToolPublicationLane::Artifact] },
    semio_framework_plugin::ArtifactToolPublicationContract { tool_id: "nudgeSelectionUp", lanes: &[semio_framework_plugin::ArtifactToolPublicationLane::Artifact] },
    semio_framework_plugin::ArtifactToolPublicationContract { tool_id: "nudgeSelectionUpFast", lanes: &[semio_framework_plugin::ArtifactToolPublicationLane::Artifact] },
    semio_framework_plugin::ArtifactToolPublicationContract { tool_id: "nudgeSelectionDown", lanes: &[semio_framework_plugin::ArtifactToolPublicationLane::Artifact] },
    semio_framework_plugin::ArtifactToolPublicationContract { tool_id: "nudgeSelectionDownFast", lanes: &[semio_framework_plugin::ArtifactToolPublicationLane::Artifact] },
    semio_framework_plugin::ArtifactToolPublicationContract { tool_id: "setSnapshot", lanes: &[semio_framework_plugin::ArtifactToolPublicationLane::HostOnly] },
    semio_framework_plugin::ArtifactToolPublicationContract { tool_id: "commitDocument", lanes: &[semio_framework_plugin::ArtifactToolPublicationLane::HostOnly] },
    semio_framework_plugin::ArtifactToolPublicationContract { tool_id: "loadDocumentJson", lanes: &[semio_framework_plugin::ArtifactToolPublicationLane::HostOnly] },
    semio_framework_plugin::ArtifactToolPublicationContract { tool_id: "setActiveExample", lanes: &[semio_framework_plugin::ArtifactToolPublicationLane::HostOnly] },
    semio_framework_plugin::ArtifactToolPublicationContract { tool_id: "setSelectedOpacity", lanes: &[semio_framework_plugin::ArtifactToolPublicationLane::Artifact] },
    semio_framework_plugin::ArtifactToolPublicationContract { tool_id: "engagementSubmit", lanes: &[semio_framework_plugin::ArtifactToolPublicationLane::Artifact] },
    semio_framework_plugin::ArtifactToolPublicationContract { tool_id: "addLayer", lanes: &[semio_framework_plugin::ArtifactToolPublicationLane::Artifact] },
    semio_framework_plugin::ArtifactToolPublicationContract { tool_id: "dropLayerKind", lanes: &[semio_framework_plugin::ArtifactToolPublicationLane::Artifact] },
    semio_framework_plugin::ArtifactToolPublicationContract { tool_id: "moveLayer", lanes: &[semio_framework_plugin::ArtifactToolPublicationLane::Artifact] },
    semio_framework_plugin::ArtifactToolPublicationContract { tool_id: "deleteLayer", lanes: &[semio_framework_plugin::ArtifactToolPublicationLane::Artifact] },
    semio_framework_plugin::ArtifactToolPublicationContract { tool_id: "duplicateLayer", lanes: &[semio_framework_plugin::ArtifactToolPublicationLane::Artifact] },
    semio_framework_plugin::ArtifactToolPublicationContract { tool_id: "toggleLayerVisible", lanes: &[semio_framework_plugin::ArtifactToolPublicationLane::Artifact] },
    semio_framework_plugin::ArtifactToolPublicationContract { tool_id: "editSelection", lanes: &[semio_framework_plugin::ArtifactToolPublicationLane::Artifact] },
    semio_framework_plugin::ArtifactToolPublicationContract { tool_id: "editFill", lanes: &[semio_framework_plugin::ArtifactToolPublicationLane::Artifact] },
    semio_framework_plugin::ArtifactToolPublicationContract { tool_id: "combineBoolean", lanes: &[semio_framework_plugin::ArtifactToolPublicationLane::Artifact] },
    semio_framework_plugin::ArtifactToolPublicationContract { tool_id: "patchLayer", lanes: &[semio_framework_plugin::ArtifactToolPublicationLane::Artifact] },
    semio_framework_plugin::ArtifactToolPublicationContract { tool_id: "patchLayers", lanes: &[semio_framework_plugin::ArtifactToolPublicationLane::Artifact] },
    semio_framework_plugin::ArtifactToolPublicationContract { tool_id: "setCamera", lanes: &[semio_framework_plugin::ArtifactToolPublicationLane::WindowConfig] },
    semio_framework_plugin::ArtifactToolPublicationContract { tool_id: "setCameraZoom", lanes: &[semio_framework_plugin::ArtifactToolPublicationLane::WindowConfig] },
    semio_framework_plugin::ArtifactToolPublicationContract { tool_id: "engagementInput", lanes: &[semio_framework_plugin::ArtifactToolPublicationLane::WindowTransient] },
];

fn drawing_bounded_contract() -> semio_framework::ToolExecutionContract {
    semio_framework::ToolExecutionContract::bounded_first_step(DRAWING_BOUNDED_RAW_BYTES, DRAWING_BOUNDED_WORK_ITEMS, 1, 262_144, 7_500)
}

/// 📏️ One work item per bounded dispatch, admitted only while the live document still fits the
/// declared envelope — a gesture id or an oversized document answers `None`, which fails the route
/// closed instead of publishing an unbounded step.
fn drawing_bounded_extent(command: &DrawingCommand, snapshot: &DrawingSnapshot, _interaction: &::protocol::InteractionState) -> Option<usize> {
    if !DRAWING_BOUNDED_TOOL_IDS.contains(&command.command_id()) {
        return None;
    }
    let items = [snapshot.layers.len(), snapshot.assets.len()].into_iter().try_fold(1usize, |total, count| total.checked_add(count))?;
    (items <= DRAWING_BOUNDED_WORK_ITEMS).then_some(1)
}

pub struct DrawingWindowCommandWork {
    pub(super) tool_id: &'static str,
    pub(super) completed: bool,
}

impl DrawingWindowCommandWork {
    pub(super) fn new(tool_id: &'static str) -> Self { Self { tool_id, completed: false } }
}

impl semio_framework_plugin::retained_command::ArtifactCommandWork<semio_framework_plugin::EditorApp<DrawingPlayApp>> for DrawingWindowCommandWork {
    fn terminal_frame_release_bytes(&self)->Option<usize>{self.terminal_is_empty().then_some(size_of::<Self>())}
    fn tool_id(&self) -> &'static str { self.tool_id }

    fn work_demands(&self, _input: &semio_framework_plugin::retained_command::ArtifactCommandInputs<'_, semio_framework_plugin::EditorApp<DrawingPlayApp>>, _maximum_copy_bytes: usize) -> Result<semio_framework_value::RetirementDemand, semio_framework_value::ValueError> {
        Ok(semio_framework_value::RetirementDemand { copy_bytes: size_of::<Self>(), depth: 1, ..Default::default() })
    }

    fn extent(
        &self,
        command: &DrawingCommand,
        snapshot: &DrawingSnapshot,
        interaction: &protocol::InteractionState,
        context: Option<&semio_framework_plugin::app::ArtifactOwnedToolJobContext<semio_framework_plugin::EditorApp<DrawingPlayApp>>>,
    ) -> Option<usize> {
        if self.completed || command.command_id() != self.tool_id || drawing_bounded_extent(command, snapshot, interaction) != Some(1) { return None; }
        match command {
            DrawingCommand::SetCamera(_) | DrawingCommand::SetCameraZoom(_) | DrawingCommand::EngagementInput(_) => context?.view_state.as_ref().map(|_| 1),
            _ => Some(1),
        }
    }

    fn step(
        &mut self,
        input: &semio_framework_plugin::retained_command::ArtifactCommandInputs<'_, semio_framework_plugin::EditorApp<DrawingPlayApp>>,
    cx: &mut semio_framework_job::StepContext<'_>,
    ) -> Result<semio_framework_plugin::retained_command::ArtifactCommandWorkStep<semio_framework_plugin::EditorApp<DrawingPlayApp>>, Fault> {
        use semio_framework_plugin::retained_command::ArtifactCommandWorkStep;
        if self.completed || input.command.command_id() != self.tool_id { return Err(Fault::from("drawing-window-work-terminal")); }
        let doc = ArtifactView::with_operation(input.snapshot, input.history, input.operation.clone());
        let cfg = ConfigView { snapshot: input.config, window: input.context.and_then(|context| context.window_config.as_ref()) };
        let active_utility = input.context.and_then(|context| context.view_state.as_ref()).map_or(DRAWING_DEFAULT_UTILITY, drawing_active_utility);
        let mut session = DrawingSession::new(active_utility, &input.operation.authoring_seed);
        session.identity_cancel=Some(cx.cancel_token());
        session.interaction.ids = input.interaction.selection.get(DRAWING_INTERACTION_DOMAIN).map(|selection| selection.ids.clone()).unwrap_or_default();
        session.interaction.points = input.interaction.selection.get(DRAWING_POINT_DOMAIN).map(|selection| selection.ids.clone()).unwrap_or_default();
        session.window_config = canvas_window::config::from_snapshot(input.context.and_then(|context| context.window_config.as_ref()));
        session.window_transient = canvas_window::transient::from_snapshot(input.context.and_then(|context| context.window_transient.as_ref()));
        let mut emit = Emit::default();
        let mut transient = None;
        match input.command {
            DrawingCommand::SetCamera(payload) => {
                payload.camera.validate().map_err(|error| Fault::from(error.to_string()))?;
                let view = input.context.and_then(|context| context.view_state.as_ref()).ok_or_else(|| Fault::from("drawing-canvas-window-required"))?;
                let mut config = session.window_config;
                config.viewport = payload.camera;
                config.framed = true;
                emit.window_config_mutations.push(canvas_window::config::addressed(view, config)?);
            }
            DrawingCommand::SetCameraZoom(payload) => {
                let view = input.context.and_then(|context| context.view_state.as_ref()).ok_or_else(|| Fault::from("drawing-canvas-window-required"))?;
                let mut config = session.window_config;
                config.viewport.zoom = payload.value;
                config.framed = true;
                config.viewport.validate().map_err(|error| Fault::from(error.to_string()))?;
                emit.window_config_mutations.push(canvas_window::config::addressed(view, config)?);
            }
            DrawingCommand::EngagementInput(payload) => {
                let view = input.context.and_then(|context| context.view_state.as_ref()).ok_or_else(|| Fault::from("drawing-canvas-window-required"))?;
                session.window_transient.engagement_input = payload.value.clone();
                transient = Some(canvas_window::transient::addressed(view, session.window_transient)?);
            }
            _ => emit = input.command.dispatch(&doc, &cfg, &mut session)?,
        }
        cx.consume_fuel(session.identity_checkpoints);
        session.identity_cancel=None;
        self.completed = true;
        Ok(match transient {
            Some(mutation) => ArtifactCommandWorkStep::CompleteWithEphemeral {
                emit,
                ephemeral: semio_framework_plugin::EphemeralEmit { window_transient: vec![mutation], ..Default::default() },
            },
            None => ArtifactCommandWorkStep::Complete(emit),
        })
    }
}

pub struct DrawingBoundedCommandJobFactory {
    pub(super) keys: Vec<semio_framework::ToolFactoryKey>,
}

impl DrawingBoundedCommandJobFactory {
    pub(super) fn new(controller_id: &str) -> Self {
        Self { keys: DRAWING_BOUNDED_TOOL_IDS.iter().map(|tool| semio_framework::ToolFactoryKey::new(controller_id, *tool)).collect() }
    }
}

impl semio_framework::ToolJobFactory for DrawingBoundedCommandJobFactory {
    type Payload = semio_framework_plugin::retained_command::ArtifactRetainedCommandPayload<semio_framework_plugin::EditorApp<DrawingPlayApp>>;
    type Job = semio_framework_plugin::retained_command::ArtifactRetainedCommandJob<semio_framework_plugin::EditorApp<DrawingPlayApp>>;

    fn keys(&self) -> &[semio_framework::ToolFactoryKey] {
        &self.keys
    }

    fn payload_schema_id(&self) -> &str {
        DRAWING_BOUNDED_PAYLOAD_SCHEMA
    }

    fn classification(&self) -> semio_framework::InteractiveJobClassification {
        semio_framework::InteractiveJobClassification::Migrated
    }

    fn execution_contract(&self) -> semio_framework::ToolExecutionContract {
        drawing_bounded_contract()
    }

    fn create_job(&mut self, _operation: semio_framework_job::Operation, payload: Self::Payload) -> Result<Self::Job, semio_framework::ToolJobFactoryError> {
        Ok(semio_framework_plugin::retained_command::ArtifactRetainedCommandJob::new(payload))
    }

    fn create_job_from_wire_pages_with_payload(
        &mut self,
        _operation: semio_framework_job::Operation,
        payload: Self::Payload,
        input: semio_framework_action_bus::RetainedToolWireInput,
        checkpoint: Option<semio_framework_action_bus::RetainedToolWireInput>,
    ) -> Result<Self::Job, (semio_framework::ToolJobFactoryError, semio_framework_action_bus::RetainedToolWireInput, Option<semio_framework_action_bus::RetainedToolWireInput>)> {
        if checkpoint.is_some() || input.declared_bytes() > DRAWING_BOUNDED_RAW_BYTES {
            return Err((semio_framework::ToolJobFactoryError::new("bounded Drawing command ingress rejects a checkpoint or oversized wire owner"), input, checkpoint));
        }
        Ok(semio_framework_plugin::retained_command::ArtifactRetainedCommandJob::from_wire(payload, input))
    }
}

impl semio_framework_plugin::ArtifactOwnedToolJobFactory for DrawingBoundedCommandJobFactory {
    type Owner = semio_framework_plugin::EditorApp<DrawingPlayApp>;
    const TOOL_IDS: &'static [&'static str] = DRAWING_BOUNDED_TOOL_IDS;
    const DOCUMENT_SCHEMA: &'static str = DRAWING_DOCUMENT_SCHEMA;
    const PUBLICATION_CONTRACTS: &'static [semio_framework_plugin::ArtifactToolPublicationContract] = DRAWING_BOUNDED_PUBLICATION_CONTRACTS;
}

/// 🏗️ The bounded half of `DrawingPlayApp::build_tool_job` — one `BoundedArtifactCommandWork` per
/// non-gesture route, refused outright when the id, the typed command and the live extent disagree.
pub(super) fn drawing_bounded_tool_job(request: semio_framework_plugin::ArtifactOwnedToolJobRequest<semio_framework_plugin::EditorApp<DrawingPlayApp>>) -> Result<semio_framework::ToolOperationSpec, Fault> {
    if request.command.command_id() != request.tool_id || drawing_bounded_extent(&request.command, &request.snapshot, &request.interaction_state) != Some(1) {
        return Err(Fault::new(FaultOrigin::App, FaultCode::new("app.command.tool-mismatch"), "bounded Drawing command does not match its exact registered tool or exceeds its declared extent"));
    }
    let tool_id = request.command.command_id();
    let work: Box<dyn semio_framework_plugin::retained_command::ArtifactCommandWork<semio_framework_plugin::EditorApp<DrawingPlayApp>>> = Box::new(DrawingWindowCommandWork::new(tool_id));
    let operation_context = semio_framework_plugin::AppOperationContext {
        app_instance_id: request.app_instance_id,
        parent_document_id: request.parent_document_id.clone(),
        operation_id: request.operation.operation.0,
        generation: request.operation.generation.0,
        canonical_base_revision: request.canonical_base_revision,
        retained: request.retained,
        authoring_seed: request.authoring_seed.clone(),
    };
    let payload = semio_framework_plugin::retained_command::ArtifactRetainedCommandPayload::new(
        semio_framework_plugin::retained_command::ArtifactRetainedCommandInputs {
            command: *request.command,
            snapshot: request.snapshot,
            snapshot_read: request.snapshot_read,
            config: request.config,
            history: request.history,
            interaction_state: request.interaction_state,
            interaction_hover: request.interaction_hover,
            context: Some(request.context),
            operation: operation_context,
            completion: request.completion,
        },
        DrawingCommand::command_id,
        DRAWING_BOUNDED_RAW_BYTES,
        DRAWING_BOUNDED_WORK_ITEMS,
        work,
    );
    Ok(semio_framework::ToolOperationSpec::new(request.controller_id, request.tool_id, request.payload_schema_id, payload, request.operation))
}
//#endregion 🧵️BoundedCommands
