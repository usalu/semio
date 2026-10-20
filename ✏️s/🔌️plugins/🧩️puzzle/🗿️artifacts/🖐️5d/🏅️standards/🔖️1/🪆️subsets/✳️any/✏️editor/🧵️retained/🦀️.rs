//! 🧵️ Retained work of the puzzle 5d editor: the window tool ids, the retained payload schema, extent and reduce helpers, the window command work and the export work. See [`super`].

use crate::standards::v1::subsets::any::schema::play_snapshot::Puzzle5dPlaySnapshot;
use crate::standards::v1::subsets::any::schema::mutations::Puzzle5dMutation;
use crate::editor::puzzle5d::commands::export_snapshot;
use crate::{editor::puzzle5d::config::Puzzle5dConfig, editor::puzzle5d::config::Puzzle5dConfigMutation};
use crate::editor::puzzle5d::window as window_ownership;
use semio_framework_plugin::kernel::UiDirtyScope;
use semio_framework_plugin::EditorApp;
use semio_framework_plugin::Emit;
use semio_framework_plugin::Fault;
use semio_framework_job::Operation;
use semio_framework_pack_json::Value;
use crate::editor::puzzle5d::dispatch::puzzle5d_handle_action;
use crate::editor::puzzle5d::interaction::Puzzle5dEditor;
use crate::editor::puzzle5d::{interaction, view};
use crate::editor::puzzle5d::entities::{PUZZLE5D_INTERACTION_DOMAIN, Puzzle5dDocument, puzzle5d_document_from_snapshot, puzzle5d_editor_projection};
use semio_framework_job::StepContext;
use crate::editor::puzzle5d::entities::{PUZZLE5D_WORLD_WINDOW_KIND_ID};
use crate::editor::puzzle5d::interaction::{Puzzle5dCommand, puzzle5d_notice_emit};

//#region 🔖️PlayApp
//#region 🧵️RetainedCommands
/// 🪟️ Every tool id whose whole semantic work IS one `handle_action_impl` turn against the addressed
/// window's runtime — routed through [`Puzzle5dWindowCommandWork`]. Camera, grid, LOD, sun and the
/// suggestion offset publish the addressed pane's `WindowConfig`; the contact tolerance publishes the
/// shared app `Config` through the same one-turn route (puzzle 3d joins the identical set to its own
/// `Puzzle3dWindowCommandWork` arm).
pub const PUZZLE5D_WINDOW_TOOL_IDS: &[&str] = &[
    "addTargetVolume",
    "setVoxelDims",
    "cycleBrushCandidate",
    "cycleBrushCandidateBack",
    "engagementAbort",
    "engagementControlSelect",
    "engagementInput",
    "engagementRepeatLast",
    "engagementSubmit",
    "setFillCount",
    "targetBrushSuggestions",
    "openVortexSuggestions",
    "closeVortexSuggestions",
    "hoverSuggestion",
    "acceptSuggestion",
    "setCamera",
    "setCamera2d",
    "setCamera3d",
    "setGridFactor",
    "setGridSnapEnabled",
    "setLodMode",
    "setSuggestionOffset",
    "toggleSun",
    "setSunAzimuth",
    "setSunElevation",
    "setSunIntensity",
    "setBrushPlacementContactTolerance",
    "setProximityRadius",
    "setChunkSize",
    "setGridVisible",
    "setGridSpacing",
    "setProjection",
    "setProjectionParam",
    "setGripShow",
    "setGripDirection",
    "setSelectableKind",
    "setLodAutomatic",
    "setLodDepthVariable",
    "setLodManual",
    "setTransformGumballFlag",
];
pub const PUZZLE5D_RETAINED_PAYLOAD_SCHEMA: &str = "puzzle.5d.tool-command.v1";

