//! 📜️ Original Pack and committed SPR receive together without replacing their supplied source authority.
use semio_framework_value_native_owned::native_decoding::NativeDecodeCloseControl;
use super::{OriginalArtifactPackReceiving,OriginalArtifactSnapshotReceiving};
use semio_framework_os_spr::history::{HistoryLog,RetainedHistorySourcePartial,RetainedHistorySourceDecode,RetainedHistoryOpMetaFault};
pub use protocol::codec::HistoryReadLimits as OriginalArtifactHistoryReadLimits;
use OriginalArtifactHistoryReadLimits as HistoryReadLimits;
use protocol::PackLimits;
use semio_framework_dsl_record::receiving::OwnedSlot;
use semio_framework_value::{RetainedCloneGrant, RetainedCloneProgress, RetainedCloneStep, RetirementDemand, ValueError, ValueRefusalKind};
use semio_framework_value_native_owned::native_decoding::NativeDecodeControl;
use std::mem::{size_of,ManuallyDrop};

/// 🧯️ The complete native or structured History failure keeps its distinct actual physical receipt.
#[derive(Debug,semio_framework_value::RetireOwned)]
pub enum OriginalArtifactPairCause { Value(ValueError),History(RetainedHistoryOpMetaFault) }
#[derive(Debug,semio_framework_value::RetireOwned)]
pub struct OriginalArtifactPairFailure { pub cause:OriginalArtifactPairCause,pub progress:RetainedCloneProgress }
impl OriginalArtifactPairFailure { pub fn retained_progress(&self)->RetainedCloneProgress { self.progress } }
impl From<ValueError> for OriginalArtifactPairFailure { fn from(error:ValueError)->Self { let progress=error.retained_progress();Self { cause:OriginalArtifactPairCause::Value(error),progress } } }
impl From<RetainedHistoryOpMetaFault> for OriginalArtifactPairFailure { fn from(error:RetainedHistoryOpMetaFault)->Self { let progress=match &error { RetainedHistoryOpMetaFault::Value(error)=>error.retained_progress(),RetainedHistoryOpMetaFault::Json(_)=>Default::default() };Self { cause:OriginalArtifactPairCause::History(error),progress } } }

