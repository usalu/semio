//! 🫴️ A selected member receives its typed snapshot directly from the original sealed Pack pages.
use super::{MemberSnapshotOpenOperation,MemberSnapshotOpenStep,MemberSnapshotOpenProgress};
use semio_framework_os_store_member_open::{MemberOpenRequest,MemberOpenAdmissionError,MemberOpenDiagnostic,MemberOpenProgress,MemberOpenPhase,MemberOpenInputStep};
use semio_framework_value::{ValueError,ValueRefusalKind,RetirementDemand,ErasedSnapshotRetirement,retirement::{RetireOwned,controlled::ControlledRetirement},retained_clone::{RetainedCloneGrant,RetainedCloneProgress,RetainedCloneSource,RetainedCloneStep,retained_clone_grant_admits}};
use semio_framework_value_native_owned::native_decoding::{owned::{NativeReceive,NativeReceivingOriginal}};
use semio_framework_dsl_record::{RecordValue,BorrowedDslRecord};
use semio_framework_job::{StepContext,OriginalTurnFailure};
use protocol::codec::RetainedWireByteSource;
use pack::{record::RetainedRecordReceiving,PackLimits};
use std::mem::{ManuallyDrop,size_of};
#[derive(Clone,Copy,PartialEq,Eq)]
enum Phase{Frame,Envelope,Parse,TakeRecord,IssueOriginal,ReceivingShell,CaptureOriginal,BirthReceiver,Receive,TakeSnapshot,CloseReceiving,CloseParser,Ready}
/// 📸️ Original wire, parsed source, native recipient and output keep separate custody until exact funded closure.
pub struct OriginalRecordMemberSnapshotOpen<S:NativeReceive<RecordValue>>{
 request:ManuallyDrop<Option<MemberOpenRequest>>,parser:ManuallyDrop<Option<RetainedRecordReceiving>>,record:ManuallyDrop<Option<RecordValue>>,source:ManuallyDrop<Option<RetainedCloneSource<RecordValue>>>,receiving:ManuallyDrop<Option<NativeReceivingOriginal<RecordValue,S>>>,snapshot:ManuallyDrop<Option<S>>,snapshot_close:Option<ControlledRetirement<S>>,record_close:Option<ControlledRetirement<RecordValue>>,phase:Phase,envelope_offset:usize,payload_start:Option<usize>,last:RetainedCloneProgress,diagnostic:Option<MemberOpenDiagnostic>,closing:bool,terminal:bool,turns:u64,
}
fn invalid()->ValueError{ValueError::literal(ValueRefusalKind::InvariantViolated,"original member snapshot opening custody changed")}
fn fixed(copy_bytes:usize)->RetirementDemand{RetirementDemand{copy_bytes,depth:1,..Default::default()}}
fn progress(copy_bytes:usize)->RetainedCloneProgress{RetainedCloneProgress{copied_items:1,copied_bytes:copy_bytes,..Default::default()}}
impl<S:NativeReceive<RecordValue>> OriginalRecordMemberSnapshotOpen<S>{
 /// 🧾️ The same caller debits this receipt even when the semantic decoder refuses its input.
 pub fn last_receipt(&self)->RetainedCloneProgress{self.last}
 fn source_demand()->RetirementDemand{let b=RetainedCloneSource::<RecordValue>::owned_constructor_demand::<()>();RetirementDemand{copy_bytes:RetainedCloneSource::<RecordValue>::constructor_copy_bytes()+2*size_of::<Option<RecordValue>>()+size_of::<Option<RetainedCloneSource<RecordValue>>>(),capacity_bytes:b.capacity_bytes,depth:b.depth,..Default::default()}}
 fn publish(&mut self,cx:&mut StepContext<'_>,result:Result<RetainedCloneProgress,ValueError>)->Result<MemberSnapshotOpenStep,OriginalTurnFailure>{
  let mut p=match &result{Ok(p)=>*p,Err(error)=>error.retained_progress()};
  let child=cx.receiving_child_progress();p.copied_items=p.copied_items.max(child.copied_items);p.copied_bytes=p.copied_bytes.max(child.copied_bytes);p.retained_capacity_bytes=p.retained_capacity_bytes.max(child.retained_capacity_bytes);p.released_bytes=p.released_bytes.max(child.released_bytes);self.last=p;
  let receipt=if let Some(issued)=cx.receiving_issued_grant(){cx.consume_receiving_retained(issued,p)}else{cx.consume_retained(p)};cx.consume_fuel(p.copied_items.saturating_add(p.copied_bytes)as u64);
  let receipt_refusal=receipt.err();
  if let Err(error)=result{self.diagnostic.get_or_insert(MemberOpenDiagnostic::Decode);let original=OriginalTurnFailure::from(error.with_retained_progress(p));return Err(OriginalTurnFailure{receipt_refusal,context_refusal:None,..original})}
  if let Some(error)=receipt_refusal{self.diagnostic.get_or_insert(MemberOpenDiagnostic::Capacity);let original=OriginalTurnFailure::from(invalid().with_retained_progress(p));return Err(OriginalTurnFailure{receipt_refusal:Some(error),context_refusal:None,..original})}
  if let Some(d)=self.diagnostic{return Ok(MemberSnapshotOpenStep::Rejected(d))}
  if p.copied_items>0{self.turns=self.turns.saturating_add(1);}if self.phase==Phase::Ready{return Ok(MemberSnapshotOpenStep::Ready)}
  Ok(MemberSnapshotOpenStep::Pending(MemberSnapshotOpenProgress{opening:MemberOpenProgress{phase:MemberOpenPhase::Snapshot,completed:self.turns,total:0},retained_progress:p}))
 }
 fn close_demand(&self,body:usize)->Result<RetirementDemand,ValueError>{
  if self.terminal{return Ok(Default::default())}if !self.closing{return Ok(fixed(2*size_of::<Self>()))}
  if let Some(r)=self.receiving.as_ref(){if !r.terminal_is_empty(){return r.close_demands(body)}return Ok(fixed(size_of::<Option<NativeReceivingOriginal<RecordValue,S>>>()))}
  if let Some(s)=self.source.as_ref(){if !s.terminal_is_empty(){return s.next_close_demand(body)}return Ok(fixed(size_of::<Option<RetainedCloneSource<RecordValue>>>()))}
  if let Some(c)=&self.record_close{if !c.terminal_is_empty(){return c.next_demand(body)}return Ok(fixed(size_of::<Option<ControlledRetirement<RecordValue>>>()))}
  if self.record.is_some(){return Ok(fixed(2*size_of::<Option<RecordValue>>()+2*size_of::<Option<ControlledRetirement<RecordValue>>>()))}
  if let Some(p)=self.parser.as_ref(){if !p.terminal_is_empty(){return p.close_demands(body)}return Ok(fixed(size_of::<Option<RetainedRecordReceiving>>()))}
  if let Some(c)=&self.snapshot_close{if !c.terminal_is_empty(){return c.next_demand(body)}return Ok(fixed(size_of::<Option<ControlledRetirement<S>>>()))}
  if self.snapshot.is_some(){return Ok(fixed(2*size_of::<Option<S>>()+2*size_of::<Option<ControlledRetirement<S>>>()))}
  if let Some(r)=self.request.as_ref(){if !r.terminal_is_empty(){return r.next_demand(body)}return Ok(fixed(size_of::<Option<MemberOpenRequest>>()))}
  Ok(fixed(size_of::<bool>()))
 }
}
impl<S:NativeReceive<RecordValue>+BorrowedDslRecord+pack::snapshot::ArtifactDsl+Send> OriginalRecordMemberSnapshotOpen<S>{
 /// 📏️ Borrows the next original opening demand without constructing parser, recipient or output backing.
 pub fn opening_demand(&self,cx:&StepContext<'_>,body:usize)->Result<RetirementDemand,ValueError>{
  let mut demand=match self.phase{
   Phase::Frame=>fixed(10*(size_of::<usize>()+size_of::<u64>())+size_of::<semio_framework_os_store_member_open::MemberOpenFrame>()),
   Phase::Envelope=>fixed(size_of::<usize>()+1),
   Phase::Parse=>if let Some(p)=self.parser.as_ref(){let source=self.request.as_ref().ok_or_else(invalid)?.original_snapshot_source(cx).map_err(semio_framework_os_store_member_open::member_admission_diagnostic)?;p.demands(&source)?}else{RetainedRecordReceiving::birth_demand()},
   Phase::TakeRecord=>fixed(size_of::<RecordValue>()),Phase::IssueOriginal=>Self::source_demand(),Phase::ReceivingShell=>NativeReceivingOriginal::<RecordValue,S>::shell_demand(),
   Phase::CaptureOriginal=>self.receiving.as_ref().ok_or_else(invalid)?.original_capture_demand()?,Phase::BirthReceiver=>self.receiving.as_ref().ok_or_else(invalid)?.receiver_birth_demand()?,
   Phase::Receive=>self.receiving.as_ref().ok_or_else(invalid)?.demands(body.saturating_sub(size_of::<Phase>()))?,Phase::TakeSnapshot=>self.receiving.as_ref().ok_or_else(invalid)?.take_demand()?,
   Phase::CloseReceiving=>self.receiving.as_ref().ok_or_else(invalid)?.close_demands(body.saturating_sub(size_of::<Phase>()))?,Phase::CloseParser=>self.parser.as_ref().ok_or_else(invalid)?.close_demands(body.saturating_sub(size_of::<Phase>()))?,Phase::Ready=>return Ok(Default::default()),
  };demand.copy_bytes=demand.copy_bytes.checked_add(size_of::<Phase>()).ok_or_else(invalid)?;Ok(demand)
 }

