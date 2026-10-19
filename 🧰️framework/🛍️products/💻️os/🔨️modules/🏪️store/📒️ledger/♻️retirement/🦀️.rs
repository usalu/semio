//! ♻️ Retirement of ledger entries: decoded edits and their factory, changes, and the paged operation-row owners with
//! their row retirement. The ledger, the snapshot-read owner catalogs and the owned schema decoder retire through them.
//!
//! Layering: `📓️2026-10-13-wave-e1-os-kernel.md` (ticket BUILD-SEMIO-TECH-PLAY-FOR-CDN-DEPLOYMENT).
use semio_framework_value::{ArtifactOwnedValueRetirementFactory, ErasedSnapshotRetirement, SnapshotRetirementFactory};
use semio_framework_value::retained_clone::{RetainedCloneGrant, RetainedCloneProgress, RetainedCloneStep};
use semio_framework_value::ValueError;
use semio_framework_os_spr::{Edit, Mutation};
use std::sync::Arc;
use semio_framework_value_snapshot_read::{artifact_retirement_box_close_step, artifact_retirement_box_demands};
use semio_framework_value_retirement_owner::{retirement_owner_close, retirement_owner_demands};
use crate::lifted_snapshot_read::{ArtifactStoreEditRetirement, ArtifactStoreHistoryMetadataRetirement, ArtifactStoreOperationRowOwners};
use semio_framework_value::retirement::frame::admit_retirement_frame as admit_artifact_retirement;
use semio_framework_os_vcs::Change;

use semio_framework_os_pack::pack_rt;

impl<Mu> From<semio_framework_value::list::PagedList<Mu, {usize::MAX}>> for ArtifactStoreOperationRowOwners<Mu> {
    fn from(rows: semio_framework_value::list::PagedList<Mu, {usize::MAX}>) -> Self { Self::Inverse(rows) }
}

impl<Mu> ArtifactStoreOperationRowOwners<Mu> {
    pub fn last(&self) -> Option<&Mu> { match self { Self::Forwards(rows) => rows.last(), Self::Inverse(rows) => rows.last() } }
    pub fn pop(&mut self) -> Option<Mu> { match self { Self::Forwards(rows) => rows.pop(), Self::Inverse(rows) => rows.pop() } }
    pub fn terminal_is_empty(&self) -> bool { match self { Self::Forwards(rows) => rows.is_empty() && (rows.capacity() == 0 || size_of::<Mu>() == 0), Self::Inverse(rows) => rows.terminal_is_empty() } }
    pub fn release_demand(&self) -> Result<usize, ValueError> {
        match self {
            Self::Forwards(rows) => rows.capacity().checked_mul(size_of::<Mu>()).ok_or_else(|| ValueError::literal(semio_framework_value::ValueRefusalKind::OwnershipLimit, "operation row backing overflow")),
            Self::Inverse(rows) => rows.next_release_allocation_bytes().map_err(ValueError::from),
        }
    }
    fn release_empty(&mut self, grant: RetainedCloneGrant) -> Result<RetainedCloneStep, ValueError> {
        use semio_framework_value::retained_clone::{RetainedCloneStep, RetainedCloneProgress};
        let empty = RetainedCloneProgress::default();
        if self.last().is_some() { return Err(ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated, "operation payload remains before backing release")); }
        if grant.maximum_items == 0 || grant.maximum_release_bytes < self.release_demand()? { return Ok(RetainedCloneStep::Progress(empty)); }
        if grant.maximum_depth == 0 { return Err(ValueError::literal(semio_framework_value::ValueRefusalKind::DepthLimit, "operation backing requires admitted depth")); }
        match self {
            Self::Forwards(rows) => {
                let released_bytes = rows.capacity().checked_mul(size_of::<Mu>()).ok_or_else(|| ValueError::literal(semio_framework_value::ValueRefusalKind::OwnershipLimit, "operation row backing overflow"))?;
                drop(std::mem::take(rows));
                Ok(RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: 1, released_bytes, ..empty }))
            }
            Self::Inverse(rows) => {
                let progress = rows.release_empty_page(grant.maximum_release_bytes).map_err(ValueError::from)?;
                Ok(RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: usize::from(progress.progressed), released_bytes: progress.released_allocation_bytes, ..empty }))
            }
        }
    }
}