pub fn puzzle5d_retained_extent(_command: &Puzzle5dCommand, snapshot: &Puzzle5dPlaySnapshot, interaction: &protocol::InteractionState) -> Option<usize> {
    let projection = puzzle5d_editor_projection(snapshot);
    let selection = interaction.selection.get(PUZZLE5D_INTERACTION_DOMAIN).map_or(0, |selection| selection.ids.len());
    let parts = projection.get("parts").and_then(Value::as_array).map_or(0, Vec::len);
    let fasteners = projection.get("fasteners").and_then(Value::as_array).map_or(0, Vec::len);
    let target_volumes = projection.get("targetVolumes").and_then(Value::as_array).map_or(0, Vec::len);
    let items = selection.checked_add(parts)?.checked_add(fasteners)?.checked_add(target_volumes)?;
    (items <= crate::retained_command::PUZZLE_COMMAND_WORK_ITEMS).then_some(items)
}

pub fn puzzle5d_retained_reduce<A: Puzzle5dEditor>(
    command: &Puzzle5dCommand,
    snapshot: &Puzzle5dPlaySnapshot,
    config: &Puzzle5dConfig,
    interaction: &protocol::InteractionState,
    _hover: &semio_framework_plugin::app::InteractionHoverState,
    view_state: Option<&semio_framework_plugin::ViewModel>,
) -> Result<Emit<Puzzle5dMutation, Puzzle5dConfigMutation>, Fault> {
    let empty_selection = protocol::DomainSelection::default();
    let selection = interaction.selection.get(PUZZLE5D_INTERACTION_DOMAIN).unwrap_or(&empty_selection);
    let window_id = view_state.and_then(|view| view.window_id.as_deref()).or_else(|| command.window_id()).unwrap_or(crate::editor::puzzle5d::entities::PUZZLE5D_WORLD_WINDOW_KIND_ID);
    let runtime = window_ownership::runtime(config, &window_ownership::Puzzle5dWindowConfig::default(), &window_ownership::Puzzle5dWindowTransient::default(), window_id);
    Ok(puzzle5d_handle_action::<A>("", command.action_id(), command.args(), command.window_id(), snapshot, &runtime, view_state, selection, None, None).0)
}

/// 🎲️ The nonce one operation's minted part and fastener ids derive from.
pub fn puzzle5d_operation_nonce(operation: &Operation) -> u64 {
    operation.operation.0 ^ operation.generation.0.rotate_left(17) ^ operation.seed.rotate_left(31)
}

/// ♻️ The owners one `Puzzle5dWindowCommandWork` still holds when its job closes, retired as one controlled bundle.
#[derive(semio_framework_value::RetireOwned)]
struct Puzzle5dWindowWorkOwners {
    pub instance_owner: Option<semio_framework_plugin::ArtifactInstanceOperationOwnerHandle>,
    pub authoring_seed: String,
}

pub struct Puzzle5dWindowCommandWork {
    pub tool_id: &'static str,
    consumed: bool,
    pub closing: bool,
    pub close_owners: crate::puzzle_job::WorkClosing<Puzzle5dWindowWorkOwners>,
    pub instance_owner: Option<semio_framework_plugin::ArtifactInstanceOperationOwnerHandle>,
    /// 🌱️ The admission's authoring seed a typed `move`/`rotate`/`scale` submit mints its tool transaction from.
    pub authoring_seed: String,
}

impl Puzzle5dWindowCommandWork {
    pub fn new(tool_id: &'static str, authoring_seed: String) -> Self {
        Self { tool_id, consumed: false, closing: false, close_owners: Default::default(), instance_owner: None, authoring_seed }
    }

    /// 🪪️ Binds the admission's retained operation owner.
    pub fn bound(mut self, owner: semio_framework_plugin::ArtifactInstanceOperationOwnerHandle) -> Self {
        self.instance_owner = Some(owner);
        self
    }
}

