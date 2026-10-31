//! 📚️ Native field and payload declarations use the same public production ports in their owning package.
pub use semio_framework_value_native_owned::*;
pub use semio_framework_value_native_owned::retained_clone::{RetainedCloneGrant,RetainedCloneProgress,RetainedCloneRef,RetainedCloneStep,RetainedCloneClose,RetainedCloneCursor,retained_clone_grant_admits};
pub use semio_framework_value_native_owned::owned::{NativeIntrinsicSharedField,NativeDefault,NativeDefaultCursor};
pub use semio_framework_value_native_owned::owned::tree;
pub use semio_framework_value_native_owned::retirement::RetireOwned;
pub use semio_framework_value_derive::{RetireOwned,NativeReceive,ToValue};
pub use semio_framework_value_intrinsic::{ToValue,NativeEncodeControl,ValueShape};
pub fn observe_retirement_allocations<R>(run:impl FnOnce()->R)->(R,(usize,usize)){semio_framework_value_testing_allocator::observe_retirement_allocations(run)}
#[path="../🦀️.rs"]mod fields;
#[path="../../../../✨️derive/🛬️receive/🔀️enum/🧪️tests/🦀️.rs"]mod payloads;

#[path="../📥️admission/🦀️.rs"]pub(crate) mod source_fixture;
