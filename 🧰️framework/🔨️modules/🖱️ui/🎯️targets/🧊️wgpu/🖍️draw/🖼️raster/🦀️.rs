//! 🖼️ Actual bounded raster reservations, resource tables and exact refusal handbacks.
use semio_framework_ui_wgpu_device as gpu_device;
use semio_framework_ui_draw_contract::*;
use bytemuck::{Pod,Zeroable};
use std::mem::size_of;
use gpu_device::util::DeviceExt;
mod kernel_3d_scene{pub use semio_framework_ui_scene_algebra::*;pub use semio_framework_ui_scene_camera::*;pub use semio_framework_ui_scene_mesh::*;pub use semio_framework_ui_scene_pass::*;}
use kernel_3d_scene::{Mat4Math,ProceduralGrid3d,ScenePass3d,SceneRenderProfile3d,PROCEDURAL_GRID_CELL_THICKNESS,PROCEDURAL_GRID_FADE_STRENGTH};
use semio_framework_ui_prepared as prepared;
use semio_framework_ui_raster_owner as raster_ownership;
use semio_framework_ui_theme as theme;
use semio_framework_ui_theme::{geometry,Theme};
#[cfg(test)]
use semio_framework_ui_theme::Rgba;
use prepared::{PreparedRasterPages,RasterContentIdentity};
use semio_framework_ui_render_prepared_shaders as shaders;
pub struct RasterTexture {
    pub texture: gpu_device::Texture,
    pub view: gpu_device::TextureView,
    pub bind_group: gpu_device::BindGroup,
    pub width: u32,
    pub height: u32,
}

pub const RASTER_TEXTURE_TABLE_CAPACITY: usize = 256;
pub const RASTER_TEXTURE_KEY_BYTES: usize = 256;
pub const RASTER_TEXTURE_ITEM_BYTE_CAPACITY: usize = 64 * 1024 * 1024;
pub const RASTER_TEXTURE_TABLE_BYTE_CAPACITY: usize = 256 * 1024 * 1024;
const RASTER_TEXTURE_PROBE_CAPACITY: usize = 8;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct RasterTextureWitness {
    pub scene_revision: u64,
    pub preview_generation: u64,
    pub operation: u64,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RasterTextureCleanupStep {
    Pending { released_roots: u8, released_scalars: u8 },
    Blocked(&'static str),
    Complete,
}

impl RasterTextureCleanupStep {
    fn root() -> Self {
        Self::Pending { released_roots: 1, released_scalars: 0 }
    }

    fn scalar() -> Self {
        Self::Pending { released_roots: 0, released_scalars: 1 }
    }

    fn retained() -> Self {
        Self::Pending { released_roots: 0, released_scalars: 0 }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct RasterTextureKey {
    bytes: [u8; RASTER_TEXTURE_KEY_BYTES],
    len: u16,
    hash: u64,
}

pub struct RasterKeepSetV1 {
    slots: Box<[Option<RasterTextureKey>; RASTER_TEXTURE_TABLE_CAPACITY]>,
    len: usize,
}

impl Default for RasterKeepSetV1 {
    fn default() -> Self {
        Self { slots: Box::new(std::array::from_fn(|_| None)), len: 0 }
    }
}

impl RasterKeepSetV1 {
    fn insert_key(&mut self, key: RasterTextureKey) -> Result<(), &'static str> {
        if self.slots.iter().flatten().any(|retained| *retained == key) {
            return Ok(());
        }
        let Some(slot) = self.slots.iter_mut().find(|slot| slot.is_none()) else { return Err("raster keep-set item credits exhausted") };
        *slot = Some(key);
        self.len += 1;
        Ok(())
    }

    pub fn insert(&mut self, key: &str) -> Result<(), &'static str> {
        self.insert_key(RasterTextureKey::new(key)?)
    }

    fn contains(&self, key: RasterTextureKey) -> bool {
        self.slots.iter().flatten().any(|retained| *retained == key)
    }

    fn remove(&mut self, key: RasterTextureKey) {
        if let Some(slot) = self.slots.iter_mut().find(|slot| slot.is_some_and(|retained| retained == key)) {
            *slot = None;
            self.len = self.len.saturating_sub(1);
        }
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn is_empty(&self) -> bool {
        self.len == 0
    }
}

#[derive(Default)]
pub struct RasterResidencyLedger {
    committed: RasterKeepSetV1,
    candidate: RasterKeepSetV1,
    previous: RasterKeepSetV1,
    candidate_witness: Option<RasterTextureWitness>,
    candidate_sealed: bool,
}

impl RasterResidencyLedger {
    pub fn begin_candidate(&mut self, witness: RasterTextureWitness) -> Result<(), &'static str> {
        if self.candidate_witness.is_some() || !self.candidate.is_empty() {
            return Err("raster keep-set candidate was occupied");
        }
        self.candidate_witness = Some(witness);
        self.candidate_sealed = false;
        Ok(())
    }

    pub fn publish_candidate_key(&mut self, witness: RasterTextureWitness, key: &str) -> Result<(), &'static str> {
        if self.candidate_witness != Some(witness) || self.candidate_sealed {
            return Err("raster keep-set candidate witness was stale or sealed");
        }
        self.candidate.insert(key)
    }

    pub fn seal_candidate(&mut self, witness: RasterTextureWitness) -> Result<(), &'static str> {
        if self.candidate_witness != Some(witness) {
            return Err("raster keep-set candidate witness was stale before seal");
        }
        self.candidate_sealed = true;
        Ok(())
    }

    fn candidate_is_sealed(&self, witness: RasterTextureWitness) -> bool {
        self.candidate_witness == Some(witness) && self.candidate_sealed
    }

    fn protects(&self, key: RasterTextureKey) -> bool {
        self.committed.contains(key) || self.candidate.contains(key) || self.previous.contains(key)
    }

    fn commit(&mut self, witness: RasterTextureWitness) -> Result<(), &'static str> {
        if !self.candidate_is_sealed(witness) {
            return Err("raster keep-set candidate was not sealed before commit");
        }
        if !self.previous.is_empty() {
            return Err("raster previous keep-set was not released before commit");
        }
        self.previous = std::mem::take(&mut self.committed);
        self.committed = std::mem::take(&mut self.candidate);
        self.candidate_witness = None;
        self.candidate_sealed = false;
        Ok(())
    }

    fn abort(&mut self, witness: RasterTextureWitness) -> Result<(), &'static str> {
        if self.candidate_witness.is_none() && self.candidate.is_empty() {
            return Ok(());
        }
        if self.candidate_witness != Some(witness) {
            return Err("raster keep-set candidate witness was stale before abort");
        }
        self.candidate = RasterKeepSetV1::default();
        self.candidate_witness = None;
        self.candidate_sealed = false;
        Ok(())
    }

    pub fn release_previous(&mut self) {
        self.previous = RasterKeepSetV1::default();
    }

    fn remove_key(&mut self, key: RasterTextureKey) {
        self.committed.remove(key);
        self.candidate.remove(key);
        self.previous.remove(key);
    }

    pub fn counts(&self) -> (usize, usize, usize) {
        (self.committed.len(), self.candidate.len(), self.previous.len())
    }

    fn clear(&mut self) {
        *self = Self::default();
    }

    fn is_empty(&self) -> bool {
        self.committed.is_empty() && self.candidate.is_empty() && self.previous.is_empty() && self.candidate_witness.is_none() && !self.candidate_sealed
    }
}

impl RasterTextureKey {
    fn new(key: &str) -> Result<Self, &'static str> {
        if key.is_empty() || key.len() > RASTER_TEXTURE_KEY_BYTES {
            return Err("raster texture key exceeded fixed credits");
        }
        let mut bytes = [0; RASTER_TEXTURE_KEY_BYTES];
        bytes[..key.len()].copy_from_slice(key.as_bytes());
        let mut hash = 0xcbf29ce484222325u64;
        for byte in key.as_bytes() {
            hash ^= u64::from(*byte);
            hash = hash.wrapping_mul(0x100000001b3);
        }
        Ok(Self { bytes, len: key.len() as u16, hash })
    }

    fn as_str(&self) -> &str {
        std::str::from_utf8(&self.bytes[..usize::from(self.len)]).expect("raster key was admitted from UTF-8")
    }

    fn start(self) -> usize {
        self.hash as usize % RASTER_TEXTURE_TABLE_CAPACITY
    }
}

fn raster_texture_bytes(width: u32, height: u32) -> Option<usize> {
    usize::try_from(width).ok()?.checked_mul(usize::try_from(height).ok()?)?.checked_mul(4)
}

fn raster_mip_bytes(width:u32,height:u32,levels:u32)->Option<usize>{if levels==0 || width==0 || height==0 || levels>u32::BITS-width.max(height).leading_zeros(){return None}let mut bytes=0usize;for level in 0..levels {bytes=bytes.checked_add(raster_texture_bytes((width>>level).max(1),(height>>level).max(1))?)?;}Some(bytes)}

fn raster_admission_fits(retained_items: usize, retained_bytes: usize, bytes: usize) -> bool {
    retained_items < RASTER_TEXTURE_TABLE_CAPACITY && retained_bytes.checked_add(bytes).is_some_and(|total| total <= RASTER_TEXTURE_TABLE_BYTE_CAPACITY)
}

fn raster_witness_is_stale(current: RasterTextureWitness, candidate: RasterTextureWitness) -> bool {
    current.scene_revision > candidate.scene_revision
        || (current.scene_revision == candidate.scene_revision && current.preview_generation > candidate.preview_generation)
        || (current.scene_revision == candidate.scene_revision && current.preview_generation == candidate.preview_generation && current.operation >= candidate.operation)
}

struct RasterTextureEntry<T> {
    key: RasterTextureKey,
    witness: RasterTextureWitness,
    content_identity: RasterContentIdentity,
    bytes: usize,
    cpu_release: Option<crate::raster_ownership::SceneRasterReleaseWitness>,
    gpu_resident: Option<crate::raster_ownership::SceneRasterGpuWitness>,
    value: T,
}

struct FixedRasterTextureRegistry<T> {
    slots: Box<[Option<RasterTextureEntry<T>>; RASTER_TEXTURE_TABLE_CAPACITY]>,
    len: usize,
    bytes: usize,
}

