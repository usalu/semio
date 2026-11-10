//! 🪞️ Public original projections retain their real parent and every cancelled receiving prefix.
pub use semio_framework_value_native_owned::*;
pub use semio_framework_value_native_owned::owned::*;
pub use semio_framework_value_native_owned::retained_clone::{RetainedCloneGrant,RetainedCloneProgress,RetainedCloneRef,RetainedCloneStep,RetainedCloneSource,retained_clone_grant_admits};
pub use semio_framework_value_retirement::retirement::RetireOwned;
pub use semio_framework_value_derive::{RetireOwned,NativeReceive};
pub fn observe_retirement_allocations<T>(f:impl FnOnce()->T)->(T,(usize,usize)){semio_framework_value_testing_allocator::observe_retirement_allocations(f)}
use crate::shared_issuance::SealedShared;
#[derive(crate::RetireOwned,crate::NativeReceive)]#[value(crate="crate",native_owned="crate")]struct Row{value:SealedShared<String>}
#[path="../../../../../🧪️testing/📥️admission/receiver-driver.rs"]mod driver;
#[path="../../../🧳️original/🪞️projection/🧪️tests/driven.rs"]mod laws;
