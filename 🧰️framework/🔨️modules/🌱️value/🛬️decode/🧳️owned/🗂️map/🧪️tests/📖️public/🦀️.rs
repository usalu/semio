//! 🗂️ Native map receiving laws use the public canonical owners of the same package.
pub use semio_framework_value_native_owned::*;
pub use semio_framework_value_native_owned::retained_clone::{RetainedCloneGrant,RetainedCloneProgress,RetainedCloneRef,RetainedCloneStep,RetainedCloneClose,RetainedCloneCursor,retained_clone_grant_admits};
pub use semio_framework_value_native_owned::retirement::RetireOwned;
pub use semio_framework_value_derive::{RetireOwned,NativeReceive,ToValue};
pub fn observe_retirement_allocations<R>(run:impl FnOnce()->R)->(R,(usize,usize)){semio_framework_value_testing_allocator::observe_retirement_allocations(run)}
#[path="../../../🧪️tests/📥️admission/🦀️.rs"]pub(crate) mod source_fixture;
#[path="."]pub mod owned{pub use semio_framework_value_native_owned::owned::*;pub mod tree{pub use semio_framework_value_native_owned::owned::tree::*;}#[path="."]pub mod map{pub use semio_framework_value_native_owned::*;pub use semio_framework_value_native_owned::owned::map::*;pub use semio_framework_value_native_owned::owned::tree::NativeValueSource;pub use semio_framework_value_ordered::OrderedMap;pub use semio_framework_value_native_owned::retirement::RetireOwned;pub fn invalid()->semio_framework_value_native_owned::ValueError{semio_framework_value_native_owned::ValueError::literal(semio_framework_value_native_owned::ValueRefusalKind::InvalidValue,"native map fixture key")}#[path="../../tests.rs"]mod tests;}}
