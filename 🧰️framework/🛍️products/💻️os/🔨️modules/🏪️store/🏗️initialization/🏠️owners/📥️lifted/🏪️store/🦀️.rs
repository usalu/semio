//! 📥️ Items of `store` that belong to `os-store-engine-owners` (PLAN-OS-KERNEL-CRATES.md, Appendix C): lifted out of `🏪️store/🦀️.rs`.
use semio_framework_os_store_text_io::ArtifactStoreSnapshot as _;
use semio_framework_value::{ArtifactOwnedValueRetirementFactory, ErasedSnapshotRetirement};
use semio_framework_value::retained_clone::{RetainedCloneGrant, RetainedCloneProgress, RetainedCloneStep};
use semio_framework_value::FromValue;
use semio_framework_value::ToValue;
use semio_framework_value::ValueError;
use semio_framework_os_spr::{Mutation, OpBinary, OpText};
use std::marker::PhantomData;
use std::sync::Arc;
use semio_framework_os_store_read::{artifact_retirement_box_close_step, artifact_retirement_box_demands};
use semio_framework_os_store_read::{ArtifactStoreCursorDisposerPhase, SnapshotReadLeaseRegistryOwner, advance_returned_snapshot_read};
//#region 🔖️EphemeralLanes

use semio_framework_os_store_ephemeral::*;
use semio_framework_os_store_backbone::Backbone;
use semio_framework_os_store_backbone::backbone_retirement::ArtifactStoreBackboneRetirement;
//#region 🧬️OpaqueSnapshotRead
/// 🔌️ What the owned disposers may ask of the store being closed: the close contract is base, the engine type behind it is not named below. The composer implements it by delegating to its components.
pub trait ArtifactStoreCloseHost<P, Mutation>
where
    P: Clone + ToValue + FromValue,
    Mutation: Clone + ToValue + FromValue + self::Mutation<P>,
{
    fn close_host_member_inbox_capacity(&self) -> usize;
    fn close_host_backbone_attached(&self) -> bool;
    fn close_host_prefix_ring_is_empty(&self) -> bool;
    fn close_host_snapshot_read_leases(&self) -> &SnapshotReadLeaseRegistryOwner;
    fn close_host_initial_snapshot_retirement_factory(&self) -> Option<&Arc<dyn ArtifactOwnedValueRetirementFactory<P>>>;
    fn snapshot_read_leases_terminal_is_empty(&self) -> bool;
    fn maintenance_retirements_terminal_is_empty(&self) -> bool;
    fn maintenance_retirements_step(&mut self, grant: RetainedCloneGrant) -> Result<RetainedCloneStep, ValueError>;
    fn close_take_backbone_retirement(&mut self, grant: RetainedCloneGrant) -> Result<(Option<Box<dyn ErasedSnapshotRetirement>>, RetainedCloneProgress), ValueError>;
    fn close_cursor_admit(&mut self, phase: ArtifactStoreCursorDisposerPhase, grant: RetainedCloneGrant) -> Result<(Option<Box<dyn ErasedSnapshotRetirement>>, RetainedCloneProgress), ValueError>;
    fn close_cursor_demands(&self, phase: ArtifactStoreCursorDisposerPhase, body: usize) -> Result<semio_framework_value::RetirementDemand, ValueError>;
    fn owned_roots_terminal_is_empty(&self) -> bool;
}

pub struct ArtifactStoreCloseView<'a, P, Mutation>
where
    P: Clone + ToValue + FromValue,
    Mutation: Clone + ToValue + FromValue + self::Mutation<P>,
{
    pub store: &'a mut dyn ArtifactStoreCloseHost<P, Mutation>,
}

impl<P, Mutation> ArtifactStoreCloseView<'_, P, Mutation>
where
    P: Clone + ToValue + FromValue + Send + Sync + 'static,
    Mutation: Clone + ToValue + FromValue + self::Mutation<P> + OpBinary + OpText + Send + 'static,
{
    /// 🎟️ Quotes the original transport retirement frame without detaching a queue or channel.
    pub fn backbone_retirement_birth_demand(&self) -> semio_framework_value::RetirementDemand {
        if self.store.close_host_member_inbox_capacity() != 0 || self.store.close_host_backbone_attached() {
            semio_framework_value::RetirementDemand { capacity_bytes: ArtifactStoreBackboneRetirement::constructor_capacity_bytes(), depth: 1, ..Default::default() }
        } else { Default::default() }
    }

    /// 🔗️ Admits the original transport frame under the caller's unchanged common grant.
    pub fn take_backbone_retirement(&mut self, grant: RetainedCloneGrant) -> Result<(Option<Box<dyn ErasedSnapshotRetirement>>, RetainedCloneProgress), ValueError> {
        self.store.close_take_backbone_retirement(grant)
    }
}

