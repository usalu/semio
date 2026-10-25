//! 🚧️ Retained commands of the remodeling editor: the retained tool ids and publication contracts, the routes and the window-config command work with its job factory. See [`super`].

use super::*;

//#region 🔖️RemodelingPlayApp
//#region 🧵️RetainedCommands
/// 🧵️ Every `RemodelingCommand` row, in `app_commands!` declaration order. The framework demands set
/// EQUALITY between this list, the `Migrated` classification list in `🔖️Manifest` and the
/// `bounded_first_step_tool_proofs!` rows (`validate_tool_job_rows`: `expected = TOOL_JOB_IDS ∩ migrated`
/// must equal the proof set, else `interactive-job.catalog-incomplete`), so all three are kept in one
/// order and asserted equal by `retained_route_dispositions_are_exact_and_exhaustive`.
pub(super) const REMODELING_RETAINED_TOOL_IDS: &[&str] = &[
    "importFramePayload",
    "importVideoFramePayload",
    "importVideoDone",
    "importVideoBytesPayload",
    "addStream",
    "removeStream",
    "setStreamSync",
    "editCalibration",
    "calibrateCameras",
    "addGcp",
    "removeGcp",
    "placeGcpObservation",
    "setIngestParams",
    "setFeatureParams",
    "setMatchParams",
    "setSfmParams",
    "setDenseParams",
    "setMeshParams",
    "setMotionParams",
    "setGeoParams",
    "resetPlaceholderMesh",
    "clearSparse",
    "clearDense",
    "clearMeshResult",
    "clearTracks",
    "clearGeoProducts",
    "clearResult",
    "setCamera",
    "setLayerVisibility",
    "setFrameCursor",
    "setReportTable",
    "importFrames",
    "importVideo",
    "exportQcReport",
    "setActiveExample",
    "importAbort",
];

const REMODELING_RETAINED_PAYLOAD_SCHEMA: &str = "remodeling.scene.tool-command.v1";

pub(super) const REMODELING_RETAINED_RAW_BYTES: usize = 65_536;

pub(super) const REMODELING_RETAINED_WORK_ITEMS: usize = 4_096;

/// 🛣️ The publication lanes per route, read off each handler's own `Emit` in `🎮️commands/*/🦀️.rs`:
/// every ingestion, calibration, parameter and clear verb builds
/// `Emit::mutations(..)`/`Emit { artifact_mutations, .. }` over `RemodelingMutation`
/// (`Artifact`); the four streamed-import verbs (`importFramePayload`, `importVideoFramePayload`, `importVideoDone`,
/// `importAbort`) also write the importing window's tool state (`WindowTransient`); the four view verbs publish to their
/// exact concrete `WindowConfig` owner; the three shell verbs build `Emit::effect(..)` only — `RequestFileOpen`,
/// `RequestMediaFrames`, `DownloadMediaExport` — and never touch a store (`HostOnly`). No remodeling
/// handler emits a draft, a presence or an app transient mutation.
const REMODELING_PUBLICATION_CONTRACTS: &[semio_framework_plugin::ArtifactToolPublicationContract] = &[
    import_route("importFramePayload"),
    import_route("importVideoFramePayload"),
    import_route("importVideoDone"),
    artifact_route("importVideoBytesPayload"),
    artifact_route("addStream"),
    artifact_route("removeStream"),
    artifact_route("setStreamSync"),
    artifact_route("editCalibration"),
    artifact_route("calibrateCameras"),
    artifact_route("addGcp"),
    artifact_route("removeGcp"),
    artifact_route("placeGcpObservation"),
    artifact_route("setIngestParams"),
    artifact_route("setFeatureParams"),
    artifact_route("setMatchParams"),
    artifact_route("setSfmParams"),
    artifact_route("setDenseParams"),
    artifact_route("setMeshParams"),
    artifact_route("setMotionParams"),
    artifact_route("setGeoParams"),
    artifact_route("resetPlaceholderMesh"),
    artifact_route("clearSparse"),
    artifact_route("clearDense"),
    artifact_route("clearMeshResult"),
    artifact_route("clearTracks"),
    artifact_route("clearGeoProducts"),
    artifact_route("clearResult"),
    window_config_route("setCamera"),
    window_config_route("setLayerVisibility"),
    window_config_route("setFrameCursor"),
    window_config_route("setReportTable"),
    host_route("importFrames"),
    host_route("importVideo"),
    host_route("exportQcReport"),
    artifact_route("setActiveExample"),
    import_route("importAbort"),
];

/// 📥️ One import-lane publication row: the streamed document edit plus the importing window's tool state.
const fn import_route(tool_id: &'static str) -> semio_framework_plugin::ArtifactToolPublicationContract {
    semio_framework_plugin::ArtifactToolPublicationContract { tool_id, lanes: &[semio_framework_plugin::ArtifactToolPublicationLane::Artifact, semio_framework_plugin::ArtifactToolPublicationLane::WindowTransient] }
}

