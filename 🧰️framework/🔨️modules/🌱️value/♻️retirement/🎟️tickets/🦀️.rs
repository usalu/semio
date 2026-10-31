//! 🎟️ Original erased child closure preserves its own selected result, physical receipt and actual CPU.
use crate::{ErasedSnapshotRetirement,RetirementDemand,RetainedWorkOutcome,ValueError,ValueRefusalKind,RetainedCloneGrant,RetainedCloneProgress,RetainedCloneStep};
pub fn factory_ticket_demands<T:ErasedSnapshotRetirement+?Sized>(ticket:&Box<T>,copy:usize)->Result<RetirementDemand,ValueError>{
 if ticket.terminal_is_empty(){return Ok(RetirementDemand{release_bytes:size_of_val(ticket.as_ref()),depth:1,..Default::default()})}
 ticket.next_demand(copy)
}
pub fn close_factory_ticket<T:ErasedSnapshotRetirement+?Sized>(slot:&mut Option<Box<T>>,grant:RetainedCloneGrant)->RetainedWorkOutcome{close_factory_ticket_inner(slot,grant,None)}
pub fn close_factory_ticket_controlled<T:ErasedSnapshotRetirement+?Sized>(slot:&mut Option<Box<T>>,grant:RetainedCloneGrant,control:&mut dyn crate::OriginalRetirementControl)->RetainedWorkOutcome{close_factory_ticket_inner(slot,grant,Some(control))}
fn close_factory_ticket_inner<T:ErasedSnapshotRetirement+?Sized>(slot:&mut Option<Box<T>>,grant:RetainedCloneGrant,control:Option<&mut dyn crate::OriginalRetirementControl>)->RetainedWorkOutcome{
 let empty=RetainedCloneProgress::default();let Some(ticket)=slot.as_mut()else{return RetainedWorkOutcome{result:Ok(RetainedCloneStep::Complete(empty)),work:0}};
 if grant.maximum_items==0{return RetainedWorkOutcome{result:Ok(RetainedCloneStep::Progress(empty)),work:0}}
 if ticket.terminal_is_empty(){if grant.maximum_depth==0{return RetainedWorkOutcome{result:Err(ValueError::literal(ValueRefusalKind::DepthLimit,"terminal ticket requires admitted depth")),work:0}}let bytes=size_of_val(ticket.as_ref());if grant.maximum_release_bytes<bytes{return RetainedWorkOutcome{result:Ok(RetainedCloneStep::Progress(empty)),work:0}}drop(slot.take());return RetainedWorkOutcome{result:Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,released_bytes:bytes,..empty})),work:1}}
 let turn=match control{Some(control)=>{let demand=match ticket.next_demand(grant.maximum_copy_bytes){Ok(d)=>d,Err(error)=>return RetainedWorkOutcome{result:Err(error),work:0}};if !crate::retained_clone::retained_clone_grant_admits(grant,demand){return RetainedWorkOutcome{result:Ok(RetainedCloneStep::Progress(empty)),work:0}}if !ticket.requires_original_control(){if let Err(error)=control.charge(demand.capacity_bytes){return RetainedWorkOutcome{result:Err(error),work:0}}}ticket.close_step_controlled(grant,control)},None=>ticket.close_step(grant)};
 let result=turn.result.and_then(|step|crate::retained_clone::admit_retained_clone_close(grant,step,ticket.terminal_is_empty(),"factory ticket payload")).map(|step|RetainedCloneStep::Progress(step.progress()));RetainedWorkOutcome{result,work:turn.work}
}
/// 🧷️ Native erased children retain their genuine original cleanup loan through terminal frame release.
pub fn close_factory_ticket_original<T:ErasedSnapshotRetirement+?Sized>(slot:&mut Option<Box<T>>,original:&mut semio_framework_value_control_contract::OriginalRetirementCloseLoan<'_>,grant:RetainedCloneGrant)->RetainedWorkOutcome{
 let Some(ticket)=slot.as_mut()else{return RetainedWorkOutcome{result:Ok(RetainedCloneStep::Complete(Default::default())),work:0}};if ticket.terminal_is_empty(){return close_factory_ticket(slot,grant)}let turn=ticket.close_original_step(original,grant);let result=turn.result.and_then(|step|crate::retained_clone::admit_retained_clone_close(grant,step,ticket.terminal_is_empty(),"original factory ticket payload")).map(|step|RetainedCloneStep::Progress(step.progress()));RetainedWorkOutcome{result,work:turn.work}
}
#[path="🎯️selection/🦀️.rs"]
mod selection;
pub use selection::{RetirementSelection,select_retirement_cursor,select_factory_ticket};