/// 🧹️ Retires each original operation and its backing through admitted current domain authority.
pub struct ArtifactStoreOperationRowsRetirement<Mu> {
    pub rows: std::mem::ManuallyDrop<ArtifactStoreOperationRowOwners<Mu>>,
    pub identity: std::mem::ManuallyDrop<Option<String>>,
    pub active: std::mem::ManuallyDrop<Option<Box<dyn ErasedSnapshotRetirement>>>,
    pub pending: std::mem::ManuallyDrop<Option<Mu>>,
    pub factory: std::mem::ManuallyDrop<Option<Arc<dyn ArtifactOwnedValueRetirementFactory<Mu>>>>,
    pub closing_factory: std::mem::ManuallyDrop<Option<semio_framework_value::FactoryAuthority>>,
    pub strings: std::mem::ManuallyDrop<Option<semio_framework_value::retirement::controlled::ControlledRetirement<Vec<String>>>>,
}

impl<Mu> ArtifactStoreOperationRowsRetirement<Mu> {
    pub fn new(rows: impl Into<ArtifactStoreOperationRowOwners<Mu>>, factory: Arc<dyn ArtifactOwnedValueRetirementFactory<Mu>>) -> Self { Self { rows: std::mem::ManuallyDrop::new(rows.into()), identity: std::mem::ManuallyDrop::new(None), active: std::mem::ManuallyDrop::new(None), pending: std::mem::ManuallyDrop::new(None), factory: std::mem::ManuallyDrop::new(Some(factory)), closing_factory: std::mem::ManuallyDrop::new(None), strings: std::mem::ManuallyDrop::new(None) } }
    pub fn with_identity(mut self, identity: String) -> Self { *self.identity = Some(identity); self }
    pub fn with_strings(mut self, strings: Vec<String>) -> Self { *self.strings = Some(semio_framework_value::retirement::controlled::ControlledRetirement::new(strings).unwrap_or_else(|_| unreachable!("String Vec declares controlled retirement"))); self }
    pub fn demands(&self, body: usize) -> Result<semio_framework_value::RetirementDemand, ValueError> {
        if let Some(active) = self.active.as_ref() { return artifact_retirement_box_demands(active, body); }
        if let Some(row) = self.pending.as_ref().or_else(|| self.rows.last()) { return Ok(semio_framework_value::RetirementDemand { capacity_bytes: self.factory.as_ref().expect("operation payload retains its factory").retirement_birth_bytes(row), depth: 2, ..Default::default() }); }
        if !self.rows.terminal_is_empty() { return Ok(semio_framework_value::RetirementDemand { release_bytes: self.rows.release_demand()?, depth: 1, ..Default::default() }); }
        if self.identity.is_some() { return Ok(semio_framework_value::RetirementDemand { release_bytes: self.identity.as_ref().unwrap().capacity(), depth: 1, ..Default::default() }); }
        if self.strings.is_some() { return retirement_owner_demands(&self.strings, body); }
        if self.factory.is_some() { return Ok(semio_framework_value::RetirementDemand { copy_bytes: size_of::<Arc<dyn ArtifactOwnedValueRetirementFactory<Mu>>>(), depth: 1, ..Default::default() }); }
        if let Some(owner) = self.closing_factory.as_ref() {
            let mut demand = owner.demands(body)?;
            demand.depth = demand.depth.checked_add(1).ok_or_else(|| ValueError::literal(semio_framework_value::ValueRefusalKind::DepthLimit, "operation factory owner depth overflow"))?;
            return Ok(demand);
        }
        Ok(Default::default())
    }
}

