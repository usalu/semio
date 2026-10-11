//! ✏️ Field-set apply edit for small config records (window configs and document config lanes): clones the mutation payload under grants, exchanges it into the post record by move and keeps the displaced content as the inverse row, all through original custody.
use super::{RetainedCloneEdit,RetainedCloneEditCursor,RetainedCloneEditStep,RetainedClonePreparationFactory,custody::RetainedEditCustody};
use crate::os_store::{ARTIFACT_STORE_ONE_ITEM_MAXIMUM_BYTES,ArtifactCanonicalJsonTree,ArtifactOwnedValueRetirementFactory,ArtifactStoreOneItemFootprint,ArtifactStoreOneItemPreparationFactory,HistoryLane,SnapshotRetirementFactory};
use semio_framework_value::{RetirementDemand,ValueError,ValueRefusalKind,retained_clone::{RetainedClone,RetainedCloneBirthDemand,RetainedCloneCursor,RetainedCloneGrant,RetainedCloneProgress,RetainedCloneRef,RetainedCloneStep,admit_retained_clone_close,admit_retained_clone_progress}};
use std::{marker::PhantomData,mem::{ManuallyDrop,size_of},sync::Arc};

type CloneCursor<M> = <M as RetainedClone>::Cursor;

/// 📝️ Declares one field-set mutation of a config record whose payload may own heap content.
///
/// `exchange` runs after the store cloned the base into the post record, so the post still equals the base: it MUST move the owned payload into the post record and return the row that moves the displaced content back. It MUST NOT allocate or release heap memory; the edit accounts it as one fixed-size move.
pub trait ConfigApplyMutation<S>: RetainedClone {
    fn exchange(self, post: &mut S) -> Result<Self, (ValueError, Self)>;
    fn admissible(&self) -> bool { true }
    fn admits_lane(&self, lane: HistoryLane) -> bool { lane == HistoryLane::Document }
    fn payload_bytes(&self) -> usize { 0 }
}

/// 🧰️ Generic field-set edit installed through `WindowConfigOwner::build_retained_edit` or `config_apply_preparation_factory`.
#[derive(semio_framework_value::FactoryPayloadRetirement)]
pub struct ConfigApplyEdit<S: 'static, M: 'static>(PhantomData<fn() -> (S, M)>);
impl<S: 'static, M: 'static> ConfigApplyEdit<S, M> { pub fn new() -> Self { Self(PhantomData) } }
impl<S: 'static, M: 'static> Default for ConfigApplyEdit<S, M> { fn default() -> Self { Self::new() } }

#[derive(Clone, Copy, PartialEq, Eq)]
enum ApplyStage { Clone, Take, CloseClone, Reserve, Exchange, Done }

/// 🪜️ Bounded turns: clone the mutation payload, take it into custody, close the clone cursor, reserve the inverse row, exchange payload and displaced content.
pub struct ConfigApplyCursor<S, M: ConfigApplyMutation<S>> {
    stage: ApplyStage,
    cancelled: bool,
    clone: ManuallyDrop<Option<CloneCursor<M>>>,
    custody: RetainedEditCustody<(Option<M>, Option<Vec<M>>)>,
    marker: PhantomData<fn() -> S>,
}

impl<S, M> RetainedCloneEdit<S, M> for ConfigApplyEdit<S, M>
where S: RetainedClone, M: ConfigApplyMutation<S> {
    type Cursor = ConfigApplyCursor<S, M>;
    fn preflight(&self, mutation: &M, lane: HistoryLane) -> Result<ArtifactStoreOneItemFootprint, String> {
        if !mutation.admits_lane(lane) || !mutation.admissible() { return Err("config apply mutation has invalid lane or record authority".into()); }
        let retained_bytes = mutation.payload_bytes().checked_mul(4).and_then(|bytes| bytes.checked_add(CONFIG_EDIT_RETAINED_BASELINE_BYTES)).filter(|bytes| *bytes <= ARTIFACT_STORE_ONE_ITEM_MAXIMUM_BYTES).ok_or("config apply payload exceeds the one-item retained footprint")?;
        Ok(ArtifactStoreOneItemFootprint { work_items: 2, retained_bytes })
    }
    fn begin_demand(&self) -> RetainedCloneBirthDemand { RetainedCloneBirthDemand { capacity_bytes: 0, depth: 1 } }
    fn snapshot_cursor_birth_demand(&self) -> RetainedCloneBirthDemand { RetainedCloneBirthDemand { capacity_bytes: 0, depth: 1 } }
    fn begin(&self, grant: RetainedCloneGrant) -> Result<(Self::Cursor, RetainedCloneProgress), ValueError> {
        let mut receipt = self.begin_demand().admit(grant)?;
        receipt.copied_bytes = size_of::<Self::Cursor>();
        if !receipt.fits(grant) { return Err(ValueError::literal(ValueRefusalKind::WorkLimit, "config apply cursor transfer exceeds grant")); }
        Ok((ConfigApplyCursor { stage: ApplyStage::Clone, cancelled: false, clone: ManuallyDrop::new(Some(M::retained_clone_cursor())), custody: RetainedEditCustody::new((None, None)), marker: PhantomData }, receipt))
    }
}

