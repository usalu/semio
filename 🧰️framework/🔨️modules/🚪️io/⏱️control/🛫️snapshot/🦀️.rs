//! 🫴️ Native snapshot encoding borrows original allocation and independent full ownership authority.
use semio_framework_value::{NativeEncodeControl,RetainedCloneGrant};
use super::NativeSnapshotBodyWallet;
/// 🎟️ Keeps the original caller's five-field grant immutable beside its cumulative native owner.
pub struct NativeSnapshotEncodeOwner<'owner,'control>{native:&'owner mut NativeEncodeControl<'control>,grant:RetainedCloneGrant,progress:RetainedCloneProgress,wallet:Option<&'owner mut RetainedCloneProgress>,pending:Option<&'owner mut Option<Box<dyn ErasedSnapshotRetirement>>>}
impl<'owner,'control> NativeSnapshotEncodeOwner<'owner,'control>{
 /// 🫴️ Requires original native authority and an independently supplied full physical grant.
 pub fn new(native:&'owner mut NativeEncodeControl<'control>,grant:RetainedCloneGrant)->Self{Self{native,grant,progress:RetainedCloneProgress::default(),wallet:None,pending:None}}
 /// 🔗️ Reborrows the caller's existing physical wallet without resetting its accepted receipts.
 pub(super) fn borrowed(native:&'owner mut NativeEncodeControl<'control>,grant:RetainedCloneGrant,wallet:&'owner mut RetainedCloneProgress)->Self{let progress=*wallet;Self{native,grant,progress,wallet:Some(wallet),pending:None}}
 /// 📏️ Returns the exact original grant without promoting descriptive demands into authority.
 pub fn grant(&self)->RetainedCloneGrant{self.grant}
 /// 🧮 Returns unspent authority from the original caller's cumulative physical receipts.
 pub fn remaining_grant(&self)->RetainedCloneGrant{RetainedCloneGrant{maximum_items:self.grant.maximum_items-self.progress.copied_items,maximum_copy_bytes:self.grant.maximum_copy_bytes-self.progress.copied_bytes,maximum_capacity_bytes:self.grant.maximum_capacity_bytes-self.progress.retained_capacity_bytes,maximum_release_bytes:self.grant.maximum_release_bytes-self.progress.released_bytes,maximum_depth:self.grant.maximum_depth}}
 /// 🧾 Observes only receipts accepted from actual owner turns.
 pub fn progress(&self)->RetainedCloneProgress{self.progress}
 /// 🔐 Refuses an overflowing or ungranted receipt before mutating the original wallet.
 pub fn record_progress(&mut self,progress:RetainedCloneProgress)->Result<(),ValueError>{if !progress.fits(self.remaining_grant()){return Err(ValueError::literal(ValueRefusalKind::OwnershipLimit,"snapshot encoding receipt exceeds original grant"))}let cumulative=self.progress.checked_add(progress)?;self.progress=cumulative;Ok(())}

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
 fn next_copy_byte_demand(&self)->Result<usize,ValueError>{self.active().next_copy_byte_demand()}
 fn next_capacity_byte_demand(&self,copy:usize)->Result<usize,ValueError>{self.active().next_capacity_byte_demand(copy)}
 fn next_release_byte_demand(&self)->Result<usize,ValueError>{if let Some(pending)=self.pending.as_ref(){if pending.terminal_is_empty(){return Ok(std::mem::size_of_val(pending.as_ref()))}}self.active().next_release_byte_demand()}
 fn next_depth_demand(&self)->Result<usize,ValueError>{if self.terminal_is_empty(){return Ok(0)}if self.pending.as_ref().is_some_and(|pending|pending.terminal_is_empty()){return Ok(2)}self.active().next_depth_demand()?.checked_add(1).ok_or_else(||ValueError::literal(ValueRefusalKind::DepthLimit,"snapshot receiving child depth overflow"))}
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
impl NativeSnapshotEncodeOwner<'_, '_>{
 /// 🪑️ Retains the caller's static cursor and accepted output until funded retirement completes.
 pub fn drive_cursor<T:RetireOwned,O:RetireOwned>(&mut self,cursor:&mut Option<T>,mut advance:impl FnMut(&mut T,&mut NativeEncodeControl<'_>,RetainedCloneGrant)->(Result<Option<O>,ValueError>,RetainedCloneProgress))->Result<O,ValueError>{
  let remaining=self.remaining_grant();let frame_bytes=std::mem::size_of::<NativeSnapshotReceiving<T,O>>();
  if cursor.is_none(){return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"snapshot cursor original is absent"))}
  if !T::controlled_retirement_supported()||!O::controlled_retirement_supported(){return Err(ValueError::literal(ValueRefusalKind::UnsupportedOwner,"snapshot cursor fields require controlled retirement"))}
  if remaining.maximum_items==0{return Err(ValueError::literal(ValueRefusalKind::WorkLimit,"snapshot cursor frame requires one original item"))}
  if remaining.maximum_depth<2{return Err(ValueError::literal(ValueRefusalKind::DepthLimit,"snapshot cursor requires parent and child depth"))}
  if remaining.maximum_capacity_bytes<frame_bytes{return Err(ValueError::literal(ValueRefusalKind::OwnershipLimit,"snapshot cursor frame exceeds remaining capacity"))}
  if self.pending.as_ref().is_some_and(|slot|slot.is_some()){return Err(ValueError::literal(ValueRefusalKind::OwnershipLimit,"snapshot cursor parent retains its refused child"))}
  let mut performed=RetainedCloneProgress::default();
  let mut construct=|native:&mut NativeEncodeControl<'_>|{
   let mut frame=Box::new(NativeSnapshotReceiving{pending:std::mem::ManuallyDrop::new(None),intermediate:ControlledRetirement::new(None::<T>).ok().unwrap(),output:ControlledRetirement::new(None::<O>).ok().unwrap()});
   *frame.intermediate.original_mut().unwrap()=cursor.take();
   let mut driver=NativeSnapshotEncodeOwner::new(native,remaining);
   let result=(||->Result<O,ValueError>{
    driver.record_progress(RetainedCloneProgress{copied_items:1,retained_capacity_bytes:frame_bytes,..Default::default()})?;
    loop{
     let mut child=driver.remaining_grant();child.maximum_depth-=1;
     if child.maximum_items==0{return Err(ValueError::literal(ValueRefusalKind::WorkLimit,"snapshot cursor exhausted original work"))}
     driver.native().checkpoint()?;
     let (result,progress)=advance(frame.intermediate.original_mut().unwrap().as_mut().unwrap(),driver.native(),child);
     let ready=match result{Ok(Some(output))=>{*frame.output.original_mut().unwrap()=Some(output);Ok(true)},Ok(None)=>Ok(false),Err(error)=>Err(error)};
     driver.record_progress(progress)?;
     if ready?{break}
     if progress==RetainedCloneProgress::default(){return Err(ValueError::literal(ValueRefusalKind::OwnershipLimit,"snapshot cursor original grant cannot admit its frontier"))}
    }
    while !frame.intermediate.terminal_is_empty(){
     let mut child=driver.remaining_grant();child.maximum_depth-=1;
     if child.maximum_items==0{return Err(ValueError::literal(ValueRefusalKind::WorkLimit,"snapshot cursor retirement exhausted original work"))}
     if frame.intermediate.next_depth_demand()?>child.maximum_depth{return Err(ValueError::literal(ValueRefusalKind::DepthLimit,"snapshot cursor retirement exceeds original depth"))}
     let copy=frame.intermediate.next_copy_byte_demand()?;let capacity=frame.intermediate.next_capacity_byte_demand(child.maximum_copy_bytes)?;let release=frame.intermediate.next_release_byte_demand()?;
     if copy>child.maximum_copy_bytes||capacity>child.maximum_capacity_bytes||release>child.maximum_release_bytes{return Err(ValueError::literal(ValueRefusalKind::OwnershipLimit,"snapshot cursor retirement exceeds remaining physical grant"))}
     driver.native().checkpoint()?;driver.native().charge(capacity)?;
     let result=frame.intermediate.step(child);let progress=match &result{Ok(step)=>step.progress(),Err(error)=>error.retained_progress()};
     driver.record_progress(progress)?;let step=result?;
     semio_framework_value::retained_clone::admit_retained_clone_close(child,step,frame.intermediate.terminal_is_empty(),"snapshot cursor original")?;
     if progress==RetainedCloneProgress::default()&&!frame.intermediate.terminal_is_empty(){return Err(ValueError::literal(ValueRefusalKind::OwnershipLimit,"snapshot cursor retirement made no funded progress"))}
    }
    let remaining=driver.remaining_grant();
    if remaining.maximum_items==0||remaining.maximum_release_bytes<frame_bytes{return Err(ValueError::literal(ValueRefusalKind::OwnershipLimit,"snapshot cursor publication requires original frame release"))}
    driver.native().checkpoint()?;
    let output=frame.output.take_original().flatten().ok_or_else(||ValueError::literal(ValueRefusalKind::InvariantViolated,"snapshot cursor output disappeared"))?;
    driver.record_progress(RetainedCloneProgress{copied_items:1,released_bytes:frame_bytes,..Default::default()})?;Ok(output)
   })();
   performed=driver.progress();
   match result{Ok(output)=>{drop(frame);(Ok(output),None)},Err(error)=>(Err(error),Some(frame as Box<dyn ErasedSnapshotRetirement>))}
  };
  let result=if let Some(parent)=self.pending.as_deref_mut(){self.native.charge(frame_bytes)?;self.native.checkpoint()?;let(result,pending)=construct(self.native);*parent=pending;result}else{self.native.with_retirement_owner(frame_bytes,construct)};
  self.record_progress(performed).map_err(|error|error.with_retained_progress(performed))?;
  result.map_err(|error|error.with_retained_progress(performed))
 }

