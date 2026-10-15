//! 🧱️ World scene resources: pools, chunking, LOD grid, environment, terrain and placeholder meshes.

use super::*;

//#region Pool
pub(super) struct RefCountPool<K>
where
    K: Eq + std::hash::Hash + Clone,
{
    counts: HashMap<K, u32>,
}

impl<K> RefCountPool<K>
where
    K: Eq + std::hash::Hash + Clone,
{
    pub(super) fn new() -> Self {
        Self { counts: HashMap::new() }
    }

    pub(super) fn acquire(&mut self, key: K) {
        *self.counts.entry(key).or_insert(0) += 1;
    }

    pub(super) fn release(&mut self, key: &K) -> bool {
        match self.counts.get_mut(key) {
            Some(count) if *count > 1 => {
                *count -= 1;
                false
            }
            Some(_) => {
                self.counts.remove(key);
                true
            }
            None => false,
        }
    }

    pub(super) fn contains(&self, key: &K) -> bool {
        self.counts.contains_key(key)
    }

    pub(super) fn keys(&self) -> impl Iterator<Item = &K> {
        self.counts.keys()
    }
}
//#endregion Pool

//#region Chunking
pub(super) fn chunk_key_indices(position: [f64; 3], chunk_size: f64) -> (i64, i64, i64) {
    ((position[0] / chunk_size).floor() as i64, (position[1] / chunk_size).floor() as i64, (position[2] / chunk_size).floor() as i64)
}

pub(super) fn chunk_center(key: (i64, i64, i64), chunk_size: f64) -> Vec3 {
    let size = chunk_size as f32;
    Vec3::new((key.0 as f32 + 0.5) * size, (key.1 as f32 + 0.5) * size, (key.2 as f32 + 0.5) * size)
}

fn chunk_bounds_radius(chunk_size: f64) -> f64 {
    chunk_size * 0.866
}

pub(super) fn chunk_distance_visible(cam_pos: Vec3, chunk_center: Vec3, chunk_size: f64, max_dist: f64, was_visible: bool) -> bool {
    let bounds_r = chunk_bounds_radius(chunk_size);
    let dist = cam_pos.sub(chunk_center).length() as f64;
    let enter_dist = max_dist + bounds_r;
    let exit_dist = enter_dist + chunk_size * 0.5;
    if dist <= enter_dist {
        return true;
    }
    if was_visible && dist <= exit_dist {
        return true;
    }
    false
}

pub(super) fn update_visible_chunks(state: &mut World3dState, cam_pos: Vec3) {
    let Some(chunking) = state.chunking.clone() else {
        return;
    };
    let chunk_size = chunking.chunk_size;
    let max_distance = chunking.max_distance;
    let mut chunk_keys = HashSet::new();
    for position in state.instance_positions.values() {
        chunk_keys.insert(chunk_key_indices(*position, chunk_size));
    }
    let previous = state.visible_chunks.clone();
    let mut next_visible = HashSet::new();
    for key in chunk_keys.iter().chain(previous.iter()) {
        let center = chunk_center(*key, chunk_size);
        let was = previous.contains(key);
        if chunk_distance_visible(cam_pos, center, chunk_size, max_distance, was) {
            next_visible.insert(*key);
        }
    }
    state.visible_chunks = next_visible;
}

pub(super) fn instance_chunk_visible(state: &World3dState, position: [f64; 3]) -> bool {
    let Some(chunking) = &state.chunking else {
        return true;
    };
    let key = chunk_key_indices(position, chunking.chunk_size);
    state.visible_chunks.contains(&key)
}
//#endregion Chunking

//#region LodGrid
pub(super) const WORLD_LOD_EPSILON: f64 = 0.01;
/// 📐️ The most divisions one LOD band may submit as real line geometry. A finer band is DROPPED
/// rather than drawn at a distorted spacing — see [`append_lod_grid_lines`].
pub(super) const WORLD_GRID_MAX_DIVISIONS: i32 = 512;

pub(super) fn default_lod_record() -> WorldLodRecord {
    WorldLodRecord {
        automatic: true,
        manual: default_manual_lod(),
        distance_reference: default_distance_reference(),
        depth_variable: false,
        grid_factor: default_grid_factor(),
        grid_snap_enabled: false,
        show_grid: true,
        grid_datum: Some([0.0, 0.0, 0.0]),
    }
}

pub(super) fn scene_lod(state: &World3dState) -> f64 {
    let camera = state.orbit.to_camera();
    let distance = f64::from(lod_orbit_distance_for_camera(&camera, camera.position.sub(camera.target).length(), state.bounds.h));
    let auto_lod = lod_from_camera_distance(distance, state.lod.distance_reference);
    if state.lod.automatic || state.lod.depth_variable {
        auto_lod
    } else {
        state.lod.manual
    }
}

#[cfg(test)]
pub(super) fn resolve_physical_mesh_id(state: &World3dState, logical_id: &str, desired_lod: f64) -> String {
    if let Some(lods) = state.mesh_lod_catalog.get(logical_id) {
        let entries: Vec<(f64, &str)> = lods.iter().map(|entry| (entry.lod, entry.url.as_str())).collect();
        let fallback = state.mesh_url_fallback.get(logical_id).map(String::as_str);
        if let Some(url) = pick_closest_mesh_url(&entries, desired_lod, fallback) {
            return mesh_id_from_url(url);
        }
    }
    if let Some(url) = state.mesh_url_fallback.get(logical_id) {
        return mesh_id_from_url(url);
    }
    logical_id.to_string()
}

/// 📐️ The world grid, drawn the way React's `WorldLodGridHelper` draws it: ONE band at
/// `lodGridStepWorld`'s spacing, sized to `cameraGridFadeDistance` and faded over that same radius.
///
/// 🩸️ Two defects lived here. It first submitted a FIXED 12 000-unit square at every LOD band and
/// clamped the divisions to 512, so the drawn spacing silently stopped being the band's spacing.
/// W7b fixed the spacing but kept the four PROGRESSIVE bands, which React has no counterpart for —
/// at the puzzle 3d boot camera that stacked 100 · 25 · 5 · 1-unit grids where React paints a single
/// 10-unit one. React's grid is a shader plane with no finite edge; this is real line geometry, so
/// ⚖️ The ONE deliberate deviation left is the [`WORLD_GRID_MAX_DIVISIONS`] ceiling: a radius past it
/// is CLAMPED — and the fade curve is clamped WITH it, so the outermost line still reaches alpha 0
/// and there is no hard edge to see, which is why a shader plane and this read the same at every
/// camera the puzzle 3d panes reach. Spending the ceiling on the STEP instead was tried and reverted:
/// coarsening to cover `camera_grid_fade_distance` (which is React's `visibleRadius × 32` FADE
/// parameter, not a draw extent) walked the boot camera's band from 10 units to 10 000 and painted
/// four giant cells where React paints a 10-unit grid
/// (ticket 26/09/17/WGPU-RENDERER-REACT-PARITY, `📓️w9b-projection-pane-framing-grid-materials.md` §3).
pub(super) fn append_lod_grid_lines(line_vertices: &mut Vec<LineVertex3d>, lod: f64, grid_factor: f64, anchor: Vec3, camera: &Camera3d, viewport: Rect, base_color: [f32; 4]) {
    let Some(step_world) = lod_grid_step_world(lod, grid_factor) else { return };
    let step = step_world as f32;
    if !step.is_finite() || step <= 0.0 {
        return;
    }
    let fade = camera_grid_fade_distance(camera, anchor.z, step_world, viewport.w, viewport.h);
    let divisions = ((fade * 2.0 / step).round() as i32).min(WORLD_GRID_MAX_DIVISIONS);
    if divisions < 2 {
        return;
    }
    let half = step * divisions as f32 * 0.5;
    let z = anchor.z + 0.002;
    for index in 0..=divisions {
        let offset = -half + index as f32 * step;
        let alpha = base_color[3] * lod_grid_fade_alpha(offset.abs(), half);
        if alpha <= 0.0 {
            continue;
        }
        let color = [base_color[0], base_color[1], base_color[2], alpha];
        let rim = [color[0], color[1], color[2], 0.0];
        line_vertices.push(LineVertex3d { position: [anchor.x - half, anchor.y + offset, z], color: rim });
        line_vertices.push(LineVertex3d { position: [anchor.x, anchor.y + offset, z], color });
        line_vertices.push(LineVertex3d { position: [anchor.x, anchor.y + offset, z], color });
        line_vertices.push(LineVertex3d { position: [anchor.x + half, anchor.y + offset, z], color: rim });
        line_vertices.push(LineVertex3d { position: [anchor.x + offset, anchor.y - half, z], color: rim });
        line_vertices.push(LineVertex3d { position: [anchor.x + offset, anchor.y, z], color });
        line_vertices.push(LineVertex3d { position: [anchor.x + offset, anchor.y, z], color });
        line_vertices.push(LineVertex3d { position: [anchor.x + offset, anchor.y + half, z], color: rim });
    }
}

/// 🎨️ React's `MeshStyleKind` — the seven paint rows `MESH_STYLE_PAINT` declares
/// (`🌐️World3dHost/🟦️.tsx`), in React's own priority order, highest first.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum MeshStyleKind {
    Disabled,
    Provisional,
    Celebrated,
    Selected,
    Highlighted,
    Hovered,
    Neutral,
}

/// 🖍️ Resolves an original instance's semantic object flags for surface and primary geometry paint.
pub(super) fn world_instance_style(state: &World3dState, instance: &Instance3d) -> MeshStyleState {
    MeshStyleState {
        disabled: state.disabled_instance_ids.contains(&instance.id),
        provisional: state.provisional_instance_ids.contains(&instance.id),
        celebrating: state.celebrating_instance_ids.contains(&instance.id),
        selected: instance.selected,
        highlighted: state.highlighted_instance_ids.contains(&instance.id),
        hovered: instance.hovered,
    }
}

/// 🎨️ What an instance is, before a row is chosen for it — React's `resolveMeshStyle` argument
/// (`🌐️World3dHost/🟦️.tsx`).
#[derive(Clone, Copy, Debug, Default)]
pub struct MeshStyleState {
    pub disabled: bool,
    pub provisional: bool,
    pub celebrating: bool,
    pub selected: bool,
    pub highlighted: bool,
    pub hovered: bool,
}

/// 🎨️ One resolved row of React's `MESH_STYLE_PAINT`: its fill, its line, its emissive intensity
/// and its opacity, all in this renderer's linear paints.
#[derive(Clone, Copy, Debug)]
pub struct MeshStylePaint {
    pub fill: [f32; 4],
    pub line: [f32; 4],
    pub emissive_intensity: f32,
    pub opacity: f32,
    /// 🎉️ The three conic stops React's `CelebratingConicMaterial` spins through
    /// (`--color-primary` → `--color-secondary` → `--color-tertiary`). Only the `celebrated` row
    /// carries them; every other row paints its flat `fill`.
    pub conic: Option<[[f32; 4]; 3]>,
}

/// 🎉️ React's `CELEBRATE_CONIC_SPIN_SECONDS` (`🌐️World3dHost/🟦️.tsx`).
pub const CELEBRATE_CONIC_SPIN_SECONDS: f32 = 1.2;

/// 🎨️ React's whole `MESH_STYLE_PAINT` table, resolved against this appearance — ONE place both
/// renderers read their mesh paint from (`🌐️World3dHost/🟦️.tsx`'s table, verbatim).
///
/// 🩸️ Only two of the seven rows existed here: `selected` and `hovered`, resolved inline by an
/// `if/else if` with no table behind it. `provisional` was painted by a SECOND, unrelated rule in
/// `render_world_3d` (`tool_run_provisional_color`, which substituted `theme.accent` for React's
/// `--color-secondary` on the belief that this theme carries no secondary tone — it does, as
/// `theme.celebrate[1]`), and `celebrated`, `highlighted` and `disabled` had no wgpu paint at all,
/// so a disabled instance painted at full opacity where React paints it at 0.45
/// (`📓️w8b-orthographic-camera-and-3d-parity.md` §7.4).
///
/// 🎨️ React's `color-mix(in oklab, var(--color-muted-foreground) 55%, var(--panel))` — the
/// 55 % is the muted side, so the mix runs FROM the muted tone TOWARDS the panel by 0.45.
pub fn mesh_style_paint(theme: &ui_wgpu::wgpu::Theme, kind: MeshStyleKind) -> MeshStylePaint {
    let rgba = |color: Rgba| [color.r, color.g, color.b, color.a];
    let (primary, secondary, tertiary) = (rgba(theme.celebrate[0]), rgba(theme.celebrate[1]), rgba(theme.celebrate[2]));
    match kind {
        MeshStyleKind::Neutral => MeshStylePaint { fill: rgba(theme.panel), line: rgba(theme.border_normal), emissive_intensity: 0.0, opacity: 1.0, conic: None },
        MeshStyleKind::Hovered => MeshStylePaint { fill: rgba(theme.row_hover), line: rgba(theme.border_emphasized), emissive_intensity: 0.08, opacity: 1.0, conic: None },
        MeshStyleKind::Selected => MeshStylePaint { fill: primary, line: primary, emissive_intensity: 0.35, opacity: 1.0, conic: None },
        MeshStyleKind::Highlighted => MeshStylePaint { fill: secondary, line: secondary, emissive_intensity: 0.2, opacity: 1.0, conic: None },
        MeshStyleKind::Provisional => MeshStylePaint { fill: secondary, line: secondary, emissive_intensity: 0.2, opacity: ui_styling::metrics::tool_run::PROVISIONAL_OPACITY as f32, conic: None },
        MeshStyleKind::Celebrated => MeshStylePaint { fill: primary, line: primary, emissive_intensity: 0.55, opacity: 1.0, conic: Some([primary, secondary, tertiary]) },
        MeshStyleKind::Disabled => MeshStylePaint { fill: ui_styling::color::oklab_mix(rgba(theme.text_muted), rgba(theme.panel), 0.45), line: rgba(theme.text_muted), emissive_intensity: 0.0, opacity: 0.45, conic: None },
    }
}

/// 🎨️ React's `resolveMeshStyle` — the one priority ladder both renderers pick a row with.
pub fn resolve_mesh_style(state: MeshStyleState) -> MeshStyleKind {
    if state.disabled {
        return MeshStyleKind::Disabled;
    }
    if state.provisional {
        return MeshStyleKind::Provisional;
    }
    if state.celebrating {
        return MeshStyleKind::Celebrated;
    }
    if state.selected {
        return MeshStyleKind::Selected;
    }
    if state.highlighted {
        return MeshStyleKind::Highlighted;
    }
    if state.hovered {
        return MeshStyleKind::Hovered;
    }
    MeshStyleKind::Neutral
}

/// 🎨️ Resolves React's retained material row while keeping semantic, authored, and environment
/// colour authority distinct across live theme changes.
pub(super) fn world3d_style_paint(theme: &ui_wgpu::wgpu::Theme, style: MeshStyleState, has_vertex_colors: bool, mut instance: Instance3d) -> Instance3d {
    let kind = resolve_mesh_style(style);
    let preserve_vertex_color = has_vertex_colors && matches!(kind, MeshStyleKind::Neutral | MeshStyleKind::Disabled);
    instance.material.preserve_vertex_color = preserve_vertex_color;
    if kind == MeshStyleKind::Neutral {
        if preserve_vertex_color {
            instance.color = [1.0, 1.0, 1.0, instance.color[3]];
        } else if instance.material.color_source == SceneColorSource3d::SemanticNeutral {
            instance.color = [theme.panel.r, theme.panel.g, theme.panel.b, instance.color[3] * theme.panel.a];
        }
        instance.material.emissive_intensity = 0.0;
        return instance;
    }
    let paint = mesh_style_paint(theme, kind);
    instance.color = if preserve_vertex_color {
        [1.0, 1.0, 1.0, instance.color[3] * paint.opacity]
    } else {
        [paint.fill[0], paint.fill[1], paint.fill[2], instance.color[3] * paint.opacity]
    };
    instance.material.emissive_intensity = paint.emissive_intensity;
    instance
}

/// 🧷️ LAW: a world draw is published only when THIS frame's uploads carry the exact mesh identity it
/// names — the source half of the prepared-frame residency law.
///
/// 🩸️ `mesh_store.get_versioned` answers a prepared world draw at SUBMIT, hundreds of host steps
/// after the build, and a miss there used to be a FATAL `present_step` fault that quarantined the
/// whole surface. Five sites published a `SceneDraw3d` whose `ensure_mesh` was conditional on a lease
/// that is admitted ASYNCHRONOUSLY — every ghost/stand-in path (`brush_preview`,
/// `catalogue_drop_preview`, the tool-run trace, the vortex arrows, the vertex markers) calls
/// `begin_world_placeholder_mesh` and then drew the placeholder on the very frame it was still being
/// built, with no upload behind it. The puzzle3d journey died on the engagement chip with
/// `prepared frame submit step: prepared world mesh was missing` (ticket
/// 26/09/17/WGPU-RENDERER-REACT-PARITY, `📓️w11a-prepared-world-mesh-missing.md`). React has no such
/// window: `BrushPreviewGhost` mounts `<boxGeometry>` with the mesh already in hand.
///
/// Each of those sites now binds its lease before it pushes; this pass is the STRUCTURAL guarantee
/// that a sixth one cannot reintroduce the fault — an unbacked draw is dropped for the one frame its
/// mesh is still landing, exactly as React renders nothing until its loader resolves.
pub(super) fn retain_ensured_world_draws(gpu: &World3dBuildContext, draws: &mut Vec<SceneDraw3d>) {
    draws.retain(|draw| gpu.has_mesh_request(&draw.mesh_key, draw.mesh_version));
}