impl<S, M: ConfigApplyMutation<S>> ConfigApplyCursor<S, M> {
    fn clone_close_demand(cursor: &CloneCursor<M>, body: usize) -> Result<RetirementDemand, ValueError> {
        if cursor.terminal_is_empty() { return Ok(RetirementDemand { copy_bytes: size_of::<CloneCursor<M>>(), depth: 1, ..Default::default() }); }
        Ok(RetirementDemand {
            copy_bytes: cursor.next_close_copy_byte_demand()?,
            capacity_bytes: cursor.next_close_capacity_byte_demand(body)?,
            release_bytes: cursor.next_close_release_byte_demand()?,
            depth: cursor.next_close_depth_demand()?.checked_add(1).ok_or_else(|| ValueError::literal(ValueRefusalKind::DepthLimit, "config apply clone close depth overflow"))?,
            ..Default::default()
        })
    }
    fn close_demand(&self, body: usize) -> Result<RetirementDemand, ValueError> {
        match self.clone.as_ref() {
            Some(cursor) => Self::clone_close_demand(cursor, body),
            None => self.custody.demands(body),
        }
    }
    fn close_clone(&mut self, grant: RetainedCloneGrant) -> Result<RetainedCloneProgress, ValueError> {
        let cursor = self.clone.as_mut().ok_or_else(|| ValueError::literal(ValueRefusalKind::InvariantViolated, "config apply lost its mutation clone cursor"))?;
        let demand = Self::clone_close_demand(cursor, grant.maximum_copy_bytes)?;
        if grant.maximum_depth < demand.depth { return Err(ValueError::literal(ValueRefusalKind::DepthLimit, "config apply clone close exceeds admitted depth")); }
        if grant.maximum_items == 0 || demand.copy_bytes > grant.maximum_copy_bytes || demand.capacity_bytes > grant.maximum_capacity_bytes || demand.release_bytes > grant.maximum_release_bytes { return Ok(Default::default()); }
        let progress = if cursor.terminal_is_empty() {
            RetainedCloneProgress { copied_items: 1, copied_bytes: demand.copy_bytes, ..Default::default() }
        } else {
            let child = RetainedCloneGrant { maximum_depth: grant.maximum_depth - 1, ..grant };
            let step = cursor.close_step(child)?;
            admit_retained_clone_close(child, step, cursor.terminal_is_empty(), "config apply mutation clone close")?.progress()
        };
        if cursor.terminal_is_empty() { *self.clone = None; }
        Ok(progress)
    }
}

