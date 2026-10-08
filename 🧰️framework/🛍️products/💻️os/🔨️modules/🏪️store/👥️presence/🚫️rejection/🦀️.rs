//! 🚫️ Exact ownership of both rejected peer identity bytes and its typed presence value.

use super::*;
use semio_framework_value::{ValueError, retained_clone::{RetainedCloneGrant, RetainedCloneProgress, RetainedCloneStep}};

//#region 🚫️AdmissionOwner
pub struct PresencePeerAdmissionRejected<P> {
    pub reason: &'static str,
    actor: std::mem::ManuallyDrop<String>,
    presence: std::mem::ManuallyDrop<Option<P>>,
    factory: std::mem::ManuallyDrop<Option<Arc<dyn SnapshotRetirementFactory<P>>>>,
}

impl<P> PresencePeerAdmissionRejected<P> {
    pub(super) fn new(reason: &'static str, actor: String, presence: P, factory: Arc<dyn SnapshotRetirementFactory<P>>) -> Self {
        Self { reason, actor: std::mem::ManuallyDrop::new(actor), presence: std::mem::ManuallyDrop::new(Some(presence)), factory: std::mem::ManuallyDrop::new(Some(factory)) }
    }

    pub fn actor(&self) -> &str {
        self.actor.as_str()
    }

    pub fn presence(&self) -> &P {
        self.presence.as_ref().expect("rejected admission retains its exact presence")
    }

    pub fn retirement_birth_demand(&self) -> semio_framework_value::retained_clone::RetainedCloneBirthDemand {
        semio_framework_value::retained_clone::RetainedCloneBirthDemand { capacity_bytes: std::mem::size_of::<PresencePeerRejectionRetirement<P>>(), depth: 1 }
    }

    pub fn into_retirement(self, grant: RetainedCloneGrant) -> Result<(Box<dyn ErasedSnapshotRetirement>, RetainedCloneProgress), (ValueError, Self)>
    where P: Send + Sync + 'static,
    {
        super::admit_artifact_retirement(self, grant, |mut original| PresencePeerRejectionRetirement {
            actor: std::mem::ManuallyDrop::new(Some(std::mem::take(&mut *original.actor).into_bytes())),
            presence: std::mem::ManuallyDrop::new(original.presence.take()),
            original_arc: std::mem::ManuallyDrop::new(None),
            active: std::mem::ManuallyDrop::new(None),
            factory: original.factory.take(),
            factory_close: None,
        })
    }
}

impl<P> Drop for PresencePeerAdmissionRejected<P> {
    fn drop(&mut self) {
        let terminal = self.actor.is_empty() && self.actor.capacity() == 0 && self.presence.is_none() && self.factory.is_none();
        if !std::thread::panicking() {
            assert!(terminal, "rejected peer admission requires exact actor and presence owner transfer");
        }
    }
}
//#endregion 🚫️AdmissionOwner

//#region 🧹️RetainedRejection
pub(super) struct PresencePeerRejectionRetirement<P> {
    actor: std::mem::ManuallyDrop<Option<Vec<u8>>>,
    presence: std::mem::ManuallyDrop<Option<P>>,
    original_arc: std::mem::ManuallyDrop<Option<Arc<P>>>,
    active: std::mem::ManuallyDrop<Option<Box<dyn ErasedSnapshotRetirement>>>,
    factory: Option<Arc<dyn SnapshotRetirementFactory<P>>>,
    factory_close: Option<semio_framework_value::FactoryAuthority>,
}

impl<P: Send + Sync + 'static> PresencePeerRejectionRetirement<P> {
    fn arc_bytes() -> Result<usize, ValueError> {
        std::alloc::Layout::new::<[usize; 2]>().extend(std::alloc::Layout::new::<P>()).map(|(layout, _)| layout.pad_to_align().size()).map_err(|_| ValueError::literal(semio_framework_value::ValueRefusalKind::AllocationFailed, "presence Arc layout overflow"))
    }
    fn demands(&self, maximum_body_bytes: usize) -> Result<semio_framework_value::RetirementDemand, ValueError> {
        use semio_framework_value::RetirementDemand;
        if let Some(actor) = self.actor.as_ref() { return Ok(RetirementDemand { release_bytes: actor.capacity(), depth: 1, ..Default::default() }); }
        if let Some(active) = self.active.as_ref() {
            let mut demand = super::artifact_retirement_box_demands(active, maximum_body_bytes)?;
            demand.depth = demand.depth.checked_add(1).ok_or_else(|| ValueError::literal(semio_framework_value::ValueRefusalKind::DepthLimit, "presence rejection depth overflow"))?;
            return Ok(demand);
        }
        if self.presence.is_some() { return Ok(RetirementDemand { capacity_bytes: Self::arc_bytes()?, depth: 1, ..Default::default() }); }
        if let Some(original_arc) = self.original_arc.as_ref() {
            return Ok(RetirementDemand { capacity_bytes: self.factory.as_ref().expect("original presence factory").retirement_birth_bytes(original_arc), depth: 2, ..Default::default() });
        }
        if self.factory.is_some() { return Ok(RetirementDemand { copy_bytes: std::mem::size_of::<Arc<dyn semio_framework_value::FactoryRetirement>>(), depth: 1, ..Default::default() }); }
        if let Some(factory) = self.factory_close.as_ref() {
            let mut demand = factory.demands(maximum_body_bytes)?;
            demand.depth = demand.depth.checked_add(1).ok_or_else(|| ValueError::literal(semio_framework_value::ValueRefusalKind::DepthLimit, "presence factory depth overflow"))?;
            return Ok(demand);
        }
        Ok(Default::default())
    }
}

