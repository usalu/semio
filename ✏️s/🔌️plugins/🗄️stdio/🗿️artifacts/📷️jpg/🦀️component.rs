//! 🎪 `stdio.jpg` artifact — stdio reference format.

use semio_framework_plugin::{ArtifactKindSpec, MediaClass, MediaForm, MediaType, OsMediaCapability};

pub use crate::artifacts::jpg::schema::snapshot::JpgSnapshot;
pub use crate::artifacts::jpg::schema::JpgArtifact;
pub use crate::artifacts::jpg::schema::diff::JpgDiff;
pub use crate::artifacts::jpg::schema::mutations::JpgMutation;

/// 🏷️ Document schema / DSL envelope id.
pub const STDIO_JPG_DOCUMENT_SCHEMA: &str = "stdio.jpg";

/// 🧬️ Artifact schema descriptor id.
pub const JPG_ARTIFACT_SCHEMA_ID: &str = "s.stdio.jpg";

//#region 🔖️ArtifactKind
/// 🗂️ This artifact's `ArtifactKindSpec`.
pub fn artifact_kind() -> ArtifactKindSpec {
    ArtifactKindSpec {
        id: "stdio.jpg".into(),
        name: "Jpg".into(),
        source_format: STDIO_JPG_DOCUMENT_SCHEMA.into(),
        component_kind: "stdio".into(),
        dimension: "data".into(),
        media_capability: OsMediaCapability::MeshOnly,
        media_type: MediaType { class: MediaClass::Data, form: MediaForm::Value },
        schema: STDIO_JPG_DOCUMENT_SCHEMA.into(),
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
    use crate::artifacts::jpg::standards::v_jfif_1_01::engine::io_registry as v_jfif_1_01;

    static ENTRIES: OnceLock<Vec<&'static ComposerEntry>> = OnceLock::new();

    pub fn entries() -> &'static [&'static ComposerEntry] {
        ENTRIES.get_or_init(|| v_jfif_1_01::entries().iter().collect()).as_slice()
    }

    pub fn compose(target: Dialect, sources: &[ErasedComposeSource]) -> Result<ComposedArtifact, ComposeError> {
        let entry = entries()
            .iter()
            .find(|e| e.writes == target)
            .ok_or_else(|| ComposeError { message: format!("JpgComposer: no entry writes {:?}", target), diagnostics: Vec::new() })?;
        (entry.compose)(sources)
    }

    pub fn register() {
        register_composer_entries(v_jfif_1_01::entries());
    }
}
//#endregion 🚪️DerivedIoRegistry
