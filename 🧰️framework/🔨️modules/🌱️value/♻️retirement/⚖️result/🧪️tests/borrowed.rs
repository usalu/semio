//! ⚖️ Result variants retain their payload while actual borrowed retirement spends finite Source work.
use semio_framework_value_control_contract::*;
use semio_framework_value_retirement::{OriginalRetirementFailure,retirement::{RetireOwned,RetirementCursor,RetirementStep}};
use semio_framework_value_testing_effect_bank::{EffectBank,Wallet};
use semio_framework_value_testing_allocator::observe_retirement_allocations;
struct Close;
impl OriginalEffectProducer for Close{
 type Output=RetirementStep;type Failure=OriginalRetirementFailure;type Context<'a,'b>=&'a mut dyn RetirementCursor where 'b:'a;
 fn validate_context<'a,'b>(&self,_:&Self::Context<'a,'b>)->Result<(),ValueError>where 'b:'a{Ok(())}
 fn source_turn<'a,'b>(&self,_:&Self::Context<'a,'b>)->Option<OriginalSourceTurnIdentity>where 'b:'a{None}
 fn receive<'r,'a,'b>(&mut self,c:&mut Self::Context<'a,'b>,e:&mut RetainedAllocationEffect<'_>,r:OriginalEffectReply<'r,RetirementStep,OriginalRetirementFailure>)->Result<OriginalEffectReplyComplete<'r,RetirementStep,OriginalRetirementFailure>,ValueError>where 'b:'a{c.close_into(e,r)}
 fn recover<'r,'a,'b>(&mut self,c:&mut Self::Context<'a,'b>,e:&mut RetainedAllocationEffect<'_>,r:OriginalEffectReply<'r,RetirementStep,OriginalRetirementFailure>)->Result<OriginalEffectReplyComplete<'r,RetirementStep,OriginalRetirementFailure>,ValueError>where 'b:'a{c.recover_close_into(e,r)}
}
#[test]fn result_borrowed_close_preserves_denied_variant_and_invoices_the_actual_source(){
 let law:serde_json::Value=serde_json::from_str(include_str!("borrowed.json")).unwrap();
 for name in law["variants"].as_array().unwrap(){
  let grant=RetainedCloneGrant{maximum_items:256,maximum_copy_bytes:65536,maximum_capacity_bytes:65536,maximum_release_bytes:65536,maximum_depth:32};
  let mut bank=EffectBank::new(Wallet::new(grant,law["fuel"].as_u64().unwrap()));let value=law["value"].as_u64().unwrap();let original=if name=="Ok"{Ok(value)}else{Err(value)};let mut cursor=original.retirement_seed();let mut authority=OriginalEffectAuthority::new(Close);let mut total=0u128;
  for denied in [true,false]{
   let demand=cursor.next_demand(65536).unwrap();let quoted=cursor.next_computational_work(65536).unwrap();let work=if denied{0}else{quoted};let g=bank.loan(1,demand.copy_bytes);let mut action=None;let mut reply=None;
   let ((poll,actual),heap)=observe_retirement_allocations(||bank.run(g,work,|effect|{let before=effect.performed_work();let mut context:&mut dyn RetirementCursor=&mut cursor;assert!(authority.drive(&mut context,effect,&mut action).unwrap());RetainedWorkPoll::Complete(RetainedWorkOutcome::<(),ValueError>{result:Ok(()),work:effect.performed_work()-before})}));
   let RetainedWorkPoll::Complete(turn)=poll else{panic!("native Result metadata must settle")};turn.result.unwrap();assert_eq!(actual,None);assert_eq!(heap,(0,0));assert!(action.as_mut().unwrap().transfer_reply(&mut reply).unwrap());authority.finish(&mut action).unwrap();let step=reply.unwrap().unwrap();let progress=match step{RetirementStep::BudgetExhausted=>{assert!(denied);Default::default()},RetirementStep::Progress(p,w)=>{assert!(!denied);assert_eq!(w,1);p},_=>panic!("unexpected initial Result step")};assert_eq!(bank.take_admitted(),if denied{None}else{Some(true)});bank.post(progress,g,heap);total+=turn.work;assert_eq!(cursor.original(),Some(original.as_ref().map_err(|e|e)));
  }
  for _ in 0..law["maximumTurns"].as_u64().unwrap(){if cursor.terminal_is_empty(){break}let demand=cursor.next_demand(65536).unwrap();let units=cursor.next_computational_work(65536).unwrap();let g=bank.loan(1,demand.copy_bytes);let mut action=None;let mut reply=None;let mut spent=0;let mut heap_sum=(0,0);
   for _ in 0..law["maximumTurns"].as_u64().unwrap(){let ((poll,actual),heap)=observe_retirement_allocations(||bank.run(g,units,|effect|{let before=effect.performed_work();let mut context:&mut dyn RetirementCursor=&mut cursor;let ready=authority.drive(&mut context,effect,&mut action).unwrap();let work=effect.performed_work()-before;if ready{RetainedWorkPoll::Complete(RetainedWorkOutcome::<(),ValueError>{result:Ok(()),work})}else{RetainedWorkPoll::Pending{work}}}));if let Some(actual)=actual{assert_eq!(actual,heap.0)}heap_sum.0+=heap.0;heap_sum.1+=heap.1;spent+=poll.work();if matches!(poll,RetainedWorkPoll::Complete(_)){assert!(action.as_mut().unwrap().transfer_reply(&mut reply).unwrap());authority.finish(&mut action).unwrap();break}}
   let step=reply.expect("actual Result reply").unwrap();let progress=match step{RetirementStep::Progress(p,_)=>p,RetirementStep::Complete(_)=>Default::default(),RetirementStep::BudgetExhausted=>panic!("funded Result stalled"),_=>panic!("unexpected Result child return")};assert_eq!(bank.take_admitted(),if spent==0{None}else{Some(true)});bank.post(progress,g,heap_sum);total+=spent;
  }
  assert!(cursor.terminal_is_empty());assert!(authority.into_producer().is_ok());assert_eq!(u128::from(law["fuel"].as_u64().unwrap())-u128::from(bank.wallet().remaining_work()),total);assert!(total>0);
 }
}

