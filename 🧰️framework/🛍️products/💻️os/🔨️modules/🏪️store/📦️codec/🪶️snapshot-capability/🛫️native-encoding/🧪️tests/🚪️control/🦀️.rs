//! 🧪️ The original record helper publishes only after admitted physical retirement.
use super::*;
use semio_framework_value::native_encoding::{NativeEncodeRetirementRecipient,NativeEncodeAllocation};
use semio_framework_dsl_record::{RecordSpec,RecordLayout,FieldSpec,Shape,FieldValue,NativeSchemaControl,RecordFields};
fn ordinary()->RecordSpec{RecordSpec::new(None,RecordLayout::Lines,vec![FieldSpec::new(1,"value",Shape::Text)])}
fn schema<C:NativeSchemaControl>(native:&mut C)->Result<RecordSpec,ValueError>{let mut fields=native.allocate_vec(1)?;fields.push(semio_framework_dsl_record::producer::field(1,"value",Shape::Text,native)?);semio_framework_dsl_record::producer::record(None,RecordLayout::Lines,fields,native)}
fn decode_schema(native:&mut semio_framework_value::NativeDecodeControl<'_>)->Result<RecordSpec,ValueError>{schema(native)}
fn encode_schema(native:&mut NativeEncodeControl<'_>)->Result<RecordSpec,ValueError>{schema(native)}
#[test]
fn original_record_helper_preserves_grant_output_and_pending_retirement(){
 let neutral:serde_json::Value=serde_json::from_str(include_str!("../../🧫️fixtures/🚪️control/🔣️.json")).unwrap();let literal=neutral["literal"].as_str().unwrap();let source=literal.as_ptr();
 for row in neutral["cases"].as_array().unwrap(){
  let mut grant:semio_framework_value::RetainedCloneGrant=serde_json::from_value(neutral["grant"].clone()).unwrap();grant.maximum_release_bytes=row["release"].as_u64().unwrap()as usize;
  let mut recipient=NativeEncodeRetirementRecipient::new();let ledger=std::sync::atomic::AtomicUsize::new(0);let mut allocation=|request:NativeEncodeAllocation|{assert_eq!(request.owned_bytes,ledger.load(std::sync::atomic::Ordering::Relaxed));ledger.store(request.next_owned_bytes,std::sync::atomic::Ordering::Relaxed);Ok(())};let mut progress=|_|true;let mut native=NativeEncodeControl::new_forwarded(neutral["nativeBytes"].as_u64().unwrap()as usize,&mut progress,&mut allocation);native.install_retirement_recipient(&mut recipient).unwrap();let mut owner=crate::os_store::NativeSnapshotEncodeOwner::new(&mut native,grant);let mut callback=|_|true;let mut sql=SqliteSnapshotControl::new(&mut callback,crate::sqlite_snapshot::SqliteDatabaseLimits::default());
  let producer=RecordSpecProducer{ordinary,decoding:decode_schema,encoding:encode_schema};
  let result=encode_sqlite_snapshot_record_native(SnapshotEncoding::Text,neutral["envelope"].as_str().unwrap(),producer,|native|{let mut fields=RecordFields::from_empty_slots(native.allocate_vec(1)?);fields.insert(1,FieldValue::Text(native.copy_text(literal)?));Ok(RecordValue{fields})},&mut sql,&mut owner);assert_eq!(owner.grant(),grant);if let Err(error)=&result{println!("[DEBUG] Original helper refusal {} kind={:?} message={}",row["id"],error.kind,error.message);}assert_eq!(result.is_ok(),row["accepted"].as_bool().unwrap());drop(owner);let owned=native.owned_bytes();assert_eq!(owned,ledger.load(std::sync::atomic::Ordering::Relaxed));
  if let Ok(crate::io_schema::IoPayload::Text(text))=result{let(header,body)=text.split_once('\n').unwrap();assert_eq!(header,neutral["expectedHeader"].as_str().unwrap());let key=neutral["expectedKey"].as_str().unwrap();let value=body.trim().strip_prefix(key).unwrap().strip_prefix('=').unwrap();assert_eq!(serde_json::from_str::<String>(value).unwrap(),literal);assert!(!native.has_retirement_owner());}else{assert!(native.has_retirement_owner());let admitted:semio_framework_value::RetainedCloneGrant=serde_json::from_value(neutral["grant"].clone()).unwrap();while native.has_retirement_owner(){native.close_retirement_recipient(admitted).unwrap();assert_eq!(native.owned_bytes(),ledger.load(std::sync::atomic::Ordering::Relaxed));}}drop(native);assert!(!recipient.has_owner());
  assert_eq!(literal.as_ptr(),source);println!("[DEBUG] Original snapshot record helper {} nativeOwned={} originalGrant=true originalRecipient=true independentSerde=true sourcePointer=true",row["id"],owned);
 }
}
