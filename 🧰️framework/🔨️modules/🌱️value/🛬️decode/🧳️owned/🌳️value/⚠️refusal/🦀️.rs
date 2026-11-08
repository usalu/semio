//! ⚠️ Receives the original refusal identity through its retained discriminator.
use crate::{ValueError,ValueRefusalKind,owned::{NativeReceive,NativeMappedReceiver,NativeReceiveMap,NativeRecordFields,NativeRecordSchema,NativeSchemaFields,NativeSchemaLeaf,tree::{NativeValueSource,variant::{NativeUnitVariants,NativeUnitEnumReceiver,NativeVariantSpec}}}};
fn variants(index:usize)->Option<NativeRecordSchema>{
let name=["invalidValue","canceled","ownershipLimit","allocationFailed","workLimit","depthLimit","unsupportedOwner","invariantViolated"].get(index)?;
 Some(NativeRecordSchema::Leaf(NativeSchemaLeaf{name,ordinal:index as u32}))
}
fn data(index:usize)->Option<NativeRecordSchema>{(index<8).then_some(NativeRecordSchema::Skip)}
impl NativeUnitVariants for ValueRefusalKind{
fn variant_spec()->NativeVariantSpec{NativeVariantSpec{all:unsafe{NativeSchemaFields::native(variants)},units:unsafe{NativeSchemaFields::native(variants)},data:unsafe{NativeSchemaFields::native(data)},count:8,tag:None}}
fn from_variant_index(index:usize)->Result<Self,ValueError>{match index{0=>Ok(Self::InvalidValue),1=>Ok(Self::Canceled),2=>Ok(Self::OwnershipLimit),3=>Ok(Self::AllocationFailed),4=>Ok(Self::WorkLimit),5=>Ok(Self::DepthLimit),6=>Ok(Self::UnsupportedOwner),7=>Ok(Self::InvariantViolated),_=>Err(crate::owned::tree::invalid())}}
}
impl<I:NativeValueSource+?Sized> NativeReceive<I> for ValueRefusalKind{
type Receiver=NativeUnitEnumReceiver<I,Self>;
fn receiver()->Self::Receiver{NativeUnitEnumReceiver::new()}
}
/// 🧩️ Typed fields held while the original refusal is assembled.
#[doc(hidden)]
#[derive(semio_framework_value_derive::NativeReceive,semio_framework_value_derive::RetireOwned)]
#[value(crate="crate",native_owned="crate",rename_all="camelCase",deny_unknown_fields)]
pub struct NativeValueErrorFields{kind:ValueRefusalKind,message:String,retained_progress:crate::RetainedCloneProgress}
impl NativeReceiveMap<NativeValueErrorFields> for ValueError{
fn from_native(fields:NativeValueErrorFields)->Self{Self::new(fields.kind,fields.message).with_retained_progress(fields.retained_progress)}
}
impl<I:NativeValueSource+?Sized> NativeReceive<I> for ValueError{
type Receiver=NativeMappedReceiver<I,NativeValueErrorFields,Self>;
fn receiver()->Self::Receiver{NativeMappedReceiver::new()}
}
impl NativeReceiveMap<ValueError> for crate::NativeAllocationRefusal{
fn from_native(error:ValueError)->Self{Self::Owned(error)}
}
impl<I:NativeValueSource+?Sized> NativeReceive<I> for crate::NativeAllocationRefusal{
type Receiver=NativeMappedReceiver<I,ValueError,Self>;
fn receiver()->Self::Receiver{NativeMappedReceiver::new()}
}
/// 📏️ Resident allocation progress fields before cross-field validation.
#[doc(hidden)]
#[derive(semio_framework_value_derive::NativeReceive,semio_framework_value_derive::RetireOwned)]
#[value(crate="crate",native_owned="crate",rename_all="camelCase",deny_unknown_fields)]
pub struct NativeAllocationProgressFields{#[value(required)] prior_admitted_bytes:Option<usize>,#[value(required)] requested_bytes:Option<usize>,#[value(required)] maximum_bytes:Option<usize>,
 admitted_bytes:usize,#[value(required)] actual_capacity_bytes:Option<usize>,
}
impl crate::owned::NativeReceiveTryMap<NativeAllocationProgressFields> for crate::NativeAllocationProgress{
fn validate_native(fields:&NativeAllocationProgressFields)->Result<(),ValueError>{
  if fields.prior_admitted_bytes.is_some()==fields.requested_bytes.is_some()&&fields.requested_bytes.is_some()==fields.maximum_bytes.is_some(){Ok(())}else{Err(ValueError::literal(ValueRefusalKind::InvalidValue,"native allocation progress requires a complete original request"))}
 }
fn from_valid_native(fields:NativeAllocationProgressFields)->Self{Self{prior_admitted_bytes:fields.prior_admitted_bytes,requested_bytes:fields.requested_bytes,maximum_bytes:fields.maximum_bytes,admitted_bytes:fields.admitted_bytes,actual_capacity_bytes:fields.actual_capacity_bytes}}
}
impl<I:NativeValueSource+?Sized> NativeReceive<I> for crate::NativeAllocationProgress{
type Receiver=crate::owned::NativeTryMappedReceiver<I,NativeAllocationProgressFields,Self>;
fn receiver()->Self::Receiver{crate::owned::NativeTryMappedReceiver::new()}
}
/// 🏷️ Keeps the schema discriminator with the actual encoding allocation owner.
#[doc(hidden)]
#[derive(semio_framework_value_derive::NativeReceive,semio_framework_value_derive::RetireOwned)]
#[value(crate="crate",native_owned="crate",tag="type",rename_all="camelCase")]
pub enum NativeTaggedEncodeAllocation{Encode(crate::NativeEncodeAllocationOutcome)}
/// 🧩️ Holds independently received encoding effect children before publication.
#[doc(hidden)]
#[derive(semio_framework_value_derive::NativeReceive,semio_framework_value_derive::RetireOwned)]
#[value(crate="crate",native_owned="crate",rename_all="camelCase",deny_unknown_fields)]
pub struct NativeEncodeEffectFields{#[value(required)]primary:Option<ValueError>,#[value(required)]context:Option<ValueError>,#[value(required)]invariant:Option<ValueError>,#[value(required)]physical_invariant:Option<ValueError>,#[value(required)]capacity_invariant:Option<ValueError>,#[value(required)]marker_failure:Option<ValueError>,#[value(required)]work:Option<ValueError>,#[value(required)]work_overrun:Option<ValueError>,#[value(required)]local:Option<ValueError>,#[value(required)]physical:Option<ValueError>,#[value(required)]physical_receipt:Option<crate::OriginalReceivingPrimary>,#[value(required)]allocation:Option<NativeTaggedEncodeAllocation>,
}
impl NativeReceiveMap<NativeEncodeEffectFields> for crate::NativeEncodeEffectFailure{
fn from_native(fields:NativeEncodeEffectFields)->Self{Self{primary:fields.primary,context:fields.context,invariant:fields.invariant,physical_invariant:fields.physical_invariant,capacity_invariant:fields.capacity_invariant,marker_failure:fields.marker_failure,work:fields.work,work_overrun:fields.work_overrun,local:fields.local,physical:fields.physical,physical_receipt:fields.physical_receipt,allocation:fields.allocation.map(|value|match value{NativeTaggedEncodeAllocation::Encode(owner)=>owner})}}
}
impl<I:NativeValueSource+?Sized> NativeReceive<I> for crate::NativeEncodeEffectFailure{
type Receiver=NativeMappedReceiver<I,NativeEncodeEffectFields,Self>;
fn receiver()->Self::Receiver{NativeMappedReceiver::new()}
}
/// 🪆️ One authenticated recursive retirement node retains its actual child owner.
#[doc(hidden)]
#[derive(semio_framework_value_derive::NativeReceive,semio_framework_value_derive::RetireOwned)]
#[value(crate="crate",native_owned="crate",rename_all="camelCase",deny_unknown_fields)]
pub struct NativeRetirementNodeFields{#[value(required)]primary:Option<ValueError>,#[value(required)]work_refusal:Option<ValueError>,#[value(required)]receipt_refusal:Option<ValueError>,#[value(required)]allocation_refusal:Option<crate::NativeAllocationRefusal>,#[value(required)]progress_override:Option<crate::RetainedCloneProgress>,#[value(required)]child:Option<crate::OriginalRetirementFailure>,
}
/// ♻️ Receives child nodes first and attaches them only to original prepaid storage.
pub struct NativeRetirementFailureReceiver<I:NativeValueSource+?Sized>{
 binding:Option<crate::retained_clone::RetainedCloneBinding>,
 fields:crate::owned::tree::NativeReceiveSlot<I,NativeRetirementNodeFields>,
 pending:std::mem::ManuallyDrop<Option<NativeRetirementNodeFields>>,
 storage:std::mem::ManuallyDrop<Option<crate::OriginalFailureStorage>>,
 output:std::mem::ManuallyDrop<Option<crate::OriginalRetirementFailure>>,
 retirement:crate::retained_clone::RetainedCloneClose,
 birth:Option<crate::RetainedCloneGrant>,
 identity:Option<(crate::NativeAllocationIdentity,Option<u64>)>,phase:u8,closing:bool,
}
impl<I:NativeValueSource+?Sized> NativeReceive<I> for crate::OriginalRetirementFailure{
type Receiver=NativeRetirementFailureReceiver<I>;
fn receiver()->Self::Receiver{NativeRetirementFailureReceiver{binding:None,fields:Default::default(),pending:std::mem::ManuallyDrop::new(None),storage:std::mem::ManuallyDrop::new(None),output:std::mem::ManuallyDrop::new(None),retirement:Default::default(),birth:None,identity:None,phase:0,closing:false}}
}
impl<I:NativeValueSource+?Sized> crate::NativeReceiver<I,crate::OriginalRetirementFailure> for NativeRetirementFailureReceiver<I>{
fn demands(&self,input:crate::retained_clone::RetainedCloneRef<'_,I>,body:usize,maximum_work_units:u64)->Result<crate::RetirementDemand,ValueError>{
  use crate::owned::tree::{fixed,nested,invalid};if self.closing||self.phase>=6{return Err(invalid())}input.check_binding(&self.binding)?;
  match self.phase{0=>Ok(fixed(input.binding_copy_bytes())),1=>if self.fields.ready(){self.fields.take_demand()}else{nested(self.fields.demands(input,body,maximum_work_units)?,0)},2=>Ok(fixed(0)),3=>if let Some(g)=self.birth{Ok(crate::RetirementDemand{copy_bytes:g.maximum_copy_bytes,capacity_bytes:g.maximum_capacity_bytes,release_bytes:g.maximum_release_bytes,depth:g.maximum_depth})}else{Ok(crate::RetirementDemand{capacity_bytes:crate::OriginalFailureStorage::birth_bytes(),depth:1,..Default::default()})},4=>Ok(fixed(std::mem::size_of::<crate::OriginalFailureStorage>())),5=>Ok(fixed(std::mem::size_of::<NativeRetirementNodeFields>().saturating_add(std::mem::size_of::<crate::OriginalRetirementFailure>()))),_=>Err(invalid())}
 }
fn select_advance(&self,input:crate::retained_clone::RetainedCloneRef<'_,I>,_body:usize,maximum_work_units:u64)->Result<semio_framework_value_native_receiver_protocol::NativeAdvanceSelection,ValueError>{
  use crate::owned::tree::invalid;use semio_framework_value_native_receiver_protocol::NativeAdvanceSelection as Selection;
  if self.closing||self.phase>=6{return Err(invalid())}let bound=input.check_binding(&self.binding)?;
  let work=match self.phase{0=>if bound{1}else{input.binding_work_demand(&self.binding)?},1=>if self.fields.ready(){self.fields.take_work_demand()?}else{return Ok(Selection::Nested)},2..=5=>return Ok(Selection::Nested),_=>return Err(invalid())};
  if work>maximum_work_units{return Err(ValueError::literal(ValueRefusalKind::WorkLimit,"retirement receiver action exceeds its original work ceiling"))}Ok(Selection::Local{work})
 }
fn advance_nested(&mut self,input:crate::retained_clone::RetainedCloneRef<'_,I>,control:&mut crate::NativeDecodeControl<'_>,body:usize,maximum_work_units:u64,g:crate::RetainedCloneGrant)->crate::RetainedWorkPoll<crate::RetainedCloneStep,crate::OriginalReceivingPrimary>{
  use crate::RetainedWorkPoll::{Complete,Pending};use crate::owned::tree::{fixed,receipt,child_grant,invalid};use crate::retained_clone::retained_clone_grant_admits;use semio_framework_value_retirement::OriginalRetirementControl;
  let fail=|error:ValueError|->crate::RetainedWorkPoll<crate::RetainedCloneStep,crate::OriginalReceivingPrimary>{Complete(crate::RetainedWorkOutcome{result:Err(error.into()),work:0})};
  if self.closing||self.phase>=6{return fail(invalid())}
  let d=match self.demands(input,body,maximum_work_units){Ok(d)=>d,Err(error)=>return fail(error)};
  if !retained_clone_grant_admits(g,d){return Complete(crate::RetainedWorkOutcome{result:Ok(crate::RetainedCloneStep::Progress(Default::default())),work:0})}
  if let Err(error)=control.checkpoint(){return fail(error)}
  if let Some((ledger,recipient))=self.identity{if control.ledger_identity()!=ledger||control.retirement_recipient_identity()!=recipient{return fail(ValueError::invariant("retirement receiver original recipient changed"))}}
  match self.phase{
   1=>{if self.fields.ready(){return fail(invalid())}self.fields.advance(input,control,body,maximum_work_units,child_grant(g,0)).map(|step|crate::RetainedCloneStep::Progress(step.progress()))}
   3=>{let saved=*self.birth.get_or_insert(crate::RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:0,maximum_capacity_bytes:crate::OriginalFailureStorage::birth_bytes(),maximum_release_bytes:0,maximum_depth:1});if saved.restricted_by(g)!=saved{return Complete(crate::RetainedWorkOutcome{result:Ok(crate::RetainedCloneStep::Progress(Default::default())),work:0})}
    match control.prepare_failure_storage(saved){
     Pending{work}=>Pending{work},
     Complete(turn)=>{let settled=!control.failure_storage_birth_pending();if settled{self.birth=None;if turn.result.is_ok()&&control.has_retirement_failure_storage(){self.phase=4}}Complete(crate::RetainedWorkOutcome{result:turn.result.map(|step|crate::RetainedCloneStep::Progress(step.progress())).map_err(Into::into),work:turn.work})}
    }
   }
   2|4|5=>{
    let result=(||->Result<crate::RetainedCloneStep,crate::OriginalReceivingPrimary>{match self.phase{
     2=>{let needs=self.pending.as_ref().ok_or_else(||ValueError::invariant("retirement node fields absent"))?.child.is_some();if needs{let recipient=control.retirement_recipient_identity().ok_or_else(||ValueError::literal(ValueRefusalKind::UnsupportedOwner,"retirement child requires its original funding recipient"))?;self.identity=Some((control.ledger_identity(),Some(recipient)));self.phase=if control.has_retirement_failure_storage(){4}else{3}}else{self.phase=5}Ok(crate::RetainedCloneStep::Progress(receipt(fixed(0))))},
     4=>{let pool_grant=crate::RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:d.copy_bytes,maximum_depth:1,..Default::default()};let storage=control.with_retirement::<_,ValueError>(pool_grant,|loan|{if !loan.failure_storage_birth().terminal_is_empty(){return Err(ValueError::invariant("retirement failure birth remains unsettled"))}Ok(loan.failure_storage_pool().take())})?;let Some(storage)=storage else{return Err(ValueError::invariant("original prepaid retirement node changed").into())};*self.storage=Some(storage);self.phase=5;Ok(crate::RetainedCloneStep::Progress(receipt(d)))},
     5=>{if self.pending.as_ref().is_some_and(|fields|fields.child.is_some())&&self.storage.is_none(){return Err(ValueError::invariant("prepaid retirement node absent").into())}let fields=self.pending.take().ok_or_else(||ValueError::invariant("retirement fields absent"))?;let NativeRetirementNodeFields{primary,work_refusal,receipt_refusal,allocation_refusal,progress_override,child}=fields;let value=match child{None=>crate::OriginalRetirementFailure::from_owned_parts(primary,work_refusal,receipt_refusal,allocation_refusal,None,progress_override),Some(child)=>{let storage=self.storage.take().ok_or_else(||ValueError::invariant("validated prepaid retirement node absent"))?;let value=storage.attach_owned_parts(child,primary,work_refusal,receipt_refusal,allocation_refusal);match progress_override{Some(progress)=>value.with_retained_progress(progress),None=>value}}};*self.output=Some(value);self.phase=6;Ok(crate::RetainedCloneStep::Complete(receipt(d)))},
     _=>Err(invalid().into())}})();
    let work=u128::from(result.is_ok());Complete(crate::RetainedWorkOutcome{result,work})
   }
   _=>fail(invalid())
  }
 }
