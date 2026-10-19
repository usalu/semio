//! 🎥️ World3d projection, native mesh admission, snapshot apply, navigation classification, dynamic retirement and draw rebuild.

use super::*;

//#region 🧭️WorldCameraSync
/// 🕒️ `pub` (was crate-private) so the wgpu renderer's wheel-zoom settle-then-dispatch sweep
/// (`AppRuntime::frame`'s `pending_camera_dispatch_deadlines_ms`) can build the same `setCamera`
/// action the pointer-release path below already dispatches — one orbit-to-action mapping, two
/// trigger sites (immediate on release, debounced on wheel settle).
/// 🔎️ The `zoom` scalar of a camera's `setCamera` pose — React's `captureNavigationSnapshot`:
/// `camera instanceof ThreeOrthographicCamera ? camera.zoom : 1`. A perspective pane reports the
/// identity whatever its camera carries; a parallel one reports the live frustum scale, which is the
/// only number its wheel ever moves ([`OrbitController::zoom`]).
pub fn world3d_camera_zoom(camera: &Camera3d) -> f64 {
    if camera.projection.is_parallel() {
        f64::from(camera.zoom)
    } else {
        1.0
    }
}

/// 📦️ Advances camera/view ownership without invalidating a sealed content publication. Snapshot
/// apply exposes its lease one transaction before the retained draw swap; camera reports, initial
/// templates, and projection framing may all legitimately land in that interval.
pub fn advance_world3d_view_revision(session_state: &mut WorldSessionState, scene_state: &mut WorldSceneState) {
    session_state.interaction_revision = session_state.interaction_revision.wrapping_add(1);
    if let Some(rebuild) = scene_state.draw_rebuild.as_mut() {
        rebuild.descriptor.revision = session_state.interaction_revision;
    }
}

/// 🔀️ Applies one locally selected projection while retaining the accepted viewport-owned pose.
/// React's `WorldProjectionSnapDriver` changes family/FOV and rotates only when orientation changes;
/// its content framer stays unmounted after initial ownership. External/initial seeds arm framing in
/// the scene bridge instead.
pub fn apply_world3d_projection_spec(session_state: &mut WorldSessionState, camera_state: &mut WorldCameraState, scene_state: &mut WorldSceneState, spec: Viewport3dProjectionSpec) -> bool {
    if camera_state.projection_selected && camera_state.projection_spec == spec {
        return false;
    }
    let orientation_changed = camera_state.projection_spec.orientation != spec.orientation;
    let projection = ui_scene::math::projection_spec_family(spec);
    if camera_state.orbit.projection != projection {
        if projection.is_parallel() {
            camera_state.orbit.zoom = ui_scene::math::world_projection_matched_ortho_zoom(camera_state.orbit.fov_y.to_degrees(), camera_state.orbit.distance, session_state.bounds.h);
        }
    }
    camera_state.orbit.projection = projection;
    camera_state.orbit.fov_y = ui_scene::math::projection_spec_fov_degrees(spec).to_radians();
    if orientation_changed {
        let (direction, up) = ui_scene::math::projection_spec_orientation_look(spec);
        let direction = direction.normalize();
        camera_state.orbit.yaw = direction.y.atan2(direction.x);
        camera_state.orbit.pitch = direction.z.clamp(-1.0, 1.0).asin();
        camera_state.orbit.up = up;
    }
    camera_state.projection_spec = spec;
    camera_state.projection_selected = true;
    camera_state.projection_frame_policy = Viewport3dProjectionFramePolicy::Content;
    camera_state.projection_frame_owed = false;
    advance_world3d_view_revision(session_state, scene_state);
    true
}

/// 🧭️ Changes a projection mode while preserving the live gizmo orientation.
pub fn apply_world3d_projection_mode(session_state: &mut WorldSessionState, camera_state: &mut WorldCameraState, scene_state: &mut WorldSceneState, mode: Viewport3dProjectionMode) -> bool {
    apply_world3d_projection_spec(session_state, camera_state, scene_state, Viewport3dProjectionSpec { mode, orientation: camera_state.projection_spec.orientation })
}

/// 🧭️ The live orientation retained across projection-mode changes.
pub fn world3d_projection_orientation(camera_state: &WorldCameraState) -> ui_scene::math::WorldProjectionOrientation {
    match camera_state.projection_spec.orientation {
        Viewport3dProjectionOrientation::Cardinal { view } => ui_scene::math::WorldProjectionOrientation::Cardinal(match view {
            semio_framework_ui_viewport::Viewport3dOrthographicView::Plan | semio_framework_ui_viewport::Viewport3dOrthographicView::Top => ui_scene::math::WorldCardinalView::Top,
            semio_framework_ui_viewport::Viewport3dOrthographicView::Bottom => ui_scene::math::WorldCardinalView::Bottom,
            semio_framework_ui_viewport::Viewport3dOrthographicView::Front => ui_scene::math::WorldCardinalView::Front,
            semio_framework_ui_viewport::Viewport3dOrthographicView::Back => ui_scene::math::WorldCardinalView::Back,
            semio_framework_ui_viewport::Viewport3dOrthographicView::Left => ui_scene::math::WorldCardinalView::Left,
            semio_framework_ui_viewport::Viewport3dOrthographicView::Right => ui_scene::math::WorldCardinalView::Right,
        }),
        _ => ui_scene::math::WorldProjectionOrientation::Free,
    }
}

/// 📐️ The complete accepted projection retained across snapshot delivery and remounts.
pub fn world3d_projection_spec(camera_state: &WorldCameraState) -> Viewport3dProjectionSpec { camera_state.projection_spec }

