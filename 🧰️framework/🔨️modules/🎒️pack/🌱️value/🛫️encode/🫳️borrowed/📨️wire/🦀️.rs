//! 📨️ One original typed source is measured and encoded by the existing Pack codec before its exact ownership returns.
use super::{BorrowedProjectedPackCursor,BorrowedProjectedPackFailure};
use semio_framework_dsl_record::native_encoding::FieldProjectionSource;
use semio_framework_value::{RetirementDemand,ValueError,ValueRefusalKind,RetainedCloneGrant,RetainedCloneProgress,RetainedCloneStep};
use semio_framework_value::retirement::{RetireOwned,controlled::ControlledRetirement};
use semio_framework_value::retained_clone::{RetainedCloneSource,RetainedCloneSourceTake};
use std::mem::ManuallyDrop;
fn header()->RetirementDemand{RetirementDemand{depth:1,..Default::default()}}
fn item()->RetainedCloneProgress{RetainedCloneProgress{copied_items:1,..Default::default()}}
fn invalid(reason:&'static str)->ValueError{ValueError::literal(ValueRefusalKind::InvariantViolated,reason)}
fn permits(grant:RetainedCloneGrant,demand:RetirementDemand)->bool{grant.maximum_items!=0&&grant.maximum_copy_bytes>=demand.copy_bytes&&grant.maximum_capacity_bytes>=demand.capacity_bytes&&grant.maximum_release_bytes>=demand.release_bytes&&grant.maximum_depth>=demand.depth}

/// 🫴️ The caller receives the identical typed owner and encoded wire after all codec symbols and source leases close.
#[derive(semio_framework_value::RetireOwned)]
pub struct OwnedProjectedPackWireReady<T:RetireOwned>{pub source:T,pub wire:Vec<u8>}
pub struct OwnedProjectedPackWireStep<T:RetireOwned>{pub ready:Option<OwnedProjectedPackWireReady<T>>,pub progress:RetainedCloneProgress}

/// 🧳️ Native source custody, symbols and partial output remain owned through every denial and cancellation.
pub struct OwnedProjectedPackWireCursor<T:FieldProjectionSource+RetireOwned+Sync>{original:ManuallyDrop<Option<T>>,source:ManuallyDrop<Option<RetainedCloneSource<T>>>,cursor:ManuallyDrop<Option<BorrowedProjectedPackCursor>>,wire:ManuallyDrop<Vec<u8>>,original_close:ManuallyDrop<Option<ControlledRetirement<T>>>,scratch:[u8;64],field_id:u16,length:usize,written:usize,phase:u8,closing:bool}
impl<T:FieldProjectionSource+RetireOwned+Sync> OwnedProjectedPackWireCursor<T>{
 /// 📥️ Moves only the already-owned original header into inline custody.
 pub fn new(original:T,field_id:u16)->Self{Self{original:ManuallyDrop::new(Some(original)),source:ManuallyDrop::new(None),cursor:ManuallyDrop::new(None),wire:ManuallyDrop::new(Vec::new()),original_close:ManuallyDrop::new(None),scratch:[0;64],field_id,length:0,written:0,phase:0,closing:false}}
 pub fn original(&self)->Option<&T>{self.original.as_ref().or_else(||self.source.as_ref().and_then(|source|source.try_borrow().ok().map(|loan|loan.get())))}
 pub fn measured_byte_length(&self)->usize{self.length}
 pub fn output_bytes_written(&self)->usize{self.written}
 pub fn phase(&self)->&'static str{if self.closing{"closing"}else{match self.phase{0=>"admittingSource",1=>"measure",2=>"measureClose",3=>"allocate",4=>"initialize",5=>"admittingEncoder",6=>"encode",7=>"encodeClose",8=>"returnSource",9=>"handoff",_=>"complete"}}}
 pub fn begin_close(&mut self){self.closing=true;}
 pub fn terminal_is_empty(&self)->bool{self.original.is_none()&&self.source.is_none()&&self.cursor.is_none()&&self.wire.capacity()==0&&self.original_close.is_none()}
 fn close_demand(&self,body:usize)->Result<RetirementDemand,ValueError>{
  if let Some(cursor)=self.cursor.as_ref(){return if cursor.terminal_is_empty(){Ok(header())}else{cursor.retirement_demands()}}
  if let Some(source)=self.source.as_ref(){return if source.terminal_is_empty(){Ok(header())}else{source.next_close_demand(body)}}
  if let Some(owner)=self.original_close.as_ref(){return if owner.terminal_is_empty(){Ok(header())}else{owner.selected_demand(body)}}
  if self.original.is_some(){return Ok(header())}Ok(RetirementDemand{release_bytes:self.wire.capacity(),depth:usize::from(self.wire.capacity()!=0),..Default::default()})
 }
 /// 📏️ Quotes the exact original source, symbol, output or ownership donation selected by this turn.
 pub fn next_demand(&self,body:usize)->Result<RetirementDemand,ValueError>{
  if self.closing{return self.close_demand(body)}
  Ok(match self.phase{
   0=>{let demand=RetainedCloneSource::<T>::owned_constructor_demand::<()>();RetirementDemand{capacity_bytes:demand.capacity_bytes,depth:demand.depth,..Default::default()}},
   1|6=>{let cursor=self.cursor.as_ref().ok_or_else(||invalid("original Pack codec is absent"))?;RetirementDemand{copy_bytes:cursor.next_minimum_copy_bytes(),capacity_bytes:cursor.next_capacity_byte_demand()?,depth:cursor.next_advance_depth_demand()?,..Default::default()}},
   2|7=>self.cursor.as_ref().map_or(Ok(header()),|cursor|if cursor.terminal_is_empty(){Ok(header())}else{cursor.retirement_demands()})?,
   3=>RetirementDemand{capacity_bytes:self.length,depth:1,..Default::default()},
   4 if self.wire.len()<self.length=>RetirementDemand{copy_bytes:usize::from(body!=0),depth:1,..Default::default()},
   4|5|9=>header(),
   8=>{let source=self.source.as_ref().ok_or_else(||invalid("original Pack source return is absent"))?;RetirementDemand{copy_bytes:source.next_take_copy_byte_demand()?,capacity_bytes:source.next_take_capacity_byte_demand(body)?,release_bytes:source.next_take_release_byte_demand()?,depth:source.next_take_depth_demand()?}},
   _=>Default::default(),
  })
 }
 fn advance_one(&mut self,grant:RetainedCloneGrant)->Result<OwnedProjectedPackWireStep<T>,BorrowedProjectedPackFailure>{
  let none=|progress|OwnedProjectedPackWireStep{ready:None,progress};let demand=self.next_demand(grant.maximum_copy_bytes)?;if !permits(grant,demand){return Ok(none(Default::default()))}if self.closing{return self.close_one(grant).map(none)}
  match self.phase{
   0=>{let original=self.original.take().ok_or_else(||invalid("original Pack input is absent"))?;match RetainedCloneSource::admit_owned(original,(),grant){Ok((source,receipt))=>{*self.source=Some(source);*self.cursor=Some(Default::default());self.phase=1;Ok(none(receipt))},Err((error,original,()))=>{*self.original=Some(original);Err(error.into())}}},
   1|6=>{
    let source=self.source.as_ref().ok_or_else(||invalid("original Pack source is absent"))?.try_borrow()?;let cursor=self.cursor.as_mut().ok_or_else(||invalid("original Pack codec is absent"))?;
    let result=if self.phase==1{cursor.advance_intrinsic(source.get(),self.field_id,&mut self.scratch,grant)}else{let end=self.written.saturating_add(64).min(self.length);cursor.advance_intrinsic(source.get(),self.field_id,&mut self.wire[self.written..end],grant)};
    match result{Ok(step)=>{if self.phase==1{self.length=self.length.checked_add(step.written_bytes).ok_or_else(||invalid("original Pack measured length overflow").with_retained_progress(step.progress))?}else{self.written=self.written.checked_add(step.written_bytes).filter(|written|*written<=self.length).ok_or_else(||invalid("original Pack source exceeded measured length").with_retained_progress(step.progress))?}if step.complete{if self.phase==6&&self.written!=self.length{return Err(invalid("original Pack source changed its measured length").with_retained_progress(step.progress).into())}self.phase+=1;}Ok(none(step.progress))},Err(original)=>{if self.phase==1{self.length=self.length.saturating_add(original.written_bytes)}else{self.written=self.written.saturating_add(original.written_bytes)}Err(original)}}
   },
   2|7=>{let cursor=self.cursor.as_mut().ok_or_else(||invalid("original Pack symbol close is absent"))?;if cursor.terminal_is_empty(){self.cursor.take();self.phase+=1;return Ok(none(item()))}cursor.close(grant).map(|step|none(step.progress))},
   3=>{self.wire.try_reserve_exact(self.length).map_err(|_|ValueError::literal(ValueRefusalKind::AllocationFailed,"original Pack wire allocation refused"))?;self.phase=4;Ok(none(RetainedCloneProgress{retained_capacity_bytes:self.wire.capacity(),..item()}))},
   4=>{if self.wire.len()==self.length{self.phase=5;return Ok(none(item()))}let count=(self.length-self.wire.len()).min(grant.maximum_copy_bytes).min(64);if count==0{return Ok(none(Default::default()))}let length=self.wire.len()+count;self.wire.resize(length,0);Ok(none(RetainedCloneProgress{copied_bytes:count,..item()}))},
   5=>{*self.cursor=Some(Default::default());self.phase=6;Ok(none(item()))},
   8=>{let source=self.source.as_mut().ok_or_else(||invalid("original Pack source return is absent"))?;match source.take_authority(grant)?{RetainedCloneSourceTake::Pending(receipt)=>Ok(none(receipt)),RetainedCloneSourceTake::Ready(original,receipt)=>{assert!(source.terminal_is_empty());self.source.take();*self.original=Some(original);self.phase=9;Ok(none(receipt))}}},
   9=>{let source=self.original.take().ok_or_else(||invalid("original Pack returned input is absent"))?;let wire=std::mem::take(&mut*self.wire);self.phase=10;Ok(OwnedProjectedPackWireStep{ready:Some(OwnedProjectedPackWireReady{source,wire}),progress:item()})},
   _=>Ok(none(Default::default())),
  }
 }
 /// ✍️ Returns every native codec cause whole while retaining original source and initialized partial bytes for paid close.
 pub fn advance(&mut self,grant:RetainedCloneGrant)->Result<OwnedProjectedPackWireStep<T>,BorrowedProjectedPackFailure>{let result=self.advance_one(grant);if result.is_err(){self.begin_close()}result}
 fn close_one(&mut self,grant:RetainedCloneGrant)->Result<RetainedCloneProgress,BorrowedProjectedPackFailure>{
  if let Some(cursor)=self.cursor.as_mut(){if cursor.terminal_is_empty(){self.cursor.take();return Ok(item())}return cursor.close(grant).map(|step|step.progress)}
  if let Some(source)=self.source.as_mut(){if source.terminal_is_empty(){self.source.take();return Ok(item())}return source.close_step(grant).map(|step|step.progress()).map_err(Into::into)}
  if let Some(owner)=self.original_close.as_mut(){if owner.terminal_is_empty(){self.original_close.take();return Ok(item())}return owner.step(grant).map(|step|step.progress()).map_err(Into::into)}
  if self.original.is_some(){let original=self.original.take().unwrap();match ControlledRetirement::new(original){Ok(owner)=>{*self.original_close=Some(owner);return Ok(item())},Err((error,original))=>{*self.original=Some(original);return Err(error.into())}}}
  if self.wire.capacity()!=0{let released_bytes=self.wire.capacity();drop(std::mem::take(&mut*self.wire));return Ok(RetainedCloneProgress{released_bytes,..item()})}Ok(Default::default())
 }
 /// ♻️ Closes the same source, symbols and partial output under the original caller's grant.
 pub fn close_step(&mut self,grant:RetainedCloneGrant)->Result<RetainedCloneStep,BorrowedProjectedPackFailure>{self.begin_close();let demand=self.close_demand(grant.maximum_copy_bytes)?;if !permits(grant,demand){return Ok(RetainedCloneStep::Progress(Default::default()))}let receipt=self.close_one(grant)?;Ok(if self.terminal_is_empty(){RetainedCloneStep::Complete(receipt)}else{RetainedCloneStep::Progress(receipt)})}
}
impl<T:FieldProjectionSource+RetireOwned+Sync> Drop for OwnedProjectedPackWireCursor<T>{fn drop(&mut self){assert!(std::thread::panicking()||self.terminal_is_empty(),"original Pack source and partial wire require paid terminal close")}}

#[derive(semio_framework_value::RetireOwned)]
struct PhysicalOwnedPack<T:RetireOwned+Sync>{original:Option<T>,source:Option<RetainedCloneSource<T>>,cursor:Option<BorrowedProjectedPackCursor>,wire:Vec<u8>,original_close:Option<ControlledRetirement<T>>}
impl<T:FieldProjectionSource+RetireOwned+Sync> RetireOwned for OwnedProjectedPackWireCursor<T>{
 fn retirement(self)->Box<dyn semio_framework_value::retirement::RetirementCursor>{let mut owner=ManuallyDrop::new(self);let physical=unsafe{PhysicalOwnedPack{original:ManuallyDrop::take(&mut owner.original),source:ManuallyDrop::take(&mut owner.source),cursor:ManuallyDrop::take(&mut owner.cursor),wire:ManuallyDrop::take(&mut owner.wire),original_close:ManuallyDrop::take(&mut owner.original_close)}};physical.retirement()}
 fn retirement_birth_bytes(&self)->Option<usize>{use semio_framework_value::retirement::{sequence_birth_bytes,deferred_birth_bytes_for};sequence_birth_bytes(&[deferred_birth_bytes_for(&*self.original),deferred_birth_bytes_for(&*self.source),deferred_birth_bytes_for(&*self.cursor),deferred_birth_bytes_for(&*self.wire),deferred_birth_bytes_for(&*self.original_close)])}
 fn controlled_retirement_supported()->bool{T::controlled_retirement_supported()}
}
