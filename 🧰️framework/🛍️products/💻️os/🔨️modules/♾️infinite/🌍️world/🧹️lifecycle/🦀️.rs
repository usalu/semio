//! 🧹️ World camera sync, dynamic mesh retirement and draw rebuild lifecycles.

use super::*;

//#region 🧭️WorldCameraSync
/** 🧭️ A world pane's camera-report debt, and the parity contract behind it.
 *
 * ⚖️ React's `WorldOrbitGated` (`♾️infinite/🌍️world/🎨️r3f/🟦️.tsx`) reports through three's
 * `OrbitControls` events, and the two of them do NOT fire on the same condition:
 *
 * - `end` is dispatched on EVERY pointer-up over the canvas, whatever the button and whether or not
 *   any camera state moved (`three/examples/jsm/controls/OrbitControls.js`'s `onPointerUp`, which
 *   calls `dispatchEvent(_endEvent)` unconditionally once the last pointer lifts). `onEnd` always
 *   calls `reportCamera`, so a plain left click and a plain right click each owe ONE `setCamera` —
 *   which is exactly what the React reference journals for the `pick-instance` and `context-menu`
 *   steps, neither of which moves the camera at all
 *   (`🗑️generated/parity-run-2/steps.json`).
 * - `start` is dispatched only when a mouse button actually claimed a navigation state, so
 *   `navigationSnapshotRef` is set only for a real gesture, and `onNavigationGestures` — the
 *   `noteWorldNavigation` dispatch — additionally requires the before/after diff to classify
 *   non-empty. The same run journals `noteWorldNavigation` for `pan-drag` and `zoom-wheel` and NOT
 *   for `orbit-drag`, because the left button is bound to nothing
 *   (`resolveWorldOrbitMouseButtonsIdle`: `LEFT: null`, `MIDDLE: PAN`, right → pan on Shift and
 *   orbit on Alt) — so the left drag marquees and the camera never moves.
 *
 * ⏱️ `reportCamera`'s dispatch is TRAILING-DEBOUNCED (`CAMERA_SYNC_DEBOUNCE_MS`), so a whole drag or
 * a whole wheel burst publishes ONE `setCamera`. This lane owes no timer to reproduce that: an
 * interaction intent queue that has drained IS the settle point, so the debt is paid once the last
 * intent of the gesture is answered, and a burst coalesces by construction. `noteWorldNavigation` is
 * NOT debounced in React, so it is published first — the order the reference journals
 * (`noteWorldNavigation` then `setCamera`).
 *
 * 🩸️ Before this, a nav drag published one `setCamera` per pointer MOVE — eight per gesture on the
 * parity probe's eight-sample drag — and a gesture that moved nothing published none at all; there
 * was no `noteWorldNavigation` anywhere in the target
 * (ticket 26/09/17/WGPU-RENDERER-REACT-PARITY, `📓️w10a-world-interaction-journal-parity.md`). */
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub(super) struct WorldCameraSync {
    pub(super) owed: bool,
    pub(super) navigating: bool,
    pub(super) start: Option<WorldNavigationSnapshot>,
    /// 🎯️ A settled camera owes ONE re-published hover: the scene moved under a pointer that did not,
    /// so what it is over changed. React gets this for free — r3f re-raycasts the moved scene and
    /// `dispatchInstanceHover` publishes again, which is the trailing `interactionHover` every one of
    /// the reference's five world steps ends on.
    pub(super) hover_owed: bool,
    /// 🖱️ The last point a pointer intent named on this surface, for that re-published hover.
    pub(super) at: Option<[f32; 2]>,
}

/// 📸️ The subset of camera state [`classify_world_navigation_gestures`] diffs before/after a
/// gesture — React's `WorldNavigationSnapshot`, whose `zoom` is the live frustum scale for a
/// parallel pane and a bare `1` for a perspective one (`captureNavigationSnapshot`).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct WorldNavigationSnapshot {
    pub position: [f32; 3],
    pub target: [f32; 3],
    pub zoom: f32,
    pub parallel: bool,
}

/// 📏️ React's `WORLD_NAVIGATION_THRESHOLDS_DEFAULT`, verbatim.
pub const WORLD_NAVIGATION_PAN_RATIO: f32 = 0.02;
pub const WORLD_NAVIGATION_ZOOM_RATIO: f32 = 0.03;
pub const WORLD_NAVIGATION_ORBIT_RADIANS: f32 = 0.05;

