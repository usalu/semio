//! 🏅️ Standard root for `s.trinity.rewriting@1` (ticket 26/08/17/CLEAN-ARTIFACT-STANDARD-SUBSET-
//! MECHANISM, fleet-trinity-recipe). Exports `standard() -> StandardDeclaration`, mounting subset
//! `any` (this artifact's only subset).

use crate::artifacts::rewriting::standards::v1::subsets;
use semio_framework_plugin::app::declarations::{MediaDeclaration, StandardDeclaration};
use semio_framework_plugin::StandardId;

/// 🎯️ `mimes` is a documented synthesis — no real MIME registration exists anywhere in the
/// pre-migration code for this artifact (the old `definition()`'s capability rows claim only a
/// codec id `trinity.rewrite.rule:rewriting` and an extension `rewriting`, never a mime type), matching
/// `🗒️note`/`🖍️draw`'s identical documented deviation. `extensions: ["rewriting"]` is the real,
/// carried-over value (the codec row's own claim).
pub fn standard() -> StandardDeclaration<crate::TrinityApps> {
    StandardDeclaration { id: StandardId("1"), media: MediaDeclaration { mimes: &["application/vnd.semio.rewriting+json"], extensions: &["rewriting"] }, subsets: vec![subsets::any::subset()] }
}