impl<Mu: Send + 'static> ErasedSnapshotRetirement for ArtifactStoreOperationRowsRetirement<Mu> {
    fn close_step(&mut self, grant: RetainedCloneGrant) -> Result<RetainedCloneStep, ValueError> {
        use semio_framework_value::retained_clone::{RetainedCloneStep, RetainedCloneProgress, admit_retained_clone_progress};
        let empty = RetainedCloneProgress::default();
        if self.terminal_is_empty() { return Ok(RetainedCloneStep::Complete(empty)); }
        if grant.maximum_items == 0 { return Ok(RetainedCloneStep::Progress(empty)); }
        let demand = self.demands(grant.maximum_copy_bytes)?;
        if grant.maximum_depth < demand.depth { return Err(ValueError::literal(semio_framework_value::ValueRefusalKind::DepthLimit, "operation retirement exceeds admitted depth")); }
        if grant.maximum_copy_bytes < demand.copy_bytes || grant.maximum_capacity_bytes < demand.capacity_bytes || grant.maximum_release_bytes < demand.release_bytes { return Ok(RetainedCloneStep::Progress(empty)); }
        if self.active.is_some() { return artifact_retirement_box_close_step(&mut self.active, grant); }
        if let Some(row) = self.pending.take().or_else(|| self.rows.pop()) {
            let child = RetainedCloneGrant { maximum_depth: grant.maximum_depth - 1, ..grant };
            return match self.factory.as_ref().expect("operation payload retains its factory").retire_owned(row, child) {
                Ok((owner, progress)) => { *self.active = Some(owner); admit_retained_clone_progress(child, progress, "operation row retirement birth")?; if progress.retained_capacity_bytes != demand.capacity_bytes { return Err(ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated, "operation row original factory changed admitted constructor receipt")); } Ok(RetainedCloneStep::Progress(progress)) }
                Err((error, row)) => { *self.pending = Some(row); Err(error) }
            };
        }
        if !self.rows.terminal_is_empty() { return self.rows.release_empty(grant); }
        if self.identity.is_some() {
            drop(self.identity.take());
            return Ok(RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: 1, released_bytes: demand.release_bytes, ..empty }));
        }
        if self.strings.is_some() { return retirement_owner_close(&mut self.strings, grant).map(|step| RetainedCloneStep::Progress(step.progress())); }
        if let Some(factory) = self.factory.take() {
            let factory: Arc<dyn semio_framework_value::FactoryRetirement> = factory;
            *self.closing_factory = Some(semio_framework_value::FactoryAuthority::new(factory));
            return Ok(RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: 1, copied_bytes: demand.copy_bytes, ..empty }));
        }
        let owner = self.closing_factory.as_mut().expect("remaining original operation factory");
        let child = RetainedCloneGrant { maximum_depth: grant.maximum_depth - 1, ..grant };
        let step = owner.step(child)?;
        semio_framework_value::retained_clone::admit_retained_clone_close(child, step, owner.terminal_is_empty(), "original operation row factory")?;
        if owner.terminal_is_empty() { drop(self.closing_factory.take()); }
        Ok(if self.terminal_is_empty() { RetainedCloneStep::Complete(step.progress()) } else { RetainedCloneStep::Progress(step.progress()) })
    }
    fn terminal_is_empty(&self) -> bool { self.rows.terminal_is_empty() && self.identity.is_none() && self.active.is_none() && self.pending.is_none() && self.strings.is_none() && self.factory.is_none() && self.closing_factory.is_none() }
    fn next_copy_byte_demand(&self) -> Result<usize, ValueError> { Ok(self.demands(0)?.copy_bytes) }
    fn next_capacity_byte_demand(&self, body: usize) -> Result<usize, ValueError> { Ok(self.demands(body)?.capacity_bytes) }
    fn next_release_byte_demand(&self) -> Result<usize, ValueError> { Ok(self.demands(0)?.release_bytes) }
    fn next_depth_demand(&self) -> Result<usize, ValueError> { Ok(self.demands(0)?.depth) }
    fn next_demand(&self, body: usize) -> Result<semio_framework_value::RetirementDemand, ValueError> { self.demands(body) }
}

impl<Mu> Drop for ArtifactStoreOperationRowsRetirement<Mu> {
    fn drop(&mut self) { assert!(std::thread::panicking() || (self.rows.terminal_is_empty() && self.identity.is_none() && self.active.is_none() && self.pending.is_none() && self.strings.is_none() && self.factory.is_none() && self.closing_factory.is_none()), "operation row retirement retains exact original owners until bounded terminal"); if self.rows.terminal_is_empty() { unsafe { std::mem::ManuallyDrop::drop(&mut self.rows); } } }
}

pub struct ArtifactStoreDecodedEditRetirement<Mutation> {
    pub edit: std::mem::ManuallyDrop<Option<Edit<Mutation>>>,
    pub active: std::mem::ManuallyDrop<Option<Box<dyn ErasedSnapshotRetirement>>>,
    pub mutation_factory: std::mem::ManuallyDrop<Option<Arc<dyn ArtifactOwnedValueRetirementFactory<Mutation>>>>,
    pub closing_factory: std::mem::ManuallyDrop<Option<semio_framework_value::FactoryAuthority>>,
    pub pending_mutation: std::mem::ManuallyDrop<Option<Mutation>>,
}