/// 🧭️ The three gestures a camera movement may have performed, in React's own push order — `pan`,
/// `zoom`, `orbit` — so the `gestures` array this lane publishes is byte-identical to the DOM host's.
pub const WORLD_NAVIGATION_GESTURE_IDS: [&str; 3] = ["pan", "zoom", "orbit"];

fn world_navigation_distance(left: [f32; 3], right: [f32; 3]) -> f32 {
    ((left[0] - right[0]).powi(2) + (left[1] - right[1]).powi(2) + (left[2] - right[2]).powi(2)).sqrt()
}

fn world_navigation_direction(from: [f32; 3], to: [f32; 3]) -> [f32; 3] {
    let delta = [to[0] - from[0], to[1] - from[1], to[2] - from[2]];
    let length = (delta[0].powi(2) + delta[1].powi(2) + delta[2].powi(2)).sqrt();
    if length > 1e-9 {
        [delta[0] / length, delta[1] / length, delta[2] / length]
    } else {
        [0.0, 0.0, 0.0]
    }
}

/// 📸️ This pane's live camera as the snapshot a navigation diff measures.
pub fn world_navigation_snapshot(state: &World3dState) -> WorldNavigationSnapshot {
    let camera = state.orbit.to_camera();
    WorldNavigationSnapshot {
        position: [camera.position.x, camera.position.y, camera.position.z],
        target: [camera.target.x, camera.target.y, camera.target.z],
        zoom: if camera.projection.is_parallel() { camera.zoom } else { 1.0 },
        parallel: camera.projection.is_parallel(),
    }
}

/// 🧭️ Which navigation gestures a camera movement performed, as a `[pan, zoom, orbit]` mask — the
/// port of React's `classifyWorldNavigationGestures`: pan when the orbit TARGET moved, zoom when the
/// parallel zoom factor or the perspective dolly distance changed, orbit when the eye's direction
/// around the target rotated. One drag may answer more than one, so this is a set, not a verdict.
pub fn classify_world_navigation_gestures(before: WorldNavigationSnapshot, after: WorldNavigationSnapshot) -> [bool; 3] {
    let reference = world_navigation_distance(before.position, before.target).max(1.0);
    let pan = world_navigation_distance(before.target, after.target) > WORLD_NAVIGATION_PAN_RATIO * reference;
    let zoom_delta = if before.parallel { (after.zoom / if before.zoom == 0.0 { 1.0 } else { before.zoom } - 1.0).abs() } else { (world_navigation_distance(after.position, after.target) / reference - 1.0).abs() };
    let zoom = zoom_delta > WORLD_NAVIGATION_ZOOM_RATIO;
    let before_direction = world_navigation_direction(before.target, before.position);
    let after_direction = world_navigation_direction(after.target, after.position);
    let dot = (before_direction[0] * after_direction[0] + before_direction[1] * after_direction[1] + before_direction[2] * after_direction[2]).clamp(-1.0, 1.0);
    [pan, zoom, dot.acos() > WORLD_NAVIGATION_ORBIT_RADIANS]
}
//#endregion 🧭️WorldCameraSync

//#region 🧹️World3dDynamicRetirement

pub(super) struct World3dDynamicRetirement {
    pub(super) phase: u8,
    pub(super) blocked: Option<WorldDynamicFault>,
}

impl World3dDynamicRetirement {
    pub(super) fn begin(state: &mut World3dState) -> Self {
        state.meshes.begin_close();
        state.mesh_versions.begin_close();
        state.mesh_appearances.begin_close();
        state.draws.begin_close();
        state.reference_pixels.begin_close();
        state.mesh_paint_textures.begin_close();
        state.asset_io.begin_close();
        state.fit_bounds_cursor = None;
        Self { phase: 0, blocked: None }
    }

