//! 🏅️ Standard root — `standard() -> StandardDeclaration` (`terra-fleet-trinity-recipe` recipe,
//! `📓️terra-fleet-trinity-recipe-report.md`), mounts subset `any`. `mimes`/`extensions`: no real MIME
//! registration exists for `s.puzzle3d` outside the old `ArtifactDefinition`'s capability rows
//! (`definition()`, kept — debt D1) — that channel only ever claimed a codec id
//! (`puzzle.3d.fixture:puzzle3d-play`) and an extension (`puzzle3d-play`, the EDITOR's own play
//! snapshot extension, not the base `Puzzle3dSnapshot`'s own extension), never a mime type.
//! `mimes: ["application/vnd.semio.puzzle3d+json"]` is a documented synthesis (matches `🗒️note`/
//! `🖍️draw`/`🔱️trinity`'s identical documented deviation); `extensions: ["puzzle3d-play"]` is the
//! real, carried-over value from `definition()`'s `s.puzzle3d.codec.document-1` row.

use crate::artifacts::puzzle3d::standards::v1::subsets;
use semio_framework_plugin::app::declarations::{MediaDeclaration, StandardDeclaration};
use semio_framework_plugin::StandardId;

pub fn standard() -> StandardDeclaration<crate::PuzzleApps> {
    StandardDeclaration { id: StandardId("1"), media: MediaDeclaration { mimes: &["application/vnd.semio.puzzle3d+json"], extensions: &["puzzle3d-play"] }, subsets: vec![subsets::any::subset()] }
}
