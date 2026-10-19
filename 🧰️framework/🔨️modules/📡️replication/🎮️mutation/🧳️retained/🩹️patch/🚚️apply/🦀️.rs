//! 🚚️ Central sparse application stages genuine authored fields and exchanges one owned header per funded turn.
use super::{RetainedRowPatch,RetainedRowPatchFields,RetainedPatchSource};
use super::super::retained_mutation_demand_fits;
use super::super::super::ApplyCapability;
use semio_framework_value::{RetirementDemand,ValueError,ValueRefusalKind,retained_clone::{RetainedCloneRef,RetainedCloneGrant,RetainedCloneProgress,RetainedCloneStep,RetainedCloneClose}};
use std::mem::{ManuallyDrop,size_of};

const META:usize=size_of::<usize>()+size_of::<u8>();
fn invalid()->ValueError{ValueError::literal(ValueRefusalKind::InvariantViolated,"sparse central application lost its staged authored fields")}
fn scalar(bytes:usize)->RetirementDemand{RetirementDemand{copy_bytes:bytes,depth:1,..Default::default()}}
fn nested(mut demand:RetirementDemand)->Result<RetirementDemand,ValueError>{demand.copy_bytes=demand.copy_bytes.checked_add(META).ok_or_else(invalid)?;demand.depth=demand.depth.checked_add(1).ok_or_else(invalid)?;Ok(demand)}
fn child(grant:RetainedCloneGrant)->RetainedCloneGrant{RetainedCloneGrant{maximum_copy_bytes:grant.maximum_copy_bytes-META,maximum_depth:grant.maximum_depth-1,..grant}}
fn receipt(mut progress:RetainedCloneProgress)->Result<RetainedCloneProgress,ValueError>{progress.copied_bytes=progress.copied_bytes.checked_add(META).ok_or_else(invalid)?;Ok(progress)}

