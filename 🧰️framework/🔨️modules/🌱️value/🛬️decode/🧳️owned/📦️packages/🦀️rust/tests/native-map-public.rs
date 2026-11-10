//! 🗂️ Public map receiving and original-alias laws execute the actual library.
pub use semio_framework_value_native_owned::*;
pub use semio_framework_value_native_owned::owned::{map::*,tree::*};
pub use semio_framework_value_native_owned::retained_clone::{RetainedCloneGrant,RetainedCloneProgress,RetainedCloneRef,RetainedCloneStep};
pub use semio_framework_value_retirement::retirement::RetireOwned;
pub use semio_framework_value_ordered::OrderedMap;
#[path="../../../../../🧪️testing/📥️admission/receiver-driver.rs"]mod driver;
pub fn observe_retirement_allocations<T>(f:impl FnOnce()->T)->(T,(usize,usize)){semio_framework_value_testing_allocator::observe_retirement_allocations(f)}
#[path="../../../🗂️map/tests.rs"]mod laws;