#[derive(semio_framework_value::FactoryPayloadRetirement)]
pub struct ArtifactStoreDecodedEditRetirementFactory<Mutation> {
    #[factory_child]
    pub mutation_factory: Arc<dyn ArtifactOwnedValueRetirementFactory<Mutation>>,
}


/// 🧮️ Quotes the original decoded Edit issuer Arc and its installed mutation factory alias.
pub(crate) fn history_edit_read_issuer_demands<Mutation:Send+'static>()->semio_framework_value::RetirementDemand {
    semio_framework_value::RetirementDemand{copy_bytes:2*size_of::<Arc<dyn ArtifactOwnedValueRetirementFactory<Mutation>>>(),capacity_bytes:semio_framework_value::factory_arc_birth_bytes::<ArtifactStoreDecodedEditRetirementFactory<Mutation>>(),depth:1,..Default::default()}
}
/// 🎟️ Admits the existing decoded Edit retirement mechanism using its actual mutation factory.
pub(crate) fn admit_history_edit_read_issuer<Mutation:Send+'static>(original:&Arc<dyn ArtifactOwnedValueRetirementFactory<Mutation>>,slot:&mut Option<Arc<dyn ArtifactOwnedValueRetirementFactory<Edit<Mutation>>>>,grant:RetainedCloneGrant)->Result<RetainedCloneStep,ValueError> {
    if slot.is_some(){return Ok(RetainedCloneStep::Complete(Default::default()));}
    let demand=history_edit_read_issuer_demands::<Mutation>();
    if !semio_framework_value::retained_clone::retained_clone_grant_admits(grant,demand){return Ok(RetainedCloneStep::Progress(Default::default()));}
    *slot=Some(Arc::new(ArtifactStoreDecodedEditRetirementFactory{mutation_factory:Arc::clone(original)}));
    Ok(RetainedCloneStep::Complete(RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,retained_capacity_bytes:demand.capacity_bytes,..Default::default()}))
}

impl<Mutation: Send + 'static> ArtifactOwnedValueRetirementFactory<Edit<Mutation>> for ArtifactStoreDecodedEditRetirementFactory<Mutation> {
    fn retirement_birth_bytes(&self, _value: &Edit<Mutation>) -> usize { size_of::<ArtifactStoreDecodedEditRetirement<Mutation>>() }
    fn retire_owned(&self, value: Edit<Mutation>, grant: RetainedCloneGrant) -> Result<(Box<dyn ErasedSnapshotRetirement>, RetainedCloneProgress), (ValueError, Edit<Mutation>)> {
        admit_artifact_retirement(value, grant, |value| ArtifactStoreDecodedEditRetirement::new(value, self.mutation_factory.clone()))
    }
}

#[derive(semio_framework_value::FactoryPayloadRetirement)]
pub struct ArtifactStoreChangeRetirementFactory;

impl ArtifactOwnedValueRetirementFactory<Change> for ArtifactStoreChangeRetirementFactory {
    fn retirement_birth_bytes(&self, _value: &Change) -> usize { size_of::<ArtifactStoreHistoryMetadataRetirement>() }
    fn retire_owned(&self, value: Change, grant: RetainedCloneGrant) -> Result<(Box<dyn ErasedSnapshotRetirement>, RetainedCloneProgress), (ValueError, Change)> {
        admit_artifact_retirement(value, grant, ArtifactStoreHistoryMetadataRetirement::change)
    }
}

