//! 🗺️ Tiled map host, tile fetch and interaction.
//!
//! Tiled map tile fetch reservation, the map host accessors, camera and pointer interaction.

use semio_framework_os_renderer_wgpu_engine_registry::engine_surface_registry::{
    engine_surface_wire_id, MapSyncCache, ENGINE_SURFACES, MAP_TILE_ASSET_FAULT,
};
use crate::element_contracts::ScenePointerTarget;
use infinite_world::world::{WorldAssetFault, WorldAssetMetadataId, WorldAssetRequestKind};
use semio_framework_surface::tiled_map::{tiles as map_tiles, MapHost, MapInteractionIntent};
use serde_json::Value;
use ui_wgpu::wgpu::{ActionDescriptor, Rect};

//#region TiledMap

pub fn take_map_tile_asset_fault() -> Option<WorldAssetFault> {
    MAP_TILE_ASSET_FAULT.with(|cell| cell.borrow_mut().take())
}

/// 🗺️🔗️ `"{z}/{x}/{y}"` substituted into a tile URL template — React's own
/// `urlTemplate.replace("{z}", …)` chain in `MapRenderer.uploadTileRow`.
pub fn map_tile_url(template: &str, z: u32, x: u32, y: u32) -> String {
    template.replace("{z}", &z.to_string()).replace("{x}", &x.to_string()).replace("{y}", &y.to_string())
}

/// 🗺️📡️ Admits ONE visible-but-unheld tile into the bounded renderer-asset pipeline as
/// `WorldAssetRequestKind::MapTile`. The renderer host drains the shared lane
/// (`take_renderer_asset_step`) and hands the bytes back through [`apply_map_tile_bytes`], which is
/// where `MapHost::upload_tile`/`upload_vector_tile` is finally called — the twin of React's
/// `fetch(url)` + `session.uploadTile(...)` pair.
///
/// Back-pressure is not a fault: a full request table leaves the tile un-pended so the next frame
/// re-offers it. Only a structural refusal records a miss.
///
/// @see `🧱️elements/🧭️TiledMapHost/🟦️.tsx` — `MapRenderer.uploadTileRow`
fn reserve_map_tile_fetch(surface: WorldAssetMetadataId, cache: &mut MapSyncCache, held: bool, template: &str, vector: bool, (z, x, y): (u32, u32, u32)) {
    let slot = (vector, z, x, y);
    if held || template.is_empty() || cache.tile_pending.contains(&slot) || cache.tile_misses.contains(&slot) {
        return;
    }
    let Ok(key) = WorldAssetMetadataId::try_from_str(&map_tiles::tile_key(z, x, y)) else {
        cache.tile_misses.insert(slot);
        return;
    };
    let url = map_tile_url(template, z, x, y);
    match crate::asset_authority::reserve_renderer_asset_request(WorldAssetRequestKind::MapTile { surface, key, vector, z, x, y }, &url) {
        Ok(_) => {
            cache.tile_pending.insert(slot);
        }
        Err(WorldAssetFault::ItemCapacity | WorldAssetFault::ByteCapacity | WorldAssetFault::Closing | WorldAssetFault::Stale) => {}
        Err(fault) => {
            cache.tile_misses.insert(slot);
            MAP_TILE_ASSET_FAULT.with(|cell| *cell.borrow_mut() = Some(fault));
        }
    }
}

/// 🗺️📡️ Offers every tile of the live raster and vector visible ranges, obeying the host's own
/// render mode (`image`/`vector`/`combined`) exactly as React's `needsRasterTiles`/
/// `needsVectorTiles` gates do, and clearing a lane's miss set when that lane's visible-range
/// revision changes (React's `refreshRasterTiles`/`refreshVectorTiles` do the same).
pub fn reserve_map_tile_fetches(surface_id: &str, host: &MapHost, cache: &mut MapSyncCache, map: &ui_wgpu::wgpu::TiledMapScene) {
    let Ok(surface) = WorldAssetMetadataId::try_from_str(surface_id) else { return };
    let mode = host.render_mode_str();
    if matches!(mode, "image" | "combined") {
        let revision = host.visible_tiles_revision();
        if cache.raster_tiles_revision != Some(revision) {
            cache.raster_tiles_revision = Some(revision);
            cache.tile_misses.retain(|(vector, ..)| *vector);
        }
        let mut cursor = host.visible_raster_tile_cursor();
        while let Some(tile) = cursor.peek() {
            let held = host.has_tile(&map_tiles::tile_key(tile.z, tile.x, tile.y));
            reserve_map_tile_fetch(surface, cache, held, &map.tile_url_template, false, (tile.z, tile.x, tile.y));
            cursor.advance();
        }
    }
    if !matches!(mode, "vector" | "combined") {
        return;
    }
    let revision = host.visible_vector_tiles_revision();
    if cache.vector_tiles_revision != Some(revision) {
        cache.vector_tiles_revision = Some(revision);
        cache.tile_misses.retain(|(vector, ..)| !*vector);
    }
    let Some(mut cursor) = host.visible_vector_tile_cursor() else { return };
    while let Some(tile) = cursor.peek() {
        let held = host.has_vector_tile(&map_tiles::tile_key(tile.z, tile.x, tile.y));
        reserve_map_tile_fetch(surface, cache, held, &map.vector_tile_url_template, true, (tile.z, tile.x, tile.y));
        cursor.advance();
    }
}