/// 📐️ Applies a display-template seed once, after the delivered camera has landed. React's pending
/// window template seeds `WorldProjectionRig` before `WorldAutoFit`; keeping the live distance and
/// target here is equivalent for the fit because it consumes only this look direction, then replaces
/// both with the measured rendered bounds.
///
/// 📦️ The snapshot lease becomes resident one transaction before its sealed draw rebuild publishes.
/// Shell applies this camera-only template in that interval, so the live rebuild carries the new
/// view-state revision instead of losing the document and its GLB request ledger as stale.
pub fn apply_world3d_initial_projection_seed(
    session_state: &mut WorldSessionState, camera_state: &mut WorldCameraState, scene_state: &mut WorldSceneState, bridge_state: &mut WorldBridgeState,
    spec: Viewport3dProjectionSpec,
) -> bool {
    if camera_state.projection_selected || bridge_state.snapshot_lease.is_none() {
        return false;
    }
    let (direction, up) = ui_scene::math::projection_spec_orientation_look(spec);
    let length = direction.length();
    if !length.is_finite() || length <= 1e-6 || up.to_array().into_iter().any(|value| !value.is_finite()) {
        return false;
    }
    let direction = direction.scale(1.0 / length);
    camera_state.orbit.yaw = direction.y.atan2(direction.x);
    camera_state.orbit.pitch = direction.z.clamp(-1.0, 1.0).asin();
    camera_state.orbit.up = up;
    let projection = ui_scene::math::projection_spec_family(spec);
    camera_state.orbit.projection = projection;
    camera_state.orbit.fov_y = ui_scene::math::projection_spec_fov_degrees(spec).to_radians();
    camera_state.projection_spec = spec;
    camera_state.projection_selected = true;
    camera_state.projection_frame_policy = Viewport3dProjectionFramePolicy::Content;
    camera_state.projection_frame_owed = true;
    camera_state.projection_frame_key = None;
    camera_state.projection_frame_zoom = None;
    advance_world3d_view_revision(session_state, scene_state);
    true
}

/// 🔎️ This surface's live camera family, for the pane chip that reports it.
pub fn world3d_camera_projection(camera_state: &WorldCameraState) -> CameraProjection3d {
    ui_scene::math::projection_spec_family(camera_state.projection_spec)
}

/// 🎥️ This surface's LIVE orbit in the very shape `setCamera` carries — the row `dumpMeshStats`
/// reports beside the wire camera, so a probe can diff "what the guest published" against "what the
/// pane is looking through". Carries the two members the wire shape has no room for and a local
/// camera move turns on: the projection FAMILY and the framing ORIENTATION.
pub fn world3d_live_camera_json(session_state: &WorldSessionState, camera_state: &WorldCameraState, selection_state: &WorldSelectionState, scene_state: &WorldSceneState) -> serde_json::Value {
    let camera = camera_state.orbit.to_camera();
    let references = scene_state.references.iter().filter(|reference| !reference.hidden.unwrap_or(false)).map(|reference| {
        let url = reference.url.as_deref().unwrap_or_default();
        let [width, height] = world_reference_plane_size(scene_state, url, world_reference_width(reference));
        json!({
            "id": reference.id.as_deref(),
            "url": url,
            "origin": reference.origin.unwrap_or([0.0; 3]),
            "planeSize": [width, height],
        })
    }).collect::<Vec<_>>();
    let content_bounds = world3d_content_bounds(selection_state, scene_state).map(|(minimum, maximum)| json!({ "minimum": minimum, "maximum": maximum }));
    let lod = scene_lod(session_state, camera_state, scene_state);
    let grid_step = lod_grid_step_world(lod, scene_state.lod.grid_factor);
    json!({
        "position": [camera.position.x as f64, camera.position.y as f64, camera.position.z as f64],
        "target": [camera.target.x as f64, camera.target.y as f64, camera.target.z as f64],
        "up": [f64::from(camera.up.x), f64::from(camera.up.y), f64::from(camera.up.z)],
        "zoom": world3d_camera_zoom(&camera),
        "fov": f64::from(camera.fov_y.to_degrees()),
        "projection": camera_state.projection_spec,
        "projectionFrame": camera_state.projection_frame_policy,
        "viewport": [session_state.bounds.x, session_state.bounds.y, session_state.bounds.w, session_state.bounds.h],
        "contentBounds": content_bounds,
        "projectionFrameOwed": camera_state.projection_frame_owed,
        "references": references,
        "grid": { "lod": lod, "factor": scene_state.lod.grid_factor, "step": grid_step },
        "userMoved": camera_state.camera_user_moved,
    })
}

pub fn orbit_camera_action(session_state: &WorldSessionState, camera_state: &WorldCameraState) -> ActionDescriptor {
    let camera = camera_state.orbit.to_camera();
    ActionDescriptor {
        controller_id: session_state.controller_id.clone(),
        action: "setCamera".into(),
        args: Some(semio_framework::dsl_value!({
            "windowId": session_state.surface_id,
            "camera": {
                "position": [
                    camera.position.x as f64,
                    camera.position.y as f64,
                    camera.position.z as f64,
                ],
                "target": [
                    camera.target.x as f64,
                    camera.target.y as f64,
                    camera.target.z as f64,
                ],
                "zoom": world3d_camera_zoom(&camera),
                "up": [f64::from(camera.up.x), f64::from(camera.up.y), f64::from(camera.up.z)],
            }
        })),
    }
}

