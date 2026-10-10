//! 📖️ Byte-bounded canonical reading over exact frozen typed root ownership.

use super::*;
use std::mem::ManuallyDrop;
use semio_framework_value::{ValueError, RetirementDemand, FactoryAuthority, retained_clone::{RetainedCloneGrant, RetainedCloneStep, RetainedCloneProgress}};

//#region 📦️ReaderOwnership
/// 📖️ Retains a frozen typed Arc and borrowed traversal; no Store publication authority is exposed.
pub struct ArtifactCanonicalJsonReader<T> {
    owned: ManuallyDrop<ReaderState<T>>,
}

struct ReaderState<T> {
    encoder: ArtifactCanonicalEditEncoder,
    root: Option<Arc<T>>,
    retirement: Option<Arc<dyn SnapshotRetirementFactory<T>>>,
    active: Option<Box<dyn ErasedSnapshotRetirement>>,
    factory_close: Option<FactoryAuthority>,
    completed_bytes: u64,
    cancelled: bool,
    failed: bool,
    closing: bool,
}

impl<T> ReaderState<T> {
    fn new(root: Arc<T>, retirement: Arc<dyn SnapshotRetirementFactory<T>>) -> Self {
        Self { encoder: ArtifactCanonicalEditEncoder::default(), root: Some(root), retirement: Some(retirement), active: None, factory_close: None, completed_bytes: 0, cancelled: false, failed: false, closing: false }
    }

    fn completed_bytes(&self) -> u64 {
        self.completed_bytes
    }
    fn is_complete(&self) -> bool {
        !self.cancelled && !self.failed && !self.closing && self.encoder.is_complete()
    }
    fn cancel(&mut self) {
        self.cancelled = true;
    }
    fn begin_close(&mut self) {
        self.closing = true;
    }

    fn take_root(&mut self) -> Option<Arc<T>> {
        let transferable = if self.closing { self.encoder.terminal_is_empty() } else { self.is_complete() };
        if !transferable || self.active.is_some() {
            return None;
        }
        self.encoder.reset().ok()?;
        self.closing = true;
        self.root.take()
    }

    fn terminal_is_empty(&self) -> bool {
        self.closing && self.encoder.terminal_is_empty() && self.root.is_none() && self.retirement.is_none() && self.active.is_none() && self.factory_close.is_none()
    }

}

impl<T: Send + Sync + 'static> ReaderState<T> {
    fn demands(&self, maximum_body_bytes: usize) -> Result<RetirementDemand, ValueError> {
        if !self.encoder.terminal_is_empty() { return Ok(RetirementDemand { depth: 1, ..Default::default() }); }
        if let Some(active) = self.active.as_ref() {
            let mut demand = super::super::artifact_retirement_box_demands(active, maximum_body_bytes)?;
            demand.depth = demand.depth.checked_add(1).ok_or_else(|| ValueError::literal(semio_framework_value::ValueRefusalKind::DepthLimit, "canonical reader depth overflow"))?;
            return Ok(demand);
        }
        if let Some(root) = self.root.as_ref() { return Ok(RetirementDemand { capacity_bytes: self.retirement.as_ref().expect("original reader factory").retirement_birth_bytes(root), depth: 2, ..Default::default() }); }
        if self.retirement.is_some() { return Ok(RetirementDemand { copy_bytes: std::mem::size_of::<Arc<dyn semio_framework_value::FactoryRetirement>>(), depth: 1, ..Default::default() }); }
        if let Some(factory) = self.factory_close.as_ref() { let mut demand = factory.demands(maximum_body_bytes)?; demand.depth = demand.depth.checked_add(1).ok_or_else(|| ValueError::literal(semio_framework_value::ValueRefusalKind::DepthLimit, "reader factory depth overflow"))?; return Ok(demand); }
        Ok(Default::default())
    }
    fn close_step(&mut self, grant: RetainedCloneGrant) -> Result<RetainedCloneStep, ValueError> {
        if self.terminal_is_empty() { return Ok(RetainedCloneStep::Complete(Default::default())); }
        if !self.closing || grant.maximum_items == 0 { return Ok(RetainedCloneStep::Progress(Default::default())); }
        let demand = self.demands(grant.maximum_copy_bytes)?;
        if grant.maximum_depth < demand.depth { return Err(ValueError::literal(semio_framework_value::ValueRefusalKind::DepthLimit, "canonical reader exceeds admitted depth")); }
        if demand.copy_bytes > grant.maximum_copy_bytes || demand.capacity_bytes > grant.maximum_capacity_bytes || demand.release_bytes > grant.maximum_release_bytes { return Ok(RetainedCloneStep::Progress(Default::default())); }
        if !self.encoder.terminal_is_empty() { let progress = self.encoder.close_step(grant)?.progress(); return Ok(if self.terminal_is_empty() { RetainedCloneStep::Complete(progress) } else { RetainedCloneStep::Progress(progress) }); }
        let child = RetainedCloneGrant { maximum_items: 1, maximum_depth: grant.maximum_depth - 1, ..grant };
        if self.active.is_some() { return super::super::artifact_retirement_box_close_step(&mut self.active, child).map(|step| RetainedCloneStep::Progress(step.progress())); }
        if let Some(root) = self.root.take() {
            return match self.retirement.as_ref().expect("reader retains root factory").retire(root, child) {
                Ok((active, progress)) => { self.active = Some(active); if !progress.fits(child) || progress.retained_capacity_bytes != demand.capacity_bytes { return Err(ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated, "canonical reader constructor changed its receipt")); } Ok(RetainedCloneStep::Progress(progress)) },
                Err((error, original)) => { self.root = Some(original); Err(error) },
            };
        }
        if let Some(factory) = self.retirement.take() {
            let factory: Arc<dyn semio_framework_value::FactoryRetirement> = factory;
            self.factory_close = Some(FactoryAuthority::new(factory));
            return Ok(RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: 1, copied_bytes: demand.copy_bytes, ..Default::default() }));
        }
        if let Some(factory) = self.factory_close.as_mut() {
            let step = factory.step(child)?;
            let step = semio_framework_value::retained_clone::admit_retained_clone_close(child, step, factory.terminal_is_empty(), "canonical reader factory")?;
            if factory.terminal_is_empty() { self.factory_close = None; }
            return Ok(if self.terminal_is_empty() { RetainedCloneStep::Complete(step.progress()) } else { RetainedCloneStep::Progress(step.progress()) });
        }
        Ok(RetainedCloneStep::Complete(Default::default()))
    }
}

