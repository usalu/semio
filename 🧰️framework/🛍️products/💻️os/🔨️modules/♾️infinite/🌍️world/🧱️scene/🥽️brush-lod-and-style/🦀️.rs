//! 🥽️ World3d brush mesh announce, reference images, mesh lease publication, chunking, lod grid, mesh style and the mesh pool.

use super::*;

//#region 🥽️WorldBrushMeshAnnounce
/// 📏️ Values — `f32` positions and `u32` indices counted together — one `registerBrushMesh` page
/// carries at most. The guest's own `PUZZLE3D_MESH_PAGE_VALUES` and React's twin of it
/// (`🛠️ShellHelpers/🟦️.tsx`): 1 024 values are 4 096 payload bytes and 5 464 base64 characters,
/// which the shared retained command's [`WORLD_BRUSH_MESH_COMMAND_RAW_BYTES`] carries with the JSON
/// envelope still inside the limit.
const WORLD_BRUSH_MESH_PAGE_VALUES: usize = 1_024;

/// 📏️ Raw JSON bytes one retained puzzle command admits — the wire budget
/// [`world_brush_mesh_page_capacity`] fits an envelope plus its payload inside.
const WORLD_BRUSH_MESH_COMMAND_RAW_BYTES: usize = 8_192;

/// 🔤️ Base64 padding characters reserved per page — one group per payload string.
const WORLD_BRUSH_MESH_PAGE_PADDING_CHARS: usize = 8;

/// 🧮️ Longest page run one mesh identity may claim, React's `PUZZLE3D_MESH_UPLOAD_MAX_PAGES`. A run
/// longer than this uploads nothing rather than emitting a command the guest refuses.
pub const WORLD_BRUSH_MESH_MAX_PAGES: u32 = 384;

/// 🚚️ How often one identity's guest-side re-upload request may be claimed under a single guest
/// instantiation — the page run the request asks for, plus exactly one retry for a run that failed
/// part-way. React's `PUZZLE3D_MESH_REUPLOAD_CLAIMS`.
const WORLD_BRUSH_MESH_REUPLOAD_CLAIMS: u32 = 2;

/// 📦️ Mesh identities the process-wide registry tracks at once — the same ceiling the mesh table
/// this lane announces out of is bounded by.
const WORLD_BRUSH_MESH_REGISTRY_CAPACITY: usize = WORLD_DYNAMIC_MESH_CAPACITY;

pub struct WorldBrushMeshEntry {
    pub digest: String,
    paged: bool,
}

/// 🥽️ What this PROCESS believes ONE guest instantiation holds — the wgpu twin of React's page-wide
/// `Puzzle3dBrushMeshRegistry` singleton (`🛠️ShellHelpers/🟦️.tsx`). It is deliberately shared by
/// every `World3dState`, exactly as React's module-level registry is shared by every mounted
/// `World3dHost`: the FIRST window pages a mesh's bytes, and every later window announces the same
/// identity by `{url, digest}` alone instead of paging 294 912 bytes again.
///
/// The claim is bounded by the GUEST's lifetime, never by the shell's. Two guest-published facts
/// close it, both on `interactionJson` and both folded in by
/// [`sync_world3d_brush_mesh_feedback`]:
///
/// - `meshResidency` — the guest's monotone install counter. A value below the high-water mark this
///   registry saw is proof of a restart and voids every entry, which bumps `generation` so every
///   surface re-announces.
/// - `meshReuploadUrls` — identities a refused id-only announcement is waiting on bytes for, claimed
///   at most [`WORLD_BRUSH_MESH_REUPLOAD_CLAIMS`] times per instantiation so a republished stale
///   world body cannot re-drive an upload that already ran.
pub struct WorldBrushMeshRegistry {
    residency: i64,
    pub generation: u64,
    pub entries: HashMap<String, WorldBrushMeshEntry>,
    pub claims: HashMap<String, u32>,
    revisions: HashMap<String, u64>,
    refused_alias: HashSet<String>,
}

impl WorldBrushMeshRegistry {
    pub fn clear(&mut self) {
        self.entries.clear();
        self.claims.clear();
        self.refused_alias.clear();
    }

    /// 🪢️ Whether some identity PAGED this exact geometry into the live guest instantiation — React's
    /// `holdsDigest`, which scans its live entries for the same reason: an entry a re-upload request
    /// deleted is no longer proof for a sibling id, so the next announcement pages the bytes.
    fn holds_digest(&self, digest: &str) -> bool {
        !digest.is_empty() && self.entries.values().any(|entry| entry.paged && entry.digest == digest)
    }
}

