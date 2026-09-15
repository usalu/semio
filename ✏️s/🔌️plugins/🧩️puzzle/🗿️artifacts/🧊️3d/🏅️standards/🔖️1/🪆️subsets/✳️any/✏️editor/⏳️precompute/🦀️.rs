//! ⏳️ Puzzle 3d play app — the precompute session: the scene the host syncs in, the registered
//! collision meshes and the background brush-candidate lane, plus `dispatch`, which drives
//! `Puzzle3dEngineCommand`/`Puzzle3dEngineOutcome` (schema types, `crate::standards::v1::subsets::any::schema`)
//! through the session. The rules the lane consults live in `🖌️brush/🦀️.rs`, the geometry in
//! `📐️geometry/🦀️.rs`, the fill planner and its tool run jobs in `🪣️fill/🦀️.rs` (owned by the framework tool
//! run ledger, never by this session). Rehomed from the former `⚙️engine/⏳️session` (ticket
//! 26/08/12/ENGINELESS-ARTIFACTS-AND-APP-STATE-MACHINES): a puzzle-3d artifact is a schema plus an io
//! system, never an engine — this interactive brush session is the app's own state machine over
//! that schema, not artifact behaviour.

//#region 🔖️Reexports
pub use crate::editor::puzzle3d::precompute::brush::apply_brush_placement_to_fixture;
//#endregion 🔖️Reexports

//#region 🔖️Constants
/// 🥽️ Collision mesh identities one session holds.
pub(crate) const COLLISION_MESH_MAX_MESHES: usize = 64;
/// 🥽️ Values (positions or indices) one collision mesh may carry.
pub(crate) const COLLISION_MESH_MAX_VALUES: usize = 196_608;
/// 🥽️ Bytes of one collision mesh identity.
pub(crate) const COLLISION_MESH_MAX_URL_BYTES: usize = 4 * 1024;
//#endregion 🔖️Constants

use crate::editor::puzzle3d::precompute::brush::{
    brush_candidate_suggestion_weight, brush_compatible_candidates, brush_preview_from_candidate, brush_target_vortex_allows_suggestion, resolve_placed_object_mesh_url, vortex_world_from_object, AttractionVortexContext, TargetVortexWorld,
};
use crate::editor::puzzle3d::precompute::fill::PlacedCollisionEntry;
use crate::editor::puzzle3d::precompute::geometry::{
    pose_isometry, world_bounds, CollisionAabb, CollisionBody, CollisionIndexMutation, CollisionIndexOwner, CollisionIndexRemoval, CollisionMutationStep, CollisionPenetrationState, CollisionQueryStep, CollisionSpatialIndex, CollisionStepContext,
    CollisionStepResult,
};
use crate::standards::v1::subsets::any::schema::{
    puzzle3d_vortex_full_id, BrushCollisionFreeResult, BrushCompatibleCandidate, BrushPlacePayload, BrushPreviewState, Fixture, FixtureObject, KindCatalogBundle, Puzzle3dEngineCommand, Puzzle3dEngineOutcome,
    SceneConfig,
};
use crate::Puzzle3dError;
use semio_framework_job::default_now_us;
use std::collections::{HashMap, VecDeque};
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::{Arc, Mutex, OnceLock};

/// 🥽️ The source buffers of one installed collision mesh, kept so the session census can bound them.
#[derive(Clone, Debug)]
pub(crate) struct CollisionMeshSource {
    url: String,
    positions: Vec<f32>,
    indices: Vec<u32>,
}

//#region 🔖️Clock
/// ⏱️ Uses the same checked real clock authority as retained jobs on every target.
fn puzzle3d_deadline(duration_us: u64) -> Option<u64> {
    default_now_us()?.checked_add(duration_us)
}

/// 🪫️ Admission deadline for one precompute turn, including its first task. It has to fit INSIDE
/// the retained step it is spent from, not equal it — and with room for the overshoot its own last
/// task unit costs. Handed the whole 2 000 µs this artifact budgets per interactive step, one
/// `refresh_brush_candidates` overshot it to a measured 2.04 ms on the 180-object Nakagin document:
/// the entire step's budget, spent on one of its halves, and still over. Halved it still reached
/// 1.54 ms, because the overshoot is one whole narrow-phase candidate (~0.5 ms there). A QUARTER
/// leaves the dispatch turn three quarters of its own step for the rest of its work, which is the
/// margin the measured laws in `✏️editor/🧪️tests/🔬️unit/🦀️.rs` need to hold. The lane redrives whatever
/// this deadline leaves unfinished, so a smaller budget only ever costs turns, never results.
/// Ticket 26/09/02/PUZZLE-3D-END-TO-END W-P3.
const PUZZLE3D_PRECOMPUTE_STEP_BUDGET_US: u64 = 500;
//#endregion 🔖️Clock

//#region 🥽️SharedBrushMeshes
/// 🥽️ Content-addressed brush-mesh decode kernel — a pure `(url, positions, indices)` bytes-in,
/// validated-geometry-bytes-out compute, so one mesh identity decodes exactly once per process and
/// every document instance reads the identical derived page instead of re-uploading it. This is the
/// kernel the WIT `engine-derive`/`engine-read` host route registers once it is threaded through
/// exchange (`🧰️framework/🛍️products/💻️os/🔨️modules/⚙️engine/🦀️.rs`), with no plugin-side change.
struct Puzzle3dMeshDecodeEngine;

/// ⚖️ Byte budget of the derived-geometry LRU: `COLLISION_MESH_MAX_MESHES` document-scale meshes at the
/// `COLLISION_MESH_MAX_VALUES` ceiling, four bytes per value, halved because positions and indices
/// never both saturate.
const BRUSH_MESH_CACHE_BYTES: usize = COLLISION_MESH_MAX_MESHES * COLLISION_MESH_MAX_VALUES * 4;

impl store::Engine for Puzzle3dMeshDecodeEngine {
    const ENGINE_ID: &'static str = "puzzle3d.mesh-decode";

    fn compute(&self, input: &[u8]) -> Result<Vec<u8>, store::EngineFault> {
        let geometry = brush_mesh_request_geometry(input).ok_or_else(|| store::EngineFault::InvalidInput("puzzle3d brush mesh request".into()))?;
        let (positions, indices) = decode_brush_mesh_geometry(geometry).ok_or_else(|| store::EngineFault::InvalidInput("puzzle3d brush mesh geometry".into()))?;
        brush_mesh_geometry_is_admissible(&positions, &indices).then(|| geometry.to_vec()).ok_or_else(|| store::EngineFault::Compute("puzzle3d brush mesh geometry is not a closed indexed triangle page".into()))
    }
}

/// 🔗️ Little-endian request wire: `url_len | url | position_count | index_count | positions | indices`.
/// The url is part of the cache key on purpose — two urls carrying byte-identical geometry stay two
/// distinct mesh identities, exactly as the collision engine's own `meshes` map keys them.
fn encode_brush_mesh_request(url: &str, positions: &[f32], indices: &[u32]) -> Option<Vec<u8>> {
    let mut bytes = u32::try_from(url.len()).ok()?.to_le_bytes().to_vec();
    bytes.extend_from_slice(url.as_bytes());
    bytes.extend_from_slice(&u32::try_from(positions.len()).ok()?.to_le_bytes());
    bytes.extend_from_slice(&u32::try_from(indices.len()).ok()?.to_le_bytes());
    bytes.extend(positions.iter().flat_map(|value| value.to_le_bytes()));
    bytes.extend(indices.iter().flat_map(|value| value.to_le_bytes()));
    Some(bytes)
}

fn brush_mesh_request_geometry(input: &[u8]) -> Option<&[u8]> {
    let url_len = usize::try_from(u32::from_le_bytes(input.get(..4)?.try_into().ok()?)).ok()?;
    input.get(url_len.checked_add(4)?..)
}

fn decode_brush_mesh_geometry(geometry: &[u8]) -> Option<(Vec<f32>, Vec<u32>)> {
    let position_bytes = usize::try_from(u32::from_le_bytes(geometry.get(..4)?.try_into().ok()?)).ok()?.checked_mul(4)?;
    let index_bytes = usize::try_from(u32::from_le_bytes(geometry.get(4..8)?.try_into().ok()?)).ok()?.checked_mul(4)?;
    let indices_at = position_bytes.checked_add(8)?;
    let positions = geometry.get(8..indices_at)?;
    let indices = geometry.get(indices_at..indices_at.checked_add(index_bytes)?)?;
    Some((positions.as_chunks::<4>().0.iter().map(|bytes| f32::from_le_bytes(*bytes)).collect(), indices.as_chunks::<4>().0.iter().map(|bytes| u32::from_le_bytes(*bytes)).collect()))
}

fn brush_mesh_geometry_is_admissible(positions: &[f32], indices: &[u32]) -> bool {
    let vertices = u32::try_from(positions.len() / 3).unwrap_or(u32::MAX);
    positions.len() >= 9
        && indices.len() >= 3
        && positions.len().is_multiple_of(3)
        && indices.len().is_multiple_of(3)
        && positions.len() <= COLLISION_MESH_MAX_VALUES
        && indices.len() <= COLLISION_MESH_MAX_VALUES
        && positions.iter().all(|value| value.is_finite())
        && indices.iter().all(|index| *index < vertices)
}

/// 🗄️ Process-wide derived-mesh authority: the framework's own content-addressed LRU plus the
/// url→handle index that lets a later command — or an entirely different open document — reach a mesh
/// the client uploaded once, so the wire only ever needs to carry the mesh id again.
struct Puzzle3dBrushMeshStore {
    engines: store::EngineCache,
    handles: HashMap<String, store::EngineHandle>,
    digests: HashMap<String, store::EngineHandle>,
}

