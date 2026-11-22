//! 🧫️ Actual generic operation cursors move original typed fields through one-item native loans.
use super::*;
use protocol::{OpBinary,OpBinaryReceiving,OpBinaryReceivingCursor};
use semio_framework_dsl_record::receiving::owned::OwnedAdvanceSelection;
use semio_framework_value::{AuthoringWallet, RetainedAllocationEffect, RetainedWorkPoll, ORIGINAL_INVOCATION_WORK, retirement::controlled::ControlledRetirement};
use semio_framework_value_native_owned::OriginalSourceTurnIssuer;
#[path="🧰️bank.rs"]
mod bank;
use bank::OperationBank;
#[derive(Clone,Debug,PartialEq,semio_framework_dsl_record_derive::DslEnum,semio_framework_dsl_record_derive::DslReceiving,semio_framework_value::RetireOwned)]
enum OriginalMutation{Rename{text:String,items:Vec<String>}}
impl OpBinary for OriginalMutation{
 fn encode_op(&self)->Result<Vec<u8>,protocol::ProtocolError>{semio_os_kernel::os_dsl::variants_binary::encode_op(self)}
 fn decode_op(bytes:&[u8])->Result<Self,protocol::ProtocolError>{semio_os_kernel::os_dsl::variants_binary::decode_op(bytes)}
}
semio_os_kernel::original_ordinal_operation_receiving!(OriginalMutation);
#[test]
fn original_command_receiving_operation_original_record_and_owned_fields_resume_one_item(){
 let fixture:serde_json::Value=serde_json::from_str(include_str!("🧫️fixtures/🔣️.json")).unwrap();let expected=OriginalMutation::Rename{text:serde_json::from_value(fixture["text"].clone()).unwrap(),items:serde_json::from_value(fixture["items"].clone()).unwrap()};let encoded=expected.encode_op().unwrap();let mut source:Vec<u8>=serde_json::from_value(fixture["source"]["prefix"].clone()).unwrap();let start=source.len();source.extend_from_slice(&encoded);let end=source.len();source.extend(serde_json::from_value::<Vec<u8>>(fixture["source"]["suffix"].clone()).unwrap());let f=&fixture["turn"];let grant=RetainedCloneGrant{maximum_items:f["items"].as_u64().unwrap()as usize,maximum_copy_bytes:f["copyBytes"].as_u64().unwrap()as usize,maximum_capacity_bytes:f["capacityBytes"].as_u64().unwrap()as usize,maximum_release_bytes:f["releaseBytes"].as_u64().unwrap()as usize,maximum_depth:f["depth"].as_u64().unwrap()as usize};
 let dispatch=OriginalSourceTurnIssuer::new().issue().unwrap();let mut bank=OperationBank::fixture(grant,dispatch.identity());let mut callback=|_|true;let mut native=NativeDecodeControl::new(bank.left.maximum_capacity_bytes,&mut callback);
 native.with_source_witness(Some(dispatch.witness()),|native|{let turn=OriginalMutation::admit_receiving(&source,(start,end),bank.grant());let(mut cursor,admitted)=turn.result.unwrap().unwrap();bank.debit(admitted,turn.work);assert_eq!(admitted.copied_items,1);let wrong=source.clone();assert!(cursor.demands(&wrong,grant.maximum_copy_bytes).is_err());let mut turns=0;let mut original_text=None;
 while !cursor.is_ready(){assert!(turns<10000);if cursor.phase()==5&&original_text.is_none(){if let Some(FieldValue::Record(record))=cursor.generic_value(){if let Some(FieldValue::Text(text))=record.get(0){original_text=Some(text.as_ptr()as usize);}}}let progress=bank.normal(native,cursor.step_work_demand(&source,grant.maximum_copy_bytes).unwrap(),|native,g|cursor.step(&source,native,grant.maximum_copy_bytes,g)).result.unwrap();assert_eq!(progress.copied_items,1);assert!(progress.fits(grant));turns+=1;}
 let mut output=None;let turn=cursor.take_into(&mut output,bank.grant());let p=turn.result.unwrap();bank.debit(p,turn.work);assert_eq!(p.copied_items,1);assert_eq!(output.as_ref(),Some(&expected));let OriginalMutation::Rename{text,..}=output.as_ref().unwrap();assert_eq!(original_text,Some(text.as_ptr()as usize));
 while !cursor.terminal_is_empty(){let turn=bank.close(|effect|cursor.close_step(effect));if matches!(&turn.result,Ok(RetainedCloneStep::Complete(_))){assert!(cursor.terminal_is_empty())}turn.result.unwrap();}let mut output=ControlledRetirement::new(output.take().unwrap()).ok().unwrap();while !output.terminal_is_empty(){bank.close(|effect|output.step(effect.grant().maximum_copy_bytes,effect)).result.unwrap();}
 });
}

