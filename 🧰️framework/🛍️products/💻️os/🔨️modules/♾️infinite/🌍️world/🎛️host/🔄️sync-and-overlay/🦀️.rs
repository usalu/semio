//! 🔄️ World3d host sync: camera fit and scene bridge steps, state sync, tool run trace draws and the modelling overlay.

use super::*;

/// 📦️ Advances at most one retained draw transition or one transformed mesh-AABB corner. This is
/// Three's `Box3.setFromObject(instancesGroup)` expressed over the resident WGPU draw registry.
pub fn step_world3d_camera_fit(state: &mut World3dState) -> World3dCameraFitStep {
    if state.retirement.dynamic_retirement.is_some() {
        state.camera.fit_bounds_cursor = None;
        return World3dCameraFitStep::Idle;
    }
    let Some(request) = state.camera.fit_request else { return World3dCameraFitStep::Idle };
    let owner = World3dCameraFitOwner { revision: request.revision, camera_digest: state.camera.scene_camera_digest };
    if state.camera.fit_owner != Some(owner) {
        state.camera.fit_owner = Some(owner);
        state.camera.fit_bounds_cursor = None;
        state.camera.camera_user_moved = false;
    }
    let key = world3d_camera_fit_key(owner, request.bounds);
    if state.camera.fit_applied_key == Some(key) || state.camera.camera_user_moved {
        state.camera.fit_bounds_cursor = None;
        return World3dCameraFitStep::Idle;
    }
    if state.bridge.scene_bridge.is_some()
        || state.bridge.snapshot_apply.is_some()
        || state.scene.draw_rebuild.is_some()
        || state.scene.retired_draws.is_some()
        || state.bridge.scene_bridge_lease.is_some_and(|lease| state.bridge.snapshot_lease != Some(lease))
    {
        return World3dCameraFitStep::Pending;
    }
    sync_world3d_projection_content_frame(&mut state.session, &mut state.camera, &mut state.selection, &mut state.scene, &mut state.bridge);
    if let Some((minimum, maximum)) = request.bounds {
        return apply_world3d_camera_fit(&mut state.session, &mut state.camera, &mut state.scene, key, request, minimum, maximum);
    }
    let mut cursor = state.camera.fit_bounds_cursor.take().unwrap_or_else(|| World3dCameraBoundsCursor::new(key, state.scene.draw_generation, state.scene.geometry_generation));
    if cursor.key != key || cursor.draw_generation != state.scene.draw_generation || cursor.geometry_generation != state.scene.geometry_generation {
        let completed_empty_generation = cursor.empty_generation();
        cursor = World3dCameraBoundsCursor::new(key, state.scene.draw_generation, state.scene.geometry_generation);
        if !completed_empty_generation {
            state.camera.fit_bounds_cursor = Some(cursor);
            return World3dCameraFitStep::Stale;
        }
    }
    if usize::from(cursor.draw) >= usize::from(state.scene.draws.len) {
        if cursor.any {
            return apply_world3d_camera_fit(&mut state.session, &mut state.camera, &mut state.scene, key, request, cursor.minimum, cursor.maximum);
        }
        cursor.draw = u16::MAX;
        state.camera.fit_bounds_cursor = Some(cursor);
        return World3dCameraFitStep::Idle;
    }
    let Some(draw) = state.scene.draws.get(usize::from(cursor.draw)) else {
        cursor.draw += 1;
        cursor.instance = 0;
        cursor.corner = 0;
        state.camera.fit_bounds_cursor = Some(cursor);
        return World3dCameraFitStep::Pending;
    };
    let Some(mesh) = state.scene.meshes.get(&draw.mesh_key).copied() else {
        cursor.draw += 1;
        cursor.instance = 0;
        cursor.corner = 0;
        state.camera.fit_bounds_cursor = Some(cursor);
        return World3dCameraFitStep::Pending;
    };
    let Ok((local_minimum, local_maximum)) = mesh.aabb() else {
        cursor.draw += 1;
        cursor.instance = 0;
        cursor.corner = 0;
        state.camera.fit_bounds_cursor = Some(cursor);
        return World3dCameraFitStep::Pending;
    };
    let Some(instance) = draw.instances.get(cursor.instance as usize) else {
        cursor.draw += 1;
        cursor.instance = 0;
        cursor.corner = 0;
        state.camera.fit_bounds_cursor = Some(cursor);
        return World3dCameraFitStep::Pending;
    };
    let corner = cursor.corner;
    let rendered_scale = if state.selection.show_edges
        && !state.selection.provisional_instance_ids.contains(&instance.id)
        && mesh.schema().is_ok_and(|schema| schema.edges == 0)
    {
        WORLD3D_RENDERED_OUTLINE_SCALE
    } else {
        1.0
    };
    let point = instance.model.transform_point(Vec3::new(
        (if corner & 1 == 0 { local_minimum[0] } else { local_maximum[0] }) * rendered_scale,
        (if corner & 2 == 0 { local_minimum[1] } else { local_maximum[1] }) * rendered_scale,
        (if corner & 4 == 0 { local_minimum[2] } else { local_maximum[2] }) * rendered_scale,
    ));
    if point.x.is_finite() && point.y.is_finite() && point.z.is_finite() {
        cursor.expand(point);
    }
    cursor.corner += 1;
    if cursor.corner == 8 {
        cursor.corner = 0;
        cursor.instance += 1;
    }
    state.camera.fit_bounds_cursor = Some(cursor);
    World3dCameraFitStep::Pending
}

