//! 🏭️ One-item preparation doubles and the published-root witnesses.
//!
//! 🧭️ Moved from the former store unit test module; the items are `pub` because the test crates of the Store use them through the public API only.

use semio_framework_os_store_testing_fixtures::prelude::*;
use semio_framework_os_store_testing_fixtures::*;
use crate::*;
use semio_framework_os_store_testing_fixtures::demo::{AddN, AssignN, DeleteN, DemoMutation, SetN};
use semio_framework_os_store_testing_fixtures::lossy::{LossyMutation, SetN as LossySetN};
use semio_framework_os_store_testing_fixtures::severity::{SetErrorN, SetFatalN, SetN as SeveritySetN, SetWarningN, SeverityMutation};
use semio_framework_os_store_testing_fixtures::timestamped::{SetN as TimestampedSetN, TimestampedMutation};
use semio_framework_os_store_testing_fixtures::validated::{AssignN as ValidatedAssignN, SetN as ValidatedSetN, ValidatedMutation};
use pack::snapshot::ArtifactSqliteSnapshot as _;
use semio_framework_os_store_snapshot_clone::RetainedCloneEdit;
use semio_framework_os_store_snapshot_clone::RetainedCloneEditCursor;
use semio_framework_os_store_snapshot_clone::RetainedCloneEditStep;
use semio_framework_os_store_snapshot_clone::RetainedClonePreparationFactory;
use semio_framework_value::retained_clone::{RetainedCloneGrant, RetainedCloneProgress, RetainedCloneRef};
use semio_framework_pack_json_retained_schema::testing::{OWNED_SCHEMA_TEST_FIELDS, owned_schema_test_cursor, drive_owned_schema, schema_test_close_grant};
use semio_framework_value::testing::physical_test_close_grant;
use crate::store::ArtifactStore;

pub struct DemoOneItemPreparationFactory {
    pub footprint: ArtifactStoreOneItemFootprint,
    pub published_root: Arc<DemoPublishedRootWitness>,
    pub snapshot_retirement: Arc<dyn SnapshotRetirementFactory<DemoSnapshot>>,
    pub mutation_retirement: Arc<dyn ArtifactOwnedValueRetirementFactory<DemoMutation>>,
    pub forge_digest: bool,
    pub stamped_clock: Option<HybridLogicalTimestamp>,
    pub stamped_mutation_id: Option<MutationId>,
}

pub struct DemoOneItemFactoryClose {
    pub published:Option<Arc<DemoPublishedRootWitness>>,
    pub published_close:DemoPublishedRootClose,
    pub identity:Option<String>,
    pub issuers:semio_framework_value::FactoryChildTickets<2>,
}

