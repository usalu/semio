//! 📄️ PDF/X Document (1.7) editor root: expands the shared PDF 1.7 subset editor kit.

use crate::editor::pdf17x::modes::edit;
use crate::editor::pdf17x::modes::edit::windows::main;

semio_s_artifact_stdio_pdf_editor_kit::pdf17_editor!(Pdf17XEditorCommand, Pdf17XEditor, PDF17X_DIALECT, create_pdf17_x_editor, PDF17X_EDIT_MODE_ID, "x", "s.stdio.pdf@1.7/x", "s.stdio.pdf@1.7/x#editor", "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/🖨️x/✏️editor/🦀️.rs", "stdio-pdf-1-7-x-snapshot-edit", <crate::standards::v1_7::subsets::x::io::PdfXBuilderConstruction as semio_plugin::ArtifactBuilder>::build(<crate::standards::v1_7::subsets::x::io::PdfXBuilderConstruction as semio_plugin::ArtifactBuilder>::empty()).map_err(|error|semio_plugin::reference_birth_fault("authored blank X owner must satisfy its profile",format_args!("{error:?}"))));
