//! ❄️ Cold one-item preparation: a family whose mutations carry only the explicit cold reference semantics (`ColdMutation`) decides, inverts and applies them on an owned scratch projection, one funded turn per step.
//!
//! Retained families use `mutation_apply_preparation_factory`; this is the cold sibling, never a default of it.
use crate::OneItemOwners;
use semio_os_spr::{apply_diff_cold, apply_diff_cold_into, ColdMutation, Edit, Mutation, MutationDiff};
use semio_os_spr::command::MutationMessagePublication;
use semio_os_store_replay_operation::native::{NativeSemanticPreparation, SemanticEdit, SemanticEditStep, SemanticProjection};
use semio_os_store_replay_operation::{ArtifactReplayPreparation, ArtifactReplayPreparationFactory, ArtifactReplayPreparationRequest};
use semio_framework_value_snapshot_projection::snapshot_projection::post::{PagedOneItemPost, PagedOneItemPostMode, PagedOneItemPostRead};
use semio_framework_value::retained_clone::{retained_clone_grant_admits, RetainedCloneClose, RetainedCloneRef};
use semio_framework_value::{ArtifactOwnedValueRetirementFactory, RetirementDemand};
use std::mem::ManuallyDrop;
use semio_os_store_command::HistoryLane;
use semio_os_store_one_item::{ArtifactStoreOneItemCheckpoint, ArtifactStoreOneItemFootprint, ArtifactStoreOneItemGrant, ArtifactStoreOneItemPreparationStep, ARTIFACT_STORE_ONE_ITEM_ID_BYTES, ARTIFACT_STORE_ONE_ITEM_MAXIMUM_BYTES};
use semio_os_store_one_item_publication::batch_digest::{admit_artifact_batch_digest, ArtifactStoreBatchDigest};
use semio_os_store_one_item_publication::{ArtifactStoreOneItemPrepared, ArtifactStoreOneItemPreparation, ArtifactStoreOneItemPreparationFactory, ArtifactStoreOneItemPreparationRequest};
use semio_framework_value::retained_clone::{RetainedCloneBirthDemand, RetainedCloneGrant, RetainedCloneProgress, RetainedCloneStep};
use semio_framework_pack_json::ArtifactCanonicalJsonTree;
use semio_framework_value::retirement::RetireOwned;
use semio_framework_value::{ValueError, ValueRefusalKind};
use std::{marker::PhantomData, mem::size_of, sync::Arc};

/// 📏️ The fixed Store envelope of one cold family: the bytes a mutation and the base snapshot may carry, priced by the domain's own measures.
pub struct ColdPreparationBounds<S, M> {
    pub maximum_bytes: usize,
    pub mutation_bytes: fn(&M) -> usize,
    pub snapshot_bytes: fn(&S) -> usize,
}

impl<S, M> Clone for ColdPreparationBounds<S, M> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<S, M> Copy for ColdPreparationBounds<S, M> {}

/// 🏭️ The cold preparation factory of one snapshot and mutation family; with bounds it refuses oversize mutations and bases at preflight and meters the base clone.
#[derive(semio_framework_value::FactoryPayloadRetirement)]
pub struct ColdMutationApplyFactory<S, M> {
    marker: PhantomData<fn() -> (S, M)>,
    bounds: Option<ColdPreparationBounds<S, M>>,
}

impl<S, M> Default for ColdMutationApplyFactory<S, M> {
    fn default() -> Self {
        Self { marker: PhantomData, bounds: None }
    }
}

impl<S, M> ColdMutationApplyFactory<S, M> {
    pub fn bounded(bounds: ColdPreparationBounds<S, M>) -> Self {
        Self { marker: PhantomData, bounds: Some(bounds) }
    }
}

/// 🧊️ Every cold mutation family answers the central document lane through its explicit cold reference semantics.
pub fn cold_mutation_apply_preparation_factory<S, M>() -> Arc<dyn ArtifactStoreOneItemPreparationFactory<S, M>>
where
    S: Clone + RetireOwned + Send + Sync + 'static,
    M: Mutation<S> + ColdMutation<S> + ArtifactCanonicalJsonTree + RetireOwned + Send + Sync + 'static,
{
    Arc::new(ColdMutationApplyFactory::<S, M>::default())
}

/// 📄️ Every cold family answers the central document lane within the fixed Store envelope its domain declares.
pub fn bounded_cold_mutation_apply_preparation_factory<S, M>(bounds: ColdPreparationBounds<S, M>) -> Arc<dyn ArtifactStoreOneItemPreparationFactory<S, M>>
where
    S: Clone + RetireOwned + Send + Sync + 'static,
    M: Mutation<S> + ColdMutation<S> + ArtifactCanonicalJsonTree + RetireOwned + Send + Sync + 'static,
{
    Arc::new(ColdMutationApplyFactory::<S, M>::bounded(bounds))
}