#[derive(Clone, Debug, PartialEq, semio_framework_dsl_record_derive::DslEnum, semio_framework_value::RetireOwned)]
enum CloseWorkMutation { Clear {} }

impl OpBinary for CloseWorkMutation {
 fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> { semio_os_kernel::os_dsl::variants_binary::encode_op(self) }
 fn decode_op(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> { semio_os_kernel::os_dsl::variants_binary::decode_op(bytes) }
}
semio_os_kernel::original_ordinal_operation_receiving!(CloseWorkMutation);

struct CloseWorkBinding { open: std::sync::Arc<std::sync::atomic::AtomicBool>, work: u64 }

static CLOSE_BINDING_OPEN: std::sync::OnceLock<std::sync::Arc<std::sync::atomic::AtomicBool>> = std::sync::OnceLock::new();
static CLOSE_BINDING_WORK: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(0);

impl DslOwnedReceivingVariants for CloseWorkMutation {
 type Cursor = CloseWorkBinding;
 fn receiving_cursor(keyword: &str) -> Result<Self::Cursor, ValueError> {
  assert_eq!(keyword, "clear");
  let open = CLOSE_BINDING_OPEN.get_or_init(|| std::sync::Arc::new(std::sync::atomic::AtomicBool::new(true))).clone();
  open.store(true, std::sync::atomic::Ordering::SeqCst);
  Ok(CloseWorkBinding { open, work: CLOSE_BINDING_WORK.load(std::sync::atomic::Ordering::SeqCst) })
 }
}

impl OwnedFieldReceiving<CloseWorkMutation> for CloseWorkBinding {
 fn demands(&self, _: &Option<FieldValue>, _: usize) -> Result<RetirementDemand, ValueError> { Ok(RetirementDemand { depth: 1, ..Default::default() }) }
 fn select_advance(&self, _: &Option<FieldValue>, _: usize, _: u64) -> Result<OwnedAdvanceSelection, ValueError> { Ok(OwnedAdvanceSelection::Local { work: 1 }) }
 fn advance_local(&mut self, _: &mut Option<FieldValue>, _: usize, _: &mut RetainedAllocationEffect<'_>) -> semio_framework_dsl_record::receiving::owned::OwnedPoll { unreachable!() }
 fn is_ready(&self) -> bool { true }
 fn take_into(&mut self, out: &mut Option<CloseWorkMutation>, effect: &mut RetainedAllocationEffect<'_>) -> semio_framework_dsl_record::receiving::owned::OwnedTake {
  let result = effect.performed(1).and_then(|ok| if ok {
   *out = Some(CloseWorkMutation::Clear {});
   Ok(RetainedCloneProgress { copied_items: 1, ..Default::default() })
  } else {
   Err(ValueError::literal(ValueRefusalKind::WorkLimit, "unfunded"))
  }).map_err(Into::into);
  RetainedWorkPoll::Complete(RetainedWorkOutcome { result, work: effect.performed_work() })
 }
 fn close_demands(&self, _: usize) -> Result<RetirementDemand, ValueError> { Ok(RetirementDemand { depth: 1, ..Default::default() }) }
 fn close_work_demand(&self, _: usize) -> Result<u64, ValueError> { Ok(self.work) }
 fn close_step(&mut self, effect: &mut RetainedAllocationEffect<'_>) -> semio_framework_dsl_record::receiving::owned::OwnedClose {
  let result = effect.performed(self.work).and_then(|ok| if ok {
   self.open.store(false, std::sync::atomic::Ordering::SeqCst);
   Ok(RetainedCloneProgress { copied_items: 1, ..Default::default() })
  } else {
   Err(ValueError::literal(ValueRefusalKind::WorkLimit, "unfunded"))
  }).map_err(Into::into);
  RetainedWorkPoll::Complete(RetainedWorkOutcome { result, work: effect.performed_work() })
 }
 fn terminal_is_empty(&self) -> bool { !self.open.load(std::sync::atomic::Ordering::SeqCst) }
}

#[test]
fn original_operation_close_quotes_and_funds_the_selected_binding_after_take() {
 let fixture: serde_json::Value = serde_json::from_str(include_str!("🧫️fixtures/🔣️.json")).unwrap();
 let child_work = fixture["closeBindingWork"].as_u64().unwrap();
 CLOSE_BINDING_WORK.store(child_work, std::sync::atomic::Ordering::SeqCst);
 let open = CLOSE_BINDING_OPEN.get_or_init(|| std::sync::Arc::new(std::sync::atomic::AtomicBool::new(true))).clone();
 let step_grant = RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: 4096, maximum_capacity_bytes: 4096, maximum_release_bytes: 4096, maximum_depth: 16 };
 let expected = CloseWorkMutation::Clear {};
 let source = expected.encode_op().unwrap();
 let dispatch = OriginalSourceTurnIssuer::new().issue().unwrap();
 let mut bank = OperationBank::fixture(step_grant, dispatch.identity());
 let mut callback = |_| true;
 let mut native = NativeDecodeControl::new(bank.left.maximum_capacity_bytes, &mut callback);
 let mut cursor = native.with_source_witness(Some(dispatch.witness()), |native| {
  let turn = CloseWorkMutation::admit_receiving(&source, (0, source.len()), bank.grant());
  let (mut cursor, admitted) = turn.result.unwrap().unwrap();
  bank.debit(admitted, turn.work);
  while !cursor.is_ready() {
   let demand = cursor.step_work_demand(&source, step_grant.maximum_copy_bytes).unwrap();
   bank.normal(native, demand, |native, g| cursor.step(&source, native, step_grant.maximum_copy_bytes, g)).result.unwrap();
  }
  cursor
 });
 let grant = RetainedCloneGrant { maximum_items: 1, maximum_depth: 2, ..Default::default() };
 let mut output = None;
 cursor.take_into(&mut output, grant).result.unwrap();
 assert_eq!(output, Some(CloseWorkMutation::decode_op(&source).unwrap()));
 let run = |cursor: &mut OriginalOperationReceiving<CloseWorkMutation>, work: u64| {
  let mut wallet = AuthoringWallet::new(grant, u128::from(work + ORIGINAL_INVOCATION_WORK));
  wallet.run_settled(grant, work, |effect| cursor.close_step(effect)).unwrap().unwrap().0
 };
 let start = cursor.close_work_demand(0).unwrap();
 assert_eq!(start, 1);
 run(&mut cursor, start).result.unwrap();
 let quoted = cursor.close_work_demand(0).unwrap();
 assert_eq!(quoted, child_work);
 let denied = run(&mut cursor, quoted - 1);
 assert_eq!(denied.work, 0);
 assert_eq!(denied.result.unwrap().progress(), RetainedCloneProgress::default());
 assert!(open.load(std::sync::atomic::Ordering::SeqCst));
 let paid = run(&mut cursor, quoted);
 assert_eq!(paid.work, u128::from(quoted));
 paid.result.unwrap();
 assert!(!open.load(std::sync::atomic::Ordering::SeqCst));
 for _ in 0..3 {
  if cursor.terminal_is_empty() { break; }
  let work = cursor.close_work_demand(0).unwrap();
  assert_eq!(work, 1);
  run(&mut cursor, work).result.unwrap();
 }
 assert!(cursor.terminal_is_empty());
 assert_eq!(cursor.close_work_demand(0).unwrap(), 0);
}

#[derive(Clone,Debug,PartialEq,semio_framework_dsl_record_derive::DslEnum,semio_framework_dsl_record_derive::DslReceiving,semio_framework_value::RetireOwned)]
enum OriginalTaggedMutation{Remove{},Rename{text:String}}
const ORIGINAL_PROTOCOL:&str=include_str!("🧫️fixtures/🏷️protocol.semio");
impl OpBinary for OriginalTaggedMutation{fn encode_op(&self)->Result<Vec<u8>,protocol::ProtocolError>{semio_os_kernel::os_dsl::variants_binary::encode_tagged_op(ORIGINAL_PROTOCOL,self)}fn decode_op(bytes:&[u8])->Result<Self,protocol::ProtocolError>{semio_os_kernel::os_dsl::variants_binary::decode_tagged_op(ORIGINAL_PROTOCOL,bytes)}}
semio_os_kernel::original_tagged_operation_receiving!(OriginalTaggedMutation,ORIGINAL_PROTOCOL);
#[test]
fn original_command_receiving_operation_authored_tagged_header_resumes_each_original_item(){
 let fixture:serde_json::Value=serde_json::from_str(include_str!("🧫️fixtures/🏷️tagged.json")).unwrap();let expected=OriginalTaggedMutation::Rename{text:fixture["text"].as_str().unwrap().into()};let source=expected.encode_op().unwrap();let tag=fixture["tag"].as_u64().unwrap();assert_eq!(&source[..3],&[1,(tag as u8&127)|128,(tag>>7)as u8]);let f=&fixture["turn"];let grant=RetainedCloneGrant{maximum_items:f["items"].as_u64().unwrap()as usize,maximum_copy_bytes:f["copyBytes"].as_u64().unwrap()as usize,maximum_capacity_bytes:f["capacityBytes"].as_u64().unwrap()as usize,maximum_release_bytes:f["releaseBytes"].as_u64().unwrap()as usize,maximum_depth:f["depth"].as_u64().unwrap()as usize};let dispatch=OriginalSourceTurnIssuer::new().issue().unwrap();let mut bank=OperationBank::fixture(grant,dispatch.identity());let mut callback=|_|true;let mut native=NativeDecodeControl::new(bank.left.maximum_capacity_bytes,&mut callback);
 native.with_source_witness(Some(dispatch.witness()),|native|{let turn=OriginalTaggedMutation::admit_receiving(&source,(0,source.len()),bank.grant());let(mut cursor,p)=turn.result.unwrap().unwrap();bank.debit(p,turn.work);let mut turns=0;let mut selections=0;while !cursor.is_ready(){assert!(turns<10000);if cursor.phase()==2{selections+=1;}let mut denied=grant;denied.maximum_items=0;let turn=bank.native(native,denied,0,|native,g|cursor.step(&source,native,grant.maximum_copy_bytes,g));assert_eq!(turn.work,0);assert_eq!(turn.result.unwrap(),Default::default());let progress=bank.normal(native,cursor.step_work_demand(&source,grant.maximum_copy_bytes).unwrap(),|native,g|cursor.step(&source,native,grant.maximum_copy_bytes,g)).result.unwrap();assert_eq!(progress.copied_items,1);assert!(progress.fits(grant));turns+=1;}assert_eq!(selections,2);let mut output=None;let turn=cursor.take_into(&mut output,bank.grant());bank.debit(turn.result.unwrap(),turn.work);assert_eq!(output.as_ref(),Some(&expected));while !cursor.terminal_is_empty(){let turn=bank.close(|effect|cursor.close_step(effect));if matches!(&turn.result,Ok(RetainedCloneStep::Complete(_))){assert!(cursor.terminal_is_empty())}turn.result.unwrap();}let mut output=ControlledRetirement::new(output.take().unwrap()).ok().unwrap();while !output.terminal_is_empty(){bank.close(|effect|output.step(effect.grant().maximum_copy_bytes,effect)).result.unwrap();}
 });
}


#[test]fn original_operation_failure_preserves_real_probe_cpu_independently_of_physical_progress(){
 let f:serde_json::Value=serde_json::from_str(include_str!("../🎟️work/📐️.json")).unwrap();let s=&f["stock"];let left=RetainedCloneGrant{maximum_items:s["items"].as_u64().unwrap()as usize,maximum_copy_bytes:s["copyBytes"].as_u64().unwrap()as usize,maximum_capacity_bytes:s["capacityBytes"].as_u64().unwrap()as usize,maximum_release_bytes:s["releaseBytes"].as_u64().unwrap()as usize,maximum_depth:s["depth"].as_u64().unwrap()as usize};let dispatch=OriginalSourceTurnIssuer::new().issue().unwrap();let mut bank=OperationBank::new(RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:4096,..left},left,s["fuel"].as_u64().unwrap(),dispatch.identity());let source:Vec<u8>=serde_json::from_value(f["source"].clone()).unwrap();assert!(OriginalMutation::decode_op(&source).is_err());let g=bank.grant();let(turn,h)=semio_framework_trace::observe_heap_allocations_on_this_thread(||OriginalMutation::admit_receiving(&source,(0,source.len()),g));assert_eq!((h.requested_bytes,h.released_bytes),(0,0));assert_eq!(turn.work,u128::from(f["admissionWork"].as_u64().unwrap()));let(mut cursor,p)=turn.result.unwrap().unwrap();assert_eq!(p.copied_bytes,f["admissionCopyBytes"].as_u64().unwrap()as usize);bank.debit(p,turn.work);let mut callback=|_|true;let mut native=NativeDecodeControl::new(bank.left.maximum_capacity_bytes,&mut callback);
 native.with_source_witness(Some(dispatch.witness()),|native|{
 let g=bank.grant();let denied=RetainedCloneGrant{maximum_items:0,..g};let turn=bank.native(native,denied,0,|native,g|cursor.step(&source,native,4096,g));assert_eq!(turn.work,0);assert_eq!(turn.result.unwrap(),Default::default());let turn=bank.normal(native,cursor.step_work_demand(&source,4096).unwrap(),|native,g|cursor.step(&source,native,4096,g));assert_eq!(turn.work,u128::from(f["failureWork"].as_u64().unwrap()));let fault=turn.result.unwrap_err();assert_eq!(fault.primary.as_ref().unwrap().kind,ValueRefusalKind::InvalidValue);assert!(fault.quote_refusal.is_none()&&fault.allocation_refusal.is_none());assert_eq!(fault.retained_progress().copied_items,f["failureItems"].as_u64().unwrap()as usize);
 let mut fault=ControlledRetirement::new(fault).unwrap_or_else(|_|panic!("actual native failure has original retirement"));for _ in 0..10000{if fault.terminal_is_empty(){break}bank.close(|effect|fault.step(effect.grant().maximum_copy_bytes,effect)).result.unwrap();}assert!(fault.terminal_is_empty());for _ in 0..10000{if cursor.terminal_is_empty(){break}let turn=bank.close(|effect|cursor.close_step(effect));if matches!(&turn.result,Ok(RetainedCloneStep::Complete(_))){assert!(cursor.terminal_is_empty())}turn.result.unwrap();}assert!(cursor.terminal_is_empty());assert_eq!(bank.close(|effect|cursor.close_step(effect)).work,0);
 });
}

