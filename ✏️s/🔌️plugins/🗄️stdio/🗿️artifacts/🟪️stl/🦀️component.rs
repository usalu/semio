//! 🎪 `stdio.stl` artifact — stdio reference format.

use semio_framework_plugin::{ArtifactKindSpec, MediaClass, MediaForm, MediaType, OsMediaCapability};

pub use crate::artifacts::stl::schema::snapshot::StlSnapshot;
pub use crate::artifacts::stl::schema::StlArtifact;
pub use crate::artifacts::stl::schema::diff::StlDiff;
pub use crate::artifacts::stl::schema::mutations::StlMutation;

/// 🏷️ Document schema / DSL envelope id.
pub const STDIO_STL_DOCUMENT_SCHEMA: &str = "stdio.stl";

/// 🧬️ Artifact schema descriptor id.
pub const STL_ARTIFACT_SCHEMA_ID: &str = "s.stdio.stl";

//#region 🔖️ArtifactKind
/// 🗂️ This artifact's `ArtifactKindSpec`.
pub fn artifact_kind() -> ArtifactKindSpec {
    ArtifactKindSpec {
        id: "stdio.stl".into(),
        name: "Stl".into(),
        source_format: STDIO_STL_DOCUMENT_SCHEMA.into(),
        component_kind: "stdio".into(),
        dimension: "data".into(),
        media_capability: OsMediaCapability::MeshOnly,
        media_type: MediaType { class: MediaClass::Text, form: MediaForm::Document },
        schema: STDIO_STL_DOCUMENT_SCHEMA.into(),
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
    use crate::artifacts::stl::standards::v_ascii::engine::io_registry as v_ascii;

    static ENTRIES: OnceLock<Vec<&'static ComposerEntry>> = OnceLock::new();

    pub fn entries() -> &'static [&'static ComposerEntry] {
        ENTRIES.get_or_init(|| v_ascii::entries().iter().collect()).as_slice()
    }

    pub fn compose(target: Dialect, sources: &[ErasedComposeSource]) -> Result<ComposedArtifact, ComposeError> {
        let entry = entries()
            .iter()
            .find(|e| e.writes == target)
            .ok_or_else(|| ComposeError { message: format!("StlComposer: no entry writes {:?}", target), diagnostics: Vec::new() })?;
        (entry.compose)(sources)
    }

    pub fn register() {
        register_composer_entries(v_ascii::entries());
    }
}
//#endregion 🚪️DerivedIoRegistry
