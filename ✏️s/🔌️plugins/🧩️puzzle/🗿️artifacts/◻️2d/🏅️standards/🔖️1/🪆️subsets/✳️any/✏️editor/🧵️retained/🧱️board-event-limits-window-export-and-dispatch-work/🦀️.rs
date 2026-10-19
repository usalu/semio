//! 🧱️ Board event limits, window, export and dispatch work: the board event batch limit and extent, the window command work, the revision hex, the export work, the dispatch emit and the generic and active example extents. See [`super`].

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

pub fn puzzle2d_board_events_extent(command: &Puzzle2dCommand, _snapshot: &Puzzle2dPlaySnapshot, _interaction: &protocol::InteractionState) -> Option<usize> {
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

pub struct Puzzle2dWindowCommandWork<A: Puzzle2dEditor> {
    pub tool_id: &'static str,
    pub extent: crate::retained_command::PuzzleCommandExtent<EditorApp<A>>,
    pub authoring_seed: String,
    pub base_revision: String,
    consumed: bool,
}

impl<A: Puzzle2dEditor> Puzzle2dWindowCommandWork<A> {
    pub fn new(tool_id: &'static str, extent: crate::retained_command::PuzzleCommandExtent<EditorApp<A>>, authoring_seed: String, base_revision: String) -> Self {
        Self { tool_id, extent, authoring_seed, base_revision, consumed: false }
    }
}

/// 📐️ A canonical document revision as lowercase hex — the identity a persisted select-tool gesture pins its base by.
pub fn puzzle2d_revision_hex(revision: &[u8; 32]) -> String {
    revision.iter().map(|byte| format!("{byte:02x}")).collect()
}

impl<A: Puzzle2dEditor> semio_framework_plugin::retained_command::ArtifactCommandWork<EditorApp<A>> for Puzzle2dWindowCommandWork<A> {
    fn tool_id(&self) -> &'static str {
        self.tool_id
    }

    fn extent(&self, command: &Puzzle2dCommand, snapshot: &Puzzle2dPlaySnapshot, interaction: &protocol::InteractionState, _context: Option<&semio_framework_plugin::app::ArtifactOwnedToolJobContext<EditorApp<A>>>) -> Option<usize> {
        (self.extent)(command, snapshot, interaction)
    }

    fn work_demands(&self, _input: &semio_framework_plugin::retained_command::ArtifactCommandInputs<'_, EditorApp<A>>, _maximum_copy_bytes: usize) -> Result<semio_framework_value::RetirementDemand, semio_framework_value::ValueError> {
        crate::retained_command::step_demands::<EditorApp<A>>(size_of::<Self>())
    }

    fn step(&mut self, input: &semio_framework_plugin::retained_command::ArtifactCommandInputs<'_, EditorApp<A>>, _cx: &mut StepContext<'_>) -> Result<semio_framework_plugin::retained_command::ArtifactCommandWorkStep<EditorApp<A>>, semio_framework_job::OriginalTurnFailure> {
        let original: Result<_, Fault> = (|| {
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
        let (emit, ephemeral) = puzzle2d_dispatch_emit::<A>(command, snapshot, config, &window_config, &window_transient, window_kind, view_state, puzzle2d_active_utility(view_state), &selection, &self.authoring_seed, &self.base_revision, gesture, None)?;
        self.consumed = true;
        Ok(semio_framework_plugin::retained_command::ArtifactCommandWorkStep::CompleteWithEphemeral { emit, ephemeral })
    
        })();
        original.map_err(semio_framework_job::OriginalTurnFailure::from)
    }
}

/// 📤 `exportSnapshot` reads the document and publishes a download; it owns no mutation, so it resolves
/// from the snapshot and picks its lane by payload size: one inline effect under the guest's contiguous
/// request ceiling, the framework's segmented-download lane above it, a notice above what one segmented
/// download may carry.
#[derive(Default)]
pub struct Puzzle2dExportWork {
    consumed: bool,
}