impl<T> Default for FixedRasterTextureRegistry<T> {
    fn default() -> Self {
        Self { slots: Box::new(std::array::from_fn(|_| None)), len: 0, bytes: 0 }
    }
}

impl<T> FixedRasterTextureRegistry<T> {
    fn locate(&self, key: RasterTextureKey) -> Result<Result<usize, usize>, &'static str> {
        let start = key.start();
        let mut vacant = None;
        for offset in 0..RASTER_TEXTURE_PROBE_CAPACITY {
            let index = (start + offset) % RASTER_TEXTURE_TABLE_CAPACITY;
            match self.slots[index].as_ref() {
                Some(entry) if entry.key == key => return Ok(Ok(index)),
                None if vacant.is_none() => vacant = Some(index),
                _ => {}
            }
        }
        vacant.map(Err).ok_or("raster texture probe credits exhausted")
    }

    fn get(&self, key: &str) -> Option<&RasterTextureEntry<T>> {
        let key = RasterTextureKey::new(key).ok()?;
        let Ok(index) = self.locate(key).ok()? else { return None };
        self.slots[index].as_ref()
    }

    fn get_mut(&mut self, key: &str) -> Option<&mut RasterTextureEntry<T>> {
        let key = RasterTextureKey::new(key).ok()?;
        let Ok(index) = self.locate(key).ok()? else { return None };
        self.slots[index].as_mut()
    }

    #[expect(clippy::result_large_err, reason = "GPU admission returns the exact resource and reservation owner without allocating during refusal.")]
    fn insert(&mut self, entry: RasterTextureEntry<T>) -> Result<Option<RasterTextureEntry<T>>, RasterTextureEntry<T>> {
        let index = match self.locate(entry.key) {
            Ok(Ok(index)) | Ok(Err(index)) => index,
            Err(_) => return Err(entry),
        };
        let previous = self.slots[index].replace(entry);
        match previous.as_ref() {
            Some(previous) => {
                self.bytes = self.bytes.saturating_sub(previous.bytes);
            }
            None => self.len += 1,
        }
        self.bytes = self.bytes.saturating_add(self.slots[index].as_ref().expect("inserted raster entry").bytes);
        Ok(previous)
    }

    #[expect(clippy::result_large_err, reason = "GPU admission returns the exact resource and reservation owner without allocating during refusal.")]
    fn insert_vacant(&mut self, index: usize, entry: RasterTextureEntry<T>) -> Result<(), RasterTextureEntry<T>> {
        let Some(slot) = self.slots.get_mut(index) else { return Err(entry) };
        if slot.is_some() {
            return Err(entry);
        }
        *slot = Some(entry);
        self.len += 1;
        self.bytes = self.bytes.saturating_add(slot.as_ref().expect("inserted raster entry").bytes);
        Ok(())
    }

    fn take(&mut self, index: usize) -> Option<RasterTextureEntry<T>> {
        let value = self.slots.get_mut(index)?.take();
        if let Some(value) = value.as_ref() {
            self.len = self.len.saturating_sub(1);
            self.bytes = self.bytes.saturating_sub(value.bytes);
        }
        value
    }

    fn is_empty(&self) -> bool {
        self.len == 0
    }
}

fn take_unowned_live_entry<T>(live: &mut FixedRasterTextureRegistry<T>, staged: &FixedRasterTextureRegistry<T>, residency: &RasterResidencyLedger, index: usize) -> Option<RasterTextureEntry<T>> {
    let selected = live.slots[index].as_ref().is_some_and(|entry| !residency.protects(entry.key) && staged.get(entry.key.as_str()).is_none());
    selected.then(|| live.take(index)).flatten()
}

fn raster_content_is_reusable<T>(live: &FixedRasterTextureRegistry<T>, staged: &FixedRasterTextureRegistry<T>, key: RasterTextureKey, identity: RasterContentIdentity, witness: RasterTextureWitness) -> Result<bool, &'static str> {
    if let Some(entry) = staged.get(key.as_str()) {
        if entry.witness != witness {
            return Err("raster staged key was owned by another generation");
        }
        return (entry.content_identity == identity).then_some(true).ok_or("raster key had conflicting content in one candidate");
    }
    Ok(live.get(key.as_str()).is_some_and(|entry| entry.content_identity == identity))
}

pub struct RasterTextureAdmission {
    key: RasterTextureKey,
    witness: RasterTextureWitness,
    content_identity: RasterContentIdentity,
    width: u32,
    height: u32,
    bytes: usize,
    staged_index: usize,
    nonce: u64,
}

