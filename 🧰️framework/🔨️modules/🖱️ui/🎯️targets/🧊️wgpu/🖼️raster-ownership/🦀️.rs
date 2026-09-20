//! 🖼️ Exact pooled ownership for decoded scene rasters.

use crate::wgpu::prepared::RasterContentIdentity;
use std::fmt;
use std::sync::{Arc, Mutex};

pub const SCENE_RASTER_ITEM_BYTES: usize = 64 * 1024 * 1024;
pub const SCENE_RASTER_POOL_SLOTS: usize = 4;
pub const SCENE_RASTER_POOL_BYTES: usize = SCENE_RASTER_ITEM_BYTES * SCENE_RASTER_POOL_SLOTS;
pub const SCENE_RASTER_LEASE_CAPACITY: usize = 64;
pub const SCENE_RASTER_TRANSFER_BYTES: usize = 1024 * 1024;
pub const SCENE_RASTER_GPU_RESIDENT_CAPACITY: usize = 256;
pub const SCENE_RASTER_GPU_RESIDENT_BYTES: usize = 256 * 1024 * 1024;
const SCENE_RASTER_GPU_KEY_BYTES: usize = 256;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SceneRasterProfile {
    ReferenceImageMapNoColorSpace,
    ReferenceCanvasSrgb,
    MeshPaintMapNoColorSpace,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SceneRasterMeshSeal {
    pub mesh_revision: u64,
    pub uv_revision: u64,
    pub uv_count: u32,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SceneRasterDescriptor {
    pub width: u32,
    pub height: u32,
    pub source_digest: [u64; 2],
    pub source_revision: u64,
    pub profile: SceneRasterProfile,
    pub mesh: Option<SceneRasterMeshSeal>,
}

impl SceneRasterDescriptor {
    pub fn byte_len(self) -> Option<usize> {
        usize::try_from(self.width).ok()?.checked_mul(usize::try_from(self.height).ok()?)?.checked_mul(4)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SceneRasterIdentity {
    descriptor: SceneRasterDescriptor,
    content: RasterContentIdentity,
}

impl SceneRasterIdentity {
    pub fn descriptor(self) -> SceneRasterDescriptor {
        self.descriptor
    }

    pub fn content(self) -> RasterContentIdentity {
        self.content
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SceneRasterPoolLimits {
    pub item_bytes: usize,
    pub pool_bytes: usize,
    pub slot_capacity: usize,
    pub lease_capacity_per_slot: usize,
    pub transfer_bytes: usize,
    pub retire_bytes: usize,
    pub gpu_resident_bytes: usize,
}

impl SceneRasterPoolLimits {
    pub const fn production() -> Self {
        Self {
            item_bytes: SCENE_RASTER_ITEM_BYTES,
            pool_bytes: SCENE_RASTER_POOL_BYTES,
            slot_capacity: SCENE_RASTER_POOL_SLOTS,
            lease_capacity_per_slot: SCENE_RASTER_LEASE_CAPACITY,
            transfer_bytes: SCENE_RASTER_TRANSFER_BYTES,
            retire_bytes: SCENE_RASTER_TRANSFER_BYTES,
            gpu_resident_bytes: SCENE_RASTER_GPU_RESIDENT_BYTES,
        }
    }
}

impl Default for SceneRasterPoolLimits {
    fn default() -> Self {
        Self::production()
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SceneRasterWriteMode {
    Streamed,
    Moved,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SceneRasterWriter {
    slot: u16,
    epoch: u64,
    owner: u64,
    mode: SceneRasterWriteMode,
    cursor: usize,
}

impl SceneRasterWriter {
    pub fn cursor(self) -> usize {
        self.cursor
    }
}

pub struct SceneRasterMovedPixels {
    writer: SceneRasterWriter,
    pixels: Vec<u8>,
    content: RasterContentIdentity,
}

impl fmt::Debug for SceneRasterMovedPixels {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.debug_struct("SceneRasterMovedPixels").field("writer", &self.writer).field("bytes", &self.pixels.len()).finish_non_exhaustive()
    }
}

pub enum SceneRasterBegin {
    Writer(SceneRasterWriter),
    Reused(SceneRasterLease),
    Backpressure(&'static str),
    Refused(&'static str),
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
struct LeaseToken {
    slot: u16,
    epoch: u64,
    lease_id: u64,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
struct LeaseRecord {
    id: u64,
    live: bool,
}

struct WritingRaster {
    token: SceneRasterWriter,
    descriptor: SceneRasterDescriptor,
    expected_bytes: usize,
    pixels: Option<Vec<u8>>,
    content: RasterContentIdentity,
}

struct ReadyRaster {
    descriptor: SceneRasterDescriptor,
    pixels: Vec<u8>,
    content: RasterContentIdentity,
    leases: [LeaseRecord; SCENE_RASTER_LEASE_CAPACITY],
    access: u64,
}

struct RetiringRaster {
    pixels: Vec<u8>,
    reserved_bytes: usize,
}

enum RasterSlotState {
    Vacant,
    Writing(WritingRaster),
    Ready(ReadyRaster),
    Retiring(RetiringRaster),
}

struct RasterSlot {
    epoch: u64,
    state: RasterSlotState,
}

struct SceneRasterPoolState {
    slots: Vec<RasterSlot>,
    next_lease_id: u64,
    access_clock: u64,
    reserved_bytes: usize,
    gpu_residents: Vec<Option<SceneRasterGpuRecord>>,
    gpu_resident_bytes: usize,
    next_gpu_epoch: u64,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct SceneRasterGpuKey {
    bytes: [u8; SCENE_RASTER_GPU_KEY_BYTES],
    len: u16,
}

impl SceneRasterGpuKey {
    fn new(key: &str) -> Option<Self> {
        if key.is_empty() || key.len() > SCENE_RASTER_GPU_KEY_BYTES {
            return None;
        }
        let mut bytes = [0; SCENE_RASTER_GPU_KEY_BYTES];
        bytes[..key.len()].copy_from_slice(key.as_bytes());
        Some(Self { bytes, len: key.len() as u16 })
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct SceneRasterGpuRecord {
    key: SceneRasterGpuKey,
    identity: SceneRasterIdentity,
    epoch: u64,
}

struct SceneRasterPoolInner {
    limits: SceneRasterPoolLimits,
    state: Mutex<SceneRasterPoolState>,
}

#[derive(Clone)]
pub struct SceneRasterPool {
    inner: Arc<SceneRasterPoolInner>,
}

impl fmt::Debug for SceneRasterPool {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.debug_struct("SceneRasterPool").field("limits", &self.inner.limits).finish_non_exhaustive()
    }
}

pub struct SceneRasterLease {
    inner: Arc<SceneRasterPoolInner>,
    token: Option<LeaseToken>,
    identity: SceneRasterIdentity,
}

pub struct SceneRasterReleaseWitness {
    inner: Arc<SceneRasterPoolInner>,
    token: LeaseToken,
    identity: SceneRasterIdentity,
}

pub struct SceneRasterGpuWitness {
    inner: Arc<SceneRasterPoolInner>,
    slot: u16,
    epoch: u64,
    identity: SceneRasterIdentity,
}

impl fmt::Debug for SceneRasterReleaseWitness {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.debug_struct("SceneRasterReleaseWitness").field("token", &self.token).finish_non_exhaustive()
    }
}

impl SceneRasterReleaseWitness {
    pub fn release(&self) -> bool {
        release_token(&self.inner, self.token)
    }

    pub fn commit_gpu(self, key: &str) -> Result<SceneRasterGpuWitness, Self> {
        let Some(key) = SceneRasterGpuKey::new(key) else { return Err(self) };
        let inner = Arc::clone(&self.inner);
        let Ok(mut state) = inner.state.lock() else { return Err(self) };
        let source_index = usize::from(self.token.slot);
        let lease_index = state.slots.get(source_index).filter(|slot| slot.epoch == self.token.epoch).and_then(|slot| match &slot.state {
            RasterSlotState::Ready(ready) => ready.leases.iter().position(|lease| lease.live && lease.id == self.token.lease_id),
            _ => None,
        });
        let Some(lease_index) = lease_index else {
            drop(state);
            return Err(self);
        };
        let slot = state.gpu_residents.iter().position(|resident| resident.is_some_and(|resident| resident.key == key)).or_else(|| state.gpu_residents.iter().position(Option::is_none));
        let Some(slot) = slot else {
            drop(state);
            return Err(self);
        };
        let Some(epoch) = state.next_gpu_epoch.checked_add(1) else {
            drop(state);
            return Err(self);
        };
        let bytes = self.identity.descriptor.byte_len().unwrap_or(usize::MAX);
        let replaced_bytes = state.gpu_residents[slot].as_ref().and_then(|resident| resident.identity.descriptor.byte_len()).unwrap_or(0);
        let Some(gpu_resident_bytes) = state.gpu_resident_bytes.checked_sub(replaced_bytes).and_then(|held| held.checked_add(bytes)) else {
            drop(state);
            return Err(self);
        };
        if gpu_resident_bytes > inner.limits.gpu_resident_bytes {
            drop(state);
            return Err(self);
        }
        let RasterSlotState::Ready(ready) = &mut state.slots[source_index].state else { unreachable!("scene raster GPU commit source was validated above") };
        ready.leases[lease_index].live = false;
        state.next_gpu_epoch = epoch;
        state.gpu_residents[slot] = Some(SceneRasterGpuRecord { key, identity: self.identity, epoch });
        state.gpu_resident_bytes = gpu_resident_bytes;
        drop(state);
        Ok(SceneRasterGpuWitness { inner, slot: slot as u16, epoch, identity: self.identity })
    }
}

impl fmt::Debug for SceneRasterGpuWitness {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.debug_struct("SceneRasterGpuWitness").field("slot", &self.slot).field("epoch", &self.epoch).field("identity", &self.identity).finish()
    }
}

impl Drop for SceneRasterGpuWitness {
    fn drop(&mut self) {
        let Ok(mut state) = self.inner.state.lock() else { return };
        let bytes = {
            let Some(slot) = state.gpu_residents.get_mut(usize::from(self.slot)) else { return };
            if !slot.as_ref().is_some_and(|resident| resident.epoch == self.epoch && resident.identity == self.identity) {
                return;
            }
            let bytes = slot.as_ref().and_then(|resident| resident.identity.descriptor.byte_len()).unwrap_or(0);
            *slot = None;
            bytes
        };
        state.gpu_resident_bytes = state.gpu_resident_bytes.saturating_sub(bytes);
    }
}

impl fmt::Debug for SceneRasterLease {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.debug_struct("SceneRasterLease").field("token", &self.token).field("identity", &self.identity).finish()
    }
}

impl SceneRasterLease {
    pub fn identity(&self) -> SceneRasterIdentity {
        self.identity
    }

    pub fn byte_len(&self) -> usize {
        self.identity.descriptor.byte_len().unwrap_or(0)
    }

    pub fn transfer_bytes(&self) -> usize {
        self.inner.limits.transfer_bytes
    }

    pub fn release_witness(&self) -> Option<SceneRasterReleaseWitness> {
        self.token.map(|token| SceneRasterReleaseWitness { inner: Arc::clone(&self.inner), token, identity: self.identity })
    }

    pub fn release_committed(&self) -> bool {
        self.token.is_some_and(|token| release_token(&self.inner, token))
    }

    pub fn with_rows<R>(&self, start_row: u32, byte_capacity: usize, read: impl FnOnce(&[u8], u32) -> R) -> Result<R, &'static str> {
        let Some(token) = self.token else { return Err("scene raster lease is released") };
        let state = self.inner.state.lock().map_err(|_| "scene raster pool lock is poisoned")?;
        let Some(slot) = state.slots.get(usize::from(token.slot)) else { return Err("scene raster lease slot is invalid") };
        if slot.epoch != token.epoch {
            return Err("scene raster lease epoch is stale");
        }
        let RasterSlotState::Ready(ready) = &slot.state else { return Err("scene raster lease is not ready") };
        if !ready.leases.iter().any(|lease| lease.live && lease.id == token.lease_id) {
            return Err("scene raster lease owner is not live");
        }
        let row_bytes = usize::try_from(ready.descriptor.width).ok().and_then(|width| width.checked_mul(4)).ok_or("scene raster row bytes overflowed")?;
        if row_bytes == 0 || start_row >= ready.descriptor.height || byte_capacity < row_bytes || byte_capacity > self.inner.limits.transfer_bytes {
            return Err("scene raster row request is invalid");
        }
        let rows = (byte_capacity / row_bytes).max(1).min(usize::try_from(ready.descriptor.height - start_row).map_err(|_| "scene raster row count overflowed")?);
        let start = usize::try_from(start_row).map_err(|_| "scene raster row offset overflowed")?.checked_mul(row_bytes).ok_or("scene raster row offset overflowed")?;
        let end = start.checked_add(rows.checked_mul(row_bytes).ok_or("scene raster row span overflowed")?).ok_or("scene raster row span overflowed")?;
        let bytes = ready.pixels.get(start..end).ok_or("scene raster row span escaped its pool owner")?;
        Ok(read(bytes, u32::try_from(rows).map_err(|_| "scene raster row count overflowed")?))
    }

    pub fn release(mut self) -> bool {
        self.release_inner()
    }

    fn release_inner(&mut self) -> bool {
        let Some(token) = self.token.take() else { return false };
        release_token(&self.inner, token)
    }
}

impl PartialEq for SceneRasterLease {
    fn eq(&self, other: &Self) -> bool {
        Arc::ptr_eq(&self.inner, &other.inner) && self.token == other.token && self.identity == other.identity
    }
}

impl Eq for SceneRasterLease {}

impl Drop for SceneRasterLease {
    fn drop(&mut self) {
        let _ = self.release_inner();
    }
}

fn release_token(inner: &Arc<SceneRasterPoolInner>, token: LeaseToken) -> bool {
    let Ok(mut state) = inner.state.lock() else { return false };
    let Some(slot) = state.slots.get_mut(usize::from(token.slot)) else { return false };
    if slot.epoch != token.epoch {
        return false;
    }
    let RasterSlotState::Ready(ready) = &mut slot.state else { return false };
    let Some(lease) = ready.leases.iter_mut().find(|lease| lease.live && lease.id == token.lease_id) else { return false };
    lease.live = false;
    true
}

impl SceneRasterPool {
    pub fn try_new(limits: SceneRasterPoolLimits) -> Result<Self, &'static str> {
        if limits.item_bytes == 0
            || limits.pool_bytes < limits.item_bytes
            || limits.slot_capacity == 0
            || limits.slot_capacity > SCENE_RASTER_POOL_SLOTS
            || limits.lease_capacity_per_slot == 0
            || limits.lease_capacity_per_slot > SCENE_RASTER_LEASE_CAPACITY
            || limits.transfer_bytes == 0
            || limits.retire_bytes == 0
            || limits.gpu_resident_bytes == 0
        {
            return Err("scene raster pool limits are invalid");
        }
        let slots = (0..limits.slot_capacity).map(|_| RasterSlot { epoch: 0, state: RasterSlotState::Vacant }).collect();
        let gpu_residents = (0..SCENE_RASTER_GPU_RESIDENT_CAPACITY).map(|_| None).collect();
        Ok(Self { inner: Arc::new(SceneRasterPoolInner { limits, state: Mutex::new(SceneRasterPoolState { slots, next_lease_id: 1, access_clock: 1, reserved_bytes: 0, gpu_residents, gpu_resident_bytes: 0, next_gpu_epoch: 1 }) }) })
    }

    pub fn new() -> Self {
        Self::try_new(SceneRasterPoolLimits::production()).expect("production scene raster limits")
    }

    pub fn begin(&self, descriptor: SceneRasterDescriptor, owner: u64, mode: SceneRasterWriteMode) -> SceneRasterBegin {
        if owner == 0 || descriptor.width == 0 || descriptor.height == 0 {
            return SceneRasterBegin::Refused("scene raster descriptor or owner is empty");
        }
        let mesh_profile_is_valid = match descriptor.profile {
            SceneRasterProfile::MeshPaintMapNoColorSpace => descriptor.mesh.is_some_and(|mesh| mesh.mesh_revision > 0 && mesh.uv_revision > 0 && mesh.uv_count > 0),
            SceneRasterProfile::ReferenceImageMapNoColorSpace | SceneRasterProfile::ReferenceCanvasSrgb => descriptor.mesh.is_none(),
        };
        if !mesh_profile_is_valid {
            return SceneRasterBegin::Refused("scene raster mesh seal is missing or invalid for its profile");
        }
        let Some(expected_bytes) = descriptor.byte_len() else { return SceneRasterBegin::Refused("scene raster dimensions overflowed") };
        let row_bytes = usize::try_from(descriptor.width).ok().and_then(|width| width.checked_mul(4));
        if expected_bytes > self.inner.limits.item_bytes || row_bytes.is_none_or(|bytes| bytes > self.inner.limits.transfer_bytes) {
            return SceneRasterBegin::Refused("scene raster exceeded fixed item bytes");
        }
        let Ok(mut state) = self.inner.state.lock() else { return SceneRasterBegin::Backpressure("scene raster pool lock is poisoned") };
        let next_access = state.access_clock.checked_add(1);
        let Some(next_access) = next_access else { return SceneRasterBegin::Backpressure("scene raster access epoch exhausted") };
        if let Some(index) = state.slots.iter().position(|slot| matches!(&slot.state, RasterSlotState::Ready(ready) if ready.descriptor == descriptor)) {
            let lease = match mint_lease(&self.inner, &mut state, index, next_access) {
                Ok(lease) => lease,
                Err(fault) => return SceneRasterBegin::Backpressure(fault),
            };
            state.access_clock = next_access;
            return SceneRasterBegin::Reused(lease);
        }
        if state.slots.iter().any(|slot| matches!(&slot.state, RasterSlotState::Writing(writing) if writing.descriptor == descriptor)) {
            return SceneRasterBegin::Backpressure("scene raster identity is building");
        }
        if state.slots.iter().any(|slot| matches!(slot.state, RasterSlotState::Retiring(_))) {
            return SceneRasterBegin::Backpressure("scene raster retirement is pending");
        }
        let Some(index) = state.slots.iter().position(|slot| matches!(slot.state, RasterSlotState::Vacant)) else {
            let candidate = state
                .slots
                .iter()
                .enumerate()
                .filter_map(|(index, slot)| match &slot.state {
                    RasterSlotState::Ready(ready) if live_lease_count(ready) == 0 => Some((index, ready.access)),
                    _ => None,
                })
                .min_by_key(|(_, access)| *access)
                .map(|(index, _)| index);
            let Some(index) = candidate else { return SceneRasterBegin::Backpressure("all scene raster slots are live") };
            let old = std::mem::replace(&mut state.slots[index].state, RasterSlotState::Vacant);
            let RasterSlotState::Ready(ready) = old else { return SceneRasterBegin::Backpressure("scene raster eviction candidate changed") };
            let reserved_bytes = ready.pixels.len();
            state.slots[index].state = RasterSlotState::Retiring(RetiringRaster { pixels: ready.pixels, reserved_bytes });
            return SceneRasterBegin::Backpressure("scene raster retirement is pending");
        };
        let Some(next_reserved) = state.reserved_bytes.checked_add(expected_bytes) else { return SceneRasterBegin::Backpressure("scene raster pool bytes overflowed") };
        if next_reserved > self.inner.limits.pool_bytes {
            return SceneRasterBegin::Backpressure("scene raster pool bytes are exhausted");
        }
        let Some(epoch) = state.slots[index].epoch.checked_add(1) else { return SceneRasterBegin::Backpressure("scene raster slot epoch exhausted") };
        let pixels = match mode {
            SceneRasterWriteMode::Streamed => {
                let mut pixels = Vec::new();
                if pixels.try_reserve_exact(expected_bytes).is_err() {
                    return SceneRasterBegin::Backpressure("scene raster backing allocation failed");
                }
                Some(pixels)
            }
            SceneRasterWriteMode::Moved => None,
        };
        let Some(mut content) = RasterContentIdentity::pixels(descriptor.width, descriptor.height, expected_bytes) else { return SceneRasterBegin::Refused("scene raster identity overflowed") };
        content.mix_word(descriptor.source_digest[0]);
        content.mix_word(descriptor.source_digest[1]);
        content.mix_word(descriptor.source_revision);
        content.mix_word(match descriptor.profile {
            SceneRasterProfile::ReferenceImageMapNoColorSpace => 1,
            SceneRasterProfile::ReferenceCanvasSrgb => 2,
            SceneRasterProfile::MeshPaintMapNoColorSpace => 3,
        });
        if let Some(mesh) = descriptor.mesh {
            content.mix_word(mesh.mesh_revision);
            content.mix_word(mesh.uv_revision);
            content.mix_word(u64::from(mesh.uv_count));
        }
        let token = SceneRasterWriter { slot: index as u16, epoch, owner, mode, cursor: 0 };
        state.slots[index].epoch = epoch;
        state.slots[index].state = RasterSlotState::Writing(WritingRaster { token, descriptor, expected_bytes, pixels, content });
        state.reserved_bytes = next_reserved;
        state.access_clock = next_access;
        SceneRasterBegin::Writer(token)
    }

    pub fn push(&self, mut writer: SceneRasterWriter, bytes: &[u8]) -> Result<SceneRasterWriter, &'static str> {
        if bytes.is_empty() || bytes.len() > self.inner.limits.transfer_bytes {
            return Err("scene raster transfer exceeded chunk credits");
        }
        let mut state = self.inner.state.lock().map_err(|_| "scene raster pool lock is poisoned")?;
        let writing = exact_writing_mut(&mut state, writer)?;
        if writer.mode != SceneRasterWriteMode::Streamed {
            return Err("moved scene raster writer does not accept copied chunks");
        }
        let pixels = writing.pixels.as_mut().ok_or("streamed scene raster lost its pool backing")?;
        let end = pixels.len().checked_add(bytes.len()).ok_or("scene raster write cursor overflowed")?;
        if end > writing.expected_bytes {
            return Err("scene raster write exceeded its exact claim");
        }
        let row_bytes = usize::try_from(writing.descriptor.width).ok().and_then(|width| width.checked_mul(4)).ok_or("scene raster row bytes overflowed")?;
        if end != writing.expected_bytes && bytes.len() % row_bytes != 0 {
            return Err("scene raster chunk split a pixel row");
        }
        writing.content.mix_bytes(writer.cursor, bytes);
        pixels.extend_from_slice(bytes);
        writer.cursor = end;
        writing.token = writer;
        Ok(writer)
    }

    pub fn seal(&self, writer: SceneRasterWriter) -> Result<SceneRasterLease, &'static str> {
        if writer.mode != SceneRasterWriteMode::Streamed {
            return Err("moved scene raster must use seal_moved");
        }
        self.seal_inner(writer)
    }

    pub fn seal_moved(&self, writer: SceneRasterWriter, pixels: Vec<u8>) -> Result<SceneRasterLease, (Vec<u8>, &'static str)> {
        let prepared = self.prepare_moved(writer, pixels)?;
        self.seal_prepared_moved(prepared)
    }

    pub fn prepare_moved(&self, writer: SceneRasterWriter, pixels: Vec<u8>) -> Result<SceneRasterMovedPixels, (Vec<u8>, &'static str)> {
        if writer.mode != SceneRasterWriteMode::Moved {
            return Err((pixels, "streamed scene raster writer cannot accept moved pixels"));
        }
        let state = match self.inner.state.lock() {
            Ok(state) => state,
            Err(_) => return Err((pixels, "scene raster pool lock is poisoned")),
        };
        let index = usize::from(writer.slot);
        let Some(slot) = state.slots.get(index) else { return Err((pixels, "scene raster writer slot is invalid")) };
        if slot.epoch != writer.epoch || !matches!(&slot.state, RasterSlotState::Writing(writing) if writing.token == writer) {
            return Err((pixels, "scene raster writer is stale"));
        }
        let RasterSlotState::Writing(writing) = &slot.state else { return Err((pixels, "scene raster writer is stale")) };
        if pixels.len() != writing.expected_bytes || pixels.capacity() > writing.expected_bytes {
            return Err((pixels, "moved scene raster did not match its exact admitted allocation"));
        }
        let mut content = writing.content;
        drop(state);
        content.mix_bytes(0, &pixels);
        Ok(SceneRasterMovedPixels { writer, pixels, content })
    }

    pub fn seal_prepared_moved(&self, prepared: SceneRasterMovedPixels) -> Result<SceneRasterLease, (Vec<u8>, &'static str)> {
        let SceneRasterMovedPixels { writer, pixels, content } = prepared;
        let mut state = match self.inner.state.lock() {
            Ok(state) => state,
            Err(_) => return Err((pixels, "scene raster pool lock is poisoned")),
        };
        let index = usize::from(writer.slot);
        let Some(slot) = state.slots.get(index) else { return Err((pixels, "scene raster writer slot is invalid")) };
        if slot.epoch != writer.epoch || !matches!(&slot.state, RasterSlotState::Writing(writing) if writing.token == writer && writing.expected_bytes == pixels.len()) {
            return Err((pixels, "scene raster writer changed before moved publication"));
        }
        if state.next_lease_id == u64::MAX {
            return Err((pixels, "scene raster lease epoch exhausted"));
        }
        let old = std::mem::replace(&mut state.slots[index].state, RasterSlotState::Vacant);
        let RasterSlotState::Writing(writing) = old else { return Err((pixels, "scene raster writer is stale")) };
        let access = state.access_clock;
        state.slots[index].state = RasterSlotState::Ready(ReadyRaster { descriptor: writing.descriptor, pixels, content, leases: [LeaseRecord::default(); SCENE_RASTER_LEASE_CAPACITY], access });
        Ok(mint_lease(&self.inner, &mut state, index, access).expect("preflighted first scene raster lease"))
    }

    fn seal_inner(&self, writer: SceneRasterWriter) -> Result<SceneRasterLease, &'static str> {
        let mut state = self.inner.state.lock().map_err(|_| "scene raster pool lock is poisoned")?;
        let index = usize::from(writer.slot);
        let old = {
            let slot = state.slots.get_mut(index).ok_or("scene raster writer slot is invalid")?;
            if slot.epoch != writer.epoch || !matches!(&slot.state, RasterSlotState::Writing(writing) if writing.token == writer) {
                return Err("scene raster writer is stale");
            }
            std::mem::replace(&mut slot.state, RasterSlotState::Vacant)
        };
        let RasterSlotState::Writing(mut writing) = old else { return Err("scene raster writer is stale") };
        let pixels = writing.pixels.take().ok_or("streamed scene raster lost its pool backing")?;
        if pixels.len() != writing.expected_bytes {
            writing.pixels = Some(pixels);
            state.slots[index].state = RasterSlotState::Writing(writing);
            return Err("scene raster sealed before its exact pixels arrived");
        }
        if state.next_lease_id == u64::MAX {
            writing.pixels = Some(pixels);
            state.slots[index].state = RasterSlotState::Writing(writing);
            return Err("scene raster lease epoch exhausted");
        }
        let access = state.access_clock;
        let ready = ReadyRaster { descriptor: writing.descriptor, pixels, content: writing.content, leases: [LeaseRecord::default(); SCENE_RASTER_LEASE_CAPACITY], access };
        state.slots[index].state = RasterSlotState::Ready(ready);
        mint_lease(&self.inner, &mut state, index, access)
    }

    pub fn cancel(&self, writer: SceneRasterWriter) -> bool {
        let Ok(mut state) = self.inner.state.lock() else { return false };
        let index = usize::from(writer.slot);
        let Some(slot) = state.slots.get_mut(index) else { return false };
        if slot.epoch != writer.epoch || !matches!(&slot.state, RasterSlotState::Writing(writing) if writing.token == writer) {
            return false;
        }
        let old = std::mem::replace(&mut slot.state, RasterSlotState::Vacant);
        let RasterSlotState::Writing(mut writing) = old else { return false };
        let pixels = writing.pixels.take().unwrap_or_default();
        slot.state = RasterSlotState::Retiring(RetiringRaster { pixels, reserved_bytes: writing.expected_bytes });
        true
    }

    pub fn maintenance_step(&self) -> bool {
        let Ok(mut state) = self.inner.state.lock() else { return false };
        let Some(index) = state.slots.iter().position(|slot| matches!(slot.state, RasterSlotState::Retiring(_))) else { return true };
        let old = std::mem::replace(&mut state.slots[index].state, RasterSlotState::Vacant);
        let RasterSlotState::Retiring(mut retiring) = old else { return false };
        retiring.pixels = Vec::new();
        let released = retiring.reserved_bytes;
        state.reserved_bytes = state.reserved_bytes.saturating_sub(released);
        false
    }

    pub fn reserved_bytes(&self) -> usize {
        self.inner.state.lock().map_or(0, |state| state.reserved_bytes)
    }

    pub fn live_lease_count(&self, descriptor: SceneRasterDescriptor) -> usize {
        self.inner.state.lock().map_or(0, |state| {
            state
                .slots
                .iter()
                .find_map(|slot| match &slot.state {
                    RasterSlotState::Ready(ready) if ready.descriptor == descriptor => Some(live_lease_count(ready)),
                    _ => None,
                })
                .unwrap_or(0)
        })
    }

    pub fn acquire(&self, identity: SceneRasterIdentity) -> Result<SceneRasterLease, &'static str> {
        let mut state = self.inner.state.lock().map_err(|_| "scene raster pool lock is poisoned")?;
        let Some(access) = state.access_clock.checked_add(1) else { return Err("scene raster access epoch exhausted") };
        let Some(index) = state.slots.iter().position(|slot| matches!(&slot.state, RasterSlotState::Ready(ready) if ready.descriptor == identity.descriptor && ready.content == identity.content)) else {
            return Err("scene raster identity is not CPU resident");
        };
        let lease = mint_lease(&self.inner, &mut state, index, access)?;
        state.access_clock = access;
        Ok(lease)
    }

    pub fn cpu_resident(&self, identity: SceneRasterIdentity) -> bool {
        self.inner.state.lock().is_ok_and(|state| state.slots.iter().any(|slot| matches!(&slot.state, RasterSlotState::Ready(ready) if ready.descriptor == identity.descriptor && ready.content == identity.content)))
    }

    pub fn gpu_resident(&self, key: &str, identity: SceneRasterIdentity) -> bool {
        let Some(key) = SceneRasterGpuKey::new(key) else { return false };
        self.inner.state.lock().is_ok_and(|state| state.gpu_residents.iter().any(|resident| resident.as_ref().is_some_and(|resident| resident.key == key && resident.identity == identity)))
    }

    pub fn gpu_resident_bytes(&self) -> usize {
        self.inner.state.lock().map_or(0, |state| state.gpu_resident_bytes)
    }
}

impl Default for SceneRasterPool {
    fn default() -> Self {
        Self::new()
    }
}

fn exact_writing_mut(state: &mut SceneRasterPoolState, writer: SceneRasterWriter) -> Result<&mut WritingRaster, &'static str> {
    let slot = state.slots.get_mut(usize::from(writer.slot)).ok_or("scene raster writer slot is invalid")?;
    if slot.epoch != writer.epoch {
        return Err("scene raster writer epoch is stale");
    }
    let RasterSlotState::Writing(writing) = &mut slot.state else { return Err("scene raster writer is not active") };
    if writing.token != writer {
        return Err("scene raster writer owner is stale");
    }
    Ok(writing)
}

fn live_lease_count(ready: &ReadyRaster) -> usize {
    ready.leases.iter().filter(|lease| lease.live).count()
}

fn mint_lease(inner: &Arc<SceneRasterPoolInner>, state: &mut SceneRasterPoolState, index: usize, access: u64) -> Result<SceneRasterLease, &'static str> {
    let lease_id = state.next_lease_id;
    let Some(next_lease_id) = lease_id.checked_add(1) else { return Err("scene raster lease epoch exhausted") };
    let (epoch, identity) = {
        let slot = state.slots.get_mut(index).ok_or("scene raster lease slot is invalid")?;
        let RasterSlotState::Ready(ready) = &mut slot.state else { return Err("scene raster lease source is not ready") };
        if live_lease_count(ready) >= inner.limits.lease_capacity_per_slot {
            return Err("scene raster lease credits are exhausted");
        }
        let Some(record) = ready.leases[..inner.limits.lease_capacity_per_slot].iter_mut().find(|lease| !lease.live) else { return Err("scene raster lease credits are exhausted") };
        *record = LeaseRecord { id: lease_id, live: true };
        ready.access = access;
        (slot.epoch, SceneRasterIdentity { descriptor: ready.descriptor, content: ready.content })
    };
    state.next_lease_id = next_lease_id;
    Ok(SceneRasterLease { inner: Arc::clone(inner), token: Some(LeaseToken { slot: index as u16, epoch, lease_id }), identity })
}

#[cfg(test)]
#[path = "../../../🧪️tests/🔬️targets-wgpu-raster-ownership-unit/🦀️.rs"]
mod tests;
