//! 🧬️ Owned field frontiers carry physical progress and actual work without changing caller authority.
use super::*;
pub use semio_framework_value_native_owned::OriginalReceivingPrimary;
/// 🧩️ Implementations construct only a pure fixed receiver header after its actual backing was admitted.
pub trait DslOwnedReceivingField:RetireOwned+Sized{type Cursor:OwnedFieldReceiving<Self>;fn receiving_cursor()->Self::Cursor;}
/// 📑️ Generated declarations retain actual typed field ownership until publication or paid cancellation.
pub trait DslOwnedReceivingRecord:DslOwnedReceivingField{}
/// 🏷️ Static variant matching selects a pure header without allocating a declaration roster.
pub trait DslOwnedReceivingVariants:RetireOwned+Sized{type Cursor:OwnedFieldReceiving<Self>;fn receiving_cursor(keyword:&str)->Result<Self::Cursor,ValueError>;}
/// ⏯️ No selected child work or native refusal may disappear in a Result-only adapter.
pub trait OwnedFieldReceiving<T:RetireOwned>:Send+'static{
 fn demands(&self,source:&Option<FieldValue>,body:usize)->Result<RetirementDemand,ValueError>;
 fn step(&mut self,source:&mut Option<FieldValue>,native:&mut NativeDecodeControl<'_>,body:usize,grant:RetainedCloneGrant)->RetainedWorkOutcome<RetainedCloneProgress,OriginalReceivingPrimary>;
 fn is_ready(&self)->bool;
 fn take_into(&mut self,output:&mut Option<T>,grant:RetainedCloneGrant)->RetainedWorkOutcome<RetainedCloneProgress>;
 fn close_demands(&self,body:usize)->Result<RetirementDemand,ValueError>;
 fn close_step(&mut self,grant:RetainedCloneGrant)->RetainedWorkOutcome<RetainedCloneProgress>;
 fn terminal_is_empty(&self)->bool;
}
/// 📏️ Moving an existing owner or fixed control header does not copy its payload.
pub fn metadata<T>()->RetirementDemand{RetirementDemand{depth:1,..Default::default()}}
/// 🥡️ A genuinely fresh scalar or retained field slot reports its actual typed payload width.
pub fn payload<T>()->RetirementDemand{RetirementDemand{copy_bytes:std::mem::size_of::<T>(),depth:1,..Default::default()}}
/// 🎟️ Every positive physical currency is checked before its defining effect.
pub fn funds(g:RetainedCloneGrant,d:RetirementDemand)->bool{g.maximum_items!=0&&g.maximum_copy_bytes>=d.copy_bytes&&g.maximum_capacity_bytes>=d.capacity_bytes&&g.maximum_release_bytes>=d.release_bytes&&g.maximum_depth>=d.depth}
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
/// 📬️ This existing-owner handoff has one actual fixed effect and no generic callback.
pub fn move_into<T>(source:&mut Option<T>,output:&mut Option<T>,g:RetainedCloneGrant)->RetainedWorkOutcome<RetainedCloneProgress>{if output.is_some()||source.is_none(){return RetainedWorkOutcome{result:Err(invariant()),work:0}}let d=metadata::<()>();if !funds(g,d){return RetainedWorkOutcome{result:Ok(Default::default()),work:0}}*output=source.take();RetainedWorkOutcome{result:Ok(receipt(d)),work:1}}
/// 🧮️ Selecting physical progress never discards the defining work returned on failure.
pub fn physical<E>(turn:RetainedWorkOutcome<RetainedCloneStep,E>)->RetainedWorkOutcome<RetainedCloneProgress,E>{RetainedWorkOutcome{result:turn.result.map(|s|s.progress()),work:turn.work}}
/// 🛂️ Value-only failures enter the complete native cause owner without formatting or cloning.
pub fn normal(turn:RetainedWorkOutcome<RetainedCloneProgress>)->RetainedWorkOutcome<RetainedCloneProgress,OriginalReceivingPrimary>{RetainedWorkOutcome{result:turn.result.map_err(Into::into),work:turn.work}}
/// 🎒️ The inline retirement cursor and its actual value remain resident through distinct paid control turns.
pub struct OwnedSlot<T:RetireOwned>{pub value:ManuallyDrop<Option<T>>,retiring:ManuallyDrop<Option<ControlledRetirement<T>>>}
impl<T:RetireOwned>Default for OwnedSlot<T>{fn default()->Self{Self{value:ManuallyDrop::new(None),retiring:ManuallyDrop::new(None)}}}
impl<T:RetireOwned>OwnedSlot<T>{
 pub fn terminal_is_empty(&self)->bool{self.value.is_none()&&self.retiring.is_none()}
 pub fn close_demands(&self,body:usize)->Result<RetirementDemand,ValueError>{match self.retiring.as_ref(){Some(owner)if !owner.terminal_is_empty()=>owner.selected_demand(body),Some(_)=>Ok(metadata::<()>()),None if self.value.is_some()=>Ok(metadata::<()>()),None=>Ok(Default::default())}}
 pub fn close_step(&mut self,g:RetainedCloneGrant)->RetainedWorkOutcome<RetainedCloneProgress>{if self.terminal_is_empty(){return RetainedWorkOutcome{result:Ok(Default::default()),work:0}}let d=match self.close_demands(g.maximum_copy_bytes){Ok(d)=>d,Err(e)=>return RetainedWorkOutcome{result:Err(e),work:0}};if !funds(g,d){return RetainedWorkOutcome{result:Ok(Default::default()),work:0}}if let Some(owner)=self.retiring.as_mut(){if owner.terminal_is_empty(){self.retiring.take();return RetainedWorkOutcome{result:Ok(receipt(d)),work:1}}let result=owner.step(g);let work=owner.last_turn_work();return RetainedWorkOutcome{result:result.map(|s|s.progress()),work}}let value=self.value.take().unwrap();match ControlledRetirement::new(value){Ok(owner)=>{*self.retiring=Some(owner);RetainedWorkOutcome{result:Ok(receipt(d)),work:1}},Err((e,value))=>{*self.value=Some(value);RetainedWorkOutcome{result:Err(e),work:1}}}}
}
impl<T:RetireOwned>Drop for OwnedSlot<T>{fn drop(&mut self){assert!(std::thread::panicking()||self.terminal_is_empty(),"owned receiving slot requires paid terminal closure");if self.terminal_is_empty(){unsafe{ManuallyDrop::drop(&mut self.value);ManuallyDrop::drop(&mut self.retiring)}}}}
/// 🪙️ Native capacity posting observes the actual selected retirement outcome, including error work.
pub fn close_source<T:RetireOwned>(owner:&mut OwnedSlot<T>,native:&mut NativeDecodeControl<'_>,g:RetainedCloneGrant)->RetainedWorkOutcome<RetainedCloneProgress,OriginalReceivingPrimary>{let turn=owner.close_step(g);let selected=RetainedWorkOutcome{result:turn.result.map(RetainedCloneStep::Progress),work:turn.work};physical(native.receive_retained_work(g,selected))}
/// 📮️ A normal child close posts its actual allocations and complete refusal before the parent selects another effect.
pub fn post_child_close(turn:RetainedWorkOutcome<RetainedCloneProgress>,native:&mut NativeDecodeControl<'_>,g:RetainedCloneGrant)->RetainedWorkOutcome<RetainedCloneProgress,OriginalReceivingPrimary>{physical(native.receive_retained_work(g,RetainedWorkOutcome{result:turn.result.map(RetainedCloneStep::Progress),work:turn.work}))}
/// 📹️ One declaration probe is bounded and exposes its actual failure work before result projection.
#[derive(Default)]pub struct DeclaredRecordShape{declaration:usize,field:usize,ready:bool,refused:bool,last_work:u64}
impl DeclaredRecordShape{
 pub fn is_ready(&self)->bool{self.ready}
 pub fn last_turn_work(&self)->u64{self.last_work}
 pub fn step(&mut self,record:&RecordValue,declared:&[(u16,bool)],native:&mut NativeDecodeControl<'_>,g:RetainedCloneGrant)->Result<RetainedCloneProgress,ValueError>{self.last_work=0;if self.ready{return Ok(Default::default())}if self.refused{return Err(invalid())}let d=metadata::<()>();if !funds(g,d){return Ok(Default::default())}native.checkpoint()?;self.last_work=1;let p=receipt(d);let result=(||{if record.fields.len()>declared.len(){return Err(invalid())}if let Some(&(id,required))=declared.get(self.declaration){let value=record.fields.at(self.field);if value.is_some_and(|(field,_)|field<id){return Err(invalid())}let present=value.filter(|(field,_)|*field==id);if required&&present.is_none_or(|(_,value)|matches!(value,FieldValue::Absent)){return Err(invalid())}if present.is_some(){self.field+=1}self.declaration+=1}if self.declaration==declared.len(){if self.field!=record.fields.len(){return Err(invalid())}self.ready=true}Ok(p)})();if result.is_err(){self.refused=true}result.map_err(|e|e.with_retained_progress(p))}
}
