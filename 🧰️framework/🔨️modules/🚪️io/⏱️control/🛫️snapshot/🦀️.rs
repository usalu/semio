//! 🫴️ Native snapshot encoding borrows original allocation and independent full ownership authority.
use semio_framework_value::{NativeEncodeControl,RetainedCloneGrant};
use super::{NativeSnapshotBodyWallet,NativeSnapshotFailure};
/// 🎟️ Keeps the original caller's five-field grant immutable beside its cumulative native owner.
pub struct NativeSnapshotEncodeOwner<'owner,'control>{native:&'owner mut NativeEncodeControl<'control>,grant:RetainedCloneGrant,progress:RetainedCloneProgress,wallet:Option<&'owner mut RetainedCloneProgress>,pending:Option<&'owner mut Option<Box<dyn ErasedSnapshotRetirement>>>}
impl<'owner,'control> NativeSnapshotEncodeOwner<'owner,'control>{
 pub(super) fn borrowed_pending(native:&'owner mut NativeEncodeControl<'control>,grant:RetainedCloneGrant,wallet:&'owner mut RetainedCloneProgress,pending:Option<&'owner mut Option<Box<dyn ErasedSnapshotRetirement>>>)->Self{let mut owner=Self::borrowed(native,grant,wallet);owner.pending=pending;owner}
 pub(super) fn with_io<T,E:Into<NativeSnapshotFailure>>(&mut self,operation:impl FnOnce(&mut super::IoRunControl<'_,'control>)->RetainedWorkOutcome<T,E>)->RetainedWorkOutcome<T,NativeSnapshotFailure>{let mut run=super::IoRunControl::borrowed_encoding(self.native,self.grant,&mut self.progress,self.pending.as_deref_mut());let turn=operation(&mut run);RetainedWorkOutcome{result:turn.result.map_err(Into::into),work:turn.work}}
 pub(super) fn with_io_companion<T,E:Into<NativeSnapshotFailure>>(&mut self,decode:&mut semio_framework_value_native_owned::native_decoding::NativeDecodeControl<'control>,operation:impl FnOnce(&mut super::IoRunControl<'_,'control>)->RetainedWorkOutcome<T,E>)->RetainedWorkOutcome<T,NativeSnapshotFailure>{let mut run=super::IoRunControl::borrowed_native(decode,self.native,self.grant,&mut self.progress,self.pending.as_deref_mut(),super::IoNativeDirection::Encode);let turn=operation(&mut run);RetainedWorkOutcome{result:turn.result.map_err(Into::into),work:turn.work}}
 /// 🧲️ Requires original native authority and an independently supplied full physical grant.
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
 /// 💻️ Keeps actual parent custody and accepted receipts through original native scopes.
 pub fn scoped_native<O,E:Into<NativeSnapshotFailure>>(&mut self,maximum:usize,observer:&mut dyn FnMut(semio_framework_value::native_encoding::NativeEncodeProgress)->bool,operation:impl FnOnce(&mut NativeSnapshotEncodeOwner<'_,'_>)->RetainedWorkOutcome<O,E>)->RetainedWorkOutcome<O,NativeSnapshotFailure>{
  let grant=self.grant;let wallet=&mut self.progress;let pending=self.pending.as_deref_mut();let mut work=0;
  let result=self.native.scoped_maximum(maximum,|native|native.scoped_observer(observer,|native|{let mut original=NativeSnapshotEncodeOwner::borrowed(native,grant,wallet);original.pending=pending;let turn=operation(&mut original);work=turn.work;turn.result.map_err(Into::into)}));
  RetainedWorkOutcome{result:result.map_err(|error:NativeSnapshotFailure|error.with_retained_progress(self.progress)),work}
 }

}


impl Drop for NativeSnapshotEncodeOwner<'_, '_>{fn drop(&mut self){if let Some(wallet)=self.wallet.as_deref_mut(){*wallet=self.progress;}}}

use semio_framework_value::{ValueError,ValueRefusalKind,ErasedSnapshotRetirement,retirement::{RetireOwned,controlled::ControlledRetirement},retained_clone::{RetainedCloneStep,RetainedCloneProgress},RetainedWorkOutcome};
struct NativeSnapshotReceiving<T:RetireOwned,O:RetireOwned>{pending:std::mem::ManuallyDrop<Option<Box<dyn ErasedSnapshotRetirement>>>,intermediate:ControlledRetirement<Option<T>>,output:ControlledRetirement<Option<O>>}
impl<T:RetireOwned,O:RetireOwned> NativeSnapshotReceiving<T,O>{
 fn terminal_pending(&self)->bool{self.pending.as_ref().is_some_and(|pending|pending.terminal_is_empty())}
 fn active(&self)->&dyn ErasedSnapshotRetirement{if let Some(pending)=self.pending.as_ref(){pending.as_ref()}else if !self.intermediate.terminal_is_empty(){&self.intermediate}else{&self.output}}
}
impl<T:RetireOwned,O:RetireOwned> ErasedSnapshotRetirement for NativeSnapshotReceiving<T,O>{
    fn next_demand(&self,body:usize)->Result<semio_framework_value::RetirementDemand,semio_framework_value::ValueError>{if self.terminal_is_empty(){return Ok(Default::default())}let mut demand=if let Some(pending)=self.pending.as_ref(){semio_framework_value::factory_ticket_demands(pending,body)?}else{self.active().next_demand(body)?};if self.terminal_pending(){demand.copy_bytes=size_of::<Option<Box<dyn ErasedSnapshotRetirement>>>();}demand.depth=demand.depth.checked_add(1).ok_or_else(||ValueError::literal(ValueRefusalKind::DepthLimit,"snapshot receiving child depth overflow"))?;Ok(demand)}