#[path="../../🎟️tickets/🎯️selection/🧪️tests/multi.rs"]
mod selected_item_authority;

#[derive(semio_framework_value_derive::RetireOwned)]
#[value(crate="semio_framework_value_retirement")]
struct DerivedBorrowedOwner{first:u64,second:u64}
struct DerivedClose;
impl OriginalEffectProducer for DerivedClose{
 type Output=RetainedCloneStep;type Failure=OriginalRetirementFailure;type Context<'a,'b>=&'a mut semio_framework_value_retirement::retirement::controlled::ControlledRetirement<DerivedBorrowedOwner> where 'b:'a;
 fn validate_context<'a,'b>(&self,_:&Self::Context<'a,'b>)->Result<(),ValueError>where 'b:'a{Ok(())}
 fn source_turn<'a,'b>(&self,_:&Self::Context<'a,'b>)->Option<OriginalSourceTurnIdentity>where 'b:'a{None}
 fn receive<'r,'a,'b>(&mut self,c:&mut Self::Context<'a,'b>,e:&mut RetainedAllocationEffect<'_>,r:OriginalEffectReply<'r,RetainedCloneStep,OriginalRetirementFailure>)->Result<OriginalEffectReplyComplete<'r,RetainedCloneStep,OriginalRetirementFailure>,ValueError>where 'b:'a{c.step_into(e,r)}
 fn recover<'r,'a,'b>(&mut self,c:&mut Self::Context<'a,'b>,e:&mut RetainedAllocationEffect<'_>,r:OriginalEffectReply<'r,RetainedCloneStep,OriginalRetirementFailure>)->Result<OriginalEffectReplyComplete<'r,RetainedCloneStep,OriginalRetirementFailure>,ValueError>where 'b:'a{c.recover_step_into(e,r)}
}
#[test]fn derived_retirement_forwards_actual_borrowed_children_to_terminal(){
 let law:serde_json::Value=serde_json::from_str(include_str!("borrowed.json")).unwrap();let grant=RetainedCloneGrant{maximum_items:256,maximum_copy_bytes:65536,maximum_capacity_bytes:65536,maximum_release_bytes:65536,maximum_depth:32};
 let mut bank=EffectBank::new(Wallet::new(grant,law["fuel"].as_u64().unwrap()));let value=law["value"].as_u64().unwrap();let mut cursor=semio_framework_value_retirement::retirement::controlled::ControlledRetirement::retain(DerivedBorrowedOwner{first:value,second:value});let mut authority=OriginalEffectAuthority::new(DerivedClose);let mut total=0;
 for _ in 0..law["maximumTurns"].as_u64().unwrap(){if cursor.terminal_is_empty(){break}let demand=cursor.next_demand(65536).unwrap();let units=cursor.next_computational_work(65536).unwrap();let g=bank.loan(256,demand.copy_bytes);let mut action=None;let mut answer=None;let mut heap_sum=(0,0);let mut spent=0;
  for _ in 0..law["maximumTurns"].as_u64().unwrap(){let ((poll,actual),heap)=observe_retirement_allocations(||bank.run(g,units,|effect|{let before=effect.performed_work();let ready=authority.drive(&mut &mut cursor,effect,&mut action).unwrap();let work=effect.performed_work()-before;if ready{RetainedWorkPoll::Complete(RetainedWorkOutcome::<(),ValueError>{result:Ok(()),work})}else{RetainedWorkPoll::Pending{work}}}));if let Some(actual)=actual{assert_eq!(actual,heap.0)}heap_sum.0+=heap.0;heap_sum.1+=heap.1;spent+=poll.work();if matches!(poll,RetainedWorkPoll::Complete(_)){assert!(action.as_mut().unwrap().transfer_reply(&mut answer).unwrap());authority.finish(&mut action).unwrap();break}}
  let progress=match answer.expect("derived original reply").unwrap(){RetainedCloneStep::Progress(p)|RetainedCloneStep::Complete(p)=>p};assert_eq!(bank.take_admitted(),if spent==0{None}else{Some(true)});bank.post(progress,g,heap_sum);total+=spent;
 }assert!(cursor.terminal_is_empty());assert!(authority.into_producer().is_ok());assert!(total>0);assert_eq!(u128::from(law["fuel"].as_u64().unwrap())-u128::from(bank.wallet().remaining_work()),total);
}

