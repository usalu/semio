//! 🧪️ Original decoder grants govern record-intermediate retirement before native publication.
use super::*;
use crate::original_close::CloseOriginal as _;
use semio_framework_value_native_owned::{native_decoding::NativeDecodeRetirementRecipient, native_decoding::{NativeDecodeControl,OriginalConstructionAuthority,OriginalWorkAuthority}};
use crate::original_close::{retire,with_port,ConstructionStock,WorkStock};
use semio_framework_value_native_owned::native_decoding::NativeAllocationEvent;
use std::sync::atomic::{AtomicUsize,Ordering};
use semio_framework_dsl_record::{RecordSpec,RecordLayout,FieldSpec,Shape,FieldValue,NativeSchemaControl};
fn closed(native:&mut NativeDecodeControl<'_>,grant:semio_framework_value::RetainedCloneGrant)->crate::original_close::Close{let mut construction=OriginalConstructionAuthority::new(ConstructionStock::new(grant));let mut work=OriginalWorkAuthority::new(WorkStock(1<<24));construction.with_loan(&mut (),|construction|work.with_loan(&mut (),|work|native.with_work_recipient(work,|native|native.with_construction_recipient(construction,|native|Ok::<_,ValueError>(native.close_original(grant)))))).unwrap().unwrap().unwrap()}
fn ordinary()->RecordSpec{RecordSpec::new(None,RecordLayout::Lines,vec![FieldSpec::new(1,"value",Shape::Text)])}
fn schema<C:NativeSchemaControl>(native:&mut C)->Result<RecordSpec,ValueError>{let mut fields=native.allocate_vec(1)?;fields.push(semio_framework_dsl_record::producer::field(1,"value",Shape::Text,native)?);semio_framework_dsl_record::producer::record(None,RecordLayout::Lines,fields,native)}
fn decode_schema(native:&mut NativeDecodeControl<'_>)->Result<RecordSpec,ValueError>{schema(native)}
fn encode_schema(native:&mut semio_framework_value::NativeEncodeControl<'_>)->Result<RecordSpec,ValueError>{schema(native)}
fn rows(forwarded:bool){
 let law:serde_json::Value=serde_json::from_str(include_str!("../🧫️fixtures/🚪️control/🔣️.json")).unwrap();let payload=semio_framework_io_schema::IoPayload::Text(law["nativeLiteral"].as_str().unwrap().to_owned());let literal=law["literal"].as_str().unwrap();let maximum=law["nativeBytes"].as_u64().unwrap()as usize;
 for row in law["cases"].as_array().unwrap(){let mut grant:semio_framework_value::RetainedCloneGrant=serde_json::from_value(law["grant"].clone()).unwrap();grant.maximum_release_bytes=row["release"].as_u64().unwrap()as usize;let mut recipient=NativeDecodeRetirementRecipient::new();let mut progress=|_|true;let mut sql=SqliteSnapshotControl::new(&mut progress,semio_framework_io_sqlite_snapshot::SqliteDatabaseLimits::default());let spec=RecordSpecProducer{ordinary,decoding:decode_schema,encoding:encode_schema};

  let mut drive=|native:&mut NativeDecodeControl<'_>|{
 let mut work=OriginalWorkAuthority::new(WorkStock(row["work"].as_u64().unwrap_or_else(||law["workLimit"].as_u64().unwrap())));
 let mut construction=OriginalConstructionAuthority::new(ConstructionStock::new(grant));
 let (result,mut output)=construction.with_loan(&mut (),|construction|work.with_loan(&mut (),|loan|native.with_work_recipient(loan,|native|native.with_construction_recipient(construction,|native|{
  let mut output=None;
  let result=decode_sqlite_snapshot_record_native(&payload,law["envelope"].as_str().unwrap(),spec,&mut |record,retained,native|{let text=match record.fields.get(&1){Some(FieldValue::Text(text))=>text,_=>return Err(ValueError::literal(ValueRefusalKind::InvalidValue,"original record fixture text differs"))};*retained=Some(native.copy_text(text)?);Ok(())},&mut sql,&mut semio_framework_io_control::NativeSnapshotDecodeOwner::new(native,grant),&mut output);
  assert_eq!(result.result.is_ok(),row["accepted"].as_bool().unwrap(),"{}",row["id"]);;if let Some(expected)=row["expectedWork"].as_u64(){assert_eq!(result.work,u128::from(expected))}
  Ok::<_,ValueError>((result,output))
 }))))
 .unwrap().unwrap().unwrap();
  let full:semio_framework_value::RetainedCloneGrant=serde_json::from_value(law["grant"].clone()).unwrap();
  match result.result{Ok(())=>match output.take().expect("accepted decode publishes"){Ok(text)=>{assert_eq!(serde_json::to_value(&text).unwrap(),serde_json::to_value(literal).unwrap());let mut last=String::new();for _ in 0..10000{if !native.has_retirement_owner(){break}let turn=closed(native,full);last=format!("{:?} work {}",turn.result.as_ref().map(|step|step.progress()).map_err(|_|()),turn.work);turn.result.unwrap();}assert!(!native.has_retirement_owner(),"row {} {last}",row["id"]);retire(text,full);},Err(error)=>{retire(error,full);panic!("accepted decode published a failure")}},Err(error)=>{assert_eq!(native.has_retirement_owner(),row["pending"].as_bool().unwrap());let mut last=String::new();for _ in 0..10000{if !native.has_retirement_owner(){break}let turn=closed(native,full);last=format!("{:?} work {}",turn.result.as_ref().map(|step|step.progress()).map_err(|_|()),turn.work);turn.result.unwrap();}assert!(!native.has_retirement_owner(),"row {} {last}",row["id"]);retire(error,full);if let Some(published)=output.take(){retire(published,full)}}}
};
  if forwarded{
   let ledger=AtomicUsize::new(0);let port=|event:NativeAllocationEvent|{match event{NativeAllocationEvent::Admit(request)=>{assert_eq!(request.owned_bytes,ledger.load(Ordering::Relaxed));ledger.store(request.next_owned_bytes,Ordering::Relaxed)}NativeAllocationEvent::Post{request,actual}=>ledger.store(request.owned_bytes+actual,Ordering::Relaxed),_=>{}}Ok(())};
   with_port(port,|loan|{let mut observer=|_|true;let mut native=NativeDecodeControl::new_forwarded(maximum,&mut observer,loan);native.install_retirement_recipient(&mut recipient).unwrap();drive(&mut native);assert_eq!(native.owned_bytes(),ledger.load(Ordering::Relaxed));drop(native);});
  }else{let mut observer=|_|true;let mut native=NativeDecodeControl::new(maximum,&mut observer);native.install_retirement_recipient(&mut recipient).unwrap();drive(&mut native);drop(native);}
  assert!(!recipient.has_owner());}
}
#[test]
fn original_record_decoder_returns_refused_retirement_and_preserves_the_native_oracle(){rows(false)}
#[test]
fn original_record_decoder_keeps_its_ledger_equal_to_the_forwarded_port_through_refused_retirement(){rows(true)}
