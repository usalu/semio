//! 🥽️ Brush meshes: the process-wide content-addressed derived-mesh store (the `puzzle3d.mesh-decode` compute engine) and the paged `registerBrushMesh` upload staging.

use super::*;

//#region 🥽️SharedBrushMeshes
/// 🥽️ Content-addressed brush-mesh decode kernel — a pure `(url, positions, indices)` bytes-in,
/// validated-geometry-bytes-out compute, so one mesh identity decodes exactly once per process and
/// every document instance reads the identical derived page instead of re-uploading it. This is the
/// kernel the WIT `engine-derive`/`engine-read` host route registers once it is threaded through
/// exchange (`🧰️framework/🔨️modules/◻️2d/🧮️compute/🦀️.rs`), with no plugin-side change.
struct Puzzle3dMeshDecodeEngine;

/// ⚖️ Byte budget of the derived-geometry LRU: `COLLISION_MESH_MAX_MESHES` document-scale meshes at the
/// `COLLISION_MESH_MAX_VALUES` ceiling, four bytes per value, halved because positions and indices
/// never both saturate.
const BRUSH_MESH_CACHE_BYTES: usize = COLLISION_MESH_MAX_MESHES * COLLISION_MESH_MAX_VALUES * 4;

impl semio_framework_2d_compute::Engine for Puzzle3dMeshDecodeEngine {
    const ENGINE_ID: &'static str = "puzzle3d.mesh-decode";

    fn compute(&self, input: &[u8]) -> Result<Vec<u8>, semio_framework_2d_compute::EngineFault> {
        let geometry = brush_mesh_request_geometry(input).ok_or_else(|| semio_framework_2d_compute::EngineFault::InvalidInput("puzzle3d brush mesh request".into()))?;
        let (positions, indices) = decode_brush_mesh_geometry(geometry).ok_or_else(|| semio_framework_2d_compute::EngineFault::InvalidInput("puzzle3d brush mesh geometry".into()))?;
        brush_mesh_geometry_is_admissible(&positions, &indices).then(|| geometry.to_vec()).ok_or_else(|| semio_framework_2d_compute::EngineFault::Compute("puzzle3d brush mesh geometry is not a closed indexed triangle page".into()))
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
    engines: semio_framework_2d_compute::EngineCache,
    handles: HashMap<String, semio_framework_2d_compute::EngineHandle>,
    digests: HashMap<String, semio_framework_2d_compute::EngineHandle>,
}

fn brush_mesh_store() -> &'static Mutex<Puzzle3dBrushMeshStore> {
    static STORE: OnceLock<Mutex<Puzzle3dBrushMeshStore>> = OnceLock::new();
    STORE.get_or_init(|| {
        let mut engines = semio_framework_2d_compute::EngineCache::new(BRUSH_MESH_CACHE_BYTES);
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
    let handle = store.engines.derive(<Puzzle3dMeshDecodeEngine as semio_framework_2d_compute::Engine>::ENGINE_ID, &request).ok()?;
    let geometry = store.engines.read(&handle).ok()?;
    store.handles.insert(url.to_string(), handle.clone());
    store.digests.insert(digest, handle);
    BRUSH_MESH_INSTALLS.fetch_add(1, Ordering::Relaxed);
    decode_brush_mesh_geometry(&geometry)
}

/// 📖 Geometry this process already derived for one mesh id, or `None` once the LRU evicted it — a
/// miss is a cache miss, never a correctness change: the client is asked for the bytes again.
pub fn shared_brush_mesh(url: &str) -> Option<(Vec<f32>, Vec<u32>)> {
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
pub fn adopt_brush_mesh_by_digest(url: &str, digest: &str) -> Option<(Vec<f32>, Vec<u32>)> {
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
pub(super) const PUZZLE3D_MESH_UPLOAD_SLOTS: usize = 4;

/// 🧮️ Longest page run one mesh identity may claim: both arrays at the engine's own
/// [`COLLISION_MESH_MAX_VALUES`] ceiling, paged at [`PUZZLE3D_MESH_PAGE_VALUES`]. The largest mesh
/// this repo ships — `🧊️placeholder.glb`, 25 344 positions plus 48 384 indices — is 72 pages.
pub(super) const PUZZLE3D_MESH_UPLOAD_MAX_PAGES: u32 = (COLLISION_MESH_MAX_VALUES * 2).div_ceil(PUZZLE3D_MESH_PAGE_VALUES) as u32;

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
pub(super) struct Puzzle3dMeshUploadSlot {
    pub(super) url: String,
    pub(super) digest: String,
    pub(super) page_count: u32,
    pub(super) next_page: u32,
    pub(super) positions: Vec<f32>,
    pub(super) indices: Vec<u32>,
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