/// 📄️ One document-lane publication row.
const fn artifact_route(tool_id: &'static str) -> semio_framework_plugin::ArtifactToolPublicationContract {
    semio_framework_plugin::ArtifactToolPublicationContract { tool_id, lanes: &[semio_framework_plugin::ArtifactToolPublicationLane::Artifact] }
}

/// 👁️ One exact window-config publication row.
const fn window_config_route(tool_id: &'static str) -> semio_framework_plugin::ArtifactToolPublicationContract {
    semio_framework_plugin::ArtifactToolPublicationContract { tool_id, lanes: &[semio_framework_plugin::ArtifactToolPublicationLane::WindowConfig] }
}

/// 🐚️ One shell-effect-only publication row.
const fn host_route(tool_id: &'static str) -> semio_framework_plugin::ArtifactToolPublicationContract {
    semio_framework_plugin::ArtifactToolPublicationContract { tool_id, lanes: &[semio_framework_plugin::ArtifactToolPublicationLane::HostOnly] }
}

fn remodeling_retained_contract() -> ToolExecutionContract {
    ToolExecutionContract::bounded_first_step(REMODELING_RETAINED_RAW_BYTES, REMODELING_RETAINED_WORK_ITEMS, 1, 262_144, 7_500)
}

/// 📏️ One bounded first step per retained route, admitted only while the WHOLE addressable document
/// (streams and their frames, embedded assets, ground control points and their observations, calibrated
/// cameras) plus this edit fit inside `REMODELING_RETAINED_WORK_ITEMS`. `streams.len()` alone would
/// understate a document whose single stream carries a thousand frames, so the frames are counted too.
pub(super) fn remodeling_retained_extent(command: &RemodelingCommand, snapshot: &RemodelingSnapshot, _interaction: &protocol::InteractionState) -> Option<usize> {
    if !REMODELING_RETAINED_TOOL_IDS.contains(&command.command_id()) {
        return None;
    }
    let frames: usize = snapshot.streams.iter().map(|stream| stream.frames.len()).sum();
    let observations: usize = snapshot.gcps.iter().map(|gcp| gcp.observations.len()).sum();
    let items = snapshot.streams.len().checked_add(frames)?.checked_add(snapshot.assets.len())?.checked_add(snapshot.gcps.len())?.checked_add(observations)?.checked_add(snapshot.calibration.cameras.len())?.checked_add(1)?;
    (items <= REMODELING_RETAINED_WORK_ITEMS).then_some(1)
}

pub(super) fn remodeling_window_config_mutation(
    command: &RemodelingCommand,
    snapshot: Option<&semio_framework_plugin::WindowConfigSnapshot>,
    view: &semio_framework_plugin::ViewModel,
) -> Result<Option<semio_framework_plugin::WindowConfigMutation>, Fault> {
    match command {
        RemodelingCommand::SetCamera(payload) => {
            model::windows::model::config::addressed_camera(view, payload.camera.clone()).map(Some)
        }
        RemodelingCommand::SetLayerVisibility(payload) => {
            let mut config = model::windows::model::config::from_snapshot(snapshot);
            match payload.layer.as_str() {
                "mesh" => config.layers.mesh = payload.visible,
                "dense" => config.layers.dense = payload.visible,
                "sparse" => config.layers.sparse = payload.visible,
                "cameras" => config.layers.cameras = payload.visible,
                "gcps" => config.layers.gcps = payload.visible,
                _ => {}
            }
            model::windows::model::config::addressed_layers(view, config.layers).map(Some)
        }
        RemodelingCommand::SetFrameCursor(payload) => {
            let mut config = capture::windows::frames::config::from_snapshot(snapshot);
            config.frame_cursor = capture::windows::frames::config::RemodelingFrameCursor { stream_id: payload.stream_id.clone(), frame_index: payload.frame_index };
            capture::windows::frames::config::addressed(view, config).map(Some)
        }
        RemodelingCommand::SetReportTable(payload) => {
            let mut config = analyze::windows::report::config::from_snapshot(snapshot);
            config.report_table = payload.table.clone();
            analyze::windows::report::config::addressed(view, config).map(Some)
        }
        _ => Ok(None),
    }
}

