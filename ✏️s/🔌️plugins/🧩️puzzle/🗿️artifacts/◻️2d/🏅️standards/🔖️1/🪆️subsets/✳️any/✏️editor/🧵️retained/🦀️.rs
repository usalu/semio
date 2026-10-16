//! 🧵️ Retained work of the puzzle 2d editor: the board-event batch limits, window and export work, the dispatch emit, the active-example reduce and the force-layout and redraw-handles work. See [`super`].

use super::*;

//#region 🔖️PlayApp
//#region 🧵️RetainedCommands
/// 🖌️ Upper bound on the events one board flush may carry into a single retained step. The browser
/// flushes a handful of events per interaction (`BOARD_FLUSH_NOW_EVENT_NAMES` in `🖥️Board2dHost`), so
/// one bounded step covers a real interaction; an oversized batch is refused rather than silently
/// truncated.
const PUZZLE2D_BOARD_EVENT_BATCH_LIMIT: usize = 256;

/// 🚚️ Whether one board-event batch carries a gesture record — a drag drop is what arms the proximity
/// auto-connect, so a batch carrying one prices the gesture's whole [`PUZZLE2D_PROXIMITY_GESTURE_MAX`] edge budget.
fn puzzle2d_batch_drops_a_node(events: &[Value]) -> bool {
    events.iter().any(|event| event.get("name").and_then(Value::as_str) == Some("gesture"))
}

pub(super) fn puzzle2d_board_events_extent(command: &Puzzle2dCommand, _snapshot: &Puzzle2dPlaySnapshot, _interaction: &protocol::InteractionState) -> Option<usize> {
    if command.action_id() != "applyBoardEvents" {
        return None;
    }
    let events = command.args().and_then(|args| args.get("eventsJson")).and_then(Value::as_str).unwrap_or("[]");
    let parsed: Value = semio_framework_pack_json::parse(events,semio_framework_pack_json::JsonMemberPolicy::Reject).ok()?;
    let rows = parsed.as_array().map_or(&[][..], Vec::as_slice);
    if rows.len() > PUZZLE2D_BOARD_EVENT_BATCH_LIMIT {
        return None;
    }
    let connects = if puzzle2d_batch_drops_a_node(rows) { PUZZLE2D_PROXIMITY_GESTURE_MAX } else { 0 };
    Some(rows.len().saturating_add(connects).max(1))
}

pub(super) struct Puzzle2dWindowCommandWork {
    pub(super) tool_id: &'static str,
    pub(super) extent: crate::retained_command::PuzzleCommandExtent<EditorApp<Puzzle2dPlayApp>>,
    pub(super) authoring_seed: String,
    pub(super) base_revision: String,
    consumed: bool,
}

impl Puzzle2dWindowCommandWork {
    pub(super) fn new(tool_id: &'static str, extent: crate::retained_command::PuzzleCommandExtent<EditorApp<Puzzle2dPlayApp>>, authoring_seed: String, base_revision: String) -> Self {
        Self { tool_id, extent, authoring_seed, base_revision, consumed: false }
    }
}

/// 📐️ A canonical document revision as lowercase hex — the identity a persisted select-tool gesture pins its base by.
pub(super) fn puzzle2d_revision_hex(revision: &[u8; 32]) -> String {
    revision.iter().map(|byte| format!("{byte:02x}")).collect()
}

impl semio_framework_plugin::retained_command::ArtifactCommandWork<EditorApp<Puzzle2dPlayApp>> for Puzzle2dWindowCommandWork {
    fn tool_id(&self) -> &'static str {
        self.tool_id
    }

    fn extent(&self, command: &Puzzle2dCommand, snapshot: &Puzzle2dPlaySnapshot, interaction: &protocol::InteractionState, _context: Option<&semio_framework_plugin::app::ArtifactOwnedToolJobContext<EditorApp<Puzzle2dPlayApp>>>) -> Option<usize> {
        (self.extent)(command, snapshot, interaction)
    }

    fn work_demands(&self, _input: &semio_framework_plugin::retained_command::ArtifactCommandInputs<'_, EditorApp<Puzzle2dPlayApp>>, _maximum_copy_bytes: usize) -> Result<semio_framework_value::RetirementDemand, semio_framework_value::ValueError> {
        crate::retained_command::step_demands::<EditorApp<Puzzle2dPlayApp>>(size_of::<Self>())
    }

    fn step(&mut self, input: &semio_framework_plugin::retained_command::ArtifactCommandInputs<'_, EditorApp<Puzzle2dPlayApp>>, _cx: &mut StepContext<'_>) -> Result<semio_framework_plugin::retained_command::ArtifactCommandWorkStep<EditorApp<Puzzle2dPlayApp>>, Fault> {
        let semio_framework_plugin::retained_command::ArtifactCommandInputs { command, snapshot, config, interaction, hover: _hover, context, .. } = *input;
        if self.consumed {
            return Err(Fault::from("puzzle2d-window-work-repeated"));
        }
        let view_state = context.and_then(|context| context.view_state.as_ref());
        let window_config_snapshot = context.and_then(|context| context.window_config.as_ref());
        let window_config = window::config_from_snapshot_or_document(window_config_snapshot, snapshot.value());
        let window_transient = window::transient_from_snapshot(context.and_then(|context| context.window_transient.as_ref()));
        let window_kind = window_config_snapshot.map(semio_framework_plugin::WindowConfigSnapshot::window_kind_id).or_else(|| view_state.and_then(window::kind_for_view)).unwrap_or(crate::editor::puzzle2d::config::PUZZLE2D_OVERVIEW_WINDOW_KIND_ID);
        let selection = interaction.selection.get(PUZZLE2D_INTERACTION_DOMAIN).cloned().unwrap_or_default();
        let detached = semio_framework_plugin::app::GestureSlot::detached();
        let gesture = context.map_or(&detached, |context| context.gesture());
        let (emit, ephemeral) = puzzle2d_dispatch_emit(command, snapshot, config, &window_config, &window_transient, window_kind, view_state, puzzle2d_active_utility(view_state), &selection, &self.authoring_seed, &self.base_revision, gesture, None)?;
        self.consumed = true;
        Ok(semio_framework_plugin::retained_command::ArtifactCommandWorkStep::CompleteWithEphemeral { emit, ephemeral })
    }
}

/// 📤 `exportSnapshot` reads the document and publishes a download; it owns no mutation, so it resolves
/// from the snapshot and picks its lane by payload size: one inline effect under the guest's contiguous
/// request ceiling, the framework's segmented-download lane above it, a notice above what one segmented
/// download may carry.
#[derive(Default)]
pub(super) struct Puzzle2dExportWork {
    consumed: bool,
}

impl semio_framework_plugin::retained_command::ArtifactCommandWork<EditorApp<Puzzle2dPlayApp>> for Puzzle2dExportWork {
    fn tool_id(&self) -> &'static str {
        "exportSnapshot"
    }

    fn extent(&self, _command: &Puzzle2dCommand, _snapshot: &Puzzle2dPlaySnapshot, _interaction: &protocol::InteractionState, _context: Option<&semio_framework_plugin::app::ArtifactOwnedToolJobContext<EditorApp<Puzzle2dPlayApp>>>) -> Option<usize> {
        Some(1)
    }

    fn work_demands(&self, _input: &semio_framework_plugin::retained_command::ArtifactCommandInputs<'_, EditorApp<Puzzle2dPlayApp>>, _maximum_copy_bytes: usize) -> Result<semio_framework_value::RetirementDemand, semio_framework_value::ValueError> {
        crate::retained_command::step_demands::<EditorApp<Puzzle2dPlayApp>>(size_of::<Self>())
    }

    fn step(&mut self, input: &semio_framework_plugin::retained_command::ArtifactCommandInputs<'_, EditorApp<Puzzle2dPlayApp>>, _cx: &mut StepContext<'_>) -> Result<semio_framework_plugin::retained_command::ArtifactCommandWorkStep<EditorApp<Puzzle2dPlayApp>>, Fault> {
        let semio_framework_plugin::retained_command::ArtifactCommandInputs { command: _command, snapshot, config: _config, interaction: _interaction, hover: _hover, context: _context, .. } = *input;
        if self.consumed {
            return Err(Fault::from("puzzle2d-export-work-repeated"));
        }
        self.consumed = true;
        Ok(match export_snapshot::puzzle2d_export_publication(snapshot.value())? {
            export_snapshot::Puzzle2dExportPublication::Inline(effect) => semio_framework_plugin::retained_command::ArtifactCommandWorkStep::Complete(Emit { effects: vec![effect], ui_scope: UiDirtyScope::None, ..Default::default() }),
            export_snapshot::Puzzle2dExportPublication::Segmented(download) => semio_framework_plugin::retained_command::ArtifactCommandWorkStep::CompleteDownload { download: download, ephemeral: Default::default() },
            export_snapshot::Puzzle2dExportPublication::Refused(message) => semio_framework_plugin::retained_command::ArtifactCommandWorkStep::Complete(Emit { effects: vec![Effect::Notify { message }], ui_scope: UiDirtyScope::None, ..Default::default() }),
        })
    }
}

