//! 🧾️ The mounted receipt laws: byte owners, operation framing, host receipts and the child document handle issuer.
pub use semio_framework_plugin_publication_mounted::mounted_child_group_receipt::*;
use semio_framework::kernel::ArtifactHandle;
use semio_framework::kernel::ArtifactVersion;
use semio_framework::kernel::HybridLogicalTimestamp;
use semio_framework::kernel::MutationId;
use semio_framework::kernel::UndoPolicy;
use semio_framework_value::ValueError;
use semio_framework_value::ValueRefusalKind;
use semio_framework_value::retained_clone::RetainedCloneGrant;
use semio_framework_value::retained_clone::RetainedCloneProgress;

include!("../../../🧾️receipt/🧪️tests/🦀️.rs");
