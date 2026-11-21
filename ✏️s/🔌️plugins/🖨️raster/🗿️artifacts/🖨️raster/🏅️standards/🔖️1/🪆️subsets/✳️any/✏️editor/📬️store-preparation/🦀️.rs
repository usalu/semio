//! 📬️ Raster editor store preparation: the preparation factory and its one item publication, the apply fuel and the owner release laws.

use super::*;
use crate::modes::edit;
use semio_s_artifact_raster_raster_mutations::RasterMutation;
use semio_s_artifact_raster_raster_model::snapshot::RasterSnapshot;
use semio_framework_job::{RetainedCloneGrant, RetainedCloneProgress, RetainedCloneStep};
use semio_framework_value::{OriginalReceivingPrimary, OriginalRetirementPoll, RetainedAllocationEffect, RetirementPollExit, ValueError, ValueRefusalKind};
use semio_framework_value::retained_clone::retained_clone_grant_admits;
use semio_framework_value::native_decoding::{NativeRetirementDecodeControl, construction::NativeRecoveryLoan};
use store::os_store::one_item_publication::ArtifactStoreOneItemPreparation;
use store::snapshot_clone_preparation::{OriginalPublicationAssembly,OriginalPublicationInputs};
#[path="🔁️replay/🦀️.rs"]mod replay;

//#region 📬️StorePreparation
pub(super) type RasterRetirementDemand = semio_framework_value::RetirementDemand;

fn raster_preparation_fault(message: &'static str) -> semio_framework_value::ValueError {
    semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated, message)
}

pub(super) fn raster_preparation_nested(mut demand: RasterRetirementDemand) -> Result<RasterRetirementDemand, semio_framework_value::ValueError> {
    demand.depth = demand.depth.checked_add(1).ok_or_else(|| semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::DepthLimit, "preparation close depth overflow"))?;
    Ok(demand)
}

fn raster_preparation_factory_demand() -> RasterRetirementDemand {
    RasterRetirementDemand { copy_bytes: size_of::<std::sync::Arc<dyn semio_framework_value::FactoryRetirement>>(), depth: 1, ..Default::default() }
}

/// 📬️ The document lane's one-item retained preparation. Without it every route declaring
/// `ArtifactToolPublicationLane::Artifact` is registered with an unsupported publication contract and
/// stays dispatch-dead, no matter how it is classified.
#[derive(semio_framework_value::FactoryPayloadRetirement)]
pub(super) struct RasterStorePreparationFactory;

struct RasterStorePreparation {
    pub(super) base: std::mem::ManuallyDrop<Option<store::os_store::snapshot_read::SnapshotRead<RasterSnapshot>>>,
    pub(super) mutation: std::mem::ManuallyDrop<Option<RasterMutation>>,
    post: std::mem::ManuallyDrop<Option<RasterSnapshot>>,
    inverse: std::mem::ManuallyDrop<Option<Vec<RasterMutation>>>,
    authority: std::mem::ManuallyDrop<Option<std::sync::Arc<store::os_store::one_item_publication::ArtifactStoreOneItemLiveAuthority>>>,
    assembly:OriginalPublicationAssembly<RasterSnapshot,RasterMutation>,
    assembling:bool,
    seal_base:store::os_store::one_item::ArtifactStoreOneItemCheckpoint,
    /// 🧮 The clone-free stepwise apply (`RasterOneItemApply`), live from the first `advance` until
    /// the post snapshot is handed over, or closed through its own retirement on cancel/fault.
    pub(super) apply: std::mem::ManuallyDrop<Option<semio_s_artifact_raster_raster_host::RasterOneItemApply>>,
    mutation_retirement: std::mem::ManuallyDrop<Option<std::sync::Arc<dyn store::ArtifactOwnedValueRetirementFactory<RasterMutation>>>>,
    snapshot_retirement: std::mem::ManuallyDrop<Option<std::sync::Arc<dyn store::SnapshotRetirementFactory<RasterSnapshot>>>>,
    pub(super) active: std::mem::ManuallyDrop<Option<Box<dyn store::ErasedSnapshotRetirement>>>,
    factories: std::mem::ManuallyDrop<[Option<semio_framework_value::FactoryAuthority>; 2]>,
    pub(super) checkpoint: store::os_store::one_item::ArtifactStoreOneItemCheckpoint,
    pub(super) cancelled: bool,
    pub(super) closing: bool,
    last_work:u64,
}