/// 🗺️📥️ Hands one fetched tile to the surface's `MapHost` and clears its pending slot — the wgpu
/// twin of `session.uploadTile`/`uploadVectorTile`. Bytes the host refuses record a miss, so the
/// tile is not re-requested until its lane's visible range changes.
pub fn apply_map_tile_bytes(kind: WorldAssetRequestKind, bytes: &[u8]) {
    let WorldAssetRequestKind::MapTile { surface, key: _, vector, z, x, y } = kind else { return };
    ENGINE_SURFACES.with(|cell| {
        let mut map = cell.borrow_mut();
        let Some(entry) = map.get_mut(surface.as_str()) else {
            return;
        };
        let slot = (vector, z, x, y);
        entry.map_sync_cache.tile_pending.remove(&slot);
        let Some(host) = entry.map_host.as_mut() else {
            return;
        };
        let uploaded = if vector { host.upload_vector_tile(z, x, y, bytes) } else { host.upload_tile(z, x, y, bytes) };
        if uploaded.is_err() {
            entry.map_sync_cache.tile_misses.insert(slot);
            return;
        }
        entry.scene_revision = entry.scene_revision.wrapping_add(1);
    });
}

pub fn with_map_host_mut<R>(surface_id: &str, f: impl FnOnce(&mut MapHost) -> R) -> Option<R> {
    ENGINE_SURFACES.with(|cell| {
        let mut map = cell.borrow_mut();
        let entry = map.get_mut(surface_id)?;
        let host = entry.map_host.as_mut()?;
        Some(f(host))
    })
}

pub fn with_map_host<R>(surface_id: &str, f: impl FnOnce(&MapHost) -> R) -> Option<R> {
    ENGINE_SURFACES.with(|cell| {
        let map = cell.borrow();
        let entry = map.get(surface_id)?;
        let host = entry.map_host.as_ref()?;
        Some(f(host))
    })
}

/// 🤏️ Reads one map camera for shared touch math.
pub fn tiled_map_camera(surface_id: &str) -> Option<[f64; 3]> {
    with_map_host(surface_id, MapHost::camera)
}

/// 🤏️ Applies an ephemeral map camera during a pinch without publishing guest state.
pub fn tiled_map_set_camera_silent(surface_id: &str, camera: [f64; 3]) -> bool {
    with_map_host_mut(surface_id, |host| host.set_camera_silent(camera[0], camera[1], camera[2])).is_some()
}

/// 🏁️ Publishes the settled map camera once when the last pinch contact lifts.
pub fn tiled_map_publish_camera_into(surface_id: &str, controller_id: &str, input: &mut ui_wgpu::wgpu::InputState<ActionDescriptor>) -> Result<bool, ui_wgpu::wgpu::BoundedActionFault> {
    let Some(camera) = tiled_map_camera(surface_id) else { return Ok(false) };
    let Some(wire_surface_id) = engine_surface_wire_id(surface_id) else { return Ok(false) };
    let mut reservation = input.reserve_actions(1, ui_wgpu::wgpu::action::ACTION_ITEM_BYTE_CAPACITY)?;
    write_map_camera_action(&mut reservation, wire_surface_id.as_str(), controller_id, camera)?;
    reservation.publish()?;
    Ok(true)
}

pub fn stamp_map_interaction_owner(surface_id: &str, owner: &ScenePointerTarget) -> bool {
    ENGINE_SURFACES.with(|cell| {
        let mut map = cell.borrow_mut();
        let Some(entry) = map.get_mut(surface_id).filter(|entry| entry.map_host.is_some()) else { return false };
        entry.map_interaction_owner = Some(owner.clone());
        true
    })
}

pub fn retire_map_interaction_owner(owner: &ScenePointerTarget) -> bool {
    ENGINE_SURFACES.with(|cell| {
        let mut map = cell.borrow_mut();
        let Some(entry) = map.get_mut(&owner.host_id) else { return false };
        if !entry.map_interaction_owner.as_ref().is_some_and(|current| current.same_component_host(owner)) {
            return false;
        }
        entry.map_interaction_owner = None;
        if let Some(host) = entry.map_host.as_mut() {
            host.cancel_interaction();
        }
        true
    })
}

pub fn map_local_pointer(inner: Rect, x: f32, y: f32) -> (f64, f64) {
    ((x - inner.x) as f64, (y - inner.y) as f64)
}

/// ✂️ A board pointer in surface-local pixels, mapped exactly as React's Board2dHost maps a DOM pointer: the position and
/// the surface origin each as the shortest decimal of their f32 value (`board2dFloat32Decimal`), then subtracted — so
/// both hosts hand the board engine the same coordinates and record the same gesture offsets
/// (`🖥️Board2dHost/🧫️fixtures/🧫️float32-decimal`).
pub fn board_local_pointer(inner: Rect, x: f32, y: f32) -> (f64, f64) {
    let decimal = |value: f32| infinite_canvas::board_pointer_offset(f64::from(value));
    (decimal(x) - decimal(inner.x), decimal(y) - decimal(inner.y))
}