/// 🌉️ One budgeted turn of the mesh-wire → snapshot bridge. Drives the staged payload through
/// parse → retained attribute candidates → per-mesh publication → snapshot page admission, then hands the sealed lease to
/// [`sync_world3d_state`], which feeds it to the very same apply ladder a typed producer's own
/// snapshot rides.
///
/// 📐️ The delivered spec seeds the framing plane only while the pane holds none of its
/// own — React's `cameraState.projectionSpec ?? externalPendingProjectionSpec` precedence
/// (`🌐️World3dHost/🟦️.tsx`), the same rule the snapshot apply keeps for the family.
pub fn step_world3d_scene_bridge(state: &mut World3dState, context: &mut semio_framework_job::StepContext<'_>) -> World3dSceneBridgeStep {
    if context.should_yield() {
        return World3dSceneBridgeStep::Pending;
    }
    if state.bridge.scene_bridge_retired.is_some() && state.bridge.snapshot_apply.is_none() && state.bridge.scene_bridge_lease == state.bridge.snapshot_lease {
        step_world3d_scene_bridge_close(&mut state.bridge,context);
        context.consume_fuel(1);
        return World3dSceneBridgeStep::Pending;
    }
    if state.bridge.scene_bridge_close.is_some()||state.bridge.scene_bridge.as_ref().is_some_and(|cursor|matches!(cursor.phase,World3dSceneBridgePhase::Close)){
        step_world3d_scene_bridge_close(&mut state.bridge,context);
        context.consume_fuel(1);
        return World3dSceneBridgeStep::Pending;
    }
    let Some(mut cursor) = state.bridge.scene_bridge.take() else {
        return World3dSceneBridgeStep::Idle;
    };
    context.consume_fuel(1);
    match cursor.phase {
        World3dSceneBridgePhase::Parse => {
            if cursor.instances_json.len() > WORLD_INTERACTION_TOPOLOGY_BYTE_CAPACITY {
                state.session.snapshot_fault = Some(World3dSnapshotFault::Capacity);
                cursor.phase=World3dSceneBridgePhase::Close;state.bridge.scene_bridge=Some(cursor);return World3dSceneBridgeStep::Fault;
            }
            cursor.meshes = semio_framework_pack_json::from_json_str::<Vec<World3dSceneMeshEntry>>(&cursor.meshes_json,semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap_or_default();
            cursor.instances = serde_json::from_str::<Vec<World3dSceneInstanceEntry>>(&cursor.instances_json).unwrap_or_default();
            cursor.camera = serde_json::from_str::<World3dSceneCameraRecord>(&cursor.camera_json).ok();
            if let Some(camera) = cursor.camera.as_ref().filter(|_| !state.camera.projection_selected) {
                state.camera.projection_frame_policy = camera.projection_frame;
                if camera.projection_frame == Viewport3dProjectionFramePolicy::PreserveCamera {
                    state.camera.projection_frame_owed = false;
                    state.camera.projection_frame_key = None;
                    state.camera.projection_frame_zoom = None;
                }
            }
            if let Some(camera) = cursor.camera.as_ref().filter(|_| !state.camera.projection_selected) {
                if let Some(spec) = camera.projection {
                    state.camera.projection_spec = spec;
                    if cursor.camera_changed && camera.projection_frame == Viewport3dProjectionFramePolicy::Content {
                        state.camera.projection_frame_owed = true;
                        state.camera.projection_frame_key = None;
                        state.camera.projection_frame_zoom = None;
                        state.camera.camera_user_moved = false;
                    }
                }
            }
            cursor.meshes.retain(World3dSceneMeshEntry::names_a_mesh);
            for mesh in &mut cursor.meshes {
                if mesh.data.normals.len() != mesh.data.positions.len() && !mesh.data.attributes.values().any(|attribute| attribute.semantic == semio_framework::MeshAttributeSemantic::Normal) {
                    mesh.data.compute_normals();
                }
            }
            if !world3d_scene_instance_interactions_are_bounded(&cursor.instances) {
                state.session.snapshot_fault = Some(World3dSnapshotFault::Capacity);
                cursor.phase=World3dSceneBridgePhase::Close;state.bridge.scene_bridge=Some(cursor);return World3dSceneBridgeStep::Fault;
            }
            cursor.instances.retain(|instance| cursor.meshes.iter().any(|mesh| mesh.id == instance.mesh_id));
            state.overlay.scalar_field_status=if cursor.scalar_field.is_some(){World3dScalarFieldStatus::Missing}else{World3dScalarFieldStatus::None};
            cursor.phase = World3dSceneBridgePhase::ScalarAttributes;
            state.bridge.scene_bridge = Some(cursor);
            World3dSceneBridgeStep::Pending
        }
        World3dSceneBridgePhase::ScalarAttributes=>{
            let Some(field)=cursor.scalar_field.as_ref()else{cursor.phase=World3dSceneBridgePhase::Meshes;state.bridge.scene_bridge=Some(cursor);return World3dSceneBridgeStep::Pending};
            let Some(mesh)=cursor.meshes.get_mut(cursor.scalar_mesh_cursor)else{cursor.phase=World3dSceneBridgePhase::Meshes;state.bridge.scene_bridge=Some(cursor);return World3dSceneBridgeStep::Pending};
            if mesh.id!=field.mesh_id{cursor.scalar_mesh_cursor+=1;state.bridge.scene_bridge=Some(cursor);return World3dSceneBridgeStep::Pending;}
            if !world3d_scalar_field_fits(&mesh.data,field){state.overlay.scalar_field_status=World3dScalarFieldStatus::Mismatch;cursor.phase=World3dSceneBridgePhase::Meshes;state.bridge.scene_bridge=Some(cursor);return World3dSceneBridgeStep::Pending;}
            if cursor.scalar_slot_cursor<mesh.data.attributes.slot_count(){
                let result=mesh.data.attributes.retire_slot_if(cursor.scalar_slot_cursor,|_,attribute|attribute.semantic!=semio_framework::MeshAttributeSemantic::Color,context.retained_grant());
                match result{Ok(progress)=>{context.consume_retained(progress).expect("original attribute move fits supplied caller grant");if progress.copied_items!=0{cursor.scalar_slot_cursor+=1;}},Err(_)=>{state.session.snapshot_fault=Some(World3dSnapshotFault::Capacity);cursor.phase=World3dSceneBridgePhase::Close;state.bridge.scene_bridge=Some(cursor);return World3dSceneBridgeStep::Fault;}}
                state.bridge.scene_bridge=Some(cursor);return World3dSceneBridgeStep::Pending;
            }
            world3d_apply_scalar_field_colors(&mut mesh.data,field);
            state.overlay.scalar_field_status=World3dScalarFieldStatus::Applied;
            cursor.phase=World3dSceneBridgePhase::Meshes;
            state.bridge.scene_bridge=Some(cursor);
            World3dSceneBridgeStep::Pending
        }
        World3dSceneBridgePhase::Close=>unreachable!("original scene cursor close is drained before advancing"),
        World3dSceneBridgePhase::Meshes => {
            if state.scene.dynamic_mesh_close.is_some() || state.scene.dynamic_blocked_mesh.is_some() {
                step_world3d_mesh_close(&mut state.scene,context);
                state.bridge.scene_bridge = Some(cursor);
                return World3dSceneBridgeStep::Pending;
            }
            if state.scene.placeholder_build.is_some() {
                step_world_placeholder_mesh_batch(&mut state.session, &mut state.interaction, &mut state.authority, &mut state.scene, WORLD3D_BRIDGE_MESH_WRITES_PER_STEP);
                state.bridge.scene_bridge = Some(cursor);
                return World3dSceneBridgeStep::Pending;
            }
            let Some(entry) = cursor.meshes.get(cursor.mesh_cursor) else {
                cursor.phase = World3dSceneBridgePhase::Pages;
                state.bridge.scene_bridge = Some(cursor);
                return World3dSceneBridgeStep::Pending;
            };
            if !entry.has_inline_geometry() {
                let (id, url, kind) = (entry.id.clone(), entry.url.clone(), entry.kind.clone());
                declare_scene_mesh_source(&mut state.session, &mut state.authority, &mut state.retirement, &mut state.scene, &id, url.as_deref(), kind.as_deref());
                cursor.mesh_cursor += 1;
                cursor.retiring_mesh = false;
                state.bridge.scene_bridge = Some(cursor);
                return World3dSceneBridgeStep::Pending;
            }
            let hashing=cursor.mesh_digest.get_or_insert_with(WorldMeshDigestCursor::default);
            let digest=match hashing.step(&entry.data){Ok(Some(digest))=>digest,Ok(None)=>{state.bridge.scene_bridge=Some(cursor);return World3dSceneBridgeStep::Pending},Err(_)=>{state.session.snapshot_fault=Some(World3dSnapshotFault::Capacity);cursor.phase=World3dSceneBridgePhase::Close;state.bridge.scene_bridge=Some(cursor);return World3dSceneBridgeStep::Fault}};
            if state.scene.scene_mesh_digests.get(&entry.id) == Some(&digest) && state.scene.meshes.contains_key(&entry.id) {
                cursor.mesh_cursor += 1;
                cursor.retiring_mesh = false;
                cursor.mesh_digest=None;
                state.bridge.scene_bridge = Some(cursor);
                return World3dSceneBridgeStep::Pending;
            }
            if state.scene.meshes.contains_key(&entry.id) && !cursor.retiring_mesh {
                cursor.retiring_mesh = true;
                retire_world_mesh(&mut state.scene, &entry.id.clone());
                state.bridge.scene_bridge = Some(cursor);
                return World3dSceneBridgeStep::Pending;
            }
            let id=entry.id.clone();
            let buffers=std::mem::take(&mut cursor.meshes[cursor.mesh_cursor].data);
            state.scene.placeholder_generation = state.scene.placeholder_generation.wrapping_add(1).max(1);
            match WorldPlaceholderMeshCursor::inline(&id, buffers, state.scene.placeholder_generation, state.session.interaction_revision) {
                Ok(build) => {
                    state.scene.placeholder_build = Some(build);
                    state.scene.scene_mesh_digests.insert(id, digest);
                    cursor.mesh_digest=None;
                    cursor.mesh_cursor += 1;
                    cursor.retiring_mesh = false;
                    state.bridge.scene_bridge = Some(cursor);
                    World3dSceneBridgeStep::Pending
                }
                Err(_) => {
                    state.session.snapshot_fault = Some(World3dSnapshotFault::Capacity);
                    cursor.phase=World3dSceneBridgePhase::Close;state.bridge.scene_bridge=Some(cursor);
                World3dSceneBridgeStep::Fault
                }
            }
        }
        World3dSceneBridgePhase::Pages if !world3d_scene_bridge_has_pages(&state.scene, &cursor) => {
            state.bridge.scene_bridge_digest = Some(cursor.digest);
            if cursor.camera_changed {
                state.camera.scene_camera_digest = Some(cursor.camera_digest);
            }
            cursor.phase=World3dSceneBridgePhase::Close;state.bridge.scene_bridge=Some(cursor);
            World3dSceneBridgeStep::Complete
        }
        World3dSceneBridgePhase::Pages => match publish_world3d_scene_bridge_snapshot(&mut state.session, &mut state.selection, &mut state.interaction, &mut state.scene, &mut state.bridge, &cursor) {
            Ok(lease) => {
                if let Some(previous) = state.bridge.scene_bridge_lease.replace(lease) {
                    state.bridge.scene_bridge_retired = Some((previous, false));
                }
                state.bridge.scene_bridge_digest = Some(cursor.digest);
                if cursor.camera_changed {
                    state.camera.scene_camera_digest = Some(cursor.camera_digest);
                }
                cursor.phase=World3dSceneBridgePhase::Close;state.bridge.scene_bridge=Some(cursor);
            World3dSceneBridgeStep::Complete
            }
            Err(fault) => {
                state.session.snapshot_fault = Some(fault);
                cursor.phase=World3dSceneBridgePhase::Close;state.bridge.scene_bridge=Some(cursor);
                World3dSceneBridgeStep::Fault
            }
        },
    }
}

pub fn sync_world3d_state(state: &mut World3dState, scene: &UiComponentSceneNode, bounds: Rect) {
    state.session.bounds = bounds;
    let Some(world) = scene.world_3d.as_ref() else {
        state.session.snapshot_fault = Some(World3dSnapshotFault::Unavailable);
        return;
    };
    state.session.bound_domain_id = world.domain_id.clone();
    state.session.bound_domain_granularity_id = world.domain_granularity_id.clone();
    state.session.presentation = world3d_presentation(world.presentation_json.as_deref());
    state.scene.environment = world.environment_json.as_deref().and_then(|json| serde_json::from_str(json).ok()).unwrap_or_default();
    sync_world3d_tool_run_trace(&mut state.trace, world);
    sync_world3d_scene_fit(&mut state.camera, world.fit_json.as_deref());
    sync_world3d_projection_content_frame(&mut state.session, &mut state.camera, &mut state.selection, &mut state.scene, &mut state.bridge);
    sync_world3d_scene_selection(&mut state.session, &mut state.selection, &mut state.gumball, &mut state.scene, &world.selection_json);
    apply_world3d_pick_filter(&mut state.session, &mut state.selection, &mut state.scene, world.modelling_options.as_ref().and_then(|options| options.pick_filter));
    sync_world3d_scene_document_lanes(&mut state.session, &mut state.selection, &mut state.scene, &mut state.bridge, &mut state.assets, world);
    sync_world3d_brush_mesh_feedback(&mut state.assets, world.interaction_json.as_deref());
    let lease = match world.snapshot {
        Some(lease) => lease,
        None => {
            stage_world3d_scene_bridge(&mut state.camera, &mut state.bridge, world);
            let Some(lease) = state.bridge.scene_bridge_lease else {
                if state.bridge.scene_bridge.is_none() {
                    state.session.snapshot_fault = Some(World3dSnapshotFault::Unavailable);
                }
                return;
            };
            lease
        }
    };
    if state.bridge.snapshot_lease == Some(lease) || state.bridge.snapshot_apply.as_ref().is_some_and(|cursor| cursor.lease == lease) {
        return;
    }
    if state.bridge.snapshot_apply.is_some() {
        state.session.snapshot_fault = Some(World3dSnapshotFault::Closing);
        return;
    }
    state.bridge.snapshot_apply = Some(World3dSnapshotApplyCursor::new(lease));
    state.session.snapshot_fault = None;
}

fn world3d_presentation(json: Option<&str>) -> World3dPresentation {
    let Some(json) = json else { return World3dPresentation::default() };
    let Ok(value) = serde_json::from_str::<World3dPresentation>(json) else { return World3dPresentation::default() };
    if value.source_aspect.is_some_and(|aspect| !aspect.is_finite() || aspect <= 0.0) {
        return World3dPresentation::default();
    }
    value
}

/// ⏯️ Feeds `World3dScene.tool_run_trace` into the resident trace layer and re-derives the mesh index →
/// mesh key table the trace subjects address (`meshes_json` order) only when that lane changed.
fn sync_world3d_tool_run_trace(trace_state: &mut WorldTraceState, world: &ui_wgpu::wgpu::World3dScene) {
    let _ = trace_state.tool_run_trace.apply_lane(world.tool_run_trace.as_deref());
    if trace_state.tool_run_trace.is_empty() {
        return;
    }
    let digest = world3d_scene_digest(&[&world.meshes_json]);
    if trace_state.tool_run_trace_meshes_digest == Some(digest) {
        return;
    }
    #[derive(Deserialize)]
    struct MeshKey {
        id: String,
    }
    trace_state.tool_run_trace_mesh_keys = serde_json::from_str::<Vec<MeshKey>>(&world.meshes_json).map(|meshes| meshes.into_iter().map(|mesh| mesh.id).collect()).unwrap_or_default();
    trace_state.tool_run_trace_meshes_digest = Some(digest);
}

/// 🧭️ The cursor this surface echoes as `toolRunTraceCursorByWindowId[window]` in its window instance view state.
pub fn world3d_tool_run_trace_cursor(trace_state: &WorldTraceState) -> Option<semio_framework_tool_run::ToolRunTraceCursor> {
    trace_state.tool_run_trace.cursor()
}

/// 🧭️ Every `(window instance, cursor)` echo the given world surfaces owe their view state — surfaces without
/// a bound window or without an applied trace lane echo nothing.
pub fn world3d_tool_run_trace_cursors<'a>(states: impl IntoIterator<Item = &'a World3dState>) -> HashMap<String, semio_framework_tool_run::ToolRunTraceCursor> {
    states.into_iter().filter_map(|state| Some((state.trace.tool_run_trace_window_id.clone()?, state.trace.tool_run_trace.cursor()?))).collect()
}