pub(super) fn retain_ensured_world_material_draws(gpu: &World3dBuildContext, draws: &mut Vec<SceneMaterialDraw3d>) {
    draws.retain(|draw| gpu.has_mesh_request(&draw.mesh_key, draw.mesh_version));
}

#[cfg(not(all(target_arch = "wasm32", target_env = "p2")))]
struct WorldTranslucentMaterialItem {
    pub(super) source: usize,
    pub(super) depth: f32,
    pub(super) ordinal: usize,
    pub(super) instance: Instance3d,
}

#[cfg(not(all(target_arch = "wasm32", target_env = "p2")))]
fn world3d_translucent_projected_depth(mesh: Mesh3dLease, instance: &Instance3d, view_proj: Mat4) -> Option<f32> {
    let (minimum, maximum) = mesh.aabb().ok()?;
    let center = Vec3 { x: (minimum[0] + maximum[0]) * 0.5, y: (minimum[1] + maximum[1]) * 0.5, z: (minimum[2] + maximum[2]) * 0.5 };
    let depth = view_proj.transform_point_m(instance.model.transform_point_m(center)).z;
    depth.is_finite().then_some(depth)
}

/// 🫧 Orders transparent material objects by Three's projected transformed bounding-sphere centre,
/// preserving producer insertion order for exact depth ties without allocating inside the sort.
#[cfg(not(all(target_arch = "wasm32", target_env = "p2")))]
pub(super) fn sort_world3d_translucent_material_draws(
    meshes: &WorldMeshRegistry,
    view_proj: Mat4,
    draws: &mut Vec<SceneMaterialDraw3d>,
) {
    let mut retained = Vec::with_capacity(draws.len());
    let mut sources = Vec::new();
    let mut items = Vec::new();
    let mut ordinal = 0usize;
    for draw in std::mem::take(draws) {
        if !draw.translucent {
            retained.push(draw);
            continue;
        }
        let SceneMaterialDraw3d { mesh_key, mesh_version, first_index, index_count, instances, material, translucent } = draw;
        let mesh = meshes.get(&mesh_key).copied();
        let source = sources.len();
        sources.push(SceneMaterialDraw3d { mesh_key, mesh_version, first_index, index_count, instances: Vec::new(), material, translucent });
        for instance in instances {
            let depth = mesh.and_then(|mesh| world3d_translucent_projected_depth(mesh, &instance, view_proj)).unwrap_or(f32::NEG_INFINITY);
            items.push(WorldTranslucentMaterialItem { source, depth, ordinal, instance });
            ordinal += 1;
        }
    }
    items.sort_unstable_by(|a, b| b.depth.total_cmp(&a.depth).then_with(|| a.ordinal.cmp(&b.ordinal)));
    let mut active_source = None;
    for item in items {
        if active_source != Some(item.source) {
            let source = &sources[item.source];
            retained.push(SceneMaterialDraw3d {
                mesh_key: source.mesh_key.clone(),
                mesh_version: source.mesh_version,
                first_index: source.first_index,
                index_count: source.index_count,
                instances: Vec::new(),
                material: source.material.clone(),
                translucent: true,
            });
            active_source = Some(item.source);
        }
        retained.last_mut().expect("a sorted material run owns a draw").instances.push(item.instance);
    }
    *draws = retained;
}

/// 🥽️ `pending_glb_urls` is keyed by URL, `mesh_source_urls` by mesh key — evicting the
/// pool entry has to drop the url the ledger bound to it, not the key itself, or the
/// lane refuses to re-fetch a mesh the scene names again.
pub(super) fn sync_mesh_pool(state: &mut World3dState, needed_mesh_keys: &HashSet<String>, gpu: &mut World3dBuildContext) {
    const PINNED: &[&str] = &["vortex-marker", "cylinder", "cone", "reference-plane", "vertex-marker", GUMBALL_PLANE_MESH];
    for key in needed_mesh_keys {
        if !state.mesh_pool.contains(key) {
            state.mesh_pool.acquire(key.clone());
        }
    }
    let stale: Vec<String> = state.mesh_pool.keys().filter(|key| !needed_mesh_keys.contains(*key) && !PINNED.contains(&key.as_str())).cloned().collect();
    for key in stale {
        if state.mesh_pool.release(&key) {
            if !retire_world_mesh(state, &key) || !retire_world_pixels(state, &key, true) {
                return;
            }
            if let Some(url) = state.mesh_source_urls.remove(&key) {
                state.pending_glb_urls.remove(&url);
            }
            gpu.evict_mesh(&key);
        }
    }
}

/// 🥽️📡️ Admits one url-declared mesh into the bounded world-asset pipeline
/// (`WorldAssetRequestKind::Glb`), the same lane DEM tiles and reference images travel: the renderer
/// host drains it (`take_next_world3d_asset`), streams the bytes through the GLB structure/schema
/// decoders and hands the sealed `Mesh3dLease` back through [`publish_world3d_asset_mesh_lease`],
/// which stores it under `mesh_id_from_url` — the very id the wire names the mesh by. The wgpu twin
/// of React's `GlbInstanceMesh`'s `useLoader(GLTFLoader, meshAssetTransportUrl(url))`; the
/// `/mesh/…` → catalog-path rewrite happens at the fetch boundary on both lanes (the frame Worker's
/// `meshAssetTransportUrl`, the native `native_renderer_asset_path`), exactly as it does in React.
///
/// `pending_glb_urls` is this lane's live predicate: the missing-mesh loop re-derives its urls from
/// the scene EVERY frame, so without it every frame walks the request table again. Back-pressure is
/// not a fault — an unpended url is re-offered next frame — but a structural refusal is a miss, the
/// same split [`reserve_terrain_tile_fetch`] makes.
fn reserve_world3d_mesh_fetch(state: &mut World3dState, url: &str) {
    match reserve_world3d_asset_request(state, WorldAssetRequestKind::Glb, url) {
        Ok(_) => {
            state.pending_glb_urls.insert(url.to_owned());
        }
        Err(WorldAssetFault::ItemCapacity | WorldAssetFault::ByteCapacity | WorldAssetFault::Closing | WorldAssetFault::Stale) => {}
        Err(_) => mark_world3d_asset_miss(state, url),
    }
}

/// 🥽️📡️ The per-frame offer: every draw the scene published whose mesh the wire named by url and
/// that has not landed yet. Re-derived from the live draw list every frame — a scene that stops
/// naming a mesh stops asking for it — with [`World3dState::pending_glb_urls`] and
/// [`World3dState::asset_url_misses`] as the two ledgers that keep the offer idempotent.
pub(super) fn offer_missing_mesh_fetches(state: &mut World3dState) {
    let missing_mesh_urls: HashSet<String> = state
        .draws
        .iter()
        .filter(|draw| !state.meshes.contains_key(&draw.mesh_key))
        .filter_map(|draw| state.mesh_source_urls.get(&draw.mesh_key).cloned())
        .filter(|url| !state.asset_url_misses.contains(url) && !state.pending_glb_urls.contains(url))
        .collect();
    for url in missing_mesh_urls {
        reserve_world3d_mesh_fetch(state, &url);
    }
}

#[cfg(test)]
fn queue_lod_mesh_fetch(state: &mut World3dState, logical_id: &str, scene_lod: f64) {
    let url = {
        let entries: Vec<(f64, &str)> = state.mesh_lod_catalog.get(logical_id).map(|lods| lods.iter().map(|entry| (entry.lod, entry.url.as_str())).collect()).unwrap_or_default();
        let fallback = state.mesh_url_fallback.get(logical_id).map(String::as_str);
        pick_closest_mesh_url(&entries, scene_lod, fallback).or(fallback).map(str::to_owned)
    };
    if let Some(url) = url {
        reserve_world3d_mesh_fetch(state, &url);
    }
}

//#endregion LodGrid

//#region Environment
/// ☀️🎨️ Resolves the renderer's scene-pass light direction from `environment.sun` when the sun is
/// explicitly enabled (horizontal coordinate system, see https://en.wikipedia.org/wiki/Horizontal_coordinate_system,
/// matching `sunPositionFromAzimuthElevation` in `ui/js/react/index.tsx`), else keeps the default.
pub(super) fn environment_light_dir(environment: &WorldEnvironmentRecord) -> [f32; 3] {
    const DEFAULT_LIGHT_DIR: [f32; 3] = [0.4, 0.6, 0.8];
    let Some(sun) = environment.sun.as_ref() else {
        return DEFAULT_LIGHT_DIR;
    };
    if sun.enabled != Some(true) {
        return DEFAULT_LIGHT_DIR;
    }
    let azimuth = sun.azimuth.unwrap_or(45.0).to_radians();
    let elevation = sun.elevation.unwrap_or(35.0).to_radians();
    let direction = Vec3::new((elevation.cos() * azimuth.cos()) as f32, (elevation.cos() * azimuth.sin()) as f32, elevation.sin() as f32);
    if direction.length() < 1e-6 {
        DEFAULT_LIGHT_DIR
    } else {
        direction.normalize().to_array()
    }
}

/// 🌞️ Resolves React's ambient plus configured-sun branch into one backend-neutral pass record.
pub(super) fn environment_scene_lighting(environment: &WorldEnvironmentRecord) -> SceneLighting3d {
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

/// 🎨️ Resolves the neutral `MeshStandardMaterial` parameters. Environment emissive is a neutral
/// color override in World3d, so it is active only beside `material.color`; IconRender supplies its
/// own neutral default color whenever a material record exists.
pub(super) fn environment_scene_material(environment: &WorldEnvironmentRecord) -> SceneMaterial3d {
    let Some(material) = environment.material.as_ref() else {
        return SceneMaterial3d::default();
    };
    let emissive = material.color.as_ref().and(material.emissive.as_deref()).map(parse_color).map(|rgba| rgba[..3].try_into().expect("fixed rgba prefix")).unwrap_or([0.0; 3]);
    SceneMaterial3d { metalness: material.metalness.unwrap_or(0.0) as f32, roughness: material.roughness.unwrap_or(1.0) as f32, emissive, emissive_intensity: if emissive == [0.0; 3] { 0.0 } else { material.emissive_intensity.unwrap_or(1.0) as f32 } }
}

pub(super) fn environment_authored_material(environment: &WorldEnvironmentRecord) -> Option<SceneAuthoredMaterial3d> {
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

pub(super) fn environment_outline_color(environment: &WorldEnvironmentRecord, theme: &ui_wgpu::wgpu::Theme) -> Option<[f32; 4]> {
    match environment.material.as_ref().and_then(|material| material.stroke.as_deref()).map(str::trim) {
        Some(stroke) if stroke.eq_ignore_ascii_case("none") || stroke.eq_ignore_ascii_case("transparent") => None,
        Some(stroke) if !stroke.is_empty() => Some(parse_color(stroke)),
        _ => Some([theme.border_normal.r, theme.border_normal.g, theme.border_normal.b, theme.border_normal.a]),
    }
}

/// 🌑️ The two React shadow-map consumers and the SVG/material-absent exclusion path.
#[cfg(not(all(target_arch = "wasm32", target_env = "p2")))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum World3dShadowProfile {
    World,
    IconPng,
    IconSvg,
    Unshadowed,
}

#[cfg(not(all(target_arch = "wasm32", target_env = "p2")))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) enum World3dShadowGeometry {
    Glb,
    Terrain,
    Paint,
}

#[cfg(not(all(target_arch = "wasm32", target_env = "p2")))]
pub(super) fn world3d_shadow_role(geometry: World3dShadowGeometry, enabled: bool) -> SceneShadowRole3d {
    if !enabled {
        return SceneShadowRole3d::default();
    }
    match geometry {
        World3dShadowGeometry::Glb => SceneShadowRole3d { casts: true, receives: true },
        World3dShadowGeometry::Terrain => SceneShadowRole3d { casts: false, receives: true },
        World3dShadowGeometry::Paint => SceneShadowRole3d::default(),
    }
}

#[cfg(not(all(target_arch = "wasm32", target_env = "p2")))]
impl World3dShadowProfile {
    pub(super) fn map_size(self) -> u32 {
        match self {
            Self::World | Self::IconSvg | Self::Unshadowed => WORLD_SHADOW_MAP_SIZE,
            Self::IconPng => ICON_SHADOW_MAP_SIZE,
        }
    }

    pub(super) fn render_profile(self) -> ui_wgpu::wgpu::SceneRenderProfile3d {
        match self {
            Self::IconSvg => ui_wgpu::wgpu::SceneRenderProfile3d::SvgFlatLit,
            Self::World | Self::IconPng | Self::Unshadowed => ui_wgpu::wgpu::SceneRenderProfile3d::RasterPbr,
        }
    }
}

/// 🌑️ Resolves only the controls React consumes: enabled plus the consumer's map profile.
#[cfg(not(all(target_arch = "wasm32", target_env = "p2")))]
pub(super) fn environment_scene_shadow(environment: &WorldEnvironmentRecord, light_dir: [f32; 3], profile: World3dShadowProfile) -> SceneShadow3d {
    let record = environment.shadow.as_ref();
    let enabled = matches!(profile, World3dShadowProfile::World | World3dShadowProfile::IconPng)
        && record.and_then(|shadow| shadow.enabled) == Some(true)
        && environment.sun.as_ref().and_then(|sun| sun.enabled) == Some(true);
    SceneShadow3d { enabled, map_size: profile.map_size(), view_proj: directional_shadow_view_projection(light_dir) }
}

/// 🖼️ Resolves the canvas clear color from `environment.background`, falling back to the ambient
/// theme clear color when absent or `"transparent"` (mirrors `isTransparentWorldBackground`).
pub(super) fn environment_clear_color(environment: &WorldEnvironmentRecord, theme_clear: Rgba) -> Rgba {
    environment_explicit_clear_color(environment).unwrap_or(theme_clear)
}

pub(super) fn environment_explicit_clear_color(environment: &WorldEnvironmentRecord) -> Option<Rgba> {
    let background = environment.background.as_deref()?;
    if background.eq_ignore_ascii_case("transparent") {
        return None;
    }
    let [r, g, b, a] = parse_color(background);
    Some(Rgba::new(r, g, b, a))
}
//#endregion Environment

//#region Terrain
#[derive(Deserialize, semio_framework_value_derive::ToValue, semio_framework_value_derive::FromValue)]
struct TerrainVisibleTileRow {
    pub(super) z: u32,
    pub(super) x: u32,
    pub(super) y: u32,
}

#[derive(Deserialize, semio_framework_value_derive::ToValue, semio_framework_value_derive::FromValue)]
#[serde(rename_all = "camelCase")]
#[value(rename_all = "camelCase")]
pub(super) struct TerrainTileMeshPayload {
    pub(super) positions: Vec<f32>,
    pub(super) normals: Vec<f32>,
    pub(super) indices: Vec<u32>,
    pub(super) uvs: Vec<f32>,
}

pub(super) fn terrain_tile_url(template: &str, z: u32, x: u32, y: u32) -> String {
    template.replace("{z}", &z.to_string()).replace("{x}", &x.to_string()).replace("{y}", &y.to_string())
}

pub(super) fn terrain_tile_mesh_key(surface_id: &str, z: u32, x: u32, y: u32) -> String {
    format!("terrain:{surface_id}:{z}:{x}:{y}")
}

/// 🏔️📡️ Admits one visible-but-unloaded DEM tile into the bounded world-asset pipeline
/// (`WorldAssetRequestKind::Terrain`), the same lane GLB meshes and reference images travel — the
/// renderer host drains it (`take_next_world3d_asset`) and hands the bytes back through
/// [`apply_world3d_terrain_tile_bytes`]. Mirrors `TerrainTileRenderer.uploadOne` in
/// `🗺️WorldTerrainLayer/🟦️.tsx`, including its `tileMiss` set: a tile whose bytes the terrain
/// session refuses is never re-requested for this style.
///
/// 📡️ A full request table is back-pressure, not a fault: the next frame re-offers the tile.
fn reserve_terrain_tile_fetch(state: &mut World3dState, tile_url_template: &str, (z, x, y): (u32, u32, u32)) {
    if state.terrain_tile_misses.contains(&(z, x, y)) {
        return;
    }
    let url = terrain_tile_url(tile_url_template, z, x, y);
    if state.pending_terrain_tile_urls.contains_key(&url) {
        return;
    }
    match reserve_world3d_asset_request(state, WorldAssetRequestKind::Terrain { z, x, y }, &url) {
        Ok(_) => {
            state.pending_terrain_tile_urls.insert(url, (z, x, y));
        }
        Err(WorldAssetFault::ItemCapacity | WorldAssetFault::ByteCapacity | WorldAssetFault::Closing | WorldAssetFault::Stale) => {}
        Err(_) => {
            state.terrain_tile_misses.insert((z, x, y));
        }
    }
}

