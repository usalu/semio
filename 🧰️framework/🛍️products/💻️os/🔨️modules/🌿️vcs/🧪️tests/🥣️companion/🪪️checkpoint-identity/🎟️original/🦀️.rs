//! 🎟️ Original checkpoint identity laws.
pub use semio_framework_os_vcs::*;
pub use semio_framework_value::retained_clone::{RetainedCloneGrant, RetainedCloneProgress, RetainedCloneStep};
pub use semio_framework_value::{RetirementDemand, ValueError, ValueRefusalKind};
pub use std::sync::Arc;
pub use semio_framework_os_vcs::io::binary::checkpoint_identity::original::*;
pub use super::*;
#[path = "../../../../🚪️io/💾️binary/🪪️checkpoint-identity/🎟️original/tests.rs"]
mod tests;
#[path = "../../../../🚪️io/💾️binary/🪪️checkpoint-identity/🎟️original/commit-identity-tests.rs"]
mod commit_identity_tests;
#[path = "commit-facts/🦀️.rs"]
pub mod commit_facts;
#[path = "existing/🦀️.rs"]
pub mod existing;
#[path = "pin-order/🦀️.rs"]
pub mod pin_order;
#[path = "repin-alternative/🦀️.rs"]
pub mod repin_alternative;
#[path = "repin-facts/🦀️.rs"]
pub mod repin_facts;