/// 🎨️ Appends the trace layer's instanced `(mesh, verdict)` draws; a mesh index with no published mesh
/// falls back to the placeholder box, exactly like the brush ghost.
#[cfg(not(all(target_arch = "wasm32", target_env = "p2")))]
pub(super) fn append_tool_run_trace_draws(session_state: &mut WorldSessionState, authority_state: &mut WorldAuthorityState, retirement_state: &mut WorldRetirementState, scene_state: &mut WorldSceneState, trace_state: &mut WorldTraceState, gpu: &mut World3dBuildContext, theme: &ui_wgpu::wgpu::Theme, translucent_draws: &mut Vec<SceneDraw3d>) {
    if trace_state.tool_run_trace.is_empty() {
        return;
    }
    let palette = tool_run_trace::ToolRunTracePalette::from_theme(theme);
    for draw in trace_state.tool_run_trace.draws(&palette, trace_state.tool_run_trace_visibility) {
        let mesh_key = trace_state.tool_run_trace_mesh_keys.get(draw.mesh as usize).filter(|key| scene_state.meshes.contains_key(key.as_str())).cloned().unwrap_or_else(|| brush_preview_mesh_id(None));
        if !scene_state.meshes.contains_key(&mesh_key) {
            begin_world_placeholder_mesh(session_state, authority_state, retirement_state, scene_state, &mesh_key, WorldPlaceholderKind::Box);
        }
        let mesh_version = live_world3d_mesh_version(scene_state, &mesh_key);
        if let Some(mesh) = scene_state.meshes.get(&mesh_key).copied() {
            gpu.ensure_mesh(&mesh_key, mesh_version, mesh);
            translucent_draws.push(SceneDraw3d { mesh_key, mesh_version, instances: draw.instances, shadow_role: Default::default() });
        }
    }
}

