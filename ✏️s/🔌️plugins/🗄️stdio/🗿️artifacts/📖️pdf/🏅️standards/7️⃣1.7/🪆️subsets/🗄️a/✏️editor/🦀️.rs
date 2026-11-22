//! 📄️ PDF/A Document (1.7) editor root: expands the shared PDF 1.7 subset editor kit.

use crate::editor::pdf17a::modes::edit;
use crate::editor::pdf17a::modes::edit::windows::main;

semio_s_artifact_stdio_pdf_editor_kit::pdf17_editor!(Pdf17AEditorCommand, Pdf17AEditor, PDF17A_DIALECT, create_pdf17_a_editor, PDF17A_EDIT_MODE_ID, "a", "s.stdio.pdf@1.7/a", "s.stdio.pdf@1.7/a#editor", "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/🗄️a/✏️editor/🦀️.rs", "stdio-pdf-1-7-a-snapshot-edit", Ok(crate::standards::v1_7::subsets::base::io::export::blank_pdf_snapshot()));