#[expect(clippy::large_enum_variant, reason = "Cancellation refusal returns the exact original admission without allocating another owner.")]
pub enum RasterTextureCancelFault {
    Returned { fault: &'static str, admission: RasterTextureAdmission },
    Retained(&'static str),
}

#[expect(clippy::large_enum_variant, reason = "Failed staging returns the exact admitted texture and view without allocating during refusal.")]
pub enum RasterTextureStageFault {
    Returned { fault: &'static str, admission: RasterTextureAdmission, texture: gpu_device::Texture, view: gpu_device::TextureView },
    Retained(&'static str),
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct RasterTextureReservation {
    key: RasterTextureKey,
    witness: RasterTextureWitness,
    content_identity: RasterContentIdentity,
    width: u32,
    height: u32,
    bytes: usize,
    staged_index: usize,
    nonce: u64,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct RasterTextureStageClaim {
    reservation: RasterTextureReservation,
    candidate: RasterTextureWitness,
    staged_index: usize,
    staged_nonce: u64,
}

impl RasterTextureReservation {
    fn matches(&self, admission: &RasterTextureAdmission) -> bool {
        self.key == admission.key
            && self.witness == admission.witness
            && self.content_identity == admission.content_identity
            && self.width == admission.width
            && self.height == admission.height
            && self.bytes == admission.bytes
            && self.staged_index == admission.staged_index
            && self.nonce == admission.nonce
    }
}

fn claim_raster_stage_tuple(
    reservation: Option<RasterTextureReservation>,
    candidate: Option<RasterTextureWitness>,
    staged_occupied: bool,
    admission: &RasterTextureAdmission,
    expected: RasterTextureWitness,
) -> Result<RasterTextureStageClaim, &'static str> {
    if admission.witness != expected {
        return Err("raster operation authority was stale before GPU allocation");
    }
    let Some(reservation) = reservation else {
        return Err("raster texture reservation was missing before GPU allocation");
    };
    if !reservation.matches(admission) {
        return Err("raster texture reservation was stale before GPU allocation");
    }
    if candidate != Some(expected) {
        return Err("raster candidate witness changed before GPU allocation");
    }
    if staged_occupied {
        return Err("raster staged slot changed before GPU allocation");
    }
    Ok(RasterTextureStageClaim { reservation, candidate: expected, staged_index: admission.staged_index, staged_nonce: admission.nonce })
}

struct RasterTextureReservationRetirement {
    key: Option<RasterTextureKey>,
    content_identity: Option<RasterContentIdentity>,
    scene_revision: Option<u64>,
    preview_generation: Option<u64>,
    operation: Option<u64>,
    width: Option<u32>,
    height: Option<u32>,
    bytes: Option<usize>,
    staged_index: Option<usize>,
    nonce: Option<u64>,
}

impl RasterTextureReservationRetirement {
    fn new(reservation: RasterTextureReservation) -> Self {
        Self {
            key: Some(reservation.key),
            content_identity: Some(reservation.content_identity),
            scene_revision: Some(reservation.witness.scene_revision),
            preview_generation: Some(reservation.witness.preview_generation),
            operation: Some(reservation.witness.operation),
            width: Some(reservation.width),
            height: Some(reservation.height),
            bytes: Some(reservation.bytes),
            staged_index: Some(reservation.staged_index),
            nonce: Some(reservation.nonce),
        }
    }

    fn step(&mut self) -> RasterTextureCleanupStep {
        if self.key.take().is_some() {
            return RasterTextureCleanupStep::root();
        }
        if self.content_identity.take().is_some() {
            return RasterTextureCleanupStep::scalar();
        }
        if self.scene_revision.take().is_some() {
            return RasterTextureCleanupStep::scalar();
        }
        if self.preview_generation.take().is_some() {
            return RasterTextureCleanupStep::scalar();
        }
        if self.operation.take().is_some() {
            return RasterTextureCleanupStep::scalar();
        }
        if self.width.take().is_some() {
            return RasterTextureCleanupStep::scalar();
        }
        if self.height.take().is_some() {
            return RasterTextureCleanupStep::scalar();
        }
        if self.bytes.take().is_some() {
            return RasterTextureCleanupStep::scalar();
        }
        if self.staged_index.take().is_some() {
            return RasterTextureCleanupStep::scalar();
        }
        if self.nonce.take().is_some() {
            return RasterTextureCleanupStep::scalar();
        }
        RasterTextureCleanupStep::Complete
    }

    fn terminal_is_empty(&self) -> bool {
        self.key.is_none()
            && self.content_identity.is_none()
            && self.scene_revision.is_none()
            && self.preview_generation.is_none()
            && self.operation.is_none()
            && self.width.is_none()
            && self.height.is_none()
            && self.bytes.is_none()
            && self.staged_index.is_none()
            && self.nonce.is_none()
    }
}

impl RasterTextureAdmission {
    fn into_retirement(self) -> RasterTextureReservationRetirement {
        RasterTextureReservationRetirement::new(RasterTextureReservation {
            key: self.key,
            witness: self.witness,
            content_identity: self.content_identity,
            width: self.width,
            height: self.height,
            bytes: self.bytes,
            staged_index: self.staged_index,
            nonce: self.nonce,
        })
    }
}

struct RasterTextureReservationCloseCursor {
    reservation_retirement: Option<RasterTextureReservationRetirement>,
    admission_retirement: Option<RasterTextureReservationRetirement>,
}

impl RasterTextureReservationCloseCursor {
    fn reservation(reservation: RasterTextureReservation) -> Self {
        Self { reservation_retirement: Some(RasterTextureReservationRetirement::new(reservation)), admission_retirement: None }
    }

    fn cancelled(reservation: RasterTextureReservation, admission: RasterTextureAdmission) -> Self {
        Self { reservation_retirement: Some(RasterTextureReservationRetirement::new(reservation)), admission_retirement: Some(admission.into_retirement()) }
    }

    fn rejected(admission: RasterTextureAdmission) -> Self {
        Self { reservation_retirement: None, admission_retirement: Some(admission.into_retirement()) }
    }

    fn step(&mut self) -> RasterTextureCleanupStep {
        if let Some(retirement) = self.reservation_retirement.as_mut() {
            let step = retirement.step();
            if matches!(step, RasterTextureCleanupStep::Complete) {
                assert!(retirement.terminal_is_empty(), "completed raster reservation must be terminal-empty");
                self.reservation_retirement = None;
                return RasterTextureCleanupStep::retained();
            }
            return step;
        }
        if let Some(retirement) = self.admission_retirement.as_mut() {
            let step = retirement.step();
            if matches!(step, RasterTextureCleanupStep::Complete) {
                assert!(retirement.terminal_is_empty(), "completed raster admission must be terminal-empty");
                self.admission_retirement = None;
                return RasterTextureCleanupStep::retained();
            }
            return step;
        }
        RasterTextureCleanupStep::Complete
    }

    fn terminal_is_empty(&self) -> bool {
        self.reservation_retirement.is_none() && self.admission_retirement.is_none()
    }

    fn retained_bytes(&self) -> usize {
        self.reservation_retirement.as_ref().and_then(|retirement| retirement.bytes).unwrap_or(0) + self.admission_retirement.as_ref().and_then(|retirement| retirement.bytes).unwrap_or(0)
    }

    fn retained_items(&self) -> usize {
        usize::from(self.reservation_retirement.is_some()) + usize::from(self.admission_retirement.is_some())
    }
}

struct RasterTextureStageClaimRetirement {
    reservation: RasterTextureReservationRetirement,
    candidate: RasterTextureWitnessSlot,
    staged_index: Option<usize>,
    staged_nonce: Option<u64>,
}

impl RasterTextureStageClaimRetirement {
    fn new(claim: RasterTextureStageClaim) -> Self {
        let mut candidate = RasterTextureWitnessSlot::default();
        candidate.set(claim.candidate);
        Self { reservation: RasterTextureReservationRetirement::new(claim.reservation), candidate, staged_index: Some(claim.staged_index), staged_nonce: Some(claim.staged_nonce) }
    }

    fn step(&mut self) -> RasterTextureCleanupStep {
        let reservation = self.reservation.step();
        if !matches!(reservation, RasterTextureCleanupStep::Complete) {
            return reservation;
        }
        if !self.candidate.is_empty() {
            self.candidate.retire_one();
            return RasterTextureCleanupStep::scalar();
        }
        if self.staged_index.take().is_some() {
            return RasterTextureCleanupStep::scalar();
        }
        if self.staged_nonce.take().is_some() {
            return RasterTextureCleanupStep::scalar();
        }
        RasterTextureCleanupStep::Complete
    }

    fn terminal_is_empty(&self) -> bool {
        self.reservation.terminal_is_empty() && self.candidate.is_empty() && self.staged_index.is_none() && self.staged_nonce.is_none()
    }
}

#[derive(Default)]
struct RasterTextureWitnessSlot {
    scene_revision: Option<u64>,
    preview_generation: Option<u64>,
    operation: Option<u64>,
}

impl RasterTextureWitnessSlot {
    fn get(&self) -> Option<RasterTextureWitness> {
        Some(RasterTextureWitness { scene_revision: self.scene_revision?, preview_generation: self.preview_generation?, operation: self.operation? })
    }

    fn set(&mut self, witness: RasterTextureWitness) {
        self.scene_revision = Some(witness.scene_revision);
        self.preview_generation = Some(witness.preview_generation);
        self.operation = Some(witness.operation);
    }

    fn retire_one(&mut self) -> bool {
        if self.scene_revision.take().is_some() {
            return false;
        }
        if self.preview_generation.take().is_some() {
            return false;
        }
        if self.operation.take().is_some() {
            return false;
        }
        true
    }

    fn is_empty(&self) -> bool {
        self.scene_revision.is_none() && self.preview_generation.is_none() && self.operation.is_none()
    }
}

/// 🌱 What one raster operation still owes when its retirement is admitted.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RasterWitnessAdmission {
    /// 🌱 The operation admitted no raster at all. An engine surface whose first raster has not been
    /// reserved yet owes NOTHING — that is "not yet", and it must never read as "stale".
    Nothing,
    /// ♻️ The operation owns witnesses a bounded retirement has to consume.
    Retire,
}

/// 🪪️ The candidate/presentation witness pair one raster operation owns, and the bounded retirement
/// that consumes it — device-free, so the lifecycle law can drive the production authority itself.
///
/// ⚖️ Admission is a ONE-TIME fact, not a per-step invariant. The retirement retires these very slots
/// one field at a time (`RasterTextureWitnessSlot::retire_one`), so a half-consumed slot compares
/// unequal to the operation that owns it; re-reading the pair on every later step made a commit refuse
/// its own progress with `raster commit candidate witness was stale`
/// (ticket 26/09/09/PROCEDURAL-3D-END-TO-END). The law is
/// `🖱️ui/🧪️tests/🧫️fixtures/🖼️raster-witness-lifecycle/🔣️.json`.
#[derive(Default)]
pub struct RasterOperationWitnessLedger {
    candidate: RasterTextureWitnessSlot,
    presenting: RasterTextureWitnessSlot,
    retiring: Option<RasterTextureWitness>,
    candidate_retired: bool,
    presenting_retired: bool,
}

impl RasterOperationWitnessLedger {
    pub fn candidate(&self) -> Option<RasterTextureWitness> {
        self.candidate.get()
    }

    pub fn presenting(&self) -> Option<RasterTextureWitness> {
        self.presenting.get()
    }

    pub fn candidate_is_empty(&self) -> bool {
        self.candidate.is_empty()
    }

    pub fn presenting_is_empty(&self) -> bool {
        self.presenting.is_empty()
    }

    pub fn is_empty(&self) -> bool {
        self.candidate.is_empty() && self.presenting.is_empty() && self.retiring.is_none() && !self.candidate_retired && !self.presenting_retired
    }

    /// 🧭️ The operation whose retirement already owns these slots, if any.
    pub fn retiring(&self) -> Option<RasterTextureWitness> {
        self.retiring
    }

    pub fn candidate_occupied_by(&self, witness: RasterTextureWitness) -> bool {
        self.candidate.get().is_some_and(|current| current != witness)
    }

    /// 🎟️ Claims the candidate slot for this operation; a slot already carrying it is left untouched
    /// so every raster of one frame shares one candidate.
    pub fn admit_candidate(&mut self, witness: RasterTextureWitness) {
        if self.candidate.get().is_none() {
            self.candidate.set(witness);
        }
    }

    pub fn arm_presentation(&mut self, witness: RasterTextureWitness) -> Result<(), &'static str> {
        if self.candidate.get() != Some(witness) {
            return Err("raster candidate witness was stale before presentation");
        }
        if !self.presenting.is_empty() {
            return Err("raster presentation witness was already occupied");
        }
        self.presenting.set(witness);
        Ok(())
    }

    /// ⚖️ Reads what this operation owes. TOTAL: an operation whose own retirement already owns the
    /// slots reads `Retire` however far that retirement has consumed them — a half-empty slot is its
    /// cursor's progress, never a foreign authority's staleness.
    pub fn retirement_admission(&self, witness: RasterTextureWitness) -> Result<RasterWitnessAdmission, &'static str> {
        if let Some(retiring) = self.retiring {
            return if retiring == witness { Ok(RasterWitnessAdmission::Retire) } else { Err("raster retirement authority was occupied") };
        }
        if self.candidate.is_empty() && self.presenting.is_empty() {
            return Ok(RasterWitnessAdmission::Nothing);
        }
        if self.candidate.get() != Some(witness) {
            return Err(if self.candidate.is_empty() { "raster retirement candidate was already retired" } else { "raster retirement candidate witness was stale" });
        }
        if !self.presenting.is_empty() && self.presenting.get() != Some(witness) {
            return Err("raster retirement presentation witness was stale");
        }
        Ok(RasterWitnessAdmission::Retire)
    }

    /// 🎟️ Admits this operation's retirement and hands it the slots. Idempotent for the operation
    /// that already owns them, so a bounded step may re-enter as often as its budget requires.
    pub fn begin_retirement(&mut self, witness: RasterTextureWitness) -> Result<RasterWitnessAdmission, &'static str> {
        let admission = self.retirement_admission(witness)?;
        if admission == RasterWitnessAdmission::Retire {
            self.retiring = Some(witness);
        }
        Ok(admission)
    }

    /// ♻️ Retires ONE witness field per call; `true` once both slots are terminal and the ledger is
    /// ready for the next operation.
    pub fn retire_step(&mut self) -> bool {
        if !self.candidate_retired {
            self.candidate_retired = self.candidate.retire_one();
            return false;
        }
        if !self.presenting_retired {
            self.presenting_retired = self.presenting.retire_one();
            return false;
        }
        self.candidate_retired = false;
        self.presenting_retired = false;
        self.retiring = None;
        true
    }
}

struct RasterTextureUploadCursor {
    mip_levels:u32,
    mip:u32,
    mip_row:u32,
    admission: Option<RasterTextureAdmission>,
    row: u32,
    texture: Option<gpu_device::Texture>,
    view: Option<gpu_device::TextureView>,
    bind_group: Option<gpu_device::BindGroup>,
    allocation_claim: Option<RasterTextureStageClaim>,
    cpu_release: Option<crate::raster_ownership::SceneRasterReleaseWitness>,
}

#[derive(Clone, Copy)]
pub enum RasterUploadPixels<'a> {
    #[cfg(test)]
    Contiguous(&'a [u8]),
    Pages(&'a PreparedRasterPages),
    Scene(&'a crate::raster_ownership::SceneRasterLease),
}

impl RasterUploadPixels<'_> {
    fn len(&self) -> usize {
        match self {
            #[cfg(test)]
            Self::Contiguous(pixels) => pixels.len(),
            Self::Pages(pixels) => pixels.byte_len(),
            Self::Scene(lease) => lease.byte_len(),
        }
    }

    fn dimensions_match(&self, width: u32, height: u32) -> bool {
        match self {
            #[cfg(test)]
            Self::Contiguous(_) => true,
            Self::Pages(pixels) => pixels.width() == width && pixels.height() == height,
            Self::Scene(lease) => {
                let descriptor = lease.identity().descriptor();
                descriptor.width == width && descriptor.height == height
            }
        }
    }

    fn content_identity(&self, _width: u32, _height: u32) -> Option<RasterContentIdentity> {
        match self {
            #[cfg(test)]
            Self::Contiguous(pixels) => RasterContentIdentity::test_pixels(width, height, pixels),
            Self::Pages(pixels) => Some(pixels.content_identity()),
            Self::Scene(lease) => Some(lease.identity().content()),
        }
    }

    fn with_rows<R>(&self, row: u32, _start: usize, _end: usize, _rows: u32, read: impl FnOnce(&[u8], u32) -> R) -> Option<R> {
        match self {
            #[cfg(test)]
            Self::Contiguous(pixels) => pixels.get(start..end).map(|page| read(page, rows)),
            Self::Pages(pixels) => pixels.page_for_row(row).map(|(page, page_rows)| read(page, page_rows)),
            Self::Scene(lease) => lease.with_rows(row, lease.transfer_bytes(), read).ok(),
        }
    }

    fn transfer_bytes(&self) -> usize {
        match self {
            Self::Scene(lease) => lease.transfer_bytes(),
            #[cfg(test)]
            Self::Contiguous(_) => crate::prepared::PREPARED_RASTER_PAGE_BYTES,
            Self::Pages(_) => crate::prepared::PREPARED_RASTER_PAGE_BYTES,
        }
    }

    fn release_witness(&self) -> Option<crate::raster_ownership::SceneRasterReleaseWitness> {
        match self {
            Self::Scene(lease) => lease.release_witness(),
            #[cfg(test)]
            Self::Contiguous(_) => None,
            Self::Pages(_) => None,
        }
    }

    fn release_reused(&self) {
        if let Self::Scene(lease) = self {
            let _ = lease.release_committed();
        }
    }

    pub fn mip_levels(&self,width:u32,height:u32)->u32{if matches!(self,Self::Scene(lease) if matches!(lease.identity().descriptor().profile,crate::raster_ownership::SceneRasterProfile::MeshBaseColorSrgb|crate::raster_ownership::SceneRasterProfile::MeshNumericLinear)){u32::BITS-width.max(height).leading_zeros()}else{1}}

    fn texture_format(&self) -> gpu_device::TextureFormat {
        if matches!(self,Self::Scene(lease) if lease.identity().descriptor().profile==crate::raster_ownership::SceneRasterProfile::MeshNumericLinear){gpu_device::TextureFormat::Rgba8Unorm}else{gpu_device::TextureFormat::Rgba8UnormSrgb}
    }
}

struct RasterTextureRetirementOwner {
    key: Option<RasterTextureKey>,
    content_identity: Option<RasterContentIdentity>,
    scene_revision: Option<u64>,
    preview_generation: Option<u64>,
    operation: Option<u64>,
    width: Option<u32>,
    height: Option<u32>,
    bytes: Option<usize>,
    bind_group: Option<gpu_device::BindGroup>,
    view: Option<gpu_device::TextureView>,
    texture: Option<gpu_device::Texture>,
    cpu_release: Option<crate::raster_ownership::SceneRasterReleaseWitness>,
    gpu_resident: Option<crate::raster_ownership::SceneRasterGpuWitness>,
}

impl RasterTextureRetirementOwner {
    fn new(entry: RasterTextureEntry<RasterTexture>) -> Self {
        Self {
            key: Some(entry.key),
            content_identity: Some(entry.content_identity),
            scene_revision: Some(entry.witness.scene_revision),
            preview_generation: Some(entry.witness.preview_generation),
            operation: Some(entry.witness.operation),
            width: Some(entry.value.width),
            height: Some(entry.value.height),
            bytes: Some(entry.bytes),
            bind_group: Some(entry.value.bind_group),
            view: Some(entry.value.view),
            texture: Some(entry.value.texture),
            cpu_release: entry.cpu_release,
            gpu_resident: entry.gpu_resident,
        }
    }

    fn step(&mut self) -> RasterTextureCleanupStep {
        if self.bind_group.take().is_some() {
            return RasterTextureCleanupStep::root();
        }
        if self.view.take().is_some() {
            return RasterTextureCleanupStep::root();
        }
        if let Some(texture) = self.texture.take() {
            texture.destroy();
            return RasterTextureCleanupStep::root();
        }
        if let Some(release) = self.cpu_release.take() {
            let _ = release.release();
            return RasterTextureCleanupStep::scalar();
        }
        if self.gpu_resident.take().is_some() {
            return RasterTextureCleanupStep::scalar();
        }
        if self.key.take().is_some() {
            return RasterTextureCleanupStep::root();
        }
        if self.content_identity.take().is_some() {
            return RasterTextureCleanupStep::scalar();
        }
        if self.scene_revision.take().is_some() {
            return RasterTextureCleanupStep::scalar();
        }
        if self.preview_generation.take().is_some() {
            return RasterTextureCleanupStep::scalar();
        }
        if self.operation.take().is_some() {
            return RasterTextureCleanupStep::scalar();
        }
        if self.width.take().is_some() {
            return RasterTextureCleanupStep::scalar();
        }
        if self.height.take().is_some() {
            return RasterTextureCleanupStep::scalar();
        }
        if self.bytes.take().is_some() {
            return RasterTextureCleanupStep::scalar();
        }
        RasterTextureCleanupStep::Complete
    }

    fn terminal_is_empty(&self) -> bool {
        self.key.is_none()
            && self.content_identity.is_none()
            && self.scene_revision.is_none()
            && self.preview_generation.is_none()
            && self.operation.is_none()
            && self.width.is_none()
            && self.height.is_none()
            && self.bytes.is_none()
            && self.bind_group.is_none()
            && self.view.is_none()
            && self.texture.is_none()
            && self.cpu_release.is_none()
            && self.gpu_resident.is_none()
    }

    fn retained_bytes(&self) -> usize {
        self.bytes.unwrap_or(0)
    }
}

struct RasterTextureUploadCloseCursor {
    source: Option<RasterTextureUploadCursor>,
    admission: Option<RasterTextureAdmission>,
    admission_retirement: Option<RasterTextureReservationRetirement>,
    allocation_claim_retirement: Option<RasterTextureStageClaimRetirement>,
    owner: Option<RasterTextureRetirementOwner>,
    row_retired: bool,
}

impl RasterTextureUploadCloseCursor {
    fn new(source: RasterTextureUploadCursor) -> Self {
        Self { source: Some(source), admission: None, admission_retirement: None, allocation_claim_retirement: None, owner: None, row_retired: false }
    }

    fn step(&mut self) -> RasterTextureCleanupStep {
        if let Some(owner) = self.owner.as_mut() {
            let step = owner.step();
            if matches!(step, RasterTextureCleanupStep::Complete) {
                assert!(owner.terminal_is_empty(), "completed raster GPU owner must be terminal-empty");
                self.owner = None;
                return RasterTextureCleanupStep::retained();
            }
            return step;
        }
        if let Some(retirement) = self.allocation_claim_retirement.as_mut() {
            let step = retirement.step();
            if matches!(step, RasterTextureCleanupStep::Complete) {
                assert!(retirement.terminal_is_empty(), "completed raster allocation claim must be terminal-empty");
                self.allocation_claim_retirement = None;
                return RasterTextureCleanupStep::retained();
            }
            return step;
        }
        if let Some(retirement) = self.admission_retirement.as_mut() {
            let step = retirement.step();
            if matches!(step, RasterTextureCleanupStep::Complete) {
                assert!(retirement.terminal_is_empty(), "completed raster admission must be terminal-empty");
                self.admission_retirement = None;
                return RasterTextureCleanupStep::retained();
            }
            return step;
        }
        if let Some(source) = self.source.as_mut() {
            if self.admission.is_none() {
                if let Some(admission) = source.admission.take() {
                    self.admission = Some(admission);
                    return RasterTextureCleanupStep::retained();
                }
            }
            if let Some(claim) = source.allocation_claim.take() {
                self.allocation_claim_retirement = Some(RasterTextureStageClaimRetirement::new(claim));
                return RasterTextureCleanupStep::retained();
            }
            if source.texture.is_some() || source.view.is_some() || source.bind_group.is_some() {
                let admission = self.admission.as_ref().expect("retained upload close admission");
                self.owner = Some(RasterTextureRetirementOwner {
                    key: Some(admission.key),
                    content_identity: Some(admission.content_identity),
                    scene_revision: Some(admission.witness.scene_revision),
                    preview_generation: Some(admission.witness.preview_generation),
                    operation: Some(admission.witness.operation),
                    width: Some(admission.width),
                    height: Some(admission.height),
                    bytes: Some(admission.bytes),
                    bind_group: source.bind_group.take(),
                    view: source.view.take(),
                    texture: source.texture.take(),
                    cpu_release: source.cpu_release.take(),
                    gpu_resident: None,
                });
                return RasterTextureCleanupStep::retained();
            }
            if !self.row_retired {
                source.row = 0;source.mip_levels=0;source.mip=0;source.mip_row=0;
                self.row_retired = true;
                return RasterTextureCleanupStep::scalar();
            }
            if let Some(release) = source.cpu_release.take() {
                let _ = release.release();
                return RasterTextureCleanupStep::scalar();
            }
            self.source = None;
            return RasterTextureCleanupStep::retained();
        }
        if let Some(admission) = self.admission.take() {
            self.admission_retirement = Some(admission.into_retirement());
            return RasterTextureCleanupStep::retained();
        }
        RasterTextureCleanupStep::Complete
    }

    fn terminal_is_empty(&self) -> bool {
        self.source.is_none() && self.admission.is_none() && self.admission_retirement.is_none() && self.allocation_claim_retirement.is_none() && self.owner.is_none()
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum RasterTextureRetirementMode {
    Abort(RasterTextureWitness),
    Commit(RasterTextureWitness),
    Unowned,
    Exact(RasterTextureKey),
    Close,
}

struct RasterTextureRetirementCursor {
    mode: RasterTextureRetirementMode,
    scan: usize,
    owner: Option<RasterTextureRetirementOwner>,
}

pub struct RasterTextureTable {
    live: FixedRasterTextureRegistry<RasterTexture>,
    staged: FixedRasterTextureRegistry<RasterTexture>,
    upload: Option<RasterTextureUploadCursor>,
    upload_close: Option<RasterTextureUploadCloseCursor>,
    retirement: Option<RasterTextureRetirementCursor>,
    reservation: Option<RasterTextureReservation>,
    reservation_retirement: Option<RasterTextureReservationCloseCursor>,
    witnesses: RasterOperationWitnessLedger,
    residency: RasterResidencyLedger,
    exact_close: Option<RasterTextureKey>,
    next_reservation_nonce: u64,
    layout: gpu_device::BindGroupLayout,
    sampler: gpu_device::Sampler,
    mip_layout:gpu_device::BindGroupLayout,
    mip_pipelines:[gpu_device::RenderPipeline;2],
    closing: bool,
}

impl RasterTextureTable {
    pub fn new(device: &gpu_device::Device, layout: &gpu_device::BindGroupLayout) -> Self {
        let sampler = device.create_sampler(&gpu_device::SamplerDescriptor { label: Some("raster_sampler"), mag_filter: gpu_device::FilterMode::Linear, min_filter: gpu_device::FilterMode::Linear, ..Default::default() });
        let shader=device.create_shader_module(gpu_device::ShaderModuleDescriptor{label:Some("raster_mip_shader"),source:gpu_device::ShaderSource::Wgsl(crate::shaders::RASTER_MIP_SHADER.into())});
        let mip_layout=device.create_bind_group_layout(&gpu_device::BindGroupLayoutDescriptor{label:Some("raster_mip_layout"),entries:&[gpu_device::BindGroupLayoutEntry{binding:0,visibility:gpu_device::ShaderStages::FRAGMENT,ty:gpu_device::BindingType::Texture{sample_type:gpu_device::TextureSampleType::Float{filterable:true},view_dimension:gpu_device::TextureViewDimension::D2,multisampled:false},count:None},gpu_device::BindGroupLayoutEntry{binding:1,visibility:gpu_device::ShaderStages::FRAGMENT,ty:gpu_device::BindingType::Sampler(gpu_device::SamplerBindingType::Filtering),count:None}]});
        let mip_pipeline_layout=device.create_pipeline_layout(&gpu_device::PipelineLayoutDescriptor{label:Some("raster_mip_pipeline_layout"),bind_group_layouts:&[Some(&mip_layout)],immediate_size:0});
        let mip_pipelines=[gpu_device::TextureFormat::Rgba8Unorm,gpu_device::TextureFormat::Rgba8UnormSrgb].map(|format|device.create_render_pipeline(&gpu_device::RenderPipelineDescriptor{label:Some("raster_mip_pipeline"),layout:Some(&mip_pipeline_layout),vertex:gpu_device::VertexState{module:&shader,entry_point:Some("vs_main"),buffers:&[],compilation_options:Default::default()},fragment:Some(gpu_device::FragmentState{module:&shader,entry_point:Some("fs_main"),targets:&[Some(gpu_device::ColorTargetState{format,blend:None,write_mask:gpu_device::ColorWrites::ALL})],compilation_options:Default::default()}),primitive:Default::default(),depth_stencil:None,multisample:Default::default(),multiview_mask:None,cache:None}));
        Self {
            live: FixedRasterTextureRegistry::default(),
            staged: FixedRasterTextureRegistry::default(),
            upload: None,
            upload_close: None,
            retirement: None,
            reservation: None,
            reservation_retirement: None,
            witnesses: RasterOperationWitnessLedger::default(),
            residency: RasterResidencyLedger::default(),
            exact_close: None,
            next_reservation_nonce: 1,
            layout: layout.clone(),
            sampler,mip_layout,mip_pipelines,
            closing: false,
        }
    }

    fn retained_bytes(&self) -> usize {
        self.live.bytes
            + self.staged.bytes
            + self.reservation.map_or(0, |reservation| reservation.bytes)
            + self.reservation_retirement.as_ref().map_or(0, RasterTextureReservationCloseCursor::retained_bytes)
            + self.retirement.as_ref().and_then(|cursor| cursor.owner.as_ref()).map_or(0, RasterTextureRetirementOwner::retained_bytes)
    }

    fn retained_items(&self) -> usize {
        self.live.len
            + self.staged.len
            + usize::from(self.reservation.is_some())
            + self.reservation_retirement.as_ref().map_or(0, RasterTextureReservationCloseCursor::retained_items)
            + usize::from(self.retirement.as_ref().is_some_and(|cursor| cursor.owner.is_some()))
    }

    fn validate_freshness(&self, key: RasterTextureKey, witness: RasterTextureWitness) -> Result<(), &'static str> {
        if let Ok(Ok(index)) = self.live.locate(key) {
            let current = self.live.slots[index].as_ref().expect("located live raster");
            if raster_witness_is_stale(current.witness, witness) {
                return Err("raster operation witness was stale or duplicated");
            }
        }
        Ok(())
    }

    pub fn begin_candidate_ownership(&mut self, witness: RasterTextureWitness) -> Result<(), &'static str> {
        if self.closing || self.retirement.is_some() {
            return Err("raster texture table is closing or retiring");
        }
        self.residency.begin_candidate(witness)
    }

    pub fn publish_candidate_ownership(&mut self, witness: RasterTextureWitness, key: &str) -> Result<(), &'static str> {
        self.residency.publish_candidate_key(witness, key)
    }

    pub fn seal_candidate_ownership(&mut self, witness: RasterTextureWitness) -> Result<(), &'static str> {
        self.residency.seal_candidate(witness)
    }

