//! 🧬️ The installed receiver retains the exact replacement payload until every original typed projection returns.
use crate::{ArtifactOperationWire,ArtifactOriginalOperationReceiving};
use semio_framework_os_spr::InputReplacement;
use semio_framework_value::{ArtifactOwnedValueRetirementFactory,RetainedCloneGrant,RetainedCloneProgress,RetainedCloneStep,RetainedCloneSource,RetainedOwnedProjection,RetirementDemand,ValueError,ValueRefusalKind};
use semio_framework_value::retained_clone::RetainedCloneSourceTake;
use semio_framework_value_native_owned::native_decoding::{NativeDecodeControl,NativeRetirementDecodeControl};
use std::{mem::size_of,sync::Arc};
/// 🧳️ The original input row, native decoder ledger and retirement recipient remain bound through cancellation.
pub struct OriginalReplacementReceiving<M:Sync>{source:Option<RetainedCloneSource<Vec<u8>>>,wire:Option<RetainedOwnedProjection<Vec<u8>>>,receiver:Option<Box<dyn ArtifactOriginalOperationReceiving<M>>>,input:Option<RetainedOwnedProjection<M>>,binding:Option<(u64,u64,usize)>,closing:bool}
impl<M:Sync> Default for OriginalReplacementReceiving<M>{fn default()->Self{Self{source:None,wire:None,receiver:None,input:None,binding:None,closing:false}}}
fn invalid()->ValueError{ValueError::literal(ValueRefusalKind::InvariantViolated,"replacement receiving requires its original input row, decoder and retirement recipient")}
fn header<T>()->RetirementDemand{RetirementDemand{copy_bytes:size_of::<T>(),depth:1,..Default::default()}}
fn permits(grant:RetainedCloneGrant,demand:RetirementDemand)->bool{semio_framework_value::retained_clone::retained_clone_grant_admits(grant,demand)}
fn progress(d:RetirementDemand)->RetainedCloneProgress{RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,retained_capacity_bytes:d.capacity_bytes,released_bytes:d.release_bytes}}
impl<M:Sync+'static> OriginalReplacementReceiving<M>{
 fn validate(&self,row:&InputReplacement,ledger:u64,recipient:Option<u64>)->Result<(u64,u64,usize),ValueError>{let identity=(ledger,recipient.ok_or_else(||ValueError::literal(ValueRefusalKind::UnsupportedOwner,"replacement receiving requires the original native retirement recipient"))?,row as*const _ as usize);if self.binding.is_some_and(|original|original!=identity){return Err(invalid())}Ok(identity)}
 /// 🎟️ Reports actual source custody without exposing or fabricating its projection authority.
 pub fn retains_source(&self)->bool{self.binding.is_some()}
 /// 📬️ A ready projection still belongs to the original receiver until its funded transfer.
 pub fn ready(&self)->bool{self.input.is_some()}
 /// 👁️ Borrows the unchanged original input wire while its real receiving source retains custody.
 pub fn borrow_payload(&self)->Result<semio_framework_value::RetainedCloneRef<'_,Vec<u8>>,ValueError>{if self.closing{return Err(invalid())}self.source.as_ref().ok_or_else(invalid)?.try_borrow()}
 /// 📏️ Prices one actual source capture, wire alias, receiver birth or native receiving step.
 pub fn demands(&self,codec:&ArtifactOperationWire<M>,body:usize)->Result<RetirementDemand,ValueError>{
  if self.source.is_none(){let mut d=RetirementDemand{capacity_bytes:RetainedCloneSource::<Vec<u8>>::owned_constructor_capacity_bytes::<()>(),..header::<Self>()};d.copy_bytes+=size_of::<Vec<u8>>();return Ok(d)}
  if self.wire.is_none()&&self.receiver.is_none(){return Ok(RetainedOwnedProjection::<Vec<u8>>::constructor_demand())}
  if self.receiver.is_none(){return Ok((codec.original_receiving.as_ref().ok_or_else(||ValueError::literal(ValueRefusalKind::UnsupportedOwner,"installed operation wire has no original receiving capability"))?.demands)())}
  let receiver=self.receiver.as_ref().unwrap();if receiver.ready(){return Ok(header::<Option<RetainedOwnedProjection<M>>>())}receiver.demands(body)
 }
 /// 🪡️ Every native birth uses the same decoder and independent grant; the source Vec is never copied or replaced.
 pub fn advance_native(&mut self,row:&mut InputReplacement,codec:&ArtifactOperationWire<M>,factory:&Arc<dyn ArtifactOwnedValueRetirementFactory<M>>,control:&mut NativeDecodeControl<'_>,grant:RetainedCloneGrant)->Result<RetainedCloneProgress,ValueError>{
  let identity=self.validate(row,control.ledger_identity(),control.retirement_recipient_identity())?;if self.closing{return Err(invalid())}if self.ready(){return Ok(Default::default())}let d=self.demands(codec,grant.maximum_copy_bytes)?;if !permits(grant,d){return Ok(Default::default())}
  if self.source.is_none(){let InputReplacement::Input{payload,..}=row else{return Err(invalid())};control.admit_turn_capacity(grant.maximum_capacity_bytes)?;control.charge(d.capacity_bytes)?;let bytes=std::mem::take(payload);return match RetainedCloneSource::admit_owned(bytes,(),grant){Ok((source,_))=>{self.source=Some(source);self.binding=Some(identity);Ok(progress(d))},Err((error,bytes,()))=>{*payload=bytes;Err(error)}}}
  if self.wire.is_none()&&self.receiver.is_none(){let(wire,p)=self.source.as_ref().unwrap().retain_original(grant)?;self.wire=Some(wire);return Ok(p)}
  if self.receiver.is_none(){return (codec.original_receiving.as_ref().unwrap().admit)(&mut self.wire,&mut self.receiver,factory,control,grant)}
  let receiver=self.receiver.as_mut().unwrap();if receiver.ready(){if let Some((input,p))=receiver.take(grant)?{self.input=Some(input);return Ok(p)}return Ok(Default::default())}receiver.advance(control,grant.maximum_copy_bytes,grant).map(|step|step.progress())
 }
 /// 🤝️ Transfers only the original typed projection after authenticating the same decoder and input recipient.
 pub fn take_input_native(&mut self,output:&mut Option<RetainedOwnedProjection<M>>,control:&NativeDecodeControl<'_>,grant:RetainedCloneGrant)->Result<RetainedCloneProgress,ValueError>{if self.binding.is_some_and(|(ledger,recipient,_)|ledger!=control.ledger_identity()||Some(recipient)!=control.retirement_recipient_identity()){return Err(invalid())}if output.is_some(){return Err(invalid())}let d=header::<Option<RetainedOwnedProjection<M>>>();if self.input.is_none()||!permits(grant,d){return Ok(Default::default())}*output=self.input.take();Ok(progress(d))}
 /// 🛑️ Stops receiving before original semantic producers return their projection aliases.
 pub fn begin_close(&mut self){self.closing=true;if let Some(receiver)=self.receiver.as_mut(){receiver.begin_close();}}
 /// 📏️ Includes actual receiver release, source handback and the original binding header.
 pub fn close_demands(&self,body:usize)->Result<RetirementDemand,ValueError>{
  if let Some(input)=self.input.as_ref(){return if input.terminal_is_empty(){Ok(header::<Option<RetainedOwnedProjection<M>>>())}else{input.close_demand(body)}}
  if let Some(receiver)=self.receiver.as_ref(){return if receiver.terminal_is_empty(){Ok(RetirementDemand{release_bytes:receiver.allocation_bytes(),..header::<Option<Box<dyn ArtifactOriginalOperationReceiving<M>>>>()})}else{receiver.close_demands(body)}}
  if let Some(wire)=self.wire.as_ref(){return if wire.terminal_is_empty(){Ok(header::<Option<RetainedOwnedProjection<Vec<u8>>>>())}else{wire.close_demand(body)}}
  if let Some(source)=self.source.as_ref(){return if source.terminal_is_empty(){Ok(header::<Option<RetainedCloneSource<Vec<u8>>>>())}else{source.next_take_demand(body)}}
  Ok(header::<Option<(u64,u64,usize)>>())
 }
 /// ↩️ Restores the same original Vec to the exact source row after every typed and wire alias has closed.
 pub fn close_native(&mut self,row:&mut InputReplacement,control:&mut dyn semio_framework_value::OriginalRetirementControl,grant:RetainedCloneGrant)->Result<RetainedCloneStep,ValueError>{
  self.validate(row,control.ledger_identity(),control.retirement_recipient_identity())?;self.begin_close();if self.terminal_is_empty(){return Ok(RetainedCloneStep::Complete(Default::default()))}let d=self.close_demands(grant.maximum_copy_bytes)?;if !permits(grant,d){return Ok(RetainedCloneStep::Progress(Default::default()))}
  if let Some(input)=self.input.as_mut(){if input.terminal_is_empty(){self.input=None;return Ok(RetainedCloneStep::Progress(progress(d)))}control.charge(d.capacity_bytes)?;return input.close_step(grant).map(|step|RetainedCloneStep::Progress(step.progress()))}
  if let Some(receiver)=self.receiver.as_mut(){if !receiver.terminal_is_empty(){return receiver.close_step(control,grant).map(|step|RetainedCloneStep::Progress(step.progress()))}self.receiver=None;return Ok(RetainedCloneStep::Progress(progress(d)))}
  if let Some(wire)=self.wire.as_mut(){if wire.terminal_is_empty(){self.wire=None;return Ok(RetainedCloneStep::Progress(progress(d)))}control.charge(d.capacity_bytes)?;return wire.close_step(grant).map(|step|RetainedCloneStep::Progress(step.progress()))}
  if let Some(source)=self.source.as_mut(){if source.terminal_is_empty(){self.source=None;return Ok(RetainedCloneStep::Progress(progress(d)))}let InputReplacement::Input{payload,..}=row else{return Err(invalid())};if !payload.is_empty()||payload.capacity()!=0{return Err(invalid())}control.charge(d.capacity_bytes)?;return match source.take_authority(grant)?{RetainedCloneSourceTake::Pending(p)=>Ok(RetainedCloneStep::Progress(p)),RetainedCloneSourceTake::Ready(bytes,p)=>{*payload=bytes;Ok(RetainedCloneStep::Progress(p))}}}
  self.binding=None;Ok(RetainedCloneStep::Complete(progress(d)))
 }
 /// 🧵️ The initializer's actual Job boundary forwards its original receiving continuation and reconciles the same receipt.
 pub fn advance(&mut self,row:&mut InputReplacement,codec:&ArtifactOperationWire<M>,factory:&Arc<dyn ArtifactOwnedValueRetirementFactory<M>>,cx:&mut semio_framework_job::StepContext<'_>)->Result<bool,semio_framework_job::OriginalTurnFailure>{if cx.should_yield(){return Ok(false)}let grant=cx.retained_grant();let result=cx.with_native_receiving(|control|self.advance_native(row,codec,factory,control,grant));let p=result.as_ref().copied().unwrap_or_else(|error|error.retained_progress());if let Some(issued)=cx.receiving_issued_grant(){cx.consume_receiving_retained(issued,p)?;}else{cx.consume_retained(p)?;}cx.consume_fuel(p.copied_items as u64);result?;Ok(self.ready())}
 /// 🤝️ Transfers the already authenticated original typed projection under the same Job receiving continuation.
 pub fn take_input(&mut self,output:&mut Option<RetainedOwnedProjection<M>>,cx:&mut semio_framework_job::StepContext<'_>)->Result<bool,semio_framework_job::OriginalTurnFailure>{if cx.should_yield(){return Ok(false)}let grant=cx.retained_grant();let result=cx.with_native_receiving(|control|self.take_input_native(output,control,grant));let p=result.as_ref().copied().unwrap_or_else(|error|error.retained_progress());if let Some(issued)=cx.receiving_issued_grant(){cx.consume_receiving_retained(issued,p)?;}else{cx.consume_retained(p)?;}cx.consume_fuel(p.copied_items as u64);result?;Ok(output.is_some())}
 /// ♻️ The original cancellation scope supplies its actual decoder loan; no receiving callback is invented for closure.
 pub fn close(&mut self,row:&mut InputReplacement,cx:&mut semio_framework_job::StepContext<'_>)->Result<bool,semio_framework_job::OriginalTurnFailure>{if self.terminal_is_empty(){return Ok(true)}cx.with_original_retirement(|cx|{let grant=cx.retained_grant();let result=cx.with_native_retirement(|control|self.close_native(row,control,grant));let p=result.as_ref().map(|step|step.progress()).unwrap_or_else(|error|error.retained_progress());cx.consume_retained(p)?;cx.consume_fuel(p.copied_items as u64);result?;Ok(self.terminal_is_empty())})}
}
impl<M:Sync> OriginalReplacementReceiving<M>{
 /// 🫙️ Every original parser, payload, typed projection and decoder binding has settled.
 pub fn terminal_is_empty(&self)->bool{self.source.is_none()&&self.wire.is_none()&&self.receiver.is_none()&&self.input.is_none()&&self.binding.is_none()}
}
impl<M:Sync> Drop for OriginalReplacementReceiving<M>{fn drop(&mut self){assert!(std::thread::panicking()||self.terminal_is_empty(),"original replacement payload must return to its exact input row before Drop")}}
