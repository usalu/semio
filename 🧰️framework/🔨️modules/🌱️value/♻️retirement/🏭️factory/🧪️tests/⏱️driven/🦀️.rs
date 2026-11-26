//! ⏱️ Funded original turns of the factory laws run under a real finite Source and Work bank.
use crate::*;
use semio_framework_value_clone_source_testing::drive;
use semio_framework_value_testing_effect_bank::{EffectBank,Wallet};
/// 🧾️ Generous per-turn Work ceiling of the fixture turns.
pub const CPU:u64=1<<20;
/// 🎟️ Funds exactly one item of the quoted demand.
pub fn grant_of(d:RetirementDemand)->RetainedCloneGrant{RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:d.copy_bytes,maximum_capacity_bytes:d.capacity_bytes,maximum_release_bytes:d.release_bytes,maximum_depth:d.depth}}
/// 🧮️ Reads the physical heap that a recorded outcome proves.
pub fn heap_of(turn:&RetainedWorkOutcome<RetainedCloneStep,OriginalRetirementFailure>)->(usize,usize){let progress=match &turn.result{Ok(step)=>step.progress(),Err(failure)=>failure.retained_progress()};(progress.retained_capacity_bytes,progress.released_bytes)}
/// 🎞️ Drives one poll-shaped turn under a real original action; its physical heap equals its recorded receipt.
pub fn poll(grant:RetainedCloneGrant,work:u64,call:impl FnMut(&mut RetainedAllocationEffect<'_>)->OriginalRetirementPoll)->(RetainedWorkOutcome<RetainedCloneStep,OriginalRetirementFailure>,(usize,usize)){drive::warm();let turn=drive::drive(grant,work,call);let heap=heap_of(&turn);(turn,heap)}
/// 🏦️ Builds a fresh finite original bank.
pub fn bank()->EffectBank{EffectBank::new(Wallet::new(RetainedCloneGrant{maximum_items:1<<30,maximum_copy_bytes:usize::MAX>>2,maximum_capacity_bytes:usize::MAX>>2,maximum_release_bytes:usize::MAX>>2,maximum_depth:1<<20},1<<50))}
/// 🪢️ Runs one synchronous operation on a fresh bank and reports its result, physical heap and allocation marker.
pub fn leaf<R>(grant:RetainedCloneGrant,work:u64,operation:impl FnOnce(&mut RetainedAllocationEffect<'_>)->R)->(R,(usize,usize),Option<usize>){
 let mut bank=bank();let mut output=None;
 let((poll,actual),heap)=crate::observe_retirement_allocations(||bank.run(grant,work,|effect|{let before=effect.performed_work();output=Some(operation(effect));RetainedWorkPoll::Complete(RetainedWorkOutcome::<(),ValueError>{result:Ok(()),work:effect.performed_work()-before})}));
 let RetainedWorkPoll::Complete(turn)=poll else{panic!("original leaf stayed pending")};turn.result.unwrap();let _=bank.take_admitted();
 (output.take().expect("original leaf ran"),heap,actual)
}
/// 🏭️ Settles one resident value through the common owned producer and records its observed original Work.
pub fn settle_owned<'reply,T:RetireOwned>(value:&mut Option<T>,effect:&mut RetainedAllocationEffect<'_>,mut reply:FactoryProducerReply<'reply,dyn ErasedSnapshotRetirement,OriginalRetirementFailure>,boundary:OriginalWorkPerformedBoundary)->FactoryProducerReplyComplete<'reply,OriginalRetirementFailure>{
 let turn=retirement::admit_owned_retirement(value,effect);let result=match turn.result{Ok(Some((owner,progress)))=>{reply.publish(owner);Ok(progress)},Ok(None)=>Ok(Default::default()),Err(error)=>Err(error.into())};reply.complete_observed(effect,boundary,result)
}
/// 📨️ Runs one reply-shaped call on a fresh bank and reports its recorded answer with the work, physical heap and allocation marker of the turn.
pub fn replied(grant:RetainedCloneGrant,work:u64,call:impl for<'r> FnOnce(&mut RetainedAllocationEffect<'_>,OriginalEffectReply<'r,RetainedCloneStep,OriginalRetirementFailure>)->Result<OriginalEffectReplyComplete<'r,RetainedCloneStep,OriginalRetirementFailure>,ValueError>)->(RetainedWorkOutcome<RetainedCloneStep,OriginalRetirementFailure>,(usize,usize),Option<usize>){
 let(answer,heap,actual)=leaf(grant,work,|effect|{let before=effect.performed_work();let mut answer=None;let reply=OriginalEffectReply::borrow(&mut answer).expect("fresh reply slot");drop(call(effect,reply).expect("the original action is not refused"));(answer.expect("synchronous reply"),effect.performed_work()-before)});
 let(result,work)=answer;(RetainedWorkOutcome{result,work},heap,actual)
}
/// 🎞️ Drives one reply-shaped call under a real original action, re-entering it as recovery after every Pending, and reports its recorded reply with its physical heap.
pub fn driven_reply<T:'static,E:'static+From<ValueError>>(grant:RetainedCloneGrant,work:u64,progress:impl Fn(&Result<T,E>)->RetainedCloneProgress,call:impl for<'r> FnMut(&mut RetainedAllocationEffect<'_>,OriginalEffectReply<'r,T,E>,bool)->Result<OriginalEffectReplyComplete<'r,T,E>,ValueError>)->(RetainedWorkOutcome<T,E>,(usize,usize)){
 drive::warm();let turn=drive::drive_reply(grant,work,&progress,call);let p=progress(&turn.result);let heap=(p.retained_capacity_bytes,p.released_bytes);(turn,heap)
}
/// 🎞️ Drives one ordinary original-retirement reply call and reports its recorded reply with its physical heap.
pub fn driven_step(grant:RetainedCloneGrant,work:u64,call:impl for<'r> FnMut(&mut RetainedAllocationEffect<'_>,OriginalEffectReply<'r,RetainedCloneStep,OriginalRetirementFailure>,bool)->Result<OriginalEffectReplyComplete<'r,RetainedCloneStep,OriginalRetirementFailure>,ValueError>)->(RetainedWorkOutcome<RetainedCloneStep,OriginalRetirementFailure>,(usize,usize)){driven_reply(grant,work,|reply:&Result<RetainedCloneStep,OriginalRetirementFailure>|match reply{Ok(step)=>step.progress(),Err(failure)=>failure.retained_progress()},call)}
/// 🚫️ Runs one under-funded poll-shaped turn on a fresh bank: the poll form answers a denial as Pending or as zero progress and never touches the heap.
pub fn underfunded(grant:RetainedCloneGrant,work:u64,call:impl FnOnce(&mut RetainedAllocationEffect<'_>)->OriginalRetirementPoll)->(RetainedWorkOutcome<RetainedCloneStep,OriginalRetirementFailure>,(usize,usize)){
 let(turn,heap,_)=leaf(grant,work,call);
 let turn=match turn{RetainedWorkPoll::Complete(turn)=>turn,RetainedWorkPoll::Pending{work}=>RetainedWorkOutcome{result:Ok(RetainedCloneStep::Progress(Default::default())),work}};
 (turn,heap)
}
