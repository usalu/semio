//! 🧰️ Retained commands: the tool id catalog, the publication contracts, the bounded reducer and the retained command job factory of the block 3d editor.

use super::*;

//#region 🧵️RetainedCommands
/// 🧾️ Every block3d tool id, in `Block3dCommand` declaration order — a bijection with the enum's 23
/// rows, with `BLOCK3D_PUBLICATION_CONTRACTS`, and with the `.action_interactive_job(…, Migrated)` set
/// `create_block3d_app` declares (asserted by `retained_route_dispositions_are_exact_and_exhaustive`).
/// `AppActionRegistry::tool_job_registration` enforces exactly that set equality at construction time:
/// a row missing here, or an action left `Unclassified`, faults the whole app with
/// `interactive-job.catalog-incomplete` instead of silently going dispatch-dead at the UI gate.
pub(super) const BLOCK3D_RETAINED_TOOL_IDS: &[&str] = &[
    "patchObjectKind",
    "addRepresentation",
    "removeRepresentation",
    "addVortexKind",
    "removeVortexKind",
    "addVortex",
    "removeVortex",
    "setActiveExample",
    "edit",
    "setActiveRepresentation",
    "setWindowRepresentations",
    "toggleWindowRepresentation",
    "setWindowArrangement",
    "setWindowSpacing",
    "setBrushVortexKind",
    "setBrushRadius",
    "setBrushFlip",
    "worldSurfaceHover",
    "worldSurfaceLeave",
    "worldSurfacePlace",
    "setCamera",
    "patchRepresentation",
];
const BLOCK3D_RETAINED_PAYLOAD_SCHEMA: &str = "block.3d.tool-command.v1";
pub(super) const BLOCK3D_RETAINED_RAW_BYTES: usize = 65_536;
pub(super) const BLOCK3D_RETAINED_WORK_ITEMS: usize = 4_096;
/// 🎒️ Real bound for one Artifact-lane edit: `setActiveExample`/`edit` replay a whole example fixture
/// (`📚️examples/*/🖼️assets/*/🗣️.dsl.semio`, ~1–2 KB of text) as the single largest document mutation any
/// of the 23 tools emits, so 64 KiB is a real ceiling rather than a rubber stamp.
pub(super) const BLOCK3D_ARTIFACT_STORE_MAXIMUM_BYTES: usize = 65_536;
/// 🎒️ Real bound for one Config-lane edit: a field-sparse `Block3dConfigDiff` whose largest member is the per-window
/// `representation_ids` view — 256 KiB covers that without being unbounded.
pub(super) const BLOCK3D_CONFIG_STORE_MAXIMUM_BYTES: usize = 262_144;

pub(super) const BLOCK3D_PUBLICATION_CONTRACTS: &[ArtifactToolPublicationContract] = &[
    ArtifactToolPublicationContract { tool_id: "patchObjectKind", lanes: &[ArtifactToolPublicationLane::Artifact] },
    ArtifactToolPublicationContract { tool_id: "addRepresentation", lanes: &[ArtifactToolPublicationLane::Artifact] },
    ArtifactToolPublicationContract { tool_id: "removeRepresentation", lanes: &[ArtifactToolPublicationLane::Artifact] },
    ArtifactToolPublicationContract { tool_id: "addVortexKind", lanes: &[ArtifactToolPublicationLane::Artifact] },
    ArtifactToolPublicationContract { tool_id: "removeVortexKind", lanes: &[ArtifactToolPublicationLane::Artifact] },
    ArtifactToolPublicationContract { tool_id: "addVortex", lanes: &[ArtifactToolPublicationLane::Artifact] },
    ArtifactToolPublicationContract { tool_id: "removeVortex", lanes: &[ArtifactToolPublicationLane::Artifact] },
    ArtifactToolPublicationContract { tool_id: "setActiveExample", lanes: &[ArtifactToolPublicationLane::HostOnly] },
    ArtifactToolPublicationContract { tool_id: "edit", lanes: &[ArtifactToolPublicationLane::HostOnly] },
    ArtifactToolPublicationContract { tool_id: "setActiveRepresentation", lanes: &[ArtifactToolPublicationLane::Config] },
    ArtifactToolPublicationContract { tool_id: "setWindowRepresentations", lanes: &[ArtifactToolPublicationLane::Config] },
    ArtifactToolPublicationContract { tool_id: "toggleWindowRepresentation", lanes: &[ArtifactToolPublicationLane::Config] },
    ArtifactToolPublicationContract { tool_id: "setWindowArrangement", lanes: &[ArtifactToolPublicationLane::Config] },
    ArtifactToolPublicationContract { tool_id: "setWindowSpacing", lanes: &[ArtifactToolPublicationLane::Config] },
    ArtifactToolPublicationContract { tool_id: "setBrushVortexKind", lanes: &[ArtifactToolPublicationLane::Config] },
    ArtifactToolPublicationContract { tool_id: "setBrushRadius", lanes: &[ArtifactToolPublicationLane::Config] },
    ArtifactToolPublicationContract { tool_id: "setBrushFlip", lanes: &[ArtifactToolPublicationLane::Config] },
    ArtifactToolPublicationContract { tool_id: "worldSurfaceHover", lanes: &[ArtifactToolPublicationLane::WindowTransient] },
    ArtifactToolPublicationContract { tool_id: "worldSurfaceLeave", lanes: &[ArtifactToolPublicationLane::WindowTransient] },
    ArtifactToolPublicationContract { tool_id: "worldSurfacePlace", lanes: &[ArtifactToolPublicationLane::Artifact, ArtifactToolPublicationLane::WindowTransient] },
    ArtifactToolPublicationContract { tool_id: "setCamera", lanes: &[ArtifactToolPublicationLane::Config] },
    ArtifactToolPublicationContract { tool_id: "patchRepresentation", lanes: &[ArtifactToolPublicationLane::Artifact] },
];

