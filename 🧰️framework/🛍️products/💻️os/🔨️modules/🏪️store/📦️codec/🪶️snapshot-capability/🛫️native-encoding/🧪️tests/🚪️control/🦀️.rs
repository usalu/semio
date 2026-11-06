//! 🧪️ The original record helper publishes only after admitted physical retirement.
use super::*;
use semio_framework_value::native_encoding::{NativeEncodeControl,NativeEncodeRetirementRecipient,NativeAllocationRequest,NativeEncodeWorkRecipient};
use semio_framework_dsl_record::{RecordSpec,RecordLayout,FieldSpec,Shape,FieldValue,NativeSchemaControl,RecordFields};
struct Loan(u64);
impl NativeEncodeWorkRecipient for Loan{fn admits(&mut self,observed:u64,additional:u64)->Result<bool,ValueError>{Ok(observed.checked_add(additional).is_some_and(|total|total<=self.0))}fn committed(&mut self,_actual:u64)->Result<(),ValueError>{Ok(())}}
fn retire<T:semio_framework_value::retirement::RetireOwned>(value:T,grant:semio_framework_value::RetainedCloneGrant){let mut owner=semio_framework_value::retirement::controlled::ControlledRetirement::new(value).ok().unwrap();for _ in 0..65536{if owner.terminal_is_empty(){break}let(result,born,released)=semio_framework_trace_memory_testing::observe_backing(||owner.step(grant));let progress=result.unwrap().progress();assert_eq!((born,released),(progress.retained_capacity_bytes,progress.released_bytes));}assert!(owner.terminal_is_empty());}
fn ordinary()->RecordSpec{RecordSpec::new(None,RecordLayout::Lines,vec![FieldSpec::new(1,"value",Shape::Text)])}
fn schema<C:NativeSchemaControl>(native:&mut C)->Result<RecordSpec,ValueError>{let mut fields=native.allocate_vec(1)?;fields.push(semio_framework_dsl_record::producer::field(1,"value",Shape::Text,native)?);semio_framework_dsl_record::producer::record(None,RecordLayout::Lines,fields,native)}
fn decode_schema(native:&mut semio_framework_value_native_owned::native_decoding::NativeDecodeControl<'_>)->Result<RecordSpec,ValueError>{schema(native)}
fn encode_schema(native:&mut NativeEncodeControl<'_>)->Result<RecordSpec,ValueError>{schema(native)}
#[test]
fn original_record_helper_preserves_grant_output_and_pending_retirement(){
 let neutral:serde_json::Value=serde_json::from_str(include_str!("../🧫️fixtures/🚪️control/🔣️.json")).unwrap();let literal=neutral["literal"].as_str().unwrap();let source=literal.as_ptr();
 for row in neutral["cases"].as_array().unwrap(){
  let mut grant:semio_framework_value::RetainedCloneGrant=serde_json::from_value(neutral["grant"].clone()).unwrap();grant.maximum_release_bytes=row["release"].as_u64().unwrap()as usize;
  let mut recipient=NativeEncodeRetirementRecipient::new();let ledger=std::sync::atomic::AtomicUsize::new(0);let mut allocation=|request:NativeAllocationRequest|{assert_eq!(request.owned_bytes,ledger.load(std::sync::atomic::Ordering::Relaxed));ledger.store(request.next_owned_bytes,std::sync::atomic::Ordering::Relaxed);Ok(())};let mut progress=|_|true;let mut native=NativeEncodeControl::new_forwarded(neutral["nativeBytes"].as_u64().unwrap()as usize,&mut progress,&mut allocation);native.install_retirement_recipient(&mut recipient).unwrap();let mut loan=Loan(row["work"].as_u64().unwrap_or_else(||neutral["workLimit"].as_u64().unwrap()));let mut callback=|_|true;let mut sql=SqliteSnapshotControl::new(&mut callback,semio_framework_io_sqlite_snapshot::SqliteDatabaseLimits::default());let producer=RecordSpecProducer{ordinary,decoding:decode_schema,encoding:encode_schema};
  native.with_work_recipient(&mut loan,|native|{
   let mut owner=semio_framework_io_control::NativeSnapshotEncodeOwner::new(native,grant);
   let result=encode_sqlite_snapshot_record_native(SnapshotEncoding::Text,neutral["envelope"].as_str().unwrap(),producer,&mut |output,owner|{
    owner.receive::<(Vec<(u16,FieldValue)>,String),(),NativeSnapshotFailure>(&mut |slot,native,body|native_snapshot_turn(|work|{
     let bytes=size_of::<(Vec<(u16,FieldValue)>,String)>();body.admit_frontier(semio_framework_value::RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:bytes,maximum_depth:1,..Default::default()})?;
     work.admit_encode(native)?;*slot=Some((Vec::new(),String::new()));body.record_progress(RetainedCloneProgress{copied_items:1,copied_bytes:bytes,..Default::default()})?;
     let frame=slot.as_mut().unwrap();work.admit_encode(native)?;body.allocate_encode_vec_into(native,1,&mut frame.0,2)?;
     work.admit_encode(native)?;body.copy_encode_text_into(native,literal,&mut frame.1,2)?;
     let bytes=size_of::<RecordValue>();body.admit_frontier(semio_framework_value::RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:bytes,maximum_depth:1,..Default::default()})?;
     work.admit_encode(native)?;let mut fields=RecordFields::from_empty_slots(std::mem::take(&mut frame.0));fields.insert(1,FieldValue::Text(std::mem::take(&mut frame.1)));*output=Some(RecordValue{fields});body.record_progress(RetainedCloneProgress{copied_items:1,copied_bytes:bytes,..Default::default()})?;Ok(())
    }))
   },&mut sql,&mut owner);
   assert_eq!(owner.grant(),grant);assert_eq!(result.result.is_ok(),row["accepted"].as_bool().unwrap());if let Some(expected)=row["expectedWork"].as_u64(){assert_eq!(result.work,expected)}drop(owner);let owned=native.owned_bytes();assert_eq!(owned,ledger.load(std::sync::atomic::Ordering::Relaxed));let admitted:semio_framework_value::RetainedCloneGrant=serde_json::from_value(neutral["grant"].clone()).unwrap();
   match result.result{
    Ok(semio_framework_io_schema::IoPayload::Text(text))=>{let(header,body)=text.split_once('\n').unwrap();assert_eq!(header,neutral["expectedHeader"].as_str().unwrap());let key=neutral["expectedKey"].as_str().unwrap();let value=body.trim().strip_prefix(key).unwrap().strip_prefix('=').unwrap();assert_eq!(serde_json::from_str::<String>(value).unwrap(),literal);assert!(!native.has_retirement_owner());retire(text,admitted);}
    Ok(other)=>{retire(other,admitted);panic!("record fixture expected text")}
    Err(error)=>{assert_eq!(native.has_retirement_owner(),row["pending"].as_bool().unwrap());while native.has_retirement_owner(){native.close_retirement_recipient(admitted).result.unwrap();assert_eq!(native.owned_bytes(),ledger.load(std::sync::atomic::Ordering::Relaxed));}retire(error,admitted);}
   }
   Ok::<(),ValueError>(())
  }).unwrap();drop(native);assert!(!recipient.has_owner());assert_eq!(literal.as_ptr(),source);
 }
}