impl<S, M> RetainedCloneEditCursor<S, M> for ConfigApplyCursor<S, M>
where S: RetainedClone, M: ConfigApplyMutation<S> {
    fn advance(&mut self, _base: RetainedCloneRef<'_, S>, post: &mut S, mutation: RetainedCloneRef<'_, M>, grant: RetainedCloneGrant) -> Result<RetainedCloneEditStep, ValueError> {
        if self.cancelled || self.custody.is_closing() || grant.maximum_items == 0 || grant.maximum_depth < 2 { return Ok(RetainedCloneEditStep::Progress(Default::default())); }
        let refusal = |message: &'static str| ValueError::literal(ValueRefusalKind::InvariantViolated, message);
        let receipt = match self.stage {
            ApplyStage::Done => return Ok(RetainedCloneEditStep::Complete(Default::default())),
            ApplyStage::Clone => {
                let child = RetainedCloneGrant { maximum_depth: grant.maximum_depth - 1, ..grant };
                let cursor = self.clone.as_mut().ok_or_else(|| refusal("config apply lost its mutation clone cursor"))?;
                let step = cursor.advance(mutation, child)?;
                let progress = admit_retained_clone_progress(child, step.progress(), "config apply mutation clone")?;
                if matches!(step, RetainedCloneStep::Complete(_)) { self.stage = ApplyStage::Take; }
                progress
            }
            ApplyStage::Take => {
                let copy = size_of::<M>() * 2;
                if grant.maximum_copy_bytes < copy { return Ok(RetainedCloneEditStep::Progress(Default::default())); }
                let cursor = self.clone.as_mut().ok_or_else(|| refusal("config apply lost its mutation clone cursor"))?;
                let owned = cursor.take().ok_or_else(|| refusal("config apply mutation clone completed without its owner"))?;
                self.custody.original_mut().0 = Some(owned);
                let _ = cursor.begin_close();
                self.stage = ApplyStage::CloseClone;
                RetainedCloneProgress { copied_items: 1, copied_bytes: copy, ..Default::default() }
            }
            ApplyStage::CloseClone => {
                let progress = self.close_clone(grant)?;
                if self.clone.is_none() { self.stage = ApplyStage::Reserve; }
                progress
            }
            ApplyStage::Reserve => {
                let (copy, capacity) = (size_of::<Vec<M>>(), size_of::<M>());
                if grant.maximum_copy_bytes < copy || grant.maximum_capacity_bytes < capacity { return Ok(RetainedCloneEditStep::Progress(Default::default())); }
                let mut rows = Vec::new();
                rows.try_reserve_exact(1).map_err(|_| ValueError::literal(ValueRefusalKind::AllocationFailed, "config apply inverse row allocation failed"))?;
                let actual = rows.capacity() * size_of::<M>();
                self.custody.original_mut().1 = Some(rows);
                if actual > grant.maximum_capacity_bytes { self.cancelled = true; return Err(refusal("config apply inverse row allocation exceeded its admitted capacity")); }
                self.stage = ApplyStage::Exchange;
                RetainedCloneProgress { copied_items: 1, copied_bytes: copy, retained_capacity_bytes: actual, released_bytes: 0 }
            }
            ApplyStage::Exchange => {
                let copy = size_of::<M>() * 2 + size_of::<S>();
                if grant.maximum_copy_bytes < copy { return Ok(RetainedCloneEditStep::Progress(Default::default())); }
                let fields = self.custody.original_mut();
                if fields.1.as_ref().is_none_or(|rows| rows.capacity() <= rows.len()) { return Err(refusal("config apply inverse row capacity is absent")); }
                let owned = fields.0.take().ok_or_else(|| refusal("config apply payload is absent"))?;
                match owned.exchange(post) {
                    Ok(inverse) => fields.1.as_mut().expect("validated inverse row capacity").push(inverse),
                    Err((error, owned)) => { fields.0 = Some(owned); self.cancelled = true; return Err(error); }
                }
                self.stage = ApplyStage::Done;
                RetainedCloneProgress { copied_items: 1, copied_bytes: copy, ..Default::default() }
            }
        };
        Ok(if self.stage == ApplyStage::Done { RetainedCloneEditStep::Complete(receipt) } else { RetainedCloneEditStep::Progress(receipt) })
    }
    fn take_inverse(&mut self) -> Option<Vec<M>> { (self.stage == ApplyStage::Done).then(|| self.custody.original_mut().1.take()).flatten() }
    fn foreign_step_presence(&self) -> bool { false }
    fn cancel(&mut self) { self.cancelled = true; }
    fn begin_close(&mut self) -> bool {
        let started = self.custody.begin_close();
        if let Some(cursor) = self.clone.as_mut() { let _ = cursor.begin_close(); }
        started
    }
    fn close_step(&mut self, grant: RetainedCloneGrant) -> Result<RetainedCloneStep, ValueError> {
        if self.terminal_is_empty() { return Ok(RetainedCloneStep::Complete(Default::default())); }
        if self.clone.is_none() { return self.custody.step(grant); }
        self.close_clone(grant).map(RetainedCloneStep::Progress)
    }
    fn next_close_copy_byte_demand(&self) -> Result<usize, ValueError> { Ok(self.close_demand(0)?.copy_bytes) }
    fn next_close_capacity_byte_demand(&self, body: usize) -> Result<usize, ValueError> { Ok(self.close_demand(body)?.capacity_bytes) }
    fn next_close_release_byte_demand(&self) -> Result<usize, ValueError> { Ok(self.close_demand(0)?.release_bytes) }
    fn next_close_depth_demand(&self) -> Result<usize, ValueError> { Ok(self.close_demand(0)?.depth) }
    fn terminal_is_empty(&self) -> bool { self.clone.is_none() && self.custody.terminal_is_empty() }
}

impl<S, M: ConfigApplyMutation<S>> Drop for ConfigApplyCursor<S, M> {
    fn drop(&mut self) {
        assert!(std::thread::panicking() || (self.clone.is_none() && self.custody.terminal_is_empty()), "config apply cursor requires complete admitted closure");
    }
}

/// 📏️ Peak live retained capacity every sealed config edit declares before its own record cost: the canonical sealer plus the record clone peak at 40645 bytes for a four-node record (`drive_one_item_preparation_law`), declared with a threefold allowance.
pub const CONFIG_EDIT_RETAINED_BASELINE_BYTES: usize = 131072;

/// 🏭️ Structural depth envelope of the config apply preparation.
pub const CONFIG_APPLY_MAXIMUM_PREPARATION_DEPTH: usize = 64;

/// 🏭️ Builds the store's one-item preparation factory for a config lane from its field-set mutation.
pub fn config_apply_preparation_factory<S, M>() -> Arc<dyn ArtifactStoreOneItemPreparationFactory<S, M>>
where S: RetainedClone, M: ConfigApplyMutation<S> + ArtifactCanonicalJsonTree {
    let mutation: Arc<dyn ArtifactOwnedValueRetirementFactory<M>> = Arc::new(semio_framework_value::retirement::OwnedValueRetirementFactory::<M>::default());
    let snapshot: Arc<dyn SnapshotRetirementFactory<S>> = Arc::new(semio_framework_value::retirement::SharedValueRetirementFactory::<S>::default());
    Arc::new(RetainedClonePreparationFactory::new(Arc::new(ConfigApplyEdit::<S, M>::new()), mutation, snapshot, CONFIG_APPLY_MAXIMUM_PREPARATION_DEPTH).expect("config apply declares a nonzero preparation depth"))
}