#[derive(Clone,Copy,Debug,PartialEq,semio_framework_dsl_record_derive::DslScalar,semio_framework_dsl_record_derive::DslReceiving,semio_framework_value::RetireOwned)]
enum ReceivedMode{Draft,Sealed,Retired}
#[derive(Clone,Debug,PartialEq,semio_framework_dsl_record_derive::DslEnum,semio_framework_dsl_record_derive::DslReceiving,semio_framework_value::RetireOwned)]
enum ScalarFieldMutation{Mark{mode:ReceivedMode,note:String},Clear{}}
impl OpBinary for ScalarFieldMutation{
 fn encode_op(&self)->Result<Vec<u8>,protocol::ProtocolError>{semio_os_kernel::os_dsl::variants_binary::encode_op(self)}
 fn decode_op(bytes:&[u8])->Result<Self,protocol::ProtocolError>{semio_os_kernel::os_dsl::variants_binary::decode_op(bytes)}
}
semio_os_kernel::original_ordinal_operation_receiving!(ScalarFieldMutation);
#[test]
fn every_unit_enum_field_value_is_received_like_its_decode_oracle(){
 let values=[ReceivedMode::Draft,ReceivedMode::Sealed,ReceivedMode::Retired].map(|mode|ScalarFieldMutation::Mark{mode,note:"n".into()});
 check_receiving_round_trip(&values).unwrap();
 check_receiving_round_trip(&[ScalarFieldMutation::Clear{}]).unwrap();
}