fn advance_local<'reply>(&mut self,input:crate::retained_clone::RetainedCloneRef<'_,I>,body:usize,effect:&mut crate::RetainedAllocationEffect<'_>,reply:semio_framework_value_native_receiver_protocol::OriginalEffectReply<'reply,crate::RetainedCloneStep,crate::OriginalReceivingPrimary>)->Result<semio_framework_value_native_receiver_protocol::OriginalEffectReplyComplete<'reply,crate::RetainedCloneStep,crate::OriginalReceivingPrimary>,ValueError>{
  use crate::owned::tree::{fixed,receipt,invalid};use crate::retained_clone::retained_clone_grant_admits;
  let g=effect.grant();let maximum_work_units=effect.maximum_work_units();
  let result:Result<Option<crate::RetainedCloneStep>,crate::OriginalReceivingPrimary>=(||{
   let d=self.demands(input,body,maximum_work_units)?;if !retained_clone_grant_admits(g,d){return Ok(Some(crate::RetainedCloneStep::Progress(Default::default())))}
   match self.phase{
    0=>{if self.binding.is_none(){let turn=input.bind(&mut self.binding,effect);return turn.result.map(|p|Some(crate::RetainedCloneStep::Progress(p.unwrap_or_default()))).map_err(Into::into)}if !effect.performed(1)?{return Err(ValueError::literal(ValueRefusalKind::WorkLimit,"retirement receiver action exceeded its original work").into())}self.phase=1;Ok(Some(crate::RetainedCloneStep::Progress(receipt(fixed(0)))))},
    1=>match self.fields.take(&mut *self.pending,effect){crate::RetainedWorkPoll::Pending{..}=>Ok(None),crate::RetainedWorkPoll::Complete(turn)=>{let step=turn.result?;if self.pending.is_none(){return Ok(Some(crate::RetainedCloneStep::Progress(Default::default())))}self.phase=2;Ok(Some(crate::RetainedCloneStep::Progress(step.progress())))}},
    _=>Err(invalid().into())
   }
  })();
  Ok(match result{Ok(Some(step))=>reply.complete(Ok(step)),Ok(None)=>reply.pending(),Err(error)=>reply.complete(Err(error))})
 }
