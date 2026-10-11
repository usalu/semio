//! 📬️ Paged one-item preparation: any plugin whose documents may exceed the one-item retained cap edits them page by page.
//!
//! The document is cloned from a sealed original source through its retained-clone cursor, so no turn owns more than its
//! grant and the footprint counts only the mutation. A [`PagedOneItemEdit`] contributes the bounded in-place edit of the
//! exclusive clone; this module owns the sealed source, the clone, the sealer and the whole grant-based close ladder.

use crate::{
    admit_artifact_batch_digest, artifact_retirement_admit_owned, artifact_retirement_box_close_step, artifact_retirement_box_demands, artifact_retirement_owned_birth_demands, ArtifactCanonicalJsonTree, ArtifactOwnedValueRetirementFactory, ArtifactStoreBatchDigest, ArtifactStoreOneItemCheckpoint,
    ArtifactStoreOneItemFootprint, ArtifactStoreOneItemGrant, ArtifactStoreOneItemLiveAuthority, ArtifactStoreOneItemPreparation, ArtifactStoreOneItemPreparationFactory, ArtifactStoreOneItemPreparationRequest, ArtifactStoreOneItemPreparationStep, ArtifactStoreOneItemPrepared, ArtifactStoreOneItemSealer, Edit,
    ErasedSnapshotRetirement, HistoryLane, Mutation, SnapshotRead, SnapshotRetirementFactory, ARTIFACT_STORE_ONE_ITEM_ID_BYTES, ARTIFACT_STORE_ONE_ITEM_MAXIMUM_BYTES,
};
use crate::os_spr::ActorId;
use semio_framework_value::retained_clone::{RetainedClone, RetainedCloneBirthDemand, RetainedCloneClose, RetainedCloneCursor, RetainedCloneGrant, RetainedCloneProgress, RetainedCloneSource, RetainedCloneStep};
use semio_framework_value::{RetirementDemand, ValueError, ValueRefusalKind};
use std::{marker::PhantomData, mem::{size_of, ManuallyDrop}, sync::Arc};

pub enum PagedOneItemEditStep<M> {
    Progress(RetainedCloneProgress),
    Complete(RetainedCloneProgress, M),
    CompleteRows(RetainedCloneProgress, Vec<M>),
}

#[path = "🧬️mutation-apply/🦀️.rs"]
mod mutation_apply;
pub use mutation_apply::{mutation_apply_preparation_factory, MutationApplyEdit};

/// 🧭️ One bounded structural edit of the exclusive page-by-page clone: one semantic unit per turn, `Complete` carries the exact inverse.
pub trait PagedOneItemEdit<S, M>: Default + Send + 'static {
    const PREFIX: &'static str;
    fn recognizes(mutation: &M) -> bool;
    fn preflight(mutation: &M) -> Result<usize, String>;
    fn advance(&mut self, post: &mut S, mutation: &M, grant: RetainedCloneGrant) -> Result<PagedOneItemEditStep<M>, ValueError>;
    /// 📏️ Receives the exact retained capacity of the completed snapshot clone before the first edit turn.
    fn cloned(&mut self, _capacity_bytes: usize) {}
    fn begin_close(&mut self) {}
    /// ♻️ Quotes the next close turn of the owners this edit holds between turns.
    fn close_demands(&self, _body: usize) -> Result<RetirementDemand, ValueError> {
        Ok(Default::default())
    }
    fn close_step(&mut self, _grant: RetainedCloneGrant) -> Result<RetainedCloneStep, ValueError> {
        Ok(RetainedCloneStep::Complete(Default::default()))
    }
    fn terminal_is_empty(&self) -> bool {
        true
    }
}

#[derive(semio_framework_value::FactoryPayloadRetirement)]
pub struct PagedOneItemPreparationFactory<S, M, E>(PhantomData<fn() -> (S, M, E)>);

impl<S, M, E> Default for PagedOneItemPreparationFactory<S, M, E> {
    fn default() -> Self {
        Self(PhantomData)
    }
}

