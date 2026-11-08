//! 📜️ Original Pack and committed SPR receive together without replacing their supplied source authority.
use semio_framework_value_native_owned::native_decoding::NativeDecodeCloseControl;
use super::{OriginalArtifactPackReceiving,OriginalArtifactSnapshotReceiving};
use semio_os_spr::history::{HistoryLog,RetainedHistorySourcePartial,RetainedHistorySourceDecode,RetainedHistoryOpMetaFault};
pub use protocol::codec::HistoryReadLimits as OriginalArtifactHistoryReadLimits;
use OriginalArtifactHistoryReadLimits as HistoryReadLimits;
use protocol::PackLimits;
use semio_framework_dsl_record::receiving::OwnedSlot;
use semio_framework_value::retained_clone::NativeAllocationIdentity;
use semio_framework_value::{OriginalRetirementPoll, RetainedAllocationEffect, RetainedCloneGrant, RetainedCloneProgress, RetainedCloneStep, RetainedWorkOutcome, RetainedWorkPoll, RetirementDemand, ValueError, ValueRefusalKind};
use semio_framework_value_native_owned::native_decoding::{NativeDecodeControl,OriginalReceivingPrimary,NativeConstructionStepResult};
use std::mem::{size_of,ManuallyDrop};

/// 🧯️ The complete native or structured History failure keeps its distinct actual physical receipt.
#[derive(Debug,semio_framework_value::RetireOwned)]
pub enum OriginalArtifactPairCause { Value(ValueError),History(RetainedHistoryOpMetaFault),Native(OriginalReceivingPrimary) }
#[derive(Debug,semio_framework_value::RetireOwned)]
pub struct OriginalArtifactPairFailure { pub cause:OriginalArtifactPairCause,pub progress:RetainedCloneProgress,pub receipt_refusal:Option<ValueError> }
impl OriginalArtifactPairFailure { pub fn retained_progress(&self)->RetainedCloneProgress { self.progress } }
impl From<ValueError> for OriginalArtifactPairFailure { fn from(error:ValueError)->Self { let progress=error.retained_progress();Self { cause:OriginalArtifactPairCause::Value(error),progress,receipt_refusal:None } } }
impl From<RetainedHistoryOpMetaFault> for OriginalArtifactPairFailure { fn from(error:RetainedHistoryOpMetaFault)->Self { let progress=match &error { RetainedHistoryOpMetaFault::Value(error)=>error.retained_progress(),RetainedHistoryOpMetaFault::Json(_)=>Default::default(),RetainedHistoryOpMetaFault::Native(error)=>error.retained_progress(),RetainedHistoryOpMetaFault::Retirement(error)=>error.retained_progress() };Self { cause:OriginalArtifactPairCause::History(error),progress,receipt_refusal:None } } }

impl From<OriginalReceivingPrimary> for OriginalArtifactPairFailure { fn from(error:OriginalReceivingPrimary)->Self { let progress=error.retained_progress();Self { cause:OriginalArtifactPairCause::Native(error),progress,receipt_refusal:None } } }

