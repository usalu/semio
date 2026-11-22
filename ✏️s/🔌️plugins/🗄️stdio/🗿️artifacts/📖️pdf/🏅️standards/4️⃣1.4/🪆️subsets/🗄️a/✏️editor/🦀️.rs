//! ✏️ PDF 1.4 subset editor root: expands the shared PDF 1.4 subset editor kit.

use crate::editor::pdf14a::modes::edit;
use crate::editor::pdf14a::modes::edit::windows::main;
use crate::editor::pdf14::page;

semio_s_artifact_stdio_pdf_editor_kit::pdf14_editor!(Pdf14AEditorCommand, Pdf14AEditor, PDF14A_DIALECT, create_pdf14_a_editor, PDF14A_EDIT_MODE_ID, "a", "a", "s.stdio.pdf@1.4/a", "s.stdio.pdf@1.4/a#editor", "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/4️⃣1.4/🪆️subsets/🗄️a/✏️editor/🦀️.rs", "stdio-pdf-1-4-a-snapshot-edit");
