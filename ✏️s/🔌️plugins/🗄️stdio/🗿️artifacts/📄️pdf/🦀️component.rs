//! 🎪 `stdio.pdf` artifact — stdio reference format.

use semio_framework_plugin::{ArtifactKindSpec, MediaClass, MediaForm, MediaType, OsMediaCapability};

pub use crate::artifacts::pdf::schema::snapshot::PdfSnapshot;
pub use crate::artifacts::pdf::schema::PdfArtifact;
pub use crate::artifacts::pdf::schema::diff::PdfDiff;
pub use crate::artifacts::pdf::schema::mutations::PdfMutation;

/// 🏷️ Document schema / DSL envelope id.
pub const STDIO_PDF_DOCUMENT_SCHEMA: &str = "stdio.pdf";

/// 🧬️ Artifact schema descriptor id.
pub const PDF_ARTIFACT_SCHEMA_ID: &str = "s.stdio.pdf";

//#region 🔖️ArtifactKind
/// 🗂️ This artifact's `ArtifactKindSpec`.
pub fn artifact_kind() -> ArtifactKindSpec {
    ArtifactKindSpec {
        id: "stdio.pdf".into(),
        name: "Pdf".into(),
        source_format: STDIO_PDF_DOCUMENT_SCHEMA.into(),
        component_kind: "stdio".into(),
        dimension: "data".into(),
        media_capability: OsMediaCapability::MeshOnly,
        media_type: MediaType { class: MediaClass::Data, form: MediaForm::Value },
        schema: STDIO_PDF_DOCUMENT_SCHEMA.into(),
        export_formats: vec![],
        import_formats: vec![],
            export_stdio_kinds: vec![],
        import_stdio_kinds: vec![],
    }
}
//#endregion 🔖️ArtifactKind
//#region 🚪️DerivedIoRegistry
pub mod io_registry {
    use std::sync::OnceLock;
    use semio_framework_plugin::{ComposerEntry, Dialect, ErasedComposeSource, ComposedArtifact, ComposeError, register_composer_entries};
    use crate::artifacts::pdf::standards::v1_4::engine::io_registry as v1_4;
    use crate::artifacts::pdf::standards::v1_7::engine::io_registry as v1_7;

    static ENTRIES: OnceLock<Vec<&'static ComposerEntry>> = OnceLock::new();

    pub fn entries() -> &'static [&'static ComposerEntry] {
        ENTRIES.get_or_init(|| v1_4::entries().iter().chain(v1_7::entries().iter()).collect()).as_slice()
    }

    pub fn compose(target: Dialect, sources: &[ErasedComposeSource]) -> Result<ComposedArtifact, ComposeError> {
        let entry = entries()
            .iter()
            .find(|e| e.writes == target)
            .ok_or_else(|| ComposeError { message: format!("PdfComposer: no entry writes {:?}", target), diagnostics: Vec::new() })?;
        (entry.compose)(sources)
    }

    pub fn register() {
        register_composer_entries(v1_4::entries());
        register_composer_entries(v1_7::entries());
    }
}
//#endregion 🚪️DerivedIoRegistry
