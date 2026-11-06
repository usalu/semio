//! ♻️ Public header handoff and physical release preserve the original pointer and skip poisoned callbacks.
pub use semio_framework_value_native_owned::*;
pub use semio_framework_value_native_owned::owned::*;
pub use semio_framework_value_native_owned::retained_clone::*;
pub use semio_framework_value_testing_allocator::observe_retirement_allocations;
pub use semio_framework_value_testing_admission as source_fixture;
#[path="../../../♻️header/🎟️work/🧪️tests/🦀️.rs"]mod laws;