impl semio_framework_value::FactoryPayloadRetirement for DemoOneItemPreparationFactory {
    type CloseState=DemoOneItemFactoryClose;
    fn close_state_birth_bytes(&self) -> usize { 0 }
    fn close_state_constructor_depth(&self)->usize { 0 }
    fn close_state_constructor_copy_bytes(&self)->usize { 0 }
    fn close_state_preparation_demands(&self,state:&Self::CloseState,body:usize)->Result<semio_framework_value::RetirementDemand,ValueError> {
        let slot=match state.issuers.prepared{0=>&state.issuers.slots[0],1=>&state.issuers.slots[1],_=>return Ok(Default::default())};
        if slot.preparation_is_complete(){return Ok(semio_framework_value::RetirementDemand{copy_bytes:0,depth:1,..Default::default()})}
        slot.preparation_demands(if state.issuers.prepared==0{self.snapshot_retirement.as_ref()}else{self.mutation_retirement.as_ref()},body)
    }
    fn prepare_close_state_step(&self,state:&mut Self::CloseState,grant:RetainedCloneGrant)->Result<RetainedCloneStep,ValueError> {
        let index=state.issuers.prepared;
        if index>=2{return Ok(RetainedCloneStep::Complete(Default::default()))}
        let slot=&mut state.issuers.slots[index];
        if slot.preparation_is_complete(){
            if grant.maximum_items==0||grant.maximum_depth==0{return Ok(RetainedCloneStep::Progress(Default::default()))}
            state.issuers.prepared+=1;
            return Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,..Default::default()}));
        }
        let step=if index==0{slot.prepare_step(self.snapshot_retirement.as_ref(),||Arc::clone(&self.snapshot_retirement)as Arc<dyn semio_framework_value::FactoryRetirement>,grant)?}
        else{slot.prepare_step(self.mutation_retirement.as_ref(),||Arc::clone(&self.mutation_retirement)as Arc<dyn semio_framework_value::FactoryRetirement>,grant)?};
        Ok(RetainedCloneStep::Progress(step.progress()))
    }
    fn close_state_preparation_is_complete(state:&Self::CloseState)->bool { state.issuers.prepared==2 }
    fn prepare_close_state(&self)->Self::CloseState { DemoOneItemFactoryClose { published:None,published_close:DemoPublishedRootClose { weak:None,root:None },identity:None,issuers:semio_framework_value::FactoryChildTickets{slots:std::array::from_fn(|_|semio_framework_value::FactoryChildSlot::empty()),prepared:0} } }
    fn transfer_payload(value:Self,state:&mut Self::CloseState) {
        let Self { published_root,stamped_mutation_id,snapshot_retirement,mutation_retirement,.. }=value;
        assert!(Self::close_state_preparation_is_complete(state));drop((snapshot_retirement,mutation_retirement));
        state.published=Some(published_root);state.identity=stamped_mutation_id.map(|identity|identity.0);
    }
    fn close_state_demands(state:&Self::CloseState,body:usize)->Result<semio_framework_value::RetirementDemand,ValueError> {
        use semio_framework_value::FactoryPayloadRetirement as _;
        if let Some(original)=state.published.as_ref() { return Ok(semio_framework_value::RetirementDemand { release_bytes:if Arc::strong_count(original)==1 { semio_framework_value::factory_arc_birth_bytes::<DemoPublishedRootWitness>() } else { 0 },depth:1,..Default::default() }); }
        if !DemoPublishedRootWitness::close_state_terminal_is_empty(&state.published_close) {
            let mut demand=DemoPublishedRootWitness::close_state_demands(&state.published_close,body)?;
            demand.depth=demand.depth.checked_add(1).ok_or_else(||ValueError::literal(semio_framework_value::ValueRefusalKind::DepthLimit,"inline witness depth overflow"))?;return Ok(demand);
        }
        if let Some(identity)=state.identity.as_ref() { return Ok(semio_framework_value::RetirementDemand { release_bytes:identity.capacity(),depth:1,..Default::default() }); }
        if !state.issuers.terminal_is_empty() { let mut demand=state.issuers.demands(body)?;demand.depth=demand.depth.checked_add(1).ok_or_else(||ValueError::literal(semio_framework_value::ValueRefusalKind::DepthLimit,"captured issuer depth overflow"))?;return Ok(demand); }
        Ok(Default::default())
    }
    fn close_state_step(state:&mut Self::CloseState,grant: RetainedCloneGrant)->Result<RetainedCloneStep,ValueError> {
        use semio_framework_value::FactoryPayloadRetirement as _;
        let empty=RetainedCloneProgress::default();
        if Self::close_state_terminal_is_empty(state) { return Ok(RetainedCloneStep::Complete(empty)); }
        if grant.maximum_items==0 { return Ok(RetainedCloneStep::Progress(empty)); }
        let demand=Self::close_state_demands(state,grant.maximum_copy_bytes)?;
        if grant.maximum_depth<demand.depth { return Err(ValueError::literal(semio_framework_value::ValueRefusalKind::DepthLimit,"inline factory ownership exceeds supplied depth")); }
        if grant.maximum_release_bytes<demand.release_bytes { return Ok(RetainedCloneStep::Progress(empty)); }
        if let Some(original)=state.published.as_ref() {
            if Arc::weak_count(original)!=0 { return Err(ValueError::literal(semio_framework_value::ValueRefusalKind::OwnershipLimit,"original factory witness retains a foreign Weak")); }
            let released_bytes=if let Some(value)=Arc::into_inner(state.published.take().expect("funded original witness remains present")) { DemoPublishedRootWitness::transfer_payload(value,&mut state.published_close);semio_framework_value::factory_arc_birth_bytes::<DemoPublishedRootWitness>() } else { 0 };
            return Ok(RetainedCloneStep::Progress(RetainedCloneProgress { copied_items:1,released_bytes,..empty }));
        }
        if !DemoPublishedRootWitness::close_state_terminal_is_empty(&state.published_close) {
            let step=DemoPublishedRootWitness::close_state_step(&mut state.published_close,RetainedCloneGrant { maximum_items:1,maximum_depth:grant.maximum_depth-1,..grant })?;
            return Ok(RetainedCloneStep::Progress(step.progress()));
        }
        if let Some(identity)=state.identity.take() { let released_bytes=identity.capacity();drop(identity);return Ok(RetainedCloneStep::Progress(RetainedCloneProgress { copied_items:1,released_bytes,..empty })); }
        if !state.issuers.terminal_is_empty() { let child=RetainedCloneGrant { maximum_items:1,maximum_depth:grant.maximum_depth-1,..grant };let step=state.issuers.close_step(child)?;semio_framework_value::retained_clone::admit_retained_clone_close(child,step,state.issuers.terminal_is_empty(),"captured original preparation issuer")?;return Ok(RetainedCloneStep::Progress(step.progress())); }
        Ok(RetainedCloneStep::Complete(empty))
    }
    fn close_state_terminal_is_empty(state:&Self::CloseState)->bool {
        use semio_framework_value::FactoryPayloadRetirement as _;
        state.published.is_none()&&DemoPublishedRootWitness::close_state_terminal_is_empty(&state.published_close)&&state.identity.is_none()&&state.issuers.terminal_is_empty()
    }
}

