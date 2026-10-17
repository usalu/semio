//! 📜️ World3d interaction plans: camera settle, wheel, drag and paint plans, plan publication and the active census.

use super::*;

//#region 🧵️WorldInteractionTransaction
//#region 🧵️WorldInteractionTransaction
/** 🧭️ The plan a settled gesture owes: the classified `noteWorldNavigation` when a navigation
 * actually started AND moved the camera, then the one `setCamera` every pointer-up owes — in that
 * order, because React's `onNavigationGestures` is immediate and its `reportCamera` is debounced.
 * See [`WorldCameraSync`] for the reference's own two conditions; `None` when nothing is owed. */
/** 👻️ Whether this surface has a hover PUBLISHED to the guest — the only thing a pointer leave has
 * to answer. The renderer asks before it enqueues one, so a pointer travelling over chrome costs the
 * bounded intent queue nothing once the hover is clear (React's canvas receives no move at all while
 * the pointer is over a DOM layer above it). */
pub fn world3d_hover_is_published(selection_state: &WorldSelectionState) -> bool {
    selection_state.local_hover_id.is_some() || selection_state.hovered_vortex_id.is_some() || selection_state.hovered_component_id.is_some() || selection_state.hovered_component_object_id.is_some() || selection_state.hovered_component_mode.is_some()
}

/** 🚪️ Whether this surface OWES a hover clear that nothing has been enqueued for yet — the exact
 * predicate a per-FRAME caller must ask, as opposed to the per-EVENT
 * [`world3d_hover_is_published`] the renderer's move path asks.
 *
 * 🩸️ Without the queued term, a frame-driven clear re-enqueues one leave per frame: the published
 * hover only goes clear when the leave is ANSWERED, and while the leave sits behind the rest of the
 * queue the predicate stays true. Measured live at `context-menu` — `queued=64 blocked=true` on
 * `puzzle3d-main-perspective`, the bounded intent queue full of identical leaves, then
 * `bounded frame input action authority faulted` and a dead page
 * (ticket 26/09/17/WGPU-RENDERER-REACT-PARITY, `🗑️generated/w13c-parity-run-20`). */
pub fn world3d_hover_clear_is_owed(selection_state: &WorldSelectionState, authority_state: &WorldAuthorityState) -> bool {
    world3d_hover_is_published(selection_state) && authority_state.interaction_authority.as_ref().is_some_and(|authority| !authority.queue.holds_pointer_leave() && !authority.blocked.is_some_and(|intent| intent.phase == WorldInteractionPhase::PointerLeave))
}

/** 🚪️ The clear a [`WorldInteractionPhase::PointerLeave`] owes, in whichever hover lane this
 * surface currently publishes: `interactionHover` with empty targets in component mode, `worldVortexHover` with a
 * null `fullId` under the brush/vertex utilities, and `interactionHover` with empty `targets`
 * otherwise — the same three clears a hover pick that MISSES produces, which is what React gets from
 * r3f's `onPointerOut` (`🌍️world/🎨️r3f/🟦️.tsx`'s `dispatchInstanceHover`).
 *
 * ⚖️ `None` when nothing is published, so a leave over an already-clear surface is retired without a
 * journal entry — React's canvas does not re-clear a hover it never had. */
/// 🧲️ A leave clears whichever hover channel is live — the instance one OR the sub-object pick
/// target one; the `Hover` publish arm with no target clears both.
pub(super) fn plan_world3d_pointer_leave(session_state: &WorldSessionState, selection_state: &WorldSelectionState, generation: u64) -> Option<WorldInteractionPlan> {
    let mut plan = WorldInteractionPlan::new(session_state.interaction_revision, generation);
    let controller = plan.push_string(&session_state.controller_id)?;
    let surface = plan.push_string(&session_state.surface_id)?;
    if component_mode_active(selection_state) {
        let mode = selection_state.hovered_component_mode.as_deref()?;
        let mode = plan.push_string(mode)?;
        let domain = plan.push_string(resolved_domain_id(session_state))?;
        let action = WorldFlatAction { kind: WorldFlatActionKind::Hover, strings: [Some(controller), Some(surface), None, Some(domain), Some(mode), None, None, None], numbers: [0.0, 1.0, 1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0] };
        return plan.push_action(action).then_some(plan);
    }
    if session_state.active_utility == "brush" || (session_state.active_utility == "select" && selection_state.granularity == "vertex") {
        selection_state.hovered_vortex_id.as_ref()?;
        let action = WorldFlatAction { kind: WorldFlatActionKind::VortexHover, strings: [Some(controller), Some(surface), None, None, None, None, None, None], numbers: [0.0; 10] };
        return plan.push_action(action).then_some(plan);
    }
    if selection_state.local_hover_id.is_none() && selection_state.pick_hover_key.is_none() {
        return None;
    }
    let domain = plan.push_string(resolved_domain_id(session_state))?;
    let action = WorldFlatAction { kind: WorldFlatActionKind::Hover, strings: [Some(controller), None, None, Some(domain), None, None, None, None], numbers: [0.0; 10] };
    plan.push_action(action).then_some(plan)
}