struct SharedBorrowedClose;
impl OriginalEffectProducer for SharedBorrowedClose{
 type Output=RetainedCloneStep;type Failure=OriginalRetirementFailure;type Context<'a,'b>=&'a mut semio_framework_value_retirement::retirement::arc_custody::SharedControlledRetirement<u64> where 'b:'a;
 fn validate_context<'a,'b>(&self,_:&Self::Context<'a,'b>)->Result<(),ValueError>where 'b:'a{Ok(())}
 fn source_turn<'a,'b>(&self,_:&Self::Context<'a,'b>)->Option<OriginalSourceTurnIdentity>where 'b:'a{None}
 fn receive<'r,'a,'b>(&mut self,c:&mut Self::Context<'a,'b>,e:&mut RetainedAllocationEffect<'_>,r:OriginalEffectReply<'r,Self::Output,Self::Failure>)->Result<OriginalEffectReplyComplete<'r,Self::Output,Self::Failure>,ValueError>where 'b:'a{c.step_into(e,r)}
 fn recover<'r,'a,'b>(&mut self,c:&mut Self::Context<'a,'b>,e:&mut RetainedAllocationEffect<'_>,r:OriginalEffectReply<'r,Self::Output,Self::Failure>)->Result<OriginalEffectReplyComplete<'r,Self::Output,Self::Failure>,ValueError>where 'b:'a{c.recover_step_into(e,r)}
}
#[test]
fn shared_borrowed_retirement_funds_both_actual_leases_on_one_bank(){
 use semio_framework_value_retirement::retirement::arc_custody::{SharedControlledRetirement,original::SharedOriginalStorage};
 let law:serde_json::Value=serde_json::from_str(include_str!("../../🔗️arc-custody/🧪️tests/borrowed.json")).unwrap();let grant=RetainedCloneGrant{maximum_items:law["stockItems"].as_u64().unwrap()as usize,maximum_copy_bytes:law["stockCopyBytes"].as_u64().unwrap()as usize,maximum_capacity_bytes:65536,maximum_release_bytes:65536,maximum_depth:32};let mut bank=EffectBank::new(Wallet::new(grant,law["fuel"].as_u64().unwrap()));
 let mut source=Some(law["sourceValue"].as_u64().unwrap());let(mut first,mut second)=(None,None);let demand=SharedOriginalStorage::<u64>::source_admission_demand().unwrap();let g=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:demand.copy_bytes,maximum_capacity_bytes:demand.capacity_bytes,maximum_release_bytes:demand.release_bytes,maximum_depth:demand.depth};let((turn,actual),heap)=observe_retirement_allocations(||bank.run(g,SharedOriginalStorage::<u64>::source_admission_work_demand(),|effect|RetainedWorkPoll::Complete(SharedOriginalStorage::admit_from(&mut source,&mut first,effect))));let turn=match turn{RetainedWorkPoll::Complete(turn)=>turn,_=>panic!("issued shared birth pending")};let progress=turn.result.unwrap().progress();assert_eq!(actual,Some(progress.retained_capacity_bytes));assert_eq!(bank.take_admitted(),Some(true));bank.post(progress,g,heap);let mut total=turn.work;
 let demand=SharedOriginalStorage::<u64>::duplicate_demand();let g=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:demand.copy_bytes,maximum_capacity_bytes:demand.capacity_bytes,maximum_release_bytes:demand.release_bytes,maximum_depth:demand.depth};let((turn,actual),heap)=observe_retirement_allocations(||bank.run(g,first.as_ref().unwrap().duplicate_work_demand().unwrap(),|effect|RetainedWorkPoll::Complete(first.as_ref().unwrap().duplicate_into(&mut second,effect))));let turn=match turn{RetainedWorkPoll::Complete(turn)=>turn,_=>panic!("issued alias duplicate pending")};assert_eq!(actual,None);let progress=turn.result.unwrap().progress();assert_eq!(bank.take_admitted(),Some(true));bank.post(progress,g,heap);total+=turn.work;assert!(first.as_ref().unwrap().same_original(first.as_ref().unwrap()));assert!(first.as_ref().unwrap().same_original(second.as_ref().unwrap()));
 let mut first=SharedControlledRetirement::new(first.take().unwrap());let mut second=SharedControlledRetirement::new(second.take().unwrap());let mut authority=OriginalEffectAuthority::new(SharedBorrowedClose);
 for owner in [&mut first,&mut second]{
  let demand=owner.next_demand(4096).unwrap();let mut denied=bank.loan(1,demand.copy_bytes);denied.maximum_copy_bytes=0;let(mut action,mut answer)=(None,None);let((poll,_),heap)=observe_retirement_allocations(||bank.run(denied,0,|effect|{let before=effect.performed_work();assert!(authority.drive(&mut &mut *owner,effect,&mut action).unwrap());RetainedWorkPoll::Complete(RetainedWorkOutcome::<(),ValueError>{result:Ok(()),work:effect.performed_work()-before})}));assert_eq!(poll.work(),0);assert!(action.as_mut().unwrap().transfer_reply(&mut answer).unwrap());authority.finish(&mut action).unwrap();assert_eq!(answer.take().unwrap().unwrap().progress(),Default::default());assert!(owner.next_demand(4096).unwrap()==demand);assert_eq!(bank.take_admitted(),None);bank.post(Default::default(),denied,heap);
  for _ in 0..law["maximumTurns"].as_u64().unwrap(){if owner.terminal_is_empty(){break}let demand=owner.next_demand(4096).unwrap();assert!(demand.copy_bytes<=4096);let work=owner.next_computational_work(4096).unwrap();let mut g=bank.loan(1,demand.copy_bytes);g.maximum_capacity_bytes=demand.capacity_bytes;g.maximum_release_bytes=demand.release_bytes;g.maximum_depth=demand.depth;let(mut action,mut answer)=(None,None);let mut observed=(0usize,0usize);let mut spent=0;
   for _ in 0..law["maximumTurns"].as_u64().unwrap(){let((poll,actual),heap)=observe_retirement_allocations(||bank.run(g,work,|effect|{let before=effect.performed_work();let ready=authority.drive(&mut &mut *owner,effect,&mut action).unwrap();let work=effect.performed_work()-before;if ready{RetainedWorkPoll::Complete(RetainedWorkOutcome::<(),ValueError>{result:Ok(()),work})}else{RetainedWorkPoll::Pending{work}}}));if let Some(actual)=actual{assert_eq!(actual,heap.0)}observed.0+=heap.0;observed.1+=heap.1;spent+=poll.work();if matches!(poll,RetainedWorkPoll::Complete(_)){assert!(action.as_mut().unwrap().transfer_reply(&mut answer).unwrap());authority.finish(&mut action).unwrap();break}}
   let progress=answer.expect("actual shared reply").unwrap().progress();assert!(progress.copied_items<=1);assert_eq!(bank.take_admitted(),if spent==0{None}else{Some(true)});bank.post(progress,g,observed);total+=spent;
  }assert!(owner.terminal_is_empty());
 }
 assert!(authority.into_producer().is_ok());assert_eq!(u128::from(law["fuel"].as_u64().unwrap())-u128::from(bank.wallet().remaining_work()),total);
}