fn brush_mesh_store() -> &'static Mutex<Puzzle3dBrushMeshStore> {
    static STORE: OnceLock<Mutex<Puzzle3dBrushMeshStore>> = OnceLock::new();
    STORE.get_or_init(|| {
        let mut engines = store::EngineCache::new(BRUSH_MESH_CACHE_BYTES);
        engines.register(Puzzle3dMeshDecodeEngine);
        Mutex::new(Puzzle3dBrushMeshStore { engines, handles: HashMap::new(), digests: HashMap::new() })
    })
}

/// 🔢️ Mesh identities THIS guest instantiation derived into the process-wide store, counted since the
/// module was instantiated. Lives beside [`brush_mesh_store`] rather than inside it so reading it can
/// never fail or block: a lock-contended read that answered `0` would be indistinguishable from a fresh
/// instantiation, which is precisely the distinction this number exists to carry.
static BRUSH_MESH_INSTALLS: AtomicU64 = AtomicU64::new(0);

/// 🔢️ Monotone-within-one-instantiation residency counter, zero in a fresh guest — see
/// [`BRUSH_MESH_INSTALLS`]. A client that watched the number climb learns its "already uploaded"
/// bookkeeping is void the moment it reads a lower one, without asking one question per mesh id and
/// without the guest needing a clock, a random source or an instance id it has no way to mint. The
/// store is deliberately NOT part of any checkpoint, so a restored actor reports zero — exactly the
/// fact the client must act on.
///
/// Published on the world-3d surface as `interactionJson.meshResidency`
/// (`✏️editor/🎭️modes/✏️edit/🪟️windows/🧊️main/🦀️.rs`), read by `Puzzle3dBrushMeshRegistry`
/// (`🧰️framework/…/🛠️ShellHelpers/🟦️.tsx`).
pub fn shared_brush_mesh_installs() -> u64 {
    BRUSH_MESH_INSTALLS.load(Ordering::Relaxed)
}

/// 🧮️ Derives one uploaded mesh into the process-wide store and hands back the validated geometry —
/// a second document registering the same identity hits the cache instead of decoding again.
pub fn derive_brush_mesh(url: &str, positions: &[f32], indices: &[u32]) -> Option<(Vec<f32>, Vec<u32>)> {
    let request = encode_brush_mesh_request(url, positions, indices)?;
    let digest = brush_mesh_digest(positions, indices);
    let mut store = brush_mesh_store().try_lock().ok()?;
    let handle = store.engines.derive(<Puzzle3dMeshDecodeEngine as store::Engine>::ENGINE_ID, &request).ok()?;
    let geometry = store.engines.read(&handle).ok()?;
    store.handles.insert(url.to_string(), handle.clone());
    store.digests.insert(digest, handle);
    BRUSH_MESH_INSTALLS.fetch_add(1, Ordering::Relaxed);
    decode_brush_mesh_geometry(&geometry)
}

/// 📖 Geometry this process already derived for one mesh id, or `None` once the LRU evicted it — a
/// miss is a cache miss, never a correctness change: the client is asked for the bytes again.
pub(crate) fn shared_brush_mesh(url: &str) -> Option<(Vec<f32>, Vec<u32>)> {
    let store = brush_mesh_store().try_lock().ok()?;
    let geometry = store.engines.read(store.handles.get(url)?).ok()?;
    decode_brush_mesh_geometry(&geometry)
}

/// 🪢️ Geometry this process derived under ANY mesh id, keyed by the client's own digest, aliased onto
/// `url` on the way out. A scene places several object kinds whose `meshUrl`s are distinct ids over
/// byte-identical geometry — every `dist/mesh/*.glb` in this repo is the same 771 728-byte capsule —
/// and keying the transfer by id alone made the client page the SAME 294 912 bytes once per id, 72
/// commands each. The collision engine still keys its own `meshes` map by url (two ids stay two
/// identities, `encode_brush_mesh_request`'s note), so only the TRANSFER is content-addressed: the
/// second id adopts the first id's derived page and the wire carries nothing but the announcement.
pub(crate) fn adopt_brush_mesh_by_digest(url: &str, digest: &str) -> Option<(Vec<f32>, Vec<u32>)> {
    let mut store = brush_mesh_store().try_lock().ok()?;
    let handle = store.digests.get(digest)?.clone();
    let geometry = store.engines.read(&handle).ok()?;
    store.handles.insert(url.to_string(), handle);
    BRUSH_MESH_INSTALLS.fetch_add(1, Ordering::Relaxed);
    decode_brush_mesh_geometry(&geometry)
}
//#endregion 🥽️SharedBrushMeshes

//#region 🧩️PagedBrushMeshUploads
/// 📏️ Values — `f32` positions and `u32` indices counted together — one `registerBrushMesh` page
/// carries. 1 024 values are 4 096 little-endian payload bytes, 5 464 base64 characters; the shared
/// retained wire admits [`PUZZLE_COMMAND_RAW_BYTES`] (8 192) bytes per command, which leaves the
/// envelope (`surfaceId`, `url`, `page`, `pageCount`, `digest`, two key names) 2 700 bytes of head
/// room. The host's own pager (`🌐️World3dHost/🟦️.tsx`) shrinks a page below this only when a long
/// mesh id would otherwise push the envelope over the same limit.
///
/// [`PUZZLE_COMMAND_RAW_BYTES`]: crate::retained_command::PUZZLE_COMMAND_RAW_BYTES
pub const PUZZLE3D_MESH_PAGE_VALUES: usize = 1_024;

/// 🔤️ Base64 characters one page's payload string may hold — [`PUZZLE3D_MESH_PAGE_VALUES`] values as
/// four bytes each, three bytes per four-character group, the last group padded. 1 024 values are
/// 4 096 bytes and 5 464 characters, which the shared retained command's 8 192 raw bytes carry with
/// the JSON envelope still inside the limit.
pub const PUZZLE3D_MESH_PAGE_BASE64_CHARS: usize = (PUZZLE3D_MESH_PAGE_VALUES * 4).div_ceil(3) * 4;

/// 📦️ Partial uploads the process stages at once. A sequence is opened and closed by one mesh
/// identity's own page run, so four covers every world window a desktop layout opens at once, and a
/// fifth identity retires the least recently advanced slot instead of growing the staging area.
const PUZZLE3D_MESH_UPLOAD_SLOTS: usize = 4;

/// 🧮️ Longest page run one mesh identity may claim: both arrays at the engine's own
/// [`COLLISION_MESH_MAX_VALUES`] ceiling, paged at [`PUZZLE3D_MESH_PAGE_VALUES`]. The largest mesh
/// this repo ships — `🧊️placeholder.glb`, 25 344 positions plus 48 384 indices — is 72 pages.
const PUZZLE3D_MESH_UPLOAD_MAX_PAGES: u32 = (COLLISION_MESH_MAX_VALUES * 2).div_ceil(PUZZLE3D_MESH_PAGE_VALUES) as u32;

/// 🚫️ Why one `registerBrushMesh` page was refused. Every arm is a wire fault the client can act on,
/// never a silent drop: the command arm turns it into a shell notification.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Puzzle3dMeshUploadFault {
    /// 🏷️ `page`/`pageCount` are absent, zero, out of order or beyond [`PUZZLE3D_MESH_UPLOAD_MAX_PAGES`].
    Envelope,
    /// 🔤️ A payload string is not base64, or carries a value count above [`PUZZLE3D_MESH_PAGE_VALUES`].
    Payload,
    /// 🕳️ The page does not continue the staged run for this identity. The broken run is dropped, so the
    /// client re-opens at page 0 instead of resuming into bytes nobody can account for.
    Gap,
    /// 📦️ The run would exceed [`COLLISION_MESH_MAX_VALUES`] in one of its two arrays.
    Capacity,
    /// #️⃣ The closed run does not hash to the digest the client declared.
    Digest,
    /// 📐️ The closed run is not a finite, closed, in-range indexed triangle page.
    Geometry,
}

impl Puzzle3dMeshUploadFault {
    /// 🏷️ Stable wire code, mirrored by the `registerBrushMesh` notification and the unit laws.
    pub fn code(self) -> &'static str {
        match self {
            Self::Envelope => "puzzle3d-register-mesh-envelope",
            Self::Payload => "puzzle3d-register-mesh-payload",
            Self::Gap => "puzzle3d-register-mesh-gap",
            Self::Capacity => "puzzle3d-register-mesh-capacity",
            Self::Digest => "puzzle3d-register-mesh-digest",
            Self::Geometry => "puzzle3d-register-mesh-geometry",
        }
    }
}

/// 🧱️ What one accepted page did to its identity's run.
#[derive(Clone, Debug, PartialEq)]
pub enum Puzzle3dMeshUploadStep {
    /// 🧱️ The page landed and the run is still open.
    Staged { next_page: u32, page_count: u32 },
    /// ✅️ The page closed the run; the geometry is the client's mesh, digest-verified.
    Complete(Vec<f32>, Vec<u32>),
}

/// 🧵️ One mesh identity's open page run.
struct Puzzle3dMeshUploadSlot {
    url: String,
    digest: String,
    page_count: u32,
    next_page: u32,
    positions: Vec<f32>,
    indices: Vec<u32>,
    touched: u64,
}

/// 🗄️ Process-wide staging area for page runs that have not closed yet. Fixed capacity in slots and,
/// per slot, in values — an abandoned run costs the process one slot until the next session retirement
/// sweeps it, never unbounded memory.
struct Puzzle3dBrushMeshUploads {
    slots: Vec<Puzzle3dMeshUploadSlot>,
    sequence: u64,
    swept: u64,
}