#[derive(Clone,Copy,PartialEq,Eq)]
enum ColdPreparationPhase{Decide,Messages,Seal,Prepared,Refused}

struct ColdMutationApplyPreparation<S: 'static, M: 'static> {
    clone_bytes: Option<usize>,
    owners: OneItemOwners<S, M>,
    checkpoint: ArtifactStoreOneItemCheckpoint,
    last_work:u64,
    phase:ColdPreparationPhase,
    messages:Option<MutationMessagePublication>,
    cancelled: bool,
}

fn fault(reason: &'static str) -> ValueError {
    ValueError::literal(ValueRefusalKind::InvariantViolated, reason)
}

impl<S:'static,M:'static> ColdMutationApplyPreparation<S,M>{
    fn record(&mut self,progress:RetainedCloneProgress)->ArtifactStoreOneItemPreparationStep{
        if progress==RetainedCloneProgress::default()&&self.last_work==0{return ArtifactStoreOneItemPreparationStep::Blocked;}
        self.checkpoint.cursor=self.checkpoint.cursor.saturating_add(1);
        self.checkpoint.completed_items=self.checkpoint.completed_items.saturating_add(u32::try_from(progress.copied_items).unwrap_or(u32::MAX));
        self.checkpoint.completed_bytes=self.checkpoint.completed_bytes.saturating_add(progress.copied_bytes.saturating_add(progress.retained_capacity_bytes).saturating_add(progress.released_bytes) as u64);
        ArtifactStoreOneItemPreparationStep::Progress(self.checkpoint,progress)
    }
    fn message_close_demand(&self)->Result<Option<RetirementDemand>,ValueError>{
        self.messages.as_ref().map(|owner|if owner.terminal_is_empty(){Ok(RetirementDemand{copy_bytes:size_of::<MutationMessagePublication>(),depth:1,..Default::default()})}else{let mut demand=owner.close_demands()?;demand.depth=demand.depth.checked_add(1).ok_or_else(||fault("cold diagnostic close depth overflow"))?;Ok(demand)}).transpose()
    }
}

impl<S, M> ArtifactStoreOneItemPreparationFactory<S, M> for ColdMutationApplyFactory<S, M>
where
    S: Clone + RetireOwned + Send + Sync + 'static,
    M: Mutation<S> + ColdMutation<S> + ArtifactCanonicalJsonTree + RetireOwned + Send + Sync + 'static,
{
    fn begin_batch_digest(&self, edit: &mut Option<Box<Edit<M>>>, grant: RetainedCloneGrant) -> Result<Option<(Box<dyn ArtifactStoreBatchDigest<M>>, RetainedCloneProgress)>, ValueError> {
        admit_artifact_batch_digest(edit, grant)
    }

    fn preflight(&self, mutation: &M, lane: HistoryLane) -> Result<ArtifactStoreOneItemFootprint, String> {
        if lane != HistoryLane::Document {
            return Err("cold mutation preparation rejected its lane".into());
        }
        let Some(bounds) = self.bounds else {
            return Ok(ArtifactStoreOneItemFootprint::for_leaf::<S, _>(mutation, ARTIFACT_STORE_ONE_ITEM_MAXIMUM_BYTES));
        };
        let bytes = (bounds.mutation_bytes)(mutation);
        if bytes > bounds.maximum_bytes {
            return Err("cold mutation exceeds its fixed Store envelope".into());
        }
        Ok(ArtifactStoreOneItemFootprint::for_leaf::<S, _>(mutation, bytes))
    }

    fn begin_demand(&self, _mutation: &M, _lane: HistoryLane) -> Result<RetainedCloneBirthDemand, ValueError> {
        Ok(RetainedCloneBirthDemand { capacity_bytes: size_of::<ColdMutationApplyPreparation<S, M>>(), depth: 1 })
    }

    fn begin_copy_byte_demand(&self, mutation: &M, lane: HistoryLane) -> Result<usize, ValueError> {
        self.begin_demand(mutation, lane)?;
        size_of::<ArtifactStoreOneItemPreparationRequest<S, M, M>>().checked_add(size_of::<ColdMutationApplyPreparation<S, M>>()).ok_or_else(|| fault("cold mutation preparation copy quote overflow"))
    }

    fn begin(&self, request: ArtifactStoreOneItemPreparationRequest<S, M, M>, grant: ArtifactStoreOneItemGrant) -> Result<(Box<dyn ArtifactStoreOneItemPreparation<S, M>>, RetainedCloneProgress), (ValueError, ArtifactStoreOneItemPreparationRequest<S, M, M>)> {
        if request.lane != HistoryLane::Document || request.operation != request.authority.operation() || request.generation != request.authority.generation() || request.base_revision != request.authority.base_revision() || request.authority.actor().len() > ARTIFACT_STORE_ONE_ITEM_ID_BYTES {
            return Err((fault("cold mutation preparation rejected its original publication authority"), request));
        }
        let clone_bytes = self.bounds.map(|bounds| (bounds.snapshot_bytes)(request.base.get()));
        if self.bounds.zip(clone_bytes).is_some_and(|(bounds, bytes)| bytes > bounds.maximum_bytes) {
            return Err((fault("cold mutation preparation base exceeds its fixed Store envelope"), request));
        }
        let demand = match self.begin_demand(&request.mutation, request.lane) {
            Ok(demand) => demand,
            Err(error) => return Err((error, request)),
        };
        let progress = match demand.admit(grant.retained_grant()) {
            Ok(progress) => progress,
            Err(error) => return Err((error, request)),
        };
        Ok((Box::new(ColdMutationApplyPreparation { clone_bytes, owners: OneItemOwners::from_request(request), checkpoint: ArtifactStoreOneItemCheckpoint::default(),last_work:0,phase:ColdPreparationPhase::Decide,messages:None,cancelled:false }), progress))
    }
}

