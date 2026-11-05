//! 🧪️ Original repin-facts laws.
pub use semio_os_vcs::*;
pub use semio_framework_value::retained_clone::{RetainedCloneGrant, RetainedCloneProgress, RetainedCloneStep};
pub use semio_framework_value::{RetirementDemand, ValueError, ValueRefusalKind};
pub use std::sync::Arc;
pub use semio_os_vcs::io::binary::checkpoint_identity::original::repin_facts::*;
pub use semio_os_vcs::io::binary::checkpoint_identity::original::*;
#[path = "../../../../../🚪️io/💾️binary/🪪️checkpoint-identity/🎟️original/repin-facts-tests.rs"]
mod tests;
