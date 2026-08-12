//! 🎪 `stdio.docx` artifact — stdio reference format.

use semio_framework_plugin::{ArtifactKindSpec, MediaClass, MediaForm, MediaType, OsMediaCapability};

pub use crate::artifacts::docx::schema::snapshot::DocxSnapshot;
pub use crate::artifacts::docx::schema::DocxArtifact;
pub use crate::artifacts::docx::schema::diff::DocxDiff;
pub use crate::artifacts::docx::schema::mutations::DocxMutation;

/// 🏷️ Document schema / DSL envelope id.
pub const STDIO_DOCX_DOCUMENT_SCHEMA: &str = "stdio.docx";

/// 🧬️ Artifact schema descriptor id.
pub const DOCX_ARTIFACT_SCHEMA_ID: &str = "s.stdio.docx";

//#region 🔖️ArtifactKind
/// 🗂️ This artifact's `ArtifactKindSpec`.
pub fn artifact_kind() -> ArtifactKindSpec {
    ArtifactKindSpec {
        id: "stdio.docx".into(),
        name: "Docx".into(),
        source_format: STDIO_DOCX_DOCUMENT_SCHEMA.into(),
        component_kind: "stdio".into(),
        dimension: "data".into(),
        media_capability: OsMediaCapability::MeshOnly,
        media_type: MediaType { class: MediaClass::Data, form: MediaForm::Value },
        schema: STDIO_DOCX_DOCUMENT_SCHEMA.into(),
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
    use crate::artifacts::docx::standards::v_ecma_376::engine::io_registry as v_ecma_376;

    static ENTRIES: OnceLock<Vec<&'static ComposerEntry>> = OnceLock::new();

    pub fn entries() -> &'static [&'static ComposerEntry] {
        ENTRIES.get_or_init(|| v_ecma_376::entries().iter().collect()).as_slice()
    }

    pub fn compose(target: Dialect, sources: &[ErasedComposeSource]) -> Result<ComposedArtifact, ComposeError> {
        let entry = entries()
            .iter()
            .find(|e| e.writes == target)
            .ok_or_else(|| ComposeError { message: format!("DocxComposer: no entry writes {:?}", target), diagnostics: Vec::new() })?;
        (entry.compose)(sources)
    }

    pub fn register() {
        register_composer_entries(v_ecma_376::entries());
    }
}
//#endregion 🚪️DerivedIoRegistry
