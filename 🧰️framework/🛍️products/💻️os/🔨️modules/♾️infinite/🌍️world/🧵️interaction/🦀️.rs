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
use semio_framework_ui_declaration::layout::ActionDescriptor;
use ui_scene::math::{axis_rotate_angle, Camera3d, gumball_eye, gumball_project_ray_onto_axis, Instance3d, interpolate_mesh_uv, LineVertex3d, marquee_is_crossing_from_path, Mat4, Mesh3dField, Mesh3dLease, ray_aabb_slab, ray_plane_point, SceneDraw3d, Vec3};
use semio_framework_ui_wgpu_input::{HitKind, HitTarget, PointerModifiers};
use semio_framework_ui_theme::geometry::Rect;
use semio_framework_ui_theme::Rgba;
use semio_framework_ui_raster_owner::{SceneRasterBegin, SceneRasterDescriptor, SceneRasterLease, SceneRasterProfile, SceneRasterWriteMode};
use semio_framework_ui_declaration::ui::{UiComponentSceneNode, World3dSnapshotFault};
use semio_framework::{GranularityDefinition, HierarchyProvider, HoverSpec, InteractionDefinition, MergeMode, SelectionMethod, SelectionMode, SelectionSpec};
use std::collections::{HashMap, HashSet};
use semio_framework_ui_locale::LocalizedLabel;
use std::sync::Mutex;
use semio_framework_value::RetainedCloneGrant;
#[cfg(not(all(target_arch = "wasm32", target_env = "p2")))] use ui_scene::math::{SceneAuthoredMaterial3d, SceneLighting3d, SceneMaterialAlpha3d};
#[cfg(any(test, feature = "testkit"))] use semio_framework_ui_draw_contract::gizmo;
#[cfg(any(test, feature = "testkit"))] use ui_scene::math::{ray_pick_instance, ray_pick_mesh_detail, screen_select_components, screen_select_instances};
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

#[path = "🖱️pointer-and-picking/🦀️.rs"]
mod pointer_and_picking;
pub use pointer_and_picking::*;

#[path = "🌪️vortex-relocate-and-terrain/🦀️.rs"]
mod vortex_relocate_and_terrain;
pub use vortex_relocate_and_terrain::*;

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

pub fn close_world3d_interaction_step(authority_state: &mut WorldAuthorityState, input: &mut semio_framework_ui_wgpu_input::InputState<ActionDescriptor>, context: &mut semio_framework_job::StepContext<'_>) -> bool {
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

#[path = "⚙️authority-core/🦀️.rs"]
mod authority_core;
use authority_core::*;

#[path = "📜️plan-and-publish/🦀️.rs"]
mod plan_and_publish;
pub use plan_and_publish::*;

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