/// 📥️ Borrows the validated host destination without moving its owner or reconstructing geometry.
pub fn world3d_admit_native_mesh_step(session_state: &mut WorldSessionState, interaction_state: &mut WorldInteractionState, scene_state: &mut WorldSceneState,donor:&mut Option<WorldNativeMeshResource>,receipt:semio_framework::kernel::ActorUiPatchReceipt,revision:WorldNativeUiRevision,resource_ordinal:usize,grant:RetainedCloneGrant)->Result<(bool,RetainedCloneProgress),WorldNativeMeshFailure>{
    let Some(row)=donor.as_ref()else{return Err(WorldNativeMeshFailure::Binding)};
    let identity=row.owner.identity();let key=row.mesh_key.as_str();
    if !receipt.is_valid()||revision.0==0||row.surface.as_str()!=session_state.surface_id||key.is_empty()||key.len()>WORLD_INTERACTION_ID_BYTE_CAPACITY{return Err(WorldNativeMeshFailure::Binding)}
    if let Some(WorldMeshSource::Native(source))=scene_state.meshes.0.get(key){return if source.receipt==receipt&&source.revision==revision&&source.resource_ordinal==resource_ordinal&&source.loan.identity()==identity&&source.loan.lease()==row.owner.lease()&&!source.frozen{Ok((true,Default::default()))}else{Err(WorldNativeMeshFailure::Binding)}}
    if grant.maximum_items==0||grant.maximum_depth==0{return Ok((false,Default::default()))}
    if scene_state.native_mesh_admission.is_none(){
        if scene_state.dynamic_mesh_close.is_some()||scene_state.dynamic_blocked_mesh.is_some(){return Ok((false,Default::default()))}
        let plan=scene_state.meshes.plan_insert(key).map_err(WorldNativeMeshFailure::Registry)?;if plan.occupied{return Err(WorldNativeMeshFailure::Registry(WorldDynamicFault::StaleToken))}
        scene_state.native_mesh_admission=Some(WorldNativeMeshAdmission{identity,lease:row.owner.lease().ok_or(WorldNativeMeshFailure::Binding)?,receipt,revision,resource_ordinal,key:String::new(),extent:key.len(),interaction_id:WorldInteractionId{bytes:[0;WORLD_INTERACTION_ID_BYTE_CAPACITY],len:0},interaction_offset:0,allocated:false});
        return Ok((false,RetainedCloneProgress{copied_items:1,..Default::default()}))
    }
    let cursor=scene_state.native_mesh_admission.as_mut().unwrap();
    if cursor.identity!=identity||Some(cursor.lease)!=row.owner.lease()||cursor.receipt!=receipt||cursor.revision!=revision||cursor.resource_ordinal!=resource_ordinal||cursor.extent!=key.len()||!key.starts_with(&cursor.key){return Err(WorldNativeMeshFailure::Binding)}
    if !cursor.allocated{if grant.maximum_capacity_bytes<cursor.extent{return Ok((false,Default::default()))}cursor.key.try_reserve_exact(cursor.extent).map_err(|_|WorldNativeMeshFailure::Registry(WorldDynamicFault::ByteCapacity))?;cursor.allocated=true;return Ok((false,RetainedCloneProgress{copied_items:1,retained_capacity_bytes:cursor.key.capacity(),..Default::default()}))}
    if cursor.key.len()<cursor.extent{let start=cursor.key.len();let mut end=start.saturating_add(grant.maximum_copy_bytes).min(cursor.extent);while !key.is_char_boundary(end){end-=1;}if end==start{return Ok((false,Default::default()))}cursor.key.push_str(&key[start..end]);return Ok((false,RetainedCloneProgress{copied_items:1,copied_bytes:end-start,..Default::default()}))}
    if cursor.interaction_offset<cursor.extent{let start=cursor.interaction_offset;let end=start.saturating_add(grant.maximum_copy_bytes).min(cursor.extent);if end==start{return Ok((false,Default::default()))}cursor.interaction_id.bytes[start..end].copy_from_slice(&key.as_bytes()[start..end]);cursor.interaction_offset=end;if end==cursor.extent{cursor.interaction_id.len=end as u16;}return Ok((false,RetainedCloneProgress{copied_items:1,copied_bytes:end-start,..Default::default()}))}
    let plan=scene_state.meshes.plan_insert(key).map_err(WorldNativeMeshFailure::Registry)?;if plan.occupied{return Err(WorldNativeMeshFailure::Registry(WorldDynamicFault::StaleToken))}
    let lease=row.owner.lease().ok_or(WorldNativeMeshFailure::Binding)?;let mut interaction=interaction_state.interaction_meshes.plan_admit_id(cursor.interaction_id,identity.cache_epoch,lease).map_err(WorldNativeMeshFailure::Registry)?;interaction.value.generation=interaction.epoch.checked_add(1).ok_or(WorldNativeMeshFailure::Registry(WorldDynamicFault::StaleToken))?;
    let Some((loan,progress))=row.owner.loan(grant).map_err(WorldNativeMeshFailure::Mesh)?else{return Ok((false,Default::default()))};
    let cursor=scene_state.native_mesh_admission.take().unwrap();let native=WorldNativeMeshSource{loan,receipt,revision,resource_ordinal,lookup_extent:cursor.extent,frozen:false,gpu_returned:false};
    scene_state.meshes.0.commit_insert(plan,cursor.key,WorldMeshSource::Native(native)).expect("exclusive World transaction preserves its prevalidated empty mesh slot");
    interaction_state.interaction_meshes.commit_admit(interaction).expect("exclusive World transaction preserves its prevalidated interaction slot");scene_state.geometry_generation=scene_state.geometry_generation.wrapping_add(1).max(1);
    Ok((true,progress))
}

