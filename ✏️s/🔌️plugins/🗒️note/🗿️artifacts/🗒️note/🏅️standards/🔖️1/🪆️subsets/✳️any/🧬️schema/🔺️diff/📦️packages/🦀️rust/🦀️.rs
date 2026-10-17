//! 🔺️ diff of `semio-s-artifact-note-note` — a path-preserving carve of the artifact root mounts.

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_schema as framework_schema;

pub use crate::schema::snapshot::NoteSnapshot;
pub use crate::standards::v1::subsets::any::schema;
pub use semio_s_artifact_note_note_schema::{NOTE_DOCUMENT_SCHEMA, NoteBlockNode, NoteCamera, NoteImageAsset, NoteTableCell, NoteTextChild, NoteTextParagraph, NoteTextRun, note_text_child_record};
pub use crate::editor::note::demo_session::artifact_items_5::{INK_CANVAS_DOCUMENT_SCHEMA, entry, ink_wire_blocks_from_note, is_kind, note_block_value_from_ink_wire, note_canvas_document_json};

#[path = "🪞️mirror/🦀️.rs"]
mod mirror;
pub use mirror::*;
