//! 🍠️ Play app: the stateless 5d play app and its action dispatch. See [`super`].

use super::*;

//#region 🔖️PlayApp
// 🧩️ B1: Puzzle-5d play app. Stateless: the persisted document (bare `Puzzle5dDocument` json) lives in
// the wrapping `VcsArtifactApp`'s document store, the ephemeral view state in the wrapping store's real,
// VCS-tracked `Puzzle5dConfig` artifact (see `🦀️config.rs`) — every read comes from `cfg.snapshot`, every write
// flows out as a `Puzzle5dConfigMutation` in the returned `Emit` — and what an instance retains across turns
// (the brush suggestions link) in its instance operation owner.
// Each action mutates a transient {@link Puzzle5dScene}, then emits the granular operation delta.
// Undo/redo/checkpoints are handled by the wrapper.
fn with_puzzle5d_app<R>(f: impl FnOnce(&Puzzle5dPlayApp) -> R) -> R {
    let app = Puzzle5dPlayApp;
    f(&app)
}

#[derive(Default)]
pub struct Puzzle5dPlayApp;

impl Puzzle5dPlayApp {
    /// 🧩️ B1: the pure per-action core, dispatched into by `ArtifactApp::handle` with
    /// `action`/`args`/`window_id` reconstructed 1:1 from the typed `Puzzle5dCommand`. Everything past
    /// this adapter boundary reads/writes the passed-in `Puzzle5dConfig` snapshot and returns a real
    /// `Emit` (document + config operations) instead of mutating `self`.
    fn handle_action_impl(
        &self,
        authoring_seed: &str,
        action: &str,
        args: Option<&Value>,
        window_id: Option<&str>,
        snapshot: &Puzzle5dPlaySnapshot,
        config: &Puzzle5dRuntime,
        view_state: Option<&semio_framework_plugin::ViewModel>,
        selection: &protocol::DomainSelection,
        instance_owner: Option<&semio_framework_plugin::ArtifactInstanceOperationOwnerHandle>,
        tool_run: Option<&semio_framework_plugin::ToolRunView>,
    ) -> (Emit<Puzzle5dMutation, Puzzle5dConfigMutation>, EphemeralEmit<EditorApp<Puzzle5dPlayApp>>) {
        let projection = puzzle5d_editor_projection(snapshot);
        let shared_before = window_ownership::shared(config);
        let window_before = window_ownership::config_from_runtime(config);
        let transient_before = window_ownership::transient_from_runtime(config, window_id.unwrap_or(world3d::WINDOW_KIND_ID));
        let active_utility_initial = puzzle5d_scene_active_utility(view_state, window_id);
        let wid = window_id.map_or_else(|| world3d::WINDOW_KIND_ID.to_string(), str::to_string);
        let window_kind = view_state.and_then(window_ownership::kind_for_view).unwrap_or(world3d::WINDOW_KIND_ID);
        let mut scene = scene_from_projection(&projection, config.clone(), &active_utility_initial);
        let mut ctx = Puzzle5dActionCtx {
            scene: &mut scene,
            snapshot,
            instance_owner,
            window_id: &wid,
            window_kind,
            selection,
            view_state,
            tool_run,
            effects: Vec::new(),
            interaction_writes: Vec::new(),
            abort: false,
            authoring_seed,
            artifact_mutations: Vec::new(),
            transaction: None,
        };
        dispatch_puzzle5d_action(&mut ctx, action, args);
        let aborted = ctx.abort;
        let mut arm_effects = std::mem::take(&mut ctx.effects);
        let interaction_writes = std::mem::take(&mut ctx.interaction_writes);
        let tool_mutations = std::mem::take(&mut ctx.artifact_mutations);
        let transaction = ctx.transaction.take();
        if aborted {
            return (Emit { effects: arm_effects, ui_scope: UiDirtyScope::None, ..Default::default() }, EphemeralEmit::default());
        }
        let next_active_utility = scene.active_utility.clone();
        let operations: Vec<Puzzle5dMutation> = tool_mutations;
        let transaction = transaction.filter(|_| !operations.is_empty());
        let initial_is_fill_tool = active_utility_initial == fill_tool::TOOL_ID;
        let next_is_fill_tool = next_active_utility == fill_tool::TOOL_ID;
        if next_is_fill_tool && !initial_is_fill_tool {
            arm_effects.push(Effect::SetActiveTool { tool_id: fill_tool::TOOL_ID.into() });
        }
        if !next_is_fill_tool && next_active_utility != active_utility_initial {
            arm_effects.push(Effect::SetActiveUtility { window_id: wid.clone(), utility_id: next_active_utility.clone() });
        }
        let effects = arm_effects;
        let shared_after = window_ownership::shared(&scene.runtime);
        let config_mutations = shared_before.mutations_to(&shared_after);
        let window_after = window_ownership::config_from_runtime(&scene.runtime);
        let window_config_mutations = if window_after != window_before {
            view_state.and_then(|view| window_ownership::addressed_config(view, window_after).ok()).into_iter().collect()
        } else {
            Vec::new()
        };
        let transient_after = window_ownership::transient_from_runtime(&scene.runtime, window_id.unwrap_or(world3d::WINDOW_KIND_ID));
        let window_transient = if transient_after != transient_before {
            view_state.and_then(|view| window_ownership::addressed_transient(view, transient_after).ok()).into_iter().collect()
        } else {
            Vec::new()
        };
        (Emit { artifact_mutations: operations, config_mutations, window_config_mutations, transaction, effects, interaction_writes, ..Default::default() }, EphemeralEmit { window_transient, ..Default::default() })
    }
}

