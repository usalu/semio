//! ✏️ Field-set apply edit for small config records (window configs and document config lanes): clones the mutation payload under grants, exchanges it into the post record by move and keeps the displaced content as the inverse row, all through original custody.
use semio_os_store_one_item_publication::native_source::pay;
use semio_framework_value::{OriginalRetirementFailure, RetainedAllocationEffect, RetainedWorkPoll, retained_clone::OriginalEffectReply};
use super::{RetainedCloneEdit,RetainedCloneEditCursor,RetainedCloneEditStep,RetainedClonePreparationFactory,ReplyStash,custody::RetainedEditCustody};
use semio_os_store_one_item::{ARTIFACT_STORE_ONE_ITEM_MAXIMUM_BYTES, ArtifactStoreOneItemFootprint};
use semio_framework_pack_json::ArtifactCanonicalJsonTree;
use semio_framework_value::{ArtifactOwnedValueRetirementFactory, SnapshotRetirementFactory};
use semio_os_store_one_item_publication::ArtifactStoreOneItemPreparationFactory;
use semio_os_store_command::HistoryLane;
use semio_framework_value::{RetirementDemand,ValueError,ValueRefusalKind,retained_clone::{RetainedClone,RetainedCloneBirthDemand,RetainedCloneCursor,RetainedCloneGrant,RetainedCloneProgress,RetainedCloneRef,RetainedCloneStep,admit_retained_clone_close,admit_retained_clone_progress}};
use std::{marker::PhantomData,mem::{ManuallyDrop,size_of},sync::Arc};

type CloneCursor<M> = <M as RetainedClone>::Cursor;

