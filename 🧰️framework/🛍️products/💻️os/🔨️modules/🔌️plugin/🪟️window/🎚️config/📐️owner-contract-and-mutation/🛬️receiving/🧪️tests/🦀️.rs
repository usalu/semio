//! 📥️ Explicit interface faults traverse real pair/window erasure under one original caller, independently of natural codec faults.
use semio_plugin_window_config::WindowConfigNativePairReceiving;
use semio_os_store_codec_registry::native_snapshot::{OriginalArtifactPackReceiving,OriginalArtifactSnapshotReceiving,OriginalArtifactPairReceiving,OriginalArtifactPairFailure,OriginalArtifactPairCause,OriginalArtifactHistoryReadLimits};
use semio_framework_os_kernel::PackLimits;
use semio_framework_value::{RetainedWorkOutcome,RetainedCloneGrant,RetainedCloneProgress,RetainedCloneStep,RetirementDemand,ValueError,ValueRefusalKind};
use semio_framework_value_native_owned::native_decoding::{NativeDecodeControl,NativeDecodeCloseControl,OriginalReceivingPrimary};
use semio_framework_dsl_record::receiving::OwnedSlot;
use semio_framework_job::{OriginalJobFixtureCaller,OriginalJobDecodeOwner,StepContext,OriginalTurnFailure,OperationId,Generation};
use std::mem::size_of;
#[derive(semio_framework_value::RetireOwned)]
struct Probe;
fn corpus()->serde_json::Value{serde_json::from_str(include_str!("../../../../../../🏪️store/📦️codec/📸️native-snapshot/🫴️receiving/📜️pair/🧪️tests/🧫️fixtures/🔣️.json")).unwrap()}
const MESSAGE_BYTES:usize="semantic".len()+"quote".len()+"allocation".len();
struct Receiving{source:(usize,usize),control:(u64,Option<u64>),failure:OwnedSlot<OriginalReceivingPrimary>,phase:u8,closed:bool}
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
 fn step(&mut self,s:&[u8],n:&mut NativeDecodeControl<'_>,_:usize,g:RetainedCloneGrant)->RetainedWorkOutcome<RetainedCloneProgress,OriginalReceivingPrimary>{
  assert_eq!(self.control,(n.ledger_identity(),n.retirement_recipient_identity()));let d=self.demands(s,4096).unwrap();if !funds(g,d){return RetainedWorkOutcome{result:Ok(Default::default()),work:0}}
  if s[0]==b'W'{let before=n.actual_work();for expected in [b'W',b'o',b'r']{if !n.prepare_work(1).unwrap(){return RetainedWorkOutcome{result:Ok(Default::default()),work:n.actual_work()-before}}assert!(s.contains(&expected));n.commit_work(1).unwrap();}return RetainedWorkOutcome{result:Ok(Default::default()),work:n.actual_work()-before}}
  if self.phase==0{
   let mut work=0;let turn=n.receive_construction_step(d,|n,_|{
    if !n.prepare_work(1).unwrap(){return RetainedWorkOutcome{result:Ok(RetainedCloneStep::Progress(Default::default())),work:0}}n.commit_work(1).unwrap();work=1;
    *self.failure.value=Some(OriginalReceivingPrimary{retirement_failure:None,work_refusal:None,primary:Some(ValueError::new(ValueRefusalKind::InvalidValue,"semantic")),quote_refusal:Some(ValueError::new(ValueRefusalKind::WorkLimit,"quote")),allocation_refusal:Some(ValueError::new(ValueRefusalKind::AllocationFailed,"allocation")),allocation_failure:None,native_allocation:None,receipt_refusal:None,progress:Default::default()});self.phase=1;let mut p=progress(d);p.retained_capacity_bytes=MESSAGE_BYTES;
    RetainedWorkOutcome{result:n.record_constructed_allocation(MESSAGE_BYTES).map(|()|RetainedCloneStep::Progress(p)).map_err(|e|e.with_retained_progress(p)),work}
   });
   return RetainedWorkOutcome{result:turn.result.map(|s|s.progress()).map_err(|e|OriginalReceivingPrimary{retirement_failure:None,work_refusal:None,primary:e.primary,quote_refusal:None,allocation_refusal:None,allocation_failure:None,native_allocation:None,receipt_refusal:e.receipt_refusal,progress:e.progress}),work:turn.work}
  }
  if !n.prepare_work(1).unwrap(){return RetainedWorkOutcome{result:Ok(Default::default()),work:0}}
  let mut failure=self.failure.value.take().unwrap();let p=progress(d);failure.progress=p;n.commit_work(1).unwrap();
  n.receive_retained_work(g,RetainedWorkOutcome{result:Ok(RetainedCloneStep::Progress(p)),work:1}).result.expect("genuine fixture recipient accepts its quoted error transfer");
  RetainedWorkOutcome{result:Err(failure),work:1}
 }
 fn is_ready(&self)->bool{false}
 fn take_demand(&self)->Result<RetirementDemand,ValueError>{Ok(metadata::<Probe>())}
 fn take_into(&mut self,_:&mut Option<Probe>,_:RetainedCloneGrant)->Result<RetainedCloneProgress,ValueError>{Ok(Default::default())}
 fn close_demands(&self,b:usize)->Result<RetirementDemand,ValueError>{if !self.failure.terminal_is_empty(){self.failure.close_demands(b)}else{Ok(metadata::<bool>())}}
 fn close_step(&mut self,n:&mut dyn NativeDecodeCloseControl,g:RetainedCloneGrant)->RetainedWorkOutcome<RetainedCloneStep>{
  assert_eq!(self.control,(n.ledger_identity(),n.retirement_recipient_identity()));let d=self.close_demands(4096).unwrap();if !funds(g,d){return RetainedWorkOutcome{result:Ok(RetainedCloneStep::Progress(Default::default())),work:0}}
  if !self.failure.terminal_is_empty(){let turn=self.failure.close_step(g);return RetainedWorkOutcome{result:turn.result.map(RetainedCloneStep::Progress),work:turn.work}}self.closed=true;RetainedWorkOutcome{result:Ok(RetainedCloneStep::Complete(progress(d))),work:1}
 }
 fn terminal_is_empty(&self)->bool{self.closed&&self.failure.terminal_is_empty()}
}
fn stock()->RetainedCloneGrant{RetainedCloneGrant{maximum_items:100000,maximum_copy_bytes:1<<26,maximum_capacity_bytes:1<<26,maximum_release_bytes:1<<26,maximum_depth:128}}
fn quantum()->RetainedCloneGrant{RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:65536,maximum_capacity_bytes:65536,maximum_release_bytes:65536,maximum_depth:128}}
fn caller()->OriginalJobFixtureCaller{OriginalJobFixtureCaller::admit(OperationId(430),Generation(2),stock(),||Some(0),u64::MAX,1000000)}
fn history_limits()->OriginalArtifactHistoryReadLimits{OriginalArtifactHistoryReadLimits{file_bytes:4096,frame_body_bytes:4096,records:64,maximum_fields:64,maximum_text_bytes:4096,maximum_binary_bytes:4096,maximum_json_bytes:4096,maximum_json_allocation_bytes:4096,maximum_json_items:64,maximum_json_depth:32}}
fn measured<T>(c:&mut OriginalJobFixtureCaller,d:&mut OriginalJobDecodeOwner,run:impl FnOnce(&mut StepContext<'_>)->Result<T,OriginalTurnFailure>)->T{let(result,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||c.turn_with_decoder(quantum(),d,run));let p=c.receipt();assert_eq!((heap.requested_bytes,heap.released_bytes),(p.retained_capacity_bytes,p.released_bytes));assert_eq!(c.stock(),stock());result.unwrap()}
fn post(cx:&mut StepContext<'_>,run:impl FnOnce(&mut NativeDecodeControl<'_>,RetainedCloneGrant)->RetainedWorkOutcome<RetainedCloneProgress,OriginalReceivingPrimary>)->Result<RetainedCloneProgress,OriginalTurnFailure>{
 let g=cx.retained_grant();let mut work=0;let result=cx.with_native_receiving::<_,OriginalReceivingPrimary>(|n|{let turn=run(n,g);work=turn.work;turn.result});let p=match &result{Ok(p)=>*p,Err(e)=>e.retained_progress()};let receipt=cx.consume_receiving_retained(cx.receiving_issued_grant().unwrap(),p);cx.consume_fuel(work);
 match result{Ok(p)=>{receipt?;Ok(p)},Err(e)=>Err(semio_framework_job::OriginalTurnFailure{effects:p,primary:semio_framework_job::OriginalTurnPrimary::Receiving(e),receipt_refusal:receipt.err(),context_refusal:None,work_refusal:None})}
}
fn pair(c:&mut OriginalJobFixtureCaller,d:&mut OriginalJobDecodeOwner,source:&[u8])->OriginalArtifactPairReceiving<Probe>{
 let mut out=None;measured(c,d,|cx|post(cx,|n,g|{let result=Ok(OriginalArtifactPairReceiving::admit(source,b"spr",PackLimits::default(),history_limits(),n,g).expect("valid fixture pair source and policy").map(|(owner,p)|{out=Some(owner);RetainedCloneStep::Progress(p)}).unwrap());let turn=n.receive_retained_work(g,RetainedWorkOutcome{result,work:1});RetainedWorkOutcome{result:turn.result.map(|s|s.progress()),work:turn.work}}));out.unwrap()
}
fn close(c:&mut OriginalJobFixtureCaller,d:&mut OriginalJobDecodeOwner,source:&[u8],pair:&mut OriginalArtifactPairReceiving<Probe>){
 c.cancel().cancel_now();for _ in 0..10000{if pair.terminal_is_empty(){return}measured(c,d,|cx|cx.with_original_retirement(|cx|{let g=cx.retained_grant();let turn=cx.with_native_retirement(|n|Ok::<_,ValueError>(pair.close_step(source,b"spr",n,g)))?;let p=match &turn.result{Ok(s)=>s.progress(),Err(e)=>e.retained_progress()};cx.consume_retained(p)?;cx.consume_fuel(turn.work);Ok({turn.result.expect("paid fixture pair closure preserves its source and recipient");})}));}panic!("finite original pair cleanup")
}
#[test]
fn actual_three_validation_work_survives_pair_and_window_erasure_with_zero_items(){
 let mut c=caller();let mut d=OriginalJobDecodeOwner::empty();let source=b"Work";let mut pair=pair(&mut c,&mut d,source);let mut seen=false;
 for _ in 0..64{let mut work=0;let p=measured(&mut c,&mut d,|cx|post(cx,|n,g|{let erased:&mut dyn WindowConfigNativePairReceiving<Probe>=&mut pair;let turn=erased.step(source,b"spr",n,4096,g);work=turn.work;RetainedWorkOutcome{result:Ok(turn.result.expect("three validation fixture does not publish a fault")),work:turn.work}}));if work==corpus()["expectedWork"].as_u64().unwrap(){assert_eq!(p,RetainedCloneProgress::default());seen=true;break}}
 assert!(seen);close(&mut c,&mut d,source,&mut pair);finish(c,d);
}
fn finish(mut c:OriginalJobFixtureCaller,mut d:OriginalJobDecodeOwner){while !d.terminal_is_empty(){c.turn(quantum(),|cx|cx.with_original_retirement(|cx|{let turn=d.close_step(cx.retained_grant());let step=turn.result?;cx.consume_retained(step.progress())?;cx.consume_fuel(turn.work);Ok(())})).unwrap();}c.retire(quantum());}

fn failure_turn(pair:&mut OriginalArtifactPairReceiving<Probe>,held:&mut OwnedSlot<OriginalArtifactPairFailure>,source:&[u8],n:&mut NativeDecodeControl<'_>,g:RetainedCloneGrant)->RetainedWorkOutcome<RetainedCloneProgress,OriginalReceivingPrimary>{
 let bytes=2*size_of::<OriginalArtifactPairFailure>()+size_of::<Option<OriginalArtifactPairFailure>>();
 if held.value.is_some()||g.maximum_items==0||g.maximum_depth==0||g.maximum_copy_bytes<bytes{return RetainedWorkOutcome{result:Ok(Default::default()),work:0}}
 let child=RetainedCloneGrant{maximum_copy_bytes:g.maximum_copy_bytes-bytes,..g};
 let erased:&mut dyn WindowConfigNativePairReceiving<Probe>=pair;let turn=erased.step(source,b"spr",n,4096,child);
 match turn.result{
  Ok(p)=>RetainedWorkOutcome{result:Ok(p),work:turn.work},
  Err(error)=>{
   assert!(held.value.is_none());let p=error.retained_progress();*held.value=Some(error);
   let captured=RetainedCloneProgress{copied_bytes:bytes,..Default::default()};
   let remaining=RetainedCloneGrant{maximum_items:g.maximum_items-p.copied_items,maximum_copy_bytes:g.maximum_copy_bytes-p.copied_bytes,maximum_capacity_bytes:g.maximum_capacity_bytes-p.retained_capacity_bytes,maximum_release_bytes:g.maximum_release_bytes-p.released_bytes,..g};
   n.receive_retained_work(remaining,RetainedWorkOutcome{result:Ok(RetainedCloneStep::Progress(captured)),work:0}).result.expect("same original grant reserved complete failure capture");
   RetainedWorkOutcome{result:Ok(p.checked_add(captured).unwrap()),work:turn.work}
  }
 }
}
#[test]
fn actual_triple_failure_owners_cross_erasure_once_and_close_with_the_original_caller(){
 let mut c=caller();let mut d=OriginalJobDecodeOwner::empty();let source=b"Fault";let mut pair=pair(&mut c,&mut d,source);let mut held=OwnedSlot::<OriginalArtifactPairFailure>::default();
 let denied=measured(&mut c,&mut d,|cx|post(cx,|n,g|failure_turn(&mut pair,&mut held,source,n,RetainedCloneGrant{maximum_copy_bytes:0,..g})));assert_eq!(denied,Default::default());assert!(held.value.is_none());
 for _ in 0..64{measured(&mut c,&mut d,|cx|post(cx,|n,g|failure_turn(&mut pair,&mut held,source,n,g)));if held.value.is_some(){break}}
 let error=held.value.as_ref().expect("full failure moved into a prequoted vacant caller slot");let OriginalArtifactPairCause::Native(error)=&error.cause else{panic!("complete native cause survives pair erasure")};
 let actual=serde_json::json!([error.primary.as_ref().unwrap().message.as_ref(),error.quote_refusal.as_ref().unwrap().message.as_ref(),error.allocation_refusal.as_ref().unwrap().message.as_ref()]);
 assert_eq!(actual,corpus()["causes"]);let pointer=error.primary.as_ref().unwrap().message.as_ptr();
 let repeated=measured(&mut c,&mut d,|cx|post(cx,|n,g|failure_turn(&mut pair,&mut held,source,n,g)));assert_eq!(repeated,Default::default());
 let OriginalArtifactPairCause::Native(error)=&held.value.as_ref().unwrap().cause else{unreachable!()};assert_eq!(error.primary.as_ref().unwrap().message.as_ptr(),pointer);
 c.cancel().cancel_now();for _ in 0..10000{if held.terminal_is_empty(){break}let demand=held.close_demands(4096).unwrap();measured(&mut c,&mut d,|cx|cx.with_original_retirement(|cx|semio_framework_job::close_original_retained_step(cx,demand,|g|{let turn=held.close_step(g);RetainedWorkOutcome{result:turn.result.map(RetainedCloneStep::Progress),work:turn.work}})));}
 assert!(held.terminal_is_empty());close(&mut c,&mut d,source,&mut pair);finish(c,d);
}

#[test]
fn actual_grant_restriction_keeps_all_five_original_bounds(){for row in corpus()["grantCases"].as_array().unwrap(){let read=|key:&str|{let values=row[key].as_array().unwrap();RetainedCloneGrant{maximum_items:values[0].as_u64().unwrap()as usize,maximum_copy_bytes:values[1].as_u64().unwrap()as usize,maximum_capacity_bytes:values[2].as_u64().unwrap()as usize,maximum_release_bytes:values[3].as_u64().unwrap()as usize,maximum_depth:values[4].as_u64().unwrap()as usize}};assert_eq!(read("caller").restricted_by(read("source")),read("expected"));}}