/// 🎬️ THE dispatch pipeline — the one implementation both [`ArtifactEditor::handle`]'s batch path and
/// every retained generic reduce run: rebuild the scene and a fresh board host from
/// `(command, document, config, selection)`, run the `🎮️commands/*` arm, replay the host's owned
/// events, then derive the granular document delta and the config snapshot. A selection transform never
/// touches the scene: its arm commits the select tool's ONE transaction, whose yielded parametric
/// mutations ride this emit stamped with the transaction ref. `operation` is the committed public
/// authority a mounted continuation carries and is simply `None` for a retained work, which never sees an
/// `ArtifactView`; `authoring_seed` is the admission's seed every tool transaction id is minted from.
pub(super) fn puzzle2d_dispatch_emit(
    command: &Puzzle2dCommand,
    document: &Puzzle2dPlaySnapshot,
    config: &Puzzle2dConfig,
    window_config: &Puzzle2dWindowConfig,
    window_transient: &Puzzle2dWindowTransient,
    window_kind: &str,
    view_state: Option<&semio_framework_plugin::ViewModel>,
    active_utility: &str,
    selection: &protocol::DomainSelection,
    authoring_seed: &str,
    base_revision: &str,
    gesture: &semio_framework_plugin::app::GestureSlot<Puzzle2dMutation>,
    operation: Option<semio_framework_plugin::AppOperationContext>,
) -> Result<(Emit<Puzzle2dMutation, Puzzle2dConfigMutation>, EphemeralEmit<EditorApp<Puzzle2dPlayApp>>), Fault> {
    let (action, args, window_id) = (command.action_id(), command.args(), command.window_id());
    let before = document.value();
    let active_utility = active_utility.to_string();
    let runtime = window::runtime(config, window_config, window_transient, Some(window_kind));
    let interaction = Puzzle2dInteractionSnapshot { granularity: selection.granularity.clone(), selected: selection.ids.clone(), ..Default::default() };
    let mut scene = Puzzle2dPlayApp::scene_with(before.clone(), runtime, &active_utility, interaction);
    // 🐚️ ArtifactApp::handle is pure (no &self) — rebuild a fresh BoardHost from the document
    // each call. The previous last_synced_fixture cache lived on &self and cannot return.
    let host = RefCell::new(puzzle_board_host());
    {
        let mut host_mut = host.borrow_mut();
        if action != "applyBoardEvents" {
            sync_host_fixture_content(&mut host_mut, &scene);
            let _ = drain_board_events_json(&mut host_mut);
        }
        sync_host_runtime_state(&mut host_mut, &scene, &selection.ids);
    }
    let mut effects: Vec<Effect> = Vec::new();
    let mut recorder = Puzzle2dRecorder::new(document);
    let mut artifact_mutations = Vec::new();
    let mut interaction_writes = Vec::new();
    let mut transaction = None;
    // 🐢️ Default to Full (safe: every unrecognized/rare action re-renders everything); the
    // narrow-tier arms below override it to the smallest scope that actually covers what they touch.
    let mut ui_scope = UiDirtyScope::Full;
    {
        let labels = puzzle2d_labels(view_state.ok_or_else(|| Fault::from("puzzle2d action requires the caller's explicit presentation axes"))?);
        let ctx = &mut Puzzle2dActionCtx {
            host: &host,
            scene: &mut scene,
            window_id,
            window_kind,
            active_utility,
            selection,
            effects: &mut effects,
            recorder: &mut recorder,
            artifact_mutations: &mut artifact_mutations,
            interaction_writes: &mut interaction_writes,
            ui_scope: &mut ui_scope,
            labels,
            operation,
            base: document,
            authoring_seed,
            base_revision,
            transaction: &mut transaction,
            gesture,
        };
        match action {
            "selectSameKind" => select_same_kind::select_same_kind(ctx),
            "createEdge" => create_edge::create_edge(ctx, args),
            "deleteEdge" => delete_edge::delete_edge(ctx, args),
            "proximityConnect" => proximity_connect::proximity_connect(ctx, args),
            "setProximityRadius" => set_proximity_radius::set_proximity_radius(ctx, args),
            "deleteSelection" => delete_selection::delete_selection(ctx),
            "duplicateSelection" => duplicate_selection::duplicate_selection(ctx),
            "setSelectionFlag" => set_selection_flag::set_selection_flag(ctx, args),
            "setSelectionHidden" => set_selection_flag::set_selection_flag_value(ctx, args, "hidden"),
            "setSelectionLocked" => set_selection_flag::set_selection_flag_value(ctx, args, "locked"),
            "addNode" => add_node::add_node(ctx, args),
            "patchInspectorNodes" => patch_inspector::patch_inspector(ctx, args),
            "forceLayout" | "reorganize" => {let mut progress=|_|true;let mut control=semio_framework_os_infinite::board::schema::layout::LayoutControl::new(100_000_000,&mut progress);force_layout::force_layout(ctx,&mut control)},
            "setCamera" => set_camera::set_camera(ctx, args),
            "focusSelection" => focus_selection::focus_selection(ctx),
            "engagementInput" => engagement_input::engagement_input(ctx, args),
            "engagementSubmit" => engagement_submit::engagement_submit(ctx, args),
            "engagementAbort" => engagement_abort::engagement_abort(ctx, args),
            "engagementControlSelect" => engagement_control_select::engagement_control_select(ctx, args),
            "setLodModeForPane" => set_lod_mode_for_pane::set_lod_mode_for_pane(ctx, args),
            "translateSelection" => translate_selection::translate_selection(ctx, args),
            "rotateSelection" => rotate_selection::rotate_selection(ctx, args),
            "scaleSelection" => scale_selection::scale_selection(ctx, args),
            "importSnapshot" => import_snapshot::import_snapshot(ctx, args),
            "openImportSnapshot" => open_import_snapshot::open_import_snapshot(ctx),
            "lodScaleJson" => lod_scale_json::lod_scale_json(ctx),
            "setGridSnapEnabled" => set_grid_snap_enabled::set_grid_snap_enabled(ctx, args),
            "setGridFactor" => set_grid_factor::set_grid_factor(ctx, args),
            "setGridVisible" => set_grid_visible::set_grid_visible(ctx, args),
            "setSelectableKind" => set_selectable_kind::set_selectable_kind(ctx, args),
            "setBrushPlacementContactTolerance" => set_brush_placement_contact_tolerance::set_brush_placement_contact_tolerance(ctx, args),
            "setBrushPlacementOverlapBudget" => set_brush_placement_overlap_budget::set_brush_placement_overlap_budget(ctx, args),
            "engagementRepeatLast" => engagement_repeat_last::engagement_repeat_last(ctx),
            "openAddNodeDialog" => {
                ctx.effects.push(Effect::OpenDialog { req: semio_framework_plugin::RequestId(semio_framework_job::allocate_operation_id().0), dialog_id: PUZZLE2D_ADD_NODE_DIALOG_ID.into(), args: None });
                *ctx.ui_scope = UiDirtyScope::None;
            }
            "setBrushKindWeights" => set_brush_kind_weights::set_brush_kind_weights(ctx, args),
            "setBrushNodeSize" => set_brush_node_size::set_brush_node_size(ctx, args),
            "setSuggestionOffset" => set_suggestion_offset::set_suggestion_offset(ctx, args),
            "setTransformGumballFlag" => set_transform_gumball_flag::set_transform_gumball_flag(ctx, args),
            "addTargetRegion" => add_target_region::add_target_region(ctx, args),
            "deleteTargetRegion" => delete_target_region::delete_target_region(ctx, args),
            "relocateTargetRegion" => relocate_target_region::relocate_target_region(ctx, args),
            "setTargetRegionFlag" => set_target_region_flag::set_target_region_flag(ctx, args),
            "setTargetRegionHidden" => set_target_region_flag::set_target_region_flag_value(ctx, args, "hidden"),
            "setTargetRegionLocked" => set_target_region_flag::set_target_region_flag_value(ctx, args, "locked"),
            "setAreaBrushSize" => set_area_brush_size::set_area_brush_size(ctx, args),
            "setFillCount" => set_fill_count::set_fill_count(ctx, args),
            "cycleBrushCandidate" => cycle_candidate::cycle_candidate(ctx, args.and_then(|value| value.get("forward")).and_then(|value| value.as_bool()).unwrap_or(true)),
            "cycleBrushCandidateBack" => cycle_candidate::cycle_candidate(ctx, false),
            "hoverSuggestion" => hover_suggestion::hover_suggestion(ctx, args),
            "openHandleSuggestions" => open_handle_suggestions::open_handle_suggestions(ctx, args),
            "acceptSuggestion" => accept_suggestion::accept_suggestion(ctx, args),
            "closeHandleSuggestions" => close_handle_suggestions::close_handle_suggestions(ctx),
            "targetBrushSuggestions" => target_brush_suggestions::target_brush_suggestions(ctx, args),
            "applyBoardEvents" => apply_board_events::apply_board_events(ctx, args),
            _ => {}
        }
    }
    apply_host_events(&mut host.borrow_mut(), &mut recorder, &mut scene.runtime);
    let mut operations = recorder.into_rows();
    operations.append(&mut artifact_mutations);
    // 🐢️ Safety net: a `None` scope claims nothing needs re-rendering — never pair that with an
    // actual document mutation (would silently desync remote clients' UI from the committed operation).
    if !operations.is_empty() && matches!(ui_scope, UiDirtyScope::None) {
        ui_scope = UiDirtyScope::Full;
    }
    // 🧮️ B1: only a REAL config change becomes a `Puzzle2dConfigMutation` — `PartialEq` (derived)
    // makes this cheap, and keeps a pure read-only action from creating a no-op undo entry.
    let (next_config, next_window_config, next_window_transient) = window::split(&scene.runtime, window_kind);
    let config_mutations = config.mutations_to(&next_config);
    let window_config_mutations = if &next_window_config != window_config { vec![window::addressed_config(view_state.ok_or_else(|| Fault::from("puzzle2d-window-context-required"))?, next_window_config)?] } else { Vec::new() };
    let window_transient = if &next_window_transient != window_transient { vec![window::addressed_transient(view_state.ok_or_else(|| Fault::from("puzzle2d-window-context-required"))?, next_window_transient)?] } else { Vec::new() };
    // 🛠️ The first tool transaction an action commits stamps every op of its ONE edit — the yielded leaves and any
    // scene delta the same dispatch carried — so history lists the gesture as one row whose inputs stay editable.
    let transaction = transaction.filter(|_| !operations.is_empty());
    Ok((Emit { artifact_mutations: operations, config_mutations, window_config_mutations, transaction, effects, ui_scope, interaction_writes, ..Default::default() }, EphemeralEmit { window_transient, ..Default::default() }))
}

