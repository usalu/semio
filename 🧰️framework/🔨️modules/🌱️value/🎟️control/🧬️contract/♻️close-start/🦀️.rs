//! 🚦️ One resident close flag has one actual physical write and no repeated work.
use super::*;
/// 📏️ Quotes the selected original in-place flag write.
pub fn retained_close_start_demand(started:bool)->RetirementDemand{RetirementDemand{copy_bytes:usize::from(!started)*std::mem::size_of::<bool>(),depth:usize::from(!started),..Default::default()}}
/// ⛽️ Quotes one actual original flag transition.
pub fn retained_close_start_work_demand(started:bool)->u64{u64::from(!started)}
/// 🚥️ Changes the resident flag only after its complete physical grant.
pub fn start_retained_close(started:&mut bool,effect:&mut RetainedAllocationEffect<'_>)->RetainedWorkOutcome<RetainedCloneProgress>{let demand=retained_close_start_demand(*started);if *started||!retained_clone_grant_admits(effect.grant(),demand){return RetainedWorkOutcome{result:Ok(Default::default()),work:0}}let remaining=match effect.remaining_work_units(){Ok(remaining)=>remaining,Err(error)=>return RetainedWorkOutcome{result:Err(error),work:0}};if remaining==0{return RetainedWorkOutcome{result:Ok(Default::default()),work:0}}let boundary=effect.performed_boundary();let performed=effect.performed(1);let work=effect.performed_since(boundary).expect("close flag retains its original performed receipt");match performed{Err(error)=>return RetainedWorkOutcome{result:Err(error),work},Ok(false)=>return RetainedWorkOutcome{result:Err(ValueError::literal(ValueRefusalKind::WorkLimit,"close flag exceeds its original Work ceiling")),work},Ok(true)=>{}}*started=true;RetainedWorkOutcome{result:Ok(RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,..Default::default()}),work}}


#[cfg(test)]
#[path="🧪️tests/🦀️.rs"]
pub(crate) mod tests;
