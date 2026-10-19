//! 🪡️ Original-bound field copying seals only the authored typed child and retains every partial owner for closure.
use crate::{NativeDecodeControl,RetirementDemand,ValueError,ValueRefusalKind,retirement::shared::sealed::{SealedShared,SharedIssuer},retained_clone::{RetainedClone,RetainedCloneBinding,RetainedCloneClose,RetainedCloneCursor,RetainedCloneGrant,RetainedCloneProgress,RetainedCloneRef,RetainedCloneStep,RetainedFieldCursor,retained_clone_grant_admits}};
use std::mem::{ManuallyDrop,size_of};
/// 🧵️ Intrinsic field payload authority is explicit; enclosing native rows choose their own independent issuer.
pub struct NativeIntrinsicSharedField<T:RetainedClone+Send+Sync+'static>{binding:Option<RetainedCloneBinding>,cursor:RetainedFieldCursor<T>,raw:ManuallyDrop<Option<T>>,output:ManuallyDrop<Option<SealedShared<T>>>,retirement:RetainedCloneClose,phase:u8,closing:bool}
impl<T:RetainedClone+Send+Sync+'static> Default for NativeIntrinsicSharedField<T>{fn default()->Self{Self{binding:None,cursor:Default::default(),raw:ManuallyDrop::new(None),output:ManuallyDrop::new(None),retirement:Default::default(),phase:0,closing:false}}}
fn invalid()->ValueError{ValueError::literal(ValueRefusalKind::InvariantViolated,"native field receiving has closed or transferred its original owner")}
fn fixed(copy_bytes:usize)->RetirementDemand{RetirementDemand{copy_bytes,depth:1,..Default::default()}}
fn receipt(d:RetirementDemand)->RetainedCloneProgress{RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,retained_capacity_bytes:d.capacity_bytes,released_bytes:d.release_bytes}}
impl<T:RetainedClone+Send+Sync+'static> NativeIntrinsicSharedField<T>{
 pub fn ready(&self)->bool{!self.closing&&self.phase==4&&self.output.is_some()}
 pub fn demands(&self,original:RetainedCloneRef<'_,T>,body:usize)->Result<RetirementDemand,ValueError>{
  if self.closing||self.phase>=4{return Err(invalid());}
  if !original.check_binding(&self.binding)?{return Ok(fixed(original.binding_copy_bytes()));}
  if self.phase==0{let d=self.cursor.advance_demands(original,body)?;return if self.cursor.output_ready(){Ok(fixed(size_of::<T>()*2+size_of::<u8>()))}else{Ok(d)};}
  if self.phase==1{let mut d=super::super::construction::NativeOwnedDecodeControl::shared_birth_demand::<T>();d.copy_bytes+=2*size_of::<bool>()+size_of::<u8>();return Ok(d);}
  if self.cursor.terminal_is_empty(){return Ok(fixed(size_of::<u8>()));}
  self.cursor.close_demands(body)
 }
 pub fn advance(&mut self,original:RetainedCloneRef<'_,T>,control:&mut NativeDecodeControl<'_>,body:usize,grant:RetainedCloneGrant)->Result<RetainedCloneStep,ValueError>{
  let d=self.demands(original,body)?;if !retained_clone_grant_admits(grant,d){return Ok(RetainedCloneStep::Progress(Default::default()));}control.checkpoint()?;
  if let Some(p)=original.bind(&mut self.binding,grant)?{return Ok(RetainedCloneStep::Progress(p));}
  if self.phase==0{
   if !self.cursor.output_ready(){control.charge(d.capacity_bytes)?;return self.cursor.advance(original,body,grant).map(|s|RetainedCloneStep::Progress(s.progress()));}
   *self.raw=Some(self.cursor.take().ok_or_else(invalid)?);self.phase=1;return Ok(RetainedCloneStep::Progress(receipt(d)));
  }
  if self.phase==1{
   let result=control.receive_shared(&mut self.raw,&mut self.output,SharedIssuer::owned());if self.output.is_some(){self.cursor.begin_close();self.phase=2;}if result.is_err()&&self.output.is_some(){self.begin_close();}let mut p=result.map_err(|error|{let mut p=error.retained_progress();if p!=RetainedCloneProgress::default(){p.copied_bytes=p.copied_bytes.saturating_add(2*size_of::<bool>()+size_of::<u8>());}error.with_retained_progress(p)})?;p.copied_bytes=d.copy_bytes;
   if !p.fits(grant){return Err(invalid().with_retained_progress(p));}return Ok(RetainedCloneStep::Progress(p));
  }
  if self.cursor.terminal_is_empty(){self.phase=4;return Ok(RetainedCloneStep::Complete(receipt(d)));}
  control.charge(d.capacity_bytes)?;self.cursor.close_step(grant).map(|s|RetainedCloneStep::Progress(s.progress()))
 }
 pub fn take_demand(&self)->Result<RetirementDemand,ValueError>{if !self.ready(){return Err(invalid());}Ok(fixed(2*size_of::<SealedShared<T>>()+size_of::<u8>()))}
 pub fn take(&mut self,grant:RetainedCloneGrant)->Result<Option<(SealedShared<T>,RetainedCloneProgress)>,ValueError>{let d=self.take_demand()?;if !retained_clone_grant_admits(grant,d){return Ok(None);}let output=self.output.take().ok_or_else(invalid)?;self.phase=5;Ok(Some((output,receipt(d))))}
 pub fn begin_close(&mut self){self.closing=true;self.cursor.begin_close();}
 pub fn close_demands(&self,body:usize)->Result<RetirementDemand,ValueError>{
  if !self.closing{return Err(invalid());}
  if !self.cursor.terminal_is_empty(){return self.cursor.close_demands(body);}
  if !self.retirement.is_empty(){return self.retirement.next_owner_demands::<T>(false,&None,body);}
  if self.raw.is_some(){return self.retirement.next_owner_demands::<T>(true,&None,body);}
  if self.output.is_some(){return self.retirement.next_owner_demands::<SealedShared<T>>(true,&None,body);}
  RetainedCloneBinding::demands(&self.binding,body)
 }
 pub fn close_step(&mut self,grant:RetainedCloneGrant)->Result<RetainedCloneStep,ValueError>{
  let d=self.close_demands(grant.maximum_copy_bytes)?;if !retained_clone_grant_admits(grant,d){return Ok(RetainedCloneStep::Progress(Default::default()));}
  if !self.cursor.terminal_is_empty(){return self.cursor.close_step(grant).map(|s|RetainedCloneStep::Progress(s.progress()));}
  if !self.retirement.is_empty(){return self.retirement.step_granted(grant).map(|s|RetainedCloneStep::Progress(s.progress()));}
  if let Some(s)=self.retirement.begin_granted(&mut self.raw,grant)?{return Ok(RetainedCloneStep::Progress(s.progress()));}
  if let Some(s)=self.retirement.begin_granted(&mut self.output,grant)?{return Ok(RetainedCloneStep::Progress(s.progress()));}
  RetainedCloneBinding::close_one(&mut self.binding,grant)
 }
 pub fn terminal_is_empty(&self)->bool{self.binding.is_none()&&self.raw.is_none()&&self.output.is_none()&&self.retirement.is_empty()&&self.cursor.terminal_is_empty()}
}
impl<T:RetainedClone+Send+Sync+'static> Drop for NativeIntrinsicSharedField<T>{fn drop(&mut self){assert!(std::thread::panicking()||self.terminal_is_empty(),"native shared field retains original physical ownership");}}