fn output_ready(&self)->bool{!self.closing&&self.phase==6&&self.output.is_some()}
fn take_demand(&self)->Result<crate::RetirementDemand,ValueError>{if !self.output_ready(){return Err(crate::owned::tree::invalid())}semio_framework_value_native_receiver_protocol::native_output_demand::<crate::OriginalRetirementFailure>(std::mem::size_of::<u8>())}
fn take_work_demand(&self)->Result<u64,ValueError>{self.take_demand().map(|_|1)}
fn take<'reply>(&mut self,output:&mut Option<crate::OriginalRetirementFailure>,effect:&mut crate::RetainedAllocationEffect<'_>,reply:semio_framework_value_native_receiver_protocol::OriginalEffectReply<'reply,crate::RetainedCloneStep,crate::OriginalReceivingPrimary>)->Result<semio_framework_value_native_receiver_protocol::OriginalEffectReplyComplete<'reply,crate::RetainedCloneStep,crate::OriginalReceivingPrimary>,crate::ValueError>{let turn=(||->crate::RetainedWorkPoll<crate::RetainedCloneStep,crate::OriginalReceivingPrimary>{if !self.output_ready(){return crate::RetainedWorkPoll::Complete(crate::RetainedWorkOutcome{result:Err(crate::owned::tree::invalid().into()),work:0})}let turn=semio_framework_value_native_receiver_protocol::native_output_take(&mut self.output,output,std::mem::size_of::<u8>(),effect);if matches!(&turn,crate::RetainedWorkPoll::Complete(crate::RetainedWorkOutcome{result:Ok(crate::RetainedCloneStep::Complete(_)),..})){self.phase=7}turn})();semio_framework_value_native_receiver_protocol::native_record_pure_take(turn,reply)}
semio_framework_value_native_receiver_protocol::native_receiver_close_start!(closing);
fn close_demands(&self,body:usize)->Result<crate::RetirementDemand,ValueError>{use crate::owned::tree::{fixed,nested};if !self.closing{return Err(crate::owned::tree::invalid())}if let Some(g)=self.birth{return Ok(crate::RetirementDemand{copy_bytes:g.maximum_copy_bytes,capacity_bytes:g.maximum_capacity_bytes,release_bytes:g.maximum_release_bytes,depth:g.maximum_depth})}if !self.fields.terminal_is_empty(){return nested(self.fields.close_demands(body)?,0)}if !self.retirement.is_empty(){return self.retirement.demands(body)}if self.pending.is_some(){return self.retirement.next_owner_demands::<NativeRetirementNodeFields>(true,&None,body)}if self.storage.is_some(){return self.retirement.next_owner_demands::<crate::OriginalFailureStorage>(true,&None,body)}if self.output.is_some(){return self.retirement.next_owner_demands::<crate::OriginalRetirementFailure>(true,&None,body)}if self.binding.is_some(){return crate::retained_clone::RetainedCloneBinding::close_demand(&self.binding,body)}Ok(fixed(0))}
fn close_work_demand(&self,body:usize)->Result<u64,ValueError>{if !self.closing{return Err(crate::owned::tree::invalid())}if self.birth.is_some(){return Err(ValueError::literal(ValueRefusalKind::UnsupportedOwner,"pending original birth requires its original loan"))}if !self.fields.terminal_is_empty(){return self.fields.close_work_demand(body)}if !self.retirement.is_empty(){return self.retirement.next_owner_work::<()>(false,&None,body)}if self.pending.is_some(){return self.retirement.next_owner_work::<NativeRetirementNodeFields>(true,&None,body)}if self.storage.is_some(){return self.retirement.next_owner_work::<crate::OriginalFailureStorage>(true,&None,body)}if self.output.is_some(){return self.retirement.next_owner_work::<crate::OriginalRetirementFailure>(true,&None,body)}crate::retained_clone::RetainedCloneBinding::close_work_demand(&self.binding,body)}
fn close_step(&mut self,effect:&mut crate::RetainedAllocationEffect<'_>)->crate::OriginalRetirementPoll{use crate::RetainedWorkPoll::Complete;use crate::retained_clone::retained_clone_grant_admits;let g=effect.grant();
  let fail=|error:ValueError|->crate::OriginalRetirementPoll{Complete(crate::RetainedWorkOutcome{result:Err(error.into()),work:0})};
  if self.requires_original_control(){return fail(ValueError::literal(ValueRefusalKind::UnsupportedOwner,"retirement receiver requires its original close loan"))}
  if self.terminal_is_empty(){return Complete(crate::RetainedWorkOutcome{result:Ok(crate::RetainedCloneStep::Complete(Default::default())),work:0})}
  let d=match self.close_demands(g.maximum_copy_bytes){Ok(d)=>d,Err(error)=>return fail(error)};if !retained_clone_grant_admits(g,d){return Complete(crate::RetainedWorkOutcome{result:Ok(crate::RetainedCloneStep::Progress(Default::default())),work:0})}
  if !self.fields.terminal_is_empty(){let mut child=effect.nested(crate::owned::tree::child_grant(g,0));return self.fields.close_step(&mut child).map(|step|crate::RetainedCloneStep::Progress(step.progress()))}
  if !self.retirement.is_empty(){return self.retirement.step_granted(effect).map(|step|crate::RetainedCloneStep::Progress(step.progress()))}
  if self.pending.is_some(){let turn=self.retirement.begin_granted(&mut self.pending,effect);return Complete(crate::RetainedWorkOutcome{result:turn.result.map(|s|s.unwrap_or(crate::RetainedCloneStep::Progress(Default::default()))).map_err(Into::into),work:turn.work})}
  if self.storage.is_some(){let turn=self.retirement.begin_granted(&mut self.storage,effect);return Complete(crate::RetainedWorkOutcome{result:turn.result.map(|s|s.unwrap_or(crate::RetainedCloneStep::Progress(Default::default()))).map_err(Into::into),work:turn.work})}
  if self.output.is_some(){let turn=self.retirement.begin_granted(&mut self.output,effect);return Complete(crate::RetainedWorkOutcome{result:turn.result.map(|s|s.unwrap_or(crate::RetainedCloneStep::Progress(Default::default()))).map_err(Into::into),work:turn.work})}
  crate::retained_clone::RetainedCloneBinding::close_one(&mut self.binding,effect)}