pub fn plan_world3d_camera_settle(session_state: &WorldSessionState, camera_state: &WorldCameraState, generation: u64) -> Option<WorldInteractionPlan> {
    if !camera_state.camera_sync.owed {
        return None;
    }
    let camera = camera_state.orbit.to_camera();
    let mut plan = WorldInteractionPlan::new(session_state.interaction_revision, generation);
    let controller = plan.push_string(&session_state.controller_id)?;
    let surface = plan.push_string(&session_state.surface_id)?;
    let gestures = camera_state.camera_sync.start.map(|start| classify_world_navigation_gestures(start, world_navigation_snapshot(camera_state))).unwrap_or([false; 3]);
    if gestures.iter().any(|moved| *moved) {
        let mut numbers = [0.0; 10];
        for (index, moved) in gestures.iter().enumerate() {
            numbers[index] = if *moved { 1.0 } else { 0.0 };
        }
        let navigation = WorldFlatAction { kind: WorldFlatActionKind::Navigation, strings: [Some(controller), Some(surface), None, None, None, None, None, None], numbers };
        if !plan.push_action(navigation) {
            return None;
        }
    }
    let sync = WorldFlatAction {
        kind: WorldFlatActionKind::Camera,
        strings: [Some(controller), Some(surface), None, None, None, None, None, None],
        numbers: [camera.position.x as f64, camera.position.y as f64, camera.position.z as f64, camera.target.x as f64, camera.target.y as f64, camera.target.z as f64, world3d_camera_zoom(&camera), 0.0, 0.0, 0.0],
    };
    plan.push_action(sync).then_some(plan)
}

pub fn plan_world3d_wheel(session_state: &WorldSessionState, camera_state: &WorldCameraState, generation: u64, delta: f32) -> Option<WorldInteractionPlan> {
    let mut next = camera_state.orbit.clone();
    next.zoom(delta);
    let camera = next.to_camera();
    let mut plan = WorldInteractionPlan::new(session_state.interaction_revision, generation);
    let controller = plan.push_string(&session_state.controller_id)?;
    let surface = plan.push_string(&session_state.surface_id)?;
    let action = WorldFlatAction {
        kind: WorldFlatActionKind::CameraLocal,
        strings: [Some(controller), Some(surface), None, None, None, None, None, None],
        numbers: [camera.position.x as f64, camera.position.y as f64, camera.position.z as f64, camera.target.x as f64, camera.target.y as f64, camera.target.z as f64, world3d_camera_zoom(&camera), delta as f64, 0.0, 0.0],
    };
    plan.push_action(action).then_some(plan)
}

/** 🖱️🎯️ The selection a NON-primary press publishes — the twin of React's own "a press selects,
 * whatever the button".
 *
 * React reaches it through R3F, not through a handler of its own: `🌐️World3dHost/🟦️.tsx`'s
 * `handlePointerDown` returns immediately for `event.button !== 0`, but the `<Canvas>`'s
 * `onPointerMissed={handleEmptyClick}` is raised by R3F on a `pointerdown` that resolved no
 * event-handling object — for ANY button — and `handleEmptyClick` publishes `interactionSelect` over
 * the target the pane is hovering (or an empty selection when it hovers nothing). A primary press
 * arms the marquee instead, and its own drag flag (`wasMarqueeDragRef`) then swallows the first of
 * the two calls; a middle or right press arms nothing, so both land. That is exactly the measured
 * shape: `orbit-drag` journals ONE `interactionSelect` on React, `pan-drag` and `context-menu` TWO,
 * and the wgpu pane journalled NONE for the latter two
 * (`🗑️generated/w12c-parity-run-19/steps.json`, ticket 26/09/17 packet W13c §1).
 *
 * 🧭️ The pick is the SAME one a primary click runs — [`WorldRayPickPurpose::Instance`], whose miss
 * arm publishes the empty selection React's own miss publishes — so the two buttons answer with one
 * verb in one lane rather than with a second, hand-rolled selection route. `None` outside that lane
 * (component mode and the brush/vertex utilities own their own press verbs) and for a point this
 * surface no longer covers, which retires the press as before. */
