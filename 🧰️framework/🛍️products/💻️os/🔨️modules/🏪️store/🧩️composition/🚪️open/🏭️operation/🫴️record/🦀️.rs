//! 🫴️ A selected member receives its typed snapshot directly from the original sealed Pack pages.
use semio_framework_value_native_receiving_original::{NativeReceivingOriginal,StagedProjection};
use super::{MemberSnapshotOpenOperation,MemberSnapshotOpenStep,MemberSnapshotOpenProgress};
use semio_framework_os_store_member_open::{MemberOpenRequest,MemberOpenAdmissionError,MemberOpenDiagnostic,MemberOpenProgress,MemberOpenPhase,MemberOpenInputStep};
use semio_framework_value::{ValueError,ValueRefusalKind,RetirementDemand,ErasedSnapshotRetirement,retirement::{RetireOwned,controlled::ControlledRetirement},retained_clone::{RetainedCloneGrant,RetainedCloneProgress,RetainedCloneSource,RetainedCloneSourceAdmission,RetainedCloneStep,retained_clone_grant_admits}};
use semio_framework_value_native_owned::native_decoding::{owned::NativeReceive};
use semio_framework_dsl_record::{RecordValue,BorrowedDslRecord};
use semio_framework_job::{StepContext,OriginalTurnFailure};
use protocol::codec::RetainedWireByteSource;
use pack::{record::RetainedRecordReceiving,PackLimits};
use std::mem::{ManuallyDrop,size_of};
#[derive(Clone,Copy,PartialEq,Eq)]
enum Phase{Frame,Envelope,Parse,TakeRecord,IssueOriginal,StartReceiving,Receive,TakeSnapshot,CloseReceiving,CloseParser,Ready}
/// 📸️ Original wire, parsed source, native recipient and output keep separate custody until exact funded closure.
pub struct OriginalRecordMemberSnapshotOpen<S:NativeReceive<RecordValue>>{
 projection:StagedProjection<RecordValue>,
 request:ManuallyDrop<Option<MemberOpenRequest>>,parser:ManuallyDrop<Option<RetainedRecordReceiving>>,record:ManuallyDrop<Option<RecordValue>>,source:ManuallyDrop<Option<RetainedCloneSource<RecordValue>>>,admission:ManuallyDrop<Option<RetainedCloneSourceAdmission<RecordValue,RecordValue,()>>>,receiving:ManuallyDrop<Option<NativeReceivingOriginal<RecordValue,S>>>,snapshot:ManuallyDrop<Option<S>>,snapshot_close:Option<ControlledRetirement<S>>,record_close:Option<ControlledRetirement<RecordValue>>,phase:Phase,envelope_offset:usize,payload_start:Option<usize>,last:RetainedCloneProgress,diagnostic:Option<MemberOpenDiagnostic>,closing:bool,terminal:bool,turns:u64,receive_request:Option<(RetainedCloneGrant,u64)>,
}
fn flat<E:semio_framework_value_native_owned::native_decoding::NativeReceivingEffectError>(error:E)->ValueError{let progress=error.receiving_progress();error.cause().cloned().unwrap_or_else(invalid).with_retained_progress(progress)}
fn receiver_close_demand<S:NativeReceive<RecordValue>>(receiver:&NativeReceivingOriginal<RecordValue,S>,body:usize)->Result<RetirementDemand,ValueError>{
 if receiver.is_closing(){receiver.close_demands(body)}else{receiver.begin_close_demand()}
}
/// ♻️ Pays the receiver close start once, then closes it through its reply-shaped retirement cursor.
fn receiver_close<S:NativeReceive<RecordValue>>(receiver:&mut NativeReceivingOriginal<RecordValue,S>,body:usize,effect:&mut semio_framework_value::RetainedAllocationEffect<'_>)->semio_framework_value::OriginalRetirementPoll{
 use semio_framework_value::{retained_clone::OriginalEffectReply,retirement::{RetirementCursor,RetirementStep},RetainedWorkOutcome,RetainedWorkPoll};
 let mut slot=None;
 let ran=if !receiver.is_closing(){OriginalEffectReply::borrow(&mut slot).and_then(|reply|receiver.begin_close_into(effect,reply).map(drop)).map(|()|None)}else{let mut stepped=None;OriginalEffectReply::borrow(&mut stepped).and_then(|reply|RetirementCursor::close_into(receiver,body,effect,reply).map(drop)).map(|()|stepped.take())};
 let work=effect.performed_work();
 match(ran,slot.take()){
  (Err(error),_)=>RetainedWorkPoll::Complete(RetainedWorkOutcome{result:Err(error.into()),work}),
  (Ok(_),Some(settled))=>RetainedWorkPoll::Complete(RetainedWorkOutcome{result:settled,work}),
  (Ok(Some(settled)),None)=>RetainedWorkPoll::Complete(RetainedWorkOutcome{result:settled.and_then(|step|match step{RetirementStep::Progress(progress,_)=>Ok(RetainedCloneStep::Progress(progress)),RetirementStep::Failure(failure,_)=>Err(failure),RetirementStep::Complete(_)|RetirementStep::Pending(_)|RetirementStep::BudgetExhausted|RetirementStep::Advanced=>Ok(RetainedCloneStep::Progress(Default::default())),_=>Err(invalid().into())}),work}),
  (Ok(None),None)=>RetainedWorkPoll::Pending{work},
 }
}
fn invalid()->ValueError{ValueError::literal(ValueRefusalKind::InvariantViolated,"original member snapshot opening custody changed")}
fn fixed(copy_bytes:usize)->RetirementDemand{RetirementDemand{copy_bytes,depth:1,..Default::default()}}
fn progress(copy_bytes:usize)->RetainedCloneProgress{RetainedCloneProgress{copied_items:1,copied_bytes:copy_bytes,..Default::default()}}
fn spent(work:u128)->u64{u64::try_from(work).unwrap_or(u64::MAX)}
fn refused(failure:OriginalTurnFailure)->ValueError{let progress=failure.retained_progress();let kind=failure.primary_value().map_or(ValueRefusalKind::InvalidValue,|error|error.kind);ValueError::new(kind,failure.primary.message()).with_retained_progress(progress)}
/// 🪙️ Runs one effect-shaped owner action in the turn's own effect and charges its performed work; `None` means the turn could not fund it.
fn funded<T,E>(cx:&mut StepContext<'_>,grant:RetainedCloneGrant,run:impl FnOnce(&mut semio_framework_value::RetainedAllocationEffect<'_>)->semio_framework_value::RetainedWorkOutcome<T,E>)->Result<Option<Result<T,E>>,ValueError>{
 let Some((turn,performed))=cx.with_original_effect(grant,|_,effect|run(effect))? else{return Ok(None)};cx.consume_fuel(spent(performed.max(turn.work)));Ok(Some(turn.result))
}
/// ♻️ Runs one effect-shaped close poll; a pending poll keeps its owner state and reports no progress.
fn closed(cx:&mut StepContext<'_>,grant:RetainedCloneGrant,run:impl FnOnce(&mut semio_framework_value::RetainedAllocationEffect<'_>)->semio_framework_value::OriginalRetirementPoll)->Result<RetainedCloneProgress,ValueError>{
 let Some((poll,performed))=cx.with_original_effect(grant,|_,effect|run(effect))? else{return Ok(Default::default())};
 match poll{semio_framework_value::RetainedWorkPoll::Pending{work}=>{cx.consume_fuel(spent(work.max(performed)));Ok(Default::default())},semio_framework_value::RetainedWorkPoll::Complete(turn)=>{cx.consume_fuel(spent(turn.work.max(performed)));turn.result.map(|step|step.progress()).map_err(semio_framework_value::OriginalRetirementFailure::into_value_error)}}
}
/// ♻️ Runs one effect-shaped receiver close poll in an effect carrying the resident Source's own turn, which a resident schema matcher requires to close.
fn closed_on_source(cx:&mut StepContext<'_>,grant:RetainedCloneGrant,run:impl FnOnce(&mut semio_framework_value::RetainedAllocationEffect<'_>)->semio_framework_value::OriginalRetirementPoll)->Result<RetainedCloneProgress,ValueError>{
 let Some((poll,performed))=cx.with_original_source_effect(grant,|_,effect|run(effect))? else{return Ok(Default::default())};
 match poll{semio_framework_value::RetainedWorkPoll::Pending{work}=>{cx.consume_fuel(spent(work.max(performed)));Ok(Default::default())},semio_framework_value::RetainedWorkPoll::Complete(turn)=>{cx.consume_fuel(spent(turn.work.max(performed)));turn.result.map(|step|step.progress()).map_err(semio_framework_value::OriginalRetirementFailure::into_value_error)}}
}
/// 🫴️ Runs one receiving step in a local scope; `None` means the turn could not fund it.
fn received<T>(cx:&mut StepContext<'_>,floor:semio_framework_value::RetirementDemand,run:impl FnOnce(&mut semio_framework_value_native_owned::native_decoding::NativeDecodeControl<'_>,RetainedCloneGrant,u64)->T)->Result<Option<T>,ValueError>{
 let mut scope=None;let mut stepped=None;
 if !cx.with_native_receiving_work(&mut scope, floor, |control,grant,work|{stepped=Some(run(control,grant,work));Ok(())}).map_err(refused)?{return Ok(None)}
 let scope=scope.ok_or_else(invalid)?;
 if scope.source_refusal.is_some()||scope.call.is_err(){return Err(ValueError::literal(ValueRefusalKind::InvalidValue,"original member snapshot receiving scope refused").with_retained_progress(scope.retained_progress()))}
 stepped.map(Some).ok_or_else(invalid)
}
impl<S:NativeReceive<RecordValue>> OriginalRecordMemberSnapshotOpen<S>{
 /// 🧾️ The whole physical spend the Source booked during the turn, including the receiving scope's own control charge; the same caller debits it even when the semantic decoder refuses its input.
 pub fn last_receipt(&self)->RetainedCloneProgress{self.last}
 fn publish(&mut self,cx:&mut StepContext<'_>,start:RetainedCloneProgress,result:Result<RetainedCloneProgress,ValueError>)->Result<MemberSnapshotOpenStep,OriginalTurnFailure>{
  let mut p=match &result{Ok(p)=>*p,Err(error)=>error.retained_progress()};
  let child=cx.receiving_child_progress();p.copied_items=p.copied_items.max(child.copied_items);p.copied_bytes=p.copied_bytes.max(child.copied_bytes);p.retained_capacity_bytes=p.retained_capacity_bytes.max(child.retained_capacity_bytes);p.released_bytes=p.released_bytes.max(child.released_bytes);
  let receipt=if let Some(issued)=cx.receiving_issued_grant(){cx.consume_receiving_retained(issued,p)}else{cx.consume_retained(p)};cx.consume_fuel(p.copied_items.saturating_add(p.copied_bytes)as u64);
  let now=cx.retained_progress();p=RetainedCloneProgress{copied_items:now.copied_items.saturating_sub(start.copied_items).max(p.copied_items),copied_bytes:now.copied_bytes.saturating_sub(start.copied_bytes).max(p.copied_bytes),retained_capacity_bytes:now.retained_capacity_bytes.saturating_sub(start.retained_capacity_bytes).max(p.retained_capacity_bytes),released_bytes:now.released_bytes.saturating_sub(start.released_bytes).max(p.released_bytes)};self.last=p;
  let receipt_refusal=receipt.err();
  if let Err(error)=result{self.diagnostic.get_or_insert(MemberOpenDiagnostic::Decode);let original=OriginalTurnFailure::from(error.with_retained_progress(p));return Err(OriginalTurnFailure{receipt_refusal,context_refusal:None,..original})}
  if let Some(error)=receipt_refusal{self.diagnostic.get_or_insert(MemberOpenDiagnostic::Capacity);let original=OriginalTurnFailure::from(invalid().with_retained_progress(p));return Err(OriginalTurnFailure{receipt_refusal:Some(error),context_refusal:None,..original})}
  if let Some(d)=self.diagnostic{return Ok(MemberSnapshotOpenStep::Rejected(d))}
  if p.copied_items>0{self.turns=self.turns.saturating_add(1);}if self.phase==Phase::Ready{return Ok(MemberSnapshotOpenStep::Ready)}
  Ok(MemberSnapshotOpenStep::Pending(MemberSnapshotOpenProgress{opening:MemberOpenProgress{phase:MemberOpenPhase::Snapshot,completed:self.turns,total:0},retained_progress:p}))
 }
 fn close_demand(&self,body:usize)->Result<RetirementDemand,ValueError>{
  if self.terminal{return Ok(Default::default())}if !self.closing{return Ok(fixed(2*size_of::<Self>()))}
  if let Some(a)=self.admission.as_ref(){
   if let Some((demand,_))=self.projection.close_quote(a)?{return Ok(demand)}
   return if a.phase()=="ready"{a.next_demand()}else{a.next_return_demand()}
  }
  if let Some(r)=self.receiving.as_ref(){if !r.terminal_is_empty(){return receiver_close_demand(r,body)}return Ok(fixed(size_of::<Option<NativeReceivingOriginal<RecordValue,S>>>()))}
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
 fn receiving_birth_copy()->usize{
  size_of::<Option<RetainedCloneSource<RecordValue>>>()+size_of::<Option<NativeReceivingOriginal<RecordValue,S>>>()
 }

 /// 📏️ Borrows the next original opening demand without constructing parser, recipient or output backing.
 pub fn opening_demand(&self,cx:&StepContext<'_>,body:usize)->Result<RetirementDemand,ValueError>{
  let mut demand=match self.phase{
   Phase::Frame=>fixed(10*(size_of::<usize>()+size_of::<u64>())+size_of::<semio_framework_os_store_member_open::MemberOpenFrame>()),
   Phase::Envelope=>fixed(size_of::<usize>()+1),
   Phase::Parse=>if let Some(p)=self.parser.as_ref(){let source=self.request.as_ref().ok_or_else(invalid)?.original_snapshot_source(cx).map_err(semio_framework_os_store_member_open::member_admission_diagnostic)?;p.demands(&source)?}else{RetainedRecordReceiving::birth_demand()},
   Phase::TakeRecord=>fixed(size_of::<RecordValue>()),
   Phase::IssueOriginal=>{
    let admission=self.admission.as_ref().ok_or_else(invalid)?;
    match self.projection.quote(admission)?{Some((demand,_))=>demand,None=>admission.next_demand()?}
   },
   Phase::StartReceiving=>fixed(Self::receiving_birth_copy()),
   Phase::Receive=>{
    let(body,work)=self.receive_request.map_or_else(||(body.saturating_sub(size_of::<Phase>()),cx.fuel_remaining()),|(grant,work)|(grant.maximum_copy_bytes,work));
    self.receiving.as_ref().ok_or_else(invalid)?.demands(body,work)?
   },
   Phase::TakeSnapshot=>self.receiving.as_ref().ok_or_else(invalid)?.take_demand()?,
   Phase::CloseReceiving=>receiver_close_demand(self.receiving.as_ref().ok_or_else(invalid)?,body.saturating_sub(size_of::<Phase>()))?,Phase::CloseParser=>self.parser.as_ref().ok_or_else(invalid)?.close_demands(body.saturating_sub(size_of::<Phase>()))?,Phase::Ready=>return Ok(Default::default()),
  };demand.copy_bytes=demand.copy_bytes.checked_add(size_of::<Phase>()).ok_or_else(invalid)?;Ok(demand)
 }

 fn parse_step(&mut self,cx:&mut StepContext<'_>,child:RetainedCloneGrant)->Result<RetainedCloneProgress,ValueError>{
  let source=self.request.as_ref().ok_or_else(invalid)?.original_snapshot_source(cx).map_err(semio_framework_os_store_member_open::member_admission_diagnostic)?;let floor=self.parser.as_ref().ok_or_else(invalid)?.demands(&source)?;let parser=&mut self.parser;
  let Some((stepped,performed))=cx.with_original_effect(child,|cx,effect|received(cx,floor,|native,offered,units|parser.as_mut().map(|parser|parser.advance(&source,native,&mut effect.nested(offered),units))))? else{return Ok(Default::default())};
  let Some(Some(turn))=stepped? else{return Ok(Default::default())};cx.consume_fuel(spent(performed.max(turn.work)));
  let p=turn.result.map_err(flat)?;if self.parser.as_ref().is_some_and(|p|p.is_ready()){self.phase=Phase::TakeRecord;}Ok(p)
 }
 fn receive_step(&mut self,cx:&mut StepContext<'_>,child:RetainedCloneGrant)->Result<RetainedCloneProgress,ValueError>{
  let receiver=self.receiving.as_mut().ok_or_else(invalid)?;
  let request=self.receive_request;
  let (body,units)=request.map_or((child.maximum_copy_bytes,cx.fuel_remaining()),|(g,u)|(g.maximum_copy_bytes,u));
  let floor=receiver.demands(body,units)?;
  let Some(issued_turn)=received(cx,floor,|native,offered,work|{
   let available=child.restricted_by(offered);
   let (grant,units)=request.unwrap_or((available,work));
   let resuming=request.is_some();
   let funded=available.maximum_items>0&&available.maximum_copy_bytes>0&&available.maximum_depth>0&&work>0;
   if (!resuming&&(grant.restricted_by(available)!=grant||units>work))||(resuming&&!funded)
     ||grant.maximum_items<receiver.minimum_advance_items(grant.maximum_copy_bytes,units)?
   {return Ok::<_,ValueError>(None)}
   let poll=match native.source_witness(){
    Ok(witness)=>receiver.advance(native,witness,grant.maximum_copy_bytes,units,grant),
    Err(error)=>semio_framework_value::RetainedWorkPoll::Complete(semio_framework_value::RetainedWorkOutcome{result:Err(error.into()),work:0}),
   };
   Ok(Some((poll,(grant,units))))
  })? else{return Ok(Default::default())};
  let Some((poll,issued))=issued_turn? else{return Ok(Default::default())};
  let turn=match poll{semio_framework_value::RetainedWorkPoll::Pending{work}=>{self.receive_request=Some(issued);cx.consume_fuel(spent(work));return Ok(Default::default())},semio_framework_value::RetainedWorkPoll::Complete(turn)=>turn};
  let pending=receiver.advance_pending();
  self.receive_request=pending.then_some(issued);
  cx.consume_fuel(spent(turn.work));let p=turn.result.map_err(flat)?.progress();
  if !pending&&receiver.output_ready(){self.phase=Phase::TakeSnapshot;}Ok(p)
 }
 fn take_step(&mut self,cx:&mut StepContext<'_>,child:RetainedCloneGrant)->Result<RetainedCloneProgress,ValueError>{
  let receiving=self.receiving.as_mut().ok_or_else(invalid)?;
  let Some((poll,performed))=cx.with_original_effect(child,|_,effect|receiving.take(&mut self.snapshot,effect))? else{return Ok(Default::default())};
  let turn=match poll{semio_framework_value::RetainedWorkPoll::Pending{work}=>{cx.consume_fuel(spent(work.max(performed)));return Ok(Default::default())},semio_framework_value::RetainedWorkPoll::Complete(turn)=>turn};
  cx.consume_fuel(spent(turn.work.max(performed)));
  let step=turn.result.map_err(flat)?;
  if matches!(step,RetainedCloneStep::Complete(_)){
   if self.snapshot.is_none(){return Err(invalid())}
   self.phase=Phase::CloseReceiving;
  }
  Ok(step.progress())
 }

 fn issue_original(&mut self,cx:&mut StepContext<'_>,child:RetainedCloneGrant)->Result<RetainedCloneProgress,ValueError>{
  use semio_framework_value::RetainedWorkPoll;
  let admission=self.admission.as_mut().ok_or_else(invalid)?;
  if let Some((_,work))=self.projection.quote(admission)?{
   if cx.fuel_remaining()<work{return Ok(Default::default())}
   let Some((poll,performed))=cx.with_original_effect(child,|_,effect|self.projection.advance(admission,effect))? else{return Ok(Default::default())};
   return match poll{
    RetainedWorkPoll::Pending{work}=>{cx.consume_fuel(spent(work.max(performed)));Ok(Default::default())},
    RetainedWorkPoll::Complete(turn)=>{cx.consume_fuel(spent(turn.work.max(performed)));turn.result},
   };
  }
  if admission.phase()!="ready"{
   return match funded(cx,child,|effect|admission.advance(effect))?{Some(turn)=>Ok(turn?.progress()),None=>Ok(Default::default())};
  }
  let Some(step)=funded(cx,child,|effect|admission.take_output(&mut self.source,effect))? else{return Ok(Default::default())};
  let step=step?;
  if admission.terminal_is_empty(){
   self.admission.take();
   self.phase=Phase::StartReceiving;
  }
  Ok(step.progress())
 }

 fn advance_original(&mut self,cx:&mut StepContext<'_>,g:RetainedCloneGrant)->Result<RetainedCloneProgress,ValueError>{
  if self.phase==Phase::Ready{return Ok(Default::default())}let metadata=size_of::<Phase>();let d=self.opening_demand(cx,g.maximum_copy_bytes)?;if !retained_clone_grant_admits(g,d){return Ok(Default::default())}let child=RetainedCloneGrant{maximum_copy_bytes:g.maximum_copy_bytes-metadata,..g};
  let result=(||->Result<RetainedCloneProgress,ValueError>{Ok(match self.phase{
   Phase::Frame=>{let before=self.request.as_ref().ok_or_else(invalid)?.input_offset;let step=self.request.as_mut().ok_or_else(invalid)?.step_input(cx);let copied=self.request.as_ref().ok_or_else(invalid)?.input_offset-before;match step{MemberOpenInputStep::Framed(_)=>self.phase=Phase::Envelope,MemberOpenInputStep::Pending(_)=>{},MemberOpenInputStep::Rejected(d)=>{self.diagnostic=Some(d);}}progress(copied*(size_of::<usize>()+size_of::<u64>())+size_of::<semio_framework_os_store_member_open::MemberOpenFrame>())},
   Phase::Envelope=>{
    let source=self.request.as_ref().ok_or_else(invalid)?.original_snapshot_source(cx).map_err(semio_framework_os_store_member_open::member_admission_diagnostic)?;let length=semio_os_semio::declared_envelope_prefix_len(S::envelope_id(),semio_os_semio::Component::Pack,1)?;
    if source.original_source_identity().extent<=length||source.byte_at(self.envelope_offset)!=semio_os_semio::declared_binary_prefix_byte(S::envelope_id(),semio_os_semio::Component::Pack,1,self.envelope_offset)?{return Err(ValueError::literal(ValueRefusalKind::InvalidValue,"original member snapshot declared envelope mismatch"))}
    self.envelope_offset+=1;if self.envelope_offset==length{self.payload_start=Some(length);self.phase=Phase::Parse;}progress(size_of::<usize>()+1)
   },
   Phase::Parse=>{
    if self.parser.is_none(){let source=self.request.as_ref().ok_or_else(invalid)?.original_snapshot_source(cx).map_err(semio_framework_os_store_member_open::member_admission_diagnostic)?;let extent=source.original_source_identity().extent;let(p,r)=RetainedRecordReceiving::admit_document(&source,(self.payload_start.ok_or_else(invalid)?,extent),S::RECORD,PackLimits::default(),child).result?.ok_or_else(invalid)?;*self.parser=Some(p);r}
    else{self.parse_step(cx,child)?}
   },
   Phase::TakeRecord=>{let p=self.parser.as_mut().ok_or_else(invalid)?.take_into(&mut self.record,child).result?;if let Some(record)=self.record.take(){*self.admission=Some(RetainedCloneSource::admission_owned(record,()));self.phase=Phase::IssueOriginal;}p},
   Phase::IssueOriginal=>self.issue_original(cx,child)?,
   Phase::StartReceiving=>{
    let source=self.source.take().ok_or_else(invalid)?;
    *self.receiving=Some(NativeReceivingOriginal::new(source));
    self.phase=Phase::Receive;
    progress(Self::receiving_birth_copy())
   },
   Phase::Receive=>self.receive_step(cx,child)?,
   Phase::TakeSnapshot=>self.take_step(cx,child)?,
   Phase::CloseReceiving=>{let receiving=self.receiving.as_mut().ok_or_else(invalid)?;let p=closed_on_source(cx,child,|effect|receiver_close(receiving,child.maximum_copy_bytes,effect))?;if self.receiving.as_ref().is_some_and(|r|r.terminal_is_empty()){self.phase=Phase::CloseParser;}p},
   Phase::CloseParser=>{let parser=self.parser.as_mut().ok_or_else(invalid)?;let p=closed(cx,child,|effect|parser.close_step(child.maximum_copy_bytes,effect))?;if self.parser.as_ref().is_some_and(|r|r.terminal_is_empty()){self.phase=Phase::Ready;}p},Phase::Ready=>Default::default(),
  })})();match result{Ok(mut p)=>{p.copied_bytes=p.copied_bytes.checked_add(metadata).ok_or_else(invalid)?;Ok(p)},Err(error)=>{let mut p=error.retained_progress();p.copied_items=p.copied_items.max(1);p.copied_bytes=p.copied_bytes.saturating_add(metadata);Err(error.with_retained_progress(p))}}
 }
}
impl<S:NativeReceive<RecordValue>+BorrowedDslRecord+pack::snapshot::ArtifactDsl+Send> MemberSnapshotOpenOperation for OriginalRecordMemberSnapshotOpen<S>{
 type Snapshot=S;
 fn begin_birth_bytes(request:&MemberOpenRequest)->Result<usize,MemberOpenDiagnostic>{request.admitted_expected().map(|_|0)}
 fn begin(request:MemberOpenRequest)->Result<Self,MemberOpenAdmissionError>{
  if let Err(diagnostic)=request.admitted_expected(){return Err(MemberOpenAdmissionError{diagnostic,request})}
  Ok(Self{projection:Default::default(),request:ManuallyDrop::new(Some(request)),parser:ManuallyDrop::new(None),record:ManuallyDrop::new(None),source:ManuallyDrop::new(None),admission:ManuallyDrop::new(None),receiving:ManuallyDrop::new(None),snapshot:ManuallyDrop::new(None),snapshot_close:None,record_close:None,phase:Phase::Frame,envelope_offset:0,payload_start:None,last:Default::default(),diagnostic:None,closing:false,terminal:false,turns:0,receive_request:None})
 }
 fn step(&mut self,cx:&mut StepContext<'_>,grant:RetainedCloneGrant)->Result<MemberSnapshotOpenStep,OriginalTurnFailure>{
  self.last=Default::default();let start=cx.retained_progress();if self.closing||self.terminal{return Ok(MemberSnapshotOpenStep::Rejected(MemberOpenDiagnostic::Stale))}if let Some(d)=self.diagnostic{return Ok(MemberSnapshotOpenStep::Rejected(d))}
  if let Err(d)=self.request.as_ref().expect("original open request").check_step_authority(cx){self.diagnostic=Some(d);return Ok(MemberSnapshotOpenStep::Rejected(d))}
  let mut g=grant.restricted_by(cx.retained_grant());
  if self.phase!=Phase::Receive{g.maximum_items=g.maximum_items.min(1);}
  if cx.should_yield(){return self.publish(cx,start,Ok(Default::default()))}let result=self.advance_original(cx,g);self.publish(cx,start,result)
 }
 fn take_ready(&mut self,cx:&mut StepContext<'_>)->Result<Option<(S,MemberOpenRequest)>,OriginalTurnFailure>{
  if self.phase!=Phase::Ready||self.closing||self.terminal||self.diagnostic.is_some(){return Ok(None)}
  self.request.as_ref().ok_or_else(invalid)?.check_step_authority(cx).map_err(semio_framework_os_store_member_open::member_admission_diagnostic)?;
  if self.snapshot.is_none()||!self.parser.as_ref().is_some_and(|p|p.terminal_is_empty())||!self.receiving.as_ref().is_some_and(|r|r.terminal_is_empty())||self.record.is_some()||self.source.is_some()||self.admission.is_some()||!self.projection.terminal_is_empty(){return Err(invalid().into())}
  let d=fixed(2*(size_of::<Option<S>>()+size_of::<Option<MemberOpenRequest>>()+size_of::<Option<RetainedRecordReceiving>>()+size_of::<Option<NativeReceivingOriginal<RecordValue,S>>>())+size_of::<bool>());if cx.should_yield()||!retained_clone_grant_admits(cx.retained_grant(),d){return Ok(None)}
  let p=progress(d.copy_bytes);let start=cx.retained_progress();self.publish(cx,start,Ok(p))?;let snapshot=self.snapshot.take().unwrap();let request=self.request.take().unwrap();self.parser.take();self.receiving.take();self.terminal=true;Ok(Some((snapshot,request)))
 }

}
impl<S:NativeReceive<RecordValue>+Send> ErasedSnapshotRetirement for OriginalRecordMemberSnapshotOpen<S>{
    fn native_header_release(&self)->Option<semio_framework_value::RetirementHeaderRelease>{self.terminal_is_empty().then(||unsafe{semio_framework_value::RetirementHeaderRelease::native(self,0,0)})}
 fn next_demand(&self,body:usize)->Result<RetirementDemand,ValueError>{self.close_demand(body)}
 fn next_copy_byte_demand(&self)->Result<usize,ValueError>{Ok(self.close_demand(0)?.copy_bytes)}
 fn next_capacity_byte_demand(&self,body:usize)->Result<usize,ValueError>{Ok(self.close_demand(body)?.capacity_bytes)}
 fn next_release_byte_demand(&self)->Result<usize,ValueError>{Ok(self.close_demand(0)?.release_bytes)}
 fn next_depth_demand(&self)->Result<usize,ValueError>{Ok(self.close_demand(0)?.depth)}
 fn terminal_is_empty(&self)->bool{self.terminal&&self.projection.terminal_is_empty()&&self.request.is_none()&&self.parser.is_none()&&self.record.is_none()&&self.source.is_none()&&self.admission.is_none()&&self.receiving.is_none()&&self.snapshot.is_none()&&self.record_close.is_none()&&self.snapshot_close.is_none()}
 fn close_step(&mut self, _body: usize, effect: &mut semio_framework_value::RetainedAllocationEffect<'_>) -> semio_framework_value::OriginalRetirementPoll { let result = (|| -> Result<RetainedCloneStep, semio_framework_value::RetirementPollExit> { let g = effect.grant();
  if self.terminal{return Ok(RetainedCloneStep::Complete(Default::default()))}
  let d=self.close_demand(g.maximum_copy_bytes)?;if !retained_clone_grant_admits(g,d){return Ok(RetainedCloneStep::Progress(Default::default()))}
  if !self.closing{self.closing=true;return Ok(RetainedCloneStep::Progress(progress(d.copy_bytes)))}
  if let Some(admission)=self.admission.as_mut(){
   if let Some((_,work))=self.projection.close_quote(admission)?{
    if effect.remaining_work_units()?<work{return Ok(RetainedCloneStep::Progress(Default::default()))}
    return Ok(RetainedCloneStep::Progress(self.projection.close(admission,effect)?));
   }
  }
  if let Some(admission)=self.admission.as_mut(){
   let step=if admission.phase()=="ready"{
    admission.take_output(&mut self.source,&mut effect.nested(g)).result?
   }else{
    let mut inputs=None;
    let step=admission.return_inputs(&mut inputs,&mut effect.nested(g)).result?;
    if let Some((record,()))=inputs{*self.record=Some(record);}
    step
   };
   if admission.terminal_is_empty(){self.admission.take();}
   return Ok(RetainedCloneStep::Progress(step.progress()));
  }
  if let Some(r)=self.receiving.as_mut(){if !r.terminal_is_empty(){return semio_framework_value::RetirementPollExit::resume(receiver_close(r,g.maximum_copy_bytes,&mut effect.nested(g))).map_err(Into::into)}self.receiving.take();}
  else if let Some(s)=self.source.as_mut(){if !s.terminal_is_empty(){return semio_framework_value::RetirementPollExit::resume(s.close_step(g.maximum_copy_bytes,&mut effect.nested(g))).map_err(Into::into)}self.source.take();}
  else if let Some(c)=self.record_close.as_mut(){if !c.terminal_is_empty(){return semio_framework_value::RetirementPollExit::resume(c.step(g.maximum_copy_bytes,&mut effect.nested(g)))}self.record_close.take();}
  else if let Some(record)=self.record.take(){match ControlledRetirement::new(record){Ok(c)=>self.record_close=Some(c),Err((error,r))=>{*self.record=Some(r);return Err(error.into())}}}
  else if let Some(p)=self.parser.as_mut(){if !p.terminal_is_empty(){return semio_framework_value::RetirementPollExit::resume(p.close_step(g.maximum_copy_bytes,&mut effect.nested(g))).map_err(Into::into)}self.parser.take();}
  else if let Some(c)=self.snapshot_close.as_mut(){if !c.terminal_is_empty(){return semio_framework_value::RetirementPollExit::resume(c.step(g.maximum_copy_bytes,&mut effect.nested(g)))}self.snapshot_close.take();}
  else if let Some(snapshot)=self.snapshot.take(){match ControlledRetirement::new(snapshot){Ok(c)=>self.snapshot_close=Some(c),Err((error,s))=>{*self.snapshot=Some(s);return Err(error.into())}}}
  else if let Some(r)=self.request.as_mut(){if !r.terminal_is_empty(){return Ok(r.close_step(g)?)}self.request.take();}
  else{self.terminal=true;}
  Ok(if self.terminal{RetainedCloneStep::Complete(progress(d.copy_bytes))}else{RetainedCloneStep::Progress(progress(d.copy_bytes))})
 })(); semio_framework_value::RetirementPollExit::settle(effect, result) }
}
impl<S:NativeReceive<RecordValue>> Drop for OriginalRecordMemberSnapshotOpen<S>{fn drop(&mut self){assert!(std::thread::panicking()||self.terminal,"original member open abandoned its received source or output");}}

#[cfg(test)]#[path="🧪️tests/🦀️.rs"]mod tests;
