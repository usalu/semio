//! 📸️ Presence owns an explicit original native factory independently of relational adapters.

use crate::os_store::{NativeSnapshotDecodeOwner, NativeSnapshotEncodeOwner};
use semio_framework_value::{retirement::RetireOwned, ValueError};

/// 🫴️ Every presence factory retains partial fields in the caller's original native receiving frame.
pub trait ArtifactPresenceSnapshot: RetireOwned {
    fn decode_presence_native(bytes: &[u8], owner: &mut NativeSnapshotDecodeOwner<'_, '_>) -> Result<Self, ValueError> where Self: Sized;
    fn encode_presence_native(&self, owner: &mut NativeSnapshotEncodeOwner<'_, '_>) -> Result<Vec<u8>, ValueError>;
}
