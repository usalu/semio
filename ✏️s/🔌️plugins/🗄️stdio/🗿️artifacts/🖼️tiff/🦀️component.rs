//! 🎪 `stdio.tiff` artifact — stdio reference format.

use semio_framework_plugin::{ArtifactKindSpec, MediaClass, MediaForm, MediaType, OsMediaCapability};

pub use crate::artifacts::tiff::schema::snapshot::TiffSnapshot;
pub use crate::artifacts::tiff::schema::TiffArtifact;
pub use crate::artifacts::tiff::schema::diff::TiffDiff;
pub use crate::artifacts::tiff::schema::mutations::TiffMutation;

/// 🏷️ Document schema / DSL envelope id.
pub const STDIO_TIFF_DOCUMENT_SCHEMA: &str = "stdio.tiff";

/// 🧬️ Artifact schema descriptor id.
pub const TIFF_ARTIFACT_SCHEMA_ID: &str = "s.stdio.tiff";

//#region 🔖️ArtifactKind
/// 🗂️ This artifact's `ArtifactKindSpec`.
pub fn artifact_kind() -> ArtifactKindSpec {
    ArtifactKindSpec {
        id: "stdio.tiff".into(),
        name: "Tiff".into(),
        source_format: STDIO_TIFF_DOCUMENT_SCHEMA.into(),
        component_kind: "stdio".into(),
        dimension: "data".into(),
        media_capability: OsMediaCapability::MeshOnly,
        media_type: MediaType { class: MediaClass::Data, form: MediaForm::Value },
        schema: STDIO_TIFF_DOCUMENT_SCHEMA.into(),
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
    use crate::artifacts::tiff::standards::v6_0::subsets::any::engine::io_registry as v6_0;

    static ENTRIES: OnceLock<Vec<&'static ComposerEntry>> = OnceLock::new();

    pub fn entries() -> &'static [&'static ComposerEntry] {
        ENTRIES.get_or_init(|| v6_0::entries().iter().collect()).as_slice()
    }

    pub fn compose(target: Dialect, sources: &[ErasedComposeSource]) -> Result<ComposedArtifact, ComposeError> {
        let entry = entries()
            .iter()
            .find(|e| e.writes == target)
            .ok_or_else(|| ComposeError { message: format!("TiffComposer: no entry writes {:?}", target), diagnostics: Vec::new() })?;
        (entry.compose)(sources)
    }

    pub fn register() {
        register_composer_entries(v6_0::entries());
    }
}
//#endregion 🚪️DerivedIoRegistry
