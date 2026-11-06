//! ♻️ Typed clone cleanup owns original tickets and source binding closure.
use super::*;
use std::mem::{size_of,ManuallyDrop};

pub use semio_framework_value_retirement::retirement::controlled::RetainedEmptyOwner;

#[derive(Default)]
pub struct RetainedCloneClose {pub(super) retirement:ManuallyDrop<Option<Box<dyn ErasedSnapshotRetirement>>>}

/// 🎯️ The retained clone owner uses the same exclusive original ticket selection as typed cursors.
pub use crate::RetirementSelection as RetainedCloseSelection;

impl RetainedCloneClose {
    /// 🪑️ Seats one already admitted original retirement so custody laws can quote and fund its close without a typed owner.
    #[doc(hidden)] pub fn from_admitted_original(retirement:Box<dyn ErasedSnapshotRetirement>)->Self{Self{retirement:ManuallyDrop::new(Some(retirement))}}
    /// 📏️ Selects all currencies from the same actual body request forwarded to the retained child.
    pub fn demands(&self,body:usize)->Result<RetirementDemand,ValueError>{self.retirement.as_ref().map_or(Ok(Default::default()),|owner|factory_ticket_demands(owner,body))}
    /// 🧷️ Loans the actual owner exclusively so the selected body cannot change before its grant is checked.
    pub fn select(&mut self,body:usize)->Result<RetainedCloseSelection<'_>,ValueError>{select_factory_ticket(&mut self.retirement,body)}
    /// ⛽️ Quotes the selected original ticket, typed admission or binding close.
    pub fn next_owner_work<T:RetireOwned>(&self,present:bool,binding:&Option<RetainedCloneBinding>,body:usize)->Result<u64,ValueError>{if let Some(owner)=self.retirement.as_ref(){return if owner.terminal_is_empty(){Ok(1)}else{owner.next_computational_work(body)}}if present{if !T::controlled_retirement_supported(){return Err(ValueError::literal(ValueRefusalKind::UnsupportedOwner,"typed clone owner lacks controlled close authority"))}return Ok(1)}RetainedCloneBinding::close_work_demand(binding,body)}
    pub fn next_owner_demands<T:RetireOwned>(&self,present:bool,binding:&Option<RetainedCloneBinding>,body:usize)->Result<RetirementDemand,ValueError>{if let Some(owner)=self.retirement.as_ref(){return factory_ticket_demands(owner,body);}if present{if !T::controlled_retirement_supported(){return Err(ValueError::literal(ValueRefusalKind::UnsupportedOwner,"typed clone owner lacks controlled close authority"));}return Ok(RetirementDemand{copy_bytes:size_of::<Option<T>>()+owned_retirement_birth_bytes::<T>(),capacity_bytes:owned_retirement_birth_bytes::<T>(),depth:1,..Default::default()});}RetainedCloneBinding::close_demand(binding,body)}
    pub fn next_owner_depth_with_binding(&self,present:bool,binding:&Option<RetainedCloneBinding>)->Result<usize,ValueError>{if !self.is_empty(){self.next_depth_demand()}else if present{Ok(1)}else{RetainedCloneBinding::depth_demand(binding)}}
    pub fn next_copy_with_binding(&self,binding:&Option<RetainedCloneBinding>)->Result<usize,ValueError>{if self.is_empty(){RetainedCloneBinding::copy_demand(binding)}else{self.next_copy_byte_demand()}}
    pub fn next_release_with_binding(&self,binding:&Option<RetainedCloneBinding>)->Result<usize,ValueError>{if self.is_empty(){RetainedCloneBinding::release_demand(binding)}else{self.next_release_byte_demand()}}
    pub fn next_owner_capacity_with_binding<T:RetireOwned>(&self,present:bool,body:usize,binding:&Option<RetainedCloneBinding>)->Result<usize,ValueError>{if self.is_empty()&&!present{RetainedCloneBinding::capacity_demand(binding,body)}else{self.next_owner_capacity_byte_demand::<T>(present,body)}}
    pub fn next_copy_byte_demand(&self)->Result<usize,ValueError> {self.retirement.as_ref().map_or(Ok(0),|owner|Ok(factory_ticket_demands(owner,0)?.copy_bytes))}
    pub fn next_capacity_byte_demand(&self,body:usize)->Result<usize,ValueError> {self.retirement.as_ref().map_or(Ok(0),|owner|Ok(factory_ticket_demands(owner,body)?.capacity_bytes))}
    pub fn next_release_byte_demand(&self)->Result<usize,ValueError> {self.retirement.as_ref().map_or(Ok(0),|owner|Ok(factory_ticket_demands(owner,0)?.release_bytes))}
    pub fn next_depth_demand(&self)->Result<usize,ValueError> {self.retirement.as_ref().map_or(Ok(0),|owner|Ok(factory_ticket_demands(owner,0)?.depth))}
    pub fn next_owner_capacity_byte_demand<T:RetireOwned>(&self,present:bool,body:usize)->Result<usize,ValueError> {
        if !self.is_empty(){return self.next_capacity_byte_demand(body);}
        if present&&!T::controlled_retirement_supported(){return Err(ValueError::literal(ValueRefusalKind::UnsupportedOwner,"typed owner has no controlled close birth authority"));}
        Ok(if present{owned_retirement_birth_bytes::<T>()}else{0})
    }
    pub fn begin_granted<T:RetireOwned>(&mut self,value:&mut Option<T>,effect:&mut crate::RetainedAllocationEffect<'_>)->RetainedWorkOutcome<Option<RetainedCloneStep>>{
        if !self.is_empty()||value.is_none(){return RetainedWorkOutcome{result:Ok(None),work:0}}
        let turn=admit_owned_retirement(value,effect);let result=match turn.result{Ok(Some((owner,progress)))=>{*self.retirement=Some(owner);Ok(Some(RetainedCloneStep::Progress(progress)))},Ok(None)=>Ok(Some(RetainedCloneStep::Progress(Default::default()))),Err(error)=>Err(error.into())};RetainedWorkOutcome{result,work:turn.work}
    }
    pub fn begin_empty_granted<T:RetainedEmptyOwner>(&mut self,value:&mut T,effect:&mut crate::RetainedAllocationEffect<'_>)->RetainedWorkOutcome<Option<RetainedCloneStep>>{if !self.is_empty()||semio_framework_value_retirement::retirement::controlled::empty_retirement_is_terminal(value){return RetainedWorkOutcome{result:Ok(None),work:0}}let turn=semio_framework_value_retirement::retirement::controlled::admit_empty_controlled_retirement(value,effect);let result=match turn.result{Ok(Some((owner,progress)))=>{*self.retirement=Some(owner);Ok(Some(RetainedCloneStep::Progress(progress)))},Ok(None)=>Ok(Some(RetainedCloneStep::Progress(Default::default()))),Err(error)=>Err(error.into())};RetainedWorkOutcome{result,work:turn.work}}
    pub fn step_granted(&mut self,effect:&mut crate::RetainedAllocationEffect<'_>)->crate::OriginalRetirementPoll{let grant=effect.grant();
        if let Some(owner)=self.retirement.as_ref(){let demand=match factory_ticket_demands(owner,grant.maximum_copy_bytes){Ok(d)=>d,Err(error)=>return crate::RetainedWorkPoll::Complete(RetainedWorkOutcome{result:Err(error.into()),work:0})};if grant.maximum_items>0&&grant.maximum_depth<demand.depth{return crate::RetainedWorkPoll::Complete(RetainedWorkOutcome{result:Err(ValueError::literal(ValueRefusalKind::DepthLimit,"clone retirement exceeds admitted depth").into()),work:0})}if !retained_clone_grant_admits(grant,demand){return crate::RetainedWorkPoll::Complete(RetainedWorkOutcome{result:Ok(RetainedCloneStep::Progress(Default::default())),work:0})}}
        close_factory_ticket(&mut self.retirement,effect).and_then(|step|{let progress=step.progress();if !progress.fits(grant){return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"clone retirement exceeded admitted full grant").with_retained_progress(progress).into())}Ok(step)})
    }
    /// 👶️ Borrows the actual child birth selector without opening another effect.
    pub fn requires_original_birth(&self)->bool{self.retirement.as_ref().is_some_and(|owner|owner.requires_original_birth())}
    pub fn step_original_birth(&mut self,original:&mut semio_framework_value_control_contract::OriginalRetirementCloseLoan<'_>,grant:RetainedCloneGrant)->crate::OriginalRetirementPoll{match self.retirement.as_mut(){Some(owner)=>owner.close_original_birth_step(original,grant),None=>crate::RetainedWorkPoll::Complete(crate::RetainedWorkOutcome{result:Ok(RetainedCloneStep::Complete(Default::default())),work:0})}}
    /// 🪪️ Borrows the actual selected child original-control requirement.
    /// 🧭️ Relays the actual controller and same producer effect into the resident original owner.
    pub fn step_controlled(&mut self,effect:&mut crate::RetainedAllocationEffect<'_>,control:&mut dyn semio_framework_value_retirement::OriginalRetirementControl)->crate::OriginalRetirementPoll{semio_framework_value_retirement::close_factory_ticket_controlled(&mut self.retirement,effect,control)}
    pub fn requires_original_control(&self)->bool{self.retirement.as_ref().is_some_and(|owner|!owner.terminal_is_empty()&&owner.requires_original_control())}
    /// 🧵️ Closes through the same original decoder loan and defining allocation effect.
    pub fn step_original(&mut self,original:&mut semio_framework_value_control_contract::OriginalRetirementCloseLoan<'_>,effect:&mut crate::RetainedAllocationEffect<'_>)->crate::OriginalRetirementPoll{semio_framework_value_retirement::close_factory_ticket_original(&mut self.retirement,original,effect)}
    pub fn is_empty(&self)->bool {self.retirement.is_none()}
}

impl Drop for RetainedCloneClose {fn drop(&mut self){assert!(std::thread::panicking()||self.is_empty(),"clone retirement abandoned original ownership");if self.is_empty(){unsafe{ManuallyDrop::drop(&mut self.retirement);}}}}

pub fn close_retained_binding(binding:&mut Option<RetainedCloneBinding>,effect:&mut crate::RetainedAllocationEffect<'_>)->crate::OriginalRetirementPoll{
    RetainedCloneBinding::close_one(binding,effect)
}

#[cfg(test)]
#[path="🧪️tests/🦀️.rs"]
mod tests;