    pub(super) fn step(&mut self, state: &mut World3dState, context:&mut semio_framework_job::StepContext<'_>) -> bool {
        self.blocked = None;
        if state.native_mesh_admission.is_some(){if let Ok((_,progress))=world3d_close_native_mesh_admission_step(state,context.retained_grant()){if context.consume_retained(progress).is_ok()&&progress.copied_items!=0{context.consume_fuel(1);}}return false}
        if step_world3d_scene_bridge_close(state,context) {
            return false;
        }
        if state.dynamic_mesh_close.is_none() {
            if let Some(entry) = state.dynamic_blocked_mesh.take() {
                state.dynamic_mesh_close = Some(entry);
                return false;
            }
        }
        if let Some(owner) = state.dynamic_blocked_owner.take() {
            if let Err(owner) = quarantine_world_owner(owner) {
                state.dynamic_blocked_owner = Some(owner);
                self.blocked = Some(WorldDynamicFault::QuarantineCapacity);
            }
            return false;
        }
        match self.phase {
            0 => {
                if state.dynamic_mesh_close.is_some() {
                    step_world3d_mesh_close(state,context);
                    return false;
                }
                let Some(entry) = state.meshes.take_one() else {
                    self.phase = 1;
                    return false;
                };
                state.dynamic_mesh_close = Some(entry);
            }
            1 => {
                if state.mesh_versions.take_one().is_some() {
                    return false;
                }
                self.phase = 2;
                return false;
            }
            2 => {
                if state.mesh_appearances.take_one().is_some() {
                    return false;
                }
                self.phase = 3;
                return false;
            }
            3 => {
                let Some(entry) = state.draws.take_last() else {
                    self.phase = 4;
                    return false;
                };
                if let Err(owner) = quarantine_world_owner(WorldOpaqueOwner::Draw(entry)) {
                    let WorldOpaqueOwner::Draw(entry) = owner else { unreachable!("draw quarantine owner") };
                    state.draws.restore_last(entry);
                    self.blocked = Some(WorldDynamicFault::QuarantineCapacity);
                }
            }
            4 => {
                let Some(entry) = state.reference_pixels.take_one() else {
                    self.phase = 5;
                    return false;
                };
                drop(entry);
            }
            5 => {
                let Some(entry) = state.mesh_paint_textures.take_one() else {
                    self.phase = 6;
                    return false;
                };
                drop(entry);
            }
            6 => {
                let Some(id) = state.instance_interaction_ids.keys().next().cloned() else {
                    self.phase = 7;
                    return false;
                };
                state.instance_interaction_ids.remove(&id);
            }
            7 => {
                let Some(id) = state.instance_interaction_granularity_ids.keys().next().cloned() else {
                    self.phase = 8;
                    return false;
                };
                state.instance_interaction_granularity_ids.remove(&id);
            }
            8 => {
                if state.local_hover_granularity_id.take().is_some() {
                    return false;
                }
                self.phase = 9;
                return false;
            }
            _ => return true,
        }
        false
    }

    pub(super) fn terminal_is_empty(&self) -> bool {
        self.phase >= 9 && self.blocked.is_none()
    }
}

pub fn begin_world3d_dynamic_retirement(state: &mut World3dState) -> bool {
    if state.dynamic_retirement.is_some() {
        return false;
    }
    state.snapshot_fault = Some(World3dSnapshotFault::Closing);
    state.dynamic_retirement = Some(World3dDynamicRetirement::begin(state));
    true
}

pub fn step_world3d_dynamic_retirement(state: &mut World3dState, context: &mut semio_framework_job::StepContext<'_>) -> bool {
    if context.retirement_should_yield() {
        return false;
    }
    if let Some(cursor) = state.placeholder_build.as_mut() {
        if !cursor.close_step() || !cursor.terminal_is_empty() {
            context.consume_fuel(1);
            return false;
        }
        state.placeholder_build = None;
        context.consume_fuel(1);
        return false;
    }
    if state.terrain_pending.is_some() || state.terrain_build.is_some() {return false;}
    if let Some(cursor) = state.face_overlay_build.as_mut() {
        if !cursor.close_step() || !cursor.terminal_is_empty() {
            context.consume_fuel(1);
            return false;
        }
        state.face_overlay_build = None;
        context.consume_fuel(1);
        return false;
    }
    if let Some(index) = state.face_overlay_colors.iter().position(Option::is_some) {
        state.face_overlay_colors[index] = None;
        context.consume_fuel(1);
        return false;
    }
    if state.face_overlay_generation.take().is_some() {
        context.consume_fuel(1);
        return false;
    }
    if state.face_overlay_retired_generation.take().is_some() {
        context.consume_fuel(1);
        return false;
    }
    if !state.asset_io.terminal_is_empty() {
        let _ = state.asset_io.close_step();
        context.consume_fuel(1);
        return false;
    }
    if !close_world3d_draw_rebuild_step(state, context) {
        return false;
    }
    let Some(mut retirement) = state.dynamic_retirement.take() else {
        return true;
    };
    let complete = retirement.step(state,context);
    context.consume_fuel(1);
    if complete && retirement.terminal_is_empty() {
        return true;
    }
    state.dynamic_retirement = Some(retirement);
    false
}