/// 🌱️ Prices the concrete structural factory; it owns no original child constructor.
pub fn paged_one_item_factory_birth_bytes<S, M, E>() -> usize
where
    PagedOneItemPreparationFactory<S, M, E>: semio_framework_value::FactoryPayloadRetirement,
{
    semio_framework_value::factory_constructor_birth_bytes::<PagedOneItemPreparationFactory<S, M, E>>(0)
}

fn fault(prefix: &'static str, owner: &'static str) -> ValueError {
    ValueError::new(ValueRefusalKind::InvariantViolated, format!("{prefix}-{owner}-owner"))
}

impl<S, M, E> ArtifactStoreOneItemPreparationFactory<S, M> for PagedOneItemPreparationFactory<S, M, E>
where
    S: RetainedClone,
    M: ArtifactCanonicalJsonTree + semio_framework_value::retirement::RetireOwned + Mutation<S> + Send + Sync + 'static,
    E: PagedOneItemEdit<S, M>,
{
    fn begin_batch_digest(&self, edit: &mut Option<Box<Edit<M>>>, grant: RetainedCloneGrant) -> Result<Option<(Box<dyn ArtifactStoreBatchDigest<M>>, RetainedCloneProgress)>, ValueError> {
        admit_artifact_batch_digest(edit, grant)
    }

    fn preflight(&self, mutation: &M, lane: HistoryLane) -> Result<ArtifactStoreOneItemFootprint, String> {
        if lane != HistoryLane::Document || !E::recognizes(mutation) {
            return Err(format!("{}-admission", E::PREFIX));
        }
        let retained_bytes = E::preflight(mutation)?;
        if retained_bytes > ARTIFACT_STORE_ONE_ITEM_MAXIMUM_BYTES {
            return Err(format!("{}-payload", E::PREFIX));
        }
        Ok(ArtifactStoreOneItemFootprint::for_leaf::<S, M>(mutation, retained_bytes.max(1)))
    }

    fn begin_demand(&self, mutation: &M, lane: HistoryLane) -> Result<RetainedCloneBirthDemand, ValueError> {
        if lane != HistoryLane::Document || !E::recognizes(mutation) {
            return Err(ValueError::literal(ValueRefusalKind::UnsupportedOwner, "structural mutation has no original preparation issuer"));
        }
        if !S::controlled_retirement_supported() || !M::controlled_retirement_supported() {
            return Err(ValueError::literal(ValueRefusalKind::UnsupportedOwner, "structural clone originals require controlled typed retirement"));
        }
        Ok(RetainedCloneBirthDemand { capacity_bytes: size_of::<PagedOneItemPreparation<S, M, E>>(), depth: 1 })
    }

    fn begin(
        &self,
        request: ArtifactStoreOneItemPreparationRequest<S, M, M>,
        grant: ArtifactStoreOneItemGrant,
    ) -> Result<(Box<dyn ArtifactStoreOneItemPreparation<S, M>>, RetainedCloneProgress), (ValueError, ArtifactStoreOneItemPreparationRequest<S, M, M>)> {
        let demand = match self.begin_demand(&request.mutation, request.lane) {
            Ok(demand) => demand,
            Err(error) => return Err((error, request)),
        };
        let receipt = match demand.admit(grant.retained_grant()) {
            Ok(receipt) => receipt,
            Err(error) => return Err((error, request)),
        };
        let admitted = request.operation == request.authority.operation() && request.generation == request.authority.generation() && request.base_revision == request.authority.base_revision() && request.authority.actor().len() <= ARTIFACT_STORE_ONE_ITEM_ID_BYTES;
        if !admitted {
            return Err((ValueError::literal(ValueRefusalKind::InvariantViolated, "structural request differs from its original authority"), request));
        }
        let ArtifactStoreOneItemPreparationRequest { authority, base, mutation, mutation_retirement, snapshot_retirement, .. } = request;
        Ok((
            Box::new(PagedOneItemPreparation::<S, M, E> {
                base: ManuallyDrop::new(Some(base)),
                source: ManuallyDrop::new(None),
                cursor: ManuallyDrop::new(None),
                post: ManuallyDrop::new(None),
                edit: E::default(),
                pending_inverse: ManuallyDrop::new(None),
                clone_capacity_bytes: 0,
                mutation: ManuallyDrop::new(Some(mutation)),
                authority: ManuallyDrop::new(Some(authority)),
                mutation_retirement: ManuallyDrop::new(Some(mutation_retirement)),
                snapshot_retirement: ManuallyDrop::new(Some(snapshot_retirement)),
                factory_close: ManuallyDrop::new(Default::default()),
                sealer: ManuallyDrop::new(None),
                external_retirement: ManuallyDrop::new(None),
                retirement: RetainedCloneClose::default(),
                checkpoint: Default::default(),
                seal_base_checkpoint: None,
                phase: Phase::Source,
                cancelled: false,
                closing: false,
            }),
            receipt,
        ))
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum Phase {
    Source,
    Cursor,
    Clone,
    CloseClone,
    Edit,
    Seal,
    Sealing,
}

struct PagedOneItemPreparation<S: RetainedClone, M, E: PagedOneItemEdit<S, M>> {
    base: ManuallyDrop<Option<SnapshotRead<S>>>,
    source: ManuallyDrop<Option<RetainedCloneSource<S, SnapshotRead<S>>>>,
    cursor: ManuallyDrop<Option<S::Cursor>>,
    post: ManuallyDrop<Option<S>>,
    edit: E,
    pending_inverse: ManuallyDrop<Option<Vec<M>>>,
    clone_capacity_bytes: usize,
    mutation: ManuallyDrop<Option<M>>,
    authority: ManuallyDrop<Option<Arc<ArtifactStoreOneItemLiveAuthority>>>,
    mutation_retirement: ManuallyDrop<Option<Arc<dyn ArtifactOwnedValueRetirementFactory<M>>>>,
    snapshot_retirement: ManuallyDrop<Option<Arc<dyn SnapshotRetirementFactory<S>>>>,
    factory_close: ManuallyDrop<[Option<semio_framework_value::FactoryAuthority>; 2]>,
    sealer: ManuallyDrop<Option<ArtifactStoreOneItemSealer<S, M>>>,
    external_retirement: ManuallyDrop<Option<Box<dyn ErasedSnapshotRetirement>>>,
    retirement: RetainedCloneClose,
    checkpoint: ArtifactStoreOneItemCheckpoint,
    seal_base_checkpoint: Option<ArtifactStoreOneItemCheckpoint>,
    phase: Phase,
    cancelled: bool,
    closing: bool,
}

impl<S, M, E> PagedOneItemPreparation<S, M, E>
where
    S: RetainedClone,
    M: ArtifactCanonicalJsonTree + semio_framework_value::retirement::RetireOwned + Send + Sync + 'static,
    E: PagedOneItemEdit<S, M>,
{
    fn record(&mut self, progress: RetainedCloneProgress) -> ArtifactStoreOneItemPreparationStep {
        if progress == RetainedCloneProgress::default() {
            return ArtifactStoreOneItemPreparationStep::Blocked;
        }
        let bytes = progress.copied_bytes.saturating_add(progress.retained_capacity_bytes).saturating_add(progress.released_bytes);
        self.checkpoint.cursor = self.checkpoint.cursor.saturating_add(1);
        self.checkpoint.completed_items = self.checkpoint.completed_items.saturating_add(u32::try_from(progress.copied_items).unwrap_or(u32::MAX));
        self.checkpoint.completed_bytes = self.checkpoint.completed_bytes.saturating_add(bytes as u64);
        ArtifactStoreOneItemPreparationStep::Progress(self.checkpoint, progress)
    }

    fn advance_sealer(&mut self, grant: ArtifactStoreOneItemGrant) -> Result<ArtifactStoreOneItemPreparationStep, ValueError> {
        let prefix = E::PREFIX;
        let sealer = self.sealer.as_mut().ok_or_else(|| fault(prefix, "sealer"))?;
        let step = sealer.advance(grant)?;
        let (checkpoint, progress) = match step {
            ArtifactStoreOneItemPreparationStep::Progress(checkpoint, progress) | ArtifactStoreOneItemPreparationStep::Prepared(checkpoint, progress) => (checkpoint, progress),
            ArtifactStoreOneItemPreparationStep::Blocked => return Ok(ArtifactStoreOneItemPreparationStep::Blocked),
        };
        let base = self.seal_base_checkpoint.ok_or_else(|| fault(prefix, "seal-checkpoint"))?;
        self.checkpoint = ArtifactStoreOneItemCheckpoint {
            cursor: base.cursor.saturating_add(checkpoint.cursor),
            completed_items: base.completed_items.saturating_add(checkpoint.completed_items),
            completed_bytes: base.completed_bytes.saturating_add(checkpoint.completed_bytes),
            digest: checkpoint.digest,
        };
        if matches!(step, ArtifactStoreOneItemPreparationStep::Prepared(..)) {
            self.checkpoint.digest = sealer.prepared().ok_or_else(|| fault(prefix, "prepared"))?.edit_digest();
            return Ok(ArtifactStoreOneItemPreparationStep::Prepared(self.checkpoint, progress));
        }
        Ok(ArtifactStoreOneItemPreparationStep::Progress(self.checkpoint, progress))
    }

    fn finish_edit(&mut self, clone_grant: RetainedCloneGrant, progress: RetainedCloneProgress, inverse: Vec<M>) -> Result<ArtifactStoreOneItemPreparationStep, ValueError> {
        let progress = semio_framework_value::retained_clone::admit_retained_clone_progress(clone_grant, progress, "structural edit")?;
        *self.pending_inverse = Some(inverse);
        self.phase = Phase::Seal;
        Ok(self.record(RetainedCloneProgress { copied_items: progress.copied_items.max(1), ..progress }))
    }

    fn begin_seal(&mut self, grant: ArtifactStoreOneItemGrant) -> Result<ArtifactStoreOneItemPreparationStep, ValueError> {
        let prefix = E::PREFIX;
        let retained = grant.retained_grant();
        let constructor = ArtifactStoreOneItemSealer::<S, M>::constructor_demand();
        let inverse_bytes = self.pending_inverse.as_ref().map_or(0, |rows| rows.capacity().saturating_mul(size_of::<M>()));
        let wrapped = size_of::<M>().checked_add(inverse_bytes).and_then(|bytes| bytes.checked_add(size_of::<S>())).and_then(|bytes| bytes.checked_add(32)).and_then(|bytes| bytes.checked_add(constructor.capacity_bytes)).ok_or_else(|| ValueError::literal(ValueRefusalKind::OwnershipLimit, "structural seal layout overflow"))?;
        if retained.maximum_items == 0 || retained.maximum_depth < constructor.depth || retained.maximum_capacity_bytes < wrapped || retained.maximum_copy_bytes < ArtifactStoreOneItemSealer::<S, M>::constructor_copy_byte_demand() {
            return Ok(ArtifactStoreOneItemPreparationStep::Blocked);
        }
        let post = self.post.take().ok_or_else(|| fault(prefix, "post"))?;
        let inverse = self.pending_inverse.take().ok_or_else(|| fault(prefix, "inverse"))?;
        let mutation = self.mutation.take().ok_or_else(|| fault(prefix, "mutation"))?;
        let authority = self.authority.take().ok_or_else(|| fault(prefix, "authority"))?;
        let mut edit = authority.next_edit(mutation, inverse);
        edit.actor = Some(authority.actor().into());
        edit.mutation_meta[0].author_id = Some(ActorId(authority.actor().into()));
        let mutation_retirement = self.mutation_retirement.take().ok_or_else(|| fault(prefix, "mutation-issuer"))?;
        let snapshot_retirement = self.snapshot_retirement.take().ok_or_else(|| fault(prefix, "snapshot-issuer"))?;
        match authority.begin_one_item_seal(edit, Arc::new(post), mutation_retirement, snapshot_retirement, retained) {
            Ok((sealer, progress)) => {
                *self.sealer = Some(sealer);
                self.seal_base_checkpoint = Some(self.checkpoint);
                self.phase = Phase::Sealing;
                Ok(self.record(RetainedCloneProgress { retained_capacity_bytes: progress.retained_capacity_bytes.saturating_add(size_of::<M>()).saturating_add(inverse_bytes).saturating_add(2 * size_of::<usize>() + size_of::<S>()), ..progress }))
            }
            Err((error, authority, _edit, _post, mutation_retirement, snapshot_retirement)) => {
                *self.authority = Some(authority);
                *self.mutation_retirement = Some(mutation_retirement);
                *self.snapshot_retirement = Some(snapshot_retirement);
                Err(error)
            }
        }
    }
}

impl<S, M, E> ArtifactStoreOneItemPreparation<S, M> for PagedOneItemPreparation<S, M, E>
where
    S: RetainedClone,
    M: ArtifactCanonicalJsonTree + semio_framework_value::retirement::RetireOwned + Send + Sync + 'static,
    E: PagedOneItemEdit<S, M>,
{
    fn advance(&mut self, grant: ArtifactStoreOneItemGrant) -> Result<ArtifactStoreOneItemPreparationStep, ValueError> {
        let prefix = E::PREFIX;
        if !grant.permits_one() || self.cancelled || self.closing {
            return Ok(ArtifactStoreOneItemPreparationStep::Blocked);
        }
        let clone_grant = RetainedCloneGrant { maximum_items: 1, ..grant.retained_grant() };
        match self.phase {
            Phase::Source => {
                let demand = RetainedCloneSource::<S>::borrowed_constructor_demand::<SnapshotRead<S>>();
                if demand.admit(clone_grant).is_err() || clone_grant.maximum_copy_bytes < RetainedCloneSource::<S>::borrowed_constructor_copy_bytes::<SnapshotRead<S>>() {
                    return Ok(ArtifactStoreOneItemPreparationStep::Blocked);
                }
                let base = self.base.take().ok_or_else(|| fault(prefix, "base"))?;
                match base.admit_retained_clone_source(clone_grant) {
                    Ok((source, progress)) => {
                        *self.source = Some(source);
                        self.phase = Phase::Cursor;
                        Ok(self.record(progress))
                    }
                    Err((error, base)) => {
                        *self.base = Some(base);
                        Err(error)
                    }
                }
            }
            Phase::Cursor => {
                *self.cursor = Some(S::retained_clone_cursor());
                self.phase = Phase::Clone;
                Ok(self.record(RetainedCloneProgress { copied_items: 1, ..Default::default() }))
            }
            Phase::Clone => {
                let cursor = self.cursor.as_mut().ok_or_else(|| fault(prefix, "cursor"))?;
                let source = self.source.as_ref().ok_or_else(|| fault(prefix, "source"))?;
                let step = cursor.advance(source.borrow(), clone_grant)?;
                let progress = semio_framework_value::retained_clone::admit_retained_clone_progress(clone_grant, step.progress(), "structural snapshot clone")?;
                self.clone_capacity_bytes = self.clone_capacity_bytes.saturating_add(progress.retained_capacity_bytes);
                if matches!(step, RetainedCloneStep::Complete(_)) {
                    *self.post = Some(cursor.take().ok_or_else(|| fault(prefix, "clone-result"))?);
                    cursor.begin_close();
                    self.phase = Phase::CloseClone;
                }
                Ok(self.record(progress))
            }
            Phase::CloseClone => {
                let cursor = self.cursor.as_mut().ok_or_else(|| fault(prefix, "cursor"))?;
                if cursor.terminal_is_empty() {
                    *self.cursor = None;
                    self.edit.cloned(self.clone_capacity_bytes);
                    self.phase = Phase::Edit;
                    return Ok(self.record(RetainedCloneProgress { copied_items: 1, ..Default::default() }));
                }
                let step = cursor.close_step(clone_grant)?;
                let step = semio_framework_value::retained_clone::admit_retained_clone_close(clone_grant, step, cursor.terminal_is_empty(), "structural snapshot clone close")?;
                Ok(self.record(step.progress()))
            }
            Phase::Edit => {
                let post = self.post.as_mut().ok_or_else(|| fault(prefix, "post"))?;
                let mutation = self.mutation.as_ref().ok_or_else(|| fault(prefix, "mutation"))?;
                match self.edit.advance(post, mutation, clone_grant)? {
                    PagedOneItemEditStep::Progress(progress) => {
                        let progress = semio_framework_value::retained_clone::admit_retained_clone_progress(clone_grant, progress, "structural edit")?;
                        Ok(self.record(progress))
                    }
                    PagedOneItemEditStep::Complete(progress, inverse) => self.finish_edit(clone_grant, progress, vec![inverse]),
                    PagedOneItemEditStep::CompleteRows(progress, inverse) => self.finish_edit(clone_grant, progress, inverse),
                }
            }
            Phase::Seal => self.begin_seal(grant),
            Phase::Sealing => self.advance_sealer(grant),
        }
    }

    fn checkpoint(&self) -> ArtifactStoreOneItemCheckpoint {
        self.checkpoint
    }

    fn prepared(&self) -> Option<&ArtifactStoreOneItemPrepared<S, M>> {
        self.sealer.as_ref().and_then(ArtifactStoreOneItemSealer::prepared)
    }

    fn take_prepared(&mut self) -> Option<ArtifactStoreOneItemPrepared<S, M>> {
        self.sealer.as_mut().and_then(ArtifactStoreOneItemSealer::take_prepared)
    }

    fn cancel(&mut self) {
        self.cancelled = true;
        if let Some(sealer) = self.sealer.as_mut() {
            sealer.cancel();
        }
    }

    fn begin_close(&mut self) {
        self.closing = true;
        self.edit.begin_close();
        if let Some(sealer) = self.sealer.as_mut() {
            sealer.begin_close();
        }
        if let Some(cursor) = self.cursor.as_mut() {
            cursor.begin_close();
        }
    }

    fn close_step(&mut self, grant: ArtifactStoreOneItemGrant) -> Result<RetainedCloneStep, ValueError> {
        if self.terminal() {
            return Ok(RetainedCloneStep::Complete(Default::default()));
        }
        if !self.closing || !grant.permits_one() {
            return Ok(RetainedCloneStep::Progress(Default::default()));
        }
        let funded = grant.retained_grant();
        let demand = self.close_demands(funded.maximum_copy_bytes)?;
        if funded.maximum_depth < demand.depth {
            return Err(ValueError::literal(ValueRefusalKind::DepthLimit, "structural preparation exceeds original nested depth"));
        }
        if funded.maximum_copy_bytes < demand.copy_bytes || funded.maximum_capacity_bytes < demand.capacity_bytes || funded.maximum_release_bytes < demand.release_bytes {
            return Ok(RetainedCloneStep::Progress(Default::default()));
        }
        let child = RetainedCloneGrant { maximum_items: 1, maximum_depth: funded.maximum_depth.saturating_sub(1), ..funded };
        let unit = |copied_bytes: usize| RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: 1, copied_bytes, ..Default::default() });
        if self.external_retirement.is_some() {
            return artifact_retirement_box_close_step(&mut self.external_retirement, child).map(|step| RetainedCloneStep::Progress(step.progress()));
        }
        if !self.edit.terminal_is_empty() {
            return self.edit.close_step(child).map(|step| RetainedCloneStep::Progress(step.progress()));
        }
        if let Some(sealer) = self.sealer.as_mut() {
            if !sealer.terminal_is_empty() {
                return sealer.close_step(child).map(|step| RetainedCloneStep::Progress(step.progress()));
            }
            self.sealer.take();
            return Ok(unit(0));
        }
        if let Some(cursor) = self.cursor.as_mut() {
            if !cursor.terminal_is_empty() {
                return cursor.close_step(child).map(|step| RetainedCloneStep::Progress(step.progress()));
            }
            *self.cursor = None;
            return Ok(unit(0));
        }
        if !self.retirement.is_empty() {
            return self.retirement.step_granted(child).map(|step| RetainedCloneStep::Progress(step.progress()));
        }
        if let Some(step) = self.retirement.begin_granted(&mut self.post, child)? {
            return Ok(step);
        }
        if let Some(step) = self.retirement.begin_granted(&mut self.pending_inverse, child)? {
            return Ok(step);
        }
        if let Some(source) = self.source.as_mut() {
            if !source.terminal_is_empty() {
                return source.close_step(child).map(|step| RetainedCloneStep::Progress(step.progress()));
            }
            *self.source = None;
            return Ok(unit(0));
        }
        if let Some(original) = self.mutation.take() {
            return match self.mutation_retirement.as_ref().ok_or_else(|| fault(E::PREFIX, "mutation-issuer"))?.retire_owned(original, child) {
                Ok((owner, progress)) => {
                    *self.external_retirement = Some(owner);
                    Ok(RetainedCloneStep::Progress(progress))
                }
                Err((error, original)) => {
                    *self.mutation = Some(original);
                    Err(error)
                }
            };
        }
        if self.base.is_some() {
            return artifact_retirement_admit_owned(&mut self.base, &mut self.external_retirement, child).map(|step| RetainedCloneStep::Progress(step.progress()));
        }
        if let Some(original) = self.authority.take() {
            return match original.retire(child) {
                Ok((owner, progress)) => {
                    *self.external_retirement = Some(owner);
                    Ok(RetainedCloneStep::Progress(progress))
                }
                Err((error, original)) => {
                    *self.authority = Some(original);
                    Err(error)
                }
            };
        }
        if let Some(original) = self.mutation_retirement.take() {
            self.factory_close[0] = Some(semio_framework_value::FactoryAuthority::new(original));
            return Ok(unit(demand.copy_bytes));
        }
        if let Some(original) = self.snapshot_retirement.take() {
            self.factory_close[1] = Some(semio_framework_value::FactoryAuthority::new(original));
            return Ok(unit(demand.copy_bytes));
        }
        if let Some(slot) = self.factory_close.iter_mut().find(|slot| slot.is_some()) {
            let owner = slot.as_mut().ok_or_else(|| fault(E::PREFIX, "factory"))?;
            if !owner.terminal_is_empty() {
                return owner.step(child).map(|step| RetainedCloneStep::Progress(step.progress()));
            }
            *slot = None;
            return Ok(unit(0));
        }
        Ok(RetainedCloneStep::Complete(Default::default()))
    }

    fn next_close_copy_byte_demand(&self) -> Result<usize, ValueError> {
        Ok(self.close_demands(0)?.copy_bytes)
    }

    fn next_close_capacity_byte_demand(&self, body: usize) -> Result<usize, ValueError> {
        Ok(self.close_demands(body)?.capacity_bytes)
    }

    fn next_close_release_byte_demand(&self) -> Result<usize, ValueError> {
        Ok(self.close_demands(0)?.release_bytes)
    }

    fn next_close_depth_demand(&self) -> Result<usize, ValueError> {
        Ok(self.close_demands(0)?.depth)
    }

    fn terminal_is_empty(&self) -> bool {
        self.terminal()
    }
}

