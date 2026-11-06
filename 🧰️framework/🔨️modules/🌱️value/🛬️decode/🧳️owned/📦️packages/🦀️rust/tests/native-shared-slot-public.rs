//! 🔐️ Original shared slots are exercised through their actual public receiver.
pub use semio_framework_value_native_owned::*;
pub use semio_framework_value_native_owned::retained_clone::{RetainedCloneGrant,RetainedCloneProgress,RetainedCloneRef,RetainedCloneStep};
pub use semio_framework_value_retirement::retirement::RetireOwned;
pub use semio_framework_value_native_owned::shared_issuance::{SealedShared,SharedIssuer};
#[path="../../../🔐️shared-slots/🧪️tests/public.rs"]mod laws;

pub use semio_framework_value_testing_admission as source_fixture;
