//! 📦️ Native cursor backing seeds and releases actual allocations in separately funded turns.
use std::mem::{ManuallyDrop,size_of};
use semio_framework_value_control_contract::{OriginalEffectReply as Reply,OriginalEffectReplyComplete as Complete,OriginalWorkBindingIdentity,RetainedAllocationEffect as Effect,RetainedCloneProgress as Progress,RetainedCloneStep as Step,RetainedWorkOutcome,RetainedWorkPoll,RetirementDemand,ValueError,retained_clone_grant_admits};
use semio_framework_value_native_storage::NativeOwnedStorage;
use semio_framework_value_native_clone_engine::RetainedCursorRelease;
use semio_framework_value_retirement_contract::OriginalRetirementFailure;
pub use semio_framework_value_native_storage::NativeSeedTemplate;
fn invalid()->ValueError{ValueError::invariant("native projection backing changed its original ownership")}
/// 🧳️ Keeps partial native seeds untyped and live cursors at their original allocated address.
pub struct NativeProjectionBacking<C>{storage:NativeOwnedStorage<C>,closed:ManuallyDrop<Option<Box<C>>>,template:NativeSeedTemplate<C>,binding:Option<OriginalWorkBindingIdentity>}
impl<C> NativeProjectionBacking<C>{
 /// 🌱️ Creates only vacant pointer metadata and a native seed descriptor.
 pub const fn new(template:NativeSeedTemplate<C>)->Self{Self{storage:NativeOwnedStorage::empty(),closed:ManuallyDrop::new(None),template,binding:None}}
 pub fn initialized(&self)->bool{self.storage.initialized()}
 pub fn get(&self)->Option<&C>{self.storage.get()}
 pub fn get_mut(&mut self)->Option<&mut C>{self.storage.get_mut()}
 pub fn retained(&self)->Option<&C>{self.storage.get().or_else(||self.closed.as_deref())}
 fn binding_demand()->RetirementDemand{RetirementDemand{copy_bytes:size_of::<Option<OriginalWorkBindingIdentity>>(),depth:1,..Default::default()}}
 pub fn validate_effect(&self,effect:&Effect<'_>)->Result<(),ValueError>{if self.binding.is_some_and(|binding|!binding.matches_effect(effect)){Err(invalid())}else{Ok(())}}
 /// 🧷️ Authenticates the same genuine Work principal before a before-effect child Birth.
 pub fn validate_work_binding(&self,work:OriginalWorkBindingIdentity)->Result<(),ValueError>{if self.binding.is_some_and(|binding|binding!=work){Err(invalid())}else{Ok(())}}
 fn funded(effect:&mut Effect<'_>,demand:RetirementDemand)->Result<bool,ValueError>{Ok(retained_clone_grant_admits(effect.grant(),demand)&&effect.remaining_work_units()?>=1)}
 fn local(turn:RetainedWorkPoll)->Result<RetainedWorkOutcome,ValueError>{match turn{RetainedWorkPoll::Complete(turn)=>Ok(turn),RetainedWorkPoll::Pending{..}=>Err(ValueError::invariant("closed native backing operation suspended"))}}
 /// 📏️ Quotes one actual binding, backing reservation or native seed part.
 pub fn birth_demand(&self)->Result<RetirementDemand,ValueError>{if self.closed.is_some(){return Err(invalid())}if self.initialized(){return Ok(Default::default())}if self.binding.is_none(){return Ok(Self::binding_demand())}if !self.storage.reserved(){return Ok(NativeOwnedStorage::<C>::reserve_demand())}self.storage.template_seed_demand(self.template)}
 pub fn birth_work_demand(&self)->u64{u64::from(!self.initialized())}
 fn birth_native(&mut self,effect:&mut Effect<'_>)->Result<Step,ValueError>{
  self.validate_effect(effect)?;let demand=self.birth_demand()?;if self.initialized(){return Ok(Step::Complete(Default::default()))}if !Self::funded(effect,demand)?{return Ok(Step::Progress(Default::default()))}
  if self.binding.is_none(){if !effect.performed(1)?{return Err(invalid())}self.binding=Some(OriginalWorkBindingIdentity::from_effect(effect));return Ok(Step::Progress(Progress{copied_items:1,copied_bytes:demand.copy_bytes,..Default::default()}))}
  let turn=if !self.storage.reserved(){Self::local(self.storage.reserve_empty(effect))?}else{self.storage.initialize_template(self.template,effect)};turn.result
 }
 /// 📥️ Records each resource-free native birth directly into its caller-owned reply.
 pub fn birth_into<'r>(&mut self,effect:&mut Effect<'_>,reply:Reply<'r,Step,ValueError>)->Result<Complete<'r,Step,ValueError>,ValueError>{self.validate_effect(effect)?;self.birth_demand()?;Ok(reply.complete(self.birth_native(effect)))}
 /// 📬️ Deposits the same native birth directly into the actual retirement failure channel.
 pub fn birth_retirement_into<'r>(&mut self,effect:&mut Effect<'_>,reply:Reply<'r,Step,OriginalRetirementFailure>)->Result<Complete<'r,Step,OriginalRetirementFailure>,ValueError>{self.validate_effect(effect)?;self.birth_demand()?;Ok(reply.complete(self.birth_native(effect).map_err(OriginalRetirementFailure::from_primary)))}
 /// 📐️ Quotes only the selected terminal box, raw partial backing or final original binding.
 pub fn release_demand(&self,proof:Option<&RetainedCursorRelease<C>>)->Result<RetirementDemand,ValueError>{
  if let Some(cursor)=self.retained(){let proof=proof.ok_or_else(invalid)?;if !proof.validates(cursor){return Err(invalid())}return Ok(if self.initialized(){NativeOwnedStorage::<C>::box_demand()}else{proof.boxed_demand()})}
  if !self.storage.terminal_is_empty(){return Ok(self.storage.release_demand())}if self.binding.is_some(){return Ok(Self::binding_demand())}Ok(Default::default())
 }
 pub fn release_work_demand(&self)->u64{u64::from(!self.terminal_is_empty())}
 fn release_native(&mut self,proof:Option<RetainedCursorRelease<C>>,effect:&mut Effect<'_>)->Result<Step,ValueError>{
  self.validate_effect(effect)?;let demand=self.release_demand(proof.as_ref())?;if self.terminal_is_empty(){return Ok(Step::Complete(Default::default()))}if !Self::funded(effect,demand)?{return Ok(Step::Progress(Default::default()))}
  if self.initialized(){return Self::local(self.storage.take_box_into(&mut self.closed,effect))?.result.map(|step|Step::Progress(step.progress()))}
  if self.closed.is_some(){let proof=proof.ok_or_else(invalid)?;if !proof.validates(self.closed.as_ref().ok_or_else(invalid)?){return Err(invalid())}if !effect.performed(1)?{return Err(invalid())}drop(self.closed.take());return Ok(Step::Progress(Progress{copied_items:1,copied_bytes:demand.copy_bytes,released_bytes:demand.release_bytes,..Default::default()}))}
  if !self.storage.terminal_is_empty(){return self.storage.release(effect).result.map(|step|Step::Progress(step.progress()))}
  if !effect.performed(1)?{return Err(invalid())}self.binding=None;Ok(Step::Complete(Progress{copied_items:1,copied_bytes:demand.copy_bytes,..Default::default()}))
 }
 /// ♻️ Releases partial storage without a typed Drop and terminal cursors through their native witness.
 pub fn release_into<'r>(&mut self,proof:Option<RetainedCursorRelease<C>>,effect:&mut Effect<'_>,reply:Reply<'r,Step,ValueError>)->Result<Complete<'r,Step,ValueError>,ValueError>{self.validate_effect(effect)?;self.release_demand(proof.as_ref())?;Ok(reply.complete(self.release_native(proof,effect)))}
 /// 📨️ Uses the fixed first-party retirement cause constructor at the same native deposit boundary.
 pub fn release_retirement_into<'r>(&mut self,proof:Option<RetainedCursorRelease<C>>,effect:&mut Effect<'_>,reply:Reply<'r,Step,OriginalRetirementFailure>)->Result<Complete<'r,Step,OriginalRetirementFailure>,ValueError>{self.validate_effect(effect)?;self.release_demand(proof.as_ref())?;Ok(reply.complete(self.release_native(proof,effect).map_err(OriginalRetirementFailure::from_primary)))}
 /// 📤️ Quotes a live same-allocation cursor transfer into the parent's actual box slot.
 pub fn take_box_demand(&self)->Result<RetirementDemand,ValueError>{if !self.initialized()||self.closed.is_some(){return Err(invalid())}Ok(NativeOwnedStorage::<C>::box_demand())}
 pub fn take_box_work_demand(&self)->u64{u64::from(self.initialized())}
 fn take_box_native(&mut self,output:&mut Option<Box<C>>,effect:&mut Effect<'_>)->Result<Step,ValueError>{self.validate_effect(effect)?;if output.is_some(){return Err(invalid())}let demand=self.take_box_demand()?;if !Self::funded(effect,demand)?{return Ok(Step::Progress(Default::default()))}Self::local(self.storage.take_box_into(output,effect))?.result}
 /// 🪑️ Publishes the real initialized cursor without moving its payload allocation.
 pub fn take_box_retirement_into<'r>(&mut self,output:&mut Option<Box<C>>,effect:&mut Effect<'_>,reply:Reply<'r,Step,OriginalRetirementFailure>)->Result<Complete<'r,Step,OriginalRetirementFailure>,ValueError>{Ok(reply.complete(self.take_box_native(output,effect).map_err(OriginalRetirementFailure::from_primary)))}
 pub fn terminal_is_empty(&self)->bool{self.storage.terminal_is_empty()&&self.closed.is_none()&&self.binding.is_none()}
}
impl<C> Drop for NativeProjectionBacking<C>{fn drop(&mut self){assert!(self.terminal_is_empty()||std::thread::panicking(),"native projection backing retains its original allocation");if self.terminal_is_empty(){unsafe{ManuallyDrop::drop(&mut self.closed)}}}}
