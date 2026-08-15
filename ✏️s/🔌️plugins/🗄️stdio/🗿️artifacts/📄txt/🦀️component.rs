//! 🎪 `stdio.txt` artifact — stdio reference format.

use semio_framework_plugin::{ArtifactKindSpec, MediaClass, MediaForm, MediaType, OsMediaCapability};

pub use crate::artifacts::txt::schema::diff::TxtDiff;
pub use crate::artifacts::txt::schema::mutations::TxtMutation;
pub use crate::artifacts::txt::schema::snapshot::TxtSnapshot;
pub use crate::artifacts::txt::schema::TxtArtifact;

/// 🏷️ Document schema / DSL envelope id.
pub const STDIO_TXT_DOCUMENT_SCHEMA: &str = "stdio.txt";

/// 🧬️ Artifact schema descriptor id.
pub const TXT_ARTIFACT_SCHEMA_ID: &str = "s.stdio.txt";

//#region 🔖️ArtifactKind
/// 🗂️ This artifact's `ArtifactKindSpec`.
pub fn artifact_kind() -> ArtifactKindSpec {
    ArtifactKindSpec {
        id: "stdio.txt".into(),
        name: "Txt".into(),
        source_format: STDIO_TXT_DOCUMENT_SCHEMA.into(),
        component_kind: "stdio".into(),
        dimension: "data".into(),
        media_capability: OsMediaCapability::MeshOnly,
        media_type: MediaType { class: MediaClass::Text, form: MediaForm::Document },
        schema: STDIO_TXT_DOCUMENT_SCHEMA.into(),
        export_formats: vec![],
        import_formats: vec![],
        export_stdio_kinds: vec![],
        import_stdio_kinds: vec![],
    }
}
//#endregion 🔖️ArtifactKind
//#region 🚪️DerivedIoRegistry
pub mod io_registry {
    use crate::artifacts::txt::standards::v_utf_8::subsets::any::io::io_registry as v_utf_8;
    use semio_framework_plugin::{register_composer_entries, ComposeError, ComposedArtifact, ComposerEntry, Dialect, ErasedComposeSource};
    use std::sync::OnceLock;

    static ENTRIES: OnceLock<Vec<&'static ComposerEntry>> = OnceLock::new();

    pub fn entries() -> &'static [&'static ComposerEntry] {
        ENTRIES.get_or_init(|| v_utf_8::entries().iter().collect()).as_slice()
    }

    pub fn compose(target: Dialect, sources: &[ErasedComposeSource]) -> Result<ComposedArtifact, ComposeError> {
        let entry = entries().iter().find(|e| e.writes == target).ok_or_else(|| ComposeError { message: format!("TxtComposer: no entry writes {:?}", target), diagnostics: Vec::new() })?;
        (entry.compose)(sources)
    }

    pub fn register() {
        register_composer_entries(v_utf_8::entries());
    }
}
//#endregion 🚪️DerivedIoRegistry