/// ⛽️ Candidate fuel units one store grant buys — each unit is one bounded owned-value step (a field
/// clone, a page shift), so a two-layer document lands within a handful of grants.
const RASTER_ONE_ITEM_APPLY_FUEL: u64 = 256;

impl RasterStorePreparation {
    pub(super) fn close_demands(&self, body: usize) -> Result<RasterRetirementDemand, semio_framework_value::ValueError> {
        if let Some(active) = self.active.as_ref() {
            return raster_preparation_nested(store::os_store::snapshot_read::artifact_retirement_box_demands(active, body)?);
        }
        if let Some(demand)=self.assembly.close_demands(body)?{return raster_preparation_nested(demand);}
        if let Some(apply) = self.apply.as_ref() {
            return if apply.terminal_is_empty() {
                Ok(RasterRetirementDemand { copy_bytes: size_of::<Option<semio_s_artifact_raster_raster_host::RasterOneItemApply>>(), depth: 1, ..Default::default() })
            } else {
                raster_preparation_nested(apply.close_demands(body)?)
            };
        }
        if self.mutation.is_some() {
            return raster_preparation_nested(store::os_store::snapshot_read::artifact_retirement_owned_birth_demands(&self.mutation)?);
        }
        if self.post.is_some() {
            return raster_preparation_nested(store::os_store::snapshot_read::artifact_retirement_owned_birth_demands(&self.post)?);
        }
        if self.inverse.is_some(){return raster_preparation_nested(store::os_store::snapshot_read::artifact_retirement_owned_birth_demands(&self.inverse)?);}
        if self.base.is_some() {
            return raster_preparation_nested(store::os_store::snapshot_read::artifact_retirement_owned_birth_demands(&self.base)?);
        }
        if let Some(authority) = self.authority.as_ref() {
            let birth = authority.retirement_birth_demand();
            return Ok(RasterRetirementDemand { capacity_bytes: birth.capacity_bytes, depth: birth.depth + 1, ..Default::default() });
        }
        if self.mutation_retirement.is_some() || self.snapshot_retirement.is_some() {
            return Ok(raster_preparation_factory_demand());
        }
        self.factories.iter().find_map(Option::as_ref).map_or(Ok(Default::default()), |factory| raster_preparation_nested(factory.demands(body)?))
    }
}

impl store::os_store::one_item_publication::ArtifactStoreOneItemPreparationFactory<RasterSnapshot, RasterMutation> for RasterStorePreparationFactory {
    fn begin_batch_digest(&self, edit: &mut Option<Box<protocol::Edit<RasterMutation>>>, grant: RetainedCloneGrant) -> Result<Option<(Box<dyn store::ArtifactStoreBatchDigest<RasterMutation>>, RetainedCloneProgress)>, semio_framework_value::ValueError> {
        store::admit_artifact_batch_digest(edit, grant)
    }

    fn preflight(&self, mutation: &RasterMutation, lane: store::os_store::store_schema::HistoryLane) -> Result<store::os_store::one_item::ArtifactStoreOneItemFootprint, String> {
        if lane != store::os_store::store_schema::HistoryLane::Document {
            return Err("Raster Store preparation rejected its lane".into());
        }
        Ok(store::os_store::one_item::ArtifactStoreOneItemFootprint::for_leaf(mutation, store::os_store::one_item::ARTIFACT_STORE_ONE_ITEM_MAXIMUM_BYTES))
    }

