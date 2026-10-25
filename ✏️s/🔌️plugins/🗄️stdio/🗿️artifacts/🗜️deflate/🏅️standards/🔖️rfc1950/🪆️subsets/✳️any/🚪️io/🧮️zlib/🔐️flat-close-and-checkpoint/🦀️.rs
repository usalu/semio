//! 🔐️ Close protocol and checkpoints of the zlib codec: the flat close admission and receipt, the flat backing, the checkpoint writers and reader and the raw deflate entry. See [`super`].

use super::*;

//#region 🦑️DissolvedEngineCodec
//#region DeflateCodec
//#region 🧵️StreamingEncode
//#region ♻️FlatClose
/// ♻️ Whether one close turn's grant covers its single owner, its exact release and its flat frontier depth.
pub(super) fn deflate_close_admits(grant: RetainedCloneGrant, release_bytes: usize, terminal_is_empty: bool) -> bool {
    grant.maximum_items != 0 && grant.maximum_release_bytes >= release_bytes && grant.maximum_depth >= usize::from(!terminal_is_empty)
}

/// ♻️ Whether one close turn's grant covers every currency of its quoted demand.
pub(super) fn deflate_close_admits_demand(grant: RetainedCloneGrant, demand: semio_framework_value::RetirementDemand) -> bool {
    grant.maximum_items != 0 && grant.maximum_copy_bytes >= demand.copy_bytes && grant.maximum_capacity_bytes >= demand.capacity_bytes && grant.maximum_release_bytes >= demand.release_bytes && grant.maximum_depth >= demand.depth
}

/// ♻️ The yield of a close turn whose grant is below a demand axis: no mutation and an empty receipt.
pub(super) fn deflate_close_idle() -> semio_framework_job::InteractiveJobCloseStep {
    semio_framework_job::InteractiveJobCloseStep::Pending { progress: RetainedCloneProgress::default() }
}

/// ♻️ The exact receipt of one close turn that handed off `copied_items` owners and released `released_bytes`.
pub(super) fn deflate_close_receipt(copied_items: usize, released_bytes: usize) -> semio_framework_job::InteractiveJobCloseStep {
    semio_framework_job::InteractiveJobCloseStep::Pending { progress: RetainedCloneProgress { copied_items, released_bytes, ..RetainedCloneProgress::default() } }
}

/// ♻️ The terminal close turn: nothing is left to hand off or release, which the owner's terminal-empty witness proves.
pub(super) fn deflate_close_complete() -> semio_framework_job::InteractiveJobCloseStep {
    semio_framework_job::InteractiveJobCloseStep::Complete { progress: RetainedCloneProgress::default() }
}

/// ♻️ A flat owner whose whole physical backing one close turn releases.
pub trait FlatBacking {
    fn backing_bytes(&self) -> usize;
    fn release_backing(&mut self);
}

impl<T> FlatBacking for Vec<T> {
    fn backing_bytes(&self) -> usize {
        self.capacity().saturating_mul(size_of::<T>())
    }

    fn release_backing(&mut self) {
        drop(std::mem::take(self));
    }
}

/// ♻️ The physical bytes of the first owner that still holds a backing, `0` once every owner is released.
pub(super) fn next_backing_bytes(backings: &[&dyn FlatBacking]) -> usize {
    backings.iter().map(|backing| backing.backing_bytes()).find(|bytes| *bytes != 0).unwrap_or(0)
}

/// ♻️ Releases the first owner that still holds a backing whole and returns its bytes, `None` once every owner is released.
pub(super) fn release_next_backing(backings: &mut [&mut dyn FlatBacking]) -> Option<usize> {
    backings.iter_mut().find_map(|backing| {
        let bytes = backing.backing_bytes();
        (bytes != 0).then(|| {
            backing.release_backing();
            bytes
        })
    })
}
//#endregion ♻️FlatClose

pub(super) fn write_usize(bytes: &mut Vec<u8>, value: usize) {
    bytes.extend_from_slice(&(value as u64).to_le_bytes());
}

pub(super) fn write_bytes(bytes: &mut Vec<u8>, value: &[u8]) {
    write_usize(bytes, value.len());
    bytes.extend_from_slice(value);
}

pub(super) fn write_i32s(bytes: &mut Vec<u8>, values: &[i32]) {
    write_usize(bytes, values.len());
    for value in values {
        bytes.extend_from_slice(&value.to_le_bytes());
    }
}

pub(super) struct CheckpointReader<'a> {
    pub(super) bytes: &'a [u8],
    pub(super) cursor: usize,
}

impl<'a> CheckpointReader<'a> {
    pub(super) fn new(bytes: &'a [u8]) -> Self {
        Self { bytes, cursor: 0 }
    }

    pub(super) fn take(&mut self, length: usize) -> Result<&'a [u8], String> {
        let end = self.cursor.checked_add(length).ok_or("DEFLATE checkpoint length overflow")?;
        let value = self.bytes.get(self.cursor..end).ok_or("truncated DEFLATE job checkpoint")?;
        self.cursor = end;
        Ok(value)
    }

    pub(super) fn read_u8(&mut self) -> Result<u8, String> {
        Ok(self.take(1)?[0])
    }

    pub(super) fn read_usize(&mut self) -> Result<usize, String> {
        let value = u64::from_le_bytes(self.take(8)?.try_into().expect("fixed checkpoint width"));
        usize::try_from(value).map_err(|_| "DEFLATE checkpoint value exceeds usize".into())
    }

    pub(super) fn read_bytes(&mut self) -> Result<Vec<u8>, String> {
        let length = self.read_usize()?;
        Ok(self.take(length)?.to_vec())
    }

    pub(super) fn read_i32s(&mut self) -> Result<Vec<i32>, String> {
        let length = self.read_usize()?;
        let mut values = Vec::with_capacity(length);
        for _ in 0..length {
            values.push(i32::from_le_bytes(self.take(4)?.try_into().expect("fixed checkpoint width")));
        }
        Ok(values)
    }

    pub(super) fn is_empty(&self) -> bool {
        self.cursor == self.bytes.len()
    }
}

/// 🗜️ Batch adapter over [`DeflateEncodeJob`] for non-interactive artifact codecs.
pub fn deflate_raw(data: &[u8]) -> Vec<u8> {
    DeflateEncodeJob::new(data.to_vec(), usize::MAX).finish()
}
//#endregion 🧵️StreamingEncode
//#endregion DeflateCodec
//#endregion 🦑️DissolvedEngineCodec
