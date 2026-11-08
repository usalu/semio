//! 📬️ Original window mutation payloads carry their captured typed retirement issuer; the displaced-snapshot refresh transfer lives beside them.
use semio_framework_value::{FactoryBoxedValue,RetainedAllocationEffect,RetainedCloneGrant,RetainedCloneProgress,RetainedWorkOutcome,RetirementDemand,ValueError,retirement::{RetireOwned,RetirementCursor,RetirementStep,admit_retirement_cursor,boxed_owner_birth_bytes}};
use std::{any::Any,mem::ManuallyDrop};
pub trait ErasedWindowMutationValue:Send+Sync{
 fn as_any(&self)->&dyn Any;
 fn into_any(self:Box<Self>)->Box<dyn Any+Send>;
 fn admit_payload(self:Box<Self>,effect:&mut RetainedAllocationEffect<'_>)->(Option<Box<dyn ErasedWindowMutationValue>>,RetainedWorkOutcome<Option<(Box<dyn RetirementCursor>,RetainedCloneProgress)>>);
 fn payload_birth_demand(&self)->RetirementDemand;
 /// 🧬️ Copies the payload into a fresh original box that keeps the captured retirement issuer.
 fn clone_boxed(&self)->Box<dyn ErasedWindowMutationValue>;
}
impl<T:Clone+Send+Sync+'static> ErasedWindowMutationValue for FactoryBoxedValue<T>{
 fn as_any(&self)->&dyn Any{self.original.as_ref()}
 fn into_any(self:Box<Self>)->Box<dyn Any+Send>{self}
 fn admit_payload(self:Box<Self>,effect:&mut RetainedAllocationEffect<'_>)->(Option<Box<dyn ErasedWindowMutationValue>>,RetainedWorkOutcome<Option<(Box<dyn RetirementCursor>,RetainedCloneProgress)>>){let mut original=Some(self);let turn=admit_retirement_cursor(&mut original,effect);(original.map(|original|original as Box<dyn ErasedWindowMutationValue>),turn)}
 fn payload_birth_demand(&self)->RetirementDemand{let capacity_bytes=boxed_owner_birth_bytes::<Self>();RetirementDemand{copy_bytes:size_of::<Option<Box<Self>>>()+capacity_bytes,capacity_bytes,depth:1,..Default::default()}}
 fn clone_boxed(&self)->Box<dyn ErasedWindowMutationValue>{Box::new(FactoryBoxedValue{original:Box::new(self.original.as_ref().clone()),factory:std::sync::Arc::clone(&self.factory)})}
}
/// 🌱️ Keeps the erased original inline until its exact typed owner cursor is funded by the closing effect.
#[doc(hidden)]
pub struct ErasedWindowMutationRetirement{original:ManuallyDrop<Option<Box<dyn ErasedWindowMutationValue>>>}
impl ErasedWindowMutationRetirement{
 fn demand(&self)->Result<RetirementDemand,ValueError>{let Some(original)=self.original.as_ref()else{return Ok(Default::default())};let mut demand=original.payload_birth_demand();demand.depth+=1;Ok(demand)}
}
impl RetirementCursor for ErasedWindowMutationRetirement{
 fn next_computational_work(&self,_:usize)->Result<u64,ValueError>{Ok(u64::from(self.original.is_some()))}
 fn next_demand(&self,_:usize)->Result<RetirementDemand,ValueError>{self.demand()}
 fn close_step(&mut self,effect:&mut RetainedAllocationEffect<'_>)->RetirementStep{
  let Some(original)=self.original.take()else{return RetirementStep::Complete(0)};
  let grant=effect.grant();
  if grant.maximum_depth<2{*self.original=Some(original);return RetirementStep::BudgetExhausted}
  let mut child=effect.nested(RetainedCloneGrant{maximum_depth:grant.maximum_depth.saturating_sub(1),..grant});
  let (rest,turn)=original.admit_payload(&mut child);*self.original=rest;
  match turn.result{Ok(Some((cursor,progress)))=>RetirementStep::ChildProgress(cursor,progress,turn.work),Ok(None)=>RetirementStep::BudgetExhausted,Err(error)=>RetirementStep::Failure(error.into(),turn.work)}
 }
 fn terminal_is_empty(&self)->bool{self.original.is_none()}
 fn next_work_byte_demand(&self)->Result<usize,ValueError>{Ok(0)}
 fn next_birth_bytes(&self,_:usize)->Option<usize>{self.original.as_ref().map_or(Some(0),|original|Some(original.payload_birth_demand().capacity_bytes))}
 fn next_close_byte_demand(&self)->Option<usize>{Some(0)}
 fn next_depth_demand(&self)->Result<usize,ValueError>{Ok(if self.original.is_some(){2}else{0})}
 fn terminal_release_bytes(&self)->Option<usize>{self.terminal_is_empty().then_some(size_of::<Self>())}
}
impl Drop for ErasedWindowMutationRetirement{fn drop(&mut self){assert!(std::thread::panicking()||self.terminal_is_empty(),"erased window mutation abandoned before terminal-empty");if self.terminal_is_empty(){unsafe{ManuallyDrop::drop(&mut self.original)}}}}
impl RetireOwned for Box<dyn ErasedWindowMutationValue>{
 type RetirementCursor=ErasedWindowMutationRetirement;
 fn retirement_seed(self)->Self::RetirementCursor{ErasedWindowMutationRetirement{original:ManuallyDrop::new(Some(self))}}
 fn retirement_birth_bytes(&self)->Option<usize>{Some(size_of::<ErasedWindowMutationRetirement>())}
 fn controlled_retirement_supported()->bool{true}
}

#[path = "../../📬️mutation/🔄️refresh/🦀️.rs"]
pub mod refresh;
pub use refresh::WindowAuthorityRefreshStep;
