//! 🔗️ Qualified member-read custody laws.
pub use semio_plugin_decl_views::child_view::child_editor::read::{ChildEditorRead, custody::*};
pub use semio_plugin_decl_views::child_view::child_editor::source::{ChildEditorCapability, ChildEditorStamp};
pub use semio_framework_value::retained_clone::{RetainedCloneGrant, RetainedCloneProgress, RetainedCloneRef, RetainedCloneStep, retained_clone_grant_admits};
pub use semio_framework_value_snapshot_read::ErasedSnapshotRead;
pub use semio_framework_value::{RetirementDemand, ValueError, ValueRefusalKind};

#[path = "../../../🎯️editor/📖️read/🔗️custody/🧪️tests/🦀️.rs"]
mod tests;
