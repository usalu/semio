//! 🌐️ `stdio.semio` artifact root — the inbuilt semio semantic artifact (standard `v1`, 13
//! schema-owning domain subsets + the `✳️any` envelope union). See the master plan's
//! "Architecture > The semio artifact" section.

use semio_framework_plugin::{ArtifactKindSpec, MediaClass, MediaForm, MediaType, OsMediaCapability};

pub use crate::artifacts::semio::standards::v1::subsets::any::schema::snapshot::SemioSnapshot;
pub use crate::artifacts::semio::standards::v1::subsets::any::schema::SemioArtifact;
pub use crate::artifacts::semio::standards::v1::subsets::any::schema::diff::SemioDiff;
pub use crate::artifacts::semio::standards::v1::subsets::any::schema::mutations::SemioMutation;

/// 🏷️ Document schema / DSL envelope id.
pub const STDIO_SEMIO_DOCUMENT_SCHEMA: &str = "stdio.semio";

/// 🧬️ Artifact schema descriptor id.
pub const SEMIO_ARTIFACT_SCHEMA_ID: &str = "s.stdio.semio";

//#region 🔖️ArtifactKind
/// 🗂️ This artifact's `ArtifactKindSpec`.
pub fn artifact_kind() -> ArtifactKindSpec {
    ArtifactKindSpec {
        id: "stdio.semio".into(),
        name: "Semio".into(),
        source_format: STDIO_SEMIO_DOCUMENT_SCHEMA.into(),
        component_kind: "stdio".into(),
        dimension: "data".into(),
        media_capability: OsMediaCapability::MeshOnly,
        media_type: MediaType { class: MediaClass::Data, form: MediaForm::Value },
        schema: STDIO_SEMIO_DOCUMENT_SCHEMA.into(),
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
    use crate::artifacts::semio::standards::v1::engine::io_registry as v1;

    static ENTRIES: OnceLock<Vec<&'static ComposerEntry>> = OnceLock::new();

    pub fn entries() -> &'static [&'static ComposerEntry] {
        ENTRIES.get_or_init(|| v1::entries().iter().collect()).as_slice()
    }

    pub fn compose(target: Dialect, sources: &[ErasedComposeSource]) -> Result<ComposedArtifact, ComposeError> {
        let entry = entries()
            .iter()
            .find(|e| e.writes == target)
            .ok_or_else(|| ComposeError { message: format!("SemioComposer: no entry writes {:?}", target), diagnostics: Vec::new() })?;
        (entry.compose)(sources)
    }

    pub fn register() {
        register_composer_entries(v1::entries());
    }
}
//#endregion 🚪️DerivedIoRegistry
