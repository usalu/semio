//! 📦️ Sequence storage work laws use the public canonical owners of the same package.
pub use semio_framework_value_native_owned::*;
pub use semio_framework_value_native_owned::retained_clone::{RetainedCloneGrant,RetainedCloneProgress,RetainedCloneRef,RetainedCloneStep,RetainedCloneClose,RetainedCloneCursor,retained_clone_grant_admits};
pub use semio_framework_value_native_owned::retirement::RetireOwned;
pub use semio_framework_value_derive::{RetireOwned,NativeReceive,ToValue};
pub fn observe_retirement_allocations<R>(run:impl FnOnce()->R)->(R,(usize,usize)){semio_framework_value_testing_allocator::observe_retirement_allocations(run)}
pub use semio_framework_value_native_owned::owned::tree::*;
#[path="../../../../🧪️tests/📥️admission/🦀️.rs"]pub(crate) mod source_fixture;
#[path="../🦀️.rs"]mod tests;