static WORLD_BRUSH_MESH_REGISTRY: LazyLock<Mutex<WorldBrushMeshRegistry>> =
    LazyLock::new(|| Mutex::new(WorldBrushMeshRegistry { residency: -1, generation: 0, entries: HashMap::new(), claims: HashMap::new(), revisions: HashMap::new(), refused_alias: HashSet::new() }));

fn with_world_brush_mesh_registry<R>(read: impl FnOnce(&mut WorldBrushMeshRegistry) -> R) -> Option<R> {
    WORLD_BRUSH_MESH_REGISTRY.lock().ok().map(|mut registry| read(&mut registry))
}

/// 🔢️ The revision an identity is announced at — React's `brushMeshRevisions.generation +
/// brushMeshRevisions.urls[url]`. A surface announces once per identity per revision.
fn world_brush_mesh_revision(url: &str) -> u64 {
    with_world_brush_mesh_registry(|registry| registry.generation.saturating_add(registry.revisions.get(url).copied().unwrap_or(0))).unwrap_or(0)
}

/// 🧹️ Drops every claim this process holds, so the next announcement pages afresh. Test-facing and
/// the reset a fresh guest instantiation performs through [`sync_world3d_brush_mesh_feedback`].
pub fn reset_world3d_brush_mesh_registry() {
    with_world_brush_mesh_registry(|registry| {
        registry.clear();
        registry.revisions.clear();
        registry.residency = -1;
        registry.generation = 0;
    });
}

/// 🚚️ Folds the guest's two published facts about what it actually holds into the process registry —
/// a residency that FELL proves a restarted guest (its mesh store is not part of any checkpoint) and
/// voids every claim, and `meshReuploadUrls` names identities an announcement already sent is waiting
/// on bytes for. Either way the answer is the same: bump the revision the surfaces announce at, so
/// every world pane re-announces its resident meshes and, with the claim gone, takes the page path
/// instead of the id-only one.
pub fn sync_world3d_brush_mesh_feedback(assets_state: &mut WorldAssetsState, interaction_json: Option<&str>) {
    let Some(interaction_json) = interaction_json else { return };
    let digest = world3d_scene_digest(&[interaction_json]);
    if assets_state.brush_mesh_feedback_digest == Some(digest) {
        return;
    }
    assets_state.brush_mesh_feedback_digest = Some(digest);
    let Ok(record) = serde_json::from_str::<World3dBrushMeshFeedbackRecord>(interaction_json) else { return };
    with_world_brush_mesh_registry(|registry| {
        if let Some(installs) = record.mesh_residency {
            let installs = i64::try_from(installs).unwrap_or(i64::MAX);
            if installs < registry.residency {
                registry.clear();
                registry.generation = registry.generation.saturating_add(1);
            }
            registry.residency = installs;
        }
        for url in &record.mesh_reupload_urls {
            let claimed = registry.claims.get(url).copied().unwrap_or(0);
            if claimed >= WORLD_BRUSH_MESH_REUPLOAD_CLAIMS || registry.revisions.len() >= WORLD_BRUSH_MESH_REGISTRY_CAPACITY && !registry.revisions.contains_key(url) {
                continue;
            }
            registry.claims.insert(url.clone(), claimed.saturating_add(1));
            if registry.entries.get(url).is_some_and(|entry| !entry.paged) {
                registry.refused_alias.insert(url.clone());
            }
            registry.entries.remove(url);
            let revision = registry.revisions.get(url).copied().unwrap_or(0).saturating_add(1);
            registry.revisions.insert(url.clone(), revision);
        }
    });
}

#[derive(Deserialize)]
pub struct World3dBrushMeshFeedbackRecord {
    #[serde(rename = "meshResidency")]
    mesh_residency: Option<u64>,
    #[serde(rename = "meshReuploadUrls", default)]
    mesh_reupload_urls: Vec<String>,
}

/// 📏️ Values this mesh id may put in one page without pushing the JSON envelope past
/// [`WORLD_BRUSH_MESH_COMMAND_RAW_BYTES`] — React's `puzzle3dBrushMeshPageCapacity`, over the same
/// probe envelope. Zero means the id alone already exhausts the wire.
fn world_brush_mesh_page_capacity(surface_id: &str, url: &str, digest: &str) -> usize {
    let probe = json!({ "surfaceId": surface_id, "windowId": surface_id, "url": url, "digest": digest, "page": 999_999, "pageCount": 999_999, "positionsB64": "", "indicesB64": "" });
    let envelope = serde_json::to_string(&json!(["registerBrushMesh", probe])).map(|text| text.chars().map(|character| if character.is_ascii() { 1 } else { 6 }).sum::<usize>()).unwrap_or(usize::MAX);
    let budget = WORLD_BRUSH_MESH_COMMAND_RAW_BYTES.saturating_sub(envelope).saturating_sub(WORLD_BRUSH_MESH_PAGE_PADDING_CHARS);
    WORLD_BRUSH_MESH_PAGE_VALUES.min(budget.saturating_mul(3) / 16)
}

