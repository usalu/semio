//! 📡️ World asset IO authority, brush mesh announce and reference image decode.

use super::*;

//#region 📡️WorldAssetIoAuthority
pub const WORLD_ASSET_REQUEST_CAPACITY: usize = 64;
pub const WORLD_ASSET_URL_BYTE_CAPACITY: usize = 2_048;
pub const WORLD_ASSET_RESPONSE_PAGE_BYTES: usize = 16 * 1024;
pub const WORLD_ASSET_RESPONSE_PAGE_CAPACITY: usize = 1_024;
pub const WORLD_ASSET_RESPONSE_BYTE_CAPACITY: usize = WORLD_ASSET_RESPONSE_PAGE_BYTES * WORLD_ASSET_RESPONSE_PAGE_CAPACITY;
pub const WORLD_ASSET_METADATA_ID_BYTES: usize = 256;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct WorldAssetMetadataId {
    pub(super) bytes: [u8; WORLD_ASSET_METADATA_ID_BYTES],
    pub(super) len: u16,
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
    pub(super) slot: u8,
    pub(super) epoch: u64,
    pub(super) generation: u64,
    pub(super) revision: u64,
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
    pub(super) bytes: Box<[u8]>,
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
    pub(super) token: WorldAssetRequestToken,
    pub(super) generation: u64,
    pub(super) revision: u64,
    pub(super) kind: WorldAssetRequestKind,
    pub(super) url: String,
    pub(super) reserved_bytes: usize,
    received_bytes: usize,
    pub(super) pages: Box<[Option<WorldAssetResponsePage>; WORLD_ASSET_RESPONSE_PAGE_CAPACITY]>,
    pub(super) page_len: u16,
    page_read: u16,
    close_page: u16,
    sealed: bool,
    pub(super) closing: bool,
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

#[cfg(not(test))]
impl Drop for WorldAssetFetchOwner {
    fn drop(&mut self) {
        assert!(self.terminal_is_empty() || std::thread::panicking(), "WorldAssetFetchOwner reached Drop before every response page and request string reached terminal handback");
    }
}

struct WorldAssetClaim {
    pub(super) epoch: u64,
    pub(super) generation: u64,
    pub(super) revision: u64,
    pub(super) kind: WorldAssetRequestKind,
    url_len: u16,
    url_bytes: Box<[u8; WORLD_ASSET_URL_BYTE_CAPACITY]>,
    pub(super) reserved_bytes: usize,
    in_flight: bool,
    fetch_complete: bool,
    pub(super) cancelled: bool,
    pub(super) owner: Option<WorldAssetFetchOwner>,
}

pub struct WorldAssetIoAuthority {
    pub(super) slots: Box<[Option<WorldAssetClaim>; WORLD_ASSET_REQUEST_CAPACITY]>,
    pub(super) epochs: [u64; WORLD_ASSET_REQUEST_CAPACITY],
    completed_cursor: u8,
    pub(super) reserved_bytes: usize,
    pub(super) closing: bool,
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

    pub(super) fn cancel_reference_image_urls_except(&mut self, references: &[WorldReferenceRecord]) {
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

pub fn reserve_world3d_asset(state: &mut World3dState, kind: WorldAssetRequestKind, url: &str, byte_credits: usize) -> Result<WorldAssetRequestToken, WorldAssetFault> {
    let generation = state.asset_generation.wrapping_add(1).max(1);
    let token = state.asset_io.reserve(generation, state.interaction_revision, kind, url, byte_credits)?;
    state.asset_generation = state.asset_generation.max(token.generation);
    Ok(token)
}

/// 🗑️ Records that the renderer's decoder REFUSED this url's bytes, so this surface stops offering
/// it. The mesh and reference-image request loops re-derive their missing urls from the scene every
/// frame, so without this ledger one refused response is re-fetched for the lifetime of the surface.
/// Sticky on purpose — the same shape as [`World3dState::terrain_tile_misses`] and as React's own
/// loader caches, where a failed texture url is not retried until the scene names a different one.
pub fn mark_world3d_asset_miss(state: &mut World3dState, url: &str) {
    state.asset_url_misses.insert(url.to_string());
    state.pending_image_urls.remove(url);
    state.pending_glb_urls.remove(url);
}

/// 🗑️ Whether [`mark_world3d_asset_miss`] refused this url for this surface.
pub fn world3d_asset_url_missed(state: &World3dState, url: &str) -> bool {
    state.asset_url_misses.contains(url)
}

pub fn reserve_world3d_asset_request(state: &mut World3dState, kind: WorldAssetRequestKind, url: &str) -> Result<WorldAssetRequestToken, WorldAssetFault> {
    let generation = state.asset_generation.wrapping_add(1).max(1);
    let token = state.asset_io.reserve_request(generation, state.interaction_revision, kind, url)?;
    state.asset_generation = state.asset_generation.max(token.generation);
    Ok(token)
}

pub fn take_next_world3d_asset(state: &mut World3dState) -> Option<WorldAssetFetchOwner> {
    state.asset_io.take_next()
}

pub fn reserve_world3d_asset_response(state: &mut World3dState, owner: &mut WorldAssetFetchOwner, byte_credits: usize) -> Result<(), WorldAssetFault> {
    state.asset_io.reserve_response(owner, byte_credits)
}

#[expect(clippy::result_large_err, reason = "Rejected transfer returns the exact admitted owner for bounded retirement without allocating on the failure path.")]
pub fn return_world3d_asset(state: &mut World3dState, owner: WorldAssetFetchOwner) -> Result<(), WorldAssetFetchOwner> {
    state.asset_io.return_owner(owner)
}

pub fn seal_world3d_asset_response(state: &mut World3dState, owner: &mut WorldAssetFetchOwner) -> Result<(), WorldAssetFault> {
    state.asset_io.seal_response(owner)
}

pub fn take_completed_world3d_asset(state: &mut World3dState, token: WorldAssetRequestToken) -> Result<WorldAssetFetchOwner, WorldAssetFault> {
    if token.revision != state.interaction_revision {
        return Err(WorldAssetFault::Stale);
    }
    state.asset_io.take_completed(token, token.generation, token.revision)
}

pub fn take_next_completed_world3d_asset_step(state: &mut World3dState) -> Option<WorldAssetFetchOwner> {
    state.asset_io.take_next_completed_step()
}

pub fn begin_world3d_asset_close(state: &mut World3dState) {
    state.asset_io.begin_close();
}

pub fn close_world3d_asset_step(state: &mut World3dState) -> bool {
    state.asset_io.close_step() && state.asset_io.terminal_is_empty()
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
pub fn world3d_asset_decode_pending(state: &World3dState) -> bool {
    state.asset_io.has_completed_step()
}

#[expect(clippy::result_large_err, reason = "Rejected transfer returns the exact admitted owner for bounded retirement without allocating on the failure path.")]
pub fn finish_world3d_asset(state: &mut World3dState, owner: WorldAssetFetchOwner) -> Result<(), WorldAssetFetchOwner> {
    state.asset_io.finish(owner)
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

pub fn retire_cancelled_world3d_asset_step(state: &mut World3dState) -> bool {
    state.asset_io.retire_cancelled_step()
}

pub fn world3d_asset_cancellation_requested(state: &World3dState, token: WorldAssetRequestToken) -> bool {
    state.asset_io.cancellation_requested(token)
}

#[cfg(not(test))]
impl Drop for WorldAssetIoAuthority {
    fn drop(&mut self) {
        assert!(self.terminal_is_empty() || std::thread::panicking(), "WorldAssetIoAuthority reached Drop before every request claim reached terminal handback");
    }
}
//#endregion 📡️WorldAssetIoAuthority

pub(super) fn mesh_id_from_url(url: &str) -> String {
    let slug = url.trim_start_matches('/').rsplit('/').next().unwrap_or(url).trim_end_matches(".glb").trim_end_matches(".gltf");
    format!("mesh:{slug}")
}

/// 🥽️📥️ Stores one decoded GLB under the id the wire names it by — `mesh_id_from_url` is the exact
/// twin of the guest's `world3d_mesh_id_from_url`, which is what makes a `{"id": "mesh:x", "url":
/// "/mesh/x.glb"}` entry's draw start painting the moment its bytes land. Clears the url's pending
/// mark on success so the lane stops offering it (see [`reserve_world3d_mesh_fetch`]).
pub fn publish_world3d_asset_mesh_lease(state: &mut World3dState, url: &str, mesh: Mesh3dLease) -> Result<(), WorldDynamicRejected<Mesh3dLease>> {
    if mesh.revision() != state.interaction_revision {
        return Err(WorldDynamicRejected { fault: WorldDynamicFault::StaleToken, id: mesh_id_from_url(url), value: mesh });
    }
    publish_world3d_mesh_lease(state, mesh_id_from_url(url), mesh)?;
    state.pending_glb_urls.remove(url);
    Ok(())
}

pub(super) fn publish_world3d_inline_asset(state:&mut World3dState,id:String,asset:World3dMeshAsset)->Result<(),World3dMeshAssetRejected> {publish_world3d_asset_by_id(state,id,asset)}

pub fn publish_world3d_asset_mesh(state: &mut World3dState, url: &str, asset: World3dMeshAsset) -> Result<(), World3dMeshAssetRejected> {
    let result=publish_world3d_asset_by_id(state,mesh_id_from_url(url),asset);
    if result.is_ok() {state.pending_glb_urls.remove(url);}
    result
}

fn publish_world3d_asset_by_id(state:&mut World3dState,id:String,asset:World3dMeshAsset)->Result<(),World3dMeshAssetRejected> {
    if asset.mesh.revision() != state.interaction_revision || state.dynamic_mesh_close.is_some() || state.dynamic_blocked_mesh.is_some() {
        return Err(World3dMeshAssetRejected { fault: if asset.mesh.revision() != state.interaction_revision { WorldDynamicFault::StaleToken } else { WorldDynamicFault::Closing }, id, value: asset });
    }
    let version = asset.mesh.generation().rotate_left(17) ^ asset.mesh.revision();
    let mesh_plan = match state.meshes.plan_insert(&id) { Ok(plan) => plan, Err(fault) => return Err(World3dMeshAssetRejected { fault, id, value: asset }) };
    let version_plan = match state.mesh_versions.plan_insert(&id) { Ok(plan) => plan, Err(fault) => return Err(World3dMeshAssetRejected { fault, id, value: asset }) };
    let appearance_plan = match state.mesh_appearances.plan_insert(&id) { Ok(plan) => plan, Err(fault) => return Err(World3dMeshAssetRejected { fault, id, value: asset }) };
    let interaction_plan = match state.interaction_meshes.plan_admit(&id, version, asset.mesh) { Ok(plan) => plan, Err(fault) => return Err(World3dMeshAssetRejected { fault, id, value: asset }) };
    let World3dMeshAsset { mesh, appearance } = asset;
    let (_, previous_mesh) = state.meshes.commit_insert(mesh_plan, id.clone(), mesh).expect("world material transaction revalidates its mesh slot");
    state.interaction_meshes.commit_admit(interaction_plan).expect("world material transaction revalidates its interaction slot");
    let (_, previous_version) = state.mesh_versions.commit_insert(version_plan, id.clone(), version).expect("world material transaction revalidates its version slot");
    let (_, previous_appearance) = state.mesh_appearances.commit_insert(appearance_plan, id, appearance).expect("world material transaction revalidates its appearance slot");
    drop(previous_version);
    drop(previous_appearance);
    if let Some(previous_mesh) = previous_mesh {
        state.dynamic_mesh_close = Some(previous_mesh);
    }
    state.geometry_generation = state.geometry_generation.wrapping_add(1).max(1);
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
pub(super) const WORLD_BRUSH_MESH_MAX_PAGES: u32 = 384;

/// 🚚️ How often one identity's guest-side re-upload request may be claimed under a single guest
/// instantiation — the page run the request asks for, plus exactly one retry for a run that failed
/// part-way. React's `PUZZLE3D_MESH_REUPLOAD_CLAIMS`.
const WORLD_BRUSH_MESH_REUPLOAD_CLAIMS: u32 = 2;

/// 📦️ Mesh identities the process-wide registry tracks at once — the same ceiling the mesh table
/// this lane announces out of is bounded by.
const WORLD_BRUSH_MESH_REGISTRY_CAPACITY: usize = WORLD_DYNAMIC_MESH_CAPACITY;

/// 🥽️ One page of a `registerBrushMesh` upload run this surface owns. `bytes` is the mesh's whole
/// wire payload — little-endian `f32` positions followed by little-endian `u32` indices, the exact
/// layout the guest's `decode_brush_mesh_page_values` reassembles and its `brush_mesh_digest`
/// hashes — read back once out of the resident [`Mesh3dLease`] when the run opened.
pub(super) struct WorldBrushMeshRun {
    pub(super) url: String,
    pub(super) digest: String,
    pub(super) bytes: Vec<u8>,
    position_values: usize,
    pub(super) capacity: usize,
    pub(super) page: u32,
    pub(super) page_count: u32,
    pub(super) revision: u64,
}

struct WorldBrushMeshEntry {
    pub(super) digest: String,
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
    pub(super) generation: u64,
    entries: HashMap<String, WorldBrushMeshEntry>,
    claims: HashMap<String, u32>,
    revisions: HashMap<String, u64>,
    refused_alias: HashSet<String>,
}

impl WorldBrushMeshRegistry {
    pub(super) fn clear(&mut self) {
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
pub(super) fn sync_world3d_brush_mesh_feedback(state: &mut World3dState, interaction_json: Option<&str>) {
    let Some(interaction_json) = interaction_json else { return };
    let digest = world3d_scene_digest(&[interaction_json]);
    if state.brush_mesh_feedback_digest == Some(digest) {
        return;
    }
    state.brush_mesh_feedback_digest = Some(digest);
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
pub fn step_world3d_brush_mesh_announce(state: &mut World3dState) -> Option<ActionDescriptor> {
    if let Some(action) = world_brush_mesh_next_page(state) {
        return Some(action);
    }
    let pending: Vec<(String, String)> = state
        .mesh_source_urls
        .iter()
        .filter(|(mesh_key, _)| state.meshes.contains_key(mesh_key.as_str()))
        .map(|(mesh_key, url)| (mesh_key.clone(), url.clone()))
        .filter(|(_, url)| state.brush_mesh_announced.get(url) != Some(&world_brush_mesh_revision(url)))
        .collect();
    let (mesh_key, url) = pending.into_iter().min_by(|left, right| left.1.cmp(&right.1))?;
    let revision = world_brush_mesh_revision(&url);
    if let Some(digest) = with_world_brush_mesh_registry(|registry| registry.entries.get(&url).map(|entry| entry.digest.clone())).flatten() {
        state.brush_mesh_announced.insert(url.clone(), revision);
        return Some(ActionDescriptor { controller_id: state.controller_id.clone(), action: "registerBrushMesh".into(), args: world_brush_mesh_args(&state.surface_id, semio_framework::dsl_value!({ "url": url, "digest": digest })) });
    }
    let mesh = state.meshes.get(&mesh_key).copied()?;
    let Some((bytes, position_values)) = world_brush_mesh_payload(mesh) else {
        state.brush_mesh_announced.insert(url, revision);
        return None;
    };
    let digest = semio_framework_hash::hash_bytes(&bytes);
    let aliasable = with_world_brush_mesh_registry(|registry| registry.holds_digest(&digest) && !registry.refused_alias.contains(&url)).unwrap_or(false);
    if aliasable {
        with_world_brush_mesh_registry(|registry| registry.entries.insert(url.clone(), WorldBrushMeshEntry { digest: digest.clone(), paged: false }));
        state.brush_mesh_announced.insert(url.clone(), revision);
        return Some(ActionDescriptor { controller_id: state.controller_id.clone(), action: "registerBrushMesh".into(), args: world_brush_mesh_args(&state.surface_id, semio_framework::dsl_value!({ "url": url, "digest": digest })) });
    }
    let capacity = world_brush_mesh_page_capacity(&state.surface_id, &url, &digest);
    let total_values = bytes.len() / 4;
    let page_count = u32::try_from(total_values.div_ceil(capacity.max(1))).unwrap_or(u32::MAX);
    if capacity == 0 || page_count == 0 || page_count > WORLD_BRUSH_MESH_MAX_PAGES {
        state.brush_mesh_announced.insert(url, revision);
        return None;
    }
    state.brush_mesh_run = Some(WorldBrushMeshRun { url, digest, bytes, position_values, capacity, page: 0, page_count, revision });
    world_brush_mesh_next_page(state)
}

fn world_brush_mesh_next_page(state: &mut World3dState) -> Option<ActionDescriptor> {
    let run = state.brush_mesh_run.as_mut()?;
    if run.page >= run.page_count {
        state.brush_mesh_run = None;
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
    let action = ActionDescriptor { controller_id: state.controller_id.clone(), action: "registerBrushMesh".into(), args: world_brush_mesh_args(&state.surface_id, page) };
    run.page = run.page.saturating_add(1);
    if run.page == run.page_count {
        let (url, digest, revision) = (run.url.clone(), run.digest.clone(), run.revision);
        state.brush_mesh_run = None;
        with_world_brush_mesh_registry(|registry| {
            if registry.entries.len() < WORLD_BRUSH_MESH_REGISTRY_CAPACITY || registry.entries.contains_key(&url) {
                registry.entries.insert(url.clone(), WorldBrushMeshEntry { digest, paged: true });
            }
        });
        state.brush_mesh_announced.insert(url, revision);
    }
    Some(action)
}

/// 🥽️ Whether this surface still owes the guest a `registerBrushMesh` — the per-frame predicate the
/// renderer's frame gate reads, so a surface mid-run keeps its own frames coming.
pub fn world3d_brush_mesh_announce_pending(state: &World3dState) -> bool {
    state.brush_mesh_run.is_some() || state.mesh_source_urls.iter().any(|(mesh_key, url)| state.meshes.contains_key(mesh_key.as_str()) && state.brush_mesh_announced.get(url) != Some(&world_brush_mesh_revision(url)))
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
pub(super) const WORLD_REFERENCE_TEXTURE_BYTES: usize = ui_wgpu::wgpu::SCENE_RASTER_ITEM_BYTES;
pub(super) const WORLD_REFERENCE_SOURCE_PIXELS: u64 = 64 * 1024 * 1024 / 4;

/// 🖼️ `image` scaled to at most [`WORLD_REFERENCE_TEXTURE_BYTES`] of RGBA, aspect preserved.
pub(super) fn bounded_reference_image(image: image::DynamicImage) -> image::DynamicImage {
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

/// 🎨️ The existing owned image decoder preserves admitted mesh texture pixels without reference-image resizing.
pub fn decode_mesh_surface_image_bytes(bytes:&[u8])->Option<semio_framework_pixels::RasterImage> {
    use image::ImageDecoder as _;
    let reader=image::ImageReader::new(std::io::Cursor::new(bytes)).with_guessed_format().ok()?;let mut decoder=reader.into_decoder().ok()?;
    let (width,height)=decoder.dimensions();if u64::from(width).checked_mul(u64::from(height))?>WORLD_REFERENCE_SOURCE_PIXELS{return None}
    let orientation=decoder.orientation().unwrap_or(image::metadata::Orientation::NoTransforms);let mut image=image::DynamicImage::from_decoder(decoder).ok()?;image.apply_orientation(orientation);let rgba=image.to_rgba8();
    Some(semio_framework_pixels::RasterImage{width:rgba.width(),height:rgba.height(),pixels:rgba.into_raw()})
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

pub fn apply_decoded_reference_image(state: &mut World3dState, url: &str, decoded: DecodedReferenceImage) -> bool {
    let expected = usize::try_from(decoded.width)
        .ok()
        .and_then(|width| usize::try_from(decoded.height).ok().and_then(|height| width.checked_mul(height)))
        .and_then(|pixels| pixels.checked_mul(4));
    if expected != Some(decoded.pixels.len()) || decoded.pixels.len() > WORLD_REFERENCE_TEXTURE_BYTES || !world3d_reference_url_is_current(state, url) {
        if !world3d_reference_url_is_current(state, url) {
            state.pending_image_urls.remove(url);
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
    apply_decoded_reference_raster(state, url, lease)
}

pub fn apply_decoded_reference_raster(state: &mut World3dState, url: &str, lease: SceneRasterLease) -> bool {
    let descriptor = lease.identity().descriptor();
    if descriptor.profile != SceneRasterProfile::ReferenceImageMapNoColorSpace || !world3d_reference_url_is_current(state, url) {
        if !world3d_reference_url_is_current(state, url) {
            state.pending_image_urls.remove(url);
        }
        return false;
    }
    if !publish_world_raster(state, url.to_string(), lease, false) {
        return false;
    }
    state.pending_image_urls.remove(url);
    true
}

pub fn apply_reference_image_bytes(state: &mut World3dState, url: &str, bytes: &[u8]) -> bool {
    decode_reference_image_bytes(bytes).is_some_and(|decoded| apply_decoded_reference_image(state, url, decoded))
}
