//! 🎪 `stdio.xml` artifact — stdio reference format.

use semio_framework_plugin::{ArtifactKindSpec, MediaClass, MediaForm, MediaType, OsMediaCapability};

pub use crate::artifacts::xml::schema::snapshot::XmlSnapshot;
pub use crate::artifacts::xml::schema::XmlArtifact;
pub use crate::artifacts::xml::schema::diff::XmlDiff;
pub use crate::artifacts::xml::schema::mutations::XmlMutation;

/// 🏷️ Document schema / DSL envelope id.
pub const STDIO_XML_DOCUMENT_SCHEMA: &str = "stdio.xml";

/// 🧬️ Artifact schema descriptor id.
pub const XML_ARTIFACT_SCHEMA_ID: &str = "s.stdio.xml";

//#region 🔖️ArtifactKind
/// 🗂️ This artifact's `ArtifactKindSpec`.
pub fn artifact_kind() -> ArtifactKindSpec {
    ArtifactKindSpec {
        id: "stdio.xml".into(),
        name: "Xml".into(),
        source_format: STDIO_XML_DOCUMENT_SCHEMA.into(),
        component_kind: "stdio".into(),
        dimension: "data".into(),
        media_capability: OsMediaCapability::MeshOnly,
        media_type: MediaType { class: MediaClass::Data, form: MediaForm::Document },
        schema: STDIO_XML_DOCUMENT_SCHEMA.into(),
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
    use crate::artifacts::xml::standards::v1_0::engine::io_registry as v1_0;

    static ENTRIES: OnceLock<Vec<&'static ComposerEntry>> = OnceLock::new();

    pub fn entries() -> &'static [&'static ComposerEntry] {
        ENTRIES.get_or_init(|| v1_0::entries().iter().collect()).as_slice()
    }

    pub fn compose(target: Dialect, sources: &[ErasedComposeSource]) -> Result<ComposedArtifact, ComposeError> {
        let entry = entries()
            .iter()
            .find(|e| e.writes == target)
            .ok_or_else(|| ComposeError { message: format!("XmlComposer: no entry writes {:?}", target), diagnostics: Vec::new() })?;
        (entry.compose)(sources)
    }

    pub fn register() {
        register_composer_entries(v1_0::entries());
    }
}
//#endregion 🚪️DerivedIoRegistry
