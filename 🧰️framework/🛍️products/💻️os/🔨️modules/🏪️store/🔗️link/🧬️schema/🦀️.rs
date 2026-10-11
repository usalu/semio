//! 🔗️ Shared artifact references and their history pin authority.

use crate::{BlobRef, FromValue, ToValue};

/// 🔗️ An independent-lifecycle reference to another artifact: a PIN (so it can be frozen to
/// a specific point in the target's history) plus a `role` (the named slot it fills on the
/// referencing artifact, e.g. `"cover-image"`). Renders as a chip, never nests inline — the
/// structural opposite of `ArtifactChild`; see the region doc's CHILD-vs-LINK split.
#[derive(Clone, Debug, PartialEq, ToValue, FromValue, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[value(rename_all = "camelCase", deny_unknown_fields)]
pub struct ArtifactLink {
    pub target: semio_framework_artifact_reference::ArtifactRef,
    pub pin: LinkPin,
    pub role: String,
}

/// 📌️ What an `ArtifactLink` is frozen to: nothing (`Head`, always the target's live tip),
/// a specific `Checkpoint`, or a content-addressed `Snapshot` blob (survives even the target
/// document's own history being pruned/GC'd, since the bytes are escrowed independently).
#[derive(Clone, Debug, PartialEq, ToValue, FromValue, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[value(tag = "kind", rename_all = "camelCase", deny_unknown_fields)]
#[cfg_attr(test, serde(tag = "kind", rename_all = "camelCase", deny_unknown_fields))]
pub enum LinkPin {
    Head,
    Checkpoint { id: String },
    Snapshot { blob: BlobRef },
}
