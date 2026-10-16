//! 🧵️ World3d interaction: pointer, wheel and gesture entry points and the interaction authority.
//!
//! Crate `semio-framework-os-infinite-world-interaction`; a downward producer of the infinite world family (wasi-excluded except tests).

#![cfg(any(test, not(all(target_arch = "wasm32", target_env = "p2"))))]

use semio_framework_os_infinite_world_model::{quarantine_world_owner, World3dBuildContext, World3dMeshAsset, WORLD_DYNAMIC_ID_BYTE_CAPACITY, WORLD_DYNAMIC_MESH_CAPACITY, WORLD_INTERACTION_INTENT_CAPACITY, world_reference_width, world_scene_raster_pool, WorldCancelReason, WorldCatalogueDropPreviewRecord, WorldDrawRebuildStep, WorldDynamicEntry, WorldDynamicFault, WorldDynamicRejected, WorldEnvironmentRecord, WorldFlatAction, WorldFlatActionKind, WorldInteractionIntent, WorldInteractionIntentQueue, WorldInteractionObjectKind, WorldInteractionPhase, WorldInteractionPlan, WorldInteractionState, WorldInteractionStep, WorldMeshSource, WorldOpaqueOwner, WorldSceneRaster, WorldSessionState};
#[cfg(any(test, feature = "testkit"))]
use semio_framework_os_infinite_world_model::world_test_mesh_query;
use semio_framework_os_infinite_world_components::{GumballHandle, pick_targets_instance, TerrainTileMeshPayload, world3d_offers_transform_gumball, World3dRelocateSession, world_component_preview_target, world_mesh_component_id, WorldCameraState, WorldCameraSync, WorldGumballState, WorldPlaceholderKind, WorldPlaceholderMeshCursor, WorldPlaceholderMeshStep, WorldPlaceholderOwner, WorldSceneState, WorldSelectionState};
use semio_framework_os_infinite_world_cursors::{CLICK_DRAG_THRESHOLD_PX, instance_interaction_granularity_id, instance_interaction_id, live_world3d_mesh_version, PICK_EDGE_SCREEN_PX, PICK_VERTEX_SCREEN_PX, pointer_in_pick_rect, render_pick_viewport, resolved_domain_id, WORLD_INTERACTION_DOMAIN_ID, WORLD_ITEM_GRANULARITY_ID, WORLD_ITEM_PATH_DELIMITER, world_merge_mode, world_pick_rect, world_write_interaction_target, WorldBrushCommitJob, WorldComponentMarqueePublishJob, WorldComponentPickCursor, WorldComponentPickPurpose, WorldGumballCommitJob, WorldGumballGesture, WorldGumballPickCursor, WorldInteractionAuthorityStep, WorldInteractionText, WorldMarqueeGesture, WorldMarqueePickCursor, WorldMarqueePublishJob, WorldObjectPickCursor, WorldObjectPickPurpose, WorldRayPickCursor, WorldRayPickPurpose};
use semio_framework_os_infinite_world_scene::{begin_world_terrain_pending, decode_reference_image_bytes, DecodedReferenceImage, environment_scene_material, mesh_id_from_url, parse_color, publish_world3d_inline_asset, publish_world3d_mesh_lease, reserve_terrain_tile_fetch, retire_world_mesh, step_world3d_terrain_retirement, terrain_family_visible, terrain_tile_mesh_key, TerrainTileDraw, TerrainVisibleTileRow, world3d_reference_url_is_current, WorldAssetsState, WorldTerrainMeshProgress, WorldTerrainMeshPublicationStep, WorldTerrainMeshStep, WorldTerrainState};
#[cfg(not(all(target_arch = "wasm32", target_env = "p2")))]
use semio_framework_os_infinite_world_scene::WORLD_REFERENCE_TEXTURE_BYTES;
use semio_framework_os_infinite_world_bridge::{advance_world3d_view_revision, classify_world_navigation_gestures, component_mode_active, DecodedMeshPaintImage, gumball_preview_in_place, mesh_face_id, reference_image_aspect, retain_world_blocked_owner, scene_mesh_awaits_its_asset, selection_centroid, world3d_camera_zoom, WORLD_NAVIGATION_GESTURE_IDS, world_navigation_snapshot, WorldInteractionActive, WorldInteractionRegistryBuildCursor, WorldRetirementState};
#[cfg(any(test, feature = "testkit"))]
use semio_framework_os_infinite_world_bridge::{apply_gumball_preview, pointer_drag_distance, reset_gumball_preview};
use ui_wgpu::wgpu::{ActionDescriptor, axis_rotate_angle, Camera3d, gumball_eye, gumball_project_ray_onto_axis, HitKind, HitTarget, Instance3d, interpolate_mesh_uv, LineVertex3d, marquee_is_crossing_from_path, Mat4, Mesh3dField, Mesh3dLease, PointerModifiers, ray_aabb_slab, ray_plane_point, Rect, Rgba, SceneDraw3d, SceneRasterBegin, SceneRasterDescriptor, SceneRasterLease, SceneRasterProfile, SceneRasterWriteMode, UiComponentSceneNode, Vec3, World3dSnapshotFault};
use semio_framework::{GranularityDefinition, HierarchyProvider, HoverSpec, InteractionDefinition, MergeMode, SelectionMethod, SelectionMode, SelectionSpec};
use std::collections::{HashMap, HashSet};
use semio_framework_ui_locale::LocalizedLabel;
use std::sync::Mutex;
use semio_framework_value::RetainedCloneGrant;
#[cfg(not(all(target_arch = "wasm32", target_env = "p2")))]
use ui_wgpu::wgpu::{SceneAuthoredMaterial3d, SceneLighting3d, SceneMaterialAlpha3d};
#[cfg(any(test, feature = "testkit"))]
use ui_wgpu::wgpu::{gizmo, ray_pick_instance, ray_pick_mesh_detail, screen_select_components, screen_select_instances};
use serde_json::json;

/// 🧩️ World3d authority component: the interaction authority state machine with its active cursor or job.
pub struct WorldAuthorityState {

    pub interaction_authority: Option<WorldInteractionAuthority>,
}

impl WorldAuthorityState {
    pub fn new() -> Self {
        Self {
            interaction_authority: Some(WorldInteractionAuthority::default()),
        }
    }
}

/// 🌉️📡️ Records how one geometry-less `meshes_json` entry resolves.
///
/// A `url` entry is left PENDING on purpose: its id is bound to the url in
/// [`World3dState::mesh_source_urls`], which is the ledger `render_world_3d`'s missing-mesh loop
/// reads to reserve a [`WorldAssetRequestKind::Glb`] fetch — the mesh table itself stays empty until
/// the decoded GLB lands through [`publish_world3d_asset_mesh_lease`]. Publishing a placeholder box
/// here would be worse than drawing nothing: the loop offers a url only while
/// `!state.meshes.contains_key(key)`, so a stand-in mesh would suppress the fetch forever, which is
/// precisely the shape of the dead url lane this replaces (ticket 26/09/17/WGPU-RENDERER-REACT-
/// PARITY, `📓️w3a-asset-decoder-boot-fault.md` §6). React draws nothing for an unloaded url either —
/// `GlbInstanceMesh` suspends until `useLoader` resolves.
///
/// A `kind` entry is the opposite: both renderers own the generator, so it resolves immediately
/// through the same fixed-credit placeholder ladder inline buffers ride.
///
/// Bounded by the mesh registry's own capacity and id budget — an id that could never publish a mesh
/// is not worth a ledger row, and the ledger never outgrows the table it indexes.
pub fn declare_scene_mesh_source(session_state: &mut WorldSessionState, authority_state: &mut WorldAuthorityState, retirement_state: &mut WorldRetirementState, scene_state: &mut WorldSceneState, id: &str, url: Option<&str>, kind: Option<&str>) {
    if let Some(url) = url {
        if id.len() > WORLD_DYNAMIC_ID_BYTE_CAPACITY || url.len() > WORLD_DYNAMIC_ID_BYTE_CAPACITY {
            return;
        }
        if scene_state.mesh_source_urls.len() >= WORLD_DYNAMIC_MESH_CAPACITY && !scene_state.mesh_source_urls.contains_key(id) {
            return;
        }
        scene_state.mesh_source_urls.insert(id.to_owned(), url.to_owned());
        return;
    }
    if let Some(kind) = kind {
        begin_world_placeholder_mesh(session_state, authority_state, retirement_state, scene_state, id, WorldPlaceholderKind::resolve(kind));
    }
}

/// 🧊️ Drains up to `budget` authority writes of the in-flight mesh publication.
pub fn step_world_placeholder_mesh_batch(session_state: &mut WorldSessionState, interaction_state: &mut WorldInteractionState, authority_state: &mut WorldAuthorityState, scene_state: &mut WorldSceneState, budget: u32) {
    for _ in 0..budget {
        if scene_state.placeholder_build.is_none() {
            return;
        }
        step_world_placeholder_mesh(session_state, interaction_state, authority_state, scene_state);
    }
}

pub fn apply_decoded_reference_image(session_state: &mut WorldSessionState, authority_state: &mut WorldAuthorityState, scene_state: &mut WorldSceneState, assets_state: &mut WorldAssetsState, url: &str, decoded: DecodedReferenceImage) -> bool {
    let expected = usize::try_from(decoded.width)
        .ok()
        .and_then(|width| usize::try_from(decoded.height).ok().and_then(|height| width.checked_mul(height)))
        .and_then(|pixels| pixels.checked_mul(4));
    if expected != Some(decoded.pixels.len()) || decoded.pixels.len() > WORLD_REFERENCE_TEXTURE_BYTES || !world3d_reference_url_is_current(scene_state, url) {
        if !world3d_reference_url_is_current(scene_state, url) {
            assets_state.pending_image_urls.remove(url);
        }
        return false;
    }
    let descriptor = SceneRasterDescriptor { width: decoded.width, height: decoded.height, source_digest: decoded.source_digest, source_revision: 1, profile: SceneRasterProfile::ReferenceImageMapNoColorSpace, mesh: None };
    let owner = decoded.source_digest[0].wrapping_add(decoded.source_digest[1]).max(1);
    let pool = world_scene_raster_pool();
    let lease = match pool.begin(descriptor, owner, SceneRasterWriteMode::Moved { capacity_bytes: decoded.pixels.capacity() }) {
        SceneRasterBegin::Reused(lease) => lease,
        SceneRasterBegin::Writer(writer) => match pool.prepare_moved(writer, decoded.pixels).and_then(|prepared| pool.seal_prepared_moved(prepared)) {
            Ok(lease) => lease,
            Err((_pixels, _fault)) => {
                let _ = pool.cancel(writer);
                return false;
            }
        },
        SceneRasterBegin::Backpressure(_) | SceneRasterBegin::Refused(_) => return false,
    };
    apply_decoded_reference_raster(session_state, authority_state, scene_state, assets_state, url, lease)
}

pub fn apply_decoded_reference_raster(session_state: &mut WorldSessionState, authority_state: &mut WorldAuthorityState, scene_state: &mut WorldSceneState, assets_state: &mut WorldAssetsState, url: &str, lease: SceneRasterLease) -> bool {
    let descriptor = lease.identity().descriptor();
    if descriptor.profile != SceneRasterProfile::ReferenceImageMapNoColorSpace || !world3d_reference_url_is_current(scene_state, url) {
        if !world3d_reference_url_is_current(scene_state, url) {
            assets_state.pending_image_urls.remove(url);
        }
        return false;
    }
    if !publish_world_raster(session_state, authority_state, scene_state, url.to_string(), lease, false) {
        return false;
    }
    assets_state.pending_image_urls.remove(url);
    true
}

pub fn apply_reference_image_bytes(session_state: &mut WorldSessionState, authority_state: &mut WorldAuthorityState, scene_state: &mut WorldSceneState, assets_state: &mut WorldAssetsState, url: &str, bytes: &[u8]) -> bool {
    decode_reference_image_bytes(bytes).is_some_and(|decoded| apply_decoded_reference_image(session_state, authority_state, scene_state, assets_state, url, decoded))
}

fn retained_gumball_gesture(authority_state: &WorldAuthorityState) -> Option<&WorldGumballGesture> {
    let authority = authority_state.interaction_authority.as_ref()?;
    authority.gumball.as_ref().or(match authority.active.as_ref() {
        Some(WorldInteractionActive::GumballCommit { job, .. }) => Some(&job.gesture),
        _ => None,
    })
}

pub fn retained_gumball_preview_model(interaction_state: &WorldInteractionState, authority_state: &WorldAuthorityState, draw_index: usize, instance_index: usize, model: Mat4) -> Mat4 {
    let Some(gesture) = retained_gumball_gesture(authority_state).filter(|gesture| !gesture.live) else {
        return model;
    };
    let selected = gesture.selected[..usize::from(gesture.selected_len)]
        .iter()
        .flatten()
        .any(|target| interaction_state.interaction_objects.resolve(target.object).is_some_and(|entry| entry.kind == WorldInteractionObjectKind::Instance && entry.values[0] == draw_index as f32 && entry.values[1] == instance_index as f32));
    if !selected {
        return model;
    }
    gumball_preview_in_place(model, gesture.handle, gesture.translate, gesture.angle, gesture.scale)
}

/// 🕒️ Whether this surface has a camera intent waiting to publish — the settle a projection switch
/// or a wheel queues.
pub fn world3d_pending_camera_settle(camera_state: &WorldCameraState, authority_state: &WorldAuthorityState) -> bool {
    camera_state.camera_sync.owed || authority_state.interaction_authority.as_ref().is_some_and(|authority| authority.queue.len > 0)
}

pub fn store_mesh(session_state: &mut WorldSessionState, interaction_state: &mut WorldInteractionState, authority_state: &mut WorldAuthorityState, scene_state: &mut WorldSceneState, id: String, mesh: Mesh3dLease) {
    if let Err(rejected) = publish_world3d_mesh_lease(interaction_state, scene_state, id, mesh) {
        retain_rejected_world_mesh(session_state, authority_state, scene_state, rejected);
    }
}

pub(crate) fn retain_rejected_world_mesh(session_state: &mut WorldSessionState, authority_state: &mut WorldAuthorityState, scene_state: &mut WorldSceneState, rejected: WorldDynamicRejected<Mesh3dLease>) {
    let fault = rejected.fault;
    if scene_state.dynamic_blocked_mesh.is_none() {
        scene_state.dynamic_blocked_mesh = Some(WorldDynamicEntry { id: rejected.id, epoch: 0, value: WorldMeshSource::Owned{lease:Some(rejected.value),closing:false} });
    }
    mark_world_dynamic_fault(session_state, authority_state, scene_state, fault);
}

#[cfg(any(test, feature = "testkit"))]
fn set_world_mesh_version(session_state: &mut WorldSessionState, authority_state: &mut WorldAuthorityState, scene_state: &mut WorldSceneState, id: String, version: u64) {
    match scene_state.mesh_versions.insert(id, version) {
        Ok((_, previous)) => drop(previous),
        Err(rejected) => {
            drop(rejected.id);
            mark_world_dynamic_fault(session_state, authority_state, scene_state, rejected.fault);
        }
    }
}

pub fn mark_world_dynamic_fault(session_state: &mut WorldSessionState, authority_state: &mut WorldAuthorityState, scene_state: &mut WorldSceneState, fault: WorldDynamicFault) {
    scene_state.dynamic_fault=Some(fault);
    session_state.snapshot_fault = Some(World3dSnapshotFault::Capacity);
    if let Some(authority) = authority_state.interaction_authority.as_mut() {
        authority.faulted = true;
    }
}

pub(crate) fn publish_world_raster(session_state: &mut WorldSessionState, authority_state: &mut WorldAuthorityState, scene_state: &mut WorldSceneState, id: String, lease: SceneRasterLease, paint: bool) -> bool {
    let value = WorldSceneRaster { identity: lease.identity(), pending: Mutex::new(Some(lease)) };
    let registry = if paint { &mut scene_state.mesh_paint_textures } else { &mut scene_state.reference_pixels };
    match registry.insert(id, value) {
        Ok((_, previous)) => {
            drop(previous);
            true
        }
        Err(rejected) => {
            let fault = rejected.fault;
            drop(rejected);
            mark_world_dynamic_fault(session_state, authority_state, scene_state, fault);
            false
        }
    }
}

/// 🎨️ Publishes exact decoded paint pixels for a resident mesh with one UV per vertex.
pub fn apply_decoded_mesh_paint_image(session_state: &mut WorldSessionState, authority_state: &mut WorldAuthorityState, scene_state: &mut WorldSceneState, mesh_key: &str, decoded: DecodedMeshPaintImage) -> bool {
    let descriptor = decoded.raster.identity().descriptor();
    let mesh_version = live_world3d_mesh_version(scene_state,mesh_key);
    let mesh_seal_is_current = scene_state.meshes.get(mesh_key).and_then(|mesh| mesh.schema().ok()).is_some_and(|schema| {
        schema.uvs == schema.vertices
            && schema.uvs > 0
            && descriptor.mesh.is_some_and(|seal| seal.mesh_revision == mesh_version && seal.uv_revision == mesh_version && seal.uv_count == schema.uvs)
    });
    descriptor.profile == SceneRasterProfile::MeshPaintMapNoColorSpace && mesh_seal_is_current && publish_world_raster(session_state, authority_state, scene_state, mesh_key.to_string(), decoded.raster, true)
}

//#region 🧭️WorldOrbitViewGizmo
/** 🧭️ The pure placement/tip-geometry/paint logic relocated to `ui_wgpu::wgpu::widgets::gizmo` (see
`.🧬semio/🦑️repo/🎫️tickets/26/08/05/FRAMEWORK-BUILDER-PASSTHROUGHS-APP-COMMANDS-MACRO-WIDGET-EXTRACTION`) — this
region now only keeps the `World3dState`-specific hover-state plumbing (app config, not paint), calling
through to `gizmo::orbit_view_gizmo_placement`/`gizmo::orbit_view_gizmo_tips`/`gizmo::orbit_view_gizmo_hit_test`. */
#[cfg(any(test, feature = "testkit"))]
fn update_world_orbit_view_gizmo_hover(camera_state: &mut WorldCameraState, x: f32, y: f32, inner: Rect) {
    let (margin_x, margin_y) = gizmo::orbit_view_gizmo_placement(inner);
    let origin_x = inner.x + inner.w - margin_x;
    let origin_y = inner.y + inner.h - margin_y;
    let zone_radius = 56.0;
    if ((x - origin_x).powi(2) + (y - origin_y).powi(2)).sqrt() > zone_radius {
        camera_state.gizmo_hovered_tip = None;
        return;
    }
    let camera = camera_state.orbit.to_camera();
    let tips = gizmo::orbit_view_gizmo_tips(&camera, inner);
    camera_state.gizmo_hovered_tip = gizmo::orbit_view_gizmo_hit_test(x, y, &tips);
}
//#endregion 🧭️WorldOrbitViewGizmo

pub fn world3d_hit_target(scene: &UiComponentSceneNode, bounds: Rect) -> HitTarget<ActionDescriptor> {
    HitTarget { rect: bounds, event: None, control_id: Some(scene.surface_id.clone()), kind: HitKind::World3d, drag_axis: None, drag_data: None }
}

#[cfg(any(test, feature = "testkit"))]
pub(crate) fn handle_world3d_pointer_move(session_state: &mut WorldSessionState, camera_state: &mut WorldCameraState, selection_state: &mut WorldSelectionState, interaction_state: &mut WorldInteractionState, scene_state: &mut WorldSceneState, x: f32, y: f32, down: bool, button: i16) -> Option<ActionDescriptor> {
    let inner = world_pick_rect(session_state);
    if !inner.contains(x, y) {
        camera_state.gizmo_hovered_tip = None;
        return None;
    }
    if down && button == 0 {
        if selection_state.marquee_active {
            selection_state.marquee_points.push([x, y]);
            update_marquee_preview(camera_state, selection_state, scene_state, render_pick_viewport(session_state));
        } else if selection_state.paint_stroke_active && selection_state.interaction_mode == "paint" {
            if let Some((object_id, u, v)) = pick_paint_hit(session_state, camera_state, scene_state, x, y, inner) {
                return Some(ActionDescriptor {
                    controller_id: session_state.controller_id.clone(),
                    action: "paintAt".into(),
                    args: Some(semio_framework::dsl_value!({
                        "surfaceId": session_state.surface_id,
                        "objectId": object_id,
                        "u": u,
                        "v": v,
                        "phase": "stream",
                    })),
                });
            }
        } else if button == 2 || button == 1 {
            return None;
        }
    }
    if !down {
        update_world_orbit_view_gizmo_hover(camera_state, x, y, inner);
        return pick_hover_action(session_state, camera_state, selection_state, interaction_state, scene_state, x, y, inner);
    }
    if button == 2 {
        return None;
    }
    None
}

