//! 📄️ PDF/VT Document (1.7) editor root: expands the shared PDF 1.7 subset editor kit.

use crate::editor::pdf17vt::modes::edit;
use crate::editor::pdf17vt::modes::edit::windows::main;

semio_s_artifact_stdio_pdf_editor_kit::pdf17_editor!(Pdf17VtEditorCommand, Pdf17VtEditor, PDF17VT_DIALECT, create_pdf17_vt_editor, PDF17VT_EDIT_MODE_ID, "vt", "s.stdio.pdf@1.7/vt", "s.stdio.pdf@1.7/vt#editor", "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/🧾️vt/✏️editor/🦀️.rs", "stdio-pdf-1-7-vt-snapshot-edit", <crate::standards::v1_7::subsets::vt::io::PdfVtBuilderConstruction as semio_plugin::ArtifactBuilder>::build(<crate::standards::v1_7::subsets::vt::io::PdfVtBuilderConstruction as semio_plugin::ArtifactBuilder>::empty()).map_err(|error|semio_plugin::reference_birth_fault("authored blank VT owner must satisfy its profile",format_args!("{error:?}"))));