/// 🥽️ One resident mesh's whole collision payload, straight out of the lease's own pages: the
/// positions field's little-endian bytes followed by the indices field's. React welds the same two
/// arrays out of the loaded GLB (`extractGlbCollisionMesh`) in the same world frame — the wgpu
/// decoder bakes `glb_world_frame()` into the mesh it seals — so both renderers announce geometry
/// the guest's brush utilities can test a candidate against.
fn world_brush_mesh_payload(mesh: Mesh3dLease) -> Option<(Vec<u8>, usize)> {
    let schema = mesh.schema().ok()?;
    let position_values = (schema.vertices as usize).checked_mul(3)?;
    let total_values = position_values.checked_add(schema.indices as usize)?;
    if total_values == 0 || total_values > WORLD_BRUSH_MESH_MAX_PAGES as usize * WORLD_BRUSH_MESH_PAGE_VALUES {
        return None;
    }
    let mut bytes = Vec::with_capacity(total_values * 4);
    for field in [Mesh3dField::Positions, Mesh3dField::Indices] {
        let mut cursor = mesh.page_cursor(field).ok()?;
        loop {
            match cursor.next(|chunk| bytes.extend_from_slice(chunk)) {
                Ok(Some(())) => {}
                Ok(None) => break,
                Err(_) => return None,
            }
        }
    }
    (bytes.len() == total_values * 4).then_some((bytes, position_values))
}

fn world_brush_mesh_args(surface_id: &str, extra: semio_framework::DslValue) -> Option<semio_framework::DslValue> {
    let semio_framework::DslValue::Object(extra) = extra else { return None };
    let mut entries = vec![("surfaceId".to_string(), semio_framework::DslValue::String(surface_id.to_string())), ("windowId".to_string(), semio_framework::DslValue::String(surface_id.to_string()))];
    entries.extend(extra.into_iter().filter(|(key, _)| key != "surfaceId" && key != "windowId"));
    Some(semio_framework::DslValue::Object(entries))
}

