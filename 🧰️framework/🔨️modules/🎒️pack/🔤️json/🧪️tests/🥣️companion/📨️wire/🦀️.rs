//! 📨️ The canonical tree wire cursor laws.
pub use semio_framework_pack_json::*;
use semio_framework_value::{RetainedCloneGrant, RetainedCloneProgress, ValueError};
use semio_framework_value::retirement::RetireOwned;
use semio_framework_value::retirement::controlled::ControlledRetirement;
pub use semio_framework_pack_json::testkit::tree::wire::*;
use crate::{ArtifactCanonicalJsonTree as Tree, ArtifactCanonicalJsonTreeCursor};

#[path = "../../../🛫️encode/🧭️tree/📨️wire/🧪️tests/🦀️.rs"]
mod tests;
