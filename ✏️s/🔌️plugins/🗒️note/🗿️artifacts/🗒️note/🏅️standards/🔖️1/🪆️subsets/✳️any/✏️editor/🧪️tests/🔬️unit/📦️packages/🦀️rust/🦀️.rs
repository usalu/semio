//! 🔬️ unit-tests of `semio-s-artifact-note-note` — a path-preserving carve of the artifact root mounts.

extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;

#[cfg(test)]
pub use crate::schema::snapshot::NoteSnapshot;
#[cfg(test)]
pub use crate::standards::v1::subsets::any::schema;
#[cfg(test)]
pub use crate::schema::mutations::NoteMutation;
#[cfg(test)]
pub mod op {
    pub use crate::standards::v1::subsets::any::schema::mutations::NoteMutation;
}
pub use semio_s_artifact_note_note_schema::{NOTE_DOCUMENT_SCHEMA, NoteBlockNode, NoteCamera, NoteImageAsset};
pub use semio_s_artifact_note_note_diff::{note_block_value_from_ink_wire, note_canvas_document_json};

#[path = "🪞️mirror/🦀️.rs"]
mod mirror;
pub use mirror::*;
