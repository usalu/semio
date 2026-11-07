//! 🧫️ Actual generic operation cursors move original typed fields through one-item native loans.
use super::*;
use protocol::{OpBinary,OpBinaryReceiving,OpBinaryReceivingCursor};
use semio_framework_value::retirement::controlled::ControlledRetirement;
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
 let mut bank=OperationBank::fixture(grant);let mut callback=|_|true;let mut native=NativeDecodeControl::new(bank.left.maximum_capacity_bytes,&mut callback);let turn=OriginalMutation::admit_receiving(&source,(start,end),bank.grant());let(mut cursor,admitted)=turn.result.unwrap().unwrap();bank.debit(admitted,turn.work);assert_eq!(admitted.copied_items,1);let wrong=source.clone();assert!(cursor.demands(&wrong,grant.maximum_copy_bytes).is_err());let mut turns=0;let mut original_text=None;
 while !cursor.is_ready(){assert!(turns<10000);if cursor.phase()==5&&original_text.is_none(){if let Some(FieldValue::Record(record))=cursor.generic_value(){if let Some(FieldValue::Text(text))=record.get(0){original_text=Some(text.as_ptr()as usize);}}}let progress=bank.normal(&mut native,|native,g|cursor.step(&source,native,grant.maximum_copy_bytes,g)).result.unwrap();assert_eq!(progress.copied_items,1);assert!(progress.fits(grant));turns+=1;}
 let mut output=None;let turn=cursor.take_into(&mut output,bank.grant());let p=turn.result.unwrap();bank.debit(p,turn.work);assert_eq!(p.copied_items,1);assert_eq!(output.as_ref(),Some(&expected));let OriginalMutation::Rename{text,..}=output.as_ref().unwrap();assert_eq!(original_text,Some(text.as_ptr()as usize));
 while !cursor.terminal_is_empty(){let turn=cursor.close_step(bank.grant());if matches!(&turn.result,Ok(RetainedCloneStep::Complete(_))){assert!(cursor.terminal_is_empty())}let p=turn.result.as_ref().map(|s|s.progress()).unwrap_or_else(|e|e.retained_progress());bank.debit(p,turn.work);turn.result.unwrap();}let mut output=ControlledRetirement::new(output.take().unwrap()).ok().unwrap();while !output.terminal_is_empty(){let result=output.step(bank.grant());let work=output.last_turn_work();let p=result.as_ref().map(|s|s.progress()).unwrap_or_else(|e|e.retained_progress());bank.debit(p,work);result.unwrap();}
}

#[derive(Clone,Debug,PartialEq,semio_framework_dsl_record_derive::DslEnum,semio_framework_dsl_record_derive::DslReceiving,semio_framework_value::RetireOwned)]
enum OriginalTaggedMutation{Remove{},Rename{text:String}}
const ORIGINAL_PROTOCOL:&str=include_str!("🧫️fixtures/🏷️protocol.semio");
impl OpBinary for OriginalTaggedMutation{fn encode_op(&self)->Result<Vec<u8>,protocol::ProtocolError>{semio_os_kernel::os_dsl::variants_binary::encode_tagged_op(ORIGINAL_PROTOCOL,self)}fn decode_op(bytes:&[u8])->Result<Self,protocol::ProtocolError>{semio_os_kernel::os_dsl::variants_binary::decode_tagged_op(ORIGINAL_PROTOCOL,bytes)}}
semio_os_kernel::original_tagged_operation_receiving!(OriginalTaggedMutation,ORIGINAL_PROTOCOL);
#[test]
fn original_command_receiving_operation_authored_tagged_header_resumes_each_original_item(){
 let fixture:serde_json::Value=serde_json::from_str(include_str!("🧫️fixtures/🏷️tagged.json")).unwrap();let expected=OriginalTaggedMutation::Rename{text:fixture["text"].as_str().unwrap().into()};let source=expected.encode_op().unwrap();let tag=fixture["tag"].as_u64().unwrap();assert_eq!(&source[..3],&[1,(tag as u8&127)|128,(tag>>7)as u8]);let f=&fixture["turn"];let grant=RetainedCloneGrant{maximum_items:f["items"].as_u64().unwrap()as usize,maximum_copy_bytes:f["copyBytes"].as_u64().unwrap()as usize,maximum_capacity_bytes:f["capacityBytes"].as_u64().unwrap()as usize,maximum_release_bytes:f["releaseBytes"].as_u64().unwrap()as usize,maximum_depth:f["depth"].as_u64().unwrap()as usize};let mut bank=OperationBank::fixture(grant);let mut callback=|_|true;let mut native=NativeDecodeControl::new(bank.left.maximum_capacity_bytes,&mut callback);let turn=OriginalTaggedMutation::admit_receiving(&source,(0,source.len()),bank.grant());let(mut cursor,p)=turn.result.unwrap().unwrap();bank.debit(p,turn.work);let mut turns=0;let mut selections=0;while !cursor.is_ready(){assert!(turns<10000);if cursor.phase()==2{selections+=1;}let mut denied=grant;denied.maximum_items=0;let turn=cursor.step(&source,&mut native,grant.maximum_copy_bytes,denied);assert_eq!(turn.work,0);assert_eq!(turn.result.unwrap(),Default::default());let progress=bank.normal(&mut native,|native,g|cursor.step(&source,native,grant.maximum_copy_bytes,g)).result.unwrap();assert_eq!(progress.copied_items,1);assert!(progress.fits(grant));turns+=1;}assert_eq!(selections,2);let mut output=None;let turn=cursor.take_into(&mut output,bank.grant());bank.debit(turn.result.unwrap(),turn.work);assert_eq!(output.as_ref(),Some(&expected));while !cursor.terminal_is_empty(){let turn=cursor.close_step(bank.grant());if matches!(&turn.result,Ok(RetainedCloneStep::Complete(_))){assert!(cursor.terminal_is_empty())}let p=turn.result.as_ref().map(|s|s.progress()).unwrap_or_else(|e|e.retained_progress());bank.debit(p,turn.work);turn.result.unwrap();}let mut output=ControlledRetirement::new(output.take().unwrap()).ok().unwrap();while !output.terminal_is_empty(){let result=output.step(bank.grant());let work=output.last_turn_work();let p=result.as_ref().map(|s|s.progress()).unwrap_or_else(|e|e.retained_progress());bank.debit(p,work);result.unwrap();}
}

