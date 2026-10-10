//! 🧹️ Retained ownership of a detached local presence root and its complete peer roster.

use super::*;
use semio_framework_value::{ValueError, retained_clone::{RetainedCloneGrant, RetainedCloneProgress, RetainedCloneStep}};

//#region 🧹️StoreRetirement
pub(super) fn advance_returned_local<P: Send + Sync + 'static>(
    registry: &SnapshotReadLeaseRegistry,
    active: &mut Option<Box<dyn ErasedSnapshotRetirement>>,
    factory: Option<&Arc<dyn SnapshotRetirementFactory<P>>>,
    grant: RetainedCloneGrant,
) -> Result<RetainedCloneStep, ValueError> {
    if grant.maximum_items == 0 { return Ok(RetainedCloneStep::Progress(Default::default())); }
    if active.is_some() { return artifact_retirement_box_close_step(active, grant).map(|step| RetainedCloneStep::Progress(step.progress())); }
    if !registry.has_returned() { return Ok(RetainedCloneStep::Complete(Default::default())); }
    let factory = factory.ok_or_else(|| ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated, "presence returned read lacks its original factory"))?;
    match registry.try_admit_one_returned::<P, _>(grant, |root, grant| factory.retire(root, grant)) {
        Ok((owner, receipt)) => {
            *active = owner;
            if !receipt.fits(grant) { return Err(ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated, "presence returned read admission exceeded its grant")); }
            Ok(RetainedCloneStep::Progress(receipt))
        }
        Err(SnapshotReadLeaseRefusal::Busy) => Ok(RetainedCloneStep::Progress(Default::default())),
        Err(reason) => Err(reason.into_value_error()),
    }
}

pub struct PresenceStoreRetirement<P> {
    base_root: std::mem::ManuallyDrop<Option<Arc<PresencePeersRoot<P>>>>,
    local: std::mem::ManuallyDrop<Option<Arc<P>>>,
    terminal_local: std::mem::ManuallyDrop<Option<Arc<P>>>,
    peers: std::mem::ManuallyDrop<Option<Arc<PresencePeersRoot<P>>>>,
    active_local: std::mem::ManuallyDrop<Option<Box<dyn ErasedSnapshotRetirement>>>,
    active_peers: std::mem::ManuallyDrop<Option<PresencePeersRetirement<P>>>,
    reads: std::mem::ManuallyDrop<Option<crate::os_store::SnapshotReadRegistryHandle>>,
    active_returned: std::mem::ManuallyDrop<Option<Box<dyn ErasedSnapshotRetirement>>>,
    local_factory: Option<Arc<dyn SnapshotRetirementFactory<P>>>,
    peer_factory: Option<Arc<dyn SnapshotRetirementFactory<P>>>,
    factory_close: [Option<semio_framework_value::FactoryAuthority>; 2],
}

