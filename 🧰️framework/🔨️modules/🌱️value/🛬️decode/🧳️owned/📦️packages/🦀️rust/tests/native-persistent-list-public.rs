//! 📋️ Public persistent-row laws exercise actual receiving and every cancelled prefix.
pub use semio_framework_value_native_owned::*;
pub use semio_framework_value_native_owned::owned::*;
pub use semio_framework_value_native_owned::retained_clone::{RetainedCloneGrant,RetainedCloneProgress,RetainedCloneRef,RetainedCloneStep,RetainedCloneSource,retained_clone_grant_admits};
pub use semio_framework_value_retirement::retirement::RetireOwned;
pub use semio_framework_value_derive::{RetireOwned,NativeReceive};
#[path="../../../../../🧪️testing/📥️admission/receiver-driver.rs"]mod driver;
pub fn observe_retirement_allocations<T>(f:impl FnOnce()->T)->(T,(usize,usize)){semio_framework_value_testing_allocator::observe_retirement_allocations(f)}
#[path="../../../📋️list/🧪️tests/public.rs"]mod laws;
