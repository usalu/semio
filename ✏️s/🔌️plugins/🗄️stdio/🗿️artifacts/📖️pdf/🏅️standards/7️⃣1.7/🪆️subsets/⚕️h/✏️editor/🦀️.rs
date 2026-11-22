//! 📄️ PDF/H Document (1.7) editor root: expands the shared PDF 1.7 subset editor kit.

use crate::editor::pdf17h::modes::edit;
use crate::editor::pdf17h::modes::edit::windows::main;

semio_s_artifact_stdio_pdf_editor_kit::pdf17_editor!(Pdf17HEditorCommand, Pdf17HEditor, PDF17H_DIALECT, create_pdf17_h_editor, PDF17H_EDIT_MODE_ID, "h", "s.stdio.pdf@1.7/h", "s.stdio.pdf@1.7/h#editor", "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/⚕️h/✏️editor/🦀️.rs", "stdio-pdf-1-7-h-snapshot-edit", Ok(crate::standards::v1_7::subsets::base::io::export::blank_pdf_snapshot()));