pub(super) fn plan_world3d_non_primary_press_pick(session_state: &WorldSessionState, camera_state: &WorldCameraState, selection_state: &WorldSelectionState, generation: u64, intent: &WorldInteractionIntent) -> Option<WorldInteractionActive> {
    if session_state.active_utility != "select" || component_mode_active(selection_state) || selection_state.granularity == "vertex" {
        return None;
    }
    WorldRayPickCursor::new(session_state, camera_state, generation, WorldRayPickPurpose::Instance, intent.x, intent.y).map(|mut cursor| {
        cursor.merge = world_merge_code(intent.shift, intent.ctrl, intent.meta);
        WorldInteractionActive::Pick { cursor, retirement: None }
    })
}

pub fn plan_world3d_drag(session_state: &WorldSessionState, camera_state: &WorldCameraState, generation: u64, dx: f32, dy: f32, button: i16, modifiers: &PointerModifiers) -> Option<WorldInteractionPlan> {
    let operation = if button == 1 || (button == 2 && modifiers.shift) {
        1.0
    } else if button == 2 && (modifiers.alt || modifiers.meta) {
        2.0
    } else {
        return None;
    };
    let mut next = camera_state.orbit.clone();
    if operation == 1.0 {
        next.pan(-dx, -dy);
    } else {
        next.orbit(dx, dy);
    }
    let camera = next.to_camera();
    let mut plan = WorldInteractionPlan::new(session_state.interaction_revision, generation);
    let controller = plan.push_string(&session_state.controller_id)?;
    let surface = plan.push_string(&session_state.surface_id)?;
    let action = WorldFlatAction {
        kind: WorldFlatActionKind::CameraLocal,
        strings: [Some(controller), Some(surface), None, None, None, None, None, None],
        numbers: [camera.position.x as f64, camera.position.y as f64, camera.position.z as f64, camera.target.x as f64, camera.target.y as f64, camera.target.z as f64, world3d_camera_zoom(&camera), dx as f64, operation, dy as f64],
    };
    plan.push_action(action).then_some(plan)
}

/// 🖌️ The release of a paint gesture: ONE `paintAt` with `phase: "commit"` that commits the app's open paint transaction,
/// clearing `paint_stroke_active` only once the action is reserved. `None` outside an open left-button paint gesture.
pub fn plan_world3d_paint_release(session_state: &WorldSessionState, selection_state: &WorldSelectionState, generation: u64, button: i16) -> Option<WorldInteractionPlan> {
    if selection_state.interaction_mode != "paint" || button != 0 || !selection_state.paint_stroke_active {
        return None;
    }
    let mut plan = WorldInteractionPlan::new(session_state.interaction_revision, generation);
    let controller = plan.push_string(&session_state.controller_id)?;
    let surface = plan.push_string(&session_state.surface_id)?;
    let action = WorldFlatAction { kind: WorldFlatActionKind::PaintCommit, strings: [Some(controller), Some(surface), None, None, None, None, None, None], numbers: [0.0; 10] };
    plan.push_action(action).then_some(plan)
}

/// 🧯️ Whether a host cancel of this surface would abort anything — a live gumball gesture that streamed into the guest's
/// open tool transaction, or an open paint stroke — so a closing shell defers retirement only when a cancel is owed.
pub fn world3d_cancel_owed(selection_state: &WorldSelectionState, authority_state: &WorldAuthorityState) -> bool {
    let gumball = authority_state.interaction_authority.as_ref().and_then(|authority| authority.gumball.as_ref()).is_some_and(|gesture| gesture.live && gesture.streamed);
    gumball || (selection_state.interaction_mode == "paint" && selection_state.paint_stroke_active)
}