pub trait ArtifactStoreOwnedDisposer<P, Mutation>: Send
where
    P: Clone + ToValue + FromValue,
    Mutation: Clone + ToValue + FromValue + self::Mutation<P>,
{
    fn close_step(&mut self, store: &mut ArtifactStoreCloseView<'_, P, Mutation>, grant: RetainedCloneGrant) -> Result<RetainedCloneStep, ValueError>;
    fn terminal_is_empty(&self, store: &dyn ArtifactStoreCloseHost<P, Mutation>) -> bool;
    fn demands(&self, store: &dyn ArtifactStoreCloseHost<P, Mutation>, maximum_body_bytes: usize) -> Result<semio_framework_value::RetirementDemand, ValueError>;
    fn close_uninstalled_step(&mut self, grant: RetainedCloneGrant) -> Result<RetainedCloneStep, ValueError>;
    fn uninstalled_demands(&self, maximum_body_bytes: usize) -> Result<semio_framework_value::RetirementDemand, ValueError>;
    fn uninstalled_terminal_is_empty(&self) -> bool;

    /// 🔭️ Which owner family this disposer is currently draining. `close_step` answers only
    /// `Pending`/`Blocked`/`Complete`, so a store that will not close cannot otherwise say WHICH of
    /// its cursor phases refuses.
    fn close_phase_witness(&self) -> String {
        "unknown".to_string()
    }
}

/// 🧹️ Explicit store-owner cursor used only when a domain installs it through its
/// owner catalog. It advances one detached authority at a time and never supplies a default proof.
pub struct ArtifactStoreCursorDisposer<P, Mutation> {
    pub phase: ArtifactStoreCursorDisposerPhase,
    pub started: bool,
    pub active: std::mem::ManuallyDrop<Option<Box<dyn ErasedSnapshotRetirement>>>,
    pub marker: PhantomData<fn() -> (P, Mutation)>,
}

impl<P, Mutation> Default for ArtifactStoreCursorDisposer<P, Mutation> {
    fn default() -> Self {
        Self::new()
    }
}

impl<P, Mutation> ArtifactStoreCursorDisposer<P, Mutation> {
    pub fn new() -> Self {
        Self { phase: ArtifactStoreCursorDisposerPhase::ReturnedReads, started: false, active: std::mem::ManuallyDrop::new(None), marker: PhantomData }
    }

}

impl<P, Mutation> ArtifactStoreOwnedDisposer<P, Mutation> for ArtifactStoreCursorDisposer<P, Mutation>
where
    P: Clone + ToValue + FromValue + Send + Sync + 'static,
    Mutation: Clone + ToValue + FromValue + self::Mutation<P> + OpBinary + OpText + Send + 'static,
{
    fn close_step(&mut self, view: &mut ArtifactStoreCloseView<'_, P, Mutation>, grant: RetainedCloneGrant) -> Result<RetainedCloneStep, ValueError> {
        let empty = RetainedCloneProgress::default();
        if self.terminal_is_empty(&*view.store) { return Ok(RetainedCloneStep::Complete(empty)); }
        if grant.maximum_items == 0 { return Ok(RetainedCloneStep::Progress(empty)); }
        let demand = self.demands(&*view.store, grant.maximum_copy_bytes)?;
        if grant.maximum_depth < demand.depth { return Err(ValueError::literal(semio_framework_value::ValueRefusalKind::DepthLimit, "installed cursor exceeds admitted owner depth")); }
        if grant.maximum_copy_bytes < demand.copy_bytes || grant.maximum_capacity_bytes < demand.capacity_bytes || grant.maximum_release_bytes < demand.release_bytes { return Ok(RetainedCloneStep::Progress(empty)); }
        self.started = true;
        if self.active.is_some() { return artifact_retirement_box_close_step(&mut self.active, RetainedCloneGrant { maximum_items: 1, ..grant }).map(|step| RetainedCloneStep::Progress(step.progress())); }
        let store = &mut *view.store;
        match self.phase {
            ArtifactStoreCursorDisposerPhase::ReturnedReads => {
                if store.snapshot_read_leases_terminal_is_empty() { self.phase = ArtifactStoreCursorDisposerPhase::Displaced; }
                else {
                    let factory = store.close_host_initial_snapshot_retirement_factory().ok_or_else(|| ValueError::literal(semio_framework_value::ValueRefusalKind::UnsupportedOwner, "returned read requires installed snapshot factory"))?;
                    return advance_returned_snapshot_read(store.close_host_snapshot_read_leases(), &mut self.active, factory, grant).map(|step| RetainedCloneStep::Progress(step.progress()));
                }
            }
            ArtifactStoreCursorDisposerPhase::Displaced => {
                if !store.maintenance_retirements_terminal_is_empty() { return store.maintenance_retirements_step(grant).map(|step| RetainedCloneStep::Progress(step.progress())); }
                self.phase = ArtifactStoreCursorDisposerPhase::RuntimeOwners(0);
            }
            ArtifactStoreCursorDisposerPhase::Complete => return Ok(RetainedCloneStep::Complete(empty)),
            phase => {
                let (owner, receipt) = store.close_cursor_admit(phase, RetainedCloneGrant { maximum_items: 1, maximum_depth: grant.maximum_depth - 1, ..grant })?;
                *self.active = owner;
                if !receipt.fits(grant) { return Err(ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated, "installed cursor birth exceeded original grant")); }
                self.phase = match phase {
                    ArtifactStoreCursorDisposerPhase::RuntimeOwners(lane) if lane < 7 => ArtifactStoreCursorDisposerPhase::RuntimeOwners(lane + 1),
                    ArtifactStoreCursorDisposerPhase::RuntimeOwners(_) => ArtifactStoreCursorDisposerPhase::PrefixSnapshots,
                    ArtifactStoreCursorDisposerPhase::PrefixSnapshots if store.close_host_prefix_ring_is_empty() => ArtifactStoreCursorDisposerPhase::TailSnapshot,
                    ArtifactStoreCursorDisposerPhase::TailSnapshot => ArtifactStoreCursorDisposerPhase::CurrentSnapshot,
                    ArtifactStoreCursorDisposerPhase::CurrentSnapshot => ArtifactStoreCursorDisposerPhase::Backbone,
                    ArtifactStoreCursorDisposerPhase::Backbone if store.close_host_member_inbox_capacity() == 0 && !store.close_host_backbone_attached() => ArtifactStoreCursorDisposerPhase::Causal,
                    ArtifactStoreCursorDisposerPhase::Causal => ArtifactStoreCursorDisposerPhase::Envelope,
                    ArtifactStoreCursorDisposerPhase::Envelope => ArtifactStoreCursorDisposerPhase::Complete,
                    phase => phase,
                };
                return Ok(RetainedCloneStep::Progress(receipt));
            }
        }
        Ok(RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: 1, ..empty }))
    }

    fn demands(&self, store: &dyn ArtifactStoreCloseHost<P, Mutation>, maximum_body_bytes: usize) -> Result<semio_framework_value::RetirementDemand, ValueError> {
        if let Some(active) = self.active.as_ref() { return artifact_retirement_box_demands(active, maximum_body_bytes); }
        store.close_cursor_demands(self.phase, maximum_body_bytes)
    }

    fn terminal_is_empty(&self, store: &dyn ArtifactStoreCloseHost<P, Mutation>) -> bool {
        self.phase == ArtifactStoreCursorDisposerPhase::Complete && self.active.is_none() && store.owned_roots_terminal_is_empty()
    }

    fn close_uninstalled_step(&mut self, grant: RetainedCloneGrant) -> Result<RetainedCloneStep, ValueError> {
        let empty = RetainedCloneProgress::default();
        if self.started || self.active.is_some() { return Err(ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated, "installed store disposer cannot retire as uninstalled")); }
        if self.phase == ArtifactStoreCursorDisposerPhase::Complete { return Ok(RetainedCloneStep::Complete(empty)); }
        if grant.maximum_items == 0 { return Ok(RetainedCloneStep::Progress(empty)); }
        if grant.maximum_depth == 0 { return Err(ValueError::literal(semio_framework_value::ValueRefusalKind::DepthLimit, "uninstalled disposer requires logical owner depth")); }
        self.phase = ArtifactStoreCursorDisposerPhase::Complete;
        Ok(RetainedCloneStep::Complete(RetainedCloneProgress { copied_items: 1, ..empty }))
    }

    fn uninstalled_demands(&self, _body: usize) -> Result<semio_framework_value::RetirementDemand, ValueError> {
        if self.started || self.active.is_some() { return Err(ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated, "installed disposer has no uninstalled custody")); }
        Ok(semio_framework_value::RetirementDemand { depth: usize::from(self.phase != ArtifactStoreCursorDisposerPhase::Complete), ..Default::default() })
    }

    fn uninstalled_terminal_is_empty(&self) -> bool {
        !self.started && self.phase == ArtifactStoreCursorDisposerPhase::Complete && self.active.is_none()
    }

    fn close_phase_witness(&self) -> String {
        format!("{:?}/started={}/active={}", self.phase, self.started, self.active.is_some())
    }
}

impl<P, Mutation> Drop for ArtifactStoreCursorDisposer<P, Mutation> {
    /// 🔒️ Fail-closed on a live drop, but NOT while the thread is already unwinding — a second
    /// panic inside a destructor during cleanup is a non-unwinding abort, which turns any single
    /// failing caller into a whole-process kill and hides every later diagnosis. Same guard the
    /// other retirement roots carry (`OrderedMap::drop`, `FlowRetirement`, `RootRetirement`).
    fn drop(&mut self) {
        assert!(std::thread::panicking() || (self.phase == ArtifactStoreCursorDisposerPhase::Complete && self.active.is_none()), "artifact store cursor disposer reached Drop before terminal-empty ownership");
    }
}
//#endregion 🧬️OpaqueSnapshotRead

//#endregion 🔖️EphemeralLanes
