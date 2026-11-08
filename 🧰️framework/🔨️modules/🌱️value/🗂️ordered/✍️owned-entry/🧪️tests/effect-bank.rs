//! 🧵️ One genuine Source bank drives the caller-resident entry loan and all paid retirement.
use crate::{OriginalOwnedEntry,OrderedMap,RetainedAllocationEffect,RetainedCloneGrant,RetainedCloneProgress,RetainedCloneStep,RetirementDemand,OriginalRetirementFailure};
use semio_framework_value_retained_clone::RetainedCloneClose;
use semio_framework_value_control_contract::{OriginalEffectProducer,OriginalEffectAuthority,OriginalEffectAction,OriginalEffectReply,OriginalEffectReplyComplete,OriginalSourceTurnIdentity,ValueError,RetainedWorkPoll,RetainedWorkOutcome};
use semio_framework_value_testing_effect_bank::{EffectBank,Wallet};
use semio_framework_value_testing_allocator::observe_retirement_allocations as observe;
use std::mem::{ManuallyDrop,size_of};
#[derive(Clone,Copy)]pub(crate) enum Op{Advance,EntryStart,EntryClose,OutputStart,OutputClose,ExtraStart,ExtraClose}
pub(crate) struct Context{source:OriginalSourceTurnIdentity,op:Op,body:usize}
pub(crate) struct Producer<'a>{pub(crate) entry:&'a mut Option<OriginalOwnedEntry<u64>>,pub(crate) output:&'a mut Option<OrderedMap<u64>>,pub(crate) entry_close:&'a mut RetainedCloneClose,pub(crate) output_close:&'a mut RetainedCloneClose,pub(crate) extra:Option<&'a mut Option<OrderedMap<u64>>>,pub(crate) extra_close:Option<&'a mut RetainedCloneClose>,source:OriginalSourceTurnIdentity}
impl Producer<'_>{pub(crate) fn empty(&self)->bool{self.entry.is_none()&&self.output.is_none()&&self.entry_close.is_empty()&&self.output_close.is_empty()&&self.extra.as_ref().is_none_or(|v|v.is_none())&&self.extra_close.as_ref().is_none_or(|v|v.is_empty())}}
impl OriginalEffectProducer for Producer<'_>{
 type Output=RetainedCloneStep;type Failure=OriginalRetirementFailure;type Context<'a,'b>=Context where 'b:'a;
 fn validate_context<'a,'b>(&self,c:&Context)->Result<(),ValueError>where 'b:'a{if c.source!=self.source{return Err(ValueError::invariant("entry fixture changed its genuine Source"))}Ok(())}
 fn source_turn<'a,'b>(&self,_:&Context)->Option<OriginalSourceTurnIdentity>where 'b:'a{Some(self.source)}
 fn receive<'r,'a,'b>(&mut self,c:&mut Context,e:&mut RetainedAllocationEffect<'_>,r:OriginalEffectReply<'r,Self::Output,Self::Failure>)->Result<OriginalEffectReplyComplete<'r,Self::Output,Self::Failure>,ValueError>where 'b:'a{
  match c.op{
   Op::Advance=>self.entry.as_mut().unwrap().advance_into(c.body,self.output,e,r),
   Op::EntryStart=>{let turn=self.entry_close.begin_granted(self.entry,e);Ok(r.complete(turn.result.map(|s|s.unwrap_or(RetainedCloneStep::Progress(Default::default()))).map_err(Into::into)))},
   Op::EntryClose=>self.entry_close.step_into(c.body,e,r),
   Op::OutputStart=>{let turn=self.output_close.begin_granted(self.output,e);Ok(r.complete(turn.result.map(|s|s.unwrap_or(RetainedCloneStep::Progress(Default::default()))).map_err(Into::into)))},
   Op::OutputClose=>self.output_close.step_into(c.body,e,r),
   Op::ExtraStart=>{let turn=self.extra_close.as_deref_mut().unwrap().begin_granted(self.extra.as_deref_mut().unwrap(),e);Ok(r.complete(turn.result.map(|s|s.unwrap_or(RetainedCloneStep::Progress(Default::default()))).map_err(Into::into)))},
   Op::ExtraClose=>self.extra_close.as_deref_mut().unwrap().step_into(c.body,e,r),
  }
 }
 fn recover<'r,'a,'b>(&mut self,c:&mut Context,e:&mut RetainedAllocationEffect<'_>,r:OriginalEffectReply<'r,Self::Output,Self::Failure>)->Result<OriginalEffectReplyComplete<'r,Self::Output,Self::Failure>,ValueError>where 'b:'a{
  match c.op{Op::Advance=>self.entry.as_mut().unwrap().recover_advance_into(c.body,self.output,e,r),Op::EntryClose=>self.entry_close.recover_step_into(c.body,e,r),Op::OutputClose=>self.output_close.recover_step_into(c.body,e,r),Op::ExtraClose=>self.extra_close.as_deref_mut().unwrap().recover_step_into(c.body,e,r),_=>Err(ValueError::invariant("closed native entry fixture seed cannot recover a callback"))}
 }
}
impl Drop for Producer<'_>{fn drop(&mut self){assert!(std::thread::panicking()||self.empty(),"entry fixture loan still protects real owners")}}
pub(crate) struct Harness<'a>{authority:ManuallyDrop<Option<OriginalEffectAuthority<Producer<'a>>>>,action:ManuallyDrop<Option<OriginalEffectAction<RetainedCloneStep,OriginalRetirementFailure>>>,reply:ManuallyDrop<Option<Result<RetainedCloneStep,OriginalRetirementFailure>>>,returned:ManuallyDrop<Option<Producer<'a>>>,pub(crate) born:usize,pub(crate) freed:usize,pub(crate) last_heap:(usize,usize),pub(crate) last_work:u128}
impl<'a> Harness<'a>{
 pub(crate) fn new(entry:&'a mut Option<OriginalOwnedEntry<u64>>,output:&'a mut Option<OrderedMap<u64>>,entry_close:&'a mut RetainedCloneClose,output_close:&'a mut RetainedCloneClose,bank:&EffectBank)->Self{let source=bank.source_witness().identity();Self{authority:ManuallyDrop::new(Some(OriginalEffectAuthority::new(Producer{entry,output,entry_close,output_close,extra:None,extra_close:None,source}))),action:ManuallyDrop::new(None),reply:ManuallyDrop::new(None),returned:ManuallyDrop::new(None),born:0,freed:0,last_heap:(0,0),last_work:0}}
 pub(crate) fn new_with_alias(entry:&'a mut Option<OriginalOwnedEntry<u64>>,output:&'a mut Option<OrderedMap<u64>>,entry_close:&'a mut RetainedCloneClose,output_close:&'a mut RetainedCloneClose,extra:&'a mut Option<OrderedMap<u64>>,extra_close:&'a mut RetainedCloneClose,bank:&EffectBank)->Self{let source=bank.source_witness().identity();Self{authority:ManuallyDrop::new(Some(OriginalEffectAuthority::new(Producer{entry,output,entry_close,output_close,extra:Some(extra),extra_close:Some(extra_close),source}))),action:ManuallyDrop::new(None),reply:ManuallyDrop::new(None),returned:ManuallyDrop::new(None),born:0,freed:0,last_heap:(0,0),last_work:0}}
 pub(crate) fn quote(&self,op:Op)->(RetirementDemand,u64){self.authority.as_ref().unwrap().inspect(|p|match op{
  Op::Advance=>(p.entry.as_ref().unwrap().next_demands(4096).unwrap(),p.entry.as_ref().unwrap().next_work_demand(4096).unwrap()),
  Op::EntryStart|Op::EntryClose=>(p.entry_close.next_owner_demands::<OriginalOwnedEntry<u64>>(p.entry.is_some(),&None,4096).unwrap(),p.entry_close.next_owner_work::<OriginalOwnedEntry<u64>>(p.entry.is_some(),&None,4096).unwrap()),
  Op::OutputStart|Op::OutputClose=>(p.output_close.next_owner_demands::<OrderedMap<u64>>(p.output.is_some(),&None,4096).unwrap(),p.output_close.next_owner_work::<OrderedMap<u64>>(p.output.is_some(),&None,4096).unwrap()),
  Op::ExtraStart|Op::ExtraClose=>(p.extra_close.as_ref().unwrap().next_owner_demands::<OrderedMap<u64>>(p.extra.as_ref().unwrap().is_some(),&None,4096).unwrap(),p.extra_close.as_ref().unwrap().next_owner_work::<OrderedMap<u64>>(p.extra.as_ref().unwrap().is_some(),&None,4096).unwrap()),
 })}
 pub(crate) fn inspect<R>(&self,f:impl FnOnce(&Producer<'_>)->R)->R{self.authority.as_ref().unwrap().inspect(f)}
 pub(crate) fn run(&mut self,bank:&mut EffectBank,op:Op,g:RetainedCloneGrant,units:u64)->RetainedCloneStep{self.run_body(bank,op,4096,g,units)}
 pub(crate) fn run_body(&mut self,bank:&mut EffectBank,op:Op,body:usize,g:RetainedCloneGrant,units:u64)->RetainedCloneStep{
  let mut total=(0usize,0usize);let original_work=*bank.wallet().original_work();
  for _ in 0..10000{
   let((poll,marker),heap)=observe(||bank.run_with_source(g,units,|e,turn|{
    let before=e.performed_boundary();let mut c=Context{source:turn.identity(),op,body};let done=self.authority.as_mut().unwrap().drive(&mut c,e,&mut self.action).unwrap();let work=e.performed_since(before).unwrap();
    if done{RetainedWorkPoll::Complete(RetainedWorkOutcome{result:Ok::<_,ValueError>(()),work})}else{RetainedWorkPoll::Pending{work}}
   }));total.0+=heap.0;total.1+=heap.1;if let Some(marker)=marker{assert_eq!(marker,total.0)}
   let RetainedWorkPoll::Complete(turn)=poll else{continue};turn.result.unwrap();assert!(self.action.as_mut().unwrap().transfer_reply(&mut self.reply).unwrap());self.authority.as_mut().unwrap().finish(&mut self.action).unwrap();if let Err(error)=self.reply.as_ref().unwrap(){panic!("entry original failure remains resident: {error:?}")}let result=self.reply.take().unwrap();let progress=result.as_ref().map_or_else(|e|e.retained_progress(),|s|s.progress());assert_eq!(total,(progress.retained_capacity_bytes,progress.released_bytes));assert!(progress.fits(g));assert_ne!(bank.take_admitted(),Some(false));bank.post(progress,g,total);self.born+=progress.retained_capacity_bytes;self.freed+=progress.released_bytes;self.last_heap=total;self.last_work=original_work-*bank.wallet().original_work();return result.unwrap()
  }
  panic!("original entry action remained pending")
 }
 pub(crate) fn release(&mut self,bank:&mut EffectBank){
  assert!(self.inspect(|p|p.empty()));let demand=OriginalEffectAuthority::<Producer<'a>>::producer_transfer_demand().unwrap();let grant=bank.loan(1,demand.copy_bytes);
  let((poll,marker),heap)=observe(||bank.run_with_source(grant,1,|effect,turn|{let c=Context{source:turn.identity(),op:Op::EntryClose,body:0};RetainedWorkPoll::Complete(OriginalEffectAuthority::transfer_producer(&mut self.authority,&c,&mut self.returned,effect))}));
  let RetainedWorkPoll::Complete(turn)=poll else{panic!("entry producer loan return pending")};let progress=turn.result.unwrap().unwrap();assert_eq!(heap,(0,0));assert_eq!(marker,None);assert_eq!(bank.take_admitted(),Some(true));bank.post(progress,grant,heap);
  let bytes=size_of::<Option<Producer<'a>>>();let grant=bank.loan(1,bytes);let((poll,marker),heap)=observe(||bank.run(grant,1,|effect|{assert!(self.returned.as_ref().unwrap().empty());assert!(effect.performed(1).unwrap());drop(self.returned.take());RetainedWorkPoll::Complete(RetainedWorkOutcome{result:Ok::<_,ValueError>(()),work:1})}));let RetainedWorkPoll::Complete(turn)=poll else{panic!("native entry loan header release pending")};turn.result.unwrap();assert_eq!(heap,(0,0));assert_eq!(marker,None);assert_eq!(bank.take_admitted(),Some(true));bank.post(RetainedCloneProgress{copied_items:1,copied_bytes:bytes,..Default::default()},grant,heap);
 }
}
impl Drop for Harness<'_>{fn drop(&mut self){let empty=self.authority.is_none()&&self.action.is_none()&&self.reply.is_none()&&self.returned.is_none();assert!(std::thread::panicking()||empty,"entry harness retains its original loan/action");if empty{unsafe{ManuallyDrop::drop(&mut self.authority);ManuallyDrop::drop(&mut self.action);ManuallyDrop::drop(&mut self.reply);ManuallyDrop::drop(&mut self.returned)}}}}
pub(crate) fn grant(d:RetirementDemand)->RetainedCloneGrant{RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:d.copy_bytes,maximum_capacity_bytes:d.capacity_bytes,maximum_release_bytes:d.release_bytes,maximum_depth:d.depth}}
