//! 🎮️ Puzzle 3d action dispatch — the shared action prologue (transient scene, precompute sync, dispatch) every action
//! runs, the one dispatch table from an action id to its command arm, and the shell-only actions that need no scene.

use {crate::editor::puzzle3d::session::Puzzle3dActionCtx, crate::editor::puzzle3d::session::Puzzle3dEditor, crate::editor::puzzle3d::session::Puzzle3dSession, crate::editor::puzzle3d::session::push_precompute_scene, crate::editor::puzzle3d::session::scene_config, crate::editor::puzzle3d::session::seed_one_precompute_mesh_fallback};
use {crate::editor::puzzle3d::snapshot::Puzzle3dInteractionSnapshot, crate::editor::puzzle3d::snapshot::Puzzle3dScene, crate::editor::puzzle3d::snapshot::puzzle3d_scene_active_utility, crate::editor::puzzle3d::snapshot::scene_from_snapshot};
use {crate::editor::puzzle3d::commands::add_target_volume, crate::editor::puzzle3d::commands::apply_sun, crate::editor::puzzle3d::commands::close_vortex_suggestions, crate::editor::puzzle3d::commands::cycle_candidate, crate::editor::puzzle3d::commands::delete_attraction, crate::editor::puzzle3d::commands::delete_selection, crate::editor::puzzle3d::commands::delete_target_volume, crate::editor::puzzle3d::commands::duplicate_selection, crate::editor::puzzle3d::commands::engagement_abort, crate::editor::puzzle3d::commands::export_snapshot, crate::editor::puzzle3d::commands::engagement_control_select, crate::editor::puzzle3d::commands::engagement_input, crate::editor::puzzle3d::commands::engagement_repeat_last, crate::editor::puzzle3d::commands::engagement_submit, crate::editor::puzzle3d::commands::focus_selection, crate::editor::puzzle3d::commands::hover_suggestion, crate::editor::puzzle3d::commands::import_snapshot, crate::editor::puzzle3d::commands::open_import_snapshot, crate::editor::puzzle3d::commands::open_vortex_suggestions, crate::editor::puzzle3d::commands::register_brush_mesh, crate::editor::puzzle3d::commands::relocate_target_volume, crate::editor::puzzle3d::commands::rotate_selection, crate::editor::puzzle3d::commands::scale_selection, crate::editor::puzzle3d::commands::select_same_kind, crate::editor::puzzle3d::commands::set_automatic, crate::editor::puzzle3d::commands::set_brush_placement_contact_tolerance, crate::editor::puzzle3d::commands::set_camera, crate::editor::puzzle3d::commands::set_chunk_size, crate::editor::puzzle3d::commands::set_depth_variable, crate::editor::puzzle3d::commands::set_kind_weight, crate::editor::puzzle3d::commands::set_manual, crate::editor::puzzle3d::commands::set_projection, crate::editor::puzzle3d::commands::set_proximity_radius, crate::editor::puzzle3d::commands::set_selectable_kind, crate::editor::puzzle3d::commands::set_selection_flag, crate::editor::puzzle3d::commands::set_snap_enabled, crate::editor::puzzle3d::commands::set_spacing, crate::editor::puzzle3d::commands::set_target_volume_flag, crate::editor::puzzle3d::commands::set_transform_gumball_flag, crate::editor::puzzle3d::commands::set_visible, crate::editor::puzzle3d::commands::set_vortex_direction, crate::editor::puzzle3d::commands::set_vortex_show, crate::editor::puzzle3d::commands::set_voxel_dims, crate::editor::puzzle3d::commands::target_brush_suggestions, crate::editor::puzzle3d::commands::translate_selection, crate::editor::puzzle3d::commands::world_relocate};
use {crate::editor::puzzle3d::config::Puzzle3dConfigMutation, crate::editor::puzzle3d::config::Puzzle3dRuntime};
use crate::editor::puzzle3d::modes::edit::windows::main;
use crate::editor::puzzle3d::window as window_ownership;
use crate::editor::puzzle3d::snapshot::Puzzle3dPlaySnapshot;
use crate::standards::v1::subsets::any::schema::mutations::Puzzle3dMutation;
use crate::Puzzle3dSnapshot;
use semio_framework::kernel::UiDirtyScope;
use semio_framework_plugin::kernel::Effect;
use semio_framework_plugin::ArtifactInstanceOperationOwnerHandle;
use semio_framework_plugin::EditorApp;
use semio_framework_plugin::Emit;
use semio_framework_plugin::SET_ACTIVE_TOOL_ACTION_ID;
use semio_framework_plugin::SET_ACTIVE_UTILITY_ACTION_ID;
use {semio_framework_pack_json::to_string, semio_framework_pack_json::Value};
use semio_framework_plugin::app::EphemeralEmit;
use crate::editor::puzzle3d::snapshot;

