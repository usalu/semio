//! 🎟️ The genuine caller's current loan admits one actual native probe and refuses a second without renewing authority.
use super::*;
use semio_framework_job::{OriginalJobFixtureCaller,OperationId,Generation};
use semio_framework_trace::observe_heap_allocations_on_this_thread as observe;
#[test]
fn native_work_recipient_uses_actual_current_caller_loan_and_posts_once(){
 let row:serde_json::Value=serde_json::from_str(include_str!("case.json")).unwrap();let schema:serde_json::Value=serde_json::from_str(include_str!("case.schema.json")).unwrap();for(k,v)in row.as_object().unwrap(){assert_eq!(&schema["properties"][k]["const"],v);}
 for reported in row["reportedWork"].as_array().unwrap(){let reported=reported.as_u64().unwrap();let bytes:Vec<u8>=serde_json::from_value(row["probe"].clone()).unwrap();let stock=super::transport_tests::bank();let mut caller=OriginalJobFixtureCaller::admit(OperationId(7679),Generation(3),stock,||Some(0),1000000,10000);
 caller.turn(stock,|cx|cx.with_original_work_loan(row["currentWork"].as_u64().unwrap(),10000,|cx|{
  let before=cx.original_work_units_spent();let mut callback=|_|true;let mut control=NativeDecodeControl::new(0,&mut callback);let ((valid,work),heap)=observe(||{
   let borrowed=ActualTurn::new(cx);let mut recipient=ActualWork{turn:&borrowed,closing:false};
   let valid=control.with_work_recipient(&mut recipient,|control|{assert!(control.prepare_work(1)?);let valid=std::str::from_utf8(&bytes).is_ok();control.commit_work(1)?;assert_eq!(control.prepare_work(1)?,row["nextWorkAdmitted"].as_bool().unwrap());Ok::<_,ValueError>(valid)}).unwrap();(valid,control.actual_work())
  });assert!(!heap.overflowed);assert_eq!([heap.requested_bytes,heap.released_bytes],[0,0]);assert_eq!(valid,row["expectedValid"].as_bool().unwrap());assert_eq!(work,row["expectedWork"].as_u64().unwrap());assert_eq!(cx.original_work_units_spent(),before);let supplied=cx.retained_grant();let received=OriginalNativeDecodeSource::receive_observed(cx,supplied,cx.fuel_remaining(),OriginalNativeDecodeIssued::<(),ValueError>{value:Err(ValueError::literal(ValueRefusalKind::InvalidValue,"actual native UTF-8 probe rejected its input")),grant:supplied,progress:Default::default(),work:reported},work);assert_eq!(received.reported_work,reported);assert_eq!(received.observed_native_work,work);assert_eq!(received.work,work);assert_eq!(received.value.unwrap_err().message.as_ref(),"actual native UTF-8 probe rejected its input");assert_eq!(received.receiving_refusal.is_some(),reported<work);if let Some(error)=received.receiving_refusal{assert_eq!(error.primary.message(),row["underreportRefusal"].as_str().unwrap());}assert_eq!(cx.original_work_units_spent()-before,u128::from(work));Ok(())
 })).unwrap();caller.retire(stock);}
}