pub(super) struct RemodelingWindowCommandWork { pub(super) tool_id: &'static str, completed: bool }

impl RemodelingWindowCommandWork {
    pub(super) fn new(tool_id: &'static str) -> Self { Self { tool_id, completed: false } }
}

impl ArtifactCommandWork<EditorApp<RemodelingPlayApp>> for RemodelingWindowCommandWork {
    fn tool_id(&self) -> &'static str { self.tool_id }
    fn extent(
        &self,
        command: &RemodelingCommand,
        snapshot: &RemodelingSnapshot,
        interaction: &protocol::InteractionState,
        _context: Option<&semio_framework_plugin::app::ArtifactOwnedToolJobContext<EditorApp<RemodelingPlayApp>>>,
    ) -> Option<usize> {
        (!self.completed && command.command_id() == self.tool_id).then(|| remodeling_retained_extent(command, snapshot, interaction)).flatten()
    }
    fn step(
        &mut self,
        input: &semio_framework_plugin::retained_command::ArtifactCommandInputs<'_, EditorApp<RemodelingPlayApp>>,
    _cx: &mut semio_framework_job::StepContext<'_>,
    ) -> Result<semio_framework_plugin::retained_command::ArtifactCommandWorkStep<EditorApp<RemodelingPlayApp>>, semio_framework_job::OriginalTurnFailure> {
        let original: Result<_, Fault> = (|| {
        if self.completed || input.command.command_id() != self.tool_id {
            return Err(Fault::new(FaultOrigin::App, FaultCode::new("remodeling.retained.route"), "the bounded Remodeling work rejects an undeclared or completed route"));
        }
        let window = input.context.and_then(|context| context.window_config.as_ref());
        let config_view = ConfigView { snapshot: input.config, window };
        let document = ArtifactView::with_operation(input.snapshot, input.history, input.operation.clone());
        let resting = transient::from_snapshot(input.context.and_then(|context| context.window_transient.as_ref()));
        if let Some(imported) = input.command.import_in_window(&document, &config_view, &resting) {
            let (emit, next) = imported?;
            let window_transient = match next == resting {
                true => Vec::new(),
                false => vec![transient::addressed(input.context.and_then(|context| context.view_state.as_ref()).ok_or_else(import_video_frame_payload::import_window_required)?, next)?],
            };
            self.completed = true;
            return Ok(semio_framework_plugin::retained_command::ArtifactCommandWorkStep::CompleteWithEphemeral { emit, ephemeral: EphemeralEmit { window_transient, ..Default::default() } });
        }
        let mut emit = input.command.dispatch(&document, &config_view)?;
        if matches!(input.command, RemodelingCommand::SetCamera(_) | RemodelingCommand::SetLayerVisibility(_) | RemodelingCommand::SetFrameCursor(_) | RemodelingCommand::SetReportTable(_)) {
            let view = input.context.and_then(|context| context.view_state.as_ref()).ok_or_else(|| Fault::from("remodeling-window-view-required"))?;
            if let Some(mutation) = remodeling_window_config_mutation(input.command, window, view)? { emit.window_config_mutations.push(mutation); }
        }
        self.completed = true;
        Ok(semio_framework_plugin::retained_command::ArtifactCommandWorkStep::Complete(emit))
    
        })();
        original.map_err(semio_framework_job::OriginalTurnFailure::from)
    }
}

/// 🏭️ The app-owned retained command job factory — `factory_type:` in the proof block below binds this
/// exact Rust type to `EditorApp<RemodelingPlayApp>`, which is what turns a bare (and therefore
/// `interactive-job.missing-owned-reducer`) proof into an exact-owner proof.
pub(super) struct RemodelingRetainedCommandJobFactory {
    keys: Vec<ToolFactoryKey>,
}

impl RemodelingRetainedCommandJobFactory {
    pub(super) fn new(controller_id: &str) -> Self {
        Self { keys: REMODELING_RETAINED_TOOL_IDS.iter().map(|tool_id| ToolFactoryKey::new(controller_id, *tool_id)).collect() }
    }
}

impl semio_framework::ToolJobFactory for RemodelingRetainedCommandJobFactory {
    type Payload = ArtifactRetainedCommandPayload<EditorApp<RemodelingPlayApp>>;
    type Job = ArtifactRetainedCommandJob<EditorApp<RemodelingPlayApp>>;

    fn keys(&self) -> &[ToolFactoryKey] {
        &self.keys
    }

    fn payload_schema_id(&self) -> &str {
        REMODELING_RETAINED_PAYLOAD_SCHEMA
    }

    fn classification(&self) -> InteractiveJobClassification {
        InteractiveJobClassification::Migrated
    }

    fn execution_contract(&self) -> ToolExecutionContract {
        remodeling_retained_contract()
    }

    semio_framework_plugin::retained_command::retained_command_factory_admission!(maximum_raw_bytes: REMODELING_RETAINED_RAW_BYTES, checkpoint_resume: false);

}

impl semio_framework_plugin::ArtifactOwnedToolJobFactory for RemodelingRetainedCommandJobFactory {
    type Owner = EditorApp<RemodelingPlayApp>;
    const TOOL_IDS: &'static [&'static str] = REMODELING_RETAINED_TOOL_IDS;
    const DOCUMENT_SCHEMA: &'static str = REMODELING_DOCUMENT_SCHEMA;
    const PUBLICATION_CONTRACTS: &'static [semio_framework_plugin::ArtifactToolPublicationContract] = REMODELING_PUBLICATION_CONTRACTS;
}
//#endregion 🧵️RetainedCommands
//#endregion 🔖️RemodelingPlayApp
