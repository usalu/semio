//! 🧵️ Retained command routes: tool ids, publication contracts, the bounded extent and reducer, and the job factory.

use super::*;
use semio_framework_plugin::ArtifactOwnedToolJobFactory;
use semio_s_artifact_raster_raster_editor_commands::RasterCommand;
use crate::config::{RasterConfig, RasterConfigMutation};
use semio_s_artifact_raster_raster_mutations::RasterMutation;
use semio_s_artifact_raster_raster_model::RASTER_DOCUMENT_SCHEMA;
use semio_s_artifact_raster_raster_model::snapshot::RasterSnapshot;
use semio_framework::{InteractiveJobClassification, ToolExecutionContract, ToolFactoryKey, ToolJobFactoryError};
use semio_framework_plugin::retained_command::{ArtifactRetainedCommandJob, ArtifactRetainedCommandPayload};
use semio_framework_plugin::AppOperationContext;
use semio_framework_plugin::ArtifactToolPublicationContract;
use semio_framework_plugin::ArtifactToolPublicationLane;
use semio_framework_plugin::ArtifactView;
use semio_framework_plugin::ConfigView;
use semio_framework_plugin::EditorApp;
use semio_framework_plugin::Emit;
use semio_framework_plugin::Fault;
use semio_framework_plugin::NoDraftMutation;

//#region 🧵️RetainedCommands
/// 🧵️ Document and session command routes; PNG download has a separate resumable host-only factory.
pub const RASTER_RETAINED_TOOL_IDS: &[&str] = &[
    "addLayer",
    "dropLayerKind",
    "setLayerVisible",
    "toggleLayerVisible",
    "deleteLayer",
    "duplicateLayer",
    "patchLayer",
    "patchLayers",
    "moveLayer",
    "setBrushSize",
    "setBrushOpacity",
    "setCompositeViewport",
    "setCamera",
    "setCameraZoom",
    "setActiveExample",
    "flattenLayers",
    "mergeDown",
    "maskFromSelection",
    "setBrushColor",
    "setBrushHardness",
    "setPaintTarget",
    "setMaskValue",
    "setPixelSelection",
    "paintStroke",
    "fillRegion",
    "setFillTolerance",
    "applyFilter",
    "transformImage",
    "fillSelection",
];

pub(super) const RASTER_RETAINED_PAYLOAD_SCHEMA: &str = "raster.tool-command.v1";

pub(super) const RASTER_RETAINED_RAW_BYTES: usize = 65_536;

pub(super) const RASTER_RETAINED_WORK_ITEMS: usize = 4_096;