pub(super) fn block3d_bounded_contract() -> ToolExecutionContract {
    ToolExecutionContract::bounded_first_step(BLOCK3D_RETAINED_RAW_BYTES, 4_096, 1, 262_144, 7_500)
}

pub(super) fn block3d_retained_extent(command: &Block3dCommand, snapshot: &Block3dSnapshot, _interaction: &crate::protocol::InteractionState) -> Option<usize> {
    if !BLOCK3D_RETAINED_TOOL_IDS.contains(&command.command_id()) {
        return None;
    }
    let collections = [snapshot.representations.len(), snapshot.vortex_kind_extra.len(), snapshot.vortices.len(), snapshot.compatibility.len(), snapshot.attributes.len(), snapshot.authors.len()];
    let items = collections.into_iter().try_fold(1usize, |total, count| total.checked_add(count))?;
    (items <= BLOCK3D_RETAINED_WORK_ITEMS).then_some(1)
}

#[expect(clippy::too_many_arguments, reason = "The retained command reducer implements the framework's eight-argument callback contract.")]
pub(super) fn block3d_retained_reduce(
    command: &Block3dCommand,
    snapshot: &Block3dSnapshot,
    config: &Block3dConfig,
    history: &semio_framework_plugin::HistoryView,
    _interaction: &crate::protocol::InteractionState,
    _hover: &semio_framework_plugin::app::InteractionHoverState,
    _context: Option<&semio_framework_plugin::app::ArtifactOwnedToolJobContext<EditorApp<Block3dPlayApp>>>,
    operation: &AppOperationContext,
) -> Result<Emit<Block3dMutation, Block3dConfigMutation, NoDraftMutation>, Fault> {
    command.dispatch(&ArtifactView::with_operation(snapshot, history, operation.clone()), &ConfigView { snapshot: config, window: None })
}

pub(super) struct Block3dWindowPreviewWork {
    pub(super) tool_id: &'static str,
    consumed: bool,
}

impl Block3dWindowPreviewWork {
    pub(super) fn new(tool_id: &'static str) -> Self {
        Self { tool_id, consumed: false }
    }
}

