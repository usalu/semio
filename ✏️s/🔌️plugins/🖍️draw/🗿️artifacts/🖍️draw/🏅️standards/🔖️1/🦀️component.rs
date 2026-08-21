//! 🏅️ Standard root for `s.draw.draw@1` (ticket 26/08/17/CLEAN-ARTIFACT-STANDARD-SUBSET-
//! MECHANISM). Exports `standard() -> StandardDeclaration`, mounting subset `any` (this artifact's
//! only subset).

use crate::artifacts::draw::standards::v1::subsets;
use semio_framework_plugin::app::declarations::{MediaDeclaration, StandardDeclaration};
use semio_framework_plugin::StandardId;

/// 🎯️ `mimes` is a documented synthesis — no real MIME registration exists anywhere in the
/// pre-migration code for this artifact (the old `definition()`'s capability rows claim only a
/// codec id `draw.document:draw` and an extension `draw`, never a mime type; `artifact_kind()`'s
/// `MediaType`/`OsMediaCapability` fields are a coarser, unrelated classification — see
/// `📓️recipe-subset.md` §4b). `extensions: ["draw"]` is the real, carried-over value (the codec
/// row's own claim).
pub async fn standard() -> StandardDeclaration {
    StandardDeclaration { id: StandardId("1"), media: MediaDeclaration { mimes: &["application/vnd.semio.draw+json"], extensions: &["draw"] }, subsets: vec![subsets::any::subset()] }
}
