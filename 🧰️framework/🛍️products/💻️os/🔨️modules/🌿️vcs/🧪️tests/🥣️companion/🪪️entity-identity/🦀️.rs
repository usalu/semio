//! 🪪️ Entity identity laws.
pub use semio_os_vcs::*;
pub use semio_framework_value::retained_clone::{RetainedCloneGrant, RetainedCloneProgress, RetainedCloneStep};
pub use semio_framework_value::{RetirementDemand, ValueError, ValueRefusalKind};
pub use std::sync::Arc;
pub use semio_os_vcs::io::binary::entity_identity::*;
#[path = "🎛️control/🦀️.rs"]
pub mod control;
#[path = "🎟️original/🦀️.rs"]
pub mod original;
