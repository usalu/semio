//! 📄️ PDF/E Document (1.7) editor root: expands the shared PDF 1.7 subset editor kit.

use crate::editor::pdf17e::modes::edit;
use crate::editor::pdf17e::modes::edit::windows::main;

semio_s_artifact_stdio_pdf_editor_kit::pdf17_editor!(Pdf17EEditorCommand, Pdf17EEditor, PDF17E_DIALECT, create_pdf17_e_editor, PDF17E_EDIT_MODE_ID, "e", "s.stdio.pdf@1.7/e", "s.stdio.pdf@1.7/e#editor", "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/📐️e/✏️editor/🦀️.rs", "stdio-pdf-1-7-e-snapshot-edit", Ok(crate::standards::v1_7::subsets::base::io::export::blank_pdf_snapshot()));
