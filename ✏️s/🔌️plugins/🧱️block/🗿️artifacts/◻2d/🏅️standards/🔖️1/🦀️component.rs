//! 🏅️ Standard root for `s.block.block2d@1` (ticket 26/08/17/MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME,
//! `descriptor-prep`, following `🔱️trinity`'s `fleet-trinity-recipe`). Exports
//! `standard() -> StandardDeclaration`, mounting subset `any` (this artifact's only subset).

use crate::artifacts::block2d::standards::v1::subsets;
use semio_framework_plugin::app::declarations::{MediaDeclaration, StandardDeclaration};
use semio_framework_plugin::StandardId;

/// 🎯️ `mimes` is a documented synthesis — no real MIME registration exists anywhere in the
/// pre-migration `definition()` (its capability rows claim only a codec id `block.2d:block2d` and an
/// extension `block2d`, never a mime type), matching `🗒️note`/`🖍️draw`/`🔱️trinity`'s identical
/// documented deviation. `extensions: ["block2d"]` is the real, carried-over value (`definition()`'s
/// own `s.block2d.codec.document-1` row's `extension` claim, D2-capability-claim-repairs comment).
pub async fn standard() -> StandardDeclaration {
    StandardDeclaration { id: StandardId("1"), media: MediaDeclaration { mimes: &["application/vnd.semio.block2d+json"], extensions: &["block2d"] }, subsets: vec![subsets::any::subset()] }
}
