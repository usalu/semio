//! 🔑️ Texture keys, ledger and registry: the raster texture, its witness and cleanup step, the keep set, the residency ledger, the texture key and the fixed texture registry. See [`super`].

use super::*;

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

pub(super) const RASTER_TEXTURE_PROBE_CAPACITY: usize = 8;

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
    pub(super) fn root() -> Self {
        Self::Pending { released_roots: 1, released_scalars: 0 }
    }

    pub(super) fn scalar() -> Self {
        Self::Pending { released_roots: 0, released_scalars: 1 }
    }

    pub(super) fn retained() -> Self {
        Self::Pending { released_roots: 0, released_scalars: 0 }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) struct RasterTextureKey {
    pub(super) bytes: [u8; RASTER_TEXTURE_KEY_BYTES],
    pub(super) len: u16,
    hash: u64,
}

pub struct RasterKeepSetV1 {
    pub(super) slots: Box<[Option<RasterTextureKey>; RASTER_TEXTURE_TABLE_CAPACITY]>,
    pub(super) len: usize,
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
    pub(super) committed: RasterKeepSetV1,
    pub(super) candidate: RasterKeepSetV1,
    pub(super) previous: RasterKeepSetV1,
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

    pub(super) fn candidate_is_sealed(&self, witness: RasterTextureWitness) -> bool {
        self.candidate_witness == Some(witness) && self.candidate_sealed
    }

    pub(super) fn protects(&self, key: RasterTextureKey) -> bool {
        self.committed.contains(key) || self.candidate.contains(key) || self.previous.contains(key)
    }

    pub(super) fn commit(&mut self, witness: RasterTextureWitness) -> Result<(), &'static str> {
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

    pub(super) fn abort(&mut self, witness: RasterTextureWitness) -> Result<(), &'static str> {
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

    pub(super) fn remove_key(&mut self, key: RasterTextureKey) {
        self.committed.remove(key);
        self.candidate.remove(key);
        self.previous.remove(key);
    }

    pub fn counts(&self) -> (usize, usize, usize) {
        (self.committed.len(), self.candidate.len(), self.previous.len())
    }

    pub(super) fn clear(&mut self) {
        *self = Self::default();
    }

    pub(super) fn is_empty(&self) -> bool {
        self.committed.is_empty() && self.candidate.is_empty() && self.previous.is_empty() && self.candidate_witness.is_none() && !self.candidate_sealed
    }
}

impl RasterTextureKey {
    pub(super) fn new(key: &str) -> Result<Self, &'static str> {
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

    pub(super) fn as_str(&self) -> &str {
        std::str::from_utf8(&self.bytes[..usize::from(self.len)]).expect("raster key was admitted from UTF-8")
    }

    pub(super) fn start(self) -> usize {
        self.hash as usize % RASTER_TEXTURE_TABLE_CAPACITY
    }
}

pub(super) fn raster_texture_bytes(width: u32, height: u32) -> Option<usize> {
    usize::try_from(width).ok()?.checked_mul(usize::try_from(height).ok()?)?.checked_mul(4)
}

pub(super) fn raster_mip_bytes(width:u32,height:u32,levels:u32)->Option<usize>{if levels==0 || width==0 || height==0 || levels>u32::BITS-width.max(height).leading_zeros(){return None}let mut bytes=0usize;for level in 0..levels {bytes=bytes.checked_add(raster_texture_bytes((width>>level).max(1),(height>>level).max(1))?)?;}Some(bytes)}

pub(super) fn raster_admission_fits(retained_items: usize, retained_bytes: usize, bytes: usize) -> bool {
    retained_items < RASTER_TEXTURE_TABLE_CAPACITY && retained_bytes.checked_add(bytes).is_some_and(|total| total <= RASTER_TEXTURE_TABLE_BYTE_CAPACITY)
}

pub(super) fn raster_witness_is_stale(current: RasterTextureWitness, candidate: RasterTextureWitness) -> bool {
    current.scene_revision > candidate.scene_revision
        || (current.scene_revision == candidate.scene_revision && current.preview_generation > candidate.preview_generation)
        || (current.scene_revision == candidate.scene_revision && current.preview_generation == candidate.preview_generation && current.operation >= candidate.operation)
}