/// 📬️ Persisted and ephemeral effects produced by one Puzzle 3D action.
pub type Puzzle3dActionEmission<A> = (Emit<Puzzle3dMutation, Puzzle3dConfigMutation>, EphemeralEmit<EditorApp<A>>);

/// 🧊️ The session sync's own three phases, in the order that costs ONE engine rebuild instead of
/// one per mesh: seed every owed collision-mesh fallback, build the engine scene, push it.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, semio_framework_value::RetireOwned)]
enum Puzzle3dPrologueSyncStage {
    #[default]
    Meshes,
    Build,
    Push,
    Done,
}

/// 🧾️ The prologue every puzzle3d action shares, split into the three halves a staged retained work
/// spends bounded turns on: `scene_step` materializes the transient scene (and the typed base every concrete
/// mutation of the action is built against), `sync_step`
/// pushes that scene into the precompute session across its own phases, and `dispatch_step` runs the
/// action arm and assembles the `Emit`.
///
/// ⏱️ Ticket 26/09/02/PUZZLE-3D-END-TO-END W-P3. Run whole, this prologue measured 17.6 ms on the
/// 180-object Nakagin document — twice the framework's 8 000 µs interactive step ceiling, inside the
/// one turn every precompute-gated command publishes from. Splitting it is what keeps each half inside
/// that ceiling; making each half typed (`scene_from_snapshot`, `scene_config`,
/// `Puzzle3dKindMeshIndex`) is what keeps every one of them under this artifact's own 2 000 µs budget.
///
/// 🔄️ `runtime`/`active_utility` are refreshed from the caller's own `config` at the top of every step,
/// so a staged work that spends many turns here reads exactly the configuration the framework handed it
/// that turn — only the expensive document-shaped halves are carried across turns.
#[derive(Default, semio_framework_value::RetireOwned)]
pub(crate) struct Puzzle3dActionPrologue {
    scene: Option<Puzzle3dScene>,
    base: Option<std::sync::Arc<Puzzle3dSnapshot>>,
    sync_stage: Puzzle3dPrologueSyncStage,
    built: Option<crate::standards::v1::subsets::any::schema::SceneConfig>,
    tool_run: Option<semio_framework_plugin::ToolRunView>,
}

impl Puzzle3dActionPrologue {
    /// 🧾️ Half one: the transient scene, straight off the snapshot's typed authority, plus the typed base every
    /// concrete mutation of the action is built against.
    pub(crate) fn scene_step(&mut self, snapshot: &Puzzle3dPlaySnapshot, config: &Puzzle3dRuntime, view_state: Option<&semio_framework_plugin::ViewModel>, window_id: Option<&str>) {
        let active_utility = puzzle3d_scene_active_utility(config, view_state, window_id);
        self.base = Some(snapshot.typed_arc());
        self.scene = Some(scene_from_snapshot(snapshot.typed(), config.clone(), &active_utility));
    }

    /// 🧊️ Half two: the precompute session sync, for the actions that read it. The ~forty session-less
    /// actions never touch this half at all. RESUMABLE — exactly one owed collision-mesh fallback per
    /// call, then the engine scene build, then its push; answers whether another call is still owed, so a
    /// staged work spends one bounded turn on each instead of 15 ms on all of them at once.
    pub(crate) fn sync_step(&mut self, session: &Puzzle3dSession, action: &str, config: &Puzzle3dRuntime, view_state: Option<&semio_framework_plugin::ViewModel>, window_id: Option<&str>) -> bool {
        if !puzzle3d_action_uses_precompute(action) {
            return false;
        }
        let stage = match self.sync_stage {
            Puzzle3dPrologueSyncStage::Done => return false,
            Puzzle3dPrologueSyncStage::Meshes => {
                let Some(scene) = self.refreshed(config, view_state, window_id) else {
                    return false;
                };
                let owed = seed_one_precompute_mesh_fallback(&mut session.precompute.borrow_mut(), scene);
                if owed {
                    Puzzle3dPrologueSyncStage::Meshes
                } else {
                    Puzzle3dPrologueSyncStage::Build
                }
            }
            Puzzle3dPrologueSyncStage::Build => {
                self.built = self.refreshed(config, view_state, window_id).and_then(scene_config);
                Puzzle3dPrologueSyncStage::Push
            }
            Puzzle3dPrologueSyncStage::Push => {
                let mut precompute = session.precompute.borrow_mut();
                if let Some(built) = self.built.take() {
                    push_precompute_scene(&mut precompute, built);
                }
                Puzzle3dPrologueSyncStage::Done
            }
        };
        self.sync_stage = stage;
        stage != Puzzle3dPrologueSyncStage::Done
    }

