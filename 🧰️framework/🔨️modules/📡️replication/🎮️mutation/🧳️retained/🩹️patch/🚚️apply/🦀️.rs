//! 🚚️ Central sparse application stages genuine authored fields and exchanges one owned header per funded turn.
use super::{RetainedRowPatch,RetainedRowPatchFields,RetainedPatchSource};
use super::super::retained_mutation_demand_fits;
use super::super::super::ApplyCapability;
use semio_framework_replication_retained_field::{settle,settle_exit};
use semio_framework_value::{OriginalRetirementFailure,OriginalRetirementOutcome,OriginalRetirementPoll,RetainedAllocationEffect,RetirementDemand,RetirementPollExit,ValueError,ValueRefusalKind,retained_clone::{RetainedCloneRef,RetainedCloneGrant,RetainedCloneProgress,RetainedCloneStep,RetainedCloneClose}};
use std::mem::{ManuallyDrop,size_of};

const META:usize=size_of::<usize>()+size_of::<u8>();
fn invalid()->ValueError{ValueError::literal(ValueRefusalKind::InvariantViolated,"sparse central application lost its staged authored fields")}
fn scalar(bytes:usize)->RetirementDemand{RetirementDemand{copy_bytes:bytes,depth:1,..Default::default()}}
fn nested(mut demand:RetirementDemand)->Result<RetirementDemand,ValueError>{demand.copy_bytes=demand.copy_bytes.checked_add(META).ok_or_else(invalid)?;demand.depth=demand.depth.checked_add(1).ok_or_else(invalid)?;Ok(demand)}
fn child(grant:RetainedCloneGrant)->RetainedCloneGrant{RetainedCloneGrant{maximum_copy_bytes:grant.maximum_copy_bytes-META,maximum_depth:grant.maximum_depth-1,..grant}}
fn receipt(mut progress:RetainedCloneProgress)->Result<RetainedCloneProgress,ValueError>{progress.copied_bytes=progress.copied_bytes.checked_add(META).ok_or_else(invalid)?;Ok(progress)}

