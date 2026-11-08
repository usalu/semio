//! ✍️ Owned map entries publish every native result into caller-resident slots.
use super::{OrderedMap,UpdateCursor,SharedOwner};
use crate::{ValueError,ValueRefusalKind,RetirementDemand,RetainedCloneGrant,RetainedCloneProgress,RetainedCloneStep,RetainedWorkOutcome,RetainedAllocationEffect,OriginalEffectReply,OriginalEffectReplyComplete,OriginalRetirementFailure,OriginalRetirementCloseLoan};
use crate::retirement::{RetireOwned,RetirementCursor,RetirementStep};
use semio_framework_value_retained_clone::{RetainedCloneClose,retained_clone_grant_admits};
use std::mem::{ManuallyDrop,size_of};
fn invalid()->ValueError{ValueError::invariant("original owned map entry changed its retained frontier")}
struct Fields<V>{base:Option<OrderedMap<V>>,key:Option<String>,value:Option<V>,shared_key:Option<SharedOwner<String>>,shared_value:Option<SharedOwner<V>>,update:Option<UpdateCursor<V>>}
/// 📝️ Retains the original inputs, actual updater and selected retirement until paid handoff.
pub struct OriginalOwnedEntry<V:RetireOwned+Sync>{fields:ManuallyDrop<Fields<V>>,retirement:RetainedCloneClose,phase:u8,closing:bool}
impl<V:RetireOwned+Sync> OriginalOwnedEntry<V>{
 pub fn new(base:OrderedMap<V>,key:String,value:V)->Self{Self{fields:ManuallyDrop::new(Fields{base:Some(base),key:Some(key),value:Some(value),shared_key:None,shared_value:None,update:None}),retirement:Default::default(),phase:0,closing:false}}
 pub fn from_shared_value(base:OrderedMap<V>,key:String,value:SharedOwner<V>)->Self{Self{fields:ManuallyDrop::new(Fields{base:Some(base),key:Some(key),value:None,shared_key:None,shared_value:Some(value),update:None}),retirement:Default::default(),phase:0,closing:false}}
 pub fn from_shared(base:OrderedMap<V>,key:SharedOwner<String>,value:SharedOwner<V>)->Self{Self{fields:ManuallyDrop::new(Fields{base:Some(base),key:None,value:None,shared_key:Some(key),shared_value:Some(value),update:None}),retirement:Default::default(),phase:2,closing:false}}
 pub fn advancing_retirement(&self)->bool{!self.closing&&self.phase==5&&(!self.retirement.is_empty()||self.fields.update.is_some())}
 pub fn requires_original_birth(&self)->bool{self.retirement.requires_original_birth()}
 pub fn requires_original_control(&self)->bool{self.retirement.requires_original_control()}
 pub fn close_original_birth_step(&mut self,o:&mut OriginalRetirementCloseLoan<'_>,g:RetainedCloneGrant)->crate::OriginalRetirementPoll{self.retirement.step_original_birth(o,g)}
 pub fn next_work_demand(&self,copy:usize)->Result<u64,ValueError>{
  if self.closing{return self.next_close_work_demand(copy)}
  match self.phase{
   0=>Ok(SharedOwner::<String>::source_admission_work_demand()+1),
   1=>Ok(if self.fields.shared_value.is_none()&&self.fields.value.is_some(){SharedOwner::<V>::source_admission_work_demand()+1}else{1}),
   2=>Ok(1+3*u64::from(self.fields.base.as_ref().is_some_and(|base|!base.is_empty()))),
   3=>{let update=self.fields.update.as_ref().ok_or_else(invalid)?;if update.is_complete(){Ok(1)}else{update.native_work_demand(copy)}},
   5 if !self.retirement.is_empty()||self.fields.update.is_some()=>self.retirement.next_owner_work::<UpdateCursor<V>>(self.fields.update.is_some(),&None,copy),
   7=>Ok(0),_=>Ok(1)
  }
 }
 pub fn next_demands(&self,copy:usize)->Result<RetirementDemand,ValueError>{
  if self.closing{return self.next_close_demands(copy)}
  Ok(match self.phase{
   0=>{let mut d=SharedOwner::<String>::source_admission_demand()?;d.copy_bytes=d.copy_bytes.checked_add(size_of::<u8>()).ok_or_else(invalid)?;d},
   1 if self.fields.shared_value.is_none()&&self.fields.value.is_some()=>{let mut d=SharedOwner::<V>::source_admission_demand()?;d.copy_bytes=d.copy_bytes.checked_add(size_of::<u8>()).ok_or_else(invalid)?;d},
   2=>RetirementDemand{copy_bytes:size_of::<Option<OrderedMap<V>>>()+size_of::<Option<SharedOwner<String>>>()+size_of::<Option<SharedOwner<V>>>()+size_of::<Option<UpdateCursor<V>>>()+SharedOwner::<V>::duplicate_copy_bytes()+size_of::<u8>(),depth:1,..Default::default()},
   3=>{let update=self.fields.update.as_ref().ok_or_else(invalid)?;if update.is_complete(){RetirementDemand{copy_bytes:size_of::<u8>(),depth:1,..Default::default()}}else{return update.native_demands(copy)}},
   4=>RetirementDemand{copy_bytes:2*size_of::<Option<OrderedMap<V>>>()+size_of::<u8>(),depth:1,..Default::default()},
   5 if !self.retirement.is_empty()||self.fields.update.is_some()=>return self.retirement.next_owner_demands::<UpdateCursor<V>>(self.fields.update.is_some(),&None,copy),
   6=>RetirementDemand{copy_bytes:2*size_of::<Option<OrderedMap<V>>>()+size_of::<u8>(),depth:1,..Default::default()},
   7=>Default::default(),_=>RetirementDemand{copy_bytes:size_of::<u8>(),depth:1,..Default::default()}
  })
 }
 fn native_advance(&mut self,body:usize,output:&mut Option<OrderedMap<V>>,effect:&mut RetainedAllocationEffect<'_>)->RetainedWorkOutcome<RetainedCloneStep,OriginalRetirementFailure>{
  let before=effect.performed_boundary();let result=(||{
   if self.closing{return Err(ValueError::literal(ValueRefusalKind::Canceled,"owned map entry is closing").into())}
   if self.phase==7{return Ok(RetainedCloneStep::Complete(Default::default()))}
   if output.is_some(){return Err(invalid().into())}
   let grant=effect.grant();let d=self.next_demands(body)?;let work=self.next_work_demand(body)?;
   if !retained_clone_grant_admits(grant,d)||effect.remaining_work_units()?<work{return Ok(RetainedCloneStep::Progress(Default::default()))}
   if self.phase==3{let turn=self.fields.update.as_mut().ok_or_else(invalid)?.advance_native(body,effect);let step=turn.result?;if matches!(step,RetainedCloneStep::Complete(_)){return Ok(RetainedCloneStep::Progress(step.progress()))}return Ok(step)}
   if self.phase==5&&(!self.retirement.is_empty()||self.fields.update.is_some()){
    if !self.retirement.is_empty(){return Err(invalid().into())}
    return self.retirement.begin_granted(&mut self.fields.update,effect).result.map(|s|s.unwrap_or(RetainedCloneStep::Progress(Default::default()))).map_err(Into::into)
   }
   if self.phase<=1{
    let fields=&mut*self.fields;
    if self.phase==1&&(fields.shared_value.is_some()||fields.value.is_none()){
     if !effect.performed(1)?{return Err(invalid().into())}self.phase=2;return Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,copied_bytes:size_of::<u8>(),..Default::default()}))
    }
    let child_grant=RetainedCloneGrant{maximum_copy_bytes:grant.maximum_copy_bytes-size_of::<u8>(),..grant};let child_work=work-1;
    let turn=if self.phase==0{SharedOwner::admit_from(&mut fields.key,&mut fields.shared_key,&mut effect.reborrow(child_grant,child_work)?)}else{SharedOwner::admit_from(&mut fields.value,&mut fields.shared_value,&mut effect.reborrow(child_grant,child_work)?)};
    let step=turn.result?;if !matches!(step,RetainedCloneStep::Complete(_)){return Ok(RetainedCloneStep::Progress(step.progress()))}
    if !effect.performed(1)?{return Err(invalid().into())}self.phase+=1;let mut progress=step.progress();progress.copied_bytes=progress.copied_bytes.checked_add(size_of::<u8>()).ok_or_else(invalid)?;return Ok(RetainedCloneStep::Progress(progress))
   }
   if !effect.performed(work)?{return Err(invalid().into())}
   match self.phase{
    2=>{let fields=&mut*self.fields;fields.update=Some(UpdateCursor::new(fields.base.take().ok_or_else(invalid)?,fields.shared_key.take().ok_or_else(invalid)?,fields.shared_value.take()));self.phase=3},
    4=>{self.fields.base=self.fields.update.as_mut().ok_or_else(invalid)?.take_result();if self.fields.base.is_none(){return Err(invalid().into())}self.phase=5},
    5=>self.phase=6,
    6=>{*output=self.fields.base.take();self.phase=7},
    _=>return Err(invalid().into())
   }
   Ok(if self.phase==7{RetainedCloneStep::Complete(RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,..Default::default()})}else{RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,..Default::default()})})
  })();RetainedWorkOutcome{result,work:effect.performed_since(before).expect("same original ordered entry receipt")}
 }
 /// 📥️ Publishes the actual ordered output only after its original native action is paid.
 pub fn advance_into<'r>(&mut self,body:usize,output:&mut Option<OrderedMap<V>>,effect:&mut RetainedAllocationEffect<'_>,reply:OriginalEffectReply<'r,RetainedCloneStep,OriginalRetirementFailure>)->Result<OriginalEffectReplyComplete<'r,RetainedCloneStep,OriginalRetirementFailure>,ValueError>{
  if self.phase==3&&self.fields.update.as_ref().is_some_and(|update|update.is_complete()){
   let demand=RetirementDemand{copy_bytes:size_of::<u8>(),depth:1,..Default::default()};if !retained_clone_grant_admits(effect.grant(),demand)||effect.remaining_work_units()?<1{return Ok(reply.complete(Ok(RetainedCloneStep::Progress(Default::default()))))}if !effect.performed(1)?{return Err(invalid())}self.phase=4;return Ok(reply.complete(Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,..Default::default()}))))
  }
  if self.phase==5&&!self.retirement.is_empty(){return self.retirement.step_into(body,effect,reply.progress())}
  let turn=self.native_advance(body,output,effect);Ok(reply.complete(turn.result))
 }
 pub fn recover_advance_into<'r>(&mut self,body:usize,_output:&mut Option<OrderedMap<V>>,effect:&mut RetainedAllocationEffect<'_>,reply:OriginalEffectReply<'r,RetainedCloneStep,OriginalRetirementFailure>)->Result<OriginalEffectReplyComplete<'r,RetainedCloneStep,OriginalRetirementFailure>,ValueError>{if self.phase!=5||self.retirement.is_empty(){return Err(invalid())}self.retirement.recover_step_into(body,effect,reply.progress())}
 pub fn begin_close_demand(&self)->RetirementDemand{crate::retained_clone::retained_close_start_demand(self.closing)}
 pub fn begin_close_work_demand(&self)->u64{u64::from(!self.closing)}
 pub fn begin_close_paid(&mut self,effect:&mut RetainedAllocationEffect<'_>)->RetainedWorkOutcome<RetainedCloneProgress>{crate::retained_clone::start_retained_close(&mut self.closing,effect)}
 pub fn next_close_demands(&self,copy:usize)->Result<RetirementDemand,ValueError>{
  if !self.retirement.is_empty(){return self.retirement.demands(copy)}if !self.closing{return Ok(self.begin_close_demand())}
  macro_rules! owner{($field:ident,$type:ty)=>{if self.fields.$field.is_some(){return self.retirement.next_owner_demands::<$type>(true,&None,copy)}}}
  owner!(base,OrderedMap<V>);owner!(key,String);owner!(value,V);owner!(shared_key,SharedOwner<String>);owner!(shared_value,SharedOwner<V>);owner!(update,UpdateCursor<V>);Ok(Default::default())
 }
 pub fn next_close_work_demand(&self,copy:usize)->Result<u64,ValueError>{
  if !self.retirement.is_empty(){return self.retirement.next_owner_work::<OrderedMap<V>>(false,&None,copy)}if !self.closing{return Ok(1)}
  macro_rules! owner{($field:ident,$type:ty)=>{if self.fields.$field.is_some(){return self.retirement.next_owner_work::<$type>(true,&None,copy)}}}
  owner!(base,OrderedMap<V>);owner!(key,String);owner!(value,V);owner!(shared_key,SharedOwner<String>);owner!(shared_value,SharedOwner<V>);owner!(update,UpdateCursor<V>);Ok(0)
 }
 fn close_native(&mut self,body:usize,effect:&mut RetainedAllocationEffect<'_>)->RetainedWorkOutcome<RetainedCloneStep,OriginalRetirementFailure>{
  if !self.closing{let turn=self.begin_close_paid(effect);return RetainedWorkOutcome{result:turn.result.map(RetainedCloneStep::Progress).map_err(Into::into),work:turn.work}}
  macro_rules! owner{($field:ident)=>{if self.fields.$field.is_some(){let turn=self.retirement.begin_granted(&mut self.fields.$field,effect);return RetainedWorkOutcome{result:turn.result.map(|s|s.unwrap_or(RetainedCloneStep::Progress(Default::default()))).map_err(Into::into),work:turn.work}}}}
  owner!(base);owner!(key);owner!(value);owner!(shared_key);owner!(shared_value);owner!(update);RetainedWorkOutcome{result:Ok(RetainedCloneStep::Complete(Default::default())),work:0}
 }
 pub fn close_into<'r>(&mut self,body:usize,effect:&mut RetainedAllocationEffect<'_>,reply:OriginalEffectReply<'r,RetainedCloneStep,OriginalRetirementFailure>)->Result<OriginalEffectReplyComplete<'r,RetainedCloneStep,OriginalRetirementFailure>,ValueError>{if !self.retirement.is_empty(){return self.retirement.step_into(body,effect,reply.progress())}let turn=self.close_native(body,effect);Ok(reply.complete(turn.result))}
 pub fn recover_close_into<'r>(&mut self,body:usize,effect:&mut RetainedAllocationEffect<'_>,reply:OriginalEffectReply<'r,RetainedCloneStep,OriginalRetirementFailure>)->Result<OriginalEffectReplyComplete<'r,RetainedCloneStep,OriginalRetirementFailure>,ValueError>{self.retirement.recover_step_into(body,effect,reply.progress())}
 pub fn close_original_into<'r>(&mut self,o:&mut OriginalRetirementCloseLoan<'_>,body:usize,effect:&mut RetainedAllocationEffect<'_>,reply:OriginalEffectReply<'r,RetainedCloneStep,OriginalRetirementFailure>)->Result<OriginalEffectReplyComplete<'r,RetainedCloneStep,OriginalRetirementFailure>,ValueError>{if !self.retirement.is_empty(){return self.retirement.step_original_into(o,body,effect,reply.progress())}let turn=self.close_native(body,effect);Ok(reply.complete(turn.result))}
 pub fn recover_close_original_into<'r>(&mut self,o:&mut OriginalRetirementCloseLoan<'_>,body:usize,effect:&mut RetainedAllocationEffect<'_>,reply:OriginalEffectReply<'r,RetainedCloneStep,OriginalRetirementFailure>)->Result<OriginalEffectReplyComplete<'r,RetainedCloneStep,OriginalRetirementFailure>,ValueError>{self.retirement.recover_step_original_into(o,body,effect,reply.progress())}
 pub fn terminal_is_empty(&self)->bool{self.closing&&self.retirement.is_empty()&&self.fields.base.is_none()&&self.fields.key.is_none()&&self.fields.value.is_none()&&self.fields.shared_key.is_none()&&self.fields.shared_value.is_none()&&self.fields.update.is_none()}
}
impl<V:RetireOwned+Sync> Drop for OriginalOwnedEntry<V>{fn drop(&mut self){assert!(std::thread::panicking()||self.terminal_is_empty(),"original ordered entry retains its actual owners");if self.terminal_is_empty(){unsafe{ManuallyDrop::drop(&mut self.fields)}}}}
#[derive(Clone,Copy,PartialEq,Eq)]
struct Route{body:usize,demand:RetirementDemand,boundary:crate::OriginalWorkActionBoundary,loan:Option<(crate::NativeAllocationIdentity,Option<u64>)>,invocation:crate::OriginalEffectInvocationIdentity,grant:RetainedCloneGrant,work:u64,original:bool}
#[doc(hidden)]
pub struct EntryRetirement<V:RetireOwned+Sync>{owner:OriginalOwnedEntry<V>,answer:ManuallyDrop<Option<Result<RetainedCloneStep,OriginalRetirementFailure>>>,route:Option<Route>}
impl<V:RetireOwned+Sync> EntryRetirement<V>{
 fn headers()->usize{size_of::<Option<Route>>()+size_of::<Option<Result<RetainedCloneStep,OriginalRetirementFailure>>>()}
 fn drive<'r>(&mut self,mut original:Option<&mut OriginalRetirementCloseLoan<'_>>,body:usize,effect:&mut RetainedAllocationEffect<'_>,reply:OriginalEffectReply<'r,RetirementStep,OriginalRetirementFailure>,recover:bool)->Result<OriginalEffectReplyComplete<'r,RetirementStep,OriginalRetirementFailure>,ValueError>{
  if self.terminal_is_empty(){return Ok(reply.complete(Ok(RetirementStep::Complete(0))))}
  let grant=effect.grant();let before=effect.performed_boundary();let loan=original.as_ref().map(|o|(o.ledger_identity(),o.retirement_recipient_identity()));let key=effect.invocation().ok_or_else(invalid)?.identity();let fresh=self.route.is_none();let route=match self.route{Some(r)=>{if !recover||r.body!=body||r.loan!=loan||r.invocation!=key||r.grant!=grant||r.original!=original.is_some(){return Err(invalid())}r},None=>{if recover{return Err(invalid())}let demand=self.next_demand(body)?;let work=self.next_computational_work(body)?;if !retained_clone_grant_admits(grant,demand)||effect.remaining_work_units()?<work{return Ok(reply.complete(Ok(RetirementStep::BudgetExhausted)))}Route{body,demand,boundary:effect.action_boundary(),loan,invocation:key,grant,work,original:original.is_some()}}};
  if fresh&&!effect.performed(1)?{return Err(invalid())}let mut scope=reply.scope(&mut self.route,route)?;
  if self.answer.is_none(){let child=OriginalEffectReply::borrow(&mut self.answer)?;let child_grant=RetainedCloneGrant{maximum_copy_bytes:grant.maximum_copy_bytes.checked_sub(Self::headers()).ok_or_else(invalid)?,..grant};let remaining=u128::from(route.work.checked_sub(1).ok_or_else(invalid)?).checked_sub(effect.performed_since_action(route.boundary)?).and_then(|work|u64::try_from(work).ok()).ok_or_else(invalid)?;let mut e=effect.reborrow(child_grant,remaining)?;match(original.as_deref_mut(),recover){(Some(o),true)=>self.owner.recover_close_original_into(o,body.saturating_sub(Self::headers()),&mut e,child)?,(Some(o),false)=>self.owner.close_original_into(o,body.saturating_sub(Self::headers()),&mut e,child)?,(None,true)=>self.owner.recover_close_into(body.saturating_sub(Self::headers()),&mut e,child)?,(None,false)=>self.owner.close_into(body.saturating_sub(Self::headers()),&mut e,child)?};}
  if self.answer.is_none(){return Ok(scope.forwarded())}if !effect.performed(1)?{return Err(invalid())}let work=effect.performed_since(before)?;let answer=self.answer.take().expect("actual recorded ordered entry close");let answer=answer.map(|step|{let mut p=step.progress();p.copied_bytes=p.copied_bytes.checked_add(Self::headers()).expect("selected entry reply headers");RetirementStep::Progress(p,work)});scope.borrow()?.complete(answer);Ok(scope.forwarded())
 }
}
impl<V:RetireOwned+Sync> RetirementCursor for EntryRetirement<V>{
 fn close_into<'r>(&mut self,body:usize,e:&mut RetainedAllocationEffect<'_>,r:OriginalEffectReply<'r,RetirementStep,OriginalRetirementFailure>)->Result<OriginalEffectReplyComplete<'r,RetirementStep,OriginalRetirementFailure>,ValueError>{self.drive(None,body,e,r,false)}
 fn recover_close_into<'r>(&mut self,body:usize,e:&mut RetainedAllocationEffect<'_>,r:OriginalEffectReply<'r,RetirementStep,OriginalRetirementFailure>)->Result<OriginalEffectReplyComplete<'r,RetirementStep,OriginalRetirementFailure>,ValueError>{self.drive(None,body,e,r,true)}
 fn close_original_into<'r>(&mut self,o:&mut OriginalRetirementCloseLoan<'_>,body:usize,e:&mut RetainedAllocationEffect<'_>,r:OriginalEffectReply<'r,RetirementStep,OriginalRetirementFailure>)->Result<OriginalEffectReplyComplete<'r,RetirementStep,OriginalRetirementFailure>,ValueError>{self.drive(Some(o),body,e,r,false)}
 fn recover_close_original_into<'r>(&mut self,o:&mut OriginalRetirementCloseLoan<'_>,body:usize,e:&mut RetainedAllocationEffect<'_>,r:OriginalEffectReply<'r,RetirementStep,OriginalRetirementFailure>)->Result<OriginalEffectReplyComplete<'r,RetirementStep,OriginalRetirementFailure>,ValueError>{self.drive(Some(o),body,e,r,true)}
 fn requires_original_birth(&self)->bool{self.route.is_none()&&self.owner.requires_original_birth()}
 fn requires_original_control(&self)->bool{self.route.map_or_else(||self.owner.requires_original_control(),|r|r.original)}
 fn close_original_birth_step(&mut self,o:&mut OriginalRetirementCloseLoan<'_>,g:RetainedCloneGrant)->RetirementStep{crate::retirement::erased_cursor_turn(self.owner.close_original_birth_step(o,g))}
 fn next_demand(&self,b:usize)->Result<RetirementDemand,ValueError>{if let Some(r)=self.route{if r.body!=b{return Err(invalid())}return Ok(r.demand)}let mut d=self.owner.next_close_demands(b.saturating_sub(Self::headers()))?;d.copy_bytes=d.copy_bytes.checked_add(Self::headers()).ok_or_else(invalid)?;Ok(d)}
 fn next_computational_work(&self,b:usize)->Result<u64,ValueError>{self.route.map_or_else(||self.owner.next_close_work_demand(b.saturating_sub(Self::headers()))?.checked_add(2).ok_or_else(invalid),|r|if r.body==b{Ok(r.work)}else{Err(invalid())})}
 fn terminal_is_empty(&self)->bool{self.owner.terminal_is_empty()&&self.answer.is_none()&&self.route.is_none()}
 fn next_work_byte_demand(&self)->Result<usize,ValueError>{Ok(self.next_demand(0)?.copy_bytes)}
 fn next_close_byte_demand(&self)->Option<usize>{self.next_demand(0).ok().map(|d|d.release_bytes)}
 fn next_birth_bytes(&self,b:usize)->Option<usize>{self.next_demand(b).ok().map(|d|d.capacity_bytes)}
 fn next_depth_demand(&self)->Result<usize,ValueError>{Ok(self.next_demand(0)?.depth)}
 fn allows_admitted_narrow_work(&self)->bool{true}
 fn terminal_release_bytes(&self)->Option<usize>{self.terminal_is_empty().then_some(size_of::<Self>())}
}
impl<V:RetireOwned+Sync> Drop for EntryRetirement<V>{fn drop(&mut self){assert!(std::thread::panicking()||self.terminal_is_empty(),"entry retirement retains its original reply");if self.terminal_is_empty(){unsafe{ManuallyDrop::drop(&mut self.answer)}}}}
impl<V:RetireOwned+Sync> RetireOwned for OriginalOwnedEntry<V>{
 type RetirementCursor=EntryRetirement<V>;fn retirement_seed(self)->Self::RetirementCursor{EntryRetirement{owner:self,answer:ManuallyDrop::new(None),route:None}}
 fn retirement_birth_bytes(&self)->Option<usize>{Some(size_of::<Self::RetirementCursor>())}
 fn controlled_retirement_supported()->bool{V::controlled_retirement_supported()}
}
#[cfg(test)]#[path="🧪️.rs"]mod original_owned_entry_tests;
#[path="➖remove/🦀️.rs"]mod original_remove;
