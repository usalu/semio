//! 🏅️ Standard root — `standard() -> StandardDeclaration` (design.md §2), mounts subset `any`.
//! `mimes`/`extensions`: no real MIME/extension registration exists for `s.note.note` outside the
//! old `ArtifactDefinition`'s capability rows (`definition()`, kept — debt D1) — that channel only
//! ever claimed a codec id (`note.document:note`) and an extension (`note`), never a mime type.
//! `mimes: ["application/vnd.semio.note+json"]` is a documented synthesis (see the fan-out report's
//! `## openQuestions`, matching the sequence pilot's identical documented deviation);
//! `extensions: ["note"]` is the real, carried-over value.

use crate::artifacts::note::standards::v1::subsets;
use semio_framework_plugin::app::declarations::{MediaDeclaration, StandardDeclaration};
use semio_framework_plugin::StandardId;

pub async fn standard() -> StandardDeclaration {
    StandardDeclaration { id: StandardId("1"), media: MediaDeclaration { mimes: &["application/vnd.semio.note+json"], extensions: &["note"] }, subsets: vec![subsets::any::subset()] }
}