    /// 🎬️ Half three: the action arm itself plus the whole `Emit` assembly — the only half that reads
    /// `args`, mutates the scene and diffs the document. A selection transform never touches the scene: its
    /// arm commits the transform tool's ONE transaction, whose yielded parametric mutations ride this emit
    /// stamped with the transaction ref minted from `authoring_seed`.
    #[allow(clippy::too_many_arguments)]
    pub fn dispatch_step<A: Puzzle3dEditor>(
        &mut self,
        session: &Puzzle3dSession,
        instance_owner: Option<&ArtifactInstanceOperationOwnerHandle>,
        authoring_seed: &str,
        command: &Puzzle3dCommand,
        window_id: Option<&str>,
        config: &Puzzle3dRuntime,
        view_state: Option<&semio_framework_plugin::ViewModel>,
        interaction: &Puzzle3dInteractionSnapshot,
    ) -> Puzzle3dActionEmission<A> {
        let action = command.action_id();
        let args = command.args();
        let active_utility_initial = puzzle3d_scene_active_utility(config, view_state, window_id);
        if self.refreshed(config, view_state, window_id).is_none() {
            return (Emit::default(), EphemeralEmit::default());
        }
        let (Some(mut scene), Some(base)) = (self.scene.take(), self.base.take()) else {
            return (Emit::default(), EphemeralEmit::default());
        };
        let shared_before = window_ownership::shared(config);
        let window_before = window_ownership::Puzzle3dWindowConfig::from_runtime(config);
        let transient_before = window_ownership::transient(config, view_state);
        // 🪟️ This action targets the one exact window owner already composed into `config`.
        let wid = window_id.map_or_else(|| main::WINDOW_KIND_ID.into(), str::to_string);
        let mut ui_scope = puzzle3d_scope(puzzle3d_command_scope_class(action));
        let mut effects = Vec::new();
        let mut interaction_writes = Vec::new();
        let mut artifact_mutations = Vec::new();
        let mut transaction = None;
        let mut ctx = Puzzle3dActionCtx {
            session,
            instance_owner,
            tool_run: self.tool_run.as_ref(),
            scene: &mut scene,
            window_id: &wid,
            config,
            view_state,
            interaction,
            ui_scope: &mut ui_scope,
            effects: &mut effects,
            interaction_writes: &mut interaction_writes,
            abort: false,
            base: &base,
            authoring_seed,
            artifact_mutations: &mut artifact_mutations,
            transaction: &mut transaction,
        };
        dispatch_puzzle3d_action(&mut ctx, action, args);
        let aborted = ctx.abort;
        if aborted {
            // 🧯️ An aborted arm emits no document/config delta — but the refusal NOTICE it pushed is the
            // user-visible half of that refusal and must survive, or "refused" and "silently did nothing"
            // look identical (exactly how `duplicateSelection` presented in the browser). Effects travel
            // on their own lane, so keeping them costs no mutation and no undo entry; the scope stays
            // `None` because nothing was painted.
            return (Emit { effects, ui_scope: UiDirtyScope::None, ..Default::default() }, EphemeralEmit::default());
        }
        let next_active_utility = scene.active_utility.clone();
        let operations = artifact_mutations;
        let transaction = transaction.filter(|_| !operations.is_empty());
        // 🧰️🛠️ Programmatic utility/tool switches push the host session. `setActiveTool` itself never
        // re-emits. Entering fill emits `SetActiveTool { fill }` only. Leaving fill is exclusively a
        // host `setActiveTool ""` — an empty tool effect here bounce-disarms a just-armed fill (Escape
        // `engagementAbort`, mid-flow utility-field rewrite). A real utility change still emits
        // `SetActiveUtility` (host mutual exclusion clears the tool when the utility is non-empty).
        let is_direct_tool_switch = action == SET_ACTIVE_TOOL_ACTION_ID;
        let is_direct_utility_switch = action == SET_ACTIVE_UTILITY_ACTION_ID;
        let initial_is_fill_tool = active_utility_initial == crate::editor::puzzle3d::snapshot::PUZZLE3D_FILL_TOOL_ID;
        let next_is_fill_tool = next_active_utility == crate::editor::puzzle3d::snapshot::PUZZLE3D_FILL_TOOL_ID;
        if !is_direct_tool_switch && next_is_fill_tool && !initial_is_fill_tool {
            effects.push(Effect::SetActiveTool { tool_id: crate::editor::puzzle3d::snapshot::PUZZLE3D_FILL_TOOL_ID.into() });
        }
        if !is_direct_utility_switch && !is_direct_tool_switch && !next_is_fill_tool && next_active_utility != active_utility_initial {
            effects.push(Effect::SetActiveUtility { window_id: wid.clone(), utility_id: next_active_utility });
        }
        // 🧮️ B1: only a REAL config change becomes a `Puzzle3dConfigMutation` — `PartialEq` (derived)
        // makes this cheap, and keeps a pure read-only action (e.g. a re-materialize/re-save of an
        // already-idle window's options) from creating a no-op undo entry.
        let shared_after = window_ownership::shared(&scene.runtime);
        let config_mutations = shared_before.mutations_to(&shared_after);
        let window_after = window_ownership::Puzzle3dWindowConfig::from_runtime(&scene.runtime);
        let window_config_mutations = if window_after != window_before { vec![window_ownership::addressed_config_for(&wid, window_after)] } else { Vec::new() };
        let transient_after = window_ownership::transient(&scene.runtime, view_state);
        let window_transient = if transient_after != transient_before { vec![window_ownership::addressed_transient_for(&wid, transient_after)] } else { Vec::new() };
        (Emit { artifact_mutations: operations, config_mutations, window_config_mutations, transaction, effects, ui_scope, interaction_writes, ..Default::default() }, EphemeralEmit { window_transient, ..Default::default() })
    }

