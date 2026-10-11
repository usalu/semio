//! 📬️ The original owners of one one-item Store preparation and the grant-based ladder that closes them.

use crate::{artifact_retirement_admit_owned, artifact_retirement_box_close_step, artifact_retirement_box_demands, artifact_retirement_owned_birth_demands, ArtifactOwnedValueRetirementFactory, ArtifactStoreOneItemLiveAuthority, ArtifactStoreOneItemPrepared, ArtifactStoreOneItemPreparationRequest, Edit, ErasedSnapshotRetirement, SnapshotRead, SnapshotRetirementFactory};
use semio_framework_value::retained_clone::{admit_retained_clone_close, admit_retained_clone_progress, RetainedCloneGrant, RetainedCloneProgress, RetainedCloneStep};
use semio_framework_value::retirement::RetireOwned;
use semio_framework_value::{FactoryAuthority, FactoryRetirement, RetirementDemand, ValueError, ValueRefusalKind};
use std::mem::{size_of, ManuallyDrop};
use std::sync::Arc;

/// 🧳️ Every original owner a one-item preparation holds, each retired in its own fully granted turn.
///
/// 🧩️ A preparation embeds one and keeps only its domain scalars beside it: `base`, `mutation` and `authority` come from the
/// transferred request ([`OneItemOwners::from_request`]); `inverse`, `candidate`, `sealed`, `refused` and `failure` are the
/// staging slots a domain moves its in-flight results through; `prepared` is the sealed publication. `begin_close` then
/// `close_step` (with the four `close_demands` quotes) implement the preparation's whole close contract.
pub struct OneItemOwners<P: 'static, M: 'static> {
    pub base: ManuallyDrop<Option<SnapshotRead<P>>>,
    pub mutation: ManuallyDrop<Option<M>>,
    pub inverse: ManuallyDrop<Option<Vec<M>>>,
    pub candidate: ManuallyDrop<Option<(P, Vec<M>, M)>>,
    pub sealed: ManuallyDrop<Option<(P, Edit<M>)>>,
    pub refused: ManuallyDrop<Option<(Edit<M>, Arc<P>)>>,
    pub failure: ManuallyDrop<Option<ValueError>>,
    pub authority: ManuallyDrop<Option<Arc<ArtifactStoreOneItemLiveAuthority>>>,
    pub prepared: ManuallyDrop<Option<ArtifactStoreOneItemPrepared<P, M>>>,
    mutation_retirement: ManuallyDrop<Option<Arc<dyn ArtifactOwnedValueRetirementFactory<M>>>>,
    snapshot_retirement: ManuallyDrop<Option<Arc<dyn SnapshotRetirementFactory<P>>>>,
    active: ManuallyDrop<Option<Box<dyn ErasedSnapshotRetirement>>>,
    factories: ManuallyDrop<[Option<FactoryAuthority>; 2]>,
    closing: bool,
}

impl<P: 'static, M: 'static> OneItemOwners<P, M> {
    fn all_empty(&self) -> bool {
        self.base.is_none()
            && self.mutation.is_none()
            && self.inverse.is_none()
            && self.candidate.is_none()
            && self.sealed.is_none()
            && self.refused.is_none()
            && self.failure.is_none()
            && self.authority.is_none()
            && self.prepared.is_none()
            && self.mutation_retirement.is_none()
            && self.snapshot_retirement.is_none()
            && self.active.is_none()
            && self.factories.iter().all(Option::is_none)
    }

    /// 🔒️ Starts the close; no further preparation work may be admitted.
    pub fn begin_close(&mut self) {
        self.closing = true;
    }

    pub fn is_closing(&self) -> bool {
        self.closing
    }

    /// 🏁️ True once the close began and every original owner has been retired.
    pub fn terminal_is_empty(&self) -> bool {
        self.closing && self.all_empty()
    }
}

impl<P: 'static, M: 'static> Drop for OneItemOwners<P, M> {
    fn drop(&mut self) {
        assert!(std::thread::panicking() || self.all_empty(), "one-item preparation must retain its original owners until supplied-grant terminal closure");
    }
}