/// 🫴️ Actual source buffers outlive typed output, receiving metadata and cancellation cleanup.
pub struct OriginalArtifactPairReceiving<P:OriginalArtifactPackReceiving> {
 source:[(usize,usize);2],control:(u64,Option<u64>),pack_limits:PackLimits,history_limits:HistoryReadLimits,
 snapshot:ManuallyDrop<Option<P::Receiving>>,history:ManuallyDrop<Option<RetainedHistorySourceDecode>>,
 pack_scanned:usize,pack_hasher:semio_framework_hash::Hasher,pack_digest:Option<[u8;32]>,snapshot_output:OwnedSlot<P>,history_output:OwnedSlot<HistoryLog>,history_partial:OwnedSlot<RetainedHistorySourcePartial>,history_fault:OwnedSlot<RetainedHistoryOpMetaFault>,phase:u8,closing:bool,closed:bool,
}
fn invalid(message:&'static str)->ValueError { ValueError::literal(ValueRefusalKind::InvariantViolated,message) }
fn metadata<T>()->RetirementDemand { RetirementDemand { copy_bytes:size_of::<T>(),depth:1,..Default::default() } }
fn nested(mut demand:RetirementDemand)->Result<RetirementDemand,ValueError> { demand.depth=demand.depth.checked_add(1).ok_or_else(||invalid("original artifact pair depth overflow"))?;Ok(demand) }
fn child(grant:RetainedCloneGrant)->RetainedCloneGrant { RetainedCloneGrant { maximum_depth:grant.maximum_depth.saturating_sub(1),..grant } }
fn funds(grant:RetainedCloneGrant,demand:RetirementDemand)->bool { grant.maximum_items>0&&grant.maximum_copy_bytes>=demand.copy_bytes&&grant.maximum_capacity_bytes>=demand.capacity_bytes&&grant.maximum_release_bytes>=demand.release_bytes&&grant.maximum_depth>=demand.depth }
fn receipt(demand:RetirementDemand)->RetainedCloneProgress { RetainedCloneProgress { copied_items:1,copied_bytes:demand.copy_bytes,..Default::default() } }
impl<P:OriginalArtifactPackReceiving> OriginalArtifactPairReceiving<P> {
 /// 🎟️ Binds both immutable original sources and their actual decoder; admission has no heap birth.
 pub fn admit(pack:&[u8],spr:&[u8],pack_limits:PackLimits,history_limits:HistoryReadLimits,native:&NativeDecodeControl<'_>,grant:RetainedCloneGrant)->Result<Option<(Self,RetainedCloneProgress)>,OriginalArtifactPairFailure> {
  let demand=metadata::<Self>();if !funds(grant,demand){return Ok(None)}history_limits.validate()?;
  if pack.len()as u64>pack_limits.max_file_len||spr.len()>history_limits.file_bytes as usize{return Err(invalid("original artifact pair exceeds declared source policies").into())}
  Ok(Some((Self { source:[(pack.as_ptr()as usize,pack.len()),(spr.as_ptr()as usize,spr.len())],control:(native.ledger_identity(),native.retirement_recipient_identity()),pack_limits,history_limits,snapshot:ManuallyDrop::new(None),history:ManuallyDrop::new(None),pack_scanned:0,pack_hasher:semio_framework_hash::Hasher::new(),pack_digest:None,snapshot_output:Default::default(),history_output:Default::default(),history_partial:Default::default(),history_fault:Default::default(),phase:0,closing:false,closed:false },receipt(demand))))
 }
 fn validate(&self,pack:&[u8],spr:&[u8],native:&dyn NativeDecodeCloseControl)->Result<(),ValueError> { if self.source!=[(pack.as_ptr()as usize,pack.len()),(spr.as_ptr()as usize,spr.len())]||self.control!=(native.ledger_identity(),native.retirement_recipient_identity()){return Err(invalid("original artifact pair source or receiving owner changed"))}Ok(()) }
 /// 📐️ Quotes the next real child action; no decoding or speculative owner construction occurs here.
 pub fn demands(&self,pack:&[u8],spr:&[u8],native:&dyn NativeDecodeCloseControl,body:usize)->Result<RetirementDemand,OriginalArtifactPairFailure> {
  self.validate(pack,spr,native)?;if self.closed{return Ok(Default::default())}if self.closing{return self.close_demands(pack,spr,native,body)}
  if self.pack_digest.is_none(){return Ok(RetirementDemand { copy_bytes:if self.pack_scanned<pack.len(){body.max(1).min(4096).min(pack.len()-self.pack_scanned)}else{32},depth:1,..Default::default() })}
  Ok(match self.phase {
   0=>nested(P::original_pack_birth_demand())?,1=>nested(self.snapshot.as_ref().unwrap().demands(pack,body)?)?,
   2=>nested(self.snapshot.as_ref().unwrap().take_demand()?)?,3=>nested(self.snapshot.as_ref().unwrap().close_demands(body)?)?,
   4=>RetirementDemand { depth:3,..Default::default() },5=>if self.history.as_ref().unwrap().fault().is_some(){nested(metadata::<RetainedHistoryOpMetaFault>())?}else{nested(self.history.as_ref().unwrap().demands(spr,native,body)?)?},
   6=>nested(metadata::<HistoryLog>())?,7=>RetirementDemand { depth:1,..Default::default() },_=>Default::default(),
  })
 }
 /// 🪜️ Receives native genesis and the exact committed History using their original typed state machines.
 pub fn step(&mut self,pack:&[u8],spr:&[u8],native:&mut NativeDecodeControl<'_>,body:usize,grant:RetainedCloneGrant)->Result<RetainedCloneProgress,OriginalArtifactPairFailure> {
  self.validate(pack,spr,native)?;if self.closing||self.closed||self.phase>=8{return Ok(Default::default())}let demand=self.demands(pack,spr,native,body)?;if !funds(grant,demand){return Ok(Default::default())}native.checkpoint()?;
  if self.pack_digest.is_none(){if self.pack_scanned<pack.len(){let end=self.pack_scanned+demand.copy_bytes;self.pack_hasher.update(&pack[self.pack_scanned..end]);self.pack_scanned=end;}else{self.pack_digest=Some(*self.pack_hasher.finalize().as_bytes());}return Ok(receipt(demand))}
  let child=child(grant);
  match self.phase {
   0=>{let Some((cursor,progress))=P::admit_original_pack(pack,self.pack_limits.clone(),native,child)?else{return Ok(Default::default())};*self.snapshot=Some(cursor);self.phase=1;return Ok(progress)},
   1=>{let cursor=self.snapshot.as_mut().unwrap();if cursor.is_ready(){self.phase=2}else{return cursor.step(pack,native,body,child).map_err(Into::into)}},
   2=>{let progress=self.snapshot.as_mut().unwrap().take_into(&mut self.snapshot_output.value,child)?;if self.snapshot_output.value.is_some(){self.phase=3}return Ok(progress)},
   3=>{let cursor=self.snapshot.as_mut().unwrap();if cursor.terminal_is_empty(){drop(self.snapshot.take());self.phase=4}else{return cursor.close_step(native,child).map(|step|step.progress()).map_err(Into::into)}},
   4=>{let Some(mut cursor)=RetainedHistorySourceDecode::admit(spr,(0,spr.len()),self.history_limits,true,native,child)?else{return Ok(Default::default())};let progress=cursor.take_receipt().map_or(Default::default(),|(_,progress)|progress);*self.history=Some(cursor);self.phase=5;return Ok(progress)},
   5=>{let cursor=self.history.as_mut().unwrap();if cursor.fault().is_some(){let mut original=None;let result=cursor.take_fault_into(&mut original,child);let mut progress=cursor.take_receipt().map_or(Default::default(),|(_,progress)|progress);result.map_err(|cause|OriginalArtifactPairFailure { cause:OriginalArtifactPairCause::History(cause),progress })?;if let Some(cause)=original{progress.copied_bytes=demand.copy_bytes;self.begin_close();return Err(OriginalArtifactPairFailure { cause:OriginalArtifactPairCause::History(cause),progress })}return Ok(progress)}if cursor.is_ready(){self.phase=6}else{let result=cursor.step(spr,native,child);let progress=cursor.take_receipt().map_or(Default::default(),|(_,progress)|progress);result.map_err(|cause|OriginalArtifactPairFailure { cause:OriginalArtifactPairCause::History(cause),progress })?;return Ok(progress)}},
   6=>{let cursor=self.history.as_mut().unwrap();let result=cursor.take_ready_into(&mut self.history_output.value,child);let progress=cursor.take_receipt().map_or(Default::default(),|(_,progress)|progress);let moved=result.map_err(|cause|OriginalArtifactPairFailure { cause:OriginalArtifactPairCause::History(cause),progress })?;if moved{self.phase=7}return Ok(RetainedCloneProgress { copied_bytes:if moved{demand.copy_bytes}else{0},..progress })},
   7=>{if !self.history.as_ref().unwrap().terminal_is_empty(){return Err(invalid("original artifact pair History still owns receiving state").into())}drop(self.history.take());self.phase=8},
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
 pub fn admit_history_source(&mut self,pack:&[u8],spr:&[u8],output:&mut Option<semio_framework_os_spr::history::OriginalHistorySource>,native:&mut NativeDecodeControl<'_>)->semio_framework_value_native_owned::native_decoding::NativeConstructionStepOutcome<bool,semio_framework_value_native_owned::native_decoding::OriginalReceivingPrimary>{
  if let Err(error)=self.validate(pack,spr,native){return semio_framework_value_native_owned::native_decoding::NativeConstructionStepOutcome{result:Some(Err(error.into())),receipt_refusal:None,progress:Default::default()}}
  if !self.is_ready(){return semio_framework_value_native_owned::native_decoding::NativeConstructionStepOutcome{result:None,receipt_refusal:None,progress:Default::default()}}
  let outcome=semio_framework_os_spr::history::OriginalHistorySource::admit(&mut self.history_output.value,output,native);
  if self.history_output.value.is_none(){self.phase=10;}
  outcome
 }
 /// 📸️ Separately hands off genesis only after its same original History source has been installed.
 pub fn take_genesis_into(&mut self,pack:&[u8],spr:&[u8],output:&mut Option<P>,native:&dyn NativeDecodeCloseControl,grant:RetainedCloneGrant)->Result<RetainedCloneProgress,ValueError>{
  self.validate(pack,spr,native)?;
  let demand=metadata::<P>();
  if self.phase!=10||self.closing||self.closed||output.is_some()||!funds(grant,demand){return Ok(Default::default())}
  *output=self.snapshot_output.value.take();
  self.phase=9;
  Ok(receipt(demand))
 }
 pub fn begin_close(&mut self) { self.closing=true;if let Some(cursor)=self.history.as_mut(){cursor.cancel()} }
 /// ♻️ Original byte sources remain borrowed until all typed outputs and parser children are terminal.
 pub fn close_demands(&self,pack:&[u8],spr:&[u8],native:&dyn NativeDecodeCloseControl,body:usize)->Result<RetirementDemand,OriginalArtifactPairFailure> {
  self.validate(pack,spr,native)?;if self.closed{return Ok(Default::default())}
  if let Some(cursor)=self.snapshot.as_ref(){return Ok(nested(cursor.close_demands(body)?)?)}if let Some(cursor)=self.history.as_ref(){if cursor.fault().is_some(){return Ok(nested(metadata::<RetainedHistoryOpMetaFault>())?)}if cursor.partial_is_receivable(){return Ok(nested(metadata::<RetainedHistorySourcePartial>())?)}return Ok(nested(cursor.demands(spr,native,body)?)?)}if !self.snapshot_output.terminal_is_empty(){return Ok(nested(self.snapshot_output.close_demands(body)?)?)}if !self.history_output.terminal_is_empty(){return Ok(nested(self.history_output.close_demands(body)?)?)}if !self.history_partial.terminal_is_empty(){return Ok(nested(self.history_partial.close_demands(body)?)?)}if !self.history_fault.terminal_is_empty(){return Ok(nested(self.history_fault.close_demands(body)?)?)}Ok(RetirementDemand { depth:1,..Default::default() })
 }
 pub fn close_step(&mut self,pack:&[u8],spr:&[u8],native:&mut dyn NativeDecodeCloseControl,grant:RetainedCloneGrant)->Result<RetainedCloneStep,OriginalArtifactPairFailure> {
  self.validate(pack,spr,native)?;if self.closed{return Ok(RetainedCloneStep::Complete(Default::default()))}self.begin_close();let demand=self.close_demands(pack,spr,native,grant.maximum_copy_bytes)?;if !funds(grant,demand){return Ok(RetainedCloneStep::Progress(Default::default()))}let child=child(grant);
  if let Some(cursor)=self.snapshot.as_mut(){if cursor.terminal_is_empty(){drop(self.snapshot.take());return Ok(RetainedCloneStep::Progress(receipt(demand)))}return cursor.close_step(native,child).map_err(Into::into)}
  if let Some(cursor)=self.history.as_mut(){if cursor.terminal_is_empty(){drop(self.history.take());return Ok(RetainedCloneStep::Progress(receipt(demand)))}let result=if cursor.fault().is_some(){cursor.take_fault_into(&mut self.history_fault.value,child).map(|_|())}else if cursor.partial_is_receivable(){cursor.take_partial_into(&mut self.history_partial.value,child).map(|_|())}else{cursor.close_step(spr,native,child).map(|_|())};let progress=cursor.take_receipt().map_or(Default::default(),|(_,progress)|progress);result.map_err(|cause|OriginalArtifactPairFailure { cause:OriginalArtifactPairCause::History(cause),progress })?;return Ok(RetainedCloneStep::Progress(progress))}
  native.charge(demand.capacity_bytes)?;
  if !self.snapshot_output.terminal_is_empty(){return self.snapshot_output.close_step(child).map(RetainedCloneStep::Progress).map_err(Into::into)}if !self.history_output.terminal_is_empty(){return self.history_output.close_step(child).map(RetainedCloneStep::Progress).map_err(Into::into)}if !self.history_partial.terminal_is_empty(){return self.history_partial.close_step(child).map(RetainedCloneStep::Progress).map_err(Into::into)}if !self.history_fault.terminal_is_empty(){return self.history_fault.close_step(child).map(RetainedCloneStep::Progress).map_err(Into::into)}self.closed=true;Ok(RetainedCloneStep::Complete(receipt(demand)))
 }
 pub fn terminal_is_empty(&self)->bool { self.closed&&self.snapshot.is_none()&&self.history.is_none()&&self.snapshot_output.terminal_is_empty()&&self.history_output.terminal_is_empty()&&self.history_partial.terminal_is_empty()&&self.history_fault.terminal_is_empty() }
}
impl<P:OriginalArtifactPackReceiving> Drop for OriginalArtifactPairReceiving<P> { fn drop(&mut self) { assert!(std::thread::panicking()||self.terminal_is_empty(),"original artifact pair abandoned source-dependent receivers or typed outputs");if self.terminal_is_empty(){unsafe{ManuallyDrop::drop(&mut self.snapshot);ManuallyDrop::drop(&mut self.history)}} } }