/// 🎯️ Port of `marqueeModeFromModifiers` (ui `⚛️react` target) for a map marquee — the unmodified
/// case is `"replace"`, the `MergeMode` variant the guest actually declares; the `"default"` this
/// used to send is in no `MergeMode` taxonomy and never merged anything.
pub fn map_marquee_mode(shift: bool, ctrl_or_meta: bool) -> &'static str {
    if shift && ctrl_or_meta {
        "invertive"
    } else if shift {
        "additive"
    } else if ctrl_or_meta {
        "subtractive"
    } else {
        "replace"
    }
}

pub fn map_marquee_crossing(method: &str, start_x: f32, end_x: f32) -> bool {
    if method == "lasso" {
        end_x < start_x
    } else {
        end_x < start_x
    }
}

/// 🗂️ Every feature id of a rect/polygon hit document (`{ <pickable overlay layer id>: [ids] }`), layer by layer.
pub fn parse_map_feature_hit(hit_json: &str) -> Vec<String> {
    let hit: Value = serde_json::from_str(hit_json).unwrap_or(Value::Null);
    hit.as_object().into_iter().flat_map(|layers| layers.values()).filter_map(Value::as_array).flatten().filter_map(|row| row.as_str().map(str::to_string)).collect()
}

pub fn parse_map_hover(hit_json: &str) -> Value {
    if hit_json == "null" {
        return Value::Null;
    }
    serde_json::from_str(hit_json).unwrap_or(Value::Null)
}

/// 🗺️📷️ The ONE action a map pan/zoom gesture publishes. React's `mirrorSessionCameraToReact`
/// dispatches `setCamera` and nothing else: feature selection and hover ride the generic
/// `interactionSelect`/`interactionHover` verbs from the pointer lane (`🎞️Scenes/🎯️targets/🧊️wgpu`'s
/// `write_tiled_map_selection`/`write_tiled_map_hover`). The deleted `setFeatureSelection`/`setHover`
/// pair this used to also emit on every wheel tick has no handler left in any map app — a map app's
/// `"features"` interaction definition auto-injects the generic verbs instead.
fn write_map_camera_action(batch: &mut ui_wgpu::wgpu::BoundedActionBatchReservation<'_>, surface_id: &str, controller_id: &str, camera: [f64; 3]) -> Result<(), ui_wgpu::wgpu::BoundedActionFault> {
    let camera_action = ui_wgpu::wgpu::tiled_map_actions::SET_CAMERA;
    let camera_bytes = ui_wgpu::wgpu::checked_action_string_bytes(&[controller_id, camera_action, "surfaceId", surface_id, "camera", "x", "y", "zoom"])?;
    batch.action(controller_id, camera_action, camera_bytes, |builder| {
        builder.begin_object(None)?;
        builder.string(Some("surfaceId"), surface_id)?;
        builder.begin_object(Some("camera"))?;
        builder.number(Some("x"), camera[0])?;
        builder.number(Some("y"), camera[1])?;
        builder.number(Some("zoom"), camera[2])?;
        builder.end_container()?;
        builder.end_container()
    })
}

pub fn with_map_interaction_into(surface_id: &str, controller_id: &str, input: &mut ui_wgpu::wgpu::InputState<ActionDescriptor>, intent: MapInteractionIntent) -> Result<bool, ui_wgpu::wgpu::BoundedActionFault> {
    let mut reservation = input.reserve_actions(1, ui_wgpu::wgpu::action::ACTION_ITEM_BYTE_CAPACITY)?;
    let planned = ENGINE_SURFACES.with(|cell| {
        let map = cell.borrow();
        let host = map.get(surface_id).and_then(|entry| entry.map_host.as_ref())?;
        let plan = host.plan_interaction(intent);
        let camera = plan.camera();
        Some((plan, camera))
    });
    let Some((plan, camera)) = planned else {
        return Ok(false);
    };
    let Some(wire_surface_id) = engine_surface_wire_id(surface_id) else { return Ok(false) };
    write_map_camera_action(&mut reservation, wire_surface_id.as_str(), controller_id, camera)?;
    reservation.publish_with_checked(|| ENGINE_SURFACES.with(|cell| cell.borrow_mut().get_mut(surface_id).and_then(|entry| entry.map_host.as_mut()).is_some_and(|host| host.commit_interaction(plan))))?;
    Ok(true)
}

pub fn tiled_map_wheel_into(surface_id: &str, controller_id: &str, inner: Rect, x: f32, y: f32, delta: f32, ctrl: bool, input: &mut ui_wgpu::wgpu::InputState<ActionDescriptor>) -> Result<bool, ui_wgpu::wgpu::BoundedActionFault> {
    let (sx, sy) = map_local_pointer(inner, x, y);
    let delta_y = if ctrl { delta as f64 * 2.5 } else { delta as f64 };
    with_map_interaction_into(surface_id, controller_id, input, MapInteractionIntent::Wheel { sx, sy, delta_y })
}

//#endregion TiledMap
