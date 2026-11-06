//! 🧪️ Public declaration contracts are exercised independently of their production source boundary.
#![cfg(test)]
#[global_allocator]
static TEST_ALLOCATION_OBSERVER:semio_framework_trace::HeapWitness=semio_framework_trace::HeapWitness;
extern crate semio_os_kernel as store;
use semio_framework_diagnostic::{Fault,FaultOrigin};
use semio_plugin_decl_app::completion_fault::*;
use semio_plugin_decl_app::media_contract::media_original_read_matches;
#[path="../../🛬️lifted/⚠️fault/🧪️tests/🦀️.rs"]
mod completion_fault_laws;
#[path="../../🛬️lifted/🎬️media-export/📖️read/🧪️tests.rs"]
mod original_media_read_laws;

#[test]
fn allocation_observer_records_real_birth_and_release(){
 let(bytes,birth)=semio_framework_trace::observe_heap_allocations_on_this_thread(||std::hint::black_box(vec![3_u8;17]));
 assert_eq!((birth.requested_bytes,birth.released_bytes),(17,0));
 let((),release)=semio_framework_trace::observe_heap_allocations_on_this_thread(||drop(std::hint::black_box(bytes)));
 assert_eq!((release.requested_bytes,release.released_bytes),(0,17));
}
