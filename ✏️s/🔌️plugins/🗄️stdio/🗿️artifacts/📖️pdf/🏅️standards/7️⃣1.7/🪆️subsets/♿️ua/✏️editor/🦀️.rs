//! 📄️ PDF/UA Document (1.7) editor root: expands the shared PDF 1.7 subset editor kit.

use crate::editor::pdf17ua::modes::edit;
use crate::editor::pdf17ua::modes::edit::windows::main;

semio_s_artifact_stdio_pdf_editor_kit::pdf17_editor!(Pdf17UaEditorCommand, Pdf17UaEditor, PDF17UA_DIALECT, create_pdf17_ua_editor, PDF17UA_EDIT_MODE_ID, "ua", "s.stdio.pdf@1.7/ua", "s.stdio.pdf@1.7/ua#editor", "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/♿️ua/✏️editor/🦀️.rs", "stdio-pdf-1-7-ua-snapshot-edit", <crate::standards::v1_7::subsets::ua::io::PdfUaBuilderConstruction as semio_plugin::ArtifactBuilder>::build(crate::standards::v1_7::subsets::ua::io::PdfUaBuilderConstruction::new("und")).map_err(|error|semio_plugin::reference_birth_fault("authored blank UA owner must satisfy its profile",format_args!("{error:?}"))));