 fn advance_original(&mut self,cx:&mut StepContext<'_>,g:RetainedCloneGrant)->Result<RetainedCloneProgress,ValueError>{
  if self.phase==Phase::Ready{return Ok(Default::default())}let metadata=size_of::<Phase>();let d=self.opening_demand(cx,g.maximum_copy_bytes)?;if !retained_clone_grant_admits(g,d){return Ok(Default::default())}let child=RetainedCloneGrant{maximum_copy_bytes:g.maximum_copy_bytes-metadata,..g};
  let result=(||->Result<RetainedCloneProgress,ValueError>{Ok(match self.phase{
   Phase::Frame=>{let before=self.request.as_ref().ok_or_else(invalid)?.input_offset;let step=self.request.as_mut().ok_or_else(invalid)?.step_input(cx);let copied=self.request.as_ref().ok_or_else(invalid)?.input_offset-before;match step{MemberOpenInputStep::Framed(_)=>self.phase=Phase::Envelope,MemberOpenInputStep::Pending(_)=>{},MemberOpenInputStep::Rejected(d)=>{self.diagnostic=Some(d);}}progress(copied*(size_of::<usize>()+size_of::<u64>())+size_of::<semio_framework_os_store_member_open::MemberOpenFrame>())},
   Phase::Envelope=>{
    let source=self.request.as_ref().ok_or_else(invalid)?.original_snapshot_source(cx).map_err(semio_framework_os_store_member_open::member_admission_diagnostic)?;let length=semio_framework_os_semio::declared_envelope_prefix_len(S::envelope_id(),semio_framework_os_semio::Component::Pack,1)?;
    if source.original_source_identity().extent<=length||source.byte_at(self.envelope_offset)!=semio_framework_os_semio::declared_binary_prefix_byte(S::envelope_id(),semio_framework_os_semio::Component::Pack,1,self.envelope_offset)?{return Err(ValueError::literal(ValueRefusalKind::InvalidValue,"original member snapshot declared envelope mismatch"))}
    self.envelope_offset+=1;if self.envelope_offset==length{self.payload_start=Some(length);self.phase=Phase::Parse;}progress(size_of::<usize>()+1)
   },
   Phase::Parse=>{
    if self.parser.is_none(){let source=self.request.as_ref().ok_or_else(invalid)?.original_snapshot_source(cx).map_err(semio_framework_os_store_member_open::member_admission_diagnostic)?;let extent=source.original_source_identity().extent;let(p,r)=RetainedRecordReceiving::admit_document(&source,(self.payload_start.ok_or_else(invalid)?,extent),S::RECORD,PackLimits::default(),child)?.ok_or_else(invalid)?;*self.parser=Some(p);r}
    else{let source=self.request.as_ref().ok_or_else(invalid)?.original_snapshot_source(cx).map_err(semio_framework_os_store_member_open::member_admission_diagnostic)?;let parser=&mut self.parser;let p=cx.with_native_receiving(|native|parser.as_mut().ok_or_else(invalid)?.step(&source,native,child))?;if self.parser.as_ref().is_some_and(|p|p.is_ready()){self.phase=Phase::TakeRecord;}p}
   },
   Phase::TakeRecord=>{let p=self.parser.as_mut().ok_or_else(invalid)?.take_into(&mut self.record,child)?;if self.record.is_some(){self.phase=Phase::IssueOriginal;}p},
   Phase::IssueOriginal=>{let record=self.record.take().ok_or_else(invalid)?;let overhead=2*size_of::<Option<RecordValue>>()+size_of::<Option<RetainedCloneSource<RecordValue>>>();let source_grant=RetainedCloneGrant{maximum_copy_bytes:child.maximum_copy_bytes-overhead,..child};match RetainedCloneSource::admit_owned(record,(),source_grant){Ok((s,mut p))=>{*self.source=Some(s);self.phase=Phase::ReceivingShell;p.copied_bytes=p.copied_bytes.checked_add(overhead).ok_or_else(invalid)?;p},Err((error,record,_))=>{*self.record=Some(record);let mut p=error.retained_progress();p.copied_bytes=p.copied_bytes.checked_add(overhead).ok_or_else(invalid)?;return Err(error.with_retained_progress(p))}}},
   Phase::ReceivingShell=>{let(r,p)=NativeReceivingOriginal::<RecordValue,S>::begin_shell(child)?;*self.receiving=Some(r);self.phase=Phase::CaptureOriginal;p},
   Phase::CaptureOriginal=>{let p=self.receiving.as_mut().ok_or_else(invalid)?.admit_original(&mut self.source,child)?;if self.source.is_none(){self.phase=Phase::BirthReceiver;}p},
   Phase::BirthReceiver=>{let p=self.receiving.as_mut().ok_or_else(invalid)?.begin_receiver(child)?;if p.copied_items>0{self.phase=Phase::Receive;}p},
   Phase::Receive=>{let receiver=&mut self.receiving;let p=cx.with_native_receiving(|native|Ok::<_,ValueError>(receiver.as_mut().ok_or_else(invalid)?.advance(native,child.maximum_copy_bytes,child)?.progress()))?;if self.receiving.as_ref().is_some_and(|r|r.output_ready()){self.phase=Phase::TakeSnapshot;}p},
   Phase::TakeSnapshot=>{let(s,p)=self.receiving.as_mut().ok_or_else(invalid)?.take(child)?.ok_or_else(invalid)?;*self.snapshot=Some(s);self.receiving.as_mut().ok_or_else(invalid)?.begin_close();self.phase=Phase::CloseReceiving;p},
   Phase::CloseReceiving=>{let p=self.receiving.as_mut().ok_or_else(invalid)?.close_step(child)?.progress();if self.receiving.as_ref().is_some_and(|r|r.terminal_is_empty()){self.phase=Phase::CloseParser;}p},
   Phase::CloseParser=>{let p=self.parser.as_mut().ok_or_else(invalid)?.close_step(child)?.progress();if self.parser.as_ref().is_some_and(|r|r.terminal_is_empty()){self.phase=Phase::Ready;}p},Phase::Ready=>Default::default(),
  })})();match result{Ok(mut p)=>{p.copied_bytes=p.copied_bytes.checked_add(metadata).ok_or_else(invalid)?;Ok(p)},Err(error)=>{let mut p=error.retained_progress();p.copied_items=p.copied_items.max(1);p.copied_bytes=p.copied_bytes.saturating_add(metadata);Err(error.with_retained_progress(p))}}
 }
}
impl<S:NativeReceive<RecordValue>+BorrowedDslRecord+pack::snapshot::ArtifactDsl+Send> MemberSnapshotOpenOperation for OriginalRecordMemberSnapshotOpen<S>{
 type Snapshot=S;
 fn begin_birth_bytes(request:&MemberOpenRequest)->Result<usize,MemberOpenDiagnostic>{request.admitted_expected().map(|_|0)}
 fn begin(request:MemberOpenRequest)->Result<Self,MemberOpenAdmissionError>{
  if let Err(diagnostic)=request.admitted_expected(){return Err(MemberOpenAdmissionError{diagnostic,request})}
  Ok(Self{request:ManuallyDrop::new(Some(request)),parser:ManuallyDrop::new(None),record:ManuallyDrop::new(None),source:ManuallyDrop::new(None),receiving:ManuallyDrop::new(None),snapshot:ManuallyDrop::new(None),snapshot_close:None,record_close:None,phase:Phase::Frame,envelope_offset:0,payload_start:None,last:Default::default(),diagnostic:None,closing:false,terminal:false,turns:0})
 }
 fn step(&mut self,cx:&mut StepContext<'_>,grant:RetainedCloneGrant)->Result<MemberSnapshotOpenStep,OriginalTurnFailure>{
  self.last=Default::default();if self.closing||self.terminal{return Ok(MemberSnapshotOpenStep::Rejected(MemberOpenDiagnostic::Stale))}if let Some(d)=self.diagnostic{return Ok(MemberSnapshotOpenStep::Rejected(d))}
  if let Err(d)=self.request.as_ref().expect("original open request").check_step_authority(cx){self.diagnostic=Some(d);return Ok(MemberSnapshotOpenStep::Rejected(d))}
  let caller=cx.retained_grant();let g=RetainedCloneGrant{maximum_items:grant.maximum_items.min(caller.maximum_items).min(1),maximum_copy_bytes:grant.maximum_copy_bytes.min(caller.maximum_copy_bytes),maximum_capacity_bytes:grant.maximum_capacity_bytes.min(caller.maximum_capacity_bytes),maximum_release_bytes:grant.maximum_release_bytes.min(caller.maximum_release_bytes),maximum_depth:grant.maximum_depth.min(caller.maximum_depth)};
  if cx.should_yield(){return self.publish(cx,Ok(Default::default()))}let result=self.advance_original(cx,g);self.publish(cx,result)
 }
 fn take_ready(&mut self,cx:&mut StepContext<'_>)->Result<Option<(S,MemberOpenRequest)>,OriginalTurnFailure>{
  if self.phase!=Phase::Ready||self.closing||self.terminal||self.diagnostic.is_some(){return Ok(None)}
  self.request.as_ref().ok_or_else(invalid)?.check_step_authority(cx).map_err(semio_framework_os_store_member_open::member_admission_diagnostic)?;
  if self.snapshot.is_none()||!self.parser.as_ref().is_some_and(|p|p.terminal_is_empty())||!self.receiving.as_ref().is_some_and(|r|r.terminal_is_empty())||self.record.is_some()||self.source.is_some(){return Err(invalid().into())}
  let d=fixed(2*(size_of::<Option<S>>()+size_of::<Option<MemberOpenRequest>>()+size_of::<Option<RetainedRecordReceiving>>()+size_of::<Option<NativeReceivingOriginal<RecordValue,S>>>())+size_of::<bool>());if cx.should_yield()||!retained_clone_grant_admits(cx.retained_grant(),d){return Ok(None)}
  let p=progress(d.copy_bytes);self.publish(cx,Ok(p))?;let snapshot=self.snapshot.take().unwrap();let request=self.request.take().unwrap();self.parser.take();self.receiving.take();self.terminal=true;Ok(Some((snapshot,request)))
 }

}
impl<S:NativeReceive<RecordValue>+Send> ErasedSnapshotRetirement for OriginalRecordMemberSnapshotOpen<S>{
 fn next_demand(&self,body:usize)->Result<RetirementDemand,ValueError>{self.close_demand(body)}
 fn next_copy_byte_demand(&self)->Result<usize,ValueError>{Ok(self.close_demand(0)?.copy_bytes)}
 fn next_capacity_byte_demand(&self,body:usize)->Result<usize,ValueError>{Ok(self.close_demand(body)?.capacity_bytes)}
 fn next_release_byte_demand(&self)->Result<usize,ValueError>{Ok(self.close_demand(0)?.release_bytes)}
 fn next_depth_demand(&self)->Result<usize,ValueError>{Ok(self.close_demand(0)?.depth)}
 fn terminal_is_empty(&self)->bool{self.terminal&&self.request.is_none()&&self.parser.is_none()&&self.record.is_none()&&self.source.is_none()&&self.receiving.is_none()&&self.snapshot.is_none()&&self.record_close.is_none()&&self.snapshot_close.is_none()}
 fn close_step(&mut self,g:RetainedCloneGrant)->Result<RetainedCloneStep,ValueError>{
  if self.terminal{return Ok(RetainedCloneStep::Complete(Default::default()))}
  let d=self.close_demand(g.maximum_copy_bytes)?;if !retained_clone_grant_admits(g,d){return Ok(RetainedCloneStep::Progress(Default::default()))}
  if !self.closing{if let Some(r)=self.receiving.as_mut(){r.begin_close();}self.closing=true;return Ok(RetainedCloneStep::Progress(progress(d.copy_bytes)))}
  if let Some(r)=self.receiving.as_mut(){if !r.terminal_is_empty(){return r.close_step(g)}self.receiving.take();}
  else if let Some(s)=self.source.as_mut(){if !s.terminal_is_empty(){return s.close_step(g)}self.source.take();}
  else if let Some(c)=self.record_close.as_mut(){if !c.terminal_is_empty(){return c.step(g)}self.record_close.take();}
  else if let Some(record)=self.record.take(){match ControlledRetirement::new(record){Ok(c)=>self.record_close=Some(c),Err((error,r))=>{*self.record=Some(r);return Err(error)}}}
  else if let Some(p)=self.parser.as_mut(){if !p.terminal_is_empty(){return p.close_step(g)}self.parser.take();}
  else if let Some(c)=self.snapshot_close.as_mut(){if !c.terminal_is_empty(){return c.step(g)}self.snapshot_close.take();}
  else if let Some(snapshot)=self.snapshot.take(){match ControlledRetirement::new(snapshot){Ok(c)=>self.snapshot_close=Some(c),Err((error,s))=>{*self.snapshot=Some(s);return Err(error)}}}
  else if let Some(r)=self.request.as_mut(){if !r.terminal_is_empty(){return r.close_step(g)}self.request.take();}
  else{self.terminal=true;}
  Ok(if self.terminal{RetainedCloneStep::Complete(progress(d.copy_bytes))}else{RetainedCloneStep::Progress(progress(d.copy_bytes))})
 }
}
impl<S:NativeReceive<RecordValue>> Drop for OriginalRecordMemberSnapshotOpen<S>{fn drop(&mut self){assert!(std::thread::panicking()||self.terminal,"original member open abandoned its received source or output");}}

#[cfg(test)]#[path="🧪️tests/🦀️.rs"]mod tests;