fn brush_mesh_uploads() -> &'static Mutex<Puzzle3dBrushMeshUploads> {
    static UPLOADS: OnceLock<Mutex<Puzzle3dBrushMeshUploads>> = OnceLock::new();
    UPLOADS.get_or_init(|| Mutex::new(Puzzle3dBrushMeshUploads { slots: Vec::new(), sequence: 0, swept: 0 }))
}

/// #️⃣ The identity a client declares for a mesh: unkeyed BLAKE3 over the positions' little-endian
/// bytes followed by the indices' — byte-identical to the renderer's own `puzzle3dBrushMeshDigest`
/// (`🌐️World3dHost/🟦️.tsx`), which hashes the same two typed arrays through the framework's
/// first-party `blake3Hex`.
pub fn brush_mesh_digest(positions: &[f32], indices: &[u32]) -> String {
    let mut bytes = Vec::with_capacity(positions.len().saturating_add(indices.len()).saturating_mul(4));
    bytes.extend(positions.iter().flat_map(|value| value.to_le_bytes()));
    bytes.extend(indices.iter().flat_map(|value| value.to_le_bytes()));
    semio_framework_hash::hash_bytes(&bytes)
}

/// 🔤️ One page's payload string as the four-byte values it carries — base64 of little-endian `f32`
/// positions or `u32` indices. Refuses a string longer than one page may carry, a byte run that is not
/// whole values, and a value count above the caller's remaining page budget.
pub fn decode_brush_mesh_page_values(encoded: &str, budget: usize) -> Option<Vec<[u8; 4]>> {
    if encoded.len() > PUZZLE3D_MESH_PAGE_BASE64_CHARS || budget > PUZZLE3D_MESH_PAGE_VALUES {
        return None;
    }
    let bytes = semio_framework_io_base64::base64_standard_decode(encoded).ok()?;
    (bytes.len().is_multiple_of(4) && bytes.len() / 4 <= budget).then(|| bytes.as_chunks::<4>().0.to_vec())
}

/// 🧩️ Admits one page of a mesh identity's upload run into the staging area, and hands back the whole
/// geometry the moment the run closes. A run is keyed by `(url, digest)`: a client that restarts an
/// upload with different bytes opens a different run instead of corrupting the staged one, and the
/// closed run is refused unless it hashes to the digest that keyed it.
pub fn stage_brush_mesh_page(url: &str, digest: &str, page: u32, page_count: u32, positions: &[f32], indices: &[u32]) -> Result<Puzzle3dMeshUploadStep, Puzzle3dMeshUploadFault> {
    if digest.is_empty() || page_count == 0 || page_count > PUZZLE3D_MESH_UPLOAD_MAX_PAGES || page >= page_count {
        return Err(Puzzle3dMeshUploadFault::Envelope);
    }
    if positions.len().saturating_add(indices.len()) > PUZZLE3D_MESH_PAGE_VALUES {
        return Err(Puzzle3dMeshUploadFault::Payload);
    }
    let mut uploads = brush_mesh_uploads().lock().unwrap_or_else(std::sync::PoisonError::into_inner);
    uploads.sequence = uploads.sequence.saturating_add(1);
    let sequence = uploads.sequence;
    let held = uploads.slots.iter().position(|slot| slot.url == url && slot.digest == digest);
    let index = match held {
        Some(index) if uploads.slots[index].next_page == page && uploads.slots[index].page_count == page_count => index,
        // 🔁️ A page this run already admitted is a RETRANSMISSION, not a gap: the bytes are staged, the
        // cursor is ahead of it, and nothing is missing. Dropping the run here made every duplicate cost
        // the client the whole 72-page upload again, so a lane that retries one page paid for all of
        // them. Acknowledged at the cursor it actually stands on, with no second append.
        Some(index) if page != 0 && page < uploads.slots[index].next_page && uploads.slots[index].page_count == page_count => {
            let slot = &mut uploads.slots[index];
            slot.touched = sequence;
            return Ok(Puzzle3dMeshUploadStep::Staged { next_page: slot.next_page, page_count: slot.page_count });
        }
        Some(index) if page != 0 => {
            uploads.slots.swap_remove(index);
            return Err(Puzzle3dMeshUploadFault::Gap);
        }
        Some(index) => {
            uploads.slots[index] = Puzzle3dMeshUploadSlot { url: url.to_string(), digest: digest.to_string(), page_count, next_page: 0, positions: Vec::new(), indices: Vec::new(), touched: sequence };
            index
        }
        None if page != 0 => return Err(Puzzle3dMeshUploadFault::Gap),
        None => {
            if uploads.slots.len() >= PUZZLE3D_MESH_UPLOAD_SLOTS {
                let stale = uploads.slots.iter().enumerate().min_by_key(|(_, slot)| slot.touched).map(|(index, _)| index).ok_or(Puzzle3dMeshUploadFault::Capacity)?;
                uploads.slots.swap_remove(stale);
            }
            uploads.slots.push(Puzzle3dMeshUploadSlot { url: url.to_string(), digest: digest.to_string(), page_count, next_page: 0, positions: Vec::new(), indices: Vec::new(), touched: sequence });
            uploads.slots.len() - 1
        }
    };
    let slot = &mut uploads.slots[index];
    if slot.positions.len().saturating_add(positions.len()) > COLLISION_MESH_MAX_VALUES || slot.indices.len().saturating_add(indices.len()) > COLLISION_MESH_MAX_VALUES {
        uploads.slots.swap_remove(index);
        return Err(Puzzle3dMeshUploadFault::Capacity);
    }
    slot.positions.extend_from_slice(positions);
    slot.indices.extend_from_slice(indices);
    slot.next_page = page.saturating_add(1);
    slot.touched = sequence;
    if slot.next_page < slot.page_count {
        return Ok(Puzzle3dMeshUploadStep::Staged { next_page: slot.next_page, page_count: slot.page_count });
    }
    let closed = uploads.slots.swap_remove(index);
    if brush_mesh_digest(&closed.positions, &closed.indices) != closed.digest {
        return Err(Puzzle3dMeshUploadFault::Digest);
    }
    if !brush_mesh_geometry_is_admissible(&closed.positions, &closed.indices) {
        return Err(Puzzle3dMeshUploadFault::Geometry);
    }
    Ok(Puzzle3dMeshUploadStep::Complete(closed.positions, closed.indices))
}

/// 🧹️ Drops every run that did not advance across a whole session-retirement cycle — the page runs a
/// closed document instance abandoned. A run that is still being paged carries a `touched` newer than
/// the watermark and survives.
pub fn retire_abandoned_brush_mesh_uploads() {
    let mut uploads = brush_mesh_uploads().lock().unwrap_or_else(std::sync::PoisonError::into_inner);
    let watermark = uploads.swept;
    uploads.slots.retain(|slot| slot.touched > watermark);
    uploads.swept = uploads.sequence;
}

/// 🔎️ Open page runs, for the unit laws and the staging census.
pub fn staged_brush_mesh_uploads() -> Vec<(String, String, u32, u32)> {
    let uploads = brush_mesh_uploads().lock().unwrap_or_else(std::sync::PoisonError::into_inner);
    uploads.slots.iter().map(|slot| (slot.url.clone(), slot.digest.clone(), slot.next_page, slot.page_count)).collect()
}
//#endregion 🧩️PagedBrushMeshUploads

//#region 🔖️Engine
/// 🗺️ Cell edge of the interactive brush broad phase — the same 8.0 world units the fill planner
/// indexes with, so the brush lane and a fill run bucket identically.
const BRUSH_INDEX_CELL_SIZE: f32 = 8.0;

/// 🗺️ What ONE scene sync invalidated, named per object instead of per document.
///
/// 🧾️ Every sync whose scene differed at all used to be a whole-document `rebuild_queue`: `brush_cache`
/// cleared for every object, both prepare cursors reset to zero so the entire object × vortex product
/// was re-walked. On the 340-object Nakagin document one pose edit therefore threw away every resolved
/// brush candidate — and the background `suggestionsTick` cadence re-paid that every 120 ms while an
/// interactive mutation waited behind it,
/// which is how a translate that reads one object burned a 30-second budget
/// (ticket 26/09/02/PUZZLE-3D-END-TO-END wave B54).
///
/// [`Self::plan`] names the members the compatibility tables and candidate weights are derived from: any of
/// them invalidates every candidate, so that case stays the whole-scene rebuild.
pub(crate) struct Puzzle3dSceneInvalidation {
    pub(crate) stale: std::collections::HashSet<String>,
    pub(crate) pending: Vec<String>,
    pub(crate) topology: bool,
    pub(crate) plan: bool,
}

impl Puzzle3dSceneInvalidation {
    pub(crate) fn between(previous: &SceneConfig, next: &SceneConfig) -> Self {
        let plan = previous.kind_catalogs != next.kind_catalogs
            || previous.kind_compatibility != next.kind_compatibility
            || previous.contact_tolerance != next.contact_tolerance
            || previous.seed != next.seed
            || previous.host_rules != next.host_rules
            || previous.weights != next.weights
            || previous.fixture.attractions != next.fixture.attractions
            || previous.fixture.target_volumes != next.fixture.target_volumes;
        let before: HashMap<&str, &FixtureObject> = previous.fixture.objects.iter().map(|object| (object.id.as_str(), object)).collect();
        let after: HashMap<&str, &FixtureObject> = next.fixture.objects.iter().map(|object| (object.id.as_str(), object)).collect();
        let mut invalidation = Self { stale: std::collections::HashSet::new(), pending: Vec::new(), topology: false, plan };
        for object in &next.fixture.objects {
            match before.get(object.id.as_str()) {
                Some(retained) if **retained == *object => continue,
                Some(retained) => invalidation.mark_stale(retained),
                None => invalidation.topology = true,
            }
            invalidation.mark_stale(object);
            invalidation.pending.extend(Self::vortex_ids(object));
        }
        for object in &previous.fixture.objects {
            if after.contains_key(object.id.as_str()) {
                continue;
            }
            invalidation.topology = true;
            invalidation.mark_stale(object);
        }
        invalidation
    }

