//! 🏛️ Plugin SDK ownership: bounded lifecycle steps and grants, SDK faults, worker session admission, store lifecycle and disposers, and transient publication.
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;

/// 🧮️ The heap witness this crate's allocation laws read through `semio_framework_trace::observe_heap_allocations_on_this_thread`.
#[cfg(test)]
#[global_allocator]
static OWNERSHIP_HEAP_WITNESS: semio_framework_trace::HeapWitness = semio_framework_trace::HeapWitness;

#[path = "../🦀️.rs"]
pub mod bounded_ownership;
#[path = "../../👷️job/🚪️session/🦀️.rs"]
pub mod reserved_job_session;
#[path = "../../📤️fault/🦀️.rs"]
pub mod sdk_fault;
#[path = "../../♻️lifecycle/🦀️.rs"]
pub mod store_lifecycle;
#[path = "../../🫧️transient/🧵️publication/🦀️.rs"]
pub mod transient_publication;

#[path="../../🌱️initialization/🌱️genesis/🦀️.rs"]
pub mod original_store_genesis;
