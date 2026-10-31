//! 🎛️ Controlled entity identity receiving laws.
pub use semio_framework_os_vcs::*;
pub use semio_framework_value::retained_clone::{RetainedCloneGrant, RetainedCloneProgress, RetainedCloneStep};
pub use semio_framework_value::{RetirementDemand, ValueError, ValueRefusalKind};
pub use std::sync::Arc;
pub use semio_framework_os_vcs::io::binary::entity_identity::control::*;
pub use semio_framework_os_vcs::io::binary::entity_identity::*;
pub use semio_framework_value::{NativeEncodeControl, native_encoding::{NativeEncodeAllocationPort, NativeEncodeContinuation, NativeEncodeProgress, NativeForwardedEncodeContinuation, NativeRetirementEncodeContinuation}};
#[path = "../../../../🚪️io/💾️binary/🪪️entity-identity/🎛️control/🧪️tests/🦀️.rs"]
mod receiving_input_tests;