impl<P: RetireOwned + Send + Sync + 'static, M: RetireOwned + Send + 'static> OneItemOwners<P, M> {
    /// 🌱️ Takes ownership of the exact request bundle the Store transferred.
    pub fn from_request(request: ArtifactStoreOneItemPreparationRequest<P, M, M>) -> Self {
        let ArtifactStoreOneItemPreparationRequest { operation: _, generation: _, base_revision: _, lane: _, authority, base, mutation, mutation_retirement, snapshot_retirement } = request;
        Self {
            base: ManuallyDrop::new(Some(base)),
            mutation: ManuallyDrop::new(Some(mutation)),
            inverse: ManuallyDrop::new(None),
            candidate: ManuallyDrop::new(None),
            sealed: ManuallyDrop::new(None),
            refused: ManuallyDrop::new(None),
            failure: ManuallyDrop::new(None),
            authority: ManuallyDrop::new(Some(authority)),
            prepared: ManuallyDrop::new(None),
            mutation_retirement: ManuallyDrop::new(Some(mutation_retirement)),
            snapshot_retirement: ManuallyDrop::new(Some(snapshot_retirement)),
            active: ManuallyDrop::new(None),
            factories: ManuallyDrop::new(Default::default()),
            closing: false,
        }
    }

    /// 🧪️ Owners holding only a mutation and the two installed issuers, detached from any Store request — for conformance
    /// laws of a preparation's cancel and close behaviour, which need no base root or publication authority.
    pub fn detached(mutation: M, mutation_retirement: Arc<dyn ArtifactOwnedValueRetirementFactory<M>>, snapshot_retirement: Arc<dyn SnapshotRetirementFactory<P>>) -> Self {
        Self {
            base: ManuallyDrop::new(None),
            mutation: ManuallyDrop::new(Some(mutation)),
            inverse: ManuallyDrop::new(None),
            candidate: ManuallyDrop::new(None),
            sealed: ManuallyDrop::new(None),
            refused: ManuallyDrop::new(None),
            failure: ManuallyDrop::new(None),
            authority: ManuallyDrop::new(None),
            prepared: ManuallyDrop::new(None),
            mutation_retirement: ManuallyDrop::new(Some(mutation_retirement)),
            snapshot_retirement: ManuallyDrop::new(Some(snapshot_retirement)),
            active: ManuallyDrop::new(None),
            factories: ManuallyDrop::new(Default::default()),
            closing: false,
        }
    }

    /// 📏️ Quotes the single indivisible turn the next close action will spend.
    pub fn close_demands(&self, body: usize) -> Result<RetirementDemand, ValueError> {
        let nested = |mut demand: RetirementDemand| -> Result<RetirementDemand, ValueError> {
            demand.depth = demand.depth.checked_add(1).ok_or_else(|| ValueError::literal(ValueRefusalKind::DepthLimit, "preparation close depth overflow"))?;
            Ok(demand)
        };
        if let Some(active) = self.active.as_ref() {
            return nested(artifact_retirement_box_demands(active, body)?);
        }
        if self.prepared.is_some() {
            let birth = ArtifactStoreOneItemPrepared::<P, M>::retirement_birth_demand();
            return Ok(RetirementDemand { capacity_bytes: birth.capacity_bytes, depth: birth.depth + 1, ..Default::default() });
        }
        if self.mutation.is_some() {
            return nested(artifact_retirement_owned_birth_demands(&*self.mutation)?);
        }
        if self.inverse.is_some() {
            return nested(artifact_retirement_owned_birth_demands(&*self.inverse)?);
        }
        if self.candidate.is_some() {
            return nested(artifact_retirement_owned_birth_demands(&*self.candidate)?);
        }
        if self.sealed.is_some() {
            return nested(artifact_retirement_owned_birth_demands(&*self.sealed)?);
        }
        if self.refused.is_some() {
            return nested(artifact_retirement_owned_birth_demands(&*self.refused)?);
        }
        if self.failure.is_some() {
            return nested(artifact_retirement_owned_birth_demands(&*self.failure)?);
        }
        if self.base.is_some() {
            return nested(artifact_retirement_owned_birth_demands(&*self.base)?);
        }
        if let Some(authority) = self.authority.as_ref() {
            let birth = authority.retirement_birth_demand();
            return Ok(RetirementDemand { capacity_bytes: birth.capacity_bytes, depth: birth.depth + 1, ..Default::default() });
        }
        if self.mutation_retirement.is_some() || self.snapshot_retirement.is_some() {
            return Ok(RetirementDemand { copy_bytes: size_of::<Arc<dyn FactoryRetirement>>(), depth: 1, ..Default::default() });
        }
        self.factories.iter().find_map(Option::as_ref).map_or(Ok(Default::default()), |factory| nested(factory.demands(body)?))
    }