    fn mark_stale(&mut self, object: &FixtureObject) {
        self.stale.extend(Self::vortex_ids(object));
    }

    fn vortex_ids(object: &FixtureObject) -> impl Iterator<Item = String> + '_ {
        object.vortices.iter().map(|vortex| puzzle3d_vortex_full_id(&object.id, &vortex.id))
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[derive(Default)]
enum BrushIndexSyncStage {
    #[default]
    Objects,
    CollectStale,
    Removals,
}

/// 🔁️ Resumable, step-budgeted reconciliation of the persistent brush broad-phase index against the
/// scene the host last synced in: one object replacement per step, then one withdrawal per stale cell,
/// so a document-scale edit never rebuilds the index and no single step exceeds the lane budget.
#[derive(Default)]
struct BrushIndexSync {
    stage: BrushIndexSyncStage,
    object_cursor: usize,
    seen: std::collections::HashSet<String>,
    stale: Vec<String>,
    mutation: Option<CollisionIndexMutation>,
    removal: Option<CollisionIndexRemoval>,
}



pub(crate) struct Puzzle3dCollision {
    pub(crate) scene: Option<Arc<SceneConfig>>,
    /// 🧊️ The last scene this engine was synced FROM, so a resync with an identical config (every
    /// action re-syncs the session, see the app's `sync_precompute_session`) can skip `rebuild_queue`
    /// instead of wiping `brush_cache`/`queue` and restarting suggestion precompute from
    /// zero. Held as the decoded value, never as its JSON text: normalizing a 180-object document to a
    /// string to compare it cost 13 ms per round trip and three round trips per sync — 39 ms of the
    /// 44 ms every command spent here, over the framework's whole 8 ms interactive step ceiling on its
    /// own.
    scene_synced: Option<Arc<SceneConfig>>,
    meshes: Arc<HashMap<String, CollisionBody>>,
    mesh_is_fallback: HashMap<String, bool>,
    mesh_sources: HashMap<String, CollisionMeshSource>,
    /// 🚚️ Mesh ids this session was announced by id alone and could NOT serve — neither from its own
    /// installed geometry nor from the process-wide store. Sorted, deduplicated, bounded by
    /// [`COLLISION_MESH_MAX_MESHES`], and published as `interactionJson.meshReuploadUrls` so the client
    /// pages the bytes again. An entry retires the instant that identity's geometry installs, so the
    /// set is empty in every steady state and a refusal can never become a standing request.
    mesh_reupload_requests: Vec<String>,
    pub(crate) brush_cache: HashMap<String, BrushCollisionFreeResult>,
    pub(crate) brush_queue: VecDeque<String>,
    brush_prepare_object_cursor: usize,
    brush_prepare_vortex_cursor: usize,
    brush_queue_preparing: bool,
    /// 🗺️ Persistent broad phase of the interactive brush lane — reconciled incrementally from the
    /// scene the host syncs in, never rebuilt per collision check, and carried across worker hops by
    /// the app's own session slot.
    brush_index: CollisionSpatialIndex,
    brush_index_owner: CollisionIndexOwner,
    brush_index_sync: Option<BrushIndexSync>,
    brush_index_ready: bool,
    /// 🧊️ World placement of every indexed owner, so a broad-phase candidate id resolves to its pose
    /// and mesh in constant time instead of a scan over the fixture.
    brush_placed: HashMap<String, PlacedCollisionEntry>,
}

impl Puzzle3dCollision {
    pub(crate) fn new() -> Self {
        Self {
            scene: None,
            scene_synced: None,
            meshes: Arc::new(HashMap::new()),
            mesh_is_fallback: HashMap::new(),
            mesh_sources: HashMap::new(),
            mesh_reupload_requests: Vec::new(),
            brush_cache: HashMap::new(),
            brush_queue: VecDeque::new(),
            brush_prepare_object_cursor: 0,
            brush_prepare_vortex_cursor: 0,
            brush_queue_preparing: false,
            brush_index: CollisionSpatialIndex::new(BRUSH_INDEX_CELL_SIZE),
            brush_index_owner: CollisionIndexOwner { operation: 1, generation: 1 },
            brush_index_sync: None,
            brush_index_ready: false,
            brush_placed: HashMap::new(),
        }
    }

    fn brush_lane_active(&self) -> bool {
        self.brush_queue_preparing || !self.brush_queue.is_empty()
    }

    #[cfg(test)]
    /// 📊️ How many resolved brush candidates the engine still holds — the observable that says whether a
    /// scene sync invalidated per object or per document.
    pub(crate) fn brush_candidate_cache_len(&self) -> usize {
        self.brush_cache.len()
    }

    fn re_enqueue_brush_targets(&mut self) {
        self.brush_prepare_object_cursor = 0;
        self.brush_prepare_vortex_cursor = 0;
        self.brush_queue_preparing = self.scene.is_some();
        self.begin_brush_index_sync();
    }

    /// 🔁️ Arms one incremental reconciliation pass of the persistent brush broad phase and bumps the
    /// index generation, so any query or replacement still in flight from the previous scene goes
    /// `Stale` instead of mixing two scenes' owners.
    fn begin_brush_index_sync(&mut self) {
        let Some(generation) = self.brush_index_owner.generation.checked_add(1) else {
            self.brush_index = CollisionSpatialIndex::new(BRUSH_INDEX_CELL_SIZE);
            self.brush_placed.clear();
            self.brush_index_sync = None;
            self.brush_index_ready = false;
            return;
        };
        self.brush_index_owner = CollisionIndexOwner { operation: self.brush_index_owner.operation, generation };
        self.brush_index_sync = self.scene.is_some().then(BrushIndexSync::default);
        self.brush_index_ready = false;
    }

    /// 🔁️ Brings the persistent broad phase up to date with the installed scene, within the caller's own
    /// deadline. A query is only ever answered against a reconciled index; if the budget runs out first the
    /// caller sees `unknown_pending` and the vortex is re-queued, which is the lane's existing resume
    /// contract. Arms a pass itself, so a scene installed without a lane tick is still indexed.
    fn reconcile_brush_index_until(&mut self, deadline_us: u64) {
        if !self.brush_index_ready && self.brush_index_sync.is_none() {
            self.begin_brush_index_sync();
        }
        while self.brush_index_sync.is_some() && default_now_us().is_some_and(|now| now < deadline_us) {
            self.step_brush_index();
        }
    }

    /// 🗺️ One unit of broad-phase reconciliation: one object replacement cell, one stale-owner scan, or
    /// one withdrawal cell. Returns whether more work is pending, which is what keeps the brush lane
    /// ticking under its own 2 ms budget instead of stalling a worker turn.
    fn step_brush_index(&mut self) -> bool {
        let Some(mut sync) = self.brush_index_sync.take() else { return false };
        let Some(scene) = self.scene.clone() else {
            self.brush_index_ready = false;
            return false;
        };
        let pending = match sync.stage {
            BrushIndexSyncStage::Objects => self.step_brush_index_objects(&mut sync, &scene),
            BrushIndexSyncStage::CollectStale => {
                let seen = std::mem::take(&mut sync.seen);
                sync.stale = self.brush_index.entry_ids().filter(|id| !seen.contains(id.as_str())).cloned().collect();
                sync.stage = BrushIndexSyncStage::Removals;
                true
            }
            BrushIndexSyncStage::Removals => self.step_brush_index_removals(&mut sync),
        };
        if pending {
            self.brush_index_sync = Some(sync);
        } else {
            self.brush_index_ready = true;
        }
        pending
    }

    fn step_brush_index_objects(&mut self, sync: &mut BrushIndexSync, scene: &SceneConfig) -> bool {
        let owner = self.brush_index_owner;
        if let Some(mutation) = sync.mutation.as_mut() {
            match self.brush_index.step_replacement(mutation, owner) {
                CollisionMutationStep::Pending => return true,
                CollisionMutationStep::Rejected(mut rejected) => while !rejected.retire_one() {},
                CollisionMutationStep::Complete | CollisionMutationStep::Stale => {}
            }
            sync.mutation = None;
            return true;
        }
        let Some(object) = scene.fixture.objects.get(sync.object_cursor) else {
            sync.stage = BrushIndexSyncStage::CollectStale;
            return true;
        };
        sync.object_cursor += 1;
        let empty_catalogs = KindCatalogBundle { objects: vec![], vortices: vec![], cables: vec![] };
        let catalogs = scene.kind_catalogs.as_ref().unwrap_or(&empty_catalogs);
        let Some(mesh_url) = resolve_placed_object_mesh_url(object, catalogs, &scene.fixture) else { return true };
        let world = pose_isometry(object.origin, object.orientation.unwrap_or([0.0, 0.0, 0.0, 1.0]), &object.scale);
        let Some(bounds) = self.meshes.get(&mesh_url).map(|body| CollisionAabb::from_body(body, &world)) else { return true };
        sync.seen.insert(object.id.clone());
        if self.brush_index.entry_bounds(object.id.as_str()) != Some(&bounds) {
            sync.mutation = Some(self.brush_index.begin_replacement(owner, object.id.clone(), bounds));
        }
        self.brush_placed.insert(object.id.clone(), PlacedCollisionEntry { object_id: object.id.clone(), mesh_url, world });
        true
    }