fn requires_original_birth(&self)->bool{self.birth.is_some()||if !self.fields.terminal_is_empty(){self.fields.requires_original_birth()}else{self.retirement.requires_original_birth()}}
fn close_original_birth_step(&mut self,original:&mut crate::OriginalRetirementCloseLoan<'_>,grant:crate::RetainedCloneGrant)->crate::OriginalRetirementPoll{use crate::RetainedWorkPoll::Complete;let fail=|error:ValueError|->crate::OriginalRetirementPoll{Complete(crate::RetainedWorkOutcome{result:Err(error.into()),work:0})};let idle=||->crate::OriginalRetirementPoll{Complete(crate::RetainedWorkOutcome{result:Ok(crate::RetainedCloneStep::Progress(Default::default())),work:0})};
  if let Some(g)=self.birth{if self.identity!=Some((original.ledger_identity(),original.retirement_recipient_identity())){return fail(ValueError::invariant("retirement close recipient changed"))}if g.restricted_by(grant)!=g{return idle()}let turn=crate::refill_original_failure_storage(original,g);if original.failure_storage_birth().terminal_is_empty(){self.birth=None}return turn}
  if self.fields.requires_original_birth(){if grant.maximum_depth==0{return idle()}return self.fields.close_original_birth_step(original,crate::owned::tree::child_grant(grant,0))}
  if self.retirement.requires_original_birth(){return self.retirement.step_original_birth(original,grant)}
  fail(ValueError::literal(ValueRefusalKind::UnsupportedOwner,"retirement receiver has no original birth to settle"))}
