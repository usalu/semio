//! 🔒️ A real session lends its native keyed latch arena to the same pure preparation and publication kernels.
use protocol::causal::transition::HistoryFoldIndex;
use semio_framework_value::{RetirementDemand,RetainedCloneGrant,RetainedCloneProgress,RetainedCloneStep,RetainedWorkOutcome,ValueError,ValueRefusalKind,retained_clone::retained_clone_grant_admits};
use super::window_key::FlowEvalWindowKey;
use std::mem::size_of;
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct FlowEvalWindowTickLatch {
    pub armed: bool,
    pub in_flight: u32,
    pub owed: bool,
    pub unfinished: bool,
}

semio_framework_value::artifact_retire_leaf!(FlowEvalWindowTickLatch);
pub type FlowWindowLatches=HistoryFoldIndex<u64,FlowEvalWindowTickLatch>;
pub fn flow_window_latch_demands(latches:&FlowWindowLatches,key:FlowEvalWindowKey,copy:usize)->Result<RetirementDemand,ValueError>{let key=key.fingerprint();if latches.contains_key(&key){return Ok(Default::default())}Ok(RetirementDemand{copy_bytes:latches.next_insert_copy_byte_demand(&key)?,capacity_bytes:latches.next_insert_capacity_byte_demand(&key,copy)?,release_bytes:latches.next_insert_release_byte_demand(&key)?,depth:latches.next_insert_depth_demand(&key)?})}
pub fn prepare_flow_window_latch(latches:&mut FlowWindowLatches,key:FlowEvalWindowKey,g:RetainedCloneGrant)->RetainedWorkOutcome{let mut work=0;let result=(||{let key=key.fingerprint();if latches.contains_key(&key){return Ok(RetainedCloneStep::Complete(Default::default()))}if g.maximum_items==0{return Ok(RetainedCloneStep::Progress(Default::default()))}if latches.next_insert_capacity_byte_demand(&key,g.maximum_copy_bytes)?>0{let turn=latches.reserve_insert_original(&key,g);work=turn.work;return turn.result.map(RetainedCloneStep::Progress).map_err(|(error,p)|error.with_retained_progress(p));}match latches.insert_reserved(key,FlowEvalWindowTickLatch::default(),g){Ok((None,p))=>{work=1;Ok(RetainedCloneStep::Complete(p))},Ok((Some(_),_))=>unreachable!("original latch preparation does not displace a live row"),Err((error,_,_))=>Err(error)}})();RetainedWorkOutcome{result,work}}
pub fn flow_window_publication_demands()->RetirementDemand{RetirementDemand{copy_bytes:2*size_of::<FlowEvalWindowTickLatch>()+2*size_of::<bool>()+size_of::<u64>(),depth:1,..Default::default()}}
pub fn publish_flow_window_evaluation(latches:&mut FlowWindowLatches,preview_cancelled:&mut bool,key:FlowEvalWindowKey,count:usize,unfinished:bool,g:RetainedCloneGrant)->RetainedWorkOutcome{let d=flow_window_publication_demands();if !retained_clone_grant_admits(g,d){return RetainedWorkOutcome{result:Ok(RetainedCloneStep::Progress(Default::default())),work:0}}let result=(||{let latch=latches.get_mut(&key.fingerprint()).ok_or_else(||ValueError::literal(ValueRefusalKind::OwnershipLimit,"original evaluation publication requires its admitted window latch"))?;latch.armed=false;latch.unfinished=unfinished||count!=0;latch.in_flight=latch.in_flight.saturating_add(u32::try_from(count).unwrap_or(u32::MAX));*preview_cancelled=false;Ok(RetainedCloneStep::Complete(RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,..Default::default()}))})();RetainedWorkOutcome{result,work:1}}

pub fn settle_flow_window_reply(latches:&mut FlowWindowLatches,key:FlowEvalWindowKey,give_up:bool,g:RetainedCloneGrant)->RetainedWorkOutcome<(bool,RetainedCloneProgress)>{
 let d=flow_window_publication_demands();if !retained_clone_grant_admits(g,d){return RetainedWorkOutcome{result:Ok((false,Default::default())),work:0}}
 let result=(||{let latch=latches.get_mut(&key.fingerprint()).ok_or_else(||ValueError::literal(ValueRefusalKind::OwnershipLimit,"original reply settlement requires its existing window latch"))?;latch.in_flight=latch.in_flight.saturating_sub(1);if give_up{latch.unfinished=false;latch.owed=false;}let armed=latch.in_flight==0&&!latch.armed&&(latch.owed||latch.unfinished);if armed{latch.armed=true;latch.owed=false;}Ok((armed,RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,..Default::default()}))})();RetainedWorkOutcome{result,work:1}
}
