//! ✍️ Original owned map entries share the existing AVL producer and preserve exact grant custody.
use super::{OrderedMap,UpdateCursor,SharedOwner};
use crate::{ValueError,ValueRefusalKind,RetirementDemand,RetainedCloneGrant,RetainedCloneProgress,RetainedCloneStep,RetainedWorkOutcome};
use crate::retirement::{RetireOwned,RetirementCursor,RetirementStep};
use semio_framework_value_retained_clone::{OriginalRetirementFailure,RetainedCloneClose,retained_clone_grant_admits};
use std::mem::{ManuallyDrop,size_of};
fn invalid()->ValueError{ValueError::literal(ValueRefusalKind::InvariantViolated,"original owned map entry changed its retained frontier")}
struct Fields<V>{base:Option<OrderedMap<V>>,key:Option<String>,value:Option<V>,shared_key:Option<SharedOwner<String>>,shared_value:Option<SharedOwner<V>>,update:Option<UpdateCursor<V>>}
/// 📝️ Owns the original map and entry without an alternate update algorithm or borrowed source mirror.
pub struct OriginalOwnedEntry<V:RetireOwned+Sync>{fields:ManuallyDrop<Fields<V>>,retirement:RetainedCloneClose,phase:u8,closing:bool,progress:RetainedCloneProgress,last_work:u64}
impl<V:RetireOwned+Sync> OriginalOwnedEntry<V>{
 /// 🌱️ Transfers actual original owners inline before any header allocation is admitted.
 pub fn new(base:OrderedMap<V>,key:String,value:V)->Self{Self{fields:ManuallyDrop::new(Fields{base:Some(base),key:Some(key),value:Some(value),shared_key:None,shared_value:None,update:None}),retirement:Default::default(),phase:0,closing:false,progress:Default::default(),last_work:0}}
 /// 🔗️ Transfers the already-issued immutable value and admits only the original new key header.
 pub fn from_shared_value(base:OrderedMap<V>,key:String,value:SharedOwner<V>)->Self{Self{fields:ManuallyDrop::new(Fields{base:Some(base),key:Some(key),value:None,shared_key:None,shared_value:Some(value),update:None}),retirement:Default::default(),phase:0,closing:false,progress:Default::default(),last_work:0}}
 /// 📥️ Transfers both actual immutable entry headers without allocating replacement authority.
 pub fn from_shared(base:OrderedMap<V>,key:SharedOwner<String>,value:SharedOwner<V>)->Self{Self{fields:ManuallyDrop::new(Fields{base:Some(base),key:None,value:None,shared_key:Some(key),shared_value:Some(value),update:None}),retirement:Default::default(),phase:2,closing:false,progress:Default::default(),last_work:0}}
 /// ♻️ Identifies the actual existing updater closure selected during normal advancement.
 pub fn advancing_retirement(&self)->bool{!self.closing&&self.phase==5&&(!self.retirement.is_empty()||self.fields.update.is_some())}
 pub fn step_progress(&self)->RetainedCloneProgress{self.progress}
 pub fn last_turn_work(&self)->u64{self.last_work}
 /// 🪙️ Quotes the same original header, AVL event or retained close that the next advance selects.
 pub fn next_demands(&self,copy:usize)->Result<RetirementDemand,ValueError>{
  if self.closing{return self.next_close_demands(copy)}
  Ok(match self.phase{
   0=>{let mut d=SharedOwner::admission_demand(&self.fields.key,&self.fields.shared_key)?;d.copy_bytes=d.copy_bytes.checked_add(size_of::<u8>()).ok_or_else(invalid)?;d},
   1=>{let mut d=if self.fields.shared_value.is_none()&&self.fields.value.is_some(){SharedOwner::admission_demand(&self.fields.value,&self.fields.shared_value)?}else{RetirementDemand{depth:1,..Default::default()}};d.copy_bytes=d.copy_bytes.checked_add(size_of::<u8>()).ok_or_else(invalid)?;d},
   3=>{let update=self.fields.update.as_ref().ok_or_else(invalid)?;RetirementDemand{copy_bytes:update.next_copy_byte_demand(),capacity_bytes:update.next_capacity_byte_demand()?,depth:update.next_depth_demand(),..Default::default()}},
   5=>if self.retirement.is_empty()&&self.fields.update.is_none(){RetirementDemand{depth:1,..Default::default()}}else{return self.retirement.next_owner_demands::<UpdateCursor<V>>(self.fields.update.is_some(),&None,copy)},
   7=>Default::default(),
   _=>RetirementDemand{depth:1,..Default::default()},
  })
 }
 /// ⏱️ Returns the defining producer's actual work on success, denial and failure.
 pub fn advance(&mut self,effect:&mut crate::RetainedAllocationEffect<'_>)->RetainedWorkOutcome<Option<OrderedMap<V>>,OriginalRetirementFailure>{
  self.progress=Default::default();self.last_work=0;if effect.maximum_work_units()<self.advance_work_demand(effect.grant().maximum_copy_bytes).unwrap_or(u64::MAX){return RetainedWorkOutcome{result:Ok(None),work:0}}let result=self.advance_original(effect).map_err(|error|error.with_retained_progress(self.progress));RetainedWorkOutcome{result,work:self.last_work}
 }
 fn advance_original(&mut self,effect:&mut crate::RetainedAllocationEffect<'_>)->Result<Option<OrderedMap<V>>,OriginalRetirementFailure>{let grant=effect.grant();
  if self.closing{return Err(ValueError::literal(ValueRefusalKind::Canceled,"original owned map entry is closing").into())}
  if grant.maximum_items==0{return Ok(None)}
  let d=self.next_demands(grant.maximum_copy_bytes)?;if grant.maximum_depth<d.depth{return Err(ValueError::literal(ValueRefusalKind::DepthLimit,"original owned map entry exceeds admitted depth").into())}if !retained_clone_grant_admits(grant,d){return Ok(None)}
  if self.phase==5&&(!self.retirement.is_empty()||self.fields.update.is_some()){let outcome=if self.retirement.is_empty(){let turn=self.retirement.begin_granted(&mut self.fields.update,&mut effect.reborrow(grant,effect.maximum_work_units())?);RetainedWorkOutcome{result:turn.result.map_err(Into::into).map(|step|step.unwrap_or(RetainedCloneStep::Progress(Default::default()))),work:turn.work}}else{self.retirement.step_granted(effect)};self.progress=outcome.result.as_ref().map_or_else(|e|e.retained_progress(),|step|step.progress());self.last_work=outcome.work;outcome.result?;return Ok(None)}
  self.progress.copied_items=1;self.last_work=1;
  match self.phase{
   0=>{let fields=&mut*self.fields;let child=RetainedCloneGrant{maximum_copy_bytes:grant.maximum_copy_bytes-size_of::<u8>(),..grant};let turn=SharedOwner::admit_into(&mut fields.key,&mut fields.shared_key,&mut effect.reborrow(child,effect.maximum_work_units())?);self.last_work=turn.work;self.progress=Default::default();if let Some(mut progress)=turn.result?{progress.copied_bytes+=size_of::<u8>();self.progress=progress;self.phase=1}},
   1=>{if self.fields.shared_value.is_some()||self.fields.value.is_none(){self.phase=2;self.progress.copied_bytes=size_of::<u8>()}else{let fields=&mut*self.fields;let child=RetainedCloneGrant{maximum_copy_bytes:grant.maximum_copy_bytes-size_of::<u8>(),..grant};let turn=SharedOwner::admit_into(&mut fields.value,&mut fields.shared_value,&mut effect.reborrow(child,effect.maximum_work_units())?);self.last_work=turn.work;self.progress=Default::default();if let Some(mut progress)=turn.result?{progress.copied_bytes+=size_of::<u8>();self.progress=progress;self.phase=2}}},
   2=>{let fields=&mut*self.fields;fields.update=Some(UpdateCursor::new(fields.base.take().ok_or_else(invalid)?,fields.shared_key.take().ok_or_else(invalid)?,fields.shared_value.take()));self.phase=3},
   3=>{let update=self.fields.update.as_mut().ok_or_else(invalid)?;let outcome=update.advance(&mut effect.reborrow(grant,effect.maximum_work_units())?);self.last_work=outcome.work;self.progress=outcome.result.as_ref().map_or_else(|e|e.retained_progress(),|step|step.progress());outcome.result?;if update.is_complete(){self.phase=4}},
   4=>{self.fields.base=self.fields.update.as_mut().ok_or_else(invalid)?.take_result();if self.fields.base.is_none(){return Err(invalid().into())}self.phase=5},
   5=>{self.phase=6},
   6=>{self.phase=7;return Ok(self.fields.base.take())},
   _=>{self.progress=Default::default();self.last_work=0;return Err(invalid().into())},
  }Ok(None)
 }
 /// 🛑️ Retains every original input and partial child until explicit closure.
 pub fn begin_close_demand(&self)->RetirementDemand{crate::retained_clone::retained_close_start_demand(self.closing)}
 pub fn begin_close_work_demand(&self)->u64{u64::from(!self.closing)}
 pub fn begin_close_paid(&mut self,effect:&mut crate::RetainedAllocationEffect<'_>)->RetainedWorkOutcome<RetainedCloneProgress>{crate::retained_clone::start_retained_close(&mut self.closing,effect)}
 pub fn advance_work_demand(&self,copy:usize)->Result<u64,ValueError>{if self.closing||self.phase==7{return Ok(0)}if self.phase==5&&(!self.retirement.is_empty()||self.fields.update.is_some()){return self.retirement.next_owner_work::<UpdateCursor<V>>(self.fields.update.is_some(),&None,copy)}Ok(1)}
 pub fn close_work_demand(&self,copy:usize)->Result<u64,ValueError>{if self.retirement.requires_original_control(){return self.retirement.next_owner_work::<V>(false,&None,copy)}if !self.closing{return Ok(1)}if !self.retirement.is_empty(){return self.retirement.next_owner_work::<V>(false,&None,copy)}Ok(u64::from(self.fields.base.is_some()||self.fields.key.is_some()||self.fields.value.is_some()||self.fields.shared_key.is_some()||self.fields.shared_value.is_some()||self.fields.update.is_some()))}
 pub fn next_close_demands(&self,copy:usize)->Result<RetirementDemand,ValueError>{if self.retirement.requires_original_control(){return self.retirement.demands(copy)}if !self.closing{return Ok(self.begin_close_demand())}
  if !self.retirement.is_empty(){return self.retirement.demands(copy)}
  macro_rules! owner{($field:ident,$type:ty)=>{if self.fields.$field.is_some(){return self.retirement.next_owner_demands::<$type>(true,&None,copy)}}}
  owner!(base,OrderedMap<V>);owner!(key,String);owner!(value,V);owner!(shared_key,SharedOwner<String>);owner!(shared_value,SharedOwner<V>);owner!(update,UpdateCursor<V>);Ok(Default::default())
 }
 /// 🧹️ Preserves the complete original close outcome, including work performed before refusal.
 pub fn close_step(&mut self,effect:&mut crate::RetainedAllocationEffect<'_>)->RetainedWorkOutcome<RetainedCloneStep,OriginalRetirementFailure>{
  self.progress=Default::default();self.last_work=0;let outcome=self.close_original_with(effect,|retirement,effect|retirement.step_granted(effect));self.last_work=outcome.work;self.progress=outcome.result.as_ref().map_or_else(|e|e.retained_progress(),|step|step.progress());outcome
 }
 fn close_original_with(&mut self,effect:&mut crate::RetainedAllocationEffect<'_>,invoke:impl FnOnce(&mut RetainedCloneClose,&mut crate::RetainedAllocationEffect<'_>)->crate::OriginalRetirementOutcome)->RetainedWorkOutcome<RetainedCloneStep,OriginalRetirementFailure>{let grant=effect.grant();if effect.maximum_work_units()<self.close_work_demand(grant.maximum_copy_bytes).unwrap_or(u64::MAX){return RetainedWorkOutcome{result:Ok(RetainedCloneStep::Progress(Default::default())),work:0}}
  if self.retirement.requires_original_control(){return invoke(&mut self.retirement,effect)}
  if !self.closing{let turn=self.begin_close_paid(grant);return RetainedWorkOutcome{result:turn.result.map_err(Into::into).map(RetainedCloneStep::Progress),work:turn.work}}
  if !self.retirement.is_empty(){return invoke(&mut self.retirement,effect)}
  macro_rules! owner{($field:ident)=>{if self.fields.$field.is_some(){let turn=self.retirement.begin_granted(&mut self.fields.$field,effect);return RetainedWorkOutcome{result:turn.result.map_err(Into::into).map(|step|step.unwrap_or(RetainedCloneStep::Progress(Default::default()))),work:turn.work}}}}
  owner!(base);owner!(key);owner!(value);owner!(shared_key);owner!(shared_value);owner!(update);RetainedWorkOutcome{result:Ok(RetainedCloneStep::Complete(Default::default())),work:0}
 }
 pub fn terminal_is_empty(&self)->bool{self.closing&&self.retirement.is_empty()&&self.fields.base.is_none()&&self.fields.key.is_none()&&self.fields.value.is_none()&&self.fields.shared_key.is_none()&&self.fields.shared_value.is_none()&&self.fields.update.is_none()}
}
impl<V:RetireOwned+Sync> Drop for OriginalOwnedEntry<V>{fn drop(&mut self){assert!(std::thread::panicking()||self.terminal_is_empty(),"original owned map entry must reach physical terminal emptiness");if self.terminal_is_empty(){unsafe{ManuallyDrop::drop(&mut self.fields)}}}}
#[doc(hidden)]
pub struct EntryRetirement<V:RetireOwned+Sync>(OriginalOwnedEntry<V>);
impl<V:RetireOwned+Sync> RetirementCursor for EntryRetirement<V>{
 fn requires_original_control(&self)->bool{self.0.retirement.requires_original_control()}
 fn requires_original_birth(&self)->bool{self.0.retirement.requires_original_birth()}
 fn close_original_birth_step(&mut self,original:&mut semio_framework_value_control_contract::OriginalRetirementCloseLoan<'_>,grant:RetainedCloneGrant)->RetirementStep{if !self.requires_original_birth(){return RetirementStep::Failure(invalid().into(),0)}entry_retirement_step(self.0.retirement.step_original_birth(original,grant))}
 fn close_original_step(&mut self,original:&mut semio_framework_value_control_contract::OriginalRetirementCloseLoan<'_>,effect:&mut crate::RetainedAllocationEffect<'_>)->RetirementStep{entry_retirement_step(self.0.close_original_with(effect,|retirement,effect|retirement.step_original(original,effect)))}
 fn close_step_controlled(&mut self,effect:&mut crate::RetainedAllocationEffect<'_>,control:&mut dyn crate::OriginalRetirementControl)->RetirementStep{entry_retirement_step(self.0.close_original_with(effect,|retirement,effect|retirement.step_controlled(effect,control)))}

 fn next_computational_work(&self,copy:usize)->Result<u64,ValueError>{self.0.close_work_demand(copy)}
 fn next_demand(&self,copy:usize)->Result<RetirementDemand,ValueError>{self.0.next_close_demands(copy)}
 fn close_step(&mut self,effect:&mut crate::RetainedAllocationEffect<'_>)->RetirementStep{let outcome=self.0.close_step(effect);match outcome.result{Ok(RetainedCloneStep::Complete(p))if p==RetainedCloneProgress::default()=>RetirementStep::Complete(outcome.work),Ok(step)if step.progress()!=RetainedCloneProgress::default()||outcome.work!=0=>RetirementStep::Progress(step.progress(),outcome.work),Ok(_)=>RetirementStep::BudgetExhausted,Err(error)=>RetirementStep::Failure(error,outcome.work)}}
 fn terminal_is_empty(&self)->bool{self.0.terminal_is_empty()}
 fn next_work_byte_demand(&self)->Result<usize,ValueError>{Ok(self.0.next_close_demands(0)?.copy_bytes)}
 fn allows_admitted_narrow_work(&self)->bool{true}
 fn next_birth_bytes(&self,copy:usize)->Option<usize>{self.0.next_close_demands(copy).ok().map(|d|d.capacity_bytes)}
 fn next_close_byte_demand(&self)->Option<usize>{self.0.next_close_demands(0).ok().map(|d|d.release_bytes)}
 fn next_depth_demand(&self)->Result<usize,ValueError>{Ok(self.0.next_close_demands(0)?.depth)}
 fn terminal_release_bytes(&self)->Option<usize>{self.0.terminal_is_empty().then_some(size_of::<Self>())}
}
impl<V:RetireOwned+Sync> RetireOwned for OriginalOwnedEntry<V>{
 type RetirementCursor=EntryRetirement<V>;
 fn retirement_seed(self)->Self::RetirementCursor{EntryRetirement(self)}
 fn retirement_birth_bytes(&self)->Option<usize>{Some(size_of::<EntryRetirement<V>>())}
 fn controlled_retirement_supported()->bool{V::controlled_retirement_supported()}
}
#[cfg(test)]
#[path="🧪️.rs"]mod tests;

#[path="➖remove/🦀️.rs"]
mod removal;

fn entry_retirement_step(turn:crate::OriginalRetirementOutcome)->RetirementStep{match turn.result{Ok(RetainedCloneStep::Complete(p))if p==Default::default()=>RetirementStep::Complete(turn.work),Ok(step)if step.progress()!=Default::default()||turn.work!=0=>RetirementStep::Progress(step.progress(),turn.work),Ok(_)=>RetirementStep::BudgetExhausted,Err(error)=>RetirementStep::Failure(error,turn.work)}}
