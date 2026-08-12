//! 🎪 `stdio.ifc` artifact — stdio reference format.

use semio_framework_plugin::{ArtifactKindSpec, MediaClass, MediaForm, MediaType, OsMediaCapability};

pub use crate::artifacts::ifc::schema::snapshot::IfcSnapshot;
pub use crate::artifacts::ifc::schema::IfcArtifact;
pub use crate::artifacts::ifc::schema::diff::IfcDiff;
pub use crate::artifacts::ifc::schema::mutations::IfcMutation;

/// 🏷️ Document schema / DSL envelope id.
pub const STDIO_IFC_DOCUMENT_SCHEMA: &str = "stdio.ifc";

/// 🧬️ Artifact schema descriptor id.
pub const IFC_ARTIFACT_SCHEMA_ID: &str = "s.stdio.ifc";

//#region 🔖️ArtifactKind
/// 🗂️ This artifact's `ArtifactKindSpec`.
pub fn artifact_kind() -> ArtifactKindSpec {
    ArtifactKindSpec {
        id: "stdio.ifc".into(),
        name: "Ifc".into(),
        source_format: STDIO_IFC_DOCUMENT_SCHEMA.into(),
        component_kind: "stdio".into(),
        dimension: "data".into(),
        media_capability: OsMediaCapability::MeshOnly,
        media_type: MediaType { class: MediaClass::Text, form: MediaForm::Document },
        schema: STDIO_IFC_DOCUMENT_SCHEMA.into(),
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
    use crate::artifacts::ifc::standards::v4::engine::io_registry as v4;
    use crate::artifacts::ifc::standards::v2x3::engine::io_registry as v2x3;

    static ENTRIES: OnceLock<Vec<&'static ComposerEntry>> = OnceLock::new();

    pub fn entries() -> &'static [&'static ComposerEntry] {
        ENTRIES.get_or_init(|| v4::entries().iter().chain(v2x3::entries().iter()).collect()).as_slice()
    }

    pub fn compose(target: Dialect, sources: &[ErasedComposeSource]) -> Result<ComposedArtifact, ComposeError> {
        let entry = entries()
            .iter()
            .find(|e| e.writes == target)
            .ok_or_else(|| ComposeError { message: format!("IfcComposer: no entry writes {:?}", target), diagnostics: Vec::new() })?;
        (entry.compose)(sources)
    }

    pub fn register() {
        register_composer_entries(v4::entries());
        register_composer_entries(v2x3::entries());
    }
}
//#endregion 🚪️DerivedIoRegistry