    /// 🔄️ Re-materializes the carried scene's view-local halves against the configuration of the turn
    /// that is about to read them.
    fn refreshed(&mut self, config: &Puzzle3dRuntime, view_state: Option<&semio_framework_plugin::ViewModel>, window_id: Option<&str>) -> Option<&Puzzle3dScene> {
        let scene = self.scene.as_mut()?;
        scene.runtime = config.clone();
        scene.active_utility = puzzle3d_scene_active_utility(config, view_state, window_id);
        Some(scene)
    }

    /// 🧹️ One retained owner per bounded grant, for a staged work's own close cursor.
    pub(crate) fn close_one(&mut self) -> bool {
        self.scene.take().is_some() || self.base.take().is_some() || self.built.take().is_some()
    }

    pub(crate) fn is_empty(&self) -> bool {
        self.scene.is_none() && self.base.is_none() && self.built.is_none()
    }

    /// ⏱️ Turns one staged run of this prologue may take: one for the scene, one per mesh identity the
    /// document seeds a collision fallback for, one that finds none owed, one to build the engine scene,
    /// one to push it, and one to dispatch. A document past this refuses on its work's own capacity
    /// guard rather than running long.
    pub(crate) const WORK_ITEMS: usize = crate::retained_command::PUZZLE_COMMAND_DECODED_ITEMS + 5;
}

/// 🗨️ The two actions with no document interaction at all, hence no scene scaffolding:
/// `openAddObjectDialog` opens the declared dialog over a glass veil and `openImportSnapshot` asks the host
/// for a file.
pub fn puzzle3d_shell_only_emit<A: Puzzle3dEditor>(action: &str) -> Option<Puzzle3dActionEmission<A>> {
    match action {
        "openAddObjectDialog" => Some((Emit::effect(Effect::OpenDialog { req: semio_framework_plugin::RequestId(120), dialog_id: "addObject".into(), args: None }), EphemeralEmit::default())),
        "openImportSnapshot" => Some((
            Emit {
                effects: vec![Effect::RequestFileOpen {
                    req: semio_framework_plugin::RequestId(121),
                    accept: "application/json,.json".into(),
                    read_as: Some("text".into()),
                    import_action: "importSnapshot".into(),
                    multiple: false,
                args: None, }],
                ui_scope: UiDirtyScope::None,
                ..Default::default()
            },
            EphemeralEmit::default(),
        )),
        _ => None,
    }
}