impl<S, M, E> PagedOneItemPreparation<S, M, E>
where
    S: RetainedClone,
    M: semio_framework_value::retirement::RetireOwned + Send + 'static,
    E: PagedOneItemEdit<S, M>,
{
    fn terminal(&self) -> bool {
        self.closing
            && self.edit.terminal_is_empty()
            && self.base.is_none()
            && self.source.is_none()
            && self.cursor.is_none()
            && self.post.is_none()
            && self.pending_inverse.is_none()
            && self.mutation.is_none()
            && self.authority.is_none()
            && self.mutation_retirement.is_none()
            && self.snapshot_retirement.is_none()
            && self.sealer.is_none()
            && self.external_retirement.is_none()
            && self.retirement.is_empty()
            && self.factory_close.iter().all(Option::is_none)
    }

    fn close_demands(&self, body: usize) -> Result<RetirementDemand, ValueError> {
        let nested = |mut value: RetirementDemand| -> Result<RetirementDemand, ValueError> {
            value.depth = value.depth.checked_add(1).ok_or_else(|| ValueError::literal(ValueRefusalKind::DepthLimit, "structural preparation depth overflow"))?;
            Ok(value)
        };
        let child = |demand: RetirementDemand| nested(demand);
        if let Some(owner) = self.external_retirement.as_ref() {
            return child(artifact_retirement_box_demands(owner, body)?);
        }
        if !self.edit.terminal_is_empty() {
            return child(self.edit.close_demands(body)?);
        }
        if let Some(owner) = self.sealer.as_ref() {
            return if owner.terminal_is_empty() { Ok(RetirementDemand { depth: 1, ..Default::default() }) } else { child(owner.retirement_demands(body)?) };
        }
        if let Some(owner) = self.cursor.as_ref() {
            return if owner.terminal_is_empty() {
                Ok(RetirementDemand { depth: 1, ..Default::default() })
            } else {
                child(RetirementDemand { copy_bytes: owner.next_close_copy_byte_demand()?, capacity_bytes: owner.next_close_capacity_byte_demand(body)?, release_bytes: owner.next_close_release_byte_demand()?, depth: owner.next_close_depth_demand()? })
            };
        }
        if !self.retirement.is_empty() {
            return child(RetirementDemand { copy_bytes: self.retirement.next_copy_byte_demand()?, capacity_bytes: self.retirement.next_capacity_byte_demand(body)?, release_bytes: self.retirement.next_release_byte_demand()?, depth: self.retirement.next_depth_demand()? });
        }
        if self.post.is_some() {
            return child(RetirementDemand { capacity_bytes: self.retirement.next_owner_capacity_byte_demand::<S>(true, body)?, depth: 1, ..Default::default() });
        }
        if self.pending_inverse.is_some() {
            return child(RetirementDemand { capacity_bytes: self.retirement.next_owner_capacity_byte_demand::<Vec<M>>(true, body)?, depth: 1, ..Default::default() });
        }
        if let Some(owner) = self.source.as_ref() {
            return if owner.terminal_is_empty() {
                Ok(RetirementDemand { depth: 1, ..Default::default() })
            } else {
                child(RetirementDemand { copy_bytes: owner.next_close_copy_byte_demand()?, capacity_bytes: owner.next_close_capacity_byte_demand(body)?, release_bytes: owner.next_close_release_byte_demand()?, depth: owner.next_close_depth_demand()? })
            };
        }
        if let Some(original) = self.mutation.as_ref() {
            let birth = self.mutation_retirement.as_ref().ok_or_else(|| fault("structural", "mutation-issuer"))?.retirement_birth_bytes(original);
            return child(RetirementDemand { capacity_bytes: birth, depth: 1, ..Default::default() });
        }
        if self.base.is_some() {
            return child(artifact_retirement_owned_birth_demands(&self.base)?);
        }
        if let Some(original) = self.authority.as_ref() {
            let birth = original.retirement_birth_demand();
            return child(RetirementDemand { capacity_bytes: birth.capacity_bytes, depth: birth.depth, ..Default::default() });
        }
        if self.mutation_retirement.is_some() || self.snapshot_retirement.is_some() {
            return Ok(RetirementDemand { copy_bytes: size_of::<Arc<dyn semio_framework_value::FactoryRetirement>>(), depth: 1, ..Default::default() });
        }
        if let Some(original) = self.factory_close.iter().find_map(Option::as_ref) {
            return if original.terminal_is_empty() { Ok(RetirementDemand { depth: 1, ..Default::default() }) } else { child(original.demands(body)?) };
        }
        Ok(Default::default())
    }
}

impl<S: RetainedClone, M, E: PagedOneItemEdit<S, M>> Drop for PagedOneItemPreparation<S, M, E> {
    fn drop(&mut self) {
        assert!(
            std::thread::panicking()
                || (self.edit.terminal_is_empty()
                    && self.base.is_none()
                    && self.source.is_none()
                    && self.cursor.is_none()
                    && self.post.is_none()
                    && self.pending_inverse.is_none()
                    && self.mutation.is_none()
                    && self.authority.is_none()
                    && self.mutation_retirement.is_none()
                    && self.snapshot_retirement.is_none()
                    && self.sealer.is_none()
                    && self.external_retirement.is_none()
                    && self.retirement.is_empty()
                    && self.factory_close.iter().all(Option::is_none)),
            "structural preparation dropped with live owners"
        );
    }
}
