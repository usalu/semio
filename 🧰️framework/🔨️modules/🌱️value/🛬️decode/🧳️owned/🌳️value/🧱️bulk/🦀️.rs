//! 🧱️ A heterogeneous dynamic subtree is quoted exactly, then read in one funded step under the caller's allocation authority.
use super::{NativeValueSource,fixed,invalid,receipt};
use crate::{DslValue,NativeDecodeControl,RetirementDemand,ValueError,ValueRefusalKind,retirement::RetireOwned,retained_clone::{RetainedCloneBinding,RetainedCloneClose,RetainedCloneGrant,RetainedCloneProgress,RetainedCloneRef,RetainedCloneStep,retained_clone_grant_admits}};
use super::super::{NativeReceive,NativeReceiver};
use std::mem::{ManuallyDrop,size_of};

/// 🌲️ Maximum nesting a dynamic subtree may carry before the quote refuses it.
pub const NATIVE_BULK_MAXIMUM_DEPTH:usize=64;

/// 📜️ One dynamic kind selected from the actual original source without any intermediate owner.
#[derive(Clone,Copy,PartialEq,Eq)]
pub enum NativeBulkKind{Null,Bool,Number,Text,Array,Object}

/// 🔍️ Classifies one original node by the single accessor its shape admits.
pub fn native_bulk_kind(original:&dyn NativeValueSource)->Result<NativeBulkKind,ValueError>{
 if original.is_null(){return Ok(NativeBulkKind::Null)}
 if original.boolean().is_ok(){return Ok(NativeBulkKind::Bool)}
 if original.number().is_ok(){return Ok(NativeBulkKind::Number)}
 if original.text().is_ok(){return Ok(NativeBulkKind::Text)}
 if original.sequence_len().is_ok(){return Ok(NativeBulkKind::Array)}
 if original.record_len().is_ok(){return Ok(NativeBulkKind::Object)}
 Err(invalid())
}

/// 📏️ Adds the exact owned bytes one dynamic subtree costs, recursing through children with the caller's array and entry slot widths.
pub fn native_bulk_measure(original:&dyn NativeValueSource,depth:usize,array_slot:usize,entry_slot:usize,capacity:&mut usize)->Result<(),ValueError>{
 if depth>NATIVE_BULK_MAXIMUM_DEPTH{return Err(ValueError::literal(ValueRefusalKind::DepthLimit,"native dynamic subtree exceeds depth limit"))}
 let add=|capacity:&mut usize,bytes:usize|->Result<(),ValueError>{*capacity=capacity.checked_add(bytes).ok_or_else(invalid)?;Ok(())};
 match native_bulk_kind(original)?{
  NativeBulkKind::Null|NativeBulkKind::Bool|NativeBulkKind::Number=>Ok(()),
  NativeBulkKind::Text=>add(capacity,original.text()?.len()),
  NativeBulkKind::Array=>{let count=original.sequence_len()?;add(capacity,count.checked_mul(array_slot).ok_or_else(invalid)?)?;for index in 0..count{native_bulk_measure(original.sequence_child(index)?,depth+1,array_slot,entry_slot,capacity)?}Ok(())}
  NativeBulkKind::Object=>{let count=original.record_len()?;add(capacity,count.checked_mul(entry_slot).ok_or_else(invalid)?)?;for index in 0..count{add(capacity,original.record_name(index)?.len())?;native_bulk_measure(original.record_child(index)?,depth+1,array_slot,entry_slot,capacity)?}Ok(())}
 }
}