/// 🎬️ Dispatch only: every arm's behaviour lives in its `🎮️commands/<group>/🦀️.rs` free
/// function. No behaviour lives in this match.
fn dispatch_puzzle3d_action(ctx: &mut Puzzle3dActionCtx<'_>, action: &str, args: Option<&Value>) {
    match action {
        "selectSameKindSelection" => select_same_kind::select_same_kind(ctx),
        "setSelectableKind" => set_selectable_kind::set_selectable_kind(ctx, args),
        "deleteSelection" => delete_selection::delete_selection(ctx),
        "duplicateSelection" => duplicate_selection::duplicate_selection(ctx),
        "exportSnapshot" => export_snapshot::export_snapshot(ctx),
        "importSnapshot" => import_snapshot::import_snapshot(ctx, args),
        "openImportSnapshot" => open_import_snapshot::open_import_snapshot(ctx),
        "setSelectionFlag" => set_selection_flag::set_selection_flag(ctx, args),
        "setSelectionHidden" => set_selection_flag::set_selection_flag_value(ctx, args, "hidden"),
        "setSelectionLocked" => set_selection_flag::set_selection_flag_value(ctx, args, "locked"),
        "deleteAttraction" => delete_attraction::delete_attraction(ctx, args),
        "addTargetVolume" => add_target_volume::add_target_volume(ctx, args),
        "deleteTargetVolume" => delete_target_volume::delete_target_volume(ctx, args),
        "setTargetVolumeFlag" => set_target_volume_flag::set_target_volume_flag(ctx, args),
        "relocateTargetVolume" => relocate_target_volume::relocate_target_volume(ctx, args),
        "setCamera" => set_camera::set_camera(ctx, args),
        "setProjection" | "setProjectionParam" => set_projection::set_projection(ctx, action, args),
        "focusSelection" => focus_selection::focus_selection(ctx),
        "toggleSun" | "setSunAzimuth" | "setSunElevation" | "setSunIntensity" => apply_sun::apply(ctx, action, args),
        "setLodAutomatic" => set_automatic::set_automatic(ctx, args),
        "setLodDepthVariable" => set_depth_variable::set_depth_variable(ctx, args),
        "setLodManual" => set_manual::set_manual(ctx, args),
        "setGridVisible" => set_visible::set_visible(ctx, args),
        "setGridSnapEnabled" => set_snap_enabled::set_snap_enabled(ctx, args),
        "setGridSpacing" => set_spacing::set_spacing(ctx, args),
        "setProximityRadius" => set_proximity_radius::set_proximity_radius(ctx, args),
        "setChunkSize" => set_chunk_size::set_chunk_size(ctx, args),
        "setBrushPlacementContactTolerance" => set_brush_placement_contact_tolerance::set_brush_placement_contact_tolerance(ctx, args),
        "setVoxelDims" => set_voxel_dims::set_voxel_dims(ctx, args),
        "setTransformGumballFlag" => set_transform_gumball_flag::set_transform_gumball_flag(ctx, args),
        "setVortexShow" => set_vortex_show::set_vortex_show(ctx, args),
        "setVortexDirection" => set_vortex_direction::set_vortex_direction(ctx, args),
        "translateSelection" => translate_selection::translate_selection(ctx, args),
        "rotateSelection" => rotate_selection::rotate_selection(ctx, args),
        "scaleSelection" => scale_selection::scale_selection(ctx, args),
        "worldRelocate" => world_relocate::world_relocate(ctx, args),
        "cycleBrushCandidate" | "cycleBrushCandidateBack" => cycle_candidate::cycle_candidate(ctx, action, args),
        "openVortexSuggestions" => open_vortex_suggestions::open_vortex_suggestions(ctx, args),
        "closeVortexSuggestions" => close_vortex_suggestions::close_vortex_suggestions(ctx, args),
        "hoverSuggestion" => hover_suggestion::hover_suggestion(ctx, args),
        "targetBrushSuggestions" => target_brush_suggestions::target_brush_suggestions(ctx, args),
        "registerBrushMesh" => register_brush_mesh::register_brush_mesh(ctx, args),
        "engagementControlSelect" => engagement_control_select::engagement_control_select(ctx, args),
        "setObjectKindWeight" | "setVortexKindWeight" => set_kind_weight::set_kind_weight(ctx, action, args),
        "engagementInput" => engagement_input::engagement_input(ctx, args),
        "engagementSubmit" => engagement_submit::engagement_submit(ctx, args),
        "engagementRepeatLast" => engagement_repeat_last::engagement_repeat_last(ctx),
        "engagementAbort" => engagement_abort::engagement_abort(ctx),
        "worldPointerDown" => {}
        _ => {}
    }
}