impl<S, M> ArtifactStoreOneItemPreparation<S, M> for ColdMutationApplyPreparation<S, M>
where
    S: Clone + RetireOwned + Send + Sync + 'static,
    M: Mutation<S> + ColdMutation<S> + ArtifactCanonicalJsonTree + RetireOwned + Send + Sync + 'static,
{
    fn advance(&mut self, _body: usize, grant: ArtifactStoreOneItemGrant, receiving: &mut semio_framework_value_native_owned::native_decoding::NativeDecodeControl<'_>) -> Result<ArtifactStoreOneItemPreparationStep, semio_framework_value::OriginalRetirementFailure> {
        self.last_work=0;
        if !grant.permits_one() || self.cancelled || self.owners.is_closing() {
            return Ok(ArtifactStoreOneItemPreparationStep::Blocked);
        }
        let before=receiving.actual_work();
        let mut coordinated=false;
        let result=(|| -> Result<ArtifactStoreOneItemPreparationStep,semio_framework_value::OriginalRetirementFailure>{
        if self.owners.refused.is_some() {
            return Err((ValueError::literal(ValueRefusalKind::InvalidValue, "cold mutation preparation retains its original Store refusal")).into());
        }
        if self.owners.prepared.is_some() || self.phase==ColdPreparationPhase::Prepared {
            return Ok(ArtifactStoreOneItemPreparationStep::Prepared(self.checkpoint, RetainedCloneProgress::default()));
        }
        match self.phase {
            ColdPreparationPhase::Decide => {
                let clone_bytes = self.clone_bytes.map_or(0, |bytes| bytes.max(size_of::<S>()));
                let movement=size_of::<(S,Vec<M>,M)>();
                let copy=clone_bytes.checked_add(movement).and_then(|bytes|bytes.checked_add(size_of::<MutationMessagePublication>())).ok_or_else(||fault("cold decision copy quote overflow"))?;
                if grant.maximum_copy_bytes < copy {
                    return Ok(ArtifactStoreOneItemPreparationStep::Blocked);
                }
                let base = self.owners.base.as_ref().ok_or_else(|| fault("cold mutation preparation lost its exact base root"))?;
                let mutation = self.owners.mutation.as_ref().ok_or_else(|| fault("cold mutation preparation lost its mutation owner"))?;
                if !receiving.prepare_work(4)?{return Ok(ArtifactStoreOneItemPreparationStep::Blocked);}
                coordinated=true;self.last_work+=1;
                let outcome=ColdMutation::diff(mutation,base.get()).stamp_op_index(0);
                let(outcome,admitted)=match outcome.admit(){semio_os_spr::MutationAdmission::Admitted(outcome)=>(outcome,true),semio_os_spr::MutationAdmission::Refused(outcome)=>(outcome,false)};
                let(diff,messages)=outcome.into_parts();
                let captured=if messages.is_empty(){0}else{self.messages=Some(MutationMessagePublication::new(messages));size_of::<MutationMessagePublication>()};
                let capture=RetainedCloneProgress{copied_items:usize::from(captured!=0),copied_bytes:captured,..Default::default()};
                if !admitted{self.last_work+=1;<M::ReferenceDiff as MutationDiff<S>>::retire_cold(diff);self.phase=ColdPreparationPhase::Refused;self.cancelled=true;return Err((ValueError::literal(ValueRefusalKind::InvalidValue,"cold mutation produced a refused semantic outcome").with_retained_progress(capture)).into());}
                self.last_work+=1;
                let inverse=match ColdMutation::inverse(mutation,base.get()){Ok(inverse)=>inverse,Err(error)=>{self.last_work+=1;<M::ReferenceDiff as MutationDiff<S>>::retire_cold(diff);self.phase=ColdPreparationPhase::Refused;self.cancelled=true;return Err((error.with_retained_progress(capture)).into());}};
                self.last_work+=1;
                let applied=apply_diff_cold(&diff,base.get());
                self.last_work+=1;<M::ReferenceDiff as MutationDiff<S>>::retire_cold(diff);
                let post=match applied{Ok(post)=>post,Err(_)=>{self.phase=ColdPreparationPhase::Refused;self.cancelled=true;return Err((ValueError::literal(ValueRefusalKind::InvalidValue,"cold mutation could not produce its post root").with_retained_progress(capture)).into());}};
                let mutation = self.owners.mutation.take().expect("observed original mutation owner");
                *self.owners.candidate = Some((post, inverse, mutation));
                self.phase=if self.messages.is_some(){ColdPreparationPhase::Messages}else{ColdPreparationPhase::Seal};
                Ok(self.record(RetainedCloneProgress{copied_items:1,copied_bytes:clone_bytes+movement+captured,..Default::default()}))
            }
            ColdPreparationPhase::Messages=>{
                let owner=self.messages.as_mut().ok_or_else(||fault("cold diagnostic owner is absent"))?;
                if owner.ready(){if grant.maximum_copy_bytes<size_of::<u8>()||!receiving.prepare_work(1)?{return Ok(ArtifactStoreOneItemPreparationStep::Blocked);}self.phase=ColdPreparationPhase::Seal;coordinated=true;return Ok(self.record(RetainedCloneProgress{copied_items:1,copied_bytes:size_of::<u8>(),..Default::default()}));}
                let mut demand=owner.demands()?;demand.depth=demand.depth.checked_add(1).ok_or_else(||fault("cold diagnostic preparation depth overflow"))?;
                if !retained_clone_grant_admits(grant.retained_grant(),demand)||!receiving.prepare_work(1)?{return Ok(ArtifactStoreOneItemPreparationStep::Blocked);}
                let child=RetainedCloneGrant{maximum_depth:grant.maximum_depth-1,..grant.retained_grant()};
                coordinated=true;self.last_work+=1;let step=owner.advance(child)?;
                let progress=semio_framework_value::retained_clone::admit_retained_clone_progress(child,step.progress(),"cold original diagnostic publication")?;
                Ok(self.record(progress))
            }
            ColdPreparationPhase::Seal => {
                let copy=size_of::<(S,Vec<M>,M)>().checked_add(if self.messages.is_some(){size_of::<MutationMessagePublication>()}else{0}).ok_or_else(||fault("cold publication copy quote overflow"))?;
                if grant.maximum_copy_bytes<copy{return Ok(ArtifactStoreOneItemPreparationStep::Blocked);}
                let Some(authority) = self.owners.authority.as_ref() else {
                    return Err((fault("cold mutation preparation lost its Store authority")).into());
                };
                if !receiving.prepare_work(1)?{return Ok(ArtifactStoreOneItemPreparationStep::Blocked);}
                let (post, inverse, mutation) = self.owners.candidate.take().ok_or_else(|| fault("cold mutation preparation lost its semantic candidate"))?;
                coordinated=true;self.last_work+=1;
                let mut prepared = match authority.prepare_one_item(authority.next_edit(mutation, inverse), Arc::new(post)) {
                    Ok(prepared) => prepared,
                    Err((error, edit, post)) => {
                        *self.owners.refused = Some((edit, post));
                        return Err((error).into());
                    }
                };
                if let Some(messages)=self.messages.take(){prepared.record_outcome_messages(messages);}
                self.phase=ColdPreparationPhase::Prepared;
                let progress=RetainedCloneProgress{copied_items:1,copied_bytes:copy,..Default::default()};
                self.record(progress);self.checkpoint.digest=prepared.edit_digest();
                *self.owners.prepared = Some(prepared);
                Ok(ArtifactStoreOneItemPreparationStep::Prepared(self.checkpoint,progress))
            }
            ColdPreparationPhase::Prepared=>Ok(ArtifactStoreOneItemPreparationStep::Prepared(self.checkpoint,RetainedCloneProgress::default())),
            ColdPreparationPhase::Refused=>Err(ValueError::literal(ValueRefusalKind::InvalidValue,"cold mutation retains its semantic refusal")),
        }
        })();
        let observed=receiving.actual_work().saturating_sub(before);
        let reported=self.last_work;self.last_work=reported.max(observed).max(u64::from(coordinated));
        let work=semio_os_store_one_item::original_one_item_turn_work(reported,observed,coordinated)?;
        if work>observed{receiving.commit_work(work-observed)?;}
        self.last_work=work;
        result
    }

    fn last_turn_work(&self)->u64{self.last_work}

    fn outcome_messages(&self)->&[semio_os_spr::MutationMessage]{if let Some(owner)=self.messages.as_ref(){owner.original_messages()}else{self.owners.prepared.as_ref().map_or(&[],ArtifactStoreOneItemPrepared::outcome_messages)}}

    fn checkpoint(&self) -> ArtifactStoreOneItemCheckpoint {
        self.checkpoint
    }

    fn prepared(&self) -> Option<&ArtifactStoreOneItemPrepared<S, M>> {
        self.owners.prepared.as_ref()
    }

    fn take_prepared(&mut self) -> Option<ArtifactStoreOneItemPrepared<S, M>> {
        self.owners.prepared.take()
    }

    fn cancel(&mut self) {
        self.cancelled = true;
    }

    fn begin_close(&mut self) {
        if let Some(owner)=self.messages.as_mut(){owner.begin_close();}
        self.owners.begin_close();
    }

    fn close_step(&mut self, grant: ArtifactStoreOneItemGrant) -> Result<RetainedCloneStep, semio_framework_value::OriginalRetirementFailure> {
        if let Some(demand)=self.message_close_demand()?{
            let retained=grant.retained_grant();if !retained_clone_grant_admits(retained,demand){return Ok(RetainedCloneStep::Progress(Default::default()));}
            let owner=self.messages.as_mut().ok_or_else(||fault("cold diagnostic close owner is absent"))?;
            if owner.terminal_is_empty(){self.messages=None;return Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,..Default::default()}));}
            return owner.close_step(RetainedCloneGrant{maximum_depth:retained.maximum_depth-1,..retained}).map(|step|RetainedCloneStep::Progress(step.progress())).map_err(Into::into);
        }
        self.owners.close_step(grant.retained_grant()).map_err(Into::into)
    }

    fn next_close_copy_byte_demand(&self) -> Result<usize, ValueError> {
        if let Some(demand)=self.message_close_demand()?{return Ok(demand.copy_bytes);}
        Ok(self.owners.close_demands(0)?.copy_bytes)
    }

    fn next_close_capacity_byte_demand(&self, maximum_copy_bytes: usize) -> Result<usize, ValueError> {
        if let Some(demand)=self.message_close_demand()?{return Ok(demand.capacity_bytes);}
        Ok(self.owners.close_demands(maximum_copy_bytes)?.capacity_bytes)
    }

    fn next_close_release_byte_demand(&self) -> Result<usize, ValueError> {
        if let Some(demand)=self.message_close_demand()?{return Ok(demand.release_bytes);}
        Ok(self.owners.close_demands(0)?.release_bytes)
    }

    fn next_close_depth_demand(&self) -> Result<usize, ValueError> {
        if let Some(demand)=self.message_close_demand()?{return Ok(demand.depth);}
        Ok(self.owners.close_demands(0)?.depth)
    }

    fn terminal_is_empty(&self) -> bool {
        self.messages.is_none()&&self.owners.terminal_is_empty()
    }
}

