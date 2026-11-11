//! 🎟️ Original erased child closure preserves its selected result, physical receipt and actual CPU.
use crate::{ErasedSnapshotRetirement,OriginalRetirementPoll,RetirementDemand,RetainedWorkOutcome,RetainedWorkPoll,ValueError,ValueRefusalKind,RetainedCloneGrant,RetainedCloneProgress,RetainedCloneStep};
pub fn factory_ticket_demands<T:ErasedSnapshotRetirement+?Sized>(ticket:&Box<T>,copy:usize)->Result<RetirementDemand,ValueError>{
 if ticket.terminal_is_empty(){return ticket.native_header_release().ok_or_else(||ValueError::literal(ValueRefusalKind::UnsupportedOwner,"terminal ticket has no explicit native header lifecycle"))?.demand(ticket.as_ref())}ticket.next_demand(copy)
}
/// 🎟️ Quotes the original recipient's refill turn before its selected child.
pub fn factory_ticket_controlled_demands<T:ErasedSnapshotRetirement+?Sized>(ticket:&Box<T>,copy:usize,control:&mut dyn crate::OriginalRetirementControl)->Result<RetirementDemand,ValueError>{match retirement_birth(Some(ticket.as_ref()),copy,control)?{Some(birth)=>Ok(birth.demand),None=>factory_ticket_demands(ticket,copy)}}
/// 🧷️ Quotes the same selected turn against the exact original decoder loan.
pub fn factory_ticket_original_demands<T:ErasedSnapshotRetirement+?Sized>(ticket:&Box<T>,copy:usize,original:&mut crate::OriginalRetirementCloseLoan<'_>)->Result<RetirementDemand,ValueError>{match retirement_birth(Some(ticket.as_ref()),copy,original)?{Some(birth)=>Ok(birth.demand),None=>factory_ticket_demands(ticket,copy)}}
fn admit<T:ErasedSnapshotRetirement+?Sized>(ticket:&T,grant:RetainedCloneGrant,poll:OriginalRetirementPoll)->OriginalRetirementPoll{poll.and_then(|step|crate::retained_clone::admit_retained_clone_close(grant,step,ticket.terminal_is_empty(),"factory ticket payload").map_err(Into::into)).map(|step|RetainedCloneStep::Progress(step.progress()))}

pub fn close_factory_ticket<T:ErasedSnapshotRetirement+?Sized>(slot:&mut Option<Box<T>>,body:usize,effect:&mut crate::RetainedAllocationEffect<'_>)->OriginalRetirementPoll{
 let grant=effect.grant();let empty=RetainedCloneProgress::default();let Some(ticket)=slot.as_mut()else{return RetainedWorkPoll::Complete(RetainedWorkOutcome{result:Ok(RetainedCloneStep::Complete(empty)),work:0})};
 let work=if ticket.terminal_is_empty(){1}else{match ticket.next_computational_work(body){Ok(work)=>work,Err(error)=>return RetainedWorkPoll::Complete(RetainedWorkOutcome{result:Err(error.into()),work:0})}};
 if grant.maximum_items==0||effect.maximum_work_units()<work{return RetainedWorkPoll::Complete(RetainedWorkOutcome{result:Ok(RetainedCloneStep::Progress(empty)),work:0})}
 if ticket.terminal_is_empty(){if grant.maximum_depth==0{return RetainedWorkPoll::Complete(RetainedWorkOutcome{result:Err(ValueError::literal(ValueRefusalKind::DepthLimit,"terminal ticket requires admitted depth").into()),work:0})}let bytes=size_of_val(ticket.as_ref());if grant.maximum_release_bytes<bytes||grant.maximum_copy_bytes<size_of::<Option<Box<T>>>(){return RetainedWorkPoll::Complete(RetainedWorkOutcome{result:Ok(RetainedCloneStep::Progress(empty)),work:0})}let work=match crate::record_retirement_work(effect,1){Ok(work)=>work,Err((error,work))=>return RetainedWorkPoll::Complete(RetainedWorkOutcome{result:Err(error.into()),work})};drop(slot.take());return RetainedWorkPoll::Complete(RetainedWorkOutcome{result:Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,copied_bytes:size_of::<Option<Box<T>>>(),released_bytes:bytes,..empty})),work})}
 let turn=close_owner(ticket.as_mut(),body,effect);admit(ticket.as_ref(),grant,turn)
}
pub fn close_factory_ticket_controlled<T:ErasedSnapshotRetirement+?Sized>(slot:&mut Option<Box<T>>,body:usize,effect:&mut crate::RetainedAllocationEffect<'_>,control:&mut dyn crate::OriginalRetirementControl)->OriginalRetirementPoll{
 let grant=effect.grant();let Some(ticket)=slot.as_ref()else{return RetainedWorkPoll::Complete(RetainedWorkOutcome{result:Ok(RetainedCloneStep::Complete(Default::default())),work:0})};
 match retirement_birth(Some(ticket.as_ref()),body,control){Ok(None)=>{},Ok(Some(_))=>return skipped_birth(),Err(error)=>return birth_refusal(error)}
 let demand=match factory_ticket_demands(ticket,body){Ok(d)=>d,Err(error)=>return RetainedWorkPoll::Complete(RetainedWorkOutcome{result:Err(error.into()),work:0})};if !crate::retained_clone::retained_clone_grant_admits(grant,demand){return RetainedWorkPoll::Complete(RetainedWorkOutcome{result:Ok(RetainedCloneStep::Progress(Default::default())),work:0})}
 if !ticket.terminal_is_empty()&&ticket.requires_original_control(){let ticket=slot.as_mut().unwrap();let turn=close_owner_controlled(ticket.as_mut(),body,effect,control);return admit(ticket.as_ref(),grant,turn)}
 close_factory_ticket(slot,body,effect)
}
/// 🧷️ Native erased children keep their same original cleanup loan and physical effect.
pub fn close_factory_ticket_original<T:ErasedSnapshotRetirement+?Sized>(slot:&mut Option<Box<T>>,original:&mut crate::OriginalRetirementCloseLoan<'_>,body:usize,effect:&mut crate::RetainedAllocationEffect<'_>)->OriginalRetirementPoll{
 let grant=effect.grant();let Some(ticket)=slot.as_ref()else{return RetainedWorkPoll::Complete(RetainedWorkOutcome{result:Ok(RetainedCloneStep::Complete(Default::default())),work:0})};
 match retirement_birth(Some(ticket.as_ref()),body,original){Ok(None)=>{},Ok(Some(_))=>return skipped_birth(),Err(error)=>return birth_refusal(error)}
 let demand=match factory_ticket_demands(ticket,body){Ok(d)=>d,Err(error)=>return RetainedWorkPoll::Complete(RetainedWorkOutcome{result:Err(error.into()),work:0})};if !crate::retained_clone::retained_clone_grant_admits(grant,demand){return RetainedWorkPoll::Complete(RetainedWorkOutcome{result:Ok(RetainedCloneStep::Progress(Default::default())),work:0})}
 if !ticket.terminal_is_empty()&&ticket.requires_original_control(){let ticket=slot.as_mut().unwrap();let turn=close_owner_original(ticket.as_mut(),original,body,effect);return admit(ticket.as_ref(),grant,turn)}
 close_factory_ticket(slot,body,effect)
}
#[path="📮️resident/🦀️.rs"]
mod resident;
pub use resident::{close_owner,close_owner_original,close_owner_controlled};
#[path="🎯️selection/🦀️.rs"]
mod selection;
pub use selection::{RetirementSelection,select_retirement_cursor,select_factory_ticket};