/// 🧊️ Whether an action's arm READS the precompute session's synced view of the scene, and therefore
/// owes [`Puzzle3dCommandPrologue::sync_step`]'s three stages — one owed mesh fallback per turn, the
/// engine scene build, its push — before it runs.
///
/// 🥽️ `registerBrushMesh` is deliberately NOT one of them, even though it is the action that touches
/// the session most directly: it only ever WRITES geometry into it. Syncing first was not merely dead
/// work, it was work the same command then threw away — the sync seeds a scaled-box fallback for every
/// mesh id the session holds no geometry for (exactly the ids a `registerBrushMesh` run is about to
/// supply) and each seeding clears the brush cache and rebuilds the queue, then the arm's own install
/// clears them again. Seven serialized announcements each paid a whole document scene build plus that
/// rebuild, which is where the 0.65 s → 2.85 s per call measured on 2026-09-09 lived — the arm itself
/// is two `HashMap` lookups. The next scene-reading action re-syncs, so nothing downstream is stale.
fn puzzle3d_action_uses_precompute(action: &str) -> bool {
    matches!(
        action,
        "setBrushPlacementContactTolerance"
            | "addBrushObject"
            | "acceptSuggestion"
            | "setObjectKindWeight"
            | "setVortexKindWeight"
            | "engagementRepeatLast"
    )
}

/// 🧩️ B1: the pure per-action core, dispatched into by `ArtifactApp::handle` with
/// `action`/`args`/`window_id` reconstructed 1:1 from the typed `Puzzle3dCommand`. Everything past
/// this adapter boundary reads/writes the passed-in `Puzzle3dConfig` snapshot and returns a real
/// `Emit` (document + config operations) instead of mutating `self`.
///
/// ⏱️ The three halves are [`Puzzle3dActionPrologue`]'s own steps, so a staged retained work can
/// spend one bounded turn on each instead of paying all of them in its single publish turn — see
/// that type. This one-call shape is what the unstaged call sites (render, tool measures, the
/// non-retained dispatch adapter) use, and is literally the three steps in a row.
pub fn puzzle3d_handle_action<A: Puzzle3dEditor>(
    session: &Puzzle3dSession,
    authoring_seed: &str,
    command: &Puzzle3dCommand,
    window_id: Option<&str>,
    snapshot: &Puzzle3dPlaySnapshot,
    config: &Puzzle3dRuntime,
    view_state: Option<&semio_framework_plugin::ViewModel>,
    interaction: &Puzzle3dInteractionSnapshot,
) -> Puzzle3dActionEmission<A> {
    let action = command.action_id();
    if let Some(shell) = puzzle3d_shell_only_emit(action) {
        return shell;
    }
    let mut prologue = Puzzle3dActionPrologue::default();
    prologue.scene_step(snapshot, config, view_state, window_id);
    let mut sync_turns = 0_u32;
    while prologue.sync_step(session, action, config, view_state, window_id) {
        sync_turns += 1;
        if sync_turns >= 8 {
            break;
        }
    }
    prologue.dispatch_step(session, None, authoring_seed, command, window_id, config, view_state, interaction)
}

use semio_framework_plugin::InteractionVerb;
use semio_framework_plugin::FRAMEWORK_HISTORY_BODY_KEY;
use crate::editor::puzzle3d::snapshot::puzzle3d_viewport_scope;

/// 🐢️ Fill/distribution slider gestures refresh the world body, fill-tool measures and utility-option
/// window measures — never the full shell chrome.
pub fn puzzle3d_fill_options_scope() -> UiDirtyScope {
    UiDirtyScope::Partial { window_bodies: vec![main::BODY_KEY.to_string()], panel_bodies: Vec::new(), utilities: false, tools: true, engagements: false, measures: true, labels: false }
}