impl<A: Puzzle5dEditor> semio_framework_plugin::retained_command::ArtifactCommandWork<EditorApp<A>> for Puzzle5dWindowCommandWork {
    fn tool_id(&self) -> &'static str { self.tool_id }
    fn extent(&self, _command: &Puzzle5dCommand, _snapshot: &Puzzle5dPlaySnapshot, _interaction: &protocol::InteractionState, _context: Option<&semio_framework_plugin::app::ArtifactOwnedToolJobContext<EditorApp<A>>>) -> Option<usize> { Some(1) }
    fn work_demands(&self, _input: &semio_framework_plugin::retained_command::ArtifactCommandInputs<'_, EditorApp<A>>, _maximum_copy_bytes: usize) -> Result<semio_framework_value::RetirementDemand, semio_framework_value::ValueError> {
        crate::retained_command::step_demands::<EditorApp<A>>(size_of::<Self>())
    }

    fn step(&mut self, input: &semio_framework_plugin::retained_command::ArtifactCommandInputs<'_, EditorApp<A>>, _cx: &mut StepContext<'_>) -> Result<semio_framework_plugin::retained_command::ArtifactCommandWorkStep<EditorApp<A>>, semio_framework_job::OriginalTurnFailure> {
        let original: Result<_, Fault> = (|| {
        let semio_framework_plugin::retained_command::ArtifactCommandInputs { command, snapshot, config, interaction, hover: _hover, context, .. } = *input;
        let view_state = context.and_then(|context| context.view_state.as_ref());
        let window_config_snapshot = context.and_then(|context| context.window_config.as_ref());
        let window_transient_snapshot = context.and_then(|context| context.window_transient.as_ref());
        if self.consumed { return Err(Fault::from("puzzle5d-window-work-repeated")); }
        let view = view_state.ok_or_else(|| Fault::from("puzzle5d-window-context-required"))?;
        let window_id = view.window_id.as_deref().or_else(|| command.window_id()).ok_or_else(|| Fault::from("puzzle5d-window-id-required"))?;
        let window_config = window_ownership::config_from_snapshot(window_config_snapshot);
        let window_transient = window_ownership::transient_from_snapshot(window_transient_snapshot);
        let runtime = window_ownership::runtime(config, &window_config, &window_transient, window_id);
        let empty_selection = protocol::DomainSelection::default();
        let selection = interaction.selection.get(PUZZLE5D_INTERACTION_DOMAIN).unwrap_or(&empty_selection);
        let (emit, ephemeral) = puzzle5d_handle_action::<A>(&self.authoring_seed, command.action_id(), command.args(), Some(window_id), snapshot, &runtime, Some(view), selection, self.instance_owner.as_ref(), if matches!(self.tool_id, "engagementAbort" | "openVortexSuggestions" | "closeVortexSuggestions") { context.and_then(|context| context.tool_run()) } else { None });
        self.consumed = true;
        Ok(semio_framework_plugin::retained_command::ArtifactCommandWorkStep::CompleteWithEphemeral { emit, ephemeral })
    
        })();
        original.map_err(semio_framework_job::OriginalTurnFailure::from)
    }

    fn begin_close(&mut self) {
        self.closing = true;
        self.close_owners.stage(Puzzle5dWindowWorkOwners { instance_owner: self.instance_owner.take(), authoring_seed: std::mem::take(&mut self.authoring_seed) });
    }

    fn close_step(&mut self, cx: &mut StepContext<'_>) -> Result<semio_framework_job::InteractiveJobCloseStep, semio_framework_job::OriginalTurnFailure> { semio_framework_job::close_original_nested_step(cx, |cx, _grant| {
        self.close_owners.close_step(cx)
    }) }

    fn next_close_copy_byte_demand(&self) -> Result<usize, semio_framework_value::ValueError> {
        Ok(self.close_owners.demands(0)?.copy_bytes)
    }

    fn next_close_capacity_byte_demand(&self, maximum_copy_bytes: usize) -> Result<usize, semio_framework_value::ValueError> {
        Ok(self.close_owners.demands(maximum_copy_bytes)?.capacity_bytes)
    }

    fn next_close_release_byte_demand(&self) -> Result<usize, semio_framework_value::ValueError> {
        Ok(self.close_owners.demands(0)?.release_bytes)
    }

    fn next_close_depth_demand(&self) -> Result<usize, semio_framework_value::ValueError> {
        Ok(self.close_owners.demands(0)?.depth)
    }

    fn terminal_is_empty(&self) -> bool {
        self.closing && self.close_owners.is_empty()
    }

    fn terminal_frame_release_bytes(&self) -> Option<usize> {
        Some(size_of::<Self>())
    }
}