static SHARED_UNWIND_CALLS:std::sync::atomic::AtomicUsize=std::sync::atomic::AtomicUsize::new(0);
static SHARED_UNWIND_RECOVERS:std::sync::atomic::AtomicUsize=std::sync::atomic::AtomicUsize::new(0);
struct SharedUnwindValue(u64);
struct SharedUnwindCursor(bool);
impl RetireOwned for SharedUnwindValue{
 type RetirementCursor=SharedUnwindCursor;fn retirement_seed(self)->Self::RetirementCursor{assert_eq!(self.0,42);SharedUnwindCursor(true)}
 fn retirement_birth_bytes(&self)->Option<usize>{Some(std::mem::size_of::<SharedUnwindCursor>())}
 fn controlled_retirement_supported()->bool{true}
}
impl RetirementCursor for SharedUnwindCursor{
 fn next_demand(&self,_:usize)->Result<RetirementDemand,ValueError>{Ok(RetirementDemand{copy_bytes:usize::from(self.0),depth:usize::from(self.0),..Default::default()})}
 fn next_computational_work(&self,_:usize)->Result<u64,ValueError>{Ok(u64::from(self.0))}
 fn terminal_is_empty(&self)->bool{!self.0}
 fn terminal_release_bytes(&self)->Option<usize>{self.terminal_is_empty().then_some(std::mem::size_of::<Self>())}
 fn close_into<'r>(&mut self,effect:&mut RetainedAllocationEffect<'_>,reply:OriginalEffectReply<'r,RetirementStep,OriginalRetirementFailure>)->Result<OriginalEffectReplyComplete<'r,RetirementStep,OriginalRetirementFailure>,ValueError>{if !self.0{return Ok(reply.complete(Ok(RetirementStep::Complete(0))))}if effect.remaining_work_units()?<1||!retained_clone_grant_admits(effect.grant(),self.next_demand(0)?){return Ok(reply.complete(Ok(RetirementStep::Progress(Default::default(),0))))}SHARED_UNWIND_CALLS.fetch_add(1,std::sync::atomic::Ordering::SeqCst);assert!(effect.performed(1)?);self.0=false;let _recorded=reply.complete(Ok(RetirementStep::Progress(RetainedCloneProgress{copied_items:1,copied_bytes:1,..Default::default()},1)));panic!("shared child recorded its original reply")}
 fn recover_close_into<'r>(&mut self,_:&mut RetainedAllocationEffect<'_>,reply:OriginalEffectReply<'r,RetirementStep,OriginalRetirementFailure>)->Result<OriginalEffectReplyComplete<'r,RetirementStep,OriginalRetirementFailure>,ValueError>{SHARED_UNWIND_RECOVERS.fetch_add(1,std::sync::atomic::Ordering::SeqCst);Ok(reply.pending())}
}
struct SharedUnwindClose;
impl OriginalEffectProducer for SharedUnwindClose{
 type Output=RetainedCloneStep;type Failure=OriginalRetirementFailure;type Context<'a,'b>=&'a mut semio_framework_value_retirement::retirement::arc_custody::SharedControlledRetirement<SharedUnwindValue> where 'b:'a;
 fn validate_context<'a,'b>(&self,_:&Self::Context<'a,'b>)->Result<(),ValueError>where 'b:'a{Ok(())}
 fn source_turn<'a,'b>(&self,_:&Self::Context<'a,'b>)->Option<OriginalSourceTurnIdentity>where 'b:'a{None}
 fn receive<'r,'a,'b>(&mut self,c:&mut Self::Context<'a,'b>,e:&mut RetainedAllocationEffect<'_>,r:OriginalEffectReply<'r,Self::Output,Self::Failure>)->Result<OriginalEffectReplyComplete<'r,Self::Output,Self::Failure>,ValueError>where 'b:'a{c.step_into(e,r)}
 fn recover<'r,'a,'b>(&mut self,c:&mut Self::Context<'a,'b>,e:&mut RetainedAllocationEffect<'_>,r:OriginalEffectReply<'r,Self::Output,Self::Failure>)->Result<OriginalEffectReplyComplete<'r,Self::Output,Self::Failure>,ValueError>where 'b:'a{c.recover_step_into(e,r)}
}
#[test]
fn shared_borrowed_child_reply_survives_unwind_without_replay(){
 use semio_framework_value_retirement::retirement::arc_custody::{SharedControlledRetirement,original::SharedOriginalStorage};use std::sync::atomic::Ordering;
 let law:serde_json::Value=serde_json::from_str(include_str!("../../🔗️arc-custody/🧪️tests/borrowed.json")).unwrap();SHARED_UNWIND_CALLS.store(0,Ordering::SeqCst);SHARED_UNWIND_RECOVERS.store(0,Ordering::SeqCst);let stock=RetainedCloneGrant{maximum_items:law["stockItems"].as_u64().unwrap()as usize,maximum_copy_bytes:law["stockCopyBytes"].as_u64().unwrap()as usize,maximum_capacity_bytes:65536,maximum_release_bytes:65536,maximum_depth:32};let mut bank=EffectBank::new(Wallet::new(stock,law["fuel"].as_u64().unwrap()));
 let(mut source,mut issued)=(Some(SharedUnwindValue(law["sourceValue"].as_u64().unwrap())),None);let d=SharedOriginalStorage::<SharedUnwindValue>::source_admission_demand().unwrap();let g=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:d.copy_bytes,maximum_capacity_bytes:d.capacity_bytes,maximum_release_bytes:d.release_bytes,maximum_depth:d.depth};let((turn,actual),heap)=observe_retirement_allocations(||bank.run(g,SharedOriginalStorage::<SharedUnwindValue>::source_admission_work_demand(),|effect|RetainedWorkPoll::Complete(SharedOriginalStorage::admit_from(&mut source,&mut issued,effect))));let turn=match turn{RetainedWorkPoll::Complete(turn)=>turn,_=>panic!("issued shared birth pending")};let p=turn.result.unwrap().progress();assert_eq!(actual,Some(p.retained_capacity_bytes));assert_eq!(bank.take_admitted(),Some(true));bank.post(p,g,heap);let mut total=turn.work;
 let mut owner=SharedControlledRetirement::new(issued.take().unwrap());let mut authority=OriginalEffectAuthority::new(SharedUnwindClose);let mut interrupted=false;
 for _ in 0..law["maximumTurns"].as_u64().unwrap(){if owner.terminal_is_empty(){break}let d=owner.next_demand(4096).unwrap();assert!(d.copy_bytes<=4096);let work=owner.next_computational_work(4096).unwrap();let g=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:d.copy_bytes,maximum_capacity_bytes:d.capacity_bytes,maximum_release_bytes:d.release_bytes,maximum_depth:d.depth};let(mut action,mut answer)=(None,None);let mut observed=(0usize,0usize);let mut spent=0u128;let mut had_unwind=false;
  for _ in 0..law["maximumTurns"].as_u64().unwrap(){let before_bank=bank.wallet().remaining_work();let(result,heap)=observe_retirement_allocations(||std::panic::catch_unwind(std::panic::AssertUnwindSafe(||bank.run(g,work,|effect|{let before=effect.performed_work();let ready=authority.drive(&mut &mut owner,effect,&mut action).unwrap();let actual=effect.performed_work()-before;if ready{RetainedWorkPoll::Complete(RetainedWorkOutcome::<(),ValueError>{result:Ok(()),work:actual})}else{RetainedWorkPoll::Pending{work:actual}}}))));let observed_work=u128::from(before_bank-bank.wallet().remaining_work());spent+=observed_work;match result{Err(_)=>{assert!(!interrupted);interrupted=true;had_unwind=true;assert_eq!(bank.actual(),None);assert!(observed_work>0);continue},Ok((poll,actual))=>{assert_eq!(poll.work(),observed_work);if let Some(actual)=actual{assert_eq!(actual,heap.0)}observed.0+=heap.0;observed.1+=heap.1;if matches!(poll,RetainedWorkPoll::Complete(_)){assert!(action.as_mut().unwrap().transfer_reply(&mut answer).unwrap());authority.finish(&mut action).unwrap();break}}}}
  let progress=answer.expect("actual shared child reply").unwrap().progress();assert!(progress.copied_items<=1);assert_eq!(bank.take_admitted(),if spent==0&&!had_unwind{None}else{Some(true)});bank.post(progress,g,observed);total+=spent;
 }
 assert!(interrupted);assert!(owner.terminal_is_empty());assert!(authority.into_producer().is_ok());assert_eq!(SHARED_UNWIND_CALLS.load(Ordering::SeqCst),law["unwindCalls"].as_u64().unwrap()as usize);assert_eq!(SHARED_UNWIND_RECOVERS.load(Ordering::SeqCst),law["unwindRecoveries"].as_u64().unwrap()as usize);assert_eq!(u128::from(law["fuel"].as_u64().unwrap())-u128::from(bank.wallet().remaining_work()),total);
}