    fn begin_demand(&self, _mutation: &RasterMutation, lane: store::os_store::store_schema::HistoryLane) -> Result<semio_framework_value::retained_clone::RetainedCloneBirthDemand, semio_framework_value::ValueError> {
        if lane != store::os_store::store_schema::HistoryLane::Document {
            return Err(semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::OwnershipLimit, "Raster Store preparation has no admissible lane"));
        }
        Ok(semio_framework_value::retained_clone::RetainedCloneBirthDemand { capacity_bytes: size_of::<RasterStorePreparation>(), depth: 1 })
    }

    fn begin_copy_byte_demand(&self, mutation: &RasterMutation, lane: store::os_store::store_schema::HistoryLane) -> Result<usize, semio_framework_value::ValueError> {
        self.begin_demand(mutation, lane)?;
        size_of::<store::os_store::one_item_publication::ArtifactStoreOneItemPreparationRequest<RasterSnapshot, RasterMutation, RasterMutation>>().checked_add(size_of::<RasterStorePreparation>()).ok_or_else(|| semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::OwnershipLimit, "original preparation header quote overflow"))
    }

    fn begin(
        &self,
        request: store::os_store::one_item_publication::ArtifactStoreOneItemPreparationRequest<RasterSnapshot, RasterMutation, RasterMutation>,
        grant: store::os_store::one_item::ArtifactStoreOneItemGrant,
    ) -> Result<(Box<dyn store::os_store::one_item_publication::ArtifactStoreOneItemPreparation<RasterSnapshot, RasterMutation>>, RetainedCloneProgress), (semio_framework_value::ValueError, store::os_store::one_item_publication::ArtifactStoreOneItemPreparationRequest<RasterSnapshot, RasterMutation, RasterMutation>)> {
        let demand = match self.begin_demand(&request.mutation, request.lane) { Ok(demand) => demand, Err(error) => return Err((error, request)) };
        let copy_bytes = match self.begin_copy_byte_demand(&request.mutation, request.lane) {
            Ok(bytes) => bytes,
            Err(error) => return Err((error, request)),
        };
        if grant.maximum_copy_bytes < copy_bytes {
            return Err((ValueError::literal(ValueRefusalKind::WorkLimit, "Raster preparation original header is unfunded"), request));
        }
        let mut progress = match demand.admit(grant.retained_grant()) {
            Ok(progress) => progress,
            Err(error) => return Err((error, request)),
        };
        progress.copied_bytes = copy_bytes;
        let item_count = semio_s_artifact_raster_raster_model::flatten_raster_layers(&request.base.get().layers).len().saturating_add(request.base.get().assets.len());
        if request.operation != request.authority.operation()
            || request.generation != request.authority.generation()
            || request.base_revision != request.authority.base_revision()
            || request.authority.actor().len() > store::os_store::one_item::ARTIFACT_STORE_ONE_ITEM_ID_BYTES
            || item_count > RASTER_RETAINED_WORK_ITEMS
        {
            return Err((raster_preparation_fault("Raster preparation original request refused"), request));
        }
        Ok((Box::new(RasterStorePreparation {
            base: std::mem::ManuallyDrop::new(Some(request.base)),
            mutation: std::mem::ManuallyDrop::new(Some(request.mutation)),
            post: std::mem::ManuallyDrop::new(None),
            inverse: std::mem::ManuallyDrop::new(None),
            authority: std::mem::ManuallyDrop::new(Some(request.authority)),
            assembly:OriginalPublicationAssembly::default(),
            assembling:false,
            seal_base:Default::default(),
            apply: std::mem::ManuallyDrop::new(None),
            mutation_retirement: std::mem::ManuallyDrop::new(Some(request.mutation_retirement)),
            snapshot_retirement: std::mem::ManuallyDrop::new(Some(request.snapshot_retirement)),
            active: std::mem::ManuallyDrop::new(None),
            factories: std::mem::ManuallyDrop::new(Default::default()),
            checkpoint: store::os_store::one_item::ArtifactStoreOneItemCheckpoint::default(),
            cancelled: false,
            closing: false,
            last_work:0,
        }), progress))
    }
}

