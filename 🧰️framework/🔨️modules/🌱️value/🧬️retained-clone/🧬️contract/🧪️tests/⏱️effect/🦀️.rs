//! ⏱️ Cursor turns run inside one original effect over [`EffectBank`].
use super::*;
use semio_framework_value_retirement::OriginalRetirementOutcome;
use semio_framework_value_testing_effect_bank::{EffectBank,Wallet};
use std::{cell::RefCell,marker::PhantomData,mem::ManuallyDrop};

pub const CPU:u64=1048576;
const BODY:usize=4096;
pub const STOCK:RetainedCloneGrant=RetainedCloneGrant{maximum_items:1048576,maximum_copy_bytes:16777216,maximum_capacity_bytes:16777216,maximum_release_bytes:33554432,maximum_depth:256};
thread_local!{static BANK:RefCell<Option<EffectBank>>=const{RefCell::new(None)};}
fn fresh()->EffectBank{EffectBank::new(Wallet::new(RetainedCloneGrant{maximum_items:usize::MAX,maximum_copy_bytes:usize::MAX,maximum_capacity_bytes:usize::MAX,maximum_release_bytes:usize::MAX,maximum_depth:usize::MAX},1<<62))}
fn lent<R>(operation:impl FnOnce(&mut EffectBank)->R)->R{let mut bank=BANK.with(|slot|slot.borrow_mut().take()).unwrap_or_else(fresh);let result=operation(&mut bank);BANK.with(|slot|*slot.borrow_mut()=Some(bank));result}
fn admitted(bank:&mut EffectBank){assert_ne!(bank.take_admitted(),Some(false))}
fn grant_of(d:RetirementDemand)->RetainedCloneGrant{RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:d.copy_bytes,maximum_capacity_bytes:d.capacity_bytes,maximum_release_bytes:d.release_bytes,maximum_depth:d.depth}}

/// 🪢️ Runs one synchronous operation.
pub fn with_effect<R>(grant:RetainedCloneGrant,maximum:u64,actual:&mut Option<usize>,operation:impl FnOnce(&mut RetainedAllocationEffect<'_>)->R)->(R,u128){
 let mut output=None;
 let(poll,marker)=lent(|bank|{let outcome=bank.run(grant,maximum,|effect|{let boundary=effect.performed_boundary();output=Some(operation(effect));let work=effect.performed_since(boundary).unwrap();RetainedWorkPoll::Complete(RetainedWorkOutcome::<(),ValueError>{result:Ok(()),work})});admitted(bank);outcome});
 let RetainedWorkPoll::Complete(turn)=poll else{panic!("pending")};*actual=marker;turn.result.unwrap();(output.take().unwrap(),turn.work)
}

/// 🏁️ Requires a finished poll.
pub fn settled<T,E>(poll:RetainedWorkPoll<T,E>)->RetainedWorkOutcome<T,E>{match poll{RetainedWorkPoll::Complete(outcome)=>outcome,RetainedWorkPoll::Pending{..}=>panic!("pending")}}

type Reply<'r,E>=OriginalEffectReply<'r,RetainedCloneStep,E>;
type Done<'r,E>=OriginalEffectReplyComplete<'r,RetainedCloneStep,E>;
struct Producer<E>(PhantomData<E>);
struct Turn<'a,E>{run:&'a mut dyn for<'r,'e> FnMut(&mut RetainedAllocationEffect<'e>,Reply<'r,E>)->Result<Done<'r,E>,ValueError>,source:OriginalSourceTurnIdentity}
impl<E:'static> OriginalEffectProducer for Producer<E>{
 type Output=RetainedCloneStep;type Failure=E;type Context<'a,'b>=Turn<'a,E> where 'b:'a;
 fn validate_context<'a,'b>(&self,_:&Self::Context<'a,'b>)->Result<(),ValueError>where 'b:'a{Ok(())}
 fn source_turn<'a,'b>(&self,c:&Self::Context<'a,'b>)->Option<OriginalSourceTurnIdentity>where 'b:'a{Some(c.source)}
 fn receive<'r,'a,'b>(&mut self,c:&mut Self::Context<'a,'b>,e:&mut RetainedAllocationEffect<'_>,r:Reply<'r,E>)->Result<Done<'r,E>,ValueError>where 'b:'a{(c.run)(e,r)}
 fn recover<'r,'a,'b>(&mut self,_:&mut Self::Context<'a,'b>,_:&mut RetainedAllocationEffect<'_>,_:Reply<'r,E>)->Result<Done<'r,E>,ValueError>where 'b:'a{Err(ValueError::invariant("no callback"))}
}
/// 🧵️ Drives one reply-shaped call through the real original action until its reply is recorded.
pub fn turn<E:From<ValueError>+'static>(grant:RetainedCloneGrant,maximum:u64,run:&mut dyn for<'r,'e> FnMut(&mut RetainedAllocationEffect<'e>,Reply<'r,E>)->Result<Done<'r,E>,ValueError>)->RetainedWorkOutcome<RetainedCloneStep,E>{
 lent(|bank|{
  let before=*bank.wallet().original_work();let source=bank.source_witness().identity();let mut context=Turn{run,source};
  let mut authority=ManuallyDrop::new(OriginalEffectAuthority::new(Producer::<E>(PhantomData)));let(mut action,mut output)=(None,None);let mut failure=None;
  for _ in 0..4096{
   let(poll,_)=bank.run(grant,maximum,|effect|{let boundary=effect.performed_boundary();let driven=authority.drive(&mut context,effect,&mut action);let work=effect.performed_since(boundary).unwrap();match driven{Ok(true)=>RetainedWorkPoll::Complete(RetainedWorkOutcome::<(),ValueError>{result:Ok(()),work}),Ok(false)=>RetainedWorkPoll::Pending{work},Err(error)=>RetainedWorkPoll::Complete(RetainedWorkOutcome{result:Err(error),work})}});
   admitted(bank);
   match poll{RetainedWorkPoll::Pending{..}=>continue,RetainedWorkPoll::Complete(done)=>{if let Err(error)=done.result{failure=Some(error)}}}
   break;
  }
  let work=before-*bank.wallet().original_work();
  if let Some(error)=failure{std::mem::forget(action);std::mem::forget(output);return RetainedWorkOutcome{result:Err(E::from(error)),work}}
  let moved=grant_of(OriginalEffectAuthority::<Producer<E>>::transfer_reply_demand().unwrap());
  let(poll,_)=bank.run(moved,1,|effect|RetainedWorkPoll::Complete(authority.transfer_reply_into(&context,&mut action,&mut output,effect)));
  let RetainedWorkPoll::Complete(moved_turn)=poll else{panic!("pending")};moved_turn.result.unwrap().unwrap();admitted(bank);
  let demand=RetirementDemand{copy_bytes:2*size_of::<Option<Result<RetainedCloneStep,E>>>(),depth:1,..Default::default()};let mut recorded=None;
  let(poll,_)=bank.run(grant_of(demand),1,|effect|{assert!(effect.performed(1).unwrap());recorded=output.take();RetainedWorkPoll::Complete(RetainedWorkOutcome::<_,ValueError>{result:Ok(()),work:1})});
  let RetainedWorkPoll::Complete(released)=poll else{panic!("pending")};released.result.unwrap();admitted(bank);
  RetainedWorkOutcome{result:recorded.unwrap(),work}
 })
}