static SHARED_PENDING_CALLS:std::sync::atomic::AtomicUsize=std::sync::atomic::AtomicUsize::new(0);
static SHARED_PENDING_RECOVERS:std::sync::atomic::AtomicUsize=std::sync::atomic::AtomicUsize::new(0);
struct SharedPendingValue(u64);
struct SharedPendingCursor(u8);
impl RetireOwned for SharedPendingValue{
 type RetirementCursor=SharedPendingCursor;fn retirement_seed(self)->Self::RetirementCursor{assert_eq!(self.0,42);SharedPendingCursor(0)}
 fn retirement_birth_bytes(&self)->Option<usize>{Some(std::mem::size_of::<SharedPendingCursor>())}
 fn controlled_retirement_supported()->bool{true}
}
impl RetirementCursor for SharedPendingCursor{
 fn next_demand(&self,_:usize)->Result<RetirementDemand,ValueError>{Ok(if self.0==0{RetirementDemand{copy_bytes:2,depth:1,..Default::default()}}else{Default::default()})}
 fn next_computational_work(&self,_:usize)->Result<u64,ValueError>{Ok(if self.0==0{2}else{0})}
 fn terminal_is_empty(&self)->bool{self.0==2}
 fn terminal_release_bytes(&self)->Option<usize>{self.terminal_is_empty().then_some(std::mem::size_of::<Self>())}
 fn close_into<'r>(&mut self,effect:&mut RetainedAllocationEffect<'_>,reply:OriginalEffectReply<'r,RetirementStep,OriginalRetirementFailure>)->Result<OriginalEffectReplyComplete<'r,RetirementStep,OriginalRetirementFailure>,ValueError>{assert_eq!(self.0,0);SHARED_PENDING_CALLS.fetch_add(1,std::sync::atomic::Ordering::SeqCst);assert!(effect.performed(1)?);self.0=1;Ok(reply.pending())}
 fn recover_close_into<'r>(&mut self,effect:&mut RetainedAllocationEffect<'_>,reply:OriginalEffectReply<'r,RetirementStep,OriginalRetirementFailure>)->Result<OriginalEffectReplyComplete<'r,RetirementStep,OriginalRetirementFailure>,ValueError>{assert_eq!(self.0,1);SHARED_PENDING_RECOVERS.fetch_add(1,std::sync::atomic::Ordering::SeqCst);assert!(effect.performed(1)?);self.0=2;Ok(reply.complete(Ok(RetirementStep::Progress(RetainedCloneProgress{copied_items:1,copied_bytes:2,..Default::default()},1))))}
}
struct SharedPendingClose;
impl OriginalEffectProducer for SharedPendingClose{
 type Output=RetainedCloneStep;type Failure=OriginalRetirementFailure;type Context<'a,'b>=&'a mut semio_framework_value_retirement::retirement::arc_custody::SharedControlledRetirement<SharedPendingValue> where 'b:'a;
 fn validate_context<'a,'b>(&self,_:&Self::Context<'a,'b>)->Result<(),ValueError>where 'b:'a{Ok(())}
 fn source_turn<'a,'b>(&self,_:&Self::Context<'a,'b>)->Option<OriginalSourceTurnIdentity>where 'b:'a{None}
 fn receive<'r,'a,'b>(&mut self,c:&mut Self::Context<'a,'b>,e:&mut RetainedAllocationEffect<'_>,r:OriginalEffectReply<'r,Self::Output,Self::Failure>)->Result<OriginalEffectReplyComplete<'r,Self::Output,Self::Failure>,ValueError>where 'b:'a{c.step_into(e,r)}
 fn recover<'r,'a,'b>(&mut self,c:&mut Self::Context<'a,'b>,e:&mut RetainedAllocationEffect<'_>,r:OriginalEffectReply<'r,Self::Output,Self::Failure>)->Result<OriginalEffectReplyComplete<'r,Self::Output,Self::Failure>,ValueError>where 'b:'a{c.recover_step_into(e,r)}
}
#[test]
fn shared_borrowed_pending_child_keeps_its_original_action_and_quote(){
 use semio_framework_value_retirement::retirement::arc_custody::{SharedControlledRetirement,original::SharedOriginalStorage};use std::sync::atomic::Ordering;
 let law:serde_json::Value=serde_json::from_str(include_str!("../../🔗️arc-custody/🧪️tests/borrowed.json")).unwrap();SHARED_PENDING_CALLS.store(0,Ordering::SeqCst);SHARED_PENDING_RECOVERS.store(0,Ordering::SeqCst);let stock=RetainedCloneGrant{maximum_items:law["stockItems"].as_u64().unwrap()as usize,maximum_copy_bytes:law["stockCopyBytes"].as_u64().unwrap()as usize,maximum_capacity_bytes:65536,maximum_release_bytes:65536,maximum_depth:32};let mut bank=EffectBank::new(Wallet::new(stock,law["fuel"].as_u64().unwrap()));
 let(mut source,mut issued)=(Some(SharedPendingValue(law["sourceValue"].as_u64().unwrap())),None);let d=SharedOriginalStorage::<SharedPendingValue>::source_admission_demand().unwrap();let g=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:d.copy_bytes,maximum_capacity_bytes:d.capacity_bytes,maximum_release_bytes:d.release_bytes,maximum_depth:d.depth};let((turn,actual),heap)=observe_retirement_allocations(||bank.run(g,SharedOriginalStorage::<SharedPendingValue>::source_admission_work_demand(),|effect|RetainedWorkPoll::Complete(SharedOriginalStorage::admit_from(&mut source,&mut issued,effect))));let turn=match turn{RetainedWorkPoll::Complete(turn)=>turn,_=>panic!("issued shared birth pending")};let p=turn.result.unwrap().progress();assert_eq!(actual,Some(p.retained_capacity_bytes));assert_eq!(bank.take_admitted(),Some(true));bank.post(p,g,heap);let mut total=turn.work;
 let mut owner=SharedControlledRetirement::new(issued.take().unwrap());let mut authority=OriginalEffectAuthority::new(SharedPendingClose);let mut saw_pending=false;
 for _ in 0..law["maximumTurns"].as_u64().unwrap(){if owner.terminal_is_empty(){break}let d=owner.next_demand(4096).unwrap();assert!(d.copy_bytes<=4096);let work=owner.next_computational_work(4096).unwrap();let g=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:d.copy_bytes,maximum_capacity_bytes:d.capacity_bytes,maximum_release_bytes:d.release_bytes,maximum_depth:d.depth};let(mut action,mut answer)=(None,None);let mut observed=(0usize,0usize);let mut spent=0u128;
  for _ in 0..law["maximumTurns"].as_u64().unwrap(){let((poll,actual),heap)=observe_retirement_allocations(||bank.run(g,work,|effect|{let before=effect.performed_work();let ready=authority.drive(&mut &mut owner,effect,&mut action).unwrap();let actual=effect.performed_work()-before;if ready{RetainedWorkPoll::Complete(RetainedWorkOutcome::<(),ValueError>{result:Ok(()),work:actual})}else{RetainedWorkPoll::Pending{work:actual}}}));if let Some(actual)=actual{assert_eq!(actual,heap.0)}observed.0+=heap.0;observed.1+=heap.1;spent+=poll.work();if matches!(poll,RetainedWorkPoll::Pending{..}){saw_pending=true;assert_eq!(owner.next_demand(4096).unwrap(),d);assert_eq!(owner.next_computational_work(4096).unwrap(),work);continue}assert!(action.as_mut().unwrap().transfer_reply(&mut answer).unwrap());authority.finish(&mut action).unwrap();break}
  let progress=answer.expect("actual shared pending reply").unwrap().progress();assert!(progress.copied_items<=1);assert_eq!(bank.take_admitted(),if spent==0{None}else{Some(true)});bank.post(progress,g,observed);total+=spent;
 }
 assert!(saw_pending);assert!(owner.terminal_is_empty());assert!(authority.into_producer().is_ok());assert_eq!(SHARED_PENDING_CALLS.load(Ordering::SeqCst),law["pendingCalls"].as_u64().unwrap()as usize);assert_eq!(SHARED_PENDING_RECOVERS.load(Ordering::SeqCst),law["pendingRecoveries"].as_u64().unwrap()as usize);assert_eq!(u128::from(law["fuel"].as_u64().unwrap())-u128::from(bank.wallet().remaining_work()),total);
}
