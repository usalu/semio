//! 📥️ World3d host snapshot apply, pointer button entry, interaction steps and dynamic retirement.

use super::*;

/// 🔢️ The revision this apply INSTALLS, decided BEFORE the draw rebuild it began is sealed.
///
/// 🩸️ `lease.revision` is this world's proposed source-publication revision: the bridge reserves
/// the next interaction revision for each changed payload — `publish_world3d_scene_bridge_snapshot`
/// is its one production writer —
/// so adopting it verbatim moves the counter BACKWARDS whenever anything bumped the revision
/// while the lease was in flight: a document-lane change (`sync_world3d_scene_document_lanes`),
/// a parallel framing (`sync_world3d_projection_content_frame`), a fit, a camera report. The
/// rollback then invalidated the very draws this apply had just built, because
/// `step_world3d_draw_rebuild` refuses a cursor whose `descriptor.revision` no longer equals
/// `state.interaction_revision`. Measured on 6118: generation3d's `procedural-preview` sat at
/// `state-meshes=3 state-instances=0 state-draws=0` with three `world3d draw rebuild step=Stale`
/// lines immediately after `world3d delivery applied`, and the tessellated column never painted
/// (ticket 26/09/17/WGPU-RENDERER-REACT-PARITY, `📓️w14b-generation3d-labels-preview-layout.md`).
///
/// 🧱️ The draws this apply built ARE the content of the revision it installs, so they are
/// re-stamped with it rather than left carrying whatever revision happened to be in force
/// when the first `Mesh` page opened the rebuild.
///
/// 📐️ A LOCALLY selected projection is view state the wire never overrides — React holds
/// it in `externalPendingProjectionSpec` and only falls back to the delivered
/// `cameraState.projectionSpec` when the pane has none (`🌐️World3dHost/🟦️.tsx`).
///
/// 🩸️ Without this the switch undid itself within one round trip: the template press
/// queues a settle, the settle publishes `setCamera`, the guest echoes a camera back, and
/// `setCamera`'s payload has no `projection` member at all (`orbit_camera_action`), so the
/// echo always arrives in the DELIVERED family. Measured live: the press moved the pane's
/// target onto the content centre and `liveCamera` still read `perspective` 8 s later
/// (`📓️w9b-projection-pane-framing-grid-materials.md` §1).
pub fn step_world3d_snapshot(state: &mut World3dState, context: &mut semio_framework_job::StepContext<'_>) -> World3dSnapshotApplyStep {
    if context.should_yield() {
        return World3dSnapshotApplyStep::Pending;
    }
    let Some(mut cursor) = state.bridge.snapshot_apply.take() else {
        return World3dSnapshotApplyStep::Idle;
    };
    if cursor.faulted {
        state.bridge.snapshot_apply = Some(cursor);
        return World3dSnapshotApplyStep::Fault;
    }
    if cursor.page == cursor.lease.page_count {
        let revision = state.session.interaction_revision.max(cursor.lease.revision);
        if cursor.draw_started {
            if let Some(rebuild) = state.scene.draw_rebuild.as_mut() {
                rebuild.descriptor.revision = revision;
            }
            if world3d_draw_rebuild_seal(&mut state.scene).is_err() {
                state.session.snapshot_fault = Some(World3dSnapshotFault::Capacity);
                cursor.faulted = true;
                state.bridge.snapshot_apply = Some(cursor);
                return World3dSnapshotApplyStep::Fault;
            }
        }
        if let Some(orbit) = cursor.staged_orbit.take() {
            state.camera.orbit = if state.camera.projection_selected {
                OrbitController { projection: ui_scene::math::projection_spec_family(state.camera.projection_spec), fov_y: ui_scene::math::projection_spec_fov_degrees(state.camera.projection_spec).to_radians(), ..orbit }
            } else {
                orbit
            };
        }
        state.session.interaction_revision = revision;
        if let Some(gesture) = state.authority.interaction_authority.as_mut().and_then(|authority| authority.gumball.as_mut()).filter(|gesture| gesture.live && gesture.streamed) {
            gesture.revision = revision;
        }
        state.bridge.snapshot_lease = Some(cursor.lease);
        state.overlay.prepared_status = cursor.status;
        state.session.snapshot_fault = None;
        context.consume_fuel(1);
        return World3dSnapshotApplyStep::Complete;
    }
    let resolved = world3d_snapshot_with_page(cursor.lease, cursor.page, |page| {
        let item_count = page.item_count();
        let item = page.item(usize::from(cursor.item)).copied();
        let id = item.and_then(|item| item.strings[0]).and_then(|span| page.string(span)).map(str::to_owned);
        let status = item.and_then(|item| {
            let text = item.strings[1].and_then(|span| page.string(span))?;
            World3dPreparedStatus::from_parts(text, [item.numbers[0], item.numbers[1], item.numbers[2]], item.indexes[0], item.indexes[1])
        });
        let component_source = if page.kind() == World3dSnapshotPageKind::Instance {
            item.map(|item| match (item.strings[1], item.strings[2]) {
                (None, None) => Ok(None),
                (Some(handle), Some(revision)) => ComponentSource3d::new(page.string(handle).ok_or(World3dSnapshotFault::PageState)?, page.string(revision).ok_or(World3dSnapshotFault::PageState)?).map(Some).ok_or(World3dSnapshotFault::PageState),
                _ => Err(World3dSnapshotFault::PageState),
            }).transpose()
        } else { Ok(None) };
        (page.kind(), item_count, item, id, status, component_source)
    });
    let (kind, item_count, item, id, status, component_source) = match resolved {
        Ok(resolved) => resolved,
        Err(fault) => {
            state.session.snapshot_fault = Some(fault);
            cursor.faulted = true;
            state.bridge.snapshot_apply = Some(cursor);
            return World3dSnapshotApplyStep::Stale;
        }
    };
    if usize::from(cursor.item) == item_count {
        cursor.page += 1;
        cursor.item = 0;
        state.bridge.snapshot_apply = Some(cursor);
        context.consume_fuel(1);
        return World3dSnapshotApplyStep::Pending;
    }
    let component_source = match component_source {
        Ok(source) => source.flatten(),
        Err(fault) => { state.session.snapshot_fault = Some(fault); cursor.faulted = true; state.bridge.snapshot_apply = Some(cursor); return World3dSnapshotApplyStep::Fault; }
    };
    let Some(item) = item else {
        state.session.snapshot_fault = Some(World3dSnapshotFault::PageState);
        cursor.faulted = true;
        state.bridge.snapshot_apply = Some(cursor);
        return World3dSnapshotApplyStep::Fault;
    };
    match kind {
        World3dSnapshotPageKind::Mesh if item.flags == 30 && item.index_len >= 2 => {
            let Some(mesh_key) = id.as_deref() else {
                state.session.snapshot_fault = Some(World3dSnapshotFault::PageState);
                cursor.faulted = true;
                state.bridge.snapshot_apply = Some(cursor);
                return World3dSnapshotApplyStep::Fault;
            };
            let instance_count = item.indexes[0];
            let byte_count = item.indexes[1].checked_add(size_of::<SceneDraw3d>() as u32).and_then(|bytes| bytes.checked_add(instance_count.checked_mul(size_of::<Instance3d>() as u32)?));
            let Some(byte_count) = byte_count else {
                state.session.snapshot_fault = Some(World3dSnapshotFault::Capacity);
                cursor.faulted = true;
                state.bridge.snapshot_apply = Some(cursor);
                return World3dSnapshotApplyStep::Fault;
            };
            let draw_bytes = item.indexes[1];
            let permit = match world3d_snapshot_claim_draw_permit(cursor.lease, 1, instance_count, draw_bytes) {
                Ok(permit) => permit,
                Err(fault) => {
                    state.session.snapshot_fault = Some(fault);
                    cursor.faulted = true;
                    state.bridge.snapshot_apply = Some(cursor);
                    return World3dSnapshotApplyStep::Fault;
                }
            };
            if !scene_mesh_awaits_its_asset(&state.scene, mesh_key) {
                begin_world_placeholder_mesh(&mut state.session, &mut state.authority, &mut state.retirement, &mut state.scene, mesh_key, WorldPlaceholderKind::Box);
            }
            let descriptor = WorldDrawRebuildDescriptor { generation: state.scene.draw_generation.wrapping_add(1), revision: state.session.interaction_revision, draw_count: 1, instance_count, byte_count };
            let mesh_version = live_world3d_mesh_version(&state.scene, mesh_key);
            let admitted = begin_world3d_draw_rebuild(&mut state.session, &mut state.retirement, &mut state.scene, descriptor).and_then(|()| world3d_draw_rebuild_admit_draw(&mut state.scene, mesh_key, mesh_version, u16::try_from(instance_count).map_err(|_| WorldDynamicFault::InstanceCapacity)?));
            if admitted.is_err() {
                state.session.snapshot_fault = Some(World3dSnapshotFault::Capacity);
                cursor.faulted = true;
                state.bridge.snapshot_apply = Some(cursor);
                return World3dSnapshotApplyStep::Fault;
            }
            cursor.draw_permit = Some(permit);
            cursor.draw_started = true;
            cursor.draw_index = 0;
        }
        World3dSnapshotPageKind::Mesh if item.flags == 31 && item.index_len >= 6 => {
            let Some(mesh_key) = id.as_deref() else {
                state.session.snapshot_fault = Some(World3dSnapshotFault::PageState);
                cursor.faulted = true;
                state.bridge.snapshot_apply = Some(cursor);
                return World3dSnapshotApplyStep::Fault;
            };
            let (own_instances, ordinal, total_draws, total_instances, total_bytes) = (item.indexes[0], item.indexes[2], item.indexes[3], item.indexes[4], item.indexes[5]);
            let Ok(total_draws) = u16::try_from(total_draws) else {
                state.session.snapshot_fault = Some(World3dSnapshotFault::Capacity);
                cursor.faulted = true;
                state.bridge.snapshot_apply = Some(cursor);
                return World3dSnapshotApplyStep::Fault;
            };
            if ordinal == 0 {
                let byte_count = total_bytes.checked_add(u32::from(total_draws).saturating_mul(size_of::<SceneDraw3d>() as u32)).and_then(|bytes| bytes.checked_add(total_instances.checked_mul(size_of::<Instance3d>() as u32)?));
                let permit = byte_count.ok_or(World3dSnapshotFault::Capacity).and_then(|_| world3d_snapshot_claim_draw_permit(cursor.lease, total_draws, total_instances, total_bytes));
                let (Some(byte_count), Ok(permit)) = (byte_count, permit) else {
                    state.session.snapshot_fault = Some(World3dSnapshotFault::Capacity);
                    cursor.faulted = true;
                    state.bridge.snapshot_apply = Some(cursor);
                    return World3dSnapshotApplyStep::Fault;
                };
                let descriptor = WorldDrawRebuildDescriptor { generation: state.scene.draw_generation.wrapping_add(1), revision: state.session.interaction_revision, draw_count: total_draws, instance_count: total_instances, byte_count };
                if begin_world3d_draw_rebuild(&mut state.session, &mut state.retirement, &mut state.scene, descriptor).is_err() {
                    state.session.snapshot_fault = Some(World3dSnapshotFault::Capacity);
                    cursor.faulted = true;
                    state.bridge.snapshot_apply = Some(cursor);
                    return World3dSnapshotApplyStep::Fault;
                }
                cursor.draw_permit = Some(permit);
                cursor.draw_started = true;
            }
            let mesh_version = live_world3d_mesh_version(&state.scene, mesh_key);
            let admitted = u16::try_from(own_instances).map_err(|_| WorldDynamicFault::InstanceCapacity).and_then(|count| world3d_draw_rebuild_admit_draw(&mut state.scene, mesh_key, mesh_version, count));
            let Ok(ordinal) = u16::try_from(ordinal) else {
                state.session.snapshot_fault = Some(World3dSnapshotFault::Capacity);
                cursor.faulted = true;
                state.bridge.snapshot_apply = Some(cursor);
                return World3dSnapshotApplyStep::Fault;
            };
            if admitted.is_err() {
                state.session.snapshot_fault = Some(World3dSnapshotFault::Capacity);
                cursor.faulted = true;
                state.bridge.snapshot_apply = Some(cursor);
                return World3dSnapshotApplyStep::Fault;
            }
            cursor.draw_index = ordinal;
        }
        World3dSnapshotPageKind::Instance if item.number_len >= 14 && cursor.draw_started => {
            let Some(id) = id.as_deref() else {
                state.session.snapshot_fault = Some(World3dSnapshotFault::PageState);
                cursor.faulted = true;
                state.bridge.snapshot_apply = Some(cursor);
                return World3dSnapshotApplyStep::Fault;
            };
            let position = [item.numbers[0] as f32, item.numbers[1] as f32, item.numbers[2] as f32];
            let rotation = [item.numbers[3] as f32, item.numbers[4] as f32, item.numbers[5] as f32, item.numbers[6] as f32];
            let scale = [item.numbers[7] as f32, item.numbers[8] as f32, item.numbers[9] as f32];
            let color = [item.numbers[10] as f32, item.numbers[11] as f32, item.numbers[12] as f32, item.numbers[13] as f32];
            let model = Instance3d::model_from_trs(position, rotation, scale);
            if world3d_draw_rebuild_admit_retained_instance(&mut state.scene, cursor.draw_index, id, model, color, SceneColorSource3d::from_snapshot_flag(item.flags), false, false, component_source).is_err() {
                state.session.snapshot_fault = Some(World3dSnapshotFault::Capacity);
                cursor.faulted = true;
                state.bridge.snapshot_apply = Some(cursor);
                return World3dSnapshotApplyStep::Fault;
            }
        }
        World3dSnapshotPageKind::Camera if item.number_len >= 12 => {
            let projection = if item.numbers[11] >= 0.5 { CameraProjection3d::Orthographic } else { CameraProjection3d::Perspective };
            cursor.staged_orbit = Some(OrbitController::from_camera(&Camera3d {
                position: Vec3::new(item.numbers[0] as f32, item.numbers[1] as f32, item.numbers[2] as f32),
                target: Vec3::new(item.numbers[3] as f32, item.numbers[4] as f32, item.numbers[5] as f32),
                up: Vec3::new(item.numbers[6] as f32, item.numbers[7] as f32, item.numbers[8] as f32),
                fov_y: item.numbers[9] as f32 * std::f32::consts::PI / 180.0,
                near: ui_scene::math::WORLD_ORBIT_CAMERA_NEAR,
                far: ui_scene::math::WORLD_ORBIT_CAMERA_MIN_FAR,
                projection,
                zoom: item.numbers[10] as f32,
            }));
        }
        World3dSnapshotPageKind::Camera => {
            state.session.snapshot_fault = Some(World3dSnapshotFault::PageState);
            cursor.faulted = true;
            state.bridge.snapshot_apply = Some(cursor);
            return World3dSnapshotApplyStep::Fault;
        }
        World3dSnapshotPageKind::Status if (10..=14).contains(&item.flags) && item.number_len >= 14 && cursor.draw_started => {
            let Some(id) = id.as_deref() else {
                state.session.snapshot_fault = Some(World3dSnapshotFault::PageState);
                cursor.faulted = true;
                state.bridge.snapshot_apply = Some(cursor);
                return World3dSnapshotApplyStep::Fault;
            };
            let vector = [item.numbers[0], item.numbers[1], item.numbers[2]];
            let magnitude = (vector[0] * vector[0] + vector[1] * vector[1] + vector[2] * vector[2]).sqrt().max(item.numbers[3].abs()).max(1.0e-6) as f32;
            let position = [item.numbers[11] as f32, item.numbers[12] as f32, item.numbers[13] as f32];
            let color = match item.flags {
                10 => [0.957, 0.447, 0.714, 1.0],
                11 => [0.918, 0.702, 0.031, 1.0],
                12 => [0.984, 0.443, 0.522, 1.0],
                13 => [0.133, 0.827, 0.933, 1.0],
                _ => [0.655, 0.545, 0.98, 1.0],
            };
            let model = Instance3d::model_from_trs(position, [0.0, 0.0, 0.0, 1.0], [0.025, 0.025, magnitude.min(1.0)]);
            if world3d_draw_rebuild_admit_instance(&mut state.scene, 0, id, model, color, false, false).is_err() {
                state.session.snapshot_fault = Some(World3dSnapshotFault::Capacity);
                cursor.faulted = true;
                state.bridge.snapshot_apply = Some(cursor);
                return World3dSnapshotApplyStep::Fault;
            }
        }
        World3dSnapshotPageKind::Status if item.flags == 20 && item.number_len >= 3 && item.index_len >= 2 => {
            let Some(status) = status else {
                state.session.snapshot_fault = Some(World3dSnapshotFault::PageState);
                cursor.faulted = true;
                state.bridge.snapshot_apply = Some(cursor);
                return World3dSnapshotApplyStep::Fault;
            };
            cursor.status[usize::from(id.as_deref() == Some("de"))] = Some(status);
        }
        _ => {}
    }
    cursor.item += 1;
    state.bridge.snapshot_apply = Some(cursor);
    context.consume_fuel(1);
    World3dSnapshotApplyStep::Pending
}