 /// 🫴️ Commits output only after exact original grants retire the admitted intermediate frame.
 pub fn receive<T:RetireOwned,O:RetireOwned>(&mut self,operation:impl FnOnce(&mut Option<T>,&mut NativeEncodeControl<'_>,&mut NativeSnapshotBodyWallet)->Result<O,ValueError>)->Result<O,ValueError>{let remaining=self.remaining_grant();let mut performed=RetainedCloneProgress::default();let result=receive(self.native,remaining,&mut performed,self.pending.as_deref_mut(),|slot,native,body,_pending|operation(slot,native,body));self.record_progress(performed).map_err(|error|error.with_retained_progress(performed))?;result.map_err(|error|error.with_retained_progress(performed))}
 /// 🪆️ Composes a child's real receiving frame under the same original recipient and physical wallet.
 pub fn receive_nested<T:RetireOwned,O:RetireOwned>(&mut self,operation:impl FnOnce(&mut Option<T>,&mut NativeSnapshotEncodeOwner<'_, '_>)->Result<O,ValueError>)->Result<O,ValueError>{let remaining=self.remaining_grant();let mut performed=RetainedCloneProgress::default();let result=receive(self.native,remaining,&mut performed,self.pending.as_deref_mut(),|slot,native,body,pending|{let mut original=NativeSnapshotEncodeOwner{native,grant:body.remaining_grant(),progress:Default::default(),wallet:None,pending:Some(pending)};let result=operation(slot,&mut original);let progress=original.progress();drop(original);body.record_progress(progress)?;result});self.record_progress(performed).map_err(|error|error.with_retained_progress(performed))?;result.map_err(|error|error.with_retained_progress(performed))}

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
      if frame.active().next_depth_demand()?>grant.maximum_depth{return Err(ValueError::literal(ValueRefusalKind::DepthLimit,"snapshot retirement exceeds original depth grant"))}
      let copy=frame.active().next_copy_byte_demand()?;let capacity=frame.active().next_capacity_byte_demand(grant.maximum_copy_bytes)?;let release=if frame.pending.as_ref().is_some_and(|pending|pending.terminal_is_empty()){std::mem::size_of_val(frame.pending.as_ref().unwrap().as_ref())}else{frame.active().next_release_byte_demand()?};
      if copy>grant.maximum_copy_bytes||capacity>grant.maximum_capacity_bytes||release>grant.maximum_release_bytes{return Err(ValueError::literal(ValueRefusalKind::OwnershipLimit,"snapshot retirement exceeds original physical grant"))}
      native.charge(capacity)?;
      let closing_child=frame.pending.is_some();let result=if closing_child{semio_framework_value::close_factory_ticket(&mut frame.pending,grant)}else{frame.intermediate.step(grant)};let progress=match &result{Ok(step)=>step.progress(),Err(error)=>error.retained_progress()};*performed=performed.checked_add(progress)?;let step=result?;semio_framework_value::retained_clone::admit_retained_clone_close(grant,step,if closing_child{frame.pending.is_none()}else{frame.intermediate.terminal_is_empty()},"snapshot receiving original")?;
      if progress==RetainedCloneProgress::default()&&(frame.pending.is_some()||!frame.intermediate.terminal_is_empty()){return Err(ValueError::literal(ValueRefusalKind::OwnershipLimit,"snapshot retirement requires a larger original grant"))}
      grant.maximum_items=grant.maximum_items.saturating_sub(progress.copied_items);grant.maximum_copy_bytes=grant.maximum_copy_bytes.saturating_sub(progress.copied_bytes);grant.maximum_capacity_bytes=grant.maximum_capacity_bytes.saturating_sub(progress.retained_capacity_bytes);grant.maximum_release_bytes=grant.maximum_release_bytes.saturating_sub(progress.released_bytes);
     }
     if grant.maximum_items==0||frame_bytes>grant.maximum_release_bytes{return Err(ValueError::literal(ValueRefusalKind::OwnershipLimit,"snapshot publication requires original frame release grant"))}
     native.checkpoint()?;let output=frame.output.take_original().flatten().ok_or_else(||ValueError::literal(ValueRefusalKind::InvariantViolated,"snapshot output lost original ownership"))?;*performed=performed.checked_add(RetainedCloneProgress{copied_items:1,released_bytes:frame_bytes,..Default::default()})?;Ok(output)
    })()
   }};
   match result{Ok(output)=>{drop(frame);(Ok(output),None)},Err(error)=>(Err(error),Some(frame as Box<dyn ErasedSnapshotRetirement>))}
  };
  if let Some(parent)=parent{native.charge(frame_bytes)?;native.checkpoint()?;let(result,pending)=construct(native);*parent=pending;result}else{native.with_retirement_owner(frame_bytes,construct)}
}
