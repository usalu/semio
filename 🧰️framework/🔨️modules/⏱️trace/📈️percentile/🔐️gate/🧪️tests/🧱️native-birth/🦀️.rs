//! 🧱️ Cold static telemetry backings keep zero native birth, measured through the one testing allocator.
use semio_framework_value_testing_allocator as _;

#[test]
fn original_static_telemetry_recording_cold_backings_preserve_zero_native_birth(){
 let law:serde_json::Value=serde_json::from_str(include_str!("../🧫️fixtures/🔣️.json")).unwrap();
 for owner in law["backingOwners"].as_array().unwrap(){
  let owner=owner.as_str().unwrap();
  let ((),heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||match owner{
   "percentile"=>{let verdict=semio_framework_trace::Watchdog::start_at("original.static-native-percentile",semio_framework_trace::OperationId(919),semio_framework_trace::Generation(3),semio_framework_trace::InteractiveStage::InteractiveStep,Some(0)).finish_at(Some(1));assert_eq!(verdict.elapsed_us(),Some(1));assert!(!verdict.is_fault());},
   "trace"=>{let event=semio_framework_trace::record_trace_event_at(semio_framework_trace::OperationId(919),semio_framework_trace::Generation(3),semio_framework_trace::TraceStage::Committed,Some(1)).unwrap();assert_eq!(event.operation,semio_framework_trace::OperationId(919));assert_eq!(event.generation,semio_framework_trace::Generation(3));},
   "violation"=>{let verdict=semio_framework_trace::Watchdog::start_at("original.static-native-violation",semio_framework_trace::OperationId(919),semio_framework_trace::Generation(3),semio_framework_trace::InteractiveStage::InteractiveStep,Some(0)).finish_at(Some(9001));assert_eq!(verdict.elapsed_us(),Some(9001));assert!(verdict.violation().is_some());},
   _=>panic!("unknown original static telemetry owner")
  });
  assert_eq!((heap.requested_bytes,heap.released_bytes),(law["heapBirthBytes"].as_u64().unwrap()as usize,0),"{owner}");
 }
}