/// 🖱️📋️ A right RELEASE owes the unconditional camera report and NOTHING else: the menu is
/// the shell's, opened on the press (see the authority's own right-release arm).
#[cfg(any(test, feature = "testkit"))]
pub fn handle_world3d_pointer_button(state: &mut World3dState, x: f32, y: f32, down: bool, button: i16, modifiers: &PointerModifiers) -> Option<ActionDescriptor> {
    let inner = world_pick_rect(&state.session);
    if !inner.contains(x, y) {
        return None;
    }
    let shift = modifiers.shift;
    let ctrl = modifiers.ctrl;
    if down {
        if button == 0 {
            if state.session.active_utility == "surfaceBrush" {
                return None;
            }
            if state.selection.interaction_mode == "paint" {
                state.selection.paint_stroke_active = true;
                let (object_id, u, v) = pick_paint_hit(&state.session, &state.camera, &state.scene, x, y, inner)?;
                return Some(ActionDescriptor { controller_id: state.session.controller_id.clone(), action: "paintAt".into(), args: Some(semio_framework::dsl_value!({ "surfaceId": state.session.surface_id, "objectId": object_id, "u": u, "v": v, "phase": "stream" })) });
            }
            if state.session.active_utility == "brush" || (state.session.active_utility == "select" && state.selection.granularity == "vertex") {
                if let Some(full_id) = pick_vortex_at(&state.session, &state.camera, &state.scene, x, y, inner) {
                    let merge = world_merge_mode(shift, ctrl).wire_label();
                    return Some(ActionDescriptor { controller_id: state.session.controller_id.clone(), action: "worldVortexSelect".into(), args: Some(semio_framework::dsl_value!({ "surfaceId": state.session.surface_id, "fullId": full_id, "merge": merge })) });
                }
            } else if state.session.active_utility == "select" {
                if !state.selection.selected_ids.is_empty() {
                    if let Some(handle) = pick_gumball_handle_at(&state.session, &state.camera, &state.selection, &state.gumball, &state.scene, x, y, inner) {
                        start_gumball_drag(&mut state.camera, &mut state.selection, &mut state.gumball, &mut state.scene, handle, x, y, inner);
                        return None;
                    }
                }
                if state.gumball.gumball_handle.is_none() && !component_mode_active(&state.selection) {
                    state.selection.press_object_id = pick_instance_at(&state.session, &state.camera, &state.scene, x, y, inner);
                } else {
                    state.selection.press_object_id = None;
                }
            }
            state.selection.marquee_active = true;
            state.selection.marquee_points = vec![[x, y]];
            return None;
        }
        if button == 2 {
            state.selection.marquee_active = false;
            state.selection.marquee_points.clear();
            state.selection.right_press_point = Some([x, y]);
        }
        return None;
    }
    if button == 0 && state.selection.marquee_active {
        state.selection.marquee_active = false;
        if is_click_gesture(&state.selection, x, y) {
            state.selection.marquee_points.clear();
            state.selection.marquee_preview_ids.clear();
            state.selection.press_object_id = None;
            state.selection.drag_object_id = None;
            state.selection.drag_last_position = None;
            if state.session.active_utility == "surfaceBrush" {
                if let Some((object_id, position, normal)) = pick_surface_at(&state.session, &state.camera, &state.scene, x, y, inner) {
                    return Some(ActionDescriptor {
                        controller_id: state.session.controller_id.clone(),
                        action: "worldSurfacePlace".into(),
                        args: Some(semio_framework::dsl_value!({
                            "surfaceId": state.session.surface_id,
                            "pane": state.session.surface_id,
                            "objectId": object_id,
                            "position": position,
                            "normal": normal,
                        })),
                    });
                }
                return None;
            }
            if state.gumball.gumball_handle.is_none() {
                return pick_select_action(&state.session, &state.camera, &state.selection, &state.interaction, &state.scene, x, y, inner, shift, ctrl);
            }
        } else {
            return marquee_select_action(&mut state.session, &mut state.camera, &mut state.selection, &mut state.interaction, &mut state.scene, render_pick_viewport(&state.session), shift, ctrl);
        }
    }
    if button == 0 && state.selection.interaction_mode == "paint" && state.selection.paint_stroke_active {
        state.selection.paint_stroke_active = false;
        let mut actions = Vec::new();
        actions.push(ActionDescriptor { controller_id: state.session.controller_id.clone(), action: "paintAt".into(), args: Some(semio_framework::dsl_value!({ "surfaceId": state.session.surface_id, "phase": "commit" })) });
        return actions.first().cloned();
    }
    if button == 0 {
        if state.session.active_utility == "surfaceBrush" {
            if let Some((object_id, position, normal)) = pick_surface_at(&state.session, &state.camera, &state.scene, x, y, inner) {
                return Some(ActionDescriptor {
                    controller_id: state.session.controller_id.clone(),
                    action: "worldSurfacePlace".into(),
                    args: Some(semio_framework::dsl_value!({
                        "surfaceId": state.session.surface_id,
                        "pane": state.session.surface_id,
                        "objectId": object_id,
                        "position": position,
                        "normal": normal,
                    })),
                });
            }
            return None;
        }
        if let Some(action) = gumball_commit_action(&state.session, &state.selection, &state.gumball) {
            state.gumball.gumball_handle = None;
            reset_gumball_preview(&mut state.gumball);
            return Some(action);
        }
        state.gumball.gumball_handle = None;
        reset_gumball_preview(&mut state.gumball);
        state.selection.press_object_id = None;
        if let Some(object_id) = state.selection.drag_object_id.take() {
            if let Some(position) = state.selection.drag_last_position {
                if !is_click_gesture(&state.selection, x, y) {
                    return Some(ActionDescriptor {
                        controller_id: state.session.controller_id.clone(),
                        action: "worldRelocate".into(),
                        args: Some(semio_framework::dsl_value!({
                            "surfaceId": state.session.surface_id,
                            "objectId": object_id,
                            "position": position,
                        })),
                    });
                }
            }
        }
        if state.session.active_utility == "brush" {
            if let Some(preview) = state.gumball.brush_preview.clone() {
                if let (Some(target), Some(kind), Some(index)) = (preview.target_vortex_full_id, preview.object_kind_id, preview.source_vortex_index) {
                    let origin = preview.origin.unwrap_or([0.0, 0.0, 0.0]);
                    return Some(ActionDescriptor {
                        controller_id: state.session.controller_id.clone(),
                        action: "addBrushObject".into(),
                        args: Some(semio_framework::dsl_value!({
                            "targetVortexFullId": target,
                            "objectKindId": kind,
                            "sourceVortexIndex": index,
                            "origin": origin,
                            "orientation": preview.orientation.unwrap_or([0.0, 0.0, 0.0, 1.0]),
                            "scale": preview.scale.as_ref().map(semio_framework::DslValue::from),
                        })),
                    });
                }
            }
        }
        return pick_select_action(&state.session, &state.camera, &state.selection, &state.interaction, &state.scene, x, y, inner, shift, ctrl);
    }
    if button == 2 {
        state.selection.right_press_point = None;
        return Some(orbit_camera_action(&state.session, &state.camera));
    }
    if button == 1 {
        return Some(orbit_camera_action(&state.session, &state.camera));
    }
    None
}