impl DemoOneItemPreparationFactory {
    pub fn admissible() -> Self {
        Self { footprint: ArtifactStoreOneItemFootprint::for_leaf(&DemoMutation::SetN(SetN { n: 0 }), 512), published_root: Arc::new(DemoPublishedRootWitness::new()), snapshot_retirement: Arc::new(DemoSnapshotRetirementFactory), mutation_retirement: Arc::new(DemoMutationRetirementFactory), forge_digest: false, stamped_clock: None, stamped_mutation_id: None }
    }

    /// 🕰️ A server-derived publication clock, as a durable committer declares it.
    pub fn stamped(clock: HybridLogicalTimestamp) -> Self {
        Self { stamped_clock: Some(clock), ..Self::admissible() }
    }

    /// 🔏 A server-derived clock AND identity, as the Hub's approval committer declares both: the
    /// pair its canonical command was hashed over before the Store was ever reached.
    pub fn stamped_identity(clock: HybridLogicalTimestamp, mutation_id: &str) -> Self {
        Self { stamped_clock: Some(clock), stamped_mutation_id: Some(MutationId(mutation_id.to_string())), ..Self::admissible() }
    }

    /// 🚫️ The repo-wide app-side defect, as a fixture: a durable preflight that declares ONE work item
    /// for a point-invertible mutation. `work_items` counts staged edit ROWS, so the item's forward row
    /// plus its inverse row overrun the declaration and the fold refuses
    /// (ticket 26/09/09/PROCEDURAL-3D-END-TO-END).
    pub fn under_declared() -> Self {
        Self { footprint: ArtifactStoreOneItemFootprint { work_items: 1, retained_bytes: 512 }, ..Self::admissible() }
    }

    pub fn forged_digest() -> Self {
        Self { forge_digest: true, ..Self::admissible() }
    }
}


pub struct DemoOneItemPreparation {
    pub base: Option<SnapshotRead<DemoSnapshot>>,
    pub mutation: Option<DemoMutation>,
    pub authority: Option<Arc<ArtifactStoreOneItemLiveAuthority>>,
    pub prepared: Option<ArtifactStoreOneItemPrepared<DemoSnapshot, DemoMutation>>,
    pub semantic_refusal: std::mem::ManuallyDrop<Option<semio_framework_os_spr::MutationApplyError>>,
    pub pending_edit: Option<Edit<DemoMutation>>,
    pub pending_post: Option<Arc<DemoSnapshot>>,
    pub sealer: Option<ArtifactStoreOneItemSealer<DemoSnapshot, DemoMutation>>,
    pub checkpoint: ArtifactStoreOneItemCheckpoint,
    pub published_root: Option<Arc<DemoPublishedRootWitness>>,
    pub snapshot_retirement: Option<Arc<dyn SnapshotRetirementFactory<DemoSnapshot>>>,
    pub mutation_retirement: Option<Arc<dyn ArtifactOwnedValueRetirementFactory<DemoMutation>>>,
    pub active_retirement: Option<Box<dyn ErasedSnapshotRetirement>>,
    pub closing_factories: [Option<semio_framework_value::FactoryAuthority>;3],
    pub forge_digest: bool,
    pub stamped_mutation_id: Option<MutationId>,
    pub cancelled: bool,
    pub closing: bool,
}

impl ArtifactStoreOneItemPreparationFactory<DemoSnapshot, DemoMutation> for DemoOneItemPreparationFactory {
    fn begin_batch_digest(&self,edit:&mut Option<Box<Edit<DemoMutation>>>,grant:RetainedCloneGrant)->Result<Option<(Box<dyn ArtifactStoreBatchDigest<DemoMutation>>,RetainedCloneProgress)>,ValueError>{admit_artifact_batch_digest(edit,grant)}
    fn stamped_clock(&self) -> Option<HybridLogicalTimestamp> {
        self.stamped_clock
    }

