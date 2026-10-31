//! 🧪️ Original pin-order laws.
pub use semio_framework_os_vcs::*;
pub use semio_framework_value::retained_clone::{RetainedCloneGrant, RetainedCloneProgress, RetainedCloneStep};
pub use semio_framework_value::{RetirementDemand, ValueError, ValueRefusalKind};
pub use std::sync::Arc;
pub use semio_framework_os_vcs::io::binary::checkpoint_identity::original::pin_order::*;
pub use semio_framework_os_vcs::io::binary::checkpoint_identity::original::*;
#[path = "../../../../../🚪️io/💾️binary/🪪️checkpoint-identity/🎟️original/pin-order-tests.rs"]
mod tests;