/// 🫴️ Actual source buffers outlive typed output, receiving metadata and cancellation cleanup.
pub struct OriginalArtifactPairReceiving<P:OriginalArtifactPackReceiving> {
 source:[(usize,usize);2],control:(NativeAllocationIdentity,Option<u64>),pack_limits:PackLimits,history_limits:HistoryReadLimits,
 snapshot:ManuallyDrop<Option<Box<P::Receiving>>>,history:ManuallyDrop<Option<RetainedHistorySourceDecode>>,history_source_birth:ManuallyDrop<semio_framework_value_native_owned::native_decoding::NativeReceivingEffect<ValueError>>,
 pack_scanned:usize,pack_hasher:semio_framework_hash::Hasher,pack_digest:Option<[u8;32]>,snapshot_output:OwnedSlot<P>,history_output:OwnedSlot<HistoryLog>,history_partial:OwnedSlot<RetainedHistorySourcePartial>,history_fault:OwnedSlot<RetainedHistoryOpMetaFault>,phase:u8,closing:bool,closed:bool,
}
fn invalid(message:&'static str)->ValueError { ValueError::literal(ValueRefusalKind::InvariantViolated,message) }
fn metadata<T>()->RetirementDemand { RetirementDemand { copy_bytes:size_of::<T>(),depth:1,..Default::default() } }
fn nested(mut demand:RetirementDemand)->Result<RetirementDemand,ValueError> { demand.depth=demand.depth.checked_add(1).ok_or_else(||invalid("original artifact pair depth overflow"))?;Ok(demand) }
fn perform(effect:&mut RetainedAllocationEffect<'_>)->Result<(),ValueError> { if effect.performed(1)? { Ok(()) } else { Err(ValueError::literal(ValueRefusalKind::WorkLimit,"original artifact pair work is not funded")) } }
fn closed(poll:OriginalRetirementPoll<RetainedCloneProgress>)->Result<RetainedCloneStep,OriginalArtifactPairFailure> { match poll { RetainedWorkPoll::Pending{..}=>Ok(RetainedCloneStep::Progress(Default::default())),RetainedWorkPoll::Complete(turn)=>turn.result.map(RetainedCloneStep::Progress).map_err(|failure|failure.into_value_error().into()) } }
fn closed_step(poll:OriginalRetirementPoll)->Result<RetainedCloneStep,OriginalArtifactPairFailure> { match poll { RetainedWorkPoll::Pending{..}=>Ok(RetainedCloneStep::Progress(Default::default())),RetainedWorkPoll::Complete(turn)=>turn.result.map(|step|RetainedCloneStep::Progress(step.progress())).map_err(|failure|failure.into_value_error().into()) } }
fn history_fault(cursor:&mut RetainedHistorySourceDecode,cause:RetainedHistoryOpMetaFault)->OriginalArtifactPairFailure { let progress=cursor.take_receipt().map_or(Default::default(),|(_,progress)|progress);OriginalArtifactPairFailure { cause:OriginalArtifactPairCause::History(cause),progress,receipt_refusal:None } }
fn child(grant:RetainedCloneGrant)->RetainedCloneGrant { RetainedCloneGrant { maximum_depth:grant.maximum_depth.saturating_sub(1),..grant } }
fn funds(grant:RetainedCloneGrant,demand:RetirementDemand)->bool { grant.maximum_items>0&&grant.maximum_copy_bytes>=demand.copy_bytes&&grant.maximum_capacity_bytes>=demand.capacity_bytes&&grant.maximum_release_bytes>=demand.release_bytes&&grant.maximum_depth>=demand.depth }
fn receipt(demand:RetirementDemand)->RetainedCloneProgress { RetainedCloneProgress { copied_items:1,copied_bytes:demand.copy_bytes,..Default::default() } }
impl<P:OriginalArtifactPackReceiving> OriginalArtifactPairReceiving<P> {
 /// 🎟️ Binds both immutable original sources and their actual decoder; admission has no heap birth.
 pub fn admit(pack:&[u8],spr:&[u8],pack_limits:PackLimits,history_limits:HistoryReadLimits,native:&NativeDecodeControl<'_>,grant:RetainedCloneGrant)->Result<Option<(Self,RetainedCloneProgress)>,OriginalArtifactPairFailure> {
  let demand=metadata::<Self>();if !funds(grant,demand){return Ok(None)}history_limits.validate()?;
  if pack.len()as u64>pack_limits.max_file_len||spr.len()>history_limits.file_bytes as usize{return Err(invalid("original artifact pair exceeds declared source policies").into())}
  Ok(Some((Self::from_source(pack,spr,pack_limits,history_limits,native),receipt(demand))))
 }
 fn from_source(pack:&[u8],spr:&[u8],pack_limits:PackLimits,history_limits:HistoryReadLimits,native:&NativeDecodeControl<'_>)->Self{Self { source:[(pack.as_ptr()as usize,pack.len()),(spr.as_ptr()as usize,spr.len())],control:(native.ledger_identity(),native.retirement_recipient_identity()),pack_limits,history_limits,snapshot:ManuallyDrop::new(None),history:ManuallyDrop::new(None),history_source_birth:ManuallyDrop::new(Default::default()),pack_scanned:0,pack_hasher:semio_framework_hash::Hasher::new(),pack_digest:None,snapshot_output:Default::default(),history_output:Default::default(),history_partial:Default::default(),history_fault:Default::default(),phase:0,closing:false,closed:false }}
 /// 📏️ Prices a fresh original pair header written directly into its separately owned allocation.
 pub fn boxed_birth_demands()->RetirementDemand{RetirementDemand{copy_bytes:size_of::<Self>()+2*size_of::<Box<Self>>(),capacity_bytes:size_of::<Self>(),depth:1,..Default::default()}}
 /// 🧳️ Receives the same source identities in a fresh boxed header before any snapshot child exists.
 pub fn admit_boxed(pack:&[u8],spr:&[u8],pack_limits:PackLimits,history_limits:HistoryReadLimits,native:&NativeDecodeControl<'_>,grant:RetainedCloneGrant)->Result<Option<(Box<Self>,RetainedCloneProgress)>,OriginalArtifactPairFailure>{
  let d=Self::boxed_birth_demands();if !funds(grant,d){return Ok(None)}history_limits.validate()?;
  if pack.len()as u64>pack_limits.max_file_len||spr.len()>history_limits.file_bytes as usize{return Err(invalid("original artifact pair exceeds declared source policies").into())}
  let storage=Box::<Self>::new_uninit();let owner=Box::write(storage,Self::from_source(pack,spr,pack_limits,history_limits,native));
  Ok(Some((owner,RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,retained_capacity_bytes:d.capacity_bytes,..Default::default()})))
 }
 fn snapshot_birth_demands()->Result<RetirementDemand,ValueError>{let mut d=nested(P::original_pack_birth_demand())?;d.copy_bytes=d.copy_bytes.checked_add(size_of::<P::Receiving>()+2*size_of::<Option<Box<P::Receiving>>>()).ok_or_else(||invalid("original snapshot receiver copy quote overflow"))?;d.capacity_bytes=d.capacity_bytes.checked_add(size_of::<P::Receiving>()).ok_or_else(||invalid("original snapshot receiver allocation quote overflow"))?;Ok(d)}
 fn snapshot_release_demands()->RetirementDemand{RetirementDemand{copy_bytes:size_of::<Option<Box<P::Receiving>>>(),release_bytes:size_of::<P::Receiving>(),depth:1,..Default::default()}}

 fn validate(&self,pack:&[u8],spr:&[u8],native:&dyn NativeDecodeCloseControl)->Result<(),ValueError> { if self.source!=[(pack.as_ptr()as usize,pack.len()),(spr.as_ptr()as usize,spr.len())]||self.control!=(native.ledger_identity(),native.retirement_recipient_identity()){return Err(invalid("original artifact pair source or receiving owner changed"))}Ok(()) }
 /// 📐️ Quotes the next real child action; no decoding or speculative owner construction occurs here.
 pub fn demands(&self,pack:&[u8],spr:&[u8],native:&dyn NativeDecodeCloseControl,body:usize)->Result<RetirementDemand,OriginalArtifactPairFailure> {
  self.validate(pack,spr,native)?;if self.closed{return Ok(Default::default())}if self.closing{return self.close_demands(pack,spr,native,body)}
  if self.pack_digest.is_none(){return Ok(RetirementDemand { copy_bytes:if self.pack_scanned<pack.len(){body.max(1).min(4096).min(pack.len()-self.pack_scanned)}else{32},depth:1,..Default::default() })}
  Ok(match self.phase {
   0=>Self::snapshot_birth_demands()?,1=>if self.snapshot.as_ref().unwrap().is_ready(){metadata::<u8>()}else{nested(self.snapshot.as_ref().unwrap().demands(pack,body)?)?},
   2=>nested(self.snapshot.as_ref().unwrap().take_demand()?)?,3=>if self.snapshot.as_ref().unwrap().terminal_is_empty(){Self::snapshot_release_demands()}else{nested(self.snapshot.as_ref().unwrap().close_demands(body)?)?},
   4=>RetirementDemand { depth:3,..metadata::<(RetainedHistorySourceDecode,Option<RetainedHistorySourceDecode>)>() },5=>if self.history.as_ref().unwrap().fault().is_some(){nested(metadata::<RetainedHistoryOpMetaFault>())?}else{nested(self.history.as_ref().unwrap().demands(spr,native,body)?)?},
   6=>nested(metadata::<HistoryLog>())?,7=>metadata::<(Option<RetainedHistorySourceDecode>,u8)>(),_=>Default::default(),
  })
 }
 /// 🪜️ Receives native genesis and the exact committed History inside the caller's one original effect; every child pays its own work once.
 pub fn step(&mut self,pack:&[u8],spr:&[u8],native:&mut NativeDecodeControl<'_>,body:usize,effect:&mut RetainedAllocationEffect<'_>)->RetainedWorkOutcome<RetainedCloneProgress,OriginalArtifactPairFailure>{
  let result=self.step_action(pack,spr,native,body,effect);RetainedWorkOutcome{result,work:effect.performed_work()}
 }
 fn step_action(&mut self,pack:&[u8],spr:&[u8],native:&mut NativeDecodeControl<'_>,body:usize,effect:&mut RetainedAllocationEffect<'_>)->Result<RetainedCloneProgress,OriginalArtifactPairFailure> {
  let demand=self.demands(pack,spr,native,body)?;let grant=effect.grant();if self.closing||self.closed||self.phase>=8||!funds(grant,demand){return Ok(Default::default())}
  let child=child(grant);
  if self.pack_digest.is_some()&&self.phase==1&&!self.snapshot.as_ref().unwrap().is_ready(){return self.snapshot.as_mut().unwrap().step(pack,native,body,&mut effect.nested(child)).result.map_err(Into::into)}
  native.checkpoint()?;
  if self.pack_digest.is_none(){perform(effect)?;if self.pack_scanned<pack.len(){let end=self.pack_scanned+demand.copy_bytes;self.pack_hasher.update(&pack[self.pack_scanned..end]);self.pack_scanned=end;}else{self.pack_digest=Some(*self.pack_hasher.finalize().as_bytes());}return Ok(receipt(demand))}
  match self.phase {
   0=>{perform(effect)?;let overhead=size_of::<P::Receiving>()+2*size_of::<Option<Box<P::Receiving>>>();let child=RetainedCloneGrant{maximum_copy_bytes:child.maximum_copy_bytes-overhead,maximum_capacity_bytes:child.maximum_capacity_bytes-size_of::<P::Receiving>(),..child};let Some((cursor,mut progress))=P::admit_original_pack(pack,self.pack_limits.clone(),native,child)?else{return Ok(Default::default())};if effect.actual().is_some(){return Err(invalid("original snapshot receiver allocation marker is already occupied").into())}*effect.actual()=Some(0);*self.snapshot=Some(Box::new(cursor));*effect.actual()=Some(size_of::<P::Receiving>());self.phase=1;progress.copied_bytes+=overhead;progress.retained_capacity_bytes+=size_of::<P::Receiving>();return Ok(progress)},
   1=>{perform(effect)?;let cursor=self.snapshot.as_mut().unwrap();if cursor.is_ready(){self.phase=2}else{unreachable!("snapshot child delegates through its complete posted outcome")}},
   2=>{perform(effect)?;let progress=self.snapshot.as_mut().unwrap().take_into(&mut self.snapshot_output.value,child)?;if self.snapshot_output.value.is_some(){self.phase=3}return Ok(progress)},
   3=>{let cursor=self.snapshot.as_mut().unwrap();if cursor.terminal_is_empty(){perform(effect)?;drop(self.snapshot.take());self.phase=4;return Ok(RetainedCloneProgress{released_bytes:demand.release_bytes,..receipt(demand)})}else{return closed_step(cursor.close_step(&mut effect.nested(child))).map(|step|step.progress())}},
   4=>{let Some(mut cursor)=RetainedHistorySourceDecode::admit(spr,(0,spr.len()),self.history_limits,true,native,&mut effect.nested(child))?else{return Ok(Default::default())};let mut progress=cursor.take_receipt().map_or(Default::default(),|(_,progress)|progress);progress.copied_bytes=progress.copied_bytes.checked_add(demand.copy_bytes).ok_or_else(||invalid("original History header copy overflow"))?;*self.history=Some(cursor);self.phase=5;return Ok(progress)},
   5=>{let cursor=self.history.as_mut().unwrap();if cursor.fault().is_some(){let mut original=None;let result=cursor.take_fault_into(&mut original,&mut effect.nested(child));if let Err(cause)=result{return Err(history_fault(cursor,cause))}let mut progress=cursor.take_receipt().map_or(Default::default(),|(_,progress)|progress);if let Some(cause)=original{progress.copied_bytes=demand.copy_bytes;self.begin_close();return Err(OriginalArtifactPairFailure { cause:OriginalArtifactPairCause::History(cause),progress,receipt_refusal:None })}return Ok(progress)}if cursor.is_ready(){perform(effect)?;self.phase=6}else{let result=cursor.step(spr,native,&mut effect.nested(child));if let Err(cause)=result{return Err(history_fault(cursor,cause))}return Ok(cursor.take_receipt().map_or(Default::default(),|(_,progress)|progress))}},
   6=>{let cursor=self.history.as_mut().unwrap();let result=cursor.take_ready_into(&mut self.history_output.value,&mut effect.nested(child));let progress=cursor.take_receipt().map_or(Default::default(),|(_,progress)|progress);let moved=result.map_err(|cause|OriginalArtifactPairFailure { cause:OriginalArtifactPairCause::History(cause),progress,receipt_refusal:None })?;if moved{self.phase=7}return Ok(RetainedCloneProgress { copied_bytes:if moved{demand.copy_bytes}else{0},..progress })},
   7=>{perform(effect)?;if !self.history.as_ref().unwrap().terminal_is_empty(){return Err(invalid("original artifact pair History still owns receiving state").into())}drop(self.history.take());self.phase=8},
   _=>unreachable!(),
  }Ok(receipt(demand))
 }
 /// 👁️ Both original typed outputs are ready only after every dependent receiving child has closed.
 pub fn is_ready(&self)->bool { self.phase==8&&!self.closing&&!self.closed&&self.snapshot_output.value.is_some()&&self.history_output.value.is_some()&&self.snapshot.is_none()&&self.history.is_none() }
 /// 🔏️ Borrows the digest received from these same original Pack bytes in bounded work chunks.
 pub fn pack_digest(&self)->Option<[u8;32]> { self.pack_digest }
 pub fn take_demand(&self)->RetirementDemand { metadata::<(P,HistoryLog)>() }
 /// 📦️ Transfers the same genesis and History owners; it does not settle, fold or discard supplied edits.
 pub fn take_into(&mut self,output:&mut Option<(P,HistoryLog)>,grant:RetainedCloneGrant)->Result<RetainedCloneProgress,ValueError> { let demand=self.take_demand();if output.is_some()||!self.is_ready()||!funds(grant,demand){return Ok(Default::default())}*output=Some((self.snapshot_output.value.take().unwrap(),self.history_output.value.take().unwrap()));self.phase=9;Ok(receipt(demand)) }
 /// 👁️ Transfers the exact received History into its actual original shared source while genesis stays resident.
 pub fn admit_history_source(&mut self,pack:&[u8],spr:&[u8],output:&mut Option<semio_os_spr::history::OriginalHistorySource>,native:&mut NativeDecodeControl<'_>)->Result<NativeConstructionStepResult<bool,semio_framework_value_native_owned::native_decoding::NativeReceivingEffectFailure<ValueError>>,OriginalReceivingPrimary>{
  self.validate(pack,spr,native)?;
  let pending=self.history_source_birth.phase()!=semio_framework_value_native_owned::native_decoding::NativeReceivingEffectPhase::Empty;
  if !pending&&!self.is_ready(){return Ok(NativeConstructionStepResult{result:Ok(false),progress:Default::default(),work:0})}
  let result=semio_os_spr::history::OriginalHistorySource::admit(&mut self.history_output.value,output,&mut self.history_source_birth,native);
  if self.history_output.value.is_none(){self.phase=10;}
  result
 }
 /// 🪪️ Reborrows pending source-birth callbacks only under the same original sources and native owner.
 pub fn history_source_birth_original(&mut self,pack:&[u8],spr:&[u8],native:&dyn NativeDecodeCloseControl)->Result<&mut semio_framework_value_native_owned::native_decoding::NativeReceivingEffect<ValueError>,ValueError>{
  self.validate(pack,spr,native)?;
  if self.history_source_birth.phase()==semio_framework_value_native_owned::native_decoding::NativeReceivingEffectPhase::Empty{return Err(invalid("original History source has no pending birth callbacks"))}
  Ok(&mut self.history_source_birth)
 }
 /// 📸️ Separately hands off genesis only after its same original History source has been installed.
 pub fn take_genesis_into(&mut self,pack:&[u8],spr:&[u8],output:&mut Option<P>,native:&dyn NativeDecodeCloseControl,grant:RetainedCloneGrant)->Result<RetainedCloneProgress,ValueError>{
  self.validate(pack,spr,native)?;
  let demand=metadata::<P>();
  if self.phase!=10||self.history_source_birth.phase()!=semio_framework_value_native_owned::native_decoding::NativeReceivingEffectPhase::Empty||self.closing||self.closed||output.is_some()||!funds(grant,demand){return Ok(Default::default())}
  *output=self.snapshot_output.value.take();
  self.phase=9;
  Ok(receipt(demand))
 }
 pub fn begin_close(&mut self) { self.closing=true;if let Some(cursor)=self.history.as_mut(){cursor.cancel()} }
 /// ♻️ Original byte sources remain borrowed until all typed outputs and parser children are terminal.
 pub fn close_demands(&self,pack:&[u8],spr:&[u8],native:&dyn NativeDecodeCloseControl,body:usize)->Result<RetirementDemand,OriginalArtifactPairFailure> {
  self.validate(pack,spr,native)?;if self.closed{return Ok(Default::default())}
  if self.history_source_birth.phase()!=semio_framework_value_native_owned::native_decoding::NativeReceivingEffectPhase::Empty{return Err(invalid("original History source birth must resolve before pair retirement").into())}
  if let Some(cursor)=self.snapshot.as_ref(){return Ok(if cursor.terminal_is_empty(){Self::snapshot_release_demands()}else{nested(cursor.close_demands(body)?)?})}if let Some(cursor)=self.history.as_ref(){if cursor.terminal_is_empty(){return Ok(metadata::<Option<RetainedHistorySourceDecode>>())}if cursor.fault().is_some(){return Ok(nested(metadata::<RetainedHistoryOpMetaFault>())?)}if cursor.partial_is_receivable(){return Ok(nested(metadata::<RetainedHistorySourcePartial>())?)}return Ok(nested(cursor.demands(spr,native,body)?)?)}if !self.snapshot_output.terminal_is_empty(){return Ok(nested(self.snapshot_output.close_demands(body)?)?)}if !self.history_output.terminal_is_empty(){return Ok(nested(self.history_output.close_demands(body)?)?)}if !self.history_partial.terminal_is_empty(){return Ok(nested(self.history_partial.close_demands(body)?)?)}if !self.history_fault.terminal_is_empty(){return Ok(nested(self.history_fault.close_demands(body)?)?)}Ok(RetirementDemand { depth:1,..Default::default() })
 }
 pub fn close_step(&mut self,pack:&[u8],spr:&[u8],native:&mut dyn NativeDecodeCloseControl,effect:&mut RetainedAllocationEffect<'_>)->RetainedWorkPoll<RetainedCloneStep,OriginalArtifactPairFailure>{let result=self.close_turn(pack,spr,native,effect);RetainedWorkPoll::Complete(RetainedWorkOutcome{result,work:effect.performed_work()})}
 fn close_turn(&mut self,pack:&[u8],spr:&[u8],native:&mut dyn NativeDecodeCloseControl,effect:&mut RetainedAllocationEffect<'_>)->Result<RetainedCloneStep,OriginalArtifactPairFailure> {
  self.validate(pack,spr,native)?;if self.closed{return Ok(RetainedCloneStep::Complete(Default::default()))}self.begin_close();let grant=effect.grant();let demand=self.close_demands(pack,spr,native,grant.maximum_copy_bytes)?;if !funds(grant,demand){return Ok(RetainedCloneStep::Progress(Default::default()))}let child=child(grant);
  if let Some(cursor)=self.snapshot.as_mut(){if cursor.terminal_is_empty(){perform(effect)?;drop(self.snapshot.take());return Ok(RetainedCloneStep::Progress(RetainedCloneProgress{released_bytes:demand.release_bytes,..receipt(demand)}))}return closed_step(cursor.close_step(&mut effect.nested(child)))}
  if let Some(cursor)=self.history.as_mut(){if cursor.terminal_is_empty(){perform(effect)?;drop(self.history.take());return Ok(RetainedCloneStep::Progress(receipt(demand)))}let result=if cursor.fault().is_some(){cursor.take_fault_into(&mut self.history_fault.value,&mut effect.nested(child)).map(|_|())}else if cursor.partial_is_receivable(){cursor.take_partial_into(&mut self.history_partial.value,&mut effect.nested(child)).map(|_|())}else{cursor.close_step(spr,native,&mut effect.nested(child)).map(|_|())};if let Err(cause)=result{return Err(history_fault(cursor,cause))}return Ok(RetainedCloneStep::Progress(cursor.take_receipt().map_or(Default::default(),|(_,progress)|progress)))}
  native.charge(demand.capacity_bytes)?;
  if !self.snapshot_output.terminal_is_empty(){return closed(self.snapshot_output.close_step(&mut effect.nested(child)))}if !self.history_output.terminal_is_empty(){return closed(self.history_output.close_step(&mut effect.nested(child)))}if !self.history_partial.terminal_is_empty(){return closed(self.history_partial.close_step(&mut effect.nested(child)))}if !self.history_fault.terminal_is_empty(){return closed(self.history_fault.close_step(&mut effect.nested(child)))}perform(effect)?;self.closed=true;Ok(RetainedCloneStep::Complete(receipt(demand)))
 }
 pub fn terminal_is_empty(&self)->bool { self.closed&&self.history_source_birth.phase()==semio_framework_value_native_owned::native_decoding::NativeReceivingEffectPhase::Empty&&self.snapshot.is_none()&&self.history.is_none()&&self.snapshot_output.terminal_is_empty()&&self.history_output.terminal_is_empty()&&self.history_partial.terminal_is_empty()&&self.history_fault.terminal_is_empty() }
}
impl<P:OriginalArtifactPackReceiving> Drop for OriginalArtifactPairReceiving<P> { fn drop(&mut self) { assert!(std::thread::panicking()||self.terminal_is_empty(),"original artifact pair abandoned source-dependent receivers or typed outputs");if self.terminal_is_empty(){unsafe{ManuallyDrop::drop(&mut self.snapshot);ManuallyDrop::drop(&mut self.history);ManuallyDrop::drop(&mut self.history_source_birth)}} } }
#[path="🫴️failure/🦀️.rs"]
mod original_failure_projection;