/// 🛣️ Publication lanes per route, read off each handler's own `Emit` in `🎮️commands/*/🦀️.rs` — the
/// document verbs build `Emit { artifact_mutations, .. }`/`Emit::mutations(..)` over
/// `RasterMutation` and never touch the config, while the session verbs build `Emit::config(..)`
/// over `RasterConfigMutation` and never touch the document. `paintStroke` alone also writes the window-transient lane:
/// a streamed stroke's tool state lives in the Composite window between its dispatches. No raster handler emits the
/// document and the config lane together, a draft or a presence.
///
/// 🎬️ `setActiveExample` is a document verb, not a session one: raster has no whole-document replace
/// mutation, so `🎮️commands/🎬️set-active-example` spells loading an example as an ordered batch of
/// real `RasterMutation`s (delete every root layer, re-point the asset pool, plant the example forest)
/// — the Artifact lane, exactly like every other layer verb.
///
/// `ActionKind::View` declarations in `🔖️Manifest` (ticket 26/07/31 — camera is runtime state, never a
/// document field, and there is no `RasterOperation::SetCamera`); `Config` is precisely the lane that
/// says "this route publishes into the config store, not into artifact history".
pub(super) const RASTER_PUBLICATION_CONTRACTS: &[ArtifactToolPublicationContract] = &[
    ArtifactToolPublicationContract { tool_id: "flattenLayers", lanes: &[ArtifactToolPublicationLane::Artifact] },
    ArtifactToolPublicationContract { tool_id: "mergeDown", lanes: &[ArtifactToolPublicationLane::Artifact] },
    ArtifactToolPublicationContract { tool_id: "paintStroke", lanes: &[ArtifactToolPublicationLane::Artifact, ArtifactToolPublicationLane::WindowTransient] },
    ArtifactToolPublicationContract { tool_id: "fillRegion", lanes: &[ArtifactToolPublicationLane::Artifact] },
    ArtifactToolPublicationContract { tool_id: "applyFilter", lanes: &[ArtifactToolPublicationLane::Artifact] },
    ArtifactToolPublicationContract { tool_id: "transformImage", lanes: &[ArtifactToolPublicationLane::Artifact] },
    ArtifactToolPublicationContract { tool_id: "fillSelection", lanes: &[ArtifactToolPublicationLane::Artifact] },
    ArtifactToolPublicationContract { tool_id: "maskFromSelection", lanes: &[ArtifactToolPublicationLane::Artifact] },
    ArtifactToolPublicationContract { tool_id: "addLayer", lanes: &[ArtifactToolPublicationLane::Artifact] },
    ArtifactToolPublicationContract { tool_id: "dropLayerKind", lanes: &[ArtifactToolPublicationLane::Artifact] },
    ArtifactToolPublicationContract { tool_id: "setLayerVisible", lanes: &[ArtifactToolPublicationLane::Artifact] },
    ArtifactToolPublicationContract { tool_id: "toggleLayerVisible", lanes: &[ArtifactToolPublicationLane::Artifact] },
    ArtifactToolPublicationContract { tool_id: "deleteLayer", lanes: &[ArtifactToolPublicationLane::Artifact] },
    ArtifactToolPublicationContract { tool_id: "duplicateLayer", lanes: &[ArtifactToolPublicationLane::Artifact] },
    ArtifactToolPublicationContract { tool_id: "patchLayer", lanes: &[ArtifactToolPublicationLane::Artifact] },
    ArtifactToolPublicationContract { tool_id: "patchLayers", lanes: &[ArtifactToolPublicationLane::Artifact] },
    ArtifactToolPublicationContract { tool_id: "moveLayer", lanes: &[ArtifactToolPublicationLane::Artifact] },
    ArtifactToolPublicationContract { tool_id: "setBrushSize", lanes: &[ArtifactToolPublicationLane::Config] },
    ArtifactToolPublicationContract { tool_id: "setBrushOpacity", lanes: &[ArtifactToolPublicationLane::Config] },
    ArtifactToolPublicationContract { tool_id: "setBrushColor", lanes: &[ArtifactToolPublicationLane::Config] },
    ArtifactToolPublicationContract { tool_id: "setBrushHardness", lanes: &[ArtifactToolPublicationLane::Config] },
    ArtifactToolPublicationContract {tool_id:"setPaintTarget",lanes:&[ArtifactToolPublicationLane::Config]},
    ArtifactToolPublicationContract {tool_id:"setMaskValue",lanes:&[ArtifactToolPublicationLane::Config]},
    ArtifactToolPublicationContract { tool_id: "setFillTolerance", lanes: &[ArtifactToolPublicationLane::Config] },
    ArtifactToolPublicationContract {tool_id:"setPixelSelection",lanes:&[ArtifactToolPublicationLane::Config]},
    ArtifactToolPublicationContract { tool_id: "setCompositeViewport", lanes: &[ArtifactToolPublicationLane::Config] },
    ArtifactToolPublicationContract { tool_id: "setCamera", lanes: &[ArtifactToolPublicationLane::Config] },
    ArtifactToolPublicationContract { tool_id: "setCameraZoom", lanes: &[ArtifactToolPublicationLane::Config] },
    ArtifactToolPublicationContract { tool_id: "setActiveExample", lanes: &[ArtifactToolPublicationLane::Artifact] },
];