    /// ♻️ Retires exactly one original owner (or one child turn of it) under the supplied grant.
    pub fn close_step(&mut self, grant: RetainedCloneGrant) -> Result<RetainedCloneStep, ValueError> {
        let empty = RetainedCloneProgress::default();
        if !self.closing || grant.maximum_items == 0 {
            return Ok(RetainedCloneStep::Progress(empty));
        }
        if self.terminal_is_empty() {
            return Ok(RetainedCloneStep::Complete(empty));
        }
        let demand = self.close_demands(grant.maximum_copy_bytes)?;
        if grant.maximum_depth < demand.depth.max(1) {
            return Err(ValueError::literal(ValueRefusalKind::DepthLimit, "preparation close exceeds original depth"));
        }
        if grant.maximum_copy_bytes < demand.copy_bytes || grant.maximum_capacity_bytes < demand.capacity_bytes || grant.maximum_release_bytes < demand.release_bytes {
            return Ok(RetainedCloneStep::Progress(empty));
        }
        let child = RetainedCloneGrant { maximum_items: 1, maximum_depth: grant.maximum_depth - 1, ..grant };
        if self.active.is_some() {
            return artifact_retirement_box_close_step(&mut self.active, child).map(|step| RetainedCloneStep::Progress(step.progress()));
        }
        if self.prepared.is_some() {
            if self.mutation_retirement.is_none() || self.snapshot_retirement.is_none() {
                return Err(ValueError::literal(ValueRefusalKind::InvariantViolated, "preparation retains its original installed issuers"));
            }
            let original = self.prepared.take().expect("observed original prepared candidate");
            let mutations = self.mutation_retirement.take().expect("original mutation issuer");
            let snapshots = self.snapshot_retirement.take().expect("original snapshot issuer");
            return match original.admit_retirement(mutations, snapshots, child) {
                Ok((owner, progress)) => {
                    *self.active = Some(owner);
                    admit_retained_clone_progress(child, progress, "original prepared close birth")?;
                    if progress.retained_capacity_bytes != demand.capacity_bytes {
                        return Err(ValueError::literal(ValueRefusalKind::InvariantViolated, "preparation child changed its actual admitted birth"));
                    }
                    Ok(RetainedCloneStep::Progress(progress))
                }
                Err((error, original, mutations, snapshots)) => {
                    *self.prepared = Some(original);
                    *self.mutation_retirement = Some(mutations);
                    *self.snapshot_retirement = Some(snapshots);
                    Err(error)
                }
            };
        }
        if self.mutation.is_some() {
            return artifact_retirement_admit_owned(&mut self.mutation, &mut self.active, child);
        }
        if self.inverse.is_some() {
            return artifact_retirement_admit_owned(&mut self.inverse, &mut self.active, child);
        }
        if self.candidate.is_some() {
            return artifact_retirement_admit_owned(&mut self.candidate, &mut self.active, child);
        }
        if self.sealed.is_some() {
            return artifact_retirement_admit_owned(&mut self.sealed, &mut self.active, child);
        }
        if self.refused.is_some() {
            return artifact_retirement_admit_owned(&mut self.refused, &mut self.active, child);
        }
        if self.failure.is_some() {
            return artifact_retirement_admit_owned(&mut self.failure, &mut self.active, child);
        }
        if self.base.is_some() {
            return artifact_retirement_admit_owned(&mut self.base, &mut self.active, child);
        }
        if let Some(authority) = self.authority.take() {
            return match authority.retire(child) {
                Ok((owner, progress)) => {
                    *self.active = Some(owner);
                    admit_retained_clone_progress(child, progress, "original preparation authority close birth")?;
                    if progress.retained_capacity_bytes != demand.capacity_bytes {
                        return Err(ValueError::literal(ValueRefusalKind::InvariantViolated, "preparation authority changed admitted birth"));
                    }
                    Ok(RetainedCloneStep::Progress(progress))
                }
                Err((error, original)) => {
                    *self.authority = Some(original);
                    Err(error)
                }
            };
        }
        if let Some(factory) = self.mutation_retirement.take() {
            let factory: Arc<dyn FactoryRetirement> = factory;
            self.factories[0] = Some(FactoryAuthority::new(factory));
            return Ok(RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: 1, copied_bytes: demand.copy_bytes, ..empty }));
        }
        if let Some(factory) = self.snapshot_retirement.take() {
            let factory: Arc<dyn FactoryRetirement> = factory;
            self.factories[1] = Some(FactoryAuthority::new(factory));
            return Ok(RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: 1, copied_bytes: demand.copy_bytes, ..empty }));
        }
        let slot = self.factories.iter_mut().find(|slot| slot.is_some()).expect("original preparation factory alias");
        let factory = slot.as_mut().expect("original preparation factory alias");
        let step = factory.step(child)?;
        let step = admit_retained_clone_close(child, step, factory.terminal_is_empty(), "original preparation factory close")?;
        if factory.terminal_is_empty() {
            *slot = None;
        }
        Ok(if self.all_empty() { RetainedCloneStep::Complete(step.progress()) } else { RetainedCloneStep::Progress(step.progress()) })
    }
}