/// 🗂️ Upper bound on the entities one selection-acting retained step may touch. Nakagin — this
/// artifact's largest example — carries 180 nodes, 179 edges and 358 handles, so a whole-board
/// selection is 717 entities: this ceiling admits that with headroom while still refusing an
/// unbounded selection well under the shared `PUZZLE_COMMAND_WORK_ITEMS` (4,096) budget.
const PUZZLE2D_SELECTION_BATCH_LIMIT: usize = 1_024;
/// 📥 Work items one whole-document import replacement claims: the largest example's entity count with
/// headroom, still well under the shared `PUZZLE_COMMAND_WORK_ITEMS` budget.
const PUZZLE2D_IMPORT_FIXTURE_WORK_ITEMS: usize = 1_024;

/// 🧮️ One work item per entity a selection-acting verb rewrites, one for every other generic verb
/// (each is a fixed-shape config/host setter whose cost is independent of document size). An
/// oversized selection returns `None` and is refused by the retained preflight rather than silently
/// truncated. `patchInspectorNodes` may address an explicit `ids` argument instead of the selection.
pub(super) fn puzzle2d_generic_extent(command: &Puzzle2dCommand, _snapshot: &Puzzle2dPlaySnapshot, interaction: &protocol::InteractionState) -> Option<usize> {
    let action = command.action_id();
    if !PUZZLE2D_GENERIC_TOOL_IDS.contains(&action) {
        return None;
    }
    if action == "importSnapshot" {
        return Some(PUZZLE2D_IMPORT_FIXTURE_WORK_ITEMS);
    }
    if !matches!(action, "patchInspectorNodes" | "setSelectionFlag" | "setSelectionHidden" | "setSelectionLocked" | "setTargetRegionFlag" | "setTargetRegionHidden" | "setTargetRegionLocked" | "deleteSelection" | "duplicateSelection" | "translateSelection" | "rotateSelection" | "scaleSelection" | "proximityConnect") {
        return Some(1);
    }
    let selected = interaction.selection.get(PUZZLE2D_INTERACTION_DOMAIN).map_or(0, |selection| selection.ids.len());
    let addressed = if matches!(action, "setTargetRegionFlag" | "setTargetRegionHidden" | "setTargetRegionLocked") && command.args().and_then(|args| args.get("id")).is_some() {
        1
    } else {
        command.args().filter(|_| matches!(action, "patchInspectorNodes" | "setSelectionHidden" | "setSelectionLocked")).and_then(|args| args.get("ids")).and_then(Value::as_array).map_or(selected, Vec::len)
    };
    if addressed > PUZZLE2D_SELECTION_BATCH_LIMIT {
        return None;
    }
    // 🧲️ `translateSelection` and `proximityConnect` land open handles, so both carry the gesture's
    // fixed auto-connect budget on top of the entities they rewrite.
    let connects = if matches!(action, "translateSelection" | "proximityConnect") { PUZZLE2D_PROXIMITY_GESTURE_MAX } else { 0 };
    Some(addressed.saturating_add(connects).max(1))
}

/// 🛍️ The example switch is a whole-document LOAD: `Effect::LoadDocument` carrying the example's pack (an unknown id loads
/// the empty board) and no mutation row, so it leaves no history row either. One bounded step, shared by the retained
/// `setActiveExample` route and the batch `handle` path.
pub(super) fn puzzle2d_active_example_emit(command: &Puzzle2dCommand, _snapshot: &Puzzle2dPlaySnapshot, _config: &Puzzle2dConfig) -> Result<Emit<Puzzle2dMutation, Puzzle2dConfigMutation>, Fault> {
    let id = set_active_example::canonical_example_id(command.args().and_then(|args| args.get("exampleId")).and_then(Value::as_str).unwrap_or(""));
    Ok(Emit { effects: vec![puzzle2d_load_document_effect(set_active_example::target(id))], ui_scope: UiDirtyScope::Full, ..Default::default() })
}

/// ➕️ `addNode` places one node through the shared window-command dispatch, so it is one admitted unit.
pub(super) fn puzzle2d_add_node_extent(command: &Puzzle2dCommand, _snapshot: &Puzzle2dPlaySnapshot, _interaction: &protocol::InteractionState) -> Option<usize> {
    (command.action_id() == "addNode").then_some(1)
}

pub(super) fn puzzle2d_active_example_reduce(
    command: &Puzzle2dCommand,
    snapshot: &Puzzle2dPlaySnapshot,
    config: &Puzzle2dConfig,
    _interaction: &protocol::InteractionState,
    _hover: &semio_framework_plugin::app::InteractionHoverState,
    _view_state: Option<&semio_framework_plugin::ViewModel>,
) -> Result<Emit<Puzzle2dMutation, Puzzle2dConfigMutation>, Fault> {
    puzzle2d_active_example_emit(command, snapshot, config)
}

pub(super) fn puzzle2d_active_example_extent(command: &Puzzle2dCommand, _snapshot: &Puzzle2dPlaySnapshot, _interaction: &protocol::InteractionState) -> Option<usize> {
    (command.action_id() == "setActiveExample").then_some(1)
}

/// 🧲️ Admission ceilings for one force-layout run. `MAX_NODES` admits the real
/// `🏗️nakagin-capsule-tower` snapshot (180 nodes / 179 edges / 358 handles) with room to spare — the
/// former 64-node ceiling made `forceLayout` structurally dead on the only non-trivial example this
/// artifact ships. `MAX_HANDLES` is checked by [`Puzzle2dForceLayoutWork::extent`] as well as by the
/// `Handles` stage, so an over-wide document is refused at preflight instead of faulting mid-run.
const PUZZLE2D_FORCE_MAX_NODES: usize = 512;
const PUZZLE2D_FORCE_MAX_EDGES: usize = 4_096;
const PUZZLE2D_FORCE_MAX_HANDLES: usize = 4_096;

/// 🔁️ Iteration schedule. A spring embedder's cost per iteration is `n·(n−1)/2` repulsion pairs plus
/// one pass per edge, so a fixed 420 iterations is affordable at 64 nodes and ~7 million pair
/// evaluations at 180. Iterations are therefore derived from the graph's own per-iteration cost
/// ([`puzzle2d_force_iterations`]) so total pair+spring work stays inside
/// [`PUZZLE2D_FORCE_WORK_BUDGET`]: small graphs keep the full 420-iteration quality, Nakagin gets
/// 122, and the 512-node ceiling gets the 24-iteration floor.
const PUZZLE2D_FORCE_ITERATIONS_MAX: u32 = 420;
const PUZZLE2D_FORCE_ITERATIONS_MIN: u32 = 24;
const PUZZLE2D_FORCE_WORK_BUDGET: u64 = 2_000_000;

/// 🍰️ Per-`step()` chunk sizes. One retained step is one checkpoint boundary, so a step must carry a
/// batch of force units rather than a single one — 8,192 pair evaluations are a few hundred
/// microseconds against the 7,500 µs step budget, while one pair per step would need ~7 million
/// checkpoints for Nakagin. `extent()` counts these chunks, which is what makes the declared budget
/// match the work actually performed.
const PUZZLE2D_FORCE_UNITS_PER_STEP: usize = 8_192;
const PUZZLE2D_FORCE_NODES_PER_STEP: usize = 512;
const PUZZLE2D_FORCE_SCAN_PER_STEP: usize = 256;
const PUZZLE2D_FORCE_ITERATION_STAGE_STEPS: usize = 4;
const PUZZLE2D_FORCE_PROLOGUE_STAGE_STEPS: usize = 8;

