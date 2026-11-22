//! ✏️ PDF 1.4 subset editor root: expands the shared PDF 1.4 subset editor kit.

use crate::editor::pdf14::modes::edit;
use crate::editor::pdf14::modes::edit::windows::main;
use crate::editor::pdf14::page;

semio_s_artifact_stdio_pdf_editor_kit::pdf14_editor!(Pdf14EditorCommand, Pdf14Editor, PDF14_DIALECT, create_pdf14_editor, PDF14_EDIT_MODE_ID, "*", "any", "s.stdio.pdf@1.4", "s.stdio.pdf@1.4/*#editor", "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/4️⃣1.4/🪆️subsets/🧱️base/✏️editor/🦀️.rs", "stdio-pdf-1-4-base-snapshot-edit");