/// 🥽️ The next `registerBrushMesh` this world surface owes the guest, or `None` when every resident
/// mesh it names is already announced at the live revision.
///
/// This is family B of the parity run's behavioural diff: React's `World3dHost` mounts a
/// `BrushMeshRegistrar` per loaded GLB per window and announces it to the puzzle guest, while the
/// wgpu host dispatched nothing at all — so the guest's brush and volume-brush utilities had no
/// collision geometry on this renderer (ticket 26/09/17/WGPU-RENDERER-REACT-PARITY,
/// `📓️w11a-prepared-world-mesh-missing.md` §6 family B).
///
/// Exactly ONE action leaves per call, and the frame drives one call per surface per frame, which is
/// React's own back pressure (`drainPuzzle3dBrushMeshQueue` awaits each page's settle before it
/// queues the next): a user action that lands mid-run ever queues behind one page, never behind a
/// whole 72-command run.
///
/// The three shapes, in the order they are taken:
///
/// 1. an open page run advances by one page, and its LAST page confirms the identity;
/// 2. an identity the process registry already holds — because a sibling window paged it, or a
///    sibling id carries byte-identical geometry — is announced by `{url, digest}` alone;
/// 3. anything else opens a page run over the resident lease's own bytes.
pub fn step_world3d_brush_mesh_announce(session_state: &mut WorldSessionState, scene_state: &mut WorldSceneState, assets_state: &mut WorldAssetsState) -> Option<ActionDescriptor> {
    if let Some(action) = world_brush_mesh_next_page(session_state, assets_state) {
        return Some(action);
    }
    let pending: Vec<(String, String)> = scene_state
        .mesh_source_urls
        .iter()
        .filter(|(mesh_key, _)| scene_state.meshes.contains_key(mesh_key.as_str()))
        .map(|(mesh_key, url)| (mesh_key.clone(), url.clone()))
        .filter(|(_, url)| assets_state.brush_mesh_announced.get(url) != Some(&world_brush_mesh_revision(url)))
        .collect();
    let (mesh_key, url) = pending.into_iter().min_by(|left, right| left.1.cmp(&right.1))?;
    let revision = world_brush_mesh_revision(&url);
    if let Some(digest) = with_world_brush_mesh_registry(|registry| registry.entries.get(&url).map(|entry| entry.digest.clone())).flatten() {
        assets_state.brush_mesh_announced.insert(url.clone(), revision);
        return Some(ActionDescriptor { controller_id: session_state.controller_id.clone(), action: "registerBrushMesh".into(), args: world_brush_mesh_args(&session_state.surface_id, semio_framework::dsl_value!({ "url": url, "digest": digest })) });
    }
    let mesh = scene_state.meshes.get(&mesh_key).copied()?;
    let Some((bytes, position_values)) = world_brush_mesh_payload(mesh) else {
        assets_state.brush_mesh_announced.insert(url, revision);
        return None;
    };
    let digest = semio_framework_hash::hash_bytes(&bytes);
    let aliasable = with_world_brush_mesh_registry(|registry| registry.holds_digest(&digest) && !registry.refused_alias.contains(&url)).unwrap_or(false);
    if aliasable {
        with_world_brush_mesh_registry(|registry| registry.entries.insert(url.clone(), WorldBrushMeshEntry { digest: digest.clone(), paged: false }));
        assets_state.brush_mesh_announced.insert(url.clone(), revision);
        return Some(ActionDescriptor { controller_id: session_state.controller_id.clone(), action: "registerBrushMesh".into(), args: world_brush_mesh_args(&session_state.surface_id, semio_framework::dsl_value!({ "url": url, "digest": digest })) });
    }
    let capacity = world_brush_mesh_page_capacity(&session_state.surface_id, &url, &digest);
    let total_values = bytes.len() / 4;
    let page_count = u32::try_from(total_values.div_ceil(capacity.max(1))).unwrap_or(u32::MAX);
    if capacity == 0 || page_count == 0 || page_count > WORLD_BRUSH_MESH_MAX_PAGES {
        assets_state.brush_mesh_announced.insert(url, revision);
        return None;
    }
    assets_state.brush_mesh_run = Some(WorldBrushMeshRun { url, digest, bytes, position_values, capacity, page: 0, page_count, revision });
    world_brush_mesh_next_page(session_state, assets_state)
}

fn world_brush_mesh_next_page(session_state: &mut WorldSessionState, assets_state: &mut WorldAssetsState) -> Option<ActionDescriptor> {
    let run = assets_state.brush_mesh_run.as_mut()?;
    if run.page >= run.page_count {
        assets_state.brush_mesh_run = None;
        return None;
    }
    let start = (run.page as usize).saturating_mul(run.capacity);
    let end = (run.bytes.len() / 4).min(start.saturating_add(run.capacity));
    let positions = &run.bytes[start.min(run.position_values) * 4..end.min(run.position_values) * 4];
    let indices = &run.bytes[start.max(run.position_values) * 4..end.max(run.position_values) * 4];
    let mut page = semio_framework::dsl_value!({ "url": run.url, "digest": run.digest, "page": run.page, "pageCount": run.page_count });
    if let semio_framework::DslValue::Object(entries) = &mut page {
        if !positions.is_empty() {
            entries.push(("positionsB64".into(), semio_framework::DslValue::String(base64_codec::base64_standard_encode(positions))));
        }
        if !indices.is_empty() {
            entries.push(("indicesB64".into(), semio_framework::DslValue::String(base64_codec::base64_standard_encode(indices))));
        }
    }
    let action = ActionDescriptor { controller_id: session_state.controller_id.clone(), action: "registerBrushMesh".into(), args: world_brush_mesh_args(&session_state.surface_id, page) };
    run.page = run.page.saturating_add(1);
    if run.page == run.page_count {
        let (url, digest, revision) = (run.url.clone(), run.digest.clone(), run.revision);
        assets_state.brush_mesh_run = None;
        with_world_brush_mesh_registry(|registry| {
            if registry.entries.len() < WORLD_BRUSH_MESH_REGISTRY_CAPACITY || registry.entries.contains_key(&url) {
                registry.entries.insert(url.clone(), WorldBrushMeshEntry { digest, paged: true });
            }
        });
        assets_state.brush_mesh_announced.insert(url, revision);
    }
    Some(action)
}

/// 🥽️ Whether this surface still owes the guest a `registerBrushMesh` — the per-frame predicate the
/// renderer's frame gate reads, so a surface mid-run keeps its own frames coming.
pub fn world3d_brush_mesh_announce_pending(scene_state: &WorldSceneState, assets_state: &WorldAssetsState) -> bool {
    assets_state.brush_mesh_run.is_some() || scene_state.mesh_source_urls.iter().any(|(mesh_key, url)| scene_state.meshes.contains_key(mesh_key.as_str()) && assets_state.brush_mesh_announced.get(url) != Some(&world_brush_mesh_revision(url)))
}
//#endregion 🥽️WorldBrushMeshAnnounce