/// 🔢️ Unordered node pairs the repulsion pass visits per iteration.
fn puzzle2d_force_pairs(nodes: usize) -> u64 {
    let nodes = nodes as u64;
    nodes.saturating_mul(nodes.saturating_sub(1)) / 2
}

/// 🔁️ Iterations for a graph of `nodes` nodes and `edges` edges — `WORK_BUDGET / cost_per_iteration`,
/// clamped to `[MIN, MAX]`. Derived from the raw document counts (not the visible subset) so
/// [`Puzzle2dForceLayoutWork::extent`] and [`Puzzle2dForceLayoutWork::step`] always agree.
fn puzzle2d_force_iterations(nodes: usize, edges: usize) -> u32 {
    let cost = puzzle2d_force_pairs(nodes).saturating_add(edges as u64);
    if cost == 0 {
        return 1;
    }
    u32::try_from(PUZZLE2D_FORCE_WORK_BUDGET / cost).unwrap_or(PUZZLE2D_FORCE_ITERATIONS_MAX).clamp(PUZZLE2D_FORCE_ITERATIONS_MIN, PUZZLE2D_FORCE_ITERATIONS_MAX)
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Puzzle2dForceStage {
    Nodes,
    Handles,
    Edges,
    Seed,
    Center,
    Reset,
    Repel,
    Springs,
    Integrate,
    Emit,
    Complete,
    Closing,
}

/// ♻️ The owners one `Puzzle2dForceLayoutWork` still holds when its job closes, retired as one controlled bundle.
#[derive(semio_framework_value::RetireOwned)]
struct Puzzle2dForceLayoutWorkOwners {
    pub(super) node_ids: Vec<String>,
    raw_node_indices: Vec<usize>,
    original: Vec<Option<[f64; 2]>>,
    positions: Vec<[f64; 2]>,
    velocities: Vec<[f64; 2]>,
    forces: Vec<[f64; 2]>,
    radii: Vec<f64>,
    id_to_index: HashMap<String, usize>,
    handle_to_node: HashMap<String, String>,
    pub(super) edges: Vec<(usize, usize)>,
    edge_set: HashSet<(usize, usize)>,
    pub(super) mutations: Vec<Puzzle2dMutation>,
}

pub(super) struct Puzzle2dForceLayoutWork {
    pub(super) tool_id: &'static str,
    pub(super) stage: Puzzle2dForceStage,
    node_cursor: usize,
    handle_cursor: usize,
    edge_cursor: usize,
    seed_cursor: usize,
    center_cursor: usize,
    force_cursor: usize,
    pair_i: usize,
    pair_j: usize,
    iteration: u32,
    rng: u64,
    center: [f64; 2],
    finite_count: usize,
    pub(super) node_ids: Vec<String>,
    raw_node_indices: Vec<usize>,
    original: Vec<Option<[f64; 2]>>,
    positions: Vec<[f64; 2]>,
    velocities: Vec<[f64; 2]>,
    forces: Vec<[f64; 2]>,
    radii: Vec<f64>,
    id_to_index: HashMap<String, usize>,
    handle_to_node: HashMap<String, String>,
    pub(super) edges: Vec<(usize, usize)>,
    edge_set: HashSet<(usize, usize)>,
    pub(super) mutations: Vec<Puzzle2dMutation>,
    pub(super) retained_bytes: usize,
    close_owners: crate::puzzle_job::WorkClosing<Puzzle2dForceLayoutWorkOwners>,
}

impl Default for Puzzle2dForceLayoutWork {
    fn default() -> Self {
        Self::new("forceLayout")
    }
}

impl Puzzle2dForceLayoutWork {
    /// 🧲️ `forceLayout` and `reorganize` are the same simulation behind two verbs, so the admitted
    /// tool id travels with the work — the retained job's decode phase rejects a payload whose
    /// action id does not equal [`crate::retained_command::PuzzleCommandWork::tool_id`].
    pub(super) fn new(tool_id: &'static str) -> Self {
        Self { close_owners: Default::default(),
            tool_id,
            stage: Puzzle2dForceStage::Nodes,
            node_cursor: 0,
            handle_cursor: 0,
            edge_cursor: 0,
            seed_cursor: 0,
            center_cursor: 0,
            force_cursor: 0,
            pair_i: 0,
            pair_j: 1,
            iteration: 0,
            rng: 0x5eedfaced0,
            center: [0.0, 0.0],
            finite_count: 0,
            node_ids: Vec::with_capacity(PUZZLE2D_FORCE_MAX_NODES),
            raw_node_indices: Vec::with_capacity(PUZZLE2D_FORCE_MAX_NODES),
            original: Vec::with_capacity(PUZZLE2D_FORCE_MAX_NODES),
            positions: Vec::with_capacity(PUZZLE2D_FORCE_MAX_NODES),
            velocities: Vec::with_capacity(PUZZLE2D_FORCE_MAX_NODES),
            forces: Vec::with_capacity(PUZZLE2D_FORCE_MAX_NODES),
            radii: Vec::with_capacity(PUZZLE2D_FORCE_MAX_NODES),
            id_to_index: HashMap::with_capacity(PUZZLE2D_FORCE_MAX_NODES),
            handle_to_node: HashMap::with_capacity(PUZZLE2D_FORCE_MAX_HANDLES),
            edges: Vec::with_capacity(PUZZLE2D_FORCE_MAX_EDGES),
            edge_set: HashSet::with_capacity(PUZZLE2D_FORCE_MAX_EDGES),
            mutations: Vec::with_capacity(PUZZLE2D_FORCE_MAX_NODES),
            retained_bytes: 0,
        }
    }

    pub(super) fn visible(object: &semio_framework_pack_json::Object) -> bool {
        object.get("hidden").and_then(Value::as_bool).map_or_else(|| object.get("visible").and_then(Value::as_bool).unwrap_or(true), |hidden| !hidden)
    }

    pub(super) fn radius(node: &Value) -> f64 {
        let Some(object) = node.as_object() else { return 32.0 };
        if object.get("shape").and_then(Value::as_str) == Some("rectangle") {
            let width = object.get("width").and_then(Value::as_f64).unwrap_or(40.0);
            let height = object.get("height").and_then(Value::as_f64).unwrap_or(40.0);
            return ((width * width + height * height).sqrt() * 0.5).max(8.0);
        }
        object.get("radius").and_then(Value::as_f64).filter(|radius| radius.is_finite() && *radius > 0.0).unwrap_or(32.0)
    }

    fn split_mix64(mut value: u64) -> u64 {
        value = (value ^ (value >> 30)).wrapping_mul(0xBF58476D1CE4E5B9);
        value = (value ^ (value >> 27)).wrapping_mul(0x94D049BB133111EB);
        value ^ (value >> 31)
    }

    fn random_unit(&mut self) -> f64 {
        self.rng = Self::split_mix64(self.rng);
        (self.rng as f64) / (u64::MAX as f64)
    }

    fn pop_one(&mut self) -> bool {
        macro_rules! pop {
            ($field:ident) => {
                if self.$field.pop().is_some() {
                    return true;
                }
            };
        }
        pop!(mutations);
        pop!(edges);
        pop!(radii);
        pop!(forces);
        pop!(velocities);
        pop!(positions);
        pop!(original);
        pop!(raw_node_indices);
        pop!(node_ids);
        if let Some(key) = self.handle_to_node.keys().next().cloned() {
            self.handle_to_node.remove(&key);
            return true;
        }
        if let Some(key) = self.id_to_index.keys().next().cloned() {
            self.id_to_index.remove(&key);
            return true;
        }
        if let Some(edge) = self.edge_set.iter().next().copied() {
            self.edge_set.remove(&edge);
            return true;
        }
        false
    }

    /// 🌡️ Simulated-annealing factor for the current iteration against the size-derived schedule.
    fn cool(&self, iterations: u32) -> f64 {
        (1.0 - f64::from(self.iteration) / f64::from(iterations.max(1))).max(0.08)
    }

    fn scan_node_one(&mut self, nodes: &[Value]) -> Result<(), Fault> {
        let Some(node) = nodes.get(self.node_cursor) else {
            if self.positions.is_empty() {
                self.stage = Puzzle2dForceStage::Emit;
                return Ok(());
            }
            if self.finite_count > 0 {
                self.center[0] /= self.finite_count as f64;
                self.center[1] /= self.finite_count as f64;
            }
            self.stage = Puzzle2dForceStage::Edges;
            return Ok(());
        };
        let object = node.as_object().ok_or_else(|| Fault::from("puzzle2d-force-node-not-object"))?;
        if !Self::visible(object) {
            self.node_cursor += 1;
            return Ok(());
        }
        let id = object.get("id").and_then(Value::as_str).ok_or_else(|| Fault::from("puzzle2d-force-node-id-missing"))?;
        if self.node_ids.len() >= PUZZLE2D_FORCE_MAX_NODES || self.retained_bytes.checked_add(id.len()).is_none_or(|bytes| bytes > crate::retained_command::PUZZLE_COMMAND_OUTPUT_BYTES) {
            return Err(Fault::from("puzzle2d-force-node-capacity"));
        }
        let x = object.get("x").and_then(Value::as_f64);
        let y = object.get("y").and_then(Value::as_f64);
        let original = match (x, y) {
            (Some(x), Some(y)) if x.is_finite() && y.is_finite() => Some([x, y]),
            _ => None,
        };
        let index = self.positions.len();
        self.retained_bytes += id.len();
        self.id_to_index.insert(id.to_string(), index);
        self.node_ids.push(id.to_string());
        self.raw_node_indices.push(self.node_cursor);
        self.original.push(original);
        self.positions.push(original.unwrap_or([0.0, 0.0]));
        if let Some(position) = original {
            self.center[0] += position[0];
            self.center[1] += position[1];
            self.finite_count += 1;
        }
        self.velocities.push([0.0, 0.0]);
        self.forces.push([0.0, 0.0]);
        self.radii.push(Self::radius(node));
        self.handle_cursor = 0;
        self.stage = Puzzle2dForceStage::Handles;
        Ok(())
    }

    fn scan_handle_one(&mut self, nodes: &[Value]) -> Result<(), Fault> {
        let object = nodes.get(self.node_cursor).and_then(Value::as_object).ok_or_else(|| Fault::from("puzzle2d-force-node-owner-lost"))?;
        let handles = object.get("handles").and_then(Value::as_array);
        let Some(handle) = handles.and_then(|handles| handles.get(self.handle_cursor)) else {
            self.node_cursor += 1;
            self.stage = Puzzle2dForceStage::Nodes;
            return Ok(());
        };
        self.handle_cursor += 1;
        let Some(handle_object) = handle.as_object() else { return Ok(()) };
        if !Self::visible(handle_object) {
            return Ok(());
        }
        let Some(handle_id) = handle_object.get("id").and_then(Value::as_str) else { return Ok(()) };
        let node_id = object.get("id").and_then(Value::as_str).ok_or_else(|| Fault::from("puzzle2d-force-node-id-owner-lost"))?;
        let added_bytes = handle_id.len().saturating_add(node_id.len());
        if self.handle_to_node.len() >= PUZZLE2D_FORCE_MAX_HANDLES || self.retained_bytes.checked_add(added_bytes).is_none_or(|bytes| bytes > crate::retained_command::PUZZLE_COMMAND_OUTPUT_BYTES) {
            return Err(Fault::from("puzzle2d-force-handle-capacity"));
        }
        self.retained_bytes += added_bytes;
        self.handle_to_node.insert(handle_id.to_string(), node_id.to_string());
        Ok(())
    }

    fn scan_edge_one(&mut self, edges: Option<&Vec<Value>>) -> Result<(), Fault> {
        let Some(edge) = edges.and_then(|edges| edges.get(self.edge_cursor)) else {
            self.stage = Puzzle2dForceStage::Seed;
            return Ok(());
        };
        self.edge_cursor += 1;
        let Some(object) = edge.as_object() else { return Ok(()) };
        if !Self::visible(object) {
            return Ok(());
        }
        let (Some(source), Some(target)) = (object.get("source").and_then(Value::as_str), object.get("target").and_then(Value::as_str)) else {
            return Ok(());
        };
        let source_node = self.handle_to_node.get(source).map_or(source, String::as_str);
        let target_node = self.handle_to_node.get(target).map_or(target, String::as_str);
        let (Some(&a), Some(&b)) = (self.id_to_index.get(source_node), self.id_to_index.get(target_node)) else {
            return Ok(());
        };
        let pair = (a.min(b), a.max(b));
        if a != b && self.edge_set.insert(pair) {
            if self.edges.len() >= PUZZLE2D_FORCE_MAX_EDGES {
                return Err(Fault::from("puzzle2d-force-edge-capacity"));
            }
            self.edges.push(pair);
        }
        Ok(())
    }

    fn seed_one(&mut self) {
        if self.seed_cursor >= self.positions.len() {
            self.center = [0.0, 0.0];
            self.stage = Puzzle2dForceStage::Center;
            return;
        }
        if self.positions[self.seed_cursor][0].hypot(self.positions[self.seed_cursor][1]) < 1e-9 {
            let t = self.seed_cursor as f64;
            let angle = t * 2.399_963_229_728_653_5;
            let radius = 10.0 + t.sqrt() * 22.0;
            let jitter_x = (self.random_unit() - 0.5) * 6.0;
            let jitter_y = (self.random_unit() - 0.5) * 6.0;
            self.positions[self.seed_cursor] = [self.center[0] + radius * angle.cos() + jitter_x, self.center[1] + radius * angle.sin() + jitter_y];
        }
        self.seed_cursor += 1;
    }

    fn center_one(&mut self) {
        if let Some(position) = self.positions.get(self.center_cursor) {
            self.center[0] += position[0];
            self.center[1] += position[1];
            self.center_cursor += 1;
            return;
        }
        if !self.positions.is_empty() {
            self.center[0] /= self.positions.len() as f64;
            self.center[1] /= self.positions.len() as f64;
        }
        self.stage = Puzzle2dForceStage::Reset;
        self.force_cursor = 0;
    }

    fn reset_one(&mut self) {
        if self.force_cursor < self.forces.len() {
            self.forces[self.force_cursor] = [0.0, 0.0];
            self.force_cursor += 1;
            return;
        }
        self.pair_i = 0;
        self.pair_j = 1;
        self.stage = Puzzle2dForceStage::Repel;
    }

    fn repel_one(&mut self, iterations: u32) {
        let count = self.positions.len();
        if self.pair_i >= count || self.pair_j >= count {
            self.edge_cursor = 0;
            self.stage = Puzzle2dForceStage::Springs;
            return;
        }
        let i = self.pair_i;
        let j = self.pair_j;
        let dx = self.positions[j][0] - self.positions[i][0];
        let dy = self.positions[j][1] - self.positions[i][1];
        let distance = dx.hypot(dy).max(1e-4);
        let repulsion = 6500.0 * self.cool(iterations) * (self.radii[i] * self.radii[j]).max(1.0) / (distance * distance);
        let fx = dx / distance * -repulsion;
        let fy = dy / distance * -repulsion;
        self.forces[i][0] += fx;
        self.forces[i][1] += fy;
        self.forces[j][0] -= fx;
        self.forces[j][1] -= fy;
        self.pair_j += 1;
        if self.pair_j >= count {
            self.pair_i += 1;
            self.pair_j = self.pair_i.saturating_add(1);
        }
    }

    fn spring_one(&mut self, iterations: u32) {
        let Some(&(i, j)) = self.edges.get(self.edge_cursor) else {
            self.force_cursor = 0;
            self.stage = Puzzle2dForceStage::Integrate;
            return;
        };
        let dx = self.positions[j][0] - self.positions[i][0];
        let dy = self.positions[j][1] - self.positions[i][1];
        let distance = dx.hypot(dy).max(1e-4);
        let magnitude = 0.028 * self.cool(iterations) * (distance - 140.0);
        let fx = dx / distance * magnitude;
        let fy = dy / distance * magnitude;
        self.forces[i][0] += fx;
        self.forces[i][1] += fy;
        self.forces[j][0] -= fx;
        self.forces[j][1] -= fy;
        self.edge_cursor += 1;
    }

    fn integrate_one(&mut self, iterations: u32) {
        if self.force_cursor >= self.positions.len() {
            self.iteration += 1;
            self.force_cursor = 0;
            self.stage = if self.iteration >= iterations { Puzzle2dForceStage::Emit } else { Puzzle2dForceStage::Reset };
            return;
        }
        let index = self.force_cursor;
        let delta_time = 0.85 * self.cool(iterations).sqrt();
        let mut velocity = [(self.velocities[index][0] + self.forces[index][0] * delta_time) * 0.88, (self.velocities[index][1] + self.forces[index][1] * delta_time) * 0.88];
        let speed = velocity[0].hypot(velocity[1]);
        if speed > 48.0 {
            velocity[0] *= 48.0 / speed;
            velocity[1] *= 48.0 / speed;
        }
        self.velocities[index] = velocity;
        self.positions[index][0] += velocity[0] * delta_time;
        self.positions[index][1] += velocity[1] * delta_time;
        self.force_cursor += 1;
    }

    fn emit_one(&mut self) -> bool {
        let index = self.force_cursor;
        let Some(position) = self.positions.get(index).copied() else {
            self.stage = Puzzle2dForceStage::Complete;
            return true;
        };
        if self.original[index] != Some(position) {
            self.mutations.push(crate::standards::v1::subsets::any::schema::mutations::move_node(self.node_ids[index].clone().into(), position[0], position[1]));
        }
        self.force_cursor += 1;
        false
    }
}

impl semio_framework_plugin::retained_command::ArtifactCommandWork<EditorApp<Puzzle2dPlayApp>> for Puzzle2dForceLayoutWork {
    fn tool_id(&self) -> &'static str {
        self.tool_id
    }

    /// 📐️ One unit of the declared extent is one `step()` call, not one force evaluation: the
    /// prologue scans `nodes + handles + edges` in [`PUZZLE2D_FORCE_SCAN_PER_STEP`] chunks, each of
    /// the [`puzzle2d_force_iterations`] iterations costs one reset pass, one integrate pass, and the
    /// repulsion/spring chunks, and the epilogue emits one mutation pass. Handles are counted here
    /// too, so a document that would trip `PUZZLE2D_FORCE_MAX_HANDLES` mid-run is refused at
    /// preflight instead.
    fn extent(&self, command: &Puzzle2dCommand, snapshot: &Puzzle2dPlaySnapshot, _interaction: &protocol::InteractionState, _context: Option<&semio_framework_plugin::app::ArtifactOwnedToolJobContext<EditorApp<Puzzle2dPlayApp>>>) -> Option<usize> {
        if command.action_id() != self.tool_id || snapshot.value().get("schema").and_then(Value::as_str) != Some(PUZZLE2D_BOARD_SNAPSHOT_SCHEMA) {
            return None;
        }
        let nodes = snapshot.value().get("nodes")?.as_array()?;
        let edges = snapshot.value().get("edges").and_then(Value::as_array).map_or(0, Vec::len);
        let handles = nodes.iter().filter_map(|node| node.get("handles")).filter_map(Value::as_array).map(Vec::len).sum::<usize>();
        if nodes.len() > PUZZLE2D_FORCE_MAX_NODES || edges > PUZZLE2D_FORCE_MAX_EDGES || handles > PUZZLE2D_FORCE_MAX_HANDLES {
            return None;
        }
        let pairs = usize::try_from(puzzle2d_force_pairs(nodes.len())).ok()?;
        let node_pass = nodes.len().div_ceil(PUZZLE2D_FORCE_NODES_PER_STEP).max(1);
        let iteration = pairs.div_ceil(PUZZLE2D_FORCE_UNITS_PER_STEP).checked_add(edges.div_ceil(PUZZLE2D_FORCE_UNITS_PER_STEP))?.checked_add(node_pass.checked_mul(2)?)?.checked_add(PUZZLE2D_FORCE_ITERATION_STAGE_STEPS)?;
        let items = nodes
            .len()
            .checked_add(handles)?
            .checked_add(edges)?
            .div_ceil(PUZZLE2D_FORCE_SCAN_PER_STEP)
            .checked_add(node_pass.checked_mul(3)?)?
            .checked_add(PUZZLE2D_FORCE_PROLOGUE_STAGE_STEPS)?
            .checked_add(iteration.checked_mul(usize::try_from(puzzle2d_force_iterations(nodes.len(), edges)).ok()?)?)?;
        (items <= crate::retained_command::PUZZLE_COMMAND_WORK_ITEMS).then_some(items.max(1))
    }

    /// 🍰️ Each call drains one chunk of the current stage and yields a progress boundary, so the
    /// number of `step()` calls matches [`Puzzle2dForceLayoutWork::extent`] instead of running one
    /// force evaluation per checkpoint.
    fn work_demands(&self, _input: &semio_framework_plugin::retained_command::ArtifactCommandInputs<'_, EditorApp<Puzzle2dPlayApp>>, _maximum_copy_bytes: usize) -> Result<semio_framework_value::RetirementDemand, semio_framework_value::ValueError> {
        crate::retained_command::step_demands::<EditorApp<Puzzle2dPlayApp>>(size_of::<Self>())
    }

    fn step(&mut self, input: &semio_framework_plugin::retained_command::ArtifactCommandInputs<'_, EditorApp<Puzzle2dPlayApp>>, _cx: &mut StepContext<'_>) -> Result<semio_framework_plugin::retained_command::ArtifactCommandWorkStep<EditorApp<Puzzle2dPlayApp>>, Fault> {
        let semio_framework_plugin::retained_command::ArtifactCommandInputs { command: _command, snapshot, config: _config, interaction: _interaction, hover: _hover, context: _context, .. } = *input;
        let nodes = snapshot.value().get("nodes").and_then(Value::as_array).ok_or_else(|| Fault::from("puzzle2d-force-nodes-missing"))?;
        let edges = snapshot.value().get("edges").and_then(Value::as_array);
        let iterations = puzzle2d_force_iterations(nodes.len(), edges.map_or(0, Vec::len));
        match self.stage {
            Puzzle2dForceStage::Nodes | Puzzle2dForceStage::Handles => {
                for _ in 0..PUZZLE2D_FORCE_SCAN_PER_STEP {
                    match self.stage {
                        Puzzle2dForceStage::Nodes => self.scan_node_one(nodes)?,
                        Puzzle2dForceStage::Handles => self.scan_handle_one(nodes)?,
                        _ => break,
                    }
                }
                Ok(crate::puzzle_progress_step!("puzzle2d-force-node", "Reading layout node", "Layout-Knoten wird gelesen"))
            }
            Puzzle2dForceStage::Edges => {
                for _ in 0..PUZZLE2D_FORCE_SCAN_PER_STEP {
                    if self.stage != Puzzle2dForceStage::Edges {
                        break;
                    }
                    self.scan_edge_one(edges)?;
                }
                Ok(crate::puzzle_progress_step!("puzzle2d-force-edge", "Indexing layout edge", "Layout-Kante wird indiziert"))
            }
            Puzzle2dForceStage::Seed => {
                for _ in 0..PUZZLE2D_FORCE_NODES_PER_STEP {
                    if self.stage != Puzzle2dForceStage::Seed {
                        break;
                    }
                    self.seed_one();
                }
                Ok(crate::puzzle_progress_step!("puzzle2d-force-seed", "Seeding layout node", "Layout-Knoten wird initialisiert"))
            }
            Puzzle2dForceStage::Center => {
                for _ in 0..PUZZLE2D_FORCE_NODES_PER_STEP {
                    if self.stage != Puzzle2dForceStage::Center {
                        break;
                    }
                    self.center_one();
                }
                Ok(crate::puzzle_progress_step!("puzzle2d-force-center", "Centering layout", "Layout wird zentriert"))
            }
            Puzzle2dForceStage::Reset => {
                for _ in 0..PUZZLE2D_FORCE_NODES_PER_STEP {
                    if self.stage != Puzzle2dForceStage::Reset {
                        break;
                    }
                    self.reset_one();
                }
                Ok(crate::puzzle_progress_step!("puzzle2d-force-reset", "Resetting layout force", "Layout-Kraft wird zurückgesetzt"))
            }
            Puzzle2dForceStage::Repel => {
                for _ in 0..PUZZLE2D_FORCE_UNITS_PER_STEP {
                    if self.stage != Puzzle2dForceStage::Repel {
                        break;
                    }
                    self.repel_one(iterations);
                }
                Ok(crate::puzzle_progress_step!("puzzle2d-force-repel", "Applying layout repulsion", "Layout-Abstoßung wird angewendet"))
            }
            Puzzle2dForceStage::Springs => {
                for _ in 0..PUZZLE2D_FORCE_UNITS_PER_STEP {
                    if self.stage != Puzzle2dForceStage::Springs {
                        break;
                    }
                    self.spring_one(iterations);
                }
                Ok(crate::puzzle_progress_step!("puzzle2d-force-spring", "Applying layout spring", "Layout-Feder wird angewendet"))
            }
            Puzzle2dForceStage::Integrate => {
                for _ in 0..PUZZLE2D_FORCE_NODES_PER_STEP {
                    if self.stage != Puzzle2dForceStage::Integrate {
                        break;
                    }
                    self.integrate_one(iterations);
                }
                Ok(crate::puzzle_progress_step!("puzzle2d-force-integrate", "Integrating layout node", "Layout-Knoten wird integriert"))
            }
            Puzzle2dForceStage::Emit => {
                for _ in 0..PUZZLE2D_FORCE_NODES_PER_STEP {
                    if self.emit_one() {
                        let mutations = std::mem::take(&mut self.mutations);
                        return Ok(semio_framework_plugin::retained_command::ArtifactCommandWorkStep::Complete(Emit { artifact_mutations: mutations, ui_scope: UiDirtyScope::Full, ..Default::default() }));
                    }
                }
                Ok(crate::puzzle_progress_step!("puzzle2d-force-emit", "Preparing layout mutation", "Layout-Mutation wird vorbereitet"))
            }
            Puzzle2dForceStage::Complete => Err(Fault::from("puzzle2d-force-complete-repolled")),
            Puzzle2dForceStage::Closing => Err(Fault::from("puzzle2d-force-closing")),
        }
    }

    fn begin_close(&mut self) {
        self.stage = Puzzle2dForceStage::Closing;
        self.close_owners.stage(Puzzle2dForceLayoutWorkOwners { node_ids: std::mem::take(&mut self.node_ids), raw_node_indices: std::mem::take(&mut self.raw_node_indices), original: std::mem::take(&mut self.original), positions: std::mem::take(&mut self.positions), velocities: std::mem::take(&mut self.velocities), forces: std::mem::take(&mut self.forces), radii: std::mem::take(&mut self.radii), id_to_index: std::mem::take(&mut self.id_to_index), handle_to_node: std::mem::take(&mut self.handle_to_node), edges: std::mem::take(&mut self.edges), edge_set: std::mem::take(&mut self.edge_set), mutations: std::mem::take(&mut self.mutations) });
    }

    fn close_step(&mut self, grant: semio_framework_value::retained_clone::RetainedCloneGrant) -> semio_framework_job::InteractiveJobCloseStep {
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
        self.stage == Puzzle2dForceStage::Closing && self.close_owners.is_empty()
    }

    fn terminal_frame_release_bytes(&self) -> Option<usize> {
        Some(size_of::<Self>())
    }
}