/// ♻️ Closes a replaced host destination loan only after its exact GPU cache returns.
#[cfg(not(all(target_arch="wasm32",target_env="p2")))]
pub fn world3d_close_native_mesh_step(scene_state: &mut WorldSceneState,key:&str,receipt:semio_framework::kernel::ActorUiPatchReceipt,revision:WorldNativeUiRevision,resource_ordinal:usize,grant:RetainedCloneGrant,retire_gpu:impl FnOnce(&str,&WorldNativeMeshLoan)->Result<Option<ui_gpu::MeshGpuSourceReturn>,&'static str>)->Result<(bool,RetainedCloneProgress),WorldNativeMeshFailure>{
    if grant.maximum_items==0||grant.maximum_depth==0{return Ok((false,Default::default()))}
    if scene_state.dynamic_mesh_close.is_none(){
        let Some(WorldMeshSource::Native(source))=scene_state.meshes.0.get(key)else{return Ok((true,Default::default()))};
        if source.receipt!=receipt||source.revision!=revision||source.resource_ordinal!=resource_ordinal{return Err(WorldNativeMeshFailure::Binding)}
        let mut entry=scene_state.meshes.remove(key).unwrap();let WorldMeshSource::Native(source)=&mut entry.value else{unreachable!()};source.frozen=true;scene_state.dynamic_mesh_close=Some(entry);return Ok((false,RetainedCloneProgress{copied_items:1,..Default::default()}))
    }
    let entry=scene_state.dynamic_mesh_close.as_mut().unwrap();let WorldMeshSource::Native(source)=&mut entry.value else{return Err(WorldNativeMeshFailure::Binding)};
    if source.receipt!=receipt||source.revision!=revision||source.resource_ordinal!=resource_ordinal||source.lookup_extent!=key.len()||!key.starts_with(&entry.id){return Err(WorldNativeMeshFailure::Binding)}
    if !source.gpu_returned{let Some(witness)=retire_gpu(key,&source.loan).map_err(WorldNativeMeshFailure::Gpu)?else{return Ok((false,RetainedCloneProgress{copied_items:1,..Default::default()}))};if !witness.validates(key,&source.loan).map_err(WorldNativeMeshFailure::Gpu)?{return Err(WorldNativeMeshFailure::Binding)}source.gpu_returned=true;return Ok((false,RetainedCloneProgress{copied_items:1,..Default::default()}))}
    if !source.loan.terminal_is_empty(){let step=source.loan.return_step(grant).map_err(WorldNativeMeshFailure::Mesh)?;return Ok((false,step.progress))}
    let close=world_close_string(&mut entry.id,grant).map_err(WorldNativeMeshFailure::Close)?;let complete=matches!(close,semio_framework_job::InteractiveJobCloseStep::Complete{..});let progress=if complete{RetainedCloneProgress{copied_items:1,..close.progress()}}else{close.progress()};if complete{scene_state.dynamic_mesh_close.take();}Ok((complete,progress))
}

/// 🧹️ Cancels an unfinished lookup without touching its retained destination owner.
pub fn world3d_close_native_mesh_admission_step(scene_state: &mut WorldSceneState,grant:RetainedCloneGrant)->Result<(bool,RetainedCloneProgress),WorldNativeMeshFailure>{if grant.maximum_items==0||grant.maximum_depth==0{return Ok((false,Default::default()))}let Some(cursor)=scene_state.native_mesh_admission.as_mut()else{return Ok((true,Default::default()))};let step=world_close_string(&mut cursor.key,grant).map_err(WorldNativeMeshFailure::Close)?;let complete=matches!(step,semio_framework_job::InteractiveJobCloseStep::Complete{..});let progress=if complete{RetainedCloneProgress{copied_items:1,..step.progress()}}else{step.progress()};if complete{scene_state.native_mesh_admission.take();}Ok((complete,progress))}

/// 🧊️ The closing World source retains its loan until the original renderer returns its cache.
#[cfg(not(all(target_arch="wasm32",target_env="p2")))]
pub fn world3d_return_native_mesh_gpu_step(scene_state: &mut WorldSceneState,grant:RetainedCloneGrant,retire_gpu:impl FnOnce(&str,&WorldNativeMeshLoan)->Result<Option<ui_gpu::MeshGpuSourceReturn>,&'static str>)->Result<RetainedCloneProgress,WorldNativeMeshFailure>{
    let Some(entry)=scene_state.dynamic_mesh_close.as_mut()else{return Ok(Default::default())};let WorldMeshSource::Native(source)=&mut entry.value else{return Ok(Default::default())};
    if source.gpu_returned||grant.maximum_items==0||grant.maximum_depth==0{return Ok(Default::default())}source.frozen=true;
    let Some(witness)=retire_gpu(&entry.id,&source.loan).map_err(WorldNativeMeshFailure::Gpu)?else{return Ok(RetainedCloneProgress{copied_items:1,..Default::default()})};if !witness.validates(&entry.id,&source.loan).map_err(WorldNativeMeshFailure::Gpu)?{return Err(WorldNativeMeshFailure::Binding)}source.gpu_returned=true;Ok(RetainedCloneProgress{copied_items:1,..Default::default()})
}

pub fn retain_world_blocked_owner(scene_state: &mut WorldSceneState, owner: WorldOpaqueOwner) {
    assert!(scene_state.dynamic_blocked_owner.is_none(), "world dynamic producers must stop after an observable ownership fault");
    scene_state.dynamic_blocked_owner = Some(owner);
}

