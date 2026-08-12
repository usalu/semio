//! 🎪 `stdio.deflate` artifact — stdio reference format.

use semio_framework_plugin::{ArtifactKindSpec, MediaClass, MediaForm, MediaType, OsMediaCapability};

pub use crate::artifacts::deflate::schema::snapshot::DeflateSnapshot;
pub use crate::artifacts::deflate::schema::DeflateArtifact;
pub use crate::artifacts::deflate::schema::diff::DeflateDiff;
pub use crate::artifacts::deflate::schema::mutations::DeflateMutation;

/// 🏷️ Document schema / DSL envelope id.
pub const STDIO_DEFLATE_DOCUMENT_SCHEMA: &str = "stdio.deflate";

/// 🧬️ Artifact schema descriptor id.
pub const DEFLATE_ARTIFACT_SCHEMA_ID: &str = "s.stdio.deflate";

//#region 🔖️ArtifactKind
/// 🗂️ This artifact's `ArtifactKindSpec`.
pub fn artifact_kind() -> ArtifactKindSpec {
    ArtifactKindSpec {
        id: "stdio.deflate".into(),
        name: "Deflate".into(),
        source_format: STDIO_DEFLATE_DOCUMENT_SCHEMA.into(),
        component_kind: "stdio".into(),
        dimension: "data".into(),
        media_capability: OsMediaCapability::MeshOnly,
        media_type: MediaType { class: MediaClass::Data, form: MediaForm::Value },
        schema: STDIO_DEFLATE_DOCUMENT_SCHEMA.into(),
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
    use crate::artifacts::deflate::standards::v_rfc1950::engine::io_registry as v_rfc1950;

    static ENTRIES: OnceLock<Vec<&'static ComposerEntry>> = OnceLock::new();

    pub fn entries() -> &'static [&'static ComposerEntry] {
        ENTRIES.get_or_init(|| v_rfc1950::entries().iter().collect()).as_slice()
    }

    pub fn compose(target: Dialect, sources: &[ErasedComposeSource]) -> Result<ComposedArtifact, ComposeError> {
        let entry = entries()
            .iter()
            .find(|e| e.writes == target)
            .ok_or_else(|| ComposeError { message: format!("DeflateComposer: no entry writes {:?}", target), diagnostics: Vec::new() })?;
        (entry.compose)(sources)
    }

    pub fn register() {
        register_composer_entries(v_rfc1950::entries());
    }
}
//#endregion 🚪️DerivedIoRegistry
