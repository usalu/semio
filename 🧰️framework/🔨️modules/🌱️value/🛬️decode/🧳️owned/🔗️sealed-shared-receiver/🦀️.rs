//! 🔗️ Original native receiving seals one typed child after its own receiver has transferred and closed.
use super::{NativeReceive,NativeReceiver};
use crate::{NativeDecodeControl,RetirementDemand,ValueError,ValueRefusalKind,retained_clone::{RetainedCloneBinding,RetainedCloneClose,RetainedCloneGrant,RetainedCloneProgress,RetainedCloneRef,RetainedCloneStep,retained_clone_grant_admits},shared_issuance::{SealedShared,SharedIssuer}};
use std::mem::{ManuallyDrop,size_of};
impl<T:super::NativeRecordFields+Send+Sync+'static> super::NativeRecordFields for SealedShared<T>{
 fn native_field_memberships(key:&str)->usize{T::native_field_memberships(key)}
 fn native_field_key_bytes()->usize{T::native_field_key_bytes()}
}

#[derive(Clone,Copy,PartialEq,Eq)]
enum Phase{Bind,Birth,Receive,Take,Close,Release,Seal,Ready,Spent}
/// 🧳️ The original input binding, boxed child and unpublished payload remain independently closeable.
pub struct NativeSharedReceiver<I:Sync+'static,T:NativeReceive<I>+Send+Sync+'static>{
 binding:Option<RetainedCloneBinding>,child:ManuallyDrop<Option<Box<T::Receiver>>>,raw:ManuallyDrop<Option<T>>,output:ManuallyDrop<Option<SealedShared<T>>>,retirement:RetainedCloneClose,phase:Phase,closing:bool,subset:bool,input:std::marker::PhantomData<fn()->I>,
}
fn invalid()->ValueError{ValueError::literal(ValueRefusalKind::InvariantViolated,"original shared receiving owner is closed, foreign or already transferred")}
fn fixed(copy_bytes:usize)->RetirementDemand{RetirementDemand{copy_bytes,depth:1,..Default::default()}}
fn receipt(d:RetirementDemand)->RetainedCloneProgress{RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,retained_capacity_bytes:d.capacity_bytes,released_bytes:d.release_bytes}}
fn nested(mut d:RetirementDemand,copy:usize)->Result<RetirementDemand,ValueError>{d.copy_bytes=d.copy_bytes.checked_add(copy).ok_or_else(invalid)?;d.depth=d.depth.checked_add(1).ok_or_else(invalid)?;Ok(d)}
fn child_grant(g:RetainedCloneGrant,copy:usize)->RetainedCloneGrant{RetainedCloneGrant{maximum_copy_bytes:g.maximum_copy_bytes-copy,maximum_depth:g.maximum_depth-1,..g}}
fn add(mut p:RetainedCloneProgress,copy:usize)->Result<RetainedCloneProgress,ValueError>{p.copied_bytes=p.copied_bytes.checked_add(copy).ok_or_else(invalid)?;Ok(p)}
impl<I:Sync+'static,T:NativeReceive<I>+Send+Sync+'static> NativeSharedReceiver<I,T>{
 fn metadata()->usize{size_of::<Phase>()}
 fn child(&self)->Result<&T::Receiver,ValueError>{self.child.as_deref().ok_or_else(invalid)}
}
impl<I:Sync+'static,T:NativeReceive<I>+Send+Sync+'static> NativeReceive<I> for SealedShared<T>{
 type Receiver=NativeSharedReceiver<I,T>;
 fn receive_birth_demand()->RetirementDemand{fixed(size_of::<Self::Receiver>()*2)}
 fn begin_receive(g:RetainedCloneGrant)->Result<(Self::Receiver,RetainedCloneProgress),ValueError>{let d=<Self as NativeReceive<I>>::receive_birth_demand();if !retained_clone_grant_admits(g,d){return Err(invalid());}Ok((NativeSharedReceiver{binding:None,child:ManuallyDrop::new(None),raw:ManuallyDrop::new(None),output:ManuallyDrop::new(None),retirement:Default::default(),phase:Phase::Bind,closing:false,subset:false,input:std::marker::PhantomData},receipt(d)))}
 fn begin_record_subset(g:RetainedCloneGrant)->Result<(Self::Receiver,RetainedCloneProgress),ValueError>{let(mut receiver,p)=Self::begin_receive(g)?;receiver.subset=true;Ok((receiver,p))}
}
impl<I:Sync+'static,T:NativeReceive<I>+Send+Sync+'static> NativeReceiver<I,SealedShared<T>> for NativeSharedReceiver<I,T>{
 fn demands(&self,original:RetainedCloneRef<'_,I>,body:usize)->Result<RetirementDemand,ValueError>{
  if self.closing||matches!(self.phase,Phase::Ready|Phase::Spent){return Err(invalid());}original.check_binding(&self.binding)?;let m=Self::metadata();
  match self.phase{
   Phase::Bind=>Ok(fixed(original.binding_copy_bytes()+m)),
   Phase::Birth=>{let mut d=nested(T::receive_birth_demand(),size_of::<T::Receiver>()+size_of::<Option<Box<T::Receiver>>>()*2+m)?;d.capacity_bytes=d.capacity_bytes.checked_add(size_of::<T::Receiver>()).ok_or_else(invalid)?;Ok(d)},
   Phase::Receive=>nested(self.child()?.demands(original,body.saturating_sub(m))?,m),
   Phase::Take=>nested(self.child()?.take_demand()?,size_of::<Option<T>>()*2+m),
   Phase::Close=>nested(self.child()?.close_demands(body.saturating_sub(m))?,m),
   Phase::Release=>Ok(RetirementDemand{copy_bytes:size_of::<Option<Box<T::Receiver>>>()*2+m,release_bytes:size_of::<T::Receiver>(),depth:1,..Default::default()}),
   Phase::Seal=>nested(crate::native_decoding::NativeOwnedDecodeControl::shared_birth_demand::<T>(),m),
   _=>Err(invalid()),
  }
 }
 fn advance(&mut self,original:RetainedCloneRef<'_,I>,control:&mut NativeDecodeControl<'_>,body:usize,g:RetainedCloneGrant)->Result<RetainedCloneStep,ValueError>{
  let d=self.demands(original,body)?;if !retained_clone_grant_admits(g,d){return Ok(RetainedCloneStep::Progress(Default::default()));}control.checkpoint()?;let m=Self::metadata();
  let p=match self.phase{
   Phase::Bind=>{let p=original.bind(&mut self.binding,RetainedCloneGrant{maximum_copy_bytes:g.maximum_copy_bytes-m,..g})?.ok_or_else(invalid)?;self.phase=Phase::Birth;add(p,m)?},
   Phase::Birth=>{let extra=size_of::<T::Receiver>()+size_of::<Option<Box<T::Receiver>>>()*2+m;control.charge(d.capacity_bytes)?;let(child,mut p)=if self.subset{T::begin_record_subset(child_grant(g,extra))?}else{T::begin_receive(child_grant(g,extra))?};*self.child=Some(Box::new(child));self.phase=Phase::Receive;p.copied_bytes=p.copied_bytes.checked_add(extra).ok_or_else(invalid)?;p.retained_capacity_bytes=p.retained_capacity_bytes.checked_add(size_of::<T::Receiver>()).ok_or_else(invalid)?;p},
   Phase::Receive=>{let child=self.child.as_mut().ok_or_else(invalid)?;let p=child.advance(original,control,body.saturating_sub(m),child_grant(g,m)).map_err(|e|{let p=e.retained_progress();e.with_retained_progress(add(p,m).unwrap_or(p))})?.progress();if child.output_ready(){self.phase=Phase::Take;}add(p,m)?},
   Phase::Take=>{let extra=size_of::<Option<T>>()*2+m;control.charge(d.capacity_bytes)?;let child=self.child.as_mut().ok_or_else(invalid)?;let(value,p)=child.take(child_grant(g,extra))?.ok_or_else(invalid)?;*self.raw=Some(value);child.begin_close();self.phase=if child.terminal_is_empty(){Phase::Release}else{Phase::Close};add(p,extra)?},
   Phase::Close=>{control.charge(d.capacity_bytes)?;let child=self.child.as_mut().ok_or_else(invalid)?;let p=child.close_step(child_grant(g,m))?.progress();if child.terminal_is_empty(){self.phase=Phase::Release;}add(p,m)?},
   Phase::Release=>{self.child.take();self.phase=Phase::Seal;receipt(d)},
   Phase::Seal=>{let result=control.receive_shared(&mut self.raw,&mut self.output,SharedIssuer::owned());if self.output.is_some(){self.phase=Phase::Ready;}if result.is_err()&&self.output.is_some(){self.begin_close();}let mut p=result.map_err(|e|{let p=e.retained_progress();e.with_retained_progress(add(p,m).unwrap_or(p))})?;p.copied_bytes=d.copy_bytes;p},
   _=>return Err(invalid()),
  };crate::retained_clone::admit_retained_clone_progress(g,p,"original shared native receiving")?;Ok(if self.output_ready(){RetainedCloneStep::Complete(p)}else{RetainedCloneStep::Progress(p)})
 }
 fn output_ready(&self)->bool{!self.closing&&self.phase==Phase::Ready&&self.output.is_some()}
 fn take_demand(&self)->Result<RetirementDemand,ValueError>{if !self.output_ready(){return Err(invalid());}Ok(fixed(size_of::<Option<SealedShared<T>>>()*2+Self::metadata()))}
 fn take(&mut self,g:RetainedCloneGrant)->Result<Option<(SealedShared<T>,RetainedCloneProgress)>,ValueError>{let d=self.take_demand()?;if !retained_clone_grant_admits(g,d){return Ok(None);}let value=self.output.take().ok_or_else(invalid)?;self.phase=Phase::Spent;Ok(Some((value,receipt(d))))}
 fn begin_close(&mut self){self.closing=true;if let Some(child)=self.child.as_mut(){child.begin_close();}}
 fn close_demands(&self,body:usize)->Result<RetirementDemand,ValueError>{
  if !self.closing{return Err(invalid());}let m=Self::metadata();let body=body.saturating_sub(m);
  if let Some(child)=self.child.as_ref(){return if child.terminal_is_empty(){Ok(RetirementDemand{copy_bytes:size_of::<Option<Box<T::Receiver>>>()*2+m,release_bytes:size_of::<T::Receiver>(),depth:1,..Default::default()})}else{nested(child.close_demands(body)?,m)};}
  if !self.retirement.is_empty(){return nested(self.retirement.next_owner_demands::<T>(false,&None,body)?,m);}
  if self.raw.is_some(){return nested(self.retirement.next_owner_demands::<T>(true,&None,body)?,m);}
  if self.output.is_some(){return nested(self.retirement.next_owner_demands::<SealedShared<T>>(true,&None,body)?,m);}
  if self.binding.is_some(){return nested(RetainedCloneBinding::close_demand(&self.binding,body)?,m);}Ok(Default::default())
 }
 fn close_step(&mut self,g:RetainedCloneGrant)->Result<RetainedCloneStep,ValueError>{
  let d=self.close_demands(g.maximum_copy_bytes)?;if !retained_clone_grant_admits(g,d){return Ok(RetainedCloneStep::Progress(Default::default()));}if self.terminal_is_empty(){return Ok(RetainedCloneStep::Complete(Default::default()));}let m=Self::metadata();
  if let Some(child)=self.child.as_mut(){if child.terminal_is_empty(){self.child.take();return Ok(RetainedCloneStep::Progress(receipt(d)));}return child.close_step(child_grant(g,m)).and_then(|s|add(s.progress(),m)).map(RetainedCloneStep::Progress);}
  let child=child_grant(g,m);let p=if !self.retirement.is_empty(){self.retirement.step_granted(child)?.progress()}else if let Some(s)=self.retirement.begin_granted(&mut self.raw,child)?{s.progress()}else if let Some(s)=self.retirement.begin_granted(&mut self.output,child)?{s.progress()}else{RetainedCloneBinding::close_one(&mut self.binding,child)?.progress()};let p=add(p,m)?;Ok(if self.terminal_is_empty(){RetainedCloneStep::Complete(p)}else{RetainedCloneStep::Progress(p)})
 }
 fn terminal_is_empty(&self)->bool{self.binding.is_none()&&self.child.is_none()&&self.raw.is_none()&&self.output.is_none()&&self.retirement.is_empty()}
}
impl<I:Sync+'static,T:NativeReceive<I>+Send+Sync+'static> Drop for NativeSharedReceiver<I,T>{fn drop(&mut self){assert!(std::thread::panicking()||self.terminal_is_empty(),"original shared receiving requires funded closure");}}

#[cfg(test)]#[path="🧪️tests/🦀️.rs"]mod tests;