impl<Mutation> ArtifactStoreDecodedEditRetirement<Mutation> {
    pub fn new(edit: Edit<Mutation>, mutation_factory: Arc<dyn ArtifactOwnedValueRetirementFactory<Mutation>>) -> Self {
        Self { edit: std::mem::ManuallyDrop::new(Some(edit)), active: std::mem::ManuallyDrop::new(None), pending_mutation: std::mem::ManuallyDrop::new(None), mutation_factory: std::mem::ManuallyDrop::new(Some(mutation_factory)), closing_factory: std::mem::ManuallyDrop::new(None) }
    }
    pub fn demands(&self, body: usize) -> Result<semio_framework_value::RetirementDemand, ValueError> {
        if let Some(active) = self.active.as_ref() { return artifact_retirement_box_demands(active, body); }
        let mutation = self.pending_mutation.as_ref().or_else(|| self.edit.as_ref().and_then(|edit| edit.inverse.last().or_else(|| edit.forwards.last())));
        if let Some(mutation) = mutation { return Ok(semio_framework_value::RetirementDemand { copy_bytes: 2 * size_of::<Mutation>(), capacity_bytes: self.mutation_factory.as_ref().expect("pending edit retains original mutation factory").retirement_birth_bytes(mutation), depth: 2, ..Default::default() }); }
        let Some(edit) = self.edit.as_ref() else {
            if self.mutation_factory.is_some() { return Ok(semio_framework_value::RetirementDemand { copy_bytes: size_of::<Arc<dyn semio_framework_value::FactoryRetirement>>(), depth: 1, ..Default::default() }); }
            return self.closing_factory.as_ref().map_or(Ok(Default::default()), |factory| { let mut demand = factory.demands(body)?; demand.depth = demand.depth.checked_add(1).ok_or_else(|| ValueError::literal(semio_framework_value::ValueRefusalKind::DepthLimit, "decoded edit factory depth overflow"))?; Ok(demand) });
        };
        let forwards = edit.forwards.capacity().checked_mul(size_of::<Mutation>()).ok_or_else(|| ValueError::literal(semio_framework_value::ValueRefusalKind::OwnershipLimit, "decoded edit backing overflow"))?;
        if forwards != 0 { return Ok(semio_framework_value::RetirementDemand { copy_bytes: size_of::<Vec<Mutation>>(), release_bytes: forwards, depth: 1, ..Default::default() }); }
        if !edit.inverse.terminal_is_empty() { return Ok(semio_framework_value::RetirementDemand { release_bytes: edit.inverse.next_release_allocation_bytes().map_err(ValueError::from)?, depth: 1, ..Default::default() }); }
        Ok(semio_framework_value::RetirementDemand { copy_bytes: size_of::<Edit<Mutation>>() + size_of::<ArtifactStoreEditRetirement<Mutation>>(), capacity_bytes: size_of::<ArtifactStoreEditRetirement<Mutation>>(), depth: 2, ..Default::default() })
    }
}