/// 📝️ Declares one field-set mutation of a config record whose payload may own heap content.
///
/// `exchange` runs after the store cloned the base into the post record, so the post still equals the base: it MUST move the owned payload into the post record and return the row that moves the displaced content back. It MUST NOT allocate or release heap memory; the edit accounts it as one fixed-size move.
pub trait ConfigApplyMutation<S>: RetainedClone {
    fn exchange(self, post: &mut S) -> Result<Self, (ValueError, Self)>;
    fn maximum_inverse_rows()->usize{1}
    fn publication_work_items(&self)->usize{2}
    fn take_followup_inverse(&self,_post:&mut S)->Option<Self>{None}
    fn validation_demands(&self,_post:&S,_cursor:usize,_body:usize)->Result<RetirementDemand,ValueError>{Ok(RetirementDemand{copy_bytes:size_of::<usize>(),depth:1,..Default::default()})}
    fn validation_work_demand(&self,_post:&S,_cursor:usize,_body:usize)->Result<u64,ValueError>{Ok(1)}
    fn validate_step(&self,_post:&S,_cursor:&mut usize,_body:usize)->semio_framework_value::RetainedWorkOutcome<(bool,RetainedCloneProgress)>{semio_framework_value::RetainedWorkOutcome{result:Ok((true,RetainedCloneProgress{copied_items:1,copied_bytes:size_of::<usize>(),..Default::default()})),work:1}}
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
enum ApplyStage { Validate, Clone, Take, CloseClone, Reserve, Exchange, Done }

/// 🪜️ Bounded turns: clone the mutation payload, take it into custody, close the clone cursor, reserve the inverse row, exchange payload and displaced content.
pub struct ConfigApplyCursor<S, M: ConfigApplyMutation<S>> {
    pub stage: ApplyStage,
    pub cancelled: bool,
    pub validation_cursor:usize,
    pub clone: ManuallyDrop<Option<CloneCursor<M>>>,
    clone_reply: ReplyStash<OriginalRetirementFailure>,
    pub custody: RetainedEditCustody<(Option<M>, Option<Vec<M>>)>,
    pub marker: PhantomData<fn() -> S>,
}

impl<S, M> RetainedCloneEdit<S, M> for ConfigApplyEdit<S, M>
where S: RetainedClone, M: ConfigApplyMutation<S> {
    type Cursor = ConfigApplyCursor<S, M>;
    fn preflight(&self, mutation: &M, lane: HistoryLane) -> Result<ArtifactStoreOneItemFootprint, &'static str> {
        if !mutation.admits_lane(lane) || !mutation.admissible() { return Err("config apply mutation has invalid lane or record authority".into()); }
        let retained_bytes = mutation.payload_bytes().checked_mul(4).and_then(|bytes| bytes.checked_add(CONFIG_EDIT_RETAINED_BASELINE_BYTES)).filter(|bytes| *bytes <= ARTIFACT_STORE_ONE_ITEM_MAXIMUM_BYTES).ok_or("config apply payload exceeds the one-item retained footprint")?;
        Ok(ArtifactStoreOneItemFootprint { work_items: mutation.publication_work_items(), retained_bytes })
    }
    fn begin_demand(&self) -> RetainedCloneBirthDemand { RetainedCloneBirthDemand { capacity_bytes: 0, depth: 1 } }
    fn snapshot_cursor_birth_demand(&self) -> RetainedCloneBirthDemand { RetainedCloneBirthDemand { capacity_bytes: 0, depth: 1 } }
    fn begin(&self, grant: RetainedCloneGrant) -> Result<(Self::Cursor, RetainedCloneProgress), ValueError> {
        let mut receipt = self.begin_demand().admit(grant)?;
        receipt.copied_bytes = size_of::<Self::Cursor>();
        if !receipt.fits(grant) { return Err(ValueError::literal(ValueRefusalKind::WorkLimit, "config apply cursor transfer exceeds grant")); }
        Ok((ConfigApplyCursor { stage: ApplyStage::Validate, cancelled: false, validation_cursor:0, clone: ManuallyDrop::new(Some(M::retained_clone_cursor())), clone_reply: ReplyStash::default(), custody: RetainedEditCustody::new((None, None)), marker: PhantomData }, receipt))
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
    pub fn close_demand(&self, body: usize) -> Result<RetirementDemand, ValueError> {
        match self.clone.as_ref() {
            Some(cursor) => Self::clone_close_demand(cursor, body),
            None => self.custody.demands(body),
        }
    }
    fn close_clone(&mut self, effect: &mut RetainedAllocationEffect<'_>) -> Result<RetainedCloneProgress, ValueError> {
        let grant = effect.grant();
        let cursor = self.clone.as_mut().ok_or_else(|| ValueError::literal(ValueRefusalKind::InvariantViolated, "config apply lost its mutation clone cursor"))?;
        let demand = Self::clone_close_demand(cursor, grant.maximum_copy_bytes)?;
        if grant.maximum_depth < demand.depth { return Err(ValueError::literal(ValueRefusalKind::DepthLimit, "config apply clone close exceeds admitted depth")); }
        if grant.maximum_items == 0 || demand.copy_bytes > grant.maximum_copy_bytes || demand.capacity_bytes > grant.maximum_capacity_bytes || demand.release_bytes > grant.maximum_release_bytes { return Ok(Default::default()); }
        let progress = if !cursor.close_started() {
            let child = RetainedCloneGrant { maximum_items:1,maximum_depth: grant.maximum_depth - 1, ..grant };
            cursor.begin_close_paid(&mut effect.nested(child)).result?
        } else if cursor.terminal_is_empty() {
            pay(effect, 1)?;
            RetainedCloneProgress { copied_items: 1, copied_bytes: demand.copy_bytes, ..Default::default() }
        } else {
            let child = RetainedCloneGrant { maximum_items:1,maximum_depth: grant.maximum_depth - 1, ..grant };
            let step=match cursor.close_step(grant.maximum_copy_bytes,&mut effect.nested(child)){RetainedWorkPoll::Pending{..}=>RetainedCloneStep::Progress(Default::default()),RetainedWorkPoll::Complete(turn)=>turn.result.map_err(|failure|failure.into_value_error())?};
            admit_retained_clone_close(child, step, cursor.terminal_is_empty(), "config apply mutation clone close")?.progress()
        };
        if cursor.terminal_is_empty() { *self.clone = None; }
        Ok(progress)
    }
}

impl<S, M> RetainedCloneEditCursor<S, M> for ConfigApplyCursor<S, M>
where S: RetainedClone, M: ConfigApplyMutation<S> {
    fn next_work_demand(&self,_base:RetainedCloneRef<'_,S>,post:&S,mutation:RetainedCloneRef<'_,M>,body:usize)->Result<u64,ValueError>{if self.cancelled||self.custody.is_closing(){return Ok(0);}if self.stage==ApplyStage::Validate{mutation.get().validation_work_demand(post,self.validation_cursor,body)}else{Ok(1)}}
    fn advance_demands(&self, _base: RetainedCloneRef<'_, S>, post: &S, mutation: RetainedCloneRef<'_, M>, body: usize) -> Result<RetirementDemand, ValueError> {
        if self.cancelled || self.custody.is_closing() { return Ok(Default::default()); }
        Ok(match self.stage {
            ApplyStage::Validate=>mutation.get().validation_demands(post,self.validation_cursor,body)?,
            ApplyStage::Clone => { let cursor = self.clone.as_ref().ok_or_else(|| ValueError::literal(ValueRefusalKind::InvariantViolated, "config apply mutation clone absent"))?; let mut demand = cursor.advance_demands(mutation, body, u64::MAX)?; demand.depth = demand.depth.checked_add(1).ok_or_else(|| ValueError::literal(ValueRefusalKind::DepthLimit, "config apply clone depth overflow"))?; demand },
            ApplyStage::Take => RetirementDemand { copy_bytes: size_of::<M>() * 2, depth: 2, ..Default::default() },
            ApplyStage::CloseClone => Self::clone_close_demand(self.clone.as_ref().ok_or_else(|| ValueError::literal(ValueRefusalKind::InvariantViolated, "config apply mutation clone absent"))?, body)?,
            ApplyStage::Reserve => RetirementDemand { copy_bytes: size_of::<Vec<M>>(), capacity_bytes: size_of::<M>()*M::maximum_inverse_rows(), depth: 2, ..Default::default() },
            ApplyStage::Exchange => RetirementDemand { copy_bytes: size_of::<M>() * (1+M::maximum_inverse_rows()) + size_of::<S>(), depth: 2, ..Default::default() },
            ApplyStage::Done => Default::default(),
        })
    }
    fn advance(&mut self, _base: RetainedCloneRef<'_, S>, post: &mut S, mutation: RetainedCloneRef<'_, M>, body:usize,effect: &mut RetainedAllocationEffect<'_>) -> Result<RetainedCloneEditStep, ValueError> {
        let grant = effect.grant();
        if self.cancelled || self.custody.is_closing() || grant.maximum_items == 0 || grant.maximum_depth < 2 { return Ok(RetainedCloneEditStep::Progress(Default::default())); }
        let refusal = |message: &'static str| ValueError::literal(ValueRefusalKind::InvariantViolated, message);
        let receipt = match self.stage {
            ApplyStage::Validate=>{
                let demand=mutation.get().validation_demands(post,self.validation_cursor,body)?;
                if !semio_framework_value::retained_clone::retained_clone_grant_admits(grant,demand){return Ok(RetainedCloneEditStep::Progress(Default::default()));}
                let turn=mutation.get().validate_step(post,&mut self.validation_cursor,body);pay(effect,turn.work)?;let(done,progress)=turn.result?;
                admit_retained_clone_progress(grant,progress,"config validation")?;
                if done{self.stage=ApplyStage::Clone;}
                progress
            }
            ApplyStage::Done => return Ok(RetainedCloneEditStep::Complete(Default::default())),
            ApplyStage::Clone => {
                let child = RetainedCloneGrant { maximum_items:1,maximum_depth: grant.maximum_depth - 1, ..grant };
                let cursor = self.clone.as_mut().ok_or_else(|| refusal("config apply lost its mutation clone cursor"))?;
                let maximum = effect.remaining_work_units()?;
                let nested = &mut effect.nested(child);
                let Some(answer) = self.clone_reply.run(|reply, recover| if recover { cursor.recover_advance_into(mutation, body, maximum, nested, reply) } else { cursor.advance_into(mutation, body, maximum, nested, reply) }.map(drop))? else { return Ok(RetainedCloneEditStep::Progress(Default::default())) };
                let step = answer.map_err(|failure| failure.into_value_error())?;
                let progress = admit_retained_clone_progress(child, step.progress(), "config apply mutation clone")?;
                if matches!(step, RetainedCloneStep::Complete(_)) { self.stage = ApplyStage::Take; }
                progress
            }
            ApplyStage::Take => {
                let copy = size_of::<M>() * 2;
                if grant.maximum_copy_bytes < copy { return Ok(RetainedCloneEditStep::Progress(Default::default())); }
                let cursor = self.clone.as_mut().ok_or_else(|| refusal("config apply lost its mutation clone cursor"))?;
                let (mut candidate, mut slot): (Option<M>, Option<Result<RetainedCloneStep, ValueError>>) = (None, None);
                {let reply = OriginalEffectReply::borrow(&mut slot)?; cursor.take(&mut candidate, &mut effect.nested(RetainedCloneGrant { maximum_depth: grant.maximum_depth - 1, ..grant }), reply)?;}
                slot.take().ok_or_else(|| refusal("config apply mutation clone take left its reply pending"))??;
                let owned = candidate.ok_or_else(|| refusal("config apply mutation clone completed without its owner"))?;
                self.custody.original_mut().0 = Some(owned);
                self.stage = ApplyStage::CloseClone;
                RetainedCloneProgress { copied_items: 1, copied_bytes: copy, ..Default::default() }
            }
            ApplyStage::CloseClone => {
                let progress = self.close_clone(effect)?;
                if self.clone.is_none() { self.stage = ApplyStage::Reserve; }
                progress
            }
            ApplyStage::Reserve => {
                let (copy, capacity) = (size_of::<Vec<M>>(), size_of::<M>()*M::maximum_inverse_rows());
                if grant.maximum_copy_bytes < copy || grant.maximum_capacity_bytes < capacity { return Ok(RetainedCloneEditStep::Progress(Default::default())); }
                let mut rows = Vec::new();
                pay(effect, 1)?;
                rows.try_reserve_exact(M::maximum_inverse_rows()).map_err(|_| ValueError::literal(ValueRefusalKind::AllocationFailed, "config apply inverse row allocation failed"))?;
                let actual = rows.capacity() * size_of::<M>();
                self.custody.original_mut().1 = Some(rows);
                if actual > grant.maximum_capacity_bytes { self.cancelled = true; return Err(refusal("config apply inverse row allocation exceeded its admitted capacity")); }
                self.stage = ApplyStage::Exchange;
                RetainedCloneProgress { copied_items: 1, copied_bytes: copy, retained_capacity_bytes: actual, released_bytes: 0 }
            }
            ApplyStage::Exchange => {
                let copy = size_of::<M>() * (1+M::maximum_inverse_rows()) + size_of::<S>();
                if grant.maximum_copy_bytes < copy { return Ok(RetainedCloneEditStep::Progress(Default::default())); }
                let fields = self.custody.original_mut();
                if fields.1.as_ref().is_none_or(|rows| rows.capacity()-rows.len()<M::maximum_inverse_rows()) { return Err(refusal("config apply inverse row capacity is absent")); }
                let owned = fields.0.take().ok_or_else(|| refusal("config apply payload is absent"))?;
                pay(effect, 1)?;
                match owned.exchange(post) {
                    Ok(inverse) => {let followup=inverse.take_followup_inverse(post);let rows=fields.1.as_mut().expect("validated inverse row capacity");rows.push(inverse);if let Some(followup)=followup{rows.push(followup);}},
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
        started
    }
    fn close_step(&mut self, effect: &mut RetainedAllocationEffect<'_>) -> Result<RetainedCloneStep, ValueError> {
        if self.terminal_is_empty() { return Ok(RetainedCloneStep::Complete(Default::default())); }
        if self.clone.is_none() { return self.custody.step(effect); }
        self.close_clone(effect).map(RetainedCloneStep::Progress)
    }
    fn next_close_copy_byte_demand(&self) -> Result<usize, ValueError> { Ok(self.close_demand(0)?.copy_bytes) }
    fn next_close_capacity_byte_demand(&self, body: usize) -> Result<usize, ValueError> { Ok(self.close_demand(body)?.capacity_bytes) }
    fn next_close_release_byte_demand(&self) -> Result<usize, ValueError> { Ok(self.close_demand(0)?.release_bytes) }
    fn next_close_depth_demand(&self) -> Result<usize, ValueError> { Ok(self.close_demand(0)?.depth) }
    fn terminal_is_empty(&self) -> bool { self.clone.is_none() && !self.clone_reply.pending() && self.custody.terminal_is_empty() }
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
    let mutation: Arc<dyn ArtifactOwnedValueRetirementFactory<M>> = Arc::new(semio_framework_value::factory::OwnedValueRetirementFactory::<M>::default());
    let snapshot: Arc<dyn SnapshotRetirementFactory<S>> = Arc::new(semio_framework_value::factory::SharedValueRetirementFactory::<S>::default());
    Arc::new(RetainedClonePreparationFactory::new(Arc::new(ConfigApplyEdit::<S, M>::new()), mutation, snapshot, CONFIG_APPLY_MAXIMUM_PREPARATION_DEPTH).expect("config apply declares a nonzero preparation depth"))
}
