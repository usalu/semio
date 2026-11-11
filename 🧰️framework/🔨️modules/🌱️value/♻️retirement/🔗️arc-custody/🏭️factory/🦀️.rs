//! 🔐️ Issued shared custody elects the original payload before its captured factory retires it.
use crate::{ArtifactOwnedValueRetirementFactory,ErasedSnapshotRetirement,RetirementDemand,ValueError,ValueRefusalKind,RetainedCloneGrant,RetainedCloneProgress,RetainedCloneStep,RetainedWorkOutcome,RetainedWorkPoll,OriginalRetirementPoll,RetainedAllocationEffect,OriginalRetirementCloseLoan};
use crate::retirement::arc_custody::original::{SharedOriginalStorage,SharedOriginal};
use semio_framework_value_control_contract::{OriginalWorkBindingIdentity,OriginalEffectReply,OriginalEffectReplyComplete};
use std::{mem::ManuallyDrop,sync::Arc};
type Reply<'a>=OriginalEffectReply<'a,RetainedCloneStep,crate::OriginalRetirementFailure>;
type Complete<'a>=OriginalEffectReplyComplete<'a,RetainedCloneStep,crate::OriginalRetirementFailure>;
fn invalid()->ValueError{ValueError::invariant("captured shared original changed its Work or destination")}
fn fixed(copy_bytes:usize)->RetirementDemand{RetirementDemand{copy_bytes,depth:1,..Default::default()}}
#[path="🌳️root/🦀️.rs"]
pub mod root;
pub use root::ArcRootRetirement;
/// 🧳️ The issued alias, elected original and installed payload issuer remain resident.
pub struct FactorySharedRetirement<T:Send+Sync+'static>{alias:ManuallyDrop<Option<SharedOriginalStorage<T>>>,elected:ManuallyDrop<Option<SharedOriginal<T>>>,owned:crate::owned::FactoryOwnedRetirement<T>,work:Option<OriginalWorkBindingIdentity>,reply:semio_framework_value_retirement::ReplyStash<crate::OriginalRetirementFailure>}
impl<T:Send+Sync+'static> FactorySharedRetirement<T>{
 pub fn new(alias:SharedOriginalStorage<T>,factory:Arc<dyn ArtifactOwnedValueRetirementFactory<T>>)->Self{Self{alias:ManuallyDrop::new(Some(alias)),elected:ManuallyDrop::new(None),owned:crate::owned::FactoryOwnedRetirement::from_captured(None,factory),work:None,reply:Default::default()}}
 pub const fn constructor_capacity_bytes()->usize{size_of::<Self>()}
 pub fn original_admission_demands(original:&Option<SharedOriginalStorage<T>>)->RetirementDemand{if original.is_none(){return Default::default()}RetirementDemand{copy_bytes:size_of::<Option<SharedOriginalStorage<T>>>()+size_of::<Option<Arc<dyn ArtifactOwnedValueRetirementFactory<T>>>>()+size_of::<Self>()+size_of::<Option<Box<dyn ErasedSnapshotRetirement>>>(),capacity_bytes:size_of::<Self>(),depth:1,..Default::default()}}
 pub fn admit_original_source_work_demand(original:&Option<SharedOriginalStorage<T>>)->u64{u64::from(original.is_some())}
 /// 📥️ Allocates the actual fixed native frame before taking either issued input.
 pub fn admit_original_into<'r>(original:&mut Option<SharedOriginalStorage<T>>,factory:&mut Option<Arc<dyn ArtifactOwnedValueRetirementFactory<T>>>,output:&mut Option<Box<dyn ErasedSnapshotRetirement>>,effect:&mut RetainedAllocationEffect<'_>,reply:Reply<'r>)->Result<Complete<'r>,ValueError>{
  if output.is_some(){return Ok(reply.complete(Err(invalid().into())))}if original.is_none(){return Ok(reply.complete(Ok(RetainedCloneStep::Complete(Default::default()))))}if factory.is_none()||effect.actual().is_some(){return Ok(reply.complete(Err(invalid().into())))}let demand=Self::original_admission_demands(original);if !crate::retained_clone_grant_admits(effect.grant(),demand)||effect.remaining_work_units()?<1{return Ok(reply.complete(Ok(RetainedCloneStep::Progress(Default::default()))))}if !effect.performed(1)?{return Err(invalid())}
  *effect.actual()=Some(0);let layout=std::alloc::Layout::new::<Self>();let Some(pointer)=std::ptr::NonNull::new(unsafe{std::alloc::alloc(layout)}.cast::<Self>())else{return Ok(reply.complete(Err(ValueError::literal(ValueRefusalKind::AllocationFailed,"captured shared frame allocation failed").into())))};*effect.actual()=Some(layout.size());let mut owner=Self::new(original.take().expect("funded issued original"),factory.take().expect("funded original issuer"));owner.work=Some(OriginalWorkBindingIdentity::from_effect(effect));let owner=unsafe{pointer.as_ptr().write(owner);Box::from_raw(pointer.as_ptr())};*output=Some(owner);Ok(reply.complete(Ok(RetainedCloneStep::Complete(RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,retained_capacity_bytes:layout.size(),..Default::default()}))))
 }
 pub fn demands(&self,body:usize)->Result<RetirementDemand,ValueError>{if self.work.is_none(){return Ok(fixed(size_of::<Option<OriginalWorkBindingIdentity>>()))}if let Some(alias)=self.alias.as_ref(){return if alias.terminal_is_empty(){Ok(fixed(size_of::<Option<SharedOriginalStorage<T>>>()))}else{alias.retire_into_demand()}}if let Some(owner)=self.elected.as_ref(){return if owner.terminal_is_empty(){Ok(fixed(size_of::<Option<SharedOriginal<T>>>()))}else{owner.take_value_demand()}}self.owned.demands(body)}
 fn close<'r>(&mut self,original:Option<&mut OriginalRetirementCloseLoan<'_>>,body:usize,effect:&mut RetainedAllocationEffect<'_>,reply:Reply<'r>,recover:bool)->Result<Complete<'r>,ValueError>{
  if self.work.is_some_and(|work|!work.matches_effect(effect)){return Err(invalid())}
  if self.work.is_none(){if recover{return Err(invalid())}let d=self.demands(body)?;if !crate::retained_clone_grant_admits(effect.grant(),d)||effect.remaining_work_units()?<1{return Ok(reply.complete(Ok(RetainedCloneStep::Progress(Default::default()))))}if !effect.performed(1)?{return Err(invalid())}self.work=Some(OriginalWorkBindingIdentity::from_effect(effect));return Ok(reply.complete(Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,..Default::default()}))))}
  if self.alias.is_none()&&self.elected.is_none(){return match(original,recover){(Some(original),true)=>self.owned.recover_close_original_into(original,body,effect,reply),(Some(original),false)=>self.owned.close_original_into(original,body,effect,reply),(None,true)=>self.owned.recover_close_into(body,effect,reply),(None,false)=>self.owned.close_into(body,effect,reply)}}
  if recover{return Err(invalid())}if let Some(alias)=self.alias.as_mut(){if !alias.terminal_is_empty(){return alias.retire_into(&mut self.elected,effect,reply.progress())}}
  let d=self.demands(body)?;if !crate::retained_clone_grant_admits(effect.grant(),d)||effect.remaining_work_units()?<self.next_computational_work(body)?{return Ok(reply.complete(Ok(RetainedCloneStep::Progress(Default::default()))))}
  if self.alias.is_some()||self.elected.as_ref().is_some_and(SharedOriginal::terminal_is_empty){if !effect.performed(1)?{return Err(invalid())}if self.alias.is_some(){self.alias.take();}else{self.elected.take();}return Ok(reply.complete(Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,..Default::default()}))))}
  let owner=self.elected.as_mut().ok_or_else(invalid)?;let turn=owner.take_value_step(self.owned.original_destination()?,effect);Ok(reply.complete(turn.result.map(|progress|RetainedCloneStep::Progress(progress.unwrap_or_default())).map_err(Into::into)))
 }
}
impl<T:Send+Sync+'static> ErasedSnapshotRetirement for FactorySharedRetirement<T>{
 fn next_computational_work(&self,body:usize)->Result<u64,ValueError>{if self.work.is_none(){return Ok(1)}if let Some(alias)=self.alias.as_ref(){return if alias.terminal_is_empty(){Ok(1)}else{alias.retire_work_demand()}}if let Some(owner)=self.elected.as_ref(){return if owner.terminal_is_empty(){Ok(1)}else{owner.take_value_work_demand()}}self.owned.next_computational_work(body)}
 fn requires_original_birth(&self)->bool{self.work.is_some()&&self.alias.is_none()&&self.elected.is_none()&&self.owned.requires_original_birth()}
 fn close_original_birth_step(&mut self,original:&mut OriginalRetirementCloseLoan<'_>,grant:RetainedCloneGrant)->OriginalRetirementPoll{let binding=match original.original_work().binding_identity(){Ok(binding)=>binding,Err(error)=>return RetainedWorkPoll::Complete(RetainedWorkOutcome{result:Err(error.into()),work:0})};if self.work!=Some(binding){return RetainedWorkPoll::Complete(RetainedWorkOutcome{result:Err(invalid().into()),work:0})}self.owned.close_original_birth_step(original,grant)}
 fn requires_original_control(&self)->bool{self.work.is_some()&&self.alias.is_none()&&self.elected.is_none()&&self.owned.requires_original_control()}
 fn native_header_release(&self)->Option<semio_framework_value_retirement::RetirementHeaderRelease>{self.terminal_is_empty().then(||unsafe{semio_framework_value_retirement::RetirementHeaderRelease::native(self,0,0)})}
 fn close_into<'r>(&mut self,body:usize,effect:&mut RetainedAllocationEffect<'_>,reply:Reply<'r>)->Result<Complete<'r>,ValueError>{self.close(None,body,effect,reply,false)}
 fn recover_close_into<'r>(&mut self,body:usize,effect:&mut RetainedAllocationEffect<'_>,reply:Reply<'r>)->Result<Complete<'r>,ValueError>{self.close(None,body,effect,reply,true)}
 fn close_original_into<'r>(&mut self,original:&mut OriginalRetirementCloseLoan<'_>,body:usize,effect:&mut RetainedAllocationEffect<'_>,reply:Reply<'r>)->Result<Complete<'r>,ValueError>{if !original.original_work().binding_identity()?.matches_effect(effect){return Err(invalid())}self.close(Some(original),body,effect,reply,false)}
 fn recover_close_original_into<'r>(&mut self,original:&mut OriginalRetirementCloseLoan<'_>,body:usize,effect:&mut RetainedAllocationEffect<'_>,reply:Reply<'r>)->Result<Complete<'r>,ValueError>{if !original.original_work().binding_identity()?.matches_effect(effect){return Err(invalid())}self.close(Some(original),body,effect,reply,true)}
 fn resident_reply(&mut self)->Option<&mut semio_framework_value_retirement::ReplyStash<crate::OriginalRetirementFailure>>{Some(&mut self.reply)}
 fn close_step(&mut self,body:usize,_:&mut RetainedAllocationEffect<'_>)->OriginalRetirementPoll{RetainedWorkPoll::Complete(RetainedWorkOutcome{result:Err(ValueError::literal(ValueRefusalKind::UnsupportedOwner,"captured shared retirement requires its borrowed producer").into()),work:0})}
 fn terminal_is_empty(&self)->bool{self.alias.is_none()&&self.elected.is_none()&&self.owned.terminal_is_empty()}
 fn next_demand(&self,body:usize)->Result<RetirementDemand,ValueError>{self.demands(body)}
 fn next_copy_byte_demand(&self)->Result<usize,ValueError>{Ok(self.demands(0)?.copy_bytes)}
 fn next_capacity_byte_demand(&self,body:usize)->Result<usize,ValueError>{Ok(self.demands(body)?.capacity_bytes)}
 fn next_release_byte_demand(&self)->Result<usize,ValueError>{Ok(self.demands(0)?.release_bytes)}
 fn next_depth_demand(&self)->Result<usize,ValueError>{Ok(self.demands(0)?.depth)}
}
impl<T:Send+Sync+'static> Drop for FactorySharedRetirement<T>{fn drop(&mut self){assert!(std::thread::panicking()||self.terminal_is_empty(),"shared factory retains original custody");if self.terminal_is_empty(){unsafe{ManuallyDrop::drop(&mut self.alias);ManuallyDrop::drop(&mut self.elected)}}}}
#[cfg(test)]#[path="🧪️tests/🦀️.rs"]mod tests;