// 🌉️ Dead on every target: repo-wide grep found zero callers of `apply_reference_image_bytes`
// (not even a test). Gated rather than deleted, to keep the diff minimal and reversible if a
// future caller lands. Does NOT remove `image` from the `wasm32-wasip2` link graph by itself —
// `🖼️canvas`'s `icon_codec::resolve_icon_kind` (used by flow's genuinely guest-reachable
// `preview_media_natural_size` widget-layout path, see
// `🔍️research/📓️infinite-host-deps-split.md`) still needs it unconditionally. RUNTIME-DEPENDENCY-
// ELIMINATION ticket 26/09/01.
#[cfg(not(all(target_arch = "wasm32", target_env = "p2")))]
/// 🖼️ The straight-RGBA budget ONE reference underlay may publish: HALF the raster lane's per-item
/// ceiling, because the producer ledger reserves a source twice over (the source plus its peak
/// workspace, `PreparedRasterProducer::try_admit`). A plan or photograph over the budget is scaled to
/// fit it — the plane it textures never covers more than a viewport, so this is a texture LOD and not
/// a dropped feature, and it is what keeps a 2275×2560 scan (23 296 000 bytes) from being refused at
/// raster admission, which the frame reports as a BUILD FAULT that quarantines the whole surface
/// (ticket 26/09/17/WGPU-RENDERER-REACT-PARITY, `📓️w3a-asset-decoder-boot-fault.md`).
pub const WORLD_REFERENCE_TEXTURE_BYTES: usize = semio_framework_ui_raster_owner::SCENE_RASTER_ITEM_BYTES;

