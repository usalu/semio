//! 🧬️ Owned field frontiers carry physical progress and actual work without changing caller authority.
use super::*;
pub use semio_framework_value_native_owned::OriginalReceivingPrimary;
/// 🧩️ Implementations construct only a pure fixed receiver header after its actual backing was admitted.
pub trait DslOwnedReceivingField:RetireOwned+Sized{type Cursor:OwnedFieldReceiving<Self>;fn receiving_cursor()->Self::Cursor;}
/// 📑️ Generated declarations retain actual typed field ownership until publication or paid cancellation.
pub trait DslOwnedReceivingRecord:DslOwnedReceivingField{}
/// 🏷️ Static variant matching selects a pure header without allocating a declaration roster.
pub trait DslOwnedReceivingVariants:RetireOwned+Sized{type Cursor:OwnedFieldReceiving<Self>;fn receiving_cursor(keyword:&str)->Result<Self::Cursor,ValueError>;}
/// ⏳️ A receiving turn keeps its native cause and reports the whole performed work of its one original effect.
pub type OwnedPoll=RetainedWorkPoll<RetainedCloneProgress,OriginalReceivingPrimary>;
/// 📬️ A handoff turn performs one fixed move or one defining child effect.
pub type OwnedTake=RetainedWorkPoll<RetainedCloneProgress>;
/// ♻️ A closing turn keeps every independently owned failure of its retirement effect.
pub type OwnedClose=OriginalRetirementPoll<RetainedCloneProgress>;
/// 🎯️ The selected action either runs inside one resident original effect or drives a control-owning child frame.
#[derive(Clone,Copy,Debug,PartialEq,Eq)]
pub enum OwnedAdvanceSelection{Local{work:u64},Nested}
/// ⏯️ The moved source never becomes a borrowed original, so no selected child work or native refusal may disappear in a Result-only adapter.
pub trait OwnedFieldReceiving<T:RetireOwned>:Send+'static{
 fn demands(&self,source:&Option<FieldValue>,body:usize)->Result<RetirementDemand,ValueError>;
 fn select_advance(&self,source:&Option<FieldValue>,body:usize,maximum_work_units:u64)->Result<OwnedAdvanceSelection,ValueError>;
 fn advance_nested(&mut self,source:&mut Option<FieldValue>,native:&mut NativeDecodeControl<'_>,body:usize,maximum_work_units:u64,grant:RetainedCloneGrant)->OwnedPoll{let _=(source,native,body,maximum_work_units,grant);RetainedWorkPoll::Complete(RetainedWorkOutcome{result:Err(invariant().into()),work:0})}
 fn advance_local(&mut self,source:&mut Option<FieldValue>,body:usize,effect:&mut RetainedAllocationEffect<'_>)->OwnedPoll;
 fn is_ready(&self)->bool;
 fn take_work_demand(&self)->Result<u64,ValueError>{Ok(1)}
 fn take_into(&mut self,output:&mut Option<T>,effect:&mut RetainedAllocationEffect<'_>)->OwnedTake;
 fn close_demands(&self,body:usize)->Result<RetirementDemand,ValueError>;
 fn close_work_demand(&self,body:usize)->Result<u64,ValueError>;
 fn close_step(&mut self,effect:&mut RetainedAllocationEffect<'_>)->OwnedClose;
 fn terminal_is_empty(&self)->bool;
}
/// 📏️ Moving an existing owner or fixed control header does not copy its payload.
pub fn metadata<T>()->RetirementDemand{RetirementDemand{depth:1,..Default::default()}}
/// 🥡️ A genuinely fresh scalar or retained field slot reports its actual typed payload width.
pub fn payload<T>()->RetirementDemand{RetirementDemand{copy_bytes:std::mem::size_of::<T>(),depth:1,..Default::default()}}
/// 🎟️ Every positive physical currency is checked before its defining effect.
pub fn funds(g:RetainedCloneGrant,d:RetirementDemand)->bool{g.maximum_items!=0&&g.maximum_copy_bytes>=d.copy_bytes&&g.maximum_capacity_bytes>=d.capacity_bytes&&g.maximum_release_bytes>=d.release_bytes&&g.maximum_depth>=d.depth}
/// 🪙️ The original effect must also carry at least one authored work unit before any selected action runs.
pub fn admits(effect:&RetainedAllocationEffect<'_>,d:RetirementDemand)->bool{effect.maximum_work_units()!=0&&funds(effect.grant(),d)}
/// 🧾️ Only the actual selected effect may commit its already-admitted fixed receipt.
pub fn receipt(d:RetirementDemand)->RetainedCloneProgress{RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,retained_capacity_bytes:d.capacity_bytes,released_bytes:d.release_bytes}}
/// 🪜️ Nested work stays within the same caller's structural ceiling.
pub fn nested(mut d:RetirementDemand)->Result<RetirementDemand,ValueError>{d.depth=d.depth.checked_add(1).ok_or_else(||ValueError::literal(ValueRefusalKind::DepthLimit,"owned receiving child depth overflow"))?;Ok(d)}
/// 📉️ No child depth view changes items or any byte currency.
pub fn child_grant(g:RetainedCloneGrant)->RetainedCloneGrant{RetainedCloneGrant{maximum_depth:g.maximum_depth.saturating_sub(1),..g}}
/// 🚫️ Literal shape refusal allocates no denied error payload.
pub fn invalid()->ValueError{ValueError::literal(ValueRefusalKind::InvalidValue,"owned receiving source differs from its declared field")}
/// 🧯️ An occupied or foreign destination retains the genuine source unchanged.
pub fn invariant()->ValueError{ValueError::literal(ValueRefusalKind::InvariantViolated,"owned receiving source or destination is already occupied")}
/// 🧱️ A selected action whose work the effect cannot fund fails typed before it runs.
pub fn work_limit()->ValueError{ValueError::literal(ValueRefusalKind::WorkLimit,"owned receiving action exceeded its original work")}
/// ⏱️ Records one defining local action in the original receipt before it runs.
pub fn perform(effect:&mut RetainedAllocationEffect<'_>)->Result<(),ValueError>{if effect.performed(1)?{Ok(())}else{Err(work_limit())}}
/// 🧮️ A grant-priced child records only the remainder it reports beyond the receipt, so each unit is charged once.
pub fn owe(effect:&mut RetainedAllocationEffect<'_>,work:u128)->Result<(),ValueError>{let owed=u64::try_from(work.saturating_sub(effect.performed_work())).map_err(|_|ValueError::invariant("owned receiving child work overflow"))?;if owed==0||effect.performed(owed)?{Ok(())}else{Err(work_limit())}}
/// 🏁️ Every turn reports the whole work its one original effect performed, children included.
pub fn done<T,E>(effect:&RetainedAllocationEffect<'_>,result:Result<T,E>)->RetainedWorkPoll<T,E>{RetainedWorkPoll::Complete(RetainedWorkOutcome{result,work:effect.performed_work()})}
/// 💤️ A denied turn performs nothing and retains every source.
pub fn idle<E>(effect:&RetainedAllocationEffect<'_>)->RetainedWorkPoll<RetainedCloneProgress,E>{done(effect,Ok(Default::default()))}
/// 🧷️ A delegated child reports the receipt total; Pending keeps its newly performed work.
pub fn settle<T,E>(effect:&RetainedAllocationEffect<'_>,poll:RetainedWorkPoll<T,E>)->RetainedWorkPoll<T,E>{match poll{RetainedWorkPoll::Pending{..}=>RetainedWorkPoll::Pending{work:effect.performed_work()},RetainedWorkPoll::Complete(turn)=>RetainedWorkPoll::Complete(RetainedWorkOutcome{result:turn.result,work:effect.performed_work()})}}
/// 🔖️ Bookkeeping follows only a completed successful child turn.
pub fn after<T,E>(effect:&RetainedAllocationEffect<'_>,poll:RetainedWorkPoll<T,E>,on_complete:impl FnOnce(&T))->RetainedWorkPoll<T,E>{if let RetainedWorkPoll::Complete(RetainedWorkOutcome{result:Ok(value),..})=&poll{on_complete(value)}settle(effect,poll)}
/// ☠️ A failure after performed work poisons the cursor; a pre-effect refusal stays retryable.
pub fn poison<T,E>(failed:&mut bool,poll:&RetainedWorkPoll<T,E>){if let RetainedWorkPoll::Complete(RetainedWorkOutcome{result:Err(_),work})=poll{if *work>0{*failed=true}}}
/// 🧮️ Selecting physical progress never discards the defining work returned on failure.
pub fn physical<E>(poll:RetainedWorkPoll<RetainedCloneStep,E>)->RetainedWorkPoll<RetainedCloneProgress,E>{poll.map(|step|step.progress())}
/// 🛂️ Value-only failures enter the complete native cause owner without formatting or cloning.
pub fn normal(poll:OwnedTake)->OwnedPoll{poll.map_err(Into::into)}
/// 🧯️ A child that closes during a receiving turn keeps its retirement causes inside the complete native primary.
pub fn receiving(poll:OwnedClose)->OwnedPoll{poll.map_err(Into::into)}
/// 🔁️ A closing child keeps its independently owned failures.
pub fn closing(poll:OwnedTake)->OwnedClose{poll.map_err(Into::into)}
/// ⏱️ One pure local action records its single work unit, runs, and reports the receipt total.
pub fn local(effect:&mut RetainedAllocationEffect<'_>,d:RetirementDemand,action:impl FnOnce()->Result<(),ValueError>)->OwnedPoll{let result=perform(effect).and_then(|()|action()).map(|()|receipt(d)).map_err(|error|OriginalReceivingPrimary::from(error.with_retained_progress(receipt(metadata::<()>()))));done(effect,result)}
/// ⏱️ One pure closing action records its single work unit before it runs.
pub fn local_close(effect:&mut RetainedAllocationEffect<'_>,d:RetirementDemand,action:impl FnOnce()->Result<(),ValueError>)->OwnedClose{let result=perform(effect).and_then(|()|action()).map(|()|receipt(d)).map_err(Into::into);done(effect,result)}
/// 📬️ This existing-owner handoff has one actual fixed effect and no generic callback.
pub fn move_into<T>(source:&mut Option<T>,output:&mut Option<T>,effect:&mut RetainedAllocationEffect<'_>)->OwnedTake{if output.is_some()||source.is_none(){return done(effect,Err(invariant()))}let d=metadata::<()>();if !admits(effect,d){return idle(effect)}let result=perform(effect).map(|()|{*output=source.take();receipt(d)});done(effect,result)}
/// 🗂️ A vector backing is allocated empty with the exact capacity recorded in the original marker.
pub fn allocate_empty_vec<T>(count:usize,actual:&mut Option<usize>)->Result<Vec<T>,ValueError>{
 if actual.is_some(){return Err(invariant())}if count==0||std::mem::size_of::<T>()==0{return Ok(Vec::new())}
 let mut vector=Vec::new();*actual=Some(0);vector.try_reserve_exact(count).map_err(|_|ValueError::literal(ValueRefusalKind::AllocationFailed,"owned receiving vector backing allocation refused"))?;*actual=Some(vector.capacity()*std::mem::size_of::<T>());Ok(vector)
}
/// 🎒️ The inline retirement cursor and its actual value remain resident through distinct paid control turns.
pub struct OwnedSlot<T:RetireOwned>{pub value:ManuallyDrop<Option<T>>,retiring:ManuallyDrop<Option<ControlledRetirement<T>>>}
impl<T:RetireOwned>Default for OwnedSlot<T>{fn default()->Self{Self{value:ManuallyDrop::new(None),retiring:ManuallyDrop::new(None)}}}
impl<T:RetireOwned>OwnedSlot<T>{
 pub fn terminal_is_empty(&self)->bool{self.value.is_none()&&self.retiring.is_none()}
 pub fn close_demands(&self,body:usize)->Result<RetirementDemand,ValueError>{match self.retiring.as_ref(){Some(owner)if !owner.terminal_is_empty()=>owner.selected_demand(body),Some(_)=>Ok(metadata::<()>()),None if self.value.is_some()=>Ok(metadata::<()>()),None=>Ok(Default::default())}}
 pub fn close_work_demand(&self,body:usize)->Result<u64,ValueError>{match self.retiring.as_ref(){Some(owner)if !owner.terminal_is_empty()=>owner.next_computational_work(body),Some(_)=>Ok(1),None=>Ok(u64::from(self.value.is_some()))}}
 pub fn close_step(&mut self,effect:&mut RetainedAllocationEffect<'_>)->OwnedClose{
  if self.terminal_is_empty(){return idle(effect)}
  let d=match self.close_demands(effect.grant().maximum_copy_bytes){Ok(d)=>d,Err(error)=>return done(effect,Err(error.into()))};if !admits(effect,d){return idle(effect)}
  if let Some(owner)=self.retiring.as_mut(){if owner.terminal_is_empty(){let result=perform(effect).map(|()|{self.retiring.take();receipt(d)}).map_err(Into::into);return done(effect,result)}let poll=owner.step(effect);return settle(effect,physical(poll))}
  if let Err(error)=perform(effect){return done(effect,Err(error.into()))}
  let value=match self.value.take(){Some(value)=>value,None=>return done(effect,Err(ValueError::invariant("value absent").into()))};
  match ControlledRetirement::new(value){Ok(owner)=>{*self.retiring=Some(owner);done(effect,Ok(receipt(d)))},Err((error,value))=>{*self.value=Some(value);done(effect,Err(error.into()))}}
 }
}
impl<T:RetireOwned>Drop for OwnedSlot<T>{fn drop(&mut self){assert!(std::thread::panicking()||self.terminal_is_empty(),"owned receiving slot requires paid terminal closure");if self.terminal_is_empty(){unsafe{ManuallyDrop::drop(&mut self.value);ManuallyDrop::drop(&mut self.retiring)}}}}
/// 📹️ One declaration probe is bounded and checks the declared fields before any effect.
#[derive(Default)]pub struct DeclaredRecordShape{declaration:usize,field:usize,ready:bool,refused:bool}
impl DeclaredRecordShape{
 pub fn is_ready(&self)->bool{self.ready}
 pub fn check(&mut self,record:&RecordValue,declared:&[(u16,bool)])->Result<(),ValueError>{
  if self.ready{return Ok(())}if self.refused{return Err(invalid())}
  let result=(||{if record.fields.len()>declared.len(){return Err(invalid())}if let Some(&(id,required))=declared.get(self.declaration){let value=record.fields.at(self.field);if value.is_some_and(|(field,_)|field<id){return Err(invalid())}let present=value.filter(|(field,_)|*field==id);if required&&present.is_none_or(|(_,value)|matches!(value,FieldValue::Absent)){return Err(invalid())}if present.is_some(){self.field+=1}self.declaration+=1}if self.declaration==declared.len(){if self.field!=record.fields.len(){return Err(invalid())}self.ready=true}Ok(())})();
  if result.is_err(){self.refused=true}result
 }
 pub fn step(&mut self,record:&RecordValue,declared:&[(u16,bool)],effect:&mut RetainedAllocationEffect<'_>)->Result<RetainedCloneProgress,ValueError>{
  if self.ready{return Ok(Default::default())}if self.refused{return Err(invalid())}let d=metadata::<()>();if !admits(effect,d){return Ok(Default::default())}
  perform(effect)?;let p=receipt(d);self.check(record,declared).map(|()|p).map_err(|error|error.with_retained_progress(p))
 }
}
