//! ✏️ PDF 1.4 subset editor root: expands the shared PDF 1.4 subset editor kit.

use crate::editor::pdf14x::modes::edit;
use crate::editor::pdf14x::modes::edit::windows::main;
use crate::editor::pdf14::page;

semio_s_artifact_stdio_pdf_editor_kit::pdf14_editor!(Pdf14XEditorCommand, Pdf14XEditor, PDF14X_DIALECT, create_pdf14_x_editor, PDF14X_EDIT_MODE_ID, "x", "x", "s.stdio.pdf@1.4/x", "s.stdio.pdf@1.4/x#editor", "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/4️⃣1.4/🪆️subsets/🖨️x/✏️editor/🦀️.rs", "stdio-pdf-1-4-x-snapshot-edit");
