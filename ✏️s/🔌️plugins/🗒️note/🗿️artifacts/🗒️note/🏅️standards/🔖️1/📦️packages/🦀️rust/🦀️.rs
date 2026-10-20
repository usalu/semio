//! 🔖️ v1 of `semio-s-artifact-note-note` — a path-preserving carve of the artifact root mounts.

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;

pub use semio_s_artifact_note_note_schema::NoteSnapshot;
pub use crate::standards::v1::subsets::any::schema;
pub use crate::schema::mutations::NoteMutation;
pub mod op {
    pub use crate::standards::v1::subsets::any::schema::mutations::NoteMutation;
}
pub use semio_s_artifact_note_note_schema::{NOTE_DOCUMENT_SCHEMA, NoteBlockNode, NoteCamera, NoteImageAsset, NoteTableCell, NoteTextParagraph, NoteTextRun, note_block_text, note_text_child_record};
pub use crate::standards::v1::artifact_items_2::{NOTE_DIALECT};
pub use crate::standards::v1::artifact_items_3::{artifact_kind};
pub use crate::standards::v1::artifact_items_6::{note_child_restore_projection};

#[path = "🪞️mirror/🦀️.rs"]
mod mirror;
pub use mirror::*;