/// 🖼️ `image` scaled to at most [`WORLD_REFERENCE_TEXTURE_BYTES`] of RGBA, aspect preserved.
pub fn bounded_reference_image(image: image::DynamicImage) -> image::DynamicImage {
    let pixels = u64::from(image.width()).saturating_mul(u64::from(image.height()));
    let budget = (WORLD_REFERENCE_TEXTURE_BYTES / 4) as u64;
    if pixels <= budget || pixels == 0 {
        return image;
    }
    let scale = ((budget as f64) / (pixels as f64)).sqrt();
    let width = ((f64::from(image.width()) * scale) as u32).max(1);
    let height = ((f64::from(image.height()) * scale) as u32).max(1);
    image.resize(width, height, image::imageops::FilterType::Triangle)
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DecodedReferenceImage {
    pub width: u32,
    pub height: u32,
    pub source_digest: [u64; 2],
    pub pixels: Vec<u8>,
}

pub fn decode_reference_image_bytes(bytes: &[u8]) -> Option<DecodedReferenceImage> {
    use image::ImageDecoder as _;

    let reader = image::ImageReader::new(std::io::Cursor::new(bytes)).with_guessed_format().ok()?;
    let mut decoder = reader.into_decoder().ok()?;
    let (source_width, source_height) = decoder.dimensions();
    if u64::from(source_width).checked_mul(u64::from(source_height))? > WORLD_REFERENCE_SOURCE_PIXELS {
        return None;
    }
    let orientation = decoder.orientation().unwrap_or(image::metadata::Orientation::NoTransforms);
    let mut decoded = image::DynamicImage::from_decoder(decoder).ok()?;
    decoded.apply_orientation(orientation);
    let rgba = bounded_reference_image(decoded).to_rgba8();
    let digest = semio_framework_hash::Sha256::digest(bytes);
    let source_digest = [u64::from_be_bytes(digest[..8].try_into().ok()?), u64::from_be_bytes(digest[8..16].try_into().ok()?)];
    Some(DecodedReferenceImage { width: rgba.width(), height: rgba.height(), source_digest, pixels: rgba.into_raw() })
}

/// 🔘️ The shell-owned placeholder plane the gumball's three drag quads are drawn with. Named here
/// beside the other shell placeholders because it must appear in THREE places that used to disagree:
/// the placeholder build, `sync_mesh_pool`'s `PINNED` list (it belongs to no document draw, so the
/// pool would otherwise evict it as stale every frame) and the GPU `ensure_mesh` the draw needs.
pub const GUMBALL_PLANE_MESH: &str = "gumball-plane";

pub fn publish_world3d_mesh_lease(interaction_state: &mut WorldInteractionState, scene_state: &mut WorldSceneState, id: String, mesh: Mesh3dLease) -> Result<(), WorldDynamicRejected<Mesh3dLease>> {
    let version = mesh.generation().rotate_left(17) ^ mesh.revision();
    if scene_state.dynamic_mesh_close.is_some() || scene_state.dynamic_blocked_mesh.is_some() {
        return Err(WorldDynamicRejected { fault: WorldDynamicFault::Closing, id, value: mesh });
    }
    let mesh_plan = scene_state.meshes.plan_insert(&id).map_err(|fault| WorldDynamicRejected { fault, id: id.clone(), value: mesh })?;
    let version_plan = scene_state.mesh_versions.plan_insert(&id).map_err(|fault| WorldDynamicRejected { fault, id: id.clone(), value: mesh })?;
    let interaction_plan = interaction_state.interaction_meshes.plan_admit(&id, version, mesh).map_err(|fault| WorldDynamicRejected { fault, id: id.clone(), value: mesh })?;
    let (_, previous_mesh) = scene_state.meshes.commit_insert(mesh_plan, id.clone(), mesh).map_err(|rejected| WorldDynamicRejected { fault: rejected.fault, id: rejected.id, value: mesh })?;
    interaction_state.interaction_meshes.commit_admit(interaction_plan).expect("world mesh transaction revalidates every fixed interaction slot before its first mutation");
    let (_, previous_version) = scene_state.mesh_versions.commit_insert(version_plan, id.clone(), version).expect("world mesh transaction revalidates every fixed version slot before its first mutation");
    if let Some(mut previous_version) = previous_version {
        previous_version.id.clear();
    }
    if let Some(previous_mesh) = previous_mesh {
        scene_state.dynamic_mesh_close = Some(previous_mesh);
    }
    drop(scene_state.mesh_appearances.remove(&id));
    scene_state.geometry_generation = scene_state.geometry_generation.wrapping_add(1).max(1);
    Ok(())
}

pub fn retire_world_mesh(scene_state: &mut WorldSceneState, id: &str) -> bool {
    if scene_state.dynamic_mesh_close.is_some() || scene_state.dynamic_blocked_mesh.is_some() {
        return false;
    }
    let Some(entry) = scene_state.meshes.remove(id) else {
        return true;
    };
    scene_state.dynamic_mesh_close = Some(entry);
    drop(scene_state.mesh_versions.remove(id));
    drop(scene_state.mesh_appearances.remove(id));
    scene_state.geometry_generation = scene_state.geometry_generation.wrapping_add(1).max(1);
    true
}

pub fn retire_world_pixels(scene_state: &mut WorldSceneState, id: &str, paint: bool) -> bool {
    let registry = if paint { &mut scene_state.mesh_paint_textures } else { &mut scene_state.reference_pixels };
    drop(registry.remove(id));
    true
}

pub fn parse_color(value: &str) -> [f32; 4] {
    let trimmed = value.trim();
    if let Some(hex) = trimmed.strip_prefix('#') {
        let expanded = match hex.len() {
            3 => hex.chars().map(|ch| format!("{ch}{ch}")).collect::<String>(),
            _ => hex.to_string(),
        };
        if expanded.len() >= 6 {
            let r = u8::from_str_radix(&expanded[0..2], 16).unwrap_or(148) as f32 / 255.0;
            let g = u8::from_str_radix(&expanded[2..4], 16).unwrap_or(163) as f32 / 255.0;
            let b = u8::from_str_radix(&expanded[4..6], 16).unwrap_or(184) as f32 / 255.0;
            return [srgb_to_linear(r), srgb_to_linear(g), srgb_to_linear(b), 1.0];
        }
    }
    [srgb_to_linear(0.58), srgb_to_linear(0.64), srgb_to_linear(0.72), 1.0]
}

/// 🎨️ One sRGB channel in the LINEAR working space every world pipeline shades in.
///
/// 🩸️ `parse_color` used to hand raw sRGB bytes straight to the GPU. The world pass renders into an
/// sRGB VIEW (`🧊️gpu/🦀️.rs`'s `color_target_format`), so those bytes were encoded a second time on
/// write and every wire colour came out washed out against React, whose `new Color(hex)` linearizes
/// through `ColorManagement` exactly like this. `Theme` was always linear; only this reader was not.
pub fn srgb_to_linear(channel: f32) -> f32 {
    if channel <= 0.04045 {
        channel / 12.92
    } else {
        ((channel + 0.055) / 1.055).powf(2.4)
    }
}

//#region Chunking
pub fn chunk_key_indices(position: [f64; 3], chunk_size: f64) -> (i64, i64, i64) {
    ((position[0] / chunk_size).floor() as i64, (position[1] / chunk_size).floor() as i64, (position[2] / chunk_size).floor() as i64)
}

pub fn chunk_center(key: (i64, i64, i64), chunk_size: f64) -> Vec3 {
    let size = chunk_size as f32;
    Vec3::new((key.0 as f32 + 0.5) * size, (key.1 as f32 + 0.5) * size, (key.2 as f32 + 0.5) * size)
}

fn chunk_bounds_radius(chunk_size: f64) -> f64 {
    chunk_size * 0.866
}

pub fn chunk_distance_visible(cam_pos: Vec3, chunk_center: Vec3, chunk_size: f64, max_dist: f64, was_visible: bool) -> bool {
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

pub fn update_visible_chunks(scene_state: &mut WorldSceneState, cam_pos: Vec3) {
    let Some(chunking) = scene_state.chunking.clone() else {
        return;
    };
    let chunk_size = chunking.chunk_size;
    let max_distance = chunking.max_distance;
    let mut chunk_keys = HashSet::new();
    for position in scene_state.instance_positions.values() {
        chunk_keys.insert(chunk_key_indices(*position, chunk_size));
    }
    let previous = scene_state.visible_chunks.clone();
    let mut next_visible = HashSet::new();
    for key in chunk_keys.iter().chain(previous.iter()) {
        let center = chunk_center(*key, chunk_size);
        let was = previous.contains(key);
        if chunk_distance_visible(cam_pos, center, chunk_size, max_distance, was) {
            next_visible.insert(*key);
        }
    }
    scene_state.visible_chunks = next_visible;
}

pub fn instance_chunk_visible(scene_state: &WorldSceneState, position: [f64; 3]) -> bool {
    let Some(chunking) = &scene_state.chunking else {
        return true;
    };
    let key = chunk_key_indices(position, chunking.chunk_size);
    scene_state.visible_chunks.contains(&key)
}
//#endregion Chunking

//#region LodGrid
pub const WORLD_LOD_EPSILON: f64 = 0.01;

/// 📐️ The most divisions one LOD band may submit as real line geometry. A finer band is DROPPED
/// rather than drawn at a distorted spacing — see [`append_lod_grid_lines`].
pub const WORLD_GRID_MAX_DIVISIONS: i32 = 512;

pub fn scene_lod(session_state: &WorldSessionState, camera_state: &WorldCameraState, scene_state: &WorldSceneState) -> f64 {
    let camera = camera_state.orbit.to_camera();
    let distance = f64::from(lod_orbit_distance_for_camera(&camera, camera.position.sub(camera.target).length(), session_state.bounds.h));
    let auto_lod = lod_from_camera_distance(distance, scene_state.lod.distance_reference);
    if scene_state.lod.automatic || scene_state.lod.depth_variable {
        auto_lod
    } else {
        scene_state.lod.manual
    }
}

#[cfg(any(test, feature = "testkit"))]
pub fn resolve_physical_mesh_id(scene_state: &WorldSceneState, logical_id: &str, desired_lod: f64) -> String {
    if let Some(lods) = scene_state.mesh_lod_catalog.get(logical_id) {
        let entries: Vec<(f64, &str)> = lods.iter().map(|entry| (entry.lod, entry.url.as_str())).collect();
        let fallback = scene_state.mesh_url_fallback.get(logical_id).map(String::as_str);
        if let Some(url) = pick_closest_mesh_url(&entries, desired_lod, fallback) {
            return mesh_id_from_url(url);
        }
    }
    if let Some(url) = scene_state.mesh_url_fallback.get(logical_id) {
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
pub fn append_lod_grid_lines(line_vertices: &mut Vec<LineVertex3d>, lod: f64, grid_factor: f64, anchor: Vec3, camera: &Camera3d, viewport: Rect, base_color: [f32; 4]) {
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
pub fn world_instance_style(selection_state: &WorldSelectionState, instance: &Instance3d) -> MeshStyleState {
    MeshStyleState {
        disabled: selection_state.disabled_instance_ids.contains(&instance.id),
        provisional: selection_state.provisional_instance_ids.contains(&instance.id),
        celebrating: selection_state.celebrating_instance_ids.contains(&instance.id),
        selected: instance.selected,
        highlighted: selection_state.highlighted_instance_ids.contains(&instance.id),
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
pub fn mesh_style_paint(theme: &semio_framework_ui_theme::Theme, kind: MeshStyleKind) -> MeshStylePaint {
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
pub fn world3d_style_paint(theme: &semio_framework_ui_theme::Theme, style: MeshStyleState, has_vertex_colors: bool, mut instance: Instance3d) -> Instance3d {
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
pub fn retain_ensured_world_draws(gpu: &World3dBuildContext, draws: &mut Vec<SceneDraw3d>) {
    draws.retain(|draw| gpu.has_mesh_request(&draw.mesh_key, draw.mesh_version));
}

pub fn retain_ensured_world_material_draws(gpu: &World3dBuildContext, draws: &mut Vec<SceneMaterialDraw3d>) {
    draws.retain(|draw| gpu.has_mesh_request(&draw.mesh_key, draw.mesh_version));
}

#[cfg(not(all(target_arch = "wasm32", target_env = "p2")))]
pub struct WorldTranslucentMaterialItem {
    pub source: usize,
    pub depth: f32,
    pub ordinal: usize,
    pub instance: Instance3d,
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
pub fn sort_world3d_translucent_material_draws(
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
pub fn sync_mesh_pool(scene_state: &mut WorldSceneState, assets_state: &mut WorldAssetsState, needed_mesh_keys: &HashSet<String>, gpu: &mut World3dBuildContext) {
    const PINNED: &[&str] = &["vortex-marker", "cylinder", "cone", "reference-plane", "vertex-marker", GUMBALL_PLANE_MESH];
    for key in needed_mesh_keys {
        if !scene_state.mesh_pool.contains(key) {
            scene_state.mesh_pool.acquire(key.clone());
        }
    }
    let stale: Vec<String> = scene_state.mesh_pool.keys().filter(|key| !needed_mesh_keys.contains(*key) && !PINNED.contains(&key.as_str())).cloned().collect();
    for key in stale {
        if scene_state.mesh_pool.release(&key) {
            if !retire_world_mesh(scene_state, &key) || !retire_world_pixels(scene_state, &key, true) {
                return;
            }
            if let Some(url) = scene_state.mesh_source_urls.remove(&key) {
                assets_state.pending_glb_urls.remove(&url);
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
fn reserve_world3d_mesh_fetch(session_state: &mut WorldSessionState, assets_state: &mut WorldAssetsState, url: &str) {
    match reserve_world3d_asset_request(session_state, assets_state, WorldAssetRequestKind::Glb, url) {
        Ok(_) => {
            assets_state.pending_glb_urls.insert(url.to_owned());
        }
        Err(WorldAssetFault::ItemCapacity | WorldAssetFault::ByteCapacity | WorldAssetFault::Closing | WorldAssetFault::Stale) => {}
        Err(_) => mark_world3d_asset_miss(assets_state, url),
    }
}

/// 🥽️📡️ The per-frame offer: every draw the scene published whose mesh the wire named by url and
/// that has not landed yet. Re-derived from the live draw list every frame — a scene that stops
/// naming a mesh stops asking for it — with [`World3dState::pending_glb_urls`] and
/// [`World3dState::asset_url_misses`] as the two ledgers that keep the offer idempotent.
pub fn offer_missing_mesh_fetches(session_state: &mut WorldSessionState, scene_state: &mut WorldSceneState, assets_state: &mut WorldAssetsState) {
    let missing_mesh_urls: HashSet<String> = scene_state
        .draws
        .iter()
        .filter(|draw| !scene_state.meshes.contains_key(&draw.mesh_key))
        .filter_map(|draw| scene_state.mesh_source_urls.get(&draw.mesh_key).cloned())
        .filter(|url| !assets_state.asset_url_misses.contains(url) && !assets_state.pending_glb_urls.contains(url))
        .collect();
    for url in missing_mesh_urls {
        reserve_world3d_mesh_fetch(session_state, assets_state, &url);
    }
}

#[cfg(any(test, feature = "testkit"))]
fn queue_lod_mesh_fetch(session_state: &mut WorldSessionState, scene_state: &mut WorldSceneState, assets_state: &mut WorldAssetsState, logical_id: &str, scene_lod: f64) {
    let url = {
        let entries: Vec<(f64, &str)> = scene_state.mesh_lod_catalog.get(logical_id).map(|lods| lods.iter().map(|entry| (entry.lod, entry.url.as_str())).collect()).unwrap_or_default();
        let fallback = scene_state.mesh_url_fallback.get(logical_id).map(String::as_str);
        pick_closest_mesh_url(&entries, scene_lod, fallback).or(fallback).map(str::to_owned)
    };
    if let Some(url) = url {
        reserve_world3d_mesh_fetch(session_state, assets_state, &url);
    }
}

//#endregion LodGrid