/// 🏔️📥️ Decodes one fetched DEM tile into the terrain session — the wgpu twin of
/// `TerrainTileRenderer.uploadOne`'s `upload_elevation_tile` call. A refusal (not a terrarium PNG)
/// records a tile miss so the pipeline stops re-requesting it; the banded meshes are built by the
/// next `sync_terrain_state` pass, which now sees a non-`null` `terrain_tile_mesh_json`.
pub fn apply_world3d_terrain_tile_bytes(state: &mut World3dState, z: u32, x: u32, y: u32, bytes: &[u8]) -> bool {
    let uploaded = state.terrain_session.upload_elevation_tile(z, x, y, bytes);
    if let Some(style) = state.terrain_style.as_ref() {
        state.pending_terrain_tile_urls.remove(&terrain_tile_url(&style.tile_url_template, z, x, y));
    }
    if !uploaded {
        state.terrain_tile_misses.insert((z, x, y));
    }
    uploaded
}

/// 🎨️ Vertical hypsometric ramp — same stops as `getHypsometricTexture` in `world-terrain-layer.tsx`
/// (green low ground -> tan -> grey -> white peaks), sampled at a band's center elevation ratio.
pub(super) fn hypsometric_color(t: f32) -> [f32; 4] {
    let stops: [(f32, [f32; 3]); 4] =
        [(0.0, [0x4b as f32 / 255.0, 0x6b as f32 / 255.0, 0x3a as f32 / 255.0]), (0.5, [0xa6 as f32 / 255.0, 0x8a as f32 / 255.0, 0x5b as f32 / 255.0]), (0.85, [0x8f as f32 / 255.0, 0x8f as f32 / 255.0, 0x8f as f32 / 255.0]), (1.0, [1.0, 1.0, 1.0])];
    let t = t.clamp(0.0, 1.0);
    for window in stops.windows(2) {
        let (t0, c0) = window[0];
        let (t1, c1) = window[1];
        if t <= t1 {
            let f = if (t1 - t0).abs() < 1e-6 { 0.0 } else { (t - t0) / (t1 - t0) };
            return [c0[0] + (c1[0] - c0[0]) * f, c0[1] + (c1[1] - c0[1]) * f, c0[2] + (c1[2] - c0[2]) * f, 1.0];
        }
    }
    [1.0, 1.0, 1.0, 1.0]
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) enum WorldTerrainMeshPhase {
    Begin,
    Allocate,
    Positions,
    Normals,
    Colors,
    Indices,
    Seal,
    Publish,
    RetireSource,
}

/// 🏔️ The terrain tile source buffers a retiring mesh cursor releases through one controlled owner.
#[derive(semio_framework_value::RetireOwned)]
struct WorldTerrainSource {
    pub(super) id: String,
    pub(super) positions: Vec<f32>,
    pub(super) normals: Vec<f32>,
    pub(super) indices: Vec<u32>,
    pub(super) uvs: Vec<f32>,
}

pub(super) struct WorldTerrainMeshCursor {
    pub(super) surface_id: String,
    pub(super) z: u32,
    pub(super) x: u32,
    pub(super) y: u32,
    pub(super) payload: TerrainTileMeshPayload,
    pub(super) generation: u64,
    source_revision: u64,
    terrain_revision: u64,
    pub(super) phase: WorldTerrainMeshPhase,
    pub(super) triangle: u32,
    pub(super) vertex: u8,
    pub(super) item: u32,
    pub(super) owner: WorldPlaceholderOwner,
    pub(super) close_started: bool,
    pub(super) source_retirement: Option<semio_framework_value::retirement::controlled::ControlledRetirement<WorldTerrainSource>>,
    pub(super) faulted: Option<WorldDynamicFault>,
}

pub(super) enum WorldTerrainMeshStep {
    Pending,
    Ready(String, Mesh3dLease),
    Complete((u32, u32, u32)),
    Fault(WorldDynamicFault),
}

impl WorldTerrainMeshCursor {

    pub(super) fn new(surface_id: &str, (z,x,y):(u32,u32,u32), payload:TerrainTileMeshPayload, generation:u64, source_revision:u64, terrain_revision:u64, grant:semio_framework_value::retained_clone::RetainedCloneGrant)->Result<(Self,semio_framework_value::retained_clone::RetainedCloneProgress),(WorldDynamicFault,TerrainTileMeshPayload)> {
        if surface_id.len()>WORLD_DYNAMIC_ID_BYTE_CAPACITY || !payload.positions.len().is_multiple_of(3) || !payload.normals.len().is_multiple_of(3) || !payload.indices.len().is_multiple_of(3) || !payload.uvs.len().is_multiple_of(2) {
            return Err((if surface_id.len()>WORLD_DYNAMIC_ID_BYTE_CAPACITY {WorldDynamicFault::IdCapacity} else {WorldDynamicFault::ByteCapacity},payload));
        }
        if u32::try_from(payload.indices.len()/3).is_err() {return Err((WorldDynamicFault::InstanceCapacity,payload));}
        let bytes=surface_id.len();
        if grant.maximum_items==0 || grant.maximum_depth==0 || bytes>grant.maximum_copy_bytes || bytes>grant.maximum_capacity_bytes {return Err((WorldDynamicFault::Ownership(semio_framework_value::ValueRefusalKind::OwnershipLimit),payload));}
        let mut id=match terrain_string_capacity(bytes) {Ok(id)=>id,Err(fault)=>return Err((fault,payload))};
        id.push_str(surface_id);
        Ok((Self{surface_id:id,z,x,y,payload,generation,source_revision,terrain_revision,phase:WorldTerrainMeshPhase::Begin,triangle:0,vertex:0,item:0,owner:WorldPlaceholderOwner::Empty,close_started:false,source_retirement:None,faulted:None},semio_framework_value::retained_clone::RetainedCloneProgress{copied_items:1,copied_bytes:bytes,retained_capacity_bytes:bytes,released_bytes:0}))
    }

    fn triangle_count(&self) -> u32 {
        (self.payload.indices.len() / 3) as u32
    }

    fn triangle_indices(&self, triangle: u32) -> Result<[u32; 3], WorldDynamicFault> {
        let start = usize::try_from(triangle).ok().and_then(|triangle| triangle.checked_mul(3)).ok_or(WorldDynamicFault::InstanceCapacity)?;
        let values = self.payload.indices.get(start..start + 3).ok_or(WorldDynamicFault::StaleToken)?;
        let result = [values[0], values[1], values[2]];
        if result.iter().any(|index| usize::try_from(*index).ok().and_then(|index| index.checked_mul(3)).is_none_or(|offset| self.payload.positions.get(offset..offset + 3).is_none())) {
            return Err(WorldDynamicFault::StaleToken);
        }
        Ok(result)
    }

    /// 🎨️ The hypsometric colour of ONE vertex of a triangle, sampled from the tile mesh's own
    /// per-vertex elevation ratio (`uvs[i * 2 + 1]`, what `build_terrain_tile_mesh` writes there and
    /// what React's `getHypsometricTexture` samples with) — a continuous ramp, not a band bucket.
    fn vertex_color(&self, triangle: u32, vertex: u8) -> Result<[f32; 4], WorldDynamicFault> {
        let index = self.triangle_indices(triangle)?[usize::from(vertex)] as usize;
        let elevation = self.payload.uvs.get(index * 2 + 1).copied().unwrap_or(0.0);
        if !elevation.is_finite() {
            return Err(WorldDynamicFault::ByteCapacity);
        }
        Ok(hypsometric_color(elevation))
    }

    pub(super) fn value(&self, triangle: u32, vertex: u8, normal: bool) -> Result<[f32; 3], WorldDynamicFault> {
        let index = self.triangle_indices(triangle)?[usize::from(vertex)] as usize;
        let source = if normal { &self.payload.normals } else { &self.payload.positions };
        let offset = index.checked_mul(3).ok_or(WorldDynamicFault::ByteCapacity)?;
        let value = match source.get(offset..offset + 3) {
            Some(value) => [value[0], value[1], value[2]],
            None if normal => [0.0, 0.0, 1.0],
            None => return Err(WorldDynamicFault::StaleToken),
        };
        if value.iter().all(|value| value.is_finite()) {
            Ok(value)
        } else {
            Err(WorldDynamicFault::ByteCapacity)
        }
    }

    pub(super) fn token(&self) -> Result<Mesh3dWriteToken, WorldDynamicFault> {
        match self.owner {
            WorldPlaceholderOwner::Writing(token) => Ok(token),
            _ => Err(WorldDynamicFault::StaleToken),
        }
    }


    pub(super) fn mesh_fault(fault: ui_wgpu::wgpu::Mesh3dFault) -> WorldDynamicFault { WorldDynamicFault::Mesh(fault) }

    pub(super) fn step(&mut self, source_revision: u64, terrain_revision: u64, grant: semio_framework_value::retained_clone::RetainedCloneGrant, context: &mut semio_framework_job::StepContext<'_>) -> Result<(WorldTerrainMeshStep, semio_framework_value::retained_clone::RetainedCloneProgress), WorldDynamicFault> {
        if context.should_yield() || grant.maximum_items == 0 { return Ok((WorldTerrainMeshStep::Pending, Default::default())); }
        context.set_stage("terrainMesh");
        if context.is_cancelled() { self.faulted=Some(WorldDynamicFault::Canceled);context.consume_fuel(1);return Ok((WorldTerrainMeshStep::Pending,Default::default())); }
        if source_revision != self.source_revision || terrain_revision != self.terrain_revision { self.faulted = Some(WorldDynamicFault::StaleToken); }
        let closing=self.faulted.is_some();
        let result = if let Some(fault) = self.faulted {
            self.close_step(grant).map(|step| (if step.complete { WorldTerrainMeshStep::Fault(fault) } else { WorldTerrainMeshStep::Pending }, step.progress))
        } else { self.step_live(grant) };
        context.consume_fuel(1);
        match result {
            Err(WorldDynamicFault::Mesh(ui_wgpu::wgpu::Mesh3dFault::Busy)) => Ok((WorldTerrainMeshStep::Pending, Default::default())),
            Err(fault) if closing=>Err(fault),
            Err(fault) => { self.faulted = Some(fault); Ok((WorldTerrainMeshStep::Pending, Default::default())) }
            value => value,
        }
    }

    pub(super) fn step_live(&mut self, grant: semio_framework_value::retained_clone::RetainedCloneGrant) -> Result<(WorldTerrainMeshStep, semio_framework_value::retained_clone::RetainedCloneProgress), WorldDynamicFault> {
        let empty = semio_framework_value::retained_clone::RetainedCloneProgress::default();
        if grant.maximum_items == 0 { return Ok((WorldTerrainMeshStep::Pending, empty)); }
        if grant.maximum_depth == 0 { return Err(WorldDynamicFault::Ownership(semio_framework_value::ValueRefusalKind::DepthLimit)); }
        let mut progress = semio_framework_value::retained_clone::RetainedCloneProgress { copied_items: 1, ..empty };
        match self.phase {
            WorldTerrainMeshPhase::Begin => {
                let items = self.triangle_count().checked_mul(3).ok_or(WorldDynamicFault::InstanceCapacity)?;
                if items == 0 { self.phase = WorldTerrainMeshPhase::RetireSource; return Ok((WorldTerrainMeshStep::Pending, progress)); }
                let schema = Mesh3dSchema { vertices: items, indices: items, face_ids:0, vertex_ids:0, edges:0, edge_ids:0, uvs:0, colors:items, surface_uvs:[0;4], tangents:0 };
                let (token, receipt) = mesh3d_begin(self.generation, self.source_revision, schema, grant).map_err(Self::mesh_fault)?;
                self.owner = WorldPlaceholderOwner::Writing(token);
                progress = receipt;
                self.phase = WorldTerrainMeshPhase::Allocate;
            }
            WorldTerrainMeshPhase::Allocate => {
                let step = mesh3d_allocate_step(self.token()?, grant).map_err(Self::mesh_fault)?;
                progress = step.progress;
                if step.complete { self.phase = WorldTerrainMeshPhase::Positions; }
            }
            WorldTerrainMeshPhase::Positions | WorldTerrainMeshPhase::Normals | WorldTerrainMeshPhase::Colors => {
                if self.triangle == self.triangle_count() {
                    self.triangle=0; self.vertex=0;
                    self.phase=match self.phase { WorldTerrainMeshPhase::Positions=>WorldTerrainMeshPhase::Normals, WorldTerrainMeshPhase::Normals=>WorldTerrainMeshPhase::Colors, _=>WorldTerrainMeshPhase::Indices };
                    return Ok((WorldTerrainMeshStep::Pending,progress));
                }
                let bytes=if self.phase==WorldTerrainMeshPhase::Colors {16} else {12};
                if grant.maximum_copy_bytes<bytes { return Ok((WorldTerrainMeshStep::Pending,empty)); }
                match self.phase {
                    WorldTerrainMeshPhase::Colors=>mesh3d_write_vec4(self.token()?,Mesh3dField::Colors,self.vertex_color(self.triangle,self.vertex)?).map_err(Self::mesh_fault)?,
                    phase=>{ let normal=phase==WorldTerrainMeshPhase::Normals; mesh3d_write_vec3(self.token()?,if normal {Mesh3dField::Normals} else {Mesh3dField::Positions},self.value(self.triangle,self.vertex,normal)?).map_err(Self::mesh_fault)?; }
                }
                progress.copied_bytes=bytes;
                self.vertex+=1;
                if self.vertex==3 { self.vertex=0; self.triangle+=1; }
            }
            WorldTerrainMeshPhase::Indices=>{
                if grant.maximum_copy_bytes<4 { return Ok((WorldTerrainMeshStep::Pending,empty)); }
                mesh3d_write_u32(self.token()?,Mesh3dField::Indices,self.item).map_err(Self::mesh_fault)?;
                progress.copied_bytes=4; self.item+=1;
                if self.item==self.triangle_count()*3 { self.item=0; self.phase=WorldTerrainMeshPhase::Seal; }
            }
            WorldTerrainMeshPhase::Seal=>{
                let lease=mesh3d_seal(self.token()?).map_err(Self::mesh_fault)?;
                self.owner=WorldPlaceholderOwner::Ready(lease); self.phase=WorldTerrainMeshPhase::Publish;
            }
            WorldTerrainMeshPhase::Publish=>{
                let WorldPlaceholderOwner::Ready(lease)=self.owner else {return Err(WorldDynamicFault::StaleToken)};
                let bytes=8+self.surface_id.len()+3+terrain_number_bytes(self.z)+terrain_number_bytes(self.x)+terrain_number_bytes(self.y);
                if grant.maximum_copy_bytes<bytes || grant.maximum_capacity_bytes<bytes {return Ok((WorldTerrainMeshStep::Pending,empty));}
                let mut key=terrain_string_capacity(bytes)?;
                use std::fmt::Write;
                write!(&mut key,"terrain:{}:{}:{}:{}",self.surface_id,self.z,self.x,self.y).map_err(|_|WorldDynamicFault::Ownership(semio_framework_value::ValueRefusalKind::InvariantViolated))?;
                progress.copied_bytes=bytes; progress.retained_capacity_bytes=key.capacity();
                self.owner=WorldPlaceholderOwner::Empty; self.phase=WorldTerrainMeshPhase::RetireSource;
                return Ok((WorldTerrainMeshStep::Ready(key,lease),progress));
            }
            WorldTerrainMeshPhase::RetireSource=>{
                let step=self.close_step(grant)?;
                return Ok((if step.complete { WorldTerrainMeshStep::Complete((self.z,self.x,self.y)) } else {WorldTerrainMeshStep::Pending},step.progress));
            }
        }
        if !progress.fits(grant) {return Err(WorldDynamicFault::Ownership(semio_framework_value::ValueRefusalKind::InvariantViolated));}
        Ok((WorldTerrainMeshStep::Pending,progress))
    }