#[derive(Debug)]
/// 🎨️ Generation-owned pooled paint raster published after asynchronous image decoding.
pub struct DecodedMeshPaintImage {
    pub raster: SceneRasterLease,
}

/// 🖼️ Offers ONE reference underlay's decoded payload to the frame, KEEPING the state's own copy.
///
/// ⚖️ The copy stays for two reasons: a sibling surface of the same document holds its own
/// `World3dState` and must be able to offer the same image, and an offer can be LOST — a producer
/// rides the frame it was admitted into, and a frame whose present is ABORTED drops its staged
/// texture (`RasterTextureTable::abort_presented_step`). The re-offer on the next render is what
/// heals that, which is why this lane offers per render rather than once.
///
/// 🩸️ It briefly `mem::take`-d the payload, so the image crossed exactly once ever — and both panes
/// then painted no plan at all, because the one frame that carried it did not commit (ticket
/// 26/09/17/WGPU-RENDERER-REACT-PARITY, W9b's `🗑️generated/w9b-final/final.png` against
/// `🗑️generated/w8b-boot-2/final.png`). What made the per-render offer dangerous was never the offer:
/// it was reporting a refused admission as a frame fault, which
/// [`World3dBuildContext::ensure_world_plane_texture`] no longer does.
///
/// ⏭️ The honest end state is an upload ACK from the raster table — the worker cannot see `live`, so
/// it cannot know the texture is resident and must keep offering. Until that exists, the table's own
/// key-dedupe absorbs the repeat.
pub fn world_raster_upload_in_pool(scene_state: &WorldSceneState, id: &str, gpu_key: &str, paint: bool, pool: &SceneRasterPool) -> Option<SceneRasterLease> {
    let registry = if paint { &scene_state.mesh_paint_textures } else { &scene_state.reference_pixels };
    let raster = registry.get(id)?;
    if pool.gpu_resident(gpu_key, raster.identity) {
        raster.pending.lock().ok()?.take();
        return None;
    }
    raster.pending.lock().ok()?.take().or_else(|| pool.acquire(raster.identity).ok())
}

pub fn world_raster_upload(scene_state: &WorldSceneState, id: &str, gpu_key: &str, paint: bool) -> Option<SceneRasterLease> {
    world_raster_upload_in_pool(scene_state, id, gpu_key, paint, &world_scene_raster_pool())
}

pub fn world_reference_raster_ready_in_pool(scene_state: &WorldSceneState, url: &str, pool: &SceneRasterPool) -> bool {
    let Some(raster) = scene_state.reference_pixels.get(url) else { return false };
    if raster.pending.lock().is_ok_and(|pending| pending.is_some()) {
        return true;
    }
    pool.cpu_resident(raster.identity) || pool.gpu_resident(url, raster.identity)
}

pub fn world_reference_raster_ready(scene_state: &WorldSceneState, url: &str) -> bool {
    world_reference_raster_ready_in_pool(scene_state, url, &world_scene_raster_pool())
}

