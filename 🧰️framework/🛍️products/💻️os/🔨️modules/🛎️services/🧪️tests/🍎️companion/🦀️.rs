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

#[path = "../🔬️file-watcher-unit"]
mod file_watcher {
    pub use semio_framework_os_services::testkit::file_watcher::*;
    use semio_framework_async::WorkerPool;
    use std::sync::atomic::Ordering;
    use std::sync::Arc;
    use std::time::{Duration, Instant, UNIX_EPOCH};

    #[path = "🦀️.rs"]
    mod tests;
}

#[path = "../🔬️native-io-unit"]
mod native_io {
    pub use semio_framework_os_services::testkit::native_io::*;
    use semio_framework_job::{InteractiveJob, InteractiveJobCloseStep, RetainedCloneGrant, RetainedCloneProgress};
    use semio_framework_job_worker::OriginalWorkerFixtureCaller;
    use semio_framework_value::RetirementDemand;
    use std::path::PathBuf;

    #[path = "🦀️.rs"]
    mod tests;
}