/// 🧳️ The original source aliases and displaced patch fields survive staging, exchange and partial cancellation.
pub struct RetainedRowPatchApply<R,P:RetainedRowPatch<R>>{
 fields:ManuallyDrop<Option<P::Fields>>,patch:ManuallyDrop<Option<P>>,retirement:RetainedCloneClose,
 phase:u8,index:usize,closing:bool,row:std::marker::PhantomData<fn()->R>,
}
impl<R,P:RetainedRowPatch<R>> Default for RetainedRowPatchApply<R,P>{fn default()->Self{Self::new()}}
impl<R,P:RetainedRowPatch<R>> RetainedRowPatchApply<R,P>{
 /// 🌱️ Empty construction captures no payload, projection or dynamic child.
 pub fn new()->Self{Self{fields:ManuallyDrop::new(None),patch:ManuallyDrop::new(None),retirement:Default::default(),phase:0,index:0,closing:false,row:std::marker::PhantomData}}
 /// 📏️ Every source capture, original field copy and fixed transfer has a separate quote.
 pub fn stage_demand(&self,patch:RetainedCloneRef<'_,P>,row:RetainedCloneRef<'_,R>,body:usize)->Result<RetirementDemand,ValueError>{
  if self.closing{return Err(invalid());}
  match self.phase{
   0=>Ok(scalar(size_of::<P::Fields>()+size_of::<Option<P::Fields>>()+META)),
   1=>nested(self.fields.as_ref().ok_or_else(invalid)?.demand(patch,row,body)?),
   2=>Ok(scalar(size_of::<P>()*2+size_of::<Option<P>>()+META)),
   _=>Err(invalid()),
  }
 }
 /// 🪡️ Copies only schema-declared authored sparse fields from their genuine immutable event source.
 pub fn stage(&mut self,patch:RetainedCloneRef<'_,P>,row:RetainedCloneRef<'_,R>,body:usize,grant:RetainedCloneGrant)->Result<RetainedCloneStep,ValueError>{
  let demand=self.stage_demand(patch,row,body)?;if !retained_mutation_demand_fits(demand,grant){return Ok(RetainedCloneStep::Progress(Default::default()));}
  let mut progress=RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,..Default::default()};
  match self.phase{
   0=>{*self.fields=Some(P::Fields::new(RetainedPatchSource::Authored));self.phase=1;},
   1=>{let step=self.fields.as_mut().ok_or_else(invalid)?.advance(patch,row,body,child(grant))?;progress=receipt(step.progress())?;if matches!(step,RetainedCloneStep::Complete(_)){self.phase=2;}},
   2=>{let(value,child)=self.fields.as_mut().ok_or_else(invalid)?.take(RetainedCloneGrant{maximum_copy_bytes:grant.maximum_copy_bytes-size_of::<Option<P>>()-META,..grant})?;*self.patch=value;progress=child;progress.copied_bytes+=size_of::<Option<P>>()+META;self.phase=3;return Ok(RetainedCloneStep::Complete(progress));},
   _=>return Err(invalid()),
  }
  Ok(RetainedCloneStep::Progress(progress))
 }
 /// 🛂️ A staged cursor is ready only after the complete original authored-field transfer.
 pub fn staged(&self)->bool{self.phase>=3&&!self.closing}
 /// 🫱️ Quotes the next sparse fixed-header exchange independently of untouched row storage.
 pub fn exchange_demand(&self)->Result<RetirementDemand,ValueError>{
  if self.closing||self.phase!=3{return Err(invalid());}
  if self.index==P::FIELD_COUNT{return Ok(scalar(META));}
  let mut demand=self.patch.as_ref().ok_or_else(invalid)?.exchange_demand(self.index)?;demand.copy_bytes=demand.copy_bytes.checked_add(META).ok_or_else(invalid)?;Ok(demand)
 }
 /// 🔑️ Only the central applier capability authorizes one validated field exchange in unpublished storage.
 pub fn exchange(&mut self,row:&mut R,capability:ApplyCapability,grant:RetainedCloneGrant)->Result<RetainedCloneStep,ValueError>{
  let demand=self.exchange_demand()?;if !retained_mutation_demand_fits(demand,grant){return Ok(RetainedCloneStep::Progress(Default::default()));}
  if self.index==P::FIELD_COUNT{self.phase=4;return Ok(RetainedCloneStep::Complete(RetainedCloneProgress{copied_items:1,copied_bytes:META,..Default::default()}));}
  let progress=self.patch.as_mut().ok_or_else(invalid)?.exchange_field(row,self.index,capability,RetainedCloneGrant{maximum_copy_bytes:grant.maximum_copy_bytes-META,..grant})?;
  self.index+=1;Ok(RetainedCloneStep::Progress(receipt(progress)?))
 }
 /// ✅️ Original source ownership remains closeable after the final field exchange.
 pub fn complete(&self)->bool{self.phase==4&&!self.closing}
 /// 🛑️ Cancellation stops staging and exchange before child-first typed closure.
 pub fn begin_close(&mut self){self.closing=true;if let Some(fields)=self.fields.as_mut(){fields.begin_close();}}
 /// ♻️ Original field cursors close before the displaced sparse patch is physically retired.
 pub fn close_demand(&self,body:usize)->Result<RetirementDemand,ValueError>{
  if !self.closing{return Err(invalid());}
  if let Some(fields)=self.fields.as_ref(){if !fields.terminal_is_empty(){return nested(fields.close_demand(body.saturating_sub(META))?);}return Ok(scalar(size_of::<P::Fields>()+size_of::<Option<P::Fields>>()+META));}
  if self.patch.is_some()||!self.retirement.is_empty(){return nested(self.retirement.next_owner_demands::<P>(self.patch.is_some(),&None,body.saturating_sub(META))?);}
  Ok(Default::default())
 }
 /// 🪜️ Each returned child and physical backing release is paid before its original owner disappears.
 pub fn close_step(&mut self,grant:RetainedCloneGrant)->Result<RetainedCloneStep,ValueError>{
  let demand=self.close_demand(grant.maximum_copy_bytes)?;if !retained_mutation_demand_fits(demand,grant){return Ok(RetainedCloneStep::Progress(Default::default()));}
  if let Some(fields)=self.fields.as_mut(){if !fields.terminal_is_empty(){let step=fields.close_step(child(grant))?;return Ok(RetainedCloneStep::Progress(receipt(step.progress())?));}drop(self.fields.take());return Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,..Default::default()}));}
  if !self.retirement.is_empty(){let step=self.retirement.step_granted(child(grant))?;return Ok(RetainedCloneStep::Progress(receipt(step.progress())?));}
  if self.patch.is_some(){if let Some(step)=self.retirement.begin_granted(&mut self.patch,child(grant))?{return Ok(RetainedCloneStep::Progress(receipt(step.progress())?));}}
  Ok(RetainedCloneStep::Complete(Default::default()))
 }
 /// 🫙️ All genuine source aliases and displaced payload allocations have returned.
 pub fn terminal_is_empty(&self)->bool{self.fields.is_none()&&self.patch.is_none()&&self.retirement.is_empty()}
}
impl<R,P:RetainedRowPatch<R>> Drop for RetainedRowPatchApply<R,P>{fn drop(&mut self){let empty=self.terminal_is_empty();assert!(std::thread::panicking()||empty,"sparse central apply requires original funded closure");if empty{unsafe{ManuallyDrop::drop(&mut self.fields);ManuallyDrop::drop(&mut self.patch);}}}}