 fn close_step(&mut self,grant:RetainedCloneGrant)->RetainedWorkOutcome{let mut work=0u64;let result=(||{if self.terminal_is_empty(){return Ok(RetainedCloneStep::Complete(Default::default()))}if grant.maximum_items==0{return Ok(RetainedCloneStep::Progress(Default::default()))}if grant.maximum_depth<self.next_depth_demand()?{return Err(ValueError::literal(ValueRefusalKind::DepthLimit,"snapshot receiving child exceeds parent depth"))}let mut child=grant;child.maximum_depth-=1;if self.terminal_pending(){let slot=size_of::<Option<Box<dyn ErasedSnapshotRetirement>>>();if grant.maximum_copy_bytes<slot{return Ok(RetainedCloneStep::Progress(Default::default()))}let turn=semio_framework_value::close_factory_ticket(&mut self.pending,child);work+=turn.work;let step=turn.result?;let mut progress=step.progress();if progress.copied_items>0{progress.copied_bytes=slot}Ok(RetainedCloneStep::Progress(progress))}else if self.pending.is_some(){let turn=semio_framework_value::close_factory_ticket(&mut self.pending,child);work+=turn.work;turn.result}else if !self.intermediate.terminal_is_empty(){let step=self.intermediate.step(child);work+=self.intermediate.last_turn_work();step}else{let step=self.output.step(child);work+=self.output.last_turn_work();step}})();RetainedWorkOutcome{result,work}}
 fn terminal_is_empty(&self)->bool{self.pending.is_none()&&self.intermediate.terminal_is_empty()&&self.output.terminal_is_empty()}
 fn next_copy_byte_demand(&self)->Result<usize,ValueError>{if self.terminal_pending(){return Ok(size_of::<Option<Box<dyn ErasedSnapshotRetirement>>>())}if let Some(pending)=self.pending.as_ref(){return Ok(semio_framework_value::factory_ticket_demands(pending,0)?.copy_bytes)}self.active().next_copy_byte_demand()}
 fn next_capacity_byte_demand(&self,copy:usize)->Result<usize,ValueError>{if let Some(pending)=self.pending.as_ref(){return Ok(semio_framework_value::factory_ticket_demands(pending,copy)?.capacity_bytes)}self.active().next_capacity_byte_demand(copy)}
 fn next_release_byte_demand(&self)->Result<usize,ValueError>{if let Some(pending)=self.pending.as_ref(){return Ok(semio_framework_value::factory_ticket_demands(pending,0)?.release_bytes)}self.active().next_release_byte_demand()}
 fn next_depth_demand(&self)->Result<usize,ValueError>{if self.terminal_is_empty(){return Ok(0)}let depth=if let Some(pending)=self.pending.as_ref(){semio_framework_value::factory_ticket_demands(pending,0)?.depth}else{self.active().next_depth_demand()?};depth.checked_add(1).ok_or_else(||ValueError::literal(ValueRefusalKind::DepthLimit,"snapshot receiving child depth overflow"))}
}
pub(super) trait SnapshotAdmission:Sized{
 fn admit_decode_frame(&mut self)->Result<bool,ValueError>;
 fn checkpoint(&mut self)->Result<(),ValueError>;
 fn charge(&mut self,bytes:usize)->Result<(),ValueError>;
 fn with_retirement_owner<T,E:From<ValueError>>(&mut self,bytes:usize,operation:impl FnOnce(&mut Self)->(Result<T,E>,Option<Box<dyn ErasedSnapshotRetirement>>))->Result<T,E>;
}
macro_rules! snapshot_admission{
 ($native:path,$admit:expr)=>{impl SnapshotAdmission for $native{
  fn admit_decode_frame(&mut self)->Result<bool,ValueError>{($admit)(self)}
  fn checkpoint(&mut self)->Result<(),ValueError>{<$native>::checkpoint(self)}
  fn charge(&mut self,bytes:usize)->Result<(),ValueError>{<$native>::charge(self,bytes)}
  fn with_retirement_owner<T,E:From<ValueError>>(&mut self,bytes:usize,operation:impl FnOnce(&mut Self)->(Result<T,E>,Option<Box<dyn ErasedSnapshotRetirement>>))->Result<T,E>{<$native>::with_retirement_owner(self,bytes,operation)}
 }};
}
snapshot_admission!(NativeEncodeControl<'_>,|_:&mut NativeEncodeControl<'_>|Ok(false));
snapshot_admission!(semio_framework_value_native_owned::native_decoding::NativeDecodeControl<'_>,|native:&mut semio_framework_value_native_owned::native_decoding::NativeDecodeControl<'_>|{
 native.checkpoint()?;
 if !native.prepare_work(1)?{return Err(ValueError::literal(ValueRefusalKind::WorkLimit,"snapshot receiving frame requires original work authority"))}
 native.commit_work(1)?;Ok(true)
});
#[cfg(test)]
#[path="🧪️tests/🦀️.rs"]
mod tests;
impl<'owner,'control> NativeSnapshotEncodeOwner<'owner,'control>{
 /// 🪑️ Retains the caller's static cursor and accepted output until funded retirement completes.
 pub fn drive_cursor<T:RetireOwned,O:RetireOwned,E:Into<NativeSnapshotFailure>>(&mut self,cursor:&mut Option<T>,advance:impl FnMut(&mut T,&mut NativeEncodeControl<'_>,RetainedCloneGrant)->RetainedWorkOutcome<(Option<O>,RetainedCloneProgress),E>)->RetainedWorkOutcome<O,NativeSnapshotFailure>{drive_cursor(self.native,self.grant,&mut self.progress,self.pending.as_deref_mut(),cursor,advance)}
 pub fn receive<T:RetireOwned,O:RetireOwned,E:Into<NativeSnapshotFailure>>(&mut self,operation:impl FnOnce(&mut Option<T>,&mut NativeEncodeControl<'_>,&mut NativeSnapshotBodyWallet)->RetainedWorkOutcome<O,E>)->RetainedWorkOutcome<O,NativeSnapshotFailure>{receive(self.native,self.grant,&mut self.progress,self.pending.as_deref_mut(),|slot,native,body,_pending|operation(slot,native,body))}
 pub fn receive_nested<T:RetireOwned,O:RetireOwned,E:Into<NativeSnapshotFailure>>(&mut self,operation:impl FnOnce(&mut Option<T>,&mut NativeSnapshotEncodeOwner<'_,'control>)->RetainedWorkOutcome<O,E>)->RetainedWorkOutcome<O,NativeSnapshotFailure>{
  receive(self.native,self.grant,&mut self.progress,self.pending.as_deref_mut(),|slot,native,body,pending|{let mut original=NativeSnapshotEncodeOwner::borrowed(native,body.grant,&mut body.progress);original.pending=Some(pending);operation(slot,&mut original)})
 }
}

fn publication_progress<O>(grant:RetainedCloneGrant,frame_bytes:usize)->Result<RetainedCloneProgress,ValueError>{
 let progress=RetainedCloneProgress{copied_items:1,copied_bytes:size_of::<Option<O>>(),retained_capacity_bytes:0,released_bytes:frame_bytes};
 if grant.maximum_depth==0{return Err(ValueError::literal(ValueRefusalKind::DepthLimit,"snapshot publication requires original header depth"))}
 if !progress.fits(grant){return Err(ValueError::literal(ValueRefusalKind::OwnershipLimit,"snapshot publication exceeds original physical grant"))}
 Ok(progress)
}

fn take_output<T:RetireOwned,O:RetireOwned>(frame:&mut NativeSnapshotReceiving<T,O>,grant:RetainedCloneGrant,performed:&mut RetainedCloneProgress,frame_bytes:usize)->Result<O,ValueError>{
 let publication=publication_progress::<O>(grant,frame_bytes)?;let receipt=performed.checked_add(publication)?;
 if frame.output.original().is_none_or(|output|output.is_none()){return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"snapshot output lost original ownership"))}
 *performed=receipt;Ok(frame.output.take_original().flatten().unwrap())
}


fn report_work(result:Result<(),super::NativeSnapshotFailure>,work:&mut u64,units:u64)->Result<(),super::NativeSnapshotFailure>{
 if let Some(total)=work.checked_add(units){*work=total;return result}
 let refusal=||ValueError::literal(ValueRefusalKind::WorkLimit,"native snapshot work report exceeds its original counter");
 match result{Ok(())=>Err(super::NativeSnapshotFailure::context(refusal())),Err(mut error)=>{if error.context_refusal.is_none(){error.context_refusal=Some(refusal())}Err(error)}}
}
fn post(result:Result<(),super::NativeSnapshotFailure>,driver:&mut NativeSnapshotBodyWallet,performed:&mut RetainedCloneProgress,progress:RetainedCloneProgress)->Result<(),super::NativeSnapshotFailure>{
 if result.as_ref().err().is_some_and(|error|error.receipt_refusal.is_some()){
  driver.progress=driver.progress.checked_add(progress).expect("actual original physical receipts fit address space");*performed=driver.progress();return result
 }
 let receipt=driver.record_progress(progress);*performed=driver.progress();
 match(result,receipt){(result,Ok(()))=>result,(Ok(()),Err(error))=>Err(super::NativeSnapshotFailure::receipt(error)),(Err(mut primary),Err(error))=>{primary.receipt_refusal=Some(error);Err(primary)}}
}
fn admission<T:RetireOwned,O:RetireOwned>(remaining:RetainedCloneGrant,parent:Option<&Option<Box<dyn ErasedSnapshotRetirement>>>)->Result<usize,super::NativeSnapshotFailure>{
 if !T::controlled_retirement_supported()||!O::controlled_retirement_supported(){return Err(ValueError::literal(ValueRefusalKind::UnsupportedOwner,"snapshot receiving fields require original retirement factories").into())}
 if remaining.maximum_items==0{return Err(ValueError::literal(ValueRefusalKind::WorkLimit,"snapshot receiving frame requires admitted item").into())}
 if remaining.maximum_depth<2{return Err(ValueError::literal(ValueRefusalKind::DepthLimit,"snapshot receiving frame requires parent and child depth").into())}
 let bytes=size_of::<NativeSnapshotReceiving<T,O>>();
 if bytes>remaining.maximum_capacity_bytes{return Err(ValueError::literal(ValueRefusalKind::OwnershipLimit,"snapshot receiving frame exceeds original capacity grant").into())}
 if parent.is_some_and(Option::is_some){return Err(ValueError::literal(ValueRefusalKind::OwnershipLimit,"snapshot parent retains its original refused child").into())}
 Ok(bytes)
}
fn retire_inputs<T:RetireOwned,O:RetireOwned,N:SnapshotAdmission>(frame:&mut NativeSnapshotReceiving<T,O>,native:&mut N,driver:&mut NativeSnapshotBodyWallet,performed:&mut RetainedCloneProgress,work:&mut u64)->Result<(),super::NativeSnapshotFailure>{
 while frame.pending.is_some()||!frame.intermediate.terminal_is_empty(){
  let mut grant=driver.remaining_grant();grant.maximum_depth-=1;
  if grant.maximum_items==0{return Err(ValueError::literal(ValueRefusalKind::WorkLimit,"snapshot retirement exhausted original item grant").into())}
  let closing_child=frame.pending.is_some();let header=if !closing_child&&frame.intermediate.original_is_untouched(){size_of::<Option<T>>()}else{0};
  let demand=if let Some(child)=frame.pending.as_ref(){semio_framework_value::factory_ticket_demands(child,grant.maximum_copy_bytes)?}else{semio_framework_value::RetirementDemand{copy_bytes:frame.intermediate.next_copy_byte_demand()?.checked_add(header).ok_or_else(||ValueError::literal(ValueRefusalKind::OwnershipLimit,"snapshot retirement copy overflow"))?,capacity_bytes:frame.intermediate.next_capacity_byte_demand(grant.maximum_copy_bytes)?,release_bytes:frame.intermediate.next_release_byte_demand()?,depth:frame.intermediate.next_depth_demand()?}};
  if demand.depth>grant.maximum_depth{return Err(ValueError::literal(ValueRefusalKind::DepthLimit,"snapshot retirement exceeds original depth grant").into())}
  if demand.copy_bytes>grant.maximum_copy_bytes||demand.capacity_bytes>grant.maximum_capacity_bytes||demand.release_bytes>grant.maximum_release_bytes{return Err(ValueError::literal(ValueRefusalKind::OwnershipLimit,"snapshot retirement exceeds original physical grant").into())}
  native.checkpoint()?;native.charge(demand.capacity_bytes)?;
  let turn=if closing_child{semio_framework_value::close_factory_ticket(&mut frame.pending,grant)}else{let result=frame.intermediate.step(grant);RetainedWorkOutcome{result,work:frame.intermediate.last_turn_work()}};
  let mut progress=turn.result.as_ref().map(|step|step.progress()).unwrap_or_else(|error|error.retained_progress());
  if header>0&&!frame.intermediate.original_is_untouched(){progress.copied_bytes=progress.copied_bytes.checked_add(header).ok_or_else(||ValueError::literal(ValueRefusalKind::OwnershipLimit,"snapshot retirement header copy overflow"))?;}
  let mut step=None;let result=turn.result.map(|value|{step=Some(value)}).map_err(Into::into);
  let result=report_work(result,work,turn.work);post(result,driver,performed,progress)?;
  semio_framework_value::retained_clone::admit_retained_clone_close(grant,step.unwrap(),if closing_child{frame.pending.is_none()}else{frame.intermediate.terminal_is_empty()},"snapshot receiving original")?;
  if progress==RetainedCloneProgress::default()&&turn.work==0&&(frame.pending.is_some()||!frame.intermediate.terminal_is_empty()){return Err(ValueError::literal(ValueRefusalKind::OwnershipLimit,"snapshot retirement requires a larger original grant").into())}
 }
 Ok(())
}
pub(super) fn receive<T:RetireOwned,O:RetireOwned,N:SnapshotAdmission,E:Into<super::NativeSnapshotFailure>>(native:&mut N,original_grant:RetainedCloneGrant,performed:&mut RetainedCloneProgress,parent:Option<&mut Option<Box<dyn ErasedSnapshotRetirement>>>,operation:impl FnOnce(&mut Option<T>,&mut N,&mut NativeSnapshotBodyWallet,&mut Option<Box<dyn ErasedSnapshotRetirement>>)->RetainedWorkOutcome<O,E>)->RetainedWorkOutcome<O,super::NativeSnapshotFailure>{
 let mut work=0;let mut driver=NativeSnapshotBodyWallet{grant:original_grant,progress:*performed};
 let result=(||{
  let frame_bytes=admission::<T,O>(driver.remaining_grant(),parent.as_deref())?;
  let admitted=native.admit_decode_frame()?;if admitted{work+=1}
  let construct=|native:&mut N|{
   if !admitted{work+=1}
   let mut frame=Box::new(NativeSnapshotReceiving{pending:std::mem::ManuallyDrop::new(None),intermediate:ControlledRetirement::new(None::<T>).ok().unwrap(),output:ControlledRetirement::new(None::<O>).ok().unwrap()});
   let result=(||{
    post(Ok(()),&mut driver,performed,RetainedCloneProgress{copied_items:1,retained_capacity_bytes:frame_bytes,..Default::default()})?;
    let mut grant=driver.remaining_grant();grant.maximum_depth-=1;let mut body=NativeSnapshotBodyWallet::new(grant);
    let turn=operation(frame.intermediate.original_mut().unwrap(),native,&mut body,&mut frame.pending);
    let result=turn.result.map(|output|{*frame.output.original_mut().unwrap()=Some(output)}).map_err(Into::into);
    let result=report_work(result,&mut work,turn.work);post(result,&mut driver,performed,body.progress())?;
    retire_inputs(&mut frame,native,&mut driver,performed,&mut work)?;
    let remaining=driver.remaining_grant();publication_progress::<O>(remaining,frame_bytes)?;native.checkpoint()?;
    let next=work.checked_add(1).ok_or_else(||super::NativeSnapshotFailure::context(ValueError::literal(ValueRefusalKind::WorkLimit,"native snapshot publication work overflow")))?;let output=take_output(&mut frame,remaining,performed,frame_bytes)?;work=next;
    Ok(output)
   })();
   match result{Ok(output)=>{drop(frame);(Ok(output),None)},Err(error)=>(Err(error),Some(frame as Box<dyn ErasedSnapshotRetirement>))}
  };
  if let Some(parent)=parent{native.charge(frame_bytes)?;native.checkpoint()?;let(result,pending)=construct(native);*parent=pending;result}else{native.with_retirement_owner(frame_bytes,construct)}
 })();
 RetainedWorkOutcome{result:result.map_err(|error:super::NativeSnapshotFailure|error.with_retained_progress(*performed)),work}
}
/// 🖱️ Original body work is independent of physical items, and every refusal returns its exact original cause.
pub(super) fn drive_cursor<T:RetireOwned,O:RetireOwned,N:SnapshotAdmission,E:Into<super::NativeSnapshotFailure>>(native:&mut N,original_grant:RetainedCloneGrant,performed:&mut RetainedCloneProgress,parent:Option<&mut Option<Box<dyn ErasedSnapshotRetirement>>>,cursor:&mut Option<T>,mut advance:impl FnMut(&mut T,&mut N,RetainedCloneGrant)->RetainedWorkOutcome<(Option<O>,RetainedCloneProgress),E>)->RetainedWorkOutcome<O,super::NativeSnapshotFailure>{
 let mut work=0;let mut driver=NativeSnapshotBodyWallet{grant:original_grant,progress:*performed};
 let result=(||{
  if cursor.is_none(){return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"snapshot cursor original is absent").into())}
  let frame_bytes=admission::<T,O>(driver.remaining_grant(),parent.as_deref())?;
  let admitted=native.admit_decode_frame()?;if admitted{work+=1}
  let mut construct=|native:&mut N|{
   if !admitted{work+=1}
   let mut frame=Box::new(NativeSnapshotReceiving{pending:std::mem::ManuallyDrop::new(None),intermediate:ControlledRetirement::new(None::<T>).ok().unwrap(),output:ControlledRetirement::new(None::<O>).ok().unwrap()});
   *frame.intermediate.original_mut().unwrap()=cursor.take();
   let result=(||{
    post(Ok(()),&mut driver,performed,RetainedCloneProgress{copied_items:1,retained_capacity_bytes:frame_bytes,..Default::default()})?;
    loop{
     let mut child=driver.remaining_grant();child.maximum_depth-=1;
     if child.maximum_items==0{return Err(ValueError::literal(ValueRefusalKind::WorkLimit,"snapshot cursor exhausted original item grant").into())}
     native.checkpoint()?;
     let turn=advance(frame.intermediate.original_mut().unwrap().as_mut().unwrap(),native,child);
     let mut ready=false;
     let(result,progress)=match turn.result{Ok((output,progress))=>{ready=output.is_some();if let Some(output)=output{*frame.output.original_mut().unwrap()=Some(output)}(Ok(()),progress)},Err(error)=>{let error:super::NativeSnapshotFailure=error.into();let progress=error.retained_progress();(Err(error),progress)}};
     let result=report_work(result,&mut work,turn.work);post(result,&mut driver,performed,progress)?;
     if ready{break}
     if progress==RetainedCloneProgress::default()&&turn.work==0{return Err(ValueError::literal(ValueRefusalKind::OwnershipLimit,"snapshot cursor original grant cannot admit its frontier").into())}
    }
    retire_inputs(&mut frame,native,&mut driver,performed,&mut work)?;
    let remaining=driver.remaining_grant();publication_progress::<O>(remaining,frame_bytes)?;native.checkpoint()?;
    let next=work.checked_add(1).ok_or_else(||super::NativeSnapshotFailure::context(ValueError::literal(ValueRefusalKind::WorkLimit,"native snapshot cursor publication work overflow")))?;let output=take_output(&mut frame,remaining,performed,frame_bytes)?;work=next;
    Ok(output)
   })();
   match result{Ok(output)=>{drop(frame);(Ok(output),None)},Err(error)=>(Err(error),Some(frame as Box<dyn ErasedSnapshotRetirement>))}
  };
  if let Some(parent)=parent{native.charge(frame_bytes)?;native.checkpoint()?;let(result,pending)=construct(native);*parent=pending;result}else{native.with_retirement_owner(frame_bytes,construct)}
 })();
 RetainedWorkOutcome{result:result.map_err(|error:super::NativeSnapshotFailure|error.with_retained_progress(*performed)),work}
}