    fn stamped_mutation_id(&self) -> Option<&MutationId> {
        self.stamped_mutation_id.as_ref()
    }

    fn preflight(&self, _mutation: &DemoMutation, _lane: HistoryLane) -> Result<ArtifactStoreOneItemFootprint, String> {
        Ok(self.footprint)
    }

    fn begin_demand(&self, _: &DemoMutation, _: HistoryLane) -> Result<semio_framework_value::retained_clone::RetainedCloneBirthDemand, ValueError> {
        let identity_bytes = self.stamped_mutation_id.as_ref().map_or(0, |identity| identity.0.len());
        let capacity_bytes = std::mem::size_of::<DemoOneItemPreparation>().checked_add(identity_bytes).ok_or_else(|| ValueError::literal(semio_framework_value::ValueRefusalKind::OwnershipLimit, "original preparation frame and identity capacity overflow"))?;
        Ok(semio_framework_value::retained_clone::RetainedCloneBirthDemand { capacity_bytes, depth: 1 })
    }

    fn begin(&self, request: ArtifactStoreOneItemPreparationRequest<DemoSnapshot, DemoMutation, DemoMutation>, grant: ArtifactStoreOneItemGrant) -> Result<(Box<dyn ArtifactStoreOneItemPreparation<DemoSnapshot, DemoMutation>>, RetainedCloneProgress), (ValueError, ArtifactStoreOneItemPreparationRequest<DemoSnapshot, DemoMutation, DemoMutation>)> {
        let demand = match self.begin_demand(&request.mutation, request.lane) { Ok(demand) => demand, Err(error) => return Err((error, request)) };
        let mut progress = match demand.admit(grant.retained_grant()) { Ok(progress) => progress, Err(error) => return Err((error, request)) };
        let copied_bytes = self.stamped_mutation_id.as_ref().map_or(0, |identity| identity.0.len());
        if grant.maximum_copy_bytes < copied_bytes { return Err((ValueError::literal(semio_framework_value::ValueRefusalKind::WorkLimit, "original stamped identity copy is not funded"), request)); }
        progress.copied_bytes = copied_bytes;
        let stamped_mutation_id = self.stamped_mutation_id.clone();
        progress.retained_capacity_bytes = std::mem::size_of::<DemoOneItemPreparation>() + stamped_mutation_id.as_ref().map_or(0, |identity| identity.0.capacity());
        Ok((Box::new(DemoOneItemPreparation {
            base: Some(request.base),
            mutation: Some(request.mutation),
            authority: Some(request.authority),
            prepared: None,
            semantic_refusal: std::mem::ManuallyDrop::new(None),
            pending_edit: None,
            pending_post: None,
            sealer: None,
            checkpoint: ArtifactStoreOneItemCheckpoint::default(),
            published_root: Some(self.published_root.clone()),
            snapshot_retirement: Some(request.snapshot_retirement),
            mutation_retirement: Some(request.mutation_retirement),
            active_retirement: None,
            closing_factories: [None,None,None],
            forge_digest: self.forge_digest,
            stamped_mutation_id,
            cancelled: false,
            closing: false,
        }), progress))
    }
}