/// 👶️ Selects an original birth before a general receiving effect.
#[derive(Clone,Copy,Debug,PartialEq,Eq)]
pub enum RetirementBirthKind{FailureStorage,Child}
/// 📏️ A birth quote carries no original action authority.
#[derive(Clone,Copy,Debug)]
pub struct RetirementBirth{pub kind:RetirementBirthKind,pub demand:RetirementDemand}
/// 🎯️ Resumes existing storage custody before selecting child birth or a fresh spare.
pub fn retirement_birth<T:ErasedSnapshotRetirement+?Sized>(owner:Option<&T>,body:usize,control:&mut dyn crate::OriginalRetirementControl)->Result<Option<RetirementBirth>,ValueError>{
 let birth=control.failure_storage_birth();
 if birth_kind(!birth.terminal_is_empty(),false,true,false,true).is_some(){return Ok(Some(RetirementBirth{kind:RetirementBirthKind::FailureStorage,demand:birth.demand()}))}
 let Some(owner)=owner else{return Ok(None)};
 match birth_kind(false,true,owner.terminal_is_empty(),owner.requires_original_birth(),!control.failure_storage_pool().is_empty()){Some(RetirementBirthKind::Child)=>Ok(Some(RetirementBirth{kind:RetirementBirthKind::Child,demand:owner.next_demand(body)?})),Some(RetirementBirthKind::FailureStorage)=>Ok(Some(RetirementBirth{kind:RetirementBirthKind::FailureStorage,demand:crate::original_failure_storage_birth_demand()})),None=>Ok(None)}
}
/// 🧷️ Drives only the selected birth through its original loan before a general effect.
pub fn close_retirement_birth<T:ErasedSnapshotRetirement+?Sized>(owner:Option<&mut T>,body:usize,original:&mut crate::OriginalRetirementCloseLoan<'_>,grant:RetainedCloneGrant)->OriginalRetirementPoll{
 let selected=match retirement_birth(owner.as_deref(),body,original){Ok(selected)=>selected,Err(error)=>return birth_refusal(error)};
 match selected{
 Some(RetirementBirth{kind:RetirementBirthKind::FailureStorage,..})=>crate::refill_original_failure_storage(original,grant),
 Some(RetirementBirth{kind:RetirementBirthKind::Child,..})=>owner.expect("selected original child birth").close_original_birth_step(original,grant),
 None=>RetainedWorkPoll::Complete(RetainedWorkOutcome{result:Ok(RetainedCloneStep::Progress(Default::default())),work:0})
 }
}
fn birth_refusal(error:ValueError)->OriginalRetirementPoll{RetainedWorkPoll::Complete(RetainedWorkOutcome{result:Err(error.into()),work:0})}
fn skipped_birth()->OriginalRetirementPoll{birth_refusal(ValueError::literal(ValueRefusalKind::InvariantViolated,"selected original birth must run before the general retirement effect"))}

fn birth_kind(pending:bool,present:bool,terminal:bool,child:bool,spare:bool)->Option<RetirementBirthKind>{if pending{Some(RetirementBirthKind::FailureStorage)}else if !present||terminal{None}else if child{Some(RetirementBirthKind::Child)}else if !spare{Some(RetirementBirthKind::FailureStorage)}else{None}}
#[cfg(test)]
mod birth_tests{
 #[test]
 fn original_birth_priority_matches_portable_cases(){
  let cases:serde_json::Value=serde_json::from_str(include_str!("🧪️tests/cases.json")).unwrap();
  for row in cases.as_array().unwrap(){let flag=|key:&str|row[key].as_bool().unwrap();let selected=super::birth_kind(flag("pending"),flag("present"),flag("terminal"),flag("child"),flag("spare")).map(|kind|match kind{super::RetirementBirthKind::FailureStorage=>"failureStorage",super::RetirementBirthKind::Child=>"child"});assert_eq!(selected,row["expected"].as_str());}
 }
}

#[path="🔒️close/🦀️.rs"]mod borrowed_close;
pub use borrowed_close::FactoryTicketClose;