/// 🕸️ Admission ceilings and chunk size for one `redrawHandles` run. The legacy handler serialized
/// the entire snapshot and called `apply_edge_handle_snap_to_host_snapshot_v1_json` once, with no size
/// guard and no yield point; these bound the same three passes so a large board is refused at
/// preflight rather than blocking a step.
const PUZZLE2D_REDRAW_MAX_NODES: usize = 4_096;
const PUZZLE2D_REDRAW_MAX_EDGES: usize = 4_096;
const PUZZLE2D_REDRAW_MAX_HANDLES: usize = 8_192;
const PUZZLE2D_REDRAW_UNITS_PER_STEP: usize = 256;
const PUZZLE2D_REDRAW_STAGE_STEPS: usize = 4;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Puzzle2dRedrawStage {
    Nodes,
    Handles,
    Edges,
    Emit,
    Complete,
    Closing,
}

/// 🔵️ The snap geometry of one node, mirroring the engine's own `NodeShapeSnap`: a node without a
/// finite centre — or a circle without a radius, or a rectangle without extents — has no rim to
/// snap onto and is skipped, exactly as `apply_edge_handle_snap_to_host_snapshot_v1_value` skips it.
#[derive(Clone, Copy)]
struct Puzzle2dRedrawShape {
    center: [f64; 2],
    pub(super) rectangle: Option<[f64; 2]>,
}

