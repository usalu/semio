//! 🍎️ Companion tests of os-services: host runtime, timer wheel, compute pool, HTTP pool, storage scheduler, event router, file watcher and native IO.
//!
//! 🧭️ The tests reach the private service modules through the `testkit` hooks of `semio_framework_os_services`, never through the production crate's own module tree.
#![cfg(test)]

#[global_allocator]
static ORIGINAL_SERVICES_HEAP: semio_framework_trace::HeapWitness = semio_framework_trace::HeapWitness;

#[path = "../🔬️component-unit"]
mod component {
    pub use semio_framework_os_services::testkit::{completion_sink, compute_pool, event_router, fixed_file_storage, host_runtime, http_contract, http_pool, http_socket, storage_scheduler, timer_wheel};

    #[path = "🦀️.rs"]
    mod tests;
}
