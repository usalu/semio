//! 📖️ Byte-bounded canonical reading over exact frozen typed root ownership.

use super::*;
use std::mem::ManuallyDrop;

//#region 📦️ReaderOwnership
/// 📖️ Retains a frozen typed Arc and borrowed traversal; no Store publication authority is exposed.
pub struct ArtifactCanonicalJsonReader<T> { owned: ManuallyDrop<ReaderState<T>> }

struct ReaderState<T> {
    encoder: ArtifactCanonicalEditEncoder,
    root: Option<Arc<T>>,
    retirement: Option<Arc<dyn SnapshotRetirementFactory<T>>>,
    active: Option<Box<dyn ErasedSnapshotRetirement>>,
    completed_bytes: u64,
    cancelled: bool,
    failed: bool,
    closing: bool,
}

impl<T> ReaderState<T> {
    fn new(root: Arc<T>, retirement: Arc<dyn SnapshotRetirementFactory<T>>) -> Self {
        Self { encoder: ArtifactCanonicalEditEncoder::default(), root: Some(root), retirement: Some(retirement), active: None, completed_bytes: 0, cancelled: false, failed: false, closing: false }
    }

    fn completed_bytes(&self) -> u64 { self.completed_bytes }
    fn is_complete(&self) -> bool { !self.cancelled && !self.failed && !self.closing && self.encoder.is_complete() }
    fn cancel(&mut self) { self.cancelled = true; }
    fn begin_close(&mut self) { self.closing = true; }

    fn take_root(&mut self) -> Option<Arc<T>> {
        let transferable = if self.closing { self.encoder.terminal_is_empty() } else { self.is_complete() };
        if !transferable || self.active.is_some() { return None; }
        self.encoder.reset().ok()?;
        self.closing = true;
        self.root.take()
    }

    fn terminal_is_empty(&self) -> bool {
        self.closing && self.encoder.terminal_is_empty() && self.root.is_none() && self.retirement.is_none() && self.active.is_none()
    }

    fn close_step(&mut self, grant: ArtifactStoreOneItemGrant) -> Result<SnapshotRetirementStep, String> {
        if !self.closing || !grant.permits_one() { return Ok(SnapshotRetirementStep::Blocked); }
        if !self.encoder.terminal_is_empty() { return self.encoder.close_step(); }
        if let Some(active) = self.active.as_mut() {
            return match active.close_step(1, grant.maximum_bytes)? {
                SnapshotRetirementStep::Complete => {
                    if !active.terminal_is_empty() { return Err("canonical-reader.retirement-witness".into()); }
                    self.active = None;
                    Ok(SnapshotRetirementStep::Pending { released_items: 1, released_bytes: 0 })
                }
                SnapshotRetirementStep::Pending { released_items, released_bytes } if released_items <= 1 && released_bytes <= grant.maximum_bytes => Ok(SnapshotRetirementStep::Pending { released_items, released_bytes }),
                SnapshotRetirementStep::Pending { .. } => Err("canonical-reader.retirement-grant".into()),
                SnapshotRetirementStep::Blocked => Ok(SnapshotRetirementStep::Blocked),
            };
        }
        if let Some(root) = self.root.take() {
            self.active = Some(self.retirement.as_ref().expect("reader retains root retirement authority").retire(root));
            return Ok(SnapshotRetirementStep::Pending { released_items: 1, released_bytes: 0 });
        }
        if self.retirement.take().is_some() { return Ok(SnapshotRetirementStep::Pending { released_items: 1, released_bytes: 0 }); }
        Ok(SnapshotRetirementStep::Complete)
    }
}

impl<T: ArtifactCanonicalJson + Send + 'static> ReaderState<T> {
    fn encode_chunk(&mut self, grant: ArtifactStoreOneItemGrant, output: &mut [u8]) -> Result<usize, String> {
        if !grant.permits_one() || self.cancelled || self.failed || self.closing || output.is_empty() { return Ok(0); }
        let maximum = grant.maximum_bytes.min(output.len()).min(ARTIFACT_CANONICAL_JSON_CHUNK_BYTES);
        let root = self.root.as_ref().ok_or_else(|| "canonical-reader.root-missing".to_string())?;
        let count = match self.encoder.encode_chunk(root.as_ref(), &mut output[..maximum]) {
            Ok(count) => count,
            Err(error) => { self.failed = true; return Err(error); }
        };
        self.completed_bytes = self.completed_bytes.checked_add(count as u64).ok_or_else(|| "canonical-reader.work-overflow".to_string())?;
        Ok(count)
    }
}

impl<T> ArtifactCanonicalJsonReader<T> {
    pub fn new(root: Arc<T>, retirement: Arc<dyn SnapshotRetirementFactory<T>>) -> Self { Self { owned: ManuallyDrop::new(ReaderState::new(root, retirement)) } }
    pub fn completed_bytes(&self) -> u64 { self.owned.completed_bytes() }
    pub fn is_complete(&self) -> bool { self.owned.is_complete() }
    pub fn cancel(&mut self) { self.owned.cancel(); }
    pub fn begin_close(&mut self) { self.owned.begin_close(); }
    pub fn take_root(&mut self) -> Option<Arc<T>> { self.owned.take_root() }
    pub fn terminal_is_empty(&self) -> bool { self.owned.terminal_is_empty() }
    pub fn close_step(&mut self, grant: ArtifactStoreOneItemGrant) -> Result<SnapshotRetirementStep, String> { self.owned.close_step(grant) }
}

impl<T: ArtifactCanonicalJson + Send + 'static> ArtifactCanonicalJsonReader<T> {
    pub fn encode_chunk(&mut self, grant: ArtifactStoreOneItemGrant, output: &mut [u8]) -> Result<usize, String> { self.owned.encode_chunk(grant, output) }
}

impl<T> Drop for ArtifactCanonicalJsonReader<T> {
    fn drop(&mut self) {
        if !self.owned.terminal_is_empty() {
            if !std::thread::panicking() { panic!("canonical reader dropped before exact root transfer or retirement"); }
            return;
        }
        unsafe { ManuallyDrop::drop(&mut self.owned); }
    }
}
//#endregion 📦️ReaderOwnership

//#region 🧪️ReaderLaws
#[cfg(test)]
#[path = "🧪️component.rs"]
mod tests;
//#endregion 🧪️ReaderLaws