/// 🎯️ A published id may name the rendered INSTANCE or the TOPOLOGY target it stands for, and both
/// reach this state: the guest's `selectionJson.ids` carries render ids (pinned by
/// `🧫️fixtures/🌉️scene-bridge/🔣️.json`'s `selectedIds`), while this surface's OWN optimistic
/// `local_hover_id` is the topology id it just dispatched in `interactionHover`. Matching only one
/// of the two leaves half the highlights dark. React resolves the same pair
/// (`useWorldInstanceChrome(instance.id)` over a selection the host maps back through
/// `interactionId`). See [`instance_interaction_id`].
///
/// 🎨️ Selection/hover flags must follow the live selection snapshot — OR-ing with the
/// instancesJson bits left deselected meshes painted selected until a later hover rebuild.
pub fn apply_runtime_draw_flags(selection_state: &mut WorldSelectionState, interaction_state: &mut WorldInteractionState, scene_state: &mut WorldSceneState) {
    let granularity = selection_state.granularity.clone();
    let component_ids: HashSet<String> = selection_state.component_ids.iter().cloned().collect();
    let local_hover_id = selection_state.local_hover_id.clone();
    let hovered_component_object_id = selection_state.hovered_component_object_id.clone();
    let selected_ids: HashSet<String> = selection_state.selected_ids.iter().cloned().collect();
    let interaction_ids = interaction_state.instance_interaction_ids.clone();
    let mut object_index_map = HashMap::new();
    let mut index = 0u32;
    for draw in &scene_state.draws {
        for instance in &draw.instances {
            object_index_map.insert(instance.id.clone(), index);
            index += 1;
        }
    }
    let component_mode = component_mode_active(selection_state);
    for draw in &mut scene_state.draws {
        for instance in &mut draw.instances {
            let mesh_selected = granularity == "mesh" && object_index_map.get(&instance.id).is_some_and(|object_index| component_ids.contains(&object_index.to_string()));
            let target_id = interaction_ids.get(&instance.id).map_or(instance.id.as_str(), String::as_str);
            let local_hovered = if component_mode { false } else { local_hover_id.as_deref() == Some(instance.id.as_str()) || local_hover_id.as_deref() == Some(target_id) || hovered_component_object_id.as_deref() == Some(instance.id.as_str()) };
            let local_selected = selected_ids.contains(&instance.id) || selected_ids.contains(target_id) || mesh_selected;
            instance.hovered = local_hovered;
            instance.selected = local_selected;
        }
    }
}

