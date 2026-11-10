//! 🌱️ Original Store semantic previews retain one installed operation producer across granted turns.
use super::*;
use semio_os_spr as protocol;
use semio_framework_value::{ArtifactOwnedValueRetirementFactory,ErasedSnapshotRetirement,ValueError,retained_clone::{RetainedCloneGrant,RetainedCloneProgress,RetainedCloneStep}};
use semio_framework_value_snapshot_read::{ErasedSnapshotRead,SnapshotRead};
use semio_framework_value_snapshot_read as snapshot_read_registry_handle;
use semio_framework_value_snapshot_read as snapshot_read_registry_retirement;
use semio_framework_value::factory::FactorySharedRetirement as ReturnedSnapshotReadRetirement;
use semio_os_store_one_item::ArtifactStoreOneItemGrant;
use std::{sync::Arc,mem::size_of};
use semio_framework_value::{RetirementDemand,ValueRefusalKind};
use std::mem::ManuallyDrop;
use crate::native::{Effect,Exit,pay,one};
enum DerivedOperationTurn<'a,'control>{Advance(&'a mut semio_framework_value_native_owned::native_decoding::NativeDecodeControl<'control>,usize),Close(Option<&'a mut dyn semio_framework_value::OriginalRetirementControl>),Original(&'a mut semio_framework_value::OriginalRetirementCloseLoan<'control>)}

/// 🧭️ History preserves diagnostic outcomes; provisional overlays obey the Store merge policy.
#[derive(Clone,Copy,Debug,PartialEq,Eq)]
pub enum ArtifactDerivedOperationMode { Provisional, History, HistoryPrefix }

/// 🧳️ The caller keeps its original operation fixed until this cursor finishes or closes.
pub struct ArtifactDerivedOperation<P,M:Sync> {
    pub registry: Option<snapshot_read_registry_handle::SnapshotReadRegistryHandle>,
    pub context: ArtifactReplayPreparationContext,
    pub base:ManuallyDrop<Option<SnapshotRead<P>>>,
    pub producer:ManuallyDrop<Option<Arc<dyn ArtifactReplayPreparationFactory<P,M>>>>,
    pub failed_read:ManuallyDrop<Option<ErasedSnapshotRead>>,
    pub failed_input:ManuallyDrop<Option<semio_framework_value::retained_clone::RetainedOwnedProjection<M>>>,
    pub registry_close:ManuallyDrop<Option<snapshot_read_registry_retirement::SnapshotReadRegistryAliasRetirement>>,
    pub mode: ArtifactDerivedOperationMode,
    pub policy:protocol::MergePolicy,
    pub task: ManuallyDrop<Option<ArtifactReplayPreparationTask<P,M>>>,
    pub task_close: ManuallyDrop<Option<ArtifactReplayPreparationRetirement<P,M>>>,
    pub prepared: ManuallyDrop<Option<ArtifactReplayPrepared<P,M>>>,
    pub mutation: ManuallyDrop<Option<M>>,
    pub retirement: ManuallyDrop<Option<Box<dyn ErasedSnapshotRetirement>>>,
    pub snapshots: ManuallyDrop<Option<Arc<dyn ArtifactOwnedValueRetirementFactory<P>>>>,
    pub mutations: ManuallyDrop<Option<Arc<dyn ArtifactOwnedValueRetirementFactory<M>>>>,
    pub factory_close: [Option<semio_framework_value::FactoryAuthority>;5],
    pub message: usize,
    pub accepted: bool,
    pub ready: bool,
    pub finished: bool,
    pub closing: bool,
    pub progress: ArtifactReplayPreparationProgress,
    pub last_work:u64,
}

fn funded(grant:RetainedCloneGrant,demand:RetirementDemand)->bool { grant.maximum_items>0&&grant.maximum_copy_bytes>=demand.copy_bytes&&grant.maximum_capacity_bytes>=demand.capacity_bytes&&grant.maximum_release_bytes>=demand.release_bytes&&grant.maximum_depth>=demand.depth }
fn header<T>()->RetirementDemand { RetirementDemand{copy_bytes:size_of::<T>(),depth:1,..Default::default()} }
fn refusal(message:&'static str)->ValueError { ValueError::literal(ValueRefusalKind::InvariantViolated,message) }

impl<P,M:Sync> ArtifactDerivedOperation<P,M> {
    /// 📈️ Counts admitted work from the actual semantic producer and every cleanup frontier.
    pub fn last_turn_work(&self)->u64{self.last_work}
    pub fn progress(&self)->ArtifactReplayPreparationProgress { self.progress }
    /// 🛑️ Stops source reads while retaining every producer and prepared owner for bounded cleanup.
    pub fn cancel(&mut self) { self.closing=true;self.finished=false; }
    /// 🏁️ Completion retains its result until the Store admits one explicit handoff.
    pub fn is_finished(&self)->bool { self.finished&&!self.closing }
    /// 🧹️ Cleanup never needs another read of the original input.
    pub fn is_closing(&self)->bool { self.closing }
    /// ♻️ Every original producer, prepared owner and authority alias has physically returned.
    pub fn terminal_is_empty(&self)->bool { self.base.is_none()&&self.producer.is_none()&&self.registry.is_none()&&self.registry_close.is_none()&&self.failed_read.is_none()&&self.failed_input.is_none()&&self.task.is_none()&&self.task_close.is_none()&&self.prepared.is_none()&&self.mutation.is_none()&&self.retirement.is_none()&&self.snapshots.is_none()&&self.mutations.is_none()&&self.factory_close.iter().all(Option::is_none) }
    pub fn record(&mut self,p:RetainedCloneProgress) { self.progress.items=self.progress.items.saturating_add(p.copied_items);self.progress.bytes=self.progress.bytes.saturating_add(p.copied_bytes).saturating_add(p.retained_capacity_bytes).saturating_add(p.released_bytes); }
}
impl<P,M:Sync> Drop for ArtifactDerivedOperation<P,M> {
    fn drop(&mut self) { assert!(self.terminal_is_empty()||std::thread::panicking(),"original derived operation requires a completed handoff or terminal cleanup");if self.terminal_is_empty(){unsafe{ManuallyDrop::drop(&mut self.base);ManuallyDrop::drop(&mut self.producer);ManuallyDrop::drop(&mut self.failed_read);ManuallyDrop::drop(&mut self.failed_input);ManuallyDrop::drop(&mut self.registry_close);ManuallyDrop::drop(&mut self.task);ManuallyDrop::drop(&mut self.task_close);ManuallyDrop::drop(&mut self.prepared);ManuallyDrop::drop(&mut self.mutation);ManuallyDrop::drop(&mut self.retirement);ManuallyDrop::drop(&mut self.snapshots);ManuallyDrop::drop(&mut self.mutations);}} }
}

impl<P:Send+Sync+'static,M:Send+Sync+'static> ArtifactDerivedOperation<P,M>{
    /// 📏️ Quotes one original semantic, settlement or cleanup action under its actual body hint.
    pub fn demands(&self,generation:u64,revision:[u8;32],body:usize,maximum_work_units:u64)->Result<RetirementDemand,ValueError> {
        if self.terminal_is_empty(){return Ok(Default::default())}
        if self.base.is_some(){return Ok(if self.closing{header::<SnapshotRead<P>>()}else{self.producer.as_ref().ok_or_else(||refusal("derived request lost its original producer issuer"))?.preparation_birth_demand()});}
        if let Some(task)=self.task.as_ref(){
            if self.closing||self.ready{return Ok(RetirementDemand{copy_bytes:size_of::<ArtifactReplayPreparationTask<P,M>>()+if self.ready&&!self.closing{size_of::<ArtifactReplayPrepared<P,M>>()}else{0},depth:1,..Default::default()});}
            return task.demands(generation,revision,body,maximum_work_units);
        }
        if let Some(close)=self.task_close.as_ref(){return if close.terminal_is_empty(){Ok(header::<ArtifactReplayPreparationRetirement<P,M>>())}else{close.next_demand(body)};}
        if self.retirement.is_some(){return semio_framework_value::factory_ticket_demands(self.retirement.as_ref().unwrap(),body);}
        if let Some(input)=self.failed_input.as_ref(){return if input.terminal_is_empty(){Ok(header::<Option<semio_framework_value::retained_clone::RetainedOwnedProjection<M>>>())}else{input.close_demand(body)};}
        if self.failed_read.is_some(){return Ok(RetirementDemand{copy_bytes:size_of::<ErasedSnapshotRead>()+size_of::<Arc<dyn semio_framework_value::FactoryRetirement>>(),capacity_bytes:ReturnedSnapshotReadRetirement::<P>::constructor_capacity_bytes(),depth:1,..Default::default()});}
        if self.mutation.is_some(){return Ok(RetirementDemand{capacity_bytes:semio_framework_value::FactoryOwnedRetirement::<M>::constructor_capacity_bytes(),depth:1,..Default::default()});}
        let Some(prepared)=self.prepared.as_ref()else{
            if self.producer.is_some(){return Ok(header::<Arc<dyn ArtifactReplayPreparationFactory<P,M>>>());}
            if self.snapshots.is_some()||self.mutations.is_some(){return Ok(header::<Arc<dyn semio_framework_value::FactoryRetirement>>());}
            if let Some(factory)=self.factory_close.iter().find_map(Option::as_ref){return if factory.terminal_is_empty(){Ok(header::<Option<semio_framework_value::FactoryAuthority>>())}else{factory.demands(body)};}
            if self.registry_close.is_some(){return snapshot_read_registry_retirement::snapshot_registry_alias_demands(&self.registry_close);}
            return Ok(header::<ArtifactDerivedOperation<P,M>>());
        };
        if !self.closing&&self.message<prepared.messages.len(){return Ok(header::<protocol::MutationMessage>());}
        if prepared.next.is_some()&&(self.closing||!self.accepted||prepared.apply_refusal.is_some()||prepared.input_refusal.is_some()){return Ok(semio_framework_value::factory::ArcRootRetirement::<P>::original_admission_demands(&prepared.next));}
        match &prepared.inverse{
            Err(_)=>return Ok(RetirementDemand{capacity_bytes:size_of::<semio_framework_value::retirement::controlled::ControlledRetirement<ValueError>>(),depth:1,..Default::default()}),
            Ok(inverse)if !inverse.is_empty()=>return Ok(RetirementDemand{copy_bytes:size_of::<M>(),depth:inverse.next_pop_depth_demand().map_err(|error|ValueError::literal(match error.kind{semio_framework_value_persistent::list::PagedListRefusalKind::OwnershipLimit=>ValueRefusalKind::OwnershipLimit,semio_framework_value_persistent::list::PagedListRefusalKind::AllocationFailed=>ValueRefusalKind::AllocationFailed,semio_framework_value_persistent::list::PagedListRefusalKind::InvariantViolated=>ValueRefusalKind::InvariantViolated},error.reason))?,..Default::default()}),
            Ok(inverse)if !inverse.terminal_is_empty()=>return Ok(RetirementDemand{release_bytes:inverse.next_release_allocation_bytes().map_err(|error|ValueError::literal(match error.kind{semio_framework_value_persistent::list::PagedListRefusalKind::OwnershipLimit=>ValueRefusalKind::OwnershipLimit,semio_framework_value_persistent::list::PagedListRefusalKind::AllocationFailed=>ValueRefusalKind::AllocationFailed,semio_framework_value_persistent::list::PagedListRefusalKind::InvariantViolated=>ValueRefusalKind::InvariantViolated},error.reason))?,depth:inverse.next_release_depth_demand().map_err(|error|ValueError::literal(match error.kind{semio_framework_value_persistent::list::PagedListRefusalKind::OwnershipLimit=>ValueRefusalKind::OwnershipLimit,semio_framework_value_persistent::list::PagedListRefusalKind::AllocationFailed=>ValueRefusalKind::AllocationFailed,semio_framework_value_persistent::list::PagedListRefusalKind::InvariantViolated=>ValueRefusalKind::InvariantViolated},error.reason))?,..Default::default()}),Ok(_)=>{}
        }
        if !self.closing&&self.mode==ArtifactDerivedOperationMode::History&&(prepared.apply_refusal.is_some()||prepared.input_refusal.is_some()){
            let old=prepared.messages.capacity()*size_of::<protocol::MutationMessage>();let needs=prepared.messages.len()==prepared.messages.capacity();let capacity=if needs{(prepared.messages.len()+1)*size_of::<protocol::MutationMessage>()}else{0};let code=if prepared.apply_refusal.is_none(){"mutation.invariant".len()}else{0};let demand=RetirementDemand{copy_bytes:size_of::<protocol::MutationMessage>()+size_of::<protocol::MutationApplyError>(),capacity_bytes:capacity+code,release_bytes:if needs{old}else{0},depth:1};
            return Ok(demand);
        }
        if prepared.apply_refusal.is_some(){return Ok(RetirementDemand{capacity_bytes:size_of::<semio_framework_value::retirement::controlled::ControlledRetirement<protocol::MutationApplyError>>(),depth:1,..Default::default()});}
        if prepared.input_refusal.is_some(){return Ok(RetirementDemand{capacity_bytes:size_of::<semio_framework_value::retirement::controlled::ControlledRetirement<String>>(),depth:1,..Default::default()});}
        if prepared.messages.capacity()!=0&&(self.closing||self.mode!=ArtifactDerivedOperationMode::History){return Ok(RetirementDemand{capacity_bytes:size_of::<semio_os_spr::command::MutationMessageLedgerRetirement>(),depth:1,..Default::default()});}
        Ok(header::<ArtifactReplayPrepared<P,M>>())
    }
    /// ⏯️ Advances exactly one funded action, leaving cancellation and incomplete work owned across turns.
    pub fn step(&mut self,generation:u64,revision:[u8;32],body:usize,grant:RetainedCloneGrant,receiving:&mut semio_framework_value_native_owned::native_decoding::NativeDecodeControl<'_>,deadline:&mut dyn FnMut()->bool)->Result<RetainedCloneStep,semio_framework_value::OriginalRetirementFailure>{
        if self.base.is_none()&&self.task.is_some()&&!self.closing&&!self.ready{return self.step_task(generation,revision,body,grant,receiving,deadline)}
        let turn=crate::native::probe_effect(receiving,grant,|control,effect|self.step_turn(generation,revision,effect,DerivedOperationTurn::Advance(control,body),deadline))?;
        match turn{None=>Ok(RetainedCloneStep::Progress(Default::default())),Some(Ok(step))=>Ok(step),Some(Err(semio_framework_value::RetirementPollExit::Pending(_)))=>Ok(RetainedCloneStep::Progress(Default::default())),Some(Err(semio_framework_value::RetirementPollExit::Failed(failure)))=>Err(failure)}
    }
    fn step_task(&mut self,generation:u64,revision:[u8;32],body:usize,grant:RetainedCloneGrant,receiving:&mut semio_framework_value_native_owned::native_decoding::NativeDecodeControl<'_>,deadline:&mut dyn FnMut()->bool)->Result<RetainedCloneStep,semio_framework_value::OriginalRetirementFailure>{
        self.last_work=0;let idle=RetainedCloneProgress::default();
        if self.terminal_is_empty(){return Ok(RetainedCloneStep::Complete(idle))}if deadline(){return Ok(RetainedCloneStep::Progress(idle));}
        let demand=self.demands(generation,revision,body,u64::MAX)?;if !funded(grant,demand){return Ok(RetainedCloneStep::Progress(idle));}
        let result=self.task.as_mut().unwrap().step(generation,revision,body,ArtifactStoreOneItemGrant{maximum_items:grant.maximum_items,maximum_copy_bytes:grant.maximum_copy_bytes,maximum_capacity_bytes:grant.maximum_capacity_bytes,maximum_release_bytes:grant.maximum_release_bytes,maximum_depth:grant.maximum_depth},receiving,deadline);self.last_work=self.task.as_ref().unwrap().last_turn_work();let step=result?;
        self.ready=matches!(step,ArtifactReplayPreparationStep::Prepared(_));let progress=step.progress();
        self.record(progress);if !progress.fits(grant){return Err((refusal("derived operation exceeded its independently admitted grant")).into());}
        Ok(if self.terminal_is_empty(){RetainedCloneStep::Complete(progress)}else{RetainedCloneStep::Progress(progress)})
    }
    fn close_original(&mut self,effect:Effect<'_,'_>)->semio_framework_value::OriginalRetirementPoll{let result=if !self.closing{Err(refusal("derived closure requires its original cancellation").into())}else{self.step_turn(self.context.generation,self.context.base_revision,effect,DerivedOperationTurn::Close(None),&mut||false)};Exit::settle(effect,result)}
    fn step_turn(&mut self,generation:u64,revision:[u8;32],effect:Effect<'_,'_>,mut turn:DerivedOperationTurn<'_,'_>,deadline:&mut dyn FnMut()->bool)->Result<RetainedCloneStep,Exit> {let grant=effect.grant();self.last_work=0;let result=(||->Result<RetainedCloneStep,Exit>{
        let idle=RetainedCloneProgress::default();if self.terminal_is_empty(){return Ok(RetainedCloneStep::Complete(idle))}if deadline(){return Ok(RetainedCloneStep::Progress(idle));}let body=match &turn{DerivedOperationTurn::Advance(_,body)=>*body,DerivedOperationTurn::Close(_)|DerivedOperationTurn::Original(_)=>grant.maximum_copy_bytes};let maximum_work_units=match &turn{DerivedOperationTurn::Advance(..)=>effect.maximum_work_units(),_=>0};let demand=self.demands(generation,revision,body,maximum_work_units)?;if !funded(grant,demand){return Ok(RetainedCloneStep::Progress(idle));}
        if matches!(turn,DerivedOperationTurn::Close(None))&&self.requires_original_control(){return Err((ValueError::literal(ValueRefusalKind::UnsupportedOwner,"derived semantic retirement requires its original decoding custody")).into());}
        if let Some(task)=self.task.as_ref(){match &mut turn{DerivedOperationTurn::Close(Some(control))=>task.validate_retirement_control(&**control)?,DerivedOperationTurn::Original(original)=>task.validate_retirement_control(&**original)?,_=>{}}}
        let progress=if self.base.is_some(){
            if self.closing{
                one(effect)?;*self.failed_read=Some(self.base.take().unwrap().into_erased());RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,..idle}
            }else{
                if generation!=self.context.generation||revision!=self.context.base_revision{return Err((refusal("derived producer birth moved from its original Store frontier")).into());}
                let DerivedOperationTurn::Advance(control,_)=&mut turn else{return Err((refusal("derived producer birth requires its original receiving authority")).into());};control.checkpoint()?;control.admit_turn_capacity(grant.maximum_capacity_bytes)?;if demand.capacity_bytes!=0{control.charge(demand.capacity_bytes)?;}
                let request=ArtifactReplayPreparationRequest{base:self.base.take().unwrap(),input:self.failed_input.take().ok_or_else(||refusal("derived producer lost its original input projection"))?,context:self.context,snapshots:Arc::clone(self.snapshots.as_ref().unwrap()),mutations:Arc::clone(self.mutations.as_ref().unwrap())};
                match self.producer.as_ref().unwrap().begin(request,grant){
                    Ok((cursor,p))=>{*self.task=Some(ArtifactReplayPreparationTask::new(self.context,cursor));if !p.fits(grant)||p.retained_capacity_bytes!=demand.capacity_bytes{self.cancel();self.record(p);return Err((refusal("derived producer changed its admitted birth receipt").with_retained_progress(p)).into());}p},
                    Err((error,request))=>{*self.base=Some(request.base);*self.failed_input=Some(request.input);self.factory_close[2]=Some(semio_framework_value::FactoryAuthority::new(request.snapshots));self.factory_close[3]=Some(semio_framework_value::FactoryAuthority::new(request.mutations));self.cancel();let p=error.retained_progress();self.record(p);return Err((error).into());}
                }
            }
        }else if self.task.is_some(){
            if self.closing||self.ready{
                one(effect)?;if self.ready&&!self.closing{let prepared=self.task.as_mut().unwrap().take_prepared().ok_or_else(||refusal("derived operation producer completed without its original owners"))?;*self.prepared=Some(prepared);}
                *self.task_close=Some(self.task.take().unwrap().into_retirement());RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,..idle}
            }else{
                return Err((refusal("derived task advance runs on its own original actions")).into());
            }
        }else if let Some(close)=self.task_close.as_mut(){
            if close.terminal_is_empty(){one(effect)?;drop(self.task_close.take());RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,..idle}}else{let poll=match turn{DerivedOperationTurn::Advance(control,_)=>control.with_retirement(grant,|original|Ok::<_,ValueError>(close.close_step(effect,original)))?,DerivedOperationTurn::Close(Some(original))=>close.close_step(effect,original),DerivedOperationTurn::Original(original)=>close.close_step(effect,original),DerivedOperationTurn::Close(None)=>return Err((ValueError::literal(ValueRefusalKind::UnsupportedOwner,"derived semantic retirement requires its original decoding custody")).into())};Exit::resume(poll)?.progress()}
        }else if self.retirement.is_some(){let poll=match turn{DerivedOperationTurn::Advance(control,_)=>control.with_retirement(grant,|original|Ok::<_,ValueError>(semio_framework_value::close_factory_ticket_controlled(&mut self.retirement,grant.maximum_copy_bytes,effect,original)))?,DerivedOperationTurn::Close(Some(original))=>semio_framework_value::close_factory_ticket_controlled(&mut self.retirement,grant.maximum_copy_bytes,effect,original),DerivedOperationTurn::Close(None)=>semio_framework_value::close_factory_ticket(&mut self.retirement,grant.maximum_copy_bytes,effect),DerivedOperationTurn::Original(original)=>semio_framework_value::close_factory_ticket_original(&mut self.retirement,original,grant.maximum_copy_bytes,effect)};Exit::resume(poll)?.progress()
        }else if let Some(input)=self.failed_input.as_mut(){if input.terminal_is_empty(){one(effect)?;self.failed_input.take();RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,..idle}}else{Exit::resume(input.close_step(grant.maximum_copy_bytes,effect))?.progress()}
        }else if let Some(read)=self.failed_read.as_mut(){let child=RetainedCloneGrant{maximum_copy_bytes:grant.maximum_copy_bytes-demand.copy_bytes,..grant};let Some((frame,mut p))=read.admit_retirement(self.registry.as_ref().unwrap(),self.snapshots.as_ref().unwrap(),&mut effect.nested(child)).result?else{return Ok(RetainedCloneStep::Progress(idle));};*self.retirement=Some(frame);self.failed_read.take();p.copied_bytes+=demand.copy_bytes;p
        }else if self.mutation.is_some(){Exit::resume(crate::native::lifted(semio_framework_value::FactoryOwnedRetirement::<M>::admit_original(&mut self.mutation,&mut self.mutations,&mut self.retirement,effect)))?.progress()
        }else if let Some(prepared)=self.prepared.as_mut(){
            if !self.closing&&self.message<prepared.messages.len(){one(effect)?;let row=&prepared.messages[self.message];if self.mode==ArtifactDerivedOperationMode::Provisional&&self.policy.rejects(row.level){self.accepted=false;}self.message+=1;RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,..idle}
            }else if prepared.next.is_some()&&(self.closing||!self.accepted||prepared.apply_refusal.is_some()||prepared.input_refusal.is_some()){
                Exit::resume(semio_framework_value::factory::ArcRootRetirement::<P>::admit_original(&mut prepared.next,&mut self.snapshots,&mut self.retirement,effect))?.progress()
            }else if prepared.inverse.is_err(){let Err(error)=std::mem::replace(&mut prepared.inverse,Ok(Default::default()))else{unreachable!()};let mut slot=Some(error);let turn=semio_framework_value::retirement::admit_owned_retirement(&mut slot,effect);match turn.result{Ok(Some((frame,p)))=>{*self.retirement=Some(frame);p},Ok(None)=>{prepared.inverse=Err(slot.take().unwrap());return Ok(RetainedCloneStep::Progress(idle));},Err(error)=>{if let Some(original)=slot.take(){prepared.inverse=Err(original);}return Err(error.into());}}
            }else if !prepared.inverse.as_ref().unwrap().is_empty(){one(effect)?;*self.mutation=prepared.inverse.as_mut().unwrap().pop();RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,..idle}
            }else if !prepared.inverse.as_ref().unwrap().terminal_is_empty(){use semio_framework_value_persistent::list::{PagedListReleaseOutcome,PagedListReleaseFailure,PagedListRefusalKind};let p=match prepared.inverse.as_mut().unwrap().release_empty_page(effect).result{Ok(PagedListReleaseOutcome::Ready(p))=>p,Ok(PagedListReleaseOutcome::Denied)=>return Ok(RetainedCloneStep::Progress(idle)),Err(PagedListReleaseFailure::Work(error))=>return Err(error.into()),Err(PagedListReleaseFailure::Release(error))=>return Err(ValueError::literal(match error.kind{PagedListRefusalKind::OwnershipLimit=>ValueRefusalKind::OwnershipLimit,PagedListRefusalKind::AllocationFailed=>ValueRefusalKind::AllocationFailed,PagedListRefusalKind::InvariantViolated=>ValueRefusalKind::InvariantViolated},error.reason).into())};RetainedCloneProgress{copied_items:usize::from(p.progressed),released_bytes:p.released_allocation_bytes,..idle}
            }else if !self.closing&&self.mode==ArtifactDerivedOperationMode::History&&(prepared.apply_refusal.is_some()||prepared.input_refusal.is_some()){
                if prepared.messages.len()==prepared.messages.capacity(){prepared.messages.try_reserve_exact(1).map_err(|_|ValueError::literal(ValueRefusalKind::AllocationFailed,"derived history diagnostic reservation failed"))?;}
                let row=if let Some(cause)=prepared.apply_refusal.take(){protocol::MutationMessage{level:semio_framework_diagnostic::Severity::Fatal,code:semio_framework_diagnostic::FaultCode(cause.code),message:cause.message,target:cause.target,op_index:None}}else{protocol::MutationMessage{level:semio_framework_diagnostic::Severity::Fatal,code:semio_framework_diagnostic::FaultCode("mutation.invariant".into()),message:prepared.input_refusal.take().unwrap(),target:Vec::new(),op_index:None}};prepared.messages.push(row);RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,retained_capacity_bytes:demand.capacity_bytes,released_bytes:demand.release_bytes}
            }else if prepared.apply_refusal.is_some(){let turn=semio_framework_value::retirement::admit_owned_retirement(&mut prepared.apply_refusal,effect);match turn.result?{Some((frame,p))=>{*self.retirement=Some(frame);p},None=>return Ok(RetainedCloneStep::Progress(idle))}
            }else if prepared.input_refusal.is_some(){let turn=semio_framework_value::retirement::admit_owned_retirement(&mut prepared.input_refusal,effect);match turn.result?{Some((frame,p))=>{*self.retirement=Some(frame);p},None=>return Ok(RetainedCloneStep::Progress(idle))}
            }else if prepared.messages.capacity()!=0&&(self.closing||self.mode!=ArtifactDerivedOperationMode::History){one(effect)?;match semio_framework_value::retirement::frame::admit_retirement_frame(std::mem::take(&mut prepared.messages),grant,|messages|semio_os_spr::command::MutationMessageLedgerRetirement::new(String::new(),messages)){Ok((frame,p))=>{*self.retirement=Some(frame);p},Err((error,original))=>{prepared.messages=original;return Err((error).into());}}
            }else{one(effect)?;if self.closing{drop(self.prepared.take());}else{self.finished=true;}RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,..idle}}
        }else if self.producer.is_some(){one(effect)?;let factory:Arc<dyn semio_framework_value::FactoryRetirement>=self.producer.take().unwrap();self.factory_close[4]=Some(semio_framework_value::FactoryAuthority::new(factory));RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,..idle}
        }else if self.snapshots.is_some(){one(effect)?;let factory:Arc<dyn semio_framework_value::FactoryRetirement>=self.snapshots.take().unwrap();self.factory_close[0]=Some(semio_framework_value::FactoryAuthority::new(factory));RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,..idle}
        }else if self.mutations.is_some(){one(effect)?;let factory:Arc<dyn semio_framework_value::FactoryRetirement>=self.mutations.take().unwrap();self.factory_close[1]=Some(semio_framework_value::FactoryAuthority::new(factory));RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,..idle}
        }else if let Some(index)=self.factory_close.iter().position(Option::is_some){if self.factory_close[index].as_ref().unwrap().terminal_is_empty(){one(effect)?;self.factory_close[index]=None;RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,..idle}}else{let factory=self.factory_close[index].as_mut().unwrap();let poll=if factory.requires_original_control(){match turn{DerivedOperationTurn::Advance(control,_)=>control.with_retirement(grant,|original|Ok::<_,ValueError>(factory.step_controlled(grant.maximum_copy_bytes,effect,original)))?,DerivedOperationTurn::Close(Some(original))=>factory.step_controlled(grant.maximum_copy_bytes,effect,original),DerivedOperationTurn::Original(original)=>factory.step_original(original,grant.maximum_copy_bytes,effect),DerivedOperationTurn::Close(None)=>factory.step(grant.maximum_copy_bytes,effect)}}else{factory.step(grant.maximum_copy_bytes,effect)};Exit::resume(poll)?.progress()}}else if self.registry_close.is_some(){Exit::resume(snapshot_read_registry_retirement::snapshot_registry_alias_close_step(&mut self.registry_close,effect))?.progress()
        }else{one(effect)?;if let Some(registry)=self.registry.take(){*self.registry_close=Some(snapshot_read_registry_retirement::SnapshotReadRegistryAliasRetirement::new(registry));}RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,..idle}};
        self.record(progress);if !progress.fits(grant){return Err((refusal("derived operation exceeded its independently admitted grant")).into());}Ok(if self.terminal_is_empty(){RetainedCloneStep::Complete(progress)}else{RetainedCloneStep::Progress(progress)})
    })();self.last_work=u64::try_from(effect.performed_work()).unwrap_or(u64::MAX);result
    }
}