impl DemoOneItemPreparation {
    pub fn close_demands(&self,body:usize)->Result<semio_framework_value::RetirementDemand,ValueError> {
        use semio_framework_value::RetirementDemand;
        let nested=|mut demand:RetirementDemand|->Result<RetirementDemand,ValueError>{demand.depth=demand.depth.checked_add(1).ok_or_else(||ValueError::literal(semio_framework_value::ValueRefusalKind::DepthLimit,"original preparation child depth overflow"))?;Ok(demand)};
        if let Some(owner)=self.active_retirement.as_ref(){return nested(artifact_retirement_box_demands(owner,body)?);}
        if self.semantic_refusal.is_some() { return nested(artifact_retirement_owned_birth_demands(&self.semantic_refusal)?); }
        if let Some(sealer)=self.sealer.as_ref(){if sealer.terminal_is_empty(){return Ok(RetirementDemand{copy_bytes:std::mem::size_of_val(&self.sealer),depth:1,..Default::default()});}return nested(sealer.retirement_demands(body)?);}
        if self.pending_edit.is_some(){return Ok(RetirementDemand{capacity_bytes:std::mem::size_of::<ArtifactStoreDecodedEditRetirement<DemoMutation>>(),depth:2,..Default::default()});}
        if let Some(post)=self.pending_post.as_ref(){return Ok(RetirementDemand{capacity_bytes:self.snapshot_retirement.as_ref().expect("original pending post issuer").retirement_birth_bytes(post),depth:2,..Default::default()});}
        if self.prepared.is_some(){let birth=ArtifactStoreOneItemPrepared::<DemoSnapshot,DemoMutation>::retirement_birth_demand();return Ok(RetirementDemand {capacity_bytes:birth.capacity_bytes,depth:birth.depth+1,..Default::default()});}
        if self.base.is_some(){return Ok(RetirementDemand {depth:1,..Default::default()});}
        if let Some(mutation)=self.mutation.as_ref(){return Ok(RetirementDemand {capacity_bytes:self.mutation_retirement.as_ref().expect("original mutation issuer").retirement_birth_bytes(mutation),depth:2,..Default::default()});}
        if let Some(authority)=self.authority.as_ref(){let birth=authority.retirement_birth_demand();return Ok(RetirementDemand {capacity_bytes:birth.capacity_bytes,depth:birth.depth+1,..Default::default()});}
        if let Some(identity)=self.stamped_mutation_id.as_ref(){return Ok(RetirementDemand {release_bytes:identity.0.capacity(),depth:1,..Default::default()});}
        if self.published_root.is_some()||self.snapshot_retirement.is_some()||self.mutation_retirement.is_some(){return Ok(RetirementDemand {copy_bytes:std::mem::size_of::<Arc<dyn semio_framework_value::FactoryRetirement>>(),depth:1,..Default::default()});}
        self.closing_factories.iter().find_map(Option::as_ref).map_or(Ok(Default::default()),|factory|nested(factory.demands(body)?))
    }
}