pub fn reference_underlay_upload(scene_state: &WorldSceneState, url: &str) -> Option<SceneRasterLease> {
    world_raster_upload(scene_state, url, url, false)
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum World3dSnapshotApplyStep {
    Idle,
    Pending,
    Complete,
    Stale,
    Fault,
}

pub fn close_world3d_snapshot_apply_step(bridge_state: &mut WorldBridgeState, context: &mut semio_framework_job::StepContext<'_>) -> bool {
    if context.should_yield() {
        return false;
    }
    let Some(cursor) = bridge_state.snapshot_apply.as_mut() else {
        return true;
    };
    let complete = cursor.close_step();
    context.consume_fuel(1);
    if complete {
        bridge_state.snapshot_apply = None;
    }
    complete
}

/// 🖍️ Resolves source targets per instance while retaining the ordinary active-object projection.
pub(crate) fn world_instance_selected_components(selection_state: &WorldSelectionState, gumball_state: &WorldGumballState, scene_state: &WorldSceneState,instance:&Instance3d,kind:&str)->HashSet<String> {
    let mut selected: HashSet<String> = gumball_state.gumball_selection_ids.iter().filter_map(|id|world_component_preview_target(scene_state,id,kind)).filter(|(object,_)|*object==instance.id).map(|(_,group)|group.to_string()).collect();
    if instance.component_source.is_none() && selection_state.active_object_id.as_deref()==Some(&instance.id) { selected.extend(selection_state.component_ids.iter().cloned()); }
    selected
}

pub fn reference_image_aspect(scene_state: &WorldSceneState, url: &str) -> f32 {
    scene_state
        .reference_pixels
        .get(url)
        .map_or(1.0, |raster| {
            let descriptor = raster.identity.descriptor();
            descriptor.width as f32 / descriptor.height.max(1) as f32
        })
        .max(0.01)
}

pub fn world_reference_plane_size(scene_state: &WorldSceneState, url: &str, width: f32) -> [f32; 2] {
    [width, width / reference_image_aspect(scene_state, url)]
}

pub struct WorldInteractionRegistryBuildCursor {
    pub revision: u64,
    initialized: bool,
    pub phase: WorldInteractionRegistryBuildPhase,
    pub draw: usize,
    pub instance: usize,
    pub mesh_probe: u16,
    pub vortex: usize,
    pub reference: usize,
    pub faulted: bool,
}

impl WorldInteractionRegistryBuildCursor {
    pub fn new(revision: u64) -> Self {
        Self { revision, initialized: false, phase: WorldInteractionRegistryBuildPhase::Instances, draw: 0, instance: 0, mesh_probe: 0, vortex: 0, reference: 0, faulted: false }
    }

    /// 🫥️ An EMPTY probe slot is "this mesh was never published", not a defect: the
    /// guest publishes a draw list and its mesh leases in separate deliveries, so a
    /// draw naming a glb whose lease has not landed is normal for the frames in
    /// between. Faulting here killed the whole page with `world3d retained
    /// interaction authority faulted` on the first pointer move after a mesh-bearing
    /// document mounted (ticket 26/09/17/WGPU-RENDERER-REACT-PARITY, `📓️w12c`).
    /// Skipping the draw is React's own behaviour — a `GlbInstanceMesh` whose loader
    /// has not resolved renders nothing and raycasts to nothing — and is the
    /// interaction twin of W11a's "a miss is a skipped draw, not a quarantine".
    pub fn step(&mut self, session_state: &mut WorldSessionState, interaction_state: &mut WorldInteractionState, scene_state: &mut WorldSceneState, context: &mut semio_framework_job::StepContext<'_>) -> WorldInteractionStep {
        if context.should_yield() {
            return WorldInteractionStep::Pending;
        }
        if self.faulted || interaction_state.interaction_objects.faulted || interaction_state.interaction_meshes.faulted {
            return WorldInteractionStep::Fault;
        }
        if self.revision != session_state.interaction_revision {
            return WorldInteractionStep::Stale;
        }
        if !self.initialized {
            interaction_state.interaction_objects.begin_instance_order();
            self.initialized = true;
            context.consume_fuel(1);
            return WorldInteractionStep::Pending;
        }
        match self.phase {
            WorldInteractionRegistryBuildPhase::Instances => {
                let Some(draw) = scene_state.draws.get(self.draw) else {
                    self.phase = WorldInteractionRegistryBuildPhase::Vortices;
                    context.consume_fuel(1);
                    return WorldInteractionStep::Pending;
                };
                if usize::from(self.mesh_probe) == WORLD_INTERACTION_MESH_CAPACITY || draw.mesh_key.len() > WORLD_INTERACTION_ID_BYTE_CAPACITY {
                    self.faulted = true;
                    return WorldInteractionStep::Fault;
                }
                let mesh_slot = (WorldInteractionMeshRegistry::hash(&draw.mesh_key) + usize::from(self.mesh_probe)) % WORLD_INTERACTION_MESH_CAPACITY;
                let mesh = match interaction_state.interaction_meshes.slots[mesh_slot] {
                    Some(entry) if entry.id.as_str() == draw.mesh_key && entry.version == live_world3d_mesh_version(scene_state, &draw.mesh_key) => Some(WorldInteractionMeshToken { slot: mesh_slot as u16, generation: entry.generation }),
                    Some(_) => {
                        self.mesh_probe += 1;
                        context.consume_fuel(1);
                        return WorldInteractionStep::Pending;
                    }
                    None => {
                        self.draw += 1;
                        self.instance = 0;
                        self.mesh_probe = 0;
                        context.consume_fuel(1);
                        return WorldInteractionStep::Pending;
                    }
                };
                let Some(instance) = draw.instances.get(self.instance) else {
                    self.draw += 1;
                    self.instance = 0;
                    self.mesh_probe = 0;
                    context.consume_fuel(1);
                    return WorldInteractionStep::Pending;
                };
                let translation = instance.model.cols[3];
                let Some(token) = interaction_state.interaction_objects.admit(
                    self.revision,
                    WorldInteractionObjectKind::Instance,
                    &instance.id,
                    mesh,
                    instance.model,
                    [self.draw as f32, self.instance as f32, if instance.selected { 1.0 } else { 0.0 }, translation[0], translation[1], translation[2], 0.0, 0.0], instance.component_source,
                ) else {
                    self.faulted = true;
                    return WorldInteractionStep::Fault;
                };
                if !interaction_state.interaction_objects.push_instance_order(token) {
                    self.faulted = true;
                    return WorldInteractionStep::Fault;
                }
                self.instance += 1;
            }
            WorldInteractionRegistryBuildPhase::Vortices => {
                let Some(vortex) = scene_state.vortices.get(self.vortex) else {
                    self.phase = WorldInteractionRegistryBuildPhase::References;
                    context.consume_fuel(1);
                    return WorldInteractionStep::Pending;
                };
                let position = vortex.position.unwrap_or([0.0, 0.0, 0.0]);
                let direction = vortex.direction.unwrap_or([0.0, 0.0, -1.0]);
                let values = [position[0] as f32, position[1] as f32, position[2] as f32, direction[0] as f32, direction[1] as f32, direction[2] as f32, vortex.radius.unwrap_or(0.36) as f32, 0.0];
                if interaction_state.interaction_objects.admit(self.revision, WorldInteractionObjectKind::Vortex, &vortex.full_id, None, Mat4::identity(), values, None).is_none() {
                    self.faulted = true;
                    return WorldInteractionStep::Fault;
                }
                self.vortex += 1;
            }
            WorldInteractionRegistryBuildPhase::References => {
                let Some(reference) = scene_state.references.get(self.reference) else {
                    self.phase = WorldInteractionRegistryBuildPhase::Complete;
                    interaction_state.interaction_objects.revision = self.revision;
                    context.consume_fuel(1);
                    return WorldInteractionStep::Pending;
                };
                self.reference += 1;
                if reference.hidden.unwrap_or(false) || reference.locked.unwrap_or(false) {
                    context.consume_fuel(1);
                    return WorldInteractionStep::Pending;
                }
                let Some(url) = reference.url.as_deref() else {
                    context.consume_fuel(1);
                    return WorldInteractionStep::Pending;
                };
                let origin = reference.origin.unwrap_or([0.0, 0.0, 0.0]);
                let width = world_reference_width(reference);
                let aspect = reference_image_aspect(scene_state, url);
                let values = [origin[0] as f32, origin[1] as f32, origin[2] as f32, width, width / aspect, 0.0, 0.0, 0.0];
                let id = reference.id.as_deref().unwrap_or(url);
                if interaction_state.interaction_objects.admit(self.revision, WorldInteractionObjectKind::Reference, id, None, Mat4::identity(), values, None).is_none() {
                    self.faulted = true;
                    return WorldInteractionStep::Fault;
                }
            }
            WorldInteractionRegistryBuildPhase::Complete => return WorldInteractionStep::Complete,
        }
        context.consume_fuel(1);
        WorldInteractionStep::Pending
    }

    pub fn close_step(&mut self, context: &mut semio_framework_job::StepContext<'_>) -> bool {
        if context.should_yield() {
            return false;
        }
        self.phase = WorldInteractionRegistryBuildPhase::Complete;
        context.consume_fuel(1);
        true
    }
}

#[expect(clippy::large_enum_variant, reason = "Interaction transitions retain fixed action claims and cursor storage without a fresh allocation between admitted steps.")]
pub enum WorldInteractionActive {
    Plan { plan: WorldInteractionPlan, retirement: Option<WorldInteractionAuthorityStep> },
    Pick { cursor: WorldRayPickCursor, retirement: Option<WorldInteractionAuthorityStep> },
    ObjectPick { cursor: WorldObjectPickCursor, retirement: Option<WorldInteractionAuthorityStep> },
    ComponentPick { cursor: WorldComponentPickCursor, retirement: Option<WorldInteractionAuthorityStep> },
    MarqueePick { cursor: WorldMarqueePickCursor, retirement: Option<WorldInteractionAuthorityStep> },
    MarqueePublish { job: WorldMarqueePublishJob, retirement: Option<WorldInteractionAuthorityStep> },
    ComponentMarqueePublish { job: WorldComponentMarqueePublishJob, retirement: Option<WorldInteractionAuthorityStep> },
    GumballPick { cursor: WorldGumballPickCursor, retirement: Option<WorldInteractionAuthorityStep> },
    GumballCommit { job: WorldGumballCommitJob, retirement: Option<WorldInteractionAuthorityStep> },
    BrushCommit { job: WorldBrushCommitJob, retirement: Option<WorldInteractionAuthorityStep> },
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
pub fn world_navigation_snapshot(camera_state: &WorldCameraState) -> WorldNavigationSnapshot {
    let camera = camera_state.orbit.to_camera();
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

pub struct World3dDynamicRetirement {
    pub phase: u8,
    pub blocked: Option<WorldDynamicFault>,
}

impl World3dDynamicRetirement {
    pub fn begin(camera_state: &mut WorldCameraState, scene_state: &mut WorldSceneState, assets_state: &mut WorldAssetsState) -> Self {
        scene_state.meshes.begin_close();
        scene_state.mesh_versions.begin_close();
        scene_state.mesh_appearances.begin_close();
        scene_state.draws.begin_close();
        scene_state.reference_pixels.begin_close();
        scene_state.mesh_paint_textures.begin_close();
        assets_state.asset_io.begin_close();
        camera_state.fit_bounds_cursor = None;
        Self { phase: 0, blocked: None }
    }

    pub fn step(&mut self, selection_state: &mut WorldSelectionState, interaction_state: &mut WorldInteractionState, scene_state: &mut WorldSceneState, bridge_state: &mut WorldBridgeState, context:&mut semio_framework_job::StepContext<'_>) -> bool {
        self.blocked = None;
        if scene_state.native_mesh_admission.is_some(){if let Ok((_,progress))=world3d_close_native_mesh_admission_step(scene_state,context.retained_grant()){if context.consume_retained(progress).is_ok()&&progress.copied_items!=0{context.consume_fuel(1);}}return false}
        if step_world3d_scene_bridge_close(bridge_state,context) {
            return false;
        }
        if scene_state.dynamic_mesh_close.is_none() {
            if let Some(entry) = scene_state.dynamic_blocked_mesh.take() {
                scene_state.dynamic_mesh_close = Some(entry);
                return false;
            }
        }
        if let Some(owner) = scene_state.dynamic_blocked_owner.take() {
            if let Err(owner) = quarantine_world_owner(owner) {
                scene_state.dynamic_blocked_owner = Some(owner);
                self.blocked = Some(WorldDynamicFault::QuarantineCapacity);
            }
            return false;
        }
        match self.phase {
            0 => {
                if scene_state.dynamic_mesh_close.is_some() {
                    step_world3d_mesh_close(scene_state,context);
                    return false;
                }
                let Some(entry) = scene_state.meshes.take_one() else {
                    self.phase = 1;
                    return false;
                };
                scene_state.dynamic_mesh_close = Some(entry);
            }
            1 => {
                if scene_state.mesh_versions.take_one().is_some() {
                    return false;
                }
                self.phase = 2;
                return false;
            }
            2 => {
                if scene_state.mesh_appearances.take_one().is_some() {
                    return false;
                }
                self.phase = 3;
                return false;
            }
            3 => {
                let Some(entry) = scene_state.draws.take_last() else {
                    self.phase = 4;
                    return false;
                };
                if let Err(owner) = quarantine_world_owner(WorldOpaqueOwner::Draw(entry)) {
                    let WorldOpaqueOwner::Draw(entry) = owner else { unreachable!("draw quarantine owner") };
                    scene_state.draws.restore_last(entry);
                    self.blocked = Some(WorldDynamicFault::QuarantineCapacity);
                }
            }
            4 => {
                let Some(entry) = scene_state.reference_pixels.take_one() else {
                    self.phase = 5;
                    return false;
                };
                drop(entry);
            }
            5 => {
                let Some(entry) = scene_state.mesh_paint_textures.take_one() else {
                    self.phase = 6;
                    return false;
                };
                drop(entry);
            }
            6 => {
                let Some(id) = interaction_state.instance_interaction_ids.keys().next().cloned() else {
                    self.phase = 7;
                    return false;
                };
                interaction_state.instance_interaction_ids.remove(&id);
            }
            7 => {
                let Some(id) = interaction_state.instance_interaction_granularity_ids.keys().next().cloned() else {
                    self.phase = 8;
                    return false;
                };
                interaction_state.instance_interaction_granularity_ids.remove(&id);
            }
            8 => {
                if selection_state.local_hover_granularity_id.take().is_some() {
                    return false;
                }
                self.phase = 9;
                return false;
            }
            _ => return true,
        }
        false
    }

    pub fn terminal_is_empty(&self) -> bool {
        self.phase >= 9 && self.blocked.is_none()
    }
}

pub fn begin_world3d_dynamic_retirement(session_state: &mut WorldSessionState, camera_state: &mut WorldCameraState, retirement_state: &mut WorldRetirementState, scene_state: &mut WorldSceneState, assets_state: &mut WorldAssetsState) -> bool {
    if retirement_state.dynamic_retirement.is_some() {
        return false;
    }
    session_state.snapshot_fault = Some(World3dSnapshotFault::Closing);
    retirement_state.dynamic_retirement = Some(World3dDynamicRetirement::begin(camera_state, scene_state, assets_state));
    true
}

//#endregion 🧹️World3dDynamicRetirement

//#region 🧱️WorldDrawRebuild
pub fn begin_world3d_draw_rebuild(session_state: &mut WorldSessionState, retirement_state: &mut WorldRetirementState, scene_state: &mut WorldSceneState, descriptor: WorldDrawRebuildDescriptor) -> Result<(), WorldDynamicFault> {
    if retirement_state.dynamic_retirement.is_some() || scene_state.draw_rebuild.is_some() || scene_state.retired_draws.is_some() || scene_state.dynamic_blocked_owner.is_some() {
        return Err(WorldDynamicFault::Closing);
    }
    if descriptor.revision != session_state.interaction_revision || descriptor.generation != scene_state.draw_generation.wrapping_add(1) {
        return Err(WorldDynamicFault::StaleToken);
    }
    scene_state.draw_rebuild = Some(WorldDrawRebuildCursor::new(descriptor)?);
    Ok(())
}

pub fn world3d_draw_rebuild_admit_draw(scene_state: &mut WorldSceneState, mesh_key: &str, mesh_version: u64, instance_count: u16) -> Result<(), WorldDynamicFault> {
    let Some(cursor) = scene_state.draw_rebuild.as_mut() else {
        return Err(WorldDynamicFault::StaleToken);
    };
    cursor.admit_draw(mesh_key, mesh_version, instance_count)
}

pub fn world3d_draw_rebuild_admit_instance(scene_state: &mut WorldSceneState, draw: u16, id: &str, model: Mat4, color: [f32; 4], selected: bool, hovered: bool) -> Result<(), WorldDynamicFault> {
    world3d_draw_rebuild_admit_retained_instance(scene_state, draw, id, model, color, SceneColorSource3d::Authored, selected, hovered, None)
}

pub fn world3d_draw_rebuild_admit_retained_instance(
    scene_state: &mut WorldSceneState,
    draw: u16,
    id: &str,
    model: Mat4,
    color: [f32; 4],
    color_source: SceneColorSource3d,
    selected: bool,
    hovered: bool,
    component_source: Option<ComponentSource3d>,
) -> Result<(), WorldDynamicFault> {
    let Some(cursor) = scene_state.draw_rebuild.as_mut() else {
        return Err(WorldDynamicFault::StaleToken);
    };
    cursor.admit_instance(draw, id, model, color, color_source, selected, hovered, component_source)
}

pub fn world3d_draw_rebuild_seal(scene_state: &mut WorldSceneState) -> Result<(), WorldDynamicFault> {
    scene_state.draw_rebuild.as_mut().ok_or(WorldDynamicFault::StaleToken)?.seal()
}

pub fn close_world3d_draw_rebuild_step(scene_state: &mut WorldSceneState, context: &mut semio_framework_job::StepContext<'_>) -> bool {
    if context.retirement_should_yield() {
        return false;
    }
    if let Some(retired) = scene_state.retired_draws.as_mut() {
        if let Some(entry) = retired.take_last() {
            if let Err(owner) = quarantine_world_owner(WorldOpaqueOwner::Draw(entry)) {
                let WorldOpaqueOwner::Draw(entry) = owner else { unreachable!("retired draw owner") };
                retired.restore_last(entry);
                return false;
            }
            context.consume_fuel(1);
            return false;
        }
        scene_state.retired_draws = None;
        context.consume_fuel(1);
        return false;
    }
    let Some(cursor) = scene_state.draw_rebuild.as_mut() else {
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
    scene_state.draw_rebuild = None;
    context.consume_fuel(1);
    true
}

pub fn world3d_draw_rebuild_terminal_is_empty(scene_state: &WorldSceneState) -> bool {
    scene_state.draw_rebuild.is_none() && scene_state.retired_draws.is_none()
}
//#endregion 🧱️WorldDrawRebuild