/// 🧳️ The original source aliases and displaced patch fields survive staging, exchange and partial cancellation.
pub struct RetainedRowPatchApply<R,P:RetainedRowPatch<R>>{
 fields:ManuallyDrop<Option<Box<P::Fields>>>,patch:ManuallyDrop<Option<P>>,retirement:RetainedCloneClose,
 phase:u8,index:usize,closing:bool,row:std::marker::PhantomData<fn()->R>,
}
impl<R,P:RetainedRowPatch<R>> Default for RetainedRowPatchApply<R,P>{fn default()->Self{Self::new()}}
impl<R,P:RetainedRowPatch<R>> RetainedRowPatchApply<R,P>{
 /// 🌱️ Empty construction captures no payload, projection or dynamic child.
 pub fn new()->Self{Self{fields:ManuallyDrop::new(None),patch:ManuallyDrop::new(None),retirement:Default::default(),phase:0,index:0,closing:false,row:std::marker::PhantomData}}
 /// 📏️ Every source capture, original field copy and fixed transfer has a separate quote.
 pub fn stage_demand(&self,patch:RetainedCloneRef<'_,P>,row:RetainedCloneRef<'_,R>,body:usize,maximum_work_units:u64)->Result<RetirementDemand,ValueError>{
  if self.closing{return Err(invalid());}
  if maximum_work_units==0{return Ok(Default::default())}
  match self.phase{
   0=>Ok(RetirementDemand{capacity_bytes:size_of::<P::Fields>(),..scalar(size_of::<P::Fields>()+size_of::<Option<Box<P::Fields>>>()+META)}),
   1=>nested(self.fields.as_ref().ok_or_else(invalid)?.demand(patch,row,body,maximum_work_units)?),
   2=>Ok(scalar(size_of::<P>()*2+size_of::<Option<P>>()+META)),
   _=>Err(invalid()),
  }
 }
 /// 🪡️ Copies only schema-declared authored sparse fields from their genuine immutable event source.
 /// ⛽️ Quotes the selected original sparse-field stage work.
 pub fn stage_work_demand(&self,patch:RetainedCloneRef<'_,P>,row:RetainedCloneRef<'_,R>,body:usize,maximum_work_units:u64)->Result<u64,ValueError>{self.stage_demand(patch,row,body,maximum_work_units)?;if maximum_work_units==0{return Ok(0)}let work=match self.phase{0=>P::Fields::new_work_demand()?.checked_add(1).ok_or_else(invalid)?,2=>1,1=>return self.fields.as_ref().ok_or_else(invalid)?.advance_work_demand(patch,row,body,maximum_work_units),_=>return Err(invalid())};Ok(if work<=maximum_work_units{work}else{0})}
 pub fn stage(&mut self,patch:RetainedCloneRef<'_,P>,row:RetainedCloneRef<'_,R>,body:usize,maximum_work_units:u64,effect:&mut RetainedAllocationEffect<'_>)->OriginalRetirementOutcome{let grant=effect.grant();let base=effect.performed_work();let mut work:u128=0;let result:Result<RetainedCloneStep,OriginalRetirementFailure>=(||{
  let selected=self.stage_work_demand(patch,row,body,maximum_work_units)?;if selected==0{return Ok(RetainedCloneStep::Progress(Default::default()))}let demand=self.stage_demand(patch,row,body,maximum_work_units)?;if !retained_mutation_demand_fits(demand,grant){return Ok(RetainedCloneStep::Progress(Default::default()));}
  let mut progress=RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,retained_capacity_bytes:if self.phase==0{demand.capacity_bytes}else{0},..Default::default()};
  match self.phase{
   0=>{let(fields,actual)=P::Fields::new(RetainedPatchSource::Authored);work=u128::from(actual)+1;*self.fields=Some(Box::new(fields));self.phase=1;if work>u128::from(selected){return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"sparse owner birth exceeded its work quote").with_retained_progress(progress).into())}},
   1=>{let turn=self.fields.as_mut().ok_or_else(invalid)?.advance(patch,row,body,maximum_work_units,&mut effect.nested(child(grant)));work+=turn.work;let step=turn.result.map_err(|error|if turn.work==0{error}else{let mut p=error.retained_progress();p.copied_bytes=p.copied_bytes.saturating_add(META);error.with_retained_progress(p)})?;progress=receipt(step.progress())?;if matches!(step,RetainedCloneStep::Complete(_)){self.phase=2;}},
   2=>{work+=1;let(value,child)=self.fields.as_mut().ok_or_else(invalid)?.take(RetainedCloneGrant{maximum_copy_bytes:grant.maximum_copy_bytes-size_of::<Option<P>>()-META,..grant})?;*self.patch=value;progress=child;progress.copied_bytes+=size_of::<Option<P>>()+META;self.phase=3;return Ok(RetainedCloneStep::Complete(progress));},
   _=>return Err(invalid().into()),
  }
  Ok(RetainedCloneStep::Progress(progress))
  })();settle(effect,base,result,work)
 }
 /// 🛂️ A staged cursor is ready only after the complete original authored-field transfer.
 pub fn staged(&self)->bool{self.phase>=3&&!self.closing}
 /// 🫱️ Quotes the next sparse fixed-header exchange independently of untouched row storage.
 pub fn exchange_demand(&self)->Result<RetirementDemand,ValueError>{
  if self.closing||self.phase!=3{return Err(invalid());}
  if self.index==P::FIELD_COUNT{return Ok(scalar(META));}
  let mut demand=self.patch.as_ref().ok_or_else(invalid)?.exchange_demand(self.index)?;demand.copy_bytes=demand.copy_bytes.checked_add(META).ok_or_else(invalid)?;Ok(demand)
 }
 /// ⛽️ Quotes the selected fixed original row-field exchange.
 pub fn exchange_work_demand(&self)->Result<u64,ValueError>{self.exchange_demand()?;Ok(1)}
 /// 🔑️ Only the central applier capability authorizes one validated field exchange in unpublished storage.
 pub fn exchange(&mut self,row:&mut R,capability:ApplyCapability,grant:RetainedCloneGrant)->semio_framework_value::OriginalRetirementOutcome{let mut work=0;let result:Result<_,semio_framework_value::OriginalRetirementFailure>=(||{
  let demand=self.exchange_demand()?;if !retained_mutation_demand_fits(demand,grant){return Ok(RetainedCloneStep::Progress(Default::default()));}
  work=1;if self.index==P::FIELD_COUNT{self.phase=4;return Ok(RetainedCloneStep::Complete(RetainedCloneProgress{copied_items:1,copied_bytes:META,..Default::default()}));}
  let progress=self.patch.as_mut().ok_or_else(invalid)?.exchange_field(row,self.index,capability,RetainedCloneGrant{maximum_copy_bytes:grant.maximum_copy_bytes-META,..grant})?;
  self.index+=1;Ok(RetainedCloneStep::Progress(receipt(progress)?))
  })();semio_framework_value::RetainedWorkOutcome{result,work}
 }
 /// ✅️ Original source ownership remains closeable after the final field exchange.
 pub fn complete(&self)->bool{self.phase==4&&!self.closing}
 /// 🛑️ Cancellation stops staging and exchange before child-first typed closure.
 pub fn begin_close(&mut self){self.closing=true;}
 pub fn close_started(&self)->bool{self.closing}
 pub fn begin_close_demand(&self)->RetirementDemand{if self.closing{Default::default()}else{scalar(size_of::<bool>())}}
 pub fn begin_close_work_demand(&self)->u64{u64::from(!self.closing)}
 pub fn begin_close_paid(&mut self,effect:&mut RetainedAllocationEffect<'_>)->semio_framework_value::RetainedWorkOutcome<RetainedCloneProgress,ValueError>{let demand=self.begin_close_demand();if self.closing||effect.maximum_work_units()==0||!retained_mutation_demand_fits(demand,effect.grant()){return semio_framework_value::RetainedWorkOutcome{result:Ok(Default::default()),work:0}}let work=match effect.performed(1){Ok(true)=>1,Ok(false)=>return semio_framework_value::RetainedWorkOutcome{result:Err(ValueError::literal(ValueRefusalKind::WorkLimit,"sparse apply close start exceeded its original action")),work:1},Err(error)=>return semio_framework_value::RetainedWorkOutcome{result:Err(error),work:1}};self.closing=true;semio_framework_value::RetainedWorkOutcome{result:Ok(RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,..Default::default()}),work}}
 /// ♻️ Original field cursors close before the displaced sparse patch is physically retired.
 pub fn close_demand(&self,body:usize)->Result<RetirementDemand,ValueError>{
  if !self.closing{return Err(invalid());}
  if let Some(fields)=self.fields.as_ref(){if !fields.terminal_is_empty(){if !fields.close_started(){return Ok(fields.begin_close_demand())}return nested(fields.close_demand(body.saturating_sub(META))?);}return Ok(RetirementDemand{release_bytes:size_of::<P::Fields>(),..scalar(size_of::<Option<Box<P::Fields>>>()+META)});}
  if self.patch.is_some()||!self.retirement.is_empty(){return nested(self.retirement.next_owner_demands::<P>(self.patch.is_some(),&None,body.saturating_sub(META))?);}
  Ok(Default::default())
 }
 /// 🪜️ Each returned child and physical backing release is paid before its original owner disappears.
 /// ⛽️ Quotes the selected sparse-field or displaced-patch close work.
 pub fn close_work_demand(&self,body:usize)->Result<u64,ValueError>{self.close_demand(body)?;if let Some(fields)=self.fields.as_ref(){return if fields.terminal_is_empty(){Ok(1)}else if !fields.close_started(){Ok(fields.begin_close_work_demand())}else{fields.close_work_demand(body.saturating_sub(META))}}self.retirement.next_owner_work::<P>(self.patch.is_some(),&None,body.saturating_sub(META))}
 pub fn close_step(&mut self,effect:&mut RetainedAllocationEffect<'_>)->OriginalRetirementPoll{let grant=effect.grant();let base=effect.performed_work();let mut work:u128=0;let result:Result<RetainedCloneStep,RetirementPollExit>=(||{
  let demand=self.close_demand(grant.maximum_copy_bytes)?;if !retained_mutation_demand_fits(demand,grant){return Ok(RetainedCloneStep::Progress(Default::default()));}
  if let Some(fields)=self.fields.as_mut(){if !fields.terminal_is_empty(){if !fields.close_started(){let turn=fields.begin_close_paid(effect);work+=turn.work;return Ok(RetainedCloneStep::Progress(turn.result?))}let turn=RetirementPollExit::settled(fields.close_step(&mut effect.nested(child(grant))))?;work+=turn.work;let step=turn.result.map_err(|error|if turn.work==0{error}else{let mut p=error.retained_progress();p.copied_bytes=p.copied_bytes.saturating_add(META);error.with_retained_progress(p)})?;return Ok(RetainedCloneStep::Progress(receipt(step.progress())?));}work+=1;drop(self.fields.take());return Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,released_bytes:demand.release_bytes,..Default::default()}));}
  if !self.retirement.is_empty(){let turn=RetirementPollExit::settled(self.retirement.step_granted(&mut effect.nested(child(grant))))?;work+=turn.work;let step=turn.result.map_err(|error|if turn.work==0{error}else{let mut p=error.retained_progress();p.copied_bytes=p.copied_bytes.saturating_add(META);error.with_retained_progress(p)})?;return Ok(RetainedCloneStep::Progress(receipt(step.progress())?));}
  if self.patch.is_some(){let turn=self.retirement.begin_granted(&mut self.patch,&mut effect.nested(child(grant)));work+=turn.work;if let Some(step)=turn.result?{return Ok(RetainedCloneStep::Progress(receipt(step.progress())?));}}
  Ok(RetainedCloneStep::Complete(Default::default()))
  })();settle_exit(effect,base,result,work)
 }
 /// 🫙️ All genuine source aliases and displaced payload allocations have returned.
 pub fn terminal_is_empty(&self)->bool{self.fields.is_none()&&self.patch.is_none()&&self.retirement.is_empty()}
}
impl<R,P:RetainedRowPatch<R>> Drop for RetainedRowPatchApply<R,P>{fn drop(&mut self){let empty=self.terminal_is_empty();assert!(std::thread::panicking()||empty,"sparse central apply requires original funded closure");if empty{unsafe{ManuallyDrop::drop(&mut self.fields);ManuallyDrop::drop(&mut self.patch);}}}}