impl ArtifactCommandWork<EditorApp<Block3dPlayApp>> for Block3dWindowPreviewWork {
    fn tool_id(&self) -> &'static str {
        self.tool_id
    }

    fn extent(
        &self,
        command: &Block3dCommand,
        _snapshot: &Block3dSnapshot,
        _interaction: &crate::protocol::InteractionState,
        context: Option<&semio_framework_plugin::app::ArtifactOwnedToolJobContext<EditorApp<Block3dPlayApp>>>,
    ) -> Option<usize> {
        let view_state = context.and_then(|context| context.view_state.as_ref())?;
        let targets_world = view_state.active_window_kind_id.as_deref() == Some(world::BLOCK3D_WINDOW_WORLD)
            && view_state.window_id.as_deref().is_some_and(|window_id| !window_id.is_empty());
        (targets_world && command.command_id() == self.tool_id).then_some(1)
    }

    fn step(&mut self, input: &ArtifactCommandInputs<'_, EditorApp<Block3dPlayApp>>, _cx: &mut semio_framework_job::StepContext<'_>) -> Result<ArtifactCommandWorkStep<EditorApp<Block3dPlayApp>>, semio_framework_job::OriginalTurnFailure> {
        let original: Result<_, Fault> = (|| {
        if self.consumed {
            return Err(Fault::from("block3d-window-preview-work-repeated"));
        }
        self.consumed = true;
        let view_state = input.context.and_then(|context| context.view_state.as_ref()).ok_or_else(|| Fault::from("block3d-window-preview-requires-view"))?;
        if view_state.active_window_kind_id.as_deref() != Some(world::BLOCK3D_WINDOW_WORLD) {
            return Err(Fault::from("block3d-window-preview-kind-mismatch"));
        }
        let window_id = view_state.window_id.as_deref().filter(|window_id| !window_id.is_empty()).ok_or_else(|| Fault::from("block3d-window-preview-requires-concrete-window"))?;
        let (emit, preview) = match input.command {
            Block3dCommand::HoverSurface(payload) => (
                Emit::default(),
                Some(Block3dBrushPreview { position: payload.position, direction: payload.normal }),
            ),
            Block3dCommand::LeaveSurface(_) => (Emit::default(), None),
            Block3dCommand::PlaceVortex(payload) => {
                let trusted = place_vortex::PlaceVortex {
                    window_id: window_id.to_string(),
                    object_id: payload.object_id.clone(),
                    position: payload.position,
                    normal: payload.normal,
                };
                let doc = ArtifactView::with_operation(input.snapshot, input.history, input.operation.clone());
                let cfg = ConfigView { snapshot: input.config, window: None };
                (place_vortex::handle(&trusted, &doc, &cfg)?, None)
            }
            _ => return Err(Fault::from("block3d-window-preview-route-mismatch")),
        };
        let mutation = semio_framework_plugin::WindowTransientMutation::of::<Block3dWorldWindowTransientOwner>(
            window_id,
            SetBrushPreview { preview }.into(),
        );
        Ok(ArtifactCommandWorkStep::CompleteWithEphemeral {
            emit,
            ephemeral: EphemeralEmit { presence: Vec::new(), transient: Vec::new(), window_transient: vec![mutation] },
        })
    
        })();
        original.map_err(semio_framework_job::OriginalTurnFailure::from)
    }
}

pub(super) struct Block3dRetainedCommandJobFactory {
    keys: Vec<ToolFactoryKey>,
}

impl Block3dRetainedCommandJobFactory {
    pub(super) fn new(controller_id: &str) -> Self {
        Self { keys: BLOCK3D_RETAINED_TOOL_IDS.iter().map(|tool_id| ToolFactoryKey::new(controller_id, *tool_id)).collect() }
    }
}

impl semio_framework::ToolJobFactory for Block3dRetainedCommandJobFactory {
    type Payload = ArtifactRetainedCommandPayload<EditorApp<Block3dPlayApp>>;
    type Job = ArtifactRetainedCommandJob<EditorApp<Block3dPlayApp>>;

    fn keys(&self) -> &[ToolFactoryKey] {
        &self.keys
    }
    fn payload_schema_id(&self) -> &str {
        BLOCK3D_RETAINED_PAYLOAD_SCHEMA
    }
    fn classification(&self) -> InteractiveJobClassification {
        InteractiveJobClassification::Migrated
    }
    fn execution_contract(&self) -> ToolExecutionContract {
        block3d_bounded_contract()
    }
    fn create_job(&mut self, _operation: semio_framework_job::Operation, payload: Self::Payload) -> Result<Self::Job, ToolJobFactoryError> {
        Ok(ArtifactRetainedCommandJob::new(payload))
    }

    fn create_job_from_wire_pages_with_payload(
        &mut self,
        _operation: semio_framework_job::Operation,
        payload: Self::Payload,
        input: semio_framework_action_bus::RetainedToolWireInput,
        checkpoint: Option<semio_framework_action_bus::RetainedToolWireInput>,
    ) -> Result<Self::Job, (ToolJobFactoryError, semio_framework_action_bus::RetainedToolWireInput, Option<semio_framework_action_bus::RetainedToolWireInput>)> {
        if input.declared_bytes() > BLOCK3D_RETAINED_RAW_BYTES || checkpoint.is_some() {
            return Err((ToolJobFactoryError::new("Block3d retained command rejects oversized wire or checkpoint owner"), input, checkpoint));
        }
        Ok(ArtifactRetainedCommandJob::from_wire(payload, input))
    }
}

impl ArtifactOwnedToolJobFactory for Block3dRetainedCommandJobFactory {
    type Owner = EditorApp<Block3dPlayApp>;
    const TOOL_IDS: &'static [&'static str] = BLOCK3D_RETAINED_TOOL_IDS;
    const DOCUMENT_SCHEMA: &'static str = BLOCK_3D_SCHEMA;
    const PUBLICATION_CONTRACTS: &'static [ArtifactToolPublicationContract] = BLOCK3D_PUBLICATION_CONTRACTS;
}
//#endregion 🧵️RetainedCommands
