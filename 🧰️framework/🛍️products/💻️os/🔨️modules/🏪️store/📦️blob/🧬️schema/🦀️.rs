//! 📦️ Shared durable identity and metadata for content-addressed blob storage.

use crate::{FromValue, ToValue};

/// @emoji 📦️ A content-addressed blob's identity + metadata. Never carries the bytes themselves —
/// callers that just put/read a blob already hold those; this is what gets embedded in a document
/// (e.g. an `ArtifactKind::ContentAddressedBlob` field) to reference it durably.
/// 🌱️ serde is carried UNCONDITIONALLY here, not `#[cfg_attr(test, …)]`: `🪐️space/🦀️.rs` serializes a
/// `BlobRef` through `workflow_kernel` at runtime, so gating it breaks the `s` plugin's wasip2 build.
#[derive(Clone, Debug, PartialEq, ToValue, FromValue, serde::Serialize, serde::Deserialize)]
#[value(rename_all = "camelCase", deny_unknown_fields)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct BlobRef {
    pub hash: String,
    pub size: u64,
    pub media_type: String,
}