/// ✂️ Closes the cut of a capped section plane over the clipped solids, the native twin of the React `World3dSectionClip` cap: the clipped solids are drawn once with `StencilToggle`
/// (each fragment inverts the stencil's section bit, so after both faces of a cut solid the bit is set exactly where the cut exposes its inside), then the flat cap plane in the cap tone
/// is drawn only where the bit is set, then the solids are drawn a second time to invert the bit back to zero. The three passes ride the translucent draw list in that order,
/// after the opaque solids, so the cap is depth-tested against the retained surface.
#[cfg(not(all(target_arch = "wasm32", target_env = "p2")))]
pub(super) fn append_world3d_section_cap(session_state: &mut WorldSessionState, authority_state: &mut WorldAuthorityState, retirement_state: &mut WorldRetirementState, scene_state: &mut WorldSceneState, gpu: &mut World3dBuildContext, theme: &ui_wgpu::wgpu::Theme, section: &ui_wgpu::wgpu::World3dSection, solids: &[SceneDraw3d], translucent: &mut Vec<SceneDraw3d>) {
    let Some(cap) = section.cap else { return };
    if solids.iter().all(|draw| draw.instances.is_empty()) {
        return;
    }
    if !scene_state.meshes.contains_key(GUMBALL_PLANE_MESH) {
        begin_world_placeholder_mesh(session_state, authority_state, retirement_state, scene_state, GUMBALL_PLANE_MESH, WorldPlaceholderKind::Plane);
    }
    let Some(plane) = scene_state.meshes.get(GUMBALL_PLANE_MESH).copied() else { return };
    let plane_version = live_world3d_mesh_version(scene_state, GUMBALL_PLANE_MESH);
    gpu.ensure_mesh(GUMBALL_PLANE_MESH, plane_version, plane);
    let (mut low, mut high) = ([f32::INFINITY; 3], [f32::NEG_INFINITY; 3]);
    for draw in solids {
        let Some(Ok((mesh_low, mesh_high))) = scene_state.meshes.get(&draw.mesh_key).map(|mesh| mesh.aabb()) else { continue };
        for instance in &draw.instances {
            let (instance_low, instance_high) = transform_aabb(instance.model, mesh_low, mesh_high);
            for axis in 0..3 {
                low[axis] = low[axis].min(instance_low[axis]);
                high[axis] = high[axis].max(instance_high[axis]);
            }
        }
    }
    if !(0..3).all(|axis| low[axis].is_finite() && high[axis].is_finite()) {
        return;
    }
    let diagonal = Vec3::new(high[0] - low[0], high[1] - low[1], high[2] - low[2]).length();
    let side = (4.0 * diagonal).max(2.0);
    let normal = section.unit_normal();
    let normal = Vec3::new(normal[0] as f32, normal[1] as f32, normal[2] as f32);
    let helper = if normal.y.abs() < 0.9 { Vec3::new(0.0, 1.0, 0.0) } else { Vec3::new(1.0, 0.0, 0.0) };
    let tangent = helper.cross(normal).normalize();
    let rotation = quat_from_basis(tangent, normal, tangent.cross(normal));
    let origin = [section.origin[0] as f32, section.origin[1] as f32, section.origin[2] as f32];
    let toggled = || -> Vec<SceneDraw3d> {
        solids
            .iter()
            .filter(|draw| !draw.instances.is_empty())
            .map(|draw| SceneDraw3d {
                mesh_key: draw.mesh_key.clone(),
                mesh_version: draw.mesh_version,
                instances: draw.instances.iter().map(|instance| Instance3d { material: SceneInstanceMaterial3d { section: SceneSectionRole3d::StencilToggle, ..instance.material }, ..instance.clone() }).collect(),
                shadow_role: Default::default(),
            })
            .collect()
    };
    translucent.extend(toggled());
    translucent.push(SceneDraw3d {
        mesh_key: GUMBALL_PLANE_MESH.into(),
        mesh_version: plane_version,
        instances: vec![Instance3d {
            component_source: None,
            id: "section-cap".into(),
            model: Instance3d::model_from_trs(origin, rotation, [side, 1.0, side]),
            color: world3d_tone_rgba(theme, cap.tone),
            selected: false,
            hovered: false,
            material: SceneInstanceMaterial3d { section: SceneSectionRole3d::Cap, ..Default::default() },
        }],
        shadow_role: Default::default(),
    });
    translucent.extend(toggled());
}

