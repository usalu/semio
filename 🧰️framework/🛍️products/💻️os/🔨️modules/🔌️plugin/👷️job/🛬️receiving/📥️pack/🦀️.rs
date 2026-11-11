//! 🛬️ An original intrinsic Pack and typed receiver settle before their unique completed value crosses a caller boundary.
use semio_framework_value_native_receiving_original::NativeReceivingOriginal;
use semio_os_kernel as store;
use semio_framework_job::{OriginalCallerSourceLoan,OriginalConstructionAdmission,OriginalConstructionDemand,OriginalConstructionFailure,OriginalConstructionStatus,OriginalNativeReceivingOutcome,OriginalSourceBinding,OriginalTurnFailure,StepContext,admit_original_construction};
use semio_framework_value::{DslValue,RetainedAllocationEffect,RetainedCloneGrant,RetainedCloneStep,RetirementDemand,ValueError,ValueRefusalKind,retained_clone::{NativeAllocationIdentity,RetainedCloneSource,RetainedCloneClose}};
use semio_framework_value_native_owned::{NativeReceive,native_decoding::NativeReceivingFailure};
use store::os_pack::intrinsic::{RetainedIntrinsicBody,RetainedIntrinsicInput,RetainedIntrinsicOriginalCall};
use super::original_pack_failure::OriginalCommandPackFailure;
use super::original_steps::{begin,fixed,forgotten,forward,forward_receiving,funds,moved,paid,receipt,retired,settle};
use std::mem::{ManuallyDrop,size_of};
fn invalid()->ValueError{ValueError::literal(ValueRefusalKind::InvariantViolated,"original Pack receiving requires its retained source and caller")}
fn construction(d:RetirementDemand)->OriginalConstructionDemand{OriginalConstructionDemand{items:1,copy_bytes:d.copy_bytes,capacity_bytes:d.capacity_bytes,release_bytes:d.release_bytes,depth:d.depth}}
fn through<R>(cx:&mut StepContext<'_>,retiring:bool,run:impl FnOnce(&mut RetainedAllocationEffect<'_>)->RetainedIntrinsicOriginalCall<R>)->Result<Option<RetainedIntrinsicOriginalCall<R>>,OriginalTurnFailure>{
 if retiring&&cx.retirement_should_yield()||!retiring&&cx.should_yield()||cx.retained_grant().maximum_items==0{return Ok(None)}
 let grant=RetainedCloneGrant{maximum_items:1,..cx.retained_grant()};
 let Some((call,performed))=cx.with_original_effect(grant,|_,effect|run(effect))?else{return Ok(None)};
 cx.consume_retained(call.progress)?;cx.consume_work(performed);Ok(Some(call))
}
fn parser_demand(parser:&RetainedIntrinsicBody<'static>,body:usize)->Result<RetirementDemand,ValueError>{if parser.terminal_is_empty(){Ok(fixed::<Option<RetainedIntrinsicBody<'static>>>())}else{parser.original_close_demands(body).map_err(|_|invalid())}}
/// 🧰️ Shared original intrinsic parser step.
pub fn start(cx:&mut StepContext<'_>,bytes:&mut Option<Vec<u8>>,limits:store::os_pack::PackLimits,maximum:usize,parser:&mut Option<RetainedIntrinsicBody<'static>>,failure:&mut Option<OriginalCommandPackFailure>)->Result<bool,OriginalTurnFailure>{
 if bytes.is_none(){return Err(invalid().into())}
 if cx.should_yield()||cx.retained_grant().maximum_items==0{return Ok(false)}
 let grant=RetainedCloneGrant{maximum_items:1,..cx.retained_grant()};
 let Some((made,performed))=cx.with_original_effect(grant,|_,effect|{let input=RetainedIntrinsicInput::OwnedBytes(bytes.take().expect("start holds its bytes"));RetainedIntrinsicBody::new(input,limits,maximum,effect)})?else{return Ok(false)};
 cx.consume_work(performed);
 match made{Ok(made)=>*parser=Some(made),Err((original,RetainedIntrinsicInput::OwnedBytes(original_bytes)))=>{*bytes=Some(original_bytes);*failure=Some(OriginalCommandPackFailure{original})},Err(_)=>unreachable!()}
 Ok(true)
}
/// 🧰️ Shared original intrinsic parser step.
pub fn parse(cx:&mut StepContext<'_>,parser:&mut RetainedIntrinsicBody<'static>,failure:&mut Option<OriginalCommandPackFailure>)->Result<bool,OriginalTurnFailure>{
 let Some(call)=through(cx,false,|effect|parser.advance_original_step(effect,false))?else{return Ok(false)};
 match call.result{Err(original)=>{*failure=Some(OriginalCommandPackFailure{original});Ok(false)},Ok(step)=>Ok(step.complete)}
}
/// 🧰️ Shared original intrinsic parser step.
pub fn close_parser(cx:&mut StepContext<'_>,parser:&mut ManuallyDrop<Option<RetainedIntrinsicBody<'static>>>,failure:&mut Option<OriginalCommandPackFailure>)->Result<bool,OriginalTurnFailure>{
 let Some(body)=parser.as_mut()else{return Ok(true)};
 if !body.terminal_is_empty(){if let Some(call)=through(cx,true,|effect|body.close_original_step(effect))?{if let Err(original)=call.result{*failure=Some(OriginalCommandPackFailure{original})}}return Ok(false)}
 forgotten(cx,fixed::<Option<RetainedIntrinsicBody<'static>>>(),||{parser.take();})?;Ok(false)
}
/// 🧷️ Every parser, original tree, typed child and caller loan stays in its defining owner until paid closure.
pub struct OriginalPackReceiving<T:NativeReceive<DslValue>+Sync>{
 binding:OriginalSourceBinding,decoder:(NativeAllocationIdentity,u64),body:usize,limits:Option<store::os_pack::PackLimits>,maximum:usize,loan:ManuallyDrop<Option<OriginalCallerSourceLoan>>,returning:bool,
 bytes:ManuallyDrop<Option<Vec<u8>>>,parser:ManuallyDrop<Option<RetainedIntrinsicBody<'static>>>,failure:ManuallyDrop<Option<OriginalCommandPackFailure>>,parsed:ManuallyDrop<Option<DslValue>>,source:ManuallyDrop<Option<RetainedCloneSource<DslValue>>>,admission:ManuallyDrop<Option<semio_framework_value::retained_clone::RetainedCloneSourceAdmission<DslValue,DslValue,()>>>,place:ManuallyDrop<Option<semio_framework_job::OriginalDomainReservation<OriginalCommandPackFailure>>>,receiver:ManuallyDrop<Option<NativeReceivingOriginal<DslValue,T>>>,outcome:ManuallyDrop<Option<OriginalNativeReceivingOutcome>>,native_failure:ManuallyDrop<Option<NativeReceivingFailure>>,pending_ceiling:Option<u64>,output:ManuallyDrop<Option<T>>,retirement:RetainedCloneClose,phase:u8,closing:bool,
}
impl<T:NativeReceive<DslValue>+Sync> OriginalPackReceiving<T>{
 /// 🧮️ Quotes the actual boxed receiving frame the caller must fund before any input transfers.
 pub fn admission_demands()->RetirementDemand{RetirementDemand{capacity_bytes:size_of::<Self>(),..fixed::<Self>()}}
 /// 🎟️ The original byte allocation transfers only after the actual boxed receiving frame is funded.
 pub fn admit(bytes:&mut Vec<u8>,limits:store::os_pack::PackLimits,cx:&mut StepContext<'_>,admission:&mut OriginalConstructionAdmission,failure:&mut Option<OriginalConstructionFailure>,output:&mut Option<Box<Self>>)->Result<bool,OriginalTurnFailure>{
  cx.original_source_parent()?;
  if output.is_some(){return if admission.terminal_is_empty(){Ok(true)}else{admission.reset_step(cx)}}
  if failure.is_some(){return Err(invalid().into())}
  let binding=OriginalSourceBinding::borrow(cx);let body=cx.retained_grant().maximum_copy_bytes;let maximum=usize::try_from(limits.max_total_alloc).map_err(|_|invalid())?;let d=Self::admission_demands();
  let status=admit_original_construction(cx,output,admission,construction(d),1,|native,_|{let result=(||{let decoder=(native.ledger_identity(),native.retirement_recipient_identity().ok_or_else(invalid)?);Ok((Box::new(Self{binding,decoder,body,limits:Some(limits),maximum,loan:ManuallyDrop::new(None),returning:false,bytes:ManuallyDrop::new(Some(std::mem::take(bytes))),parser:ManuallyDrop::new(None),failure:ManuallyDrop::new(None),parsed:ManuallyDrop::new(None),source:ManuallyDrop::new(None),admission:ManuallyDrop::new(None),place:ManuallyDrop::new(None),receiver:ManuallyDrop::new(None),outcome:ManuallyDrop::new(None),native_failure:ManuallyDrop::new(None),pending_ceiling:None,output:ManuallyDrop::new(None),retirement:Default::default(),phase:0,closing:false}),receipt(d)))})();semio_framework_value::RetainedWorkOutcome{work:u128::from(result.is_ok()),result}})?;
  match status{OriginalConstructionStatus::Ready=>admission.reset_step(cx),OriginalConstructionStatus::Pending=>Ok(false),OriginalConstructionStatus::Failed=>{if !admission.take_failure_into_step(failure,cx)?{return Ok(false)}Err(invalid().into())}}
 }
 /// 🌳️ A genuine already-owned intrinsic value enters the same typed receiving continuation without encoding or reparsing it.
 pub fn admit_value(value:&mut Option<DslValue>,cx:&mut StepContext<'_>,admission:&mut OriginalConstructionAdmission,failure:&mut Option<OriginalConstructionFailure>,output:&mut Option<Box<Self>>)->Result<bool,OriginalTurnFailure>{
  cx.original_source_parent()?;
  if output.is_some(){return if admission.terminal_is_empty(){Ok(true)}else{admission.reset_step(cx)}}
  if failure.is_some()||value.is_none(){return Err(invalid().into())}
  let binding=OriginalSourceBinding::borrow(cx);let body=cx.retained_grant().maximum_copy_bytes;let d=Self::admission_demands();
  let status=admit_original_construction(cx,output,admission,construction(d),1,|native,_|{let result=(||{let decoder=(native.ledger_identity(),native.retirement_recipient_identity().ok_or_else(invalid)?);Ok((Box::new(Self{binding,decoder,body,limits:None,maximum:0,loan:ManuallyDrop::new(None),returning:false,bytes:ManuallyDrop::new(None),parser:ManuallyDrop::new(None),failure:ManuallyDrop::new(None),parsed:ManuallyDrop::new(value.take()),source:ManuallyDrop::new(None),admission:ManuallyDrop::new(None),place:ManuallyDrop::new(None),receiver:ManuallyDrop::new(None),outcome:ManuallyDrop::new(None),native_failure:ManuallyDrop::new(None),pending_ceiling:None,output:ManuallyDrop::new(None),retirement:Default::default(),phase:1,closing:false}),receipt(d)))})();semio_framework_value::RetainedWorkOutcome{work:u128::from(result.is_ok()),result}})?;
  match status{OriginalConstructionStatus::Ready=>admission.reset_step(cx),OriginalConstructionStatus::Pending=>Ok(false),OriginalConstructionStatus::Failed=>{if !admission.take_failure_into_step(failure,cx)?{return Ok(false)}Err(invalid().into())}}
 }
 fn validate(&self,cx:&StepContext<'_>)->Result<(),OriginalTurnFailure>{if !self.binding.matches(cx)||cx.native_receiving_identity()!=Some(self.decoder){return Err(invalid().into())}if !self.returning{if let Some(loan)=self.loan.as_ref(){loan.validate_context(cx)?}}Ok(())}
 /// 🪜️ The original typed value becomes ready only after its input, receiver and source loan physically close.
 pub fn advance(&mut self,cx:&mut StepContext<'_>)->Result<bool,OriginalTurnFailure>{
  self.validate(cx)?;if self.closing{return Err(invalid().into())}if cx.is_cancelled(){return Err(ValueError::literal(ValueRefusalKind::Canceled,"original Pack receiving was cancelled").into())}if self.failure.is_some()||self.place.is_some(){if let Some(failure)=self.admit_failure(cx)?{return Err(failure)}return Ok(false)}if self.phase==10{return Ok(true)}if cx.should_yield(){return Ok(false)}if self.phase<9{if !cx.original_source_parent()?.admit_loan(&mut *self.loan,cx)?{return Ok(false)}}
  match self.phase{
   0=>{if self.parser.is_none(){start(cx,&mut self.bytes,self.limits.clone().ok_or_else(invalid)?,self.maximum,&mut self.parser,&mut self.failure)?;return Ok(false)}if parse(cx,self.parser.as_mut().ok_or_else(invalid)?,&mut self.failure)?{self.phase=1}},
   1=>{let d=fixed::<DslValue>();if funds(cx,d){if let Some(parser)=self.parser.as_mut(){*self.parsed=parser.take_output();}let value=self.parsed.take().ok_or_else(invalid)?;*self.admission=Some(RetainedCloneSource::admission_owned(value,()));self.phase=2;paid(cx,d)?}},
   2=>{let admission=self.admission.as_ref().ok_or_else(invalid)?;let ready=admission.phase()=="ready";let d=if ready{fixed::<Option<NativeReceivingOriginal<DslValue,T>>>()}else{admission.next_demand()?};
    forward(cx,d,|effect|{let admission=self.admission.as_mut().expect("admission checked present");if !ready{let turn=admission.advance(effect);return settle(effect,turn.result)}let mut taken=None;let turn=admission.take_output(&mut taken,effect);let result=turn.result.map(|step|{if let Some(source)=taken{*self.receiver=Some(NativeReceivingOriginal::new(source));self.admission.take();self.phase=6;}step});settle(effect,result)})?;},
   3|4|5=>return Err(invalid().into()),
   6=>{
    let receiver=self.receiver.as_mut().ok_or_else(invalid)?;let ceiling=self.pending_ceiling.unwrap_or_else(||cx.fuel_remaining());let d=receiver.demands(self.body,ceiling)?;let work=receiver.advance_work_demand(self.body,ceiling)?;let quote=(RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:d.copy_bytes,maximum_capacity_bytes:d.capacity_bytes,maximum_release_bytes:d.release_bytes,maximum_depth:d.depth},work);let body=self.body;let native_failure=&mut self.native_failure;
    let ran=cx.with_native_receiving_quoted(&mut self.outcome,quote,|native,grant|{let witness=native.source_witness()?;if let semio_framework_value::RetainedWorkPoll::Complete(turn)=receiver.advance(native,witness,body,ceiling,grant){if let Err(failure)=turn.result{**native_failure=Some(failure);return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"original Pack receiver failed; its causes remain retained"))}}Ok(())})?;
    if ran{let held=self.outcome.as_ref().is_some_and(|outcome|outcome.call.is_err()||outcome.source_refusal.is_some());if held||self.native_failure.is_some(){return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"original Pack receiving failed; its complete causes remain retained").into())}*self.outcome=None;if let Some(receiver)=self.receiver.as_ref(){self.pending_ceiling=receiver.advance_pending().then_some(ceiling);if receiver.output_ready(){self.phase=7}}}
   },
   7=>{let receiver=self.receiver.as_mut().ok_or_else(invalid)?;let d=receiver.take_demand()?;forward_receiving(cx,d,|effect|receiver.take(&mut *self.output,effect))?;if self.output.is_some(){self.phase=8}},
   8=>{if self.close_sources(cx)?&&moved(cx,fixed::<u8>(),||{})?{self.phase=9}},
   9=>{if let Some(loan)=self.loan.as_mut(){self.returning=true;loan.close_step(cx.original_source_parent()?,cx)?;if loan.terminal_is_empty(){self.loan.take();}}else if moved(cx,fixed::<u8>(),||{})?{self.phase=10}},
   _=>unreachable!(),
  }Ok(self.phase==10)
 }
 fn admit_failure(&mut self,cx:&mut StepContext<'_>)->Result<Option<OriginalTurnFailure>,OriginalTurnFailure>{if !self.place.as_ref().is_some_and(|place|place.ready()){if self.place.is_none(){*self.place=Some(Default::default());}self.place.as_mut().unwrap().prepare(cx)?;return Ok(None)}let place=self.place.as_mut().unwrap();if !place.pending(){let original=&mut *self.failure;place.with_body(cx,|_,slot|{*slot=original.take();Ok(())})?;}let admitted=semio_framework_job::admit_original_domain_failure(place,cx)?;if self.place.as_ref().is_some_and(|place|place.terminal_is_empty()){*self.place=None;}Ok(admitted)}
 fn close_sources(&mut self,cx:&mut StepContext<'_>)->Result<bool,OriginalTurnFailure>{
  let body=self.body;
  if !self.retirement.is_empty(){let d=self.retirement.next_owner_demands::<()>(false,&None,body)?;retired(cx,d,|effect|self.retirement.step_granted(effect.grant().maximum_copy_bytes,effect))?;return Ok(false)}
  if let Some(admission)=self.admission.as_ref(){let ready=admission.phase()=="ready";let d=if ready{fixed::<Option<RetainedCloneSource<DslValue>>>()}else{admission.next_return_demand()?};
   retired(cx,d,|effect|{let admission=self.admission.as_mut().expect("admission checked present");if ready{let mut taken=None;let turn=admission.take_output(&mut taken,effect);let result=turn.result.map(|step|{if taken.is_some(){*self.source=taken;self.admission.take();}RetainedCloneStep::Progress(step.progress())});return settle(effect,result)}let mut back=None;let turn=admission.return_inputs(&mut back,effect);let result=turn.result.map(|step|{if let Some((value,()))=back{*self.parsed=Some(value);self.admission.take();}RetainedCloneStep::Progress(step.progress())});settle(effect,result)})?;return Ok(false)}
  if self.receiver.is_some(){let d=self.retirement.next_owner_demands::<NativeReceivingOriginal<DslValue,T>>(true,&None,body)?;retired(cx,d,|effect|begin(&mut self.retirement,&mut self.receiver,effect))?;return Ok(false)}
  if self.outcome.is_some(){let d=self.retirement.next_owner_demands::<OriginalNativeReceivingOutcome>(true,&None,body)?;retired(cx,d,|effect|begin(&mut self.retirement,&mut self.outcome,effect))?;return Ok(false)}
  if self.native_failure.is_some(){let d=self.retirement.next_owner_demands::<NativeReceivingFailure>(true,&None,body)?;retired(cx,d,|effect|begin(&mut self.retirement,&mut self.native_failure,effect))?;return Ok(false)}
  if let Some(source)=self.source.as_mut(){if !source.terminal_is_empty(){let d=source.next_close_demand(body)?;retired(cx,d,|effect|source.close_step(body,effect))?;return Ok(false)}forgotten(cx,fixed::<Option<RetainedCloneSource<DslValue>>>(),||{self.source.take();})?;return Ok(false)}
  if self.parsed.is_some(){let d=self.retirement.next_owner_demands::<DslValue>(true,&None,body)?;retired(cx,d,|effect|begin(&mut self.retirement,&mut self.parsed,effect))?;return Ok(false)}
  if self.bytes.is_some(){let d=self.retirement.next_owner_demands::<Vec<u8>>(true,&None,body)?;retired(cx,d,|effect|begin(&mut self.retirement,&mut self.bytes,effect))?;return Ok(false)}
  if self.parser.is_some(){return close_parser(cx,&mut self.parser,&mut self.failure).map(|_|false)}
  Ok(true)
 }
 /// 📤️ A completed value moves once; every original parsing obligation is already terminal.
 pub fn take(&mut self,cx:&mut StepContext<'_>,output:&mut Option<T>)->Result<bool,OriginalTurnFailure>{self.validate(cx)?;if self.phase!=10||self.closing||self.output.is_none()||output.is_some(){return Err(invalid().into())}moved(cx,fixed::<(Option<T>,Option<T>)>(),||{*output=self.output.take();})}
 /// 📐️ The mounted owner can select the actual next original child or source-return event before spending its turn.
 pub fn close_demands(&self)->Result<RetirementDemand,ValueError>{
  let body=self.body;if self.place.is_some(){return Ok(Default::default())}if self.failure.is_some(){return Ok(semio_framework_job::original_domain_failure_demand::<OriginalCommandPackFailure>())}
  if !self.retirement.is_empty(){return self.retirement.next_owner_demands::<()>(false,&None,body)}
  if let Some(admission)=self.admission.as_ref(){return if admission.phase()=="ready"{Ok(fixed::<Option<RetainedCloneSource<DslValue>>>())}else{admission.next_return_demand()}}
  if self.receiver.is_some(){return self.retirement.next_owner_demands::<NativeReceivingOriginal<DslValue,T>>(true,&None,body)}
  if self.outcome.is_some(){return self.retirement.next_owner_demands::<OriginalNativeReceivingOutcome>(true,&None,body)}
  if self.native_failure.is_some(){return self.retirement.next_owner_demands::<NativeReceivingFailure>(true,&None,body)}
  if let Some(source)=self.source.as_ref(){return if source.terminal_is_empty(){Ok(fixed::<Option<RetainedCloneSource<DslValue>>>())}else{source.next_close_demand(body)}}
  if self.parsed.is_some(){return self.retirement.next_owner_demands::<DslValue>(true,&None,body)}
  if self.bytes.is_some(){return self.retirement.next_owner_demands::<Vec<u8>>(true,&None,body)}
  if let Some(parser)=self.parser.as_ref(){return parser_demand(parser,body)}
  if self.output.is_some(){return self.retirement.next_owner_demands::<T>(true,&None,body)}
  Ok(self.loan.as_ref().map_or(Default::default(),|loan|loan.retirement_demands()))
 }
 /// 🛑️ Cancellation retains each original field until its funded close finishes.
 pub fn begin_close(&mut self){self.closing=true}
 pub fn close_step(&mut self,cx:&mut StepContext<'_>)->Result<bool,OriginalTurnFailure>{
  self.validate(cx)?;self.begin_close();if self.failure.is_some()||self.place.is_some(){if let Some(failure)=self.admit_failure(cx)?{return Err(failure)}return Ok(false)}
  if !self.close_sources(cx)?{return Ok(false)}
  if self.output.is_some(){let d=self.retirement.next_owner_demands::<T>(true,&None,self.body)?;retired(cx,d,|effect|begin(&mut self.retirement,&mut self.output,effect))?;return Ok(false)}
  if let Some(loan)=self.loan.as_mut(){self.returning=true;loan.close_step(cx.original_source_parent()?,cx)?;if loan.terminal_is_empty(){self.loan.take();}return Ok(false)}
  Ok(self.terminal_is_empty())
 }
 pub fn terminal_is_empty(&self)->bool{self.loan.is_none()&&self.bytes.is_none()&&self.parser.is_none()&&self.failure.is_none()&&self.place.is_none()&&self.admission.is_none()&&self.parsed.is_none()&&self.source.is_none()&&self.receiver.is_none()&&self.outcome.is_none()&&self.native_failure.is_none()&&self.output.is_none()&&self.retirement.is_empty()}
 pub fn release_demands()->RetirementDemand{RetirementDemand{release_bytes:size_of::<Self>(),depth:1,..Default::default()}}
}
impl<T:NativeReceive<DslValue>+Sync> Drop for OriginalPackReceiving<T>{fn drop(&mut self){assert!(std::thread::panicking()||self.terminal_is_empty(),"original Pack receiving requires complete funded closure")}}

/// 🌳️ An intrinsic Pack returns its actual parsed value after its parser and original source loan close.
pub struct OriginalIntrinsicReceiving{binding:OriginalSourceBinding,decoder:(NativeAllocationIdentity,u64),body:usize,limits:store::os_pack::PackLimits,maximum:usize,loan:ManuallyDrop<Option<OriginalCallerSourceLoan>>,returning:bool,bytes:ManuallyDrop<Option<Vec<u8>>>,parser:ManuallyDrop<Option<RetainedIntrinsicBody<'static>>>,failure:ManuallyDrop<Option<OriginalCommandPackFailure>>,place:ManuallyDrop<Option<semio_framework_job::OriginalDomainReservation<OriginalCommandPackFailure>>>,output:ManuallyDrop<Option<DslValue>>,retirement:RetainedCloneClose,phase:u8,closing:bool}
impl OriginalIntrinsicReceiving{
 /// 🧮️ Quotes the actual boxed receiving frame the caller must fund before any input transfers.
 pub fn admission_demands()->RetirementDemand{RetirementDemand{capacity_bytes:size_of::<Self>(),..fixed::<Self>()}}
 /// 🌱️ The original byte allocation transfers only after its receiving frame is funded.
 pub fn admit(bytes:&mut Vec<u8>,limits:store::os_pack::PackLimits,cx:&mut StepContext<'_>,admission:&mut OriginalConstructionAdmission,failure:&mut Option<OriginalConstructionFailure>,output:&mut Option<Box<Self>>)->Result<bool,OriginalTurnFailure>{
  cx.original_source_parent()?;
  if output.is_some(){return if admission.terminal_is_empty(){Ok(true)}else{admission.reset_step(cx)}}
  if failure.is_some(){return Err(invalid().into())}
  let binding=OriginalSourceBinding::borrow(cx);let body=cx.retained_grant().maximum_copy_bytes;let maximum=usize::try_from(limits.max_total_alloc).map_err(|_|invalid())?;let d=Self::admission_demands();
  let status=admit_original_construction(cx,output,admission,construction(d),1,|native,_|{let result=(||{let decoder=(native.ledger_identity(),native.retirement_recipient_identity().ok_or_else(invalid)?);Ok((Box::new(Self{binding,decoder,body,limits,maximum,loan:ManuallyDrop::new(None),returning:false,bytes:ManuallyDrop::new(Some(std::mem::take(bytes))),parser:ManuallyDrop::new(None),failure:ManuallyDrop::new(None),place:ManuallyDrop::new(None),output:ManuallyDrop::new(None),retirement:Default::default(),phase:0,closing:false}),receipt(d)))})();semio_framework_value::RetainedWorkOutcome{work:u128::from(result.is_ok()),result}})?;
  match status{OriginalConstructionStatus::Ready=>admission.reset_step(cx),OriginalConstructionStatus::Pending=>Ok(false),OriginalConstructionStatus::Failed=>{if !admission.take_failure_into_step(failure,cx)?{return Ok(false)}Err(invalid().into())}}
 }
 fn validate(&self,cx:&StepContext<'_>)->Result<(),OriginalTurnFailure>{if !self.binding.matches(cx)||cx.native_receiving_identity()!=Some(self.decoder){return Err(invalid().into())}if !self.returning{if let Some(loan)=self.loan.as_ref(){loan.validate_context(cx)?}}Ok(())}
 fn admit_failure(&mut self,cx:&mut StepContext<'_>)->Result<Option<OriginalTurnFailure>,OriginalTurnFailure>{if !self.place.as_ref().is_some_and(|place|place.ready()){if self.place.is_none(){*self.place=Some(Default::default());}self.place.as_mut().unwrap().prepare(cx)?;return Ok(None)}let place=self.place.as_mut().unwrap();if !place.pending(){let original=&mut *self.failure;place.with_body(cx,|_,slot|{*slot=original.take();Ok(())})?;}let admitted=semio_framework_job::admit_original_domain_failure(place,cx)?;if self.place.as_ref().is_some_and(|place|place.terminal_is_empty()){*self.place=None;}Ok(admitted)}
 fn close_parser(&mut self,cx:&mut StepContext<'_>)->Result<bool,OriginalTurnFailure>{close_parser(cx,&mut self.parser,&mut self.failure)}
 /// 🪜️ Advances the original parser, then settles its source before making the value available.
 pub fn advance(&mut self,cx:&mut StepContext<'_>)->Result<bool,OriginalTurnFailure>{self.validate(cx)?;if self.closing{return Err(invalid().into())}if cx.is_cancelled(){return Err(ValueError::literal(ValueRefusalKind::Canceled,"original intrinsic receiving was cancelled").into())}if self.failure.is_some()||self.place.is_some(){if let Some(failure)=self.admit_failure(cx)?{return Err(failure)}return Ok(false)}if self.phase==4{return Ok(true)}if cx.should_yield(){return Ok(false)}if self.phase<3&&!cx.original_source_parent()?.admit_loan(&mut self.loan,cx)?{return Ok(false)}match self.phase{
  0=>{if self.parser.is_none(){start(cx,&mut self.bytes,self.limits.clone(),self.maximum,&mut self.parser,&mut self.failure)?;return Ok(false)}if parse(cx,self.parser.as_mut().ok_or_else(invalid)?,&mut self.failure)?{self.phase=1}},
  1=>{let d=fixed::<DslValue>();if funds(cx,d){let value=self.parser.as_mut().ok_or_else(invalid)?.take_output().ok_or_else(invalid)?;*self.output=Some(value);self.phase=2;paid(cx,d)?}},
  2=>{if self.close_parser(cx)?&&moved(cx,fixed::<u8>(),||{})?{self.phase=3}},
  3=>{if let Some(loan)=self.loan.as_mut(){self.returning=true;loan.close_step(cx.original_source_parent()?,cx)?;if loan.terminal_is_empty(){self.loan.take();}}else if moved(cx,fixed::<u8>(),||{})?{self.phase=4}},_=>return Err(invalid().into())}Ok(self.phase==4)}
 /// 📤️ Hands off the unique parsed tree once the original parser and loan are terminal.
 pub fn take(&mut self,cx:&mut StepContext<'_>,output:&mut Option<DslValue>)->Result<bool,OriginalTurnFailure>{self.validate(cx)?;if self.phase!=4||self.closing||self.output.is_none()||output.is_some(){return Err(invalid().into())}moved(cx,fixed::<(Option<DslValue>,Option<DslValue>)>(),||{*output=self.output.take();})}
 /// 📐️ Quotes the next actual parser, value or source-loan retirement turn.
 pub fn close_demands(&self)->Result<RetirementDemand,ValueError>{if self.place.is_some(){return Ok(Default::default())}if self.failure.is_some(){return Ok(semio_framework_job::original_domain_failure_demand::<OriginalCommandPackFailure>())}if !self.retirement.is_empty(){return self.retirement.next_owner_demands::<()>(false,&None,self.body)}if self.bytes.is_some(){return self.retirement.next_owner_demands::<Vec<u8>>(true,&None,self.body)}if let Some(parser)=self.parser.as_ref(){return parser_demand(parser,self.body)}if self.output.is_some(){return self.retirement.next_owner_demands::<DslValue>(true,&None,self.body)}Ok(self.loan.as_ref().map_or(Default::default(),|loan|loan.retirement_demands()))}
 /// 🛑️ Cancellation preserves all original owners until their funded close completes.
 pub fn close_step(&mut self,cx:&mut StepContext<'_>)->Result<bool,OriginalTurnFailure>{
  self.validate(cx)?;self.closing=true;let body=self.body;if self.failure.is_some()||self.place.is_some(){if let Some(failure)=self.admit_failure(cx)?{return Err(failure)}return Ok(false)}
  if !self.retirement.is_empty(){let d=self.retirement.next_owner_demands::<()>(false,&None,body)?;retired(cx,d,|effect|self.retirement.step_granted(effect.grant().maximum_copy_bytes,effect))?;return Ok(false)}
  if self.bytes.is_some(){let d=self.retirement.next_owner_demands::<Vec<u8>>(true,&None,body)?;retired(cx,d,|effect|begin(&mut self.retirement,&mut self.bytes,effect))?;return Ok(false)}
  if self.parser.is_some(){self.close_parser(cx)?;return Ok(false)}
  if self.output.is_some(){let d=self.retirement.next_owner_demands::<DslValue>(true,&None,body)?;retired(cx,d,|effect|begin(&mut self.retirement,&mut self.output,effect))?;return Ok(false)}
  if let Some(loan)=self.loan.as_mut(){self.returning=true;loan.close_step(cx.original_source_parent()?,cx)?;if loan.terminal_is_empty(){self.loan.take();}return Ok(false)}Ok(self.terminal_is_empty())}
 pub fn terminal_is_empty(&self)->bool{self.loan.is_none()&&self.bytes.is_none()&&self.parser.is_none()&&self.failure.is_none()&&self.place.is_none()&&self.output.is_none()&&self.retirement.is_empty()}
 pub fn release_demands()->RetirementDemand{RetirementDemand{release_bytes:size_of::<Self>(),depth:1,..Default::default()}}
}
impl Drop for OriginalIntrinsicReceiving{fn drop(&mut self){assert!(std::thread::panicking()||self.terminal_is_empty(),"original intrinsic receiving requires complete funded closure")}}

#[cfg(test)]
#[path="🧪️tests/🦀️.rs"]
mod tests;
