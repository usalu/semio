//! 🧪️ The original record helper publishes only after admitted physical retirement.
use super::*;
use crate::original_close::CloseOriginal as _;
use semio_framework_value::native_encoding::{NativeEncodeControl,NativeEncodeRetirementRecipient,OriginalConstructionAuthority,OriginalWorkAuthority};
use crate::original_close::{retire,with_port,ConstructionStock,WorkStock};
use semio_framework_value::native_encoding::NativeAllocationEvent;
use std::sync::atomic::{AtomicUsize,Ordering};
use semio_framework_dsl_record::{RecordSpec,RecordLayout,FieldSpec,Shape,FieldValue,NativeSchemaControl,RecordFields};
fn ordinary()->RecordSpec{RecordSpec::new(None,RecordLayout::Lines,vec![FieldSpec::new(1,"value",Shape::Text)])}
fn schema<C:NativeSchemaControl>(native:&mut C)->Result<RecordSpec,ValueError>{let mut fields=native.allocate_vec(1)?;fields.push(semio_framework_dsl_record::producer::field(1,"value",Shape::Text,native)?);semio_framework_dsl_record::producer::record(None,RecordLayout::Lines,fields,native)}
fn decode_schema(native:&mut semio_framework_value_native_owned::native_decoding::NativeDecodeControl<'_>)->Result<RecordSpec,ValueError>{schema(native)}
fn encode_schema(native:&mut NativeEncodeControl<'_>)->Result<RecordSpec,ValueError>{schema(native)}
fn rows(forwarded:bool){
 let neutral:serde_json::Value=serde_json::from_str(include_str!("../🧫️fixtures/🚪️control/🔣️.json")).unwrap();let literal=neutral["literal"].as_str().unwrap();let source=literal.as_ptr();let maximum=neutral["nativeBytes"].as_u64().unwrap()as usize;
 for row in neutral["cases"].as_array().unwrap(){
  let mut grant:semio_framework_value::RetainedCloneGrant=serde_json::from_value(neutral["grant"].clone()).unwrap();grant.maximum_release_bytes=row["release"].as_u64().unwrap()as usize;
  let mut recipient=NativeEncodeRetirementRecipient::new();let mut work=OriginalWorkAuthority::new(WorkStock(row["work"].as_u64().unwrap_or_else(||neutral["workLimit"].as_u64().unwrap())));let mut callback=|_|true;let mut sql=SqliteSnapshotControl::new(&mut callback,semio_framework_io_sqlite_snapshot::SqliteDatabaseLimits::default());let producer=RecordSpecProducer{ordinary,decoding:decode_schema,encoding:encode_schema};
  let mut construction=OriginalConstructionAuthority::new(ConstructionStock::new(grant));
  let mut drive=|native:&mut NativeEncodeControl<'_>|{
  construction.with_loan(&mut (),|construction|work.with_loan(&mut (),|loan|native.with_work_recipient(loan,|native|native.with_construction_recipient(construction,|native|{
   let mut owner=semio_framework_io_control::NativeSnapshotEncodeOwner::new(native,grant);
   let mut output=None;
   let result=encode_sqlite_snapshot_record_native(SnapshotEncoding::Text,neutral["envelope"].as_str().unwrap(),producer,&mut |native|{let slots=native.allocate_vec::<(u16,FieldValue)>(1)?;let text=native.copy_text(literal)?;let mut fields=RecordFields::from_empty_slots(slots);fields.insert(1,FieldValue::Text(text));Ok(RecordValue{fields})},&mut sql,&mut owner,&mut output);
   assert_eq!(owner.grant(),grant);assert_eq!(result.result.is_ok(),row["accepted"].as_bool().unwrap(),"{}",row["id"]);;if let Some(expected)=row["expectedWork"].as_u64(){assert_eq!(result.work,u128::from(expected))}drop(owner);let admitted:semio_framework_value::RetainedCloneGrant=serde_json::from_value(neutral["grant"].clone()).unwrap();
   match result.result{
    Ok(())=>match output.take().expect("accepted encode publishes"){
     Ok(semio_framework_io_schema::IoPayload::Text(text))=>{let(header,body)=text.split_once('\n').unwrap();assert_eq!(header,neutral["expectedHeader"].as_str().unwrap());let key=neutral["expectedKey"].as_str().unwrap();let value=body.trim().strip_prefix(key).unwrap().strip_prefix('=').unwrap();assert_eq!(serde_json::from_str::<String>(value).unwrap(),literal);for _ in 0..10000{if !native.has_retirement_owner(){break}native.close_original(admitted).result.unwrap();}assert!(!native.has_retirement_owner(),"row {}",row["id"]);retire(text,admitted);}
     Ok(other)=>{retire(other,admitted);panic!("record fixture expected text")}
     Err(error)=>{retire(error,admitted);panic!("accepted encode published a failure")}
    },
    Err(error)=>{assert_eq!(native.has_retirement_owner(),row["pending"].as_bool().unwrap());for _ in 0..10000{if !native.has_retirement_owner(){break}native.close_original(admitted).result.unwrap();}assert!(!native.has_retirement_owner(),"row {}",row["id"]);retire(error,admitted);if let Some(published)=output.take(){retire(published,admitted)}}
   }
   Ok::<(),ValueError>(())
  })))).unwrap().unwrap().unwrap();};
  if forwarded{
   let ledger=AtomicUsize::new(0);let port=|event:NativeAllocationEvent|{match event{NativeAllocationEvent::Admit(request)=>{assert_eq!(request.owned_bytes,ledger.load(Ordering::Relaxed));ledger.store(request.next_owned_bytes,Ordering::Relaxed)}NativeAllocationEvent::Quote(request)=>assert_eq!(request.owned_bytes,ledger.load(Ordering::Relaxed)),NativeAllocationEvent::Post{request,actual}=>ledger.store(request.owned_bytes+actual,Ordering::Relaxed),NativeAllocationEvent::Cancel(_)=>{}}Ok(())};
   with_port(port,|loan|{let mut observer=|_|true;let mut native=NativeEncodeControl::new_forwarded(maximum,&mut observer,loan);native.install_retirement_recipient(&mut recipient).unwrap();drive(&mut native);assert_eq!(native.owned_bytes(),ledger.load(Ordering::Relaxed));drop(native);});
  }else{let mut observer=|_|true;let mut native=NativeEncodeControl::new(maximum,&mut observer);native.install_retirement_recipient(&mut recipient).unwrap();drive(&mut native);drop(native);}
  assert!(!recipient.has_owner());assert_eq!(literal.as_ptr(),source);
 }
}
#[test]
fn original_record_helper_preserves_grant_output_and_pending_retirement(){rows(false)}
#[test]
fn original_record_helper_keeps_its_ledger_equal_to_the_forwarded_port_through_pending_retirement(){rows(true)}
