//! 🧪️ Original decoder grants govern record-intermediate retirement before native publication.
use super::*;
use semio_framework_value_native_owned::{native_decoding::NativeDecodeRetirementRecipient, native_decoding::{NativeDecodeAllocation,NativeDecodeControl,NativeDecodeWorkRecipient}};
use semio_framework_dsl_record::{RecordSpec,RecordLayout,FieldSpec,Shape,FieldValue,NativeSchemaControl};
fn retire<T:semio_framework_value::retirement::RetireOwned>(value:T,grant:semio_framework_value::RetainedCloneGrant){let mut owner=semio_framework_value::retirement::controlled::ControlledRetirement::new(value).ok().unwrap();for _ in 0..65536{if owner.terminal_is_empty(){break}let(result,born,released)=semio_framework_trace_memory_testing::observe_backing(||owner.step(grant));let progress=result.unwrap().progress();assert_eq!((born,released),(progress.retained_capacity_bytes,progress.released_bytes));}assert!(owner.terminal_is_empty());}
fn ordinary()->RecordSpec{RecordSpec::new(None,RecordLayout::Lines,vec![FieldSpec::new(1,"value",Shape::Text)])}
fn schema<C:NativeSchemaControl>(native:&mut C)->Result<RecordSpec,ValueError>{let mut fields=native.allocate_vec(1)?;fields.push(semio_framework_dsl_record::producer::field(1,"value",Shape::Text,native)?);semio_framework_dsl_record::producer::record(None,RecordLayout::Lines,fields,native)}
fn decode_schema(native:&mut NativeDecodeControl<'_>)->Result<RecordSpec,ValueError>{schema(native)}
fn encode_schema(native:&mut semio_framework_value::NativeEncodeControl<'_>)->Result<RecordSpec,ValueError>{schema(native)}
#[test]
fn original_record_decoder_returns_refused_retirement_and_preserves_the_native_oracle(){
 let law:serde_json::Value=serde_json::from_str(include_str!("../🧫️fixtures/🚪️control/🔣️.json")).unwrap();let payload=semio_framework_io_schema::IoPayload::Text(law["nativeLiteral"].as_str().unwrap().to_owned());let literal=law["literal"].as_str().unwrap();
 for row in law["cases"].as_array().unwrap(){let mut grant:semio_framework_value::RetainedCloneGrant=serde_json::from_value(law["grant"].clone()).unwrap();grant.maximum_release_bytes=row["release"].as_u64().unwrap()as usize;let mut recipient=NativeDecodeRetirementRecipient::new();let ledger=std::sync::atomic::AtomicUsize::new(0);let mut allocation=|request:NativeDecodeAllocation|{assert_eq!(request.owned_bytes,ledger.load(std::sync::atomic::Ordering::Relaxed));ledger.store(request.next_owned_bytes,std::sync::atomic::Ordering::Relaxed);Ok(())};let mut progress=|_|true;let mut native=NativeDecodeControl::new_forwarded(law["nativeBytes"].as_u64().unwrap()as usize,&mut progress,&mut allocation);native.install_retirement_recipient(&mut recipient).unwrap();let mut progress=|_|true;let mut sql=SqliteSnapshotControl::new(&mut progress,semio_framework_io_sqlite_snapshot::SqliteDatabaseLimits::default());let spec=RecordSpecProducer{ordinary,decoding:decode_schema,encoding:encode_schema};

 struct Loan(u64);
 impl NativeDecodeWorkRecipient for Loan{fn admits(&mut self,observed:u64,additional:u64)->Result<bool,ValueError>{Ok(observed.checked_add(additional).is_some_and(|total|total<=self.0))}}
 let mut loan=Loan(row["work"].as_u64().unwrap_or_else(||law["workLimit"].as_u64().unwrap()));
 native.with_work_recipient(&mut loan,|native|{
  let result=decode_sqlite_snapshot_record_native(&payload,law["envelope"].as_str().unwrap(),spec,&mut |record,snapshot_output,owner,_sql|{
   let text=match record.fields.get(&1){Some(FieldValue::Text(text))=>text,_=>return semio_framework_value::RetainedWorkOutcome{result:Err(ValueError::literal(ValueRefusalKind::InvalidValue,"original record fixture text differs").into()),work:0}};
   owner.receive::<(),(),NativeSnapshotFailure>(&mut |_,native,body|native_snapshot_turn(|work|{
    let bytes=size_of::<String>();body.admit_frontier(semio_framework_value::RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:bytes,maximum_depth:1,..Default::default()})?;
    work.admit(native)?;*snapshot_output=Some(String::new());body.record_progress(RetainedCloneProgress{copied_items:1,copied_bytes:bytes,..Default::default()})?;
    work.admit(native)?;body.copy_text_into(native,text,snapshot_output.as_mut().unwrap(),1)?;Ok(())
   }))
  },&mut sql,&mut semio_framework_io_control::NativeSnapshotDecodeOwner::new(native,grant));
  assert_eq!(result.result.is_ok(),row["accepted"].as_bool().unwrap());if let Some(expected)=row["expectedWork"].as_u64(){assert_eq!(result.work,expected)}
  let full:semio_framework_value::RetainedCloneGrant=serde_json::from_value(law["grant"].clone()).unwrap();
  match result.result{Ok(text)=>{assert_eq!(serde_json::to_value(&text).unwrap(),serde_json::to_value(literal).unwrap());assert!(!native.has_retirement_owner());retire(text,full);},Err(error)=>{assert_eq!(native.has_retirement_owner(),row["pending"].as_bool().unwrap());while native.has_retirement_owner(){native.close_retirement_recipient(full).result.unwrap();assert_eq!(native.owned_bytes(),ledger.load(std::sync::atomic::Ordering::Relaxed));}retire(error,full);}}
  Ok::<(),ValueError>(())
 }).unwrap();
 drop(native);assert!(!recipient.has_owner());}
}
