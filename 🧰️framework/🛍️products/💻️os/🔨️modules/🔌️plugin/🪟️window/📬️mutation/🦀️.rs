//! 📬️ Original window mutation payloads carry their captured typed retirement issuer.
use semio_framework_value::{FactoryBoxedValue,retirement::{RetireOwned,RetirementCursor,boxed_owner_birth_bytes}};
use std::any::Any;
pub(crate) trait ErasedWindowMutationValue:Send{
 fn as_any(&self)->&dyn Any;
 fn into_any(self:Box<Self>)->Box<dyn Any+Send>;
 fn clone_value(&self)->Box<dyn ErasedWindowMutationValue>;
 fn retire_payload(self:Box<Self>)->Box<dyn RetirementCursor>;
 fn payload_birth_bytes(&self)->usize;
}
impl<T:Clone+Send+'static> ErasedWindowMutationValue for FactoryBoxedValue<T>{
 fn as_any(&self)->&dyn Any{self.original.as_ref()}
 fn into_any(self:Box<Self>)->Box<dyn Any+Send>{self}
 fn clone_value(&self)->Box<dyn ErasedWindowMutationValue>{Box::new(Self{original:self.original.clone(),factory:self.factory.clone()})}
 fn retire_payload(self:Box<Self>)->Box<dyn RetirementCursor>{self.retirement()}
 fn payload_birth_bytes(&self)->usize{boxed_owner_birth_bytes::<Self>()}
}
impl RetireOwned for Box<dyn ErasedWindowMutationValue>{
 fn retirement(self)->Box<dyn RetirementCursor>{self.retire_payload()}
 fn retirement_birth_bytes(&self)->Option<usize>{Some(self.payload_birth_bytes())}
 fn controlled_retirement_supported()->bool{true}
}

#[cfg(test)]
#[path="🧪️tests/🦀️.rs"]
mod tests;
#[cfg(test)]
pub(crate) use tests::{close_original as close_test_original,close_with as close_test_original_with_pump};

pub(crate) fn address_terminal(address:&String,owner:&Option<semio_framework_value::retirement::controlled::ControlledRetirement<String>>)->bool{address.capacity()==0&&owner.is_none()}
pub(crate) fn address_demands(address:&String,owner:&Option<semio_framework_value::retirement::controlled::ControlledRetirement<String>>,body:usize)->Result<semio_framework_value::RetirementDemand,semio_framework_value::ValueError>{
 use semio_framework_value::{RetirementDemand,retirement::controlled::ControlledRetirement};if let Some(owner)=owner.as_ref(){return if owner.terminal_is_empty(){Ok(RetirementDemand{copy_bytes:std::mem::size_of::<Option<ControlledRetirement<String>>>(),depth:1,..Default::default()})}else{Ok(RetirementDemand{copy_bytes:owner.next_copy_byte_demand()?,capacity_bytes:owner.next_capacity_byte_demand(body)?,release_bytes:owner.next_release_byte_demand()?,depth:owner.next_depth_demand()?})};}Ok(if address.capacity()==0{Default::default()}else{RetirementDemand{copy_bytes:std::mem::size_of::<String>(),depth:1,..Default::default()}})
}
pub(crate) fn address_close(address:&mut String,owner:&mut Option<semio_framework_value::retirement::controlled::ControlledRetirement<String>>,grant:semio_framework_value::retained_clone::RetainedCloneGrant)->Result<semio_framework_value::retained_clone::RetainedCloneStep,semio_framework_value::ValueError>{
 use semio_framework_value::{retirement::controlled::ControlledRetirement,retained_clone::{RetainedCloneProgress,RetainedCloneStep}};if address_terminal(address,owner){return Ok(RetainedCloneStep::Complete(Default::default()));}let demand=address_demands(address,owner,grant.maximum_copy_bytes)?;if grant.maximum_items==0||grant.maximum_copy_bytes<demand.copy_bytes||grant.maximum_capacity_bytes<demand.capacity_bytes||grant.maximum_release_bytes<demand.release_bytes||grant.maximum_depth<demand.depth{return Ok(RetainedCloneStep::Progress(Default::default()));}if let Some(active)=owner.as_mut(){if active.terminal_is_empty(){*owner=None;return Ok(RetainedCloneStep::Complete(RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,..Default::default()}));}return active.step(grant).map(|step|RetainedCloneStep::Progress(step.progress()));}*owner=Some(ControlledRetirement::new(std::mem::take(address)).map_err(|(error,_)|error).expect("original window address supports physical retirement"));Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,..Default::default()}))
}
#[path="🔄️refresh/🦀️.rs"]
mod refresh;
pub use refresh::WindowAuthorityRefreshStep;
pub(crate) use refresh::{WindowRefreshSnapshot,refresh_demands,refresh_transfer};