fn replay_header<T>() -> RetirementDemand {
    RetirementDemand { copy_bytes: size_of::<T>(), depth: 1, ..Default::default() }
}

/// 📽️ Clones the immutable base into the owned post projection the cold edit then advances in place.
pub struct ColdReplayProjection<S: 'static> {
    last_work:u64,
    snapshot: Option<(fn(&S) -> usize, usize)>,
    clone: ManuallyDrop<Option<S>>,
    retirement: RetainedCloneClose,
    cloned: bool,
    closing: bool,
}

impl<S: 'static> Default for ColdReplayProjection<S> {
    fn default() -> Self {
        Self { last_work:0,snapshot: None, clone: ManuallyDrop::new(None), retirement: RetainedCloneClose::default(), cloned: false, closing: false }
    }
}

impl<S: 'static> ColdReplayProjection<S> {
    pub fn bounded(snapshot_bytes: fn(&S) -> usize, maximum_bytes: usize) -> Self {
        Self { snapshot: Some((snapshot_bytes, maximum_bytes)), ..Self::default() }
    }

    fn clone_demand(&self, source: &S) -> Result<RetirementDemand, ValueError> {
        let Some((snapshot_bytes, maximum_bytes)) = self.snapshot else {
            return Ok(replay_header::<S>());
        };
        let bytes = snapshot_bytes(source);
        if bytes > maximum_bytes {
            return Err(fault("cold replay base exceeds its fixed Store envelope"));
        }
        Ok(RetirementDemand { copy_bytes: bytes.max(size_of::<S>()), depth: 1, ..Default::default() })
    }
}

