//! 🎪 `stdio.tsv` artifact — new-format artifact (master plan "New format artifacts" table).

use semio_framework_plugin::{ArtifactKindSpec, MediaClass, MediaForm, MediaType, OsMediaCapability};

pub use crate::artifacts::tsv::standards::iana::subsets::any::schema::snapshot::TsvSnapshot;
pub use crate::artifacts::tsv::standards::iana::subsets::any::schema::TsvArtifact;
pub use crate::artifacts::tsv::standards::iana::subsets::any::schema::diff::TsvDiff;
pub use crate::artifacts::tsv::standards::iana::subsets::any::schema::mutations::TsvMutation;

/// 🏷️ Document schema / DSL envelope id.
pub const STDIO_TSV_DOCUMENT_SCHEMA: &str = "stdio.tsv";

/// 🧬️ Artifact schema descriptor id.
pub const TSV_ARTIFACT_SCHEMA_ID: &str = "s.stdio.tsv";

//#region 🔖️ArtifactKind
/// 🗂️ This artifact's `ArtifactKindSpec`.
pub fn artifact_kind() -> ArtifactKindSpec {
    ArtifactKindSpec {
        id: "stdio.tsv".into(),
        name: "Tsv".into(),
        source_format: STDIO_TSV_DOCUMENT_SCHEMA.into(),
        component_kind: "stdio".into(),
        dimension: "data".into(),
        media_capability: OsMediaCapability::MeshOnly,
        media_type: MediaType { class: MediaClass::Data, form: MediaForm::Value },
        schema: STDIO_TSV_DOCUMENT_SCHEMA.into(),
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
    use crate::artifacts::tsv::standards::iana::engine::io_registry as std_composer;

    static ENTRIES: OnceLock<Vec<&'static ComposerEntry>> = OnceLock::new();

    pub fn entries() -> &'static [&'static ComposerEntry] {
        ENTRIES.get_or_init(|| std_composer::entries().iter().collect()).as_slice()
    }

    pub fn compose(target: Dialect, sources: &[ErasedComposeSource]) -> Result<ComposedArtifact, ComposeError> {
        let entry = entries()
            .iter()
            .find(|e| e.writes == target)
            .ok_or_else(|| ComposeError { message: format!("TsvComposer: no entry writes {:?}", target), diagnostics: Vec::new() })?;
        (entry.compose)(sources)
    }

    pub fn register() {
        register_composer_entries(std_composer::entries());
    }
}
//#endregion 🚪️DerivedIoRegistry