pub fn step_world3d_interaction(state: &mut World3dState, generation: u64, input: &mut semio_framework_ui_wgpu_input::InputState<ActionDescriptor>, context: &mut semio_framework_job::StepContext<'_>) -> WorldInteractionAuthorityStep {
    let Some(mut authority) = state.authority.interaction_authority.take() else {
        return WorldInteractionAuthorityStep::Fault;
    };
    let step = authority.step(&state.selection, &state.gumball, &state.scene, generation, input, context);
    state.authority.interaction_authority = Some(authority);
    step
}

pub fn step_world3d_dynamic_retirement(state: &mut World3dState, context: &mut semio_framework_job::StepContext<'_>) -> bool {
    if context.retirement_should_yield() {
        return false;
    }
    if let Some(cursor) = state.scene.placeholder_build.as_mut() {
        if !cursor.close_step() || !cursor.terminal_is_empty() {
            context.consume_fuel(1);
            return false;
        }
        state.scene.placeholder_build = None;
        context.consume_fuel(1);
        return false;
    }
    if state.terrain.terrain_pending.is_some() || state.terrain.terrain_build.is_some() {return false;}
    if let Some(cursor) = state.overlay.face_overlay_build.as_mut() {
        if !cursor.close_step() || !cursor.terminal_is_empty() {
            context.consume_fuel(1);
            return false;
        }
        state.overlay.face_overlay_build = None;
        context.consume_fuel(1);
        return false;
    }
    if let Some(index) = state.overlay.face_overlay_colors.iter().position(Option::is_some) {
        state.overlay.face_overlay_colors[index] = None;
        context.consume_fuel(1);
        return false;
    }
    if state.overlay.face_overlay_generation.take().is_some() {
        context.consume_fuel(1);
        return false;
    }
    if state.overlay.face_overlay_retired_generation.take().is_some() {
        context.consume_fuel(1);
        return false;
    }
    if !state.assets.asset_io.terminal_is_empty() {
        let _ = state.assets.asset_io.close_step();
        context.consume_fuel(1);
        return false;
    }
    if !close_world3d_draw_rebuild_step(&mut state.scene, context) {
        return false;
    }
    let Some(mut retirement) = state.retirement.dynamic_retirement.take() else {
        return true;
    };
    let complete = retirement.step(&state.selection, &state.gumball, &state.scene,context);
    context.consume_fuel(1);
    if complete && retirement.terminal_is_empty() {
        return true;
    }
    state.retirement.dynamic_retirement = Some(retirement);
    false
}