impl<S: Clone + RetireOwned + Send + Sync + 'static> SemanticProjection<S> for ColdReplayProjection<S> {
    fn birth_work_demand(&self)->Result<u64,ValueError>{Ok(1)}
    fn take_work_demand(&self)->Result<u64,ValueError>{Ok(u64::from(self.clone.is_some()))}
    fn last_turn_work(&self)->u64{self.last_work}
    fn birth_demand(&self) -> RetirementDemand {
        replay_header::<Self>()
    }

    fn begin(&mut self, grant: RetainedCloneGrant) -> Result<RetainedCloneProgress, ValueError> {
        self.last_work=0;
        let demand = self.birth_demand();
        if !retained_clone_grant_admits(grant, demand) {
            return Err(fault("cold replay projection birth not funded"));
        }
        self.last_work=1;
        Ok(RetainedCloneProgress { copied_items: 1, copied_bytes: demand.copy_bytes, ..Default::default() })
    }

    fn demands(&self, source: RetainedCloneRef<'_, S>, _body: usize) -> Result<RetirementDemand, ValueError> {
        self.clone_demand(source.get())
    }

    fn advance(&mut self, source: RetainedCloneRef<'_, S>, _body: usize, grant: RetainedCloneGrant) -> Result<RetainedCloneStep, semio_framework_value::OriginalRetirementFailure> {
        self.last_work=0;
        let demand = self.clone_demand(source.get())?;
        if self.closing || !retained_clone_grant_admits(grant, demand) {
            return Ok(RetainedCloneStep::Progress(RetainedCloneProgress::default()));
        }
        self.last_work=1;*self.clone = Some(source.get().clone());
        self.cloned = true;
        Ok(RetainedCloneStep::Complete(RetainedCloneProgress { copied_items: 1, copied_bytes: demand.copy_bytes, ..Default::default() }))
    }

    fn take_demand(&self) -> Result<RetirementDemand, ValueError> {
        Ok(RetirementDemand { copy_bytes: size_of::<S>() * 2, depth: 1, ..Default::default() })
    }

    fn take(&mut self, grant: RetainedCloneGrant) -> Result<Option<(S, RetainedCloneProgress)>, ValueError> {
        self.last_work=0;
        let demand = self.take_demand()?;
        if !retained_clone_grant_admits(grant, demand) {
            return Ok(None);
        }
        Ok(self.clone.take().map(|post| {self.last_work=1;(post, RetainedCloneProgress { copied_items: 1, copied_bytes: demand.copy_bytes, ..Default::default() })}))
    }

    fn begin_close(&mut self) {
        self.closing = true;
    }

    fn close_demand(&self, body: usize) -> Result<RetirementDemand, ValueError> {
        if !self.retirement.is_empty() || self.clone.is_some() {
            return self.retirement.next_owner_demands::<S>(self.clone.is_some(), &None, body);
        }
        Ok(if self.cloned { RetirementDemand { copy_bytes:size_of::<bool>(), depth:1, ..Default::default() } } else { RetirementDemand::default() })
    }

    fn close_step(&mut self, grant: RetainedCloneGrant) -> Result<RetainedCloneStep, semio_framework_value::OriginalRetirementFailure> {
        self.last_work=0;
        if !self.closing {
            return Ok(RetainedCloneStep::Progress(RetainedCloneProgress::default()));
        }
        if !self.retirement.is_empty() {
            let turn=self.retirement.step_granted(grant);self.last_work=turn.work;
            return turn.result.map(|step| RetainedCloneStep::Progress(step.progress()));
        }
        let turn=self.retirement.begin_granted(&mut self.clone, grant);self.last_work=turn.work;
        if let Some(step) = turn.result? {
            return Ok(RetainedCloneStep::Progress(step.progress()));
        }
        if self.cloned {
            let demand=RetirementDemand { copy_bytes:size_of::<bool>(), depth:1, ..Default::default() };
            if !retained_clone_grant_admits(grant,demand) { return Ok(RetainedCloneStep::Progress(Default::default())); }
            self.cloned=false;
            self.last_work+=1;
            return Ok(RetainedCloneStep::Complete(RetainedCloneProgress { copied_items:1, copied_bytes:demand.copy_bytes, ..Default::default() }));
        }
        Ok(RetainedCloneStep::Complete(RetainedCloneProgress::default()))
    }

    fn terminal_is_empty(&self) -> bool {
        self.closing && self.clone.is_none() && self.retirement.is_empty() && !self.cloned
    }
}

