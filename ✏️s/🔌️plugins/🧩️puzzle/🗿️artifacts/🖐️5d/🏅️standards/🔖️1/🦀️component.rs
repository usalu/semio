//! 🏅️ Standard root — `standard() -> StandardDeclaration` (`terra-fleet-trinity-recipe` recipe,
//! `📓️terra-fleet-trinity-recipe-report.md`), mounts subset `any`. `mimes`/`extensions`: no real MIME
//! registration exists for `s.puzzle5d` outside the old `ArtifactDefinition`'s capability rows
//! (`definition()`, kept — debt D1) — that channel only ever claimed a codec id (`puzzle.5d:
//! puzzle5d-play`) and an extension (`puzzle5d-play`, the EDITOR's own play snapshot extension, not
//! the base `Puzzle5dSnapshot`'s own extension), never a mime type.
//! `mimes: ["application/vnd.semio.puzzle5d+json"]` is a documented synthesis (matches `🗒️note`/
//! `🖍️draw`/`🔱️trinity`'s identical documented deviation); `extensions: ["puzzle5d-play"]` is the
//! real, carried-over value from `definition()`'s `s.puzzle5d.codec.document-1` row.

use crate::artifacts::puzzle5d::standards::v1::subsets;
use semio_framework_plugin::app::declarations::{MediaDeclaration, StandardDeclaration};
use semio_framework_plugin::StandardId;

pub async fn standard() -> StandardDeclaration {
    StandardDeclaration { id: StandardId("1"), media: MediaDeclaration { mimes: &["application/vnd.semio.puzzle5d+json"], extensions: &["puzzle5d-play"] }, subsets: vec![subsets::any::subset()] }
}