impl ArtifactStoreOneItemPreparation<RasterSnapshot, RasterMutation> for RasterStorePreparation {
    fn advance(&mut self, _body: usize, grant: store::os_store::one_item::ArtifactStoreOneItemGrant, receiving: &mut semio_framework_value_native_owned::native_decoding::NativeDecodeControl<'_>) -> Result<store::os_store::one_item::ArtifactStoreOneItemPreparationStep, OriginalReceivingPrimary> {
        use semio_framework_value::retained_clone::RetainedCloneProgress;
        self.last_work=0;
        if !grant.permits_one() || self.cancelled || self.closing {
            return Ok(store::os_store::one_item::ArtifactStoreOneItemPreparationStep::Blocked);
        }
        if self.assembling{return self.advance_publication(grant,receiving);}
        if self.post.is_none() {
            let base = self.base.as_ref().ok_or_else(|| raster_preparation_fault("Raster preparation lost its exact base root"))?;
            let authority = self.authority.as_ref().ok_or_else(|| raster_preparation_fault("Raster preparation lost its Store authority"))?;
            let mutation = self.mutation.as_ref().ok_or_else(|| raster_preparation_fault("Raster preparation lost its mutation owner"))?;
            if self.apply.is_none(){let bytes=size_of::<semio_s_artifact_raster_raster_host::RasterOneItemApply>();if grant.maximum_copy_bytes<bytes||!receiving.prepare_work(1)?{return Ok(store::os_store::one_item::ArtifactStoreOneItemPreparationStep::Blocked);}*self.apply=Some(semio_s_artifact_raster_raster_host::RasterOneItemApply::new());self.last_work=1;let receipt=RetainedCloneProgress{copied_items:1,copied_bytes:bytes,..Default::default()};receiving.commit_work(1).map_err(|error|error.with_retained_progress(receipt))?;self.checkpoint.cursor=self.checkpoint.cursor.saturating_add(1);return Ok(store::os_store::one_item::ArtifactStoreOneItemPreparationStep::Progress(self.checkpoint,receipt));}
            let mut budget = RASTER_ONE_ITEM_APPLY_FUEL;
            let ((turn, stop_error), work) = loop {
                let attempt = receiving.with_effect_reported(grant.retained_grant(), budget, |receiving, effect| {
                    let mut stop_error = None;
                    let turn = self.apply.as_mut().expect("Raster apply was admitted").advance_controlled(
                        base.get(), mutation, authority.operation(), authority.generation(), effect,
                        || match receiving.checkpoint() {
                            Ok(()) => false,
                            Err(error) => { stop_error = Some(error); true }
                        },
                    );
                    (turn, stop_error)
                })?;
                if let Some(attempt) = attempt { break attempt; }
                if budget == 1 { return Ok(store::os_store::one_item::ArtifactStoreOneItemPreparationStep::Blocked); }
                budget /= 2;
            };
            self.last_work = u64::try_from(work).map_err(|_| raster_preparation_fault("Raster apply work exceeds Store receipt"))?;
            let spent = turn.result.as_ref().map(|(_, progress)| *progress).unwrap_or_else(|error| error.retained_progress());
            if let Some(error) = stop_error { return Err(error.with_retained_progress(spent).into()); }
            let (post, progress)=turn.result?;
            let receipt = progress;
            if progress!=RetainedCloneProgress::default()||self.last_work>0{self.checkpoint.cursor=self.checkpoint.cursor.saturating_add(1);self.checkpoint.completed_bytes=self.checkpoint.completed_bytes.saturating_add(progress.copied_bytes as u64);}
            let Some(post) = post else {
                return Ok(store::os_store::one_item::ArtifactStoreOneItemPreparationStep::Progress(self.checkpoint, receipt));
            };
            *self.post = Some(post.snapshot);
            *self.inverse = Some(post.inverse.ok_or_else(||raster_preparation_fault("Raster semantic candidate completed without its retained inverse"))?);
            self.assembling=true;
            drop(self.apply.take());
            return Ok(store::os_store::one_item::ArtifactStoreOneItemPreparationStep::Progress(self.checkpoint,receipt));
        }
        self.advance_publication(grant,receiving)
    }

    fn checkpoint(&self) -> store::os_store::one_item::ArtifactStoreOneItemCheckpoint {
        self.checkpoint
    }
    fn last_turn_work(&self)->u64{self.last_work}
    fn prepared(&self) -> Option<&store::os_store::one_item_publication::ArtifactStoreOneItemPrepared<RasterSnapshot, RasterMutation>> {
        self.assembly.sealer().and_then(|sealer|sealer.prepared())
    }
    fn take_prepared(&mut self) -> Option<store::os_store::one_item_publication::ArtifactStoreOneItemPrepared<RasterSnapshot, RasterMutation>> {
        self.assembly.sealer_mut().and_then(|sealer|sealer.take_prepared())
    }
    fn cancel(&mut self) {
        self.cancelled = true;
        if let Some(sealer)=self.assembly.sealer_mut(){sealer.cancel();}
    }
    fn has_pending_receiving_recovery(&self) -> bool { false }
    fn pending_receiving_demand(&self) -> Option<RasterRetirementDemand> { None }
    fn pending_receiving_recovery(&mut self) -> Option<NativeRecoveryLoan<'_>> { None }
    fn finish_receiving_recovery(&mut self, _: &mut NativeRetirementDecodeControl<'_, '_>) -> semio_framework_value::RetainedWorkOutcome<RetainedCloneProgress, OriginalReceivingPrimary> {
        semio_framework_value::RetainedWorkOutcome { result: Err(raster_preparation_fault("Raster preparation has no pending native receiving callback").into()), work: 0 }
    }
    fn begin_close(&mut self) {
        self.closing = true;
        if let Some(sealer)=self.assembly.sealer_mut(){sealer.begin_close();}
    }