    fn step_brush_index_removals(&mut self, sync: &mut BrushIndexSync) -> bool {
        let owner = self.brush_index_owner;
        if let Some(removal) = sync.removal.as_mut() {
            if matches!(self.brush_index.step_removal(removal, owner), CollisionMutationStep::Pending) {
                return true;
            }
            sync.removal = None;
            return true;
        }
        let Some(id) = sync.stale.pop() else { return false };
        self.brush_placed.remove(id.as_str());
        sync.removal = self.brush_index.begin_removal(owner, id);
        true
    }

    fn prepare_one_brush_target(&mut self) {
        let Some(scene) = &self.scene else {
            self.brush_queue_preparing = false;
            return;
        };
        let Some(object) = scene.fixture.objects.get(self.brush_prepare_object_cursor) else {
            self.brush_queue_preparing = false;
            return;
        };
        let Some(vortex) = object.vortices.get(self.brush_prepare_vortex_cursor) else {
            self.brush_prepare_object_cursor += 1;
            self.brush_prepare_vortex_cursor = 0;
            return;
        };
        self.brush_prepare_vortex_cursor += 1;
        let full_id = puzzle3d_vortex_full_id(&object.id, &vortex.id);
        if !self.brush_cache.contains_key(&full_id) {
            self.brush_queue.push_back(full_id);
        }
    }

    /// 🧊️ Everything derived from the scene is stale: the brush lane's queue and its cached
    /// collision-free candidates go. ONLY a real scene change may call this.
    fn rebuild_queue(&mut self) {
        self.brush_queue.clear();
        self.brush_cache.clear();
        self.re_enqueue_brush_targets();
    }

    pub(crate) fn update_kind_weights(&mut self, object_weights: std::collections::BTreeMap<String, f64>, vortex_weights: std::collections::BTreeMap<String, f64>) {
        if let Some(scene) = &mut self.scene {
            let scene = Arc::make_mut(scene);
            scene.weights.object_weights = object_weights;
            scene.weights.vortex_weights = vortex_weights;
            self.scene_synced = Some(Arc::new(scene.clone()));
        }
        self.rebuild_queue();
    }

    #[cfg(test)]
    pub(crate) fn set_scene(&mut self, json: &str) -> Result<(), Puzzle3dError> {
        self.set_scene_config(dsl::os_pack::json::from_json_str(json)?);
        Ok(())
    }

    /// 🧊️ The typed sync every caller actually takes: the app already holds a decoded `SceneConfig`,
    /// so routing it through a JSON string and back is three whole-document round trips of pure loss.
    pub(crate) fn set_scene_config(&mut self, scene: SceneConfig) {
        if self.scene_synced.as_deref() == Some(&scene) {
            return;
        }
        self.replace_scene(scene);
    }

    /// 🗺️ Invalidates the brush derivation for exactly the objects one sync changed. The cached
    /// collision-free candidates of the changed and removed objects are evicted by their own vortex
    /// ids, the changed objects' vortices are re-queued, and the persistent broad phase is re-armed — it
    /// is already incremental and replaces only the entries whose bounds actually moved
    /// ([`Self::step_brush_index_objects`]).
    fn invalidate_scene_objects(&mut self, invalidation: Puzzle3dSceneInvalidation) {
        for full_id in &invalidation.stale {
            self.brush_cache.remove(full_id);
        }
        self.brush_queue.retain(|full_id| !invalidation.stale.contains(full_id));
        self.brush_queue.extend(invalidation.pending);
        self.begin_brush_index_sync();
    }

    fn replace_scene(&mut self, scene: SceneConfig) {
        let invalidation = self.scene_synced.as_deref().map(|synced| Puzzle3dSceneInvalidation::between(synced, &scene));
        let scene = Arc::new(scene);
        self.scene = Some(Arc::clone(&scene));
        self.scene_synced = Some(scene);
        match invalidation {
            Some(invalidation) if !invalidation.plan => self.invalidate_scene_objects(invalidation),
            _ => self.rebuild_queue(),
        }
    }

    /// 🥽️ Real geometry for one mesh identity, derived once per process through the content-addressed
    /// decode kernel — the id-only wire path a second document (or a later command on the same one)
    /// takes instead of re-uploading buffers.
    pub(crate) fn adopt_shared_mesh(&mut self, url: &str, digest: Option<&str>) -> bool {
        if self.mesh_is_fallback.get(url) == Some(&false) && digest.is_none_or(|digest| self.mesh_sources.get(url).is_some_and(|mesh| brush_mesh_digest(&mesh.positions, &mesh.indices) == digest)) {
            self.retire_mesh_reupload(url);
            return true;
        }
        let resident = shared_brush_mesh(url).filter(|(positions, indices)| digest.is_none_or(|digest| brush_mesh_digest(positions, indices) == digest));
        let Some((positions, indices)) = resident.or_else(|| digest.and_then(|digest| adopt_brush_mesh_by_digest(url, digest))) else {
            return false;
        };
        self.install_collision_mesh(url.to_string(), &positions, &indices, false);
        self.retire_mesh_reupload(url);
        true
    }

    /// 🚚️ Retires one identity's standing re-upload request, because that identity is now — or was
    /// already — servable from this session.
    ///
    /// 🐛️ RESIDENCY is the authority and the request set is only a cache of "announced by id alone and
    /// could not be served", so EVERY path that answers an announcement has to prune it. Until this
    /// existed the single prune sat inside [`Puzzle3dCollision::place_collision_mesh`], behind that
    /// function's own already-resident bail — so the three paths that satisfy an announcement WITHOUT
    /// writing geometry (this function's resident fast return, `stage_mesh_page`'s page-0 short circuit,
    /// and that bail itself) all left the request standing. The world body then published the id in
    /// `interactionJson.meshReuploadUrls` forever
    /// (`✏️editor/🎭️modes/✏️edit/🪟️windows/🧊️main/🦀️.rs`), the client re-claimed it on every residency
    /// climb and paged the whole 72-command run again, and every page after the first was refused as a
    /// `Gap`. Measured on the live `:6013` shell at wasm #58: 123 of 285 console lines in one 150-second
    /// window were `registerBrushMesh`, seq 22 → 124 over 306 s, still arriving 8 minutes after the
    /// example switch (ticket 26/09/02/PUZZLE-3D-END-TO-END wave B46 §5, wave B48 §1.2).
    fn retire_mesh_reupload(&mut self, url: &str) {
        self.mesh_reupload_requests.retain(|pending| pending != url);
    }

    /// 🚚️ Records that one mesh id was announced by identity alone and could not be served, so the
    /// world body's next projection asks the client for the bytes. Bounded and idempotent: a client
    /// that re-announces the same dead id on every window activation never grows the set.
    ///
    /// 🐢️ The return value is that idempotence made VISIBLE to the caller: `true` only when this call
    /// added a request the world body has not published yet. A re-announcement of an already-pending id
    /// changes nothing any surface renders, and republishing the world body for it is a refresh storm —
    /// measured on the live `:6013` shell (ticket 26/09/02/PUZZLE-3D-END-TO-END wave B32) as 23 of 33
    /// typed-operation completions carrying the viewport scope in a 75 s window with four user actions,
    /// every one of them re-rendering all three world bodies and 23 answering `unchanged`.
    pub(crate) fn request_mesh_reupload(&mut self, url: &str) -> bool {
        if url.len() > COLLISION_MESH_MAX_URL_BYTES || self.mesh_reupload_requests.iter().any(|pending| pending == url) {
            return false;
        }
        if self.mesh_reupload_requests.len() >= COLLISION_MESH_MAX_MESHES {
            return false;
        }
        self.mesh_reupload_requests.push(url.to_string());
        self.mesh_reupload_requests.sort_unstable();
        true
    }

    /// 🚚️ The open re-upload requests, in a stable order so an unchanged set hashes to an unchanged
    /// world-body lane — see [`Puzzle3dCollision::request_mesh_reupload`].
    pub(crate) fn mesh_reupload_requests(&self) -> &[String] {
        &self.mesh_reupload_requests
    }

    fn place_collision_mesh(&mut self, url: String, positions: &[f32], indices: &[u32], is_fallback: bool) -> bool {
        if url.len() > COLLISION_MESH_MAX_URL_BYTES || positions.len() > COLLISION_MESH_MAX_VALUES || indices.len() > COLLISION_MESH_MAX_VALUES {
            return false;
        }
        if !self.mesh_sources.contains_key(&url) && self.mesh_sources.len() >= COLLISION_MESH_MAX_MESHES {
            return false;
        }
        let Some(body) = crate::editor::puzzle3d::precompute::geometry::collision_body_from_buffers(positions, indices) else {
            return false;
        };
        if self.mesh_is_fallback.get(&url) == Some(&false) {
            self.retire_mesh_reupload(&url);
            return false;
        }
        Arc::make_mut(&mut self.meshes).insert(url.clone(), body);
        self.mesh_is_fallback.insert(url.clone(), is_fallback);
        if !is_fallback {
            self.retire_mesh_reupload(&url);
        }
        self.mesh_sources.insert(url.clone(), CollisionMeshSource { url, positions: positions.to_vec(), indices: indices.to_vec() });
        true
    }

    fn install_collision_mesh(&mut self, url: String, positions: &[f32], indices: &[u32], is_fallback: bool) {
        if !self.place_collision_mesh(url, positions, indices, is_fallback) {
            return;
        }
        self.rebuild_queue();
    }

    pub(crate) fn register_mesh_fallback(&mut self, url: String, positions: &[f32], indices: &[u32]) {
        self.install_collision_mesh(url, positions, indices, true);
    }

