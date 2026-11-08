//! 💾️ Both physical observers must weigh the one allocator's same successful system operations.
use super::*;
#[test]
fn canonical_allocator_records_same_physical_events_for_both_observers(){
 let fixture:serde_json::Value=serde_json::from_str(include_str!("./🧫️fixtures/🔣️.json")).unwrap();let initial=fixture["initialBytes"].as_u64().unwrap()as usize;let grown=fixture["grownBytes"].as_u64().unwrap()as usize;
 let(((),physical),trace)=semio_framework_trace::observe_heap_allocations_on_this_thread(||observe_retirement_allocations(||{let mut bytes=Vec::<u8>::with_capacity(initial);bytes.resize(initial,0);bytes.reserve_exact(grown-initial);assert_eq!(bytes.capacity(),grown);drop(bytes)}));
 assert_eq!(physical,(fixture["expected"]["requested"].as_u64().unwrap()as usize,fixture["expected"]["released"].as_u64().unwrap()as usize));assert_eq!(physical,(trace.requested_bytes,trace.released_bytes));assert_eq!(trace.largest_release_bytes,fixture["expected"]["largestRelease"].as_u64().unwrap()as usize);assert!(!trace.overflowed);
}
#[test]
fn canonical_allocator_keeps_nested_trace_observations(){
 let (((),physical),outer)=semio_framework_trace::observe_heap_allocations_on_this_thread(||observe_retirement_allocations(||{let((),inner)=semio_framework_trace::observe_heap_allocations_on_this_thread(||drop(Vec::<u8>::with_capacity(17)));assert_eq!((inner.requested_bytes,inner.released_bytes),(17,17));}));assert_eq!(physical,(17,17));assert_eq!(physical,(outer.requested_bytes,outer.released_bytes));assert!(!outer.overflowed);
}

#[test]
fn canonical_allocator_preserves_requested_trace_and_retirement_observers_with_real_null(){
 let law:serde_json::Value=serde_json::from_str(include_str!("🎟️requested/case.json")).unwrap();let schema:serde_json::Value=serde_json::from_str(include_str!("🎟️requested/case.schema.json")).unwrap();for(k,v)in law.as_object().unwrap(){assert_eq!(&schema["properties"][k]["const"],v);}
 for row in law["cases"].as_array().unwrap(){
  let bytes=row["bytes"].as_u64().unwrap()as usize;let refused=row["refuse"].as_bool().unwrap();let expected=row["expectedBirth"].as_u64().unwrap()as usize;
  let(((result,trace),physical),requested,released)=semio_framework_trace_memory_testing::observe_backing(||observe_retirement_allocations(||semio_framework_trace::observe_heap_allocations_on_this_thread(||{
   let allocate=||{let mut value=Vec::<u8>::new();value.try_reserve_exact(bytes)?;value.resize(bytes,7);Ok::<_,std::collections::TryReserveError>(value)};
   if refused{let(result,attempts)=refuse_one_allocation_on_this_thread(allocate);assert_eq!(attempts,1);result}else{allocate()}
  })));
  assert_eq!(physical,(expected,0));assert_eq!((requested,released),physical);assert_eq!((trace.requested_bytes,trace.released_bytes),physical);assert!(!trace.overflowed);
  if refused{assert!(result.is_err());assert!(row["expectedValue"].is_null());}else{let value=result.unwrap();assert_eq!(serde_json::to_value(&value).unwrap(),row["expectedValue"]);let((((),trace),physical),requested,released)=semio_framework_trace_memory_testing::observe_backing(||observe_retirement_allocations(||semio_framework_trace::observe_heap_allocations_on_this_thread(||drop(value))));assert_eq!(physical,(0,bytes));assert_eq!((requested,released),physical);assert_eq!((trace.requested_bytes,trace.released_bytes),physical);assert!(!trace.overflowed);}
 }
}

#[test]
fn canonical_allocator_releases_original_observer_after_unwind(){
 let law:serde_json::Value=serde_json::from_str(include_str!("unwind.json")).unwrap();
 let interrupted=std::panic::catch_unwind(||observe_retirement_allocations(||panic!("original physical observation interrupted")));assert!(interrupted.is_err());assert!(RETIREMENT_ALLOCATION_EVENTS.with(|events|events.get().is_none()));
 let bytes=law["afterBytes"].as_u64().unwrap()as usize;let(((),physical),trace)=semio_framework_trace::observe_heap_allocations_on_this_thread(||observe_retirement_allocations(||drop(Vec::<u8>::with_capacity(bytes))));assert_eq!(physical,(bytes,bytes));assert_eq!(physical,(trace.requested_bytes,trace.released_bytes));assert!(!trace.overflowed);
}
#[test]
fn canonical_allocator_refuses_nested_retirement_scope_without_replacing_original(){
 let law:serde_json::Value=serde_json::from_str(include_str!("unwind.json")).unwrap();let bytes=law["afterBytes"].as_u64().unwrap()as usize;
 let(((),physical),trace)=semio_framework_trace::observe_heap_allocations_on_this_thread(||observe_retirement_allocations(||{drop(Vec::<u8>::with_capacity(bytes));let nested=std::panic::catch_unwind(||observe_retirement_allocations(||panic!("nested observer must not enter")));assert!(nested.is_err());assert!(RETIREMENT_ALLOCATION_EVENTS.with(|events|events.get().is_some()));drop(Vec::<u8>::with_capacity(bytes));}));assert!(physical.0>=bytes*2&&physical.1>=bytes*2);assert_eq!(physical,(trace.requested_bytes,trace.released_bytes));assert!(!trace.overflowed);
}