fn requires_original_control(&self)->bool{self.birth.is_some()||if !self.fields.terminal_is_empty(){self.fields.requires_original_control()}else{self.retirement.requires_original_control()}}
fn close_original_step(&mut self,original:&mut crate::OriginalRetirementCloseLoan<'_>,effect:&mut crate::RetainedAllocationEffect<'_>)->crate::OriginalRetirementPoll{
  if self.requires_original_birth(){return crate::RetainedWorkPoll::Complete(crate::RetainedWorkOutcome{result:Err(ValueError::literal(ValueRefusalKind::UnsupportedOwner,"settle original birth before opening a close effect").into()),work:0})}
  if self.fields.requires_original_control(){let g=effect.grant();let mut child=effect.nested(crate::owned::tree::child_grant(g,0));return self.fields.close_original_step(original,&mut child)}
  if self.retirement.requires_original_control(){return self.retirement.step_original(original,effect)}self.close_step(effect)}
fn terminal_is_empty(&self)->bool{self.birth.is_none()&&self.fields.terminal_is_empty()&&self.pending.is_none()&&self.storage.is_none()&&self.output.is_none()&&self.retirement.is_empty()&&self.binding.is_none()}
}
impl<I:NativeValueSource+?Sized> Drop for NativeRetirementFailureReceiver<I>{fn drop(&mut self){use crate::NativeReceiver;let empty=self.terminal_is_empty();assert!(std::thread::panicking()||empty,"retirement receiving requires original funded closure");if empty{unsafe{std::mem::ManuallyDrop::drop(&mut self.pending);std::mem::ManuallyDrop::drop(&mut self.storage);std::mem::ManuallyDrop::drop(&mut self.output)}}}}
/// 🏷️ Carries the actual decoded allocation and its validated wire discriminator.
#[doc(hidden)]
#[derive(semio_framework_value_derive::NativeReceive,semio_framework_value_derive::RetireOwned)]
#[value(crate="crate",native_owned="crate",tag="type",rename_all="camelCase")]
pub enum NativeTaggedDecodeAllocation{Decode(crate::NativeDecodeAllocationOutcome)}
/// 🧩️ Holds each original receiving cause until paid publication.
#[doc(hidden)]
#[derive(semio_framework_value_derive::NativeReceive,semio_framework_value_derive::RetireOwned)]
#[value(crate="crate",native_owned="crate",rename_all="camelCase",deny_unknown_fields)]
pub struct NativeReceivingFields{#[value(required)]primary:Option<ValueError>,#[value(required)]quote_refusal:Option<ValueError>,#[value(required)]allocation_refusal:Option<ValueError>,#[value(required)]work_refusal:Option<ValueError>,#[value(required)]work_overrun:Option<ValueError>,#[value(required)]native_allocation:Option<NativeTaggedDecodeAllocation>,#[value(required)]retirement_failure:Option<crate::OriginalRetirementFailure>,#[value(required)]allocation_failure:Option<crate::NativeAllocationFailure>,#[value(required)]receipt_refusal:Option<ValueError>,#[value(required)]receipt_completion:Option<ValueError>,
 progress:crate::RetainedCloneProgress,
}
impl NativeReceiveMap<NativeReceivingFields> for crate::OriginalReceivingPrimary{
fn from_native(fields:NativeReceivingFields)->Self{Self::from_decoded_fields(fields.primary,fields.quote_refusal,fields.allocation_refusal,fields.work_refusal,fields.work_overrun,fields.retirement_failure,fields.allocation_failure,fields.native_allocation.map(|value|match value{NativeTaggedDecodeAllocation::Decode(owner)=>owner}),fields.receipt_refusal,fields.receipt_completion,fields.progress)}
}
unsafe impl NativeRecordFields for crate::OriginalReceivingPrimary{
fn native_schema_count()->usize{<NativeReceivingFields as NativeRecordFields>::native_schema_count()}
fn native_schema_field(index:usize)->Option<crate::owned::NativeRecordSchema>{<NativeReceivingFields as NativeRecordFields>::native_schema_field(index)}
}
impl<I:NativeValueSource+?Sized> NativeReceive<I> for crate::OriginalReceivingPrimary{
type Receiver=NativeMappedReceiver<I,NativeReceivingFields,Self>;
fn receiver()->Self::Receiver{NativeMappedReceiver::new()}
fn record_subset_receiver()->Option<Self::Receiver>{NativeMappedReceiver::original_record_subset()}
}
#[cfg(test)]
mod tests{
 use super::*;
 #[test]fn refusal_variants_match_wire_schema_and_serde(){
  let schema:serde_json::Value=serde_json::from_str(include_str!("../../../../⚠️refusal/🔁️codec/🧬️schema/🔣️.json")).unwrap();
  let expected=schema["$defs"]["kind"]["enum"].as_array().unwrap();
  assert_eq!(expected.len(),8);
  for (index,wire) in expected.iter().enumerate(){
   let NativeRecordSchema::Leaf(leaf)=variants(index).unwrap() else{panic!("expected leaf")};
   assert_eq!(serde_json::to_value(ValueRefusalKind::from_variant_index(index).unwrap()).unwrap(),*wire);
   assert_eq!(leaf.name,wire.as_str().unwrap());
   assert_eq!(leaf.ordinal,index as u32);
  }
  assert!(variants(expected.len()).is_none());
  assert!(ValueRefusalKind::from_variant_index(expected.len()).is_err());
 }
 #[test]fn refusal_receiver_retains_source_and_resumes_under_single_item_grants(){
  use crate::{DslValue,NativeDecodeControl,NativeReceiver,RetainedAllocationEffect,RetainedCloneGrant};
  let grant=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:65536,maximum_capacity_bytes:65536,maximum_release_bytes:65536,maximum_depth:64};
  let mut wallet=();
  let mut source=crate::source_fixture::capture(DslValue::String("ownershipLimit".into()),&mut wallet,|_|grant,|_,_,_,_,_|{});
  let mut other=crate::source_fixture::capture(DslValue::String("ownershipLimit".into()),&mut wallet,|_|grant,|_,_,_,_,_|{});
  let mut receiver=<ValueRefusalKind as NativeReceive<DslValue>>::receiver();
  let mut callback=|_|true;
  let mut control=NativeDecodeControl::new(65536,&mut callback);let fuel=10000000;
  let denied=RetainedCloneGrant{maximum_items:0,..grant};
  let first=crate::source_fixture::receive(&mut control,denied,crate::source_fixture::WorkLoan::new(&fuel,1),|control|receiver.advance(source.borrow(),control,65536,denied));
  assert_eq!(first.work,0);
  assert_eq!(first.result.unwrap().progress(),Default::default());
  let first=crate::source_fixture::receive(&mut control,grant,crate::source_fixture::WorkLoan::new(&fuel,1),|control|receiver.advance(source.borrow(),control,65536,grant));
  assert!(first.result.unwrap().progress().fits(grant));
  assert!(receiver.demands(other.borrow(),65536).is_err());
  for _ in 0..10000{if receiver.output_ready(){break}let turn=crate::source_fixture::receive(&mut control,grant,crate::source_fixture::WorkLoan::new(&fuel,1),|control|receiver.advance(source.borrow(),control,65536,grant));assert!(turn.result.unwrap().progress().fits(grant));}
  assert!(receiver.output_ready());
  let turn=receiver.take(grant);
  assert_eq!(turn.result.unwrap().unwrap().0,ValueRefusalKind::OwnershipLimit);
  receiver.begin_close();
  let mut actual=None;
  for _ in 0..10000{if receiver.terminal_is_empty(){break}let turn=receiver.close_step(&mut RetainedAllocationEffect::new(grant,u64::MAX,&mut actual));assert!(turn.result.unwrap().progress().fits(grant));}
  assert!(receiver.terminal_is_empty());
  for source in [&mut source,&mut other]{for _ in 0..10000{if source.terminal_is_empty(){break}let turn=source.close_step(&mut RetainedAllocationEffect::new(grant,u64::MAX,&mut actual));assert!(turn.result.unwrap().progress().fits(grant));}assert!(source.terminal_is_empty());}
 }
 #[test]fn allocation_progress_validates_all_presence_combinations(){
  use crate::owned::NativeReceiveTryMap;
  let schema:serde_json::Value=serde_json::from_str(crate::NATIVE_ALLOCATION_SCHEMA).unwrap();let variants=schema["$defs"]["allocationProgress"]["oneOf"].as_array().unwrap();
  for mask in 0..8{let wire=serde_json::json!({"priorAdmittedBytes":if mask&1!=0{Some(1)}else{None},"requestedBytes":if mask&2!=0{Some(2)}else{None},"maximumBytes":if mask&4!=0{Some(3)}else{None},"admittedBytes":4,"actualCapacityBytes":null});let number=|key:&str|wire[key].as_u64().map(|n|n as usize);let fields=NativeAllocationProgressFields{prior_admitted_bytes:number("priorAdmittedBytes"),requested_bytes:number("requestedBytes"),maximum_bytes:number("maximumBytes"),admitted_bytes:number("admittedBytes").unwrap(),actual_capacity_bytes:number("actualCapacityBytes")};let accepted=<crate::NativeAllocationProgress as NativeReceiveTryMap<NativeAllocationProgressFields>>::validate_native(&fields).is_ok();let schema_accepts=variants.iter().any(|branch|["priorAdmittedBytes","requestedBytes","maximumBytes"].into_iter().all(|key|match branch["properties"][key]["type"].as_str().unwrap(){"null"=>wire[key].is_null(),"integer"=>wire[key].as_u64().is_some(),_=>false}));assert_eq!(accepted,schema_accepts);if accepted{let actual=<crate::NativeAllocationProgress as NativeReceiveTryMap<NativeAllocationProgressFields>>::from_valid_native(fields);assert!(actual.request_is_complete());assert_eq!(actual.admitted_bytes,4);}}
 }
 #[test]fn invalid_progress_retains_actual_input_and_refusal_until_funded_close(){
  use crate::{DslValue,NativeDecodeControl,NativeReceiver,RetainedAllocationEffect,RetainedCloneGrant};
  let grant=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:65536,maximum_capacity_bytes:65536,maximum_release_bytes:65536,maximum_depth:64};let mut wallet=();
  let wire=serde_json::json!({"priorAdmittedBytes":1,"requestedBytes":null,"maximumBytes":3,"admittedBytes":0,"actualCapacityBytes":null});
  let mut source=crate::source_fixture::capture(DslValue::from(wire.clone()),&mut wallet,|_|grant,|_,_,_,_,_|{});
  let mut other=crate::source_fixture::capture(DslValue::from(wire),&mut wallet,|_|grant,|_,_,_,_,_|{});
  let mut receiver=<crate::NativeAllocationProgress as NativeReceive<DslValue>>::receiver();let mut callback=|_|true;let mut control=NativeDecodeControl::new(65536,&mut callback);let fuel=10000000;
  for _ in 0..50000{if receiver.validation_refusal().is_some(){break}let turn=receiver.advance(source.borrow(),&mut control,65536,grant);assert!(turn.result.unwrap().progress().fits(grant));}
  let error=receiver.validation_refusal().expect("actual validation refusal retained");assert_eq!(error.kind,ValueRefusalKind::InvalidValue);assert!(receiver.retained_input().is_some());assert!(receiver.demands(other.borrow(),65536).is_err());
  let denied_grant=RetainedCloneGrant{maximum_items:0,..grant};let denied=crate::source_fixture::receive(&mut control,denied_grant,crate::source_fixture::WorkLoan::new(&fuel,1),|control|receiver.advance(source.borrow(),control,65536,denied_grant));assert_eq!(denied.work,0);assert_eq!(denied.result.unwrap().progress(),Default::default());assert!(receiver.validation_refusal().is_some());assert!(receiver.retained_input().is_some());
  receiver.begin_close();let mut actual=None;for _ in 0..50000{if receiver.terminal_is_empty(){break}let turn=receiver.close_step(&mut RetainedAllocationEffect::new(grant,u64::MAX,&mut actual));assert!(turn.result.unwrap().progress().fits(grant));}assert!(receiver.terminal_is_empty());
  for source in [&mut source,&mut other]{for _ in 0..50000{if source.terminal_is_empty(){break}let turn=source.close_step(&mut RetainedAllocationEffect::new(grant,u64::MAX,&mut actual));assert!(turn.result.unwrap().progress().fits(grant));}assert!(source.terminal_is_empty());}
 }
 #[test]fn pending_original_node_birth_survives_cancellation_and_same_loan_closes(){
  use crate::{DslValue,NativeAllocationAuthority,NativeAllocationAuthorityPort,NativeAllocationEvent,NativeAllocationRefusal,NativeDecodeControl,NativeReceiver,RetainedAllocationEffect,RetainedCloneGrant,RetainedCloneProgress,RetirementDemand};
  use crate::native_decoding::{NativeDecodeConstructionRecipient,NativeDecodeRetirementRecipient,OriginalWorkRecipient,OriginalWorkQuote,OriginalWorkQuoteComplete,OriginalWorkActionIssuer};
  use std::sync::atomic::{AtomicBool,AtomicUsize,Ordering};
  struct Work;impl OriginalWorkRecipient for Work{fn admits<'a>(&mut self,q:OriginalWorkQuote<'a>,observed:u64,added:u64)->OriginalWorkQuoteComplete<'a>{q.complete(Ok(observed.checked_add(added).is_some_and(|n|n<=10000000)))}}
  struct Physical{grant:RetainedCloneGrant,submitted:RetainedCloneProgress}impl NativeDecodeConstructionRecipient for Physical{fn submitted(&self)->RetainedCloneProgress{self.submitted}fn grant(&mut self,_:RetirementDemand)->Result<RetainedCloneGrant,ValueError>{Ok(self.grant)}fn accept(&mut self,_:RetainedCloneGrant,p:RetainedCloneProgress)->Result<(),ValueError>{self.submitted=self.submitted.checked_add(p)?;Ok(())}}
  struct Port<'a>{armed:&'a AtomicBool,quotes:&'a AtomicUsize,recoveries:&'a AtomicUsize,saved:Option<NativeAllocationEvent>}
  impl NativeAllocationAuthorityPort for Port<'_>{type Context<'turn,'source>=() where 'source:'turn;fn validate_context<'turn,'source>(&self,_:&())->Result<(),ValueError>where 'source:'turn{Ok(())}fn receive<'turn,'source>(&mut self,_:&mut (),event:NativeAllocationEvent)->Result<(),NativeAllocationRefusal>where 'source:'turn{if matches!(event,NativeAllocationEvent::Quote(_))&&self.armed.swap(false,Ordering::SeqCst){self.quotes.fetch_add(1,Ordering::SeqCst);self.saved=Some(event);panic!("actual original birth quote interrupted after admission")}Ok(())}fn recover<'turn,'source>(&mut self,_:&mut (),event:NativeAllocationEvent)->Result<Option<Result<(),NativeAllocationRefusal>>,ValueError>where 'source:'turn{if self.saved!=Some(event){return Err(ValueError::invariant("original pending birth event changed"))}if self.recoveries.fetch_add(1,Ordering::SeqCst)==0{return Ok(None)}self.saved=None;Ok(Some(Ok(())))}}
  let fixture:serde_json::Value=serde_json::from_str(include_str!("../../../../../🚪️io/⏱️control/🧾️failure/🧪️tests/🧫️fixtures/♻️recursive-retirement.json")).unwrap();let wire=DslValue::from(fixture["cause"]["value"].clone());let grant=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:65536,maximum_capacity_bytes:65536,maximum_release_bytes:65536,maximum_depth:256};let mut wallet=();let mut source=crate::source_fixture::capture(wire,&mut wallet,|_|grant,|_,_,_,_,_|{});let armed=AtomicBool::new(false);let quotes=AtomicUsize::new(0);let recoveries=AtomicUsize::new(0);let mut authority=NativeAllocationAuthority::new(Port{armed:&armed,quotes:&quotes,recoveries:&recoveries,saved:None});
  authority.with_send_loan(&mut (),|allocation|{let mut observe=|_|true;let mut recipient=NativeDecodeRetirementRecipient::new();let mut native=NativeDecodeControl::new_forwarded(1<<30,&mut observe,allocation);native.install_retirement_recipient(&mut recipient).unwrap();let mut work=Work;let mut work_issuer=OriginalWorkActionIssuer::new();let mut work_loan=work_issuer.loan(&mut work);let mut physical=Physical{grant,submitted:Default::default()};let mut physical_issuer=crate::native_decoding::NativeConstructionRecipientIssuer::new();let mut physical_loan=physical_issuer.loan(&mut physical);let mut receiver=<crate::OriginalRetirementFailure as NativeReceive<DslValue>>::receiver();
   native.with_work_recipient(&mut work_loan,|native|{let mut interrupted=false;for _ in 0..1000000{if receiver.phase==3{armed.store(true,Ordering::SeqCst)}let turn=std::panic::catch_unwind(std::panic::AssertUnwindSafe(||native.with_construction_recipient(&mut physical_loan,|native|Ok::<_,ValueError>(receiver.advance(source.borrow(),native,grant.maximum_copy_bytes,grant)))));match turn{Ok(result)=>{let turn=result?;turn.result.unwrap();},Err(_)=>{interrupted=true;break}}}assert!(interrupted);assert_eq!(quotes.load(Ordering::SeqCst),1);assert!(receiver.birth.is_some());assert!(receiver.pending.is_some());receiver.begin_close();assert!(receiver.requires_original_birth());
    let first=native.with_retirement::<_,ValueError>(grant,|loan|loan.with_original_close(|original|Ok(receiver.close_original_birth_step(original,grant))))?;assert_eq!(first.work,0);assert_eq!(first.result.unwrap().progress(),Default::default());assert!(receiver.requires_original_birth());assert_eq!(recoveries.load(Ordering::SeqCst),1);
    for _ in 0..1000000{if receiver.terminal_is_empty(){break}let turn=if receiver.requires_original_birth(){native.with_retirement::<_,ValueError>(grant,|loan|loan.with_original_close(|original|Ok(receiver.close_original_birth_step(original,grant))))?}else{let mut actual=None;let mut effect=RetainedAllocationEffect::new(grant,u64::MAX,&mut actual);if receiver.requires_original_control(){native.with_retirement::<_,ValueError>(grant,|loan|loan.with_original_close(|original|Ok(receiver.close_original_step(original,&mut effect))))?}else{receiver.close_step(&mut effect)}};assert!(turn.result.unwrap().progress().fits(grant));}assert!(receiver.terminal_is_empty());assert_eq!(recoveries.load(Ordering::SeqCst),2);assert_eq!(quotes.load(Ordering::SeqCst),1);
    for _ in 0..1000000{if source.terminal_is_empty(){break}let mut actual=None;source.close_step(&mut RetainedAllocationEffect::new(grant,u64::MAX,&mut actual)).result.unwrap();}assert!(source.terminal_is_empty());while native.has_retirement_owner()||native.has_retirement_failure_storage(){native.close_retirement_recipient(grant).result.unwrap();}Ok::<_,ValueError>(())}).unwrap();
  }).unwrap();assert!(authority.into_port().is_ok());
 }
}