//#region 📏️Modelling overlay
const WORLD_OVERLAY_ARC_SEGMENT_PX: f64 = 6.0;

const WORLD_OVERLAY_LEGEND_MARGIN_PX: f32 = 8.0;

const WORLD_OVERLAY_LEGEND_PADDING_PX: f32 = 6.0;

const WORLD_OVERLAY_LEGEND_BAR_WIDTH_PX: f32 = 12.0;

const WORLD_OVERLAY_LEGEND_BAR_HEIGHT_PX: f32 = 96.0;

const WORLD_OVERLAY_LEGEND_BAR_SLICES: usize = 32;

const WORLD_OVERLAY_LEGEND_GAP_PX: f32 = 6.0;

const WORLD_OVERLAY_LABEL_PADDING_X_PX: f32 = 4.0;

const WORLD_OVERLAY_LABEL_PADDING_Y_PX: f32 = 1.0;

/// 📐️ The three corners of a dimension arrowhead whose tip sits at `(x, y)` and points along `angle` — React's `arrowPoints`.
fn world3d_arrow_points(x: f64, y: f64, angle: f64) -> [[f32; 2]; 3] {
    let length = ui_wgpu::wgpu::WORLD3D_ANNOTATION_ARROW_PX;
    let (cos, sin) = (angle.cos(), angle.sin());
    let (base_x, base_y) = (x - cos * length, y - sin * length);
    let half = length * 0.35;
    [[x as f32, y as f32], [(base_x - sin * half) as f32, (base_y + cos * half) as f32], [(base_x + sin * half) as f32, (base_y - cos * half) as f32]]
}

/// 🌙️ The polyline of an angle arc, one vertex per `WORLD_OVERLAY_ARC_SEGMENT_PX` of arc length (at least four, at most thirty-two segments).
fn world3d_arc_points(arc: &ui_wgpu::wgpu::World3dProjectedArc) -> Vec<[f32; 2]> {
    let segments = ((arc.sweep.abs() * arc.radius / WORLD_OVERLAY_ARC_SEGMENT_PX).ceil() as usize).clamp(4, 32);
    (0..=segments)
        .map(|index| {
            let angle = arc.start + arc.sweep * index as f64 / segments as f64;
            [(arc.cx + angle.cos() * arc.radius) as f32, (arc.cy + angle.sin() * arc.radius) as f32]
        })
        .collect()
}

fn world3d_circle_points(x: f64, y: f64, radius: f64) -> Vec<[f32; 2]> {
    (0..=16).map(|index| index as f64 / 16.0 * std::f64::consts::TAU).map(|angle| [(x + angle.cos() * radius) as f32, (y + angle.sin() * radius) as f32]).collect()
}

fn world3d_overlay_color(theme: &ui_wgpu::wgpu::Theme, tone: ui_wgpu::wgpu::World3dTone) -> Rgba {
    let color = world3d_tone_rgba(theme, tone);
    Rgba::new(color[0], color[1], color[2], color[3])
}