#[cfg(any(test, feature = "testkit"))]
pub(crate) fn handle_world3d_paint_actions(session_state: &mut WorldSessionState, camera_state: &mut WorldCameraState, selection_state: &mut WorldSelectionState, scene_state: &mut WorldSceneState, x: f32, y: f32, down: bool, button: i16) -> Vec<ActionDescriptor> {
    if selection_state.interaction_mode != "paint" || button != 0 {
        return Vec::new();
    }
    let inner = world_pick_rect(session_state);
    if !inner.contains(x, y) {
        return Vec::new();
    }
    if down && selection_state.paint_stroke_active {
        let Some((object_id, u, v)) = pick_paint_hit(session_state, camera_state, scene_state, x, y, inner) else {
            return Vec::new();
        };
        return vec![ActionDescriptor {
            controller_id: session_state.controller_id.clone(),
            action: "paintAt".into(),
            args: Some(semio_framework::dsl_value!({
                "surfaceId": session_state.surface_id,
                "objectId": object_id,
                "u": u,
                "v": v,
                "phase": "stream",
            })),
        }];
    }
    Vec::new()
}

/// 🖱️📋️ Resolves which entity a right-click context menu targets, in the React reference's exact
/// priority order — `resolveWorldContextMenuTarget` in `🧱️elements/🌐️World3dHost/🟦️.tsx:1893`: a
/// hovered vortex wins first, then a hovered mesh component (vertex/edge/face — reported as kind
/// `"object"`, the component's owning instance, matching the React source's own naming), then a
/// hovered reference image plane, then any other plain hover as `"object"`; `None` (no target) if
/// nothing is currently hovered.
pub fn resolve_world_context_menu_target(selection_state: &WorldSelectionState) -> Option<(&'static str, String)> {
    if let Some(vortex_id) = selection_state.hovered_vortex_id.clone() {
        return Some(("vortex", vortex_id));
    }
    if selection_state.hovered_component_mode.is_some() {
        if let Some(object_id) = selection_state.hovered_component_object_id.clone() {
            return Some(("object", object_id));
        }
    }
    let hovered = selection_state.local_hover_id.as_deref()?;
    if let Some(reference_id) = hovered.strip_prefix("reference:") {
        return Some(("reference", reference_id.to_string()));
    }
    Some(("object", hovered.to_string()))
}

/// 🪧️ The `surface` half of one world right-click's context-menu request: the entity under the
/// pointer as the single `hits` row, and the painted selection as per-domain `selection` groups —
/// the Rust twin of `world3dContextMenuSurfaceV1`
/// (`🧱️elements/🌐️World3dHost/🟦️.tsx:1909`). The hit target rides THIS request; no separate
/// target-recording dispatch precedes the menu.
pub fn world3d_context_menu_surface(selection_state: &WorldSelectionState) -> (Vec<ui_wgpu::wgpu::ContextMenuHit>, Vec<ui_wgpu::wgpu::ContextMenuSelectionGroup>) {
    let hits = resolve_world_context_menu_target(selection_state).map(|(domain, id)| ui_wgpu::wgpu::ContextMenuHit { domain: domain.to_string(), id, label: None }).into_iter().collect();
    let mut selection = Vec::new();
    if !selection_state.selected_ids.is_empty() {
        selection.push(ui_wgpu::wgpu::ContextMenuSelectionGroup { domain: "object".into(), ids: selection_state.selected_ids.clone() });
    }
    if !selection_state.component_ids.is_empty() {
        selection.push(ui_wgpu::wgpu::ContextMenuSelectionGroup { domain: "feature".into(), ids: selection_state.component_ids.clone() });
    }
    (hits, selection)
}

#[cfg(any(test, feature = "testkit"))]
pub(crate) fn handle_world3d_pointer_drag(session_state: &mut WorldSessionState, camera_state: &mut WorldCameraState, selection_state: &mut WorldSelectionState, gumball_state: &mut WorldGumballState, scene_state: &mut WorldSceneState, x: f32, y: f32, dx: f32, dy: f32, button: i16, modifiers: &PointerModifiers) {
    let inner = world_pick_rect(session_state);
    if button == 0 {
        if gumball_state.gumball_handle.is_some() && inner.contains(x, y) {
            gumball_drag_update(camera_state, selection_state, gumball_state, scene_state, x, y, inner);
            return;
        }
        if selection_state.drag_object_id.is_none() && gumball_state.gumball_handle.is_none() && !component_mode_active(selection_state) && pointer_drag_distance(selection_state, x, y) > CLICK_DRAG_THRESHOLD_PX {
            if let Some(object_id) = selection_state.press_object_id.take().or_else(|| pick_instance_at(session_state, camera_state, scene_state, x, y, inner)) {
                selection_state.drag_object_id = Some(object_id.clone());
                selection_state.marquee_active = false;
                selection_state.marquee_points.clear();
                selection_state.marquee_preview_ids.clear();
                selection_state.drag_last_position = object_world_position(scene_state, &object_id);
                selection_state.drag_object_z = selection_state.drag_last_position.map(|p| p[2]).unwrap_or(0.0);
            }
        }
        if selection_state.drag_object_id.is_some() && inner.contains(x, y) {
            if let Some(position) = world3d_ground_plane_pick(session_state, camera_state, x, y, selection_state.drag_object_z) {
                selection_state.drag_last_position = Some(position);
                if let Some(object_id) = selection_state.drag_object_id.clone() {
                    update_dragged_instance_position(scene_state, &object_id, position);
                }
            }
            return;
        }
    }
    if button == 1 || (button == 2 && modifiers.shift) {
        camera_state.orbit.pan(-dx, -dy);
    } else if button == 2 && (modifiers.alt || modifiers.meta) {
        camera_state.orbit.orbit(dx, dy);
    }
}

#[cfg(any(test, feature = "testkit"))]
pub(crate) fn handle_world3d_wheel(camera_state: &mut WorldCameraState, delta: f32) {
    camera_state.orbit.zoom(delta);
}

/// 🎯️ Optimistic local set algebra over the ONE framework merge vocabulary — the five
/// `MergeMode` words of `🕹️interaction/🧬️schema/🔣️.json` (fixture `🎯️merge-modes.json`). It takes a
/// decoded mode, never a word: ticket 26/09/09/PROCEDURAL-3D-END-TO-END deleted this file's private
/// `add`/`remove`/`toggle` spelling, which made `worldPick` and the domain-bound `interactionSelect`
/// disagree about what a shift-click means. `Range` needs an ordered topology this local preview does
/// not carry, so it replaces — the documented degradation, identical to `next_selection`'s.
fn merge_string_ids(existing: &[String], incoming: &[String], merge: MergeMode) -> Vec<String> {
    match merge {
        MergeMode::Additive => {
            let mut merged = existing.to_vec();
            for id in incoming {
                if !merged.contains(id) {
                    merged.push(id.clone());
                }
            }
            merged
        }
        MergeMode::Invertive => {
            let mut merged = existing.to_vec();
            for id in incoming {
                if let Some(index) = merged.iter().position(|entry| entry == id) {
                    merged.remove(index);
                } else {
                    merged.push(id.clone());
                }
            }
            merged
        }
        MergeMode::Subtractive => existing.iter().filter(|id| !incoming.contains(id)).cloned().collect(),
        MergeMode::Replace | MergeMode::Range => incoming.to_vec(),
    }
}

pub fn world_interaction_definition() -> InteractionDefinition {
    InteractionDefinition {
        id: WORLD_INTERACTION_DOMAIN_ID.into(),
        label: LocalizedLabel::native("World", "Welt"),
        granularities: vec![
            GranularityDefinition { id: "surface".into(), label: LocalizedLabel::native("Surface", "Fläche"), icon_id: "layers".into() },
            GranularityDefinition { id: WORLD_ITEM_GRANULARITY_ID.into(), label: LocalizedLabel::native("Item", "Objekt"), icon_id: "box".into() },
            GranularityDefinition { id: "vertex".into(), label: LocalizedLabel::native("Vertex", "Ecke"), icon_id: "circle-dot".into() },
            GranularityDefinition { id: "edge".into(), label: LocalizedLabel::native("Edge", "Kante"), icon_id: "minus".into() },
            GranularityDefinition { id: "face".into(), label: LocalizedLabel::native("Face", "Fläche"), icon_id: "square".into() },
        ],
        hierarchy: HierarchyProvider::PathDelimited { delimiter: WORLD_ITEM_PATH_DELIMITER.into() },
        hover: HoverSpec::default(),
        selection: SelectionSpec {
            modes: vec![SelectionMode::Multiple, SelectionMode::Single],
            methods: vec![SelectionMethod::Pick, SelectionMethod::Rectangle, SelectionMethod::Lasso],
            merges: vec![MergeMode::Replace, MergeMode::Additive, MergeMode::Subtractive, MergeMode::Invertive],
            transitive: false,
            broadcast: true,
        },
    }
}

#[cfg(any(test, feature = "testkit"))]
fn world_item_target_id(surface_id: &str, object_id: &str) -> String {
    format!("{surface_id}{WORLD_ITEM_PATH_DELIMITER}{object_id}")
}

/// 🔤️ Strips this state's own `surfaceId/` prefix from a `world`-domain item target id — the inverse
/// of [`world_item_target_id`]. Targets for a different surface (cross-surface batch, not produced by
/// this file's single-surface pointer handlers) are dropped rather than mis-parsed.
fn world_item_id_for_surface<'a>(session_state: &WorldSessionState, target_id: &'a str) -> Option<&'a str> {
    target_id.strip_prefix(&session_state.surface_id).and_then(|rest| rest.strip_prefix(WORLD_ITEM_PATH_DELIMITER))
}

/// 🧮️ Builds the wire target id for a plain pick/hover hit: a bare object id when a real app domain is
/// bound (`HierarchyProvider::Flat`-style, single-surface-scoped), else `world_item_target_id`'s
/// `"surfaceId/id"` `PathDelimited` shape for the shared `world` domain.
#[cfg(any(test, feature = "testkit"))]
pub fn resolved_item_id(session_state: &WorldSessionState, object_id: &str) -> String {
    if session_state.bound_domain_id.is_some() {
        object_id.to_string()
    } else {
        world_item_target_id(&session_state.surface_id, object_id)
    }
}

/// 🔤️ Inverse of [`resolved_item_id`], for parsing ids back out of an incoming action's targets during
/// optimistic local preview.
fn parse_resolved_item_id<'a>(session_state: &WorldSessionState, target_id: &'a str) -> Option<&'a str> {
    if session_state.bound_domain_id.is_some() {
        Some(target_id)
    } else {
        world_item_id_for_surface(session_state, target_id)
    }
}

/// 🕹️ [`world_merge_mode`] as the fixed `u8` the bounded pick cursors carry through a retained
/// gesture — the cursors hold no `String`, so the mode travels as its ordinal and is spelled once,
/// at publication, by [`world_merge_wire_label`].
pub fn world_merge_code(shift: bool, ctrl: bool, meta: bool) -> u8 {
    match world_merge_mode(shift, ctrl || meta) {
        MergeMode::Additive => 1,
        MergeMode::Subtractive => 2,
        MergeMode::Invertive => 3,
        _ => 0,
    }
}

/// 🕹️ Projects the original encoded interaction targets into the existing local preview fields.
pub fn apply_world_action_preview(session_state: &mut WorldSessionState, selection_state: &mut WorldSelectionState, gumball_state: &mut WorldGumballState, scene_state: &mut WorldSceneState, action: &ActionDescriptor) {
    let Some(args) = action.args.as_ref() else { return; };
    if args.get("domainId").and_then(|value| value.as_str()) != Some(resolved_domain_id(session_state)) || !matches!(action.action.as_str(), "interactionSelect" | "interactionHover") { return; }
    let Some(text) = args.get("targets").and_then(|value| value.as_str()).filter(|text| text.len() <= ui_wgpu::wgpu::ACTION_STRING_BYTE_CAPACITY) else { return; };
    let Ok(targets) = semio_framework_pack_json::parse(text, semio_framework_pack_json::JsonMemberPolicy::Reject) else { return; };
    let Some(targets) = targets.as_array() else { return; };
    let mut ids = Vec::new();
    let mut components = Vec::new();
    let mut component_object = None;
    let mut component_mode = None;
    for target in targets {
        let (Some(granularity), Some(id)) = (target.get("granularity").and_then(|value| value.as_str()), target.get("id").and_then(|value| value.as_str()).and_then(|id| parse_resolved_item_id(session_state, id))) else { return; };
        if matches!(granularity, "vertex" | "edge" | "face") {
            let Some((object, group)) = world_component_preview_target(scene_state, id, granularity) else { return; };
            components.push(group.to_string());
            component_object.get_or_insert_with(|| object.to_string());
            component_mode.get_or_insert_with(|| granularity.to_string());
        }
        ids.push(id.to_string());
    }
    if action.action == "interactionSelect" {
        let Some(merge) = args.get("merge").and_then(|value| value.as_str()).and_then(MergeMode::from_wire_label) else { return; };
        if let Some(mode) = component_mode {
            gumball_state.gumball_selection_ids = merge_string_ids(&gumball_state.gumball_selection_ids, &ids, merge);
            let selected: Vec<(String,u32)> = gumball_state.gumball_selection_ids.iter().filter_map(|id| world_component_preview_target(scene_state,id,&mode).map(|(object,group)|(object.to_string(),group))).collect();
            selection_state.selected_ids = selected.iter().map(|(object,_)|object.clone()).collect::<std::collections::BTreeSet<_>>().into_iter().collect();
            let retained = |object:&str|selected.iter().any(|(selected,_)|selected==object);
            selection_state.active_object_id = component_object.filter(|object|retained(object)).or_else(||selection_state.active_object_id.take().filter(|object|retained(object))).or_else(||selected.first().map(|(object,_)|object.clone()));
            selection_state.component_ids = selected.iter().filter(|(object,_)|Some(object)==selection_state.active_object_id.as_ref()).map(|(_,group)|group.to_string()).collect::<std::collections::BTreeSet<_>>().into_iter().collect();
            selection_state.granularity = mode;
        } else {
            selection_state.selected_ids = merge_string_ids(&selection_state.selected_ids, &ids, merge);
            if merge == MergeMode::Replace { gumball_state.gumball_selection_ids.clear();selection_state.component_ids.clear();selection_state.active_object_id=None; }
        }
    } else if component_mode.is_some() {
        selection_state.hovered_component_id = components.into_iter().next();
        selection_state.hovered_component_object_id = component_object;
        selection_state.hovered_component_mode = component_mode;
        selection_state.local_hover_id = None;
        selection_state.local_hover_granularity_id = None;
    } else {
        selection_state.hovered_component_id = None;
        selection_state.hovered_component_object_id = None;
        selection_state.hovered_component_mode = None;
        selection_state.local_hover_id = ids.into_iter().next();
        selection_state.local_hover_granularity_id = targets.first().and_then(|target| target.get("granularity")).and_then(|value| value.as_str()).map(str::to_string);
    }
}

/// 🖼️ Falls back to reference-image plane hit-testing when no mesh instance is under the
/// cursor — mirrors React's `hoveredId` covering both mesh instances and `"reference:"`-prefixed
/// reference planes (see `resolveWorldContextMenuTarget`'s reference tier).
///
/// 🕹️ `interactionHover` — empty `targets` clears the channel (see `HoverInput`/`next_hover`);
/// `domainId`/target id shape follow this surface's resolved (app-bound or `world`-fallback) domain.
#[cfg(any(test, feature = "testkit"))]
pub fn pick_hover_action(session_state: &mut WorldSessionState, camera_state: &mut WorldCameraState, selection_state: &mut WorldSelectionState, interaction_state: &mut WorldInteractionState, scene_state: &mut WorldSceneState, x: f32, y: f32, inner: Rect) -> Option<ActionDescriptor> {
    if session_state.active_utility == "surfaceBrush" {
        if let Some((object_id, position, normal)) = pick_surface_at(session_state, camera_state, scene_state, x, y, inner) {
            return Some(ActionDescriptor {
                controller_id: session_state.controller_id.clone(),
                action: "worldSurfaceHover".into(),
                args: Some(semio_framework::dsl_value!({
                    "surfaceId": session_state.surface_id,
                    "pane": session_state.surface_id,
                    "objectId": object_id,
                    "position": position,
                    "normal": normal,
                })),
            });
        }
        return Some(ActionDescriptor { controller_id: session_state.controller_id.clone(), action: "worldSurfaceLeave".into(), args: Some(semio_framework::dsl_value!({ "surfaceId": session_state.surface_id, "pane": session_state.surface_id })) });
    }
    if session_state.active_utility == "brush" || (session_state.active_utility == "select" && selection_state.granularity == "vertex") {
        let hit = pick_vortex_at(session_state, camera_state, scene_state, x, y, inner);
        if selection_state.hovered_vortex_id == hit {
            return None;
        }
        selection_state.hovered_vortex_id = hit.clone();
        return Some(ActionDescriptor { controller_id: session_state.controller_id.clone(), action: "worldVortexHover".into(), args: Some(semio_framework::dsl_value!({ "surfaceId": session_state.surface_id, "fullId": hit })) });
    }
    if component_mode_active(selection_state) {
        let hit = pick_component_at(session_state, camera_state, selection_state, scene_state, x, y, inner);
        if selection_state.hovered_component_id.as_deref() == hit.as_ref().map(|(_, id, _)| id.as_str()) && selection_state.hovered_component_object_id.as_deref() == hit.as_ref().map(|(_, _, object)| object.as_str()) && selection_state.hovered_component_mode.as_deref() == hit.as_ref().map(|(kind, _, _)| kind.as_str()) { return None; }
        selection_state.hovered_component_id = hit.as_ref().map(|(_, id, _)| id.clone());
        selection_state.hovered_component_object_id = hit.as_ref().map(|(_, _, object)| object.clone());
        selection_state.hovered_component_mode = hit.as_ref().map(|(kind, _, _)| kind.clone());
        selection_state.local_hover_id = None; selection_state.local_hover_granularity_id = None;
        let id = hit.as_ref().map(|(kind, group, object)| format!("{object}.{kind}.{group}"));
        let targets = world_interaction_targets(Some(&session_state.surface_id), id.as_deref(), hit.as_ref().map(|(kind, _, _)| kind.as_str()), session_state.bound_domain_id.is_some()).ok()?;
        return Some(ActionDescriptor { controller_id: session_state.controller_id.clone(), action: "interactionHover".into(), args: Some(semio_framework::dsl_value!({ "domainId": resolved_domain_id(session_state), "channel": "pointer", "targets": targets.as_str() })) });
    }
    let hit = pick_instance_at(session_state, camera_state, scene_state, x, y, inner).or_else(|| pick_reference_at(session_state, camera_state, scene_state, x, y, inner).map(|url| format!("reference:{url}")));
    let target = hit.as_deref().map(|id| (instance_interaction_id(interaction_state, id).to_string(), instance_interaction_granularity_id(session_state, interaction_state, id).to_string()));
    if selection_state.local_hover_id.as_deref() == target.as_ref().map(|(id, _)| id.as_str()) && selection_state.local_hover_granularity_id.as_deref() == target.as_ref().map(|(_, granularity)| granularity.as_str()) {
        return None;
    }
    selection_state.local_hover_id = target.as_ref().map(|(id, _)| id.clone());
    selection_state.local_hover_granularity_id = target.as_ref().map(|(_, granularity)| granularity.clone());
    let targets = world_interaction_targets(Some(&session_state.surface_id), target.as_ref().map(|(id, _)| id.as_str()), target.as_ref().map(|(_, kind)| kind.as_str()), session_state.bound_domain_id.is_some()).ok()?;
    Some(ActionDescriptor { controller_id: session_state.controller_id.clone(), action: "interactionHover".into(), args: Some(semio_framework::dsl_value!({ "domainId": resolved_domain_id(session_state), "channel": "pointer", "targets": targets.as_str() })) })
}

