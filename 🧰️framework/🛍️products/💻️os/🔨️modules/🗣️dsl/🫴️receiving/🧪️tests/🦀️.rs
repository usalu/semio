//! 🧫️ Actual generic operation cursors move original typed fields through one-item native loans.
use super::*;
use semio_framework_replication as protocol;
use protocol::{OpBinary,OpBinaryReceiving,OpBinaryReceivingCursor};
use semio_framework_value::{AuthoringWallet, OriginalRetirementPoll, RetainedAllocationEffect, ORIGINAL_INVOCATION_WORK, retirement::controlled::ControlledRetirement};
use semio_framework_value_native_owned::{OriginalSourceTurnIssuer, native_decoding::{
    OriginalConstructionAccept, OriginalConstructionAcceptComplete, OriginalConstructionAuthority,
    OriginalConstructionGrant, OriginalConstructionGrantComplete, OriginalConstructionRecipient,
    OriginalSourceTurnIdentity, OriginalWorkAuthority, OriginalWorkPartialCheck,
    OriginalWorkPartialCheckComplete, OriginalWorkQuote, OriginalWorkQuoteComplete, OriginalWorkRecipient,
}};
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
 while !cursor.is_ready(){assert!(turns<10000);if cursor.phase()==5&&original_text.is_none(){if let Some(FieldValue::Record(record))=cursor.generic_value(){if let Some(FieldValue::Text(text))=record.get(0){original_text=Some(text.as_ptr()as usize);}}}let progress=bank.normal(native,|native,g|cursor.step(&source,native,grant.maximum_copy_bytes,g)).result.unwrap();assert_eq!(progress.copied_items,1);assert!(progress.fits(grant));turns+=1;}
 let mut output=None;let turn=cursor.take_into(&mut output,bank.grant());let p=turn.result.unwrap();bank.debit(p,turn.work);assert_eq!(p.copied_items,1);assert_eq!(output.as_ref(),Some(&expected));let OriginalMutation::Rename{text,..}=output.as_ref().unwrap();assert_eq!(original_text,Some(text.as_ptr()as usize));
 while !cursor.terminal_is_empty(){let turn=bank.close(|effect|cursor.close_step(effect));if matches!(&turn.result,Ok(RetainedCloneStep::Complete(_))){assert!(cursor.terminal_is_empty())}turn.result.unwrap();}let mut output=ControlledRetirement::new(output.take().unwrap()).ok().unwrap();while !output.terminal_is_empty(){bank.close(|effect|output.step(effect.grant().maximum_copy_bytes,effect)).result.unwrap();}
 });
}

