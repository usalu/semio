//! 📸️ Presence owns an explicit original native factory independently of relational adapters.

use crate::os_store::{NativeSnapshotDecodeOwner, NativeSnapshotEncodeOwner};
use semio_framework_value::{retirement::RetireOwned, FromValue, ToValue, ValueError};

#[path = "🔤️json/🦀️.rs"]
mod json;

/// 🫴️ Every presence factory retains partial fields in the caller's original native receiving frame; the provided methods are the canonical JSON codec of the value projection, so `impl ArtifactPresenceSnapshot for MyPresence {}` is complete for any `ToValue + FromValue + RetireOwned` presence.
pub trait ArtifactPresenceSnapshot: RetireOwned + ToValue + FromValue {
    fn decode_presence_native(bytes: &[u8], owner: &mut NativeSnapshotDecodeOwner<'_, '_>) -> Result<Self, ValueError> where Self: Sized { json::decode_presence_json(bytes, owner) }
    fn encode_presence_native(&self, owner: &mut NativeSnapshotEncodeOwner<'_, '_>) -> Result<Vec<u8>, ValueError> { json::encode_presence_json(self, owner) }
}