semio_framework_value::artifact_retire_leaf!(Puzzle2dRedrawShape);

/// 🕸️ `redrawHandles` as a bounded, resumable walk. `Nodes`/`Handles` index every visible handle to
/// its owning node, `Edges` resolves each visible edge into one angle per endpoint (last edge wins
/// on a shared handle, matching the engine), and `Emit` publishes one `replace-node-handle` per
/// handle whose angle actually changed, each from the handle's own snap geometry — never a whole-board difference.
/// ♻️ The owners one `Puzzle2dRedrawHandlesWork` still holds when its job closes, retired as one controlled bundle.
#[derive(semio_framework_value::RetireOwned)]
struct Puzzle2dRedrawHandlesWorkOwners {
    shapes: Vec<Option<Puzzle2dRedrawShape>>,
    handle_locations: HashMap<String, (usize, usize)>,
    angles: std::collections::BTreeMap<(usize, usize), f64>,
    pub(super) mutations: Vec<Puzzle2dMutation>,
}

pub(super) struct Puzzle2dRedrawHandlesWork {
    pub(super) stage: Puzzle2dRedrawStage,
    node_cursor: usize,
    handle_cursor: usize,
    edge_cursor: usize,
    shapes: Vec<Option<Puzzle2dRedrawShape>>,
    handle_locations: HashMap<String, (usize, usize)>,
    angles: std::collections::BTreeMap<(usize, usize), f64>,
    pub(super) mutations: Vec<Puzzle2dMutation>,
    pub(super) retained_bytes: usize,
    close_owners: crate::puzzle_job::WorkClosing<Puzzle2dRedrawHandlesWorkOwners>,
}