impl<P:Send+Sync+'static,M:Send+Sync+'static> ErasedSnapshotRetirement for ArtifactDerivedOperation<P,M>{
    fn requires_original_control(&self)->bool{
        if self.base.is_some()||self.task.is_some(){return false}
        if let Some(close)=self.task_close.as_ref(){return !close.terminal_is_empty()}
        if let Some(owner)=self.retirement.as_ref(){return !owner.terminal_is_empty()&&owner.requires_original_control()}
        if self.failed_input.is_some()||self.failed_read.is_some()||self.mutation.is_some()||self.prepared.is_some()||self.producer.is_some()||self.snapshots.is_some()||self.mutations.is_some(){return false}
        self.factory_close.iter().find_map(Option::as_ref).is_some_and(|owner|!owner.terminal_is_empty()&&owner.requires_original_control())
    }
    fn next_computational_work(&self,body:usize)->Result<u64,ValueError>{
        if !self.closing{return Err(refusal("derived operation requires cancellation before close work selection"))}
        if self.terminal_is_empty(){return Ok(0)}
        if self.base.is_some()||self.task.is_some(){return Ok(1)}
        if let Some(close)=self.task_close.as_ref(){return if close.terminal_is_empty(){Ok(1)}else{close.next_computational_work(body)}}
        if let Some(owner)=self.retirement.as_ref(){return if owner.terminal_is_empty(){Ok(1)}else{owner.next_computational_work(body)}}
        if let Some(input)=self.failed_input.as_ref(){return if input.terminal_is_empty(){Ok(1)}else{input.close_work_demand(body)}}
        if self.failed_read.is_some(){return Err(ValueError::literal(ValueRefusalKind::UnsupportedOwner,"erased snapshot retirement admission lacks its original work quote"))}
        if self.mutation.is_some(){return Ok(semio_framework_value::FactoryOwnedRetirement::<M>::admit_original_source_work_demand(&self.mutation))}
        if let Some(prepared)=self.prepared.as_ref(){
            if prepared.next.is_some(){return Ok(semio_framework_value::factory::ArcRootRetirement::<P>::admit_original_source_work_demand(&prepared.next))}
            return Ok(1)
        }
        if self.producer.is_some()||self.snapshots.is_some()||self.mutations.is_some(){return Ok(1)}
        if let Some(factory)=self.factory_close.iter().find_map(Option::as_ref){return if factory.terminal_is_empty(){Ok(1)}else{factory.next_computational_work(body)}}
        if self.registry_close.is_some(){return snapshot_read_registry_retirement::snapshot_registry_alias_work_demand(&self.registry_close)}
        Ok(u64::from(self.registry.is_some()))
    }
    fn next_demand(&self,body:usize)->Result<RetirementDemand,ValueError>{if !self.closing{return Err(refusal("derived operation requires explicit cancellation before retirement"));}self.demands(self.context.generation,self.context.base_revision,body,0)}
    fn close_step(&mut self,_body:usize,effect:Effect<'_,'_>)->semio_framework_value::OriginalRetirementPoll{if !self.closing{return Exit::settle(effect,Err(refusal("derived operation requires explicit cancellation before retirement").into()));}self.close_original(effect)}
    fn close_step_controlled(&mut self,_body:usize,effect:Effect<'_,'_>,control:&mut dyn semio_framework_value::OriginalRetirementControl)->semio_framework_value::OriginalRetirementPoll{
        let result=(||->Result<RetainedCloneStep,Exit>{
            let grant=effect.grant();
            if !self.closing{return Err(refusal("derived operation requires explicit cancellation before retirement").into())}
            if self.terminal_is_empty(){return Ok(RetainedCloneStep::Complete(Default::default()))}
            if self.requires_original_control(){return self.step_turn(self.context.generation,self.context.base_revision,effect,DerivedOperationTurn::Close(Some(control)),&mut||false)}
            let d=self.next_demand(grant.maximum_copy_bytes)?;
            if !funded(grant,d){return Ok(RetainedCloneStep::Progress(Default::default()))}
            control.charge(d.capacity_bytes)?;
            self.step_turn(self.context.generation,self.context.base_revision,effect,DerivedOperationTurn::Close(None),&mut||false)
        })();
        Exit::settle(effect,result)
    }
    fn close_original_step(&mut self,original:&mut semio_framework_value::OriginalRetirementCloseLoan<'_>,_body:usize,effect:Effect<'_,'_>)->semio_framework_value::OriginalRetirementPoll{
        let result=(||->Result<RetainedCloneStep,Exit>{
            let grant=effect.grant();
            if !self.closing{return Err(refusal("derived operation requires explicit cancellation before retirement").into())}
            if self.terminal_is_empty(){return Ok(RetainedCloneStep::Complete(Default::default()))}
            if self.requires_original_control(){return self.step_turn(self.context.generation,self.context.base_revision,effect,DerivedOperationTurn::Original(original),&mut||false)}
            let d=self.next_demand(grant.maximum_copy_bytes)?;
            if !funded(grant,d){return Ok(RetainedCloneStep::Progress(Default::default()))}
            original.charge(d.capacity_bytes)?;
            self.step_turn(self.context.generation,self.context.base_revision,effect,DerivedOperationTurn::Close(None),&mut||false)
        })();
        Exit::settle(effect,result)
    }
    fn terminal_is_empty(&self)->bool{ArtifactDerivedOperation::terminal_is_empty(self)}
    fn next_copy_byte_demand(&self)->Result<usize,ValueError>{Ok(self.next_demand(4096)?.copy_bytes)}
    fn next_capacity_byte_demand(&self,body:usize)->Result<usize,ValueError>{Ok(self.next_demand(body)?.capacity_bytes)}
    fn next_release_byte_demand(&self)->Result<usize,ValueError>{Ok(self.next_demand(4096)?.release_bytes)}
    fn next_depth_demand(&self)->Result<usize,ValueError>{Ok(self.next_demand(4096)?.depth)}
}

#[cfg(test)]
#[path="🧪️tests/🦀️.rs"]
mod tests;