    /// 🥽️ Uploaded geometry for one mesh identity: derived into the process-wide content-addressed
    /// store first, so every other open document and every later command reaches it by id alone.
    pub(crate) fn register_mesh(&mut self, url: String, positions: &[f32], indices: &[u32]) {
        match derive_brush_mesh(&url, positions, indices) {
            Some((positions, indices)) => self.install_collision_mesh(url, &positions, &indices, false),
            None => self.install_collision_mesh(url, positions, indices, false),
        }
    }

    pub(crate) fn has_mesh(&self, url: &str) -> bool {
        self.meshes.contains_key(url)
    }

    /// 🧊️ Drops a cached brush-candidate entry and re-queues that vortex at the front so a just-opened
    /// suggestion popup is not stuck on a stale empty / pending result.
    pub(crate) fn invalidate_brush_target(&mut self, vortex_full_id: &str) {
        self.brush_cache.remove(vortex_full_id);
        self.brush_queue.retain(|id| id != vortex_full_id);
        self.brush_queue.push_front(vortex_full_id.to_string());
    }

    pub(crate) fn enqueue_brush_target(&mut self, vortex_full_id: &str) {
        if !self.brush_queue.iter().any(|id| id == vortex_full_id) {
            self.brush_queue.push_back(vortex_full_id.to_string());
        }
    }

    /// 🧊️ Recomputes and caches brush candidates for one vortex immediately (used when opening / accepting
    /// the suggestion popup so the UI does not wait on the background queue).
    pub(crate) fn refresh_brush_candidates(&mut self, vortex_full_id: &str) {
        let Some(deadline) = puzzle3d_deadline(PUZZLE3D_PRECOMPUTE_STEP_BUDGET_US) else {
            return;
        };
        let prior = self.brush_cache.get(vortex_full_id).cloned();
        let resume_from = prior.as_ref().map_or(0, |entry| entry.resume_candidate_index);
        let prior_free = prior.map(|entry| entry.free).unwrap_or_default();
        let result = self.compute_brush_cache_entry_partial(vortex_full_id, resume_from, prior_free, deadline);
        // 🖌️ A NON-TERMINAL entry is owed another slice, whatever its resume cursor says. Gating the
        // re-queue on `resume_candidate_index > 0` livelocked every target that could not clear its
        // broad-phase index (`brush_index_ready`, `brush_collision_free_until`) or its very first
        // narrow-phase candidate inside one 500 µs slice: the entry landed in `brush_cache` pending,
        // which also stops `prepare_one_brush_target` re-enqueuing it, so the lane could never come
        // back to it and the suggestion popup opened empty forever.
        if result.unknown_pending && !self.brush_queue.iter().any(|id| id == vortex_full_id) {
            self.brush_queue.push_front(vortex_full_id.to_string());
        }
        self.brush_cache.insert(vortex_full_id.to_string(), result);
    }

    fn preview_collides(meshes: &HashMap<String, CollisionBody>, preview: &BrushPreviewState, placed: &[PlacedCollisionEntry], contact_tolerance: f64, deadline_us: u64) -> Option<bool> {
        struct BrushCollisionContext {
            deadline_us: u64,
        }
        impl CollisionStepContext for BrushCollisionContext {
            fn is_cancelled(&self) -> bool {
                false
            }
            fn should_yield(&self) -> bool {
                default_now_us().is_none_or(|now| now >= self.deadline_us)
            }
            fn consume_fuel(&mut self, _units: u64) {}
        }
        let preview_body = meshes.get(&preview.mesh_url)?;
        let preview_world = pose_isometry(preview.origin, preview.orientation, &preview.scale);
        let (pmin, pmax) = world_bounds(preview_body, &preview_world);
        let mut context = BrushCollisionContext { deadline_us };
        for entry in placed {
            let other = meshes.get(&entry.mesh_url)?;
            let (omin, omax) = world_bounds(other, &entry.world);
            if pmax.x() < omin.x() || pmin.x() > omax.x() || pmax.y() < omin.y() || pmin.y() > omax.y() || pmax.z() < omin.z() || pmin.z() > omax.z() {
                continue;
            }
            let mut collision = CollisionPenetrationState::new(contact_tolerance);
            loop {
                match collision.step(&mut context, preview_body, &preview_world, other, &entry.world) {
                    CollisionStepResult::Pending if context.should_yield() => return None,
                    CollisionStepResult::Pending => {}
                    CollisionStepResult::Cancelled => return None,
                    CollisionStepResult::Complete { depth, .. } if depth > contact_tolerance => return Some(true),
                    CollisionStepResult::Complete { .. } => break,
                }
            }
        }
        Some(false)
    }

    /// 🔎️ One resumable slice of ONE vortex's collision-free search, the background lane puzzle 5d still
    /// reads. A candidate already held free is never pushed twice, so a pass that restarts at 0 (a mesh only
    /// arrived later) refines the same list instead of duplicating it.
    fn brush_collision_free_until(&mut self, target_full_id: &str, candidates: &[BrushCompatibleCandidate], contact_tolerance: f64, resume_from: usize, mut free: Vec<BrushCompatibleCandidate>, deadline_us: u64) -> BrushCollisionFreeResult {
        self.reconcile_brush_index_until(deadline_us);
        let Some(scene) = self.scene.clone() else {
            return BrushCollisionFreeResult { free: vec![], unknown_pending: true, resume_candidate_index: resume_from };
        };
        let empty_catalogs = KindCatalogBundle { objects: vec![], vortices: vec![], cables: vec![] };
        let catalogs = scene.kind_catalogs.as_ref().unwrap_or(&empty_catalogs);
        let target = scene.fixture.objects.iter().find_map(|object| object.vortices.iter().position(|vortex| puzzle3d_vortex_full_id(&object.id, &vortex.id) == target_full_id).map(|index| (object, index)));
        let Some((host, vortex_index)) = target else {
            return BrushCollisionFreeResult { free: vec![], unknown_pending: false, resume_candidate_index: 0 };
        };
        let Some((position, direction)) = vortex_world_from_object(host, vortex_index) else {
            return BrushCollisionFreeResult { free: vec![], unknown_pending: false, resume_candidate_index: 0 };
        };
        let target_ctx = AttractionVortexContext { object_kind: host.object_kind.clone(), vortex_kind: host.vortices[vortex_index].vortex_kind.clone() };
        if !self.brush_index_ready {
            return BrushCollisionFreeResult { free, unknown_pending: true, resume_candidate_index: resume_from };
        }
        let mut unknown_pending = false;
        for (index, candidate) in candidates.iter().enumerate().skip(resume_from) {
            if default_now_us().is_none_or(|now| now >= deadline_us) {
                return BrushCollisionFreeResult { free, unknown_pending: true, resume_candidate_index: index };
            }
            let world = TargetVortexWorld { position, direction, reference_orientation: host.orientation };
            let Some(preview) = brush_preview_from_candidate(target_full_id, candidate, &target_ctx, world, catalogs, &scene.fixture) else {
                continue;
            };
            if !self.meshes.contains_key(&preview.mesh_url) {
                unknown_pending = true;
                continue;
            }
            match self.preview_collides_indexed(&preview, contact_tolerance, deadline_us) {
                None => unknown_pending = true,
                Some(true) => {}
                Some(false) => {
                    if !free.iter().any(|held| held.object_kind_id == candidate.object_kind_id && held.source_vortex_index == candidate.source_vortex_index) {
                        free.push(candidate.clone());
                    }
                }
            }
        }
        BrushCollisionFreeResult { free, unknown_pending, resume_candidate_index: 0 }
    }

    /// 🗺️ Broad phase for ONE brush preview: the persistent spatial index resolves the candidate page
    /// its own world bounds actually overlap, so the narrow phase never sees an object from a distant
    /// cell. Replaces the former full-fixture rebuild plus linear scan per candidate.
    fn preview_collides_indexed(&self, preview: &BrushPreviewState, contact_tolerance: f64, deadline_us: u64) -> Option<bool> {
        let (page, _) = self.brush_broad_phase_page(preview, deadline_us)?;
        Self::preview_collides(&self.meshes, preview, &page, contact_tolerance, deadline_us)
    }

    /// 📏️ The queried candidate page plus the `(cells, members)` the cursor actually examined — the
    /// witness a document-scale test reads to prove the query stayed inside its own cells.
    fn brush_broad_phase_page(&self, preview: &BrushPreviewState, deadline_us: u64) -> Option<(Vec<PlacedCollisionEntry>, (usize, usize))> {
        let preview_body = self.meshes.get(&preview.mesh_url)?;
        let preview_world = pose_isometry(preview.origin, preview.orientation, &preview.scale);
        let owner = self.brush_index_owner;
        let mut query = self.brush_index.begin_query(owner, CollisionAabb::from_body(preview_body, &preview_world));
        loop {
            match self.brush_index.step_query(&mut query, owner) {
                CollisionQueryStep::Pending if default_now_us().is_none_or(|now| now >= deadline_us) => return None,
                CollisionQueryStep::Pending => {}
                CollisionQueryStep::Stale => return None,
                CollisionQueryStep::Complete => break,
            }
        }
        // 🧲️ The docking host is a collision pair like any other: a candidate may only touch it where the two faces meet, and the
        // depth measure keeps that flush contact at 0 (`CollisionPenetrationState`).
        let page = (0..query.len()).filter_map(|index| query.candidate(index)).filter_map(|id| self.brush_placed.get(id.as_str()).cloned()).collect();
        Some((page, query.examined()))
    }