#[derive(Clone,Debug,PartialEq,semio_framework_dsl_record_derive::DslEnum,semio_framework_dsl_record_derive::DslReceiving,semio_framework_value::RetireOwned)]
enum OriginalTaggedMutation{Remove{},Rename{text:String}}
const ORIGINAL_PROTOCOL:&str=include_str!("🧫️fixtures/🏷️protocol.semio");
impl OpBinary for OriginalTaggedMutation{fn encode_op(&self)->Result<Vec<u8>,protocol::ProtocolError>{semio_os_kernel::os_dsl::variants_binary::encode_tagged_op(ORIGINAL_PROTOCOL,self)}fn decode_op(bytes:&[u8])->Result<Self,protocol::ProtocolError>{semio_os_kernel::os_dsl::variants_binary::decode_tagged_op(ORIGINAL_PROTOCOL,bytes)}}
semio_os_kernel::original_tagged_operation_receiving!(OriginalTaggedMutation,ORIGINAL_PROTOCOL);
#[test]
fn original_command_receiving_operation_authored_tagged_header_resumes_each_original_item(){
 let fixture:serde_json::Value=serde_json::from_str(include_str!("🧫️fixtures/🏷️tagged.json")).unwrap();let expected=OriginalTaggedMutation::Rename{text:fixture["text"].as_str().unwrap().into()};let source=expected.encode_op().unwrap();let tag=fixture["tag"].as_u64().unwrap();assert_eq!(&source[..3],&[1,(tag as u8&127)|128,(tag>>7)as u8]);let f=&fixture["turn"];let grant=RetainedCloneGrant{maximum_items:f["items"].as_u64().unwrap()as usize,maximum_copy_bytes:f["copyBytes"].as_u64().unwrap()as usize,maximum_capacity_bytes:f["capacityBytes"].as_u64().unwrap()as usize,maximum_release_bytes:f["releaseBytes"].as_u64().unwrap()as usize,maximum_depth:f["depth"].as_u64().unwrap()as usize};let dispatch=OriginalSourceTurnIssuer::new().issue().unwrap();let mut bank=OperationBank::fixture(grant,dispatch.identity());let mut callback=|_|true;let mut native=NativeDecodeControl::new(bank.left.maximum_capacity_bytes,&mut callback);
 native.with_source_witness(Some(dispatch.witness()),|native|{let turn=OriginalTaggedMutation::admit_receiving(&source,(0,source.len()),bank.grant());let(mut cursor,p)=turn.result.unwrap().unwrap();bank.debit(p,turn.work);let mut turns=0;let mut selections=0;while !cursor.is_ready(){assert!(turns<10000);if cursor.phase()==2{selections+=1;}let mut denied=grant;denied.maximum_items=0;let turn=bank.native(native,denied,0,|native,g|cursor.step(&source,native,grant.maximum_copy_bytes,g));assert_eq!(turn.work,0);assert_eq!(turn.result.unwrap(),Default::default());let progress=bank.normal(native,|native,g|cursor.step(&source,native,grant.maximum_copy_bytes,g)).result.unwrap();assert_eq!(progress.copied_items,1);assert!(progress.fits(grant));turns+=1;}assert_eq!(selections,2);let mut output=None;let turn=cursor.take_into(&mut output,bank.grant());bank.debit(turn.result.unwrap(),turn.work);assert_eq!(output.as_ref(),Some(&expected));while !cursor.terminal_is_empty(){let turn=bank.close(|effect|cursor.close_step(effect));if matches!(&turn.result,Ok(RetainedCloneStep::Complete(_))){assert!(cursor.terminal_is_empty())}turn.result.unwrap();}let mut output=ControlledRetirement::new(output.take().unwrap()).ok().unwrap();while !output.terminal_is_empty(){bank.close(|effect|output.step(effect.grant().maximum_copy_bytes,effect)).result.unwrap();}
 });
}

struct OperationBank {
    left: RetainedCloneGrant,
    fuel: u64,
    quantum: RetainedCloneGrant,
    source: OriginalSourceTurnIdentity,
    work: OriginalWorkAuthority<OperationWorkDriver>,
    physical: OriginalConstructionAuthority<OperationPhysicalDriver>,
}

struct OperationWorkDriver;
impl OriginalWorkRecipient for OperationWorkDriver {
    type Context<'turn, 'source> = (u64, OriginalSourceTurnIdentity) where 'source: 'turn;
    fn validate_context<'turn, 'source>(&self, _: &Self::Context<'turn, 'source>) -> Result<(), ValueError> where 'source: 'turn { Ok(()) }
    fn source_turn<'turn, 'source>(&self, context: &Self::Context<'turn, 'source>) -> Option<OriginalSourceTurnIdentity> where 'source: 'turn { Some(context.1) }
    fn admits<'reply, 'turn, 'source>(&mut self, context: &mut Self::Context<'turn, 'source>, quote: OriginalWorkQuote<'reply>, observed: u64, additional: u64) -> OriginalWorkQuoteComplete<'reply> where 'source: 'turn {
        quote.complete(Ok(observed.checked_add(additional).is_some_and(|work| work <= context.0)))
    }
    fn settle_partial<'reply, 'turn, 'source>(&mut self, _: &mut Self::Context<'turn, 'source>, check: OriginalWorkPartialCheck<'reply>) -> OriginalWorkPartialCheckComplete<'reply> where 'source: 'turn { check.complete(Ok(())) }
    fn recover_partial<'reply, 'turn, 'source>(&mut self, _: &mut Self::Context<'turn, 'source>, check: OriginalWorkPartialCheck<'reply>) -> OriginalWorkPartialCheckComplete<'reply> where 'source: 'turn { check.pending() }
}
struct OperationPhysicalLoan {
    submitted: RetainedCloneProgress,
    left: RetainedCloneGrant,
    source: OriginalSourceTurnIdentity,
}
struct OperationPhysicalDriver;
impl OriginalConstructionRecipient for OperationPhysicalDriver {
    type Context<'turn, 'source> = OperationPhysicalLoan where 'source: 'turn;
    fn validate_context<'turn, 'source>(&self, _: &OperationPhysicalLoan) -> Result<(), ValueError> where 'source: 'turn { Ok(()) }
    fn source_turn<'turn, 'source>(&self, loan: &OperationPhysicalLoan) -> Option<OriginalSourceTurnIdentity> where 'source: 'turn { Some(loan.source) }
    fn grant<'reply, 'turn, 'source>(&mut self, loan: &mut OperationPhysicalLoan, quote: OriginalConstructionGrant<'reply>) -> OriginalConstructionGrantComplete<'reply> where 'source: 'turn { quote.complete(Ok(loan.left)) }
    fn accept<'reply, 'turn, 'source>(&mut self, loan: &mut OperationPhysicalLoan, receipt: OriginalConstructionAccept<'reply>) -> OriginalConstructionAcceptComplete<'reply> where 'source: 'turn {
        let progress = receipt.progress();
        assert!(progress.fits(receipt.grant()));
        debit_grant(&mut loan.left, progress);
        receipt.complete(loan.submitted.checked_add(progress).map(|total| loan.submitted = total))
    }
    fn submitted<'turn, 'source>(&self, loan: &OperationPhysicalLoan) -> RetainedCloneProgress where 'source: 'turn { loan.submitted }
}

