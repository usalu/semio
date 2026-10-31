//! 🤝️ Qualified admission provider laws.
pub use semio_framework_plugin_decl_views::child_view::child_editor::read::admission::*;
pub use semio_framework_plugin_decl_views::child_view::child_editor::read::custody::{ChildEditorSourceCustody, ChildReadSourceCustody};
pub use semio_framework_plugin_decl_views::child_view::child_editor::{registration::ChildEditorRegistration, source::ChildEditorStamp};
pub use semio_framework_value::retained_clone::{RetainedCloneGrant, RetainedCloneProgress, retained_clone_grant_admits};
pub use semio_framework_io_ports::{ChildAdmissionAddress, ChildAdmissionProviderV1, ChildAdmissionQualificationV1};
pub use semio_framework_value_snapshot_read::ErasedSnapshotRead;
pub use semio_framework_value::{RetirementDemand, ValueError, ValueRefusalKind};

#[path = "../../../🎯️editor/📖️read/🧩️admission/🧪️tests.rs"]
mod tests;
