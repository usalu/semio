//! 🧱️ World3d scene resources and asset io: pools, chunking, LOD grid, environment, terrain, placeholder meshes and asset requests.
//!
//! Crate `semio-framework-os-infinite-world-scene`; a downward producer of the infinite world family (wasi-excluded except tests).

#![cfg(any(test, not(all(target_arch = "wasm32", target_env = "p2"))))]

use semio_framework_os_infinite_world_model::{World3dBuildContext, World3dMeshAsset, World3dMeshAssetRejected, WORLD_DYNAMIC_ID_BYTE_CAPACITY, WORLD_DYNAMIC_MESH_CAPACITY, WorldDynamicFault, WorldDynamicRejected, WorldEnvironmentRecord, WorldInteractionState, WorldMeshRegistry, WorldReferenceRecord, WorldSessionState, WorldTerrainStyle};
use semio_framework_os_infinite_world_components::{placeholder_normalize3, placeholder_scale3, TerrainTileMeshPayload, WORLD_REFERENCE_SOURCE_PIXELS, WorldBrushMeshRun, WorldCameraState, WorldMeshBuffers, WorldPlaceholderOwner, WorldSceneState, WorldSelectionState};
use ui_wgpu::wgpu::{ActionDescriptor, Camera3d, camera_grid_fade_distance, Instance3d, LineVertex3d, lod_from_camera_distance, lod_grid_fade_alpha, lod_grid_step_world, lod_orbit_distance_for_camera, Mat4, mesh3d_abort, mesh3d_abort_step, mesh3d_allocate_step, mesh3d_begin, mesh3d_begin_close, mesh3d_close_step, mesh3d_seal, mesh3d_write_u32, mesh3d_write_vec3, mesh3d_write_vec4, Mesh3dField, Mesh3dLease, Mesh3dSchema, Mesh3dWriteToken, Rect, Rgba, SceneColorSource3d, SceneDraw3d, Theme, Vec3};
use serde::Deserialize;
use std::collections::{HashMap, HashSet};
#[cfg(not(all(target_arch = "wasm32", target_env = "p2")))]
use ui_wgpu::wgpu::{directional_shadow_view_projection, ICON_SHADOW_MAP_SIZE, SceneMaterial3d, SceneMaterialDraw3d, SceneShadow3d, SceneShadowRole3d, WORLD_SHADOW_MAP_SIZE};
use std::sync::{LazyLock, Mutex};
use semio_framework_value::{RetainedCloneGrant, RetainedCloneProgress};
use semio_framework_surface_terrain::TerrainSessionCore;
use serde_json::json;
#[cfg(any(test, feature = "testkit"))]
use ui_wgpu::wgpu::pick_closest_mesh_url;
use ui_styling::metrics::tool_run;

/// 🧩️ World3d terrain component: terrain style, tiles, session and build.
pub struct WorldTerrainState {

    pub terrain_style: Option<WorldTerrainStyle>,

    pub terrain_applied_signature: Option<(String, f64, f64, f64)>,

    pub terrain_session: TerrainSessionCore,

    pub terrain_visible_tiles: HashSet<(u32, u32, u32)>,

    pub terrain_built_tiles: HashSet<(u32, u32, u32)>,

    pub pending_terrain_tile_urls: HashMap<String, (u32, u32, u32)>,

    pub terrain_tile_misses: HashSet<(u32, u32, u32)>,

    pub terrain_build: Option<WorldTerrainMeshCursor>,

    pub terrain_pending: Option<((u32,u32,u32),TerrainTileMeshPayload,u64,u64,u64)>,

    pub terrain_revision: u64,
}

/// 🧩️ World3d assets component: asset io authority, url bookkeeping and brush mesh announce.
pub struct WorldAssetsState {

    pub pending_glb_urls: HashSet<String>,

    pub pending_image_urls: HashSet<String>,

    /// 🥽️ Mesh url → the [`world_brush_mesh_revision`] THIS surface last announced it to the guest
    /// at. The wgpu twin of the `revision` prop React hands each mounted `BrushMeshRegistrar`: a
    /// surface announces an identity once per revision, and a surface that is rebuilt (window
    /// reopen, role switch) starts empty and announces again — see
    /// [`step_world3d_brush_mesh_announce`].
    pub brush_mesh_announced: HashMap<String, u64>,

    /// 🥽️ The ONE `registerBrushMesh` page run this surface has in flight. One page leaves per
    /// frame, so a user action never queues behind more than a single page — React's
    /// `drainPuzzle3dBrushMeshQueue` back pressure, expressed as a frame cursor.
    pub brush_mesh_run: Option<WorldBrushMeshRun>,

    pub brush_mesh_feedback_digest: Option<u64>,

    pub asset_url_misses: HashSet<String>,

    pub asset_generation: u64,

    pub asset_io: WorldAssetIoAuthority,
}

impl WorldTerrainState {
    pub fn new() -> Self {
        Self {
            terrain_style: None,
            terrain_applied_signature: None,
            terrain_session: TerrainSessionCore::default(),
            terrain_visible_tiles: HashSet::new(),
            terrain_built_tiles: HashSet::new(),
            pending_terrain_tile_urls: HashMap::new(),
            terrain_tile_misses: HashSet::new(),
            terrain_build: None,
            terrain_pending: None,
            terrain_revision: 0,
        }
    }
}

impl WorldAssetsState {
    pub fn new() -> Self {
        Self {
            pending_glb_urls: HashSet::new(),
            pending_image_urls: HashSet::new(),
            brush_mesh_announced: HashMap::new(),
            brush_mesh_run: None,
            brush_mesh_feedback_digest: None,
            asset_url_misses: HashSet::new(),
            asset_generation: 0,
            asset_io: WorldAssetIoAuthority::default(),
        }
    }
}

/// 🔏️ FNV-1a over the payload lanes that decide whether a bridge rebuild is needed. Content
/// addressing, not a clock: an app that republishes an identical scene must not cost a rebuild.
pub fn world3d_scene_digest(parts: &[&str]) -> u64 {
    let mut hash = 0xcbf2_9ce4_8422_2325u64;
    for part in parts {
        for byte in part.as_bytes() {
            hash ^= u64::from(*byte);
            hash = hash.wrapping_mul(0x0000_0100_0000_01b3);
        }
        hash ^= 0xff;
        hash = hash.wrapping_mul(0x0000_0100_0000_01b3);
    }
    hash
}

/// 🖼️ Whether the active document still owns `url`; a decoded response must answer this in addition
/// to the pane-generation guard because the same live pane may replace its reference while bytes are
/// in flight.
pub fn world3d_reference_url_is_current(scene_state: &WorldSceneState, url: &str) -> bool {
    scene_state.references.iter().any(|reference| reference.url.as_deref() == Some(url))
}
//#endregion 🌉️World3dSceneBridge

//#region 📡️WorldAssetIoAuthority
pub const WORLD_ASSET_REQUEST_CAPACITY: usize = 64;

pub const WORLD_ASSET_URL_BYTE_CAPACITY: usize = 2_048;

pub const WORLD_ASSET_RESPONSE_PAGE_BYTES: usize = 16 * 1024;

pub const WORLD_ASSET_RESPONSE_PAGE_CAPACITY: usize = 1_024;

pub const WORLD_ASSET_RESPONSE_BYTE_CAPACITY: usize = WORLD_ASSET_RESPONSE_PAGE_BYTES * WORLD_ASSET_RESPONSE_PAGE_CAPACITY;

