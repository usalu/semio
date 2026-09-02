//! 🏅️ Standard root for `s.trinity.jack@1` (ticket 26/08/17/CLEAN-ARTIFACT-STANDARD-SUBSET-
//! MECHANISM, fleet-trinity-recipe). Exports `standard() -> StandardDeclaration`, mounting subset
//! `any` (this artifact's only subset).

use crate::artifacts::jack::standards::v1::subsets;
use semio_framework_plugin::app::declarations::{MediaDeclaration, StandardDeclaration};
use semio_framework_plugin::StandardId;

/// 🎯️ `mimes` is a documented synthesis — no real MIME registration exists anywhere in the
/// pre-migration code for this artifact (the old `definition()`'s capability rows claim only a
/// codec id `trinity.graph:trinity` and an extension `trinity`, never a mime type — see that
/// function's own D2-capability-claim-repairs comment for why the extension is `trinity`, not
/// `jack`), matching `🗒️note`/`🖍️draw`'s identical documented deviation. `extensions: ["trinity"]`
/// is the real, carried-over value (the codec row's own claim).
pub fn standard() -> StandardDeclaration<crate::TrinityApps> {
    StandardDeclaration { id: StandardId("1"), media: MediaDeclaration { mimes: &["application/vnd.semio.jack+json"], extensions: &["trinity"] }, subsets: vec![subsets::any::subset()] }
}
