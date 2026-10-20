//! 🙈️ tests-hides-the-grid of `semio-s-artifact-note-note` — a path-preserving carve of the artifact root mounts.

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;

pub use crate::standards::v1::subsets::any::schema;
pub use semio_s_artifact_note_note_schema::NoteSnapshot;
#[cfg(test)]
pub use semio_s_artifact_note_note_diff::standards::v1::subsets::any::schema::diff::NoteDiff;
pub mod op {
    pub use crate::standards::v1::subsets::any::schema::mutations::NoteMutation;
}
pub use semio_s_artifact_note_note_schema::{NOTE_DOCUMENT_SCHEMA, NoteBlockNode, NoteCamera, NoteImageAsset, NoteTextParagraph, NoteTextRun};
pub use semio_s_artifact_note_note_diff::{note_block_value_from_ink_wire};

#[path = "🪞️mirror/🦀️.rs"]
mod mirror;
pub use mirror::*;
