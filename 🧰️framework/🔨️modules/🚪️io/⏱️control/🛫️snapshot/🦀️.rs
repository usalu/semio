//! 🫴️ Native snapshot encoding borrows original allocation and independent full ownership authority.
use semio_framework_value::{NativeEncodeControl,RetainedCloneGrant};
use super::NativeSnapshotBodyWallet;
/// 🎟️ Keeps the original caller's five-field grant immutable beside its cumulative native owner.
pub struct NativeSnapshotEncodeOwner<'owner,'control>{native:&'owner mut NativeEncodeControl<'control>,grant:RetainedCloneGrant,progress:RetainedCloneProgress,wallet:Option<&'owner mut RetainedCloneProgress>,pending:Option<&'owner mut Option<Box<dyn ErasedSnapshotRetirement>>>}
impl<'owner,'control> NativeSnapshotEncodeOwner<'owner,'control>{
 pub(super) fn borrowed_pending(native:&'owner mut NativeEncodeControl<'control>,grant:RetainedCloneGrant,wallet:&'owner mut RetainedCloneProgress,pending:Option<&'owner mut Option<Box<dyn ErasedSnapshotRetirement>>>)->Self{let mut owner=Self::borrowed(native,grant,wallet);owner.pending=pending;owner}
 pub(super) fn with_io<T>(&mut self,operation:impl FnOnce(&mut super::IoRunControl<'_,'control>)->Result<T,ValueError>)->Result<T,ValueError>{let mut run=super::IoRunControl::borrowed_encoding(self.native,self.grant,&mut self.progress,self.pending.as_deref_mut());operation(&mut run)}
 pub(super) fn with_io_companion<T>(&mut self,decode:&mut semio_framework_value::NativeDecodeControl<'control>,operation:impl FnOnce(&mut super::IoRunControl<'_,'control>)->Result<T,ValueError>)->Result<T,ValueError>{let mut run=super::IoRunControl::borrowed_native(decode,self.native,self.grant,&mut self.progress,self.pending.as_deref_mut(),super::IoNativeDirection::Encode);operation(&mut run)}
 /// 🫴️ Requires original native authority and an independently supplied full physical grant.
 pub fn new(native:&'owner mut NativeEncodeControl<'control>,grant:RetainedCloneGrant)->Self{Self{native,grant,progress:RetainedCloneProgress::default(),wallet:None,pending:None}}
 /// 🔗️ Reborrows the caller's existing physical wallet without resetting its accepted receipts.
 pub(super) fn borrowed(native:&'owner mut NativeEncodeControl<'control>,grant:RetainedCloneGrant,wallet:&'owner mut RetainedCloneProgress)->Self{let progress=*wallet;Self{native,grant,progress,wallet:Some(wallet),pending:None}}
 /// 📏️ Returns the exact original grant without promoting descriptive demands into authority.
 pub fn grant(&self)->RetainedCloneGrant{self.grant}
 /// 🧮 Returns unspent authority from the original caller's cumulative physical receipts.
 pub fn remaining_grant(&self)->RetainedCloneGrant{RetainedCloneGrant{maximum_items:self.grant.maximum_items.saturating_sub(self.progress.copied_items),maximum_copy_bytes:self.grant.maximum_copy_bytes.saturating_sub(self.progress.copied_bytes),maximum_capacity_bytes:self.grant.maximum_capacity_bytes.saturating_sub(self.progress.retained_capacity_bytes),maximum_release_bytes:self.grant.maximum_release_bytes.saturating_sub(self.progress.released_bytes),maximum_depth:self.grant.maximum_depth}}
 /// 🧾 Observes only receipts accepted from actual owner turns.
 pub fn progress(&self)->RetainedCloneProgress{self.progress}
 /// 🔐 Refuses an overflowing or ungranted receipt before mutating the original wallet.
 pub fn record_progress(&mut self,progress:RetainedCloneProgress)->Result<(),ValueError>{let admitted=self.progress.fits(self.grant)&&progress.fits(self.remaining_grant());self.progress=self.progress.checked_add(progress).map_err(|error|error.with_retained_progress(progress))?;if !admitted{return Err(ValueError::literal(ValueRefusalKind::OwnershipLimit,"snapshot encoding receipt exceeds original grant").with_retained_progress(progress))}Ok(())}

 /// 🔭️ Reborrows the same original callback, allocation port and cumulative native receipt.
 pub fn native(&mut self)->&mut NativeEncodeControl<'control>{self.native}
 /// 🔗️ Keeps actual parent custody and accepted receipts through original native scopes.
 pub fn scoped_native<O>(&mut self,maximum:usize,observer:&mut dyn FnMut(semio_framework_value::native_encoding::NativeEncodeProgress)->bool,operation:impl FnOnce(&mut NativeSnapshotEncodeOwner<'_,'_>)->Result<O,ValueError>)->Result<O,ValueError>{let grant=self.remaining_grant();let pending=self.pending.as_deref_mut();let mut performed=RetainedCloneProgress::default();let result=self.native.scoped_maximum(maximum,|native|native.scoped_observer(observer,|native|{let mut original=NativeSnapshotEncodeOwner{native,grant,progress:Default::default(),wallet:None,pending};let result=operation(&mut original);performed=original.progress();result}));self.record_progress(performed).map_err(|error|error.with_retained_progress(performed))?;result.map_err(|error|error.with_retained_progress(performed))}

}


impl Drop for NativeSnapshotEncodeOwner<'_, '_>{fn drop(&mut self){if let Some(wallet)=self.wallet.as_deref_mut(){*wallet=self.progress;}}}

use semio_framework_value::{ValueError,ValueRefusalKind,ErasedSnapshotRetirement,retirement::{RetireOwned,controlled::ControlledRetirement},retained_clone::{RetainedCloneStep,RetainedCloneProgress}};
struct NativeSnapshotReceiving<T:RetireOwned,O:RetireOwned>{pending:std::mem::ManuallyDrop<Option<Box<dyn ErasedSnapshotRetirement>>>,intermediate:ControlledRetirement<Option<T>>,output:ControlledRetirement<Option<O>>}
impl<T:RetireOwned,O:RetireOwned> NativeSnapshotReceiving<T,O>{
 fn active(&self)->&dyn ErasedSnapshotRetirement{if let Some(pending)=self.pending.as_ref(){pending.as_ref()}else if !self.intermediate.terminal_is_empty(){&self.intermediate}else{&self.output}}
}
impl<T:RetireOwned,O:RetireOwned> ErasedSnapshotRetirement for NativeSnapshotReceiving<T,O>{
 fn close_step(&mut self,grant:RetainedCloneGrant)->Result<RetainedCloneStep,ValueError>{if self.terminal_is_empty(){return Ok(RetainedCloneStep::Complete(Default::default()))}if grant.maximum_items==0{return Ok(RetainedCloneStep::Progress(Default::default()))}if grant.maximum_depth<self.next_depth_demand()?{return Err(ValueError::literal(ValueRefusalKind::DepthLimit,"snapshot receiving child exceeds parent depth"))}let mut child=grant;child.maximum_depth-=1;if self.pending.is_some(){semio_framework_value::close_factory_ticket(&mut self.pending,child)}else if !self.intermediate.terminal_is_empty(){self.intermediate.step(child)}else{self.output.step(child)}}
 fn terminal_is_empty(&self)->bool{self.pending.is_none()&&self.intermediate.terminal_is_empty()&&self.output.terminal_is_empty()}
 fn next_copy_byte_demand(&self)->Result<usize,ValueError>{if let Some(pending)=self.pending.as_ref(){return Ok(semio_framework_value::factory_ticket_demands(pending,0)?.copy_bytes)}self.active().next_copy_byte_demand()}
 fn next_capacity_byte_demand(&self,copy:usize)->Result<usize,ValueError>{if let Some(pending)=self.pending.as_ref(){return Ok(semio_framework_value::factory_ticket_demands(pending,copy)?.capacity_bytes)}self.active().next_capacity_byte_demand(copy)}
 fn next_release_byte_demand(&self)->Result<usize,ValueError>{if let Some(pending)=self.pending.as_ref(){return Ok(semio_framework_value::factory_ticket_demands(pending,0)?.release_bytes)}self.active().next_release_byte_demand()}
 fn next_depth_demand(&self)->Result<usize,ValueError>{if self.terminal_is_empty(){return Ok(0)}let depth=if let Some(pending)=self.pending.as_ref(){semio_framework_value::factory_ticket_demands(pending,0)?.depth}else{self.active().next_depth_demand()?};depth.checked_add(1).ok_or_else(||ValueError::literal(ValueRefusalKind::DepthLimit,"snapshot receiving child depth overflow"))}
}
pub(super) trait SnapshotAdmission:Sized{
 fn checkpoint(&mut self)->Result<(),ValueError>;
 fn charge(&mut self,bytes:usize)->Result<(),ValueError>;
 fn with_retirement_owner<T>(&mut self,bytes:usize,operation:impl FnOnce(&mut Self)->(Result<T,ValueError>,Option<Box<dyn ErasedSnapshotRetirement>>))->Result<T,ValueError>;
}
macro_rules! snapshot_admission{
 ($native:path)=>{impl SnapshotAdmission for $native{
  fn checkpoint(&mut self)->Result<(),ValueError>{<$native>::checkpoint(self)}
  fn charge(&mut self,bytes:usize)->Result<(),ValueError>{<$native>::charge(self,bytes)}
  fn with_retirement_owner<T>(&mut self,bytes:usize,operation:impl FnOnce(&mut Self)->(Result<T,ValueError>,Option<Box<dyn ErasedSnapshotRetirement>>))->Result<T,ValueError>{<$native>::with_retirement_owner(self,bytes,operation)}
 }};
}
snapshot_admission!(NativeEncodeControl<'_>);
snapshot_admission!(semio_framework_value::NativeDecodeControl<'_>);
#[cfg(test)]
#[path="🧪️tests/🦀️.rs"]
mod tests;
impl<'owner,'control> NativeSnapshotEncodeOwner<'owner,'control>{
 /// 🪑️ Retains the caller's static cursor and accepted output until funded retirement completes.
 pub fn drive_cursor<T:RetireOwned,O:RetireOwned>(&mut self,cursor:&mut Option<T>,advance:impl FnMut(&mut T,&mut NativeEncodeControl<'_>,RetainedCloneGrant)->(Result<Option<O>,ValueError>,RetainedCloneProgress))->Result<O,ValueError>{
  let remaining=self.remaining_grant();let mut performed=RetainedCloneProgress::default();
  let result=drive_cursor(self.native,remaining,&mut performed,self.pending.as_deref_mut(),cursor,advance);
  self.record_progress(performed).map_err(|error|error.with_retained_progress(performed))?;
  result.map_err(|error|error.with_retained_progress(performed))
 }

 /// 🫴️ Commits output only after exact original grants retire the admitted intermediate frame.
 pub fn receive<T:RetireOwned,O:RetireOwned>(&mut self,operation:impl FnOnce(&mut Option<T>,&mut NativeEncodeControl<'_>,&mut NativeSnapshotBodyWallet)->Result<O,ValueError>)->Result<O,ValueError>{let remaining=self.remaining_grant();let mut performed=RetainedCloneProgress::default();let result=receive(self.native,remaining,&mut performed,self.pending.as_deref_mut(),|slot,native,body,_pending|operation(slot,native,body));self.record_progress(performed).map_err(|error|error.with_retained_progress(performed))?;result.map_err(|error|error.with_retained_progress(performed))}
 /// 🪆️ Composes a child's real receiving frame under the same original recipient and physical wallet.
 pub fn receive_nested<T:RetireOwned,O:RetireOwned>(&mut self,operation:impl FnOnce(&mut Option<T>,&mut NativeSnapshotEncodeOwner<'_, 'control>)->Result<O,ValueError>)->Result<O,ValueError>{let remaining=self.remaining_grant();let mut performed=RetainedCloneProgress::default();let result=receive(self.native,remaining,&mut performed,self.pending.as_deref_mut(),|slot,native,body,pending|{let mut original=NativeSnapshotEncodeOwner{native,grant:body.remaining_grant(),progress:Default::default(),wallet:None,pending:Some(pending)};let result=operation(slot,&mut original);let progress=original.progress();drop(original);body.record_progress(progress)?;result});self.record_progress(performed).map_err(|error|error.with_retained_progress(performed))?;result.map_err(|error|error.with_retained_progress(performed))}

}
fn publication_progress<O>(grant:RetainedCloneGrant,frame_bytes:usize)->Result<RetainedCloneProgress,ValueError>{
 let progress=RetainedCloneProgress{copied_items:1,copied_bytes:std::mem::size_of::<Option<O>>(),retained_capacity_bytes:0,released_bytes:frame_bytes};
 if grant.maximum_depth==0{return Err(ValueError::literal(ValueRefusalKind::DepthLimit,"snapshot publication requires original header depth"))}
 if !progress.fits(grant){return Err(ValueError::literal(ValueRefusalKind::OwnershipLimit,"snapshot publication exceeds original physical grant"))}
 Ok(progress)
}

fn take_output<T:RetireOwned,O:RetireOwned>(frame:&mut NativeSnapshotReceiving<T,O>,grant:RetainedCloneGrant,performed:&mut RetainedCloneProgress,frame_bytes:usize)->Result<O,ValueError>{
 let publication=publication_progress::<O>(grant,frame_bytes)?;let receipt=performed.checked_add(publication)?;
 if frame.output.original().is_none_or(|output|output.is_none()){return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"snapshot output lost original ownership"))}
 *performed=receipt;Ok(frame.output.take_original().flatten().unwrap())
}

pub(super) fn receive<T:RetireOwned,O:RetireOwned,N:SnapshotAdmission>(native:&mut N,original_grant:RetainedCloneGrant,performed:&mut RetainedCloneProgress,parent:Option<&mut Option<Box<dyn ErasedSnapshotRetirement>>>,operation:impl FnOnce(&mut Option<T>,&mut N,&mut NativeSnapshotBodyWallet,&mut Option<Box<dyn ErasedSnapshotRetirement>>)->Result<O,ValueError>)->Result<O,ValueError>{
  let mut grant=original_grant;let frame_bytes=std::mem::size_of::<NativeSnapshotReceiving<T,O>>();
  if !T::controlled_retirement_supported()||!O::controlled_retirement_supported(){return Err(ValueError::literal(ValueRefusalKind::UnsupportedOwner,"snapshot receiving fields require original retirement factories"))}
  if grant.maximum_items==0{return Err(ValueError::literal(ValueRefusalKind::WorkLimit,"snapshot receiving frame requires admitted item"))}
  if grant.maximum_depth<2{return Err(ValueError::literal(ValueRefusalKind::DepthLimit,"snapshot receiving frame requires parent and child depth"))}
  if frame_bytes>grant.maximum_capacity_bytes{return Err(ValueError::literal(ValueRefusalKind::OwnershipLimit,"snapshot receiving frame exceeds original capacity grant"))}
  if parent.as_ref().is_some_and(|slot|slot.is_some()){return Err(ValueError::literal(ValueRefusalKind::OwnershipLimit,"nested parent retains its original refused child"))}
  let mut construct=|native:&mut N|{
   grant.maximum_items-=1;grant.maximum_capacity_bytes-=frame_bytes;*performed=RetainedCloneProgress{copied_items:1,retained_capacity_bytes:frame_bytes,..Default::default()};
   let mut frame=Box::new(NativeSnapshotReceiving{pending:std::mem::ManuallyDrop::new(None),intermediate:ControlledRetirement::new(None::<T>).ok().unwrap(),output:ControlledRetirement::new(None::<O>).ok().unwrap()});
   grant.maximum_depth-=1;let mut body=NativeSnapshotBodyWallet::new(grant);
   let result=operation(frame.intermediate.original_mut().unwrap(),native,&mut body,&mut frame.pending);
   *performed=performed.checked_add(body.progress()).unwrap();grant=body.remaining_grant();
   let result=match result{Err(error)=>Err(error),Ok(output)=>{
    *frame.output.original_mut().unwrap()=Some(output);
    (||{
     while frame.pending.is_some()||!frame.intermediate.terminal_is_empty(){
      native.checkpoint()?;
      if grant.maximum_items==0{return Err(ValueError::literal(ValueRefusalKind::WorkLimit,"snapshot retirement exhausted original item grant"))}
      let demand=if let Some(pending)=frame.pending.as_ref(){semio_framework_value::factory_ticket_demands(pending,grant.maximum_copy_bytes)?}else{semio_framework_value::RetirementDemand{copy_bytes:frame.intermediate.next_copy_byte_demand()?,capacity_bytes:frame.intermediate.next_capacity_byte_demand(grant.maximum_copy_bytes)?,release_bytes:frame.intermediate.next_release_byte_demand()?,depth:frame.intermediate.next_depth_demand()?}};
      if demand.depth>grant.maximum_depth{return Err(ValueError::literal(ValueRefusalKind::DepthLimit,"snapshot retirement exceeds original depth grant"))}
      let copy=demand.copy_bytes;let capacity=demand.capacity_bytes;let release=demand.release_bytes;
      if copy>grant.maximum_copy_bytes||capacity>grant.maximum_capacity_bytes||release>grant.maximum_release_bytes{return Err(ValueError::literal(ValueRefusalKind::OwnershipLimit,"snapshot retirement exceeds original physical grant"))}
      native.charge(capacity)?;
      let closing_child=frame.pending.is_some();let result=if closing_child{semio_framework_value::close_factory_ticket(&mut frame.pending,grant)}else{frame.intermediate.step(grant)};let progress=match &result{Ok(step)=>step.progress(),Err(error)=>error.retained_progress()};*performed=performed.checked_add(progress)?;let step=result?;semio_framework_value::retained_clone::admit_retained_clone_close(grant,step,if closing_child{frame.pending.is_none()}else{frame.intermediate.terminal_is_empty()},"snapshot receiving original")?;
      if progress==RetainedCloneProgress::default()&&(frame.pending.is_some()||!frame.intermediate.terminal_is_empty()){return Err(ValueError::literal(ValueRefusalKind::OwnershipLimit,"snapshot retirement requires a larger original grant"))}
      grant.maximum_items=grant.maximum_items.saturating_sub(progress.copied_items);grant.maximum_copy_bytes=grant.maximum_copy_bytes.saturating_sub(progress.copied_bytes);grant.maximum_capacity_bytes=grant.maximum_capacity_bytes.saturating_sub(progress.retained_capacity_bytes);grant.maximum_release_bytes=grant.maximum_release_bytes.saturating_sub(progress.released_bytes);
     }
     publication_progress::<O>(grant,frame_bytes)?;
     native.checkpoint()?;take_output(&mut frame,grant,performed,frame_bytes)
    })()
   }};
   match result{Ok(output)=>{drop(frame);(Ok(output),None)},Err(error)=>(Err(error),Some(frame as Box<dyn ErasedSnapshotRetirement>))}
  };
  if let Some(parent)=parent{native.charge(frame_bytes)?;native.checkpoint()?;let(result,pending)=construct(native);*parent=pending;result}else{native.with_retirement_owner(frame_bytes,construct)}
}

/// 🪑️ Retains the exact typed cursor and output through either original native direction.
pub(super) fn drive_cursor<T:RetireOwned,O:RetireOwned,N:SnapshotAdmission>(native:&mut N,original_grant:RetainedCloneGrant,performed:&mut RetainedCloneProgress,parent:Option<&mut Option<Box<dyn ErasedSnapshotRetirement>>>,cursor:&mut Option<T>,mut advance:impl FnMut(&mut T,&mut N,RetainedCloneGrant)->(Result<Option<O>,ValueError>,RetainedCloneProgress))->Result<O,ValueError>{
  let remaining=original_grant;let frame_bytes=std::mem::size_of::<NativeSnapshotReceiving<T,O>>();
  if cursor.is_none(){return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"snapshot cursor original is absent"))}
  if !T::controlled_retirement_supported()||!O::controlled_retirement_supported(){return Err(ValueError::literal(ValueRefusalKind::UnsupportedOwner,"snapshot cursor fields require controlled retirement"))}
  if remaining.maximum_items==0{return Err(ValueError::literal(ValueRefusalKind::WorkLimit,"snapshot cursor frame requires one original item"))}
  if remaining.maximum_depth<2{return Err(ValueError::literal(ValueRefusalKind::DepthLimit,"snapshot cursor requires parent and child depth"))}
  if remaining.maximum_capacity_bytes<frame_bytes{return Err(ValueError::literal(ValueRefusalKind::OwnershipLimit,"snapshot cursor frame exceeds remaining capacity"))}
  if parent.as_ref().is_some_and(|slot|slot.is_some()){return Err(ValueError::literal(ValueRefusalKind::OwnershipLimit,"snapshot cursor parent retains its refused child"))}
  let mut construct=|native:&mut N|{
   let mut frame=Box::new(NativeSnapshotReceiving{pending:std::mem::ManuallyDrop::new(None),intermediate:ControlledRetirement::new(None::<T>).ok().unwrap(),output:ControlledRetirement::new(None::<O>).ok().unwrap()});
   *frame.intermediate.original_mut().unwrap()=cursor.take();
   let mut driver=NativeSnapshotBodyWallet::new(remaining);
   let result=(||->Result<O,ValueError>{
    *performed=RetainedCloneProgress{copied_items:1,retained_capacity_bytes:frame_bytes,..Default::default()};driver.record_progress(*performed)?;
    loop{
     let mut child=driver.remaining_grant();child.maximum_depth-=1;
     if child.maximum_items==0{return Err(ValueError::literal(ValueRefusalKind::WorkLimit,"snapshot cursor exhausted original work"))}
     native.checkpoint()?;
     let (result,progress)=advance(frame.intermediate.original_mut().unwrap().as_mut().unwrap(),native,child);
     let ready=match result{Ok(Some(output))=>{*frame.output.original_mut().unwrap()=Some(output);Ok(true)},Ok(None)=>Ok(false),Err(error)=>Err(error)};
     *performed=performed.checked_add(progress)?;driver.record_progress(progress)?;
     if ready?{break}
     if progress==RetainedCloneProgress::default(){return Err(ValueError::literal(ValueRefusalKind::OwnershipLimit,"snapshot cursor original grant cannot admit its frontier"))}
    }
    while !frame.intermediate.terminal_is_empty(){
     let mut child=driver.remaining_grant();child.maximum_depth-=1;
     if child.maximum_items==0{return Err(ValueError::literal(ValueRefusalKind::WorkLimit,"snapshot cursor retirement exhausted original work"))}
     if frame.intermediate.next_depth_demand()?>child.maximum_depth{return Err(ValueError::literal(ValueRefusalKind::DepthLimit,"snapshot cursor retirement exceeds original depth"))}
     let copy=frame.intermediate.next_copy_byte_demand()?;let capacity=frame.intermediate.next_capacity_byte_demand(child.maximum_copy_bytes)?;let release=frame.intermediate.next_release_byte_demand()?;
     if copy>child.maximum_copy_bytes||capacity>child.maximum_capacity_bytes||release>child.maximum_release_bytes{return Err(ValueError::literal(ValueRefusalKind::OwnershipLimit,"snapshot cursor retirement exceeds remaining physical grant"))}
     native.checkpoint()?;native.charge(capacity)?;
     let result=frame.intermediate.step(child);let progress=match &result{Ok(step)=>step.progress(),Err(error)=>error.retained_progress()};
     *performed=performed.checked_add(progress)?;driver.record_progress(progress)?;let step=result?;
     semio_framework_value::retained_clone::admit_retained_clone_close(child,step,frame.intermediate.terminal_is_empty(),"snapshot cursor original")?;
     if progress==RetainedCloneProgress::default()&&!frame.intermediate.terminal_is_empty(){return Err(ValueError::literal(ValueRefusalKind::OwnershipLimit,"snapshot cursor retirement made no funded progress"))}
    }
    let remaining=driver.remaining_grant();
    let publication=publication_progress::<O>(remaining,frame_bytes)?;
    native.checkpoint()?;
    driver.record_progress(publication)?;take_output(&mut frame,remaining,performed,frame_bytes)
   })();
   match result{Ok(output)=>{drop(frame);(Ok(output),None)},Err(error)=>(Err(error),Some(frame as Box<dyn ErasedSnapshotRetirement>))}
  };
  if let Some(parent)=parent{native.charge(frame_bytes)?;native.checkpoint()?;let(result,pending)=construct(native);*parent=pending;result}else{native.with_retirement_owner(frame_bytes,construct)}
}