fn debit_grant(left: &mut RetainedCloneGrant, progress: RetainedCloneProgress) {
    assert!(progress.fits(*left));
    left.maximum_items -= progress.copied_items;
    left.maximum_copy_bytes -= progress.copied_bytes;
    left.maximum_capacity_bytes -= progress.retained_capacity_bytes;
    left.maximum_release_bytes -= progress.released_bytes;
}

impl OperationBank {
    fn new(quantum: RetainedCloneGrant, left: RetainedCloneGrant, fuel: u64, source: OriginalSourceTurnIdentity) -> Self {
        Self {
            left, fuel, quantum, source,
            work: OriginalWorkAuthority::new(OperationWorkDriver),
            physical: OriginalConstructionAuthority::new(OperationPhysicalDriver),
        }
    }

    fn fixture(quantum: RetainedCloneGrant, source: OriginalSourceTurnIdentity) -> Self {
        let fixture: serde_json::Value = serde_json::from_str(include_str!("../🎟️work/📐️.json")).unwrap();
        let stock = &fixture["originalStock"];
        Self::new(quantum, RetainedCloneGrant {
            maximum_items: stock["items"].as_u64().unwrap() as usize,
            maximum_copy_bytes: stock["copyBytes"].as_u64().unwrap() as usize,
            maximum_capacity_bytes: stock["capacityBytes"].as_u64().unwrap() as usize,
            maximum_release_bytes: stock["releaseBytes"].as_u64().unwrap() as usize,
            maximum_depth: stock["depth"].as_u64().unwrap() as usize,
        }, stock["fuel"].as_u64().unwrap(), source)
    }

    fn grant(&self) -> RetainedCloneGrant {
        let grant = self.quantum.restricted_by(self.left);
        RetainedCloneGrant { maximum_items: if self.fuel == 0 { 0 } else { grant.maximum_items }, ..grant }
    }

    fn debit(&mut self, progress: RetainedCloneProgress, work: u128) {
        debit_grant(&mut self.left, progress);
        self.fuel = self.fuel.checked_sub(u64::try_from(work).unwrap()).unwrap();
    }