impl<P: Send + Sync + 'static> PresenceStoreRetirement<P> {
    pub fn retirement_demands(&self, body: usize) -> Result<semio_framework_value::RetirementDemand, ValueError> {
        use semio_framework_value::RetirementDemand;
        let nested = |mut demand: RetirementDemand| -> Result<RetirementDemand, ValueError> { demand.depth = demand.depth.checked_add(1).ok_or_else(|| ValueError::literal(semio_framework_value::ValueRefusalKind::DepthLimit, "presence store retirement depth overflow"))?; Ok(demand) };
        if let Some(active) = self.active_returned.as_ref() { return nested(artifact_retirement_box_demands(active, body)?); }
        if let Some(reads) = self.reads.as_ref().filter(|reads| reads.has_returned()) {
            let factory = self.local_factory.as_ref().ok_or_else(|| ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated, "returned presence read lacks its original factory"))?;
            return nested(reads.returned_admission_demands::<P>(|root| RetirementDemand { capacity_bytes: factory.retirement_birth_bytes(root), depth: 1, ..Default::default() }).map_err(SnapshotReadLeaseRefusal::into_value_error)?);
        }
        if let Some(active) = self.active_local.as_ref() { return nested(artifact_retirement_box_demands(active, body)?); }
        if let Some(active) = self.active_peers.as_ref() { return nested(active.retirement_demands(body)?); }
        if let Some(local) = self.local.as_ref().or(self.terminal_local.as_ref()) { return Ok(RetirementDemand { capacity_bytes: self.local_factory.as_ref().ok_or_else(|| ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated, "presence local owner lacks its original factory"))?.retirement_birth_bytes(local), depth: 2, ..Default::default() }); }
        if self.base_root.is_some() || self.peers.is_some() { return Ok(RetirementDemand { copy_bytes: 0, depth: 1, ..Default::default() }); }
        if self.reads.is_some() { return snapshot_registry_alias_demands(&self.reads); }
        if self.local_factory.is_some() || self.peer_factory.is_some() { return Ok(RetirementDemand { copy_bytes: 0, depth: 1, ..Default::default() }); }
        if let Some(factory) = self.factory_close.iter().flatten().next() { return nested(factory.demands(body)?); }
        Ok(Default::default())
    }
    pub fn close_step(&mut self, grant: RetainedCloneGrant) -> Result<RetainedCloneStep, ValueError> {
        if self.terminal_is_empty() { return Ok(RetainedCloneStep::Complete(Default::default())); }
        if grant.maximum_items == 0 { return Ok(RetainedCloneStep::Progress(Default::default())); }
        let demand = self.retirement_demands(grant.maximum_copy_bytes)?;
        if grant.maximum_depth < demand.depth { return Err(ValueError::literal(semio_framework_value::ValueRefusalKind::DepthLimit, "presence store retirement exceeds admitted depth")); }
        if demand.copy_bytes > grant.maximum_copy_bytes || demand.capacity_bytes > grant.maximum_capacity_bytes || demand.release_bytes > grant.maximum_release_bytes { return Ok(RetainedCloneStep::Progress(Default::default())); }
        let child = RetainedCloneGrant { maximum_items: 1, maximum_depth: grant.maximum_depth - 1, ..grant };
        if let Some(reads) = self.reads.as_ref() {
            if self.active_returned.is_some() || reads.has_returned() { return advance_returned_local(reads, &mut self.active_returned, self.local_factory.as_ref(), child).map(|step| RetainedCloneStep::Progress(step.progress())); }
        }
        if self.active_local.is_some() { return artifact_retirement_box_close_step(&mut self.active_local, child).map(|step| RetainedCloneStep::Progress(step.progress())); }
        if let Some(active) = self.active_peers.as_mut() {
            let step = active.close_step(child)?;
            let step = semio_framework_value::retained_clone::admit_retained_clone_close(child, step, active.terminal_is_empty(), "presence peer roster")?;
            if active.terminal_is_empty() { self.active_peers.take(); }
            return Ok(RetainedCloneStep::Progress(step.progress()));
        }
        let slot = if self.local.is_some() { &mut self.local } else { &mut self.terminal_local };
        if let Some(original) = slot.take() {
            return match self.local_factory.as_ref().expect("observed original local factory").retire(original, child) {
                Ok((owner, receipt)) => { *self.active_local = Some(owner); if !receipt.fits(child) || receipt.retained_capacity_bytes != demand.capacity_bytes { return Err(ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated, "presence local constructor changed its receipt")); } Ok(RetainedCloneStep::Progress(receipt)) },
                Err((error, original)) => { **slot = Some(original); Err(error) },
            };
        }
        let custody = if self.base_root.is_some() { PresencePeerRootCustody::BaseAlias } else { PresencePeerRootCustody::Displaced };
        let root = if self.base_root.is_some() { self.base_root.take() } else { self.peers.take() };
        if let Some(root) = root { *self.active_peers = Some(PresencePeersRetirement::from_root(root, self.peer_factory.clone(), custody)); return Ok(RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: 1, copied_bytes: demand.copy_bytes, ..Default::default() })); }
        if let Some(reads) = self.reads.as_ref() { if !reads.terminal_is_empty() { return Ok(RetainedCloneStep::Progress(Default::default())); } return snapshot_registry_alias_close_step(&mut self.reads, grant).map(|step| RetainedCloneStep::Progress(step.progress())); }
        if let Some(factory) = self.local_factory.take() { let factory: Arc<dyn semio_framework_value::FactoryRetirement> = factory; self.factory_close[0] = Some(semio_framework_value::FactoryAuthority::new(factory)); return Ok(RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: 1, copied_bytes: demand.copy_bytes, ..Default::default() })); }
        if let Some(factory) = self.peer_factory.take() { let factory: Arc<dyn semio_framework_value::FactoryRetirement> = factory; self.factory_close[1] = Some(semio_framework_value::FactoryAuthority::new(factory)); return Ok(RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: 1, copied_bytes: demand.copy_bytes, ..Default::default() })); }
        if let Some(slot) = self.factory_close.iter_mut().find(|slot| slot.is_some()) { let factory = slot.as_mut().unwrap(); let step = factory.step(child)?; let step = semio_framework_value::retained_clone::admit_retained_clone_close(child, step, factory.terminal_is_empty(), "presence store factory")?; if factory.terminal_is_empty() { *slot = None; } return Ok(if self.terminal_is_empty() { RetainedCloneStep::Complete(step.progress()) } else { RetainedCloneStep::Progress(step.progress()) }); }
        Ok(RetainedCloneStep::Complete(Default::default()))
    }
}
impl<P> PresenceStoreRetirement<P> {
    pub fn terminal_is_empty(&self) -> bool { self.base_root.is_none() && self.local.is_none() && self.terminal_local.is_none() && self.peers.is_none() && self.active_local.is_none() && self.active_peers.is_none() && self.active_returned.is_none() && self.reads.is_none() && self.local_factory.is_none() && self.peer_factory.is_none() && self.factory_close.iter().all(Option::is_none) }
}
impl<P> Drop for PresenceStoreRetirement<P> {
    fn drop(&mut self) { if !std::thread::panicking() { assert!(self.terminal_is_empty(), "presence store retirement requires its exact terminal-empty witness"); } }
}
impl<P: Send + Sync + 'static> ErasedSnapshotRetirement for PresenceStoreRetirement<P> {
    fn next_copy_byte_demand(&self) -> Result<usize, ValueError> { Ok(self.retirement_demands(0)?.copy_bytes) }
    fn next_capacity_byte_demand(&self, body: usize) -> Result<usize, ValueError> { Ok(self.retirement_demands(body)?.capacity_bytes) }
    fn next_release_byte_demand(&self) -> Result<usize, ValueError> { Ok(self.retirement_demands(0)?.release_bytes) }
    fn next_depth_demand(&self) -> Result<usize, ValueError> { Ok(self.retirement_demands(0)?.depth) }
    fn close_step(&mut self, grant: RetainedCloneGrant) -> Result<RetainedCloneStep, ValueError> { PresenceStoreRetirement::close_step(self, grant) }
    fn terminal_is_empty(&self) -> bool { PresenceStoreRetirement::terminal_is_empty(self) }
}
impl<P: Send + Sync + 'static> PresencePeersCommit<P> {
    pub fn into_retirement(self, grant: RetainedCloneGrant) -> Result<(PresenceStoreRetirement<P>, RetainedCloneProgress), (ValueError, Self)> {
        let progress = RetainedCloneProgress { copied_items: 1, copied_bytes: 0, ..Default::default() };
        if grant.maximum_items == 0 { return Err((ValueError::literal(semio_framework_value::ValueRefusalKind::WorkLimit, "peer commit retirement requires its original item grant"), self)); }
        if grant.maximum_depth == 0 { return Err((ValueError::literal(semio_framework_value::ValueRefusalKind::DepthLimit, "peer commit retirement requires its original structural grant"), self)); }
        if !progress.fits(grant) { return Err((ValueError::literal(semio_framework_value::ValueRefusalKind::OwnershipLimit, "peer commit retirement exceeds original transfer grant"), self)); }
        let (base_root, root, retirement, factory) = self.into_parts();
        Ok((PresenceStoreRetirement { base_root: std::mem::ManuallyDrop::new(Some(base_root)), local: std::mem::ManuallyDrop::new(None), terminal_local: std::mem::ManuallyDrop::new(None), peers: std::mem::ManuallyDrop::new(Some(root)), active_local: std::mem::ManuallyDrop::new(None), active_peers: std::mem::ManuallyDrop::new(retirement), reads: std::mem::ManuallyDrop::new(None), active_returned: std::mem::ManuallyDrop::new(None), local_factory: None, peer_factory: Some(factory), factory_close: Default::default() }, progress))
    }
}
//#endregion 🧹️StoreRetirement