/// 🎟️ Runs one source advance.
pub fn advance<T:RetainedClone,C:RetainedCloneCursor<T>+?Sized>(cursor:&mut C,source:RetainedCloneRef<'_,T>,body:usize,maximum:u64,grant:RetainedCloneGrant)->OriginalRetirementOutcome{turn(grant,maximum,&mut |effect,reply|cursor.advance_into(source,body,maximum,effect,reply))}

/// 📤️ Runs one owner transfer.
pub fn take<T:RetainedClone,C:RetainedCloneCursor<T>+?Sized>(cursor:&mut C,grant:RetainedCloneGrant)->RetainedWorkOutcome<Option<(T,RetainedCloneProgress)>,ValueError>{
 let mut output=None;let outcome=turn(grant,CPU,&mut |effect,reply|cursor.take(&mut output,effect,reply));
 RetainedWorkOutcome{result:outcome.result.map(|step|output.map(|value|(value,step.progress()))),work:outcome.work}
}

/// 🚦️ Runs the admitted close-start action.
pub fn begin_close_paid<T:RetainedClone,C:RetainedCloneCursor<T>+?Sized>(cursor:&mut C,grant:RetainedCloneGrant)->RetainedWorkOutcome<RetainedCloneProgress,ValueError>{let outcome=turn::<semio_framework_value_retirement::OriginalRetirementFailure>(grant,CPU,&mut |effect,reply|cursor.begin_close_into(effect,reply));RetainedWorkOutcome{work:outcome.work,result:outcome.result.map(|step|step.progress()).map_err(|failure|failure.primary().map(|error|ValueError::literal(error.kind,"original close start refused")).unwrap_or_else(||ValueError::invariant("original close start refused")))}}

/// ♻️ Runs one actual close step to completion.
pub fn close_step<T:RetainedClone,C:RetainedCloneCursor<T>+?Sized>(cursor:&mut C,grant:RetainedCloneGrant)->OriginalRetirementOutcome{turn(grant,CPU,&mut |effect,reply|cursor.close_into(BODY,effect,reply))}

/// 🎯️ Runs one body-selected close to completion.
pub fn close_selected<T:RetainedClone,C:RetainedCloneCursor<T>+?Sized>(cursor:&mut C,body:usize,grant:RetainedCloneGrant)->OriginalRetirementOutcome{turn(grant,CPU,&mut |effect,reply|cursor.close_into(body,effect,reply))}

/// 🚦️ Runs the admitted close-start of one close frame over its owner.
pub fn begin_granted<T:semio_framework_value_retirement::retirement::RetireOwned>(close:&mut RetainedCloneClose,value:&mut Option<T>,grant:RetainedCloneGrant)->RetainedWorkOutcome<Option<RetainedCloneStep>>{let mut actual=None;with_effect(grant,CPU,&mut actual,|effect|close.begin_granted(value,effect)).0}

/// ♻️ Runs one close frame step to completion.
pub fn step_granted(close:&mut RetainedCloneClose,grant:RetainedCloneGrant)->OriginalRetirementOutcome{turn(grant,CPU,&mut |effect,reply|close.step_into(BODY,effect,reply))}

/// 🪢️ Runs any operation.
pub fn run<R>(grant:RetainedCloneGrant,operation:impl FnOnce(&mut RetainedAllocationEffect<'_>)->R)->R{let mut actual=None;with_effect(grant,CPU,&mut actual,operation).0}

/// 🏁️ Runs a retirement turn.
pub fn settle<T,E>(grant:RetainedCloneGrant,operation:impl FnOnce(&mut RetainedAllocationEffect<'_>)->RetainedWorkPoll<T,E>)->RetainedWorkOutcome<T,E>{settled(run(grant,operation))}

/// 🚦️ Runs a quoted close-start.
pub fn begin_close<T:RetainedClone,C:RetainedCloneCursor<T>+?Sized>(cursor:&mut C)->u128{let d=cursor.begin_close_demand().unwrap();let grant=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:d.copy_bytes,maximum_capacity_bytes:d.capacity_bytes,maximum_release_bytes:d.release_bytes,maximum_depth:d.depth};let turn=begin_close_paid(cursor,grant);turn.result.unwrap();turn.work}

/// 📄️ Reports whether one funded list reservation made progress.
pub fn progressed(outcome:&semio_framework_value_paged_list::list::PagedListReservationOutcome)->bool{matches!(outcome,semio_framework_value_paged_list::list::PagedListReservationOutcome::Ready(progress) if progress.progressed)}

/// 🎫️ Admits one owned value as an original Source through the real projection protocol and reports its progress and work.
pub fn admit<T:semio_framework_value_retirement::retirement::RetireOwned+Sync+'static>(value:T)->(semio_framework_value_clone_source::RetainedCloneSource<T>,RetainedCloneProgress,u128){lent(|bank|{let outcome=semio_framework_value_clone_source_testing::admit_owned(bank,value,(),STOCK);let work=outcome.work;match outcome.result{Ok((source,progress))=>(source,progress,work),Err(_)=>panic!("original Source admission refused")}})}

/// 🎞️ Drives one poll-shaped close under a real original action; the physical heap receipt equals the recorded progress.
pub fn driven(grant:RetainedCloneGrant,call:impl FnMut(&mut RetainedAllocationEffect<'_>)->semio_framework_value_retirement::OriginalRetirementPoll)->(OriginalRetirementOutcome,(usize,usize)){
 semio_framework_value_clone_source_testing::drive::warm();
 let outcome=semio_framework_value_clone_source_testing::drive::drive(grant,CPU,call);
 let progress=match &outcome.result{Ok(step)=>step.progress(),Err(error)=>error.retained_progress()};
 let heap=(progress.retained_capacity_bytes,progress.released_bytes);(outcome,heap)
}

/// 🧮️ Reads the physical heap receipt that a recorded outcome proves.
pub fn heap_of<E:HeapProgress>(outcome:&RetainedWorkOutcome<RetainedCloneStep,E>)->(usize,usize){let progress=match &outcome.result{Ok(step)=>step.progress(),Err(error)=>error.progress()};(progress.retained_capacity_bytes,progress.released_bytes)}
/// 🧾️ Names the progress that a refusal retained.
pub trait HeapProgress{fn progress(&self)->RetainedCloneProgress;}
impl HeapProgress for ValueError{fn progress(&self)->RetainedCloneProgress{self.retained_progress()}}
impl HeapProgress for semio_framework_value_retirement::OriginalRetirementFailure{fn progress(&self)->RetainedCloneProgress{self.retained_progress()}}
