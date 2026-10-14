//! 🎬️ Puzzle 3d retained action works — the one-action-per-window route through the shared action prologue, and the
//! staged placement works (transform, brush placement, suggestion acceptance, focus, precompute commands) that each spend
//! bounded turns. Every work is written against the [`Puzzle3dEditor`] contract, not the concrete editor.

#![allow(clippy::result_large_err)]

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_os_kernel as vcs;
extern crate semio_framework_value_derive as value_derive;

use {semio_s_artifact_puzzle_3d_editor_dispatch::puzzle3d_handle_action, semio_s_artifact_puzzle_3d_editor_dispatch::Puzzle3dActionPrologue, semio_s_artifact_puzzle_3d_editor_dispatch::puzzle3d_shell_only_emit};
use {semio_s_artifact_puzzle_3d_editor_session::Puzzle3dInstanceOperationOwner, semio_s_artifact_puzzle_3d_editor_session::with_puzzle3d_session_for};
use {semio_s_artifact_puzzle_3d_editor_snapshot::PUZZLE3D_GRANULARITY_OBJECT, semio_s_artifact_puzzle_3d_editor_snapshot::PUZZLE3D_GRANULARITY_TARGET_VOLUME, semio_s_artifact_puzzle_3d_editor_snapshot::PUZZLE3D_GRANULARITY_VORTEX, semio_s_artifact_puzzle_3d_editor_snapshot::PUZZLE3D_INTERACTION_DOMAIN, semio_s_artifact_puzzle_3d_editor_snapshot::PUZZLE3D_LOCALIZATION_UNSUPPORTED, semio_s_artifact_puzzle_3d_editor_snapshot::Puzzle3dInteractionSnapshot, semio_s_artifact_puzzle_3d_editor_snapshot::Puzzle3dObject, semio_s_artifact_puzzle_3d_editor_snapshot::Puzzle3dVortex, semio_s_artifact_puzzle_3d_editor_snapshot::puzzle3d_addressed_window_id, semio_s_artifact_puzzle_3d_editor_dispatch::puzzle3d_command_scope_class, semio_s_artifact_puzzle_3d_editor_snapshot::puzzle3d_inspector_origin_nudge, semio_s_artifact_puzzle_3d_editor_snapshot::puzzle3d_next_object_label, semio_s_artifact_puzzle_3d_editor_snapshot::puzzle3d_scene_snapshot_from_document, semio_s_artifact_puzzle_3d_editor_dispatch::puzzle3d_scope, semio_s_artifact_puzzle_3d_editor_snapshot::value_as_vec3};
use {semio_s_artifact_puzzle_3d_commands_document::add_target_volume, semio_s_artifact_puzzle_3d_commands_document::export_snapshot, semio_s_artifact_puzzle_3d_commands_runtime::set_fill_count};
use {semio_s_artifact_puzzle_3d_editor_config::Puzzle3dConfig, semio_s_artifact_puzzle_3d_editor_config::Puzzle3dConfigMutation, semio_s_artifact_puzzle_3d_editor_config::Puzzle3dConfigSetFillCount};
use semio_s_artifact_puzzle_3d_editor_panels::artifact;
use {semio_s_artifact_puzzle_3d_editor_terminology::puzzle3d_labels, semio_s_artifact_puzzle_3d_editor_terminology::Puzzle3dLabels};
use semio_s_artifact_puzzle_3d_editor_window as window_ownership;
use semio_s_artifact_puzzle_3d_editor_snapshot::Puzzle3dPlaySnapshot;
use semio_s_artifact_puzzle_3d_mutations::Puzzle3dMutation;
use semio_s_artifact_puzzle_3d_model::snapshot::Puzzle3dSnapshot;
use semio_framework::kernel::UiDirtyScope;
use semio_framework_plugin::kernel::Effect;
use {semio_framework_job::InteractiveJobCloseStep, semio_framework_job::StepContext};
use semio_framework_plugin::ArtifactInstanceOperationOwnerHandle;
use semio_framework_plugin::EditorApp;
use semio_framework_plugin::Emit;
use semio_framework_plugin::Fault;
use semio_framework_plugin::InteractionWrite;
use semio_framework_value::FromValue;
use {semio_framework_pack_json::object, semio_framework_pack_json::to_string, semio_framework_pack_json::Value};
use std::collections::HashSet;
use semio_s_artifact_puzzle_3d_model::puzzle3d_vortex_full_id;
use {semio_s_artifact_puzzle_3d_retained_document::PUZZLE3D_RELOCATE_VORTICES_PER_OBJECT, semio_s_artifact_puzzle_3d_editor_session as session};

/// 🧯️ ONE bounded, localized notice as a terminal `Emit` — how a retained placement work whose gesture
/// produced nothing tells the user WHY, instead of the bare `Emit::default()` every refusal path used
/// to complete with (`📓️2026-09-09-user-feature-checklist.md` §9/§13: "a real placement error produces
/// no visible feedback even though the menu correctly closes"). The message is resolved against the
/// host's declared locale×terminology axes; an axis this app never authored carries
/// [`PUZZLE3D_LOCALIZATION_UNSUPPORTED`] rather than an English sentence, because this UI has no
/// default language and a silent drop would restore the very defect being fixed.
///
/// 🐢️ The scope is [`UiDirtyScope::None`], not `Emit::effect`'s `Default` (which is `Full`): a work
/// that refused published nothing, so there is nothing to repaint — the notice itself travels on the
/// effect lane, which the host applies before it ever consults the scope.
fn puzzle3d_notice_emit(view_state: Option<&semio_framework_plugin::ViewModel>, message: impl Fn(&Puzzle3dLabels) -> &'static str) -> Emit<Puzzle3dMutation, Puzzle3dConfigMutation> {
    let text = view_state.and_then(puzzle3d_labels).map_or_else(|| PUZZLE3D_LOCALIZATION_UNSUPPORTED.to_string(), |labels| message(labels).to_string());
    Emit { effects: vec![Effect::Notify { message: text }], ui_scope: UiDirtyScope::None, ..Default::default() }
}

pub const PUZZLE3D_RETAINED_PAYLOAD_SCHEMA: &str = "puzzle.3d.tool-command.v1";

pub fn puzzle3d_retained_extent(command: &Puzzle3dCommand, snapshot: &Puzzle3dPlaySnapshot, interaction: &protocol::InteractionState) -> Option<usize> {
    if matches!(command.action_id(), "addTargetVolume" | "openAddObjectDialog" | "worldPointerDown") {
        return Some(1);
    }
    let selection = interaction.selection.get(PUZZLE3D_INTERACTION_DOMAIN).map_or(0, |selection| selection.ids.len());
    let artifact = snapshot.typed();
    let artifact_items = match command.action_id() {
        "focusSelection" | "patchInspector" | "translateSelection" | "rotateSelection" | "scaleSelection" => artifact.objects.len().checked_add(artifact.target_volumes.len())?,
        "createAttraction" | "worldRelocate" => artifact.objects.len().checked_add(artifact.attractions.len())?,
        "addObjectKind" | "setObjectKindWeight" | "setVortexKindWeight" => artifact.meta.kind_catalogs.as_ref().map_or(0, |catalogs| catalogs.objects.len().saturating_add(catalogs.vortices.len())),
        _ => 1,
    };
    selection.checked_add(artifact_items).filter(|items| *items <= semio_s_plugin_puzzle_retained::PUZZLE_COMMAND_WORK_ITEMS)
}

/// 🧵️ Session-less entry point — the shape `PuzzleCommandReducer` demands of a plain fn pointer, for
/// the retained actions that only rewrite the document and gain nothing from a warm cache.
#[expect(clippy::unnecessary_wraps, reason = "Implements the fallible PuzzleCommandReducer callback signature.")]
pub fn puzzle3d_retained_reduce<A: Puzzle3dEditor>(
    command: &Puzzle3dCommand,
    snapshot: &Puzzle3dPlaySnapshot,
    config: &Puzzle3dConfig,
    interaction: &protocol::InteractionState,
    hover: &semio_framework_plugin::app::InteractionHoverState,
    view_state: Option<&semio_framework_plugin::ViewModel>,
) -> Result<Emit<Puzzle3dMutation, Puzzle3dConfigMutation>, Fault> {
    let runtime = window_ownership::runtime(config, &window_ownership::Puzzle3dWindowConfig::default(), &window_ownership::Puzzle3dWindowTransient::default(), view_state);
    if command.action_id() == "openAddObjectDialog" {
        return Ok(Emit::effect(Effect::OpenDialog { req: semio_framework_plugin::RequestId(120), dialog_id: "addObject".into(), args: None }));
    }
    if command.action_id() == "worldPointerDown" {
        return Ok(Emit::default());
    }
    if command.action_id() == "addTargetVolume" {
        let Some(origin) = command.args().and_then(|args| args.get("origin")).and_then(value_as_vec3) else { return Ok(Emit::default()) };
        let mutation = add_target_volume::add_target_volume_mutation(origin, runtime.grid_spacing, runtime.voxel_dims);
        return Ok(Emit { artifact_mutations: vec![mutation], ui_scope: puzzle3d_scope(puzzle3d_command_scope_class("addTargetVolume")), ..Default::default() });
    }
    let snapshot_interaction = Puzzle3dInteractionSnapshot::from_state(interaction, hover);
    let window_id = puzzle3d_addressed_window_id(view_state, None, command.window_id(), &runtime.window_ids);
    Ok(with_puzzle3d_session_for(None, &runtime, |session| puzzle3d_handle_action::<A>(session, "", command, Some(window_id), snapshot, &runtime, view_state, &snapshot_interaction).0))
}

/// 🧾️ The shared action prologue's three halves as this work's own stages.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Puzzle3dWindowCommandStage {
    Scene,
    Sync,
    Dispatch,
    Complete,
    Closing,
}

/// 🪟️ The one-action-per-window route: every tool id whose whole semantic work IS
/// [`Puzzle3dActionPrologue`] — no document scan of its own, no extra cursor. It used to run that
/// prologue whole in ONE turn, which on the 180-object Nakagin document measured 17.6 ms, twice the
/// framework's interactive step ceiling (ticket 26/09/02/PUZZLE-3D-END-TO-END W-P3); it now spends one
/// bounded turn on the scene, one per owed mesh fallback, and one on the dispatch.
/// 🪪️ The admission identity every instance-bound puzzle3d work takes at construction.
pub struct Puzzle3dInstanceBinding {
    app_instance_id: u32,
    parent_document_id: String,
    owner: ArtifactInstanceOperationOwnerHandle,
}

/// ♻️ The owners one `Puzzle3dWindowCommandWork` still holds when its job closes, retired as one controlled bundle.
#[derive(semio_framework_value::RetireOwned)]
struct Puzzle3dWindowWorkOwners {
    prologue: Puzzle3dActionPrologue,
    session: Option<(u32, Option<String>)>,
    instance_owner: Option<ArtifactInstanceOperationOwnerHandle>,
}

pub struct Puzzle3dWindowCommandWork {
    tool_id: &'static str,
    stage: Puzzle3dWindowCommandStage,
    turns: usize,
    prologue: Puzzle3dActionPrologue,
    session: Option<(u32, Option<String>)>,
    instance_owner: Option<ArtifactInstanceOperationOwnerHandle>,
    close_owners: semio_s_plugin_puzzle_jobs::WorkClosing<Puzzle3dWindowWorkOwners>,
}

impl Puzzle3dWindowCommandWork {
    fn new(tool_id: &'static str) -> Self {
        Self { tool_id, stage: Puzzle3dWindowCommandStage::Scene, turns: 0, prologue: Puzzle3dActionPrologue::default(), session: None, instance_owner: None, close_owners: Default::default() }
    }

    /// ⏯️ Binds the instance's tool run as of admission, for the arms that dispatch run actions.
    fn with_tool_run(mut self, tool_run: Option<semio_framework_plugin::ToolRunView>) -> Self {
        self.prologue.tool_run = tool_run;
        self
    }