impl<Mutation: Send + 'static> ErasedSnapshotRetirement for ArtifactStoreDecodedEditRetirement<Mutation> {
    fn close_step(&mut self, grant: RetainedCloneGrant) -> Result<RetainedCloneStep, ValueError> {
        use semio_framework_value::retained_clone::{RetainedCloneProgress, RetainedCloneStep};
        let empty = RetainedCloneProgress::default();
        if self.terminal_is_empty() { return Ok(RetainedCloneStep::Complete(empty)); }
        if grant.maximum_items == 0 { return Ok(RetainedCloneStep::Progress(empty)); }
        let demand = self.demands(grant.maximum_copy_bytes)?;
        if grant.maximum_depth < demand.depth { return Err(ValueError::literal(semio_framework_value::ValueRefusalKind::DepthLimit, "decoded edit retirement exceeds admitted depth")); }
        if grant.maximum_capacity_bytes < demand.capacity_bytes || grant.maximum_release_bytes < demand.release_bytes || grant.maximum_copy_bytes < demand.copy_bytes { return Ok(RetainedCloneStep::Progress(empty)); }
        if self.active.is_some() { return artifact_retirement_box_close_step(&mut self.active, grant); }
        if self.edit.is_none() {
            if self.mutation_factory.is_some() { let factory: Arc<dyn semio_framework_value::FactoryRetirement> = self.mutation_factory.take().unwrap(); *self.closing_factory = Some(semio_framework_value::FactoryAuthority::new(factory)); return Ok(RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: 1, copied_bytes: demand.copy_bytes, ..empty })); }
            let factory = self.closing_factory.as_mut().expect("nonterminal decoded edit retains original closing capability");
            let child = RetainedCloneGrant { maximum_items: 1, maximum_depth: grant.maximum_depth - 1, ..grant };
            let step = factory.step(child)?;
            let step = semio_framework_value::retained_clone::admit_retained_clone_close(child, step, factory.terminal_is_empty(), "decoded edit original factory capability")?;
            if factory.terminal_is_empty() { drop(self.closing_factory.take()); }
            return Ok(if self.terminal_is_empty() { RetainedCloneStep::Complete(step.progress()) } else { RetainedCloneStep::Progress(step.progress()) });
        }
        let edit = self.edit.as_mut().expect("decoded edit retains its original owner");
        if let Some(mutation) = self.pending_mutation.take().or_else(|| edit.inverse.pop().or_else(|| edit.forwards.pop())) {
            let child = RetainedCloneGrant { maximum_copy_bytes: grant.maximum_copy_bytes - demand.copy_bytes, maximum_depth: grant.maximum_depth - 1, ..grant };
            return match self.mutation_factory.as_ref().unwrap().retire_owned(mutation, child) {
                Ok((owner, mut progress)) => {
                    *self.active = Some(owner);
                    let valid = progress.fits(child) && progress.retained_capacity_bytes == demand.capacity_bytes;
                    progress.copied_bytes = progress.copied_bytes.checked_add(demand.copy_bytes).ok_or_else(|| ValueError::literal(semio_framework_value::ValueRefusalKind::OwnershipLimit, "decoded edit original row transfer receipt overflow"))?;
                    progress.copied_items = progress.copied_items.max(1);
                    if !valid { return Err(ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated, "decoded edit original factory changed admitted constructor receipt").with_retained_progress(progress)); }
                    Ok(RetainedCloneStep::Progress(progress))
                }
                Err((error, mutation)) => {
                    *self.pending_mutation = Some(mutation);
                    let mut progress = error.retained_progress();
                    progress.copied_bytes = progress.copied_bytes.checked_add(demand.copy_bytes).ok_or_else(|| ValueError::literal(semio_framework_value::ValueRefusalKind::OwnershipLimit, "decoded edit refused row transfer receipt overflow"))?;
                    progress.copied_items = progress.copied_items.max(1);
                    Err(error.with_retained_progress(progress))
                }
            };
        }
        if edit.forwards.capacity() != 0 && size_of::<Mutation>() != 0 {
            drop(std::mem::take(&mut edit.forwards));
            return Ok(RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: 1, copied_bytes: demand.copy_bytes, released_bytes: demand.release_bytes, ..empty }));
        }
        if !edit.inverse.terminal_is_empty() {
            let progress = edit.inverse.release_empty_page(grant.maximum_release_bytes).map_err(ValueError::from)?;
            return Ok(RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: usize::from(progress.progressed), released_bytes: progress.released_allocation_bytes, ..empty }));
        }
        let edit = self.edit.take().unwrap();
        let retirement = match ArtifactStoreEditRetirement::new(edit) { Ok(retirement) => retirement, Err(edit) => { *self.edit = Some(edit); return Err(ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated, "decoded edit structural handoff retained native backing")); } };
        *self.active = Some(Box::new(retirement));
        Ok(RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: 1, copied_bytes: demand.copy_bytes, retained_capacity_bytes: demand.capacity_bytes, ..empty }))
    }
    fn terminal_is_empty(&self) -> bool { self.edit.is_none() && self.active.is_none() && self.pending_mutation.is_none() && self.mutation_factory.is_none() && self.closing_factory.is_none() }
    fn next_copy_byte_demand(&self) -> Result<usize, ValueError> { Ok(self.demands(0)?.copy_bytes) }
    fn next_capacity_byte_demand(&self, body: usize) -> Result<usize, ValueError> { Ok(self.demands(body)?.capacity_bytes) }
    fn next_release_byte_demand(&self) -> Result<usize, ValueError> { Ok(self.demands(0)?.release_bytes) }
    fn next_depth_demand(&self) -> Result<usize, ValueError> { Ok(self.demands(0)?.depth) }
    fn next_demand(&self, body: usize) -> Result<semio_framework_value::RetirementDemand, ValueError> { self.demands(body) }
}

impl<Mutation> Drop for ArtifactStoreDecodedEditRetirement<Mutation> {
    fn drop(&mut self) { assert!(std::thread::panicking() || (self.edit.is_none() && self.active.is_none() && self.pending_mutation.is_none() && self.mutation_factory.is_none() && self.closing_factory.is_none()), "decoded edit retirement retains its original pending payload until bounded terminal close"); }
}