/// 📏️ Paints the projected annotation layer on top of the world pass: lines, arrowheads, arcs and marks at screen-constant size in the tone colours of the live theme,
/// and the caller's text in the active language on a translucent plate. Clipped to the viewport by the caller's scissor.
#[cfg(not(all(target_arch = "wasm32", target_env = "p2")))]
pub(super) fn paint_world3d_annotations(ctx: &mut ui_wgpu::wgpu::widgets::WidgetContext<'_, ActionDescriptor>, layer: &ui_wgpu::wgpu::World3dAnnotationLayer, view_proj: &[f32; 16], inner: Rect, locale: &str) {
    use ui_wgpu::wgpu::{World3dLabelAlign, World3dMarkerShape};
    let theme = ctx.theme;
    let stroke = ui_wgpu::wgpu::WORLD3D_ANNOTATION_STROKE_PX as f32;
    let marker_radius = ui_wgpu::wgpu::WORLD3D_ANNOTATION_MARKER_PX / 2.0;
    let size = theme.font_size_small;
    let (origin_x, origin_y) = (f64::from(inner.x), f64::from(inner.y));
    let at = |points: Vec<[f32; 2]>| -> Vec<[f32; 2]> { points.into_iter().map(|point| [point[0] + inner.x, point[1] + inner.y]).collect() };
    let projected = ui_wgpu::wgpu::project_world3d_annotations(layer, view_proj, inner.w, inner.h);
    for (item, entry) in layer.items.iter().zip(&projected).filter(|(_, entry)| entry.visible) {
        let color = world3d_overlay_color(theme, entry.tone);
        for line in &entry.lines {
            ctx.draw.push_line_overlay(inner.x + line[0] as f32, inner.y + line[1] as f32, inner.x + line[2] as f32, inner.y + line[3] as f32, color, stroke);
        }
        for arrow in &entry.arrows {
            ctx.draw.push_triangle_fan_overlay(&at(world3d_arrow_points(arrow.x, arrow.y, arrow.angle).to_vec()), color);
        }
        if let Some(arc) = entry.arc.as_ref() {
            for pair in at(world3d_arc_points(arc)).windows(2) {
                ctx.draw.push_line_overlay(pair[0][0], pair[0][1], pair[1][0], pair[1][1], color, stroke);
            }
        }
        if let Some(marker) = entry.marker {
            let (x, y) = (marker.x + origin_x, marker.y + origin_y);
            match marker.shape {
                World3dMarkerShape::Dot => ctx.draw.push_rounded_overlay([(x - marker_radius / 2.0) as f32, (y - marker_radius / 2.0) as f32, marker_radius as f32, marker_radius as f32], color, marker_radius as f32 / 2.0),
                World3dMarkerShape::Cross => {
                    for (dx, dy) in [(-1.0, -1.0), (-1.0, 1.0)] {
                        ctx.draw.push_line_overlay((x + dx * marker_radius) as f32, (y + dy * marker_radius) as f32, (x - dx * marker_radius) as f32, (y - dy * marker_radius) as f32, color, stroke);
                    }
                }
                World3dMarkerShape::Ring => {
                    for pair in world3d_circle_points(x, y, marker_radius).windows(2) {
                        ctx.draw.push_line_overlay(pair[0][0], pair[0][1], pair[1][0], pair[1][1], color, stroke);
                    }
                }
            }
        }
        if let Some(label) = entry.label {
            let text = item.text().resolve(locale);
            let (width, height) = ctx.atlas.measure_text(text, size);
            let left = match label.align {
                World3dLabelAlign::Middle => label.x - f64::from(width) / 2.0,
                World3dLabelAlign::Start => label.x,
                World3dLabelAlign::End => label.x - f64::from(width),
            } as f32
                + inner.x;
            let top = label.y as f32 + inner.y - height / 2.0;
            let plate = Rgba::new(theme.background.r, theme.background.g, theme.background.b, 0.8);
            ctx.draw.push_rounded_overlay([left - WORLD_OVERLAY_LABEL_PADDING_X_PX, top - WORLD_OVERLAY_LABEL_PADDING_Y_PX, width + 2.0 * WORLD_OVERLAY_LABEL_PADDING_X_PX, height + 2.0 * WORLD_OVERLAY_LABEL_PADDING_Y_PX], plate, theme.border_radius.min(4.0));
            ui_wgpu::wgpu::widgets::draw_text_overlay(ctx, text, left, label.y as f32 + inner.y + size * 0.35, size, color);
        }
    }
}

/// 🌡️ The panel rectangle of a scalar-field legend (or its mismatch message) pinned to the viewport's lower left, `content` pixels wide and tall.
fn world3d_legend_rect(inner: Rect, content: (f32, f32)) -> [f32; 4] {
    let (width, height) = (content.0 + 2.0 * WORLD_OVERLAY_LEGEND_PADDING_PX, content.1 + 2.0 * WORLD_OVERLAY_LEGEND_PADDING_PX);
    [inner.x + WORLD_OVERLAY_LEGEND_MARGIN_PX, inner.y + inner.h - WORLD_OVERLAY_LEGEND_MARGIN_PX - height, width, height]
}

/// 🌡️ Paints the scalar-field legend: the title with its unit, the ramp as a vertical gradient (range minimum at the bottom), the evenly spaced tick values beside it and the no-data caption.
#[cfg(not(all(target_arch = "wasm32", target_env = "p2")))]
pub(super) fn paint_world3d_legend(ctx: &mut ui_wgpu::wgpu::widgets::WidgetContext<'_, ActionDescriptor>, field: &ui_wgpu::wgpu::World3dScalarField, inner: Rect, locale: &str) {
    let theme = ctx.theme;
    let size = theme.font_size_small;
    let lines = field.legend_lines(locale);
    let (title_width, title_height) = ctx.atlas.measure_text(&lines.title, size);
    let tick_width = lines.ticks.iter().map(|tick| ctx.atlas.measure_text(&tick.text, size).0).fold(0.0_f32, f32::max);
    let no_data = lines.no_data.map(|text| (text, ctx.atlas.measure_text(text, size)));
    let ramp_width = WORLD_OVERLAY_LEGEND_BAR_WIDTH_PX + WORLD_OVERLAY_LEGEND_GAP_PX + tick_width;
    let no_data_width = no_data.map_or(0.0, |(_, (width, _))| 10.0 + WORLD_OVERLAY_LEGEND_GAP_PX + width);
    let no_data_height = no_data.map_or(0.0, |(_, (_, height))| WORLD_OVERLAY_LEGEND_GAP_PX + height.max(10.0));
    let content = (title_width.max(ramp_width).max(no_data_width), title_height + WORLD_OVERLAY_LEGEND_GAP_PX + WORLD_OVERLAY_LEGEND_BAR_HEIGHT_PX + no_data_height);
    let rect = world3d_legend_rect(inner, content);
    ctx.draw.push_rounded_overlay(rect, Rgba::new(theme.panel.r, theme.panel.g, theme.panel.b, 0.9), theme.border_radius.min(4.0));
    let (left, top) = (rect[0] + WORLD_OVERLAY_LEGEND_PADDING_PX, rect[1] + WORLD_OVERLAY_LEGEND_PADDING_PX);
    ui_wgpu::wgpu::widgets::draw_text_overlay(ctx, &lines.title, left, top + size, size, theme.text);
    let bar_top = top + title_height + WORLD_OVERLAY_LEGEND_GAP_PX;
    let slice = WORLD_OVERLAY_LEGEND_BAR_HEIGHT_PX / WORLD_OVERLAY_LEGEND_BAR_SLICES as f32;
    for index in 0..WORLD_OVERLAY_LEGEND_BAR_SLICES {
        let [r, g, b] = field.ramp.sample(1.0 - (index as f64 + 0.5) / WORLD_OVERLAY_LEGEND_BAR_SLICES as f64);
        ctx.draw.push_solid_overlay([left, bar_top + index as f32 * slice, WORLD_OVERLAY_LEGEND_BAR_WIDTH_PX, slice + 0.5], Rgba::from_srgb8(r, g, b, 255));
    }
    let steps = lines.ticks.len().saturating_sub(1).max(1) as f32;
    for (index, tick) in lines.ticks.iter().enumerate() {
        let y = bar_top + WORLD_OVERLAY_LEGEND_BAR_HEIGHT_PX - index as f32 / steps * WORLD_OVERLAY_LEGEND_BAR_HEIGHT_PX;
        ui_wgpu::wgpu::widgets::draw_text_overlay(ctx, &tick.text, left + WORLD_OVERLAY_LEGEND_BAR_WIDTH_PX + WORLD_OVERLAY_LEGEND_GAP_PX, y + size * 0.35, size, theme.text);
    }
    if let Some((text, (_, height))) = no_data {
        let y = bar_top + WORLD_OVERLAY_LEGEND_BAR_HEIGHT_PX + WORLD_OVERLAY_LEGEND_GAP_PX;
        let [r, g, b] = ui_wgpu::wgpu::WORLD3D_SCALAR_NO_DATA_RGB;
        ctx.draw.push_solid_overlay([left, y, 10.0, 10.0], Rgba::from_srgb8(r, g, b, 255));
        ui_wgpu::wgpu::widgets::draw_text_overlay(ctx, text, left + 10.0 + WORLD_OVERLAY_LEGEND_GAP_PX, y + height.max(10.0) * 0.5 + size * 0.35, size, theme.text);
    }
}