impl Default for Puzzle2dRedrawHandlesWork {
    fn default() -> Self {
        Self { close_owners: Default::default(),
            stage: Puzzle2dRedrawStage::Nodes,
            node_cursor: 0,
            handle_cursor: 0,
            edge_cursor: 0,
            shapes: Vec::with_capacity(PUZZLE2D_REDRAW_MAX_NODES),
            handle_locations: HashMap::with_capacity(PUZZLE2D_REDRAW_MAX_HANDLES),
            angles: std::collections::BTreeMap::new(),
            mutations: Vec::with_capacity(PUZZLE2D_REDRAW_MAX_HANDLES),
            retained_bytes: 0,
        }
    }
}

impl Puzzle2dRedrawHandlesWork {
    pub(super) fn shape(object: &semio_framework_pack_json::Object) -> Option<Puzzle2dRedrawShape> {
        let cx = object.get("x").and_then(Value::as_f64)?;
        let cy = object.get("y").and_then(Value::as_f64)?;
        if object.get("shape").and_then(Value::as_str) == Some("rectangle") {
            let width = object.get("width").and_then(Value::as_f64)?;
            let height = object.get("height").and_then(Value::as_f64)?;
            return Some(Puzzle2dRedrawShape { center: [cx, cy], rectangle: Some([width, height]) });
        }
        object.get("radius").and_then(Value::as_f64)?;
        Some(Puzzle2dRedrawShape { center: [cx, cy], rectangle: None })
    }

    /// 🧭️ Delegates to the board engine's own conventions — east-zero for circles, north-zero for
    /// rectangles — so a chunked redraw and the engine's whole-snapshot snap agree numerically.
    fn angle_toward(from: Puzzle2dRedrawShape, toward: Puzzle2dRedrawShape) -> Option<f64> {
        let center = Point::new(from.center[0], from.center[1]);
        let target = Point::new(toward.center[0], toward.center[1]);
        if crate::editor::puzzle2d::engine::distance_between(center, target) <= 1e-9 {
            return None;
        }
        Some(match from.rectangle {
            Some([width, height]) => crate::editor::puzzle2d::engine::rectangle_handle_angle_toward(center, width, height, target),
            None => crate::editor::puzzle2d::engine::circle_handle_angle_toward(center, target),
        })
    }

    fn admit_bytes(&mut self, bytes: usize) -> Result<(), Fault> {
        let next = self.retained_bytes.checked_add(bytes).ok_or_else(|| Fault::from("puzzle2d-redraw-byte-capacity"))?;
        if next > crate::retained_command::PUZZLE_COMMAND_OUTPUT_BYTES {
            return Err(Fault::from("puzzle2d-redraw-byte-capacity"));
        }
        self.retained_bytes = next;
        Ok(())
    }

    fn index_node_one(&mut self, nodes: &[Value]) -> Result<(), Fault> {
        let Some(node) = nodes.get(self.node_cursor) else {
            self.edge_cursor = 0;
            self.stage = Puzzle2dRedrawStage::Edges;
            return Ok(());
        };
        let object = node.as_object().ok_or_else(|| Fault::from("puzzle2d-redraw-node-not-object"))?;
        if self.shapes.len() >= PUZZLE2D_REDRAW_MAX_NODES {
            return Err(Fault::from("puzzle2d-redraw-node-capacity"));
        }
        let visible = crate::editor::puzzle2d::engine::board_visible_or_true(&crate::editor::puzzle2d::engine::BoardVisibility {hidden:object.get("hidden").and_then(|v|v.as_bool()),visible:object.get("visible").and_then(|v|v.as_bool()),locked:object.get("locked").and_then(|v|v.as_bool())});
        self.shapes.push(visible.then(|| Self::shape(object)).flatten());
        if !visible {
            self.node_cursor += 1;
            return Ok(());
        }
        self.handle_cursor = 0;
        self.stage = Puzzle2dRedrawStage::Handles;
        Ok(())
    }

    fn index_handle_one(&mut self, nodes: &[Value]) -> Result<(), Fault> {
        let object = nodes.get(self.node_cursor).and_then(Value::as_object).ok_or_else(|| Fault::from("puzzle2d-redraw-node-owner-lost"))?;
        let Some(handle) = object.get("handles").and_then(Value::as_array).and_then(|handles| handles.get(self.handle_cursor)) else {
            self.node_cursor += 1;
            self.stage = Puzzle2dRedrawStage::Nodes;
            return Ok(());
        };
        let index = self.handle_cursor;
        self.handle_cursor += 1;
        let Some(handle_object) = handle.as_object() else { return Ok(()) };
        if !crate::editor::puzzle2d::engine::board_visible_or_true(&crate::editor::puzzle2d::engine::BoardVisibility {hidden:handle_object.get("hidden").and_then(|v|v.as_bool()),visible:handle_object.get("visible").and_then(|v|v.as_bool()),locked:handle_object.get("locked").and_then(|v|v.as_bool())}) {
            return Ok(());
        }
        let Some(handle_id) = handle_object.get("id").and_then(Value::as_str) else { return Ok(()) };
        if self.handle_locations.len() >= PUZZLE2D_REDRAW_MAX_HANDLES {
            return Err(Fault::from("puzzle2d-redraw-handle-capacity"));
        }
        self.admit_bytes(handle_id.len())?;
        self.handle_locations.insert(handle_id.to_string(), (self.node_cursor, index));
        Ok(())
    }