/// 🎯️ Geometry oracle publishes the same shared interaction command and target text.
#[cfg(any(test, feature = "testkit"))]
pub fn pick_select_action(session_state: &WorldSessionState, camera_state: &WorldCameraState, selection_state: &WorldSelectionState, interaction_state: &WorldInteractionState, scene_state: &WorldSceneState, x: f32, y: f32, inner: Rect, shift: bool, ctrl: bool) -> Option<ActionDescriptor> {
    let merge = world_merge_mode(shift, ctrl).wire_label();
    if selection_state.interaction_mode == "paint" {
        return None;
    }
    if component_mode_active(selection_state) {
        let hit = pick_component_at(session_state, camera_state, selection_state, scene_state, x, y, inner);
        let target = hit.as_ref().map(|(kind, group, object)| format!("{object}.{kind}.{group}"));
        let targets = world_interaction_targets(Some(&session_state.surface_id), target.as_deref(), hit.as_ref().map(|(kind, _, _)| kind.as_str()), session_state.bound_domain_id.is_some()).ok()?;
        return Some(ActionDescriptor {
            controller_id: session_state.controller_id.clone(), action: "interactionSelect".into(),
            args: Some(semio_framework::dsl_value!({ "domainId": resolved_domain_id(session_state), "targets": targets.as_str(), "merge": merge, "method": "pick" })),
        });
    }
    let hit = pick_instance_at(session_state, camera_state, scene_state, x, y, inner);
    let targets = world_interaction_targets(Some(&session_state.surface_id), hit.as_deref().map(|id| instance_interaction_id(interaction_state, id)), hit.as_deref().map(|id| instance_interaction_granularity_id(session_state, interaction_state, id)), session_state.bound_domain_id.is_some()).ok()?;
    Some(ActionDescriptor {
        controller_id: session_state.controller_id.clone(),
        action: "interactionSelect".into(),
        args: Some(semio_framework::dsl_value!({
            "domainId": resolved_domain_id(session_state),
            "targets": targets.as_str(),
            "merge": merge,
            "method": selection_method_wire_str(SelectionMethod::Pick),
        })),
    })
}

#[cfg(any(test, feature = "testkit"))]
pub fn merge_u32_ids(existing: &[String], incoming: &[String], merge: MergeMode) -> Vec<u32> {
    let parse = |ids: &[String]| -> Vec<u32> { ids.iter().filter_map(|id| id.parse().ok()).collect() };
    let existing_ids = parse(existing);
    let incoming_ids = parse(incoming);
    match merge {
        MergeMode::Additive => {
            let mut merged = existing_ids;
            for id in incoming_ids {
                if !merged.contains(&id) {
                    merged.push(id);
                }
            }
            merged
        }
        MergeMode::Invertive => {
            let mut merged = existing_ids;
            for id in incoming_ids {
                if let Some(index) = merged.iter().position(|entry| *entry == id) {
                    merged.remove(index);
                } else {
                    merged.push(id);
                }
            }
            merged
        }
        MergeMode::Subtractive => existing_ids.into_iter().filter(|id| !incoming_ids.contains(id)).collect(),
        MergeMode::Replace | MergeMode::Range => incoming_ids,
    }
}

/// 🕹️ ONE decoded `MergeMode` drives both the local merge and the wire word — see
/// `pick_select_action`'s identical rationale.
///
/// 🕹️ Marquee/lasso stays GEOMETRIC here — `screen_select_instances` above is the surface's own
/// hit-test, this just batches its raw hits into ONE `interactionSelect`; the merge/mode algebra is
/// the os-kernel `next_selection` machine's job, not this file's.
#[cfg(any(test, feature = "testkit"))]
pub fn marquee_select_action(session_state: &mut WorldSessionState, camera_state: &mut WorldCameraState, selection_state: &mut WorldSelectionState, interaction_state: &mut WorldInteractionState, scene_state: &mut WorldSceneState, inner: Rect, shift: bool, ctrl: bool) -> Option<ActionDescriptor> {
    if selection_state.marquee_points.len() < 2 {
        return None;
    }
    let camera = camera_state.orbit.to_camera();
    let aspect = (inner.w / inner.h.max(1.0)).max(0.1);
    let view_proj = ui_wgpu::wgpu::projection_spec_view_proj(&camera, camera_state.projection_spec, inner.w, inner.h);
    let (polygon, rectangle, crossing) = marquee_local_polygon(selection_state, inner);
    let (meshes, draws) = scene_geometry(scene_state);
    let ids = if component_mode_active(selection_state) {
        screen_select_components(&meshes, &draws, view_proj, camera_state.projection_spec, inner.w, inner.h, &polygon, rectangle, selection_state.granularity.as_str(), selection_state.active_object_id.as_deref(), crossing)
    } else {
        screen_select_instances(&meshes, &draws, view_proj, camera_state.projection_spec, inner.w, inner.h, &polygon, rectangle, crossing)
    };
    selection_state.marquee_points.clear();
    selection_state.marquee_preview_ids.clear();
    let merge = world_merge_mode(shift, ctrl);
    if component_mode_active(selection_state) {
        let object = selection_state.active_object_id.as_deref().or_else(|| scene_state.draws.iter().flat_map(|draw| &draw.instances).next().map(|instance| instance.id.as_str()))?;
        let targets: Vec<serde_json::Value> = ids.iter().map(|id| serde_json::json!({ "granularity": selection_state.granularity, "id": resolved_item_id(session_state, &format!("{object}.{}.{id}", selection_state.granularity)) })).collect();
        return Some(ActionDescriptor { controller_id: session_state.controller_id.clone(), action: "interactionSelect".into(), args: Some(semio_framework::dsl_value!({ "domainId": resolved_domain_id(session_state), "targets": serde_json::to_string(&targets).expect("test targets encode"), "merge": merge.wire_label(), "method": "rectangle" })) });
    }
    let mut seen = HashSet::new();
    let targets: Vec<serde_json::Value> = ids
        .iter()
        .filter_map(|id| {
            let granularity = instance_interaction_granularity_id(session_state, interaction_state, id);
            let target_id = instance_interaction_id(interaction_state, id);
            seen.insert((granularity.to_string(), target_id.to_string())).then(|| serde_json::json!({ "granularity": granularity, "id": resolved_item_id(session_state, target_id) }))
        })
        .collect();
    Some(ActionDescriptor {
        controller_id: session_state.controller_id.clone(),
        action: "interactionSelect".into(),
        args: Some(semio_framework::dsl_value!({
            "domainId": resolved_domain_id(session_state),
            "targets": serde_json::to_string(&targets).expect("test targets encode"),
            "merge": merge.wire_label(),
            "method": selection_method_wire_str(SelectionMethod::Rectangle),
        })),
    })
}

#[cfg(any(test, feature = "testkit"))]
fn gumball_drag_update(camera_state: &mut WorldCameraState, selection_state: &mut WorldSelectionState, gumball_state: &mut WorldGumballState, scene_state: &mut WorldSceneState, x: f32, y: f32, inner: Rect) {
    let Some(handle) = gumball_state.gumball_handle else {
        return;
    };
    let camera = camera_state.orbit.to_camera();
    let aspect = (inner.w / inner.h.max(1.0)).max(0.1);
    let local_x = x - inner.x;
    let local_y = y - inner.y;
    let (origin, dir) = ui_wgpu::wgpu::projection_spec_ray_from_screen(&camera, camera_state.projection_spec, local_x, local_y, inner.w, inner.h);
    let pivot = gumball_state.gumball_pivot;
    let eye = gumball_eye(&camera, pivot);
    reset_gumball_preview(gumball_state);
    if handle.is_translate() {
        if let Some(axis) = handle.axis_dir() {
            if let Some(current) = gumball_project_ray_onto_axis(origin, dir, pivot, axis, eye) {
                let delta = current - gumball_state.gumball_drag_anchor;
                gumball_state.gumball_preview_translate = axis.normalize().scale(delta);
            }
        } else if let Some(normal) = handle.plane_normal() {
            if let Some(current) = ray_plane_point(origin, dir, pivot, normal) {
                gumball_state.gumball_preview_translate = current.sub(gumball_state.gumball_drag_start_vec);
            }
        }
    } else if handle.is_rotate() {
        if let Some(normal) = handle.plane_normal() {
            if let Some(hit) = ray_plane_point(origin, dir, pivot, normal) {
                let current = hit.sub(pivot);
                if current.length() > 1e-4 && gumball_state.gumball_drag_start_vec.length() > 1e-4 {
                    gumball_state.gumball_preview_angle = axis_rotate_angle(gumball_state.gumball_drag_start_vec, current, normal);
                }
            }
        }
    } else if handle.is_scale() {
        if let Some(axis) = handle.axis_dir() {
            if let Some(current) = gumball_project_ray_onto_axis(origin, dir, pivot, axis, eye) {
                let factor = if gumball_state.gumball_drag_anchor.abs() > 1e-4 { current / gumball_state.gumball_drag_anchor } else { 1.0 };
                let factor = factor.clamp(0.05, 20.0);
                match handle {
                    GumballHandle::ScaleX => gumball_state.gumball_preview_scale.x = factor,
                    GumballHandle::ScaleY => gumball_state.gumball_preview_scale.y = factor,
                    GumballHandle::ScaleZ => gumball_state.gumball_preview_scale.z = factor,
                    _ => {}
                }
            }
        }
    }
    apply_gumball_preview(selection_state, gumball_state, scene_state);
}

#[cfg(any(test, feature = "testkit"))]
pub fn start_gumball_drag(camera_state: &mut WorldCameraState, selection_state: &mut WorldSelectionState, gumball_state: &mut WorldGumballState, scene_state: &mut WorldSceneState, handle: GumballHandle, x: f32, y: f32, inner: Rect) {
    let Some(pivot) = selection_centroid(selection_state, gumball_state, scene_state) else {
        return;
    };
    let camera = camera_state.orbit.to_camera();
    let aspect = (inner.w / inner.h.max(1.0)).max(0.1);
    let local_x = x - inner.x;
    let local_y = y - inner.y;
    let (origin, dir) = ui_wgpu::wgpu::projection_spec_ray_from_screen(&camera, camera_state.projection_spec, local_x, local_y, inner.w, inner.h);
    let eye = gumball_eye(&camera, pivot);
    gumball_state.gumball_handle = Some(handle);
    gumball_state.gumball_pivot = pivot;
    reset_gumball_preview(gumball_state);
    if let Some(axis) = handle.axis_dir() {
        gumball_state.gumball_drag_anchor = gumball_project_ray_onto_axis(origin, dir, pivot, axis, eye).unwrap_or(0.0);
    } else if let Some(normal) = handle.plane_normal() {
        gumball_state.gumball_drag_start_vec = ray_plane_point(origin, dir, pivot, normal).unwrap_or(pivot);
        if handle.is_rotate() {
            gumball_state.gumball_drag_start_vec = gumball_state.gumball_drag_start_vec.sub(pivot);
        }
        gumball_state.gumball_drag_anchor = 0.0;
    }
}

#[cfg(any(test, feature = "testkit"))]
pub fn pick_component_at(session_state: &WorldSessionState, camera_state: &WorldCameraState, selection_state: &WorldSelectionState, scene_state: &WorldSceneState, x: f32, y: f32, _inner: Rect) -> Option<(String, String, String)> {
    let (local_x, local_y, rect) = pointer_in_pick_rect(session_state, x, y)?;
    let camera = camera_state.orbit.to_camera();
    let aspect = (rect.w / rect.h.max(1.0)).max(0.1);
    let view_proj = ui_wgpu::wgpu::projection_spec_view_proj(&camera, camera_state.projection_spec, rect.w, rect.h);
    let granularity = selection_state.granularity.as_str();
    match granularity {
        "vertex" => {
            let mut best: Option<(f32, String, String)> = None;
            for draw in &scene_state.draws {
                let Some(&mesh) = scene_state.meshes.get(&draw.mesh_key) else {
                    continue;
                };
                let Ok(schema) = mesh.schema() else { continue };
                for instance in &draw.instances {
                    if !pick_targets_instance(selection_state, &instance.id) {
                        continue;
                    }
                    for vertex_index in 0..schema.vertices {
                        let Ok(point) = mesh.vec3(Mesh3dField::Positions, vertex_index) else { continue };
                        let world = instance.model.transform_point(Vec3::new(point[0], point[1], point[2]));
                        let Some(screen) = ui_wgpu::wgpu::projection_spec_project_point(view_proj, camera_state.projection_spec, world, rect.w, rect.h) else {
                            continue;
                        };
                        let dx = screen[0] - local_x;
                        let dy = screen[1] - local_y;
                        let dist = (dx * dx + dy * dy).sqrt();
                        if dist <= PICK_VERTEX_SCREEN_PX && best.as_ref().is_none_or(|(best_dist, _, _)| dist < *best_dist) {
                            let group = world_mesh_component_id(mesh, Mesh3dField::VertexIds, vertex_index);
                    if group == u32::MAX { continue; }
                    let id = group.to_string();
                            best = Some((dist, id, instance.id.clone()));
                        }
                    }
                }
            }
            return best.map(|(_, id, object_id)| (granularity.to_string(), id, object_id));
        }
        "edge" => {
            let (origin, dir) = ui_wgpu::wgpu::projection_spec_ray_from_screen(&camera, camera_state.projection_spec, local_x, local_y, rect.w, rect.h);
            let mut best: Option<(f32, f32, String, String)> = None;
            for draw in &scene_state.draws {
                let Some(&mesh) = scene_state.meshes.get(&draw.mesh_key) else {
                    continue;
                };
                let Ok(schema) = mesh.schema() else { continue };
                if schema.edges == 0 || schema.edge_ids != schema.edges {
                    continue;
                }
                for instance in &draw.instances {
                    if !pick_targets_instance(selection_state, &instance.id) {
                        continue;
                    }
                    for edge_index in 0..schema.edges {
                        let Ok(edge) = mesh.edge(edge_index) else { continue };
                        let a = instance.model.transform_point(Vec3::new(edge[0][0], edge[0][1], edge[0][2]));
                        let b = instance.model.transform_point(Vec3::new(edge[1][0], edge[1][1], edge[1][2]));
                        let Some(segment) = ui_wgpu::wgpu::projection_spec_project_segment(view_proj, camera_state.projection_spec, a, b, rect.w, rect.h) else { continue; };
                        let [screen_a, screen_b] = segment.screen;
                        let screen_dist = ui_wgpu::wgpu::screen_segment_distance(local_x, local_y, screen_a[0], screen_a[1], screen_b[0], screen_b[1]);
                        if screen_dist > PICK_EDGE_SCREEN_PX {
                            continue;
                        }
                        let Some(closest) = ui_wgpu::wgpu::ray_segment_closest(origin, dir, segment.points[0], segment.points[1]) else { continue };
                        let ray_dist = closest.distance;
                        let depth = closest.segment_point.sub(origin).dot(dir);
                        if !depth.is_finite() || depth < 0.0 { continue; }
                        let better = match &best {
                            None => true,
                            Some((best_ray, best_depth, _, _)) => depth < *best_depth - 1e-4 || ((depth - *best_depth).abs() <= 1e-4 && ray_dist < *best_ray),
                        };
                        if better {
                            let id = world_mesh_component_id(mesh, Mesh3dField::EdgeIds, edge_index).to_string();
                            best = Some((ray_dist, depth, id, instance.id.clone()));
                        }
                    }
                }
            }
            return best.map(|(_, _, id, object_id)| (granularity.to_string(), id, object_id));
        }
        "face" => {
            let (origin, dir) = ui_wgpu::wgpu::projection_spec_ray_from_screen(&camera, camera_state.projection_spec, local_x, local_y, rect.w, rect.h);
            let mut best: Option<(f32, String, String)> = None;
            for draw in &scene_state.draws {
                let Some(&mesh) = scene_state.meshes.get(&draw.mesh_key) else {
                    continue;
                };
                for instance in &draw.instances {
                    if !pick_targets_instance(selection_state, &instance.id) {
                        continue;
                    }
                    let Some(hit) = world_test_mesh_query(||ray_pick_mesh_detail(origin, dir, mesh, instance)).expect("finite caller mesh query admission") else {
                        continue;
                    };
                    if best.as_ref().is_none_or(|(best_depth, _, _)| hit.distance < *best_depth) {
                        let id = mesh_face_id(mesh, u32::try_from(hit.triangle_index).ok()?);
                        best = Some((hit.distance, id, instance.id.clone()));
                    }
                }
            }
            return best.map(|(_, id, object_id)| (granularity.to_string(), id, object_id));
        }
        _ => {}
    }
    None
}

#[cfg(any(test, feature = "testkit"))]
pub fn pick_paint_hit(session_state: &WorldSessionState, camera_state: &WorldCameraState, scene_state: &WorldSceneState, x: f32, y: f32, _inner: Rect) -> Option<(String, f32, f32)> {
    let (local_x, local_y, viewport) = pointer_in_pick_rect(session_state, x, y)?;
    let camera = camera_state.orbit.to_camera();
    let aspect = (viewport.w / viewport.h.max(1.0)).max(0.1);
    let (origin, dir) = ui_wgpu::wgpu::projection_spec_ray_from_screen(&camera, camera_state.projection_spec, local_x, local_y, viewport.w, viewport.h);
    let mut best: Option<(f32, String, f32, f32)> = None;
    for draw in &scene_state.draws {
        let Some(&mesh) = scene_state.meshes.get(&draw.mesh_key) else {
            continue;
        };
        for instance in &draw.instances {
            if let Some(hit) = world_test_mesh_query(||ray_pick_mesh_detail(origin, dir, mesh, instance)).expect("finite caller mesh query admission") {
                if let Some((u, v)) = world_test_mesh_query(||interpolate_mesh_uv(mesh, hit.triangle_index, hit.bary_u, hit.bary_v)).expect("finite caller mesh query admission") {
                    if best.as_ref().is_none_or(|(best_dist, _, _, _)| hit.distance < *best_dist) {
                        best = Some((hit.distance, instance.id.clone(), u, v));
                    }
                }
            }
        }
    }
    best.map(|(_, object_id, u, v)| (object_id, u, v))
}

/// 🖱️ Screen-space marquee path for the selection rectangle overlay. Production pointer routing
/// retains the gesture on `WorldInteractionAuthority::marquee`; the legacy `marquee_points` /
/// `marquee_active` fields remain for the oracle tests only.
pub fn world_marquee_overlay_points(selection_state: &WorldSelectionState, authority_state: &WorldAuthorityState) -> Option<Vec<[f32; 2]>> {
    if let Some(marquee) = authority_state.interaction_authority.as_ref()?.marquee.as_ref() {
        if !marquee.retiring && marquee.len >= 2 {
            let mut points = Vec::with_capacity(usize::from(marquee.len));
            for index in 0..usize::from(marquee.len) {
                points.push(marquee.points[index].expect("marquee gesture point"));
            }
            return Some(points);
        }
    }
    if selection_state.marquee_active && selection_state.marquee_points.len() >= 2 {
        return Some(selection_state.marquee_points.clone());
    }
    None
}

#[cfg(any(test, feature = "testkit"))]
fn marquee_local_polygon(selection_state: &WorldSelectionState, rect: Rect) -> (Vec<[f32; 2]>, bool, bool) {
    let rectangle = selection_state.selection_method != "lasso";
    let crossing = marquee_is_crossing_from_path(&selection_state.marquee_points, !rectangle);
    let global: Vec<[f32; 2]> = if rectangle {
        let start = selection_state.marquee_points[0];
        let end = selection_state.marquee_points[selection_state.marquee_points.len() - 1];
        vec![start, end]
    } else {
        selection_state.marquee_points.clone()
    };
    let local = global.iter().map(|point| [point[0] - rect.x, point[1] - rect.y]).collect();
    (local, rectangle, crossing)
}

#[cfg(any(test, feature = "testkit"))]
pub fn update_marquee_preview(camera_state: &mut WorldCameraState, selection_state: &mut WorldSelectionState, scene_state: &mut WorldSceneState, inner: Rect) {
    if selection_state.marquee_points.len() < 2 {
        selection_state.marquee_preview_ids.clear();
        return;
    }
    let camera = camera_state.orbit.to_camera();
    let aspect = (inner.w / inner.h.max(1.0)).max(0.1);
    let view_proj = ui_wgpu::wgpu::projection_spec_view_proj(&camera, camera_state.projection_spec, inner.w, inner.h);
    let (polygon, rectangle, crossing) = marquee_local_polygon(selection_state, inner);
    let (meshes, draws) = scene_geometry(scene_state);
    selection_state.marquee_preview_ids = if component_mode_active(selection_state) {
        screen_select_components(&meshes, &draws, view_proj, camera_state.projection_spec, inner.w, inner.h, &polygon, rectangle, selection_state.granularity.as_str(), selection_state.active_object_id.as_deref(), crossing)
    } else {
        screen_select_instances(&meshes, &draws, view_proj, camera_state.projection_spec, inner.w, inner.h, &polygon, rectangle, crossing)
    };
}

#[cfg(any(test, feature = "testkit"))]
pub fn scene_geometry(scene_state: &WorldSceneState) -> (HashMap<String, Mesh3dLease>, Vec<SceneDraw3d>) {
    (scene_state.meshes.iter().map(|(id, mesh)| (id.clone(), *mesh)).collect(), scene_state.draws.iter().cloned().collect())
}