impl ArtifactStoreOneItemPreparation<DemoSnapshot, DemoMutation> for DemoOneItemPreparation {
    fn advance(&mut self, grant: ArtifactStoreOneItemGrant) -> Result<ArtifactStoreOneItemPreparationStep, ValueError> {
        if !grant.permits_one() {
            return Ok(ArtifactStoreOneItemPreparationStep::Blocked);
        }
        if self.cancelled {
            return Ok(ArtifactStoreOneItemPreparationStep::Blocked);
        }
        if self.semantic_refusal.is_some() { return Err(ValueError::literal(semio_framework_value::ValueRefusalKind::InvalidValue, "original preparation retains its semantic refusal owner")); }
        if self.prepared.is_some() {
            return Ok(ArtifactStoreOneItemPreparationStep::Prepared(self.checkpoint, RetainedCloneProgress::default()));
        }
        if let Some(sealer)=self.sealer.as_mut(){
            let step=sealer.advance(grant)?;
            let progress=step.ownership_progress();
            if let Some(mut prepared)=sealer.take_prepared(){
                if self.forge_digest{prepared.sealed_parts_mut().1[0]^=0xff;}
                self.checkpoint=ArtifactStoreOneItemCheckpoint{cursor:1,completed_items:1,completed_bytes:1,digest:prepared.edit_digest()};
                self.prepared=Some(prepared);
                return Ok(ArtifactStoreOneItemPreparationStep::Prepared(self.checkpoint,progress));
            }
            return Ok(match step{ArtifactStoreOneItemPreparationStep::Blocked=>step,_=>ArtifactStoreOneItemPreparationStep::Progress(self.checkpoint,progress)});
        }
        if self.pending_edit.is_some(){
            let demand=ArtifactStoreOneItemSealer::<DemoSnapshot,DemoMutation>::constructor_demand();
            if demand.capacity_bytes>grant.maximum_capacity_bytes||demand.depth>grant.maximum_depth||ArtifactStoreOneItemSealer::<DemoSnapshot,DemoMutation>::constructor_copy_byte_demand()>grant.maximum_copy_bytes{return Ok(ArtifactStoreOneItemPreparationStep::Blocked);}
            let authority=Arc::clone(self.authority.as_ref().ok_or_else(||ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated, "demo preparation lost its live Store authority"))?);
            let edit=self.pending_edit.take().expect("original pending edit");let post=self.pending_post.take().expect("original pending post");
            let mutations=Arc::clone(self.mutation_retirement.as_ref().expect("original pending mutation issuer"));let snapshots=Arc::clone(self.snapshot_retirement.as_ref().expect("original pending snapshot issuer"));
            return match authority.begin_one_item_seal(edit,post,mutations,snapshots,grant.retained_grant()){
                Ok((sealer,progress))=>{self.sealer=Some(sealer);Ok(ArtifactStoreOneItemPreparationStep::Progress(self.checkpoint,progress))},
                Err((error,authority,edit,post,mutations,snapshots))=>{self.pending_edit=Some(edit);self.pending_post=Some(post);drop((authority,mutations,snapshots));Err(error)}
            };
        }
        let base = self.base.as_ref().ok_or_else(|| ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated, "demo preparation lost its base root"))?;
        let authority = self.authority.as_ref().ok_or_else(|| ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated, "demo preparation lost its live Store authority"))?;
        let id_capacity=self.stamped_mutation_id.as_ref().map_or(29,|id|id.0.len());
        let mutation_identity_capacity=if self.stamped_mutation_id.is_some(){0}else{id_capacity+2};
        let group_capacity=authority.group_id().map_or(0,str::len);
        let mut inverse_list=semio_framework_value::list::PagedList::<DemoMutation,{usize::MAX}>::empty();
        let inverse_metadata=inverse_list.next_exact_capacity_allocation_bytes(1).map_err(ValueError::from)?.unwrap();
        let mutation_bytes=std::mem::size_of::<DemoMutation>();
        let capacity=semio_framework_value::shared_retirement_allocation_bytes::<DemoSnapshot>()+id_capacity+mutation_identity_capacity+group_capacity+3*mutation_bytes+std::mem::size_of::<MutationMeta>()+inverse_metadata;
        let copied_bytes=std::mem::size_of::<Edit<DemoMutation>>()+std::mem::size_of::<DemoSnapshot>()+2*std::mem::size_of::<semio_framework_value::SharedUtf8>()+id_capacity+mutation_identity_capacity+group_capacity;
        if capacity>grant.maximum_capacity_bytes||copied_bytes>grant.maximum_copy_bytes||mutation_bytes>grant.maximum_release_bytes||grant.maximum_depth<3{return Ok(ArtifactStoreOneItemPreparationStep::Blocked);}
        let mut progress=RetainedCloneProgress{copied_items:1,copied_bytes,retained_capacity_bytes:capacity,released_bytes:mutation_bytes};
        let mutation = self.mutation.take().ok_or_else(|| ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated, "demo preparation lost its mutation"))?;
        let outcome = mutation.diff(base.get());
        if outcome.worst_level().is_some_and(|level| level >= semio_framework_diagnostic::Severity::Error) {
            self.mutation=Some(mutation);
            return Err(ValueError::literal(semio_framework_value::ValueRefusalKind::InvalidValue, "demo retained mutation rejected against its base"));
        }
        let post=match semio_framework_os_spr::apply_diff(outcome.diff(),base.get()){Ok(post)=>post,Err(error)=>{self.mutation=Some(mutation);*self.semantic_refusal = Some(error);return Err(ValueError::literal(semio_framework_value::ValueRefusalKind::InvalidValue, "original mutation application refused"));}};
        let post_snapshot = Arc::new(post);
        let inverse = mutation.inverse(base.get()).expect("valid retained mutation inverse fixture");
        let sequence_number = authority.next_sequence_number();
        let stamped = self.stamped_mutation_id.take();
        let mut id=String::with_capacity(id_capacity);
        if let Some(stamped)=stamped.as_ref(){id.push_str(&stamped.0);}else{use std::fmt::Write;write!(&mut id,"retained-one-item-{sequence_number}").expect("bounded original identity");}
        let mutation_id=stamped.unwrap_or_else(||{let mut value=String::with_capacity(mutation_identity_capacity);value.push_str(&id);value.push_str("#0");MutationId(value)});
        progress.copied_bytes-=id_capacity-id.len();
        if mutation_identity_capacity!=0{progress.copied_bytes-=mutation_identity_capacity-mutation_id.0.len();}
        if inverse.is_empty(){progress.retained_capacity_bytes-=mutation_bytes+inverse_metadata;progress.released_bytes=0;}
        else{
            assert_eq!(inverse.len(),1,"bounded Demo inverse is exactly one original mutation");
            while let Some(bytes)=inverse_list.next_exact_capacity_allocation_bytes(1).map_err(ValueError::from)?{let allocation=inverse_list.reserve_exact_capacity_one(1,bytes).map_err(|error|ValueError::from(error.refusal()).with_retained_progress(RetainedCloneProgress {copied_items:1,retained_capacity_bytes:error.allocated_bytes,..Default::default()}))?;assert_eq!(allocation.allocated_bytes,bytes);}
        }
        for original in inverse{inverse_list.push_reserved(original).unwrap_or_else(|_|panic!("original inverse slot admitted before owner movement"));}
        let forward = mutation;
        let edit = Edit {
            line: None,
            id,
            actor: Some(authority.actor_utf8().clone()),
            forwards: vec![forward],
            inverse: inverse_list,
            mutation_meta: vec![MutationMeta {
                mutation_id: Some(mutation_id),
                dependencies: Vec::new(),
                base_version: 0,
                author_id: Some(ActorId(authority.actor_utf8().clone())),
                timestamp: authority.next_clock(),
                undo_policy: UndoPolicy::ExactBaseOnly,
                payload_hash: None,
                semantic_kind: None,
                label: None,
                group_id: authority.group_id().map(str::to_owned),
                origin: Default::default(),
                transaction: None,
            }],
            verb: None,
            sequence_number,
            started_at: String::new(),
            finished_at: None,
        };
        self.published_root.as_ref().ok_or_else(|| ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated, "demo preparation lost its original witness"))?.publish(&post_snapshot);
        self.pending_edit=Some(edit);self.pending_post=Some(post_snapshot);
        Ok(ArtifactStoreOneItemPreparationStep::Progress(self.checkpoint,progress))
    }

    fn checkpoint(&self) -> ArtifactStoreOneItemCheckpoint {
        self.checkpoint
    }

    fn prepared(&self) -> Option<&ArtifactStoreOneItemPrepared<DemoSnapshot, DemoMutation>> {
        self.prepared.as_ref()
    }

    fn take_prepared(&mut self) -> Option<ArtifactStoreOneItemPrepared<DemoSnapshot, DemoMutation>> {
        self.prepared.take()
    }

    fn cancel(&mut self) {
        self.cancelled = true;
        if let Some(sealer)=self.sealer.as_mut(){sealer.cancel();}
    }

    fn begin_close(&mut self) {
        self.closing = true;
        if let Some(sealer)=self.sealer.as_mut(){sealer.begin_close();}
    }

    fn close_step(&mut self, grant: ArtifactStoreOneItemGrant) -> Result<RetainedCloneStep, ValueError> {
        let empty=RetainedCloneProgress::default();let grant=grant.retained_grant();
        if !self.closing||grant.maximum_items==0 { return Ok(RetainedCloneStep::Progress(empty)); }
        if self.terminal_is_empty() { return Ok(RetainedCloneStep::Complete(empty)); }
        let demand=self.close_demands(grant.maximum_copy_bytes)?;
        if grant.maximum_depth<demand.depth { return Err(ValueError::literal(semio_framework_value::ValueRefusalKind::DepthLimit,"original preparation close depth is not funded")); }
        if grant.maximum_copy_bytes<demand.copy_bytes||grant.maximum_capacity_bytes<demand.capacity_bytes||grant.maximum_release_bytes<demand.release_bytes { return Ok(RetainedCloneStep::Progress(empty)); }
        let child=RetainedCloneGrant { maximum_items:1,maximum_depth:grant.maximum_depth-1,..grant };
        if self.active_retirement.is_some() { return artifact_retirement_box_close_step(&mut self.active_retirement,child).map(|step|RetainedCloneStep::Progress(step.progress())); }
        if self.semantic_refusal.is_some() { return artifact_retirement_admit_owned(&mut self.semantic_refusal, &mut self.active_retirement, child); }
        if let Some(sealer)=self.sealer.as_mut(){if sealer.terminal_is_empty(){self.sealer=None;return Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,..empty}));}return sealer.close_step(child).map(|step|RetainedCloneStep::Progress(step.progress()));}
        if let Some(edit)=self.pending_edit.take(){let issuer=Arc::clone(self.mutation_retirement.as_ref().expect("original pending edit issuer"));return match admit_artifact_retirement(edit,child,|original|ArtifactStoreDecodedEditRetirement::new(original,issuer)){Ok((owner,progress))=>{self.active_retirement=Some(owner);Ok(RetainedCloneStep::Progress(progress))},Err((error,original))=>{self.pending_edit=Some(original);Err(error)}};}
        if let Some(post)=self.pending_post.take(){return match self.snapshot_retirement.as_ref().expect("original pending post issuer").retire(post,child){Ok((owner,progress))=>{self.active_retirement=Some(owner);Ok(RetainedCloneStep::Progress(progress))},Err((error,original))=>{self.pending_post=Some(original);Err(error)}};}
        if let Some(prepared)=self.prepared.take() {
            let mutation=Arc::clone(self.mutation_retirement.as_ref().expect("original candidate mutation issuer"));let snapshot=Arc::clone(self.snapshot_retirement.as_ref().expect("original candidate snapshot issuer"));
            return match prepared.admit_retirement(mutation,snapshot,child) { Ok((owner,progress))=>{self.active_retirement=Some(owner);Ok(RetainedCloneStep::Progress(progress))},Err((error,original,mutation,snapshot))=>{self.prepared = Some(original);drop((mutation,snapshot));Err(error)} };
        }
        if let Some(base)=self.base.take() { return match base.try_return_to_registry_witness() { Ok(witness)=>{drop(witness);Ok(RetainedCloneStep::Progress(RetainedCloneProgress { copied_items:1,..empty }))},Err(original)=>{self.base=Some(original);Err(ValueError::literal(semio_framework_value::ValueRefusalKind::WorkLimit,"original read registry return is busy"))} }; }
        if let Some(mutation)=self.mutation.take() { return match self.mutation_retirement.as_ref().expect("original cancellation mutation issuer").retire_owned(mutation,child) { Ok((owner,progress))=>{self.active_retirement=Some(owner);Ok(RetainedCloneStep::Progress(progress))},Err((error,original))=>{self.mutation=Some(original);Err(error)} }; }
        if let Some(authority)=self.authority.take() { return match authority.retire(child) { Ok((owner,progress))=>{self.active_retirement=Some(owner);Ok(RetainedCloneStep::Progress(progress))},Err((error,original))=>{self.authority=Some(original);Err(error)} }; }
        if let Some(identity)=self.stamped_mutation_id.take() { let released_bytes=identity.0.capacity();drop(identity);return Ok(RetainedCloneStep::Progress(RetainedCloneProgress { copied_items:1,released_bytes,..empty })); }
        if let Some(witness)=self.published_root.take() { let factory:Arc<dyn semio_framework_value::FactoryRetirement>=witness;self.closing_factories[0]=Some(semio_framework_value::FactoryAuthority::new(factory));return Ok(RetainedCloneStep::Progress(RetainedCloneProgress { copied_items:1,copied_bytes:demand.copy_bytes,..empty })); }
        if let Some(snapshot)=self.snapshot_retirement.take() { let factory:Arc<dyn semio_framework_value::FactoryRetirement>=snapshot;self.closing_factories[1]=Some(semio_framework_value::FactoryAuthority::new(factory));return Ok(RetainedCloneStep::Progress(RetainedCloneProgress { copied_items:1,copied_bytes:demand.copy_bytes,..empty })); }
        if let Some(mutation)=self.mutation_retirement.take() { let factory:Arc<dyn semio_framework_value::FactoryRetirement>=mutation;self.closing_factories[2]=Some(semio_framework_value::FactoryAuthority::new(factory));return Ok(RetainedCloneStep::Progress(RetainedCloneProgress { copied_items:1,copied_bytes:demand.copy_bytes,..empty })); }
        let slot=self.closing_factories.iter_mut().find(|slot|slot.is_some()).expect("retained original issuer close capability");let factory=slot.as_mut().unwrap();let step=factory.step(child)?;semio_framework_value::retained_clone::admit_retained_clone_close(child,step,factory.terminal_is_empty(),"original Demo preparation issuer close")?;if factory.terminal_is_empty(){*slot=None;}Ok(if self.terminal_is_empty(){RetainedCloneStep::Complete(step.progress())}else{RetainedCloneStep::Progress(step.progress())})
    }

    fn next_close_copy_byte_demand(&self)->Result<usize,ValueError>{Ok(self.close_demands(0)?.copy_bytes)}
    fn next_close_capacity_byte_demand(&self,body:usize)->Result<usize,ValueError>{Ok(self.close_demands(body)?.capacity_bytes)}
    fn next_close_release_byte_demand(&self)->Result<usize,ValueError>{Ok(self.close_demands(0)?.release_bytes)}
    fn next_close_depth_demand(&self)->Result<usize,ValueError>{Ok(self.close_demands(0)?.depth)}
    fn terminal_is_empty(&self)->bool { self.closing&&self.semantic_refusal.is_none()&&self.base.is_none()&&self.mutation.is_none()&&self.prepared.is_none()&&self.pending_edit.is_none()&&self.pending_post.is_none()&&self.sealer.is_none()&&self.authority.is_none()&&self.stamped_mutation_id.is_none()&&self.published_root.is_none()&&self.snapshot_retirement.is_none()&&self.mutation_retirement.is_none()&&self.active_retirement.is_none()&&self.closing_factories.iter().all(Option::is_none) }
}
