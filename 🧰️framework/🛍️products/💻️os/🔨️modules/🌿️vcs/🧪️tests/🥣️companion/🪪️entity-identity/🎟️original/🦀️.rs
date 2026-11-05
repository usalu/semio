//! 🎟️ Original entity identity laws.
pub use semio_os_vcs::*;
pub use semio_framework_value::retained_clone::{RetainedCloneGrant, RetainedCloneProgress, RetainedCloneStep};
pub use semio_framework_value::{RetirementDemand, ValueError, ValueRefusalKind};
pub use std::sync::Arc;
pub use semio_os_vcs::io::binary::entity_identity::original::*;
pub use semio_os_vcs::io::binary::entity_identity::*;
pub use semio_framework_value::{NativeEncodeControl, native_encoding::{NativeEncodeAllocationPort, NativeEncodeContinuation, NativeEncodeProgress, NativeForwardedEncodeContinuation, NativeRetirementEncodeContinuation}};
#[path = "../../../../🚪️io/💾️binary/🪪️entity-identity/🎟️original/tests.rs"]
mod tests;