pub fn raster_retained_contract() -> ToolExecutionContract {
    ToolExecutionContract::bounded_first_step(RASTER_RETAINED_RAW_BYTES, 4_096, 1, 262_144, 7_500)
}

/// 📏️ One bounded first step per retained route, admitted only while the WHOLE layer tree (groups and
/// their nested children, via `flatten_raster_layers`) plus the asset pool plus this edit fit inside
/// `RASTER_RETAINED_WORK_ITEMS`. `layers.len()` alone would understate a grouped document, so the
/// recursive walk is used rather than the top-level count.
pub(super) fn raster_retained_extent(command: &RasterCommand, snapshot: &RasterSnapshot, _interaction: &protocol::InteractionState) -> Option<usize> {
    if !RASTER_RETAINED_TOOL_IDS.contains(&command.command_id()) {
        return None;
    }
    let items = semio_s_artifact_raster_raster_model::flatten_raster_layers(&snapshot.layers).len().checked_add(snapshot.assets.len())?.checked_add(1)?;
    (items <= RASTER_RETAINED_WORK_ITEMS).then_some(1)
}

#[expect(clippy::too_many_arguments, reason = "Implements the framework ArtifactCommandReducer callback signature.")]
pub(super) fn raster_retained_reduce(
    command: &RasterCommand,
    snapshot: &RasterSnapshot,
    config: &RasterConfig,
    history: &semio_framework_plugin::HistoryView,
    _interaction: &protocol::InteractionState,
    _hover: &semio_framework_plugin::app::InteractionHoverState,
    _context: Option<&semio_framework_plugin::app::ArtifactOwnedToolJobContext<EditorApp<RasterPlayApp>>>,
    operation: &AppOperationContext,
) -> Result<Emit<RasterMutation, RasterConfigMutation, NoDraftMutation>, Fault> {
    command.dispatch(&ArtifactView::with_operation(snapshot, history, operation.clone()), &ConfigView { snapshot: config, window: None })
}

/// 🏭️ The app-owned retained command job factory — `factory_type:` in the proof block below binds this
/// exact Rust type to `EditorApp<RasterPlayApp>`, which is what turns a bare (and therefore
/// `interactive-job.missing-owned-reducer`) proof into an exact-owner proof.
pub struct RasterRetainedCommandJobFactory {
    pub(super) keys: Vec<ToolFactoryKey>,
}

impl RasterRetainedCommandJobFactory {
    pub(super) fn new(controller_id: &str) -> Self {
        Self { keys: RASTER_RETAINED_TOOL_IDS.iter().map(|tool_id| ToolFactoryKey::new(controller_id, *tool_id)).collect() }
    }
}

impl semio_framework::ToolJobFactory for RasterRetainedCommandJobFactory {
    type Payload = ArtifactRetainedCommandPayload<EditorApp<RasterPlayApp>>;
    type Job = ArtifactRetainedCommandJob<EditorApp<RasterPlayApp>>;

    fn keys(&self) -> &[ToolFactoryKey] {
        &self.keys
    }
    fn payload_schema_id(&self) -> &str {
        RASTER_RETAINED_PAYLOAD_SCHEMA
    }
    fn classification(&self) -> InteractiveJobClassification {
        InteractiveJobClassification::Migrated
    }
    fn execution_contract(&self) -> ToolExecutionContract {
        raster_retained_contract()
    }
    semio_framework_plugin::retained_command::retained_command_factory_admission!(maximum_raw_bytes: RASTER_RETAINED_RAW_BYTES, checkpoint_resume: false);

}

impl ArtifactOwnedToolJobFactory for RasterRetainedCommandJobFactory {
    type Owner = EditorApp<RasterPlayApp>;
    const TOOL_IDS: &'static [&'static str] = RASTER_RETAINED_TOOL_IDS;
    const DOCUMENT_SCHEMA: &'static str = RASTER_DOCUMENT_SCHEMA;
    const PUBLICATION_CONTRACTS: &'static [ArtifactToolPublicationContract] = RASTER_PUBLICATION_CONTRACTS;
}
