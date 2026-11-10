//! 🌳️ Recursive receiving laws exercise the actual canonical native owner.
pub use semio_framework_value_native_owned::*;
pub use semio_framework_value_native_owned::owned::*;
pub use semio_framework_value_native_owned::retained_clone::{RetainedCloneGrant,RetainedCloneProgress,RetainedCloneRef,RetainedCloneStep};
pub use semio_framework_value_derive::{RetireOwned,NativeReceive};
#[path="../../../../../🧪️testing/📥️admission/receiver-driver.rs"]mod driver;
pub fn observe_retirement_allocations<T>(f:impl FnOnce()->T)->(T,(usize,usize)){semio_framework_value_testing_allocator::observe_retirement_allocations(f)}
#[path="../../../♻️header/🎟️work/🧪️tests/public.rs"]mod laws;
