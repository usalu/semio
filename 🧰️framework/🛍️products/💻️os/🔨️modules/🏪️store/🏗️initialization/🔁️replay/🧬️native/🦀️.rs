//! 🧬️ Initialization folds the actual ledger operation through its installed original semantic producer.
use super::*;
use semio_framework_os_store_replay_operation::{ArtifactDerivedOperation,ArtifactDerivedOperationMode,ArtifactReplayPreparationContext,ArtifactReplayPreparationMode,ArtifactReplayPreparationRequest,ArtifactReplayPreparationTask};
use semio_framework_os_store_replay_operation::native::history::OriginalHistorySource;
use semio_framework_value::retained_clone::{RetainedOwnedProjection,retained_clone_grant_admits};

pub struct NativeInitializationFold<P,M:Sync+'static>{
    replacement:semio_framework_os_store_operation_wire::OriginalReplacementReceiving<M>,
    replacement_key:Option<(usize,usize)>,
    failure:Option<semio_framework_job::OriginalTurnFailure>,
    history:Option<OriginalHistorySource<Edit<M>>>,
    issuer:Option<Arc<dyn ArtifactOwnedValueRetirementFactory<Edit<M>>>>,
    issuer_close:Option<semio_framework_value::FactoryAuthority>,
    input:Option<RetainedOwnedProjection<M>>,
    owner:Option<ArtifactDerivedOperation<P,M>>,
    next:Option<Arc<P>>,
    next_close:Option<Box<dyn ErasedSnapshotRetirement>>,
    revision:[u8;32],
    phase:u8,
    closing:bool,
}
impl<P,M:Sync+'static> Default for NativeInitializationFold<P,M>{fn default()->Self{Self{replacement:Default::default(),replacement_key:None,failure:None,history:None,issuer:None,issuer_close:None,input:None,owner:None,next:None,next_close:None,revision:[0;32],phase:0,closing:false}}}
fn demand<T>()->RetirementDemand{RetirementDemand{copy_bytes:size_of::<T>(),depth:1,..Default::default()}}
fn invalid(message:&'static str)->ValueError{ValueError::literal(ValueRefusalKind::InvariantViolated,message)}

impl<P,M> NativeInitializationFold<P,M>
where P:ToValue+RetireOwned+Send+Sync+'static,M:Mutation<P>+ToValue+FromValue+OpBinary+OpText+RetireOwned+Send+'static,
{
    pub fn take_failure(&mut self)->Option<semio_framework_job::OriginalTurnFailure>{self.failure.take()}
    pub fn active(&self)->bool{self.phase!=0||!self.terminal_is_empty()}
    pub fn terminal_is_empty(&self)->bool{self.replacement.terminal_is_empty()&&self.replacement_key.is_none()&&self.history.is_none()&&self.issuer.is_none()&&self.issuer_close.is_none()&&self.input.is_none()&&self.owner.is_none()&&self.next.is_none()&&self.next_close.is_none()}
    fn physical(cx:&mut StepContext<'_>,quote:RetirementDemand,work:impl FnOnce(RetainedCloneGrant)->Result<RetainedCloneProgress,ValueError>)->Result<RetainedCloneProgress,ValueError>{
        let grant=cx.retained_grant();
        if cx.should_yield()||!retained_clone_grant_admits(grant,quote){return Ok(Default::default());}
        let result=cx.with_native_receiving(|control|{control.admit_turn_capacity(grant.maximum_capacity_bytes)?;if quote.capacity_bytes!=0{control.charge(quote.capacity_bytes)?;}work(grant)});
        let receipt=match &result{Ok(receipt)=>*receipt,Err(error)=>error.retained_progress()};
        cx.consume_receiving_retained(grant,receipt)?;cx.consume_fuel(receipt.copied_items as u64);result
    }
    pub fn advance(&mut self,envelope:&mut ArtifactEnvelope<P,M>,runtime:&mut ArtifactStoreInitializationRuntime<P>,owners:&DocumentStoreOwners<P,M>,edit:usize,mutation:usize,cx:&mut StepContext<'_>)->Result<bool,ValueError>{
        if self.closing{return Err(invalid("closing initialization fold cannot accept another original operation"));}
        if self.phase>=4{return self.finish(envelope,runtime,owners,cx);}
        if self.phase==0&&self.terminal_is_empty()&&!runtime.supersessions.is_empty(){
            let original=envelope.vcs.edits.get(edit).ok_or_else(||invalid("native effective initialization edit absent"))?;
            let selected=super::native_digest::with_operation_identity::<P,M,_>(original,mutation,|id|Ok(runtime.supersessions.get(id).map(|value|matches!(value.replacement,semio_framework_os_spr::InputReplacement::Withdrawn))))?;
            if selected==Some(true){let mut completed=false;Self::physical(cx,RetirementDemand{copy_bytes:256,depth:1,..Default::default()},|_|{completed=true;Ok(RetainedCloneProgress{copied_items:1,copied_bytes:256,..Default::default()})})?;return Ok(completed)}
            if selected==Some(false){let quote=demand::<semio_framework_os_store_operation_wire::OriginalReplacementReceiving<M>>();Self::physical(cx,quote,|_|{self.replacement=Default::default();self.replacement_key=Some((edit,mutation));self.phase=1;Ok(RetainedCloneProgress{copied_items:1,copied_bytes:quote.copy_bytes,..Default::default()})})?;return Ok(false)}
        }
        if self.replacement_key.is_some()&&self.owner.is_none()&&self.input.is_none(){
            let original=envelope.vcs.edits.get(edit).ok_or_else(||invalid("native replacement original edit absent"))?;let factory=owners.mutation_retirement.as_ref().ok_or_else(||invalid("native replacement original mutation issuer absent"))?;
            let result=super::native_digest::with_operation_identity::<P,M,_>(original,mutation,|id|{let row=runtime.supersessions.get_mut(id).ok_or_else(||invalid("native replacement original supersession absent"))?;if let semio_framework_os_spr::InputReplacement::Input{schema,..}=&row.replacement{if schema!=&envelope.schema{return Err(ValueError::literal(ValueRefusalKind::InvalidValue,"native replacement schema differs from its original artifact"))}}Ok(self.replacement.advance(&mut row.replacement,&owners.operation_wire,factory,cx))})?;
            match result{Ok(true)=>{if let Err(error)=self.replacement.take_input(&mut self.input,cx){self.failure=Some(error)}},Ok(false)=>{},Err(error)=>self.failure=Some(error)}return Ok(false);
        }
        if self.replacement_key.is_none()&&self.issuer.is_none(){
            let quote=owners.history_edit_read_issuer_demands();
            Self::physical(cx,quote,|grant|owners.admit_history_edit_read_issuer(&mut self.issuer,grant).map(RetainedCloneStep::progress))?;return Ok(false);
        }
        if self.replacement_key.is_none()&&self.history.is_none(){
            let key=envelope.vcs.edits.retained_read_key_at(edit).ok_or_else(||invalid("initialization original edit key absent"))?;
            Self::physical(cx,OriginalHistorySource::<Edit<M>>::constructor_demand(),|grant|{let Some((source,receipt))=OriginalHistorySource::admit(key,self.issuer.as_ref().unwrap(),grant)?else{return Ok(Default::default());};self.history=Some(source);self.phase=1;Ok(receipt)})?;return Ok(false);
        }
        if self.replacement_key.is_none()&&self.input.is_none()&&self.owner.is_none(){
            let history=self.history.as_mut().unwrap();let quote=history.demands(&envelope.vcs.edits)?;
            Self::physical(cx,quote,|grant|history.advance(&mut envelope.vcs.edits,&mut self.input,mutation+1,|edit|&edit.forwards[mutation],grant).map(RetainedCloneStep::progress))?;return Ok(false);
        }
        if self.owner.is_none(){
            let factory=owners.replay_preparation.as_ref().ok_or_else(||ValueError::literal(ValueRefusalKind::UnsupportedOwner,"initialization requires its original installed native semantic producer"))?;
            let mut quote=factory.preparation_birth_demand();
            let overhead=ArtifactStoreInitializationRuntime::<P>::native_snapshot_read_demand().copy_bytes+size_of::<ArtifactDerivedOperation<P,M>>()+2*size_of::<Arc<dyn semio_framework_value::FactoryRetirement>>();
            quote.copy_bytes=quote.copy_bytes.checked_add(overhead).ok_or_else(||invalid("native initialization producer movement overflow"))?;
            let generation=cx.generation().0;
            Self::physical(cx,quote,|grant|{
                let Some((base,_))=runtime.admit_native_snapshot_read(generation,grant)?else{return Ok(Default::default());};
                self.revision=runtime.revision.revision(runtime.current_checkpoint_id.as_deref());
                let context=ArtifactReplayPreparationContext{mode:ArtifactReplayPreparationMode::Prefix,operation_index:mutation.try_into().map_err(|_|invalid("initialization operation index exceeds native wire"))?,generation,base_revision:self.revision};
                let snapshots=owners.initial_snapshot_retirement.as_ref().ok_or_else(||invalid("initialization original snapshot issuer absent"))?;
                let mutations=owners.mutation_retirement.as_ref().ok_or_else(||invalid("initialization original mutation issuer absent"))?;
                let request=ArtifactReplayPreparationRequest{base,input:self.input.take().unwrap(),context,snapshots:Arc::clone(snapshots),mutations:Arc::clone(mutations)};
                let mut owner=ArtifactDerivedOperation{base:ManuallyDrop::new(None),producer:ManuallyDrop::new(None),registry:Some((**runtime.read_registry).clone()),context,failed_read:ManuallyDrop::new(None),failed_input:ManuallyDrop::new(None),registry_close:ManuallyDrop::new(None),mode:ArtifactDerivedOperationMode::HistoryPrefix,policy:semio_framework_os_spr::MergePolicy::default(),task:ManuallyDrop::new(None),task_close:ManuallyDrop::new(None),prepared:ManuallyDrop::new(None),mutation:ManuallyDrop::new(None),retirement:ManuallyDrop::new(None),snapshots:ManuallyDrop::new(Some(Arc::clone(snapshots))),mutations:ManuallyDrop::new(Some(Arc::clone(mutations))),factory_close:std::array::from_fn(|_|None),message:0,accepted:true,ready:false,finished:false,closing:false,progress:Default::default()};
                match factory.begin(request,RetainedCloneGrant{maximum_copy_bytes:grant.maximum_copy_bytes-overhead,..grant}){
                    Ok((cursor,mut receipt))=>{*owner.task=Some(ArtifactReplayPreparationTask::new(context,cursor));self.owner=Some(owner);self.phase=2;receipt.copied_bytes+=overhead;Ok(receipt)}
                    Err((error,request))=>{*owner.failed_read=Some(request.base.into_erased());*owner.failed_input=Some(request.input);owner.factory_close[2]=Some(semio_framework_value::FactoryAuthority::new(request.snapshots));owner.factory_close[3]=Some(semio_framework_value::FactoryAuthority::new(request.mutations));owner.cancel();self.owner=Some(owner);let mut receipt=error.retained_progress();receipt.copied_items=receipt.copied_items.max(1);receipt.copied_bytes+=overhead;Err(error.with_retained_progress(receipt))}
                }
            })?;return Ok(false);
        }
        let owner=self.owner.as_mut().unwrap();
        if owner.is_finished(){
            let foreign=owner.prepared.as_ref().is_some_and(|prepared|prepared.foreign_steps);
            if foreign{
                let original=envelope.vcs.edits.get(edit).ok_or_else(||invalid("native initialization foreign edit absent"))?;let identity=CursorRevisionAccumulator::hash_record(b"edit-id",&[original.id.as_bytes()]);let key=(identity,mutation);let index=&mut runtime.revision.unit_flags;
                if !index.contains_key(&key){
                    let capacity=index.next_insert_capacity_byte_demand(&key,cx.retained_grant().maximum_copy_bytes)?;let quote=RetirementDemand{copy_bytes:if capacity==0{size_of::<(([u8;32],usize),bool)>()}else{0},capacity_bytes:capacity,depth:index.next_insert_depth_demand(&key)?,release_bytes:0};
                    Self::physical(cx,quote,|grant|{if capacity!=0{return index.reserve_insert_step(&key,grant).map_err(|(error,receipt)|error.with_retained_progress(receipt));}index.insert_reserved(key,true,grant).map_err(|_|invalid("native initialization original foreign flag insertion refused"))?;Ok(RetainedCloneProgress{copied_items:1,copied_bytes:quote.copy_bytes,..Default::default()})})?;return Ok(false);
                }
            }
            Self::physical(cx,demand::<semio_framework_os_store_replay_operation::ArtifactReplayPrepared<P,M>>(),|_|{let mut prepared=owner.prepared.take().ok_or_else(||invalid("native initialization result owner absent"))?;if !matches!(&prepared.inverse,Ok(rows)if rows.terminal_is_empty())||prepared.messages.capacity()!=0||prepared.apply_refusal.is_some()||prepared.input_refusal.is_some(){*owner.prepared=Some(prepared);return Err(invalid("native initialization producer left unsettled result fields"));}self.next=prepared.next.take();owner.cancel();self.phase=4;Ok(RetainedCloneProgress{copied_items:1,copied_bytes:size_of::<semio_framework_os_store_replay_operation::ArtifactReplayPrepared<P,M>>(),..Default::default()})})?;return Ok(false);
        }
        let grant=cx.retained_grant();let quote=owner.demands(cx.generation().0,self.revision,grant.maximum_copy_bytes)?;
        if cx.should_yield()||!retained_clone_grant_admits(grant,quote){return Ok(false);}
        let generation=cx.generation().0;let clock=cx.clock_source();let deadline=cx.deadline_us();
        let result=cx.with_native_receiving(|control|owner.step(generation,self.revision,grant.maximum_copy_bytes,grant,control,&mut||clock().is_none_or(|now|now>=deadline)));
        let receipt=match &result{Ok(step)=>step.progress(),Err(error)=>error.retained_progress()};cx.consume_receiving_retained(grant,receipt)?;cx.consume_fuel(receipt.copied_items as u64);result?;Ok(false)
    }
    pub fn close_demands(&self,envelope:&ArtifactEnvelope<P,M>,runtime:&ArtifactStoreInitializationRuntime<P>,body:usize)->Result<RetirementDemand,ValueError>{
        if let Some(owner)=self.owner.as_ref(){return if owner.terminal_is_empty(){Ok(demand::<Option<ArtifactDerivedOperation<P,M>>>())}else{owner.demands(owner.context.generation,owner.context.base_revision,body)};}
        if let Some(input)=self.input.as_ref(){return if input.terminal_is_empty(){Ok(demand::<Option<RetainedOwnedProjection<M>>>())}else{input.close_demand(body)};}
        if let Some(owner)=self.next_close.as_ref(){return semio_framework_value::factory_ticket_demands(owner,body);}
        if self.closing&&self.next.is_some(){return Ok(RetirementDemand{capacity_bytes:ReturnedSnapshotReadRetirement::<P>::constructor_capacity_bytes(),depth:1,..Default::default()});}
        if self.replacement_key.is_some(){return self.replacement.close_demands(body);}
        if let Some(history)=self.history.as_ref(){return if history.terminal_is_empty(){Ok(demand::<Option<OriginalHistorySource<Edit<M>>>>())}else{history.close_demands(&envelope.vcs.edits,body)};}
        let read=runtime.native_read_retirement_demands(body)?;
        if read!=Default::default(){return Ok(read);}
        if self.next.is_some(){return Ok(ArtifactStoreInitializationRuntime::<P>::adopt_native_next_demand());}
        if self.issuer.is_some(){return Ok(demand::<Arc<dyn semio_framework_value::FactoryRetirement>>());}
        if let Some(owner)=self.issuer_close.as_ref(){return if owner.terminal_is_empty(){Ok(demand::<Option<semio_framework_value::FactoryAuthority>>())}else{owner.demands(body)};}
        Ok(demand::<u8>())
    }
    fn retire_owner(&mut self,cx:&mut StepContext<'_>)->Result<bool,ValueError>{
        let Some(owner)=self.owner.as_mut()else{return Ok(true);};
        owner.cancel();
        if owner.terminal_is_empty(){Self::physical(cx,demand::<Option<ArtifactDerivedOperation<P,M>>>(),|_|{self.owner=None;Ok(RetainedCloneProgress{copied_items:1,copied_bytes:size_of::<Option<ArtifactDerivedOperation<P,M>>>(),..Default::default()})})?;return Ok(self.owner.is_none());}
        let grant=cx.retained_grant();let quote=owner.demands(owner.context.generation,owner.context.base_revision,grant.maximum_copy_bytes)?;
        if !retained_clone_grant_admits(grant,quote){return Ok(false);}
        let result=cx.with_native_retirement(|control|owner.close_step_controlled(grant,control));
        let receipt=match &result{Ok(step)=>step.progress(),Err(error)=>error.retained_progress()};cx.consume_retained(receipt)?;cx.consume_fuel(receipt.copied_items as u64);result?;Ok(false)
    }
    fn finish(&mut self,envelope:&mut ArtifactEnvelope<P,M>,runtime:&mut ArtifactStoreInitializationRuntime<P>,owners:&DocumentStoreOwners<P,M>,cx:&mut StepContext<'_>)->Result<bool,ValueError>{
        if !self.retire_owner(cx)?{return Ok(false);}
        if let Some(input)=self.input.as_ref(){
            if input.terminal_is_empty(){Self::physical(cx,demand::<Option<RetainedOwnedProjection<M>>>(),|_|{self.input=None;Ok(RetainedCloneProgress{copied_items:1,copied_bytes:size_of::<Option<RetainedOwnedProjection<M>>>(),..Default::default()})})?;}
            else{let quote=input.close_demand(cx.retained_grant().maximum_copy_bytes)?;Self::physical(cx,quote,|grant|self.input.as_mut().unwrap().close_step(grant).map(RetainedCloneStep::progress))?;}
            return Ok(false);
        }
        if self.next_close.is_some(){
            let quote=semio_framework_value::factory_ticket_demands(self.next_close.as_ref().unwrap(),cx.retained_grant().maximum_copy_bytes)?;let grant=cx.retained_grant();
            if !retained_clone_grant_admits(grant,quote){return Ok(false);}
            let result=cx.with_native_retirement(|control|semio_framework_value::close_factory_ticket_controlled(&mut self.next_close,grant,control));
            let receipt=match &result{Ok(step)=>step.progress(),Err(error)=>error.retained_progress()};cx.consume_retained(receipt)?;cx.consume_fuel(receipt.copied_items as u64);result?;return Ok(false);
        }
        if self.closing&&self.next.is_some(){
            let quote=RetirementDemand{capacity_bytes:ReturnedSnapshotReadRetirement::<P>::constructor_capacity_bytes(),depth:1,..Default::default()};
            Self::physical(cx,quote,|grant|{let factory=owners.initial_snapshot_retirement.as_ref().ok_or_else(||invalid("initialization original snapshot issuer absent"))?;let Some((owner,receipt))=ReturnedSnapshotReadRetirement::<P>::admit_original(&mut self.next,factory,grant)?else{return Ok(Default::default());};self.next_close=Some(owner);Ok(receipt)})?;return Ok(false);
        }
        if let Some((edit,mutation))=self.replacement_key{
            let original=envelope.vcs.edits.get(edit).ok_or_else(||invalid("closing native replacement edit absent"))?;
            let result=super::native_digest::with_operation_identity::<P,M,_>(original,mutation,|id|{let row=runtime.supersessions.get_mut(id).ok_or_else(||invalid("closing original supersession absent"))?;Ok(self.replacement.close(&mut row.replacement,cx))})?;
            match result{Ok(true)=>{let quote=demand::<Option<(usize,usize)>>();Self::physical(cx,quote,|_|{self.replacement_key=None;Ok(RetainedCloneProgress{copied_items:1,copied_bytes:quote.copy_bytes,..Default::default()})})?;},Ok(false)=>{},Err(error)=>self.failure=Some(error)}return Ok(false);
        }
        if let Some(history)=self.history.as_mut(){
            history.begin_close();
            if history.terminal_is_empty(){Self::physical(cx,demand::<Option<OriginalHistorySource<Edit<M>>>>(),|_|{self.history=None;Ok(RetainedCloneProgress{copied_items:1,copied_bytes:size_of::<Option<OriginalHistorySource<Edit<M>>>>(),..Default::default()})})?;}
            else{let quote=history.close_demands(&envelope.vcs.edits,cx.retained_grant().maximum_copy_bytes)?;Self::physical(cx,quote,|grant|self.history.as_mut().unwrap().close_step(&mut envelope.vcs.edits,grant).map(RetainedCloneStep::progress))?;}
            return Ok(false);
        }
        let factory=owners.initial_snapshot_retirement.as_ref().ok_or_else(||invalid("initialization original snapshot issuer absent"))?;
        let quote=runtime.native_read_retirement_demands(cx.retained_grant().maximum_copy_bytes)?;
        if quote!=Default::default(){Self::physical(cx,quote,|grant|runtime.settle_native_read_step(factory,grant).map(RetainedCloneStep::progress))?;return Ok(false);}
        if self.next.is_some(){Self::physical(cx,ArtifactStoreInitializationRuntime::<P>::adopt_native_next_demand(),|grant|runtime.adopt_native_next(&mut self.next,factory,grant))?;return Ok(false);}
        if self.issuer.is_some(){Self::physical(cx,demand::<Arc<dyn semio_framework_value::FactoryRetirement>>(),|_|{self.issuer_close=Some(semio_framework_value::FactoryAuthority::new(self.issuer.take().unwrap()));Ok(RetainedCloneProgress{copied_items:1,copied_bytes:size_of::<Arc<dyn semio_framework_value::FactoryRetirement>>(),..Default::default()})})?;return Ok(false);}
        if let Some(owner)=self.issuer_close.as_ref(){
            if owner.terminal_is_empty(){Self::physical(cx,demand::<Option<semio_framework_value::FactoryAuthority>>(),|_|{self.issuer_close=None;Ok(RetainedCloneProgress{copied_items:1,copied_bytes:size_of::<Option<semio_framework_value::FactoryAuthority>>(),..Default::default()})})?;}
            else{let quote=owner.demands(cx.retained_grant().maximum_copy_bytes)?;Self::physical(cx,quote,|grant|self.issuer_close.as_mut().unwrap().step(grant).map(RetainedCloneStep::progress))?;}
            return Ok(false);
        }
        let mut complete=false;
        Self::physical(cx,demand::<u8>(),|_|{self.phase=0;self.closing=false;complete=true;Ok(RetainedCloneProgress{copied_items:1,copied_bytes:size_of::<u8>(),..Default::default()})})?;
        Ok(complete)
    }
    pub fn begin_close(&mut self){self.closing=true;if let Some(owner)=self.owner.as_mut(){owner.cancel();}if let Some(history)=self.history.as_mut(){history.begin_close();}}
    pub fn close(&mut self,envelope:&mut ArtifactEnvelope<P,M>,runtime:&mut ArtifactStoreInitializationRuntime<P>,owners:&DocumentStoreOwners<P,M>,cx:&mut StepContext<'_>)->Result<bool,ValueError>{self.closing=true;self.finish(envelope,runtime,owners,cx)}
}