pub fn world3d_dynamic_retirement_terminal_is_empty(state: &World3dState) -> bool {
    state.dynamic_retirement.is_none()
        && state.scene_bridge.is_none()
        && state.scene_bridge_close.is_none()
        && state.scene_bridge_lease.is_none()
        && state.scene_bridge_retired.is_none()
        && state.dynamic_blocked_owner.is_none()
        && state.dynamic_mesh_close.is_none()
        && state.dynamic_blocked_mesh.is_none()
        && state.native_mesh_admission.is_none()
        && state.placeholder_build.is_none()
        && state.terrain_build.is_none() && state.terrain_pending.is_none()
        && state.face_overlay_build.is_none()
        && state.face_overlay_generation.is_none()
        && state.face_overlay_retired_generation.is_none()
        && state.face_overlay_colors.iter().all(Option::is_none)
        && state.fit_bounds_cursor.is_none()
        && world3d_draw_rebuild_terminal_is_empty(state)
        && state.meshes.is_empty()
        && state.mesh_versions.is_empty()
        && state.draws.is_empty()
        && state.reference_pixels.is_empty()
        && state.mesh_paint_textures.is_empty()
        && state.instance_interaction_ids.is_empty()
        && state.instance_interaction_granularity_ids.is_empty()
        && state.local_hover_granularity_id.is_none()
        && state.asset_io.terminal_is_empty()
}

/// ⏰️ Whether this surface still owes its own frame — the one predicate the renderer turns into a
/// cursor wake, and therefore the only reason a settled, event-driven shell schedules another frame
/// for a World3d surface at all.
///
/// ⚖️ It used to name only the three MESH builders. The two cursors that turn a producer's
/// `meshes_json`/`instances_json` into `state.draws` — the scene bridge and the typed snapshot apply
/// — were missing, so a shell that had quiesced never scheduled the frames they needed: measured on
/// 6118 as `procedural-preview` parked at `bridge=false bridge-lease=true apply=true state-meshes=2
/// draws=0` for 95 s with the solid `extrude@solid` sitting unconsumed in its mesh lane
/// (ticket 26/09/09/PROCEDURAL-3D-END-TO-END). Every bounded cursor the frame transaction's
/// `World3dSnapshot` phase drives belongs here, or its work only advances while something ELSE keeps
/// the frames coming.
pub fn world3d_cursor_work_pending(state: &World3dState) -> bool {
    state.native_mesh_admission.is_some()
        || state.placeholder_build.is_some()
        || state.terrain_build.is_some() || state.terrain_pending.is_some()
        || state.face_overlay_build.is_some()
        || state.scene_bridge.is_some()
        || state.snapshot_apply.is_some()
        || state.draw_rebuild.is_some()
        || state.retired_draws.is_some()
        || state.dynamic_retirement.is_some()
        || state.scene_bridge_retired.is_some()
        || state.scene_bridge_lease.is_some_and(|lease| state.snapshot_lease != Some(lease))
        || world3d_camera_fit_work_pending(state)
}

//#endregion 🧹️World3dDynamicRetirement

//#region 🧱️WorldDrawRebuild
pub fn begin_world3d_draw_rebuild(state: &mut World3dState, descriptor: WorldDrawRebuildDescriptor) -> Result<(), WorldDynamicFault> {
    if state.dynamic_retirement.is_some() || state.draw_rebuild.is_some() || state.retired_draws.is_some() || state.dynamic_blocked_owner.is_some() {
        return Err(WorldDynamicFault::Closing);
    }
    if descriptor.revision != state.interaction_revision || descriptor.generation != state.draw_generation.wrapping_add(1) {
        return Err(WorldDynamicFault::StaleToken);
    }
    state.draw_rebuild = Some(WorldDrawRebuildCursor::new(descriptor)?);
    Ok(())
}