//#region 🔌️OwnerTransfer
impl<P: Clone + Send + Sync + 'static, M: Mutation<P>> PresenceStore<P, M> {
    /// 🧹️ Detaches exact roots once after the concrete domain validates its empty terminal value.
    pub fn begin_retirement(&mut self, terminal_local: Arc<P>, terminal_is_empty: fn(&P) -> bool) -> Result<PresenceStoreRetirement<P>, (&'static str, Arc<P>)> {
        if self.close_started || !terminal_is_empty(terminal_local.as_ref()) {
            return Err(("presence close requires a fresh store and an exact empty domain terminal", terminal_local));
        }
        if self.local_retirement_factory.is_none() { return Err(("presence close requires its installed local-root retirement factory", terminal_local)); }
        let peers = self.peers.as_ref().expect("live presence peer owner");
        if !peers.is_empty() && self.peer_retirement_factory.is_none() { return Err(("presence close requires its installed peer retirement factory", terminal_local)); }
        self.close_started = true;
        Ok(PresenceStoreRetirement {
            base_root: std::mem::ManuallyDrop::new(None),
            local: std::mem::ManuallyDrop::new(self.local.take()),
            terminal_local: std::mem::ManuallyDrop::new(Some(terminal_local)),
            peers: std::mem::ManuallyDrop::new(self.peers.take()),
            active_local: std::mem::ManuallyDrop::new(None),
            active_peers: std::mem::ManuallyDrop::new(None),
            reads: std::mem::ManuallyDrop::new(self.local_reads.take()),
            active_returned: std::mem::ManuallyDrop::new(self.active_returned_local.take()),
            local_factory: self.local_retirement_factory.take(),
            peer_factory: self.peer_retirement_factory.take(),
            factory_close: Default::default(),
        })
    }

    pub fn retirement_started(&self) -> bool {
        self.close_started
    }
}

impl<P, M> Drop for PresenceStore<P, M> {
    fn drop(&mut self) {
        let terminal = self.close_started && self.local.is_none() && self.peers.is_none() && self.local_reads.is_none() && self.active_returned_local.is_none() && self.local_retirement_factory.is_none() && self.peer_retirement_factory.is_none();
        if !std::thread::panicking() {
            assert!(terminal, "presence store requires its exact detached terminal-empty owner before Drop");
        }
        if terminal {
            unsafe {
                std::mem::ManuallyDrop::drop(&mut self.local);
                std::mem::ManuallyDrop::drop(&mut self.peers);
                std::mem::ManuallyDrop::drop(&mut self.local_reads);
            }
        }
    }
}
//#endregion 🔌️OwnerTransfer

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️testing/🧬️mutations/🦀️.rs"]
mod fixture_mutations;

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