pub fn world3d_dynamic_retirement_terminal_is_empty(state: &World3dState) -> bool {
    state.retirement.dynamic_retirement.is_none()
        && state.bridge.scene_bridge.is_none()
        && state.bridge.scene_bridge_close.is_none()
        && state.bridge.scene_bridge_lease.is_none()
        && state.bridge.scene_bridge_retired.is_none()
        && state.scene.dynamic_blocked_owner.is_none()
        && state.scene.dynamic_mesh_close.is_none()
        && state.scene.dynamic_blocked_mesh.is_none()
        && state.scene.native_mesh_admission.is_none()
        && state.scene.placeholder_build.is_none()
        && state.terrain.terrain_build.is_none() && state.terrain.terrain_pending.is_none()
        && state.overlay.face_overlay_build.is_none()
        && state.overlay.face_overlay_generation.is_none()
        && state.overlay.face_overlay_retired_generation.is_none()
        && state.overlay.face_overlay_colors.iter().all(Option::is_none)
        && state.camera.fit_bounds_cursor.is_none()
        && world3d_draw_rebuild_terminal_is_empty(&state.scene)
        && state.scene.meshes.is_empty()
        && state.scene.mesh_versions.is_empty()
        && state.scene.draws.is_empty()
        && state.scene.reference_pixels.is_empty()
        && state.scene.mesh_paint_textures.is_empty()
        && state.interaction.instance_interaction_ids.is_empty()
        && state.interaction.instance_interaction_granularity_ids.is_empty()
        && state.selection.local_hover_granularity_id.is_none()
        && state.assets.asset_io.terminal_is_empty()
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
    state.scene.native_mesh_admission.is_some()
        || state.scene.placeholder_build.is_some()
        || state.terrain.terrain_build.is_some() || state.terrain.terrain_pending.is_some()
        || state.overlay.face_overlay_build.is_some()
        || state.bridge.scene_bridge.is_some()
        || state.bridge.snapshot_apply.is_some()
        || state.scene.draw_rebuild.is_some()
        || state.scene.retired_draws.is_some()
        || state.retirement.dynamic_retirement.is_some()
        || state.bridge.scene_bridge_retired.is_some()
        || state.bridge.scene_bridge_lease.is_some_and(|lease| state.bridge.snapshot_lease != Some(lease))
        || world3d_camera_fit_work_pending(&state.camera, &state.scene)
}
