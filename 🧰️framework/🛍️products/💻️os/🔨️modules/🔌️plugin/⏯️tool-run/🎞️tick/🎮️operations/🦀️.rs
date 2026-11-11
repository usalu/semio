//! 🎮️ Original encoded run operations retain native decoding and Store semantic preparation across caller turns.
extern crate semio_os_kernel as store;
extern crate semio_os_kernel as protocol;
use semio_framework_value::{RetirementDemand,RetainedCloneGrant,RetainedCloneProgress,RetainedCloneStep,ValueError,ValueRefusalKind};
use store::ArtifactStore;
use semio_framework_tool_run::TOOL_RUN_PROVISIONAL_OPS_MAX;
use semio_framework_job::{StepContext,OriginalTurnFailure,OriginalConstructionDemand};
use semio_framework_value::retained_clone::{RetainedCloneSourceAdmission,RetainedOwnedProjectionAdmission};
use semio_os_store_replay_operation::native::ReplyStash;
use semio_framework_value::{RetainedCloneSource,RetainedOwnedProjection,ErasedSnapshotRetirement,FactoryAuthority};
use semio_framework_value::retirement::controlled::ControlledRetirement;
use semio_framework_value_native_owned::native_decoding::{NativeDecodeControl,NativeConstructionStepFailure};
use std::{mem::size_of,sync::Arc};
type Decoder=(semio_framework_value_native_owned::NativeAllocationIdentity,u64);
/// 🪙️ One quoted construction runs inside one actual original effect: its work is the performed receipt, debited once, and its physical receipt is retained; `false` retries on a fresh turn.
fn built<O>(cx:&mut semio_framework_job::StepContext<'_>,output:&mut Option<O>,demand:semio_framework_job::OriginalConstructionDemand,construct:impl FnOnce()->Result<(O,semio_framework_value::RetainedCloneProgress),semio_framework_value::ValueError>)->Result<bool,semio_framework_job::OriginalTurnFailure>{built_with(cx,output,demand,|_|construct())}
fn built_with<O>(cx:&mut semio_framework_job::StepContext<'_>,output:&mut Option<O>,demand:semio_framework_job::OriginalConstructionDemand,construct:impl FnOnce(Option<Decoder>)->Result<(O,semio_framework_value::RetainedCloneProgress),semio_framework_value::ValueError>)->Result<bool,semio_framework_job::OriginalTurnFailure>{
 built_effect(cx,output,demand,|identity,effect|{if !effect.performed(1)?{return Err(semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::WorkLimit,"original tool construction is not funded"))}construct(identity)})
}
/// 🪙️ Runs a quoted action inside one actual original effect; the action debits its own work on the effect and the turn is charged exactly that receipt.
fn built_effect<O>(cx:&mut semio_framework_job::StepContext<'_>,output:&mut Option<O>,demand:semio_framework_job::OriginalConstructionDemand,construct:impl FnOnce(Option<Decoder>,&mut semio_framework_value::RetainedAllocationEffect<'_>)->Result<(O,semio_framework_value::RetainedCloneProgress),semio_framework_value::ValueError>)->Result<bool,semio_framework_job::OriginalTurnFailure>{
 let physical=semio_framework_value::RetirementDemand{copy_bytes:demand.copy_bytes,capacity_bytes:demand.capacity_bytes,release_bytes:demand.release_bytes,depth:demand.depth,..Default::default()};
 if cx.should_yield()||!cx.retained_work_can_enter(physical){return Ok(false)}
 let identity=cx.native_receiving_identity();let grant=cx.retained_grant();
 let Some((result,performed))=cx.with_original_effect(grant,|_,effect|construct(identity,effect))?else{return Ok(false)};
 let progress=match &result{Ok((_,progress))=>*progress,Err(error)=>error.retained_progress()};
 cx.consume_retained(progress)?;cx.consume_fuel(u64::try_from(performed).unwrap_or(u64::MAX));
 *output=Some(result?.0);Ok(true)
}
/// 🪙️ A paid action that debits its own work on the original effect.
pub fn paid_effect(cx:&mut semio_framework_job::StepContext<'_>,d:semio_framework_value::RetirementDemand,run:impl FnOnce(&mut semio_framework_value::RetainedAllocationEffect<'_>)->Result<semio_framework_value::RetainedCloneProgress,semio_framework_value::ValueError>)->Result<bool,semio_framework_job::OriginalTurnFailure>{let mut done=None;built_effect(cx,&mut done,semio_framework_job::OriginalConstructionDemand{items:1,copy_bytes:d.copy_bytes,capacity_bytes:d.capacity_bytes,release_bytes:d.release_bytes,depth:d.depth},|_,effect|run(effect).map(|p|((),p)))?;Ok(done.is_some())}
/// 🏁️ A settled grant-shaped child turn as a close poll.
fn settled_poll<T>(turn:semio_framework_value::RetainedWorkOutcome<T,semio_framework_value::ValueError>)->semio_framework_value::RetainedWorkPoll<T,semio_framework_value::OriginalRetirementFailure>{semio_framework_value::RetainedWorkPoll::Complete(semio_framework_value::RetainedWorkOutcome{result:turn.result.map_err(Into::into),work:turn.work})}
fn retired(cx:&mut semio_framework_job::StepContext<'_>,demand:semio_framework_value::RetirementDemand,advance:impl FnOnce(semio_framework_value::RetainedCloneGrant)->Result<semio_framework_value::RetainedCloneStep,semio_framework_value::ValueError>)->Result<semio_framework_job::InteractiveJobCloseStep,semio_framework_job::OriginalTurnFailure>{semio_framework_job::close_original_retained_step(cx,demand,|grant|semio_framework_value::RetainedWorkOutcome{result:advance(grant),work:1})}
type Source=RetainedCloneSource<Vec<u8>>;
type Receiver<M>=Box<dyn semio_os_store_operation_wire::ArtifactOriginalOperationReceiving<M>>;
type Derived<P>=crate::store::os_store::snapshot_read::ArtifactDerivedSnapshot<P>;
type Semantic<P,M>=crate::store::os_store::ArtifactDerivedOperation<P,M>;
type Input<M>=crate::store::os_store::ArtifactMutationSource<M>;
/// 🧮️ Retirement demand of an optional erased snapshot owner, one level deeper than the owner itself.
pub fn tool_erased_retirement_demand(owner:&Option<Box<dyn store::ErasedSnapshotRetirement>>,body:usize)->Result<RetirementDemand,ValueError>{
 let Some(owner)=owner.as_ref()else{return Ok(Default::default())};
 if owner.terminal_is_empty(){return Ok(RetirementDemand{release_bytes:size_of_val(owner.as_ref()),copy_bytes:size_of::<Option<Box<dyn store::ErasedSnapshotRetirement>>>(),depth:1,..Default::default()});}
 Ok({let mut demand=owner.next_demand(body)?;demand.depth=demand.depth.checked_add(1).ok_or_else(||ValueError::literal(ValueRefusalKind::DepthLimit,"tool retained close depth overflow"))?;demand})
}
fn invalid(message:&'static str)->ValueError{ValueError::literal(ValueRefusalKind::InvariantViolated,message)}
fn header<T>()->RetirementDemand{RetirementDemand{copy_bytes:2*size_of::<T>(),depth:1,..Default::default()}}
fn fits(cx:&StepContext<'_>,d:RetirementDemand)->bool{semio_framework_value::retained_clone::retained_clone_grant_admits(cx.retained_grant(),d)}
pub fn paid(cx:&mut StepContext<'_>,d:RetirementDemand,run:impl FnOnce(RetainedCloneGrant)->Result<RetainedCloneProgress,ValueError>)->Result<bool,OriginalTurnFailure>{let grant=cx.retained_grant();let mut done=None;built(cx,&mut done,OriginalConstructionDemand{items:1,copy_bytes:d.copy_bytes,capacity_bytes:d.capacity_bytes,release_bytes:d.release_bytes,depth:d.depth},||run(grant).map(|p|((),p)))?;Ok(done.is_some())}
/// 🧯️ Any native receiving failure as its first typed cause carrying the physical receipt it already spent.
fn receiving_fault<E:semio_framework_value_native_owned::native_decoding::NativeReceivingEffectError>(error:E)->ValueError{let progress=error.receiving_progress();error.cause().cloned().unwrap_or_else(||ValueError::invariant("native receiving failed without a typed cause")).with_retained_progress(progress)}
/// 🫴️ One native receiving turn of a polled cursor: the context loans the decoder control, the cursor reports its own work, and the settled turn is posted once against the issued grant.
pub fn receive<E:semio_framework_value_native_owned::native_decoding::NativeReceivingEffectError>(cx:&mut StepContext<'_>,d:RetirementDemand,run:impl FnOnce(&mut NativeDecodeControl<'_>,u64,RetainedCloneGrant)->semio_framework_value::RetainedWorkPoll<RetainedCloneStep,E>)->Result<(),OriginalTurnFailure>{
 if cx.should_yield()||!fits(cx,d){return Ok(())}
 let maximum=cx.fuel_remaining();let mut out=None;let mut polled=None;
 cx.with_native_receiving(&mut out,|native,grant|{polled=Some(run(native,maximum,grant));Ok(())})?;
 match polled{Some(semio_framework_value::RetainedWorkPoll::Complete(turn))=>{semio_framework_job::receive_original_receiving_work(cx,semio_framework_value::RetainedWorkOutcome{result:turn.result.map_err(receiving_fault),work:turn.work})?;Ok(())},Some(semio_framework_value::RetainedWorkPoll::Pending{work})=>{cx.consume_fuel(u64::try_from(work).unwrap_or(u64::MAX));Ok(())},None=>Ok(())}
}
/// 🫴️ One native receiving turn of an effect-shaped cursor inside the context's receiving frame.
pub fn receive_effect<E:semio_framework_value_native_owned::native_decoding::NativeReceivingEffectError>(cx:&mut StepContext<'_>,d:RetirementDemand,run:impl FnOnce(&mut NativeDecodeControl<'_>,&mut semio_framework_value::RetainedAllocationEffect<'_>)->semio_framework_value::RetainedWorkOutcome<RetainedCloneStep,E>)->Result<(),OriginalTurnFailure>{
 use semio_framework_value_native_owned::native_decoding::{NativeReceivingEffect,NativeReceivingEffectPoll};
 if cx.should_yield()||!fits(cx,d){return Ok(())}
 let maximum=cx.fuel_remaining();let mut out=None;let mut frame=NativeReceivingEffect::<E>::default();let mut done=None;
 cx.with_native_receiving(&mut out,|native,_|{done=Some(native.receive_effect(d,maximum,&mut frame,run)?);Ok(())})?;
 match done{Some(NativeReceivingEffectPoll::Complete(turn))=>{semio_framework_job::receive_original_receiving_work(cx,semio_framework_value::RetainedWorkOutcome{result:turn.result.map_err(receiving_fault),work:turn.work})?;Ok(())},Some(_)=>Err(invalid("synchronous native receiving stayed pending").into()),None=>Ok(())}
}

/// 🧳️ Every row is the same owned wire source received from the original job page.
pub struct OriginalToolRunOperations<P,M:Send+Sync+'static>{
 rows:Vec<Source>,
 row_admission:Option<RetainedCloneSourceAdmission<Vec<u8>,Vec<u8>,()>>,
 retiring_wire:Option<Source>,
 projection:Option<RetainedOwnedProjection<Vec<u8>>>,
 projecting:RetainedOwnedProjectionAdmission<Vec<u8>>,
 admit_reply:ReplyStash<ValueError>,
 take_reply:ReplyStash<ValueError>,
 admitted:bool,
 receiver:Option<Receiver<M>>,
 mutation:Option<M>,
 source:Option<Input<M>>,
 input:Option<RetainedOwnedProjection<M>>,
 overlay:OriginalToolOverlayFold<P,M>,
 factory:Option<Arc<dyn crate::store::ArtifactOwnedValueRetirementFactory<M>>>,
 factory_close:Option<FactoryAuthority>,
 index:usize,
 incoming:usize,
 phase:u8,
 ingest:u8,
 taking:bool,
 resetting:bool,
 closing:bool,
 finalizing:bool,
}
impl<P,M:Send+Sync+'static> Default for OriginalToolRunOperations<P,M>{fn default()->Self{Self{rows:Vec::new(),row_admission:None,retiring_wire:None,projection:None,projecting:Default::default(),admit_reply:Default::default(),take_reply:Default::default(),admitted:false,receiver:None,mutation:None,source:None,input:None,factory:None,factory_close:None,index:0,incoming:0,phase:0,ingest:0,taking:false,overlay:OriginalToolOverlayFold::default(),resetting:false,closing:false,finalizing:false}}}
impl<P,M> OriginalToolRunOperations<P,M> where P: semio_os_kernel::StoreState,M: semio_os_kernel::StoreMutationFor<P>{
 /// 🧭️ Exposes only borrowed scalar stages for actual bounded-progress regression witnesses.
 #[cfg(feature="artifact-app-testing")]
 pub fn diagnostic_cursor(&self,store:&ArtifactStore<P,M>,maximum_work_units:u64)->Result<(usize,u8,u8,u32,RetirementDemand),ValueError>{let flags=u32::from(self.mutation.is_some())|u32::from(self.source.is_some())<<1|u32::from(self.input.is_some())<<2|u32::from(self.receiver.is_some())<<3|u32::from(self.factory.is_some())<<4|u32::from(self.overlay.messages.is_some())<<5|u32::from(self.projection.is_some())<<6|u32::from(self.overlay.retirement.is_some())<<7|u32::from(self.overlay.alias.is_some())<<8|u32::from(self.overlay.base.is_some())<<9|u32::from(self.overlay.running.is_some())<<10|u32::from(self.resetting)<<11;let d=if let Some(owner)=self.overlay.semantic.as_ref(){if owner.is_closing(){owner.next_demand(4096)?}else{store.derived_operation_demands(owner,4096,maximum_work_units)?}}else if let Some(owner)=self.receiver.as_ref(){if self.mutation.is_some(){owner.close_demands(4096)?}else if self.taking{owner.take_owned_demands(4096)?}else{owner.demands(4096)?}}else{Default::default()};Ok((self.index,self.phase,self.ingest,flags,d))}
 /// 🔢️ Cardinality follows the retained encoded rows, independently of decoding progress.
 pub fn len(&self)->usize{self.rows.len()}
 /// 🔁️ Reuses the installed native Store producer and atomically exposes only the complete overlay.
 pub fn refold(&mut self,store:&ArtifactStore<P,M>,overlay:&mut Arc<P>,cx:&mut StepContext<'_>)->Result<bool,OriginalTurnFailure>{if !self.fold(store,cx)?{return Ok(false)}self.overlay.publish(store,overlay,cx)}
 /// 🧷️ The fixture inspects the original retained byte address without minting another source.
 #[cfg(feature="artifact-app-testing")]
 pub fn original_wire(&self,rank:usize)->Option<&[u8]>{self.rows.get(rank).map(|row|row.borrow().get().as_slice())}
 /// 🔄️ A moved Store frontier restarts only after the current original producer has closed.
 pub fn rebase(&mut self){self.resetting=true;self.overlay.dirty=true;}
 /// 📤️ Finalization receives each retained original wire row once into the publication's owned mutation vector.
 pub fn prepare_finalization(&mut self,store:&ArtifactStore<P,M>,output:&mut Vec<M>,cx:&mut StepContext<'_>)->Result<bool,OriginalTurnFailure>{
  if !self.finalizing{if !self.close_active(store,cx)?{return Ok(false)}let d=header::<(bool,usize,u8)>();paid(cx,d,|_|{self.finalizing=true;self.index=output.len();self.phase=0;Ok(RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,..Default::default()})})?;return Ok(false)}
  if self.phase==4{if self.close_active(store,cx)?{self.phase=0;self.index=output.len();}return Ok(false)}
  if output.len()==self.rows.len(){return Ok(true)}
  if !self.decode(store,cx)?{return Ok(false)}
  if output.len()==output.capacity(){let old=output.capacity()*size_of::<M>();let capacity=(output.len()+1).checked_mul(size_of::<M>()).ok_or_else(||invalid("native final operation vector extent overflow"))?;let d=RetirementDemand{capacity_bytes:capacity,release_bytes:old,..header::<Vec<M>>()};paid(cx,d,|_|{output.try_reserve_exact(1).map_err(|_|invalid("native final operation backing refused"))?;Ok(RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,retained_capacity_bytes:output.capacity()*size_of::<M>(),released_bytes:old})})?;return Ok(false)}
  let d=header::<M>();paid(cx,d,|_|{output.push(self.mutation.take().unwrap());self.phase=4;Ok(RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,..Default::default()})})?;Ok(false)
 }
 /// 📥️ Moves one original wire owner per funded event before the tick can acknowledge its producer.
 pub fn ingest(&mut self,tick:&mut semio_framework_tool_run::ToolRunTick,store:&ArtifactStore<P,M>,overlay:&mut Arc<P>,cx:&mut StepContext<'_>)->Result<bool,OriginalTurnFailure>{
  if self.ingest==0{if tick.retract_to.is_some_and(|length|(length as usize)<self.rows.len()){self.rebase();self.ingest=3;return Ok(false)}self.ingest=1;}
  if self.ingest==3{if self.resetting{self.reset(store,cx)?;return Ok(false)}if self.retiring_wire.is_some(){self.close_wire(cx)?;return Ok(false)}if self.rows.len()>(match tick.retract_to{Some(length)=>length as usize,None=>return Err(invalid("original retraction lost its length").into())}){let d=header::<Option<Source>>();close_paid(cx,d,|_|{self.retiring_wire=self.rows.pop();Ok(RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,..Default::default()})})?;return Ok(false)}self.ingest=1;}
  if self.ingest==1{if self.incoming<tick.append_ops.len(){if self.rows.len()>=TOOL_RUN_PROVISIONAL_OPS_MAX as usize{return Err(invalid("original run operation count exceeds its declared bound").into())}if self.rows.len()==self.rows.capacity(){let old=self.rows.capacity()*size_of::<Source>();let capacity=(self.rows.len()+1).checked_mul(size_of::<Source>()).ok_or_else(||invalid("original operation row backing overflow"))?;let d=RetirementDemand{capacity_bytes:capacity,release_bytes:old,..header::<Vec<Source>>()};paid(cx,d,|_|{self.rows.try_reserve_exact(1).map_err(|_|invalid("original operation row backing refused"))?;Ok(RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,retained_capacity_bytes:self.rows.capacity()*size_of::<Source>(),released_bytes:old})})?;return Ok(false)}let birth=Source::owned_constructor_demand::<()>();let copied=Source::constructor_copy_bytes()+2*size_of::<Vec<u8>>();let d=RetirementDemand{copy_bytes:copied,capacity_bytes:birth.capacity_bytes,depth:birth.depth,..Default::default()};paid_effect(cx,d,|effect|{let grant=effect.grant();if self.row_admission.is_none(){let original=std::mem::take(&mut tick.append_ops[self.incoming]);self.row_admission=Some(RetainedCloneSource::<Vec<u8>,Vec<u8>>::admission_owned(original,()));return Ok(RetainedCloneProgress{copied_items:1,copied_bytes:size_of::<Vec<u8>>(),..Default::default()})}let ready=self.row_admission.as_ref().unwrap().phase()=="ready";let mut slot=None;let turn=if ready{self.row_admission.as_mut().unwrap().take_output(&mut slot,effect)}else{self.row_admission.as_mut().unwrap().advance(effect)};let step=turn.result?;if let Some(source)=slot{self.rows.push(source);self.incoming+=1;self.overlay.dirty=true;}if self.row_admission.as_ref().is_some_and(|admission|admission.terminal_is_empty()){self.row_admission=None}Ok(step.progress())})?;return Ok(false)}self.ingest=2;}
  if self.ingest==2{if !self.refold(store,overlay,cx)?{return Ok(false)}self.ingest=0;self.incoming=0;return Ok(true)}Ok(false)
 }
 fn fold(&mut self,store:&ArtifactStore<P,M>,cx:&mut StepContext<'_>)->Result<bool,OriginalTurnFailure>{
  if self.resetting{self.reset(store,cx)?;return Ok(false)}
  if self.overlay.alias.is_some()||self.overlay.retirement.is_some(){self.close_alias(store,cx)?;return Ok(false)}
  if self.overlay.base.is_none(){if self.rows.is_empty()&&!self.overlay.dirty{return Ok(true)}let d=header::<Derived<P>>();paid(cx,d,|_|{self.overlay.base=Some(store.derived_snapshot_head());Ok(RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,..Default::default()})})?;return Ok(false)}
  if self.index==self.rows.len(){return Ok(true)}
  match self.phase{
   0=>{if self.decode(store,cx)?{self.phase=1;}},
   1=>{if self.source.is_none(){let factory=store.owned_mutation_retirement_factory()?;let d=Input::<M>::constructor_demand();paid(cx,d,|grant|{let Some((source,p))=Input::admit(&mut self.mutation,&factory,grant)?else{return Ok(Default::default())};self.source=Some(source);Ok(p)})?;return Ok(false)}if self.input.is_none()&&!self.overlay.input_pending(){let source=self.source.as_mut().unwrap();let d=source.demands(4096)?;if source.ready(){paid_effect(cx,d,|effect|{let grant=effect.grant();source.take(&mut self.input,grant).map(|p|p.unwrap_or_default())})?;}else{receive(cx,d,|native,_maximum,grant|semio_framework_value::RetainedWorkPoll::Complete(semio_framework_value::RetainedWorkOutcome{result:source.advance_chained(native,grant),work:0}))?;}return Ok(false)}if self.overlay.advance_input(store,&mut self.input,self.index as u32,cx)?{self.phase=3;}},

   3=>{if self.close_active(store,cx)?{self.phase=0;self.index+=1;}},
   _=>return Err(invalid("original operation fold phase is invalid").into()),
  }Ok(false)
 }
 fn decode(&mut self,store:&ArtifactStore<P,M>,cx:&mut StepContext<'_>)->Result<bool,OriginalTurnFailure>{
  if let Some(receiver)=self.receiver.as_mut(){if !self.taking&&!receiver.ready()&&self.mutation.is_none(){let d=receiver.demands(4096)?;receive(cx,d,|native,maximum,grant|receiver.advance(native,4096,maximum,grant))?;return Ok(false)}if self.mutation.is_none(){self.taking=true;let d=receiver.take_owned_demands(4096)?;receive_effect(cx,d,|native,effect|receiver.take_owned_into(&mut self.mutation,native,4096,effect))?;return Ok(false)}receiver.begin_close();if !receiver.terminal_is_empty(){let d=receiver.close_demands(4096)?;close_native(cx,d,|native,effect|receiver.close_step(native,effect))?;return Ok(false)}let d=RetirementDemand{release_bytes:receiver.allocation_bytes(),..header::<Option<Receiver<M>>>()};paid(cx,d,|_|{self.receiver=None;self.taking=false;Ok(RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,released_bytes:d.release_bytes,..Default::default()})})?;return Ok(false)}
  if self.mutation.is_some(){return Ok(true)}
  if self.projection.is_none(){
   if !self.admitted{let d=self.projecting.demand(self.rows[self.index].try_borrow()?)?;paid_effect(cx,d,|effect|{let original=self.rows[self.index].try_borrow()?;let admission=&mut self.projecting;let Some(answer)=self.admit_reply.run(|reply,recover|(if recover{admission.recover_admit(reply)}else{admission.admit(original,effect,reply)}).map(drop))?else{return Ok(Default::default())};let step=answer?;if matches!(step,RetainedCloneStep::Complete(_)){self.admitted=true;}Ok(step.progress())})?;return Ok(false)}
   let d=self.projecting.take_demand()?;paid_effect(cx,d,|effect|{let admission=&mut self.projecting;let slot=&mut self.projection;let Some(answer)=self.take_reply.run(|reply,recover|(if recover{admission.recover_take(reply)}else{admission.take(slot,effect,reply)}).map(drop))?else{return Ok(Default::default())};let step=answer?;if matches!(step,RetainedCloneStep::Complete(_)){self.admitted=false;}Ok(step.progress())})?;return Ok(false)
  }
  if self.factory.is_none(){let d=header::<Arc<dyn crate::store::ArtifactOwnedValueRetirementFactory<M>>>();paid(cx,d,|_|{self.factory=Some(Arc::clone(store.owned_mutation_retirement_factory()?));Ok(RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,..Default::default()})})?;return Ok(false)}
  let codec=store.operation_wire().ok_or_else(||invalid("original Store operation wire is absent"))?;let admission=codec.original_receiving.ok_or_else(||invalid("original Store operation receiver is absent"))?;receive(cx,(admission.demands)(),|native,_maximum,grant|{let result=(||->Result<RetainedCloneStep,ValueError>{(admission.admit)(&mut self.projection,&mut self.receiver,self.factory.as_ref().ok_or_else(||ValueError::invariant("factory absent"))?,native,grant).map(RetainedCloneStep::Progress)})();semio_framework_value::RetainedWorkPoll::Complete(semio_framework_value::RetainedWorkOutcome{result,work:1})})?;Ok(false)
 }
 fn step_row(&mut self,cx:&mut StepContext<'_>)->Result<bool,OriginalTurnFailure>{
  let admission=self.row_admission.as_ref().ok_or_else(||invalid("original operation row admission is absent"))?;
  let d=if admission.phase()=="ready"{header::<Source>()}else{admission.next_demand()?};
  paid_effect(cx,d,|effect|{let grant=effect.grant();let admission=self.row_admission.as_mut().ok_or_else(||invalid("original operation row admission is absent"))?;if admission.phase()!="ready"{return Ok(admission.advance(effect).result?.progress())}let mut output=None;let step=admission.take_output(&mut output,effect).result?;if let Some(source)=output{self.rows.push(source);self.incoming+=1;self.overlay.dirty=true;self.row_admission=None;}Ok(step.progress())})
 }
 fn close_active(&mut self,store:&ArtifactStore<P,M>,cx:&mut StepContext<'_>)->Result<bool,OriginalTurnFailure>{
  if !self.overlay.cancel_input(cx)?{return Ok(false)}
  if self.row_admission.is_some(){self.step_row(cx)?;return Ok(false)}
  if let Some(owner)=self.receiver.as_mut(){owner.begin_close();if owner.terminal_is_empty(){let d=RetirementDemand{release_bytes:owner.allocation_bytes(),..header::<Option<Receiver<M>>>()};close_paid(cx,d,|_|{self.receiver=None;self.taking=false;Ok(RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,released_bytes:d.release_bytes,..Default::default()})})?;}else{let d=owner.close_demands(4096)?;close_native(cx,d,|native,effect|owner.close_step(native,effect))?;}return Ok(false)}

  if self.overlay.retirement.is_some(){let d=tool_erased_retirement_demand(&self.overlay.retirement,4096)?;close_native(cx,d,|native,effect|semio_framework_value::close_factory_ticket_controlled(&mut self.overlay.retirement,4096,effect,native))?;return Ok(false)}
  if let Some(owner)=self.input.as_mut(){if owner.terminal_is_empty(){let d=header::<Option<RetainedOwnedProjection<M>>>();close_paid(cx,d,|_|{self.input=None;Ok(RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,..Default::default()})})?;}else{let d=owner.close_demand(4096)?;semio_framework_job::close_original_effect_step(cx,d,|effect|owner.close_step(4096,effect))?;}return Ok(false)}
  if let Some(owner)=self.source.as_mut(){owner.begin_close();if owner.terminal_is_empty(){let d=header::<Option<Input<M>>>();close_paid(cx,d,|_|{self.source=None;Ok(RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,..Default::default()})})?;}else{let d=owner.close_demand(4096)?;semio_framework_job::close_original_effect_step(cx,d,|effect|{let result=owner.close_step(effect);settled_poll(semio_framework_value::RetainedWorkOutcome{result,work:effect.performed_work()})})?;}return Ok(false)}
  if self.mutation.is_some(){let d=RetirementDemand{capacity_bytes:store.owned_mutation_retirement_birth_bytes()?,..header::<Option<M>>()};semio_framework_job::close_original_effect_step(cx,d,|effect|{let result=(||{let p=store.retire_owned_mutation(&mut self.mutation,effect)?.map_or(Default::default(),|(owner,p)|{self.overlay.retirement=Some(owner);p});Ok(RetainedCloneStep::Progress(p))})();settled_poll(semio_framework_value::RetainedWorkOutcome{result,work:effect.performed_work()})})?;return Ok(false)}
  if !self.projecting.terminal_is_empty(){let d=self.projecting.close_demand(4096)?;semio_framework_job::close_original_effect_step(cx,d,|effect|self.projecting.close_step(4096,effect))?;return Ok(false)}
  if let Some(owner)=self.projection.as_mut(){if owner.terminal_is_empty(){let d=header::<Option<RetainedOwnedProjection<Vec<u8>>>>();close_paid(cx,d,|_|{self.projection=None;Ok(RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,..Default::default()})})?;}else{let d=owner.close_demand(4096)?;semio_framework_job::close_original_effect_step(cx,d,|effect|owner.close_step(4096,effect))?;}return Ok(false)}
  if self.factory.is_some(){let d=header::<FactoryAuthority>();close_paid(cx,d,|_|{self.factory_close=Some(FactoryAuthority::new(self.factory.take().unwrap()));Ok(RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,..Default::default()})})?;return Ok(false)}
  if let Some(owner)=self.factory_close.as_mut(){if owner.terminal_is_empty(){let d=header::<Option<FactoryAuthority>>();close_paid(cx,d,|_|{self.factory_close=None;Ok(RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,..Default::default()})})?;}else{let d=owner.demands(4096)?;semio_framework_job::close_original_effect_step(cx,d,|effect|owner.step(4096,effect))?;}return Ok(false)}
  Ok(true)
 }
 fn close_alias(&mut self,store:&ArtifactStore<P,M>,cx:&mut StepContext<'_>)->Result<(),OriginalTurnFailure>{self.overlay.close_alias(store,cx)}
 fn close_wire(&mut self,cx:&mut StepContext<'_>)->Result<(),OriginalTurnFailure>{if let Some(source)=self.retiring_wire.as_mut(){if source.terminal_is_empty(){let d=header::<Option<Source>>();close_paid(cx,d,|_|{self.retiring_wire=None;Ok(RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,..Default::default()})})?;}else{let d=source.next_close_demand(4096)?;semio_framework_job::close_original_effect_step(cx,d,|effect|source.close_step(4096,effect))?;}}Ok(())}
 fn reset(&mut self,store:&ArtifactStore<P,M>,cx:&mut StepContext<'_>)->Result<bool,OriginalTurnFailure>{if !self.close_active(store,cx)?{return Ok(false)}if self.overlay.alias.is_some()||self.overlay.retirement.is_some(){self.close_alias(store,cx)?;return Ok(false)}if self.overlay.running.is_some()||self.overlay.base.is_some(){let d=header::<Option<Derived<P>>>();close_paid(cx,d,|_|{self.overlay.alias=self.overlay.running.take().or_else(||self.overlay.base.take()).map(Derived::into_snapshot_owner);Ok(RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,..Default::default()})})?;return Ok(false)}let d=header::<(usize,u8,bool)>();close_paid(cx,d,|_|{self.index=0;self.phase=0;self.resetting=false;Ok(RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,..Default::default()})})?;Ok(!self.resetting)}
 /// ♻️ The original decoder and installed Store issuers close before retained encoded rows retire.
 pub fn close_step(&mut self,store:&ArtifactStore<P,M>,cx:&mut StepContext<'_>)->Result<bool,OriginalTurnFailure>{if self.row_admission.as_ref().is_some_and(|admission|!admission.terminal_is_empty()){let d=self.row_admission.as_ref().unwrap().next_return_demand()?;let mut sink=None;semio_framework_job::close_original_effect_step(cx,d,|effect|{let turn=self.row_admission.as_mut().unwrap().return_inputs(&mut sink,effect);if self.row_admission.as_ref().is_some_and(|admission|admission.terminal_is_empty()){self.row_admission=None}settled_poll(turn)})?;return Ok(false)}if self.row_admission.is_some(){self.row_admission=None;}if !self.closing{self.closing=true;self.resetting=true;}if self.resetting{self.reset(store,cx)?;return Ok(false)}if self.retiring_wire.is_some(){self.close_wire(cx)?;return Ok(false)}if !self.rows.is_empty(){let d=header::<Option<Source>>();close_paid(cx,d,|_|{self.retiring_wire=self.rows.pop();Ok(RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,..Default::default()})})?;return Ok(false)}if self.rows.capacity()!=0{let d=RetirementDemand{release_bytes:self.rows.capacity()*size_of::<Source>(),..header::<Vec<Source>>()};close_paid(cx,d,|_|{drop(std::mem::take(&mut self.rows));Ok(RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,released_bytes:d.release_bytes,..Default::default()})})?;return Ok(false)}Ok(self.terminal_is_empty())}
 /// 📏️ Quotes the actual selected original owner before cancellation or entry retirement advances it.
 pub fn close_demands(&self,store:&ArtifactStore<P,M>,body:usize)->Result<RetirementDemand,ValueError>{
  if self.overlay.input_pending(){return self.overlay.input_close_demands(body)}
  if let Some(owner)=self.receiver.as_ref(){return if owner.terminal_is_empty(){Ok(RetirementDemand{release_bytes:owner.allocation_bytes(),..header::<Option<Receiver<M>>>()})}else{owner.close_demands(body)}}

  if self.overlay.retirement.is_some(){return tool_erased_retirement_demand(&self.overlay.retirement,body)}
  if let Some(owner)=self.input.as_ref(){return if owner.terminal_is_empty(){Ok(header::<Option<RetainedOwnedProjection<M>>>())}else{owner.close_demand(body)}}
  if let Some(owner)=self.source.as_ref(){return if owner.terminal_is_empty(){Ok(header::<Option<Input<M>>>())}else{owner.close_demand(body)}}
  if self.mutation.is_some(){return Ok(RetirementDemand{capacity_bytes:store.owned_mutation_retirement_birth_bytes()?,..header::<Option<M>>()})}
  if !self.projecting.terminal_is_empty(){return self.projecting.close_demand(body)}
  if let Some(owner)=self.projection.as_ref(){return if owner.terminal_is_empty(){Ok(header::<Option<RetainedOwnedProjection<Vec<u8>>>>())}else{owner.close_demand(body)}}
  if self.factory.is_some(){return Ok(header::<FactoryAuthority>())}
  if let Some(owner)=self.factory_close.as_ref(){return if owner.terminal_is_empty(){Ok(header::<Option<FactoryAuthority>>())}else{owner.demands(body)}}
  if self.overlay.alias.is_some(){return Ok(RetirementDemand{capacity_bytes:semio_framework_value::factory::FactorySharedRetirement::<P>::constructor_capacity_bytes(),depth:2,..Default::default()})}
  if self.overlay.running.is_some()||self.overlay.base.is_some(){return Ok(header::<Option<Derived<P>>>())}
  if self.resetting||!self.closing{return Ok(header::<(usize,u8,bool)>())}
  if let Some(owner)=self.retiring_wire.as_ref(){return if owner.terminal_is_empty(){Ok(header::<Option<Source>>())}else{owner.next_close_demand(body)}}
  if !self.rows.is_empty(){return Ok(header::<Option<Source>>())}
  if self.rows.capacity()!=0{return Ok(RetirementDemand{release_bytes:self.rows.capacity()*size_of::<Source>(),..header::<Vec<Source>>()})}Ok(Default::default())
 }
 /// 🕳️ No original row, typed result, native producer, factory or snapshot alias remains.
 pub fn terminal_is_empty(&self)->bool{self.rows.is_empty()&&self.rows.capacity()==0&&self.row_admission.as_ref().map_or(true,|admission|admission.terminal_is_empty())&&self.retiring_wire.is_none()&&self.projection.is_none()&&self.projecting.terminal_is_empty()&&self.receiver.is_none()&&self.mutation.is_none()&&self.source.is_none()&&self.input.is_none()&&!self.overlay.input_pending()&&self.overlay.semantic.is_none()&&self.factory.is_none()&&self.factory_close.is_none()&&self.overlay.messages.is_none()&&self.overlay.retirement.is_none()&&self.overlay.base.is_none()&&self.overlay.running.is_none()&&self.overlay.alias.is_none()}
}
pub fn close_paid(cx:&mut StepContext<'_>,d:RetirementDemand,run:impl FnOnce(RetainedCloneGrant)->Result<RetainedCloneProgress,ValueError>)->Result<(),OriginalTurnFailure>{retired(cx,d,|grant|run(grant).map(RetainedCloneStep::Progress))?;Ok(())}
/// ♻️ One controlled native retirement turn: the decoder's retirement control and an effect over the context's receiving frame.
pub fn close_native(cx:&mut StepContext<'_>,d:RetirementDemand,run:impl FnOnce(&mut semio_framework_value_native_owned::native_decoding::NativeRetirementDecodeControl<'_,'_>,&mut semio_framework_value::RetainedAllocationEffect<'_>)->semio_framework_value::OriginalRetirementPoll)->Result<(),OriginalTurnFailure>{
 use semio_framework_value_native_owned::native_decoding::{NativeReceivingEffect,NativeReceivingEffectPoll};
 if cx.retirement_should_yield()||!fits(cx,d){return Ok(())}
 let maximum=cx.fuel_remaining();let mut source=None;let mut frame=NativeReceivingEffect::<ValueError>::default();let mut done=None;
 cx.with_native_retirement_receiving(&mut source,StepContext::native_retirement_quote(d,maximum),|native,_|{done=Some(native.receive_effect(d,maximum,&mut frame,|control,effect|match semio_framework_value::RetirementPollExit::settled(run(control,effect)){Ok(turn)=>semio_framework_value::RetainedWorkOutcome{result:turn.result.map_err(semio_framework_value::OriginalRetirementFailure::into_value_error),work:turn.work},Err(_)=>semio_framework_value::RetainedWorkOutcome{result:Err(invalid("synchronous native retirement stayed pending")),work:0}})?);Ok(())})?;
 match done{Some(NativeReceivingEffectPoll::Complete(turn))=>{semio_framework_job::receive_original_receiving_work(cx,semio_framework_value::RetainedWorkOutcome{result:turn.result.map_err(receiving_fault),work:turn.work})?;Ok(())},Some(_)=>Err(invalid("synchronous native retirement stayed pending").into()),None=>Ok(())}
}

#[path="🔁️fold/🦀️.rs"]mod original_fold;
pub use original_fold::OriginalToolOverlayFold;

#[path="📖️source/🦀️.rs"]
mod ledger_read;
pub use ledger_read::{OriginalToolLedgerRead,OriginalToolLedgerWalk};
