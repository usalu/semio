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
