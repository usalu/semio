//! 🔗️ Original native receiving seals one typed child after its own receiver has transferred and closed.
use crate::OriginalReceivingPrimary;
use crate::RetainedWorkOutcome;
use super::{NativeReceive,NativeReceiver};
use crate::{NativeDecodeControl,RetirementDemand,ValueError,ValueRefusalKind,retained_clone::{RetainedCloneBinding,RetainedCloneClose,RetainedCloneGrant,RetainedCloneProgress,RetainedCloneRef,RetainedCloneStep,retained_clone_grant_admits},shared_issuance::{SealedShared,SharedIssuer}};
use std::mem::ManuallyDrop;
impl<T:super::NativeRecordFields+Send+Sync+'static> super::NativeRecordFields for SealedShared<T>{
 fn native_schema_count()->usize{T::native_schema_count()}
 fn native_schema_field(index:usize)->Option<super::NativeRecordSchema>{T::native_schema_field(index)}
}

/// 🪆️ Root construction keeps the same original decoder and receiving recipient.
pub trait NativeSharedRoot<T:crate::retirement::RetireOwned+Send+Sync+'static>:crate::retirement::RetireOwned+Send+Sync+'static+Sized{
 fn birth_demand(owner:&super::NativeSharedSlots<T,Self>)->RetirementDemand;
 fn receive(control:&mut NativeDecodeControl<'_>,owner:&mut super::NativeSharedSlots<T,Self>)->crate::RetainedWorkOutcome<RetainedCloneProgress,crate::OriginalReceivingPrimary>;
}
impl<T:crate::retirement::RetireOwned+Send+Sync+'static>NativeSharedRoot<T>for SealedShared<T>{
 fn birth_demand(owner:&super::NativeSharedSlots<T,Self>)->RetirementDemand{owner.demand()}
 fn receive(control:&mut NativeDecodeControl<'_>,owner:&mut super::NativeSharedSlots<T,Self>)->crate::RetainedWorkOutcome<RetainedCloneProgress,crate::OriginalReceivingPrimary>{owner.advance(control,SharedIssuer::owned())}
}
impl<T:crate::retirement::RetireOwned+Send+Sync+'static>NativeSharedRoot<T>for std::sync::Arc<T>{
 fn birth_demand(owner:&super::NativeSharedSlots<T,Self>)->RetirementDemand{owner.demand()}
 fn receive(control:&mut NativeDecodeControl<'_>,owner:&mut super::NativeSharedSlots<T,Self>)->crate::RetainedWorkOutcome<RetainedCloneProgress,crate::OriginalReceivingPrimary>{owner.advance(control)}
}
impl<T:super::NativeRecordFields>super::NativeRecordFields for std::sync::Arc<T>{
 fn native_schema_count()->usize{T::native_schema_count()}
 fn native_schema_field(index:usize)->Option<super::NativeRecordSchema>{T::native_schema_field(index)}
}
impl<I:Sync+'static+?Sized,T:NativeReceive<I>+Send+Sync+'static>NativeReceive<I>for SealedShared<T>{
 type Receiver=NativeSharedReceiver<I,T>;
 fn receiver()->Self::Receiver{Self::Receiver::new(false)}
 fn record_subset_receiver()->Option<Self::Receiver>{Some(Self::Receiver::new(true))}
}
impl<I:Sync+'static+?Sized,T:NativeReceive<I>+Send+Sync+'static>NativeReceive<I>for std::sync::Arc<T>{
 type Receiver=NativeSharedReceiver<I,T,Self>;
 fn receiver()->Self::Receiver{Self::Receiver::new(false)}
 fn record_subset_receiver()->Option<Self::Receiver>{Some(Self::Receiver::new(true))}
}

#[derive(Clone,Copy,PartialEq,Eq)]
enum Phase{Bind,Birth,Receive,Take,Close,Release,Seal,Ready,Spent}
/// 🧳️ The original input binding, boxed child and unpublished payload remain independently closeable.
pub struct NativeSharedReceiver<I:Sync+'static+?Sized,T:NativeReceive<I>+Send+Sync+'static,R:NativeSharedRoot<T>=SealedShared<T>>{
 binding:Option<RetainedCloneBinding>,child:semio_framework_value_native_storage::NativeOwnedStorage<T::Receiver>,retired_header:Option<Box<T::Receiver>>,raw:ManuallyDrop<Option<T>>,output:ManuallyDrop<Option<R>>,slots:super::NativeSharedSlots<T,R>,retirement:RetainedCloneClose,phase:Phase,closing:bool,subset:bool,input:std::marker::PhantomData<fn(&I)>,
}
fn invalid()->ValueError{ValueError::literal(ValueRefusalKind::InvariantViolated,"original shared receiving owner is closed, foreign or already transferred")}
fn fixed(copy_bytes:usize)->RetirementDemand{RetirementDemand{copy_bytes,depth:1,..Default::default()}}
fn receipt(d:RetirementDemand)->RetainedCloneProgress{RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,retained_capacity_bytes:d.capacity_bytes,released_bytes:d.release_bytes}}
fn nested(mut d:RetirementDemand,copy:usize)->Result<RetirementDemand,ValueError>{d.copy_bytes=d.copy_bytes.checked_add(copy).ok_or_else(invalid)?;d.depth=d.depth.checked_add(1).ok_or_else(invalid)?;Ok(d)}
fn child_grant(g:RetainedCloneGrant,copy:usize)->RetainedCloneGrant{RetainedCloneGrant{maximum_copy_bytes:g.maximum_copy_bytes-copy,maximum_depth:g.maximum_depth-1,..g}}
impl<I:Sync+'static+?Sized,T:NativeReceive<I>+Send+Sync+'static,R:NativeSharedRoot<T>> NativeSharedReceiver<I,T,R>{
 fn new(subset:bool)->Self{Self{binding:None,child:semio_framework_value_native_storage::NativeOwnedStorage::empty(),retired_header:None,raw:ManuallyDrop::new(None),output:ManuallyDrop::new(None),slots:Default::default(),retirement:Default::default(),phase:Phase::Bind,closing:false,subset,input:std::marker::PhantomData}}
 pub fn shared_admission(&mut self)->&mut crate::native_decoding::NativeSharedAdmission<T>{self.slots.admission()}
 fn child(&self)->Result<&T::Receiver,ValueError>{self.child.get().ok_or_else(invalid)}
}