/// 📤 `exportSnapshot` reads the document and publishes a download; it owns no mutation, so it resolves
/// straight from the snapshot and picks its lane by payload size: one inline effect under the guest's
/// contiguous request ceiling, the framework's segmented-download lane above it, a localized notice
/// above what one segmented download may carry. It is NOT a `dispatch_puzzle5d_action` arm because a
/// segmented download is a `PuzzleCommandWorkStep`, not an `Emit` — `◻️2d`'s `Puzzle2dExportWork` is
/// the same shape.
#[derive(Default)]
pub struct Puzzle5dExportWork {
    consumed: bool,
}

impl<A: Puzzle5dEditor> semio_framework_plugin::retained_command::ArtifactCommandWork<EditorApp<A>> for Puzzle5dExportWork {
    fn tool_id(&self) -> &'static str {
        "exportSnapshot"
    }

    fn extent(&self, _command: &Puzzle5dCommand, _snapshot: &Puzzle5dPlaySnapshot, _interaction: &protocol::InteractionState, _context: Option<&semio_framework_plugin::app::ArtifactOwnedToolJobContext<EditorApp<A>>>) -> Option<usize> {
        Some(1)
    }

    fn work_demands(&self, _input: &semio_framework_plugin::retained_command::ArtifactCommandInputs<'_, EditorApp<A>>, _maximum_copy_bytes: usize) -> Result<semio_framework_value::RetirementDemand, semio_framework_value::ValueError> {
        crate::retained_command::step_demands::<EditorApp<A>>(size_of::<Self>())
    }

    fn step(&mut self, input: &semio_framework_plugin::retained_command::ArtifactCommandInputs<'_, EditorApp<A>>, _cx: &mut StepContext<'_>) -> Result<semio_framework_plugin::retained_command::ArtifactCommandWorkStep<EditorApp<A>>, semio_framework_job::OriginalTurnFailure> {
        let original = (|| -> Result<semio_framework_plugin::retained_command::ArtifactCommandWorkStep<EditorApp<A>>, semio_framework_job::OriginalTurnFailure> {

        let original: Result<_, Fault> = (|| {
        let semio_framework_plugin::retained_command::ArtifactCommandInputs { command: _command, snapshot, config: _config, interaction: _interaction, hover: _hover, context, .. } = *input;
        let view_state = context.and_then(|context| context.view_state.as_ref());
        if self.consumed {
            return Err(Fault::from("puzzle5d-export-work-repeated"));
        }
        self.consumed = true;
        let document: Puzzle5dDocument = puzzle5d_document_from_snapshot(snapshot.typed()).map_err(|_| Fault::from("puzzle5d-export-document-malformed"))?;
        Ok(match export_snapshot::puzzle5d_export_publication(&document)? {
            export_snapshot::Puzzle5dExportPublication::Inline(effect) => {
                semio_framework_plugin::retained_command::ArtifactCommandWorkStep::Complete(Emit { effects: vec![effect], ui_scope: UiDirtyScope::None, ..Default::default() })
            }
            export_snapshot::Puzzle5dExportPublication::Segmented(download) => semio_framework_plugin::retained_command::ArtifactCommandWorkStep::CompleteDownload { download: download, ephemeral: Default::default() },
            export_snapshot::Puzzle5dExportPublication::Refused(_) => semio_framework_plugin::retained_command::ArtifactCommandWorkStep::Complete(puzzle5d_notice_emit(view_state, |labels| labels.export_too_large.as_str())),
        })
        })();
        original.map_err(semio_framework_job::OriginalTurnFailure::from)
    
        })();
        original.map_err(semio_framework_job::OriginalTurnFailure::from)
    }

    fn terminal_frame_release_bytes(&self) -> Option<usize> {
        Some(size_of::<Self>())
    }
}
//#endregion 🧵️RetainedCommands
//#endregion 🔖️PlayApp