/// 🌡️ Announces on the viewport that the scalar field does not fit the mesh it names, in place of a wrong heatmap.
#[cfg(not(all(target_arch = "wasm32", target_env = "p2")))]
pub(super) fn paint_world3d_scalar_mismatch(ctx: &mut ui_wgpu::wgpu::widgets::WidgetContext<'_, ActionDescriptor>, inner: Rect, locale: &str) {
    let theme = ctx.theme;
    let size = theme.font_size_small;
    let text = ui_wgpu::wgpu::world3d_modelling_strings(locale).mismatch;
    let (width, height) = ctx.atlas.measure_text(text, size);
    let rect = world3d_legend_rect(inner, (width, height));
    ctx.draw.push_rounded_overlay(rect, Rgba::new(theme.panel.r, theme.panel.g, theme.panel.b, 0.9), theme.border_radius.min(4.0));
    ui_wgpu::wgpu::widgets::draw_text_overlay(ctx, text, rect[0] + WORLD_OVERLAY_LEGEND_PADDING_PX, rect[1] + WORLD_OVERLAY_LEGEND_PADDING_PX + size, size, theme.error);
}

/// ♿️ One node of the accessibility tree a world surface publishes for its modelling overlays: a stable key, an ARIA role, the text a screen reader speaks and its nesting below the surface.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct World3dAccessibilityEntry {
    pub key: String,
    pub role: &'static str,
    pub label: String,
    pub depth: u8,
    pub polite: bool,
}

/// ♿️ The text alternative of everything `render_world_3d` paints for the modelling layer, in the surface's language and with the same structure as the React host: a `list` of annotations
/// named by the layer title, one `listitem` per annotation (`Dimension: Width 40 mm`), a `group` for an applied scalar-field legend with its scale ticks and no-data caption, or a polite `status`
/// when the field does not fit its mesh. The shell hands these to the accessibility mirror (the screen reader's `aria-live` region), so nothing drawn here is pointer-only.
pub fn world3d_modelling_accessibility(session_state: &WorldSessionState, overlay_state: &WorldOverlayState, scene: &UiComponentSceneNode) -> Vec<World3dAccessibilityEntry> {
    let Some(world) = scene.world_3d.as_ref() else { return Vec::new() };
    let locale = session_state.modelling_locale;
    let strings = ui_wgpu::wgpu::world3d_modelling_strings(locale);
    let host = scene.host_id.as_str();
    let entry = |key: String, role: &'static str, label: String, depth: u8, polite: bool| World3dAccessibilityEntry { key, role, label, depth, polite };
    let mut entries = Vec::new();
    if let Some(layer) = world.annotations.as_ref().filter(|layer| !layer.items.is_empty()) {
        entries.push(entry(format!("{host}::annotations"), "list", layer.title.as_ref().map_or(strings.annotations, |title| title.resolve(locale)).to_string(), 0, false));
        entries.extend(layer.items.iter().map(|item| entry(format!("{host}::annotation::{}", item.id()), "listitem", item.accessible_name(locale), 1, false)));
    }
    if let Some(field) = world.scalar_field.as_ref() {
        match overlay_state.scalar_field_status {
            World3dScalarFieldStatus::Applied => {
                let lines = field.legend_lines(locale);
                entries.push(entry(format!("{host}::legend"), "group", format!("{}: {}", strings.legend, field.legend.title.resolve(locale)), 0, false));
                entries.push(entry(format!("{host}::legend::scale"), "list", strings.scale.to_string(), 1, false));
                entries.extend(lines.ticks.iter().enumerate().map(|(index, tick)| entry(format!("{host}::legend::tick::{index}"), "listitem", tick.text.clone(), 2, false)));
                if let Some(caption) = lines.no_data {
                    entries.push(entry(format!("{host}::legend::no-data"), "paragraph", caption.to_string(), 1, false));
                }
            }
            World3dScalarFieldStatus::Mismatch => entries.push(entry(format!("{host}::scalar-field-mismatch"), "status", strings.mismatch.to_string(), 0, true)),
            World3dScalarFieldStatus::None | World3dScalarFieldStatus::Missing => {}
        }
    }
    entries
}
//#endregion 📏️Modelling overlay