impl<I:Sync+'static+?Sized,T:NativeReceive<I>+Send+Sync+'static,R:NativeSharedRoot<T>>NativeSharedReceiver<I,T,R>{
 fn unposted_step(&self)->Result<bool,ValueError>{Ok(match self.phase{Phase::Take=>true,Phase::Bind=>self.binding.is_some(),Phase::Birth=>self.child.initialized(),Phase::Receive=>self.child()?.output_ready(),Phase::Close=>self.child()?.terminal_is_empty(),Phase::Release=>self.child.terminal_is_empty()&&self.retired_header.is_none(),Phase::Seal=>self.output.is_some(),_=>false})}
 fn advance_one(&mut self,original:RetainedCloneRef<'_,I>,control:&mut NativeDecodeControl<'_>,body:usize,maximum_work_units:u64,g:RetainedCloneGrant)->RetainedWorkOutcome<RetainedCloneStep,OriginalReceivingPrimary>{let mut work=0;let result:Result<RetainedCloneStep,OriginalReceivingPrimary>=(||{let d=self.demands(original,body,maximum_work_units)?;if !retained_clone_grant_admits(g,d){return Ok(RetainedCloneStep::Progress(Default::default()))}control.checkpoint()?;crate::owned::receiving_capacity(control,d.capacity_bytes)?;match self.phase{
  Phase::Bind=>{if self.binding.is_none(){let turn=original.bind(&mut self.binding,g);let turn=control.receive_retained_work(g,RetainedWorkOutcome{result:turn.result.map(|p|RetainedCloneStep::Progress(p.unwrap_or_default())),work:turn.work});work=turn.work;return turn.result.map_err(Into::into)}work=1;self.phase=Phase::Birth;},
  Phase::Birth=>{if !self.child.reserved(){let turn=self.child.reserve(g);let turn=control.receive_retained_work(g,turn);work=turn.work;return turn.result.map_err(Into::into)}if !self.child.initialized(){let turn=if self.subset{let Some(receiver)=T::record_subset_receiver()else{work=1;return Err(ValueError::literal(ValueRefusalKind::UnsupportedOwner,"native shared owner has no flattened original receiver").with_retained_progress(receipt(fixed(0))).into())};{let mut header=Some(receiver);self.child.put_from(&mut header,g)}}else{{let mut header=Some(T::receiver());self.child.put_from(&mut header,g)}};work=turn.work;return turn.result.map(|s|RetainedCloneStep::Progress(s.progress())).map_err(Into::into)}work=1;self.phase=Phase::Receive;},
  Phase::Receive=>{if self.child()?.output_ready(){work=1;self.phase=Phase::Take;}else{let turn=self.child.get_mut().ok_or_else(||ValueError::invariant("get_mut target absent"))?.advance(original,control,body,maximum_work_units,child_grant(g,0));work=turn.work;return turn.result.map(|s|RetainedCloneStep::Progress(s.progress())).map_err(Into::into)}},
  Phase::Take=>{let turn=self.child.get_mut().ok_or_else(||ValueError::invariant("get_mut target absent"))?.take(child_grant(g,0));work=turn.work;let Some((value,p))=turn.result?else{return Ok(RetainedCloneStep::Progress(Default::default()))};*self.raw=Some(value);self.child.get_mut().ok_or_else(||ValueError::invariant("get_mut target absent"))?.begin_close();self.phase=Phase::Close;return Ok(RetainedCloneStep::Progress(p))},
  Phase::Close=>{if self.child()?.terminal_is_empty(){work=1;self.phase=Phase::Release;}else{let turn=self.child.get_mut().ok_or_else(||ValueError::invariant("get_mut target absent"))?.close_step(child_grant(g,0));let turn=control.receive_retained_work(g,turn);work=turn.work;return turn.result.map(|s|RetainedCloneStep::Progress(s.progress())).map_err(Into::into)}},
  Phase::Release=>{if self.child.reserved()||self.retired_header.is_some(){let turn=super::close_native_receiver_backing::<I,T,_>(&mut self.child,&mut self.retired_header,g);work=turn.work;return turn.result.map(|s|RetainedCloneStep::Progress(s.progress())).map_err(Into::into)}work=1;self.phase=Phase::Seal;},
  Phase::Seal=>{if self.output.is_some(){work=1;self.phase=Phase::Ready;}else if self.raw.is_some(){let turn=self.slots.capture(&mut self.raw,g);work=turn.work;return turn.result.map(|step|RetainedCloneStep::Progress(step.progress())).map_err(Into::into)}else if self.slots.output_ready(){let turn=self.slots.take(&mut self.output,g);work=turn.work;return turn.result.map(|step|RetainedCloneStep::Progress(step.progress())).map_err(Into::into)}else{let turn=R::receive(control,&mut self.slots);work=turn.work;return turn.result.map(RetainedCloneStep::Progress).map_err(Into::into)}},_=>return Err(invalid().into())}Ok(if self.output_ready(){RetainedCloneStep::Complete(receipt(fixed(0)))}else{RetainedCloneStep::Progress(receipt(fixed(0)))})})();crate::RetainedWorkOutcome{result,work}}
}
impl<I:Sync+'static+?Sized,T:NativeReceive<I>+Send+Sync+'static,R:NativeSharedRoot<T>> NativeReceiver<I,R> for NativeSharedReceiver<I,T,R>{
 fn demands(&self,original:RetainedCloneRef<'_,I>,body:usize,maximum_work_units:u64)->Result<RetirementDemand,ValueError>{if self.closing||matches!(self.phase,Phase::Ready|Phase::Spent){return Err(invalid())}original.check_binding(&self.binding)?;let metadata=||Ok(fixed(0));match self.phase{
  Phase::Bind=>if original.check_binding(&self.binding)?{metadata()}else{Ok(fixed(original.binding_copy_bytes()))},
  Phase::Birth=>if !self.child.reserved(){Ok(semio_framework_value_native_storage::NativeOwnedStorage::<T::Receiver>::reserve_demand())}else if !self.child.initialized(){Ok(semio_framework_value_native_storage::NativeOwnedStorage::<T::Receiver>::move_demand())}else{metadata()},
  Phase::Receive=>if self.child()?.output_ready(){metadata()}else{nested(self.child()?.demands(original,body,maximum_work_units)?,0)},Phase::Take=>nested(self.child()?.take_demand()?,0),
  Phase::Close=>if self.child()?.terminal_is_empty(){metadata()}else{nested(self.child()?.close_demands(body)?,0)},Phase::Release=>if self.child.reserved()||self.retired_header.is_some(){super::native_receiver_backing_demands::<I,T,_>(&self.child,&self.retired_header,body)}else{metadata()},Phase::Seal=>if self.output.is_some(){metadata()}else if self.raw.is_some(){Ok(super::NativeSharedSlots::<T,R>::capture_demand())}else if self.slots.output_ready(){Ok(super::NativeSharedSlots::<T,R>::take_demand())}else{Ok(R::birth_demand(&self.slots))},_=>Err(invalid())}}
 fn advance(&mut self,original:RetainedCloneRef<'_,I>,control:&mut NativeDecodeControl<'_>,body:usize,maximum_work_units:u64,g:RetainedCloneGrant)->crate::RetainedWorkOutcome<RetainedCloneStep,crate::OriginalReceivingPrimary>{semio_framework_value_native_receiver_protocol::native_receive_zero_work!(maximum_work_units);let mut batch=super::NativeReceivingBatch::new(g);while batch.grant().maximum_items!=0{let remaining=match batch.remaining_work(maximum_work_units){Ok(0)=>return batch.finish(),Ok(work)=>work,Err(error)=>return batch.refuse(error)};let own=match self.unposted_step(){Ok(own)=>own,Err(e)=>return batch.refuse(e)};let turn=self.advance_one(original,control,body,remaining,batch.grant());batch=match batch.record(turn,own,self.output_ready()){Ok(next)=>next,Err(done)=>return done};}batch.finish()}
 fn output_ready(&self)->bool{!self.closing&&self.phase==Phase::Ready&&self.output.is_some()}
 semio_framework_value_native_receiver_protocol::native_owned_output_state!(R,invalid,phase,Phase::Spent);
 fn begin_close(&mut self){self.closing=true;if let Some(child)=self.child.get_mut(){child.begin_close()}}
 fn close_demands(&self,body:usize)->Result<RetirementDemand,ValueError>{if !self.slots.terminal_is_empty(){return self.slots.close_demand(body)}if self.child.reserved()||self.retired_header.is_some(){return super::native_receiver_backing_demands::<I,T,_>(&self.child,&self.retired_header,body)}if !self.retirement.is_empty(){return self.retirement.next_owner_demands::<T>(false,&None,body)}if self.raw.is_some(){return self.retirement.next_owner_demands::<T>(true,&None,body)}if self.output.is_some(){return self.retirement.next_owner_demands::<R>(true,&None,body)}RetainedCloneBinding::close_demand(&self.binding,body)}
 semio_framework_value_native_receiver_protocol::shared_close_work_quote!();
 fn close_step(&mut self,effect:&mut crate::RetainedAllocationEffect<'_>)->crate::OriginalRetirementOutcome{let g=effect.grant();let mut work=0;let result:Result<RetainedCloneStep,crate::OriginalRetirementFailure>=(||{if self.terminal_is_empty(){return Ok(RetainedCloneStep::Complete(Default::default()))}let d=self.close_demands(g.maximum_copy_bytes)?;if !retained_clone_grant_admits(g,d){return Ok(RetainedCloneStep::Progress(Default::default()))}if !self.slots.terminal_is_empty(){let turn=self.slots.close_step(&mut effect.reborrow(g,effect.maximum_work_units()).expect("retirement child exceeds original effect"));work=turn.work;return turn.result.map(|step|RetainedCloneStep::Progress(step.progress()))}if self.child.reserved()||self.retired_header.is_some(){let turn=if let Some(child)=self.child.get_mut().filter(|child|!child.terminal_is_empty()){child.close_step(&mut effect.reborrow(child_grant(g,0),effect.maximum_work_units()).expect("retirement child exceeds original effect"))}else{let turn=super::close_native_receiver_backing::<I,T,_>(&mut self.child,&mut self.retired_header,g);crate::RetainedWorkOutcome{result:turn.result.map_err(Into::into),work:turn.work}};work=turn.work;return turn.result.map(|s|RetainedCloneStep::Progress(s.progress()))}if !self.retirement.is_empty(){let turn=self.retirement.step_granted(effect);work=turn.work;return turn.result}if self.raw.is_some(){let turn=self.retirement.begin_granted(&mut self.raw,effect);work=turn.work;return turn.result.map(|s|s.unwrap_or(RetainedCloneStep::Progress(Default::default()))).map_err(Into::into)}if self.output.is_some(){let turn=self.retirement.begin_granted(&mut self.output,effect);work=turn.work;return turn.result.map(|s|s.unwrap_or(RetainedCloneStep::Progress(Default::default()))).map_err(Into::into)}let turn=RetainedCloneBinding::close_one(&mut self.binding,effect);work=turn.work;turn.result.map_err(Into::into)})();crate::RetainedWorkOutcome{result,work}}
 semio_framework_value_native_receiver_protocol::shared_original_close_bridge!();
 fn terminal_is_empty(&self)->bool{self.slots.terminal_is_empty()&&self.child.terminal_is_empty()&&self.retired_header.is_none()&&self.raw.is_none()&&self.output.is_none()&&self.retirement.is_empty()&&self.binding.is_none()}

}
impl<I:Sync+'static+?Sized,T:NativeReceive<I>+Send+Sync+'static,R:NativeSharedRoot<T>> Drop for NativeSharedReceiver<I,T,R>{fn drop(&mut self){assert!(std::thread::panicking()||self.terminal_is_empty(),"original shared receiving requires funded closure");}}


#[cfg(test)]#[path="🧪️tests/🦀️.rs"]mod tests;