#[cfg(any(test, feature = "testkit"))]
pub fn pick_instance_at(session_state: &WorldSessionState, camera_state: &WorldCameraState, scene_state: &WorldSceneState, x: f32, y: f32, _inner: Rect) -> Option<String> {
    let (local_x, local_y, viewport) = pointer_in_pick_rect(session_state, x, y)?;
    let camera = camera_state.orbit.to_camera();
    let aspect = (viewport.w / viewport.h.max(1.0)).max(0.1);
    let (origin, dir) = ui_wgpu::wgpu::projection_spec_ray_from_screen(&camera, camera_state.projection_spec, local_x, local_y, viewport.w, viewport.h);
    let mut best: Option<(f32, String)> = None;
    for draw in &scene_state.draws {
        let Some(&mesh) = scene_state.meshes.get(&draw.mesh_key) else {
            continue;
        };
        for instance in &draw.instances {
            if let Some(distance) = world_test_mesh_query(||ray_pick_instance(origin, dir, mesh, instance)).expect("finite caller mesh query admission") {
                if best.as_ref().is_none_or(|(best_distance, _)| distance < *best_distance) {
                    best = Some((distance, instance.id.clone()));
                }
            }
        }
    }
    best.map(|(_, id)| id)
}

#[cfg(any(test, feature = "testkit"))]
pub fn pick_surface_at(session_state: &WorldSessionState, camera_state: &WorldCameraState, scene_state: &WorldSceneState, x: f32, y: f32, _inner: Rect) -> Option<(String, [f64; 3], [f64; 3])> {
    let (local_x, local_y, viewport) = pointer_in_pick_rect(session_state, x, y)?;
    let camera = camera_state.orbit.to_camera();
    let aspect = (viewport.w / viewport.h.max(1.0)).max(0.1);
    let (origin, dir) = ui_wgpu::wgpu::projection_spec_ray_from_screen(&camera, camera_state.projection_spec, local_x, local_y, viewport.w, viewport.h);
    let mut best: Option<(f32, String, Vec3, Vec3)> = None;
    for draw in &scene_state.draws {
        let Some(&mesh) = scene_state.meshes.get(&draw.mesh_key) else {
            continue;
        };
        for instance in &draw.instances {
            let Some(hit) = world_test_mesh_query(||ray_pick_mesh_detail(origin, dir, mesh, instance)).expect("finite caller mesh query admission") else {
                continue;
            };
            if best.as_ref().is_none_or(|(best_distance, _, _, _)| hit.distance < *best_distance) {
                best = Some((hit.distance, instance.id.clone(), hit.point, hit.normal));
            }
        }
    }
    best.map(|(_, id, point, normal)| (id, [point.x as f64, point.y as f64, point.z as f64], [normal.x as f64, normal.y as f64, normal.z as f64]))
}

//#region VortexArrow
const VORTEX_ARROW_SHAFT_MESH: &str = "cylinder";

const VORTEX_ARROW_HEAD_MESH: &str = "cone";

struct VortexArrowLayout {
    point_radius: f32,
    shaft_radius: f32,
    shaft_length: f32,
    head_length: f32,
    shaft_center: [f32; 3],
    head_base: [f32; 3],
    pub rotation: [f32; 4],
}

fn quat_normalize_f32(quat: [f32; 4]) -> [f32; 4] {
    let len = (quat[0] * quat[0] + quat[1] * quat[1] + quat[2] * quat[2] + quat[3] * quat[3]).sqrt();
    if len < 1e-9 {
        [0.0, 0.0, 0.0, 1.0]
    } else {
        [quat[0] / len, quat[1] / len, quat[2] / len, quat[3] / len]
    }
}

fn quat_from_unit_vectors(from: Vec3, to: Vec3) -> [f32; 4] {
    let from = from.normalize();
    let to = to.normalize();
    let r = from.dot(to) + 1.0;
    let quat = if r < 0.000_001 {
        if from.x.abs() > from.z.abs() {
            [-from.y, from.x, 0.0, r]
        } else {
            [0.0, -from.z, from.y, r]
        }
    } else {
        let cross = from.cross(to);
        [cross.x, cross.y, cross.z, r]
    };
    quat_normalize_f32(quat)
}

fn vortex_unit_direction(direction: Option<[f64; 3]>) -> Vec3 {
    let dir = direction.map_or(Vec3::new(0.0, 0.0, -1.0), |value| Vec3::new(value[0] as f32, value[1] as f32, value[2] as f32));
    if dir.dot(dir) < 1e-12 {
        Vec3::new(0.0, 0.0, -1.0)
    } else {
        dir.normalize()
    }
}

fn vortex_arrow_layout(position: [f64; 3], direction: Option<[f64; 3]>, radius: f32, display_direction: Option<&str>) -> VortexArrowLayout {
    let dir = vortex_unit_direction(direction);
    let pos = Vec3::new(position[0] as f32, position[1] as f32, position[2] as f32);
    let arrow_length = radius;
    let head_length = radius * 0.28;
    let shaft_length = (arrow_length - head_length).max(radius * 0.2);
    let shaft_radius = radius * 0.055;
    let point_radius = radius * 0.18;
    let outward = !display_direction.is_some_and(|mode| mode == "inwards");
    let rotation = quat_from_unit_vectors(Vec3::new(0.0, 1.0, 0.0), dir);
    let (shaft_center, head_base) = if outward { (pos.add(dir.scale(shaft_length * 0.5)), pos.add(dir.scale(shaft_length))) } else { (pos.sub(dir.scale(head_length + shaft_length * 0.5)), pos.sub(dir.scale(head_length))) };
    VortexArrowLayout { point_radius, shaft_radius, shaft_length, head_length, shaft_center: shaft_center.to_array(), head_base: head_base.to_array(), rotation }
}

fn ensure_primitive_mesh(session_state: &mut WorldSessionState, authority_state: &mut WorldAuthorityState, retirement_state: &mut WorldRetirementState, scene_state: &mut WorldSceneState, mesh_key: &str) {
    if scene_state.meshes.contains_key(mesh_key) {
        return;
    }
    begin_world_placeholder_mesh(session_state, authority_state, retirement_state, scene_state, mesh_key, WorldPlaceholderKind::resolve(mesh_key));
}

pub fn append_vortex_arrow_draws(session_state: &mut WorldSessionState, selection_state: &mut WorldSelectionState, authority_state: &mut WorldAuthorityState, retirement_state: &mut WorldRetirementState, scene_state: &mut WorldSceneState, gpu: &mut World3dBuildContext, extra_draws: &mut Vec<SceneDraw3d>) {
    if scene_state.vortices.is_empty() {
        return;
    }
    let mut point_instances = Vec::new();
    let mut shaft_instances = Vec::new();
    let mut head_instances = Vec::new();
    for vortex in &scene_state.vortices {
        let position = vortex.position.unwrap_or([0.0, 0.0, 0.0]);
        let radius = vortex.radius.unwrap_or(0.36) as f32;
        let layout = vortex_arrow_layout(position, vortex.direction, radius, vortex.display_direction.as_deref());
        let color = parse_color(vortex.color.as_deref().unwrap_or("#38bdf8"));
        let hovered = selection_state.hovered_vortex_id.as_deref() == Some(vortex.full_id.as_str());
        let id = vortex.full_id.clone();
        point_instances.push(Instance3d { component_source: None,
            id: format!("{id}:point"),
            model: Instance3d::model_from_trs([position[0] as f32, position[1] as f32, position[2] as f32], [0.0, 0.0, 0.0, 1.0], [layout.point_radius, layout.point_radius, layout.point_radius]),
            color,
            selected: false,
            hovered,
         material: Default::default() });
        shaft_instances.push(Instance3d { component_source: None,
            id: format!("{id}:shaft"),
            model: Instance3d::model_from_trs(layout.shaft_center, layout.rotation, [layout.shaft_radius * 2.0, layout.shaft_length, layout.shaft_radius * 2.0]),
            color,
            selected: false,
            hovered,
         material: Default::default() });
        head_instances.push(Instance3d { component_source: None, id: format!("{id}:head"), model: Instance3d::model_from_trs(layout.head_base, layout.rotation, [layout.shaft_radius * 3.6, layout.head_length, layout.shaft_radius * 3.6]), color, selected: false, hovered , material: Default::default() });
    }
    ensure_primitive_mesh(session_state, authority_state, retirement_state, scene_state, "vortex-marker");
    ensure_primitive_mesh(session_state, authority_state, retirement_state, scene_state, VORTEX_ARROW_SHAFT_MESH);
    ensure_primitive_mesh(session_state, authority_state, retirement_state, scene_state, VORTEX_ARROW_HEAD_MESH);
    for (mesh_key, instances) in [("vortex-marker", point_instances), (VORTEX_ARROW_SHAFT_MESH, shaft_instances), (VORTEX_ARROW_HEAD_MESH, head_instances)] {
        let mesh_version = live_world3d_mesh_version(scene_state, mesh_key);
        if let Some(mesh) = scene_state.meshes.get(mesh_key).copied() {
            gpu.ensure_mesh(mesh_key, mesh_version, mesh);
            extra_draws.push(SceneDraw3d { mesh_key: mesh_key.into(), mesh_version, instances, shadow_role: Default::default() });
        }
    }
}
//#endregion VortexArrow

#[cfg(any(test, feature = "testkit"))]
pub fn pick_vortex_at(session_state: &WorldSessionState, camera_state: &WorldCameraState, scene_state: &WorldSceneState, x: f32, y: f32, _inner: Rect) -> Option<String> {
    let (local_x, local_y, viewport) = pointer_in_pick_rect(session_state, x, y)?;
    let camera = camera_state.orbit.to_camera();
    let aspect = (viewport.w / viewport.h.max(1.0)).max(0.1);
    let (origin, dir) = ui_wgpu::wgpu::projection_spec_ray_from_screen(&camera, camera_state.projection_spec, local_x, local_y, viewport.w, viewport.h);
    let mut best: Option<(f32, String)> = None;
    for vortex in &scene_state.vortices {
        let position = vortex.position.unwrap_or([0.0, 0.0, 0.0]);
        let radius = vortex.radius.unwrap_or(0.36) as f32;
        let center = Vec3::new(position[0] as f32, position[1] as f32, position[2] as f32);
        let min = [center.x - radius, center.y - radius, center.z - radius];
        let max = [center.x + radius, center.y + radius, center.z + radius];
        if let Some(distance) = ray_aabb_slab(origin, dir, min, max) {
            if best.as_ref().is_none_or(|(best_distance, _)| distance < *best_distance) {
                best = Some((distance, vortex.full_id.clone()));
            }
        }
    }
    best.map(|(_, id)| id)
}

/// 🖼️ Ray-vs-quad hit test against visible reference-image planes, returning the closest hit
/// reference's authored `id`. References are flat rectangles lying in the local XY plane (normal +Z,
/// width along X, height along Y) centered at `origin` — matching this renderer's Z-up convention.
#[cfg(any(test, feature = "testkit"))]
fn pick_reference_at(session_state: &WorldSessionState, camera_state: &WorldCameraState, scene_state: &WorldSceneState, x: f32, y: f32, _inner: Rect) -> Option<String> {
    let (local_x, local_y, viewport) = pointer_in_pick_rect(session_state, x, y)?;
    let camera = camera_state.orbit.to_camera();
    let aspect = (viewport.w / viewport.h.max(1.0)).max(0.1);
    let (origin, dir) = ui_wgpu::wgpu::projection_spec_ray_from_screen(&camera, camera_state.projection_spec, local_x, local_y, viewport.w, viewport.h);
    let plane_normal = Vec3::new(0.0, 0.0, 1.0);
    let mut best: Option<(f32, String)> = None;
    for reference in &scene_state.references {
        if reference.hidden.unwrap_or(false) || reference.locked.unwrap_or(false) {
            continue;
        }
        let Some(url) = reference.url.as_deref() else {
            continue;
        };
        let plane_position = reference.origin.unwrap_or([0.0, 0.0, 0.0]);
        let plane_origin = Vec3::new(plane_position[0] as f32, plane_position[1] as f32, plane_position[2] as f32);
        let width = world_reference_width(reference);
        let image_aspect = reference_image_aspect(scene_state, url);
        let height = width / image_aspect.max(0.01);
        let Some(hit) = ray_plane_point(origin, dir, plane_origin, plane_normal) else {
            continue;
        };
        let offset = hit.sub(plane_origin);
        if offset.x.abs() > width * 0.5 || offset.y.abs() > height * 0.5 {
            continue;
        }
        let distance = origin.sub(hit).length();
        if best.as_ref().is_none_or(|(best_distance, _)| distance < *best_distance) {
            best = Some((distance, reference.id.as_deref().unwrap_or(url).to_string()));
        }
    }
    best.map(|(_, id)| id)
}

/// 📍️ One instance's own world ORIGIN — the translation column of its model matrix. Production since
/// the relocate gesture landed: `world3d_begin_relocate_drag` needs the grabbed object's origin to
/// apply React's `origin + (to - from)` travel (`world3dRelocateDispatchArgsV1`).
fn object_world_position(scene_state: &WorldSceneState, object_id: &str) -> Option<[f32; 3]> {
    for draw in &scene_state.draws {
        for instance in &draw.instances {
            if instance.id == object_id {
                let translation = instance.model.cols[3];
                return Some([translation[0], translation[1], translation[2]]);
            }
        }
    }
    None
}

#[cfg(any(test, feature = "testkit"))]
fn update_dragged_instance_position(scene_state: &mut WorldSceneState, object_id: &str, position: [f32; 3]) {
    for draw in &mut scene_state.draws {
        for instance in &mut draw.instances {
            if instance.id == object_id {
                instance.model.cols[3] = [position[0], position[1], position[2], 1.0];
            }
        }
    }
}

/// 🎯️ Intersects the camera ray through a shell client point with the world Z=0 ground plane.
pub fn world3d_ground_plane_pick(session_state: &WorldSessionState, camera_state: &WorldCameraState, x: f32, y: f32, plane_z: f32) -> Option<[f32; 3]> {
    let (local_x, local_y, viewport) = pointer_in_pick_rect(session_state, x, y)?;
    let camera = camera_state.orbit.to_camera();
    let (origin, dir) = ui_wgpu::wgpu::projection_spec_ray_from_screen(&camera, camera_state.projection_spec, local_x, local_y, viewport.w, viewport.h);
    if dir.z.abs() < 1e-5 {
        return None;
    }
    let t = (plane_z - origin.z) / dir.z;
    if t < 0.0 {
        return None;
    }
    let hit = origin.add(dir.scale(t));
    Some([hit.x, hit.y, hit.z])
}

/// 📏️ `GUMBALL_TRANSFORM_EPSILON` (`🌐️World3dHost/🟦️.tsx:2496`) — a drag that lands within this of
/// where it started writes no document edit.
const WORLD_RELOCATE_EPSILON: f64 = 1e-6;

/// 🚚️ Which object a relocate press grabs — React `world3dRelocateDragTargetV1`
/// (`🌐️World3dHost/🟦️.tsx:4726`). A press on empty canvas relocates the first selected object; a press
/// with no selection relocates what it landed on; a press INSIDE a selection relocates that object,
/// and a press on something outside the selection relocates nothing (it is a selection gesture, and
/// `clearSelection` stays the way to drop a selection).
pub fn world3d_relocate_drag_target(pressed_id: Option<&str>, selected_ids: &[String]) -> Option<String> {
    let Some(pressed_id) = pressed_id.filter(|id| !id.is_empty()) else {
        return selected_ids.first().cloned();
    };
    if selected_ids.is_empty() {
        return Some(pressed_id.to_string());
    }
    selected_ids.iter().any(|id| id == pressed_id).then(|| pressed_id.to_string())
}

/// 🚚️ The ONE `worldRelocate` payload a finished relocate drag commits — React
/// `world3dRelocateDispatchArgsV1` (`🌐️World3dHost/🟦️.tsx:4738`). The ground travel `from → to` is
/// added to the grabbed object's own origin and snapped exactly like a catalogue drop; a drag that
/// lands within [`WORLD_RELOCATE_EPSILON`] of where it started answers `None`, so a click without
/// travel never writes a document edit.
pub fn world3d_relocate_dispatch_args(session: &World3dRelocateSession, to: [f32; 3], grid_snap_enabled: bool, grid_factor: f64) -> Option<[f64; 3]> {
    let moved = [session.origin[0] + (to[0] - session.from[0]), session.origin[1] + (to[1] - session.from[1]), session.origin[2] + (to[2] - session.from[2])];
    let position = snap_world_point_to_grid(moved, grid_snap_enabled, grid_factor);
    let unchanged = (0..3).all(|axis| (position[axis] - f64::from(session.origin[axis])).abs() < WORLD_RELOCATE_EPSILON);
    (!unchanged).then_some(position)
}

/// 🚚️ Whether this surface is in React's `relocateMode` — `activeUtility === "worldRelocate"`
/// (`🌐️World3dHost/🟦️.tsx:5656`).
pub fn world3d_relocate_mode(session_state: &WorldSessionState) -> bool {
    session_state.active_utility == "worldRelocate"
}

/// 🚚️ Arms a relocate drag at a primary press. The pressed object is the one this surface's own hover
/// authority last resolved (`local_hover_id`) — the wgpu stand-in for the r3f pointer event's own
/// `object` target, which is the same object by construction since the hover cursor resolves every
/// move before the press lands. Answers `false` when the press grabs nothing, which leaves the press
/// to the ordinary selection route.
pub fn world3d_begin_relocate_drag(session_state: &mut WorldSessionState, camera_state: &mut WorldCameraState, selection_state: &mut WorldSelectionState, gumball_state: &mut WorldGumballState, scene_state: &mut WorldSceneState, x: f32, y: f32) -> bool {
    if !world3d_relocate_mode(session_state) {
        return false;
    }
    let Some(object_id) = world3d_relocate_drag_target(selection_state.local_hover_id.as_deref(), &selection_state.selected_ids) else {
        return false;
    };
    let Some(origin) = object_world_position(scene_state, &object_id) else {
        return false;
    };
    let Some(from) = world3d_ground_plane_pick(session_state, camera_state, x, y, origin[2]) else {
        return false;
    };
    gumball_state.relocate = Some(World3dRelocateSession { object_id, origin, from });
    true
}

/// 🚚️ Moves the relocate ghost. React previews the travel with the SAME catalogue-drop ghost a drop
/// from the catalogue uses (`setWorldCatalogueDropPreview`, `🌐️World3dHost/🟦️.tsx:6851`) and
/// dispatches nothing until the release.
pub fn world3d_update_relocate_drag(session_state: &mut WorldSessionState, camera_state: &mut WorldCameraState, gumball_state: &mut WorldGumballState, scene_state: &mut WorldSceneState, x: f32, y: f32) -> bool {
    let Some(session) = gumball_state.relocate.clone() else {
        return false;
    };
    let origin = world3d_ground_plane_pick(session_state, camera_state, x, y, session.origin[2]).and_then(|to| world3d_relocate_dispatch_args(&session, to, scene_state.lod.grid_snap_enabled, scene_state.lod.grid_factor)).unwrap_or([
        f64::from(session.origin[0]),
        f64::from(session.origin[1]),
        f64::from(session.origin[2]),
    ]);
    let object_kind = gumball_state.relocate.as_ref().map(|session| session.object_id.clone()).unwrap_or_default();
    gumball_state.catalogue_drop_preview = Some(WorldCatalogueDropPreviewRecord { object_kind, mesh_url: world3d_mesh_url_for_object(scene_state, &session.object_id), origin });
    true
}

/// 🚚️ Closes the relocate drag and answers the position to commit, or `None` for an abort or a drag
/// that never travelled. `to` is `None` for Escape and for a cancelled pointer — React's
/// `endRelocateDrag(null)`, which drops the ghost and dispatches nothing.
pub fn world3d_end_relocate_drag(session_state: &mut WorldSessionState, camera_state: &mut WorldCameraState, gumball_state: &mut WorldGumballState, scene_state: &mut WorldSceneState, to: Option<(f32, f32)>) -> Option<(String, [f64; 3])> {
    let session = gumball_state.relocate.take()?;
    gumball_state.catalogue_drop_preview = None;
    let (x, y) = to?;
    let point = world3d_ground_plane_pick(session_state, camera_state, x, y, session.origin[2])?;
    let position = world3d_relocate_dispatch_args(&session, point, scene_state.lod.grid_snap_enabled, scene_state.lod.grid_factor)?;
    Some((session.object_id, position))
}

