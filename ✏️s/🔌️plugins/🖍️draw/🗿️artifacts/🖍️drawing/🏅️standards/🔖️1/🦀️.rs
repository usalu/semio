//! 🏅️ Standard root for `s.draw.drawing@1` (ticket 26/08/17/CLEAN-ARTIFACT-STANDARD-SUBSET-
//! MECHANISM). Exports `standard() -> StandardDeclaration`, mounting subset `any` (this artifact's
//! only subset).

use crate::artifacts::drawing::standards::v1::subsets;
use semio_framework_plugin::app::declarations::{MediaDeclaration, StandardDeclaration};
use semio_framework_plugin::StandardId;

/// 🎯️ `mimes` is a documented synthesis — no real MIME registration exists anywhere in the
/// pre-migration code for this artifact (the old `definition()`'s capability rows claim only a
/// codec id `drawing.document:drawing` and an extension `drawing`, never a mime type; `artifact_kind()`'s
/// `MediaType`/`OsMediaCapability` fields are a coarser, unrelated classification — see
/// `📓️recipe-subset.md` §4b). `extensions: ["drawing"]` is the real, carried-over value (the codec
/// row's own claim).
pub fn standard() -> StandardDeclaration<crate::DrawApps> {
    StandardDeclaration { id: StandardId("1"), media: MediaDeclaration { mimes: &["application/vnd.semio.drawing+json"], extensions: &["drawing"] }, subsets: vec![subsets::any::subset()] }
}