    /// 🪪️ Binds the admission's instance identity and retained operation owner.
    fn bound(mut self, instance: &Puzzle3dInstanceBinding) -> Self {
        self.session = Some((instance.app_instance_id, Some(instance.parent_document_id.clone())));
        self.instance_owner = Some(instance.owner.clone());
        self
    }

}

impl<A: Puzzle3dEditor> semio_framework_plugin::retained_command::ArtifactCommandWork<EditorApp<A>> for Puzzle3dWindowCommandWork {
    fn tool_id(&self) -> &'static str {
        self.tool_id
    }
    fn extent(&self, _command: &Puzzle3dCommand, _snapshot: &Puzzle3dPlaySnapshot, _interaction: &protocol::InteractionState, _context: Option<&semio_framework_plugin::app::ArtifactOwnedToolJobContext<EditorApp<A>>>) -> Option<usize> {
        Some(1)
    }
    fn work_demands(&self, _input: &semio_framework_plugin::retained_command::ArtifactCommandInputs<'_, EditorApp<A>>, _maximum_copy_bytes: usize) -> Result<semio_framework_value::RetirementDemand, semio_framework_value::ValueError> {
        semio_s_plugin_puzzle_retained::step_demands::<EditorApp<A>>(size_of::<Self>())
    }

    fn step(&mut self, input: &semio_framework_plugin::retained_command::ArtifactCommandInputs<'_, EditorApp<A>>, _cx: &mut StepContext<'_>) -> Result<semio_framework_plugin::retained_command::ArtifactCommandWorkStep<EditorApp<A>>, Fault> {
        let semio_framework_plugin::retained_command::ArtifactCommandInputs { command, snapshot, config, interaction, hover: hover, context, .. } = *input;
        let view_state = context.and_then(|context| context.view_state.as_ref());
        let window_config_snapshot = context.and_then(|context| context.window_config.as_ref());
        let window_transient_snapshot = context.and_then(|context| context.window_transient.as_ref());
        if self.turns >= Puzzle3dActionPrologue::WORK_ITEMS {
            return Err(Fault::from("puzzle3d-window-work-capacity"));
        }
        self.turns += 1;
        let resolved_window_id = {
            let view = view_state.ok_or_else(|| Fault::from("puzzle3d-window-context-required"))?;
            let roster: Vec<String> = view.window_instances.iter().map(|instance| instance.id.clone()).collect();
            puzzle3d_addressed_window_id(Some(view), None, command.window_id(), &roster).to_string()
        };
        let addressed_view = view_state.filter(|view| view.window_id.as_deref() != Some(resolved_window_id.as_str())).map(|view| {
            view.for_window_instance(&resolved_window_id).unwrap_or_else(|| {
                let mut next = view.clone();
                next.window_id = Some(resolved_window_id.clone());
                next
            })
        });
        let view = addressed_view.as_ref().or(view_state).ok_or_else(|| Fault::from("puzzle3d-window-context-required"))?;
        let window_id = resolved_window_id.as_str();
        let window_config = window_ownership::config_from_snapshot(window_config_snapshot);
        let window_transient = window_ownership::transient_from_snapshot(window_transient_snapshot);
        let runtime = window_ownership::runtime(config, &window_config, &window_transient, Some(view));
        match self.stage {
            Puzzle3dWindowCommandStage::Scene => {
                if let Some(shell) = puzzle3d_shell_only_emit(command.action_id()) {
                    self.stage = Puzzle3dWindowCommandStage::Complete;
                    return Ok(semio_framework_plugin::retained_command::ArtifactCommandWorkStep::CompleteWithEphemeral { emit: shell.0, ephemeral: shell.1 });
                }
                // 📤️ `exportSnapshot` reads the document and publishes a download; it owns no mutation, no
                // precompute session and no placement scene, so it resolves here, from the snapshot, and
                // picks its lane by payload size: one inline effect under the guest's contiguous-request
                // ceiling, the framework's segmented-download lane above it (a 145 714 B Nakagin export
                // reached no file as an inline effect — B36 §5, B38 §2).
                if command.action_id() == "exportSnapshot" {
                    self.stage = Puzzle3dWindowCommandStage::Complete;
                    return match export_snapshot::puzzle3d_export_publication(&puzzle3d_scene_snapshot_from_document(snapshot.typed()), &runtime.active_example_id)? {
                        export_snapshot::Puzzle3dExportPublication::Inline(effect) => Ok(semio_framework_plugin::retained_command::ArtifactCommandWorkStep::Complete(Emit::effect(effect))),
                        export_snapshot::Puzzle3dExportPublication::Segmented(download) => Ok(semio_framework_plugin::retained_command::ArtifactCommandWorkStep::CompleteDownload { download: download, ephemeral: Default::default() }),
                        // 🧾️ An export above what ONE segmented download may carry is refused with a
                        // notice, never with a fault: a payload the wire cannot admit is an answer the
                        // user must read, and a fault here lands as a dead job instead.
                        export_snapshot::Puzzle3dExportPublication::Refused(message) => Ok(semio_framework_plugin::retained_command::ArtifactCommandWorkStep::Complete(Emit::effect(Effect::Notify { message }))),
                    };
                }
                self.prologue.scene_step(snapshot, &runtime, Some(view), Some(window_id));
                self.stage = Puzzle3dWindowCommandStage::Sync;
                Ok(semio_s_plugin_puzzle_retained::puzzle_progress_step!("puzzle3d-action-scene", "Reading the artifact", "Artefakt wird gelesen"))
            }
            Puzzle3dWindowCommandStage::Sync => {
                let owed = with_puzzle3d_session_for(self.session.clone(), &runtime, |app| self.prologue.sync_step(app, command.action_id(), &runtime, Some(view), Some(window_id)));
                if !owed {
                    self.stage = Puzzle3dWindowCommandStage::Dispatch;
                }
                Ok(semio_s_plugin_puzzle_retained::puzzle_progress_step!("puzzle3d-action-sync", "Preparing the placement session", "Platzierungssitzung wird vorbereitet"))
            }
            Puzzle3dWindowCommandStage::Dispatch => {
                let snapshot_interaction = Puzzle3dInteractionSnapshot::from_state(interaction, hover);
                let instance_owner = self.instance_owner.as_ref();
                let (emit, ephemeral) = with_puzzle3d_session_for(self.session.clone(), &runtime, |app| self.prologue.dispatch_step(app, instance_owner, "", command, Some(window_id), &runtime, Some(view), &snapshot_interaction));
                self.stage = Puzzle3dWindowCommandStage::Complete;
                Ok(semio_framework_plugin::retained_command::ArtifactCommandWorkStep::CompleteWithEphemeral { emit, ephemeral })
            }
            Puzzle3dWindowCommandStage::Complete => Err(Fault::from("puzzle3d-window-work-repeated")),
            Puzzle3dWindowCommandStage::Closing => Err(Fault::from("puzzle3d-window-work-closing")),
        }
    }

    fn begin_close(&mut self) {
        self.stage = Puzzle3dWindowCommandStage::Closing;
        self.close_owners.stage(Puzzle3dWindowWorkOwners { prologue: std::mem::take(&mut self.prologue), session: self.session.take(), instance_owner: self.instance_owner.take() });
    }

    fn close_step(&mut self, grant: semio_framework_value::retained_clone::RetainedCloneGrant) -> InteractiveJobCloseStep {
        self.close_owners.close_step(grant)
    }

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
        self.stage == Puzzle3dWindowCommandStage::Closing && self.close_owners.is_empty()
    }

    fn terminal_frame_release_bytes(&self) -> Option<usize> {
        Some(size_of::<Self>())
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Puzzle3dTransformStage {
    Read,
    Scan,
    Commit,
    Complete,
    Closing,
}

/// 🎞️ What `Read` states for one transform: the finished record, or the drop whose proximity scan `Scan` pages.
enum Puzzle3dTransformGesture {
    Stated(semio_s_artifact_puzzle_3d_editor_transform::Puzzle3dSelectionRecord),
    Scanning(semio_s_artifact_puzzle_3d_editor_transform::Puzzle3dRelocateScan),
}

/// 🔖️ A refusal notice selector: a plain function pointer owns nothing, so it retires as a copy leaf.
#[derive(Clone, Copy)]
struct Puzzle3dRefusal(fn(&Puzzle3dLabels) -> &'static str);

semio_framework_value::artifact_retire_leaf!(Puzzle3dRefusal);

/// 🛠️ The ONE retained work of every selection transform — a gumball translate/rotate/scale, a target-volume
/// gumball relocate, a Relocate-utility drop, an inspector origin nudge. `Read` states the gesture as ONE
/// [`utilities::transform::Puzzle3dSelectionRecord`] (the gesture's own ids, else the live selection); a drop's
/// proximity scan then runs page by page in `Scan` (one progress report per page, cancellable between pages), and
/// `Commit` runs the record through the transform tool: one `ToolTransaction` whose parametric leaves publish as ONE
/// edit stamped with the ref minted from the admission's `authoring_seed`. A gesture that moves nothing leaves
/// zero trace; nothing addressed, or everything addressed locked, is one localized refusal.
/// ♻️ The owners one `Puzzle3dTransformWork` still holds when its job closes, retired as one controlled bundle.
#[derive(semio_framework_value::RetireOwned)]
struct Puzzle3dTransformWorkOwners {
    record: Option<semio_s_artifact_puzzle_3d_editor_transform::Puzzle3dSelectionRecord>,
    scan: Option<semio_s_artifact_puzzle_3d_editor_transform::Puzzle3dRelocateScan>,
    refusal: Option<Puzzle3dRefusal>,
}

pub struct Puzzle3dTransformWork {
    tool_id: &'static str,
    authoring_seed: String,
    stage: Puzzle3dTransformStage,
    record: Option<semio_s_artifact_puzzle_3d_editor_transform::Puzzle3dSelectionRecord>,
    scan: Option<semio_s_artifact_puzzle_3d_editor_transform::Puzzle3dRelocateScan>,
    refusal: Option<fn(&Puzzle3dLabels) -> &'static str>,
    close_owners: semio_s_plugin_puzzle_jobs::WorkClosing<Puzzle3dTransformWorkOwners>,
}

impl Puzzle3dTransformWork {
    fn new(tool_id: &'static str, authoring_seed: String) -> Self {
        Self { close_owners: Default::default(), tool_id, authoring_seed, stage: Puzzle3dTransformStage::Read, record: None, scan: None, refusal: None }
    }

    /// 🕹️ The ids a selection-scoped verb addresses: the command's own `ids`, else the live selection at
    /// `granularity`.
    fn addressed(command: &Puzzle3dCommand, interaction: &protocol::InteractionState, granularity: &str) -> Vec<String> {
        let explicit: Vec<String> = command.args().and_then(|args| args.get("ids")).and_then(Value::as_array).into_iter().flatten().filter_map(Value::as_str).map(str::to_string).collect();
        if !explicit.is_empty() {
            return explicit;
        }
        Self::selected(interaction, granularity)
    }

    fn selected(interaction: &protocol::InteractionState, granularity: &str) -> Vec<String> {
        interaction.selection.get(PUZZLE3D_INTERACTION_DOMAIN).filter(|selection| selection.granularity == granularity).map_or_else(Vec::new, |selection| selection.ids.clone())
    }

    /// 🎬️ The gesture this command states on `document`, or the refusal it answers with instead.
    fn read(&self, command: &Puzzle3dCommand, document: &Puzzle3dSnapshot, interaction: &protocol::InteractionState, window_config: Option<&semio_framework_plugin::WindowConfigSnapshot>) -> Result<Option<Puzzle3dTransformGesture>, fn(&Puzzle3dLabels) -> &'static str> {
        let args = command.args();
        match self.tool_id {
            "translateSelection" | "rotateSelection" | "scaleSelection" => {
                let targets = [Self::addressed(command, interaction, PUZZLE3D_GRANULARITY_OBJECT), Self::selected(interaction, PUZZLE3D_GRANULARITY_TARGET_VOLUME)].concat();
                Ok(semio_s_artifact_puzzle_3d_editor_transform::Puzzle3dSelectionRecord::from_gumball(self.tool_id, args, targets).map(Puzzle3dTransformGesture::Stated))
            }
            "relocateTargetVolume" => Ok(semio_s_artifact_puzzle_3d_editor_transform::Puzzle3dSelectionRecord::from_pose_delta(args).map(Puzzle3dTransformGesture::Stated)),
            "worldRelocate" => {
                let object_id = args.and_then(|args| args.get("objectId")).and_then(Value::as_str).unwrap_or("");
                let Some(position) = args.and_then(|args| args.get("position")).and_then(value_as_vec3) else { return Ok(None) };
                if document.objects.iter().any(|object| object.id == object_id && (object.locked || object.hidden)) {
                    return Err(|labels| labels.selection_locked.as_str());
                }
                let radius = window_ownership::config_from_snapshot(window_config).proximity_radius;
                Ok(semio_s_artifact_puzzle_3d_editor_transform::Puzzle3dRelocateScan::begin(document, object_id, position, radius).map(Puzzle3dTransformGesture::Scanning))
            }
            _ => {
                let Some(offset) = puzzle3d_inspector_origin_nudge(args) else { return Ok(None) };
                let granularity = if args.and_then(|args| args.get("entity")).and_then(Value::as_str) == Some("targetVolume") { PUZZLE3D_GRANULARITY_TARGET_VOLUME } else { PUZZLE3D_GRANULARITY_OBJECT };
                Ok(Some(Puzzle3dTransformGesture::Stated(semio_s_artifact_puzzle_3d_editor_transform::Puzzle3dSelectionRecord::new(Self::addressed(command, interaction, granularity), semio_s_artifact_puzzle_3d_editor_transform::Puzzle3dSelectionMotion::Drag { offset }))))
            }
        }
    }

    /// 🏁️ The ONE terminal emit: the committed transaction, a refusal, or nothing at all.
    fn commit(&mut self, snapshot: &Puzzle3dPlaySnapshot, view_state: Option<&semio_framework_plugin::ViewModel>) -> Emit<Puzzle3dMutation, Puzzle3dConfigMutation> {
        self.stage = Puzzle3dTransformStage::Complete;
        if let Some(refusal) = self.refusal.take() {
            return puzzle3d_notice_emit(view_state, refusal);
        }
        let Some(record) = self.record.take() else { return Emit { ui_scope: UiDirtyScope::None, ..Default::default() } };
        let base = snapshot.typed_arc();
        let known = record.targets.iter().any(|id| base.objects.iter().any(|object| &object.id == id) || base.target_volumes.iter().any(|volume| &volume.id == id));
        if !known {
            return puzzle3d_notice_emit(view_state, |labels| labels.nothing_selected.as_str());
        }
        if record.refused_as_locked(&base) {
            return puzzle3d_notice_emit(view_state, |labels| labels.selection_locked.as_str());
        }
        let request = semio_s_artifact_puzzle_3d_editor_transform::TransformToolRequest { base, records: vec![record] };
        match semio_s_artifact_puzzle_3d_editor_transform::puzzle3d_transform_tool_commit(self.tool_id, &self.authoring_seed, request) {
            Some((transaction, mutations)) => Emit { artifact_mutations: mutations, transaction: (!self.authoring_seed.is_empty()).then_some(transaction), ui_scope: puzzle3d_scope(puzzle3d_command_scope_class(self.tool_id)), ..Default::default() },
            None => Emit { ui_scope: UiDirtyScope::None, ..Default::default() },
        }
    }
}

impl<A: Puzzle3dEditor> semio_framework_plugin::retained_command::ArtifactCommandWork<EditorApp<A>> for Puzzle3dTransformWork {
    fn tool_id(&self) -> &'static str {
        self.tool_id
    }

    /// 🔢️ `Read` + `Commit`, plus one `Scan` step per page of objects a drop measures.
    fn extent(&self, command: &Puzzle3dCommand, snapshot: &Puzzle3dPlaySnapshot, interaction: &protocol::InteractionState, _context: Option<&semio_framework_plugin::app::ArtifactOwnedToolJobContext<EditorApp<A>>>) -> Option<usize> {
        let addressed = Self::addressed(command, interaction, PUZZLE3D_GRANULARITY_OBJECT).len().checked_add(Self::selected(interaction, PUZZLE3D_GRANULARITY_TARGET_VOLUME).len())?;
        let pages = if self.tool_id == "worldRelocate" { snapshot.typed().objects.len().div_ceil(semio_s_artifact_puzzle_3d_editor_transform::PUZZLE3D_RELOCATE_SCAN_PAGE).max(1) } else { 0 };
        (addressed <= semio_s_plugin_puzzle_retained::PUZZLE_COMMAND_DECODED_ITEMS).then_some(2 + pages)
    }

    fn work_demands(&self, _input: &semio_framework_plugin::retained_command::ArtifactCommandInputs<'_, EditorApp<A>>, _maximum_copy_bytes: usize) -> Result<semio_framework_value::RetirementDemand, semio_framework_value::ValueError> {
        semio_s_plugin_puzzle_retained::step_demands::<EditorApp<A>>(size_of::<Self>())
    }

    fn step(&mut self, input: &semio_framework_plugin::retained_command::ArtifactCommandInputs<'_, EditorApp<A>>, _cx: &mut StepContext<'_>) -> Result<semio_framework_plugin::retained_command::ArtifactCommandWorkStep<EditorApp<A>>, Fault> {
        let semio_framework_plugin::retained_command::ArtifactCommandInputs { command, snapshot, config: _config, interaction, hover: _hover, context, .. } = *input;
        match self.stage {
            Puzzle3dTransformStage::Read => {
                match self.read(command, snapshot.typed(), interaction, context.and_then(|context| context.window_config.as_ref())) {
                    Ok(Some(Puzzle3dTransformGesture::Stated(record))) => self.record = Some(record),
                    Ok(Some(Puzzle3dTransformGesture::Scanning(scan))) => self.scan = Some(scan),
                    Ok(None) => {}
                    Err(refusal) => self.refusal = Some(refusal),
                }
                self.stage = if self.scan.is_some() { Puzzle3dTransformStage::Scan } else { Puzzle3dTransformStage::Commit };
                Ok(semio_s_plugin_puzzle_retained::puzzle_progress_step!("puzzle3d-transform-read", "Reading the gesture", "Geste wird gelesen"))
            }
            Puzzle3dTransformStage::Scan => {
                let scan = self.scan.as_mut().ok_or_else(|| Fault::from("puzzle3d-transform-scan-owner"))?;
                if scan.step(snapshot.typed(), semio_s_artifact_puzzle_3d_editor_transform::PUZZLE3D_RELOCATE_SCAN_PAGE) {
                    self.record = self.scan.take().map(semio_s_artifact_puzzle_3d_editor_transform::Puzzle3dRelocateScan::finish);
                    self.stage = Puzzle3dTransformStage::Commit;
                }
                Ok(semio_s_plugin_puzzle_retained::puzzle_progress_step!("puzzle3d-transform-scan", "Measuring nearby vortices", "Nahe Vortices werden gemessen"))
            }
            Puzzle3dTransformStage::Commit => Ok(semio_framework_plugin::retained_command::ArtifactCommandWorkStep::Complete(self.commit(snapshot, context.and_then(|context| context.view_state.as_ref())))),
            Puzzle3dTransformStage::Complete => Err(Fault::from("puzzle3d-transform-complete-repolled")),
            Puzzle3dTransformStage::Closing => Err(Fault::from("puzzle3d-transform-closing")),
        }
    }

    fn begin_close(&mut self) {
        self.stage = Puzzle3dTransformStage::Closing;
        self.close_owners.stage(Puzzle3dTransformWorkOwners { record: std::mem::take(&mut self.record), scan: std::mem::take(&mut self.scan), refusal: std::mem::take(&mut self.refusal).map(Puzzle3dRefusal) });
    }

    fn close_step(&mut self, grant: semio_framework_value::retained_clone::RetainedCloneGrant) -> InteractiveJobCloseStep {
        self.close_owners.close_step(grant)
    }

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
        self.stage == Puzzle3dTransformStage::Closing && self.close_owners.is_empty()
    }

    fn terminal_frame_release_bytes(&self) -> Option<usize> {
        Some(size_of::<Self>())
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Puzzle3dAddBrushObjectStage {
    Decode,
    Kind,
    Representation,
    Vortices,
    ExistingAttractions,
    PublishObject,
    PublishAttraction,
    Complete,
    Closing,
}

#[derive(semio_framework_value::RetireOwned)]

struct Puzzle3dBrushPayloadOwner {
    target_vortex_id: String,
    object_kind_id: String,
    source_vortex_index: usize,
    origin: [f64; 3],
    orientation: [f64; 4],
    scale: Option<semio_s_artifact_puzzle_3d_model::Puzzle3dScale>,
}

/// ♻️ The owners one `Puzzle3dAddBrushObjectWork` still holds when its job closes, retired as one controlled bundle.
#[derive(semio_framework_value::RetireOwned)]
struct Puzzle3dAddBrushObjectWorkOwners {
    payload: Option<Puzzle3dBrushPayloadOwner>,
    object_id: Option<String>,
    mesh_url: Option<String>,
    vortices: Vec<semio_s_artifact_puzzle_3d_model::Puzzle3dVortex>,
    mutations: Vec<Puzzle3dMutation>,
}

pub struct Puzzle3dAddBrushObjectWork {
    stage: Puzzle3dAddBrushObjectStage,
    kind_cursor: usize,
    representation_cursor: usize,
    vortex_cursor: usize,
    attraction_cursor: usize,
    kind_index: Option<usize>,
    payload: Option<Puzzle3dBrushPayloadOwner>,
    object_id: Option<String>,
    mesh_url: Option<String>,
    vortices: Vec<semio_s_artifact_puzzle_3d_model::Puzzle3dVortex>,
    mutations: Vec<Puzzle3dMutation>,
    close_owners: semio_s_plugin_puzzle_jobs::WorkClosing<Puzzle3dAddBrushObjectWorkOwners>,
}

impl Default for Puzzle3dAddBrushObjectWork {
    fn default() -> Self {
        Self { close_owners: Default::default(),
            stage: Puzzle3dAddBrushObjectStage::Decode,
            kind_cursor: 0,
            representation_cursor: 0,
            vortex_cursor: 0,
            attraction_cursor: 0,
            kind_index: None,
            payload: None,
            object_id: None,
            mesh_url: None,
            vortices: Vec::with_capacity(PUZZLE3D_RELOCATE_VORTICES_PER_OBJECT),
            mutations: Vec::with_capacity(2),
        }
    }
}

impl Puzzle3dAddBrushObjectWork {
    fn vector3(value: Option<&Value>) -> Option<[f64; 3]> {
        let values = value.and_then(Value::as_array)?;
        Some([values.first().and_then(Value::as_f64)?, values.get(1).and_then(Value::as_f64)?, values.get(2).and_then(Value::as_f64)?])
    }

    fn quaternion(value: Option<&Value>) -> Option<[f64; 4]> {
        let values = value.and_then(Value::as_array)?;
        Some([values.first().and_then(Value::as_f64)?, values.get(1).and_then(Value::as_f64)?, values.get(2).and_then(Value::as_f64)?, values.get(3).and_then(Value::as_f64)?])
    }

    fn scale(value: Option<&Value>) -> Option<semio_s_artifact_puzzle_3d_model::Puzzle3dScale> {
        match value {
            Some(Value::Number(value)) => Some(semio_s_artifact_puzzle_3d_model::Puzzle3dScale::Uniform(value.as_f64())),
            Some(Value::Array(values)) if values.len() >= 3 => Some(semio_s_artifact_puzzle_3d_model::Puzzle3dScale::Vec3([values.first().and_then(Value::as_f64)?, values.get(1).and_then(Value::as_f64)?, values.get(2).and_then(Value::as_f64)?])),
            _ => None,
        }
    }

    fn decode(command: &Puzzle3dCommand) -> Option<Puzzle3dBrushPayloadOwner> {
        let args = command.args()?;
        let target_vortex_id = args.get("targetVortexFullId").and_then(Value::as_str)?.to_string();
        let object_kind_id = args.get("objectKindId").and_then(Value::as_str)?.to_string();
        let source_vortex_index = args.get("sourceVortexIndex").and_then(Value::as_u64)? as usize;
        Some(Puzzle3dBrushPayloadOwner { target_vortex_id, object_kind_id, source_vortex_index, origin: Self::vector3(args.get("origin"))?, orientation: Self::quaternion(args.get("orientation"))?, scale: Self::scale(args.get("scale")) })
    }

    fn object_id(snapshot: &Puzzle3dPlaySnapshot, payload: &Puzzle3dBrushPayloadOwner) -> String {
        use std::hash::{Hash, Hasher};
        let mut hasher = std::collections::hash_map::DefaultHasher::new();
        snapshot.typed().objects.len().hash(&mut hasher);
        payload.target_vortex_id.hash(&mut hasher);
        payload.object_kind_id.hash(&mut hasher);
        payload.source_vortex_index.hash(&mut hasher);
        for value in payload.origin.into_iter().chain(payload.orientation) {
            value.to_bits().hash(&mut hasher);
        }
        format!("puzzle3d.brush.{:016x}", hasher.finish())
    }

}

impl<A: Puzzle3dEditor> semio_framework_plugin::retained_command::ArtifactCommandWork<EditorApp<A>> for Puzzle3dAddBrushObjectWork {
    fn tool_id(&self) -> &'static str {
        "addBrushObject"
    }

    fn extent(&self, _command: &Puzzle3dCommand, snapshot: &Puzzle3dPlaySnapshot, _interaction: &protocol::InteractionState, _context: Option<&semio_framework_plugin::app::ArtifactOwnedToolJobContext<EditorApp<A>>>) -> Option<usize> {
        let catalogs = snapshot.typed().meta.kind_catalogs.as_ref()?;
        let items = catalogs.objects.len().checked_add(PUZZLE3D_RELOCATE_VORTICES_PER_OBJECT.checked_mul(2)?)?.checked_add(snapshot.typed().attractions.len())?.checked_add(4)?;
        (items <= semio_s_plugin_puzzle_retained::PUZZLE_COMMAND_WORK_ITEMS).then_some(items)
    }

    fn work_demands(&self, _input: &semio_framework_plugin::retained_command::ArtifactCommandInputs<'_, EditorApp<A>>, _maximum_copy_bytes: usize) -> Result<semio_framework_value::RetirementDemand, semio_framework_value::ValueError> {
        semio_s_plugin_puzzle_retained::step_demands::<EditorApp<A>>(size_of::<Self>())
    }

    fn step(&mut self, input: &semio_framework_plugin::retained_command::ArtifactCommandInputs<'_, EditorApp<A>>, _cx: &mut StepContext<'_>) -> Result<semio_framework_plugin::retained_command::ArtifactCommandWorkStep<EditorApp<A>>, Fault> {
        let semio_framework_plugin::retained_command::ArtifactCommandInputs { command, snapshot, config: _config, interaction: _interaction, hover: _hover, context, .. } = *input;
        let view_state = context.and_then(|context| context.view_state.as_ref());
        // 🧯️ Every early terminal below is a click that placed nothing. Each one now says so — an
        // absent/mismatched kind, a kind with no mesh or no source vortex, and a target vortex that is
        // already attracted are all invisible refusals otherwise.
        let Some(catalogs) = snapshot.typed().meta.kind_catalogs.as_ref() else {
            self.stage = Puzzle3dAddBrushObjectStage::Complete;
            return Ok(semio_framework_plugin::retained_command::ArtifactCommandWorkStep::Complete(puzzle3d_notice_emit(view_state, |labels| labels.placement_unavailable.as_str())));
        };
        match self.stage {
            Puzzle3dAddBrushObjectStage::Decode => {
                let Some(payload) = Self::decode(command) else {
                    self.stage = Puzzle3dAddBrushObjectStage::Complete;
                    return Ok(semio_framework_plugin::retained_command::ArtifactCommandWorkStep::Complete(puzzle3d_notice_emit(view_state, |labels| labels.placement_rejected.as_str())));
                };
                self.object_id = Some(Self::object_id(snapshot, &payload));
                self.payload = Some(payload);
                self.stage = Puzzle3dAddBrushObjectStage::Kind;
                Ok(semio_s_plugin_puzzle_retained::puzzle_progress_step!("puzzle3d-brush-kind", "Finding brush object kind", "Pinselobjektart wird gesucht"))
            }
            Puzzle3dAddBrushObjectStage::Kind => {
                let payload = self.payload.as_ref().ok_or_else(|| Fault::from("puzzle3d-brush-payload-owner"))?;
                let Some(kind) = catalogs.objects.get(self.kind_cursor) else {
                    self.stage = Puzzle3dAddBrushObjectStage::Complete;
                    return Ok(semio_framework_plugin::retained_command::ArtifactCommandWorkStep::Complete(puzzle3d_notice_emit(view_state, |labels| labels.placement_unavailable.as_str())));
                };
                if kind.id == payload.object_kind_id {
                    if kind.representations.len() > PUZZLE3D_RELOCATE_VORTICES_PER_OBJECT || kind.vortices.len() > PUZZLE3D_RELOCATE_VORTICES_PER_OBJECT {
                        return Err(Fault::from("puzzle3d-brush-catalog-capacity"));
                    }
                    self.kind_index = Some(self.kind_cursor);
                    self.stage = Puzzle3dAddBrushObjectStage::Representation;
                } else {
                    self.kind_cursor += 1;
                }
                Ok(semio_s_plugin_puzzle_retained::puzzle_progress_step!("puzzle3d-brush-kind", "Scanning brush object kind", "Pinselobjektart wird geprüft"))
            }
            Puzzle3dAddBrushObjectStage::Representation => {
                let kind = catalogs.objects.get(self.kind_index.ok_or_else(|| Fault::from("puzzle3d-brush-kind-owner"))?).ok_or_else(|| Fault::from("puzzle3d-brush-kind-cursor"))?;
                let Some(representation) = kind.representations.get(self.representation_cursor) else {
                    self.stage = Puzzle3dAddBrushObjectStage::Complete;
                    return Ok(semio_framework_plugin::retained_command::ArtifactCommandWorkStep::Complete(puzzle3d_notice_emit(view_state, |labels| labels.placement_unavailable.as_str())));
                };
                self.representation_cursor += 1;
                if !representation.url.is_empty() {
                    self.mesh_url = Some(representation.url.clone());
                    self.stage = Puzzle3dAddBrushObjectStage::Vortices;
                }
                Ok(semio_s_plugin_puzzle_retained::puzzle_progress_step!("puzzle3d-brush-representation", "Finding brush mesh", "Pinsel-Mesh wird gesucht"))
            }
            Puzzle3dAddBrushObjectStage::Vortices => {
                let kind = catalogs.objects.get(self.kind_index.ok_or_else(|| Fault::from("puzzle3d-brush-kind-owner"))?).ok_or_else(|| Fault::from("puzzle3d-brush-kind-cursor"))?;
                let Some(template) = kind.vortices.get(self.vortex_cursor) else {
                    let payload = self.payload.as_ref().ok_or_else(|| Fault::from("puzzle3d-brush-payload-owner"))?;
                    if payload.source_vortex_index >= self.vortices.len() {
                        self.stage = Puzzle3dAddBrushObjectStage::Complete;
                        return Ok(semio_framework_plugin::retained_command::ArtifactCommandWorkStep::Complete(puzzle3d_notice_emit(view_state, |labels| labels.placement_rejected.as_str())));
                    }
                    self.stage = Puzzle3dAddBrushObjectStage::ExistingAttractions;
                    return Ok(semio_s_plugin_puzzle_retained::puzzle_progress_step!("puzzle3d-brush-existing-attraction", "Checking brush target", "Pinselziel wird geprüft"));
                };
                let object_id = self.object_id.as_ref().ok_or_else(|| Fault::from("puzzle3d-brush-object-owner"))?;
                let index = self.vortex_cursor;
                self.vortex_cursor += 1;
                self.vortices.push(semio_s_artifact_puzzle_3d_model::Puzzle3dVortex {
                    id: format!("{object_id}:v{index}"),
                    label: None,
                    vortex_kind: template.vortex_kind.clone(),
                    position: template.point,
                    direction: Some(template.direction),
                    radius: template.radius,
                    hidden: false,
                    locked: false,
                });
                Ok(semio_s_plugin_puzzle_retained::puzzle_progress_step!("puzzle3d-brush-vortex", "Building brush vortex", "Pinsel-Vortex wird aufgebaut"))
            }
            Puzzle3dAddBrushObjectStage::ExistingAttractions => {
                let payload = self.payload.as_ref().ok_or_else(|| Fault::from("puzzle3d-brush-payload-owner"))?;
                let source = self.vortices.get(payload.source_vortex_index).ok_or_else(|| Fault::from("puzzle3d-brush-source-vortex-owner"))?;
                let Some(attraction) = snapshot.typed().attractions.get(self.attraction_cursor) else {
                    self.stage = Puzzle3dAddBrushObjectStage::PublishObject;
                    return Ok(semio_s_plugin_puzzle_retained::puzzle_progress_step!("puzzle3d-brush-publish-object", "Preparing brush object", "Pinselobjekt wird vorbereitet"));
                };
                self.attraction_cursor += 1;
                if attraction.attracting == payload.target_vortex_id || attraction.attracted == source.id {
                    self.stage = Puzzle3dAddBrushObjectStage::Complete;
                    return Ok(semio_framework_plugin::retained_command::ArtifactCommandWorkStep::Complete(puzzle3d_notice_emit(view_state, |labels| labels.placement_occupied.as_str())));
                }
                Ok(semio_s_plugin_puzzle_retained::puzzle_progress_step!("puzzle3d-brush-existing-attraction", "Scanning brush target", "Pinselziel wird geprüft"))
            }
            Puzzle3dAddBrushObjectStage::PublishObject => {
                let payload = self.payload.as_ref().ok_or_else(|| Fault::from("puzzle3d-brush-payload-owner"))?;
                let scene_snapshot = puzzle3d_scene_snapshot_from_document(snapshot.typed());
                let object = semio_s_artifact_puzzle_3d_model::Puzzle3dObject {
                    id: self.object_id.clone().ok_or_else(|| Fault::from("puzzle3d-brush-object-owner"))?,
                    label: Some(puzzle3d_next_object_label(&scene_snapshot.objects, &scene_snapshot, &payload.object_kind_id)),
                    object_kind: Some(payload.object_kind_id.clone()),
                    anchor: Default::default(),
                    origin: payload.origin,
                    orientation: Some(payload.orientation),
                    scale: payload.scale,
                    mesh_url: self.mesh_url.clone(),
                    vortices: std::mem::take(&mut self.vortices),
                    hidden: false,
                    locked: false,
                };
                self.mutations.push(semio_s_artifact_puzzle_3d_mutations_object::create_object::mutation::create_object(object, None));
                self.stage = Puzzle3dAddBrushObjectStage::PublishAttraction;
                Ok(semio_s_plugin_puzzle_retained::puzzle_progress_step!("puzzle3d-brush-publish-object", "Publishing brush object", "Pinselobjekt wird veröffentlicht"))
            }
            Puzzle3dAddBrushObjectStage::PublishAttraction => {
                let payload = self.payload.take().ok_or_else(|| Fault::from("puzzle3d-brush-payload-owner"))?;
                let object_id = self.object_id.take().ok_or_else(|| Fault::from("puzzle3d-brush-object-owner"))?;
                let attracted = format!("{object_id}:v{}", payload.source_vortex_index);
                let attraction_id = format!("attraction-{}-{attracted}", payload.target_vortex_id);
                self.mutations.push(semio_s_artifact_puzzle_3d_mutations_vortex::connect_vortices::mutation::connect_vortices(attraction_id, payload.target_vortex_id, attracted, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, None));
                self.stage = Puzzle3dAddBrushObjectStage::Complete;
                Ok(semio_framework_plugin::retained_command::ArtifactCommandWorkStep::Complete(Emit { artifact_mutations: std::mem::take(&mut self.mutations), ui_scope: puzzle3d_scope(puzzle3d_command_scope_class("addBrushObject")), ..Default::default() }))
            }
            Puzzle3dAddBrushObjectStage::Complete => Err(Fault::from("puzzle3d-brush-complete-repolled")),
            Puzzle3dAddBrushObjectStage::Closing => Err(Fault::from("puzzle3d-brush-closing")),
        }
    }

    fn begin_close(&mut self) {
        self.stage = Puzzle3dAddBrushObjectStage::Closing;
        self.close_owners.stage(Puzzle3dAddBrushObjectWorkOwners { payload: std::mem::take(&mut self.payload), object_id: std::mem::take(&mut self.object_id), mesh_url: std::mem::take(&mut self.mesh_url), vortices: std::mem::take(&mut self.vortices), mutations: std::mem::take(&mut self.mutations) });
    }

    fn close_step(&mut self, grant: semio_framework_value::retained_clone::RetainedCloneGrant) -> InteractiveJobCloseStep {
        self.close_owners.close_step(grant)
    }

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
        self.stage == Puzzle3dAddBrushObjectStage::Closing && self.close_owners.is_empty()
    }

    fn terminal_frame_release_bytes(&self) -> Option<usize> {
        Some(size_of::<Self>())
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Puzzle3dFocusSelectionStage {
    Selection,
    SumObjects,
    DistanceObjects,
    Publish,
    Complete,
    Closing,
}

/// ♻️ The owners one `Puzzle3dFocusSelectionWork` still holds when its job closes, retired as one controlled bundle.
#[derive(semio_framework_value::RetireOwned)]
struct Puzzle3dFocusSelectionWorkOwners {
    selected: HashSet<String>,
}

pub struct Puzzle3dFocusSelectionWork {
    stage: Puzzle3dFocusSelectionStage,
    selection_cursor: usize,
    object_cursor: usize,
    selected: HashSet<String>,
    center: [f64; 3],
    matched: usize,
    maximum_distance: f64,
    close_owners: semio_s_plugin_puzzle_jobs::WorkClosing<Puzzle3dFocusSelectionWorkOwners>,
}

impl Default for Puzzle3dFocusSelectionWork {
    fn default() -> Self {
        Self { close_owners: Default::default(),
            stage: Puzzle3dFocusSelectionStage::Selection,
            selection_cursor: 0,
            object_cursor: 0,
            selected: HashSet::with_capacity(semio_s_plugin_puzzle_retained::PUZZLE_COMMAND_DECODED_ITEMS),
            center: [0.0; 3],
            matched: 0,
            maximum_distance: 1.0,
        }
    }
}

impl Puzzle3dFocusSelectionWork {
    fn selection(interaction: &protocol::InteractionState) -> &[String] {
        interaction.selection.get(PUZZLE3D_INTERACTION_DOMAIN).filter(|selection| selection.granularity == PUZZLE3D_GRANULARITY_OBJECT).map(|selection| selection.ids.as_slice()).unwrap_or_default()
    }

    /// 🎥️ Which objects this focus frames. An EMPTY selection frames the whole document rather than
    /// completing with `Emit::default()` — a camera verb with nothing selected has an obvious subject
    /// (everything), and the silent no-op made `f` a dead key on every boot before the user had picked
    /// anything (`📓️2026-09-11-wave-B1-battery-extension.md` §5 defect 14). Costs no extra capacity: the
    /// two object scans this work already declares in `extent` run either way.
    fn frames(&self, object_id: &str) -> bool {
        self.selected.is_empty() || self.selected.contains(object_id)
    }

}

impl<A: Puzzle3dEditor> semio_framework_plugin::retained_command::ArtifactCommandWork<EditorApp<A>> for Puzzle3dFocusSelectionWork {
    fn tool_id(&self) -> &'static str {
        "focusSelection"
    }

    fn extent(&self, _command: &Puzzle3dCommand, snapshot: &Puzzle3dPlaySnapshot, interaction: &protocol::InteractionState, _context: Option<&semio_framework_plugin::app::ArtifactOwnedToolJobContext<EditorApp<A>>>) -> Option<usize> {
        let selected = Self::selection(interaction).len();
        let objects = snapshot.typed().objects.len();
        let items = selected.checked_add(objects.checked_mul(2)?)?.checked_add(1)?;
        (selected <= semio_s_plugin_puzzle_retained::PUZZLE_COMMAND_DECODED_ITEMS && items <= semio_s_plugin_puzzle_retained::PUZZLE_COMMAND_WORK_ITEMS).then_some(items)
    }

    fn work_demands(&self, _input: &semio_framework_plugin::retained_command::ArtifactCommandInputs<'_, EditorApp<A>>, _maximum_copy_bytes: usize) -> Result<semio_framework_value::RetirementDemand, semio_framework_value::ValueError> {
        semio_s_plugin_puzzle_retained::step_demands::<EditorApp<A>>(size_of::<Self>())
    }

    fn step(&mut self, input: &semio_framework_plugin::retained_command::ArtifactCommandInputs<'_, EditorApp<A>>, _cx: &mut StepContext<'_>) -> Result<semio_framework_plugin::retained_command::ArtifactCommandWorkStep<EditorApp<A>>, Fault> {
        let semio_framework_plugin::retained_command::ArtifactCommandInputs { command: _command, snapshot, config: _config, interaction, hover: _hover, context, .. } = *input;
        let view_state = context.and_then(|context| context.view_state.as_ref());
        let window_config_snapshot = context.and_then(|context| context.window_config.as_ref());
        match self.stage {
            Puzzle3dFocusSelectionStage::Selection => {
                if let Some(id) = Self::selection(interaction).get(self.selection_cursor) {
                    if self.selected.len() >= semio_s_plugin_puzzle_retained::PUZZLE_COMMAND_DECODED_ITEMS {
                        return Err(Fault::from("puzzle3d-focus-selection-capacity"));
                    }
                    self.selected.insert(id.clone());
                    self.selection_cursor += 1;
                    return Ok(semio_s_plugin_puzzle_retained::puzzle_progress_step!("puzzle3d-focus-selection-owner", "Reading selected object", "Ausgewähltes Objekt wird gelesen"));
                }
                self.stage = Puzzle3dFocusSelectionStage::SumObjects;
                Ok(semio_s_plugin_puzzle_retained::puzzle_progress_step!("puzzle3d-focus-selection-center", "Measuring selection center", "Auswahlzentrum wird gemessen"))
            }
            Puzzle3dFocusSelectionStage::SumObjects => {
                let Some(object) = snapshot.typed().objects.get(self.object_cursor) else {
                    self.object_cursor = 0;
                    if self.matched > 0 {
                        let divisor = self.matched as f64;
                        self.center = [self.center[0] / divisor, self.center[1] / divisor, self.center[2] / divisor];
                    }
                    self.stage = Puzzle3dFocusSelectionStage::DistanceObjects;
                    return Ok(semio_s_plugin_puzzle_retained::puzzle_progress_step!("puzzle3d-focus-selection-distance", "Measuring selection radius", "Auswahlradius wird gemessen"));
                };
                self.object_cursor += 1;
                if self.frames(&object.id) {
                    self.center[0] += object.origin[0];
                    self.center[1] += object.origin[1];
                    self.center[2] += object.origin[2];
                    self.matched += 1;
                }
                Ok(semio_s_plugin_puzzle_retained::puzzle_progress_step!("puzzle3d-focus-selection-center", "Scanning selected object", "Ausgewähltes Objekt wird geprüft"))
            }
            Puzzle3dFocusSelectionStage::DistanceObjects => {
                let Some(object) = snapshot.typed().objects.get(self.object_cursor) else {
                    self.stage = Puzzle3dFocusSelectionStage::Publish;
                    return Ok(semio_s_plugin_puzzle_retained::puzzle_progress_step!("puzzle3d-focus-selection-publish", "Preparing camera focus", "Kamerafokus wird vorbereitet"));
                };
                self.object_cursor += 1;
                if self.frames(&object.id) {
                    let delta = [object.origin[0] - self.center[0], object.origin[1] - self.center[1], object.origin[2] - self.center[2]];
                    self.maximum_distance = self.maximum_distance.max((delta[0] * delta[0] + delta[1] * delta[1] + delta[2] * delta[2]).sqrt());
                }
                Ok(semio_s_plugin_puzzle_retained::puzzle_progress_step!("puzzle3d-focus-selection-distance", "Scanning selection radius", "Auswahlradius wird geprüft"))
            }
            Puzzle3dFocusSelectionStage::Publish => {
                self.stage = Puzzle3dFocusSelectionStage::Complete;
                if self.matched == 0 {
                    return Ok(semio_framework_plugin::retained_command::ArtifactCommandWorkStep::Complete(Emit::default()));
                }
                let distance = self.maximum_distance * 3.0 + 2.0;
                let mut next = window_ownership::config_from_snapshot(window_config_snapshot);
                next.camera.position = [self.center[0] + distance * 0.6, self.center[1] - distance * 0.6, self.center[2] + distance * 0.5];
                next.camera.target = self.center;
                let view = view_state.ok_or_else(|| Fault::from("puzzle3d-focus-window-context-required"))?;
                Ok(semio_framework_plugin::retained_command::ArtifactCommandWorkStep::Complete(Emit { window_config_mutations: vec![window_ownership::addressed_config(view, next)?], ui_scope: puzzle3d_scope(puzzle3d_command_scope_class("focusSelection")), ..Default::default() }))
            }
            Puzzle3dFocusSelectionStage::Complete => Err(Fault::from("puzzle3d-focus-selection-complete-repolled")),
            Puzzle3dFocusSelectionStage::Closing => Err(Fault::from("puzzle3d-focus-selection-closing")),
        }
    }

    fn begin_close(&mut self) {
        self.stage = Puzzle3dFocusSelectionStage::Closing;
        self.close_owners.stage(Puzzle3dFocusSelectionWorkOwners { selected: std::mem::take(&mut self.selected) });
    }

    fn close_step(&mut self, grant: semio_framework_value::retained_clone::RetainedCloneGrant) -> InteractiveJobCloseStep {
        self.close_owners.close_step(grant)
    }

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
        self.stage == Puzzle3dFocusSelectionStage::Closing && self.close_owners.is_empty()
    }

    fn terminal_frame_release_bytes(&self) -> Option<usize> {
        Some(size_of::<Self>())
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Puzzle3dAcceptSuggestionStage {
    Target,
    Candidate,
    Vortices,
    ExistingAttractions,
    PublishObject,
    PublishAttraction,
    PublishResult,
    Complete,
    Closing,
}

/// ♻️ The owners one `Puzzle3dAcceptSuggestionWork` still holds when its job closes, retired as one controlled bundle.
#[derive(semio_framework_value::RetireOwned)]
struct Puzzle3dAcceptSuggestionWorkOwners {
    vortices: Vec<semio_s_artifact_puzzle_3d_model::Puzzle3dVortex>,
    mutations: Vec<Puzzle3dMutation>,
    target_id: Option<String>,
    object_id: Option<String>,
    candidate: Option<semio_s_artifact_puzzle_3d_model::BrushPreviewState>,
    instance_owner: Option<ArtifactInstanceOperationOwnerHandle>,
}

pub struct Puzzle3dAcceptSuggestionWork {
    stage: Puzzle3dAcceptSuggestionStage,
    object_cursor: usize,
    vortex_cursor: usize,
    attraction_cursor: usize,
    target_id: Option<String>,
    kind_index: Option<usize>,
    object_id: Option<String>,
    candidate: Option<semio_s_artifact_puzzle_3d_model::BrushPreviewState>,
    vortices: Vec<semio_s_artifact_puzzle_3d_model::Puzzle3dVortex>,
    mutations: Vec<Puzzle3dMutation>,
    instance_owner: Option<ArtifactInstanceOperationOwnerHandle>,
    close_owners: semio_s_plugin_puzzle_jobs::WorkClosing<Puzzle3dAcceptSuggestionWorkOwners>,
}

impl Default for Puzzle3dAcceptSuggestionWork {
    fn default() -> Self {
        Self { close_owners: Default::default(),
            stage: Puzzle3dAcceptSuggestionStage::Target,
            object_cursor: 0,
            vortex_cursor: 0,
            attraction_cursor: 0,
            target_id: None,
            kind_index: None,
            object_id: None,
            candidate: None,
            vortices: Vec::with_capacity(PUZZLE3D_RELOCATE_VORTICES_PER_OBJECT),
            mutations: Vec::with_capacity(2),
            instance_owner: None,
        }
    }
}

impl Puzzle3dAcceptSuggestionWork {
    fn requested_target(command: &Puzzle3dCommand, transient: &window_ownership::Puzzle3dWindowTransient, interaction: &protocol::InteractionState) -> Option<String> {
        command
            .args()
            .and_then(|args| args.get("fullId"))
            .and_then(Value::as_str)
            .map(str::to_string)
            .or_else(|| transient.suggestion_menu.as_ref().map(|menu| menu.vortex_full_id.clone()).filter(|id| !id.is_empty()))
            .or_else(|| interaction.selection.get(PUZZLE3D_INTERACTION_DOMAIN).filter(|selection| selection.granularity == PUZZLE3D_GRANULARITY_VORTEX).and_then(|selection| selection.ids.first().cloned()))
    }

    /// 🪪️ Binds the admission's retained operation owner.
    fn bound(mut self, instance: &Puzzle3dInstanceBinding) -> Self {
        self.instance_owner = Some(instance.owner.clone());
        self
    }
}

impl<A: Puzzle3dEditor> semio_framework_plugin::retained_command::ArtifactCommandWork<EditorApp<A>> for Puzzle3dAcceptSuggestionWork {
    fn tool_id(&self) -> &'static str {
        "acceptSuggestion"
    }

    fn extent(&self, _command: &Puzzle3dCommand, snapshot: &Puzzle3dPlaySnapshot, _interaction: &protocol::InteractionState, _context: Option<&semio_framework_plugin::app::ArtifactOwnedToolJobContext<EditorApp<A>>>) -> Option<usize> {
        let document = snapshot.typed();
        document.meta.kind_catalogs.as_ref()?;
        let mut object_vortices = 0usize;
        for object in &document.objects {
            object_vortices = object_vortices.checked_add(object.vortices.len())?;
        }
        let target_scan_stage = object_vortices.checked_add(document.objects.len())?.checked_add(1)?;
        let kind_scan_stage = PUZZLE3D_RELOCATE_VORTICES_PER_OBJECT.checked_mul(2)?.checked_add(2)?;
        let items = target_scan_stage.checked_add(1)?.checked_add(kind_scan_stage)?.checked_add(document.attractions.len())?.checked_add(1)?.checked_add(3)?;
        (items <= semio_s_plugin_puzzle_retained::PUZZLE_COMMAND_WORK_ITEMS).then_some(items)
    }

    fn work_demands(&self, _input: &semio_framework_plugin::retained_command::ArtifactCommandInputs<'_, EditorApp<A>>, _maximum_copy_bytes: usize) -> Result<semio_framework_value::RetirementDemand, semio_framework_value::ValueError> {
        semio_s_plugin_puzzle_retained::step_demands::<EditorApp<A>>(size_of::<Self>())
    }

    fn step(&mut self, input: &semio_framework_plugin::retained_command::ArtifactCommandInputs<'_, EditorApp<A>>, _cx: &mut StepContext<'_>) -> Result<semio_framework_plugin::retained_command::ArtifactCommandWorkStep<EditorApp<A>>, Fault> {
        let semio_framework_plugin::retained_command::ArtifactCommandInputs { command, snapshot, config: _config, interaction, hover: _hover, context, .. } = *input;
        let view_state = context.and_then(|context| context.view_state.as_ref());
        let window_transient_snapshot = context.and_then(|context| context.window_transient.as_ref());
        let document = snapshot.typed();
        let transient = window_ownership::transient_from_snapshot(window_transient_snapshot);
        // 🧯️ The popup always retires, but a dismissed popup that placed nothing is
        // indistinguishable from a successful one unless the refusal itself speaks — `accept_suggestion_
        // closes_menu_even_when_placement_fails` is literally named after that silence.
        let Some(catalogs) = document.meta.kind_catalogs.as_ref() else {
            self.stage = Puzzle3dAcceptSuggestionStage::Complete;
            return Ok(semio_framework_plugin::retained_command::ArtifactCommandWorkStep::Complete(puzzle3d_notice_emit(view_state, |labels| labels.placement_unavailable.as_str())));
        };
        match self.stage {
            Puzzle3dAcceptSuggestionStage::Target => {
                let Some(requested) = Self::requested_target(command, &transient, interaction) else {
                    self.stage = Puzzle3dAcceptSuggestionStage::Complete;
                    return Ok(semio_framework_plugin::retained_command::ArtifactCommandWorkStep::Complete(puzzle3d_notice_emit(view_state, |labels| labels.placement_unavailable.as_str())));
                };
                let Some(object) = document.objects.get(self.object_cursor) else {
                    self.stage = Puzzle3dAcceptSuggestionStage::Complete;
                    return Ok(semio_framework_plugin::retained_command::ArtifactCommandWorkStep::Complete(puzzle3d_notice_emit(view_state, |labels| labels.placement_unavailable.as_str())));
                };
                if object.vortices.len() > PUZZLE3D_RELOCATE_VORTICES_PER_OBJECT {
                    return Err(Fault::from("puzzle3d-accept-target-vortex-capacity"));
                }
                let Some(vortex) = object.vortices.get(self.vortex_cursor) else {
                    self.object_cursor += 1;
                    self.vortex_cursor = 0;
                    return Ok(semio_s_plugin_puzzle_retained::puzzle_progress_step!("puzzle3d-accept-target-object", "Scanning target object", "Zielobjekt wird geprüft"));
                };
                self.vortex_cursor += 1;
                if puzzle3d_vortex_full_id(&object.id, &vortex.id) == requested {
                    self.target_id = Some(requested);
                    self.stage = Puzzle3dAcceptSuggestionStage::Candidate;
                }
                Ok(semio_s_plugin_puzzle_retained::puzzle_progress_step!("puzzle3d-accept-target-vortex", "Scanning target vortex", "Ziel-Vortex wird geprüft"))
            }
            Puzzle3dAcceptSuggestionStage::Candidate => {
                let target = self.target_id.clone().ok_or_else(|| Fault::from("puzzle3d-accept-target-owner"))?;
                let requested = command.args().and_then(|args| args.get("index")).and_then(Value::as_u64).unwrap_or(transient.brush_candidate_index as u64) as usize;
                let candidate = self.instance_owner.as_ref().and_then(|owner| {
                    owner
                        .with_mut::<Puzzle3dInstanceOperationOwner, _>(|owner| {
                            let link = &mut owner.brush_suggestions;
                            let free: Vec<_> = link.found(&target).into_iter().flat_map(|found| found.free()).cloned().collect();
                            link.close_menu();
                            Ok((!free.is_empty()).then(|| free[requested % free.len()].clone()))
                        })
                        .ok()
                        .flatten()
                });
                let Some((index, candidate)) = candidate.and_then(|candidate| catalogs.objects.iter().position(|kind| kind.id == candidate.object_kind_id).map(|index| (index, candidate))) else {
                    self.stage = Puzzle3dAcceptSuggestionStage::Complete;
                    return Ok(semio_framework_plugin::retained_command::ArtifactCommandWorkStep::Complete(puzzle3d_notice_emit(view_state, |labels| labels.placement_unavailable.as_str())));
                };
                let kind = &catalogs.objects[index];
                if kind.representations.len() > PUZZLE3D_RELOCATE_VORTICES_PER_OBJECT || kind.vortices.len() > PUZZLE3D_RELOCATE_VORTICES_PER_OBJECT {
                    return Err(Fault::from("puzzle3d-accept-candidate-capacity"));
                }
                use std::hash::{Hash, Hasher};
                let mut hasher = std::collections::hash_map::DefaultHasher::new();
                target.hash(&mut hasher);
                kind.id.hash(&mut hasher);
                candidate.source_vortex_index.hash(&mut hasher);
                document.objects.len().hash(&mut hasher);
                self.object_id = Some(format!("puzzle3d.suggestion.{:016x}", hasher.finish()));
                self.kind_index = Some(index);
                self.candidate = Some(candidate);
                self.stage = Puzzle3dAcceptSuggestionStage::Vortices;
                Ok(semio_s_plugin_puzzle_retained::puzzle_progress_step!("puzzle3d-accept-candidate", "Selecting suggestion candidate", "Vorschlagskandidat wird gewählt"))
            }
            Puzzle3dAcceptSuggestionStage::Vortices => {
                let kind = catalogs.objects.get(self.kind_index.ok_or_else(|| Fault::from("puzzle3d-accept-kind-owner"))?).ok_or_else(|| Fault::from("puzzle3d-accept-kind-cursor"))?;
                let Some(template) = kind.vortices.get(self.vortices.len()) else {
                    if self.vortices.is_empty() {
                        self.stage = Puzzle3dAcceptSuggestionStage::Complete;
                        return Ok(semio_framework_plugin::retained_command::ArtifactCommandWorkStep::Complete(puzzle3d_notice_emit(view_state, |labels| labels.placement_unavailable.as_str())));
                    }
                    self.stage = Puzzle3dAcceptSuggestionStage::ExistingAttractions;
                    return Ok(semio_s_plugin_puzzle_retained::puzzle_progress_step!("puzzle3d-accept-existing", "Checking target ownership", "Zielinhaberschaft wird geprüft"));
                };
                let object_id = self.object_id.as_ref().ok_or_else(|| Fault::from("puzzle3d-accept-object-owner"))?;
                let index = self.vortices.len();
                self.vortices.push(semio_s_artifact_puzzle_3d_model::Puzzle3dVortex {
                    id: format!("{object_id}:v{index}"),
                    label: None,
                    vortex_kind: template.vortex_kind.clone(),
                    position: template.point,
                    direction: Some(template.direction),
                    radius: template.radius,
                    hidden: false,
                    locked: false,
                });
                Ok(semio_s_plugin_puzzle_retained::puzzle_progress_step!("puzzle3d-accept-vortex", "Building one suggested vortex", "Ein Vorschlags-Vortex wird aufgebaut"))
            }
            Puzzle3dAcceptSuggestionStage::ExistingAttractions => {
                let target = self.target_id.as_ref().ok_or_else(|| Fault::from("puzzle3d-accept-target-owner"))?;
                let Some(attraction) = document.attractions.get(self.attraction_cursor) else {
                    self.stage = Puzzle3dAcceptSuggestionStage::PublishObject;
                    return Ok(semio_s_plugin_puzzle_retained::puzzle_progress_step!("puzzle3d-accept-publish-object", "Preparing suggested object", "Vorschlagsobjekt wird vorbereitet"));
                };
                self.attraction_cursor += 1;
                if attraction.attracting == *target || attraction.attracted == *target {
                    self.stage = Puzzle3dAcceptSuggestionStage::Complete;
                    return Ok(semio_framework_plugin::retained_command::ArtifactCommandWorkStep::Complete(puzzle3d_notice_emit(view_state, |labels| labels.placement_occupied.as_str())));
                }
                Ok(semio_s_plugin_puzzle_retained::puzzle_progress_step!("puzzle3d-accept-existing", "Scanning existing attraction", "Bestehende Anziehung wird geprüft"))
            }
            Puzzle3dAcceptSuggestionStage::PublishObject => {
                let kind = catalogs.objects.get(self.kind_index.ok_or_else(|| Fault::from("puzzle3d-accept-kind-owner"))?).ok_or_else(|| Fault::from("puzzle3d-accept-kind-cursor"))?;
                let candidate = self.candidate.as_ref().ok_or_else(|| Fault::from("puzzle3d-accept-candidate-owner"))?;
                let scene_snapshot = puzzle3d_scene_snapshot_from_document(document);
                let object = semio_s_artifact_puzzle_3d_model::Puzzle3dObject {
                    id: self.object_id.clone().ok_or_else(|| Fault::from("puzzle3d-accept-object-owner"))?,
                    label: Some(puzzle3d_next_object_label(&scene_snapshot.objects, &scene_snapshot, &kind.id)),
                    object_kind: Some(kind.id.clone()),
                    anchor: Default::default(),
                    origin: candidate.origin,
                    orientation: Some(candidate.orientation),
                    scale: candidate.scale.clone().and_then(|value| FromValue::from_value(value).ok()),
                    mesh_url: Some(candidate.mesh_url.clone()),
                    vortices: std::mem::take(&mut self.vortices),
                    hidden: false,
                    locked: false,
                };
                self.mutations.push(semio_s_artifact_puzzle_3d_mutations_object::create_object::mutation::create_object(object, None));
                self.stage = Puzzle3dAcceptSuggestionStage::PublishAttraction;
                Ok(semio_s_plugin_puzzle_retained::puzzle_progress_step!("puzzle3d-accept-publish-object", "Transferring suggested object", "Vorschlagsobjekt wird übertragen"))
            }
            Puzzle3dAcceptSuggestionStage::PublishAttraction => {
                let target = self.target_id.take().ok_or_else(|| Fault::from("puzzle3d-accept-target-owner"))?;
                let object_id = self.object_id.clone().ok_or_else(|| Fault::from("puzzle3d-accept-object-owner"))?;
                let source = format!("{object_id}:v{}", self.candidate.take().ok_or_else(|| Fault::from("puzzle3d-accept-candidate-owner"))?.source_vortex_index);
                self.mutations.push(semio_s_artifact_puzzle_3d_mutations_vortex::connect_vortices::mutation::connect_vortices(format!("attraction-{target}-{source}"), target, source, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, None));
                self.stage = Puzzle3dAcceptSuggestionStage::PublishResult;
                Ok(semio_s_plugin_puzzle_retained::puzzle_progress_step!("puzzle3d-accept-publish-attraction", "Transferring suggested attraction", "Vorschlagsanziehung wird übertragen"))
            }
            Puzzle3dAcceptSuggestionStage::PublishResult => {
                self.stage = Puzzle3dAcceptSuggestionStage::Complete;
                let placed = self.object_id.take().ok_or_else(|| Fault::from("puzzle3d-accept-object-owner"))?;
                Ok(semio_framework_plugin::retained_command::ArtifactCommandWorkStep::Complete(Emit {
                    artifact_mutations: std::mem::take(&mut self.mutations),
                    config_mutations: Vec::new(),
                    ui_scope: puzzle3d_scope(puzzle3d_command_scope_class("acceptSuggestion")),
                    interaction_writes: vec![InteractionWrite::replace(PUZZLE3D_INTERACTION_DOMAIN, PUZZLE3D_GRANULARITY_OBJECT, [placed])],
                    ..Default::default()
                }))
            }
            Puzzle3dAcceptSuggestionStage::Complete => Err(Fault::from("puzzle3d-accept-complete-repolled")),
            Puzzle3dAcceptSuggestionStage::Closing => Err(Fault::from("puzzle3d-accept-closing")),
        }
    }

    fn begin_close(&mut self) {
        self.stage = Puzzle3dAcceptSuggestionStage::Closing;
        self.close_owners.stage(Puzzle3dAcceptSuggestionWorkOwners { vortices: std::mem::take(&mut self.vortices), mutations: std::mem::take(&mut self.mutations), target_id: std::mem::take(&mut self.target_id), object_id: std::mem::take(&mut self.object_id), candidate: std::mem::take(&mut self.candidate), instance_owner: std::mem::take(&mut self.instance_owner) });
    }

    fn close_step(&mut self, grant: semio_framework_value::retained_clone::RetainedCloneGrant) -> InteractiveJobCloseStep {
        self.close_owners.close_step(grant)
    }

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
        self.stage == Puzzle3dAcceptSuggestionStage::Closing && self.close_owners.is_empty()
    }

    fn terminal_frame_release_bytes(&self) -> Option<usize> {
        Some(size_of::<Self>())
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Puzzle3dPrecomputeCommandStage {
    Decode,
    Objects,
    Vortices,
    Attractions,
    CatalogObjects,
    CatalogVortices,
    Positions,
    Indices,
    PrologueScene,
    PrologueSync,
    Publish,
    Complete,
    Closing,
}

/// ♻️ The owners one `Puzzle3dPrecomputeCommandWork` still holds when its job closes, retired as one controlled bundle.
#[derive(semio_framework_value::RetireOwned)]
struct Puzzle3dPrecomputeWorkOwners {
    prologue: Puzzle3dActionPrologue,
    session: Option<(u32, String)>,
    instance_owner: Option<ArtifactInstanceOperationOwnerHandle>,
}

pub struct Puzzle3dPrecomputeCommandWork {
    tool_id: &'static str,
    stage: Puzzle3dPrecomputeCommandStage,
    object_cursor: usize,
    child_cursor: usize,
    attraction_cursor: usize,
    catalog_object_cursor: usize,
    catalog_vortex_cursor: usize,
    payload_cursor: usize,
    requested_count: u32,
    delta: isize,
    candidate_count: usize,
    processed_units: usize,
    /// 🪪️ Session key of the admission that built this work — bound by `build_tool_job` on every
    /// admission and every worker-hop resume, so the precompute-gated actions reach the same slot.
    session: Option<(u32, String)>,
    prologue: Puzzle3dActionPrologue,
    instance_owner: Option<ArtifactInstanceOperationOwnerHandle>,
    close_owners: semio_s_plugin_puzzle_jobs::WorkClosing<Puzzle3dPrecomputeWorkOwners>,
}

impl Puzzle3dPrecomputeCommandWork {
    /// 🪪️ Binds the admission's instance identity and retained operation owner.
    fn bound(mut self, instance: &Puzzle3dInstanceBinding) -> Self {
        self.session = Some((instance.app_instance_id, instance.parent_document_id.clone()));
        self.instance_owner = Some(instance.owner.clone());
        self
    }

    fn session(&self) -> Option<(u32, Option<String>)> {
        self.session.as_ref().map(|(app_instance_id, parent_artifact_id)| (*app_instance_id, Some(parent_artifact_id.clone())))
    }

    fn new(tool_id: &'static str) -> Self {
        Self {
            tool_id,
            close_owners: Default::default(),
            stage: Puzzle3dPrecomputeCommandStage::Decode,
            object_cursor: 0,
            child_cursor: 0,
            attraction_cursor: 0,
            catalog_object_cursor: 0,
            catalog_vortex_cursor: 0,
            payload_cursor: 0,
            requested_count: 0,
            delta: if tool_id == "cycleBrushCandidateBack" { -1 } else { 1 },
            candidate_count: 0,
            processed_units: 0,
            session: None,
            prologue: Puzzle3dActionPrologue::default(),
            instance_owner: None,
        }
    }

    /// 🔤️ Validates one bounded slice of a `registerBrushMesh` page's base64 payload — the alphabet and,
    /// once the stream ends, its four-character grouping. `payload_cursor` returns to zero exactly when
    /// the stream is fully scanned, which is what advances the stage.
    fn scan_mesh_page(&mut self, command: &Puzzle3dCommand, key: &str) -> bool {
        let payload = command.args().and_then(|args| args.get(key)).and_then(Value::as_str).unwrap_or_default().as_bytes();
        let start = self.payload_cursor.min(payload.len());
        let end = payload.len().min(start.saturating_add(PUZZLE3D_MESH_PAGE_SCAN_CHARS));
        let admissible = payload[start..end].iter().all(|byte| byte.is_ascii_alphanumeric() || matches!(byte, b'+' | b'/' | b'='));
        self.payload_cursor = if end >= payload.len() { 0 } else { end };
        admissible && payload.len().is_multiple_of(4)
    }
}

/// 🔤️ Base64 characters one interactive step of a `registerBrushMesh` page validates. A whole page is
/// at most [`crate::editor::puzzle3d::precompute::PUZZLE3D_MESH_PAGE_BASE64_CHARS`] characters, so a
/// page costs at most eleven steps per stream and no step approaches the lane's own budget.
const PUZZLE3D_MESH_PAGE_SCAN_CHARS: usize = 512;

impl<A: Puzzle3dEditor> semio_framework_plugin::retained_command::ArtifactCommandWork<EditorApp<A>> for Puzzle3dPrecomputeCommandWork {
    fn tool_id(&self) -> &'static str {
        self.tool_id
    }

    /// 📏️ `registerBrushMesh` carries one page of a mesh upload run as two base64 payload strings, never
    /// as JSON number arrays — a whole document-scale mesh is 64 KB of them and the shared retained wire
    /// admits 8 192 bytes. The extent a page claims is the characters it must validate, so a page that
    /// declares more than [`crate::editor::puzzle3d::precompute::PUZZLE3D_MESH_PAGE_BASE64_CHARS`] per
    /// stream is refused before a single character is read.
    fn extent(&self, command: &Puzzle3dCommand, _snapshot: &Puzzle3dPlaySnapshot, _interaction: &protocol::InteractionState, _context: Option<&semio_framework_plugin::app::ArtifactOwnedToolJobContext<EditorApp<A>>>) -> Option<usize> {
        let payload = |key: &str| command.args().and_then(|args| args.get(key)).and_then(Value::as_str).map_or(0, str::len);
        let (positions, indices) = (payload("positionsB64"), payload("indicesB64"));
        let maximum = semio_s_artifact_puzzle_3d_precompute::PUZZLE3D_MESH_PAGE_BASE64_CHARS;
        let items = positions.max(indices).div_ceil(PUZZLE3D_MESH_PAGE_SCAN_CHARS).checked_mul(2)?.checked_add(1)?;
        (positions <= maximum && indices <= maximum && items <= semio_s_plugin_puzzle_retained::PUZZLE_COMMAND_WORK_ITEMS).then_some(items)
    }

    fn work_demands(&self, _input: &semio_framework_plugin::retained_command::ArtifactCommandInputs<'_, EditorApp<A>>, _maximum_copy_bytes: usize) -> Result<semio_framework_value::RetirementDemand, semio_framework_value::ValueError> {
        semio_s_plugin_puzzle_retained::step_demands::<EditorApp<A>>(size_of::<Self>())
    }

    fn step(&mut self, input: &semio_framework_plugin::retained_command::ArtifactCommandInputs<'_, EditorApp<A>>, _cx: &mut StepContext<'_>) -> Result<semio_framework_plugin::retained_command::ArtifactCommandWorkStep<EditorApp<A>>, Fault> {
        let semio_framework_plugin::retained_command::ArtifactCommandInputs { command, snapshot, config, interaction, hover: hover, context, .. } = *input;
        let view_state = context.and_then(|context| context.view_state.as_ref());
        let window_config_snapshot = context.and_then(|context| context.window_config.as_ref());
        let window_transient_snapshot = context.and_then(|context| context.window_transient.as_ref());
        if self.processed_units >= semio_s_plugin_puzzle_retained::PUZZLE_COMMAND_WORK_ITEMS {
            return Err(Fault::from("puzzle3d-precompute-work-capacity"));
        }
        self.processed_units += 1;
        let document = snapshot.typed();
        match self.stage {
            Puzzle3dPrecomputeCommandStage::Decode => {
                self.requested_count = set_fill_count::parse_count(command.args());
                self.delta = command.args().and_then(|args| args.get("delta")).and_then(Value::as_i64).map_or(self.delta, |value| value.clamp(isize::MIN as i64, isize::MAX as i64) as isize);
                self.stage = match self.tool_id {
                    "registerBrushMesh" => Puzzle3dPrecomputeCommandStage::Positions,
                    "setFillCount" => Puzzle3dPrecomputeCommandStage::Publish,
                    _ => Puzzle3dPrecomputeCommandStage::Objects,
                };
                Ok(semio_s_plugin_puzzle_retained::puzzle_progress_step!("puzzle3d-precompute-decode", "Reading precompute command", "Vorberechnungsbefehl wird gelesen"))
            }
            Puzzle3dPrecomputeCommandStage::Objects => {
                let Some(object) = document.objects.get(self.object_cursor) else {
                    self.stage = Puzzle3dPrecomputeCommandStage::Attractions;
                    return Ok(semio_s_plugin_puzzle_retained::puzzle_progress_step!("puzzle3d-precompute-attractions", "Scanning attraction owner", "Anziehungsinhaber wird geprüft"));
                };
                if object.vortices.len() > PUZZLE3D_RELOCATE_VORTICES_PER_OBJECT {
                    return Err(Fault::from("puzzle3d-precompute-vortex-capacity"));
                }
                self.stage = Puzzle3dPrecomputeCommandStage::Vortices;
                Ok(semio_s_plugin_puzzle_retained::puzzle_progress_step!("puzzle3d-precompute-object", "Scanning object owner", "Objektinhaber wird geprüft"))
            }
            Puzzle3dPrecomputeCommandStage::Vortices => {
                let object = document.objects.get(self.object_cursor).ok_or_else(|| Fault::from("puzzle3d-precompute-object-cursor"))?;
                if object.vortices.get(self.child_cursor).is_some() {
                    self.child_cursor += 1;
                    return Ok(semio_s_plugin_puzzle_retained::puzzle_progress_step!("puzzle3d-precompute-vortex", "Scanning one vortex owner", "Ein Vortexinhaber wird geprüft"));
                }
                self.object_cursor += 1;
                self.child_cursor = 0;
                self.stage = Puzzle3dPrecomputeCommandStage::Objects;
                Ok(semio_s_plugin_puzzle_retained::puzzle_progress_step!("puzzle3d-precompute-object", "Advancing object cursor", "Objektzeiger wird fortgesetzt"))
            }
            Puzzle3dPrecomputeCommandStage::Attractions => {
                if document.attractions.get(self.attraction_cursor).is_some() {
                    self.attraction_cursor += 1;
                    return Ok(semio_s_plugin_puzzle_retained::puzzle_progress_step!("puzzle3d-precompute-attraction", "Scanning one attraction owner", "Ein Anziehungsinhaber wird geprüft"));
                }
                self.stage = Puzzle3dPrecomputeCommandStage::CatalogObjects;
                Ok(semio_s_plugin_puzzle_retained::puzzle_progress_step!("puzzle3d-precompute-catalog-object", "Scanning object kind owner", "Objektartinhaber wird geprüft"))
            }
            Puzzle3dPrecomputeCommandStage::CatalogObjects => {
                let entries = document.meta.kind_catalogs.as_ref().map(|catalogs| catalogs.objects.as_slice()).unwrap_or_default();
                if entries.get(self.catalog_object_cursor).is_some() {
                    self.catalog_object_cursor += 1;
                    self.candidate_count += 1;
                    return Ok(semio_s_plugin_puzzle_retained::puzzle_progress_step!("puzzle3d-precompute-catalog-object", "Scanning one object kind", "Eine Objektart wird geprüft"));
                }
                self.stage = Puzzle3dPrecomputeCommandStage::CatalogVortices;
                Ok(semio_s_plugin_puzzle_retained::puzzle_progress_step!("puzzle3d-precompute-catalog-vortex", "Scanning vortex kind owner", "Vortexartinhaber wird geprüft"))
            }
            Puzzle3dPrecomputeCommandStage::CatalogVortices => {
                let entries = document.meta.kind_catalogs.as_ref().map(|catalogs| catalogs.vortices.as_slice()).unwrap_or_default();
                if entries.get(self.catalog_vortex_cursor).is_some() {
                    self.catalog_vortex_cursor += 1;
                    return Ok(semio_s_plugin_puzzle_retained::puzzle_progress_step!("puzzle3d-precompute-catalog-vortex", "Scanning one vortex kind", "Eine Vortexart wird geprüft"));
                }
                self.stage = Puzzle3dPrecomputeCommandStage::PrologueScene;
                Ok(semio_s_plugin_puzzle_retained::puzzle_progress_step!("puzzle3d-precompute-transfer", "Transferring precompute census", "Vorberechnungszensus wird übertragen"))
            }
            Puzzle3dPrecomputeCommandStage::Positions => {
                if !self.scan_mesh_page(command, "positionsB64") {
                    return Err(Fault::from("puzzle3d-register-mesh-position-malformed"));
                }
                if self.payload_cursor > 0 {
                    return Ok(semio_s_plugin_puzzle_retained::puzzle_progress_step!("puzzle3d-register-mesh-position", "Reading one mesh position page", "Eine Mesh-Positionsseite wird gelesen"));
                }
                self.stage = Puzzle3dPrecomputeCommandStage::Indices;
                Ok(semio_s_plugin_puzzle_retained::puzzle_progress_step!("puzzle3d-register-mesh-index", "Reading mesh indices", "Mesh-Indizes werden gelesen"))
            }
            Puzzle3dPrecomputeCommandStage::Indices => {
                if !self.scan_mesh_page(command, "indicesB64") {
                    return Err(Fault::from("puzzle3d-register-mesh-index-malformed"));
                }
                if self.payload_cursor > 0 {
                    return Ok(semio_s_plugin_puzzle_retained::puzzle_progress_step!("puzzle3d-register-mesh-index", "Reading one mesh index page", "Eine Mesh-Indexseite wird gelesen"));
                }
                self.stage = Puzzle3dPrecomputeCommandStage::PrologueScene;
                Ok(semio_s_plugin_puzzle_retained::puzzle_progress_step!("puzzle3d-register-mesh-transfer", "Transferring validated mesh owner", "Geprüfter Mesh-Inhaber wird übertragen"))
            }
            Puzzle3dPrecomputeCommandStage::PrologueScene => {
                let view = view_state.ok_or_else(|| Fault::from("puzzle3d-precompute-window-context-required"))?;
                let window_config = window_ownership::config_from_snapshot(window_config_snapshot);
                let window_transient = window_ownership::transient_from_snapshot(window_transient_snapshot);
                let runtime = window_ownership::runtime(config, &window_config, &window_transient, Some(view));
                let window_id = puzzle3d_addressed_window_id(Some(view), None, command.window_id(), &runtime.window_ids);
                self.prologue.scene_step(snapshot, &runtime, Some(view), Some(window_id));
                self.stage = Puzzle3dPrecomputeCommandStage::PrologueSync;
                Ok(semio_s_plugin_puzzle_retained::puzzle_progress_step!("puzzle3d-action-scene", "Reading the artifact", "Artefakt wird gelesen"))
            }
            Puzzle3dPrecomputeCommandStage::PrologueSync => {
                let view = view_state.ok_or_else(|| Fault::from("puzzle3d-precompute-window-context-required"))?;
                let window_config = window_ownership::config_from_snapshot(window_config_snapshot);
                let window_transient = window_ownership::transient_from_snapshot(window_transient_snapshot);
                let runtime = window_ownership::runtime(config, &window_config, &window_transient, Some(view));
                let window_id = puzzle3d_addressed_window_id(Some(view), None, command.window_id(), &runtime.window_ids);
                let session = self.session();
                let owed = with_puzzle3d_session_for(session, &runtime, |app| self.prologue.sync_step(app, command.action_id(), &runtime, Some(view), Some(window_id)));
                if !owed {
                    self.stage = Puzzle3dPrecomputeCommandStage::Publish;
                }
                Ok(semio_s_plugin_puzzle_retained::puzzle_progress_step!("puzzle3d-action-sync", "Preparing the placement session", "Platzierungssitzung wird vorbereitet"))
            }
            Puzzle3dPrecomputeCommandStage::Publish => {
                if self.tool_id == "setFillCount" {
                    let config_mutations = (self.requested_count != config.fill_count).then_some(Puzzle3dConfigMutation::SetFillCount(Puzzle3dConfigSetFillCount{ count: self.requested_count })).into_iter().collect();
                    self.stage = Puzzle3dPrecomputeCommandStage::Complete;
                    return Ok(semio_framework_plugin::retained_command::ArtifactCommandWorkStep::Complete(Emit { config_mutations, ui_scope: puzzle3d_scope(puzzle3d_command_scope_class("setFillCount")), ..Default::default() }));
                }
                let view = view_state.ok_or_else(|| Fault::from("puzzle3d-precompute-window-context-required"))?;
                let window_config = window_ownership::config_from_snapshot(window_config_snapshot);
                let window_transient = window_ownership::transient_from_snapshot(window_transient_snapshot);
                let runtime = window_ownership::runtime(config, &window_config, &window_transient, Some(view));
                let snapshot_interaction = Puzzle3dInteractionSnapshot::from_state(interaction, hover);
                let window_id = puzzle3d_addressed_window_id(Some(view), None, command.window_id(), &runtime.window_ids);
                let session = self.session();
                let instance_owner = self.instance_owner.as_ref();
                let (emit, ephemeral) = with_puzzle3d_session_for(session, &runtime, |app| self.prologue.dispatch_step(app, instance_owner, "", command, Some(window_id), &runtime, Some(view), &snapshot_interaction));
                self.stage = Puzzle3dPrecomputeCommandStage::Complete;
                Ok(semio_framework_plugin::retained_command::ArtifactCommandWorkStep::CompleteWithEphemeral { emit, ephemeral })
            }
            Puzzle3dPrecomputeCommandStage::Complete => Err(Fault::from("puzzle3d-precompute-complete-repolled")),
            Puzzle3dPrecomputeCommandStage::Closing => Err(Fault::from("puzzle3d-precompute-closing")),
        }
    }

    fn begin_close(&mut self) {
        self.stage = Puzzle3dPrecomputeCommandStage::Closing;
        self.close_owners.stage(Puzzle3dPrecomputeWorkOwners { prologue: std::mem::take(&mut self.prologue), session: self.session.take(), instance_owner: self.instance_owner.take() });
    }

    fn close_step(&mut self, grant: semio_framework_value::retained_clone::RetainedCloneGrant) -> InteractiveJobCloseStep {
        self.close_owners.close_step(grant)
    }

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
        self.stage == Puzzle3dPrecomputeCommandStage::Closing && self.close_owners.is_empty()
    }

    fn terminal_frame_release_bytes(&self) -> Option<usize> {
        Some(size_of::<Self>())
    }
}