/// ⎋️ Escape drops a live relocate drag and dispatches nothing — React binds it as a CAPTURE-phase
/// `keydown` while `relocateMode` is on (`🌐️World3dHost/🟦️.tsx:6875`). Answers whether a drag was
/// actually cancelled, so the caller can stop the key there exactly as React calls
/// `event.stopPropagation()` only when `endRelocateDrag` returns true.
pub fn world3d_cancel_relocate_drag(session_state: &mut WorldSessionState, camera_state: &mut WorldCameraState, gumball_state: &mut WorldGumballState, scene_state: &mut WorldSceneState) -> bool {
    if gumball_state.relocate.is_none() {
        return false;
    }
    world3d_end_relocate_drag(session_state, camera_state, gumball_state, scene_state, None);
    true
}

/// 🚚️ Whether a relocate drag is live on this surface.
pub fn world3d_relocate_active(gumball_state: &WorldGumballState) -> bool {
    gumball_state.relocate.is_some()
}

/// 🥽️ The mesh url the relocate ghost should draw with — the grabbed instance's own mesh, so the
/// preview is the object being moved rather than a stand-in box.
fn world3d_mesh_url_for_object(scene_state: &WorldSceneState, object_id: &str) -> Option<String> {
    for draw in &scene_state.draws {
        if draw.instances.iter().any(|instance| instance.id == object_id) {
            return scene_state.mesh_source_urls.get(&draw.mesh_key).cloned();
        }
    }
    None
}

/// 🚚️ The `worldRelocate` plan a finished relocate drag publishes.
pub(crate) fn plan_world3d_relocate(session_state: &WorldSessionState, generation: u64, object_id: &str, position: [f64; 3]) -> Option<WorldInteractionPlan> {
    let mut plan = WorldInteractionPlan::new(session_state.interaction_revision, generation);
    let controller = plan.push_string(&session_state.controller_id)?;
    let surface = plan.push_string(&session_state.surface_id)?;
    let object = plan.push_string(object_id)?;
    let action = WorldFlatAction { kind: WorldFlatActionKind::Relocate, strings: [Some(controller), Some(surface), Some(object), None, None, None, None, None], numbers: [position[0], position[1], position[2], 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0] };
    plan.push_action(action).then_some(plan)
}
//#endregion WorldRelocateGesture

fn snap_world_point_to_grid(point: [f32; 3], grid_snap_enabled: bool, grid_factor: f64) -> [f64; 3] {
    if !grid_snap_enabled || grid_factor <= 0.0 {
        return [point[0] as f64, point[1] as f64, point[2] as f64];
    }
    let snap = |value: f32| (value as f64 / grid_factor).round() * grid_factor;
    [snap(point[0]), snap(point[1]), snap(point[2])]
}

/// 👻️ Updates the live catalogue-drop ghost for a wgpu `World3d` pane under the pointer.
pub fn world3d_update_catalogue_drop_preview(session_state: &mut WorldSessionState, camera_state: &mut WorldCameraState, gumball_state: &mut WorldGumballState, scene_state: &mut WorldSceneState, client_x: f32, client_y: f32, object_kind: &str, mesh_url: Option<&str>) {
    let Some(hit) = world3d_ground_plane_pick(session_state, camera_state, client_x, client_y, 0.0) else {
        gumball_state.catalogue_drop_preview = None;
        return;
    };
    let origin = snap_world_point_to_grid(hit, scene_state.lod.grid_snap_enabled, scene_state.lod.grid_factor);
    gumball_state.catalogue_drop_preview = Some(WorldCatalogueDropPreviewRecord { object_kind: object_kind.to_string(), mesh_url: mesh_url.filter(|url| !url.is_empty()).map(str::to_string), origin });
}

/// 👻️ Clears the catalogue-drop ghost on one `World3d` pane.
pub fn world3d_clear_catalogue_drop_preview(gumball_state: &mut WorldGumballState) {
    gumball_state.catalogue_drop_preview = None;
}

/// 🎯️ Grid-snapped ground origin for committing a catalogue drop on a wgpu `World3d` pane.
pub fn world3d_catalogue_drop_origin(session_state: &WorldSessionState, camera_state: &WorldCameraState, scene_state: &WorldSceneState, client_x: f32, client_y: f32) -> Option<[f64; 3]> {
    world3d_ground_plane_pick(session_state, camera_state, client_x, client_y, 0.0).map(|hit| snap_world_point_to_grid(hit, scene_state.lod.grid_snap_enabled, scene_state.lod.grid_factor))
}

pub fn preview_scale(scale: Option<&serde_json::Value>) -> [f32; 3] {
    match scale {
        Some(serde_json::Value::Array(values)) if values.len() >= 3 => [values[0].as_f64().unwrap_or(1.0) as f32, values[1].as_f64().unwrap_or(1.0) as f32, values[2].as_f64().unwrap_or(1.0) as f32],
        _ => [1.0, 1.0, 1.0],
    }
}

/// 👻️ Mesh key for `BrushPreviewGhost`: the real GLB's resolved id when a `meshUrl` is given
/// (loaded lazily, same as any other mesh), else the shared "box" primitive fallback ghost.
pub fn brush_preview_mesh_id(mesh_url: Option<&str>) -> String {
    mesh_url.map_or_else(|| "box".to_string(), mesh_id_from_url)
}

/// 👻️ The key a GHOST actually draws through. React's `BrushPreviewGhost` renders a translucent unit
/// box until its GLB resolves and the real mesh after — but React's ghost owns a separate three.js
/// object, while here every draw shares ONE mesh table keyed by mesh id. Minting the ghost's box
/// under a `mesh:…` id the scene is still waiting on would make that id resident, and
/// `offer_missing_mesh_fetches` offers a url only while its key is NOT resident — the ghost would
/// permanently suppress the fetch for the mesh it is standing in for. So a ghost over an unresolved
/// url draws through the shared `box` primitive instead, and switches to the real id the frame its
/// mesh lands.
pub fn ghost_mesh_id(scene_state: &WorldSceneState, mesh_url: Option<&str>) -> String {
    let id = brush_preview_mesh_id(mesh_url);
    if scene_state.meshes.contains_key(&id) || !scene_mesh_awaits_its_asset(scene_state, &id) {
        return id;
    }
    "box".to_string()
}

pub fn append_box_wireframe(lines: &mut Vec<LineVertex3d>, origin: [f64; 3], orientation: [f64; 4], scale: [f64; 3], color: [f32; 4]) {
    let corners = [[-0.5, -0.5, -0.5], [0.5, -0.5, -0.5], [0.5, 0.5, -0.5], [-0.5, 0.5, -0.5], [-0.5, -0.5, 0.5], [0.5, -0.5, 0.5], [0.5, 0.5, 0.5], [-0.5, 0.5, 0.5]];
    let model = Instance3d::model_from_trs([origin[0] as f32, origin[1] as f32, origin[2] as f32], [orientation[0] as f32, orientation[1] as f32, orientation[2] as f32, orientation[3] as f32], [scale[0] as f32, scale[1] as f32, scale[2] as f32]);
    let world_corners: Vec<[f32; 3]> = corners.iter().map(|corner| model.transform_point(Vec3::new(corner[0], corner[1], corner[2])).to_array()).collect();
    let edges = [(0, 1), (1, 2), (2, 3), (3, 0), (4, 5), (5, 6), (6, 7), (7, 4), (0, 4), (1, 5), (2, 6), (3, 7)];
    for (a, b) in edges {
        lines.push(LineVertex3d { position: world_corners[a], color });
        lines.push(LineVertex3d { position: world_corners[b], color });
    }
}

/// 🌞️ Resolves React's ambient plus configured-sun branch into one backend-neutral pass record.
pub fn environment_scene_lighting(environment: &WorldEnvironmentRecord) -> SceneLighting3d {
    let ambient = environment.ambient.as_ref();
    let sun = environment.sun.as_ref();
    let ambient_rgba = parse_color(ambient.and_then(|light| light.color.as_deref()).unwrap_or("#ffffff"));
    let sun_rgba = parse_color(sun.and_then(|light| light.color.as_deref()).unwrap_or("#ffffff"));
    SceneLighting3d {
        ambient_color: ambient_rgba[..3].try_into().expect("fixed rgba prefix"),
        ambient_intensity: ambient.and_then(|light| light.intensity).unwrap_or(1.15) as f32,
        sun_color: sun_rgba[..3].try_into().expect("fixed rgba prefix"),
        sun_intensity: sun.and_then(|light| light.intensity).unwrap_or(0.85) as f32,
        sun_enabled: sun.and_then(|light| light.enabled) == Some(true),
    }
}

pub fn environment_authored_material(environment: &WorldEnvironmentRecord) -> Option<SceneAuthoredMaterial3d> {
    let material = environment.material.as_ref()?;
    let color = parse_color(material.color.as_deref()?);
    let scene = environment_scene_material(environment);
    Some(SceneAuthoredMaterial3d {
        base_color: color,
        emissive: [scene.emissive[0] * scene.emissive_intensity, scene.emissive[1] * scene.emissive_intensity, scene.emissive[2] * scene.emissive_intensity],
        metalness: scene.metalness,
        roughness: scene.roughness,
        alpha: if color[3] < 1.0 { SceneMaterialAlpha3d::Blend } else { SceneMaterialAlpha3d::Opaque },
        alpha_cutoff: 0.5,
        double_sided: false,
        preserve_vertex_color: false,
        base_color_texture: None,
        texture_sampler: Default::default(),
        metallic_roughness_texture:None,normal_texture:None,occlusion_texture:None,emissive_texture:None,additional_texture_samplers:[Default::default();4],
    normal_scale:[1.0;2],occlusion_strength:1.0,})
}

/// 🖼️ Resolves the canvas clear color from `environment.background`, falling back to the ambient
/// theme clear color when absent or `"transparent"` (mirrors `isTransparentWorldBackground`).
pub fn environment_clear_color(environment: &WorldEnvironmentRecord, theme_clear: Rgba) -> Rgba {
    environment_explicit_clear_color(environment).unwrap_or(theme_clear)
}

pub fn environment_explicit_clear_color(environment: &WorldEnvironmentRecord) -> Option<Rgba> {
    let background = environment.background.as_deref()?;
    if background.eq_ignore_ascii_case("transparent") {
        return None;
    }
    let [r, g, b, a] = parse_color(background);
    Some(Rgba::new(r, g, b, a))
}
//#endregion Environment

pub fn step_world3d_terrain(session_state: &mut WorldSessionState, interaction_state: &mut WorldInteractionState, authority_state: &mut WorldAuthorityState, scene_state: &mut WorldSceneState, terrain_state: &mut WorldTerrainState,grant:semio_framework_value::retained_clone::RetainedCloneGrant,context:&mut semio_framework_job::StepContext<'_>)->WorldTerrainMeshProgress {
    if context.is_cancelled() {if let Some(cursor)=terrain_state.terrain_build.as_mut(){cursor.faulted=Some(WorldDynamicFault::Canceled);}scene_state.dynamic_fault=Some(WorldDynamicFault::Canceled);return WorldTerrainMeshProgress{step:WorldTerrainMeshPublicationStep::Pending,ownership:Default::default()};}
    if scene_state.dynamic_fault==Some(WorldDynamicFault::Canceled) {return step_world3d_terrain_retirement(session_state, terrain_state,grant,context);}
    let mut result=WorldTerrainMeshProgress{step:WorldTerrainMeshPublicationStep::Idle,ownership:Default::default()};
    if context.should_yield() || grant.maximum_items==0 {result.step=WorldTerrainMeshPublicationStep::Pending;return result;}
    if terrain_state.terrain_build.is_none() && terrain_state.terrain_pending.is_some() {
        match begin_world_terrain_pending(session_state, terrain_state,grant,context.is_cancelled()) {
            Ok(progress)=>{context.set_stage("terrainMeshAdmission");context.consume_fuel(1);result.step=WorldTerrainMeshPublicationStep::Pending;result.ownership=progress;return result;}
            Err(fault)=>{result.step=WorldTerrainMeshPublicationStep::Fault(fault);return result;}
        }
    }
    let Some(mut cursor)=terrain_state.terrain_build.take() else{return result};
    if !terrain_state.terrain_visible_tiles.contains(&(cursor.z,cursor.x,cursor.y)) || scene_state.dynamic_blocked_mesh.is_some() || scene_state.dynamic_mesh_close.is_some() {cursor.faulted=Some(WorldDynamicFault::StaleToken);}
    match cursor.step(terrain_state.terrain_revision,terrain_state.terrain_revision,grant,context) {
        Ok((step,progress))=>{
            result.ownership=progress;
            match step {
                WorldTerrainMeshStep::Pending=>{terrain_state.terrain_build=Some(cursor);result.step=WorldTerrainMeshPublicationStep::Pending;}
                WorldTerrainMeshStep::Ready(key,lease)=>{
                    if let Err(rejected)=publish_world3d_mesh_lease(interaction_state, scene_state,key,lease) {
                        cursor.faulted=Some(rejected.fault);retain_rejected_world_mesh(session_state, authority_state, scene_state,rejected);
                    }
                    terrain_state.terrain_build=Some(cursor);result.step=WorldTerrainMeshPublicationStep::Pending;
                }
                WorldTerrainMeshStep::Complete(tile)=>{terrain_state.terrain_built_tiles.insert(tile);result.step=WorldTerrainMeshPublicationStep::Complete;}
                WorldTerrainMeshStep::Fault(fault)=>{mark_world_dynamic_fault(session_state, authority_state, scene_state,fault);result.step=WorldTerrainMeshPublicationStep::Fault(fault);}
            }
        }
        Err(fault)=>{terrain_state.terrain_build=Some(cursor);result.step=WorldTerrainMeshPublicationStep::Fault(fault);}
    }
    result
}

/// 🏔️ GPU-free half of `sync_terrain`: asks `TerrainSessionCore` which DEM tiles are visible for
/// the current camera, evicts (CPU-side) tiles that scrolled out of view, admits byte-fetches for
/// tiles not yet uploaded (see `reserve_terrain_tile_fetch`), and builds/caches one vertex-coloured
/// mesh per tile whose elevation data is already available. Returns the tiles to draw this frame
/// plus the mesh keys the caller must evict from the GPU.
pub fn sync_terrain_state(session_state: &mut WorldSessionState, authority_state: &mut WorldAuthorityState, scene_state: &mut WorldSceneState, terrain_state: &mut WorldTerrainState, assets_state: &mut WorldAssetsState, camera: &Camera3d) -> (Vec<TerrainTileDraw>, Vec<String>) {
    let Some(style) = terrain_state.terrain_style.clone() else {
        return (Vec::new(), Vec::new());
    };
    let camera_json = json!({
        "position": [camera.position.x as f64, camera.position.y as f64, camera.position.z as f64],
        "target": [camera.target.x as f64, camera.target.y as f64, camera.target.z as f64],
    })
    .to_string();
    let visible_json = terrain_state.terrain_session.visible_terrain_tiles_json(&camera_json);
    let visible_rows: Vec<TerrainVisibleTileRow> = serde_json::from_str(&visible_json).unwrap_or_default();
    let visible_set: HashSet<(u32, u32, u32)> = visible_rows.iter().map(|row| (row.z, row.x, row.y)).collect();

    let stale: Vec<(u32, u32, u32)> = terrain_state.terrain_visible_tiles.iter().copied().filter(|key| !visible_set.contains(key)).collect();
    let mut evicted_mesh_keys = Vec::new();
    for (z, x, y) in stale {
        terrain_state.terrain_session.evict_terrain_tile(z, x, y);
        terrain_state.terrain_built_tiles.remove(&(z, x, y));
        let mesh_key = terrain_tile_mesh_key(&session_state.surface_id, z, x, y);
        if !retire_world_mesh(scene_state, &mesh_key) {
            return (Vec::new(), evicted_mesh_keys);
        }
        evicted_mesh_keys.push(mesh_key);
    }
    terrain_state.pending_terrain_tile_urls.retain(|_, tile| visible_set.contains(tile));
    terrain_state.terrain_visible_tiles = visible_set.clone();

    let mut tile_draws = Vec::new();
    for (z, x, y) in visible_set {
        if !terrain_family_visible(terrain_state, (z, x, y)) {
            let mesh_json = terrain_state.terrain_session.terrain_tile_mesh_json(z, x, y);
            if mesh_json == "null" {
                reserve_terrain_tile_fetch(session_state, terrain_state, assets_state, &style.tile_url_template, (z, x, y));
            } else if terrain_state.terrain_build.is_none() && terrain_state.terrain_pending.is_none() && scene_state.dynamic_mesh_close.is_none() && scene_state.dynamic_blocked_mesh.is_none() && session_state.snapshot_fault.is_none() {
                if let Ok(mesh_payload) = serde_json::from_str::<TerrainTileMeshPayload>(&mesh_json) {
                    let next = scene_state.placeholder_generation.checked_add(1);
                    if let Some(next) = next {
                        scene_state.placeholder_generation=next;
                        terrain_state.terrain_pending=Some(((z,x,y),mesh_payload,next,terrain_state.terrain_revision,terrain_state.terrain_revision));
                    } else {
                        mark_world_dynamic_fault(session_state, authority_state, scene_state, WorldDynamicFault::StaleToken);
                    }
                }
            }
            continue;
        }
        let mesh_key = terrain_tile_mesh_key(&session_state.surface_id, z, x, y);
        if !scene_state.meshes.contains_key(&mesh_key) {
            continue;
        }
        let mesh_version = live_world3d_mesh_version(scene_state, &mesh_key);
        tile_draws.push(TerrainTileDraw { mesh_key, mesh_version });
    }
    (tile_draws, evicted_mesh_keys)
}

/// 🏔️ Per-frame terrain sync entry point used by `render_world_3d` — see `sync_terrain_state` for
/// the (unit-testable) tile-visibility/meshing/fetch-admission logic this wraps with GPU upload and
/// eviction calls.
pub fn sync_terrain(session_state: &mut WorldSessionState, authority_state: &mut WorldAuthorityState, scene_state: &mut WorldSceneState, terrain_state: &mut WorldTerrainState, assets_state: &mut WorldAssetsState, gpu: &mut World3dBuildContext, camera: &Camera3d) -> Vec<SceneDraw3d> {
    let (tile_draws, evicted_mesh_keys) = sync_terrain_state(session_state, authority_state, scene_state, terrain_state, assets_state, camera);
    for key in evicted_mesh_keys {
        gpu.evict_mesh(&key);
    }
    tile_draws
        .into_iter()
        .filter_map(|tile| {
            let mesh = scene_state.meshes.get(&tile.mesh_key)?;
            gpu.ensure_mesh(&tile.mesh_key, tile.mesh_version, *mesh);
            Some(SceneDraw3d {
                mesh_key: tile.mesh_key.clone(),
                mesh_version: tile.mesh_version,
                instances: vec![Instance3d { component_source: None, id: format!("terrain-{}", tile.mesh_key), model: Mat4::identity(), color: [1.0, 1.0, 1.0, 1.0], selected: false, hovered: false, material: Default::default() }],
                shadow_role: Default::default(),
            })
        })
        .collect()
}
//#endregion Terrain

pub fn begin_world_placeholder_mesh(session_state: &mut WorldSessionState, authority_state: &mut WorldAuthorityState, retirement_state: &mut WorldRetirementState, scene_state: &mut WorldSceneState, key: &str, kind: WorldPlaceholderKind) {
    if scene_state.meshes.contains_key(key) || scene_state.placeholder_build.is_some() || retirement_state.dynamic_retirement.is_some() {
        return;
    }
    scene_state.placeholder_generation = scene_state.placeholder_generation.wrapping_add(1).max(1);
    match WorldPlaceholderMeshCursor::new(key, kind, scene_state.placeholder_generation, session_state.interaction_revision) {
        Ok(cursor) => scene_state.placeholder_build = Some(cursor),
        Err(_) => mark_world_dynamic_fault(session_state, authority_state, scene_state, WorldDynamicFault::ByteCapacity),
    }
}

pub fn step_world_placeholder_mesh(session_state: &mut WorldSessionState, interaction_state: &mut WorldInteractionState, authority_state: &mut WorldAuthorityState, scene_state: &mut WorldSceneState) {
    let Some(mut cursor) = scene_state.placeholder_build.take() else { return };
    match cursor.step() {
        WorldPlaceholderMeshStep::Pending => scene_state.placeholder_build = Some(cursor),
        WorldPlaceholderMeshStep::Ready(key,lease,appearance) => {if let Some(appearance)=appearance {let asset=World3dMeshAsset{mesh:lease,appearance};if let Err(rejected)=publish_world3d_inline_asset(session_state, interaction_state, scene_state,key,asset){cursor.owner=WorldPlaceholderOwner::Ready(rejected.value.mesh);cursor.appearance=Some(rejected.value.appearance);cursor.faulted=true;scene_state.placeholder_build=Some(cursor);mark_world_dynamic_fault(session_state, authority_state, scene_state,rejected.fault);}}else{store_mesh(session_state, interaction_state, authority_state, scene_state,key,lease)}},
        WorldPlaceholderMeshStep::Fault => mark_world_dynamic_fault(session_state, authority_state, scene_state, WorldDynamicFault::Closing),
    }
}
//#endregion PlaceholderMeshAuthority