    pub(super) fn close_step(&mut self, grant: semio_framework_value::retained_clone::RetainedCloneGrant) -> Result<ui_wgpu::wgpu::Mesh3dOwnershipStep,WorldDynamicFault> {
        let empty=ui_wgpu::wgpu::Mesh3dOwnershipStep::default();
        if self.terminal_is_empty() {return Ok(ui_wgpu::wgpu::Mesh3dOwnershipStep {complete:true,..empty});}
        if grant.maximum_items==0 {return Ok(empty);}
        if grant.maximum_depth==0 {return Err(WorldDynamicFault::Ownership(semio_framework_value::ValueRefusalKind::DepthLimit));}
        match self.owner {
            WorldPlaceholderOwner::Writing(token)=>{
                if !self.close_started {
                    match mesh3d_abort(token) {Ok(())|Err(ui_wgpu::wgpu::Mesh3dFault::Closing)=>self.close_started=true,Err(fault)=>return Err(Self::mesh_fault(fault))}
                    return Ok(ui_wgpu::wgpu::Mesh3dOwnershipStep{progress:semio_framework_value::retained_clone::RetainedCloneProgress{copied_items:1,..Default::default()},..empty});
                }
                let step=mesh3d_abort_step(token,grant).map_err(Self::mesh_fault)?;
                if !step.progress.fits(grant) {return Err(WorldDynamicFault::Ownership(semio_framework_value::ValueRefusalKind::InvariantViolated));}
                if step.complete {self.owner=WorldPlaceholderOwner::Empty;self.close_started=false;}
                return Ok(ui_wgpu::wgpu::Mesh3dOwnershipStep{complete:false,..step});
            }
            WorldPlaceholderOwner::Ready(lease)=>{
                if !self.close_started {
                    match mesh3d_begin_close(lease) {Ok(())|Err(ui_wgpu::wgpu::Mesh3dFault::Closing)=>self.close_started=true,Err(fault)=>return Err(Self::mesh_fault(fault))}
                    return Ok(ui_wgpu::wgpu::Mesh3dOwnershipStep{progress:semio_framework_value::retained_clone::RetainedCloneProgress{copied_items:1,..Default::default()},..empty});
                }
                let step=mesh3d_close_step(lease,grant).map_err(Self::mesh_fault)?;
                if !step.progress.fits(grant) {return Err(WorldDynamicFault::Ownership(semio_framework_value::ValueRefusalKind::InvariantViolated));}
                if step.complete {self.owner=WorldPlaceholderOwner::Empty;self.close_started=false;}
                return Ok(ui_wgpu::wgpu::Mesh3dOwnershipStep{complete:false,..step});
            }
            WorldPlaceholderOwner::Empty=>{}
        }
        if self.source_retirement.is_none() {
            let value=WorldTerrainSource{id:std::mem::take(&mut self.surface_id),positions:std::mem::take(&mut self.payload.positions),normals:std::mem::take(&mut self.payload.normals),indices:std::mem::take(&mut self.payload.indices),uvs:std::mem::take(&mut self.payload.uvs)};
            match semio_framework_value::retirement::controlled::ControlledRetirement::new(value) {
                Ok(owner)=>self.source_retirement=Some(owner),
                Err((error,WorldTerrainSource{id,positions,normals,indices,uvs}))=>{
                    self.surface_id=id;self.payload.positions=positions;self.payload.normals=normals;self.payload.indices=indices;self.payload.uvs=uvs;
                    return Err(WorldDynamicFault::Ownership(error.kind));
                }
            }
        }
        let owner=self.source_retirement.as_mut().unwrap();
        let step=owner.step(grant).map_err(|error|WorldDynamicFault::Ownership(error.kind))?;
        let progress=match step {semio_framework_value::retained_clone::RetainedCloneStep::Progress(progress)|semio_framework_value::retained_clone::RetainedCloneStep::Complete(progress)=>progress};
        if !progress.fits(grant) {return Err(WorldDynamicFault::Ownership(semio_framework_value::ValueRefusalKind::InvariantViolated));}
        if owner.terminal_is_empty() {self.source_retirement=None;}
        Ok(ui_wgpu::wgpu::Mesh3dOwnershipStep{complete:self.terminal_is_empty(),progress})
    }

    pub(super) fn terminal_is_empty(&self)->bool {
        matches!(self.owner,WorldPlaceholderOwner::Empty) && self.source_retirement.is_none() && self.payload.positions.capacity()==0 && self.payload.normals.capacity()==0 && self.payload.indices.capacity()==0 && self.payload.uvs.capacity()==0 && self.surface_id.capacity()==0
    }

}

#[cfg(not(test))]
impl Drop for WorldTerrainMeshCursor {
    fn drop(&mut self) {
        assert!(self.terminal_is_empty() || std::thread::panicking(), "terrain mesh cursor reached Drop before its exact page/source witness");
    }
}


fn terrain_string_capacity(bytes:usize)->Result<String,WorldDynamicFault> {
    if bytes==0 {return Ok(String::new());}
    let layout=std::alloc::Layout::array::<u8>(bytes).map_err(|_|WorldDynamicFault::Ownership(semio_framework_value::ValueRefusalKind::OwnershipLimit))?;
    let pointer=std::ptr::NonNull::new(unsafe{std::alloc::alloc(layout)}).ok_or(WorldDynamicFault::Ownership(semio_framework_value::ValueRefusalKind::AllocationFailed))?;
    Ok(unsafe{String::from_raw_parts(pointer.as_ptr(),0,bytes)})
}
fn terrain_number_bytes(mut value:u32)->usize {let mut bytes=1;while value>=10{value/=10;bytes+=1;}bytes}

#[derive(Clone,Copy,Debug,PartialEq,Eq)]
pub enum WorldTerrainMeshPublicationStep {Idle,Pending,Complete,Fault(WorldDynamicFault)}
#[derive(Clone,Copy,Debug)]
pub struct WorldTerrainMeshProgress {pub step:WorldTerrainMeshPublicationStep,pub ownership:semio_framework_value::retained_clone::RetainedCloneProgress}

pub fn step_world3d_terrain(state:&mut World3dState,grant:semio_framework_value::retained_clone::RetainedCloneGrant,context:&mut semio_framework_job::StepContext<'_>)->WorldTerrainMeshProgress {
    if context.is_cancelled() {if let Some(cursor)=state.terrain_build.as_mut(){cursor.faulted=Some(WorldDynamicFault::Canceled);}state.dynamic_fault=Some(WorldDynamicFault::Canceled);return WorldTerrainMeshProgress{step:WorldTerrainMeshPublicationStep::Pending,ownership:Default::default()};}
    if state.dynamic_fault==Some(WorldDynamicFault::Canceled) {return step_world3d_terrain_retirement(state,grant,context);}
    let mut result=WorldTerrainMeshProgress{step:WorldTerrainMeshPublicationStep::Idle,ownership:Default::default()};
    if context.should_yield() || grant.maximum_items==0 {result.step=WorldTerrainMeshPublicationStep::Pending;return result;}
    if state.terrain_build.is_none() && state.terrain_pending.is_some() {
        match begin_world_terrain_pending(state,grant,context.is_cancelled()) {
            Ok(progress)=>{context.set_stage("terrainMeshAdmission");context.consume_fuel(1);result.step=WorldTerrainMeshPublicationStep::Pending;result.ownership=progress;return result;}
            Err(fault)=>{result.step=WorldTerrainMeshPublicationStep::Fault(fault);return result;}
        }
    }
    let Some(mut cursor)=state.terrain_build.take() else{return result};
    if !state.terrain_visible_tiles.contains(&(cursor.z,cursor.x,cursor.y)) || state.dynamic_blocked_mesh.is_some() || state.dynamic_mesh_close.is_some() {cursor.faulted=Some(WorldDynamicFault::StaleToken);}
    match cursor.step(state.terrain_revision,state.terrain_revision,grant,context) {
        Ok((step,progress))=>{
            result.ownership=progress;
            match step {
                WorldTerrainMeshStep::Pending=>{state.terrain_build=Some(cursor);result.step=WorldTerrainMeshPublicationStep::Pending;}
                WorldTerrainMeshStep::Ready(key,lease)=>{
                    if let Err(rejected)=publish_world3d_mesh_lease(state,key,lease) {
                        cursor.faulted=Some(rejected.fault);retain_rejected_world_mesh(state,rejected);
                    }
                    state.terrain_build=Some(cursor);result.step=WorldTerrainMeshPublicationStep::Pending;
                }
                WorldTerrainMeshStep::Complete(tile)=>{state.terrain_built_tiles.insert(tile);result.step=WorldTerrainMeshPublicationStep::Complete;}
                WorldTerrainMeshStep::Fault(fault)=>{mark_world_dynamic_fault(state,fault);result.step=WorldTerrainMeshPublicationStep::Fault(fault);}
            }
        }
        Err(fault)=>{state.terrain_build=Some(cursor);result.step=WorldTerrainMeshPublicationStep::Fault(fault);}
    }
    result
}


pub fn step_world3d_terrain_retirement(state:&mut World3dState,grant:semio_framework_value::retained_clone::RetainedCloneGrant,context:&mut semio_framework_job::StepContext<'_>)->WorldTerrainMeshProgress {
    let mut result=WorldTerrainMeshProgress{step:WorldTerrainMeshPublicationStep::Idle,ownership:Default::default()};
    if context.should_yield() || grant.maximum_items==0 {result.step=WorldTerrainMeshPublicationStep::Pending;return result;}
    context.set_stage("terrainMeshRetirement");
    if state.terrain_build.is_none() && state.terrain_pending.is_some() {
        match begin_world_terrain_pending(state,grant,true) {
            Ok(progress)=>{result.ownership=progress;result.step=WorldTerrainMeshPublicationStep::Pending;}
            Err(fault)=>result.step=WorldTerrainMeshPublicationStep::Fault(fault),
        }
        context.consume_fuel(1);return result;
    }
    let Some(cursor)=state.terrain_build.as_mut() else{return result};
    let step=cursor.close_step(grant);
    context.consume_fuel(1);
    match step {
        Ok(step)=>{
            result.ownership=step.progress;
            if !step.progress.fits(grant) {result.step=WorldTerrainMeshPublicationStep::Fault(WorldDynamicFault::Ownership(semio_framework_value::ValueRefusalKind::InvariantViolated));}
            else if step.complete {state.terrain_build=None;result.step=WorldTerrainMeshPublicationStep::Complete;}
            else {result.step=WorldTerrainMeshPublicationStep::Pending;}
        }
        Err(WorldDynamicFault::Mesh(ui_wgpu::wgpu::Mesh3dFault::Busy))=>result.step=WorldTerrainMeshPublicationStep::Pending,
        Err(fault)=>result.step=WorldTerrainMeshPublicationStep::Fault(fault),
    }
    result
}

fn begin_world_terrain_pending(state:&mut World3dState,grant:semio_framework_value::retained_clone::RetainedCloneGrant,canceled:bool)->Result<semio_framework_value::retained_clone::RetainedCloneProgress,WorldDynamicFault> {
    let Some((tile,payload,generation,revision,terrain_revision))=state.terrain_pending.take() else{return Ok(Default::default())};
    match WorldTerrainMeshCursor::new(if canceled {""} else {&state.surface_id},tile,payload,generation,revision,terrain_revision,grant) {
        Ok((mut cursor,progress))=>{if canceled {cursor.faulted=Some(WorldDynamicFault::Canceled);}state.terrain_build=Some(cursor);Ok(progress)}
        Err((fault,payload))=>{state.terrain_pending=Some((tile,payload,generation,revision,terrain_revision));Err(fault)}
    }
}

#[cfg(test)]
pub(super) struct TerrainMeshOracleData {
    pub(super) positions: Vec<f32>,
    pub(super) normals: Vec<f32>,
    pub(super) indices: Vec<u32>,
    pub(super) face_ids: Vec<u32>,
    pub(super) vertex_ids: Vec<u32>,
    pub(super) edge_positions: Vec<f32>,
    pub(super) edge_ids: Vec<u32>,
    pub(super) uvs: Vec<f32>,
    pub(super) colors: Vec<f32>,
}

/// 🏔️🎨️ Language-neutral oracle for `WorldTerrainMeshCursor`: the de-indexed, vertex-coloured tile
/// mesh the cursor must produce — one triangle soup with the CONTINUOUS hypsometric ramp sampled per
/// vertex from the tile's own elevation ratio, which is what React's textured `MeshStandardMaterial`
/// shows.
#[cfg(test)]
pub(super) fn build_terrain_tile_mesh_oracle(mesh: &TerrainTileMeshPayload) -> Option<TerrainMeshOracleData> {
    let mut positions = Vec::new();
    let mut normals = Vec::new();
    let mut indices = Vec::new();
    let mut colors = Vec::new();
    for triangle in mesh.indices.as_chunks::<3>().0 {
        let base = (positions.len() / 3) as u32;
        for index in triangle {
            let offset = *index as usize * 3;
            let position = mesh.positions.get(offset..offset + 3)?;
            let normal = mesh.normals.get(offset..offset + 3).unwrap_or(&[0.0, 0.0, 1.0]);
            positions.extend_from_slice(position);
            normals.extend_from_slice(normal);
            colors.extend_from_slice(&hypsometric_color(mesh.uvs.get(*index as usize * 2 + 1).copied().unwrap_or(0.0)));
        }
        indices.extend_from_slice(&[base, base + 1, base + 2]);
    }
    (!positions.is_empty()).then_some(TerrainMeshOracleData { positions, normals, indices, face_ids: Vec::new(), vertex_ids: Vec::new(), edge_positions: Vec::new(), edge_ids: Vec::new(), uvs: Vec::new(), colors })
}

/// 🔄️ GPU-free half of `apply_terrain_style_if_changed`: applies `state.terrain_style` to the tile
/// session and purges cached tile meshes from CPU-side maps whenever the tile source, project
/// origin, or exaggeration changes (their old positions/heights are baked into cached geometry and
/// would otherwise render stale). Returns the mesh keys the caller must also evict from the GPU.
pub(super) fn apply_terrain_style_if_changed_state(state: &mut World3dState) -> Vec<String> {
    let signature = state.terrain_style.as_ref().map(|style| (style.tile_url_template.clone(), style.project_origin_lon, style.project_origin_lat, style.exaggeration));
    if signature == state.terrain_applied_signature {
        return Vec::new();
    }
    state.terrain_revision = state.terrain_revision.wrapping_add(1).max(1);
    if let Some(cursor) = state.terrain_build.as_mut() {
        cursor.faulted = Some(WorldDynamicFault::StaleToken);
    }
    let prefix = format!("terrain:{}:", state.surface_id);
    let stale_keys: Vec<String> = state.meshes.keys().filter(|key| key.starts_with(&prefix)).cloned().collect();
    for key in &stale_keys {
        if !retire_world_mesh(state, key) {
            return Vec::new();
        }
    }
    let visible = std::mem::take(&mut state.terrain_visible_tiles);
    for (z, x, y) in visible {
        state.terrain_session.evict_terrain_tile(z, x, y);
    }
    state.terrain_built_tiles.clear();
    state.pending_terrain_tile_urls.clear();
    state.terrain_tile_misses.clear();
    if let Some(style) = &state.terrain_style {
        state.terrain_session.set_project_origin(style.project_origin_lon, style.project_origin_lat);
        state.terrain_session.set_exaggeration(style.exaggeration);
    }
    state.terrain_applied_signature = signature;
    stale_keys
}

pub(super) fn apply_terrain_style_if_changed(state: &mut World3dState, gpu: &mut World3dBuildContext) {
    for key in apply_terrain_style_if_changed_state(state) {
        gpu.evict_mesh(&key);
    }
}

/// 🏔️ One terrain band's resolved GPU draw inputs: mesh key/version (already present in
/// `state.meshes`/`state.mesh_versions`) plus the flat hypsometric color for that band.
/// 🏔️ One terrain tile's resolved GPU draw inputs: mesh key/version (already present in
/// `state.meshes`/`state.mesh_versions`). The hypsometric ramp rides the mesh's OWN per-vertex
/// colours, so the instance tint is neutral white — one draw per tile, never one per colour band.
struct TerrainTileDraw {
    pub(super) mesh_key: String,
    pub(super) mesh_version: u64,
}

pub(super) fn terrain_family_visible(state: &World3dState, tile: (u32, u32, u32)) -> bool {
    state.terrain_built_tiles.contains(&tile)
}

/// 🏔️ GPU-free half of `sync_terrain`: asks `TerrainSessionCore` which DEM tiles are visible for
/// the current camera, evicts (CPU-side) tiles that scrolled out of view, admits byte-fetches for
/// tiles not yet uploaded (see `reserve_terrain_tile_fetch`), and builds/caches one vertex-coloured
/// mesh per tile whose elevation data is already available. Returns the tiles to draw this frame
/// plus the mesh keys the caller must evict from the GPU.
pub(super) fn sync_terrain_state(state: &mut World3dState, camera: &Camera3d) -> (Vec<TerrainTileDraw>, Vec<String>) {
    let Some(style) = state.terrain_style.clone() else {
        return (Vec::new(), Vec::new());
    };
    let camera_json = json!({
        "position": [camera.position.x as f64, camera.position.y as f64, camera.position.z as f64],
        "target": [camera.target.x as f64, camera.target.y as f64, camera.target.z as f64],
    })
    .to_string();
    let visible_json = state.terrain_session.visible_terrain_tiles_json(&camera_json);
    let visible_rows: Vec<TerrainVisibleTileRow> = serde_json::from_str(&visible_json).unwrap_or_default();
    let visible_set: HashSet<(u32, u32, u32)> = visible_rows.iter().map(|row| (row.z, row.x, row.y)).collect();

    let stale: Vec<(u32, u32, u32)> = state.terrain_visible_tiles.iter().copied().filter(|key| !visible_set.contains(key)).collect();
    let mut evicted_mesh_keys = Vec::new();
    for (z, x, y) in stale {
        state.terrain_session.evict_terrain_tile(z, x, y);
        state.terrain_built_tiles.remove(&(z, x, y));
        let mesh_key = terrain_tile_mesh_key(&state.surface_id, z, x, y);
        if !retire_world_mesh(state, &mesh_key) {
            return (Vec::new(), evicted_mesh_keys);
        }
        evicted_mesh_keys.push(mesh_key);
    }
    state.pending_terrain_tile_urls.retain(|_, tile| visible_set.contains(tile));
    state.terrain_visible_tiles = visible_set.clone();

    let mut tile_draws = Vec::new();
    for (z, x, y) in visible_set {
        if !terrain_family_visible(state, (z, x, y)) {
            let mesh_json = state.terrain_session.terrain_tile_mesh_json(z, x, y);
            if mesh_json == "null" {
                reserve_terrain_tile_fetch(state, &style.tile_url_template, (z, x, y));
            } else if state.terrain_build.is_none() && state.terrain_pending.is_none() && state.dynamic_mesh_close.is_none() && state.dynamic_blocked_mesh.is_none() && state.snapshot_fault.is_none() {
                if let Ok(mesh_payload) = serde_json::from_str::<TerrainTileMeshPayload>(&mesh_json) {
                    let next = state.placeholder_generation.checked_add(1);
                    if let Some(next) = next {
                        state.placeholder_generation=next;
                        state.terrain_pending=Some(((z,x,y),mesh_payload,next,state.terrain_revision,state.terrain_revision));
                    } else {
                        mark_world_dynamic_fault(state, WorldDynamicFault::StaleToken);
                    }
                }
            }
            continue;
        }
        let mesh_key = terrain_tile_mesh_key(&state.surface_id, z, x, y);
        if !state.meshes.contains_key(&mesh_key) {
            continue;
        }
        let mesh_version = live_world3d_mesh_version(state, &mesh_key);
        tile_draws.push(TerrainTileDraw { mesh_key, mesh_version });
    }
    (tile_draws, evicted_mesh_keys)
}