struct OperationBank{left:RetainedCloneGrant,fuel:u64,quantum:RetainedCloneGrant}
struct OperationWorkLoan{fuel:u64}
impl semio_framework_value_native_owned::native_decoding::NativeDecodeWorkRecipient for OperationWorkLoan{fn available_work(&mut self,observed:u64)->Result<u64,ValueError>{let limit:u64=self.fuel;limit.checked_sub(observed).ok_or_else(||ValueError::invariant("native observed work exceeds its actual finite scope"))}fn admits(&mut self,observed:u64,additional:u64)->Result<bool,ValueError>{Ok(observed.checked_add(additional).is_some_and(|work|work<=self.fuel))}}
struct OperationPhysicalLoan{submitted:RetainedCloneProgress,left:RetainedCloneGrant}
impl semio_framework_value_native_owned::native_decoding::NativeDecodeConstructionRecipient for OperationPhysicalLoan{
 fn submitted(&self)->RetainedCloneProgress{self.submitted}fn grant(&mut self,_:RetirementDemand)->Result<RetainedCloneGrant,ValueError>{Ok(self.left)}
 fn accept(&mut self,g:RetainedCloneGrant,p:RetainedCloneProgress)->Result<(),ValueError>{self.submitted=self.submitted.checked_add(p)?;assert!(p.fits(g)&&p.fits(self.left));self.left.maximum_items-=p.copied_items;self.left.maximum_copy_bytes-=p.copied_bytes;self.left.maximum_capacity_bytes-=p.retained_capacity_bytes;self.left.maximum_release_bytes-=p.released_bytes;Ok(())}
}
impl OperationBank{
 fn fixture(quantum:RetainedCloneGrant)->Self{let f:serde_json::Value=serde_json::from_str(include_str!("../🎟️work/📐️.json")).unwrap();let s=&f["originalStock"];Self{quantum,left:RetainedCloneGrant{maximum_items:s["items"].as_u64().unwrap()as usize,maximum_copy_bytes:s["copyBytes"].as_u64().unwrap()as usize,maximum_capacity_bytes:s["capacityBytes"].as_u64().unwrap()as usize,maximum_release_bytes:s["releaseBytes"].as_u64().unwrap()as usize,maximum_depth:s["depth"].as_u64().unwrap()as usize},fuel:s["fuel"].as_u64().unwrap()}}
 fn grant(&self)->RetainedCloneGrant{RetainedCloneGrant{maximum_items:if self.fuel==0{0}else{self.left.maximum_items.min(self.quantum.maximum_items)},maximum_copy_bytes:self.left.maximum_copy_bytes.min(self.quantum.maximum_copy_bytes),maximum_capacity_bytes:self.left.maximum_capacity_bytes.min(self.quantum.maximum_capacity_bytes),maximum_release_bytes:self.left.maximum_release_bytes.min(self.quantum.maximum_release_bytes),maximum_depth:self.left.maximum_depth.min(self.quantum.maximum_depth)}}
 fn debit(&mut self,p:RetainedCloneProgress,work:u64){assert!(p.fits(self.left));self.left.maximum_items-=p.copied_items;self.left.maximum_copy_bytes-=p.copied_bytes;self.left.maximum_capacity_bytes-=p.retained_capacity_bytes;self.left.maximum_release_bytes-=p.released_bytes;self.fuel=self.fuel.checked_sub(work).unwrap();}
 fn normal(&mut self,native:&mut NativeDecodeControl<'_>,operation:impl FnOnce(&mut NativeDecodeControl<'_>,RetainedCloneGrant)->RetainedWorkOutcome<RetainedCloneProgress,OriginalReceivingPrimary>)->RetainedWorkOutcome<RetainedCloneProgress,OriginalReceivingPrimary>{let g=self.grant();let mut physical=OperationPhysicalLoan{submitted:Default::default(),left:g};let mut work=OperationWorkLoan{fuel:self.fuel.min(1)};let turn=native.with_work_recipient(&mut work,|native|native.with_construction_recipient(&mut physical,|native|Ok::<_,ValueError>(operation(native,g)))).unwrap();let p=turn.result.as_ref().copied().unwrap_or_else(|e|e.retained_progress());self.debit(p,turn.work);turn}
}
#[test]fn original_operation_failure_preserves_real_probe_cpu_independently_of_physical_progress(){
 let f:serde_json::Value=serde_json::from_str(include_str!("../🎟️work/📐️.json")).unwrap();let s=&f["stock"];let left=RetainedCloneGrant{maximum_items:s["items"].as_u64().unwrap()as usize,maximum_copy_bytes:s["copyBytes"].as_u64().unwrap()as usize,maximum_capacity_bytes:s["capacityBytes"].as_u64().unwrap()as usize,maximum_release_bytes:s["releaseBytes"].as_u64().unwrap()as usize,maximum_depth:s["depth"].as_u64().unwrap()as usize};let mut bank=OperationBank{left,fuel:s["fuel"].as_u64().unwrap(),quantum:RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:4096,..left}};let source:Vec<u8>=serde_json::from_value(f["source"].clone()).unwrap();assert!(OriginalMutation::decode_op(&source).is_err());let g=bank.grant();let(turn,h)=semio_framework_trace::observe_heap_allocations_on_this_thread(||OriginalMutation::admit_receiving(&source,(0,source.len()),g));assert_eq!((h.requested_bytes,h.released_bytes),(0,0));assert_eq!(turn.work,f["admissionWork"].as_u64().unwrap());let(mut cursor,p)=turn.result.unwrap().unwrap();assert_eq!(p.copied_bytes,f["admissionCopyBytes"].as_u64().unwrap()as usize);bank.debit(p,turn.work);let mut callback=|_|true;let mut native=NativeDecodeControl::new(bank.left.maximum_capacity_bytes,&mut callback);
 let g=bank.grant();let denied=RetainedCloneGrant{maximum_items:0,..g};let turn=cursor.step(&source,&mut native,4096,denied);assert_eq!(turn.work,0);assert_eq!(turn.result.unwrap(),Default::default());let g=bank.grant();let turn=bank.normal(&mut native,|native,g|cursor.step(&source,native,4096,g));assert_eq!(turn.work,f["failureWork"].as_u64().unwrap());let fault=turn.result.unwrap_err();assert_eq!(fault.primary.as_ref().unwrap().kind,ValueRefusalKind::InvalidValue);assert!(fault.quote_refusal.is_none()&&fault.allocation_refusal.is_none());assert_eq!(fault.retained_progress().copied_items,f["failureItems"].as_u64().unwrap()as usize);
 let mut fault=ControlledRetirement::new(fault).unwrap_or_else(|_|panic!("actual native failure has original retirement"));for _ in 0..10000{if fault.terminal_is_empty(){break}let result=fault.step(bank.grant());let work=fault.last_turn_work();let p=result.as_ref().map(|s|s.progress()).unwrap_or_else(|e|e.retained_progress());bank.debit(p,work);result.unwrap();}assert!(fault.terminal_is_empty());for _ in 0..10000{if cursor.terminal_is_empty(){break}let turn=cursor.close_step(bank.grant());if matches!(&turn.result,Ok(RetainedCloneStep::Complete(_))){assert!(cursor.terminal_is_empty())}let p=turn.result.as_ref().map(|s|s.progress()).unwrap_or_else(|e|e.retained_progress());bank.debit(p,turn.work);turn.result.unwrap();}assert!(cursor.terminal_is_empty());assert_eq!(cursor.close_step(Default::default()).work,0);
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