    pub fn residency_counts(&self) -> (usize, usize, usize) {
        self.residency.counts()
    }

    pub fn prepare_admission_step(&mut self, key: &str, width: u32, height: u32,mip_levels:u32, content_identity: RasterContentIdentity, witness: RasterTextureWitness) -> Result<bool, &'static str> {
        let base_bytes = raster_texture_bytes(width,height).ok_or("raster texture byte credits overflowed")?;
        let bytes=raster_mip_bytes(width,height,mip_levels).ok_or("raster mip byte credits overflowed")?;
        if !content_identity.dimensions_match(width, height, base_bytes) {
            return Err("raster content identity dimensions were stale");
        }
        if !self.residency.candidate_is_sealed(witness) {
            return Err("raster keep-set candidate was not sealed before admission");
        }
        let key = RasterTextureKey::new(key)?;
        self.validate_freshness(key, witness)?;
        if raster_content_is_reusable(&self.live, &self.staged, key, content_identity, witness)? {
            return Ok(true);
        }
        if self.upload.is_some() || self.reservation.is_some() {
            return Ok(true);
        }
        if self.retirement.is_some() {
            if !self.retire_unowned_step()? {
                return Ok(false);
            }
            let admitted = raster_admission_fits(self.retained_items(), self.retained_bytes(), bytes);
            return admitted.then_some(true).ok_or("raster texture credits are owned by retained frames");
        }
        let admitted = raster_admission_fits(self.retained_items(), self.retained_bytes(), bytes);
        if admitted {
            return Ok(true);
        }
        let retained_items = self.retained_items();
        let retained_bytes = self.retained_bytes();
        if !self.retire_unowned_step()? {
            return Ok(false);
        }
        if self.retained_items() < retained_items || self.retained_bytes() < retained_bytes {
            return Ok(false);
        }
        Err("raster texture credits are owned by retained frames")
    }

    pub fn reserve_engine_texture(&mut self, key: &str, width: u32, height: u32,mip_levels:u32, content_identity: RasterContentIdentity, candidate: RasterTextureWitness, expected: RasterTextureWitness) -> Result<RasterTextureAdmission, &'static str> {
        if let Some(retirement) = self.reservation_retirement.as_mut() {
            if matches!(retirement.step(), RasterTextureCleanupStep::Complete) {
                assert!(retirement.terminal_is_empty(), "completed raster reservation must be terminal-empty");
                self.reservation_retirement = None;
            }
            return Err("raster cancelled reservation retirement is pending");
        }
        if candidate != expected {
            return Err("raster operation authority was stale before admission");
        }
        if self.closing || self.upload.is_some() || self.upload_close.is_some() || self.retirement.is_some() || self.reservation.is_some() {
            return Err("raster texture table is occupied");
        }
        if self.witnesses.candidate_occupied_by(candidate) {
            return Err("raster candidate generation was occupied");
        }
        let key = RasterTextureKey::new(key)?;
        let base_bytes = raster_texture_bytes(width,height).ok_or("raster texture byte credits overflowed")?;
        let bytes=raster_mip_bytes(width,height,mip_levels).ok_or("raster mip byte credits overflowed")?;
        if !content_identity.dimensions_match(width, height, base_bytes) {
            return Err("raster content identity dimensions were stale");
        }
        if width == 0 || height == 0 || bytes > RASTER_TEXTURE_ITEM_BYTE_CAPACITY {
            return Err("raster texture exceeded fixed item byte credits");
        }
        if self.retained_items() >= RASTER_TEXTURE_TABLE_CAPACITY {
            return Err("raster texture table item credits exhausted");
        }
        if self.retained_bytes().checked_add(bytes).is_none_or(|total| total > RASTER_TEXTURE_TABLE_BYTE_CAPACITY) {
            return Err("raster texture table byte credits exhausted");
        }
        self.validate_freshness(key, candidate)?;
        if self.staged.get(key.as_str()).is_some() {
            return Err("raster staged generation was duplicated");
        }
        let _ = self.live.locate(key)?;
        let staged_index = match self.staged.locate(key)? {
            Ok(index) | Err(index) => index,
        };
        let nonce = self.next_reservation_nonce;
        self.next_reservation_nonce = self.next_reservation_nonce.checked_add(1).ok_or("raster reservation generation exhausted")?;
        let reservation = RasterTextureReservation { key, witness: candidate, content_identity, width, height, bytes, staged_index, nonce };
        self.reservation = Some(reservation);
        self.witnesses.admit_candidate(candidate);
        Ok(RasterTextureAdmission { key, witness: candidate, content_identity, width, height, bytes, staged_index, nonce })
    }

    pub fn content_is_reusable(&self, key: &str, identity: RasterContentIdentity, candidate: RasterTextureWitness, expected: RasterTextureWitness) -> Result<bool, &'static str> {
        if candidate != expected {
            return Err("raster operation authority was stale before content reuse");
        }
        let key = RasterTextureKey::new(key)?;
        self.validate_freshness(key, candidate)?;
        raster_content_is_reusable(&self.live, &self.staged, key, identity, candidate)
    }

    #[expect(clippy::result_large_err, reason = "Cancellation must return its exact original admission on a full close slot.")]
    pub fn cancel_engine_texture_admission(&mut self, admission: RasterTextureAdmission) -> Result<(), RasterTextureCancelFault> {
        if self.reservation.as_ref().is_some_and(|reservation| reservation.matches(&admission)) {
            if self.reservation_retirement.is_some() {
                return Err(RasterTextureCancelFault::Returned { fault: "raster reservation retirement capacity exhausted", admission });
            }
            let reservation = self.reservation.take().expect("matching raster reservation");
            self.reservation_retirement = Some(RasterTextureReservationCloseCursor::cancelled(reservation, admission));
            return Ok(());
        }
        if self.reservation_retirement.is_some() {
            return Err(RasterTextureCancelFault::Returned { fault: "raster rejected admission retirement capacity exhausted", admission });
        }
        self.reservation_retirement = Some(RasterTextureReservationCloseCursor::rejected(admission));
        Err(RasterTextureCancelFault::Retained("raster admission was stale and retained for bounded retirement"))
    }

    fn claim_stage_before_gpu_allocation(&self, admission: &RasterTextureAdmission, expected: RasterTextureWitness) -> Result<RasterTextureStageClaim, &'static str> {
        let staged_occupied = self.staged.slots.get(admission.staged_index).is_none_or(Option::is_some);
        claim_raster_stage_tuple(self.reservation, self.witnesses.candidate(), staged_occupied, admission, expected)
    }

    fn claim_texture_allocation(&self, admission: &RasterTextureAdmission, expected: RasterTextureWitness) -> Result<RasterTextureStageClaim, &'static str> {
        self.claim_stage_before_gpu_allocation(admission, expected)
    }

    fn claim_view_allocation(&self, admission: &RasterTextureAdmission, expected: RasterTextureWitness) -> Result<RasterTextureStageClaim, &'static str> {
        self.claim_stage_before_gpu_allocation(admission, expected)
    }

    fn claim_bind_group_allocation(&self, admission: &RasterTextureAdmission, expected: RasterTextureWitness) -> Result<RasterTextureStageClaim, &'static str> {
        self.claim_stage_before_gpu_allocation(admission, expected)
    }

    pub(super) fn validate_engine_renderer_allocation(&self, admission: &RasterTextureAdmission, expected: RasterTextureWitness) -> Result<(), &'static str> {
        self.claim_stage_before_gpu_allocation(admission, expected).map(|_| ())
    }

    pub(super) fn validate_engine_target_texture_allocation(&self, admission: &RasterTextureAdmission, expected: RasterTextureWitness) -> Result<(), &'static str> {
        self.claim_stage_before_gpu_allocation(admission, expected).map(|_| ())
    }

    pub(super) fn validate_engine_target_view_allocation(&self, admission: &RasterTextureAdmission, expected: RasterTextureWitness) -> Result<(), &'static str> {
        self.claim_stage_before_gpu_allocation(admission, expected).map(|_| ())
    }

    pub(super) fn retain_engine_allocation_fault(&mut self, admission: RasterTextureAdmission, texture: Option<gpu_device::Texture>, view: Option<gpu_device::TextureView>) {
        assert!(self.upload.is_none() && self.upload_close.is_none(), "matching raster allocation fault must own its reserved close slot");
        self.upload_close = Some(RasterTextureUploadCloseCursor::new(RasterTextureUploadCursor {mip_levels:1,mip:1,mip_row:0, admission: Some(admission), row: 0, texture, view, bind_group: None, allocation_claim: None, cpu_release: None }));
    }

    #[expect(clippy::result_large_err, reason = "GPU admission returns the exact resource and reservation owner without allocating during refusal.")]
    fn stage_claimed_texture(
        &mut self,
        admission: RasterTextureAdmission,
        value: RasterTexture,
        claim: RasterTextureStageClaim,
        cpu_release: Option<crate::raster_ownership::SceneRasterReleaseWitness>,
    ) -> Result<(), (&'static str, RasterTextureAdmission, RasterTexture, Option<crate::raster_ownership::SceneRasterReleaseWitness>)> {
        if self.reservation != Some(claim.reservation) || !claim.reservation.matches(&admission) || claim.candidate != admission.witness || claim.staged_index != admission.staged_index || claim.staged_nonce != admission.nonce {
            return Err(("raster allocation claim changed before publication", admission, value, cpu_release));
        }
        if self.witnesses.candidate() != Some(claim.candidate) {
            return Err(("raster candidate witness changed after GPU allocation", admission, value, cpu_release));
        }
        if self.staged.slots[claim.staged_index].is_some() {
            return Err(("raster staged slot changed after GPU allocation", admission, value, cpu_release));
        }
        let entry = RasterTextureEntry { key: admission.key, witness: admission.witness, content_identity: admission.content_identity, bytes: admission.bytes, cpu_release, gpu_resident: None, value };
        if let Err(entry) = self.staged.insert_vacant(claim.staged_index, entry) {
            return Err(("raster staged slot changed after preflight", admission, entry.value, entry.cpu_release));
        }
        let _completed_reservation = self.reservation.take().expect("published raster reservation");
        Ok(())
    }

    #[allow(clippy::too_many_arguments, reason = "one arg per GPU resource/dimension; grouping into a struct is a T2 restructure, out of scope")]
    pub fn ensure_raster_step(
        &mut self,
        device: &gpu_device::Device,
        queue: &gpu_device::Queue,
        globals_buffer: &gpu_device::Buffer,
        glyph_view: &gpu_device::TextureView,
        glyph_sampler: &gpu_device::Sampler,
        _icon_view: &gpu_device::TextureView,
        _icon_sampler: &gpu_device::Sampler,
        key: &str,
        pixels: RasterUploadPixels<'_>,
        width: u32,
        height: u32,
        candidate: RasterTextureWitness,
        expected: RasterTextureWitness,
    ) -> Result<bool, &'static str> {
        if self.closing || self.retirement.is_some() {
            return Err("raster texture table is closing or retiring");
        }
        let row_bytes = usize::try_from(width).ok().and_then(|value| value.checked_mul(4)).ok_or("raster row byte credits overflowed")?;
        let expected_bytes = row_bytes.checked_mul(usize::try_from(height).map_err(|_| "raster height exceeded fixed credits")?).ok_or("raster byte credits overflowed")?;
        let transfer_bytes = pixels.transfer_bytes();
        if width == 0 || height == 0 || row_bytes > transfer_bytes || pixels.len() != expected_bytes || !pixels.dimensions_match(width, height) {
            return Err("raster upload exceeded fixed page or byte credits");
        }
        let content_identity = pixels.content_identity(width, height).ok_or("raster content identity was unavailable")?;
        if self.content_is_reusable(key, content_identity, candidate, expected)? {
            if let Some(live) = self.live.get_mut(key).filter(|entry| entry.content_identity == content_identity && entry.gpu_resident.is_none()) {
                if let Some(release) = live.cpu_release.take().or_else(|| pixels.release_witness()) {
                    match release.commit_gpu(key) {
                        Ok(resident) => live.gpu_resident = Some(resident),
                        Err(_release) => {}
                    }
                }
            }
            pixels.release_reused();
            return Ok(true);
        }
        let mip_levels=pixels.mip_levels(width,height);
        if self.upload.is_none() {
            let admission = self.reserve_engine_texture(key, width, height,mip_levels, content_identity, candidate, expected)?;
            self.upload = Some(RasterTextureUploadCursor {mip_levels,mip:1,mip_row:0, admission: Some(admission), row: 0, texture: None, view: None, bind_group: None, allocation_claim: None, cpu_release: pixels.release_witness() });
            let allocation_claim = {
                let admission = self.upload.as_ref().and_then(|cursor| cursor.admission.as_ref()).expect("retained raster texture admission");
                self.claim_texture_allocation(admission, expected)?
            };
            self.upload.as_mut().expect("retained raster texture claim").allocation_claim = Some(allocation_claim);
            let texture = device.create_texture(&gpu_device::TextureDescriptor {
                label: Some("raster_texture"),
                size: gpu_device::Extent3d { width, height, depth_or_array_layers: 1 },
                mip_level_count: mip_levels,
                sample_count: 1,
                dimension: gpu_device::TextureDimension::D2,
                format: pixels.texture_format(),
                usage: gpu_device::TextureUsages::TEXTURE_BINDING | gpu_device::TextureUsages::COPY_DST | gpu_device::TextureUsages::RENDER_ATTACHMENT,
                view_formats: &[],
            });
            self.upload.as_mut().expect("retained raster texture upload").texture = Some(texture);
            let allocation_claim = {
                let admission = self.upload.as_ref().and_then(|cursor| cursor.admission.as_ref()).expect("retained raster view admission");
                self.claim_view_allocation(admission, expected)?
            };
            self.upload.as_mut().expect("retained raster view claim").allocation_claim = Some(allocation_claim);
            let view = self.upload.as_ref().and_then(|cursor| cursor.texture.as_ref()).expect("retained raster texture owner").create_view(&gpu_device::TextureViewDescriptor::default());
            self.upload.as_mut().expect("retained raster view upload").view = Some(view);
            let allocation_claim = {
                let admission = self.upload.as_ref().and_then(|cursor| cursor.admission.as_ref()).expect("retained raster bind-group admission");
                self.claim_bind_group_allocation(admission, expected)?
            };
            self.upload.as_mut().expect("retained raster bind-group claim").allocation_claim = Some(allocation_claim);
            let view = self.upload.as_ref().and_then(|cursor| cursor.view.as_ref()).expect("retained raster view owner");
            let bind_group = device.create_bind_group(&gpu_device::BindGroupDescriptor {
                label: Some("raster_texture_bind_group"),
                layout: &self.layout,
                entries: &[
                    gpu_device::BindGroupEntry { binding: 0, resource: globals_buffer.as_entire_binding() },
                    gpu_device::BindGroupEntry { binding: 1, resource: gpu_device::BindingResource::TextureView(glyph_view) },
                    gpu_device::BindGroupEntry { binding: 2, resource: gpu_device::BindingResource::Sampler(glyph_sampler) },
                    gpu_device::BindGroupEntry { binding: 3, resource: gpu_device::BindingResource::TextureView(view) },
                    gpu_device::BindGroupEntry { binding: 4, resource: gpu_device::BindingResource::Sampler(&self.sampler) },
                ],
            });
            let upload = self.upload.as_mut().expect("retained raster bind-group upload");
            upload.bind_group = Some(bind_group);
        }
        let cursor = self.upload.as_mut().expect("raster upload cursor initialized above");
        let admission = cursor.admission.as_ref().ok_or("raster upload admission was retired")?;
        if admission.key.as_str() != key || admission.witness != candidate || admission.content_identity != content_identity || admission.width != width || admission.height != height || candidate != expected {
            return Err("raster upload authority was occupied by another generation");
        }
        if cursor.row < height {
            let rows = (transfer_bytes / row_bytes).max(1).min(usize::try_from(height - cursor.row).unwrap_or(usize::MAX));
            let start = usize::try_from(cursor.row).unwrap_or(usize::MAX).saturating_mul(row_bytes);
            let end = start.saturating_add(rows.saturating_mul(row_bytes));
            let texture = cursor.texture.as_ref().ok_or("raster upload texture was retired")?;
            let page_rows = pixels
                .with_rows(cursor.row, start, end, rows as u32, |page, page_rows| {
                    queue.write_texture(
                        gpu_device::TexelCopyTextureInfo { texture, mip_level: 0, origin: gpu_device::Origin3d { x: 0, y: cursor.row, z: 0 }, aspect: gpu_device::TextureAspect::All },
                        page,
                        gpu_device::TexelCopyBufferLayout { offset: 0, bytes_per_row: Some(width * 4), rows_per_image: Some(page_rows) },
                        gpu_device::Extent3d { width, height: page_rows, depth_or_array_layers: 1 },
                    );
                    page_rows
                })
                .ok_or("raster prepared page ownership was incomplete")?;
            cursor.row += page_rows;
            return Ok(false);
        }
        if cursor.mip<cursor.mip_levels {
            let texture=cursor.texture.as_ref().ok_or("raster mip texture was retired")?;let width=(width>>cursor.mip).max(1);let height=(height>>cursor.mip).max(1);
            let view=|level|texture.create_view(&gpu_device::TextureViewDescriptor{base_mip_level:level,mip_level_count:Some(1),..Default::default()});let source=view(cursor.mip-1);let target=view(cursor.mip);
            let group=device.create_bind_group(&gpu_device::BindGroupDescriptor{label:Some("raster_mip_bind_group"),layout:&self.mip_layout,entries:&[gpu_device::BindGroupEntry{binding:0,resource:gpu_device::BindingResource::TextureView(&source)},gpu_device::BindGroupEntry{binding:1,resource:gpu_device::BindingResource::Sampler(&self.sampler)}]});
            let mut encoder=device.create_command_encoder(&gpu_device::CommandEncoderDescriptor{label:Some("raster_mip_encoder")});let rows=(height-cursor.mip_row).min(8);
            {let mut pass=encoder.begin_render_pass(&gpu_device::RenderPassDescriptor{label:Some("raster_mip_pass"),color_attachments:&[Some(gpu_device::RenderPassColorAttachment{view:&target,resolve_target:None,ops:gpu_device::Operations{load:gpu_device::LoadOp::Load,store:gpu_device::StoreOp::Store},depth_slice:None})],depth_stencil_attachment:None,timestamp_writes:None,occlusion_query_set:None,multiview_mask:None});pass.set_pipeline(&self.mip_pipelines[usize::from(pixels.texture_format()==gpu_device::TextureFormat::Rgba8UnormSrgb)]);pass.set_bind_group(0,&group,&[]);pass.set_scissor_rect(0,cursor.mip_row,width,rows);pass.draw(0..3,0..1);}
            queue.submit(encoder.finish()).expect("same-session recorded command");cursor.mip_row+=rows;if cursor.mip_row==height{cursor.mip_row=0;cursor.mip+=1;}return Ok(false);
        }
        let mut cursor = self.upload.take().expect("completed raster upload cursor");
        let admission = cursor.admission.take().expect("completed raster upload admission");
        let allocation_claim = cursor.allocation_claim.take().expect("completed raster allocation claim");
        let value = RasterTexture {
            texture: cursor.texture.take().expect("completed raster upload texture"),
            view: cursor.view.take().expect("completed raster upload view"),
            bind_group: cursor.bind_group.take().expect("completed raster upload bind group"),
            width,
            height,
        };
        let cpu_release = cursor.cpu_release.take();
        if let Err((fault, admission, value, cpu_release)) = self.stage_claimed_texture(admission, value, allocation_claim, cpu_release) {
            self.upload_close = Some(RasterTextureUploadCloseCursor::new(RasterTextureUploadCursor {mip_levels:1,mip:1,mip_row:0,
                admission: Some(admission),
                row: height,
                texture: Some(value.texture),
                view: Some(value.view),
                bind_group: Some(value.bind_group),
                allocation_claim: Some(allocation_claim),
                cpu_release,
            }));
            return Err(fault);
        }
        Ok(true)
    }

    pub fn get(&self, key: &str) -> Option<&RasterTexture> {
        if let Some(witness) = self.witnesses.presenting() {
            if let Some(entry) = self.staged.get(key).filter(|entry| entry.witness == witness) {
                return Some(&entry.value);
            }
        }
        self.live.get(key).map(|entry| &entry.value)
    }

    #[allow(clippy::too_many_arguments, reason = "one arg per GPU resource/dimension; grouping into a struct is a T2 restructure, out of scope")]
    #[expect(clippy::result_large_err, reason = "GPU admission returns the exact resource and reservation owner without allocating during refusal.")]
    pub fn stage_gpu_bind_group(
        &mut self,
        device: &gpu_device::Device,
        globals_buffer: &gpu_device::Buffer,
        glyph_view: &gpu_device::TextureView,
        glyph_sampler: &gpu_device::Sampler,
        admission: RasterTextureAdmission,
        raster_view: gpu_device::TextureView,
        texture: gpu_device::Texture,
        expected: RasterTextureWitness,
    ) -> Result<(), RasterTextureStageFault> {
        let allocation_claim = match self.claim_bind_group_allocation(&admission, expected) {
            Ok(claim) => claim,
            Err(fault) => return Err(RasterTextureStageFault::Returned { fault, admission, texture, view: raster_view }),
        };
        let bind_group = device.create_bind_group(&gpu_device::BindGroupDescriptor {
            label: Some("raster_bind_group"),
            layout: &self.layout,
            entries: &[
                gpu_device::BindGroupEntry { binding: 0, resource: globals_buffer.as_entire_binding() },
                gpu_device::BindGroupEntry { binding: 1, resource: gpu_device::BindingResource::TextureView(glyph_view) },
                gpu_device::BindGroupEntry { binding: 2, resource: gpu_device::BindingResource::Sampler(glyph_sampler) },
                gpu_device::BindGroupEntry { binding: 3, resource: gpu_device::BindingResource::TextureView(&raster_view) },
                gpu_device::BindGroupEntry { binding: 4, resource: gpu_device::BindingResource::Sampler(&self.sampler) },
            ],
        });
        let width = admission.width;
        let height = admission.height;
        let value = RasterTexture { texture, view: raster_view, bind_group, width, height };
        if let Err((fault, admission, value, cpu_release)) = self.stage_claimed_texture(admission, value, allocation_claim, None) {
            assert!(self.upload.is_none() && self.upload_close.is_none(), "matching raster publication fault must own its reserved close slot");
            self.upload_close = Some(RasterTextureUploadCloseCursor::new(RasterTextureUploadCursor {mip_levels:1,mip:1,mip_row:0,
                admission: Some(admission),
                row: height,
                texture: Some(value.texture),
                view: Some(value.view),
                bind_group: Some(value.bind_group),
                allocation_claim: Some(allocation_claim),
                cpu_release,
            }));
            return Err(RasterTextureStageFault::Retained(fault));
        }
        Ok(())
    }

    pub fn begin_presenting(&mut self, witness: RasterTextureWitness) -> Result<bool, &'static str> {
        if self.closing || self.retirement.is_some() || self.upload.is_some() {
            return Err("raster candidate was not terminal before presentation");
        }
        if !self.residency.candidate_is_sealed(witness) {
            return Err("raster keep-set candidate was not sealed before presentation");
        }
        if self.staged.is_empty() {
            return Ok(false);
        }
        self.witnesses.arm_presentation(witness)?;
        Ok(true)
    }

    /// 🧾️ Admits a retirement ONCE, then hands the witness slots to its cursor.
    ///
    /// ⚖️ The candidate/presentation witnesses are an ADMISSION fact, not a per-step invariant: the
    /// cursor retires those very slots one field at a time, so re-reading them on every later step
    /// compares a half-consumed slot against the operation that owns it and refuses its own progress
    /// (ticket 26/09/09/PROCEDURAL-3D-END-TO-END — `raster commit candidate witness was stale`).
    /// An EMPTY presentation is admitted: an operation that reserved a raster the surface never
    /// presented owes a candidate retirement and nothing else — "not yet", never "stale".
    fn begin_retirement(&mut self, mode: RasterTextureRetirementMode) -> Result<(), &'static str> {
        if let Some(cursor) = self.retirement.as_ref() {
            return (cursor.mode == mode).then_some(()).ok_or("raster retirement authority was occupied");
        }
        self.retirement = Some(RasterTextureRetirementCursor { mode, scan: 0, owner: None });
        Ok(())
    }

    fn retirement_step(&mut self) -> Result<bool, &'static str> {
        let Some(cursor) = self.retirement.as_mut() else { return Ok(true) };
        if let Some(owner) = cursor.owner.as_mut() {
            return match owner.step() {
                RasterTextureCleanupStep::Pending { .. } => Ok(false),
                RasterTextureCleanupStep::Blocked(fault) => Err(fault),
                RasterTextureCleanupStep::Complete => {
                    assert!(owner.terminal_is_empty(), "completed raster retirement owner must be terminal-empty");
                    cursor.owner = None;
                    Ok(false)
                }
            };
        }
        if cursor.scan >= RASTER_TEXTURE_TABLE_CAPACITY {
            if !matches!(cursor.mode, RasterTextureRetirementMode::Unowned | RasterTextureRetirementMode::Exact(_)) && !self.witnesses.retire_step() {
                return Ok(false);
            }
            self.retirement = None;
            return Ok(true);
        }
        let index = cursor.scan;
        cursor.scan += 1;
        match cursor.mode {
            RasterTextureRetirementMode::Abort(witness) => {
                let selected = self.staged.slots[index].as_ref().is_some_and(|entry| entry.witness == witness);
                if selected {
                    let entry = self.staged.take(index).expect("selected staged raster");
                    cursor.owner = Some(RasterTextureRetirementOwner::new(entry));
                }
            }
            RasterTextureRetirementMode::Commit(witness) => {
                let selected = self.staged.slots[index].as_ref().is_some_and(|entry| entry.witness == witness);
                if selected {
                    let entry = self.staged.take(index).expect("selected staged raster");
                    let key = entry.key;
                    match self.live.insert(entry) {
                        Ok(previous) => {
                            let live = self.live.get_mut(key.as_str()).expect("committed raster remains live under its exact key");
                            if let Some(release) = live.cpu_release.take() {
                                match release.commit_gpu(key.as_str()) {
                                    Ok(resident) => live.gpu_resident = Some(resident),
                                    Err(_release) => {}
                                }
                            }
                            if let Some(previous) = previous {
                                cursor.owner = Some(RasterTextureRetirementOwner::new(previous));
                            }
                        }
                        Err(entry) => {
                            if let Err(entry) = self.staged.insert_vacant(index, entry) {
                                cursor.owner = Some(RasterTextureRetirementOwner::new(entry));
                            }
                            return Err("raster live table capacity exhausted");
                        }
                    }
                }
            }
            RasterTextureRetirementMode::Unowned => {
                if let Some(entry) = take_unowned_live_entry(&mut self.live, &self.staged, &self.residency, index) {
                    cursor.owner = Some(RasterTextureRetirementOwner::new(entry));
                }
            }
            RasterTextureRetirementMode::Exact(key) => {
                let staged = self.staged.slots[index].as_ref().is_some_and(|entry| entry.key == key).then(|| self.staged.take(index)).flatten();
                if staged.is_some() && self.live.slots[index].as_ref().is_some_and(|entry| entry.key == key) {
                    cursor.scan = index;
                }
                let live = if staged.is_none() && self.live.slots[index].as_ref().is_some_and(|entry| entry.key == key) { self.live.take(index) } else { None };
                if let Some(entry) = staged.or(live) {
                    cursor.owner = Some(RasterTextureRetirementOwner::new(entry));
                }
            }
            RasterTextureRetirementMode::Close => {
                let staged = self.staged.take(index);
                if staged.is_some() && self.live.slots[index].is_some() {
                    cursor.scan = index;
                }
                if let Some(entry) = staged.or_else(|| self.live.take(index)) {
                    cursor.owner = Some(RasterTextureRetirementOwner::new(entry));
                }
            }
        }
        Ok(false)
    }

    /// 🩺️ `(candidate, presenting, staged items)` — the three facts a stale-witness refusal is about,
    /// so a fault banner can name what actually moved instead of only that something did.
    pub fn presentation_witnesses(&self) -> (Option<RasterTextureWitness>, Option<RasterTextureWitness>, usize) {
        (self.witnesses.candidate(), self.witnesses.presenting(), self.staged.len)
    }

    /// 🪶 Commits the rasters this operation staged, one bounded step at a time.
    ///
    /// ⚖️ An operation that admitted NO raster owes nothing: an engine surface whose first raster has
    /// not been reserved yet is "not yet", never "stale". Once the retirement is admitted the cursor
    /// owns the witness slots — see [`RasterTextureTable::begin_retirement`].
    pub fn commit_presented_step(&mut self, witness: RasterTextureWitness) -> Result<bool, &'static str> {
        if self.witnesses.begin_retirement(witness)? == RasterWitnessAdmission::Nothing {
            self.residency.commit(witness)?;
            return Ok(true);
        }
        self.begin_retirement(RasterTextureRetirementMode::Commit(witness))?;
        if !self.retirement_step()? {
            return Ok(false);
        }
        self.residency.commit(witness)?;
        Ok(true)
    }

    pub fn abort_presented_step(&mut self, witness: RasterTextureWitness) -> Result<bool, &'static str> {
        if self.witnesses.begin_retirement(witness)? == RasterWitnessAdmission::Nothing && self.staged.is_empty() && self.upload.is_none() {
            self.residency.abort(witness)?;
            return Ok(true);
        }
        self.begin_retirement(RasterTextureRetirementMode::Abort(witness))?;
        if !self.retirement_step()? {
            return Ok(false);
        }
        self.residency.abort(witness)?;
        Ok(true)
    }

    pub fn release_previous_ownership(&mut self) {
        self.residency.release_previous();
    }

    pub fn retire_unowned_step(&mut self) -> Result<bool, &'static str> {
        self.begin_retirement(RasterTextureRetirementMode::Unowned)?;
        self.retirement_step()
    }

    pub fn begin_exact_close(&mut self, key: &str) -> Result<bool, &'static str> {
        let key = RasterTextureKey::new(key)?;
        if self.exact_close == Some(key) {
            return Ok(true);
        }
        if self.exact_close.is_some() {
            return Err("raster exact-close authority was occupied");
        }
        if self.closing || self.retirement.is_some() || self.upload.is_some() || self.upload_close.is_some() || self.reservation.is_some() || self.reservation_retirement.is_some() || !self.witnesses.is_empty() {
            return Ok(false);
        }
        self.residency.remove_key(key);
        self.exact_close = Some(key);
        Ok(true)
    }

    pub fn close_exact_step(&mut self, key: &str) -> Result<bool, &'static str> {
        let key = RasterTextureKey::new(key)?;
        if self.exact_close != Some(key) {
            return Err("raster exact-close token was stale");
        }
        if self.retirement.is_none() {
            let retained = self.live.get(key.as_str()).is_some() || self.staged.get(key.as_str()).is_some();
            if !retained {
                self.exact_close = None;
                return Ok(true);
            }
            self.begin_retirement(RasterTextureRetirementMode::Exact(key))?;
        }
        Ok(self.retirement_step()? && self.live.get(key.as_str()).is_none() && self.staged.get(key.as_str()).is_none())
    }

    pub fn exact_close_terminal_is_empty(&self, key: &str) -> bool {
        let Ok(key) = RasterTextureKey::new(key) else { return false };
        self.exact_close.is_none() && self.live.get(key.as_str()).is_none() && self.staged.get(key.as_str()).is_none() && !self.residency.protects(key)
    }

    pub fn close_upload_step(&mut self) -> RasterTextureCleanupStep {
        if let Some(retirement) = self.reservation_retirement.as_mut() {
            let step = retirement.step();
            if matches!(step, RasterTextureCleanupStep::Complete) {
                assert!(retirement.terminal_is_empty(), "completed raster reservation must be terminal-empty");
                self.reservation_retirement = None;
                return RasterTextureCleanupStep::retained();
            }
            return step;
        }
        if let Some(reservation) = self.reservation.take() {
            self.reservation_retirement = Some(RasterTextureReservationCloseCursor::reservation(reservation));
            return RasterTextureCleanupStep::retained();
        }
        if self.upload_close.is_none() {
            if let Some(upload) = self.upload.take() {
                self.upload_close = Some(RasterTextureUploadCloseCursor::new(upload));
                return RasterTextureCleanupStep::retained();
            }
            return RasterTextureCleanupStep::Complete;
        }
        let cursor = self.upload_close.as_mut().expect("retained raster upload close cursor");
        let step = cursor.step();
        if matches!(step, RasterTextureCleanupStep::Complete) {
            assert!(cursor.terminal_is_empty(), "completed raster upload close must be terminal-empty");
            self.upload_close = None;
            return RasterTextureCleanupStep::retained();
        }
        step
    }

    pub fn close_step(&mut self) -> Result<bool, &'static str> {
        self.closing = true;
        match self.close_upload_step() {
            RasterTextureCleanupStep::Pending { .. } => return Ok(false),
            RasterTextureCleanupStep::Blocked(fault) => return Err(fault),
            RasterTextureCleanupStep::Complete => {}
        }
        if self.retirement.is_some() && !self.retirement_step()? {
            return Ok(false);
        }
        if self.retirement.is_none() && (!self.live.is_empty() || !self.staged.is_empty() || !self.witnesses.is_empty()) {
            self.begin_retirement(RasterTextureRetirementMode::Close)?;
            return Ok(false);
        }
        self.residency.clear();
        Ok(self.terminal_is_empty())
    }

    pub fn terminal_is_empty(&self) -> bool {
        self.closing
            && self.live.is_empty()
            && self.staged.is_empty()
            && self.upload.is_none()
            && self.upload_close.is_none()
            && self.retirement.is_none()
            && self.reservation.is_none()
            && self.reservation_retirement.is_none()
            && self.witnesses.is_empty()
            && self.residency.is_empty()
            && self.exact_close.is_none()
    }
}


#[cfg(test)]
#[path="🧪️tests/🔬️unit/🦀️.rs"]
mod tests;

#[cfg(test)]
#[path="🧪️tests/🖼️raster-residency/🦀️.rs"]
mod raster_residency_tests;

#[cfg(test)]
#[path="🧪️tests/🖼️raster-witness-lifecycle/🦀️.rs"]
mod raster_witness_lifecycle_tests;