/// 🧯️ A host cancel of an open paint stroke: ONE `paintAt` with `phase: "abort"` and the `reason`, clearing
/// `paint_stroke_active` only once the action is reserved. `None` outside an open paint stroke.
pub fn plan_world3d_paint_abort(session_state: &WorldSessionState, selection_state: &WorldSelectionState, generation: u64, reason: WorldCancelReason) -> Option<WorldInteractionPlan> {
    if selection_state.interaction_mode != "paint" || !selection_state.paint_stroke_active {
        return None;
    }
    let mut plan = WorldInteractionPlan::new(session_state.interaction_revision, generation);
    let controller = plan.push_string(&session_state.controller_id)?;
    let surface = plan.push_string(&session_state.surface_id)?;
    let action = WorldFlatAction { kind: WorldFlatActionKind::PaintAbort, strings: [Some(controller), Some(surface), None, None, None, None, None, None], numbers: [f64::from(reason as i16), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0] };
    plan.push_action(action).then_some(plan)
}

pub const INTERACTION_TARGETS_EMPTY: &str = "[]";

pub(crate) fn world_interaction_targets(surface: Option<&str>, object: Option<&str>, granularity: Option<&str>, bare_id: bool) -> Result<WorldInteractionText, ui_wgpu::wgpu::BoundedActionFault> {
    use std::fmt::Write;
    let mut text = WorldInteractionText::default();
    text.write_char('[').map_err(|_| ui_wgpu::wgpu::BoundedActionFault::StringCredits)?;
    if let (Some(surface), Some(object), Some(granularity)) = (surface, object, granularity) {
        let id = if bare_id { ["", "", object] } else { [surface, WORLD_ITEM_PATH_DELIMITER, object] };
        world_write_interaction_target(&mut text, granularity, &id).map_err(|_| ui_wgpu::wgpu::BoundedActionFault::StringCredits)?;
    }
    text.write_char(']').map_err(|_| ui_wgpu::wgpu::BoundedActionFault::StringCredits)?;
    Ok(text)
}