    fn close_step(&mut self, effect: &mut RetainedAllocationEffect<'_>) -> OriginalRetirementPoll {
        let before = effect.performed_work();
        let result = self.close_ladder(effect);
        let poll = RetirementPollExit::settle(effect, result);
        self.last_work = u64::try_from(effect.performed_work().saturating_sub(before)).unwrap_or(u64::MAX);
        poll
    }

    fn next_close_copy_byte_demand(&self) -> Result<usize, semio_framework_value::ValueError> {
        Ok(self.close_demands(0)?.copy_bytes)
    }
    fn next_close_capacity_byte_demand(&self, body: usize) -> Result<usize, semio_framework_value::ValueError> {
        Ok(self.close_demands(body)?.capacity_bytes)
    }
    fn next_close_release_byte_demand(&self) -> Result<usize, semio_framework_value::ValueError> {
        Ok(self.close_demands(0)?.release_bytes)
    }
    fn next_close_depth_demand(&self) -> Result<usize, semio_framework_value::ValueError> {
        Ok(self.close_demands(0)?.depth)
    }

    fn terminal_is_empty(&self) -> bool {
        self.closing && self.raster_owners_released()
    }
}

impl RasterStorePreparation {
    fn close_ladder(&mut self, effect: &mut RetainedAllocationEffect<'_>) -> Result<RetainedCloneStep, RetirementPollExit> {
        let grant = effect.grant();
        let body = grant.maximum_copy_bytes;
        let empty = RetainedCloneProgress::default();
        if self.terminal_is_empty() {
            return Ok(RetainedCloneStep::Complete(empty));
        }
        if !self.closing || grant.maximum_items == 0 || effect.remaining_work_units()? == 0 {
            return Ok(RetainedCloneStep::Progress(empty));
        }
        let demand = self.close_demands(body)?;
        if grant.maximum_depth < demand.depth {
            return Err(ValueError::literal(ValueRefusalKind::DepthLimit, "preparation close exceeds original depth").into());
        }
        if !retained_clone_grant_admits(grant, demand) {
            return Ok(RetainedCloneStep::Progress(empty));
        }
        let child = RetainedCloneGrant { maximum_items: 1, maximum_depth: grant.maximum_depth - 1, ..grant };
        if self.active.is_some() {
            let step = RetirementPollExit::resume(store::os_store::snapshot_read::artifact_retirement_box_close_step(&mut self.active, body, &mut effect.nested(child)))?;
            return Ok(RetainedCloneStep::Progress(step.progress()));
        }
        if !self.assembly.terminal_is_empty() {
            return self.assembly.close_step(&mut effect.nested(child));
        }
        if let Some(apply) = self.apply.as_mut() {
            if apply.terminal_is_empty() {
                RetirementPollExit::pay(effect, 1)?;
                drop(self.apply.take());
                return Ok(RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: 1, copied_bytes: demand.copy_bytes, ..empty }));
            }
            let step = apply.close_step(&mut effect.nested(child)).result?;
            return Ok(RetainedCloneStep::Progress(step.progress()));
        }
        if self.mutation.is_some() {
            return Ok(store::os_store::snapshot_read::artifact_retirement_admit_owned(&mut self.mutation, &mut self.active, &mut effect.nested(child)).result?);
        }
        if self.post.is_some() {
            return Ok(store::os_store::snapshot_read::artifact_retirement_admit_owned(&mut self.post, &mut self.active, &mut effect.nested(child)).result?);
        }
        if self.inverse.is_some() {
            return Ok(store::os_store::snapshot_read::artifact_retirement_admit_owned(&mut self.inverse, &mut self.active, &mut effect.nested(child)).result?);
        }
        if self.base.is_some() {
            return Ok(store::os_store::snapshot_read::artifact_retirement_admit_owned(&mut self.base, &mut self.active, &mut effect.nested(child)).result?);
        }
        if self.authority.is_some() {
            if effect.actual().is_some() {
                return Err(raster_preparation_fault("Raster authority retirement allocation marker is occupied").into());
            }
            RetirementPollExit::pay(effect, 1)?;
            return match self.authority.take().expect("quoted Raster authority").retire(child) {
                Ok((owner, progress)) => {
                    *effect.actual() = Some(progress.retained_capacity_bytes);
                    *self.active = Some(owner);
                    Ok(RetainedCloneStep::Progress(progress))
                }
                Err((error, original)) => {
                    *self.authority = Some(original);
                    Err(error.into())
                }
            };
        }
        if self.mutation_retirement.is_some() || self.snapshot_retirement.is_some() {
            RetirementPollExit::pay(effect, 1)?;
            let (index, factory): (_, std::sync::Arc<dyn semio_framework_value::FactoryRetirement>) = match self.mutation_retirement.take() {
                Some(factory) => (0, factory),
                None => (1, self.snapshot_retirement.take().expect("quoted Raster snapshot factory")),
            };
            self.factories[index] = Some(semio_framework_value::FactoryAuthority::new(factory));
            return Ok(RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: 1, copied_bytes: demand.copy_bytes, ..empty }));
        }
        if let Some(slot) = self.factories.iter_mut().find(|slot| slot.is_some()) {
            let factory = slot.as_mut().expect("quoted Raster factory");
            let step = RetirementPollExit::resume(factory.step(body, &mut effect.nested(child)))?;
            let step = semio_framework_value::retained_clone::admit_retained_clone_close(child, step, factory.terminal_is_empty(), "original preparation factory close")?;
            if factory.terminal_is_empty() {
                *slot = None;
            }
            return Ok(RetainedCloneStep::Progress(step.progress()));
        }
        Ok(RetainedCloneStep::Complete(empty))
    }

    fn advance_publication(&mut self,grant:store::os_store::one_item::ArtifactStoreOneItemGrant,receiving:&mut semio_framework_value_native_owned::native_decoding::NativeDecodeControl<'_>)->Result<store::os_store::one_item::ArtifactStoreOneItemPreparationStep,OriginalReceivingPrimary>{
        use store::os_store::one_item::ArtifactStoreOneItemPreparationStep as Step;
        if let Some(sealer)=self.assembly.sealer_mut(){
            let result=sealer.advance(grant,receiving);self.last_work=sealer.last_turn_work();
            let step=result?;let (checkpoint,receipt,done)=match step{Step::Blocked=>return Ok(Step::Blocked),Step::Progress(checkpoint,receipt)=>(checkpoint,receipt,false),Step::Prepared(checkpoint,receipt)=>(checkpoint,receipt,true)};
            self.checkpoint=store::os_store::one_item::ArtifactStoreOneItemCheckpoint{cursor:self.seal_base.cursor.saturating_add(checkpoint.cursor),completed_bytes:self.seal_base.completed_bytes.saturating_add(checkpoint.completed_bytes),..checkpoint};
            return Ok(if done{Step::Prepared(self.checkpoint,receipt)}else{Step::Progress(self.checkpoint,receipt)});
        }
        let work = self.assembly.advance_work_demand(grant.maximum_copy_bytes)?;
        let Some((result, actual)) = receiving.with_effect_reported(grant.retained_grant(), work, |_, effect| {
            self.assembly.advance(OriginalPublicationInputs {
                forward: &mut self.mutation, post: &mut self.post, inverse: &mut self.inverse,
                authority: &mut self.authority, mutations: &mut self.mutation_retirement, snapshots: &mut self.snapshot_retirement,
            }, effect)
        })? else { return Ok(Step::Blocked); };
        self.last_work = u64::try_from(actual).map_err(|_| raster_preparation_fault("Raster assembly work exceeds Store receipt"))?;
        let Some(receipt) = result? else { return Ok(Step::Blocked); };
        self.checkpoint.cursor=self.checkpoint.cursor.saturating_add(1);self.checkpoint.completed_bytes=self.checkpoint.completed_bytes.saturating_add(receipt.copied_bytes as u64);
        if self.assembly.sealer().is_some(){self.seal_base=self.checkpoint;}
        Ok(Step::Progress(self.checkpoint,receipt))
    }
    fn raster_owners_released(&self) -> bool {
        self.active.is_none() && self.factories.iter().all(Option::is_none) && self.mutation_retirement.is_none() && self.snapshot_retirement.is_none() && self.apply.is_none() && self.post.is_none() && self.inverse.is_none() && self.base.is_none() && self.mutation.is_none() && self.authority.is_none() && self.assembly.terminal_is_empty()
    }
}

impl Drop for RasterStorePreparation {
    fn drop(&mut self) {
        assert!(std::thread::panicking() || self.raster_owners_released(), "preparation must retain original owners until supplied-grant terminal closure");
    }
}
//#endregion 📬️StorePreparation