/// ✏️ Decides, inverts and applies one cold mutation in place on the owned post projection in one funded turn.
pub struct ColdReplayEdit<S: 'static, M: 'static> {
    last_work:u64,
    operation_index: u32,
    applied: bool,
    failed: bool,
    closing: bool,
    closed: bool,
    marker: PhantomData<fn() -> (S, M)>,
}

impl<S: 'static, M: 'static> Default for ColdReplayEdit<S, M> {
    fn default() -> Self {
        Self { last_work:0,operation_index: 0, applied: false, failed: false, closing: false, closed: false, marker: PhantomData }
    }
}

impl<S: Clone + RetireOwned + Send + Sync + 'static, M: Mutation<S> + ColdMutation<S> + RetireOwned + Send + Sync + 'static> SemanticEdit<S, M> for ColdReplayEdit<S, M> {
    fn birth_work_demand(&self)->u64{1}
    fn last_turn_work(&self)->u64{self.last_work}
    fn birth_demand(&self) -> RetirementDemand {
        replay_header::<Self>()
    }

    fn begin(&mut self, grant: RetainedCloneGrant) -> Result<RetainedCloneProgress, ValueError> {
        self.last_work=0;
        let demand = self.birth_demand();
        if !retained_clone_grant_admits(grant, demand) {
            return Err(fault("cold replay edit birth not funded"));
        }
        self.last_work=1;
        Ok(RetainedCloneProgress { copied_items: 1, copied_bytes: demand.copy_bytes, ..Default::default() })
    }