impl<T: ArtifactCanonicalJson + Send + 'static> ReaderState<T> {
    fn encode_chunk(&mut self, grant: ArtifactStoreOneItemGrant, output: &mut [u8]) -> Result<ArtifactCanonicalJsonTreeStep, ArtifactCanonicalJsonEncodeError> {
        if !grant.permits_one() || self.cancelled || self.failed || self.closing || output.is_empty() {
            return Ok(ArtifactCanonicalJsonTreeStep { ownership: RetainedCloneStep::Progress(Default::default()), written_bytes: 0 });
        }
        let maximum = grant.maximum_copy_bytes.min(output.len()).min(ARTIFACT_CANONICAL_JSON_CHUNK_BYTES);
        let root = self.root.as_ref().ok_or_else(|| ArtifactCanonicalJsonEncodeError { written_bytes: 0, reason: ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated,"canonical-reader.root-missing") })?;
        let result = self.encoder.encode_chunk(root.as_ref(), &mut output[..maximum],grant.retained_grant());
        let count = match &result {
            Ok(step) => step.written_bytes,
            Err(error) => {
                self.failed = true;
                error.written_bytes
            }
        };
        let Some(completed) = self.completed_bytes.checked_add(count as u64) else {
            self.failed = true;
            let progress = match &result { Ok(step) => step.ownership.progress(), Err(error) => error.reason.retained_progress() };
            return Err(ArtifactCanonicalJsonEncodeError { written_bytes: count, reason: ValueError::literal(semio_framework_value::ValueRefusalKind::WorkLimit,"canonical-reader.work-overflow").with_retained_progress(progress) });
        };
        self.completed_bytes = completed;
        result
    }
}

impl<T> ArtifactCanonicalJsonReader<T> {
    pub fn new(root: Arc<T>, retirement: Arc<dyn SnapshotRetirementFactory<T>>) -> Self {
        Self { owned: ManuallyDrop::new(ReaderState::new(root, retirement)) }
    }
    pub fn completed_bytes(&self) -> u64 {
        self.owned.completed_bytes()
    }
    pub fn is_complete(&self) -> bool {
        self.owned.is_complete()
    }
    pub fn cancel(&mut self) {
        self.owned.cancel();
    }
    pub fn begin_close(&mut self) {
        self.owned.begin_close();
    }
    pub fn take_root(&mut self) -> Option<Arc<T>> {
        self.owned.take_root()
    }
    pub fn terminal_is_empty(&self) -> bool {
        self.owned.terminal_is_empty()
    }

}

impl<T: Send + Sync + 'static> ArtifactCanonicalJsonReader<T> {
    pub fn retirement_demands(&self, maximum_body_bytes: usize) -> Result<RetirementDemand, ValueError> { self.owned.demands(maximum_body_bytes) }
    pub fn close_step(&mut self, grant: RetainedCloneGrant) -> Result<RetainedCloneStep, ValueError> { self.owned.close_step(grant) }
}

impl<T: ArtifactCanonicalJson + Send + 'static> ArtifactCanonicalJsonReader<T> {
    pub fn encode_chunk(&mut self, grant: ArtifactStoreOneItemGrant, output: &mut [u8]) -> Result<ArtifactCanonicalJsonTreeStep, ArtifactCanonicalJsonEncodeError> {
        self.owned.encode_chunk(grant, output)
    }
}

impl<T> Drop for ArtifactCanonicalJsonReader<T> {
    fn drop(&mut self) {
        if !self.owned.terminal_is_empty() {
            if !std::thread::panicking() {
                panic!("canonical reader dropped before exact root transfer or retirement");
            }
            return;
        }
        unsafe {
            ManuallyDrop::drop(&mut self.owned);
        }
    }
}
//#endregion 📦️ReaderOwnership

//#region 🧪️ReaderLaws
#[cfg(test)]
#[path = "🧪️tests/📖️reader/🦀️.rs"]
mod tests;
//#endregion 🧪️ReaderLaws
