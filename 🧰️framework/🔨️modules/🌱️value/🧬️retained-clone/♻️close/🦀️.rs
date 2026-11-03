//! ♻️ Typed clone cleanup owns original tickets and source binding closure.
use super::*;
use std::mem::{size_of,ManuallyDrop};

mod empty_owner{
 pub trait Sealed:Sized{fn take_empty(&mut self)->Self;fn already_empty(&self)->bool;}
 impl<T> Sealed for Vec<T>{fn take_empty(&mut self)->Self{std::mem::take(self)}fn already_empty(&self)->bool{self.is_empty()&&(size_of::<T>()==0||self.capacity()==0)}}
 impl<T,const N:usize> Sealed for semio_framework_value_retirement::list::PagedList<T,N>{fn take_empty(&mut self)->Self{std::mem::take(self)}fn already_empty(&self)->bool{self.terminal_is_empty()}}
}
/// 🥡️ Only defining zero-allocation empty Vec and paged owners can transfer without an arbitrary Default callback.
/// ```compile_fail,E0277
/// use semio_framework_value_clone_source::{RetainedCloneClose,RetainedCloneGrant};
/// use semio_framework_value_clone_source::retirement::{RetireOwned,RetirementCursor};
/// struct Foreign(String);
/// impl Default for Foreign{fn default()->Self{Self("unfunded".to_owned())}}
/// impl RetireOwned for Foreign{
///  fn retirement(self)->Box<dyn RetirementCursor>{self.0.retirement()}
///  fn retirement_birth_bytes(&self)->Option<usize>{self.0.retirement_birth_bytes()}
///  fn controlled_retirement_supported()->bool{true}
/// }
/// let mut value=Foreign(String::new());
/// RetainedCloneClose::default().begin_empty_granted(&mut value,RetainedCloneGrant::default());
/// ```
pub trait RetainedEmptyOwner:RetireOwned+empty_owner::Sealed{}
impl<T:RetireOwned> RetainedEmptyOwner for Vec<T>{}
impl<T:RetireOwned,const N:usize> RetainedEmptyOwner for semio_framework_value_retirement::list::PagedList<T,N>{}

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
    pub fn next_owner_demands<T:RetireOwned>(&self,present:bool,binding:&Option<RetainedCloneBinding>,body:usize)->Result<RetirementDemand,ValueError>{if let Some(owner)=self.retirement.as_ref(){return factory_ticket_demands(owner,body);}if present{if !T::controlled_retirement_supported(){return Err(ValueError::literal(ValueRefusalKind::UnsupportedOwner,"typed clone owner lacks controlled close authority"));}return Ok(RetirementDemand{capacity_bytes:owned_retirement_birth_bytes::<T>(),depth:1,..Default::default()});}RetainedCloneBinding::close_demand(binding,body)}
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
    pub fn begin_granted<T:RetireOwned>(&mut self,value:&mut Option<T>,grant:RetainedCloneGrant)->RetainedWorkOutcome<Option<RetainedCloneStep>>{
        if !self.is_empty()||value.is_none(){return RetainedWorkOutcome{result:Ok(None),work:0}}
        let bytes=owned_retirement_birth_bytes::<T>();if grant.maximum_items==0||grant.maximum_depth==0||bytes>grant.maximum_capacity_bytes{return RetainedWorkOutcome{result:Ok(Some(RetainedCloneStep::Progress(Default::default()))),work:0}}
        let turn=admit_owned_retirement(match value.take(){Some(v)=>v,None=>return RetainedWorkOutcome{result:Err(ValueError::invariant("value absent").into()),work:0}},grant);let result=match turn.result{Ok((owner,progress))=>{*self.retirement=Some(owner);Ok(Some(RetainedCloneStep::Progress(progress)))},Err((error,owner))=>{*value=Some(owner);Err(error)}};RetainedWorkOutcome{result,work:turn.work}
    }
    pub fn begin_empty_granted<T:RetainedEmptyOwner>(&mut self,value:&mut T,grant:RetainedCloneGrant)->RetainedWorkOutcome<Option<RetainedCloneStep>>{
        if !self.is_empty()||empty_owner::Sealed::already_empty(value){return RetainedWorkOutcome{result:Ok(None),work:0}}
        let bytes=owned_retirement_birth_bytes::<T>();if grant.maximum_items==0||grant.maximum_depth==0||bytes>grant.maximum_capacity_bytes{return RetainedWorkOutcome{result:Ok(Some(RetainedCloneStep::Progress(Default::default()))),work:0}}
        let mut owner=Some(empty_owner::Sealed::take_empty(value));let turn=self.begin_granted(&mut owner,grant);if let Some(owner)=owner{*value=owner;}turn
    }
    pub fn step_granted(&mut self,grant:RetainedCloneGrant)->RetainedWorkOutcome{
        if let Some(owner)=self.retirement.as_ref(){let demand=match factory_ticket_demands(owner,grant.maximum_copy_bytes){Ok(d)=>d,Err(error)=>return RetainedWorkOutcome{result:Err(error),work:0}};if grant.maximum_items>0&&grant.maximum_depth<demand.depth{return RetainedWorkOutcome{result:Err(ValueError::literal(ValueRefusalKind::DepthLimit,"clone retirement exceeds admitted depth")),work:0}}if !retained_clone_grant_admits(grant,demand){return RetainedWorkOutcome{result:Ok(RetainedCloneStep::Progress(Default::default())),work:0}}}
        let turn=close_factory_ticket(&mut self.retirement,grant);let result=turn.result.and_then(|step|{let progress=step.progress();if !progress.fits(grant){return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"clone retirement exceeded admitted full grant").with_retained_progress(progress))}Ok(step)});RetainedWorkOutcome{result,work:turn.work}
    }
    pub fn is_empty(&self)->bool {self.retirement.is_none()}
}

impl Drop for RetainedCloneClose {fn drop(&mut self){assert!(std::thread::panicking()||self.is_empty(),"clone retirement abandoned original ownership");if self.is_empty(){unsafe{ManuallyDrop::drop(&mut self.retirement);}}}}

pub fn close_retained_binding(binding: &mut Option<RetainedCloneBinding>, grant: RetainedCloneGrant) -> RetainedWorkOutcome {
    RetainedCloneBinding::close_one(binding, grant)
}

#[cfg(test)]
#[path="🧪️tests/🦀️.rs"]
mod tests;
