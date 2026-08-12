//! 🎪 `stdio.gltf` artifact — stdio reference format.

use semio_framework_plugin::{ArtifactKindSpec, MediaClass, MediaForm, MediaType, OsMediaCapability};

pub use crate::artifacts::gltf::schema::snapshot::GltfSnapshot;
pub use crate::artifacts::gltf::schema::GltfArtifact;
pub use crate::artifacts::gltf::schema::diff::GltfDiff;
pub use crate::artifacts::gltf::schema::mutations::GltfMutation;

/// 🏷️ Document schema / DSL envelope id.
pub const STDIO_GLTF_DOCUMENT_SCHEMA: &str = "stdio.gltf";

/// 🧬️ Artifact schema descriptor id.
pub const GLTF_ARTIFACT_SCHEMA_ID: &str = "s.stdio.gltf";

//#region 🔖️ArtifactKind
/// 🗂️ This artifact's `ArtifactKindSpec`.
pub fn artifact_kind() -> ArtifactKindSpec {
    ArtifactKindSpec {
        id: "stdio.gltf".into(),
        name: "Gltf".into(),
        source_format: STDIO_GLTF_DOCUMENT_SCHEMA.into(),
        component_kind: "stdio".into(),
        dimension: "data".into(),
        media_capability: OsMediaCapability::MeshOnly,
        media_type: MediaType { class: MediaClass::Data, form: MediaForm::Value },
        schema: STDIO_GLTF_DOCUMENT_SCHEMA.into(),
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
    use crate::artifacts::gltf::standards::v2_0::engine::io_registry as v2_0;

    static ENTRIES: OnceLock<Vec<&'static ComposerEntry>> = OnceLock::new();

    pub fn entries() -> &'static [&'static ComposerEntry] {
        ENTRIES.get_or_init(|| v2_0::entries().iter().collect()).as_slice()
    }

    pub fn compose(target: Dialect, sources: &[ErasedComposeSource]) -> Result<ComposedArtifact, ComposeError> {
        let entry = entries()
            .iter()
            .find(|e| e.writes == target)
            .ok_or_else(|| ComposeError { message: format!("GltfComposer: no entry writes {:?}", target), diagnostics: Vec::new() })?;
        (entry.compose)(sources)
    }

    pub fn register() {
        register_composer_entries(v2_0::entries());
    }
}
//#endregion 🚪️DerivedIoRegistry
