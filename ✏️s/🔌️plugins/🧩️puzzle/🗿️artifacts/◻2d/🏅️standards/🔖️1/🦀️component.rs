//! 🏅️ Standard root — `standard() -> StandardDeclaration` (`terra-fleet-trinity-recipe` recipe,
//! `📓️terra-fleet-trinity-recipe-report.md`), mounts subset `any`. `mimes`/`extensions`: no real MIME
//! registration exists for `s.puzzle2d` outside the old `ArtifactDefinition`'s capability rows
//! (`definition()`, kept — debt D1) — that channel only ever claimed a codec id
//! (`puzzle.2d.fixture:puzzle2d-play`) and an extension (`puzzle2d-play`, the EDITOR's own
//! `Puzzle2dPlaySnapshot::EXTENSION`, not the base `Puzzle2dSnapshot`'s `"puzzle2d"` — see this
//! artifact root's own `definition()` D2 comment), never a mime type.
//! `mimes: ["application/vnd.semio.puzzle2d+json"]` is a documented synthesis (matches `🗒️note`/
//! `🖍️draw`/`🔱️trinity`'s identical documented deviation); `extensions: ["puzzle2d-play"]` is the
//! real, carried-over value from `definition()`'s `s.puzzle2d.codec.document-1` row.

use crate::artifacts::puzzle2d::standards::v1::subsets;
use semio_framework_plugin::app::declarations::{MediaDeclaration, StandardDeclaration};
use semio_framework_plugin::StandardId;

pub async fn standard() -> StandardDeclaration {
    StandardDeclaration { id: StandardId("1"), media: MediaDeclaration { mimes: &["application/vnd.semio.puzzle2d+json"], extensions: &["puzzle2d-play"] }, subsets: vec![subsets::any::subset()] }
}