/// 🔢️ The version `publish_world3d_mesh_lease` admitted this mesh key under. `mesh_versions` and
/// `interaction_meshes` are written in ONE transaction there, so this is the freshness witness every
/// interaction-registry lookup must match — never `SceneDraw3d::mesh_version`, which the typed
/// snapshot apply admits before the mesh's own publication ladder has run and which therefore stays
/// at its `0` placeholder for every bridged draw. Matching the stale field faulted the whole
/// interaction authority — every pick, hover, orbit and wheel on that surface.
pub(super) fn live_world3d_mesh_version(state: &World3dState, mesh_key: &str) -> u64 {
    match state.meshes.0.get(mesh_key){Some(WorldMeshSource::Native(source)) if !source.frozen=>source.loan.identity().cache_epoch,_=>state.mesh_versions.get(mesh_key).copied().unwrap_or(0)}
}

pub fn world3d_draw_rebuild_admit_draw(state: &mut World3dState, mesh_key: &str, mesh_version: u64, instance_count: u16) -> Result<(), WorldDynamicFault> {
    let Some(cursor) = state.draw_rebuild.as_mut() else {
        return Err(WorldDynamicFault::StaleToken);
    };
    cursor.admit_draw(mesh_key, mesh_version, instance_count)
}

pub fn world3d_draw_rebuild_admit_instance(state: &mut World3dState, draw: u16, id: &str, model: Mat4, color: [f32; 4], selected: bool, hovered: bool) -> Result<(), WorldDynamicFault> {
    world3d_draw_rebuild_admit_retained_instance(state, draw, id, model, color, SceneColorSource3d::Authored, selected, hovered, None)
}

pub(super) fn world3d_draw_rebuild_admit_retained_instance(
    state: &mut World3dState,
    draw: u16,
    id: &str,
    model: Mat4,
    color: [f32; 4],
    color_source: SceneColorSource3d,
    selected: bool,
    hovered: bool,
    component_source: Option<ComponentSource3d>,
) -> Result<(), WorldDynamicFault> {
    let Some(cursor) = state.draw_rebuild.as_mut() else {
        return Err(WorldDynamicFault::StaleToken);
    };
    cursor.admit_instance(draw, id, model, color, color_source, selected, hovered, component_source)
}

pub fn world3d_draw_rebuild_seal(state: &mut World3dState) -> Result<(), WorldDynamicFault> {
    state.draw_rebuild.as_mut().ok_or(WorldDynamicFault::StaleToken)?.seal()
}

