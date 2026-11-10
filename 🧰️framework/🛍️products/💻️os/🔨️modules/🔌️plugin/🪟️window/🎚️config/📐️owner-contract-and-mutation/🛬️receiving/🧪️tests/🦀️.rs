//! 📥️ Explicit interface faults traverse real pair/window erasure under one original caller, independently of natural codec faults.
use semio_plugin_window_config::WindowConfigNativePairReceiving;
use semio_os_store_codec_registry::native_snapshot::{OriginalArtifactPackReceiving,OriginalArtifactSnapshotReceiving,OriginalArtifactPairReceiving,OriginalArtifactPairFailure,OriginalArtifactPairCause,OriginalArtifactHistoryReadLimits};
use semio_os_kernel::PackLimits;
use semio_framework_value::{OriginalRetirementFailure,OriginalRetirementPoll,RetainedAllocationEffect,RetainedWorkOutcome,RetainedWorkPoll,RetainedCloneGrant,RetainedCloneProgress,RetainedCloneStep,RetirementDemand,ValueError,ValueRefusalKind};
use semio_framework_value_native_owned::{NativeAllocationIdentity,native_decoding::{NativeDecodeControl,OriginalReceivingPrimary}};
use semio_framework_dsl_record::receiving::OwnedSlot;
use semio_framework_job::{OriginalJobFixtureCaller,OriginalJobDecodeOwner,OriginalNativeReceivingOutcome,StepContext,OriginalTurnFailure,OperationId,Generation,close_original_effect_step,receive_original_receiving_work,receive_original_retained_work};
use std::mem::size_of;
#[derive(semio_framework_value::RetireOwned)]
struct Probe;
fn corpus()->serde_json::Value{serde_json::from_str(include_str!("../../../../../../🏪️store/📦️codec/📸️native-snapshot/🫴️receiving/📜️pair/🧪️tests/🧫️fixtures/🔣️.json")).unwrap()}
const MESSAGE_BYTES:usize="semantic".len()+"quote".len()+"allocation".len();
struct Receiving{source:(usize,usize),control:(NativeAllocationIdentity,Option<u64>),failure:OwnedSlot<OriginalReceivingPrimary>,phase:u8,closed:bool}
fn metadata<T>()->RetirementDemand{RetirementDemand{copy_bytes:size_of::<T>(),depth:1,..Default::default()}}
fn progress(d:RetirementDemand)->RetainedCloneProgress{RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,..Default::default()}}
fn funds(g:RetainedCloneGrant,d:RetirementDemand)->bool{semio_framework_value::retained_clone::retained_clone_grant_admits(g,d)}
fn invalid()->ValueError{ValueError::literal(ValueRefusalKind::InvalidValue,"probe source identity changed")}
impl OriginalArtifactPackReceiving for Probe{
 type Receiving=Receiving;
 fn original_pack_birth_demand()->RetirementDemand{metadata::<Receiving>()}
 fn admit_original_pack(source:&[u8],_:PackLimits,n:&NativeDecodeControl<'_>,g:RetainedCloneGrant)->Result<Option<(Receiving,RetainedCloneProgress)>,ValueError>{
  let d=Self::original_pack_birth_demand();if !funds(g,d){return Ok(None)}if !matches!(source.first(),Some(b'W'|b'F')){return Err(invalid())}
  Ok(Some((Receiving{source:(source.as_ptr()as usize,source.len()),control:(n.ledger_identity(),n.retirement_recipient_identity()),failure:Default::default(),phase:0,closed:false},progress(d))))
 }
}
impl OriginalArtifactSnapshotReceiving<Probe> for Receiving{
 fn demands(&self,s:&[u8],_:usize)->Result<RetirementDemand,ValueError>{if self.source!=(s.as_ptr()as usize,s.len()){return Err(invalid())}Ok(if s[0]==b'F'&&self.phase==0{RetirementDemand{capacity_bytes:MESSAGE_BYTES,..metadata::<(OriginalReceivingPrimary,u8)>()}}else if self.failure.value.is_some(){metadata::<OriginalReceivingPrimary>()}else{RetirementDemand{depth:1,..Default::default()}})}
 fn step(&mut self,s:&[u8],n:&mut NativeDecodeControl<'_>,_:usize,e:&mut RetainedAllocationEffect<'_>)->RetainedWorkOutcome<RetainedCloneProgress,OriginalReceivingPrimary>{
  assert_eq!(self.control,(n.ledger_identity(),n.retirement_recipient_identity()));let result=self.action(s,e);RetainedWorkOutcome{result,work:e.performed_work()}
 }
 fn is_ready(&self)->bool{false}
 fn take_demand(&self)->Result<RetirementDemand,ValueError>{Ok(metadata::<Probe>())}
 fn take_into(&mut self,_:&mut Option<Probe>,_:RetainedCloneGrant)->Result<RetainedCloneProgress,ValueError>{Ok(Default::default())}
 fn close_demands(&self,b:usize)->Result<RetirementDemand,ValueError>{if !self.failure.terminal_is_empty(){self.failure.close_demands(b)}else{Ok(metadata::<bool>())}}
 fn close_step(&mut self,e:&mut RetainedAllocationEffect<'_>)->OriginalRetirementPoll{
  let settle=|e:&RetainedAllocationEffect<'_>,result:Result<RetainedCloneStep,ValueError>|RetainedWorkPoll::Complete(RetainedWorkOutcome{result:result.map_err(OriginalRetirementFailure::from),work:e.performed_work()});
  let d=match self.close_demands(4096){Ok(d)=>d,Err(error)=>return settle(e,Err(error))};if !funds(e.grant(),d){return settle(e,Ok(RetainedCloneStep::Progress(Default::default())))}
  if !self.failure.terminal_is_empty(){return stepped(self.failure.close_step(&mut e.nested(e.grant())))}
  if let Err(error)=e.performed(1){return settle(e,Err(error))}self.closed=true;settle(e,Ok(RetainedCloneStep::Complete(progress(d))))
 }
 fn terminal_is_empty(&self)->bool{self.closed&&self.failure.terminal_is_empty()}
}
impl Receiving{
 fn action(&mut self,s:&[u8],e:&mut RetainedAllocationEffect<'_>)->Result<RetainedCloneProgress,OriginalReceivingPrimary>{
  let d=OriginalArtifactSnapshotReceiving::<Probe>::demands(self,s,4096)?;if !funds(e.grant(),d){return Ok(Default::default())}
  if s[0]==b'W'{for expected in [b'W',b'o',b'r']{if !e.performed(1)?{return Ok(Default::default())}assert!(s.contains(&expected));}return Ok(Default::default())}
  if !e.performed(1)?{return Ok(Default::default())}
  if self.phase==0{
   if e.actual().is_some(){return Err(invalid().into())}*e.actual()=Some(0);
   *self.failure.value=Some(OriginalReceivingPrimary::from_fields(Some(ValueError::new(ValueRefusalKind::InvalidValue,"semantic")),Some(ValueError::new(ValueRefusalKind::WorkLimit,"quote")),Some(ValueError::new(ValueRefusalKind::AllocationFailed,"allocation")),None,None,None,None,None,None,Default::default()));
   *e.actual()=Some(MESSAGE_BYTES);self.phase=1;let mut p=progress(d);p.retained_capacity_bytes=MESSAGE_BYTES;return Ok(p)
  }
  let mut failure=self.failure.value.take().unwrap();failure.progress=progress(d);Err(failure)
 }
}
fn stepped(poll:OriginalRetirementPoll<RetainedCloneProgress>)->OriginalRetirementPoll{match poll{RetainedWorkPoll::Pending{work}=>RetainedWorkPoll::Pending{work},RetainedWorkPoll::Complete(turn)=>RetainedWorkPoll::Complete(RetainedWorkOutcome{result:turn.result.map(RetainedCloneStep::Progress),work:turn.work})}}
fn stock()->RetainedCloneGrant{RetainedCloneGrant{maximum_items:100000,maximum_copy_bytes:1<<26,maximum_capacity_bytes:1<<26,maximum_release_bytes:1<<26,maximum_depth:128}}
fn quantum()->RetainedCloneGrant{RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:65536,maximum_capacity_bytes:65536,maximum_release_bytes:65536,maximum_depth:128}}
fn caller()->OriginalJobFixtureCaller{OriginalJobFixtureCaller::admit(OperationId(430),Generation(2),stock(),||Some(0),u64::MAX,1000000)}
fn history_limits()->OriginalArtifactHistoryReadLimits{OriginalArtifactHistoryReadLimits{file_bytes:4096,frame_body_bytes:4096,records:64,maximum_fields:64,maximum_text_bytes:4096,maximum_binary_bytes:4096,maximum_json_bytes:4096,maximum_json_allocation_bytes:4096,maximum_json_items:64,maximum_json_depth:32}}
fn measured_with<T>(grant:RetainedCloneGrant,c:&mut OriginalJobFixtureCaller,d:&mut OriginalJobDecodeOwner,run:impl FnOnce(&mut StepContext<'_>)->Result<T,OriginalTurnFailure>)->T{let(result,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||c.turn_with_decoder(grant,d,run));let p=c.receipt();assert_eq!((heap.requested_bytes,heap.released_bytes),(p.retained_capacity_bytes,p.released_bytes));assert_eq!(c.stock(),stock());result.unwrap()}
fn measured<T>(c:&mut OriginalJobFixtureCaller,d:&mut OriginalJobDecodeOwner,run:impl FnOnce(&mut StepContext<'_>)->Result<T,OriginalTurnFailure>)->T{measured_with(quantum(),c,d,run)}
fn settled<R>(cx:&mut StepContext<'_>,scope:Option<OriginalNativeReceivingOutcome>,stepped:Option<Result<R,ValueError>>)->Result<R,OriginalTurnFailure>{
 let scope=scope.ok_or_else(invalid)?;cx.settle_receiving_scope()?;
 if scope.source_refusal.is_some()||scope.call.is_err(){return Err(invalid().with_retained_progress(scope.retained_progress()).into())}
 Ok(stepped.ok_or_else(invalid)??)
}
fn received<R>(cx:&mut StepContext<'_>,run:impl FnOnce(&mut NativeDecodeControl<'_>,RetainedCloneGrant)->Result<R,ValueError>)->Result<Option<R>,OriginalTurnFailure>{
 if cx.should_yield(){return Ok(None)}
 let(mut scope,mut stepped,mut committed)=(None,None,0);
 if !cx.with_native_receiving(&mut scope,|native,grant|{let before=native.actual_work();stepped=Some(run(native,grant));committed=native.actual_work().saturating_sub(before);Ok(())})?{return Ok(None)}
 cx.consume_fuel(u64::try_from(committed).unwrap_or(u64::MAX));settled(cx,scope,stepped).map(Some)
}
thread_local!{static BOUND:std::cell::RefCell<Option<(NativeAllocationIdentity,Option<u64>)>>=const{std::cell::RefCell::new(None)};}
fn bound()->Option<(NativeAllocationIdentity,Option<u64>)>{BOUND.with(|b|b.borrow().clone())}
fn pair(c:&mut OriginalJobFixtureCaller,d:&mut OriginalJobDecodeOwner,source:&[u8])->OriginalArtifactPairReceiving<Probe>{
 let mut out=None;
 measured(c,d,|cx|{let mut scope=None;let mut admitted=None;cx.with_native_receiving(&mut scope,|n,g|{BOUND.with(|b|*b.borrow_mut()=Some((n.ledger_identity(),n.retirement_recipient_identity())));admitted=Some(OriginalArtifactPairReceiving::admit(source,b"spr",PackLimits::default(),history_limits(),n,g).expect("valid fixture pair source and policy"));Ok(())})?;
  let(owner,p)=admitted.flatten().expect("funded fixture pair admission");out=Some(owner);receive_original_receiving_work(cx,RetainedWorkOutcome::<RetainedCloneStep,ValueError>{result:Ok(RetainedCloneStep::Progress(p)),work:1})?;Ok(())});
 out.unwrap()
}
fn step_pair(cx:&mut StepContext<'_>,pair:&mut dyn WindowConfigNativePairReceiving<Probe>,source:&[u8],mut held:Option<&mut OwnedSlot<OriginalArtifactPairFailure>>,work:&mut u128)->Result<RetainedCloneProgress,OriginalTurnFailure>{
 let maximum=cx.fuel_remaining();let capture=held.as_ref().map_or(0,|_|2*size_of::<OriginalArtifactPairFailure>()+size_of::<Option<OriginalArtifactPairFailure>>());let had=held.as_ref().is_some_and(|slot|slot.value.is_some());
 let slot=held.as_deref_mut();
 let progress=received(cx,|native,grant|{
  let mut d=pair.demands(source,b"spr",&*native,4096).expect("fixture pair quote");d.copy_bytes+=capture;if had||!funds(grant,d){return Ok(None)}
  Ok(native.with_allocated_effect(d,maximum,|native,effect|{let turn=pair.step(source,b"spr",native,4096,effect);*work=turn.work;match turn.result{Ok(p)=>p,Err(error)=>{let p=error.retained_progress();let slot=slot.expect("fixture pair refused without a capture slot");assert!(slot.value.is_none());*slot.value=Some(error);p}}})?)
 })?.flatten();
 if capture>0&&!had&&held.as_ref().is_some_and(|slot|slot.value.is_some()){cx.consume_retained(RetainedCloneProgress{copied_bytes:capture,..Default::default()})?;}
 Ok(progress.unwrap_or_default())
}
fn close(c:&mut OriginalJobFixtureCaller,d:&mut OriginalJobDecodeOwner,source:&[u8],pair:&mut OriginalArtifactPairReceiving<Probe>){
 c.cancel().cancel_now();for _ in 0..10000{if pair.terminal_is_empty(){return}measured(c,d,|cx|cx.with_original_retirement(|cx|{let grant=RetainedCloneGrant{maximum_items:1,..cx.retained_grant()};
  let Some((polled,performed))=cx.with_original_effect(grant,|cx,effect|cx.with_native_retirement(|n|{assert_eq!(bound(),Some((n.ledger_identity(),n.retirement_recipient_identity())),"close runs under the decoder control bound at admission");Ok::<_,ValueError>(pair.close_step(source,b"spr",n,effect))}))?else{return Ok(())};
  let turn=match polled?{RetainedWorkPoll::Pending{work}=>RetainedWorkOutcome::<RetainedCloneStep,ValueError>{result:Ok(RetainedCloneStep::Progress(Default::default())),work},RetainedWorkPoll::Complete(turn)=>RetainedWorkOutcome{result:Ok(turn.result.expect("paid fixture pair closure preserves its source and recipient")),work:turn.work}};
  assert!(performed<=turn.work);receive_original_retained_work(cx,turn)?;Ok(())}));}panic!("finite original pair cleanup")
}
#[test]
fn actual_three_validation_work_survives_pair_and_window_erasure_with_zero_items(){
 let mut c=caller();let mut d=OriginalJobDecodeOwner::empty();let source=b"Work";let mut pair=pair(&mut c,&mut d,source);let mut seen=false;
 for _ in 0..64{let mut work=0;let p=measured(&mut c,&mut d,|cx|step_pair(cx,&mut pair,source,None,&mut work));if work==u128::from(corpus()["expectedWork"].as_u64().unwrap()){assert_eq!(p,RetainedCloneProgress::default());seen=true;break}}
 assert!(seen);close(&mut c,&mut d,source,&mut pair);finish(c,d);
}
fn finish(mut c:OriginalJobFixtureCaller,mut d:OriginalJobDecodeOwner){while !d.terminal_is_empty(){c.turn(quantum(),|cx|cx.with_original_retirement(|cx|{d.close_on_source(cx)?;Ok(())})).unwrap();}c.retire(quantum());}

#[test]
fn actual_triple_failure_owners_cross_erasure_once_and_close_with_the_original_caller(){
 let mut c=caller();let mut d=OriginalJobDecodeOwner::empty();let source=b"Fault";let mut pair=pair(&mut c,&mut d,source);let mut held=OwnedSlot::<OriginalArtifactPairFailure>::default();let mut work=0;
 let denied=measured_with(RetainedCloneGrant{maximum_copy_bytes:0,..quantum()},&mut c,&mut d,|cx|step_pair(cx,&mut pair,source,Some(&mut held),&mut work));assert_eq!(denied,Default::default());assert!(held.value.is_none());
 for _ in 0..64{measured(&mut c,&mut d,|cx|step_pair(cx,&mut pair,source,Some(&mut held),&mut work));if held.value.is_some(){break}}
 let error=held.value.as_ref().expect("full failure moved into a prequoted vacant caller slot");let OriginalArtifactPairCause::Native(error)=&error.cause else{panic!("complete native cause survives pair erasure")};
 let actual=serde_json::json!([error.primary.as_ref().unwrap().message.as_ref(),error.quote_refusal.as_ref().unwrap().message.as_ref(),error.allocation_refusal.as_ref().unwrap().message.as_ref()]);
 assert_eq!(actual,corpus()["causes"]);let pointer=error.primary.as_ref().unwrap().message.as_ptr();
 let repeated=measured(&mut c,&mut d,|cx|step_pair(cx,&mut pair,source,Some(&mut held),&mut work));assert_eq!(repeated,Default::default());
 let OriginalArtifactPairCause::Native(error)=&held.value.as_ref().unwrap().cause else{unreachable!()};assert_eq!(error.primary.as_ref().unwrap().message.as_ptr(),pointer);
 c.cancel().cancel_now();for _ in 0..10000{if held.terminal_is_empty(){break}let demand=held.close_demands(4096).unwrap();measured(&mut c,&mut d,|cx|cx.with_original_retirement(|cx|close_original_effect_step(cx,demand,|effect|stepped(held.close_step(effect))).map(|_|())));}
 assert!(held.terminal_is_empty());close(&mut c,&mut d,source,&mut pair);finish(c,d);
}

#[test]
fn actual_grant_restriction_keeps_all_five_original_bounds(){for row in corpus()["grantCases"].as_array().unwrap(){let read=|key:&str|{let values=row[key].as_array().unwrap();RetainedCloneGrant{maximum_items:values[0].as_u64().unwrap()as usize,maximum_copy_bytes:values[1].as_u64().unwrap()as usize,maximum_capacity_bytes:values[2].as_u64().unwrap()as usize,maximum_release_bytes:values[3].as_u64().unwrap()as usize,maximum_depth:values[4].as_u64().unwrap()as usize}};assert_eq!(read("caller").restricted_by(read("source")),read("expected"));}}