/// 🧱️ A type whose whole original subtree is read once; `read_demand` quotes the exact owned bytes before any allocation.
pub trait NativeBulkValue<I:NativeValueSource+?Sized>:RetireOwned+Sized{
 fn read_demand(original:&I)->Result<RetirementDemand,ValueError>;
 fn read(original:&I,control:&mut NativeDecodeControl<'_>)->Result<Self,ValueError>;
}

/// 🫴️ The bulk receiver holds the original binding, the read output and funded closure like every other receiver.
pub struct NativeBulkReceiver<I:NativeValueSource+?Sized,T:NativeBulkValue<I>>{binding:Option<RetainedCloneBinding>,output:ManuallyDrop<Option<T>>,retirement:RetainedCloneClose,closing:bool,taken:bool,input:std::marker::PhantomData<fn(&I)>}
impl<I:NativeValueSource+?Sized,T:NativeBulkValue<I>> NativeBulkReceiver<I,T>{
 pub fn birth()->RetirementDemand{fixed(size_of::<Self>())}
 pub fn begin(g:RetainedCloneGrant)->Result<(Self,RetainedCloneProgress),ValueError>{let d=Self::birth();if !retained_clone_grant_admits(g,d){return Err(invalid())}Ok((Self{binding:None,output:ManuallyDrop::new(None),retirement:Default::default(),closing:false,taken:false,input:std::marker::PhantomData},receipt(d)))}
}
impl<I:NativeValueSource+?Sized,T:NativeBulkValue<I>> NativeReceiver<I,T> for NativeBulkReceiver<I,T>{
 fn demands(&self,input:RetainedCloneRef<'_,I>,_:usize)->Result<RetirementDemand,ValueError>{if self.closing||self.taken||self.output.is_some(){return Err(invalid())}if !input.check_binding(&self.binding)?{return Ok(fixed(input.binding_copy_bytes()))}let mut d=T::read_demand(input.get())?;d.copy_bytes=d.copy_bytes.checked_add(size_of::<Option<T>>()).ok_or_else(invalid)?;d.depth=d.depth.max(1);Ok(d)}
 fn advance(&mut self,input:RetainedCloneRef<'_,I>,control:&mut NativeDecodeControl<'_>,body:usize,g:RetainedCloneGrant)->Result<RetainedCloneStep,ValueError>{let d=self.demands(input,body)?;if !retained_clone_grant_admits(g,d){return Ok(RetainedCloneStep::Progress(Default::default()))}control.checkpoint()?;if let Some(p)=input.bind(&mut self.binding,g)?{return Ok(RetainedCloneStep::Progress(p))}*self.output=Some(T::read(input.get(),control)?);Ok(RetainedCloneStep::Complete(receipt(d)))}
 fn output_ready(&self)->bool{!self.closing&&self.output.is_some()}
 fn take_demand(&self)->Result<RetirementDemand,ValueError>{if !self.output_ready(){return Err(invalid())}Ok(fixed(2*size_of::<Option<T>>()+size_of::<bool>()))}
 fn take(&mut self,g:RetainedCloneGrant)->Result<Option<(T,RetainedCloneProgress)>,ValueError>{let d=self.take_demand()?;if !retained_clone_grant_admits(g,d){return Ok(None)}self.taken=true;Ok(Some((self.output.take().ok_or_else(invalid)?,receipt(d))))}
 fn begin_close(&mut self){self.closing=true}
 fn close_demands(&self,body:usize)->Result<RetirementDemand,ValueError>{if !self.retirement.is_empty(){return self.retirement.next_owner_demands::<T>(false,&None,body)}if self.output.is_some(){return self.retirement.next_owner_demands::<T>(true,&None,body)}RetainedCloneBinding::close_demand(&self.binding,body)}
 fn close_step(&mut self,g:RetainedCloneGrant)->Result<RetainedCloneStep,ValueError>{let d=self.close_demands(g.maximum_copy_bytes)?;if !retained_clone_grant_admits(g,d){return Ok(RetainedCloneStep::Progress(Default::default()))}if !self.retirement.is_empty(){return self.retirement.step_granted(g)}if let Some(s)=self.retirement.begin_granted(&mut self.output,g)?{return Ok(s)}RetainedCloneBinding::close_one(&mut self.binding,g)}
 fn terminal_is_empty(&self)->bool{self.binding.is_none()&&self.output.is_none()&&self.retirement.is_empty()}
}
impl<I:NativeValueSource+?Sized,T:NativeBulkValue<I>> Drop for NativeBulkReceiver<I,T>{fn drop(&mut self){assert!(std::thread::panicking()||self.terminal_is_empty(),"native bulk receiving requires funded closure")}}

fn quote(original:&dyn NativeValueSource)->Result<RetirementDemand,ValueError>{
 let mut capacity=0;
 native_bulk_measure(original,0,size_of::<DslValue>(),size_of::<(String,DslValue)>(),&mut capacity)?;
 Ok(RetirementDemand{copy_bytes:size_of::<DslValue>(),capacity_bytes:capacity,depth:1,..Default::default()})
}
fn build(original:&dyn NativeValueSource,depth:usize,control:&mut NativeDecodeControl<'_>)->Result<DslValue,ValueError>{
 if depth>NATIVE_BULK_MAXIMUM_DEPTH{return Err(ValueError::literal(ValueRefusalKind::DepthLimit,"native dynamic subtree exceeds depth limit"))}
 control.step()?;
 Ok(match native_bulk_kind(original)?{
  NativeBulkKind::Null=>DslValue::Null,
  NativeBulkKind::Bool=>DslValue::Bool(original.boolean()?),
  NativeBulkKind::Number=>DslValue::Number(original.number()?),
  NativeBulkKind::Text=>DslValue::String(control.copy_text(original.text()?)?),
  NativeBulkKind::Array=>{let count=original.sequence_len()?;let mut items=control.allocate_vec::<DslValue>(count)?;for index in 0..count{items.push(build(original.sequence_child(index)?,depth+1,control)?)}DslValue::Array(items)}
  NativeBulkKind::Object=>{let count=original.record_len()?;let mut entries=control.allocate_vec::<(String,DslValue)>(count)?;for index in 0..count{let key=control.copy_text(original.record_name(index)?)?;entries.push((key,build(original.record_child(index)?,depth+1,control)?))}DslValue::Object(entries)}
 })
}
impl<I:NativeValueSource+?Sized> NativeBulkValue<I> for DslValue{
 fn read_demand(original:&I)->Result<RetirementDemand,ValueError>{quote(original.as_source())}
 fn read(original:&I,control:&mut NativeDecodeControl<'_>)->Result<Self,ValueError>{build(original.as_source(),0,control)}
}
impl<I:NativeValueSource+?Sized> NativeReceive<I> for DslValue{
 type Receiver=NativeBulkReceiver<I,Self>;
 fn receive_birth_demand()->RetirementDemand{Self::Receiver::birth()}
 fn begin_receive(g:RetainedCloneGrant)->Result<(Self::Receiver,RetainedCloneProgress),ValueError>{Self::Receiver::begin(g)}
}