pub fn step_world3d_draw_rebuild(state: &mut World3dState, context: &mut semio_framework_job::StepContext<'_>) -> WorldDrawRebuildStep {
    if context.should_yield() {
        return WorldDrawRebuildStep::Pending;
    }
    if let Some(retired) = state.retired_draws.as_mut() {
        let Some(entry) = retired.take_last() else {
            state.retired_draws = None;
            context.consume_fuel(1);
            return WorldDrawRebuildStep::Pending;
        };
        if let Err(owner) = quarantine_world_owner(WorldOpaqueOwner::Draw(entry)) {
            let WorldOpaqueOwner::Draw(entry) = owner else { unreachable!("retired draw owner") };
            state.retired_draws.as_mut().expect("retired draw registry remains owned").restore_last(entry);
            mark_world_dynamic_fault(state, WorldDynamicFault::QuarantineCapacity);
            return WorldDrawRebuildStep::Fault;
        }
        context.consume_fuel(1);
        return WorldDrawRebuildStep::Pending;
    }
    let Some(mut cursor) = state.draw_rebuild.take() else {
        return WorldDrawRebuildStep::Complete;
    };
    if cursor.descriptor.revision != state.interaction_revision || cursor.descriptor.generation != state.draw_generation.wrapping_add(1) {
        state.draw_rebuild = Some(cursor);
        return WorldDrawRebuildStep::Stale;
    }
    if !cursor.publishing || cursor.faulted {
        state.draw_rebuild = Some(cursor);
        return WorldDrawRebuildStep::Pending;
    }
    if let Some(output) = cursor.output.as_mut() {
        let draft = cursor.drafts[usize::from(cursor.publish_draw)].as_mut().expect("publishing draw draft");
        if let Some(instance) = draft.take_next() {
            output.instances.push(instance);
            state.draw_rebuild = Some(cursor);
            context.consume_fuel(1);
            return WorldDrawRebuildStep::Pending;
        }
        let output = cursor.output.take().expect("completed draw output");
        if let Err(rejected) = cursor.staged.push_prevalidated(output) {
            retain_world_blocked_owner(state, WorldOpaqueOwner::Draw(WorldDynamicEntry { id: rejected.id, epoch: 0, value: rejected.value }));
            cursor.faulted = true;
            state.draw_rebuild = Some(cursor);
            mark_world_dynamic_fault(state, rejected.fault);
            return WorldDrawRebuildStep::Fault;
        }
        let draft = cursor.drafts[usize::from(cursor.publish_draw)].take().expect("terminal draw draft");
        assert!(draft.terminal_is_empty());
        drop(draft);
        cursor.publish_draw += 1;
        state.draw_rebuild = Some(cursor);
        context.consume_fuel(1);
        return WorldDrawRebuildStep::Pending;
    }
    if cursor.publish_draw < cursor.descriptor.draw_count {
        let draft = cursor.drafts[usize::from(cursor.publish_draw)].as_ref().expect("sealed draw draft");
        cursor.output = Some(SceneDraw3d { mesh_key: draft.mesh_key.clone(), mesh_version: draft.mesh_version, instances: Vec::with_capacity(draft.admitted), shadow_role: Default::default() });
        state.draw_rebuild = Some(cursor);
        context.consume_fuel(1);
        return WorldDrawRebuildStep::Pending;
    }
    let staged = std::mem::take(&mut cursor.staged);
    let previous = std::mem::replace(&mut state.draws, staged);
    if !previous.is_empty() {
        state.retired_draws = Some(previous);
    }
    state.draw_generation = cursor.descriptor.generation;
    assert!(cursor.terminal_is_empty());
    context.consume_fuel(1);
    WorldDrawRebuildStep::Complete
}

pub fn close_world3d_draw_rebuild_step(state: &mut World3dState, context: &mut semio_framework_job::StepContext<'_>) -> bool {
    if context.retirement_should_yield() {
        return false;
    }
    if let Some(retired) = state.retired_draws.as_mut() {
        if let Some(entry) = retired.take_last() {
            if let Err(owner) = quarantine_world_owner(WorldOpaqueOwner::Draw(entry)) {
                let WorldOpaqueOwner::Draw(entry) = owner else { unreachable!("retired draw owner") };
                retired.restore_last(entry);
                return false;
            }
            context.consume_fuel(1);
            return false;
        }
        state.retired_draws = None;
        context.consume_fuel(1);
        return false;
    }
    let Some(cursor) = state.draw_rebuild.as_mut() else {
        return true;
    };
    if let Some(output) = cursor.output.as_mut() {
        if output.instances.pop().is_some() {
            context.consume_fuel(1);
            return false;
        }
        cursor.output = None;
        context.consume_fuel(1);
        return false;
    }
    if let Some(draft) = cursor.drafts.iter_mut().flatten().find(|draft| !draft.terminal_is_empty()) {
        drop(draft.take_next());
        context.consume_fuel(1);
        return false;
    }
    if let Some(slot) = cursor.drafts.iter().position(Option::is_some) {
        let draft = cursor.drafts[slot].take().expect("terminal draft shell");
        assert!(draft.terminal_is_empty());
        drop(draft);
        context.consume_fuel(1);
        return false;
    }
    if let Some(entry) = cursor.staged.take_last() {
        if let Err(owner) = quarantine_world_owner(WorldOpaqueOwner::Draw(entry)) {
            let WorldOpaqueOwner::Draw(entry) = owner else { unreachable!("staged draw owner") };
            cursor.staged.restore_last(entry);
            return false;
        }
        context.consume_fuel(1);
        return false;
    }
    assert!(cursor.terminal_is_empty());
    state.draw_rebuild = None;
    context.consume_fuel(1);
    true
}

pub fn world3d_draw_rebuild_terminal_is_empty(state: &World3dState) -> bool {
    state.draw_rebuild.is_none() && state.retired_draws.is_none()
}
//#endregion 🧱️WorldDrawRebuild
