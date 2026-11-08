//! 🏗️ The original cancellation node and its physical backing stay in distinct paid slots.
use super::*;
use semio_framework_value::{RetainedAllocationEffect,RetainedCloneProgress,RetainedCloneStep,RetainedWorkOutcome,RetirementDemand,ValueError,ValueRefusalKind};
use semio_framework_value_allocation_return::{shared_uninitialized_demand,shared_uninitialized_work_demand,try_uninitialized_arc_into,shared_arc_publication_demand,shared_arc_publication_work_demand,publish_uninitialized_arc_from,uninitialized_arc_release_demand,release_uninitialized_arc_from};
use std::mem::{ManuallyDrop,MaybeUninit,size_of};
/// 🪪️ Identifies one actual cancellation backing without granting authority.
#[derive(Clone,Copy,Debug,PartialEq,Eq)]
pub struct OriginalCancelRootIdentity(usize);
/// 🧳️ Holds the original node, uninitialized backing and published Arc until actual transfer.
pub struct OriginalCancelRootAdmission{input:ManuallyDrop<Option<CancelNode>>,backing:ManuallyDrop<Option<Arc<MaybeUninit<CancelNode>>>>,published:ManuallyDrop<Option<Arc<CancelNode>>>}
impl OriginalCancelRootAdmission{
 /// 🪹️ Constructs empty inline slots without allocating.
 pub fn new()->Self{Self{input:ManuallyDrop::new(None),backing:ManuallyDrop::new(None),published:ManuallyDrop::new(None)}}
 pub fn terminal_is_empty(&self)->bool{self.input.is_none()&&self.backing.is_none()&&self.published.is_none()}
 /// 👁️ Observes the resident node and backing without lending their authority.
 pub fn has_input(&self)->bool{self.input.is_some()}
 pub fn has_backing(&self)->bool{self.backing.is_some()}
 /// 🪪️ Observes the same actual backing through its resident slot.
 pub fn backing_identity(&self)->Option<OriginalCancelRootIdentity>{self.backing.as_ref().map(|value|OriginalCancelRootIdentity(Arc::as_ptr(value).cast::<()>() as usize))}
 pub fn token_identity(token:&CancelToken)->OriginalCancelRootIdentity{OriginalCancelRootIdentity(Arc::as_ptr(&token.0) as usize)}
 pub fn is_published(&self)->bool{self.published.is_some()}
 pub fn prepare_demand()->RetirementDemand{RetirementDemand{copy_bytes:size_of::<Option<CancelNode>>(),depth:1,..Default::default()}}
 /// 🌱️ Creates only the fresh root node after the same actual Work receipt.
 pub fn prepare(&mut self,effect:&mut RetainedAllocationEffect<'_>)->RetainedWorkOutcome{let before=effect.performed_boundary();let result=(||{if !self.terminal_is_empty(){return Err(ValueError::invariant("cancellation root admission already owns a node"))}let demand=Self::prepare_demand();if effect.remaining_work_units()?<1||!semio_framework_value::retained_clone::retained_clone_grant_admits(effect.grant(),demand){return Ok(RetainedCloneStep::Progress(Default::default()))}if !effect.performed(1)?{return Err(ValueError::literal(ValueRefusalKind::WorkLimit,"cancellation node preparation exceeded original work"))}*self.input=Some(CancelNode{local:AtomicU8::new(0),parent:None,waiters:CancelWaiters::new()});Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,..Default::default()}))})();RetainedWorkOutcome{result,work:effect.performed_since(before).expect("same cancellation preparation receipt")}}
 pub fn backing_demand()->Result<RetirementDemand,ValueError>{shared_uninitialized_demand::<CancelNode>()}
 pub fn backing_work_demand()->u64{shared_uninitialized_work_demand()}
 /// 👶️ Keeps the actual backing and allocator marker in the original slots.
 pub fn admit_backing(&mut self,effect:&mut RetainedAllocationEffect<'_>)->RetainedWorkOutcome{if self.input.is_none()||self.published.is_some(){return RetainedWorkOutcome{result:Err(ValueError::invariant("cancellation backing has no eligible node")),work:0}}try_uninitialized_arc_into(&mut self.backing,effect)}
 pub fn publication_demand()->Result<RetirementDemand,ValueError>{shared_arc_publication_demand::<CancelNode>()}
 pub fn publication_work_demand()->u64{shared_arc_publication_work_demand()}
 /// 📦️ Initializes the same paid backing before publishing its original Arc.
 pub fn publish(&mut self,effect:&mut RetainedAllocationEffect<'_>)->RetainedWorkOutcome{publish_uninitialized_arc_from(&mut self.backing,&mut self.input,&mut self.published,effect)}
 pub fn take_demand()->RetirementDemand{RetirementDemand{copy_bytes:size_of::<Option<Arc<CancelNode>>>()+size_of::<Option<CancelToken>>(),depth:1,..Default::default()}}
 /// 📤️ Transfers the same live root to its caller-owned token slot.
 pub fn take_token(&mut self,output:&mut Option<CancelToken>,effect:&mut RetainedAllocationEffect<'_>)->RetainedWorkOutcome{let before=effect.performed_boundary();let result=(||{if output.is_some(){return Err(ValueError::invariant("cancellation token output is occupied"))}if self.published.is_none(){return Ok(RetainedCloneStep::Progress(Default::default()))}let demand=Self::take_demand();if effect.remaining_work_units()?<1||!semio_framework_value::retained_clone::retained_clone_grant_admits(effect.grant(),demand){return Ok(RetainedCloneStep::Progress(Default::default()))}if !effect.performed(1)?{return Err(ValueError::literal(ValueRefusalKind::WorkLimit,"cancellation token transfer exceeded original work"))}*output=Some(CancelToken(self.published.take().expect("published cancellation Arc checked")));Ok(RetainedCloneStep::Complete(RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,..Default::default()}))})();RetainedWorkOutcome{result,work:effect.performed_since(before).expect("same cancellation token receipt")}}
 pub fn backing_release_demand(&self)->RetirementDemand{uninitialized_arc_release_demand(&self.backing)}
 /// ♻️ Releases only the actual uninitialized backing under the same paid effect.
 pub fn release_backing(&mut self,effect:&mut RetainedAllocationEffect<'_>)->RetainedWorkOutcome{release_uninitialized_arc_from(&mut self.backing,effect)}
 /// ♻️ Discards only the never-published fresh node after paid header work.
 pub fn discard_input(&mut self,effect:&mut RetainedAllocationEffect<'_>)->RetainedWorkOutcome{let before=effect.performed_boundary();let result=(||{if self.backing.is_some()||self.published.is_some(){return Err(ValueError::invariant("cancellation node retains its backing"))}if self.input.is_none(){return Ok(RetainedCloneStep::Complete(Default::default()))}let demand=Self::prepare_demand();if effect.remaining_work_units()?<1||!semio_framework_value::retained_clone::retained_clone_grant_admits(effect.grant(),demand){return Ok(RetainedCloneStep::Progress(Default::default()))}if !effect.performed(1)?{return Err(ValueError::literal(ValueRefusalKind::WorkLimit,"cancellation node return exceeded original work"))}drop(self.input.take());Ok(RetainedCloneStep::Complete(RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,..Default::default()}))})();RetainedWorkOutcome{result,work:effect.performed_since(before).expect("same cancellation discard receipt")}}
}
impl Default for OriginalCancelRootAdmission{fn default()->Self{Self::new()}}
impl Drop for OriginalCancelRootAdmission{fn drop(&mut self){assert!(std::thread::panicking()||self.terminal_is_empty(),"cancellation admission retains original owners");if self.terminal_is_empty(){unsafe{ManuallyDrop::drop(&mut self.input);ManuallyDrop::drop(&mut self.backing);ManuallyDrop::drop(&mut self.published)}}}}