/// 🪟️ One per-window option repaints the world body it configures, the measures rail that binds every
/// one of its controls, and the Settings panel — which mirrors four of the same fields and must never
/// disagree with the rail about them. Nothing else: not the outliner, not the catalogue, not the
/// inspector, not the history, not the labels.
///
/// 🐢️ These verbs fell through the table to [`Puzzle3dScopeClass::Chrome`], i.e. `UiDirtyScope::Full`,
/// so ONE grid toggle re-rendered every window body (180 Nakagin instances), every panel body and the
/// whole label overlay before the rail's own checkbox could show the new value. Measured on the live
/// `:6013` shell (ticket 26/09/02/PUZZLE-3D-END-TO-END wave B12): 0.7 s idle, seconds under load — long
/// enough that a second click lands on a control still rendering the value the user just left.
pub fn puzzle3d_window_option_scope() -> UiDirtyScope {
    UiDirtyScope::Partial {
        window_bodies: vec![main::BODY_KEY.to_string()],
        panel_bodies: vec![settings_panel::BODY_KEY.to_string()],
        utilities: false,
        tools: false,
        engagements: false,
        measures: true,
        labels: false,
    }
}

/// 🔍️ The panel bodies a SELECTION change invalidates: the field inspector (it renders the selected
/// entity's own fields, `panels::inspection::render`), the artifact outliner (it marks the selected
/// rows) and the framework history panel (the framework records one command row per
/// `interactionSelect`). The catalogue lists object KINDS and the settings panel reads window options
/// — neither can move when only the selection moves.
pub fn puzzle3d_selection_panel_bodies() -> Vec<String> {
    vec![inspection::BODY_KEY.to_string(), artifact::BODY_KEY.to_string(), FRAMEWORK_HISTORY_BODY_KEY.to_string()]
}

/// 📝️ The panel bodies a DOCUMENT edit invalidates: everything a selection change does, plus the
/// catalogue (object/vortex kind rosters are document data).
pub fn puzzle3d_artifact_panel_bodies() -> Vec<String> {
    let mut bodies = puzzle3d_selection_panel_bodies();
    bodies.push(catalogue::BODY_KEY.to_string());
    bodies
}

/// 🕹️ The scope a SELECTION change invalidates: the world body every window renders the marks into,
/// [`puzzle3d_selection_panel_bodies`] (inspector + outliner rows + history) and the window measures
/// whose Select controls bind the active mode/granularity. Never the catalogue, never the settings
/// panel, never the utilities/tools/engagements rails, never the label overlay.
pub fn puzzle3d_selection_scope() -> UiDirtyScope {
    UiDirtyScope::Partial { window_bodies: vec![main::BODY_KEY.to_string()], panel_bodies: puzzle3d_selection_panel_bodies(), utilities: false, tools: false, engagements: false, measures: true, labels: false }
}

/// 🪜️ Switching a domain's selection MODE or GRANULARITY moves no selection and no document — it moves
/// the window's own Select chrome (the measures rail binds both controls) and what the world body
/// highlights at the new level of detail. No panel renders either field.
pub fn puzzle3d_interaction_chrome_scope() -> UiDirtyScope {
    UiDirtyScope::Partial { window_bodies: vec![main::BODY_KEY.to_string()], panel_bodies: Vec::new(), utilities: false, tools: false, engagements: false, measures: true, labels: false }
}

/// 🐢️ What class of shell state one declared command invalidates — the ONE place this app
/// decides a `UiDirtyScope`, next to the command catalogue whose ids it keys on.
///
/// 🧯️ Why this is a table and not a default: `dispatch_step` used to start every arm at
/// [`UiDirtyScope::Full`] and let four viewport verbs opt out, so the only alternatives an author had
/// were "repaint the entire shell" or "hand-write a Partial" — and every hand-written Partial in this
/// app carried `panel_bodies: Vec::new()`, which is correct for a fill tick and silently wrong for
/// anything that moves the selection or the document. The class says WHAT changed; the scope is
/// derived from it exactly once, so a command can never name a window body and forget its panels.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Puzzle3dScopeClass {
    /// 🈳️ Paints nothing (a host-only upload, an idle tick, a refused command).
    Quiet,
    /// 🎥️ Camera/projection only — the world body, no panels, no chrome.
    Viewport,
    /// ⚖️ Fill/distribution slider gestures: world body + fill-tool measures + window measures.
    FillOptions,
    /// 🪟️ One per-window option from the measures rail or the Settings panel (grid, LOD, vortex, sun,
    /// selectable kinds, projection, the four steppers). It publishes on the per-window `WindowConfig`
    /// lane ONLY: the world body re-derives from it, the rail's own controls are bound to it, and the
    /// Settings panel mirrors four of its fields — nothing else in the shell can move.
    WindowOption,
    /// 🕹️ Selection changed: world body + [`puzzle3d_selection_panel_bodies`] + window measures.
    Selection,
    /// 🎮️ One of the framework's six interaction verbs ([`InteractionVerb`]). These never reach
    /// `dispatch_step` — the framework's `dispatch_interaction_action` owns them — so this app declares
    /// them back through `ArtifactEditor::interaction_scope` instead, out of the SAME table.
    Interaction(InteractionVerb),
    /// 📝️ Artifact edited: world body + [`puzzle3d_artifact_panel_bodies`] + window measures.
    Artifact,
    /// 🏛️ Anything that can move the shell chrome itself — utilities, tools, engagements, labels, or
    /// a whole-document switch. The honest answer for a command whose blast radius is not narrower.
    Chrome,
}

