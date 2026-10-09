//! 🧪️ Original error effects remain present after diagnostic conversion and controlled wire transport.
use super::*;
#[global_allocator]
static ORIGINAL_FAULT_HEAP:semio_framework_trace::HeapWitness=semio_framework_trace::HeapWitness;
#[test]
fn original_fault_keeps_actual_failed_progress_through_value_error_conversion_and_wire(){
 let mut original=Vec::<u8>::new();original.try_reserve_exact(64).unwrap();let pointer=original.as_ptr();
 let progress=semio_framework_value::RetainedCloneProgress{copied_items:1,copied_bytes:0,retained_capacity_bytes:original.capacity(),released_bytes:0};
 let error=ValueError::literal(ValueRefusalKind::InvariantViolated,"original reservation failed").with_retained_progress(progress);let fault=error.into_fault();assert_eq!(fault.retained_progress(),progress);assert_eq!(original.as_ptr(),pointer);
 let wire=fault.to_value();let decoded=Fault::from_value(wire.clone()).unwrap();assert_eq!(decoded.retained_progress(),progress);assert_eq!(decoded,fault);
 let mut callback=|_|true;let mut control=semio_framework_value::NativeDecodeControl::new(1048576,&mut callback);let decoded=Fault::from_value_controlled(&wire,&mut control).unwrap();assert_eq!(decoded.retained_progress(),progress);
 let mut callback=|_|true;let mut control=semio_framework_value::NativeEncodeControl::new(1048576,&mut callback);assert_eq!(decoded.to_value_controlled(&mut control).unwrap(),wire);
 let DslValue::Object(mut members)=wire else{panic!("original fault wire")};members.retain(|(key,_)|key!="retainedProgress");assert!(Fault::from_value(DslValue::Object(members)).is_err());
 eprintln!("[DEBUG] original ValueError born64 receipt survives Fault conversion, cold and controlled original wire; missing mandatory receipt refused, same original producer pointer retained");
}

#[test]
fn original_fault_borrows_canonical_refusal_identity_without_allocating_or_changing_receipt(){
 let law:serde_json::Value=serde_json::from_str(include_str!("../🧫️fixtures/⚠️kind/🔣️.json")).unwrap();let progress=semio_framework_value::RetainedCloneProgress{copied_items:1,retained_capacity_bytes:64,..Default::default()};
 for word in law["kinds"].as_array().unwrap(){let word=word.as_str().unwrap();let kind:ValueRefusalKind=serde_json::from_value(serde_json::Value::String(word.to_owned())).unwrap();let fault=ValueError::literal(kind,"original child").with_retained_progress(progress).into_fault();let pointer=fault.param("refusalKind").unwrap().as_ptr();let(value,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||fault.value_refusal_kind());assert_eq!(value,Some(kind));assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));assert_eq!(fault.retained_progress(),progress);assert_eq!(fault.param("refusalKind").unwrap().as_ptr(),pointer);assert_eq!(Fault::from_value(fault.to_value()).unwrap().value_refusal_kind(),Some(kind));}
 for word in [None,Some(""),Some("inventedKind")]{let mut fault=Fault::new(FaultOrigin::Framework,FaultCode::new("original.child"),"same child").with_retained_progress(progress);if let Some(word)=word{fault=fault.with_param("refusalKind",word);}let(value,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||fault.value_refusal_kind());assert_eq!(value,None);assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));assert_eq!(fault.retained_progress(),progress);}
 println!("[DEBUG] Original Fault borrows all8 exact canonical refusal kinds and preserves same param pointer/actual64 receipt; absent and unknown giveNone with getter0/0");
}