/// 🪟️ `windowId`, not `surfaceId`. `setCamera` is a WINDOW-OWNED action: the app declares
/// it on its preview window kinds only, and the shell resolves
/// `ActionAddress::window_instance_id` from this very argument
/// (`🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs`'s `dispatch_action`). Without it the address fell back
/// to the focused window and the guest refused, measured verbatim on 6118 as
/// `handle_action promise failed: window kind procedural-main does not own action
/// setCamera` — the same defect class as the retained row's `addGeneration`
/// (`📓️wgpu-input-hit-runtime-2026-09-13.md` §10.3). A World3d surface IS keyed by its
/// window instance id (`ShellState::world3d_states`), so its own id is that address, and
/// React's `worldCameraSetCameraDispatchArgs` sends exactly `{windowId, camera}`.
///
/// 📷️ `zoom`/`up`, never `fov`: `buildWorldCameraDispatchArgs` sends exactly
/// `{position, target, zoom, up?}`, and the guest camera value has no `fov` member — a
/// `fov` key makes the whole `camera` object fail deserialization and silently drops the
/// dispatch, the same trap React's own docstring records for a bare `projection` string.
///
/// 🚚️ React sends `{objectId, position}` and nothing else (`dispatch("worldRelocate", args)`
/// with `world3dRelocateDispatchArgsV1`'s own return); `surfaceId` rides along the way every
/// other verb on this surface carries it.
///
/// 🧲️ `numbers[1]` marks a SUB-OBJECT hover (`WorldRayPickCursor::pick_target_plan`): the
/// published target is a pick target's `kind:id`, so the dedupe key it updates is
/// `pick_hover_key`, not the instance-level `local_hover_id`. A CLEAR (no object) clears
/// both, because a move off the model leaves neither channel hovered.
pub fn publish_world3d_plan_step(
    session_state: &mut WorldSessionState, camera_state: &mut WorldCameraState, selection_state: &mut WorldSelectionState, scene_state: &mut WorldSceneState,
    plan: &mut WorldInteractionPlan,
    generation: u64,
    input: &mut ui_wgpu::wgpu::InputState<ActionDescriptor>,
    context: &mut semio_framework_job::StepContext<'_>,
) -> Result<WorldInteractionStep, ui_wgpu::wgpu::BoundedActionFault> {
    if context.should_yield() {
        return Ok(WorldInteractionStep::Pending);
    }
    if plan.faulted {
        return Ok(WorldInteractionStep::Fault);
    }
    if plan.generation != generation || plan.revision != session_state.interaction_revision {
        return Ok(WorldInteractionStep::Stale);
    }
    if plan.cursor == plan.action_len {
        plan.byte_len = 0;
        return Ok(WorldInteractionStep::Complete);
    }
    let action = plan.actions[usize::from(plan.cursor)].expect("world interaction action cursor remains admitted");
    match action.kind {
        WorldFlatActionKind::Camera => {
            let controller = plan.string(action.strings[0].expect("camera controller span"));
            let surface = plan.string(action.strings[1].expect("camera surface span"));
            let action_id = "setCamera";
            let bytes = ui_wgpu::wgpu::checked_action_string_bytes(&[controller, action_id, "windowId", surface, "camera", "position", "target", "zoom", "up"])?;
            let mut reservation = input.reserve_action(controller, action_id, bytes)?;
            let builder = reservation.builder();
            builder.begin_object(None)?;
            builder.string(Some("windowId"), surface)?;
            builder.begin_object(Some("camera"))?;
            builder.begin_array(Some("position"))?;
            for value in &action.numbers[..3] {
                builder.number(None, *value)?;
            }
            builder.end_container()?;
            builder.begin_array(Some("target"))?;
            for value in &action.numbers[3..6] {
                builder.number(None, *value)?;
            }
            builder.end_container()?;
            builder.number(Some("zoom"), action.numbers[6])?;
            builder.begin_array(Some("up"))?;
            for value in [camera_state.orbit.up.x, camera_state.orbit.up.y, camera_state.orbit.up.z] {
                builder.number(None, f64::from(value))?;
            }
            builder.end_container()?;
            builder.end_container()?;
            builder.end_container()?;
            reservation.publish_with(|| {
                camera_state.camera_sync = WorldCameraSync { hover_owed: true, at: camera_state.camera_sync.at, ..WorldCameraSync::default() };
                advance_world3d_view_revision(session_state, scene_state);
            })?;
        }
        WorldFlatActionKind::CameraLocal => {
            if !camera_state.camera_sync.navigating {
                camera_state.camera_sync.start = Some(world_navigation_snapshot(camera_state));
                camera_state.camera_sync.navigating = true;
            }
            let first = action.numbers[7] as f32;
            let operation = action.numbers[8] as u8;
            let second = action.numbers[9] as f32;
            match operation {
                0 => camera_state.orbit.zoom(first),
                1 => camera_state.orbit.pan(-first, -second),
                2 => camera_state.orbit.orbit(first, second),
                _ => unreachable!("world camera plan operation is schema-bounded"),
            }
            camera_state.camera_user_moved = true;
            camera_state.camera_sync.owed = true;
            advance_world3d_view_revision(session_state, scene_state);
        }
        WorldFlatActionKind::Navigation => {
            let controller = plan.string(action.strings[0].expect("navigation controller span"));
            let surface = plan.string(action.strings[1].expect("navigation surface span"));
            let action_id = "noteWorldNavigation";
            let bytes = ui_wgpu::wgpu::checked_action_string_bytes(&[controller, action_id, "windowId", surface, "gestures", WORLD_NAVIGATION_GESTURE_IDS[0], WORLD_NAVIGATION_GESTURE_IDS[1], WORLD_NAVIGATION_GESTURE_IDS[2]])?;
            let mut reservation = input.reserve_action(controller, action_id, bytes)?;
            let builder = reservation.builder();
            builder.begin_object(None)?;
            builder.string(Some("windowId"), surface)?;
            builder.begin_array(Some("gestures"))?;
            for (index, name) in WORLD_NAVIGATION_GESTURE_IDS.iter().enumerate() {
                if action.numbers[index] != 0.0 {
                    builder.string(None, name)?;
                }
            }
            builder.end_container()?;
            builder.end_container()?;
            reservation.publish()?;
        }
        WorldFlatActionKind::PaintCommit => {
            let controller = plan.string(action.strings[0].expect("paint controller span"));
            let surface = plan.string(action.strings[1].expect("paint surface span"));
            let action_id = "paintAt";
            let bytes = ui_wgpu::wgpu::checked_action_string_bytes(&[controller, action_id, "surfaceId", surface, "phase", "commit"])?;
            let mut reservation = input.reserve_action(controller, action_id, bytes)?;
            let builder = reservation.builder();
            builder.begin_object(None)?;
            builder.string(Some("surfaceId"), surface)?;
            builder.string(Some("phase"), "commit")?;
            builder.end_container()?;
            reservation.publish_with(|| {
                selection_state.paint_stroke_active = false;
                session_state.interaction_revision = session_state.interaction_revision.wrapping_add(1);
            })?;
        }
        WorldFlatActionKind::PaintAbort => {
            let controller = plan.string(action.strings[0].expect("paint controller span"));
            let surface = plan.string(action.strings[1].expect("paint surface span"));
            let reason = WorldCancelReason::from_button(action.numbers[0] as i16).code();
            let action_id = "paintAt";
            let bytes = ui_wgpu::wgpu::checked_action_string_bytes(&[controller, action_id, "surfaceId", surface, "phase", "abort", "reason", reason])?;
            let mut reservation = input.reserve_action(controller, action_id, bytes)?;
            let builder = reservation.builder();
            builder.begin_object(None)?;
            builder.string(Some("surfaceId"), surface)?;
            builder.string(Some("phase"), "abort")?;
            builder.string(Some("reason"), reason)?;
            builder.end_container()?;
            reservation.publish_with(|| {
                selection_state.paint_stroke_active = false;
                session_state.interaction_revision = session_state.interaction_revision.wrapping_add(1);
            })?;
        }
        WorldFlatActionKind::PaintAt => {
            let controller = plan.string(action.strings[0].expect("paint controller span"));
            let surface = plan.string(action.strings[1].expect("paint surface span"));
            let object = plan.string(action.strings[2].expect("paint object span"));
            let action_id = "paintAt";
            let bytes = ui_wgpu::wgpu::checked_action_string_bytes(&[controller, action_id, "surfaceId", surface, "objectId", object, "u", "v", "phase", "stream"])?;
            let mut reservation = input.reserve_action(controller, action_id, bytes)?;
            let builder = reservation.builder();
            builder.begin_object(None)?;
            builder.string(Some("surfaceId"), surface)?;
            builder.string(Some("objectId"), object)?;
            builder.number(Some("u"), action.numbers[0])?;
            builder.number(Some("v"), action.numbers[1])?;
            builder.string(Some("phase"), "stream")?;
            builder.end_container()?;
            reservation.publish()?;
        }
        WorldFlatActionKind::SurfacePlace => {
            let controller = plan.string(action.strings[0].expect("surface controller span"));
            let surface = plan.string(action.strings[1].expect("surface span"));
            let object = plan.string(action.strings[2].expect("surface object span"));
            let action_id = "worldSurfacePlace";
            let bytes = ui_wgpu::wgpu::checked_action_string_bytes(&[controller, action_id, "surfaceId", surface, "pane", surface, "objectId", object, "position", "normal"])?;
            let mut reservation = input.reserve_action(controller, action_id, bytes)?;
            let builder = reservation.builder();
            builder.begin_object(None)?;
            builder.string(Some("surfaceId"), surface)?;
            builder.string(Some("pane"), surface)?;
            builder.string(Some("objectId"), object)?;
            builder.begin_array(Some("position"))?;
            for value in &action.numbers[..3] {
                builder.number(None, *value)?;
            }
            builder.end_container()?;
            builder.begin_array(Some("normal"))?;
            for value in &action.numbers[3..6] {
                builder.number(None, *value)?;
            }
            builder.end_container()?;
            builder.end_container()?;
            reservation.publish()?;
        }
        WorldFlatActionKind::Relocate => {
            let controller = plan.string(action.strings[0].expect("relocate controller span"));
            let surface = plan.string(action.strings[1].expect("relocate surface span"));
            let object = plan.string(action.strings[2].expect("relocate object span"));
            let action_id = "worldRelocate";
            let bytes = ui_wgpu::wgpu::checked_action_string_bytes(&[controller, action_id, "surfaceId", surface, "objectId", object, "position"])?;
            let mut reservation = input.reserve_action(controller, action_id, bytes)?;
            let builder = reservation.builder();
            builder.begin_object(None)?;
            builder.string(Some("surfaceId"), surface)?;
            builder.string(Some("objectId"), object)?;
            builder.begin_array(Some("position"))?;
            for value in &action.numbers[..3] {
                builder.number(None, *value)?;
            }
            builder.end_container()?;
            builder.end_container()?;
            reservation.publish()?;
        }
        WorldFlatActionKind::Select => {
            let controller = plan.string(action.strings[0].expect("selection controller span"));
            let surface = action.strings[1].map(|span| plan.string(span));
            let object = action.strings[2].map(|span| plan.string(span));
            let domain = plan.string(action.strings[3].expect("selection domain span"));
            let granularity = action.strings[4].map(|span| plan.string(span));
            let merge = plan.string(action.strings[5].expect("selection merge span"));
            let method = plan.string(action.strings[6].expect("selection method span"));
            let action_id = "interactionSelect";
            let targets = world_interaction_targets(surface, object, granularity, action.numbers[0] != 0.0)?;
            let bytes = ui_wgpu::wgpu::checked_action_string_bytes(&[controller, action_id, "domainId", domain, "targets", targets.as_str(), "merge", merge, "method", method])?;
            let mut reservation = input.reserve_action(controller, action_id, bytes)?;
            let builder = reservation.builder();
            builder.begin_object(None)?;
            builder.string(Some("domainId"), domain)?;
            builder.string(Some("targets"), targets.as_str())?;
            builder.string(Some("merge"), merge)?;
            builder.string(Some("method"), method)?;
            builder.end_container()?;
            reservation.publish()?;
        }
        WorldFlatActionKind::Hover => {
            let controller = plan.string(action.strings[0].expect("hover controller span"));
            let domain = plan.string(action.strings[3].expect("hover domain span"));
            let object = action.strings[2].map(|span| plan.string(span));
            let surface = action.strings[1].map(|span| plan.string(span));
            let granularity = action.strings[4].map(|span| plan.string(span));
            let action_id = "interactionHover";
            let targets = world_interaction_targets(surface, object, granularity, action.numbers[0] != 0.0)?;
            let bytes = ui_wgpu::wgpu::checked_action_string_bytes(&[controller, action_id, "domainId", domain, "channel", "pointer", "targets", targets.as_str()])?;
            let mut reservation = input.reserve_action(controller, action_id, bytes)?;
            let builder = reservation.builder();
            builder.begin_object(None)?;
            builder.string(Some("domainId"), domain)?;
            builder.string(Some("channel"), "pointer")?;
            builder.string(Some("targets"), targets.as_str())?;
            builder.end_container()?;
            let sub_object = action.numbers[1] != 0.0;
            let pick_key = match (sub_object, granularity, object) {
                (true, Some(granularity), Some(object)) => Some(format!("{granularity}:{object}")),
                _ => None,
            };
            let component = action.numbers[2] != 0.0;
            let component_object = action.strings[7].map(|span| plan.string(span));
            reservation.publish_with(|| {
                if component {
                    selection_state.hovered_component_id = component_object.map(|_| (action.numbers[3] as u32).to_string());
                    selection_state.hovered_component_object_id = component_object.map(str::to_owned);
                    selection_state.hovered_component_mode = component_object.and(granularity).map(str::to_owned);
                    selection_state.pick_hover_key = pick_key;
                    if component_object.is_none() { selection_state.local_hover_id = None; selection_state.local_hover_granularity_id = None; }
                } else { match (sub_object, object) {
                    (true, Some(_)) => selection_state.pick_hover_key = pick_key,
                    (true, None) => {
                        selection_state.pick_hover_key = None;
                        selection_state.local_hover_id = None;
                        selection_state.local_hover_granularity_id = None;
                    }
                    (false, _) => {
                        selection_state.local_hover_id = object.map(str::to_owned);
                        selection_state.local_hover_granularity_id = granularity.map(str::to_owned);
                        if object.is_none() {
                            selection_state.pick_hover_key = None;
                        }
                    }
                } }
                session_state.interaction_revision = session_state.interaction_revision.wrapping_add(1);
            })?;
        }
        WorldFlatActionKind::VortexHover => {
            let controller = plan.string(action.strings[0].expect("vortex hover controller span"));
            let surface = plan.string(action.strings[1].expect("vortex hover surface span"));
            let hit = action.strings[2].map(|span| plan.string(span));
            let action_id = "worldVortexHover";
            let bytes = match hit {
                Some(hit) => ui_wgpu::wgpu::checked_action_string_bytes(&[controller, action_id, "surfaceId", surface, "fullId", hit])?,
                None => ui_wgpu::wgpu::checked_action_string_bytes(&[controller, action_id, "surfaceId", surface, "fullId"])?,
            };
            let mut reservation = input.reserve_action(controller, action_id, bytes)?;
            let builder = reservation.builder();
            builder.begin_object(None)?;
            builder.string(Some("surfaceId"), surface)?;
            match hit {
                Some(hit) => builder.string(Some("fullId"), hit)?,
                None => builder.null(Some("fullId"))?,
            }
            builder.end_container()?;
            reservation.publish_with(|| {
                selection_state.hovered_vortex_id = hit.map(str::to_owned);
                session_state.interaction_revision = session_state.interaction_revision.wrapping_add(1);
            })?;
        }
        WorldFlatActionKind::VortexSelect => {
            let controller = plan.string(action.strings[0].expect("vortex selection controller span"));
            let surface = plan.string(action.strings[1].expect("vortex selection surface span"));
            let hit = plan.string(action.strings[2].expect("vortex selection id span"));
            let merge = plan.string(action.strings[3].expect("vortex selection merge span"));
            let action_id = "worldVortexSelect";
            let bytes = ui_wgpu::wgpu::checked_action_string_bytes(&[controller, action_id, "surfaceId", surface, "fullId", hit, "merge", merge])?;
            let mut reservation = input.reserve_action(controller, action_id, bytes)?;
            let builder = reservation.builder();
            builder.begin_object(None)?;
            builder.string(Some("surfaceId"), surface)?;
            builder.string(Some("fullId"), hit)?;
            builder.string(Some("merge"), merge)?;
            builder.end_container()?;
            reservation.publish()?;
        }

    }
    plan.actions[usize::from(plan.cursor)] = None;
    plan.cursor += 1;
    plan.revision = session_state.interaction_revision;
    context.consume_fuel(1);
    Ok(WorldInteractionStep::Pending)
}
//#endregion 🧵️WorldInteractionTransaction