pub fn enqueue_world3d_intent(authority_state: &mut WorldAuthorityState, intent: WorldInteractionIntent) -> Result<(), WorldInteractionIntent> {
    let Some(authority) = authority_state.interaction_authority.as_mut() else {
        return Err(intent);
    };
    if authority.faulted || authority.closing || authority.blocked.is_some() || authority.next_generation == u64::MAX || intent.generation != authority.next_generation {
        return Err(intent);
    }
    match authority.queue.push(intent) {
        Ok(()) => {
            authority.next_generation += 1;
            Ok(())
        }
        Err(intent) if authority.blocked.is_none() => {
            authority.blocked = Some(intent);
            authority.next_generation += 1;
            Ok(())
        }
        Err(intent) => Err(intent),
    }
}

/** 🫧 Whether this surface's pointer MOVES may be folded onto one another
 * ([`WorldInteractionIntentQueue::coalesce_tail_pointer_move`]).
 *
 * Two gestures read the move SAMPLES rather than the latest point plus the accumulated delta, and
 * folding would change what they produce, which is the one thing the fold may never do:
 * - a LASSO marquee is its own path — `marquee_is_crossing_from_path` and the painted outline both
 *   walk every point, where a rectangle needs only its start and its newest corner;
 * - a live PAINT stroke raycasts once per move (`WorldRayPickPurpose::Paint`), so a folded move
 *   would leave a gap in the painted run.
 *
 * Everything else — hover, orbit/pan/zoom, the rectangle marquee, the vertex/brush object hover —
 * reads only `(x, y)` and the summed `(dx, dy)`, which the fold preserves exactly. */
pub fn world3d_pointer_move_folds(selection_state: &WorldSelectionState) -> bool {
    selection_state.selection_method != "lasso" && !(selection_state.interaction_mode == "paint" && selection_state.paint_stroke_active)
}

pub fn enqueue_world3d_event(selection_state: &mut WorldSelectionState, authority_state: &mut WorldAuthorityState, mut intent: WorldInteractionIntent) -> Result<u64, WorldInteractionIntent> {
    let folds = world3d_pointer_move_folds(selection_state);
    let Some(authority) = authority_state.interaction_authority.as_mut() else {
        return Err(intent);
    };
    if folds && !authority.faulted && !authority.closing && authority.blocked.is_none() {
        if let Some(generation) = authority.queue.coalesce_tail_pointer_move(&intent) {
            return Ok(generation);
        }
    }
    intent.generation = authority.next_generation;
    let generation = intent.generation;
    enqueue_world3d_intent(authority_state, intent).map(|()| generation)
}

pub fn enqueue_world3d_events<const N: usize>(authority_state: &mut WorldAuthorityState, mut intents: [WorldInteractionIntent; N]) -> Result<[u64; N], [WorldInteractionIntent; N]> {
    let Some(authority) = authority_state.interaction_authority.as_mut() else {
        return Err(intents);
    };
    let available = if authority.blocked.is_some() { 0 } else { WORLD_INTERACTION_INTENT_CAPACITY - usize::from(authority.queue.len) + 1 };
    if N == 0 || N > 4 || authority.faulted || authority.closing || authority.next_generation.checked_add(N as u64).is_none() || N > available {
        return Err(intents);
    }
    let mut generations = [0; N];
    for index in 0..N {
        intents[index].generation = authority.next_generation;
        generations[index] = authority.next_generation;
        match authority.queue.push(intents[index]) {
            Ok(()) => {}
            Err(intent) if authority.blocked.is_none() => authority.blocked = Some(intent),
            Err(_) => unreachable!("world intent batch preflight owns exact fixed credits"),
        }
        authority.next_generation += 1;
    }
    Ok(generations)
}

/** 🎟️ The generation the frame driver must step this surface at, or `None` when it owes nothing.
 *
 * 🧭️ A drained queue is NOT idle while a camera settle is owed: the debounced `setCamera` (and the
 * `noteWorldNavigation` ahead of it, and the re-published hover behind it) are published by
 * `WorldInteractionAuthority::step` with no intent of their own, so they need a generation to be
 * stepped at. The LAST minted one is that generation — no queued intent can ever carry it again, so
 * the settle plan's `retire_front` is a no-op and a fresh intent supersedes it as `Stale`, which is
 * what a gesture that resumed mid-settle deserves. See [`WorldCameraSync`]. */
pub fn world3d_interaction_front_generation(camera_state: &WorldCameraState, authority_state: &WorldAuthorityState) -> Option<u64> {
    let authority = authority_state.interaction_authority.as_ref()?;
    if let Some(intent) = authority.queue.front() {
        return Some(intent.generation);
    }
    let settle_owed = !authority.closing && (camera_state.camera_sync.owed || (camera_state.camera_sync.hover_owed && camera_state.camera_sync.at.is_some()));
    (authority.active.is_some() || settle_owed).then(|| authority.next_generation.wrapping_sub(1))
}

pub fn begin_world3d_interaction_close(authority_state: &mut WorldAuthorityState) {
    if let Some(authority) = authority_state.interaction_authority.as_mut() {
        authority.closing = true;
        authority.queue.begin_close();
    }
}

pub fn close_world3d_interaction_step(authority_state: &mut WorldAuthorityState, input: &mut ui_wgpu::wgpu::InputState<ActionDescriptor>, context: &mut semio_framework_job::StepContext<'_>) -> bool {
    if context.should_yield() {
        return false;
    }
    let Some(authority) = authority_state.interaction_authority.as_mut() else {
        return true;
    };
    authority.closing = true;
    authority.queue.begin_close();
    if let Some(active) = authority.active.as_mut() {
        let complete = match active {
            WorldInteractionActive::Plan { plan, .. } => plan.close_step(),
            WorldInteractionActive::Pick { cursor, .. } => cursor.close_step(),
            WorldInteractionActive::ObjectPick { cursor, .. } => cursor.close_step(),
            WorldInteractionActive::ComponentPick { cursor, .. } => cursor.close_step(),
            WorldInteractionActive::MarqueePick { cursor, .. } => cursor.close_step(),
            WorldInteractionActive::MarqueePublish { job, .. } => job.close_step(input),
            WorldInteractionActive::ComponentMarqueePublish { job, .. } => job.close_step(input),
            WorldInteractionActive::GumballPick { cursor, .. } => cursor.close_step(),
            WorldInteractionActive::GumballCommit { job, .. } => job.close_step(input),
            WorldInteractionActive::BrushCommit { job, .. } => job.close_step(input),
        };
        context.consume_fuel(1);
        if complete {
            authority.active = None;
        }
        return false;
    }
    if let Some(registry) = authority.registry.as_mut() {
        if registry.close_step(context) {
            authority.registry = None;
        }
        return false;
    }
    if authority.blocked.take().is_some() {
        context.consume_fuel(1);
        return false;
    }
    if authority.right_press.take().is_some() || authority.right_dragged {
        authority.right_dragged = false;
        context.consume_fuel(1);
        return false;
    }
    if let Some(marquee) = authority.marquee.as_mut() {
        if marquee.close_step() {
            authority.marquee = None;
        }
        context.consume_fuel(1);
        return false;
    }
    if let Some(gumball) = authority.gumball.as_mut() {
        if gumball.close_step() {
            authority.gumball = None;
        }
        context.consume_fuel(1);
        return false;
    }
    let complete = authority.queue.close_step();
    context.consume_fuel(1);
    complete
}

pub fn world3d_interaction_terminal_is_empty(authority_state: &WorldAuthorityState) -> bool {
    authority_state.interaction_authority.as_ref().is_none_or(|authority| {
        authority.closing
            && authority.registry.is_none()
            && authority.active.is_none()
            && authority.right_press.is_none()
            && !authority.right_dragged
            && authority.marquee.is_none()
            && authority.gumball.is_none()
            && authority.blocked.is_none()
            && authority.queue.terminal_is_empty()
    })
}