impl<P: Send + Sync + 'static> ErasedSnapshotRetirement for PresencePeerRejectionRetirement<P> {
    fn next_copy_byte_demand(&self) -> Result<usize, ValueError> { Ok(self.demands(0)?.copy_bytes) }
    fn next_capacity_byte_demand(&self, maximum_body_bytes: usize) -> Result<usize, ValueError> { Ok(self.demands(maximum_body_bytes)?.capacity_bytes) }
    fn next_release_byte_demand(&self) -> Result<usize, ValueError> { Ok(self.demands(0)?.release_bytes) }
    fn next_depth_demand(&self) -> Result<usize, ValueError> { Ok(self.demands(0)?.depth) }
    fn close_step(&mut self, grant: RetainedCloneGrant) -> Result<RetainedCloneStep, ValueError> {
        if self.terminal_is_empty() { return Ok(RetainedCloneStep::Complete(Default::default())); }
        if grant.maximum_items == 0 { return Ok(RetainedCloneStep::Progress(Default::default())); }
        let demand = self.demands(grant.maximum_copy_bytes)?;
        if grant.maximum_depth < demand.depth { return Err(ValueError::literal(semio_framework_value::ValueRefusalKind::DepthLimit, "presence rejection exceeds admitted depth")); }
        if demand.copy_bytes > grant.maximum_copy_bytes || demand.capacity_bytes > grant.maximum_capacity_bytes || demand.release_bytes > grant.maximum_release_bytes { return Ok(RetainedCloneStep::Progress(Default::default())); }
        if let Some(actor) = self.actor.take() {
            let bytes = actor.capacity();
            drop(actor);
            return Ok(RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: 1, released_bytes: bytes, ..Default::default() }));
        }
        let child = RetainedCloneGrant { maximum_items: 1, maximum_depth: grant.maximum_depth - 1, ..grant };
        if self.active.is_some() { return super::artifact_retirement_box_close_step(&mut self.active, child).map(|step| RetainedCloneStep::Progress(step.progress())); }
        if let Some(presence) = self.presence.take() {
            *self.original_arc = Some(Arc::new(presence));
            return Ok(RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: 1, retained_capacity_bytes: demand.capacity_bytes, ..Default::default() }));
        }
        if let Some(original_arc) = self.original_arc.take() {
            return match self.factory.as_ref().expect("original presence factory").retire(original_arc, child) {
                Ok((active, progress)) => {
                    if !progress.fits(child) || progress.retained_capacity_bytes != demand.capacity_bytes { *self.active = Some(active); return Err(ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated, "presence rejection constructor changed its declared receipt")); }
                    *self.active = Some(active);
                    Ok(RetainedCloneStep::Progress(progress))
                },
                Err((error, original_arc)) => { *self.original_arc = Some(original_arc); Err(error) },
            };
        }
        if let Some(factory) = self.factory.take() {
            let factory: Arc<dyn semio_framework_value::FactoryRetirement> = factory;
            self.factory_close = Some(semio_framework_value::FactoryAuthority::new(factory));
            return Ok(RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: 1, copied_bytes: demand.copy_bytes, ..Default::default() }));
        }
        if let Some(factory) = self.factory_close.as_mut() {
            let step = factory.step(child)?;
            let step = semio_framework_value::retained_clone::admit_retained_clone_close(child, step, factory.terminal_is_empty(), "presence rejection factory")?;
            if factory.terminal_is_empty() { self.factory_close = None; }
            return Ok(if self.terminal_is_empty() { RetainedCloneStep::Complete(step.progress()) } else { RetainedCloneStep::Progress(step.progress()) });
        }
        Ok(RetainedCloneStep::Complete(Default::default()))
    }
    fn terminal_is_empty(&self) -> bool { self.actor.is_none() && self.presence.is_none() && self.original_arc.is_none() && self.active.is_none() && self.factory.is_none() && self.factory_close.is_none() }
}

impl<P> Drop for PresencePeerRejectionRetirement<P> {
    fn drop(&mut self) {
        assert!(std::thread::panicking() || (self.actor.is_none() && self.presence.is_none() && self.original_arc.is_none() && self.active.is_none() && self.factory.is_none() && self.factory_close.is_none()), "rejected peer cursor requires its exact empty terminal witness");
    }
}
//#endregion 🧹️RetainedRejection

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