    #[cfg(test)]
    fn brush_collision_free(&mut self, target_full_id: &str, candidates: &[BrushCompatibleCandidate], contact_tolerance: f64) -> BrushCollisionFreeResult {
        let Some(deadline) = puzzle3d_deadline(PUZZLE3D_PRECOMPUTE_STEP_BUDGET_US * 8) else {
            return BrushCollisionFreeResult { free: Vec::new(), unknown_pending: true, resume_candidate_index: 0 };
        };
        self.brush_collision_free_until(target_full_id, candidates, contact_tolerance, 0, Vec::new(), deadline)
    }

    #[cfg(test)]
    pub(crate) fn compute_brush_cache_entry(&mut self, target_full_id: &str) -> BrushCollisionFreeResult {
        let Some(scene) = &self.scene else {
            return BrushCollisionFreeResult { free: vec![], unknown_pending: true, resume_candidate_index: 0 };
        };
        let catalogs = scene.kind_catalogs.as_ref().cloned().unwrap_or(KindCatalogBundle { objects: vec![], vortices: vec![], cables: vec![] });
        let target_obj = scene.fixture.objects.iter().find_map(|o| {
            o.vortices.iter().enumerate().find_map(|(i, v)| {
                let full_id = puzzle3d_vortex_full_id(&o.id, &v.id);
                if full_id == target_full_id {
                    Some((o, i, v))
                } else {
                    None
                }
            })
        });
        let Some((host, _, vortex)) = target_obj else {
            return BrushCollisionFreeResult { free: vec![], unknown_pending: false, resume_candidate_index: 0 };
        };
        let target_ctx = AttractionVortexContext { object_kind: host.object_kind.clone(), vortex_kind: vortex.vortex_kind.clone() };
        if !brush_target_vortex_allows_suggestion(vortex.vortex_kind.as_deref(), &scene.weights) {
            return BrushCollisionFreeResult { free: vec![], unknown_pending: false, resume_candidate_index: 0 };
        }
        let compatible = brush_compatible_candidates(&target_ctx, &catalogs, &scene.kind_compatibility, &scene.host_rules);
        let compatible: Vec<BrushCompatibleCandidate> = compatible.into_iter().filter(|candidate| brush_candidate_suggestion_weight(candidate, &scene.weights, &catalogs) > 0.0).collect();
        self.brush_collision_free(target_full_id, &compatible, scene.contact_tolerance)
    }

    pub(crate) fn brush_preview(&self, target_full_id: &str, candidate_index: usize) -> Option<BrushPreviewState> {
        let scene = self.scene.as_ref()?;
        let result = self.brush_cache.get(target_full_id)?;
        if result.unknown_pending && result.free.is_empty() {
            return None;
        }
        if result.free.is_empty() {
            return None;
        }
        let candidate = &result.free[candidate_index % result.free.len()];
        let catalogs = scene.kind_catalogs.as_ref().cloned().unwrap_or(KindCatalogBundle { objects: vec![], vortices: vec![], cables: vec![] });
        let target_obj = scene.fixture.objects.iter().find_map(|object| {
            object.vortices.iter().enumerate().find_map(|(index, vortex)| {
                let full_id = puzzle3d_vortex_full_id(&object.id, &vortex.id);
                if full_id == target_full_id {
                    Some((object, index))
                } else {
                    None
                }
            })
        })?;
        let (host, vortex_index) = target_obj;
        let (position, direction) = vortex_world_from_object(host, vortex_index)?;
        let target_ctx = AttractionVortexContext { object_kind: host.object_kind.clone(), vortex_kind: host.vortices[vortex_index].vortex_kind.clone() };
        let world = TargetVortexWorld { position, direction, reference_orientation: host.orientation };
        brush_preview_from_candidate(target_full_id, candidate, &target_ctx, world, &catalogs, &scene.fixture)
    }

    pub(crate) fn precompute_step(&mut self, budget: u32) -> bool {
        let Some(deadline) = puzzle3d_deadline(PUZZLE3D_PRECOMPUTE_STEP_BUDGET_US) else {
            return self.brush_lane_active();
        };
        let mut remaining = budget as usize;
        while remaining > 0 {
            if default_now_us().is_none_or(|now| now >= deadline) {
                break;
            }
            if self.brush_queue_preparing {
                self.prepare_one_brush_target();
                remaining -= 1;
                continue;
            }
            let Some(full_id) = self.brush_queue.pop_front() else {
                break;
            };
            let prior = self.brush_cache.get(&full_id).cloned();
            let resume_from = prior.as_ref().map_or(0, |entry| entry.resume_candidate_index);
            let prior_free = prior.map(|entry| entry.free).unwrap_or_default();
            let result = self.compute_brush_cache_entry_partial(&full_id, resume_from, prior_free, deadline);
            // 🖌️ Same law as `refresh_brush_candidates`: pending IS the owed signal.
            if result.unknown_pending {
                self.brush_queue.push_front(full_id.clone());
            }
            self.brush_cache.insert(full_id, result);
            remaining -= 1;
        }
        self.brush_lane_active()
    }

    fn compute_brush_cache_entry_partial(&mut self, target_full_id: &str, resume_from: usize, prior_free: Vec<BrushCompatibleCandidate>, deadline_us: u64) -> BrushCollisionFreeResult {
        let Some(scene) = &self.scene else {
            return BrushCollisionFreeResult { free: vec![], unknown_pending: true, resume_candidate_index: resume_from };
        };
        let catalogs = scene.kind_catalogs.as_ref().cloned().unwrap_or(KindCatalogBundle { objects: vec![], vortices: vec![], cables: vec![] });
        let target_obj = scene.fixture.objects.iter().find_map(|o| {
            o.vortices.iter().enumerate().find_map(|(i, v)| {
                let full_id = puzzle3d_vortex_full_id(&o.id, &v.id);
                if full_id == target_full_id {
                    Some((o, i, v))
                } else {
                    None
                }
            })
        });
        let Some((host, _, vortex)) = target_obj else {
            return BrushCollisionFreeResult { free: prior_free, unknown_pending: false, resume_candidate_index: 0 };
        };
        let target_ctx = AttractionVortexContext { object_kind: host.object_kind.clone(), vortex_kind: vortex.vortex_kind.clone() };
        if !brush_target_vortex_allows_suggestion(vortex.vortex_kind.as_deref(), &scene.weights) {
            return BrushCollisionFreeResult { free: prior_free, unknown_pending: false, resume_candidate_index: 0 };
        }
        let compatible = brush_compatible_candidates(&target_ctx, &catalogs, &scene.kind_compatibility, &scene.host_rules);
        let compatible: Vec<BrushCompatibleCandidate> = compatible.into_iter().filter(|candidate| brush_candidate_suggestion_weight(candidate, &scene.weights, &catalogs) > 0.0).collect();
        self.brush_collision_free_until(target_full_id, &compatible, scene.contact_tolerance, resume_from, prior_free, deadline_us)
    }

    #[cfg(test)]
    pub(crate) fn work_pending_for_test(&self) -> usize {
        self.brush_queue.len()
    }

    /// 📈️ Monotone witness of precompute work ACTUALLY completed — resolved suggestion targets and
    /// reconciled broad-phase owners. The lane is cursorized, so a queue length is no longer a progress
    /// measure: preparing one brush target moves it from the preparation cursor INTO the queue.
    #[cfg(test)]
    pub(crate) fn precompute_progress_for_test(&self) -> usize {
        self.brush_cache.len() + self.brush_index.entry_len()
    }

    pub(crate) fn apply_brush_placement(&mut self, payload: &BrushPlacePayload) -> Option<Fixture> {
        let catalogs = self.scene.as_ref()?.kind_catalogs.as_ref()?.clone();
        let fixture = &self.scene.as_ref()?.fixture;
        let next = apply_brush_placement_to_fixture(fixture, payload, &catalogs);
        if next.objects.len() == fixture.objects.len() {
            return None;
        }
        if let Some(scene) = &mut self.scene {
            Arc::make_mut(scene).fixture = next.clone();
        }
        self.rebuild_queue();
        Some(next)
    }

    fn take_session(&mut self) -> Puzzle3dCollisionSession {
        Puzzle3dCollisionSession {
            scene: self.scene.take(),
            scene_synced: self.scene_synced.take(),
            meshes: std::mem::replace(&mut self.meshes, Arc::new(HashMap::new())),
            mesh_is_fallback: std::mem::take(&mut self.mesh_is_fallback),
            mesh_sources: std::mem::take(&mut self.mesh_sources),
            mesh_reupload_requests: std::mem::take(&mut self.mesh_reupload_requests),
            brush_cache: std::mem::take(&mut self.brush_cache),
            brush_queue: std::mem::take(&mut self.brush_queue),
            brush_prepare_object_cursor: self.brush_prepare_object_cursor,
            brush_prepare_vortex_cursor: self.brush_prepare_vortex_cursor,
            brush_queue_preparing: std::mem::take(&mut self.brush_queue_preparing),
            brush_index: std::mem::replace(&mut self.brush_index, CollisionSpatialIndex::new(BRUSH_INDEX_CELL_SIZE)),
            brush_index_owner: self.brush_index_owner,
            brush_index_sync: self.brush_index_sync.take(),
            brush_index_ready: std::mem::take(&mut self.brush_index_ready),
            brush_placed: std::mem::take(&mut self.brush_placed),
        }
    }