    fn post_mode(&self) -> PagedOneItemPostMode {
        PagedOneItemPostMode::Mutable
    }

    fn demands(&self, _before: RetainedCloneRef<'_, S>, _input: RetainedCloneRef<'_, M>, _post: PagedOneItemPostRead<'_, S>, _body: usize) -> Result<RetirementDemand, ValueError> {
        Ok(RetirementDemand { copy_bytes: size_of::<S>(), capacity_bytes: size_of::<M>(), depth: 1, ..Default::default() })
    }

    fn advance(&mut self, before: RetainedCloneRef<'_, S>, input: RetainedCloneRef<'_, M>, post: PagedOneItemPost<'_, S>, _body: usize, grant: RetainedCloneGrant, receiving: &mut semio_framework_value_native_owned::native_decoding::NativeDecodeControl<'_>) -> Result<SemanticEditStep<M>, ValueError> {
        self.last_work=0;
        let observed_before=receiving.actual_work();
        let result=(||{
        let demand = RetirementDemand { copy_bytes: size_of::<S>(), capacity_bytes: size_of::<M>(), depth: 1, ..Default::default() };
        if self.failed {
            return Err(fault("cold replay edit refused after a failed in-place application"));
        }
        if self.closing || self.applied || !retained_clone_grant_admits(grant, demand) {
            return Ok(SemanticEditStep::Progress(RetainedCloneProgress::default()));
        }
        if !receiving.prepare_work(4)?{return Ok(SemanticEditStep::Progress(Default::default()));}
        let post=post.mutable()?;
        receiving.admit_turn_capacity(grant.maximum_capacity_bytes)?;
        receiving.charge(demand.capacity_bytes)?;
        let mutation = input.get();
        self.last_work+=1;let outcome = ColdMutation::diff(mutation, before.get()).stamp_op_index(self.operation_index);
        let (outcome, admitted) = match outcome.admit() {
            semio_os_spr::MutationAdmission::Admitted(outcome) => (outcome, true),
            semio_os_spr::MutationAdmission::Refused(outcome) => (outcome, false),
        };
        let (diff, messages) = outcome.into_parts();
        let progress = RetainedCloneProgress { copied_items: 1, copied_bytes: demand.copy_bytes, retained_capacity_bytes: demand.capacity_bytes, ..Default::default() };
        if !admitted {
            self.last_work+=1;
            <M::ReferenceDiff as MutationDiff<S>>::retire_cold(diff);
            self.applied = true;
            return Ok(SemanticEditStep::Refused(progress, messages));
        }
        self.failed = true;
        self.last_work+=1;
        let applied = apply_diff_cold_into(&diff, post);
        self.last_work+=1;
        <M::ReferenceDiff as MutationDiff<S>>::retire_cold(diff);
        applied.map_err(|_| ValueError::literal(ValueRefusalKind::InvalidValue, "cold replay mutation could not produce its post root"))?;
        self.last_work+=1;let inverse = ColdMutation::inverse(mutation, before.get())?;
        self.failed = false;
        self.applied = true;
        Ok(SemanticEditStep::Complete(progress, inverse, messages, false))
        })();
        let observed=receiving.actual_work().saturating_sub(observed_before);
        let work=semio_os_store_one_item::original_one_item_turn_work(self.last_work,observed,self.last_work!=0)?;
        self.last_work=work;
        if work>observed{receiving.commit_work(work-observed)?;}
        result
    }