/// 🔎️ The active transition named together with the one fact that decides whether it will produce a
/// guest action — a pick's purpose and whether it found anything, a plan's pending action kind.
/// Used by {@link World3dState::interaction_census}.
pub fn world_interaction_active_census(active: &WorldInteractionActive) -> String {
    match active {
        WorldInteractionActive::Plan { plan, .. } => format!("Plan[{}/{} {:?}]", plan.cursor, plan.action_len, plan.actions.get(usize::from(plan.cursor)).copied().flatten().map(|action| action.kind)),
        WorldInteractionActive::Pick { cursor, .. } => format!("Pick[{:?} hit={} done={}]", cursor.purpose, cursor.best.is_some(), cursor.complete),
        WorldInteractionActive::ObjectPick { cursor, .. } => format!("ObjectPick[{:?} hit={} done={}]", cursor.purpose, cursor.best.is_some(), cursor.complete),
        WorldInteractionActive::ComponentPick { cursor, .. } => format!("ComponentPick[{:?} hit={} done={}]", cursor.purpose, cursor.best.is_some(), cursor.complete),
        WorldInteractionActive::MarqueePick { .. } => "MarqueePick".to_string(),
        WorldInteractionActive::MarqueePublish { job, .. } => format!("MarqueePublish[page={} stage={} targets={}b]", job.page, job.stage, job.targets_len),
        WorldInteractionActive::ComponentMarqueePublish { .. } => "ComponentMarqueePublish".to_string(),
        WorldInteractionActive::GumballPick { .. } => "GumballPick".to_string(),
        WorldInteractionActive::GumballCommit { .. } => "GumballCommit".to_string(),
        WorldInteractionActive::BrushCommit { .. } => "BrushCommit".to_string(),
    }
}

pub struct WorldInteractionAuthority {
    pub queue: WorldInteractionIntentQueue,
    pub blocked: Option<WorldInteractionIntent>,
    pub registry: Option<WorldInteractionRegistryBuildCursor>,
    pub active: Option<WorldInteractionActive>,
    pub right_press: Option<[f32; 2]>,
    pub(crate) right_dragged: bool,
    pub marquee: Option<WorldMarqueeGesture>,
    pub gumball: Option<WorldGumballGesture>,
    /// 🧯️ A cancelled live gumball drag: its remaining moves and its release are swallowed.
    pub(crate) gumball_swallow: bool,
    pub next_generation: u64,
    pub faulted: bool,
    pub closing: bool,
}

impl Default for WorldInteractionAuthority {
    fn default() -> Self {
        Self { queue: WorldInteractionIntentQueue::default(), blocked: None, registry: None, active: None, right_press: None, right_dragged: false, marquee: None, gumball: None, gumball_swallow: false, next_generation: 1, faulted: false, closing: false }
    }
}
//#endregion 🧵️WorldInteractionTransaction
