//! 🧪️ Persistent semantic edit fixtures borrow one actual caller, decoder and original source pair throughout a test case.
pub use semio_framework_job_testing_caller::OriginalJobFixtureCaller;
pub use semio_framework_job_context::{OriginalJobDecodeOwner,OperationId,Generation};
use semio_framework_job_context::{StepContext,OriginalTurnFailure,close_original_retained_step};
use semio_framework_value::{RetireOwned,RetirementDemand,RetainedCloneSource,RetainedCloneGrant,RetainedCloneProgress,RetainedCloneStep,RetainedWorkOutcome,ValueError,ValueRefusalKind,OwnedValueRetirementFactory};
use semio_framework_value::retirement::controlled::ControlledRetirement;
use semio_framework_value_snapshot_projection::snapshot_projection::post::OriginalPostRead;
use crate::semantic::{PagedOneItemEdit,PagedOneItemEditStep,PagedOneItemPostMode};
use std::mem::{ManuallyDrop,size_of};

/// 🧾️ Semantic refusal remains owned by its caller alongside exact paid work and physical effects.
pub struct PagedEditFixtureOutcome<M>{pub edit:Result<PagedOneItemEditStep<M>,ValueError>,pub work:u64,pub progress:RetainedCloneProgress}
/// 🪙️ Sources and authority are supplied once; this helper creates neither replacement controls nor initiating stock.
pub struct PagedEditFixture<S:RetireOwned+Send+Sync+'static,M:RetireOwned+Send+Sync+'static,E:PagedOneItemEdit<S,M>>{
 caller:Option<OriginalJobFixtureCaller>,decoder:OriginalJobDecodeOwner,
 before:ManuallyDrop<Option<RetainedCloneSource<S>>>,mutation:ManuallyDrop<Option<RetainedCloneSource<M>>>,
 post:ManuallyDrop<Option<S>>,post_read:OriginalPostRead<S>,post_close:Option<ControlledRetirement<S>>,
 edit:E,pending:ManuallyDrop<Option<Result<PagedOneItemEditStep<M>,ValueError>>>,last_work:u64,last_progress:RetainedCloneProgress,closed:bool,
}
fn progress<M>(result:&Result<PagedOneItemEditStep<M>,ValueError>)->RetainedCloneProgress{match result{
 Ok(PagedOneItemEditStep::Progress(p)|PagedOneItemEditStep::Complete(p,_)|PagedOneItemEditStep::CompleteRows(p,_)|PagedOneItemEditStep::CompleteOutcome(p,_,_)|PagedOneItemEditStep::RefusedOutcome(p,_))=>*p,Err(error)=>error.retained_progress(),
}}
fn fault()->ValueError{ValueError::literal(ValueRefusalKind::InvariantViolated,"persistent edit fixture lost original custody")}
impl<S:RetireOwned+Send+Sync+'static,M:RetireOwned+Send+Sync+'static,E:PagedOneItemEdit<S,M>>PagedEditFixture<S,M,E>{
 /// 🧷️ Retains provided initiating authorities and already admitted source owners without minting replacements.
 pub fn new(caller:OriginalJobFixtureCaller,decoder:OriginalJobDecodeOwner,before:RetainedCloneSource<S>,mutation:RetainedCloneSource<M>,post:S,edit:E)->Self{
  Self{caller:Some(caller),decoder,before:ManuallyDrop::new(Some(before)),mutation:ManuallyDrop::new(Some(mutation)),post:ManuallyDrop::new(Some(post)),post_read:Default::default(),post_close:None,edit,pending:ManuallyDrop::new(None),last_work:0,last_progress:Default::default(),closed:false}
 }
 pub fn before(&self)->&S{self.before.as_ref().expect("live original before source").borrow().get()}
 pub fn mutation(&self)->&M{self.mutation.as_ref().expect("live original mutation source").borrow().get()}
 pub fn post(&self)->Result<&S,ValueError>{self.post_read.read(&self.post,self.edit.post_mode()).map(|post|post.get())}
 pub fn take_pending_outcome(&mut self)->Option<Result<PagedOneItemEditStep<M>,ValueError>>{self.pending.take()}
 pub fn last_work(&self)->u64{self.last_work}
 pub fn last_progress(&self)->RetainedCloneProgress{self.last_progress}
 pub fn advance(&mut self,grant:RetainedCloneGrant)->Result<PagedEditFixtureOutcome<M>,OriginalTurnFailure>{self.advance_inner(grant,None)}
 /// ⛽️ A genuine narrower work loan pauses the same original stock without replacing its decoder or sources.
 pub fn advance_with_work(&mut self,grant:RetainedCloneGrant,maximum_work:u64)->Result<PagedEditFixtureOutcome<M>,OriginalTurnFailure>{self.advance_inner(grant,Some(maximum_work))}
 fn advance_inner(&mut self,grant:RetainedCloneGrant,loan:Option<u64>)->Result<PagedEditFixtureOutcome<M>,OriginalTurnFailure>{
  if self.closed||self.pending.is_some(){return Err(fault().into())}
  self.last_work=0;self.last_progress=Default::default();
  let Self{caller,decoder,before,mutation,post,post_read,edit,pending,last_work,last_progress,..}=self;
  let (result,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||caller.as_mut().ok_or_else(fault)?.turn_with_decoder(grant,decoder,|cx|{
   let mut run=|cx:&mut StepContext<'_>|->Result<(),OriginalTurnFailure>{
    let fuel=cx.fuel_remaining();let mode=edit.post_mode();
    let result=if post_read.demand(post,mode,grant.maximum_copy_bytes)?.is_some(){
     if fuel==0{Ok(PagedOneItemEditStep::Progress(Default::default()))}
     else{let turn=post_read.prepare(post,mode,grant.maximum_copy_bytes,grant);*last_work=turn.work;let result=turn.result.map(|p|PagedOneItemEditStep::Progress(p.expect("quoted post ownership frontier")));*pending=Some(result);assert!(*last_work<=fuel);cx.consume_fuel(*last_work);cx.consume_retained(progress(pending.as_ref().ok_or_else(fault)?))?;pending.take().ok_or_else(fault)?}
    }else{
     let observed=cx.with_native_receiving(|receiving|{
      let start=receiving.actual_work();
      *pending=Some(edit.advance(before.as_ref().ok_or_else(fault)?.try_borrow()?,mutation.as_ref().ok_or_else(fault)?.try_borrow()?,post_read.access(post,mode)?,grant.maximum_copy_bytes,grant,receiving));
      Ok::<_,ValueError>(receiving.actual_work().saturating_sub(start))
     })?;
     *last_work=crate::original_one_item_turn_work(edit.last_turn_work(),observed,false)?;assert!(*last_work<=fuel);cx.consume_fuel(*last_work);
     cx.consume_receiving_retained(grant,progress(pending.as_ref().ok_or_else(fault)?))?;
     pending.take().ok_or_else(fault)?
    };
    *pending=Some(result);
    assert!(*last_work<=fuel,"edit cannot spend CPU beyond its original loan");assert_eq!(fuel-cx.fuel_remaining(),*last_work,"reported edit work is paid once");
    *last_progress=progress(pending.as_ref().ok_or_else(fault)?);Ok(())
   };
   if let Some(work)=loan{let deadline=cx.original_absolute_deadline_us();cx.with_original_work_loan(work,deadline,|same|run(same))?;}else{run(cx)?;}Ok(())
  }));
  let receipt=caller.as_ref().expect("original caller retained through fixture closure").receipt();
  assert_eq!((heap.requested_bytes,heap.released_bytes),(receipt.retained_capacity_bytes,receipt.released_bytes),"semantic edit including refusal must publish every physical effect");
  result?;assert_eq!(receipt,*last_progress,"original semantic receipt is settled exactly once");
  Ok(PagedEditFixtureOutcome{edit:pending.take().ok_or_else(fault)?,work:*last_work,progress:*last_progress})
 }
 /// 📦️ Caller-owned outputs keep their retirement cursor on refusal or an unaffordable frontier.
 pub fn retire_owned<T:RetireOwned+'static>(&mut self,pending:&mut Option<T>,active:&mut Option<ControlledRetirement<T>>,grant:RetainedCloneGrant,maximum_turns:usize)->Result<bool,OriginalTurnFailure>{
  for _ in 0..maximum_turns{
   if pending.is_none()&&active.is_none(){return Ok(true)}
   let d=if let Some(owner)=active.as_ref(){if owner.terminal_is_empty(){RetirementDemand{copy_bytes:size_of::<Option<ControlledRetirement<T>>>(),depth:1,..Default::default()}}else{owner.next_demand(grant.maximum_copy_bytes)?}}else{RetirementDemand{copy_bytes:2*size_of::<Option<T>>()+size_of::<Option<ControlledRetirement<T>>>(),depth:1,..Default::default()}};
   self.close_turn(grant,|cx|{close_original_retained_step(cx,d,|issued|{
    if let Some(owner)=active.as_mut(){if owner.terminal_is_empty(){*active=None;RetainedWorkOutcome{work:1,result:Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,..Default::default()}))}}else{let result=owner.step(issued);RetainedWorkOutcome{work:owner.last_turn_work(),result}}}
    else{*active=Some(ControlledRetirement::retain(pending.take().expect("retained fixture output")));RetainedWorkOutcome{work:1,result:Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,..Default::default()}))}}
   })?;Ok(())})?;
  }Ok(pending.is_none()&&active.is_none())
 }
 fn close_turn<T>(&mut self,grant:RetainedCloneGrant,run:impl FnOnce(&mut StepContext<'_>)->Result<T,OriginalTurnFailure>)->Result<T,OriginalTurnFailure>{
  let caller=self.caller.as_mut().ok_or_else(fault)?;let(result,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||caller.turn_with_decoder(grant,&mut self.decoder,|cx|cx.with_original_retirement(run)));
  let p=caller.receipt();assert_eq!((heap.requested_bytes,heap.released_bytes),(p.retained_capacity_bytes,p.released_bytes));result
 }
 /// ♻️ Returns edit, post, original sources, decoder and initiating caller under the same remaining stock.
 pub fn close(&mut self,grant:RetainedCloneGrant,maximum_turns:usize)->Result<bool,OriginalTurnFailure>{
  if self.closed{return Ok(true)}
  if self.pending.is_some(){return Err(fault().into())}
  self.edit.begin_close();
  for _ in 0..maximum_turns{
   if !self.edit.terminal_is_empty(){
    let factory=OwnedValueRetirementFactory::<M>::default();let d=self.edit.close_demands(&factory,grant.maximum_copy_bytes)?;
    let caller=self.caller.as_mut().ok_or_else(fault)?;let edit=&mut self.edit;
    let(result,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||caller.turn_with_decoder(grant,&mut self.decoder,|cx|cx.with_original_retirement(|cx|{close_original_retained_step(cx,d,|issued|RetainedWorkOutcome{result:edit.close_step(&factory,issued),work:1})?;Ok(())})));
    let p=caller.receipt();assert_eq!((heap.requested_bytes,heap.released_bytes),(p.retained_capacity_bytes,p.released_bytes));result?;continue
   }
   if !self.post_read.terminal_is_empty(){
    let d=self.post_read.demand(&self.post,PagedOneItemPostMode::Mutable,grant.maximum_copy_bytes)?.ok_or_else(fault)?;
    let Self{caller,decoder,post,post_read,..}=self;
    let caller=caller.as_mut().ok_or_else(fault)?;let(result,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||caller.turn_with_decoder(grant,decoder,|cx|cx.with_original_retirement(|cx|{close_original_retained_step(cx,d,|issued|{let turn=post_read.prepare(post,PagedOneItemPostMode::Mutable,issued.maximum_copy_bytes,issued);RetainedWorkOutcome{work:turn.work,result:turn.result.map(|p|RetainedCloneStep::Progress(p.expect("quoted post return")))}})?;Ok(())})));
    let p=caller.receipt();assert_eq!((heap.requested_bytes,heap.released_bytes),(p.retained_capacity_bytes,p.released_bytes));result?;continue
   }
   macro_rules! source{($field:ident,$ty:ty)=>{if let Some(source)=self.$field.as_mut(){
    let d=if source.terminal_is_empty(){RetirementDemand{copy_bytes:size_of::<Option<RetainedCloneSource<$ty>>>(),depth:1,..Default::default()}}else{source.next_close_demand(grant.maximum_copy_bytes)?};
    let caller=self.caller.as_mut().ok_or_else(fault)?;let slot=&mut self.$field;
    let(result,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||caller.turn_with_decoder(grant,&mut self.decoder,|cx|cx.with_original_retirement(|cx|{close_original_retained_step(cx,d,|issued|{if slot.as_ref().unwrap().terminal_is_empty(){slot.take();RetainedWorkOutcome{work:1,result:Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,..Default::default()}))}}else{slot.as_mut().unwrap().close_step(issued)}})?;Ok(())})));
    let p=caller.receipt();assert_eq!((heap.requested_bytes,heap.released_bytes),(p.retained_capacity_bytes,p.released_bytes));result?;continue
   }}}
   source!(before,S);source!(mutation,M);
   if self.post.is_some()||self.post_close.is_some(){
    let d=if let Some(owner)=self.post_close.as_ref(){if owner.terminal_is_empty(){RetirementDemand{copy_bytes:size_of::<Option<ControlledRetirement<S>>>(),depth:1,..Default::default()}}else{owner.next_demand(grant.maximum_copy_bytes)?}}else{RetirementDemand{copy_bytes:2*size_of::<Option<S>>()+size_of::<Option<ControlledRetirement<S>>>(),depth:1,..Default::default()}};
    let Self{caller,decoder,post,post_close,..}=self;let caller=caller.as_mut().ok_or_else(fault)?;
    let(result,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||caller.turn_with_decoder(grant,decoder,|cx|cx.with_original_retirement(|cx|{close_original_retained_step(cx,d,|issued|{
     if let Some(owner)=post_close.as_mut(){if owner.terminal_is_empty(){*post_close=None;RetainedWorkOutcome{work:1,result:Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,..Default::default()}))}}else{let result=owner.step(issued);RetainedWorkOutcome{work:owner.last_turn_work(),result}}}
     else{*post_close=Some(ControlledRetirement::retain(post.take().unwrap()));RetainedWorkOutcome{work:1,result:Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,..Default::default()}))}}
    })?;Ok(())})));let p=caller.receipt();assert_eq!((heap.requested_bytes,heap.released_bytes),(p.retained_capacity_bytes,p.released_bytes));result?;continue
   }
   if !self.decoder.terminal_is_empty(){
    let d=self.decoder.retirement_demands(grant.maximum_copy_bytes)?;let caller=self.caller.as_mut().ok_or_else(fault)?;let decoder=&mut self.decoder;
    let(result,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||caller.turn(grant,|cx|cx.with_original_retirement(|cx|{close_original_retained_step(cx,d,|issued|decoder.close_step(issued))?;Ok(())})));
    let p=caller.receipt();assert_eq!((heap.requested_bytes,heap.released_bytes),(p.retained_capacity_bytes,p.released_bytes));result?;continue
   }
   let mut caller=self.caller.take().ok_or_else(fault)?;caller.retire(grant);self.closed=true;return Ok(true)
  }Ok(false)
 }
}
impl<S:RetireOwned+Send+Sync+'static,M:RetireOwned+Send+Sync+'static,E:PagedOneItemEdit<S,M>>Drop for PagedEditFixture<S,M,E>{fn drop(&mut self){assert!(std::thread::panicking()||self.closed,"persistent edit fixture requires explicit original funded closure")}}
