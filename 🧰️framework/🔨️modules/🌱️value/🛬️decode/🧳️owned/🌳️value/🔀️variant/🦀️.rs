//! 🔀️ Variant selection retains actual original discriminators and bounded schema comparisons.
use crate::{NativeDecodeControl,OriginalReceivingPrimary,RetainedWorkOutcome,RetirementDemand,RetainedCloneGrant,RetainedCloneProgress,RetainedCloneStep,ValueError,retained_clone::{RetainedCloneBinding,RetainedCloneClose,RetainedCloneRef,retained_clone_grant_admits},owned::{native_close_settle,NativeRecordKey,NativeRecordMatcher,NativeRecordSchema,NativeSchemaFields,NativeSchemaLeaf,NativeSchemaKeySource,NativeFieldMatch,tree::{NativeValueSource,NativeTreeKind,fixed,receipt,invalid,nested,child_grant}}};
pub use semio_framework_value_native_record_machine::selector::{NativeVariantSpec,NativeVariantSelection,NativeVariantSelector};

#[path="🔢️unit/🦀️.rs"]mod unit;
pub use unit::{NativeUnitVariants,NativeUnitEnumReceiver};

use crate::owned::{NativeReceive,NativeReceiver,NativeAdvanceSelection,tree::{NativeReceiveSlot}};
use crate::{OriginalEffectReply,OriginalEffectReplyComplete};
use crate::retained_clone::RetainedOwnedProjection;
use std::mem::ManuallyDrop;
fn result_variant(index:usize)->Option<NativeRecordSchema>{["Ok","Err"].get(index).map(|name|NativeRecordSchema::Leaf(NativeSchemaLeaf{name,ordinal:index as u32}))}
fn result_unit(index:usize)->Option<NativeRecordSchema>{(index<2).then_some(NativeRecordSchema::Skip)}
pub struct NativeResultReceiver<I:NativeValueSource+?Sized,T:NativeReceive<dyn NativeValueSource>,E:NativeReceive<dyn NativeValueSource>>{selector:NativeVariantSelector<I>,ok:NativeReceiveSlot<dyn NativeValueSource,T>,err:NativeReceiveSlot<dyn NativeValueSource,E>,staged_ok:std::mem::ManuallyDrop<Option<T>>,staged_err:std::mem::ManuallyDrop<Option<E>>,retirement:RetainedCloneClose,closing:bool,taken:bool}
impl<I:NativeValueSource+?Sized,T:NativeReceive<dyn NativeValueSource>,E:NativeReceive<dyn NativeValueSource>> NativeReceive<I> for Result<T,E>{type Receiver=NativeResultReceiver<I,T,E>;fn receiver()->Self::Receiver{NativeResultReceiver{selector:NativeVariantSelector::new(NativeVariantSpec{all:unsafe{NativeSchemaFields::native(result_variant)},units:unsafe{NativeSchemaFields::native(result_unit)},data:unsafe{NativeSchemaFields::native(result_variant)},statements:unsafe{NativeSchemaFields::native(result_variant)},count:2,tag:None}),ok:Default::default(),err:Default::default(),staged_ok:std::mem::ManuallyDrop::new(None),staged_err:std::mem::ManuallyDrop::new(None),retirement:Default::default(),closing:false,taken:false}}}
impl<I:NativeValueSource+?Sized,T:NativeReceive<dyn NativeValueSource>,E:NativeReceive<dyn NativeValueSource>> NativeResultReceiver<I,T,E>{
 fn payload<'a>(&self,input:RetainedCloneRef<'a,I>)->Result<RetainedCloneRef<'a,dyn NativeValueSource>,ValueError>{let selected=self.selector.result().ok_or_else(invalid)?;if selected.direct{input.get().variant_payload()?;}else{input.get().record_child(0)?;}Ok(input.project(selected.index,move|value|if selected.direct{value.variant_payload().expect("bound Result variant payload")}else{value.record_child(0).expect("bound Result record payload")}))}
 fn child_ready(&self)->bool{self.selector.result().is_some_and(|selected|if selected.index==0{self.staged_ok.is_some()||self.ok.ready()}else{self.staged_err.is_some()||self.err.ready()})}
}
impl<I:NativeValueSource+?Sized,T:NativeReceive<dyn NativeValueSource>,E:NativeReceive<dyn NativeValueSource>> NativeReceiver<I,Result<T,E>> for NativeResultReceiver<I,T,E>{
 fn demands(&self,input:RetainedCloneRef<'_,I>,body:usize,maximum_work_units:u64)->Result<RetirementDemand,ValueError>{if self.closing||self.taken{return Err(invalid())}self.selector.validate_input(input)?;let Some(selected)=self.selector.result()else{return self.selector.demands(input,body,maximum_work_units)};if self.child_ready(){return self.selector.settlement_demand(body)}let payload=self.payload(input)?;nested(if selected.index==0{self.ok.demands(payload,body,maximum_work_units)?}else{self.err.demands(payload,body,maximum_work_units)?},0)}
 fn select_advance(&self,input:RetainedCloneRef<'_,I>,body:usize,maximum_work_units:u64)->Result<NativeAdvanceSelection,ValueError>{if self.closing||self.taken{return Err(invalid())}self.selector.validate_input(input)?;if self.selector.result().is_none(){return self.selector.select_advance(input,body,maximum_work_units)}if self.child_ready(){return Ok(NativeAdvanceSelection::Local{work:self.selector.settlement_work_demand(body)?})}Ok(NativeAdvanceSelection::Nested)}
 fn advance_nested(&mut self,input:RetainedCloneRef<'_,I>,control:&mut NativeDecodeControl<'_>,witness:semio_framework_value_control_contract::OriginalSourceTurnWitness<'_>,body:usize,maximum_work_units:u64,g:RetainedCloneGrant)->crate::RetainedWorkPoll<RetainedCloneStep,crate::native_decoding::NativeReceivingFailure>{
  let Some(selected)=self.selector.result()else{return self.selector.advance_nested(input,control,witness,body,maximum_work_units,g)};if self.child_ready(){return crate::RetainedWorkPoll::Complete(RetainedWorkOutcome{result:Err(invalid().into()),work:0})}
  let payload=match self.payload(input){Ok(payload)=>payload,Err(error)=>return crate::RetainedWorkPoll::Complete(RetainedWorkOutcome{result:Err(error.into()),work:0})};
  if selected.index==0{self.ok.advance(payload,control,witness,body,maximum_work_units,child_grant(g,0)).map(|s|RetainedCloneStep::Progress(s.progress()))}else{self.err.advance(payload,control,witness,body,maximum_work_units,child_grant(g,0)).map(|s|RetainedCloneStep::Progress(s.progress()))}
 }
 fn advance_local<'reply>(&mut self,input:RetainedCloneRef<'_,I>,body:usize,effect:&mut crate::RetainedAllocationEffect<'_>,reply:OriginalEffectReply<'reply,RetainedCloneStep,OriginalReceivingPrimary>)->Result<OriginalEffectReplyComplete<'reply,RetainedCloneStep,OriginalReceivingPrimary>,ValueError>{
  if self.selector.result().is_none(){return self.selector.advance_local(input,body,effect,reply.progress())}if self.child_ready(){return self.selector.settle_into(effect,reply)}Ok(reply.complete(Err(invalid().into())))
 }
 fn advance_pending(&self)->bool{self.selector.advance_pending()||self.ok.advance_pending()||self.err.advance_pending()}
 fn output_ready(&self)->bool{!self.closing&&!self.taken&&self.child_ready()&&self.selector.settled()}
 semio_framework_value_native_receiver_protocol::result_output_engine!();
 semio_framework_value_native_receiver_protocol::native_receiver_close_start!(closing);
 semio_framework_value_native_receiver_protocol::result_close_engine!();
 fn native_terminal_release(&self)->Option<semio_framework_value_native_receiver_protocol::RetainedCursorRelease<Self>>{self.terminal_is_empty().then(||unsafe{semio_framework_value_native_receiver_protocol::RetainedCursorRelease::native(|value:&Self|value.terminal_is_empty())})}
  fn terminal_is_empty(&self)->bool{self.ok.terminal_is_empty()&&self.err.terminal_is_empty()&&self.staged_ok.is_none()&&self.staged_err.is_none()&&self.retirement.is_empty()&&self.selector.terminal_is_empty()}
}

impl<I:NativeValueSource+?Sized,T:NativeReceive<dyn NativeValueSource>,E:NativeReceive<dyn NativeValueSource>> Drop for NativeResultReceiver<I,T,E>{fn drop(&mut self){let empty=self.terminal_is_empty();assert!(std::thread::panicking()||empty,"native result receiver requires funded closure");if empty{unsafe{std::mem::ManuallyDrop::drop(&mut self.staged_ok);std::mem::ManuallyDrop::drop(&mut self.staged_err);}}}}