    fn install_session(&mut self, session: Puzzle3dCollisionSession) {
        self.scene = session.scene;
        self.scene_synced = session.scene_synced;
        self.meshes = session.meshes;
        self.mesh_is_fallback = session.mesh_is_fallback;
        self.mesh_sources = session.mesh_sources;
        self.mesh_reupload_requests = session.mesh_reupload_requests;
        self.brush_cache = session.brush_cache;
        self.brush_queue = session.brush_queue;
        self.brush_prepare_object_cursor = session.brush_prepare_object_cursor;
        self.brush_prepare_vortex_cursor = session.brush_prepare_vortex_cursor;
        self.brush_queue_preparing = session.brush_queue_preparing;
        self.brush_index = session.brush_index;
        self.brush_index_owner = session.brush_index_owner;
        self.brush_index_sync = session.brush_index_sync;
        self.brush_index_ready = session.brush_index_ready;
        self.brush_placed = session.brush_placed;
    }
}

/// 🧵️ One document instance's collision engine: the scene it was last synced from, the registered mesh
/// geometry, the suggestion cache and the persistent broad phase. This is everything the app's session
/// slot carries across a dispatch or a worker hop.
pub(crate) struct Puzzle3dCollisionSession {
    scene: Option<Arc<SceneConfig>>,
    scene_synced: Option<Arc<SceneConfig>>,
    meshes: Arc<HashMap<String, CollisionBody>>,
    mesh_is_fallback: HashMap<String, bool>,
    mesh_sources: HashMap<String, CollisionMeshSource>,
    mesh_reupload_requests: Vec<String>,
    brush_cache: HashMap<String, BrushCollisionFreeResult>,
    brush_queue: VecDeque<String>,
    brush_prepare_object_cursor: usize,
    brush_prepare_vortex_cursor: usize,
    brush_queue_preparing: bool,
    brush_index: CollisionSpatialIndex,
    brush_index_owner: CollisionIndexOwner,
    brush_index_sync: Option<BrushIndexSync>,
    brush_index_ready: bool,
    brush_placed: HashMap<String, PlacedCollisionEntry>,
}

impl Puzzle3dCollisionSession {
    /// 🧮️ Resident bytes this slot keeps alive, so the app's session registry can bound the whole
    /// process against a census instead of trusting a slot count.
    pub(crate) fn bytes(&self) -> usize {
        let meshes = self.mesh_sources.values().map(|mesh| mesh.url.len().saturating_add(mesh.positions.len().saturating_mul(4)).saturating_add(mesh.indices.len().saturating_mul(4))).fold(0_usize, usize::saturating_add);
        self.scene_synced
            .as_ref()
            .map_or(0, |scene| scene.fixture.objects.len().saturating_mul(size_of::<crate::standards::v1::subsets::any::schema::FixtureObject>()))
            .saturating_add(meshes)
            .saturating_add(self.brush_placed.len().saturating_mul(size_of::<PlacedCollisionEntry>()))
            .saturating_add(self.brush_index.entry_len().saturating_mul(size_of::<CollisionAabb>()))
    }
}
//#endregion 🔖️Engine

//#region 🔖️Session
pub struct Puzzle3dPrecomputeSession {
    engine: Puzzle3dCollision,
}

impl Default for Puzzle3dPrecomputeSession {
    fn default() -> Self {
        Self::new()
    }
}

impl Puzzle3dPrecomputeSession {
    #[cfg(test)]
    /// 📊️ See [`Puzzle3dCollision::brush_candidate_cache_len`].
    pub(crate) fn brush_candidate_cache_len(&self) -> usize {
        self.engine.brush_candidate_cache_len()
    }

    pub fn new() -> Self {
        Self { engine: Puzzle3dCollision::new() }
    }

    pub fn set_scene(&mut self, json: &str) -> Result<(), Puzzle3dError> {
        self.set_scene_config(dsl::os_pack::json::from_json_str(json)?);
        Ok(())
    }

    /// 🧊️ Typed sync — see [`Puzzle3dCollision::set_scene_config`].
    pub(crate) fn set_scene_config(&mut self, scene: SceneConfig) {
        self.engine.set_scene_config(scene);
    }

    pub fn register_mesh(&mut self, url: &str, positions: &[f32], indices: &[u32]) {
        self.engine.register_mesh(url.to_string(), positions, indices);
    }

    pub fn register_mesh_fallback(&mut self, url: &str, positions: &[f32], indices: &[u32]) {
        self.engine.register_mesh_fallback(url.to_string(), positions, indices);
    }

    /// 🧩️ Admits one page of a client's mesh upload run and installs the geometry the moment the run
    /// closes — the paged half of the `registerBrushMesh` wire contract, which is how a document-scale
    /// GLB reaches this session at all: one whole mesh never fitted in the shared retained command's
    /// 8 192 raw bytes. `Ok(None)` is a closed, digest-verified, installed mesh; `Ok(Some(next))` is an
    /// open run waiting for page `next`. See [`stage_brush_mesh_page`].
    pub fn stage_mesh_page(&mut self, url: &str, digest: &str, page: u32, page_count: u32, positions: &[f32], indices: &[u32]) -> Result<Option<u32>, Puzzle3dMeshUploadFault> {
        // 🪢️ A run that opens on geometry this process ALREADY derived — under this id or any other id
        // with the same digest — closes on its first page and the remaining `pageCount - 1` commands are
        // never needed. Checked at page 0 only, so a live run costs no store lookup per page.
        if page == 0 && !digest.is_empty() && self.adopt_shared_mesh(url, Some(digest)) {
            return Ok(None);
        }
        match stage_brush_mesh_page(url, digest, page, page_count, positions, indices)? {
            Puzzle3dMeshUploadStep::Staged { next_page, .. } => Ok(Some(next_page)),
            Puzzle3dMeshUploadStep::Complete(positions, indices) => {
                self.register_mesh(url, &positions, &indices);
                Ok(None)
            }
        }
    }

    /// 🧵️ Hands the brush lane out to the app's session slot, leaving this engine empty — the check-in
    /// half of the per-instance session.
    pub(crate) fn take_collision_session(&mut self) -> Puzzle3dCollisionSession {
        self.engine.take_session()
    }

    /// 🧵️ Adopts a brush lane a previous call (possibly on another worker) left in the session slot.
    pub(crate) fn install_collision_session(&mut self, session: Puzzle3dCollisionSession) {
        self.engine.install_session(session);
    }

    /// 🥽️ Installs real geometry for one mesh identity out of the process-wide derived-mesh store —
    /// the id-only wire path, so a mesh uploaded once serves every document instance.
    pub fn adopt_shared_mesh(&mut self, url: &str, digest: Option<&str>) -> bool {
        self.engine.adopt_shared_mesh(url, digest)
    }

    /// 🚚️ Turns a refused id-only announcement into a standing request for the bytes — see
    /// [`Puzzle3dCollision::request_mesh_reupload`]. Answers whether the standing set actually GREW, which
    /// is the only case whose world body has anything new to say.
    pub fn request_mesh_reupload(&mut self, url: &str) -> bool {
        self.engine.request_mesh_reupload(url)
    }

    /// 🚚️ The mesh ids this session is waiting on bytes for, stable-ordered — see
    /// [`Puzzle3dCollision::mesh_reupload_requests`].
    pub fn mesh_reupload_requests(&self) -> &[String] {
        self.engine.mesh_reupload_requests()
    }

    pub fn has_mesh(&self, url: &str) -> bool {
        self.engine.has_mesh(url)
    }

    pub fn precompute_step(&mut self, budget: u32) -> bool {
        self.engine.precompute_step(budget)
    }

    pub fn enqueue_brush_target(&mut self, vortex_full_id: &str) {
        self.engine.enqueue_brush_target(vortex_full_id);
    }

    pub fn invalidate_brush_target(&mut self, vortex_full_id: &str) {
        self.engine.invalidate_brush_target(vortex_full_id);
    }

    pub fn refresh_brush_candidates(&mut self, vortex_full_id: &str) {
        self.engine.refresh_brush_candidates(vortex_full_id);
    }

    /// 🎯️ Typed readout — was a JSON string before the headless-engine-law fix; the app now reads
    /// `.free`/`.unknown_pending` directly.
    pub fn brush_candidates(&self, vortex_full_id: &str) -> BrushCollisionFreeResult {
        self.engine.brush_cache.get(vortex_full_id).cloned().unwrap_or(BrushCollisionFreeResult { free: vec![], unknown_pending: true, resume_candidate_index: 0 })
    }

    pub fn brush_preview(&self, vortex_full_id: &str, candidate_index: usize) -> Option<BrushPreviewState> {
        self.engine.brush_preview(vortex_full_id, candidate_index)
    }

    /// 🎯️ Single typed entry point for every mutating engine action — the headless replacement for the
    /// old per-action `apply_brush_placement_json`/`update_kind_weights`/`brush_preview_json` wasm-bindgen
    /// methods. Each arm calls the SAME underlying typed `Puzzle3dCollision` method those JSON wrappers
    /// always delegated to — no reimplementation.
    pub fn dispatch(&mut self, command: Puzzle3dEngineCommand) -> Result<Puzzle3dEngineOutcome, Puzzle3dError> {
        match command {
            Puzzle3dEngineCommand::SetScene { scene } => {
                self.set_scene_config(scene);
                Ok(Puzzle3dEngineOutcome::Unit)
            }
            Puzzle3dEngineCommand::ApplyBrushPlacement { payload } => {
                let fixture = self.engine.apply_brush_placement(&payload).ok_or(Puzzle3dError::BrushPlacementRejected)?;
                Ok(Puzzle3dEngineOutcome::Fixture(fixture))
            }
            Puzzle3dEngineCommand::UpdateKindWeights { object_weights, vortex_weights } => {
                self.engine.update_kind_weights(object_weights, vortex_weights);
                Ok(Puzzle3dEngineOutcome::Unit)
            }
            Puzzle3dEngineCommand::BrushPreview { vortex_full_id, candidate_index } => Ok(Puzzle3dEngineOutcome::BrushPreview(self.engine.brush_preview(&vortex_full_id, candidate_index as usize))),
        }
    }
}
//#endregion 🔖️Session

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
