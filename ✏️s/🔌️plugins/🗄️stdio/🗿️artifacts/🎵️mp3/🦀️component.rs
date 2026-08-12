//! 🎪 `stdio.mp3` artifact — new-format artifact (master plan "New format artifacts" table).

use semio_framework_plugin::{ArtifactKindSpec, MediaClass, MediaForm, MediaType, OsMediaCapability};

pub use crate::artifacts::mp3::standards::mpeg1_layer3::subsets::any::schema::snapshot::Mp3Snapshot;
pub use crate::artifacts::mp3::standards::mpeg1_layer3::subsets::any::schema::Mp3Artifact;
pub use crate::artifacts::mp3::standards::mpeg1_layer3::subsets::any::schema::diff::Mp3Diff;
pub use crate::artifacts::mp3::standards::mpeg1_layer3::subsets::any::schema::mutations::Mp3Mutation;

/// 🏷️ Document schema / DSL envelope id.
pub const STDIO_MP3_DOCUMENT_SCHEMA: &str = "stdio.mp3";

/// 🧬️ Artifact schema descriptor id.
pub const MP3_ARTIFACT_SCHEMA_ID: &str = "s.stdio.mp3";

//#region 🔖️ArtifactKind
/// 🗂️ This artifact's `ArtifactKindSpec`.
pub fn artifact_kind() -> ArtifactKindSpec {
    ArtifactKindSpec {
        id: "stdio.mp3".into(),
        name: "Mp3".into(),
        source_format: STDIO_MP3_DOCUMENT_SCHEMA.into(),
        component_kind: "stdio".into(),
        dimension: "data".into(),
        media_capability: OsMediaCapability::MeshOnly,
        media_type: MediaType { class: MediaClass::Data, form: MediaForm::Value },
        schema: STDIO_MP3_DOCUMENT_SCHEMA.into(),
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
    use crate::artifacts::mp3::standards::mpeg1_layer3::engine::io_registry as std_composer;

    static ENTRIES: OnceLock<Vec<&'static ComposerEntry>> = OnceLock::new();

    pub fn entries() -> &'static [&'static ComposerEntry] {
        ENTRIES.get_or_init(|| std_composer::entries().iter().collect()).as_slice()
    }

    pub fn compose(target: Dialect, sources: &[ErasedComposeSource]) -> Result<ComposedArtifact, ComposeError> {
        let entry = entries()
            .iter()
            .find(|e| e.writes == target)
            .ok_or_else(|| ComposeError { message: format!("Mp3Composer: no entry writes {:?}", target), diagnostics: Vec::new() })?;
        (entry.compose)(sources)
    }

    pub fn register() {
        register_composer_entries(std_composer::entries());
    }
}
//#endregion 🚪️DerivedIoRegistry