    fn resolve_edge_one(&mut self, edges: Option<&Vec<Value>>) -> Result<(), Fault> {
        let Some(edge) = edges.and_then(|edges| edges.get(self.edge_cursor)) else {
            self.stage = Puzzle2dRedrawStage::Emit;
            return Ok(());
        };
        self.edge_cursor += 1;
        let Some(object) = edge.as_object() else { return Ok(()) };
        if !crate::editor::puzzle2d::engine::board_visible_or_true(&crate::editor::puzzle2d::engine::BoardVisibility {hidden:object.get("hidden").and_then(|v|v.as_bool()),visible:object.get("visible").and_then(|v|v.as_bool()),locked:object.get("locked").and_then(|v|v.as_bool())}) {
            return Ok(());
        }
        let Some((source, target)) = crate::editor::puzzle2d::engine::board_edge_handle_ids(object.get("source").and_then(|v|v.as_str()),object.get("target").and_then(|v|v.as_str())) else { return Ok(()) };
        let (Some(&source_location), Some(&target_location)) = (self.handle_locations.get(source), self.handle_locations.get(target)) else {
            return Ok(());
        };
        let (Some(Some(source_shape)), Some(Some(target_shape))) = (self.shapes.get(source_location.0).copied(), self.shapes.get(target_location.0).copied()) else {
            return Ok(());
        };
        if let Some(angle) = Self::angle_toward(source_shape, target_shape) {
            self.angles.insert(source_location, angle);
        }
        if let Some(angle) = Self::angle_toward(target_shape, source_shape) {
            self.angles.insert(target_location, angle);
        }
        Ok(())
    }

    fn publish_handle_one(&mut self, nodes: &[Value]) -> Result<bool, Fault> {
        let Some(((node_index, handle_index), angle)) = self.angles.pop_first() else {
            self.stage = Puzzle2dRedrawStage::Complete;
            return Ok(true);
        };
        let object = nodes.get(node_index).and_then(Value::as_object).ok_or_else(|| Fault::from("puzzle2d-redraw-node-owner-lost"))?;
        let node_id = object.get("id").and_then(Value::as_str).ok_or_else(|| Fault::from("puzzle2d-redraw-node-id-missing"))?;
        let handle = object.get("handles").and_then(Value::as_array).and_then(|handles| handles.get(handle_index)).ok_or_else(|| Fault::from("puzzle2d-redraw-handle-owner-lost"))?;
        let original = <crate::standards::v1::subsets::any::schema::Puzzle2dHandle as semio_framework_value::FromValue>::from_value(semio_framework_value::ToValue::to_value(handle)).map_err(|_| Fault::from("puzzle2d-redraw-handle-malformed"))?;
        let mut next = original.clone();
        next.angle = angle;
        if next == original {
            return Ok(false);
        }
        self.admit_bytes(node_id.len().saturating_add(next.id.len()))?;
        self.mutations.push(crate::standards::v1::subsets::any::schema::mutations::replace_node_handle(node_id.into(), next.id.clone(), next));
        Ok(false)
    }

    fn pop_one(&mut self) -> bool {
        if self.mutations.pop().is_some() {
            return true;
        }
        if self.angles.pop_first().is_some() {
            return true;
        }
        if self.shapes.pop().is_some() {
            return true;
        }
        if let Some(key) = self.handle_locations.keys().next().cloned() {
            self.handle_locations.remove(&key);
            return true;
        }
        false
    }
}

impl semio_framework_plugin::retained_command::ArtifactCommandWork<EditorApp<Puzzle2dPlayApp>> for Puzzle2dRedrawHandlesWork {
    fn tool_id(&self) -> &'static str {
        "redrawHandles"
    }

    /// 📐️ One index chunk per [`PUZZLE2D_REDRAW_UNITS_PER_STEP`] node-or-handle records, one chunk
    /// per that many edges, one chunk per that many published handles, plus one stage handover each.
    fn extent(&self, command: &Puzzle2dCommand, snapshot: &Puzzle2dPlaySnapshot, _interaction: &protocol::InteractionState, _context: Option<&semio_framework_plugin::app::ArtifactOwnedToolJobContext<EditorApp<Puzzle2dPlayApp>>>) -> Option<usize> {
        if command.action_id() != "redrawHandles" || snapshot.value().get("schema").and_then(Value::as_str) != Some(PUZZLE2D_BOARD_SNAPSHOT_SCHEMA) {
            return None;
        }
        let nodes = snapshot.value().get("nodes")?.as_array()?;
        let edges = snapshot.value().get("edges").and_then(Value::as_array).map_or(0, Vec::len);
        let handles = nodes.iter().filter_map(|node| node.get("handles")).filter_map(Value::as_array).map(Vec::len).sum::<usize>();
        if nodes.len() > PUZZLE2D_REDRAW_MAX_NODES || edges > PUZZLE2D_REDRAW_MAX_EDGES || handles > PUZZLE2D_REDRAW_MAX_HANDLES {
            return None;
        }
        let items = nodes
            .len()
            .checked_add(handles)?
            .div_ceil(PUZZLE2D_REDRAW_UNITS_PER_STEP)
            .checked_add(edges.div_ceil(PUZZLE2D_REDRAW_UNITS_PER_STEP))?
            .checked_add(handles.div_ceil(PUZZLE2D_REDRAW_UNITS_PER_STEP))?
            .checked_add(PUZZLE2D_REDRAW_STAGE_STEPS)?;
        (items <= crate::retained_command::PUZZLE_COMMAND_WORK_ITEMS).then_some(items.max(1))
    }

    fn work_demands(&self, _input: &semio_framework_plugin::retained_command::ArtifactCommandInputs<'_, EditorApp<Puzzle2dPlayApp>>, _maximum_copy_bytes: usize) -> Result<semio_framework_value::RetirementDemand, semio_framework_value::ValueError> {
        crate::retained_command::step_demands::<EditorApp<Puzzle2dPlayApp>>(size_of::<Self>())
    }

    fn step(&mut self, input: &semio_framework_plugin::retained_command::ArtifactCommandInputs<'_, EditorApp<Puzzle2dPlayApp>>, _cx: &mut StepContext<'_>) -> Result<semio_framework_plugin::retained_command::ArtifactCommandWorkStep<EditorApp<Puzzle2dPlayApp>>, Fault> {
        let semio_framework_plugin::retained_command::ArtifactCommandInputs { command: _command, snapshot, config: _config, interaction: _interaction, hover: _hover, context: _context, .. } = *input;
        let nodes = snapshot.value().get("nodes").and_then(Value::as_array).ok_or_else(|| Fault::from("puzzle2d-redraw-nodes-missing"))?;
        match self.stage {
            Puzzle2dRedrawStage::Nodes | Puzzle2dRedrawStage::Handles => {
                for _ in 0..PUZZLE2D_REDRAW_UNITS_PER_STEP {
                    match self.stage {
                        Puzzle2dRedrawStage::Nodes => self.index_node_one(nodes)?,
                        Puzzle2dRedrawStage::Handles => self.index_handle_one(nodes)?,
                        _ => break,
                    }
                }
                Ok(crate::puzzle_progress_step!("puzzle2d-redraw-node", "Indexing board node", "Board-Knoten wird indiziert"))
            }
            Puzzle2dRedrawStage::Edges => {
                let edges = snapshot.value().get("edges").and_then(Value::as_array);
                for _ in 0..PUZZLE2D_REDRAW_UNITS_PER_STEP {
                    if self.stage != Puzzle2dRedrawStage::Edges {
                        break;
                    }
                    self.resolve_edge_one(edges)?;
                }
                Ok(crate::puzzle_progress_step!("puzzle2d-redraw-edge", "Resolving handle angle", "Anschlusswinkel wird bestimmt"))
            }
            Puzzle2dRedrawStage::Emit => {
                for _ in 0..PUZZLE2D_REDRAW_UNITS_PER_STEP {
                    if self.publish_handle_one(nodes)? {
                        let mutations = std::mem::take(&mut self.mutations);
                        return Ok(semio_framework_plugin::retained_command::ArtifactCommandWorkStep::Complete(Emit { artifact_mutations: mutations, ui_scope: UiDirtyScope::Full, ..Default::default() }));
                    }
                }
                Ok(crate::puzzle_progress_step!("puzzle2d-redraw-emit", "Publishing handle angle", "Anschlusswinkel wird veröffentlicht"))
            }
            Puzzle2dRedrawStage::Complete => Err(Fault::from("puzzle2d-redraw-complete-repolled")),
            Puzzle2dRedrawStage::Closing => Err(Fault::from("puzzle2d-redraw-closing")),
        }
    }

    fn begin_close(&mut self) {
        self.stage = Puzzle2dRedrawStage::Closing;
        self.close_owners.stage(Puzzle2dRedrawHandlesWorkOwners { shapes: std::mem::take(&mut self.shapes), handle_locations: std::mem::take(&mut self.handle_locations), angles: std::mem::take(&mut self.angles), mutations: std::mem::take(&mut self.mutations) });
    }

    fn close_step(&mut self, grant: semio_framework_value::retained_clone::RetainedCloneGrant) -> semio_framework_job::InteractiveJobCloseStep {
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
        self.stage == Puzzle2dRedrawStage::Closing && self.close_owners.is_empty()
    }

    fn terminal_frame_release_bytes(&self) -> Option<usize> {
        Some(size_of::<Self>())
    }
}
//#endregion 🧵️RetainedCommands
//#endregion 🔖️PlayApp
