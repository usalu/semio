//! 🏅️ DAG standard root — `s.dag.dag@1`. Mounts the one subset (`any`) and declares this
//! standard's media (design.md §1/§2).

use crate::artifacts::dag::standards::v1::subsets;
use semio_framework_plugin::app::declarations::{MediaDeclaration, StandardDeclaration};
use semio_framework_plugin::StandardId;

/// 🏅️ `mimes` is a documented synthesis, not a literal carry-over — no real MIME registration
/// exists anywhere in the pre-migration code for this artifact (the old `definition()`'s capability
/// rows only ever claimed a codec id `dag.dag` and an extension `dag`, never a mime type; same
/// finding as the `🎬️sequence` pilot). `extensions` IS the real, carried-over value.
pub fn standard() -> StandardDeclaration {
    StandardDeclaration { id: StandardId("1"), media: MediaDeclaration { mimes: &["application/vnd.semio.dag+json"], extensions: &["dag"] }, subsets: vec![subsets::any::subset()] }
}