pub(super) struct RasterTextureEntry<T> {
    pub(super) key: RasterTextureKey,
    pub(super) witness: RasterTextureWitness,
    pub(super) content_identity: RasterContentIdentity,
    pub(super) bytes: usize,
    pub(super) cpu_release: Option<crate::raster_ownership::SceneRasterReleaseWitness>,
    pub(super) gpu_resident: Option<crate::raster_ownership::SceneRasterGpuWitness>,
    pub(super) value: T,
}

pub(super) struct FixedRasterTextureRegistry<T> {
    pub(super) slots: Box<[Option<RasterTextureEntry<T>>; RASTER_TEXTURE_TABLE_CAPACITY]>,
    pub(super) len: usize,
    pub(super) bytes: usize,
}

impl<T> Default for FixedRasterTextureRegistry<T> {
    fn default() -> Self {
        Self { slots: Box::new(std::array::from_fn(|_| None)), len: 0, bytes: 0 }
    }
}

impl<T> FixedRasterTextureRegistry<T> {
    pub(super) fn locate(&self, key: RasterTextureKey) -> Result<Result<usize, usize>, &'static str> {
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

    pub(super) fn get(&self, key: &str) -> Option<&RasterTextureEntry<T>> {
        let key = RasterTextureKey::new(key).ok()?;
        let Ok(index) = self.locate(key).ok()? else { return None };
        self.slots[index].as_ref()
    }

    pub(super) fn get_mut(&mut self, key: &str) -> Option<&mut RasterTextureEntry<T>> {
        let key = RasterTextureKey::new(key).ok()?;
        let Ok(index) = self.locate(key).ok()? else { return None };
        self.slots[index].as_mut()
    }

    #[expect(clippy::result_large_err, reason = "GPU admission returns the exact resource and reservation owner without allocating during refusal.")]
    pub(super) fn insert(&mut self, entry: RasterTextureEntry<T>) -> Result<Option<RasterTextureEntry<T>>, RasterTextureEntry<T>> {
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
    pub(super) fn insert_vacant(&mut self, index: usize, entry: RasterTextureEntry<T>) -> Result<(), RasterTextureEntry<T>> {
        let Some(slot) = self.slots.get_mut(index) else { return Err(entry) };
        if slot.is_some() {
            return Err(entry);
        }
        *slot = Some(entry);
        self.len += 1;
        self.bytes = self.bytes.saturating_add(slot.as_ref().expect("inserted raster entry").bytes);
        Ok(())
    }

    pub(super) fn take(&mut self, index: usize) -> Option<RasterTextureEntry<T>> {
        let value = self.slots.get_mut(index)?.take();
        if let Some(value) = value.as_ref() {
            self.len = self.len.saturating_sub(1);
            self.bytes = self.bytes.saturating_sub(value.bytes);
        }
        value
    }

    pub(super) fn is_empty(&self) -> bool {
        self.len == 0
    }
}

pub(super) fn take_unowned_live_entry<T>(live: &mut FixedRasterTextureRegistry<T>, staged: &FixedRasterTextureRegistry<T>, residency: &RasterResidencyLedger, index: usize) -> Option<RasterTextureEntry<T>> {
    let selected = live.slots[index].as_ref().is_some_and(|entry| !residency.protects(entry.key) && staged.get(entry.key.as_str()).is_none());
    selected.then(|| live.take(index)).flatten()
}

pub(super) fn raster_content_is_reusable<T>(live: &FixedRasterTextureRegistry<T>, staged: &FixedRasterTextureRegistry<T>, key: RasterTextureKey, identity: RasterContentIdentity, witness: RasterTextureWitness) -> Result<bool, &'static str> {
    if let Some(entry) = staged.get(key.as_str()) {
        if entry.witness != witness {
            return Err("raster staged key was owned by another generation");
        }
        return (entry.content_identity == identity).then_some(true).ok_or("raster key had conflicting content in one candidate");
    }
    Ok(live.get(key.as_str()).is_some_and(|entry| entry.content_identity == identity))
}