/// 🐢️ The exact `UiDirtyScope` one [`Puzzle3dScopeClass`] stands for.
pub fn puzzle3d_scope(class: Puzzle3dScopeClass) -> UiDirtyScope {
    match class {
        Puzzle3dScopeClass::Quiet => UiDirtyScope::None,
        Puzzle3dScopeClass::Viewport => puzzle3d_viewport_scope(),
        Puzzle3dScopeClass::FillOptions => puzzle3d_fill_options_scope(),
        Puzzle3dScopeClass::WindowOption => puzzle3d_window_option_scope(),
        Puzzle3dScopeClass::Selection => puzzle3d_selection_scope(),
        Puzzle3dScopeClass::Interaction(InteractionVerb::Hover) => puzzle3d_viewport_scope(),
        Puzzle3dScopeClass::Interaction(InteractionVerb::Select | InteractionVerb::ClearSelection | InteractionVerb::SelectAll) => puzzle3d_selection_scope(),
        Puzzle3dScopeClass::Interaction(InteractionVerb::SetSelectionMode | InteractionVerb::SetGranularity) => puzzle3d_interaction_chrome_scope(),
        Puzzle3dScopeClass::Artifact => {
            UiDirtyScope::Partial { window_bodies: vec![main::BODY_KEY.to_string()], panel_bodies: puzzle3d_artifact_panel_bodies(), utilities: false, tools: false, engagements: false, measures: true, labels: false }
        }
        Puzzle3dScopeClass::Chrome => UiDirtyScope::Full,
    }
}

/// 🐢️ The scope table: one class per declared action id. An id absent from the table is
/// [`Puzzle3dScopeClass::Chrome`] — the widest, always-correct answer — so a newly declared command
/// is slow before it is wrong, and `command_scope_classes_name_the_panels_they_change` in
/// `🧪️tests/🔬️unit/🦀️.rs` is what forces every MUTATING id to earn a narrower class.
pub fn puzzle3d_command_scope_class(action: &str) -> Puzzle3dScopeClass {
    match action {
        "setCamera" | "focusSelection" => Puzzle3dScopeClass::Viewport,
        "setGridVisible" | "setGridSnapEnabled" | "setGridSpacing" | "setLodAutomatic" | "setLodDepthVariable" | "setLodManual" | "setVortexShow" | "setVortexDirection" | "toggleSun" | "setSunAzimuth" | "setSunElevation"
        | "setSunIntensity" | "setSelectableKind" | "setTransformGumballFlag" | "setProximityRadius" | "setChunkSize" | "setVoxelDims" | "setProjection" | "setProjectionParam" => Puzzle3dScopeClass::WindowOption,
        "setFillCount" | "setObjectKindWeight" | "setVortexKindWeight" => Puzzle3dScopeClass::FillOptions,
        "registerBrushMesh" | "exportSnapshot" | "openImportSnapshot" | "targetBrushSuggestions" => Puzzle3dScopeClass::Quiet,
        "setActiveUtility" | "setActiveTool" => Puzzle3dScopeClass::Viewport,
        "selectSameKindSelection" => Puzzle3dScopeClass::Selection,
        "duplicateSelection" | "deleteSelection" | "translateSelection" | "rotateSelection" | "scaleSelection" | "patchInspector" | "setSelectionFlag" | "setSelectionHidden" | "setSelectionLocked" | "setTargetVolumeFlag" | "deleteAttraction" | "deleteTargetVolume" | "createAttraction"
        | "worldRelocate" | "relocateTargetVolume" | "addObjectKind" | "addTargetVolume" | "importSnapshot" => Puzzle3dScopeClass::Artifact,
        "setActiveExample" => Puzzle3dScopeClass::Chrome,
        _ => Puzzle3dScopeClass::Chrome,
    }
}