impl WorldInteractionAuthority {
    /// 🧭️ A DRAINED queue is this lane's settle point — React's `CAMERA_SYNC_DEBOUNCE_MS`
    /// trailing debounce without a timer, since the last answered intent of a gesture is
    /// exactly when that debounce would fire. See [`WorldCameraSync`].
    ///
    /// 🧭️ three's `OrbitControls.onPointerUp` dispatches `_endEvent` UNCONDITIONALLY once the
    /// last pointer lifts, and React's `onEnd` always reports the camera — so every release
    /// over a world pane owes one `setCamera`, whatever the button and whether or not the
    /// camera moved. The reference journals exactly that for `pick-instance` and
    /// `context-menu` (`🗑️generated/parity-run-2/steps.json`).
    ///
    /// 🖱️ The MIDDLE button starts no CAMERA gesture of its own: pan is a middle-DRAG, planned by
    /// `plan_world3d_drag` from the moves that follow it. What the press itself answers is the
    /// SELECTION React publishes on any button (`plan_world3d_non_primary_press_pick`); with
    /// neither, the intent used to fall through to the unclaimed-intent fault below, so the FIRST
    /// middle-click on any world surface recorded a frame fault and killed the page
    /// (ticket 26/09/17/WGPU-RENDERER-REACT-PARITY, `📓️w9c-behaviour-parity-run-2.md` step 20,
    /// where React simply pans).
    ///
    /// 🖱️ Tracking the click/drag discrimination must NOT swallow the gesture: a right-drag is
    /// also how `plan_world3d_drag` reaches pan (`+shift`) and orbit (`+alt`/`+meta`). Retiring
    /// here unconditionally is what left the wgpu shell with zoom and middle-button pan only.
    ///
    /// 🖱️📋️ A right RELEASE closes the click/drag discrimination and answers nothing else. The
    /// menu itself is the SHELL's (`🐚️Shell/🎯️targets/🧊️wgpu`'s `open_context_menu`, whose
    /// `resolve_context_menu_surface` already fills a World3d surface's `hits`/`selection` from
    /// [`world3d_context_menu_surface`]) and it opens on the PRESS, exactly as React's own
    /// `onContextMenu` does.
    ///
    /// 🩸️ This arm used to run a `WorldContextMenuCursor` and publish `worldContextMenuAt`. No
    /// app declares that verb — React deleted its twin `contextMenuAt` when the target moved onto
    /// the menu REQUEST (`world3dContextMenuSurfaceV1`'s own docstring: "dispatching it anyway
    /// only produced an `undeclaredActionDiagnostic` drop on every right-click"), and the whole
    /// repository had exactly one mention of it left, here. So the wgpu target journalled a verb
    /// the guest refuses where React journals a menu
    /// (ticket 26/09/17/WGPU-RENDERER-REACT-PARITY, `📓️w10a-world-interaction-journal-parity.md`).
    ///
    /// 🚚️ A live relocate drag owns every move it receives: it only moves its own ghost and
    /// publishes nothing until the release, so it must not also reach the camera arms below
    /// (React returns early from `onPointerMove` for the same reason).
    ///
    /// 🚚️ The relocate gesture is checked FIRST, exactly as React's `handlePointerDown`
    /// opens with `if (relocateMode && beginRelocateDrag(event)) return;`
    /// (`🌐️World3dHost/🟦️.tsx:6883`). The press publishes nothing; the release publishes the
    /// ONE absolute `worldRelocate`, or nothing at all when the drag never travelled.
    ///
    /// 🚪️ A leave over a surface that publishes no hover has nothing to clear — it is answered,
    /// not refused, wherever it was aimed (the point it left at is usually OUTSIDE this
    /// surface's own rect by construction).
    ///
    /// 🖱️ An intent aimed at a point this surface no longer covers is not this surface's to
    /// answer. Every pick cursor is built through `pointer_in_pick_rect`, which refuses a point
    /// outside the CURRENT pick rect — and a queued intent outlives a relayout: dragging the
    /// dock's split gutter moves the pane out from under moves enqueued against its previous
    /// bounds (measured: a hover at x=534.4 against `pick=…+535.6,54.4`). Faulting on that
    /// refusal is a FRAME fault, which kills the page, so the whole shell tore down mid-drag
    /// (ticket 26/09/17/WGPU-RENDERER-REACT-PARITY, `📓️w9c-behaviour-parity-run-2.md` step 15).
    /// A stale aim retires like any other answered intent; a fault stays a fault for a point the
    /// surface DOES cover, which is a real refusal.
    pub fn step(&mut self, session_state: &mut WorldSessionState, camera_state: &mut WorldCameraState, selection_state: &mut WorldSelectionState, gumball_state: &mut WorldGumballState, interaction_state: &mut WorldInteractionState, scene_state: &mut WorldSceneState, generation: u64, input: &mut ui_wgpu::wgpu::InputState<ActionDescriptor>, context: &mut semio_framework_job::StepContext<'_>) -> WorldInteractionAuthorityStep {
        if self.closing || context.should_yield() {
            return WorldInteractionAuthorityStep::Pending;
        }
        if let Some(active) = self.active.take() {
            return self.step_active(active, session_state, camera_state, selection_state, gumball_state, interaction_state, scene_state, generation, input, context);
        }
        if self.faulted {
            return WorldInteractionAuthorityStep::Fault;
        }
        if self.marquee.as_ref().is_some_and(|marquee| marquee.revision != session_state.interaction_revision || marquee.retiring) {
            let complete = self.marquee.as_mut().expect("stale marquee owner").close_step();
            if complete {
                self.marquee = None;
            }
            context.consume_fuel(1);
            return WorldInteractionAuthorityStep::Stale;
        }
        if self.gumball.as_ref().is_some_and(|gumball| gumball.revision != session_state.interaction_revision) {
            let complete = self.gumball.as_mut().expect("stale gumball owner").close_step();
            if complete {
                self.gumball = None;
            }
            context.consume_fuel(1);
            return WorldInteractionAuthorityStep::Stale;
        }
        if interaction_state.interaction_objects.revision != session_state.interaction_revision || self.registry.is_some() {
            let mut registry = self.registry.take().unwrap_or_else(|| WorldInteractionRegistryBuildCursor::new(session_state.interaction_revision));
            match registry.step(selection_state, gumball_state, scene_state, context) {
                WorldInteractionStep::Pending => {
                    self.registry = Some(registry);
                    return WorldInteractionAuthorityStep::Pending;
                }
                WorldInteractionStep::Complete => {
                    context.consume_fuel(1);
                    return WorldInteractionAuthorityStep::Pending;
                }
                WorldInteractionStep::Stale => {
                    self.registry = Some(WorldInteractionRegistryBuildCursor::new(session_state.interaction_revision));
                    context.consume_fuel(1);
                    return WorldInteractionAuthorityStep::Stale;
                }
                WorldInteractionStep::Fault => {
                    self.registry = Some(registry);
                    self.faulted = true;
                    return WorldInteractionAuthorityStep::Fault;
                }
            }
        }
        if let Some(blocked) = self.blocked.take() {
            match self.queue.push(blocked) {
                Ok(()) => {
                    context.consume_fuel(1);
                    return WorldInteractionAuthorityStep::Pending;
                }
                Err(blocked) => {
                    self.blocked = Some(blocked);
                }
            }
        }
        let Some(intent) = self.queue.front().copied() else {
            if let Some(plan) = plan_world3d_camera_settle(session_state, camera_state, generation) {
                self.active = Some(WorldInteractionActive::Plan { plan, retirement: None });
                context.consume_fuel(1);
                return WorldInteractionAuthorityStep::Pending;
            }
            if let (true, Some([x, y])) = (camera_state.camera_sync.hover_owed, camera_state.camera_sync.at) {
                camera_state.camera_sync.hover_owed = false;
                let mut hover = WorldInteractionIntent::pointer_move(x, y, 0.0, 0.0, false, 0, &PointerModifiers::default());
                hover.generation = self.next_generation;
                if self.queue.push(hover).is_ok() {
                    self.next_generation += 1;
                    context.consume_fuel(1);
                    return WorldInteractionAuthorityStep::Pending;
                }
            }
            return WorldInteractionAuthorityStep::Idle;
        };
        if !matches!(intent.phase, WorldInteractionPhase::Close | WorldInteractionPhase::PointerLeave | WorldInteractionPhase::Cancel) {
            camera_state.camera_sync.at = Some([intent.x, intent.y]);
            if intent.phase == WorldInteractionPhase::PointerButton && !intent.down {
                camera_state.camera_sync.owed = true;
            }
        }
        if intent.generation < generation {
            self.queue.retire_front(intent.generation);
            context.consume_fuel(1);
            return WorldInteractionAuthorityStep::Stale;
        }
        if intent.generation > generation {
            self.faulted = true;
            return WorldInteractionAuthorityStep::Fault;
        }
        if intent.phase == WorldInteractionPhase::Cancel {
            let reason = WorldCancelReason::from_button(intent.button);
            if let Some((phase, motion)) = self.gumball.as_ref().and_then(|gesture| gesture.cancel(reason)) {
                let gesture = self.gumball.take().expect("cancelled live gumball gesture");
                let (translate, angle, scale) = motion;
                self.gumball_swallow = true;
                self.active = Some(WorldInteractionActive::GumballCommit { job: WorldGumballCommitJob::new(intent.generation, WorldGumballGesture { translate, angle, scale, ..gesture }, phase), retirement: None });
                context.consume_fuel(1);
                return WorldInteractionAuthorityStep::Pending;
            }
            if let Some(plan) = plan_world3d_paint_abort(session_state, selection_state, generation, reason) {
                self.active = Some(WorldInteractionActive::Plan { plan, retirement: None });
                context.consume_fuel(1);
                return WorldInteractionAuthorityStep::Pending;
            }
            self.queue.retire_front(intent.generation);
            context.consume_fuel(1);
            return WorldInteractionAuthorityStep::Complete;
        }
        if self.gumball_swallow && intent.button == 0 && (intent.phase == WorldInteractionPhase::PointerMove && intent.down || intent.phase == WorldInteractionPhase::PointerButton) {
            if intent.phase == WorldInteractionPhase::PointerButton && !intent.down {
                self.gumball_swallow = false;
            }
            self.queue.retire_front(intent.generation);
            context.consume_fuel(1);
            return WorldInteractionAuthorityStep::Complete;
        }
        if intent.phase == WorldInteractionPhase::PointerButton && intent.button == 2 && intent.down {
            self.right_press = Some([intent.x, intent.y]);
            self.right_dragged = false;
            if let Some(active) = plan_world3d_non_primary_press_pick(session_state, camera_state, selection_state, generation, &intent) {
                self.active = Some(active);
                context.consume_fuel(1);
                return WorldInteractionAuthorityStep::Pending;
            }
            self.queue.retire_front(intent.generation);
            context.consume_fuel(1);
            return WorldInteractionAuthorityStep::Complete;
        }
        if intent.phase == WorldInteractionPhase::PointerButton && intent.button == 1 {
            if intent.down {
                if let Some(active) = plan_world3d_non_primary_press_pick(session_state, camera_state, selection_state, generation, &intent) {
                    self.active = Some(active);
                    context.consume_fuel(1);
                    return WorldInteractionAuthorityStep::Pending;
                }
            }
            self.queue.retire_front(intent.generation);
            context.consume_fuel(1);
            return WorldInteractionAuthorityStep::Complete;
        }
        if intent.phase == WorldInteractionPhase::PointerMove && intent.button == 2 && intent.down {
            if let Some(start) = self.right_press {
                let dx = intent.x - start[0];
                let dy = intent.y - start[1];
                self.right_dragged |= (dx * dx + dy * dy).sqrt() > CLICK_DRAG_THRESHOLD_PX;
            }
            let modifiers = PointerModifiers { shift: intent.shift, ctrl: intent.ctrl, alt: intent.alt, meta: intent.meta };
            if let Some(plan) = plan_world3d_drag(session_state, camera_state, generation, intent.dx, intent.dy, intent.button, &modifiers) {
                self.active = Some(WorldInteractionActive::Plan { plan, retirement: None });
                context.consume_fuel(1);
                return WorldInteractionAuthorityStep::Pending;
            }
            self.queue.retire_front(intent.generation);
            context.consume_fuel(1);
            return WorldInteractionAuthorityStep::Complete;
        }
        if intent.phase == WorldInteractionPhase::PointerButton && intent.button == 2 && !intent.down {
            self.right_press = None;
            self.right_dragged = false;
            self.queue.retire_front(intent.generation);
            context.consume_fuel(1);
            return WorldInteractionAuthorityStep::Complete;
        }
        if intent.phase == WorldInteractionPhase::PointerButton && intent.button == 0 && intent.down && session_state.active_utility == "select" && self.gumball.is_none() && world3d_offers_transform_gumball(session_state) {
            self.active = Some(WorldInteractionActive::GumballPick { cursor: WorldGumballPickCursor::new(session_state, selection_state, generation, intent.x, intent.y), retirement: None });
            context.consume_fuel(1);
            return WorldInteractionAuthorityStep::Pending;
        }
        if let (true, Some(gumball)) = (intent.button == 0 && (intent.phase == WorldInteractionPhase::PointerMove && intent.down || intent.phase == WorldInteractionPhase::PointerButton && !intent.down), self.gumball.as_mut()) {
            match gumball.begin_update(intent.generation, intent.x, intent.y) {
                WorldInteractionStep::Stale => return WorldInteractionAuthorityStep::Stale,
                WorldInteractionStep::Fault => {
                    self.faulted = true;
                    return WorldInteractionAuthorityStep::Fault;
                }
                _ => {}
            }
            match gumball.update_step(session_state, camera_state, gumball_state, interaction_state, scene_state) {
                WorldInteractionStep::Pending => {
                    context.consume_fuel(1);
                    return WorldInteractionAuthorityStep::Pending;
                }
                WorldInteractionStep::Stale => return WorldInteractionAuthorityStep::Stale,
                WorldInteractionStep::Fault => {
                    self.faulted = true;
                    return WorldInteractionAuthorityStep::Fault;
                }
                WorldInteractionStep::Complete => {}
            }
            if intent.phase == WorldInteractionPhase::PointerButton {
                let gesture = self.gumball.take().expect("gumball final pointer pose is complete");
                let (phase, (translate, angle, scale)) = gesture.release();
                self.active = Some(WorldInteractionActive::GumballCommit { job: WorldGumballCommitJob::new(intent.generation, WorldGumballGesture { translate, angle, scale, ..gesture }, phase), retirement: None });
                context.consume_fuel(1);
                return WorldInteractionAuthorityStep::Pending;
            }
            if let Some((phase, motion)) = gumball.stream() {
                let tick = gumball.publication(motion);
                self.active = Some(WorldInteractionActive::GumballCommit { job: WorldGumballCommitJob::new(intent.generation, tick, phase), retirement: None });
                context.consume_fuel(1);
                return WorldInteractionAuthorityStep::Pending;
            }
            self.queue.retire_front(intent.generation);
            context.consume_fuel(1);
            return WorldInteractionAuthorityStep::Complete;
        }
        if intent.phase == WorldInteractionPhase::PointerButton && intent.button == 0 && !intent.down && session_state.active_utility == "brush" {
            match WorldBrushCommitJob::new(session_state, gumball_state, intent.generation) {
                Ok(Some(job)) => {
                    self.active = Some(WorldInteractionActive::BrushCommit { job, retirement: None });
                    context.consume_fuel(1);
                    return WorldInteractionAuthorityStep::Pending;
                }
                Ok(None) => {
                    self.queue.retire_front(intent.generation);
                    context.consume_fuel(1);
                    return WorldInteractionAuthorityStep::Complete;
                }
                Err(_) => {
                    self.faulted = true;
                    return WorldInteractionAuthorityStep::Fault;
                }
            }
        }
        let select_marquee_route = session_state.active_utility == "select";
        if intent.phase == WorldInteractionPhase::PointerButton && intent.button == 0 && intent.down && select_marquee_route {
            if self.marquee.is_some() {
                self.faulted = true;
                return WorldInteractionAuthorityStep::Fault;
            }
            self.marquee = Some(WorldMarqueeGesture::new(session_state.interaction_revision, intent.generation, [intent.x, intent.y]));
            self.queue.retire_front(intent.generation);
            context.consume_fuel(1);
            return WorldInteractionAuthorityStep::Complete;
        }
        if let (true, Some(marquee)) = (intent.phase == WorldInteractionPhase::PointerMove && intent.button == 0 && intent.down, self.marquee.as_mut()) {
            if intent.generation <= marquee.start_generation || !marquee.push([intent.x, intent.y]) {
                self.faulted = true;
                return WorldInteractionAuthorityStep::Fault;
            }
            self.queue.retire_front(intent.generation);
            context.consume_fuel(1);
            return WorldInteractionAuthorityStep::Complete;
        }
        if intent.phase == WorldInteractionPhase::PointerButton && intent.button == 0 && !intent.down && self.marquee.is_some() {
            let marquee = self.marquee.as_mut().expect("marquee gesture retained above");
            if intent.generation <= marquee.start_generation {
                self.faulted = true;
                return WorldInteractionAuthorityStep::Fault;
            }
            if marquee.is_click([intent.x, intent.y]) {
                marquee.retiring = true;
                context.consume_fuel(1);
                return WorldInteractionAuthorityStep::Pending;
            }
            let marquee = self.marquee.take().expect("non-click marquee owner");
            let Some(cursor) = WorldMarqueePickCursor::new(session_state, camera_state, selection_state, intent.generation, marquee) else {
                self.faulted = true;
                return WorldInteractionAuthorityStep::Fault;
            };
            self.active = Some(WorldInteractionActive::MarqueePick { cursor, retirement: None });
            context.consume_fuel(1);
            return WorldInteractionAuthorityStep::Pending;
        }
        let active = match intent.phase {
            WorldInteractionPhase::Wheel => plan_world3d_wheel(session_state, camera_state, generation, intent.delta).map(|plan| WorldInteractionActive::Plan { plan, retirement: None }),
            WorldInteractionPhase::PointerLeave => plan_world3d_pointer_leave(session_state, selection_state, generation).map(|plan| WorldInteractionActive::Plan { plan, retirement: None }),
            WorldInteractionPhase::PointerDrag => {
                if world3d_relocate_active(gumball_state) {
                    world3d_update_relocate_drag(session_state, camera_state, gumball_state, scene_state, intent.x, intent.y);
                    self.queue.retire_front(intent.generation);
                    context.consume_fuel(1);
                    return WorldInteractionAuthorityStep::Complete;
                }
                let modifiers = PointerModifiers { shift: intent.shift, ctrl: intent.ctrl, alt: intent.alt, meta: intent.meta };
                let Some(plan) = plan_world3d_drag(session_state, camera_state, generation, intent.dx, intent.dy, intent.button, &modifiers) else {
                    self.queue.retire_front(intent.generation);
                    context.consume_fuel(1);
                    return WorldInteractionAuthorityStep::Complete;
                };
                Some(WorldInteractionActive::Plan { plan, retirement: None })
            }
            WorldInteractionPhase::PointerMove => {
                if world3d_relocate_active(gumball_state) {
                    world3d_update_relocate_drag(session_state, camera_state, gumball_state, scene_state, intent.x, intent.y);
                    self.queue.retire_front(intent.generation);
                    context.consume_fuel(1);
                    return WorldInteractionAuthorityStep::Complete;
                }
                let modifiers = PointerModifiers { shift: intent.shift, ctrl: intent.ctrl, alt: intent.alt, meta: intent.meta };
                if intent.down {
                    if let Some(plan) = plan_world3d_drag(session_state, camera_state, generation, intent.dx, intent.dy, intent.button, &modifiers) {
                        Some(WorldInteractionActive::Plan { plan, retirement: None })
                    } else if intent.button == 0 && selection_state.interaction_mode == "paint" && selection_state.paint_stroke_active {
                        WorldRayPickCursor::new(session_state, camera_state, generation, WorldRayPickPurpose::Paint, intent.x, intent.y).map(|cursor| WorldInteractionActive::Pick { cursor, retirement: None })
                    } else {
                        None
                    }
                } else {
                    if session_state.active_utility == "brush" || (session_state.active_utility == "select" && selection_state.granularity == "vertex") {
                        WorldObjectPickCursor::new(session_state, camera_state, generation, WorldObjectPickPurpose::VortexHover, intent.x, intent.y).map(|cursor| WorldInteractionActive::ObjectPick { cursor, retirement: None })
                    } else if component_mode_active(selection_state) {
                        WorldComponentPickCursor::new(session_state, camera_state, selection_state, generation, WorldComponentPickPurpose::Hover, intent.x, intent.y).map(|cursor| WorldInteractionActive::ComponentPick { cursor, retirement: None })
                    } else {
                        WorldRayPickCursor::new(session_state, camera_state, generation, WorldRayPickPurpose::Hover, intent.x, intent.y).map(|cursor| WorldInteractionActive::Pick { cursor, retirement: None })
                    }
                }
            }
            WorldInteractionPhase::PointerButton => {
                if intent.button == 0 && (world3d_relocate_active(gumball_state) || world3d_relocate_mode(session_state)) {
                    if intent.down {
                        if world3d_begin_relocate_drag(session_state, camera_state, selection_state, gumball_state, scene_state, intent.x, intent.y) {
                            self.queue.retire_front(intent.generation);
                            context.consume_fuel(1);
                            return WorldInteractionAuthorityStep::Complete;
                        }
                    } else if world3d_relocate_active(gumball_state) {
                        let commit = world3d_end_relocate_drag(session_state, camera_state, gumball_state, scene_state, Some((intent.x, intent.y)));
                        match commit.and_then(|(object_id, position)| plan_world3d_relocate(session_state, generation, &object_id, position)) {
                            Some(plan) => {
                                self.active = Some(WorldInteractionActive::Plan { plan, retirement: None });
                                context.consume_fuel(1);
                                return WorldInteractionAuthorityStep::Pending;
                            }
                            None => {
                                self.queue.retire_front(intent.generation);
                                context.consume_fuel(1);
                                return WorldInteractionAuthorityStep::Complete;
                            }
                        }
                    }
                }
                if intent.down && intent.button == 0 && selection_state.interaction_mode == "paint" && !selection_state.paint_stroke_active {
                    // 🖌️ A press opens the paint gesture and dabs where it lands (a `paintAt` stream tick when the ray hits
                    // a mesh); the moves stream the drag's dabs and the release commits them as ONE edit.
                    selection_state.paint_stroke_active = true;
                    session_state.interaction_revision = session_state.interaction_revision.wrapping_add(1);
                    WorldRayPickCursor::new(session_state, camera_state, generation, WorldRayPickPurpose::Paint, intent.x, intent.y).map(|cursor| WorldInteractionActive::Pick { cursor, retirement: None })
                } else if let Some(plan) = (!intent.down).then(|| plan_world3d_paint_release(session_state, selection_state, generation, intent.button)).flatten() {
                    Some(WorldInteractionActive::Plan { plan, retirement: None })
                } else if !intent.down && intent.button == 0 && session_state.active_utility == "surfaceBrush" {
                    WorldRayPickCursor::new(session_state, camera_state, generation, WorldRayPickPurpose::Surface, intent.x, intent.y).map(|cursor| WorldInteractionActive::Pick { cursor, retirement: None })
                } else if intent.down && intent.button == 0 && (session_state.active_utility == "brush" || (session_state.active_utility == "select" && selection_state.granularity == "vertex")) {
                    WorldObjectPickCursor::new(session_state, camera_state, generation, WorldObjectPickPurpose::VortexSelect, intent.x, intent.y).map(|mut cursor| {
                        cursor.merge = world_merge_code(intent.shift, intent.ctrl, intent.meta);
                        WorldInteractionActive::ObjectPick { cursor, retirement: None }
                    })
                } else if !intent.down && intent.button == 0 && session_state.active_utility == "select" && component_mode_active(selection_state) {
                    WorldComponentPickCursor::new(session_state, camera_state, selection_state, generation, WorldComponentPickPurpose::Select, intent.x, intent.y).map(|mut cursor| {
                        cursor.merge = world_merge_code(intent.shift, intent.ctrl, intent.meta);
                        WorldInteractionActive::ComponentPick { cursor, retirement: None }
                    })
                } else if intent.down && intent.button == 0 && session_state.active_utility == "select" && component_mode_active(selection_state) {
                    self.queue.retire_front(intent.generation);
                    context.consume_fuel(1);
                    return WorldInteractionAuthorityStep::Complete;
                } else if intent.button == 0 && session_state.active_utility == "select" && !component_mode_active(selection_state) {
                    if intent.down {
                        self.queue.retire_front(intent.generation);
                        context.consume_fuel(1);
                        return WorldInteractionAuthorityStep::Complete;
                    }
                    WorldRayPickCursor::new(session_state, camera_state, generation, WorldRayPickPurpose::Instance, intent.x, intent.y).map(|mut cursor| {
                        cursor.merge = world_merge_code(intent.shift, intent.ctrl, intent.meta);
                        WorldInteractionActive::Pick { cursor, retirement: None }
                    })
                } else {
                    None
                }
            }
            WorldInteractionPhase::Cancel => None,
            WorldInteractionPhase::Close => {
                self.closing = true;
                self.queue.begin_close();
                None
            }
        };
        let Some(active) = active else {
            if intent.phase == WorldInteractionPhase::Close {
                return WorldInteractionAuthorityStep::Pending;
            }
            if intent.phase == WorldInteractionPhase::PointerLeave {
                self.queue.retire_front(intent.generation);
                context.consume_fuel(1);
                return WorldInteractionAuthorityStep::Complete;
            }
            if !world_pick_rect(session_state).contains(intent.x, intent.y) {
                self.queue.retire_front(intent.generation);
                context.consume_fuel(1);
                return WorldInteractionAuthorityStep::Complete;
            }
            self.faulted = true;
            return WorldInteractionAuthorityStep::Fault;
        };
        self.active = Some(active);
        context.consume_fuel(1);
        WorldInteractionAuthorityStep::Pending
    }

