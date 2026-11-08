//! ♻️ One finite Source drives and retires the actual controlled cleanup producer.
use super::*;
use semio_framework_value_control_contract::*;
use semio_framework_value_retirement::{ErasedSnapshotRetirement,OriginalRetirementFailure,retirement::{RetireOwned,controlled::ControlledRetirement}};
use semio_framework_value_testing_effect_bank::EffectBank;
use std::mem::{size_of,ManuallyDrop};

struct Producer<T:RetireOwned>{owner:ManuallyDrop<ControlledRetirement<T>>,source:OriginalSourceTurnIdentity}
impl<T:RetireOwned> Drop for Producer<T>{fn drop(&mut self){if std::thread::panicking(){return}assert!(self.owner.terminal_is_empty(),"cleanup producer retains its original owner");unsafe{ManuallyDrop::drop(&mut self.owner)}}}
impl<T:RetireOwned> OriginalEffectProducer for Producer<T>{
 type Output=RetainedCloneStep;type Failure=OriginalRetirementFailure;type Context<'a,'b>=OriginalSourceTurnIdentity where 'b:'a;
 fn validate_context<'a,'b>(&self,source:&Self::Context<'a,'b>)->Result<(),ValueError>where 'b:'a{if *source==self.source{Ok(())}else{Err(ValueError::invariant("cleanup changed original Source"))}}
 fn source_turn<'a,'b>(&self,source:&Self::Context<'a,'b>)->Option<OriginalSourceTurnIdentity>where 'b:'a{Some(*source)}
 fn receive<'r,'a,'b>(&mut self,_:&mut Self::Context<'a,'b>,effect:&mut RetainedAllocationEffect<'_>,reply:OriginalEffectReply<'r,Self::Output,Self::Failure>)->Result<OriginalEffectReplyComplete<'r,Self::Output,Self::Failure>,ValueError>where 'b:'a{self.owner.close_into(4096,effect,reply)}
 fn recover<'r,'a,'b>(&mut self,_:&mut Self::Context<'a,'b>,effect:&mut RetainedAllocationEffect<'_>,reply:OriginalEffectReply<'r,Self::Output,Self::Failure>)->Result<OriginalEffectReplyComplete<'r,Self::Output,Self::Failure>,ValueError>where 'b:'a{self.owner.recover_close_into(4096,effect,reply)}
}

fn grant(d:RetirementDemand)->RetainedCloneGrant{RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:d.copy_bytes,maximum_capacity_bytes:d.capacity_bytes,maximum_release_bytes:d.release_bytes,maximum_depth:d.depth}}
fn accept(bank:&mut EffectBank,progress:RetainedCloneProgress,grant:RetainedCloneGrant,heap:(usize,usize),work:u128){assert_eq!(bank.take_admitted(),if work==0{None}else{Some(true)});bank.post(progress,grant,heap)}

/// 🧾️ Runs one settled native operation against the caller's same original finite bank.
pub(crate) fn run<T,E>(bank:&mut EffectBank,grant:RetainedCloneGrant,maximum:u64,operation:impl FnOnce(&mut RetainedAllocationEffect<'_>)->RetainedWorkPoll<T,E>)->(RetainedWorkPoll<T,E>,Option<usize>,(usize,usize)){let((poll,marker),heap)=semio_framework_value_testing_allocator::observe_retirement_allocations(||bank.run(grant,maximum,operation));(poll,marker,heap)}
/// 💳️ Settles the actual marker, Work verdict and independently observed heap.
pub(crate) fn post(bank:&mut EffectBank,grant:RetainedCloneGrant,work:u128,progress:RetainedCloneProgress,marker:Option<usize>,heap:(usize,usize)){if let Some(bytes)=marker{assert_eq!(bytes,progress.retained_capacity_bytes)}assert_eq!(heap,(progress.retained_capacity_bytes,progress.released_bytes));accept(bank,progress,grant,heap,work)}

/// 🧵️ Retires one actual original without replacing its Source or replaying a child.
pub(crate) fn close<T:RetireOwned>(value:T,bank:&mut EffectBank,maximum_turns:usize){
 let source=bank.source_witness().identity();let owner=ControlledRetirement::new(value).map_err(|(error,_)|error).unwrap();let mut authority=Some(OriginalEffectAuthority::new(Producer{owner:ManuallyDrop::new(owner),source}));let mut action=None;let mut answer=None;
 for _ in 0..maximum_turns{
  if authority.as_ref().unwrap().inspect(|p|p.owner.terminal_is_empty()){break}
  let(d,work)=authority.as_ref().unwrap().inspect(|p|{assert!(!p.owner.requires_original_birth()&&!p.owner.requires_original_control(),"cleanup needs its original close loan");(p.owner.next_demand(4096).unwrap(),p.owner.next_computational_work(4096).unwrap())});let g=grant(d);let mut heap=(0,0);let mut performed=0;
  loop{let((poll,marker),observed)=semio_framework_value_testing_allocator::observe_retirement_allocations(||bank.run_with_source(g,work,|effect,witness|{let before=effect.performed_work();let mut context=witness.identity();let ready=authority.as_mut().unwrap().drive(&mut context,effect,&mut action).unwrap();let delta=effect.performed_work()-before;if ready{RetainedWorkPoll::Complete(RetainedWorkOutcome{result:Ok::<_,ValueError>(()),work:delta})}else{RetainedWorkPoll::Pending{work:delta}}}));heap.0+=observed.0;heap.1+=observed.1;performed+=poll.work();if let Some(bytes)=marker{assert_eq!(bytes,heap.0)}if let RetainedWorkPoll::Complete(done)=poll{done.result.unwrap();assert!(action.as_mut().unwrap().transfer_reply(&mut answer).unwrap());authority.as_mut().unwrap().finish(&mut action).unwrap();let step=answer.take().unwrap().unwrap();assert_eq!(heap,(step.progress().retained_capacity_bytes,step.progress().released_bytes));accept(bank,step.progress(),g,heap,performed);break}}
 }
 assert!(authority.as_ref().unwrap().inspect(|p|p.owner.terminal_is_empty()),"cleanup exceeded its original turn limit");let demand=OriginalEffectAuthority::<Producer<T>>::producer_transfer_demand().unwrap();let g=grant(demand);let mut producer=None;let(poll,marker,heap)=run(bank,g,1,|effect|RetainedWorkPoll::Complete(OriginalEffectAuthority::transfer_producer(&mut authority,&source,&mut producer,effect)));let RetainedWorkPoll::Complete(done)=poll else{panic!("cleanup authority transfer remained pending")};let progress=done.result.unwrap().unwrap();post(bank,g,done.work,progress,marker,heap);
 let demand=RetirementDemand{copy_bytes:size_of::<Option<Producer<T>>>(),depth:1,..Default::default()};let g=grant(demand);let(poll,marker,heap)=run(bank,g,1,|effect|{assert!(effect.performed(1).unwrap());drop(producer.take());RetainedWorkPoll::Complete(RetainedWorkOutcome::<_,ValueError>{result:Ok(()),work:1})});let RetainedWorkPoll::Complete(done)=poll else{panic!("cleanup producer release remained pending")};done.result.unwrap();post(bank,g,done.work,RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,..Default::default()},marker,heap);assert!(bank.is_idle());
}