impl<A: Puzzle2dEditor> semio_framework_plugin::retained_command::ArtifactCommandWork<EditorApp<A>> for Puzzle2dExportWork {
    fn tool_id(&self) -> &'static str {
        "exportSnapshot"
    }

    fn extent(&self, _command: &Puzzle2dCommand, _snapshot: &Puzzle2dPlaySnapshot, _interaction: &protocol::InteractionState, _context: Option<&semio_framework_plugin::app::ArtifactOwnedToolJobContext<EditorApp<A>>>) -> Option<usize> {
        Some(1)
    }

    fn work_demands(&self, _input: &semio_framework_plugin::retained_command::ArtifactCommandInputs<'_, EditorApp<A>>, _maximum_copy_bytes: usize) -> Result<semio_framework_value::RetirementDemand, semio_framework_value::ValueError> {
        crate::retained_command::step_demands::<EditorApp<A>>(size_of::<Self>())
    }

    fn step(&mut self, input: &semio_framework_plugin::retained_command::ArtifactCommandInputs<'_, EditorApp<A>>, _cx: &mut StepContext<'_>) -> Result<semio_framework_plugin::retained_command::ArtifactCommandWorkStep<EditorApp<A>>, semio_framework_job::OriginalTurnFailure> {
        let original: Result<_, Fault> = (|| {
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
    
        })();
        original.map_err(semio_framework_job::OriginalTurnFailure::from)
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
pub fn puzzle2d_dispatch_emit<A: Puzzle2dEditor>(
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
) -> Result<(Emit<Puzzle2dMutation, Puzzle2dConfigMutation>, EphemeralEmit<EditorApp<A>>), Fault> {
    let (action, args, window_id) = (command.action_id(), command.args(), command.window_id());
    let before = document.value();
    let active_utility = active_utility.to_string();
    let runtime = window::runtime(config, window_config, window_transient, Some(window_kind));
    let interaction = Puzzle2dInteractionSnapshot { granularity: selection.granularity.clone(), selected: selection.ids.clone(), ..Default::default() };
    let mut scene = puzzle2d_scene_with(before.clone(), runtime, &active_utility, interaction);
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
pub fn puzzle2d_generic_extent(command: &Puzzle2dCommand, _snapshot: &Puzzle2dPlaySnapshot, interaction: &protocol::InteractionState) -> Option<usize> {
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
pub fn puzzle2d_active_example_emit(command: &Puzzle2dCommand, _snapshot: &Puzzle2dPlaySnapshot, _config: &Puzzle2dConfig) -> Result<Emit<Puzzle2dMutation, Puzzle2dConfigMutation>, Fault> {
    let id = set_active_example::canonical_example_id(command.args().and_then(|args| args.get("exampleId")).and_then(Value::as_str).unwrap_or(""));
    Ok(Emit { effects: vec![puzzle2d_load_document_effect(set_active_example::target(id))], ui_scope: UiDirtyScope::Full, ..Default::default() })
}

/// ➕️ `addNode` places one node through the shared window-command dispatch, so it is one admitted unit.
pub fn puzzle2d_add_node_extent(command: &Puzzle2dCommand, _snapshot: &Puzzle2dPlaySnapshot, _interaction: &protocol::InteractionState) -> Option<usize> {
    (command.action_id() == "addNode").then_some(1)
}

pub fn puzzle2d_active_example_reduce(
    command: &Puzzle2dCommand,
    snapshot: &Puzzle2dPlaySnapshot,
    config: &Puzzle2dConfig,
    _interaction: &protocol::InteractionState,
    _hover: &semio_framework_plugin::app::InteractionHoverState,
    _view_state: Option<&semio_framework_plugin::ViewModel>,
) -> Result<Emit<Puzzle2dMutation, Puzzle2dConfigMutation>, Fault> {
    puzzle2d_active_example_emit(command, snapshot, config)
}

pub fn puzzle2d_active_example_extent(command: &Puzzle2dCommand, _snapshot: &Puzzle2dPlaySnapshot, _interaction: &protocol::InteractionState) -> Option<usize> {
    (command.action_id() == "setActiveExample").then_some(1)
}
//#endregion 🧵️RetainedCommands
//#endregion 🔖️PlayApp
