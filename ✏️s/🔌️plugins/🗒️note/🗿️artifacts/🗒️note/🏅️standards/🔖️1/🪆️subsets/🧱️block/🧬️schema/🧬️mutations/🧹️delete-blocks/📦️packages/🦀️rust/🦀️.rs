//! 🧹️ delete-blocks of `semio-s-artifact-note-note` — a path-preserving carve of the artifact root mounts.

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;

pub use crate::schema::snapshot::NoteSnapshot;
pub use crate::schema::diff::NoteDiff;
pub use crate::standards::v1::subsets::any::schema;
pub use semio_s_artifact_note_note_schema::{NoteBlockNode, NoteCamera};
pub use semio_s_artifact_note_note_diff::{note_canvas_document_json};

#[path = "🪞️mirror/🦀️.rs"]
mod mirror;
pub use mirror::*;