    fn native(
        &mut self, native: &mut NativeDecodeControl<'_>, grant: RetainedCloneGrant, maximum_work: u64,
        operation: impl FnOnce(&mut NativeDecodeControl<'_>, RetainedCloneGrant) -> RetainedWorkOutcome<RetainedCloneProgress, OriginalReceivingPrimary>,
    ) -> RetainedWorkOutcome<RetainedCloneProgress, OriginalReceivingPrimary> {
        let mut work = (self.fuel.min(maximum_work), self.source);
        let mut physical = OperationPhysicalLoan { submitted: Default::default(), left: grant, source: self.source };
        let before = native.actual_work();
        let turn = self.work.with_loan(&mut work, |work| native.with_work_recipient(work, |native| {
            self.physical.with_loan(&mut physical, |physical| {
                native.with_construction_recipient(physical, |native| Ok::<_, ValueError>(operation(native, grant)))
            })?
        })).unwrap().unwrap();
        let progress = turn.result.as_ref().copied().unwrap_or_else(|error| error.retained_progress());
        self.debit(progress, native.actual_work().checked_sub(before).unwrap());
        turn
    }

    fn normal(
        &mut self, native: &mut NativeDecodeControl<'_>,
        operation: impl FnOnce(&mut NativeDecodeControl<'_>, RetainedCloneGrant) -> RetainedWorkOutcome<RetainedCloneProgress, OriginalReceivingPrimary>,
    ) -> RetainedWorkOutcome<RetainedCloneProgress, OriginalReceivingPrimary> {
        self.native(native, self.grant(), 1 + ORIGINAL_INVOCATION_WORK, operation)
    }

    fn close(&mut self, operation: impl FnOnce(&mut RetainedAllocationEffect<'_>) -> OriginalRetirementPoll) -> RetainedWorkOutcome<RetainedCloneStep, semio_framework_value::OriginalRetirementFailure> {
        let mut wallet = AuthoringWallet::new(self.left, u128::from(self.fuel));
        let (turn, _) = wallet.run_settled(self.grant(), self.fuel, operation).unwrap().unwrap();
        let progress = turn.result.as_ref().map_or_else(|error| error.retained_progress(), |step| step.progress());
        self.debit(progress, u128::from(self.fuel) - wallet.remaining_work());
        turn
    }
}
#[test]fn original_operation_failure_preserves_real_probe_cpu_independently_of_physical_progress(){
 let f:serde_json::Value=serde_json::from_str(include_str!("../🎟️work/📐️.json")).unwrap();let s=&f["stock"];let left=RetainedCloneGrant{maximum_items:s["items"].as_u64().unwrap()as usize,maximum_copy_bytes:s["copyBytes"].as_u64().unwrap()as usize,maximum_capacity_bytes:s["capacityBytes"].as_u64().unwrap()as usize,maximum_release_bytes:s["releaseBytes"].as_u64().unwrap()as usize,maximum_depth:s["depth"].as_u64().unwrap()as usize};let dispatch=OriginalSourceTurnIssuer::new().issue().unwrap();let mut bank=OperationBank::new(RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:4096,..left},left,s["fuel"].as_u64().unwrap(),dispatch.identity());let source:Vec<u8>=serde_json::from_value(f["source"].clone()).unwrap();assert!(OriginalMutation::decode_op(&source).is_err());let g=bank.grant();let(turn,h)=semio_framework_trace::observe_heap_allocations_on_this_thread(||OriginalMutation::admit_receiving(&source,(0,source.len()),g));assert_eq!((h.requested_bytes,h.released_bytes),(0,0));assert_eq!(turn.work,u128::from(f["admissionWork"].as_u64().unwrap()));let(mut cursor,p)=turn.result.unwrap().unwrap();assert_eq!(p.copied_bytes,f["admissionCopyBytes"].as_u64().unwrap()as usize);bank.debit(p,turn.work);let mut callback=|_|true;let mut native=NativeDecodeControl::new(bank.left.maximum_capacity_bytes,&mut callback);
 native.with_source_witness(Some(dispatch.witness()),|native|{
 let g=bank.grant();let denied=RetainedCloneGrant{maximum_items:0,..g};let turn=bank.native(native,denied,0,|native,g|cursor.step(&source,native,4096,g));assert_eq!(turn.work,0);assert_eq!(turn.result.unwrap(),Default::default());let turn=bank.normal(native,|native,g|cursor.step(&source,native,4096,g));assert_eq!(turn.work,u128::from(f["failureWork"].as_u64().unwrap()));let fault=turn.result.unwrap_err();assert_eq!(fault.primary.as_ref().unwrap().kind,ValueRefusalKind::InvalidValue);assert!(fault.quote_refusal.is_none()&&fault.allocation_refusal.is_none());assert_eq!(fault.retained_progress().copied_items,f["failureItems"].as_u64().unwrap()as usize);
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
  while !cursor.is_ready(){assert!(turns<10000);let progress=bank.normal(native,|native,g|cursor.step(&source,native,grant.maximum_copy_bytes,g)).result.unwrap();assert!(progress.fits(grant));turns+=1;}
  let mut output=None;let turn=cursor.take_into(&mut output,bank.grant());let p=turn.result.unwrap();bank.debit(p,turn.work);assert_eq!(output.as_ref(),Some(&oracle));
  while !cursor.terminal_is_empty(){let turn=bank.close(|effect|cursor.close_step(effect));turn.result.unwrap();}
  let mut output=ControlledRetirement::new(output.take().unwrap()).ok().unwrap();while !output.terminal_is_empty(){bank.close(|effect|output.step(effect.grant().maximum_copy_bytes,effect)).result.unwrap();}
 });
 }
}