/// 🎬️ Dispatch only: every arm's behaviour lives in its `🎮️commands/<group>/🦀️.rs` free
/// function. No behaviour lives in this match.
fn dispatch_puzzle5d_action(ctx: &mut Puzzle5dActionCtx<'_>, action: &str, args: Option<&Value>) {
    match action {
        "importSnapshot" => import_snapshot::import_snapshot(ctx, args),
        "openImportSnapshot" => open_import_snapshot::open_import_snapshot(ctx),
        "openAddPartDialog" => open_add_part_dialog::open_add_part_dialog(ctx),
        "selectSameKindSelection" => select_same_kind::select_same_kind(ctx),
        "deleteSelection" => delete_selection::delete_selection(ctx),
        "duplicateSelection" => duplicate_selection::duplicate_selection(ctx),
        "setSelectionFlag" => set_selection_flag::set_selection_flag(ctx, args),
        "setSelectionHidden" => set_selection_flag::set_selection_flag_value(ctx, args, "hidden"),
        "setSelectionLocked" => set_selection_flag::set_selection_flag_value(ctx, args, "locked"),
        "setCamera" => set_camera::set_camera(ctx, args),
        "setCamera2d" => set_camera_2d::set_camera_2d(ctx, args),
        "setCamera3d" => set_camera_3d::set_camera_3d(ctx, args),
        "focusSelection" => focus_selection::focus_selection(ctx),
        "toggleSun" | "setSunAzimuth" | "setSunElevation" | "setSunIntensity" => apply_sun::apply(ctx, action, args),
        "setLodMode" => set_lod_mode::set_lod_mode(ctx, args),
        "setGridSnapEnabled" => set_grid_snap_enabled::set_grid_snap_enabled(ctx, args),
        "setGridFactor" => set_grid_factor::set_grid_factor(ctx, args),
        "setGridVisible" => set_grid_visible::set_grid_visible(ctx, args),
        "setGridSpacing" => set_grid_spacing::set_grid_spacing(ctx, args),
        "setProjection" | "setProjectionParam" => set_projection::set_projection(ctx, action, args),
        "setGripShow" => set_grip_show::set_grip_show(ctx, args),
        "setGripDirection" => set_grip_direction::set_grip_direction(ctx, args),
        "setSelectableKind" => set_selectable_kind::set_selectable_kind(ctx, args),
        "setLodAutomatic" => set_lod_automatic::set_lod_automatic(ctx, args),
        "setLodDepthVariable" => set_lod_depth_variable::set_lod_depth_variable(ctx, args),
        "setLodManual" => set_lod_manual::set_lod_manual(ctx, args),
        "setTransformGumballFlag" => set_transform_gumball_flag::set_transform_gumball_flag(ctx, args),
        "cycleBrushCandidate" => cycle_brush_candidate::cycle_brush_candidate(ctx),
        "cycleBrushCandidateBack" => cycle_brush_candidate::cycle_brush_candidate_back(ctx),
        "targetBrushSuggestions" => target_brush_suggestions::target_brush_suggestions(ctx, args),
        "openVortexSuggestions" => open_vortex_suggestions::open_vortex_suggestions(ctx, args),
        "closeVortexSuggestions" => close_vortex_suggestions::close_vortex_suggestions(ctx, args),
        "hoverSuggestion" => hover_suggestion::hover_suggestion(ctx, args),
        "acceptSuggestion" => accept_suggestion::accept_suggestion(ctx, args),
        "registerBrushMesh" => register_brush_mesh::register_brush_mesh(ctx, args),
        "setBrushPlacementContactTolerance" => set_brush_placement_contact_tolerance::set_brush_placement_contact_tolerance(ctx, args),
        "setProximityRadius" => set_proximity_radius::set_proximity_radius(ctx, args),
        "setChunkSize" => set_chunk_size::set_chunk_size(ctx, args),
        "setPartKindWeight" | "setGripKindWeight" => set_kind_weight::set_kind_weight(ctx, action, args),
        "engagementControlSelect" => engagement_control_select::engagement_control_select(ctx, args),
        "setSuggestionOffset" => set_suggestion_offset::set_suggestion_offset(ctx, args),
        "setFillCount" => set_fill_count::set_fill_count(ctx, args),
        "engagementInput" => engagement_input::engagement_input(ctx, args),
        "engagementSubmit" => engagement_submit::engagement_submit(ctx, args),
        "engagementRepeatLast" => engagement_repeat_last::engagement_repeat_last(ctx),
        "engagementAbort" => engagement_abort::engagement_abort(ctx, args),
        "translateSelection" => translate_selection::translate_selection(ctx, args),
        "rotateSelection" => rotate_selection::rotate_selection(ctx, args),
        "scaleSelection" => scale_selection::scale_selection(ctx, args),
        "worldRelocate" => world_relocate::world_relocate(ctx, args),
        "addTargetVolume" => add_target_volume::add_target_volume(ctx, args),
        "deleteTargetVolume" => delete_target_volume::delete_target_volume(ctx, args),
        "relocateTargetVolume" => relocate_target_volume::relocate_target_volume(ctx, args),
        "setTargetVolumeFlag" => set_target_volume_flag::set_target_volume_flag(ctx, args),
        "setTargetVolumeHidden" => set_target_volume_flag::set_target_volume_flag_value(ctx, args, "hidden"),
        "setTargetVolumeLocked" => set_target_volume_flag::set_target_volume_flag_value(ctx, args, "locked"),
        "setVoxelDims" => set_voxel_dims::set_voxel_dims(ctx, args),
        "worldPointerDown" | "canvasPointerDown" => ctx.abort = true,
        _ => {}
    }
}
//#endregion 🔖️PlayApp