pub const WORLD_ASSET_METADATA_ID_BYTES: usize = 256;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct WorldAssetMetadataId {
    pub bytes: [u8; WORLD_ASSET_METADATA_ID_BYTES],
    pub len: u16,
}

impl WorldAssetMetadataId {
    pub fn try_from_str(value: &str) -> Result<Self, WorldAssetFault> {
        if value.len() > WORLD_ASSET_METADATA_ID_BYTES {
            return Err(WorldAssetFault::MetadataCapacity);
        }
        let mut bytes = [0; WORLD_ASSET_METADATA_ID_BYTES];
        bytes[..value.len()].copy_from_slice(value.as_bytes());
        Ok(Self { bytes, len: value.len() as u16 })
    }

    pub fn as_str(&self) -> &str {
        std::str::from_utf8(&self.bytes[..usize::from(self.len)]).expect("asset metadata originates from UTF-8")
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[expect(clippy::large_enum_variant, reason = "Request metadata uses fixed-capacity inline identifiers so copying a request kind does not allocate.")]
pub enum WorldAssetRequestKind {
    Glb,
    ReferenceImage,
    Terrain { z: u32, x: u32, y: u32 },
    MapTile { surface: WorldAssetMetadataId, key: WorldAssetMetadataId, vector: bool, z: u32, x: u32, y: u32 },
    UiImage { id: WorldAssetMetadataId },
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct WorldAssetRequestToken {
    pub slot: u8,
    pub epoch: u64,
    pub generation: u64,
    pub revision: u64,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WorldAssetFault {
    Closing,
    ItemCapacity,
    ByteCapacity,
    UrlCapacity,
    MetadataCapacity,
    PageCapacity,
    Stale,
    Incomplete,
}

#[derive(Debug)]
pub struct WorldAssetResponsePage {
    pub bytes: Box<[u8]>,
}

impl WorldAssetResponsePage {
    pub fn try_from_owned(bytes: Vec<u8>) -> Result<Self, Vec<u8>> {
        if bytes.len() > WORLD_ASSET_RESPONSE_PAGE_BYTES {
            return Err(bytes);
        }
        Ok(Self { bytes: bytes.into_boxed_slice() })
    }

    pub fn bytes(&self) -> &[u8] {
        &self.bytes
    }
}

#[derive(Debug)]
pub struct WorldAssetFetchOwner {
    pub token: WorldAssetRequestToken,
    pub generation: u64,
    pub revision: u64,
    pub kind: WorldAssetRequestKind,
    pub url: String,
    pub reserved_bytes: usize,
    received_bytes: usize,
    pub pages: Box<[Option<WorldAssetResponsePage>; WORLD_ASSET_RESPONSE_PAGE_CAPACITY]>,
    pub page_len: u16,
    page_read: u16,
    close_page: u16,
    sealed: bool,
    pub closing: bool,
}

impl WorldAssetFetchOwner {
    pub fn token(&self) -> WorldAssetRequestToken {
        self.token
    }

    pub fn generation(&self) -> u64 {
        self.generation
    }

    pub fn revision(&self) -> u64 {
        self.revision
    }

    pub fn url(&self) -> &str {
        &self.url
    }

    pub fn kind(&self) -> WorldAssetRequestKind {
        self.kind
    }

    pub fn push_page(&mut self, page: WorldAssetResponsePage) -> Result<(), WorldAssetResponsePage> {
        let next = self.received_bytes.saturating_add(page.bytes.len());
        if self.closing || self.sealed || usize::from(self.page_len) == WORLD_ASSET_RESPONSE_PAGE_CAPACITY || next > self.reserved_bytes {
            return Err(page);
        }
        self.pages[usize::from(self.page_len)] = Some(page);
        self.page_len += 1;
        self.received_bytes = next;
        Ok(())
    }

    pub fn seal(&mut self) -> Result<(), WorldAssetFault> {
        if self.closing {
            return Err(WorldAssetFault::Closing);
        }
        self.sealed = true;
        Ok(())
    }

    pub fn take_decode_page(&mut self) -> Result<Option<WorldAssetResponsePage>, WorldAssetFault> {
        if !self.sealed {
            return Err(WorldAssetFault::Incomplete);
        }
        if self.page_read == self.page_len {
            return Ok(None);
        }
        let page = self.pages[usize::from(self.page_read)].take().ok_or(WorldAssetFault::Stale)?;
        self.page_read += 1;
        Ok(Some(page))
    }

    pub fn decode_page(&self) -> Result<Option<&WorldAssetResponsePage>, WorldAssetFault> {
        if !self.sealed {
            return Err(WorldAssetFault::Incomplete);
        }
        if self.page_read == self.page_len {
            return Ok(None);
        }
        self.pages[usize::from(self.page_read)].as_ref().map(Some).ok_or(WorldAssetFault::Stale)
    }

    pub fn decode_page_at(&self, index: u16) -> Result<Option<&WorldAssetResponsePage>, WorldAssetFault> {
        if !self.sealed || self.closing {
            return Err(WorldAssetFault::Incomplete);
        }
        if index >= self.page_len {
            return Ok(None);
        }
        self.pages[usize::from(index)].as_ref().map(Some).ok_or(WorldAssetFault::Stale)
    }

    pub fn decode_page_len(&self) -> Result<u16, WorldAssetFault> {
        if !self.sealed || self.closing {
            return Err(WorldAssetFault::Incomplete);
        }
        Ok(self.page_len)
    }

    pub fn advance_decode_page(&mut self) -> Result<(), WorldAssetFault> {
        if self.decode_page()?.is_none() {
            return Err(WorldAssetFault::Incomplete);
        }
        self.page_read += 1;
        Ok(())
    }

    pub fn rewind_decode_pages(&mut self) -> Result<(), WorldAssetFault> {
        if !self.sealed || self.closing {
            return Err(WorldAssetFault::Incomplete);
        }
        self.page_read = 0;
        Ok(())
    }

    pub fn received_bytes(&self) -> usize {
        self.received_bytes
    }

    /// 🔏️ Whether this response already sealed its exact byte claim. A host that must RETRY a seal —
    /// because the surface's own state was momentarily checked out by an apply — reads this to skip
    /// the half it already completed; sealing twice is refused by the authority's own witness.
    pub fn is_sealed(&self) -> bool {
        self.sealed
    }

    pub fn begin_close(&mut self) {
        self.closing = true;
    }

    pub fn close_step(&mut self) -> bool {
        self.closing = true;
        if self.close_page < self.page_len {
            self.pages[usize::from(self.close_page)] = None;
            self.close_page += 1;
            return false;
        }
        if !self.url.is_empty() {
            self.url.clear();
            return false;
        }
        self.page_len = 0;
        self.page_read = 0;
        self.close_page = 0;
        self.received_bytes = 0;
        self.reserved_bytes = 0;
        true
    }

    pub fn terminal_is_empty(&self) -> bool {
        self.url.is_empty() && self.pages.iter().all(Option::is_none) && self.reserved_bytes == 0 && self.received_bytes == 0
    }
}

#[cfg(not(any(test, feature = "testkit")))]
impl Drop for WorldAssetFetchOwner {
    fn drop(&mut self) {
        assert!(self.terminal_is_empty() || std::thread::panicking(), "WorldAssetFetchOwner reached Drop before every response page and request string reached terminal handback");
    }
}

struct WorldAssetClaim {
    pub epoch: u64,
    pub generation: u64,
    pub revision: u64,
    pub kind: WorldAssetRequestKind,
    url_len: u16,
    url_bytes: Box<[u8; WORLD_ASSET_URL_BYTE_CAPACITY]>,
    pub reserved_bytes: usize,
    in_flight: bool,
    fetch_complete: bool,
    pub cancelled: bool,
    pub owner: Option<WorldAssetFetchOwner>,
}

pub struct WorldAssetIoAuthority {
    pub slots: Box<[Option<WorldAssetClaim>; WORLD_ASSET_REQUEST_CAPACITY]>,
    pub epochs: [u64; WORLD_ASSET_REQUEST_CAPACITY],
    completed_cursor: u8,
    pub reserved_bytes: usize,
    pub closing: bool,
}

impl Default for WorldAssetIoAuthority {
    fn default() -> Self {
        Self { slots: semio_framework_async::boxed_fixed_slots(|| None), epochs: [0; WORLD_ASSET_REQUEST_CAPACITY], completed_cursor: 0, reserved_bytes: 0, closing: false }
    }
}

impl WorldAssetIoAuthority {
    pub fn reserve_request(&mut self, generation: u64, revision: u64, kind: WorldAssetRequestKind, url: &str) -> Result<WorldAssetRequestToken, WorldAssetFault> {
        self.reserve(generation, revision, kind, url, 0)
    }

    pub fn reserve(&mut self, generation: u64, revision: u64, kind: WorldAssetRequestKind, url: &str, byte_credits: usize) -> Result<WorldAssetRequestToken, WorldAssetFault> {
        if self.closing {
            return Err(WorldAssetFault::Closing);
        }
        if url.len() > WORLD_ASSET_URL_BYTE_CAPACITY {
            return Err(WorldAssetFault::UrlCapacity);
        }
        if let Some((slot, claim)) = self.slots.iter().enumerate().find_map(|(slot, claim)| {
            let claim = claim.as_ref()?;
            (!claim.cancelled && claim.kind == kind && usize::from(claim.url_len) == url.len() && &claim.url_bytes[..url.len()] == url.as_bytes()).then_some((slot, claim))
        }) {
            if claim.revision != revision {
                return Err(WorldAssetFault::Stale);
            }
            return Ok(WorldAssetRequestToken { slot: slot as u8, epoch: claim.epoch, generation: claim.generation, revision: claim.revision });
        }
        let next = self.reserved_bytes.checked_add(byte_credits).ok_or(WorldAssetFault::ByteCapacity)?;
        if byte_credits > WORLD_ASSET_RESPONSE_BYTE_CAPACITY || next > WORLD_ASSET_RESPONSE_BYTE_CAPACITY {
            return Err(WorldAssetFault::ByteCapacity);
        }
        let slot = self.slots.iter().position(Option::is_none).ok_or(WorldAssetFault::ItemCapacity)?;
        self.epochs[slot] = self.epochs[slot].wrapping_add(1).max(1);
        let token = WorldAssetRequestToken { slot: slot as u8, epoch: self.epochs[slot], generation, revision };
        let mut url_bytes = Box::new([0; WORLD_ASSET_URL_BYTE_CAPACITY]);
        url_bytes[..url.len()].copy_from_slice(url.as_bytes());
        let owner = WorldAssetFetchOwner {
            token,
            generation,
            revision,
            kind,
            url: url.to_owned(),
            reserved_bytes: byte_credits,
            received_bytes: 0,
            pages: semio_framework_async::boxed_fixed_slots(|| None),
            page_len: 0,
            page_read: 0,
            close_page: 0,
            sealed: false,
            closing: false,
        };
        self.slots[slot] = Some(WorldAssetClaim { epoch: token.epoch, generation, revision, kind, url_len: url.len() as u16, url_bytes, reserved_bytes: byte_credits, in_flight: false, fetch_complete: false, cancelled: false, owner: Some(owner) });
        self.reserved_bytes = next;
        Ok(token)
    }

    pub fn take_next(&mut self) -> Option<WorldAssetFetchOwner> {
        let claim = self.slots.iter_mut().flatten().find(|claim| !claim.cancelled && !claim.in_flight && !claim.fetch_complete && claim.owner.is_some())?;
        claim.in_flight = true;
        claim.owner.take()
    }

    pub fn reserve_response(&mut self, owner: &mut WorldAssetFetchOwner, byte_credits: usize) -> Result<(), WorldAssetFault> {
        let slot = usize::from(owner.token.slot);
        let claim = self.slots.get_mut(slot).and_then(Option::as_mut).ok_or(WorldAssetFault::Stale)?;
        if claim.cancelled || claim.epoch != owner.token.epoch || claim.generation != owner.generation || claim.revision != owner.revision || !claim.in_flight || claim.reserved_bytes != 0 || owner.reserved_bytes != 0 {
            return Err(WorldAssetFault::Stale);
        }
        let next = self.reserved_bytes.checked_add(byte_credits).ok_or(WorldAssetFault::ByteCapacity)?;
        if byte_credits > WORLD_ASSET_RESPONSE_BYTE_CAPACITY || next > WORLD_ASSET_RESPONSE_BYTE_CAPACITY {
            return Err(WorldAssetFault::ByteCapacity);
        }
        claim.reserved_bytes = byte_credits;
        owner.reserved_bytes = byte_credits;
        self.reserved_bytes = next;
        Ok(())
    }

    #[expect(clippy::result_large_err, reason = "Rejected transfer returns the exact admitted owner for bounded retirement without allocating on the failure path.")]
    pub fn return_owner(&mut self, mut owner: WorldAssetFetchOwner) -> Result<(), WorldAssetFetchOwner> {
        let slot = usize::from(owner.token.slot);
        let Some(claim) = self.slots.get_mut(slot).and_then(Option::as_mut) else {
            return Err(owner);
        };
        if claim.epoch != owner.token.epoch || claim.generation != owner.generation || claim.revision != owner.revision || !claim.in_flight || claim.owner.is_some() {
            return Err(owner);
        }
        if claim.cancelled {
            owner.begin_close();
        }
        claim.in_flight = false;
        claim.fetch_complete = owner.sealed || owner.closing;
        claim.owner = Some(owner);
        Ok(())
    }

    pub fn seal_response(&mut self, owner: &mut WorldAssetFetchOwner) -> Result<(), WorldAssetFault> {
        let slot = usize::from(owner.token.slot);
        let claim = self.slots.get_mut(slot).and_then(Option::as_mut).ok_or(WorldAssetFault::Stale)?;
        if claim.cancelled
            || claim.epoch != owner.token.epoch
            || claim.generation != owner.generation
            || claim.revision != owner.revision
            || !claim.in_flight
            || claim.reserved_bytes != owner.reserved_bytes
            || owner.received_bytes > owner.reserved_bytes
        {
            return Err(WorldAssetFault::Stale);
        }
        owner.seal()?;
        let unused = owner.reserved_bytes - owner.received_bytes;
        owner.reserved_bytes = owner.received_bytes;
        claim.reserved_bytes = owner.received_bytes;
        self.reserved_bytes -= unused;
        Ok(())
    }

    pub fn take_completed(&mut self, token: WorldAssetRequestToken, generation: u64, revision: u64) -> Result<WorldAssetFetchOwner, WorldAssetFault> {
        let slot = usize::from(token.slot);
        let claim = self.slots.get_mut(slot).and_then(Option::as_mut).ok_or(WorldAssetFault::Stale)?;
        if claim.cancelled || claim.epoch != token.epoch || claim.generation != generation || claim.revision != revision || claim.in_flight || !claim.fetch_complete || claim.owner.as_ref().is_none_or(|owner| !owner.sealed) {
            return Err(WorldAssetFault::Stale);
        }
        claim.in_flight = true;
        Ok(claim.owner.take().expect("validated completed asset owner"))
    }

    /// 📡️ Whether any claim holds a sealed response the decode pump has not taken yet — the
    /// non-consuming twin of [`Self::take_next_completed_step`]. See also
    /// [`WorldAssetFetchOwner::is_sealed`].
    pub fn has_completed_step(&self) -> bool {
        self.slots.iter().flatten().any(|claim| !claim.cancelled && !claim.in_flight && claim.fetch_complete && claim.owner.as_ref().is_some_and(|owner| owner.sealed))
    }

    pub fn take_next_completed_step(&mut self) -> Option<WorldAssetFetchOwner> {
        let slot = usize::from(self.completed_cursor);
        self.completed_cursor = ((slot + 1) % WORLD_ASSET_REQUEST_CAPACITY) as u8;
        let claim = self.slots[slot].as_mut()?;
        if claim.cancelled || claim.in_flight || !claim.fetch_complete || claim.owner.as_ref().is_none_or(|owner| !owner.sealed) {
            return None;
        }
        claim.in_flight = true;
        claim.owner.take()
    }

    #[expect(clippy::result_large_err, reason = "Rejected transfer returns the exact admitted owner for bounded retirement without allocating on the failure path.")]
    pub fn finish(&mut self, owner: WorldAssetFetchOwner) -> Result<(), WorldAssetFetchOwner> {
        let slot = usize::from(owner.token.slot);
        let Some(claim) = self.slots.get(slot).and_then(Option::as_ref) else {
            return Err(owner);
        };
        if claim.epoch != owner.token.epoch || !claim.in_flight || !claim.fetch_complete || !owner.terminal_is_empty() {
            return Err(owner);
        }
        let claim = self.slots[slot].take().expect("validated terminal asset claim");
        self.reserved_bytes -= claim.reserved_bytes;
        drop(owner);
        Ok(())
    }

    /// 🛑️ Cancels only the exact request generation, including an in-flight fetch.
    pub fn cancel_request(&mut self, token: WorldAssetRequestToken) -> bool {
        let Some(claim) = self.slots.get_mut(usize::from(token.slot)).and_then(Option::as_mut) else { return false };
        if claim.epoch != token.epoch || claim.generation != token.generation || claim.revision != token.revision {
            return false;
        }
        claim.cancelled = true;
        if !claim.in_flight {
            if let Some(owner) = claim.owner.as_mut() {
                owner.begin_close();
            }
            claim.fetch_complete = true;
        }
        true
    }

    pub fn cancellation_requested(&self, token: WorldAssetRequestToken) -> bool {
        if self.closing {
            return true;
        }
        self.slots.get(usize::from(token.slot)).and_then(Option::as_ref).is_none_or(|claim| claim.cancelled || claim.epoch != token.epoch || claim.generation != token.generation || claim.revision != token.revision)
    }

    pub fn cancel_reference_image_urls_except(&mut self, references: &[WorldReferenceRecord]) {
        for claim in self.slots.iter_mut().flatten() {
            if claim.kind != WorldAssetRequestKind::ReferenceImage {
                continue;
            }
            let url = std::str::from_utf8(&claim.url_bytes[..usize::from(claim.url_len)]).expect("asset URLs originate from UTF-8");
            if references.iter().any(|reference| reference.url.as_deref() == Some(url)) {
                continue;
            }
            claim.cancelled = true;
            if !claim.in_flight {
                if let Some(owner) = claim.owner.as_mut() {
                    owner.begin_close();
                }
                claim.fetch_complete = true;
            }
        }
    }

    /// 🗺️ Cancels every pending `MapTile` claim for one surface so a closed map returns its shared
    /// renderer-asset slots — without this, fixture paints that reserve visible tiles and then close
    /// the host leave the process-wide lane at `ItemCapacity` and later maps reserve nothing.
    pub fn cancel_map_tiles_for_surface(&mut self, surface: &str) {
        for claim in self.slots.iter_mut().flatten() {
            let WorldAssetRequestKind::MapTile { surface: claim_surface, .. } = claim.kind else { continue };
            if claim_surface.as_str() != surface {
                continue;
            }
            claim.cancelled = true;
            if !claim.in_flight {
                if let Some(owner) = claim.owner.as_mut() {
                    owner.begin_close();
                }
                claim.fetch_complete = true;
            }
        }
    }

    pub fn retire_cancelled_step(&mut self) -> bool {
        let Some(slot) = self.slots.iter().position(|claim| claim.as_ref().is_some_and(|claim| !claim.in_flight && claim.owner.as_ref().is_some_and(|owner| owner.closing))) else {
            return false;
        };
        let claim = self.slots[slot].as_mut().expect("cancelled asset claim");
        let owner = claim.owner.as_mut().expect("cancelled asset owner");
        if !owner.close_step() {
            return true;
        }
        self.reserved_bytes -= claim.reserved_bytes;
        self.slots[slot] = None;
        true
    }

    pub fn begin_close(&mut self) {
        self.closing = true;
    }

    pub fn close_step(&mut self) -> bool {
        self.closing = true;
        let Some(slot) = self.slots.iter().position(Option::is_some) else {
            let empty = self.reserved_bytes == 0;
            if empty {
                self.closing = false;
            }
            return empty;
        };
        let claim = self.slots[slot].as_mut().expect("asset close claim");
        if claim.in_flight {
            return false;
        }
        let owner = claim.owner.as_mut().expect("non-flight asset claim owns request");
        if !owner.close_step() {
            return false;
        }
        self.reserved_bytes -= claim.reserved_bytes;
        self.slots[slot] = None;
        if self.reserved_bytes == 0 && self.slots.iter().all(Option::is_none) {
            self.closing = false;
            return true;
        }
        false
    }

    pub fn terminal_is_empty(&self) -> bool {
        self.reserved_bytes == 0 && self.slots.iter().all(Option::is_none)
    }
}

pub fn reserve_world3d_asset(session_state: &mut WorldSessionState, assets_state: &mut WorldAssetsState, kind: WorldAssetRequestKind, url: &str, byte_credits: usize) -> Result<WorldAssetRequestToken, WorldAssetFault> {
    let generation = assets_state.asset_generation.wrapping_add(1).max(1);
    let token = assets_state.asset_io.reserve(generation, session_state.interaction_revision, kind, url, byte_credits)?;
    assets_state.asset_generation = assets_state.asset_generation.max(token.generation);
    Ok(token)
}

/// 🗑️ Records that the renderer's decoder REFUSED this url's bytes, so this surface stops offering
/// it. The mesh and reference-image request loops re-derive their missing urls from the scene every
/// frame, so without this ledger one refused response is re-fetched for the lifetime of the surface.
/// Sticky on purpose — the same shape as [`World3dState::terrain_tile_misses`] and as React's own
/// loader caches, where a failed texture url is not retried until the scene names a different one.
pub fn mark_world3d_asset_miss(assets_state: &mut WorldAssetsState, url: &str) {
    assets_state.asset_url_misses.insert(url.to_string());
    assets_state.pending_image_urls.remove(url);
    assets_state.pending_glb_urls.remove(url);
}

/// 🗑️ Whether [`mark_world3d_asset_miss`] refused this url for this surface.
pub fn world3d_asset_url_missed(assets_state: &WorldAssetsState, url: &str) -> bool {
    assets_state.asset_url_misses.contains(url)
}

pub fn reserve_world3d_asset_request(session_state: &mut WorldSessionState, assets_state: &mut WorldAssetsState, kind: WorldAssetRequestKind, url: &str) -> Result<WorldAssetRequestToken, WorldAssetFault> {
    let generation = assets_state.asset_generation.wrapping_add(1).max(1);
    let token = assets_state.asset_io.reserve_request(generation, session_state.interaction_revision, kind, url)?;
    assets_state.asset_generation = assets_state.asset_generation.max(token.generation);
    Ok(token)
}

pub fn take_next_world3d_asset(assets_state: &mut WorldAssetsState) -> Option<WorldAssetFetchOwner> {
    assets_state.asset_io.take_next()
}

pub fn reserve_world3d_asset_response(assets_state: &mut WorldAssetsState, owner: &mut WorldAssetFetchOwner, byte_credits: usize) -> Result<(), WorldAssetFault> {
    assets_state.asset_io.reserve_response(owner, byte_credits)
}

#[expect(clippy::result_large_err, reason = "Rejected transfer returns the exact admitted owner for bounded retirement without allocating on the failure path.")]
pub fn return_world3d_asset(assets_state: &mut WorldAssetsState, owner: WorldAssetFetchOwner) -> Result<(), WorldAssetFetchOwner> {
    assets_state.asset_io.return_owner(owner)
}

pub fn seal_world3d_asset_response(assets_state: &mut WorldAssetsState, owner: &mut WorldAssetFetchOwner) -> Result<(), WorldAssetFault> {
    assets_state.asset_io.seal_response(owner)
}

pub fn take_completed_world3d_asset(session_state: &mut WorldSessionState, assets_state: &mut WorldAssetsState, token: WorldAssetRequestToken) -> Result<WorldAssetFetchOwner, WorldAssetFault> {
    if token.revision != session_state.interaction_revision {
        return Err(WorldAssetFault::Stale);
    }
    assets_state.asset_io.take_completed(token, token.generation, token.revision)
}

pub fn take_next_completed_world3d_asset_step(assets_state: &mut WorldAssetsState) -> Option<WorldAssetFetchOwner> {
    assets_state.asset_io.take_next_completed_step()
}

pub fn begin_world3d_asset_close(assets_state: &mut WorldAssetsState) {
    assets_state.asset_io.begin_close();
}

pub fn close_world3d_asset_step(assets_state: &mut WorldAssetsState) -> bool {
    assets_state.asset_io.close_step() && assets_state.asset_io.terminal_is_empty()
}

/// 📡️🎞️ Whether this surface holds a fetched asset response the decode pump still owes a turn.
///
/// ⚖️ The decode ladder runs INSIDE the frame transaction, so on an event-driven shell it advances
/// only while frames happen. A GLB arrives long after the document that named it has settled — the
/// puzzle3d playground fetches its 86 KB concrete-forest mesh at t≈11.8 s, on the very last frame
/// the boot produces — and nothing then turned that sealed response into another frame: the mesh sat
/// undecoded for the rest of the session. This is the same per-frame predicate shape as
/// [`world3d_cursor_work_pending`]: the browser tick turns it into `request_frame`, so the work the
/// frame transaction drives keeps its own frames coming
/// (ticket 26/09/17/WGPU-RENDERER-REACT-PARITY, `📓️w3d-world3d-glb-url-lane.md`).
pub fn world3d_asset_decode_pending(assets_state: &WorldAssetsState) -> bool {
    assets_state.asset_io.has_completed_step()
}

#[expect(clippy::result_large_err, reason = "Rejected transfer returns the exact admitted owner for bounded retirement without allocating on the failure path.")]
pub fn finish_world3d_asset(assets_state: &mut WorldAssetsState, owner: WorldAssetFetchOwner) -> Result<(), WorldAssetFetchOwner> {
    assets_state.asset_io.finish(owner)
}

/// 📦️ Concatenates every page of a sealed world-asset response — used when a retained decode
/// finishes without materializing a typed mesh lease (reference images).
/// 📥️ The whole sealed response as ONE payload, for the kinds whose applier takes bytes rather than
/// a cursor (reference images, DEM tiles, UI images, map tiles).
///
/// 🩸️ This walked `decode_page()` in a `while let` and never advanced the cursor, so it re-read page 0
/// forever and grew the payload until `RawVec` refused a capacity past `isize::MAX` — a
/// `capacity overflow` panic that killed the whole frame Worker. It went unseen because every lane
/// that calls it was dead until 2026-09-17 (ticket 26/09/17/WGPU-RENDERER-REACT-PARITY, W1f/W2a and
/// `📓️w3a-asset-decoder-boot-fault.md`); the first real response it ever saw was the puzzle3d
/// playground's 483 496-byte reference plan. It is a BOUNDED walk over the sealed page index now, so
/// the loop's length is the response's own page count and cannot depend on a cursor at all.
pub fn collect_world3d_asset_bytes(owner: &mut WorldAssetFetchOwner) -> Result<Vec<u8>, WorldAssetFault> {
    let pages = owner.decode_page_len()?;
    let mut bytes = Vec::with_capacity(owner.received_bytes());
    for index in 0..pages {
        let Some(page) = owner.decode_page_at(index)? else { break };
        bytes.extend_from_slice(page.bytes());
    }
    owner.rewind_decode_pages()?;
    Ok(bytes)
}

pub fn retire_cancelled_world3d_asset_step(assets_state: &mut WorldAssetsState) -> bool {
    assets_state.asset_io.retire_cancelled_step()
}

pub fn world3d_asset_cancellation_requested(assets_state: &WorldAssetsState, token: WorldAssetRequestToken) -> bool {
    assets_state.asset_io.cancellation_requested(token)
}

#[cfg(not(any(test, feature = "testkit")))]
impl Drop for WorldAssetIoAuthority {
    fn drop(&mut self) {
        assert!(self.terminal_is_empty() || std::thread::panicking(), "WorldAssetIoAuthority reached Drop before every request claim reached terminal handback");
    }
}
//#endregion 📡️WorldAssetIoAuthority

pub fn mesh_id_from_url(url: &str) -> String {
    let slug = url.trim_start_matches('/').rsplit('/').next().unwrap_or(url).trim_end_matches(".glb").trim_end_matches(".gltf");
    format!("mesh:{slug}")
}

pub fn publish_world3d_inline_asset(session_state: &mut WorldSessionState, interaction_state: &mut WorldInteractionState, scene_state: &mut WorldSceneState,id:String,asset:World3dMeshAsset)->Result<(),World3dMeshAssetRejected> {publish_world3d_asset_by_id(session_state, interaction_state, scene_state,id,asset)}

pub fn publish_world3d_asset_mesh(session_state: &mut WorldSessionState, interaction_state: &mut WorldInteractionState, scene_state: &mut WorldSceneState, assets_state: &mut WorldAssetsState, url: &str, asset: World3dMeshAsset) -> Result<(), World3dMeshAssetRejected> {
    let result=publish_world3d_asset_by_id(session_state, interaction_state, scene_state,mesh_id_from_url(url),asset);
    if result.is_ok() {assets_state.pending_glb_urls.remove(url);}
    result
}

fn publish_world3d_asset_by_id(session_state: &mut WorldSessionState, interaction_state: &mut WorldInteractionState, scene_state: &mut WorldSceneState,id:String,asset:World3dMeshAsset)->Result<(),World3dMeshAssetRejected> {
    if asset.mesh.revision() != session_state.interaction_revision || scene_state.dynamic_mesh_close.is_some() || scene_state.dynamic_blocked_mesh.is_some() {
        return Err(World3dMeshAssetRejected { fault: if asset.mesh.revision() != session_state.interaction_revision { WorldDynamicFault::StaleToken } else { WorldDynamicFault::Closing }, id, value: asset });
    }
    let version = asset.mesh.generation().rotate_left(17) ^ asset.mesh.revision();
    let mesh_plan = match scene_state.meshes.plan_insert(&id) { Ok(plan) => plan, Err(fault) => return Err(World3dMeshAssetRejected { fault, id, value: asset }) };
    let version_plan = match scene_state.mesh_versions.plan_insert(&id) { Ok(plan) => plan, Err(fault) => return Err(World3dMeshAssetRejected { fault, id, value: asset }) };
    let appearance_plan = match scene_state.mesh_appearances.plan_insert(&id) { Ok(plan) => plan, Err(fault) => return Err(World3dMeshAssetRejected { fault, id, value: asset }) };
    let interaction_plan = match interaction_state.interaction_meshes.plan_admit(&id, version, asset.mesh) { Ok(plan) => plan, Err(fault) => return Err(World3dMeshAssetRejected { fault, id, value: asset }) };
    let World3dMeshAsset { mesh, appearance } = asset;
    let (_, previous_mesh) = scene_state.meshes.commit_insert(mesh_plan, id.clone(), mesh).expect("world material transaction revalidates its mesh slot");
    interaction_state.interaction_meshes.commit_admit(interaction_plan).expect("world material transaction revalidates its interaction slot");
    let (_, previous_version) = scene_state.mesh_versions.commit_insert(version_plan, id.clone(), version).expect("world material transaction revalidates its version slot");
    let (_, previous_appearance) = scene_state.mesh_appearances.commit_insert(appearance_plan, id, appearance).expect("world material transaction revalidates its appearance slot");
    drop(previous_version);
    drop(previous_appearance);
    if let Some(previous_mesh) = previous_mesh {
        scene_state.dynamic_mesh_close = Some(previous_mesh);
    }
    scene_state.geometry_generation = scene_state.geometry_generation.wrapping_add(1).max(1);
    Ok(())
}

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

struct WorldBrushMeshEntry {
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
struct WorldBrushMeshRegistry {
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
struct World3dBrushMeshFeedbackRecord {
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
pub const WORLD_REFERENCE_TEXTURE_BYTES: usize = ui_wgpu::wgpu::SCENE_RASTER_ITEM_BYTES;

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
pub fn world3d_style_paint(theme: &ui_wgpu::wgpu::Theme, style: MeshStyleState, has_vertex_colors: bool, mut instance: Instance3d) -> Instance3d {
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
struct WorldTranslucentMaterialItem {
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

//#region Environment
/// ☀️🎨️ Resolves the renderer's scene-pass light direction from `environment.sun` when the sun is
/// explicitly enabled (horizontal coordinate system, see https://en.wikipedia.org/wiki/Horizontal_coordinate_system,
/// matching `sunPositionFromAzimuthElevation` in `ui/js/react/index.tsx`), else keeps the default.
pub fn environment_light_dir(environment: &WorldEnvironmentRecord) -> [f32; 3] {
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

/// 🎨️ Resolves the neutral `MeshStandardMaterial` parameters. Environment emissive is a neutral
/// color override in World3d, so it is active only beside `material.color`; IconRender supplies its
/// own neutral default color whenever a material record exists.
pub fn environment_scene_material(environment: &WorldEnvironmentRecord) -> SceneMaterial3d {
    let Some(material) = environment.material.as_ref() else {
        return SceneMaterial3d::default();
    };
    let emissive = material.color.as_ref().and(material.emissive.as_deref()).map(parse_color).map(|rgba| rgba[..3].try_into().expect("fixed rgba prefix")).unwrap_or([0.0; 3]);
    SceneMaterial3d { metalness: material.metalness.unwrap_or(0.0) as f32, roughness: material.roughness.unwrap_or(1.0) as f32, emissive, emissive_intensity: if emissive == [0.0; 3] { 0.0 } else { material.emissive_intensity.unwrap_or(1.0) as f32 } }
}

pub fn environment_outline_color(environment: &WorldEnvironmentRecord, theme: &ui_wgpu::wgpu::Theme) -> Option<[f32; 4]> {
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
pub enum World3dShadowGeometry {
    Glb,
    Terrain,
    Paint,
}

#[cfg(not(all(target_arch = "wasm32", target_env = "p2")))]
pub fn world3d_shadow_role(geometry: World3dShadowGeometry, enabled: bool) -> SceneShadowRole3d {
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
    pub fn map_size(self) -> u32 {
        match self {
            Self::World | Self::IconSvg | Self::Unshadowed => WORLD_SHADOW_MAP_SIZE,
            Self::IconPng => ICON_SHADOW_MAP_SIZE,
        }
    }

    pub fn render_profile(self) -> ui_wgpu::wgpu::SceneRenderProfile3d {
        match self {
            Self::IconSvg => ui_wgpu::wgpu::SceneRenderProfile3d::SvgFlatLit,
            Self::World | Self::IconPng | Self::Unshadowed => ui_wgpu::wgpu::SceneRenderProfile3d::RasterPbr,
        }
    }
}

/// 🌑️ Resolves only the controls React consumes: enabled plus the consumer's map profile.
#[cfg(not(all(target_arch = "wasm32", target_env = "p2")))]
pub fn environment_scene_shadow(environment: &WorldEnvironmentRecord, light_dir: [f32; 3], profile: World3dShadowProfile) -> SceneShadow3d {
    let record = environment.shadow.as_ref();
    let enabled = matches!(profile, World3dShadowProfile::World | World3dShadowProfile::IconPng)
        && record.and_then(|shadow| shadow.enabled) == Some(true)
        && environment.sun.as_ref().and_then(|sun| sun.enabled) == Some(true);
    SceneShadow3d { enabled, map_size: profile.map_size(), view_proj: directional_shadow_view_projection(light_dir) }
}

//#region Terrain
#[derive(Deserialize, semio_framework_value_derive::ToValue, semio_framework_value_derive::FromValue)]
pub struct TerrainVisibleTileRow {
    pub z: u32,
    pub x: u32,
    pub y: u32,
}

pub fn terrain_tile_url(template: &str, z: u32, x: u32, y: u32) -> String {
    template.replace("{z}", &z.to_string()).replace("{x}", &x.to_string()).replace("{y}", &y.to_string())
}

pub fn terrain_tile_mesh_key(surface_id: &str, z: u32, x: u32, y: u32) -> String {
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
pub fn reserve_terrain_tile_fetch(session_state: &mut WorldSessionState, terrain_state: &mut WorldTerrainState, assets_state: &mut WorldAssetsState, tile_url_template: &str, (z, x, y): (u32, u32, u32)) {
    if terrain_state.terrain_tile_misses.contains(&(z, x, y)) {
        return;
    }
    let url = terrain_tile_url(tile_url_template, z, x, y);
    if terrain_state.pending_terrain_tile_urls.contains_key(&url) {
        return;
    }
    match reserve_world3d_asset_request(session_state, assets_state, WorldAssetRequestKind::Terrain { z, x, y }, &url) {
        Ok(_) => {
            terrain_state.pending_terrain_tile_urls.insert(url, (z, x, y));
        }
        Err(WorldAssetFault::ItemCapacity | WorldAssetFault::ByteCapacity | WorldAssetFault::Closing | WorldAssetFault::Stale) => {}
        Err(_) => {
            terrain_state.terrain_tile_misses.insert((z, x, y));
        }
    }
}

/// 🏔️📥️ Decodes one fetched DEM tile into the terrain session — the wgpu twin of
/// `TerrainTileRenderer.uploadOne`'s `upload_elevation_tile` call. A refusal (not a terrarium PNG)
/// records a tile miss so the pipeline stops re-requesting it; the banded meshes are built by the
/// next `sync_terrain_state` pass, which now sees a non-`null` `terrain_tile_mesh_json`.
pub fn apply_world3d_terrain_tile_bytes(terrain_state: &mut WorldTerrainState, z: u32, x: u32, y: u32, bytes: &[u8]) -> bool {
    let uploaded = terrain_state.terrain_session.upload_elevation_tile(z, x, y, bytes);
    if let Some(style) = terrain_state.terrain_style.as_ref() {
        terrain_state.pending_terrain_tile_urls.remove(&terrain_tile_url(&style.tile_url_template, z, x, y));
    }
    if !uploaded {
        terrain_state.terrain_tile_misses.insert((z, x, y));
    }
    uploaded
}

/// 🎨️ Vertical hypsometric ramp — same stops as `getHypsometricTexture` in `world-terrain-layer.tsx`
/// (green low ground -> tan -> grey -> white peaks), sampled at a band's center elevation ratio.
pub fn hypsometric_color(t: f32) -> [f32; 4] {
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
pub enum WorldTerrainMeshPhase {
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
    pub id: String,
    pub positions: Vec<f32>,
    pub normals: Vec<f32>,
    pub indices: Vec<u32>,
    pub uvs: Vec<f32>,
}

pub struct WorldTerrainMeshCursor {
    pub surface_id: String,
    pub z: u32,
    pub x: u32,
    pub y: u32,
    pub payload: TerrainTileMeshPayload,
    pub generation: u64,
    pub source_revision: u64,
    pub terrain_revision: u64,
    pub phase: WorldTerrainMeshPhase,
    pub triangle: u32,
    pub vertex: u8,
    pub item: u32,
    pub owner: WorldPlaceholderOwner,
    pub close_started: bool,
    pub source_retirement: Option<semio_framework_value::retirement::controlled::ControlledRetirement<WorldTerrainSource>>,
    pub faulted: Option<WorldDynamicFault>,
}

pub enum WorldTerrainMeshStep {
    Pending,
    Ready(String, Mesh3dLease),
    Complete((u32, u32, u32)),
    Fault(WorldDynamicFault),
}

impl WorldTerrainMeshCursor {

    pub fn new(surface_id: &str, (z,x,y):(u32,u32,u32), payload:TerrainTileMeshPayload, generation:u64, source_revision:u64, terrain_revision:u64, grant:semio_framework_value::retained_clone::RetainedCloneGrant)->Result<(Self,semio_framework_value::retained_clone::RetainedCloneProgress),(WorldDynamicFault,TerrainTileMeshPayload)> {
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

    pub fn value(&self, triangle: u32, vertex: u8, normal: bool) -> Result<[f32; 3], WorldDynamicFault> {
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

    pub fn token(&self) -> Result<Mesh3dWriteToken, WorldDynamicFault> {
        match self.owner {
            WorldPlaceholderOwner::Writing(token) => Ok(token),
            _ => Err(WorldDynamicFault::StaleToken),
        }
    }


    pub fn mesh_fault(fault: ui_wgpu::wgpu::Mesh3dFault) -> WorldDynamicFault { WorldDynamicFault::Mesh(fault) }

    pub fn step(&mut self, source_revision: u64, terrain_revision: u64, grant: semio_framework_value::retained_clone::RetainedCloneGrant, context: &mut semio_framework_job::StepContext<'_>) -> Result<(WorldTerrainMeshStep, semio_framework_value::retained_clone::RetainedCloneProgress), WorldDynamicFault> {
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

    pub fn step_live(&mut self, grant: semio_framework_value::retained_clone::RetainedCloneGrant) -> Result<(WorldTerrainMeshStep, semio_framework_value::retained_clone::RetainedCloneProgress), WorldDynamicFault> {
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

    pub fn close_step(&mut self, grant: semio_framework_value::retained_clone::RetainedCloneGrant) -> Result<ui_wgpu::wgpu::Mesh3dOwnershipStep,WorldDynamicFault> {
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

    pub fn terminal_is_empty(&self)->bool {
        matches!(self.owner,WorldPlaceholderOwner::Empty) && self.source_retirement.is_none() && self.payload.positions.capacity()==0 && self.payload.normals.capacity()==0 && self.payload.indices.capacity()==0 && self.payload.uvs.capacity()==0 && self.surface_id.capacity()==0
    }

}

#[cfg(not(any(test, feature = "testkit")))]
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

pub fn step_world3d_terrain_retirement(session_state: &mut WorldSessionState, terrain_state: &mut WorldTerrainState,grant:semio_framework_value::retained_clone::RetainedCloneGrant,context:&mut semio_framework_job::StepContext<'_>)->WorldTerrainMeshProgress {
    let mut result=WorldTerrainMeshProgress{step:WorldTerrainMeshPublicationStep::Idle,ownership:Default::default()};
    if context.should_yield() || grant.maximum_items==0 {result.step=WorldTerrainMeshPublicationStep::Pending;return result;}
    context.set_stage("terrainMeshRetirement");
    if terrain_state.terrain_build.is_none() && terrain_state.terrain_pending.is_some() {
        match begin_world_terrain_pending(session_state, terrain_state,grant,true) {
            Ok(progress)=>{result.ownership=progress;result.step=WorldTerrainMeshPublicationStep::Pending;}
            Err(fault)=>result.step=WorldTerrainMeshPublicationStep::Fault(fault),
        }
        context.consume_fuel(1);return result;
    }
    let Some(cursor)=terrain_state.terrain_build.as_mut() else{return result};
    let step=cursor.close_step(grant);
    context.consume_fuel(1);
    match step {
        Ok(step)=>{
            result.ownership=step.progress;
            if !step.progress.fits(grant) {result.step=WorldTerrainMeshPublicationStep::Fault(WorldDynamicFault::Ownership(semio_framework_value::ValueRefusalKind::InvariantViolated));}
            else if step.complete {terrain_state.terrain_build=None;result.step=WorldTerrainMeshPublicationStep::Complete;}
            else {result.step=WorldTerrainMeshPublicationStep::Pending;}
        }
        Err(WorldDynamicFault::Mesh(ui_wgpu::wgpu::Mesh3dFault::Busy))=>result.step=WorldTerrainMeshPublicationStep::Pending,
        Err(fault)=>result.step=WorldTerrainMeshPublicationStep::Fault(fault),
    }
    result
}

pub fn begin_world_terrain_pending(session_state: &mut WorldSessionState, terrain_state: &mut WorldTerrainState,grant:semio_framework_value::retained_clone::RetainedCloneGrant,canceled:bool)->Result<semio_framework_value::retained_clone::RetainedCloneProgress,WorldDynamicFault> {
    let Some((tile,payload,generation,revision,terrain_revision))=terrain_state.terrain_pending.take() else{return Ok(Default::default())};
    match WorldTerrainMeshCursor::new(if canceled {""} else {&session_state.surface_id},tile,payload,generation,revision,terrain_revision,grant) {
        Ok((mut cursor,progress))=>{if canceled {cursor.faulted=Some(WorldDynamicFault::Canceled);}terrain_state.terrain_build=Some(cursor);Ok(progress)}
        Err((fault,payload))=>{terrain_state.terrain_pending=Some((tile,payload,generation,revision,terrain_revision));Err(fault)}
    }
}

#[cfg(any(test, feature = "testkit"))]
pub struct TerrainMeshOracleData {
    pub positions: Vec<f32>,
    pub normals: Vec<f32>,
    pub indices: Vec<u32>,
    pub face_ids: Vec<u32>,
    pub vertex_ids: Vec<u32>,
    pub edge_positions: Vec<f32>,
    pub edge_ids: Vec<u32>,
    pub uvs: Vec<f32>,
    pub colors: Vec<f32>,
}

/// 🏔️🎨️ Language-neutral oracle for `WorldTerrainMeshCursor`: the de-indexed, vertex-coloured tile
/// mesh the cursor must produce — one triangle soup with the CONTINUOUS hypsometric ramp sampled per
/// vertex from the tile's own elevation ratio, which is what React's textured `MeshStandardMaterial`
/// shows.
#[cfg(any(test, feature = "testkit"))]
pub fn build_terrain_tile_mesh_oracle(mesh: &TerrainTileMeshPayload) -> Option<TerrainMeshOracleData> {
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
pub fn apply_terrain_style_if_changed_state(session_state: &mut WorldSessionState, scene_state: &mut WorldSceneState, terrain_state: &mut WorldTerrainState) -> Vec<String> {
    let signature = terrain_state.terrain_style.as_ref().map(|style| (style.tile_url_template.clone(), style.project_origin_lon, style.project_origin_lat, style.exaggeration));
    if signature == terrain_state.terrain_applied_signature {
        return Vec::new();
    }
    terrain_state.terrain_revision = terrain_state.terrain_revision.wrapping_add(1).max(1);
    if let Some(cursor) = terrain_state.terrain_build.as_mut() {
        cursor.faulted = Some(WorldDynamicFault::StaleToken);
    }
    let prefix = format!("terrain:{}:", session_state.surface_id);
    let stale_keys: Vec<String> = scene_state.meshes.keys().filter(|key| key.starts_with(&prefix)).cloned().collect();
    for key in &stale_keys {
        if !retire_world_mesh(scene_state, key) {
            return Vec::new();
        }
    }
    let visible = std::mem::take(&mut terrain_state.terrain_visible_tiles);
    for (z, x, y) in visible {
        terrain_state.terrain_session.evict_terrain_tile(z, x, y);
    }
    terrain_state.terrain_built_tiles.clear();
    terrain_state.pending_terrain_tile_urls.clear();
    terrain_state.terrain_tile_misses.clear();
    if let Some(style) = &terrain_state.terrain_style {
        terrain_state.terrain_session.set_project_origin(style.project_origin_lon, style.project_origin_lat);
        terrain_state.terrain_session.set_exaggeration(style.exaggeration);
    }
    terrain_state.terrain_applied_signature = signature;
    stale_keys
}

pub fn apply_terrain_style_if_changed(session_state: &mut WorldSessionState, scene_state: &mut WorldSceneState, terrain_state: &mut WorldTerrainState, gpu: &mut World3dBuildContext) {
    for key in apply_terrain_style_if_changed_state(session_state, scene_state, terrain_state) {
        gpu.evict_mesh(&key);
    }
}

/// 🏔️ One terrain band's resolved GPU draw inputs: mesh key/version (already present in
/// `state.meshes`/`state.mesh_versions`) plus the flat hypsometric color for that band.
/// 🏔️ One terrain tile's resolved GPU draw inputs: mesh key/version (already present in
/// `state.meshes`/`state.mesh_versions`). The hypsometric ramp rides the mesh's OWN per-vertex
/// colours, so the instance tint is neutral white — one draw per tile, never one per colour band.
pub struct TerrainTileDraw {
    pub mesh_key: String,
    pub mesh_version: u64,
}

pub fn terrain_family_visible(terrain_state: &WorldTerrainState, tile: (u32, u32, u32)) -> bool {
    terrain_state.terrain_built_tiles.contains(&tile)
}

//#endregion WorldMeshBuffers

//#region PlaceholderMesh
/// 🧊️ Viewport placeholder markers only (gumball plane, reference plane, vortex arrow parts, and
/// the box fallback for an unresolved `mesh_id`) — demo geometry, never document content. Ports
/// only the primitive kinds actually reachable from this file's `mesh_from_kind` call sites
/// (census in 📓️wave-g1b-infinite-report.md); the dissolved mesh-engine's other primitives
/// (uv-sphere, ico-sphere at other radii, torus, …) are not reachable here and were not ported.
#[cfg(any(test, feature = "testkit"))]
pub fn placeholder_mesh(kind: &str) -> WorldMeshBuffers {
    match kind {
        "vortex-marker" => placeholder_ico_sphere(0.12, 1),
        "plane" => placeholder_plane(1.0, 1.0),
        "cylinder" => placeholder_cylinder(0.5, 1.0, 16),
        "cone" => placeholder_cone(0.5, 1.0, 16),
        _ => placeholder_box(1.0, 1.0, 1.0),
    }
}

#[cfg(any(test, feature = "testkit"))]
fn placeholder_push_triangle(mesh: &mut WorldMeshBuffers, a: [f32; 3], b: [f32; 3], c: [f32; 3]) {
    let base = mesh.vertex_count() as u32;
    mesh.positions.extend_from_slice(&[a[0], a[1], a[2], b[0], b[1], b[2], c[0], c[1], c[2]]);
    mesh.indices.extend_from_slice(&[base, base + 1, base + 2]);
}

#[cfg(any(test, feature = "testkit"))]
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

#[cfg(any(test, feature = "testkit"))]
fn placeholder_plane(width: f32, depth: f32) -> WorldMeshBuffers {
    let hw = width * 0.5;
    let hd = depth * 0.5;
    let mut mesh = WorldMeshBuffers::default();
    placeholder_push_triangle(&mut mesh, [-hw, 0.0, -hd], [hw, 0.0, -hd], [hw, 0.0, hd]);
    placeholder_push_triangle(&mut mesh, [-hw, 0.0, -hd], [hw, 0.0, hd], [-hw, 0.0, hd]);
    mesh.compute_normals();
    mesh
}

#[cfg(any(test, feature = "testkit"))]
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

#[cfg(any(test, feature = "testkit"))]
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

#[cfg(any(test, feature = "testkit"))]
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

#[cfg(any(test, feature = "testkit"))]
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