    fn begin_close(&mut self) {
        self.closing = true;
    }

    fn close_demand(&self, _factory: &dyn ArtifactOwnedValueRetirementFactory<M>, _body: usize) -> Result<RetirementDemand, ValueError> {
        Ok(replay_header::<Self>())
    }

    fn close_work_demand(&self,_factory:&dyn ArtifactOwnedValueRetirementFactory<M>,_body:usize)->Result<u64,ValueError>{Ok(u64::from(self.closing&&!self.closed))}
    fn close_step(&mut self, _factory: &dyn ArtifactOwnedValueRetirementFactory<M>, grant: RetainedCloneGrant) -> Result<RetainedCloneStep, semio_framework_value::OriginalRetirementFailure> {
        self.last_work=0;
        let demand = replay_header::<Self>();
        if !self.closing || !retained_clone_grant_admits(grant, demand) {
            return Ok(RetainedCloneStep::Progress(RetainedCloneProgress::default()));
        }
        if self.closed{return Ok(RetainedCloneStep::Complete(Default::default()));}
        self.closed = true;self.last_work=1;
        Ok(RetainedCloneStep::Complete(RetainedCloneProgress { copied_items: 1, copied_bytes: demand.copy_bytes, ..Default::default() }))
    }

    fn terminal_is_empty(&self) -> bool {
        self.closing && self.closed
    }
}

/// 🪢 The cold sibling of native replay: any family with explicit cold reference semantics replays its history through the shared native producer.
impl<S, M> ArtifactReplayPreparationFactory<S, M> for ColdMutationApplyFactory<S, M>
where
    S: Clone + RetireOwned + Send + Sync + 'static,
    M: Mutation<S> + ColdMutation<S> + ArtifactCanonicalJsonTree + RetireOwned + Send + Sync + 'static,
{
    fn preparation_birth_demand(&self) -> RetirementDemand {
        type Producer<S, M> = NativeSemanticPreparation<S, M, ColdReplayProjection<S>, ColdReplayEdit<S, M>>;
        RetirementDemand { copy_bytes: size_of::<ArtifactReplayPreparationRequest<S, M>>() + size_of::<Producer<S, M>>(), capacity_bytes: size_of::<Producer<S, M>>(), depth: 1, ..Default::default() }
    }

    fn begin(&self, request: ArtifactReplayPreparationRequest<S, M>, grant: RetainedCloneGrant) -> Result<(Box<dyn ArtifactReplayPreparation<S, M>>, RetainedCloneProgress), (ValueError, ArtifactReplayPreparationRequest<S, M>)> {
        let demand = ArtifactReplayPreparationFactory::preparation_birth_demand(self);
        if !retained_clone_grant_admits(grant, demand) {
            return Err((ValueError::literal(ValueRefusalKind::WorkLimit, "cold replay producer birth not funded"), request));
        }
        let projection = self.bounds.map_or_else(ColdReplayProjection::<S>::default, |bounds| ColdReplayProjection::<S>::bounded(bounds.snapshot_bytes, bounds.maximum_bytes));
        let edit = ColdReplayEdit::<S, M> { operation_index: request.context.operation_index, ..Default::default() };
        let producer = NativeSemanticPreparation::new(request, projection, edit);
        Ok((Box::new(producer), RetainedCloneProgress { copied_items: 1, copied_bytes: demand.copy_bytes, retained_capacity_bytes: demand.capacity_bytes, ..Default::default() }))
    }
}


/// 🧫️ Runs the real post-handoff metadata boundary without constructing a replacement projection owner.
#[cfg(feature="test-support")]
pub fn cold_projection_metadata_case(cloned:bool,grant:RetainedCloneGrant)->Result<(RetainedCloneProgress,u64,bool),ValueError>{
 let mut owner=ColdReplayProjection::<u8>{cloned,closing:true,last_work:99,..Default::default()};
 let step=owner.close_step(grant)?;
 Ok((step.progress(),owner.last_turn_work(),owner.terminal_is_empty()))
}