/// 🏔️ Per-frame terrain sync entry point used by `render_world_3d` — see `sync_terrain_state` for
/// the (unit-testable) tile-visibility/meshing/fetch-admission logic this wraps with GPU upload and
/// eviction calls.
pub(super) fn sync_terrain(state: &mut World3dState, gpu: &mut World3dBuildContext, camera: &Camera3d) -> Vec<SceneDraw3d> {
    let (tile_draws, evicted_mesh_keys) = sync_terrain_state(state, camera);
    for key in evicted_mesh_keys {
        gpu.evict_mesh(&key);
    }
    tile_draws
        .into_iter()
        .filter_map(|tile| {
            let mesh = state.meshes.get(&tile.mesh_key)?;
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

//#region Gumball
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) enum GumballHandle {
    MoveX,
    MoveY,
    MoveZ,
    MoveXY,
    MoveYZ,
    MoveXZ,
    RotateX,
    RotateY,
    RotateZ,
    ScaleX,
    ScaleY,
    ScaleZ,
}

impl GumballHandle {
    pub(super) fn axis_dir(self) -> Option<Vec3> {
        match self {
            Self::MoveX | Self::RotateX | Self::ScaleX => Some(Vec3::new(1.0, 0.0, 0.0)),
            Self::MoveY | Self::RotateY | Self::ScaleY => Some(Vec3::new(0.0, 1.0, 0.0)),
            Self::MoveZ | Self::RotateZ | Self::ScaleZ => Some(Vec3::new(0.0, 0.0, 1.0)),
            _ => None,
        }
    }

    pub(super) fn plane_normal(self) -> Option<Vec3> {
        match self {
            Self::MoveXY => Some(Vec3::new(0.0, 0.0, 1.0)),
            Self::MoveYZ => Some(Vec3::new(1.0, 0.0, 0.0)),
            Self::MoveXZ => Some(Vec3::new(0.0, 1.0, 0.0)),
            Self::RotateX => Some(Vec3::new(1.0, 0.0, 0.0)),
            Self::RotateY => Some(Vec3::new(0.0, 1.0, 0.0)),
            Self::RotateZ => Some(Vec3::new(0.0, 0.0, 1.0)),
            _ => None,
        }
    }

    pub(super) fn is_translate(self) -> bool {
        matches!(self, Self::MoveX | Self::MoveY | Self::MoveZ | Self::MoveXY | Self::MoveYZ | Self::MoveXZ)
    }

    pub(super) fn is_rotate(self) -> bool {
        matches!(self, Self::RotateX | Self::RotateY | Self::RotateZ)
    }

    pub(super) fn is_scale(self) -> bool {
        matches!(self, Self::ScaleX | Self::ScaleY | Self::ScaleZ)
    }
}
//#endregion Gumball

//#region MeshHelpers
//#region PlaceholderMeshAuthority
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) enum WorldPlaceholderKind {
    Box,
    Plane,
    Cylinder,
    Cone,
    Icosphere,
}

impl WorldPlaceholderKind {
    pub(super) fn resolve(value: &str) -> Self {
        match value {
            "vortex-marker" => Self::Icosphere,
            "plane" => Self::Plane,
            "cylinder" => Self::Cylinder,
            "cone" => Self::Cone,
            _ => Self::Box,
        }
    }

    pub(super) fn triangles(self) -> u32 {
        match self {
            Self::Box => 12,
            Self::Plane => 2,
            Self::Cylinder => 64,
            Self::Cone => 32,
            Self::Icosphere => 80,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum WorldPlaceholderMeshPhase {
    Allocate,
    Positions,
    Normals,
    Tangents,
    Uvs,
    Colors,
    Indices,
    FaceIds,
    VertexIds,
    Edges,
    EdgeIds,
    Materials,
    Textures,
    Provenance,
    AdmitProvenance,
    Seal,
    Publish,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) enum WorldPlaceholderOwner {
    Writing(Mesh3dWriteToken),
    Ready(Mesh3dLease),
    Empty,
}

/// 🧊️ Where one in-flight mesh publication reads its vertices from: a generated marker primitive,
/// or an inline `World3dScene.meshes_json` buffer set bridged in by [`stage_world3d_scene_bridge`].
/// Both ride the identical fixed-credit `mesh3d_*` authority ladder below.
pub(super) enum WorldMeshInput {
    Placeholder(WorldPlaceholderKind),
    Inline(Box<WorldMeshBuffers>),
}

impl WorldMeshInput {
    pub(super) fn position(&self, item: u32) -> [f32; 3] {
        match self {
            Self::Placeholder(kind) => placeholder_triangle(*kind, item / 3)[(item % 3) as usize],
            Self::Inline(buffers) if buffers.positions.is_empty() => {
                let offset = item as usize * 3;
                [buffers.edge_positions[offset], buffers.edge_positions[offset + 1], buffers.edge_positions[offset + 2]]
            }
            Self::Inline(buffers) => buffers.position(item),
        }
    }

    pub(super) fn normal(&self, item: u32) -> [f32; 3] {
        match self {
            Self::Placeholder(kind) => placeholder_triangle_normal(placeholder_triangle(*kind, item / 3)),
            Self::Inline(buffers) => buffers.normal(item),
        }
    }

    pub(super) fn index(&self, item: u32) -> u32 {
        match self {
            Self::Placeholder(_) => item,
            Self::Inline(buffers) => buffers.indices.get(item as usize).copied().unwrap_or(0),
        }
    }
}

pub(super) struct WorldPlaceholderMeshCursor {
    pub(super) key: String,
    pub(super) source: WorldMeshInput,
    vertex_items: u32,
    index_items: u32,
    pub(super) phase: WorldPlaceholderMeshPhase,
    pub(super) item: u32,
    pub(super) owner: WorldPlaceholderOwner,
    pub(super) close_started: bool,
    pub(super) faulted: bool,
    expanded: bool,
    source_tail_start: u32,
    pub(super) schema:Mesh3dSchema,
    uv_role:usize,
    texture_cursor:usize,
    texture_role:usize,
    texture_name:Option<String>,
    pub(super) texture_job:Option<semio_framework_pixels::png_decoding::PngDecodeJob>,
    pub(super) texture_retirement:Option<semio_framework_value::retirement::controlled::ControlledRetirement<semio_framework_pixels::png_decoding::PngDecodeJob>>,
    texture_image:Option<semio_framework_pixels::RasterImage>,
    texture_digest:[u64;2],
    texture_bytes:Vec<u8>,
    texture_byte:usize,
    texture_scan:usize,
    texture_required:bool,
    texture_hash:semio_framework_hash::Sha256,
    texture_submission:Option<semio_framework_async::Job>,
    texture_cancelled:std::sync::Arc<std::sync::atomic::AtomicBool>,
    texture_worker:Option<std::sync::Arc<Mutex<Option<Option<semio_framework_pixels::RasterImage>>>>>,
    pub(super) primitives:Vec<World3dPrimitiveMaterial>,
    pub(super) textures:Vec<(String,SceneRasterLease)>,
    pub(super) appearance:Option<World3dMeshAppearance>,
    pub(super) source_retirement:Option<semio_framework_value::retirement::controlled::ControlledRetirement<Box<WorldMeshBuffers>>>,
}

pub(super) enum WorldPlaceholderMeshStep {
    Pending,
    Ready(String, Mesh3dLease,Option<World3dMeshAppearance>),
    Fault,
}

impl WorldPlaceholderMeshCursor {
    pub(super) fn new(key: &str, kind: WorldPlaceholderKind, generation: u64, revision: u64) -> Result<Self, ui_wgpu::wgpu::Mesh3dFault> {
        let items = kind.triangles().checked_mul(3).ok_or(ui_wgpu::wgpu::Mesh3dFault::ItemCapacity)?;
        Self::begin(key, WorldMeshInput::Placeholder(kind), items, items, generation, revision)
    }

    /// 🌉️ The inline-wire twin of [`Self::new`]: an already-welded `meshes_json` buffer set, whose
    /// vertex and index counts are independent (the marker primitives above emit one vertex per
    /// face corner, a tessellated preview mesh does not).
    pub(super) fn inline(key: &str, buffers: WorldMeshBuffers, generation: u64, revision: u64) -> Result<Self, ui_wgpu::wgpu::Mesh3dFault> {
        let edge_only = buffers.positions.is_empty() && !buffers.edge_positions.is_empty();
        if !buffers.positions.len().is_multiple_of(3) || !buffers.edge_positions.len().is_multiple_of(6)
            || (edge_only && (!buffers.indices.is_empty() || !buffers.vertex_ids.is_empty())) {
            return Err(ui_wgpu::wgpu::Mesh3dFault::Schema);
        }
        let vertex_items = u32::try_from(if edge_only { buffers.edge_positions.len() / 3 } else { buffers.vertex_count() }).map_err(|_| ui_wgpu::wgpu::Mesh3dFault::ItemCapacity)?;
        let index_items = u32::try_from(buffers.indices.len()).map_err(|_| ui_wgpu::wgpu::Mesh3dFault::ItemCapacity)?;
        if vertex_items == 0 || !index_items.is_multiple_of(3) || buffers.indices.iter().any(|index| *index >= vertex_items) {
            return Err(ui_wgpu::wgpu::Mesh3dFault::Schema);
        }
        semio_framework::validate_mesh_surface_assets(&buffers.materials,&buffers.textures).map_err(|_|ui_wgpu::wgpu::Mesh3dFault::Schema)?;
        for(name,attribute)in &buffers.attributes {let uses=if attribute.domain==semio_framework::MeshAttributeDomain::Edge{buffers.edge_ids.len()}else{index_items as usize};semio_framework::validate_mesh_attribute(name,attribute,vertex_items as usize,index_items as usize/3,uses,&buffers.materials).map_err(|_|ui_wgpu::wgpu::Mesh3dFault::Schema)?;}
        let expanded=!buffers.indices.is_empty() && buffers.attributes.values().any(|attribute|attribute.domain==semio_framework::MeshAttributeDomain::Corner || attribute.domain==semio_framework::MeshAttributeDomain::Face);
        let source_tail_start = if expanded { buffers.vertex_ids.iter().position(|id| *id != u32::MAX).map_or(vertex_items, |item| item as u32) } else { vertex_items };
        let expanded_items = index_items.checked_add(vertex_items - source_tail_start).ok_or(ui_wgpu::wgpu::Mesh3dFault::ItemCapacity)?;
        let mut cursor=Self::begin(key,WorldMeshInput::Inline(Box::new(buffers)),if expanded {expanded_items}else{vertex_items},index_items,generation,revision)?;
        cursor.expanded=expanded;
        cursor.source_tail_start=source_tail_start;
        Ok(cursor)
    }

    pub(super) fn begin(key: &str, source: WorldMeshInput, vertex_items: u32, index_items: u32, generation: u64, revision: u64) -> Result<Self, ui_wgpu::wgpu::Mesh3dFault> {
        if key.len() > WORLD_DYNAMIC_ID_BYTE_CAPACITY {
            return Err(ui_wgpu::wgpu::Mesh3dFault::ByteCapacity);
        }
        let (uvs,colors)=match &source {WorldMeshInput::Inline(buffers)=>(!buffers.uvs.is_empty()||buffers.attributes.values().any(|attribute|attribute.semantic==semio_framework::MeshAttributeSemantic::Uv),!buffers.colors.is_empty()||buffers.attributes.values().any(|attribute|attribute.semantic==semio_framework::MeshAttributeSemantic::Color)),_ => (false,false)};
        let (face_ids,vertex_ids,edges,edge_ids)=match &source {WorldMeshInput::Inline(mesh)=>(mesh.face_ids.len() as u32,if mesh.vertex_ids.is_empty(){0}else{vertex_items},mesh.edge_positions.len() as u32/6,mesh.edge_ids.len() as u32),_ => (0,0,0,0)};
        let schema=Mesh3dSchema{face_ids,vertex_ids,edges,edge_ids,uvs:if uvs {vertex_items}else{0},colors:if colors {vertex_items}else{0},surface_uvs:if uvs {[vertex_items;4]}else{[0;4]},tangents:if matches!(&source,WorldMeshInput::Inline(mesh) if mesh.attributes.contains_key("tangent")){vertex_items}else{0},..Mesh3dSchema::triangle_mesh(vertex_items,index_items)};
        let grant = world_mesh_ownership_grant(0, ui_wgpu::wgpu::mesh3d_begin_capacity_byte_demand(), 0, 1);
        let (token, progress) = mesh3d_begin(generation, revision, schema, grant)?;
        let owner = WorldPlaceholderOwner::Writing(token);
        assert!(progress.fits(grant), "original mesh constructor exceeded its admitted ownership grant");
        Ok(Self { key: key.to_owned(), source, vertex_items, index_items, phase: WorldPlaceholderMeshPhase::Allocate, item: 0, owner, close_started: false, faulted: false,expanded:false,source_tail_start:0,schema,uv_role:0,texture_cursor:0,texture_role:0,texture_name:None,texture_job:None,texture_retirement:None,texture_image:None,texture_digest:[0;2],texture_bytes:Vec::new(),texture_byte:0,texture_scan:0,texture_required:false,texture_hash:Default::default(),texture_submission:None,texture_cancelled:std::sync::Arc::new(std::sync::atomic::AtomicBool::new(false)),texture_worker:None,primitives:Vec::new(),textures:Vec::new(),appearance:None,source_retirement:None })
    }

    fn source_vertex(&self,item:u32)->u32 {if self.expanded {if item < self.index_items {self.source.index(item)}else{self.source_tail_start + item - self.index_items}}else{item}}

    pub(super) fn sample<const N:usize>(&self,semantic:semio_framework::MeshAttributeSemantic,item:u32,fallback:[f32;N])->Result<[f32;N],ui_wgpu::wgpu::Mesh3dFault> {
        let WorldMeshInput::Inline(mesh)=&self.source else{return Ok(fallback)};
        let Some(attribute)=mesh.attributes.values().find(|attribute|attribute.semantic==semantic) else{return Ok(fallback)};
        if self.expanded && item >= self.index_items && attribute.domain != semio_framework::MeshAttributeDomain::Vertex { return Ok(fallback); }
        let domain=match attribute.domain {semio_framework::MeshAttributeDomain::Vertex=>self.source_vertex(item) as usize,semio_framework::MeshAttributeDomain::Corner=>item as usize,semio_framework::MeshAttributeDomain::Face=>item as usize/3,_=>return Err(ui_wgpu::wgpu::Mesh3dFault::Schema)};
        let values=attribute.value_at(domain).and_then(|value|value.as_array()).filter(|values|values.len()==N).ok_or(ui_wgpu::wgpu::Mesh3dFault::Schema)?;
        let mut result=[0.0;N];for axis in 0..N {result[axis]=values[axis].as_f64().filter(|value|value.is_finite()&&value.abs()<=f32::MAX as f64).ok_or(ui_wgpu::wgpu::Mesh3dFault::Schema)? as f32;}Ok(result)
    }

    fn uv_sample(&self,role:usize,item:u32)->Result<[f32;2],ui_wgpu::wgpu::Mesh3dFault> {
        let WorldMeshInput::Inline(mesh)=&self.source else{return Ok([0.0;2])};
        let fields=["baseColorTexture","metallicRoughnessTexture","normalTexture","occlusionTexture","emissiveTexture"];
        let name=mesh.attributes.values().find(|attribute|attribute.semantic==semio_framework::MeshAttributeSemantic::Material).and_then(|attribute|attribute.value_at(item as usize/3)).and_then(|value|value.as_str()).or_else(||mesh.materials.keys().next().map(String::as_str));
        let material=name.and_then(|name|mesh.materials.get(name)).and_then(|value|value.as_object());
        let coordinates=material.and_then(|fields|fields.iter().find_map(|(key,value)|(key=="textureCoordinates").then_some(value))).and_then(|value|value.as_object());
        let set=coordinates.and_then(|values|values.iter().find_map(|(key,value)|(key==fields[role]).then_some(value))).and_then(|value|value.as_u64()).unwrap_or(0) as usize;
        let declared=mesh.attributes.get(if set==0{"uv"}else{""}).or_else(||mesh.attributes.get(&format!("uv{set}")));
        let attribute=declared.or_else(||(set==0).then(||mesh.attributes.values().find(|attribute|attribute.semantic==semio_framework::MeshAttributeSemantic::Uv)).flatten());
        let Some(attribute)=attribute else {if set!=0{return Err(ui_wgpu::wgpu::Mesh3dFault::Schema)}let at=self.source_vertex(item) as usize*2;return Ok([mesh.uvs.get(at).copied().unwrap_or(0.0),mesh.uvs.get(at+1).copied().unwrap_or(0.0)])};
        if self.expanded && item >= self.index_items && attribute.domain != semio_framework::MeshAttributeDomain::Vertex { let at=self.source_vertex(item) as usize*2;return Ok([mesh.uvs.get(at).copied().unwrap_or(0.0),mesh.uvs.get(at+1).copied().unwrap_or(0.0)]); }
        let domain=match attribute.domain{semio_framework::MeshAttributeDomain::Vertex=>self.source_vertex(item) as usize,semio_framework::MeshAttributeDomain::Corner=>item as usize,semio_framework::MeshAttributeDomain::Face=>item as usize/3,_=>return Err(ui_wgpu::wgpu::Mesh3dFault::Schema)};
        let values=attribute.value_at(domain).and_then(|value|value.as_array()).filter(|values|values.len()==2).ok_or(ui_wgpu::wgpu::Mesh3dFault::Schema)?;
        Ok([values[0].as_f64().ok_or(ui_wgpu::wgpu::Mesh3dFault::Schema)? as f32,values[1].as_f64().ok_or(ui_wgpu::wgpu::Mesh3dFault::Schema)? as f32])
    }

    fn material_step(&mut self)->Result<(),ui_wgpu::wgpu::Mesh3dFault> {
        let WorldMeshInput::Inline(mesh)=&self.source else{return Ok(())};
        if mesh.materials.is_empty(){return Ok(())}
        let attribute=mesh.attributes.values().find(|attribute|attribute.semantic==semio_framework::MeshAttributeSemantic::Material);
        let name=attribute.and_then(|attribute|attribute.value_at(self.item as usize/3)).and_then(|value|value.as_str()).or_else(||mesh.materials.keys().next().map(String::as_str)).ok_or(ui_wgpu::wgpu::Mesh3dFault::Schema)?;
        let source=mesh.materials.get(name).and_then(|value|value.as_object()).ok_or(ui_wgpu::wgpu::Mesh3dFault::Schema)?;
        let find=|name:&str|source.iter().find_map(|(key,value)|(key==name).then_some(value));
        let tuple=|name:&str,defaults:[f32;4],width:usize|->Result<[f32;4],ui_wgpu::wgpu::Mesh3dFault>{let Some(value)=find(name)else{return Ok(defaults)};let values=value.as_array().filter(|values|values.len()==width).ok_or(ui_wgpu::wgpu::Mesh3dFault::Schema)?;let mut result=defaults;for axis in 0..width{result[axis]=values[axis].as_f64().ok_or(ui_wgpu::wgpu::Mesh3dFault::Schema)? as f32;}Ok(result)};
        let sampler=|role:usize| {let fields=["baseColorTexture","metallicRoughnessTexture","normalTexture","occlusionTexture","emissiveTexture"];let owned=find("textureSamplers").and_then(|value|value.as_object()).and_then(|values|values.iter().find_map(|(key,value)|(key==fields[role]).then_some(value))).and_then(|value|value.as_object());let value=|name:&str,default:u64|owned.and_then(|values|values.iter().find_map(|(key,value)|(key==name).then_some(value))).and_then(|value|value.as_u64()).unwrap_or(default);let wrap=|value|match value{33071=>ui_wgpu::wgpu::SceneTextureWrap3d::ClampToEdge,33648=>ui_wgpu::wgpu::SceneTextureWrap3d::MirrorRepeat,_=>ui_wgpu::wgpu::SceneTextureWrap3d::Repeat};let filter=|value|match value{9728=>ui_wgpu::wgpu::SceneTextureFilter3d::Nearest,9984=>ui_wgpu::wgpu::SceneTextureFilter3d::NearestMipmapNearest,9985=>ui_wgpu::wgpu::SceneTextureFilter3d::LinearMipmapNearest,9986=>ui_wgpu::wgpu::SceneTextureFilter3d::NearestMipmapLinear,9987=>ui_wgpu::wgpu::SceneTextureFilter3d::LinearMipmapLinear,_=>ui_wgpu::wgpu::SceneTextureFilter3d::Linear};ui_wgpu::wgpu::SceneTextureSampler3d{wrap_u:wrap(value("wrapS",10497)),wrap_v:wrap(value("wrapT",10497)),mag_filter:filter(value("magFilter",9729)),min_filter:filter(value("minFilter",9729))}};
        let scalar=|name:&str,default:f32|find(name).and_then(|value|value.as_f64()).map_or(default,|value|value as f32);
        let alpha=match find("alphaMode").and_then(|value|value.as_str()).unwrap_or("OPAQUE"){"OPAQUE"=>SceneMaterialAlpha3d::Opaque,"MASK"=>SceneMaterialAlpha3d::Mask,"BLEND"=>SceneMaterialAlpha3d::Blend,_=>return Err(ui_wgpu::wgpu::Mesh3dFault::Schema)};
        let mut color=tuple("baseColor",[1.0;4],4)?;if alpha==SceneMaterialAlpha3d::Opaque{color[3]=1.0;}let emissive=tuple("emissive",[0.0;4],3)?;
        let reference=|field:&str,role:usize|find(field).and_then(|value|value.as_str()).map(|id|format!("inline:{}:{}:{}",self.key,id,if role==0||role==4{"srgb"}else{"linear"}));
        let material=SceneAuthoredMaterial3d{base_color:color,emissive:[emissive[0],emissive[1],emissive[2]],metalness:scalar("metallic",1.0),roughness:scalar("roughness",1.0),alpha,alpha_cutoff:scalar("alphaCutoff",0.5),double_sided:find("doubleSided").and_then(|value|value.as_bool()).unwrap_or(false),preserve_vertex_color:self.schema.colors>0,base_color_texture:reference("baseColorTexture",0),texture_sampler:sampler(0),metallic_roughness_texture:reference("metallicRoughnessTexture",1),normal_texture:reference("normalTexture",2),occlusion_texture:reference("occlusionTexture",3),emissive_texture:reference("emissiveTexture",4),additional_texture_samplers:[sampler(1),sampler(2),sampler(3),sampler(4)],normal_scale:find("normalScale").and_then(|value|value.as_array()).map(|values|[values[0].as_f64().unwrap_or(1.0) as f32,values[1].as_f64().unwrap_or(1.0) as f32]).unwrap_or([scalar("normalScale",1.0);2]),occlusion_strength:scalar("occlusionStrength",1.0),};
        if let Some(previous)=self.primitives.last_mut().filter(|previous|previous.material==material){previous.index_count+=3;}else{if self.primitives.len()==WORLD_DYNAMIC_MESH_CAPACITY{return Err(ui_wgpu::wgpu::Mesh3dFault::ItemCapacity)}self.primitives.push(World3dPrimitiveMaterial{first_index:self.item,index_count:3,material});}Ok(())
    }

    fn texture_step(&mut self)->Result<bool,ui_wgpu::wgpu::Mesh3dFault> {
        if self.texture_retirement.is_some(){self.retire_texture_job()?;return Ok(false)}
        let WorldMeshInput::Inline(mesh)=&mut self.source else{return Ok(true)};
        if self.texture_name.is_none(){let Some(id)=mesh.textures.keys().nth(self.texture_cursor)else{return Ok(true)};self.texture_name=Some(id.clone());self.texture_scan=0;self.texture_required=false;}
        let id=self.texture_name.as_ref().unwrap();let role=if self.texture_role==0 {"srgb"}else{"linear"};let key=format!("inline:{}:{id}:{role}",self.key);
        if key.len()>WORLD_DYNAMIC_ID_BYTE_CAPACITY{return Err(ui_wgpu::wgpu::Mesh3dFault::ByteCapacity)}
        if self.texture_scan<self.primitives.len(){let material=&self.primitives[self.texture_scan].material;let references=if self.texture_role==0 {[material.base_color_texture.as_ref(),material.emissive_texture.as_ref(),None]}else{[material.metallic_roughness_texture.as_ref(),material.normal_texture.as_ref(),material.occlusion_texture.as_ref()]};self.texture_required|=references.into_iter().flatten().any(|reference|reference==&key);self.texture_scan+=1;return Ok(false)}
        if !self.texture_required {self.next_texture_role();return Ok(false)}
        if self.texture_image.is_none() {
            if let Some(submission)=self.texture_submission.take(){
                let cores=if cfg!(target_arch="wasm32"){1}else{std::thread::available_parallelism().map_or(1,std::num::NonZeroUsize::get)};
                let pool=semio_framework_async::process_worker_pool(semio_framework_async::WorkerPoolConfig::new(semio_framework_async::ProcessKind::InteractiveNative,cores));
                if let Err(refusal)=pool.try_submit(semio_framework_async::Lane::Maintenance,submission){self.texture_submission=Some(refusal.into_job());}return Ok(false)
            }
            if let Some(worker)=self.texture_worker.as_ref(){let result=worker.lock().map_err(|_|ui_wgpu::wgpu::Mesh3dFault::Schema)?.take();let Some(image)=result else{return Ok(false)};self.texture_image=Some(image.ok_or(ui_wgpu::wgpu::Mesh3dFault::Schema)?);self.texture_worker=None;return Ok(false)}
            if self.texture_job.is_none() {
                let texture=mesh.textures.get(id).ok_or(ui_wgpu::wgpu::Mesh3dFault::Schema)?;
                if self.texture_byte<texture.bytes.len(){if self.texture_byte==0{self.texture_bytes.try_reserve_exact(texture.bytes.len()).map_err(|_|ui_wgpu::wgpu::Mesh3dFault::ByteCapacity)?;}let end=(self.texture_byte+4096).min(texture.bytes.len());let bytes=&texture.bytes[self.texture_byte..end];self.texture_bytes.extend_from_slice(bytes);self.texture_hash.update(bytes);self.texture_byte=end;return Ok(false)}
                let digest=std::mem::take(&mut self.texture_hash).finalize();self.texture_digest=[u64::from_be_bytes(digest[..8].try_into().unwrap()),u64::from_be_bytes(digest[8..16].try_into().unwrap())];
                if texture.mime=="image/png" {match semio_framework_pixels::png_decoding::PngDecodeJob::new(semio_framework_pixels::png_decoding::PngDecodeInput{data:std::mem::take(&mut self.texture_bytes),max_pixels:WORLD_REFERENCE_SOURCE_PIXELS as usize,max_bytes:16_000_000,max_chunks:65536}){Ok(job)=>self.texture_job=Some(job),Err((_,input))=>{self.texture_bytes=input.data;return Err(ui_wgpu::wgpu::Mesh3dFault::Schema)}}}
                else if texture.mime=="image/jpeg" {
                    let worker=std::sync::Arc::new(Mutex::new(None));let output=worker.clone();let canceled=self.texture_cancelled.clone();let bytes=std::mem::take(&mut self.texture_bytes);
                    self.texture_submission=Some(Box::new(move ||{let image=if canceled.load(std::sync::atomic::Ordering::Acquire){None}else{decode_mesh_surface_image_bytes(&bytes)};*output.lock().unwrap_or_else(|poisoned|poisoned.into_inner())=Some(image);}));self.texture_worker=Some(worker);return Ok(false)
                }else{return Err(ui_wgpu::wgpu::Mesh3dFault::Schema)}
            }
            let job=self.texture_job.as_mut().unwrap();
            let grant=semio_framework_value::retained_clone::RetainedCloneGrant{maximum_items:4096,maximum_copy_bytes:job.next_copy_byte_demand().map_err(|_|ui_wgpu::wgpu::Mesh3dFault::Schema)?.max(32)*4096,maximum_capacity_bytes:job.next_capacity_byte_demand().map_err(|_|ui_wgpu::wgpu::Mesh3dFault::Schema)?,maximum_release_bytes:0,maximum_depth:1};
            let (progress,receipt)=job.advance(grant).map_err(|_|ui_wgpu::wgpu::Mesh3dFault::Schema)?;
            if !receipt.fits(grant){return Err(ui_wgpu::wgpu::Mesh3dFault::Schema)}
            if progress.done {
                let grant=world_mesh_ownership_grant(size_of::<semio_framework_pixels::RasterImage>(),0,0,1);
                let(image,receipt)=job.take_result(grant).map_err(|_|ui_wgpu::wgpu::Mesh3dFault::Schema)?.ok_or(ui_wgpu::wgpu::Mesh3dFault::Schema)?;
                if !receipt.fits(grant){return Err(ui_wgpu::wgpu::Mesh3dFault::Schema)}
                self.texture_image=Some(image);
                self.retire_texture_job()?;
            }
            return Ok(false)
        }
        let image=self.texture_image.as_ref().unwrap();
        let descriptor=SceneRasterDescriptor{width:image.width,height:image.height,source_digest:self.texture_digest,source_revision:1,profile:if self.texture_role==0{SceneRasterProfile::MeshBaseColorSrgb}else{SceneRasterProfile::MeshNumericLinear},mesh:None};
        let pool=world_scene_raster_pool();let owner=self.texture_digest[0].wrapping_add(self.texture_digest[1]).max(1);
        let lease=match pool.begin(descriptor,owner,SceneRasterWriteMode::Moved { capacity_bytes: image.pixels.capacity() }){SceneRasterBegin::Reused(lease)=>{self.texture_image=None;lease},SceneRasterBegin::Writer(writer)=>{let image=self.texture_image.take().unwrap();match pool.prepare_moved(writer,image.pixels).and_then(|prepared|pool.seal_prepared_moved(prepared)){Ok(lease)=>lease,Err((pixels,_))=>{self.texture_image=Some(semio_framework_pixels::RasterImage{width:descriptor.width,height:descriptor.height,pixels});return Err(ui_wgpu::wgpu::Mesh3dFault::ByteCapacity)}}},SceneRasterBegin::Backpressure(_)=>return Err(ui_wgpu::wgpu::Mesh3dFault::ByteCapacity),SceneRasterBegin::Refused(_)=>return Err(ui_wgpu::wgpu::Mesh3dFault::Schema)};
        self.textures.push((key,lease));self.next_texture_role();Ok(false)
    }

    fn next_texture_role(&mut self){self.texture_role+=1;if self.texture_role==2{self.texture_role=0;self.texture_cursor+=1;}self.texture_name=None;self.texture_byte=0;self.texture_scan=0;self.texture_required=false;}

    pub(super) fn token(&self) -> Result<Mesh3dWriteToken, ui_wgpu::wgpu::Mesh3dFault> {
        match self.owner {
            WorldPlaceholderOwner::Writing(token) => Ok(token),
            _ => Err(ui_wgpu::wgpu::Mesh3dFault::Stale),
        }
    }

    pub(super) fn step(&mut self) -> WorldPlaceholderMeshStep {
        if self.faulted {
            return if self.close_step() && self.terminal_is_empty() { WorldPlaceholderMeshStep::Fault } else { WorldPlaceholderMeshStep::Pending };
        }
        let result = self.step_live();
        if result.is_err() {
            self.faulted = true;
            let _ = self.close_step();
            return WorldPlaceholderMeshStep::Pending;
        }
        result.expect("placeholder result checked above")
    }

    pub(super) fn step_live(&mut self) -> Result<WorldPlaceholderMeshStep, ui_wgpu::wgpu::Mesh3dFault> {
        match self.phase {
            WorldPlaceholderMeshPhase::Allocate => {
                let token = self.token()?;
                let grant = world_mesh_ownership_grant(0, ui_wgpu::wgpu::mesh3d_allocate_capacity_byte_demand(token)?, 0, 1);
                if world_mesh_ownership_complete(grant, mesh3d_allocate_step(token, grant)?)? {
                    self.phase = WorldPlaceholderMeshPhase::Positions;
                }
            }
            WorldPlaceholderMeshPhase::Positions => {
                mesh3d_write_vec3(self.token()?, Mesh3dField::Positions, self.source.position(self.source_vertex(self.item)))?;
                self.item += 1;
                if self.item == self.vertex_items {
                    self.item = 0;
                    self.phase = WorldPlaceholderMeshPhase::Normals;
                }
            }
            WorldPlaceholderMeshPhase::Normals => {
                mesh3d_write_vec3(self.token()?, Mesh3dField::Normals, self.sample::<3>(semio_framework::MeshAttributeSemantic::Normal,self.item,self.source.normal(self.source_vertex(self.item)))?)?;
                self.item += 1;
                if self.item == self.vertex_items {
                    self.item = 0;
                    self.phase = WorldPlaceholderMeshPhase::Tangents;
                }
            }
            WorldPlaceholderMeshPhase::Tangents=>{
                if self.schema.tangents>0 {
                    let WorldMeshInput::Inline(mesh)=&self.source else{return Err(ui_wgpu::wgpu::Mesh3dFault::Schema)};
                    let attribute=mesh.attributes.get("tangent").ok_or(ui_wgpu::wgpu::Mesh3dFault::Schema)?;
                    let mut value=[1.0,0.0,0.0,1.0];
                    if !(self.expanded && self.item>=self.index_items && attribute.domain!=semio_framework::MeshAttributeDomain::Vertex) {
                        let domain=match attribute.domain{semio_framework::MeshAttributeDomain::Vertex=>self.source_vertex(self.item) as usize,semio_framework::MeshAttributeDomain::Corner=>self.item as usize,semio_framework::MeshAttributeDomain::Face=>self.item as usize/3,_=>return Err(ui_wgpu::wgpu::Mesh3dFault::Schema)};
                        let values=attribute.value_at(domain).and_then(|value|value.as_array()).filter(|values|values.len()==4).ok_or(ui_wgpu::wgpu::Mesh3dFault::Schema)?;
                        for axis in 0..4 {value[axis]=values[axis].as_f64().filter(|value|value.is_finite() && value.abs()<=f32::MAX as f64).ok_or(ui_wgpu::wgpu::Mesh3dFault::Schema)? as f32;}
                    }
                    if value[..3].iter().all(|value|*value==0.0) || value[3].abs()!=1.0{return Err(ui_wgpu::wgpu::Mesh3dFault::Schema)}
                    mesh3d_write_vec4(self.token()?,Mesh3dField::Tangents,value)?;self.item+=1;
                }
                if self.schema.tangents==0 || self.item==self.vertex_items{self.item=0;self.phase=WorldPlaceholderMeshPhase::Uvs;}
            }
            WorldPlaceholderMeshPhase::Uvs => {
                if self.schema.uvs>0 {let role=self.uv_role;let value=self.uv_sample(role,self.item)?;ui_wgpu::wgpu::mesh3d_write_vec2(self.token()?,[Mesh3dField::Uvs,Mesh3dField::UvsMetallicRoughness,Mesh3dField::UvsNormal,Mesh3dField::UvsOcclusion,Mesh3dField::UvsEmissive][role],value)?;self.uv_role+=1;if self.uv_role==5{self.uv_role=0;self.item+=1;}}
                if self.schema.uvs==0 || self.item==self.vertex_items {self.item=0;self.phase=WorldPlaceholderMeshPhase::Colors;}
            }
            WorldPlaceholderMeshPhase::Colors => {
                let token=self.token()?;
                if self.schema.colors>0 {let fallback=match &self.source {WorldMeshInput::Inline(mesh)=>{let at=self.source_vertex(self.item) as usize*4;std::array::from_fn(|axis|mesh.colors.get(at+axis).copied().unwrap_or(1.0))},_=>[1.0;4]};mesh3d_write_vec4(token,Mesh3dField::Colors,self.sample::<4>(semio_framework::MeshAttributeSemantic::Color,self.item,fallback)?)?;self.item+=1;}
                if self.schema.colors==0 || self.item==self.vertex_items {self.item=0;self.phase=WorldPlaceholderMeshPhase::Indices;}
            }
            WorldPlaceholderMeshPhase::Indices => {
                if self.item < self.index_items {
                    mesh3d_write_u32(self.token()?, Mesh3dField::Indices, if self.expanded {self.item}else{self.source.index(self.item)})?;
                    self.item += 1;
                } else {
                    self.item = 0;
                    self.phase = WorldPlaceholderMeshPhase::FaceIds;
                }
            }
            WorldPlaceholderMeshPhase::FaceIds => {
                if self.item < self.schema.face_ids {let WorldMeshInput::Inline(mesh)=&self.source else{return Err(ui_wgpu::wgpu::Mesh3dFault::Schema)};mesh3d_write_u32(self.token()?,Mesh3dField::FaceIds,mesh.face_ids[self.item as usize])?;self.item+=1;}
                else {self.item=0;self.phase=WorldPlaceholderMeshPhase::VertexIds;}
            }
            WorldPlaceholderMeshPhase::VertexIds => {
                if self.item < self.schema.vertex_ids {let WorldMeshInput::Inline(mesh)=&self.source else{return Err(ui_wgpu::wgpu::Mesh3dFault::Schema)};let id=*mesh.vertex_ids.get(self.source_vertex(self.item) as usize).ok_or(ui_wgpu::wgpu::Mesh3dFault::Schema)?;mesh3d_write_u32(self.token()?,Mesh3dField::VertexIds,id)?;self.item+=1;}
                else {self.item=0;self.phase=WorldPlaceholderMeshPhase::Edges;}
            }
            WorldPlaceholderMeshPhase::Edges => {
                if self.item < self.schema.edges {let WorldMeshInput::Inline(mesh)=&self.source else{return Err(ui_wgpu::wgpu::Mesh3dFault::Schema)};let at=self.item as usize*6;ui_wgpu::wgpu::mesh3d_write_edge(self.token()?,[[mesh.edge_positions[at],mesh.edge_positions[at+1],mesh.edge_positions[at+2]],[mesh.edge_positions[at+3],mesh.edge_positions[at+4],mesh.edge_positions[at+5]]])?;self.item+=1;}
                else {self.item=0;self.phase=WorldPlaceholderMeshPhase::EdgeIds;}
            }
            WorldPlaceholderMeshPhase::EdgeIds => {
                if self.item < self.schema.edge_ids {let WorldMeshInput::Inline(mesh)=&self.source else{return Err(ui_wgpu::wgpu::Mesh3dFault::Schema)};mesh3d_write_u32(self.token()?,Mesh3dField::EdgeIds,mesh.edge_ids[self.item as usize])?;self.item+=1;}
                else {self.item=0;self.phase=WorldPlaceholderMeshPhase::Materials;}
            }
            WorldPlaceholderMeshPhase::Materials => {
                if self.item==self.index_items {self.item=0;self.phase=WorldPlaceholderMeshPhase::Textures;}
                else {self.material_step()?;self.item+=3;}
            }
            WorldPlaceholderMeshPhase::Textures => {
                if self.texture_step()? {
                    if !self.primitives.is_empty() {self.appearance=Some(World3dMeshAppearance::new(std::mem::take(&mut self.primitives),std::mem::take(&mut self.textures)).map_err(|(primitives,textures)|{self.primitives=primitives;self.textures=textures;ui_wgpu::wgpu::Mesh3dFault::ByteCapacity})?);}
                    self.phase=WorldPlaceholderMeshPhase::Provenance;
                }
            }
            WorldPlaceholderMeshPhase::Provenance => {
                let token = self.token()?;
                if let WorldMeshInput::Inline(mesh) = &mut self.source {
                    let grant = world_mesh_ownership_grant(0, ui_wgpu::wgpu::mesh3d_component_capacity_byte_demand(&mesh.component_references)?, 0, 1);
                    let progress = ui_wgpu::wgpu::mesh3d_move_component_references(token, &mut mesh.component_references, grant)?;
                    if !progress.fits(grant) { return Err(ui_wgpu::wgpu::Mesh3dFault::ByteCapacity); }
                }
                self.phase = WorldPlaceholderMeshPhase::AdmitProvenance;
            }
            WorldPlaceholderMeshPhase::AdmitProvenance => {
                let grant = world_mesh_ownership_grant(ui_wgpu::wgpu::kernel_3d_scene::MESH3D_PAGE_BYTES, 0, 0, 1);
                let step = ui_wgpu::wgpu::mesh3d_component_admission_step(self.token()?, grant)?;
                if !step.ownership.fits(grant) { return Err(ui_wgpu::wgpu::Mesh3dFault::Closing); }
                if step.complete { self.phase = WorldPlaceholderMeshPhase::Seal; }
            }
            WorldPlaceholderMeshPhase::Seal => {
                let lease = mesh3d_seal(self.token()?)?;
                self.owner = WorldPlaceholderOwner::Ready(lease);
                self.phase = WorldPlaceholderMeshPhase::Publish;
            }
            WorldPlaceholderMeshPhase::Publish => {
                let grant=self.source_retirement_grant(4096)?;
                if !matches!(self.retire_source_step(grant)?,semio_framework_value::retained_clone::RetainedCloneStep::Complete(_)){return Ok(WorldPlaceholderMeshStep::Pending)}
                let WorldPlaceholderOwner::Ready(lease) = self.owner else { return Err(ui_wgpu::wgpu::Mesh3dFault::Stale) };
                self.owner = WorldPlaceholderOwner::Empty;
                return Ok(WorldPlaceholderMeshStep::Ready(std::mem::take(&mut self.key), lease,self.appearance.take()));
            }
        }
        Ok(WorldPlaceholderMeshStep::Pending)
    }

    pub(super) fn source_retirement_grant(&self,copy_bytes:usize)->Result<semio_framework_value::retained_clone::RetainedCloneGrant,ui_wgpu::wgpu::Mesh3dFault> {
        let Some(owner)=self.source_retirement.as_ref() else{return Ok(world_mesh_ownership_grant(0,0,0,usize::from(matches!(self.source,WorldMeshInput::Inline(_)))))};
        owner.next_copy_byte_demand().map_err(|_|ui_wgpu::wgpu::Mesh3dFault::Closing)?;
        Ok(world_mesh_ownership_grant(copy_bytes,owner.next_capacity_byte_demand(copy_bytes).map_err(|_|ui_wgpu::wgpu::Mesh3dFault::Closing)?,owner.next_release_byte_demand().map_err(|_|ui_wgpu::wgpu::Mesh3dFault::Closing)?,owner.next_depth_demand().map_err(|_|ui_wgpu::wgpu::Mesh3dFault::Closing)?))
    }

    pub(super) fn retire_source_step(&mut self,grant:semio_framework_value::retained_clone::RetainedCloneGrant)->Result<semio_framework_value::retained_clone::RetainedCloneStep,ui_wgpu::wgpu::Mesh3dFault> {
        use semio_framework_value::retained_clone::{RetainedCloneProgress,RetainedCloneStep};
        if let Some(owner)=self.source_retirement.as_mut(){
            let step=owner.step(grant).map_err(|_|ui_wgpu::wgpu::Mesh3dFault::Closing)?;
            let progress=match step{RetainedCloneStep::Progress(progress)|RetainedCloneStep::Complete(progress)=>progress};
            if !progress.fits(grant){return Err(ui_wgpu::wgpu::Mesh3dFault::Closing)}
            if matches!(step,RetainedCloneStep::Complete(_)){
                if !owner.terminal_is_empty(){return Err(ui_wgpu::wgpu::Mesh3dFault::Closing)}
                self.source_retirement=None;
            }
            return Ok(step)
        }
        if !matches!(self.source,WorldMeshInput::Inline(_)){return Ok(RetainedCloneStep::Complete(RetainedCloneProgress::default()))}
        if grant.maximum_items==0 || grant.maximum_depth==0{return Ok(RetainedCloneStep::Progress(RetainedCloneProgress::default()))}
        let WorldMeshInput::Inline(mesh)=std::mem::replace(&mut self.source,WorldMeshInput::Placeholder(WorldPlaceholderKind::Plane)) else{unreachable!()};
        match semio_framework_value::retirement::controlled::ControlledRetirement::new(mesh){
            Ok(owner)=>{self.source_retirement=Some(owner);Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,..RetainedCloneProgress::default()}))},
            Err((_,mesh))=>{self.source=WorldMeshInput::Inline(mesh);Err(ui_wgpu::wgpu::Mesh3dFault::Closing)}
        }
    }

    fn retire_texture_job(&mut self)->Result<bool,ui_wgpu::wgpu::Mesh3dFault> {
        if let Some(owner)=self.texture_retirement.as_mut(){
            let copy=owner.next_copy_byte_demand().map_err(|_|ui_wgpu::wgpu::Mesh3dFault::Closing)?;
            let grant=world_mesh_ownership_grant(copy,owner.next_capacity_byte_demand(copy).map_err(|_|ui_wgpu::wgpu::Mesh3dFault::Closing)?,owner.next_release_byte_demand().map_err(|_|ui_wgpu::wgpu::Mesh3dFault::Closing)?,owner.next_depth_demand().map_err(|_|ui_wgpu::wgpu::Mesh3dFault::Closing)?);
            let step=owner.step(grant).map_err(|_|ui_wgpu::wgpu::Mesh3dFault::Closing)?;
            let receipt=match step{semio_framework_value::retained_clone::RetainedCloneStep::Progress(receipt)|semio_framework_value::retained_clone::RetainedCloneStep::Complete(receipt)=>receipt};
            if !receipt.fits(grant){return Err(ui_wgpu::wgpu::Mesh3dFault::Closing)}
            if matches!(step,semio_framework_value::retained_clone::RetainedCloneStep::Complete(_)){self.texture_retirement=None;return Ok(true)}
            return Ok(false)
        }
        let Some(job)=self.texture_job.take() else{return Ok(true)};
        match semio_framework_value::retirement::controlled::ControlledRetirement::new(job){Ok(owner)=>{self.texture_retirement=Some(owner);Ok(false)},Err((_,job))=>{self.texture_job=Some(job);Err(ui_wgpu::wgpu::Mesh3dFault::Closing)}}
    }

    pub(super) fn close_step(&mut self) -> bool {
        self.texture_cancelled.store(true,std::sync::atomic::Ordering::Release);
        if self.texture_submission.take().is_some(){self.texture_worker=None;return false}
        if let Some(worker)=self.texture_worker.as_ref(){let Ok(mut output)=worker.try_lock()else{return false};let Some(result)=output.take()else{return false};self.texture_image=result;drop(output);self.texture_worker=None;return false}
        if self.texture_bytes.pop().is_some(){return false}
        if let Some(name)=self.texture_name.as_mut(){if name.pop().is_some(){return false}self.texture_name=None;return false}
        match self.source_retirement_grant(4096).and_then(|grant|self.retire_source_step(grant)){
            Ok(semio_framework_value::retained_clone::RetainedCloneStep::Complete(_))=>{},
            Ok(semio_framework_value::retained_clone::RetainedCloneStep::Progress(_))=>return false,
            Err(_)=>{self.faulted=true;return false}
        }
        if let Some(appearance)=self.appearance.as_mut(){if !appearance.close_step(){return false}self.appearance=None;return false}
        if self.primitives.pop().is_some()||self.textures.pop().is_some(){return false}
        if let Some(job)=self.texture_job.as_mut(){job.cancel();}
        match self.retire_texture_job(){Ok(true)=>{},Ok(false)=>return false,Err(_)=>{self.faulted=true;return false}}
        if let Some(image)=self.texture_image.as_mut(){if image.pixels.pop().is_some(){return false}self.texture_image=None;return false}

        match self.owner {
            WorldPlaceholderOwner::Writing(token) => {
                if !self.close_started {
                    match mesh3d_abort(token) {
                        Ok(()) | Err(ui_wgpu::wgpu::Mesh3dFault::Closing) => self.close_started = true,
                        Err(ui_wgpu::wgpu::Mesh3dFault::Stale) => self.owner = WorldPlaceholderOwner::Empty,
                        Err(_) => return false,
                    }
                    return false;
                }
                match ui_wgpu::wgpu::mesh3d_abort_demands(token, ui_wgpu::wgpu::kernel_3d_scene::MESH3D_PAGE_BYTES).and_then(|demand| { let grant = world_mesh_ownership_grant(ui_wgpu::wgpu::kernel_3d_scene::MESH3D_PAGE_BYTES, demand.capacity_bytes, demand.release_bytes, demand.depth); world_mesh_ownership_complete(grant, mesh3d_abort_step(token, grant)?) }) {
                    Ok(true) | Err(ui_wgpu::wgpu::Mesh3dFault::Stale) => self.owner = WorldPlaceholderOwner::Empty,
                    Ok(false) | Err(_) => return false,
                }
                false
            }
            WorldPlaceholderOwner::Ready(lease) => {
                if !self.close_started {
                    match mesh3d_begin_close(lease) {
                        Ok(()) | Err(ui_wgpu::wgpu::Mesh3dFault::Closing) => self.close_started = true,
                        Err(ui_wgpu::wgpu::Mesh3dFault::Stale) => self.owner = WorldPlaceholderOwner::Empty,
                        Err(_) => return false,
                    }
                    return false;
                }
                match ui_wgpu::wgpu::mesh3d_close_demands(lease, ui_wgpu::wgpu::kernel_3d_scene::MESH3D_PAGE_BYTES).and_then(|demand| { let grant = world_mesh_ownership_grant(ui_wgpu::wgpu::kernel_3d_scene::MESH3D_PAGE_BYTES, demand.capacity_bytes, demand.release_bytes, demand.depth); world_mesh_ownership_complete(grant, mesh3d_close_step(lease, grant)?) }) {
                    Ok(true) | Err(ui_wgpu::wgpu::Mesh3dFault::Stale) => self.owner = WorldPlaceholderOwner::Empty,
                    Ok(false) | Err(_) => return false,
                }
                false
            }
            WorldPlaceholderOwner::Empty => {
                if self.key.pop().is_some() {
                    return false;
                }
                true
            }
        }
    }

    pub(super) fn terminal_is_empty(&self) -> bool {
        match self.owner {
            WorldPlaceholderOwner::Writing(_) => false,
            WorldPlaceholderOwner::Ready(lease) => mesh3d_terminal_is_empty(lease),
            WorldPlaceholderOwner::Empty => self.key.is_empty() && self.source_retirement.is_none() && !matches!(self.source,WorldMeshInput::Inline(_)) && self.appearance.is_none() && self.primitives.is_empty() && self.textures.is_empty() && self.texture_job.is_none() && self.texture_retirement.is_none() && self.texture_image.is_none() && self.texture_worker.is_none() && self.texture_submission.is_none() && self.texture_bytes.is_empty() && self.texture_name.is_none(),
        }
    }
}

#[cfg(not(test))]
impl Drop for WorldPlaceholderMeshCursor {
    fn drop(&mut self) {
        assert!(self.terminal_is_empty() || std::thread::panicking(), "placeholder mesh cursor reached Drop before its exact authority witness");
    }
}

pub(super) fn placeholder_triangle_normal(triangle: [[f32; 3]; 3]) -> [f32; 3] {
    let ab = [triangle[1][0] - triangle[0][0], triangle[1][1] - triangle[0][1], triangle[1][2] - triangle[0][2]];
    let ac = [triangle[2][0] - triangle[0][0], triangle[2][1] - triangle[0][1], triangle[2][2] - triangle[0][2]];
    placeholder_normalize3([ab[1] * ac[2] - ab[2] * ac[1], ab[2] * ac[0] - ab[0] * ac[2], ab[0] * ac[1] - ab[1] * ac[0]])
}

pub(super) fn placeholder_triangle(kind: WorldPlaceholderKind, triangle: u32) -> [[f32; 3]; 3] {
    match kind {
        WorldPlaceholderKind::Box => {
            let faces = [
                ([-0.5, -0.5, 0.5], [0.5, -0.5, 0.5], [0.5, 0.5, 0.5], [-0.5, 0.5, 0.5]),
                ([0.5, -0.5, -0.5], [-0.5, -0.5, -0.5], [-0.5, 0.5, -0.5], [0.5, 0.5, -0.5]),
                ([-0.5, 0.5, 0.5], [0.5, 0.5, 0.5], [0.5, 0.5, -0.5], [-0.5, 0.5, -0.5]),
                ([-0.5, -0.5, -0.5], [0.5, -0.5, -0.5], [0.5, -0.5, 0.5], [-0.5, -0.5, 0.5]),
                ([0.5, -0.5, 0.5], [0.5, -0.5, -0.5], [0.5, 0.5, -0.5], [0.5, 0.5, 0.5]),
                ([-0.5, -0.5, -0.5], [-0.5, -0.5, 0.5], [-0.5, 0.5, 0.5], [-0.5, 0.5, -0.5]),
            ];
            let (a, b, c, d) = faces[(triangle / 2) as usize];
            if triangle.is_multiple_of(2) {
                [a, b, c]
            } else {
                [a, c, d]
            }
        }
        WorldPlaceholderKind::Plane => {
            if triangle == 0 {
                [[-0.5, 0.0, -0.5], [0.5, 0.0, -0.5], [0.5, 0.0, 0.5]]
            } else {
                [[-0.5, 0.0, -0.5], [0.5, 0.0, 0.5], [-0.5, 0.0, 0.5]]
            }
        }
        WorldPlaceholderKind::Cylinder => {
            let segment = triangle / 4;
            let a0 = segment as f32 / 16.0 * std::f32::consts::TAU;
            let a1 = (segment + 1) as f32 / 16.0 * std::f32::consts::TAU;
            let p00 = [0.5 * a0.cos(), -0.5, 0.5 * a0.sin()];
            let p01 = [0.5 * a1.cos(), -0.5, 0.5 * a1.sin()];
            let p10 = [0.5 * a0.cos(), 0.5, 0.5 * a0.sin()];
            let p11 = [0.5 * a1.cos(), 0.5, 0.5 * a1.sin()];
            match triangle % 4 {
                0 => [p00, p01, p11],
                1 => [p00, p11, p10],
                2 => [[0.0, -0.5, 0.0], p01, p00],
                _ => [[0.0, 0.5, 0.0], p10, p11],
            }
        }
        WorldPlaceholderKind::Cone => {
            let segment = triangle / 2;
            let a0 = segment as f32 / 16.0 * std::f32::consts::TAU;
            let a1 = (segment + 1) as f32 / 16.0 * std::f32::consts::TAU;
            let p0 = [0.5 * a0.cos(), 0.0, 0.5 * a0.sin()];
            let p1 = [0.5 * a1.cos(), 0.0, 0.5 * a1.sin()];
            if triangle.is_multiple_of(2) {
                [[0.0, 1.0, 0.0], p1, p0]
            } else {
                [[0.0, 0.0, 0.0], p0, p1]
            }
        }
        WorldPlaceholderKind::Icosphere => {
            let t = (1.0 + 5.0_f32.sqrt()) * 0.5;
            let vertices = [[-1.0, t, 0.0], [1.0, t, 0.0], [-1.0, -t, 0.0], [1.0, -t, 0.0], [0.0, -1.0, t], [0.0, 1.0, t], [0.0, -1.0, -t], [0.0, 1.0, -t], [t, 0.0, -1.0], [t, 0.0, 1.0], [-t, 0.0, -1.0], [-t, 0.0, 1.0]];
            let faces =
                [[0, 11, 5], [0, 5, 1], [0, 1, 7], [0, 7, 10], [0, 10, 11], [1, 5, 9], [5, 11, 4], [11, 10, 2], [10, 7, 6], [7, 1, 8], [3, 9, 4], [3, 4, 2], [3, 2, 6], [3, 6, 8], [3, 8, 9], [4, 9, 5], [2, 4, 11], [6, 2, 10], [8, 6, 7], [9, 8, 1]];
            let face = faces[(triangle / 4) as usize];
            let a = placeholder_normalize3(vertices[face[0]]);
            let b = placeholder_normalize3(vertices[face[1]]);
            let c = placeholder_normalize3(vertices[face[2]]);
            let ab = placeholder_normalize3([(a[0] + b[0]) * 0.5, (a[1] + b[1]) * 0.5, (a[2] + b[2]) * 0.5]);
            let bc = placeholder_normalize3([(b[0] + c[0]) * 0.5, (b[1] + c[1]) * 0.5, (b[2] + c[2]) * 0.5]);
            let ca = placeholder_normalize3([(c[0] + a[0]) * 0.5, (c[1] + a[1]) * 0.5, (c[2] + a[2]) * 0.5]);
            let triangle = match triangle % 4 {
                0 => [a, ab, ca],
                1 => [b, bc, ab],
                2 => [c, ca, bc],
                _ => [ab, bc, ca],
            };
            triangle.map(|vertex| placeholder_scale3(vertex, 0.12))
        }
    }
}

pub(super) fn begin_world_placeholder_mesh(state: &mut World3dState, key: &str, kind: WorldPlaceholderKind) {
    if state.meshes.contains_key(key) || state.placeholder_build.is_some() || state.dynamic_retirement.is_some() {
        return;
    }
    state.placeholder_generation = state.placeholder_generation.wrapping_add(1).max(1);
    match WorldPlaceholderMeshCursor::new(key, kind, state.placeholder_generation, state.interaction_revision) {
        Ok(cursor) => state.placeholder_build = Some(cursor),
        Err(_) => mark_world_dynamic_fault(state, WorldDynamicFault::ByteCapacity),
    }
}

pub(super) fn step_world_placeholder_mesh(state: &mut World3dState) {
    let Some(mut cursor) = state.placeholder_build.take() else { return };
    match cursor.step() {
        WorldPlaceholderMeshStep::Pending => state.placeholder_build = Some(cursor),
        WorldPlaceholderMeshStep::Ready(key,lease,appearance) => {if let Some(appearance)=appearance {let asset=World3dMeshAsset{mesh:lease,appearance};if let Err(rejected)=publish_world3d_inline_asset(state,key,asset){cursor.owner=WorldPlaceholderOwner::Ready(rejected.value.mesh);cursor.appearance=Some(rejected.value.appearance);cursor.faulted=true;state.placeholder_build=Some(cursor);mark_world_dynamic_fault(state,rejected.fault);}}else{store_mesh(state,key,lease)}},
        WorldPlaceholderMeshStep::Fault => mark_world_dynamic_fault(state, WorldDynamicFault::Closing),
    }
}
//#endregion PlaceholderMeshAuthority

//#region WorldMeshBuffers
pub(super) type WorldMeshBuffers=semio_framework::MeshData;

trait WorldMeshSurface {
    fn position(&self,vertex:u32)->[f32;3];
    fn normal(&self,vertex:u32)->[f32;3];
}

impl WorldMeshSurface for WorldMeshBuffers {
    fn position(&self,vertex:u32)->[f32;3] {let at=vertex as usize*3;[self.positions.get(at).copied().unwrap_or(0.0),self.positions.get(at+1).copied().unwrap_or(0.0),self.positions.get(at+2).copied().unwrap_or(0.0)]}
    fn normal(&self,vertex:u32)->[f32;3] {let at=vertex as usize*3;[self.normals.get(at).copied().unwrap_or(0.0),self.normals.get(at+1).copied().unwrap_or(0.0),self.normals.get(at+2).copied().unwrap_or(1.0)]}
}

//#endregion WorldMeshBuffers

//#region PlaceholderMesh
/// 🧊️ Viewport placeholder markers only (gumball plane, reference plane, vortex arrow parts, and
/// the box fallback for an unresolved `mesh_id`) — demo geometry, never document content. Ports
/// only the primitive kinds actually reachable from this file's `mesh_from_kind` call sites
/// (census in 📓️wave-g1b-infinite-report.md); the dissolved mesh-engine's other primitives
/// (uv-sphere, ico-sphere at other radii, torus, …) are not reachable here and were not ported.
#[cfg(test)]
pub(super) fn placeholder_mesh(kind: &str) -> WorldMeshBuffers {
    match kind {
        "vortex-marker" => placeholder_ico_sphere(0.12, 1),
        "plane" => placeholder_plane(1.0, 1.0),
        "cylinder" => placeholder_cylinder(0.5, 1.0, 16),
        "cone" => placeholder_cone(0.5, 1.0, 16),
        _ => placeholder_box(1.0, 1.0, 1.0),
    }
}

#[cfg(test)]
fn placeholder_push_triangle(mesh: &mut WorldMeshBuffers, a: [f32; 3], b: [f32; 3], c: [f32; 3]) {
    let base = mesh.vertex_count() as u32;
    mesh.positions.extend_from_slice(&[a[0], a[1], a[2], b[0], b[1], b[2], c[0], c[1], c[2]]);
    mesh.indices.extend_from_slice(&[base, base + 1, base + 2]);
}

#[cfg(test)]
fn placeholder_box(width: f32, height: f32, depth: f32) -> WorldMeshBuffers {
    let hw = width * 0.5;
    let hh = height * 0.5;
    let hd = depth * 0.5;
    let mut mesh = WorldMeshBuffers::default();
    let faces = [
        ([-hw, -hh, hd], [hw, -hh, hd], [hw, hh, hd], [-hw, hh, hd]),
        ([hw, -hh, -hd], [-hw, -hh, -hd], [-hw, hh, -hd], [hw, hh, -hd]),
        ([-hw, hh, hd], [hw, hh, hd], [hw, hh, -hd], [-hw, hh, -hd]),
        ([-hw, -hh, -hd], [hw, -hh, -hd], [hw, -hh, hd], [-hw, -hh, hd]),
        ([hw, -hh, hd], [hw, -hh, -hd], [hw, hh, -hd], [hw, hh, hd]),
        ([-hw, -hh, -hd], [-hw, -hh, hd], [-hw, hh, hd], [-hw, hh, -hd]),
    ];
    for (a, b, c, d) in faces {
        placeholder_push_triangle(&mut mesh, a, b, c);
        placeholder_push_triangle(&mut mesh, a, c, d);
    }
    mesh.compute_normals();
    mesh
}

#[cfg(test)]
fn placeholder_plane(width: f32, depth: f32) -> WorldMeshBuffers {
    let hw = width * 0.5;
    let hd = depth * 0.5;
    let mut mesh = WorldMeshBuffers::default();
    placeholder_push_triangle(&mut mesh, [-hw, 0.0, -hd], [hw, 0.0, -hd], [hw, 0.0, hd]);
    placeholder_push_triangle(&mut mesh, [-hw, 0.0, -hd], [hw, 0.0, hd], [-hw, 0.0, hd]);
    mesh.compute_normals();
    mesh
}

#[cfg(test)]
fn placeholder_cylinder(radius: f32, height: f32, segments: u32) -> WorldMeshBuffers {
    let mut mesh = WorldMeshBuffers::default();
    let half = height * 0.5;
    for seg in 0..segments {
        let u0 = seg as f32 / segments as f32;
        let u1 = (seg + 1) as f32 / segments as f32;
        let a0 = u0 * std::f32::consts::TAU;
        let a1 = u1 * std::f32::consts::TAU;
        let p00 = [radius * a0.cos(), -half, radius * a0.sin()];
        let p01 = [radius * a1.cos(), -half, radius * a1.sin()];
        let p10 = [radius * a0.cos(), half, radius * a0.sin()];
        let p11 = [radius * a1.cos(), half, radius * a1.sin()];
        placeholder_push_triangle(&mut mesh, p00, p01, p11);
        placeholder_push_triangle(&mut mesh, p00, p11, p10);
        placeholder_push_triangle(&mut mesh, [0.0, -half, 0.0], p01, p00);
        placeholder_push_triangle(&mut mesh, [0.0, half, 0.0], p10, p11);
    }
    mesh.compute_normals();
    mesh
}

#[cfg(test)]
fn placeholder_cone(radius: f32, height: f32, segments: u32) -> WorldMeshBuffers {
    let mut mesh = WorldMeshBuffers::default();
    let apex = [0.0, height, 0.0];
    for seg in 0..segments {
        let u0 = seg as f32 / segments as f32;
        let u1 = (seg + 1) as f32 / segments as f32;
        let a0 = u0 * std::f32::consts::TAU;
        let a1 = u1 * std::f32::consts::TAU;
        let p0 = [radius * a0.cos(), 0.0, radius * a0.sin()];
        let p1 = [radius * a1.cos(), 0.0, radius * a1.sin()];
        placeholder_push_triangle(&mut mesh, apex, p1, p0);
        placeholder_push_triangle(&mut mesh, [0.0, 0.0, 0.0], p0, p1);
    }
    mesh.compute_normals();
    mesh
}

fn placeholder_normalize3(v: [f32; 3]) -> [f32; 3] {
    let len = (v[0] * v[0] + v[1] * v[1] + v[2] * v[2]).sqrt();
    [v[0] / len, v[1] / len, v[2] / len]
}

fn placeholder_scale3(v: [f32; 3], s: f32) -> [f32; 3] {
    [v[0] * s, v[1] * s, v[2] * s]
}

#[cfg(test)]
fn placeholder_midpoint(verts: &mut Vec<[f32; 3]>, cache: &mut HashMap<(u32, u32), u32>, a: u32, b: u32) -> u32 {
    let key = if a < b { (a, b) } else { (b, a) };
    if let Some(index) = cache.get(&key) {
        return *index;
    }
    let mid = placeholder_normalize3([(verts[a as usize][0] + verts[b as usize][0]) * 0.5, (verts[a as usize][1] + verts[b as usize][1]) * 0.5, (verts[a as usize][2] + verts[b as usize][2]) * 0.5]);
    let index = verts.len() as u32;
    verts.push(mid);
    cache.insert(key, index);
    index
}

#[cfg(test)]
fn placeholder_ico_sphere(radius: f32, subdivisions: u32) -> WorldMeshBuffers {
    let t = (1.0 + 5.0_f32.sqrt()) * 0.5;
    let mut verts = vec![
        placeholder_normalize3([-1.0, t, 0.0]),
        placeholder_normalize3([1.0, t, 0.0]),
        placeholder_normalize3([-1.0, -t, 0.0]),
        placeholder_normalize3([1.0, -t, 0.0]),
        placeholder_normalize3([0.0, -1.0, t]),
        placeholder_normalize3([0.0, 1.0, t]),
        placeholder_normalize3([0.0, -1.0, -t]),
        placeholder_normalize3([0.0, 1.0, -t]),
        placeholder_normalize3([t, 0.0, -1.0]),
        placeholder_normalize3([t, 0.0, 1.0]),
        placeholder_normalize3([-t, 0.0, -1.0]),
        placeholder_normalize3([-t, 0.0, 1.0]),
    ];
    let mut faces =
        vec![[0, 11, 5], [0, 5, 1], [0, 1, 7], [0, 7, 10], [0, 10, 11], [1, 5, 9], [5, 11, 4], [11, 10, 2], [10, 7, 6], [7, 1, 8], [3, 9, 4], [3, 4, 2], [3, 2, 6], [3, 6, 8], [3, 8, 9], [4, 9, 5], [2, 4, 11], [6, 2, 10], [8, 6, 7], [9, 8, 1]];
    for _ in 0..subdivisions {
        let mut next = Vec::new();
        let mut midpoint_cache = HashMap::new();
        for face in &faces {
            let a = placeholder_midpoint(&mut verts, &mut midpoint_cache, face[0], face[1]);
            let b = placeholder_midpoint(&mut verts, &mut midpoint_cache, face[1], face[2]);
            let c = placeholder_midpoint(&mut verts, &mut midpoint_cache, face[2], face[0]);
            next.extend_from_slice(&[[face[0], a, c], [face[1], b, a], [face[2], c, b], [a, b, c]]);
        }
        faces = next;
    }
    let mut mesh = WorldMeshBuffers::default();
    for face in faces {
        let a = placeholder_scale3(verts[face[0] as usize], radius);
        let b = placeholder_scale3(verts[face[1] as usize], radius);
        let c = placeholder_scale3(verts[face[2] as usize], radius);
        placeholder_push_triangle(&mut mesh, a, b, c);
    }
    mesh.compute_normals();
    mesh
}
//#endregion PlaceholderMesh
//#endregion MeshHelpers
