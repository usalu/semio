//! 🎪 `stdio.dwg` artifact — stdio reference format.

use semio_framework_plugin::{ArtifactKindSpec, MediaClass, MediaForm, MediaType, OsMediaCapability};

pub use crate::artifacts::dwg::schema::snapshot::DwgSnapshot;
pub use crate::artifacts::dwg::schema::snapshot::{DwgDecodeStatus, DwgSection, DwgSectionPage};
pub use crate::artifacts::dwg::schema::DwgArtifact;
pub use crate::artifacts::dwg::schema::diff::DwgDiff;
pub use crate::artifacts::dwg::schema::mutations::DwgMutation;

/// 🏷️ Document schema / DSL envelope id.
pub const STDIO_DWG_DOCUMENT_SCHEMA: &str = "stdio.dwg";

/// 🧬️ Artifact schema descriptor id.
pub const DWG_ARTIFACT_SCHEMA_ID: &str = "s.stdio.dwg";

//#region 🔖️ArtifactKind
/// 🗂️ This artifact's `ArtifactKindSpec`.
pub fn artifact_kind() -> ArtifactKindSpec {
    ArtifactKindSpec {
        id: "stdio.dwg".into(),
        name: "Dwg".into(),
        source_format: STDIO_DWG_DOCUMENT_SCHEMA.into(),
        component_kind: "stdio".into(),
        dimension: "data".into(),
        media_capability: OsMediaCapability::MeshOnly,
        media_type: MediaType { class: MediaClass::Data, form: MediaForm::Value },
        schema: STDIO_DWG_DOCUMENT_SCHEMA.into(),
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
    use crate::artifacts::dwg::standards::v_ac1018::engine::io_registry as v_ac1018;
    use crate::artifacts::dwg::standards::v_ac1024::engine::io_registry as v_ac1024;

    static ENTRIES: OnceLock<Vec<&'static ComposerEntry>> = OnceLock::new();

    pub fn entries() -> &'static [&'static ComposerEntry] {
        ENTRIES.get_or_init(|| v_ac1018::entries().iter().chain(v_ac1024::entries().iter()).collect()).as_slice()
    }

    pub fn compose(target: Dialect, sources: &[ErasedComposeSource]) -> Result<ComposedArtifact, ComposeError> {
        let entry = entries()
            .iter()
            .find(|e| e.writes == target)
            .ok_or_else(|| ComposeError { message: format!("DwgComposer: no entry writes {:?}", target), diagnostics: Vec::new() })?;
        (entry.compose)(sources)
    }

    pub fn register() {
        register_composer_entries(v_ac1018::entries());
        register_composer_entries(v_ac1024::entries());
    }
}
//#endregion 🚪️DerivedIoRegistry