#[derive(Clone,Debug,PartialEq,semio_framework_dsl_record_derive::DslEnum,semio_framework_dsl_record_derive::DslReceiving,semio_framework_value::RetireOwned)]
enum StatementBody{Text{text:String},Number{value:i64}}
#[derive(Clone,Debug,PartialEq,semio_framework_dsl_record_derive::DslEnum,semio_framework_dsl_record_derive::DslReceiving,semio_framework_value::RetireOwned)]
enum StatementFieldMutation{Put{id:String,#[dsl(statements)] body:Box<StatementBody>}}
impl OpBinary for StatementFieldMutation{
 fn encode_op(&self)->Result<Vec<u8>,protocol::ProtocolError>{semio_os_kernel::os_dsl::variants_binary::encode_op(self)}
 fn decode_op(bytes:&[u8])->Result<Self,protocol::ProtocolError>{semio_os_kernel::os_dsl::variants_binary::decode_op(bytes)}
}
semio_os_kernel::original_ordinal_operation_receiving!(StatementFieldMutation);
#[test]
fn every_statement_enum_field_variant_is_received_like_its_decode_oracle(){
 let values=[StatementBody::Text{text:"t".into()},StatementBody::Number{value:-7}].map(|body|StatementFieldMutation::Put{id:"i".into(),body:Box::new(body)});
 check_receiving_round_trip(&values).unwrap();
}
#[test]
fn receive_to_completion_refuses_when_fuel_runs_out_mid_operation_without_dropping_a_live_cursor(){
 let value=OriginalMutation::Rename{text:"abc".into(),items:vec!["x".into(),"y".into()]};let bytes=value.encode_op().unwrap();
 let grant=RetainedCloneGrant{maximum_items:1<<10,maximum_copy_bytes:1<<16,maximum_capacity_bytes:1<<16,maximum_release_bytes:1<<16,maximum_depth:64};
 let mut refused=0;
 for fuel in 1..96{match receive_to_completion::<OriginalMutation>(&bytes,(0,bytes.len()),ReceivingStock{stock:grant,quantum:RetainedCloneGrant{maximum_items:1,..grant},fuel}){Ok(received)=>assert_eq!(received,value),Err(_)=>refused+=1}}
 assert!(refused>0,"no fuel level exhausted mid-operation");
}

#[derive(Clone,Debug,PartialEq,semio_framework_value::ToValue,semio_framework_value::FromValue,semio_framework_value::RetireOwned)]
#[value(tag="operation",content="payload",rename_all="camelCase",deny_unknown_fields)]
enum PackValueMutation{Rename{text:String,items:Vec<String>},Clear}
impl OpBinary for PackValueMutation{
 fn encode_op(&self)->Result<Vec<u8>,protocol::ProtocolError>{Ok(semio_os_pack::pack_rt::encode_pack_value(&semio_framework_value::ToValue::to_value(self)))}
 fn decode_op(bytes:&[u8])->Result<Self,protocol::ProtocolError>{let value=semio_os_pack::pack_rt::decode_pack_value(bytes).map_err(|error|protocol::ProtocolError::Malformed{what:"pack value op",offset:0,detail:error.to_string()})?;semio_framework_value::FromValue::from_value(semio_os_pack::pack_rt::renormalize_whole_number_floats(value)).map_err(|error|protocol::ProtocolError::Malformed{what:"pack value op",offset:0,detail:error.to_string()})}
}
semio_os_kernel::original_pack_value_operation_receiving!(PackValueMutation);

/// 🧪️ Third-party oracle: the bounded pack-value cursor decodes exactly the value the whole-buffer `pack_rt` path decodes, for every sample, and every turn fits its granted stock.
#[test]
fn original_pack_value_receiving_decodes_the_same_value_as_the_pack_rt_oracle(){
 let fixture:serde_json::Value=serde_json::from_str(include_str!("🧫️fixtures/🔣️.json")).unwrap();
 let samples=[PackValueMutation::Clear,PackValueMutation::Rename{text:"a".into(),items:Vec::new()},PackValueMutation::Rename{text:fixture["text"].as_str().unwrap().into(),items:serde_json::from_value(fixture["items"].clone()).unwrap()}];
 for expected in samples{
  let encoded=expected.encode_op().unwrap();
  let oracle=PackValueMutation::decode_op(&encoded).unwrap();assert_eq!(oracle,expected);
  let mut source:Vec<u8>=serde_json::from_value(fixture["source"]["prefix"].clone()).unwrap();let start=source.len();source.extend_from_slice(&encoded);let end=source.len();source.extend(serde_json::from_value::<Vec<u8>>(fixture["source"]["suffix"].clone()).unwrap());
  let f=&fixture["turn"];let grant=RetainedCloneGrant{maximum_items:f["items"].as_u64().unwrap()as usize,maximum_copy_bytes:f["copyBytes"].as_u64().unwrap()as usize,maximum_capacity_bytes:f["capacityBytes"].as_u64().unwrap()as usize,maximum_release_bytes:f["releaseBytes"].as_u64().unwrap()as usize,maximum_depth:f["depth"].as_u64().unwrap()as usize};
  let dispatch=OriginalSourceTurnIssuer::new().issue().unwrap();let mut bank=OperationBank::fixture(grant,dispatch.identity());let mut callback=|_|true;let mut native=NativeDecodeControl::new(bank.left.maximum_capacity_bytes,&mut callback);
 native.with_source_witness(Some(dispatch.witness()),|native|{
  let turn=PackValueMutation::admit_receiving(&source,(start,end),bank.grant());let(mut cursor,admitted)=turn.result.unwrap().unwrap();bank.debit(admitted,turn.work);
  let wrong=source.clone();assert!(cursor.demands(&wrong,grant.maximum_copy_bytes).is_err());
  let mut turns=0;
  while !cursor.is_ready(){assert!(turns<10000);let progress=bank.normal(native,cursor.step_work_demand(&source,grant.maximum_copy_bytes).unwrap(),|native,g|cursor.step(&source,native,grant.maximum_copy_bytes,g)).result.unwrap();assert!(progress.fits(grant));turns+=1;}
  let mut output=None;let turn=cursor.take_into(&mut output,bank.grant());let p=turn.result.unwrap();bank.debit(p,turn.work);assert_eq!(output.as_ref(),Some(&oracle));
  while !cursor.terminal_is_empty(){let turn=bank.close(|effect|cursor.close_step(effect));turn.result.unwrap();}
  let mut output=ControlledRetirement::new(output.take().unwrap()).ok().unwrap();while !output.terminal_is_empty(){bank.close(|effect|output.step(effect.grant().maximum_copy_bytes,effect)).result.unwrap();}
 });
 }
}

#[test]
fn receive_to_completion_names_the_unpayable_quote_and_the_quantum_when_no_turn_progresses(){
 let bytes=PackValueMutation::Clear.encode_op().unwrap();
 let stock=RetainedCloneGrant{maximum_items:1<<10,maximum_copy_bytes:1<<16,maximum_capacity_bytes:1<<16,maximum_release_bytes:1<<16,maximum_depth:64};
 let quantum=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:0,..stock};
 let message=receive_to_completion::<PackValueMutation>(&bytes,(0,bytes.len()),ReceivingStock{stock,quantum,fuel:1<<10}).unwrap_err().message().to_string();
 assert!(message.contains("made no progress")&&message.contains("quote Ok(RetirementDemand")&&message.contains("maximum_copy_bytes: 0"),"{message}");
}