    /// 🧲️ A hover that hit NEITHER a mesh nor a sub-object pick target falls through to
    /// the reference-plane tier; one that hit a pick target has its answer already.
    fn step_active(&mut self, active: WorldInteractionActive, session_state: &mut WorldSessionState, camera_state: &mut WorldCameraState, selection_state: &mut WorldSelectionState, gumball_state: &mut WorldGumballState, interaction_state: &mut WorldInteractionState, scene_state: &mut WorldSceneState, generation: u64, input: &mut ui_wgpu::wgpu::InputState<ActionDescriptor>, context: &mut semio_framework_job::StepContext<'_>) -> WorldInteractionAuthorityStep {
        match active {
            WorldInteractionActive::Plan { mut plan, retirement } => {
                if let Some(outcome) = retirement {
                    if plan.close_step() {
                        self.queue.retire_front(plan.generation);
                        return outcome;
                    }
                    self.active = Some(WorldInteractionActive::Plan { plan, retirement: Some(outcome) });
                    context.consume_fuel(1);
                    return WorldInteractionAuthorityStep::Pending;
                }
                match publish_world3d_plan_step(session_state, camera_state, selection_state, scene_state, &mut plan, generation, input, context) {
                    Ok(WorldInteractionStep::Pending) => {
                        self.active = Some(WorldInteractionActive::Plan { plan, retirement: None });
                        WorldInteractionAuthorityStep::Pending
                    }
                    Ok(WorldInteractionStep::Complete) => {
                        self.queue.retire_front(plan.generation);
                        WorldInteractionAuthorityStep::Complete
                    }
                    Ok(WorldInteractionStep::Stale) => {
                        self.active = Some(WorldInteractionActive::Plan { plan, retirement: Some(WorldInteractionAuthorityStep::Stale) });
                        WorldInteractionAuthorityStep::Pending
                    }
                    Ok(WorldInteractionStep::Fault) => {
                        self.active = Some(WorldInteractionActive::Plan { plan, retirement: Some(WorldInteractionAuthorityStep::Fault) });
                        self.faulted = true;
                        WorldInteractionAuthorityStep::Pending
                    }
                    Err(ui_wgpu::wgpu::BoundedActionFault::ItemCredits | ui_wgpu::wgpu::BoundedActionFault::ByteCredits) => {
                        self.active = Some(WorldInteractionActive::Plan { plan, retirement: None });
                        WorldInteractionAuthorityStep::OutputBlocked
                    }
                    Err(_) => {
                        self.active = Some(WorldInteractionActive::Plan { plan, retirement: Some(WorldInteractionAuthorityStep::Fault) });
                        self.faulted = true;
                        WorldInteractionAuthorityStep::Pending
                    }
                }
            }
            WorldInteractionActive::Pick { mut cursor, retirement } => {
                if let Some(outcome) = retirement {
                    if cursor.close_step() {
                        self.queue.retire_front(cursor.generation);
                        return outcome;
                    }
                    self.active = Some(WorldInteractionActive::Pick { cursor, retirement: Some(outcome) });
                    context.consume_fuel(1);
                    return WorldInteractionAuthorityStep::Pending;
                }
                match cursor.step(selection_state, gumball_state, scene_state, generation, context) {
                    WorldInteractionStep::Pending => {
                        self.active = Some(WorldInteractionActive::Pick { cursor, retirement: None });
                        WorldInteractionAuthorityStep::Pending
                    }
                    WorldInteractionStep::Complete if cursor.purpose == WorldRayPickPurpose::Hover && cursor.best.is_none() && cursor.best_target.is_none() => {
                        self.active = Some(WorldInteractionActive::ObjectPick { cursor: WorldObjectPickCursor::from_ray(cursor.revision, cursor.generation, WorldObjectPickPurpose::ReferenceHover, cursor.origin, cursor.direction), retirement: None });
                        context.consume_fuel(1);
                        WorldInteractionAuthorityStep::Pending
                    }
                    WorldInteractionStep::Complete if cursor.purpose == WorldRayPickPurpose::Instance && cursor.best.is_none() && cursor.best_target.is_none() => {
                        let mut reference = WorldObjectPickCursor::from_ray(cursor.revision, cursor.generation, WorldObjectPickPurpose::ReferenceSelect, cursor.origin, cursor.direction);
                        reference.merge = cursor.merge;
                        self.active = Some(WorldInteractionActive::ObjectPick { cursor: reference, retirement: None });
                        context.consume_fuel(1);
                        WorldInteractionAuthorityStep::Pending
                    }
                    WorldInteractionStep::Complete => match cursor.finish_plan(session_state, selection_state, interaction_state, scene_state, generation) {
                        Ok(Some(plan)) => {
                            self.active = Some(WorldInteractionActive::Plan { plan, retirement: None });
                            WorldInteractionAuthorityStep::Pending
                        }
                        Ok(None) => {
                            self.queue.retire_front(cursor.generation);
                            WorldInteractionAuthorityStep::Complete
                        }
                        Err(WorldInteractionStep::Pending) => {
                            self.active=Some(WorldInteractionActive::Pick{cursor,retirement:None});
                            context.consume_fuel(1);
                            WorldInteractionAuthorityStep::Pending
                        }
                        Err(outcome) => {
                            let outcome = if outcome == WorldInteractionStep::Stale { WorldInteractionAuthorityStep::Stale } else { WorldInteractionAuthorityStep::Fault };
                            self.active = Some(WorldInteractionActive::Pick { cursor, retirement: Some(outcome) });
                            WorldInteractionAuthorityStep::Pending
                        }
                    },
                    WorldInteractionStep::Stale => {
                        self.active = Some(WorldInteractionActive::Pick { cursor, retirement: Some(WorldInteractionAuthorityStep::Stale) });
                        WorldInteractionAuthorityStep::Pending
                    }
                    WorldInteractionStep::Fault => {
                        self.active = Some(WorldInteractionActive::Pick { cursor, retirement: Some(WorldInteractionAuthorityStep::Fault) });
                        self.faulted = true;
                        WorldInteractionAuthorityStep::Pending
                    }
                }
            }
            WorldInteractionActive::ObjectPick { mut cursor, retirement } => {
                if let Some(outcome) = retirement {
                    if cursor.close_step() {
                        self.queue.retire_front(cursor.generation);
                        return outcome;
                    }
                    self.active = Some(WorldInteractionActive::ObjectPick { cursor, retirement: Some(outcome) });
                    context.consume_fuel(1);
                    return WorldInteractionAuthorityStep::Pending;
                }
                match cursor.step(selection_state, gumball_state, scene_state, generation, context) {
                    WorldInteractionStep::Pending => {
                        self.active = Some(WorldInteractionActive::ObjectPick { cursor, retirement: None });
                        WorldInteractionAuthorityStep::Pending
                    }
                    WorldInteractionStep::Complete => match cursor.finish_plan(session_state, selection_state, interaction_state, scene_state, generation) {
                        Ok(Some(plan)) => {
                            self.active = Some(WorldInteractionActive::Plan { plan, retirement: None });
                            WorldInteractionAuthorityStep::Pending
                        }
                        Ok(None) => {
                            self.queue.retire_front(cursor.generation);
                            WorldInteractionAuthorityStep::Complete
                        }
                        Err(outcome) => {
                            let outcome = if outcome == WorldInteractionStep::Stale { WorldInteractionAuthorityStep::Stale } else { WorldInteractionAuthorityStep::Fault };
                            self.active = Some(WorldInteractionActive::ObjectPick { cursor, retirement: Some(outcome) });
                            WorldInteractionAuthorityStep::Pending
                        }
                    },
                    WorldInteractionStep::Stale => {
                        self.active = Some(WorldInteractionActive::ObjectPick { cursor, retirement: Some(WorldInteractionAuthorityStep::Stale) });
                        WorldInteractionAuthorityStep::Pending
                    }
                    WorldInteractionStep::Fault => {
                        self.active = Some(WorldInteractionActive::ObjectPick { cursor, retirement: Some(WorldInteractionAuthorityStep::Fault) });
                        self.faulted = true;
                        WorldInteractionAuthorityStep::Pending
                    }
                }
            }
            WorldInteractionActive::ComponentPick { mut cursor, retirement } => {
                if let Some(outcome) = retirement {
                    if cursor.close_step() {
                        self.queue.retire_front(cursor.generation);
                        return outcome;
                    }
                    self.active = Some(WorldInteractionActive::ComponentPick { cursor, retirement: Some(outcome) });
                    context.consume_fuel(1);
                    return WorldInteractionAuthorityStep::Pending;
                }
                match cursor.step(selection_state, gumball_state, scene_state, generation, context) {
                    WorldInteractionStep::Pending => {
                        self.active = Some(WorldInteractionActive::ComponentPick { cursor, retirement: None });
                        WorldInteractionAuthorityStep::Pending
                    }
                    WorldInteractionStep::Complete => match cursor.finish_plan(session_state, selection_state, interaction_state, scene_state, generation) {
                        Ok(Some(plan)) => {
                            self.active = Some(WorldInteractionActive::Plan { plan, retirement: None });
                            WorldInteractionAuthorityStep::Pending
                        }
                        Ok(None) => {
                            self.queue.retire_front(cursor.generation);
                            WorldInteractionAuthorityStep::Complete
                        }
                        Err(outcome) => {
                            let outcome = if outcome == WorldInteractionStep::Stale { WorldInteractionAuthorityStep::Stale } else { WorldInteractionAuthorityStep::Fault };
                            self.active = Some(WorldInteractionActive::ComponentPick { cursor, retirement: Some(outcome) });
                            WorldInteractionAuthorityStep::Pending
                        }
                    },
                    WorldInteractionStep::Stale => {
                        self.active = Some(WorldInteractionActive::ComponentPick { cursor, retirement: Some(WorldInteractionAuthorityStep::Stale) });
                        WorldInteractionAuthorityStep::Pending
                    }
                    WorldInteractionStep::Fault => {
                        self.active = Some(WorldInteractionActive::ComponentPick { cursor, retirement: Some(WorldInteractionAuthorityStep::Fault) });
                        self.faulted = true;
                        WorldInteractionAuthorityStep::Pending
                    }
                }
            }

            WorldInteractionActive::MarqueePick { mut cursor, retirement } => {
                if let Some(outcome) = retirement {
                    if cursor.close_step() {
                        self.queue.retire_front(cursor.generation);
                        return outcome;
                    }
                    self.active = Some(WorldInteractionActive::MarqueePick { cursor, retirement: Some(outcome) });
                    context.consume_fuel(1);
                    return WorldInteractionAuthorityStep::Pending;
                }
                match cursor.step(selection_state, gumball_state, scene_state, generation, context) {
                    WorldInteractionStep::Pending => {
                        self.active = Some(WorldInteractionActive::MarqueePick { cursor, retirement: None });
                        WorldInteractionAuthorityStep::Pending
                    }
                    WorldInteractionStep::Complete => {
                        let cursor_generation = cursor.generation;
                        let component_kind = cursor.component_kind;
                        match cursor.finish(session_state, gumball_state, generation) {
                            Ok((gesture, results)) => {
                                let Some(intent) = self.queue.front().copied().filter(|intent| intent.generation == cursor_generation) else {
                                    self.faulted = true;
                                    return WorldInteractionAuthorityStep::Fault;
                                };
                                self.active = if let Some(kind) = component_kind {
                                    Some(WorldInteractionActive::ComponentMarqueePublish { job: WorldComponentMarqueePublishJob::new(cursor_generation, gesture, results, kind, intent.shift, intent.ctrl), retirement: None })
                                } else {
                                    Some(WorldInteractionActive::MarqueePublish { job: WorldMarqueePublishJob::new(cursor_generation, gesture, results, intent.shift, intent.ctrl), retirement: None })
                                };
                                context.consume_fuel(1);
                                WorldInteractionAuthorityStep::Pending
                            }
                            Err((cursor, outcome)) => {
                                let outcome = if outcome == WorldInteractionStep::Stale { WorldInteractionAuthorityStep::Stale } else { WorldInteractionAuthorityStep::Fault };
                                self.active = Some(WorldInteractionActive::MarqueePick { cursor, retirement: Some(outcome) });
                                WorldInteractionAuthorityStep::Pending
                            }
                        }
                    }
                    WorldInteractionStep::Stale => {
                        self.active = Some(WorldInteractionActive::MarqueePick { cursor, retirement: Some(WorldInteractionAuthorityStep::Stale) });
                        WorldInteractionAuthorityStep::Pending
                    }
                    WorldInteractionStep::Fault => {
                        self.active = Some(WorldInteractionActive::MarqueePick { cursor, retirement: Some(WorldInteractionAuthorityStep::Fault) });
                        self.faulted = true;
                        WorldInteractionAuthorityStep::Pending
                    }
                }
            }
            WorldInteractionActive::MarqueePublish { mut job, retirement } => {
                if let Some(outcome) = retirement {
                    if job.close_step(input) {
                        self.queue.retire_front(job.generation);
                        return outcome;
                    }
                    self.active = Some(WorldInteractionActive::MarqueePublish { job, retirement: Some(outcome) });
                    context.consume_fuel(1);
                    return WorldInteractionAuthorityStep::Pending;
                }
                match job.step(selection_state, gumball_state, scene_state, generation, input, context) {
                    Ok(WorldInteractionStep::Pending) => {
                        self.active = Some(WorldInteractionActive::MarqueePublish { job, retirement: None });
                        WorldInteractionAuthorityStep::Pending
                    }
                    Ok(WorldInteractionStep::Complete) => {
                        self.queue.retire_front(job.generation);
                        WorldInteractionAuthorityStep::Complete
                    }
                    Ok(WorldInteractionStep::Stale) => {
                        self.active = Some(WorldInteractionActive::MarqueePublish { job, retirement: Some(WorldInteractionAuthorityStep::Stale) });
                        WorldInteractionAuthorityStep::Pending
                    }
                    Ok(WorldInteractionStep::Fault) => {
                        self.active = Some(WorldInteractionActive::MarqueePublish { job, retirement: Some(WorldInteractionAuthorityStep::Fault) });
                        self.faulted = true;
                        WorldInteractionAuthorityStep::Pending
                    }
                    Err(ui_wgpu::wgpu::BoundedActionFault::ItemCredits | ui_wgpu::wgpu::BoundedActionFault::ByteCredits) if job.prepared.is_none() => {
                        self.active = Some(WorldInteractionActive::MarqueePublish { job, retirement: None });
                        WorldInteractionAuthorityStep::OutputBlocked
                    }
                    Err(_) => {
                        self.active = Some(WorldInteractionActive::MarqueePublish { job, retirement: Some(WorldInteractionAuthorityStep::Fault) });
                        self.faulted = true;
                        WorldInteractionAuthorityStep::Pending
                    }
                }
            }
            WorldInteractionActive::ComponentMarqueePublish { mut job, retirement } => {
                if let Some(outcome) = retirement {
                    if job.close_step(input) {
                        self.queue.retire_front(job.generation);
                        return outcome;
                    }
                    self.active = Some(WorldInteractionActive::ComponentMarqueePublish { job, retirement: Some(outcome) });
                    context.consume_fuel(1);
                    return WorldInteractionAuthorityStep::Pending;
                }
                match job.step(selection_state, gumball_state, scene_state, generation, input, context) {
                    Ok(WorldInteractionStep::Pending) => {
                        self.active = Some(WorldInteractionActive::ComponentMarqueePublish { job, retirement: None });
                        WorldInteractionAuthorityStep::Pending
                    }
                    Ok(WorldInteractionStep::Complete) => {
                        self.queue.retire_front(job.generation);
                        WorldInteractionAuthorityStep::Complete
                    }
                    Ok(WorldInteractionStep::Stale) => {
                        self.active = Some(WorldInteractionActive::ComponentMarqueePublish { job, retirement: Some(WorldInteractionAuthorityStep::Stale) });
                        WorldInteractionAuthorityStep::Pending
                    }
                    Ok(WorldInteractionStep::Fault) => {
                        self.active = Some(WorldInteractionActive::ComponentMarqueePublish { job, retirement: Some(WorldInteractionAuthorityStep::Fault) });
                        self.faulted = true;
                        WorldInteractionAuthorityStep::Pending
                    }
                    Err(ui_wgpu::wgpu::BoundedActionFault::ItemCredits | ui_wgpu::wgpu::BoundedActionFault::ByteCredits) if job.claim.is_none() => {
                        self.active = Some(WorldInteractionActive::ComponentMarqueePublish { job, retirement: None });
                        WorldInteractionAuthorityStep::OutputBlocked
                    }
                    Err(_) => {
                        self.active = Some(WorldInteractionActive::ComponentMarqueePublish { job, retirement: Some(WorldInteractionAuthorityStep::Fault) });
                        self.faulted = true;
                        WorldInteractionAuthorityStep::Pending
                    }
                }
            }
            WorldInteractionActive::GumballPick { mut cursor, retirement } => {
                if let Some(outcome) = retirement {
                    if cursor.close_step() {
                        self.queue.retire_front(cursor.generation);
                        return outcome;
                    }
                    self.active = Some(WorldInteractionActive::GumballPick { cursor, retirement: Some(outcome) });
                    context.consume_fuel(1);
                    return WorldInteractionAuthorityStep::Pending;
                }
                match cursor.step(selection_state, gumball_state, scene_state, generation, context) {
                    WorldInteractionStep::Pending => {
                        self.active = Some(WorldInteractionActive::GumballPick { cursor, retirement: None });
                        WorldInteractionAuthorityStep::Pending
                    }
                    WorldInteractionStep::Complete => {
                        let cursor_generation = cursor.generation;
                        let cursor_revision = cursor.revision;
                        let point = [cursor.x, cursor.y];
                        match cursor.finish(session_state, gumball_state, generation) {
                            Ok(Some(gesture)) => {
                                self.gumball = Some(gesture);
                                self.queue.retire_front(cursor_generation);
                                WorldInteractionAuthorityStep::Complete
                            }
                            Ok(None) => {
                                self.marquee = Some(WorldMarqueeGesture::new(cursor_revision, cursor_generation, point));
                                self.queue.retire_front(cursor_generation);
                                WorldInteractionAuthorityStep::Complete
                            }
                            Err((cursor, outcome)) => {
                                let outcome = if outcome == WorldInteractionStep::Stale { WorldInteractionAuthorityStep::Stale } else { WorldInteractionAuthorityStep::Fault };
                                self.active = Some(WorldInteractionActive::GumballPick { cursor, retirement: Some(outcome) });
                                WorldInteractionAuthorityStep::Pending
                            }
                        }
                    }
                    WorldInteractionStep::Stale => {
                        self.active = Some(WorldInteractionActive::GumballPick { cursor, retirement: Some(WorldInteractionAuthorityStep::Stale) });
                        WorldInteractionAuthorityStep::Pending
                    }
                    WorldInteractionStep::Fault => {
                        self.active = Some(WorldInteractionActive::GumballPick { cursor, retirement: Some(WorldInteractionAuthorityStep::Fault) });
                        self.faulted = true;
                        WorldInteractionAuthorityStep::Pending
                    }
                }
            }
            WorldInteractionActive::GumballCommit { mut job, retirement } => {
                if let Some(outcome) = retirement {
                    if job.close_step(input) {
                        self.queue.retire_front(job.generation);
                        return outcome;
                    }
                    self.active = Some(WorldInteractionActive::GumballCommit { job, retirement: Some(outcome) });
                    context.consume_fuel(1);
                    return WorldInteractionAuthorityStep::Pending;
                }
                match job.step(selection_state, gumball_state, scene_state, generation, input, context) {
                    Ok(WorldInteractionStep::Pending) => {
                        self.active = Some(WorldInteractionActive::GumballCommit { job, retirement: None });
                        WorldInteractionAuthorityStep::Pending
                    }
                    Ok(WorldInteractionStep::Complete) if job.terminal_is_empty() => {
                        self.queue.retire_front(job.generation);
                        WorldInteractionAuthorityStep::Complete
                    }
                    Ok(WorldInteractionStep::Complete) => {
                        self.active = Some(WorldInteractionActive::GumballCommit { job, retirement: None });
                        WorldInteractionAuthorityStep::Pending
                    }
                    Ok(WorldInteractionStep::Stale) => {
                        self.active = Some(WorldInteractionActive::GumballCommit { job, retirement: Some(WorldInteractionAuthorityStep::Stale) });
                        WorldInteractionAuthorityStep::Pending
                    }
                    Ok(WorldInteractionStep::Fault) => {
                        self.active = Some(WorldInteractionActive::GumballCommit { job, retirement: Some(WorldInteractionAuthorityStep::Fault) });
                        self.faulted = true;
                        WorldInteractionAuthorityStep::Pending
                    }
                    Err(ui_wgpu::wgpu::BoundedActionFault::ItemCredits | ui_wgpu::wgpu::BoundedActionFault::ByteCredits) if job.claim.is_none() => {
                        self.active = Some(WorldInteractionActive::GumballCommit { job, retirement: None });
                        WorldInteractionAuthorityStep::OutputBlocked
                    }
                    Err(_) => {
                        self.active = Some(WorldInteractionActive::GumballCommit { job, retirement: Some(WorldInteractionAuthorityStep::Fault) });
                        self.faulted = true;
                        WorldInteractionAuthorityStep::Pending
                    }
                }
            }
            WorldInteractionActive::BrushCommit { mut job, retirement } => {
                if let Some(outcome) = retirement {
                    if job.close_step(input) {
                        self.queue.retire_front(job.generation);
                        return outcome;
                    }
                    self.active = Some(WorldInteractionActive::BrushCommit { job, retirement: Some(outcome) });
                    context.consume_fuel(1);
                    return WorldInteractionAuthorityStep::Pending;
                }
                match job.step(selection_state, gumball_state, scene_state, generation, input, context) {
                    Ok(WorldInteractionStep::Pending) => {
                        self.active = Some(WorldInteractionActive::BrushCommit { job, retirement: None });
                        WorldInteractionAuthorityStep::Pending
                    }
                    Ok(WorldInteractionStep::Complete) => {
                        self.queue.retire_front(job.generation);
                        WorldInteractionAuthorityStep::Complete
                    }
                    Ok(WorldInteractionStep::Stale) => {
                        self.active = Some(WorldInteractionActive::BrushCommit { job, retirement: Some(WorldInteractionAuthorityStep::Stale) });
                        WorldInteractionAuthorityStep::Pending
                    }
                    Ok(WorldInteractionStep::Fault) => {
                        self.active = Some(WorldInteractionActive::BrushCommit { job, retirement: Some(WorldInteractionAuthorityStep::Fault) });
                        self.faulted = true;
                        WorldInteractionAuthorityStep::Pending
                    }
                    Err(ui_wgpu::wgpu::BoundedActionFault::ItemCredits | ui_wgpu::wgpu::BoundedActionFault::ByteCredits) if job.claim.is_none() => {
                        self.active = Some(WorldInteractionActive::BrushCommit { job, retirement: None });
                        WorldInteractionAuthorityStep::OutputBlocked
                    }
                    Err(_) => {
                        self.active = Some(WorldInteractionActive::BrushCommit { job, retirement: Some(WorldInteractionAuthorityStep::Fault) });
                        self.faulted = true;
                        WorldInteractionAuthorityStep::Pending
                    }
                }
            }
        }
    }
}

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
fn plan_world3d_pointer_leave(session_state: &WorldSessionState, selection_state: &WorldSelectionState, generation: u64) -> Option<WorldInteractionPlan> {
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
fn plan_world3d_non_primary_press_pick(session_state: &WorldSessionState, camera_state: &WorldCameraState, selection_state: &WorldSelectionState, generation: u64, intent: &WorldInteractionIntent) -> Option<WorldInteractionActive> {
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

pub fn step_world3d_draw_rebuild(session_state: &mut WorldSessionState, authority_state: &mut WorldAuthorityState, scene_state: &mut WorldSceneState, context: &mut semio_framework_job::StepContext<'_>) -> WorldDrawRebuildStep {
    if context.should_yield() {
        return WorldDrawRebuildStep::Pending;
    }
    if let Some(retired) = scene_state.retired_draws.as_mut() {
        let Some(entry) = retired.take_last() else {
            scene_state.retired_draws = None;
            context.consume_fuel(1);
            return WorldDrawRebuildStep::Pending;
        };
        if let Err(owner) = quarantine_world_owner(WorldOpaqueOwner::Draw(entry)) {
            let WorldOpaqueOwner::Draw(entry) = owner else { unreachable!("retired draw owner") };
            scene_state.retired_draws.as_mut().expect("retired draw registry remains owned").restore_last(entry);
            mark_world_dynamic_fault(session_state, authority_state, scene_state, WorldDynamicFault::QuarantineCapacity);
            return WorldDrawRebuildStep::Fault;
        }
        context.consume_fuel(1);
        return WorldDrawRebuildStep::Pending;
    }
    let Some(mut cursor) = scene_state.draw_rebuild.take() else {
        return WorldDrawRebuildStep::Complete;
    };
    if cursor.descriptor.revision != session_state.interaction_revision || cursor.descriptor.generation != scene_state.draw_generation.wrapping_add(1) {
        scene_state.draw_rebuild = Some(cursor);
        return WorldDrawRebuildStep::Stale;
    }
    if !cursor.publishing || cursor.faulted {
        scene_state.draw_rebuild = Some(cursor);
        return WorldDrawRebuildStep::Pending;
    }
    if let Some(output) = cursor.output.as_mut() {
        let draft = cursor.drafts[usize::from(cursor.publish_draw)].as_mut().expect("publishing draw draft");
        if let Some(instance) = draft.take_next() {
            output.instances.push(instance);
            scene_state.draw_rebuild = Some(cursor);
            context.consume_fuel(1);
            return WorldDrawRebuildStep::Pending;
        }
        let output = cursor.output.take().expect("completed draw output");
        if let Err(rejected) = cursor.staged.push_prevalidated(output) {
            retain_world_blocked_owner(scene_state, WorldOpaqueOwner::Draw(WorldDynamicEntry { id: rejected.id, epoch: 0, value: rejected.value }));
            cursor.faulted = true;
            scene_state.draw_rebuild = Some(cursor);
            mark_world_dynamic_fault(session_state, authority_state, scene_state, rejected.fault);
            return WorldDrawRebuildStep::Fault;
        }
        let draft = cursor.drafts[usize::from(cursor.publish_draw)].take().expect("terminal draw draft");
        assert!(draft.terminal_is_empty());
        drop(draft);
        cursor.publish_draw += 1;
        scene_state.draw_rebuild = Some(cursor);
        context.consume_fuel(1);
        return WorldDrawRebuildStep::Pending;
    }
    if cursor.publish_draw < cursor.descriptor.draw_count {
        let draft = cursor.drafts[usize::from(cursor.publish_draw)].as_ref().expect("sealed draw draft");
        cursor.output = Some(SceneDraw3d { mesh_key: draft.mesh_key.clone(), mesh_version: draft.mesh_version, instances: Vec::with_capacity(draft.admitted), shadow_role: Default::default() });
        scene_state.draw_rebuild = Some(cursor);
        context.consume_fuel(1);
        return WorldDrawRebuildStep::Pending;
    }
    let staged = std::mem::take(&mut cursor.staged);
    let previous = std::mem::replace(&mut scene_state.draws, staged);
    if !previous.is_empty() {
        scene_state.retired_draws = Some(previous);
    }
    scene_state.draw_generation = cursor.descriptor.generation;
    assert!(cursor.terminal_is_empty());
    context.consume_fuel(1);
    WorldDrawRebuildStep::Complete
}
