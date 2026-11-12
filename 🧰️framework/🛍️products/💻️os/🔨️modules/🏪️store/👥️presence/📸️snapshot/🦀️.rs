//! 📸️ Presence owns an explicit original native factory independently of relational adapters.

use semio_framework_io_control::NativeSnapshotDecodeOwner;
use semio_framework_io_control::NativeSnapshotEncodeOwner;
use semio_framework_io_control::NativeSnapshotFailure;
use semio_framework_value::RetainedWorkOutcome;
use semio_framework_value::retirement::RetireOwned;
use semio_framework_value::ToValue;

#[path = "🔤️json/🦀️.rs"]
mod json;
pub use json::decode_presence_json;

/// 🫴️ A presence format installs its actual original receiving port and preserves all partial native owners.
pub trait ArtifactPresenceSnapshot: RetireOwned + ToValue {
    fn decode_presence_native(bytes:&[u8],owner:&mut NativeSnapshotDecodeOwner<'_,'_>)->RetainedWorkOutcome<Self,NativeSnapshotFailure> where Self:Sized;
    fn encode_presence_native(&self,owner:&mut NativeSnapshotEncodeOwner<'_,'_>)->RetainedWorkOutcome<Vec<u8>,NativeSnapshotFailure> where Self:Sized {json::encode_presence_json(self,owner)}
}

#[cfg(test)]
use semio_framework_value_testing_allocator as _;
