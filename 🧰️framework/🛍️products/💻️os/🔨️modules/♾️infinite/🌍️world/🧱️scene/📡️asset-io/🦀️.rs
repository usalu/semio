//! 📡️ World3d asset io authority: request ladder, fetch owner, responses and asset publication.

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
    backing: WorldAssetResponseBacking,
}

#[derive(Debug)]
enum WorldAssetResponseBacking{Owned(Box<[u8]>),Original(semio_framework_job::OriginalSuppliedJobPage)}

impl WorldAssetResponsePage {
    /// 📮️ Native bytes keep their actual supplier return ticket without copying or rebinding authority.
    pub fn try_from_original(page:semio_framework_job::OriginalSuppliedJobPage)->Result<Self,semio_framework_job::OriginalSuppliedJobPage>{if page.bytes().len()>WORLD_ASSET_RESPONSE_PAGE_BYTES{return Err(page)}Ok(Self{backing:WorldAssetResponseBacking::Original(page)})}


    pub fn try_from_owned(bytes: Vec<u8>) -> Result<Self, Vec<u8>> {
        if bytes.len() > WORLD_ASSET_RESPONSE_PAGE_BYTES {
            return Err(bytes);
        }
        Ok(Self { backing: WorldAssetResponseBacking::Owned(bytes.into_boxed_slice()) })
    }

    pub fn bytes(&self) -> &[u8] {
        match &self.backing{WorldAssetResponseBacking::Owned(bytes)=>bytes,WorldAssetResponseBacking::Original(page)=>page.bytes()}
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
        received_pages: u16,
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
        self.push_page_at(self.page_len,page)
    }

    /// 🧮️ An explicit original ordinal preserves wire order without shifting or copying other page owners.
    pub fn push_page_at(&mut self,index:u16,page:WorldAssetResponsePage)->Result<(),WorldAssetResponsePage>{
        let next = self.received_bytes.saturating_add(page.bytes().len());
        if self.closing || self.sealed || usize::from(index) >= WORLD_ASSET_RESPONSE_PAGE_CAPACITY || self.pages[usize::from(index)].is_some() || next > self.reserved_bytes {
            return Err(page);
        }
        self.pages[usize::from(index)] = Some(page);
        self.page_len = self.page_len.max(index+1);
        self.received_pages += 1;
        self.received_bytes = next;
        Ok(())
    }

    pub fn seal(&mut self) -> Result<(), WorldAssetFault> {
        if self.closing {
            return Err(WorldAssetFault::Closing);
        }
                if self.received_pages != self.page_len{return Err(WorldAssetFault::Incomplete)}
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
                self.received_pages = 0;
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
                        received_pages: 0,
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
