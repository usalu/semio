//! 🎪 `stdio.html` artifact — new-format artifact (master plan "New format artifacts" table).

use semio_framework_plugin::{ArtifactKindSpec, Dialect, MediaClass, MediaForm, MediaType, OsMediaCapability, StandardId, SubsetId};

pub use crate::artifacts::html::standards::v5::subsets::any::schema::diff::HtmlDiff;
pub use crate::artifacts::html::standards::v5::subsets::any::schema::mutations::HtmlMutation;
pub use crate::artifacts::html::standards::v5::subsets::any::schema::snapshot::HtmlSnapshot;
pub use crate::artifacts::html::standards::v5::subsets::any::schema::HtmlArtifact;

/// 🏷️ Document schema / DSL envelope id.
pub const STDIO_HTML_DOCUMENT_SCHEMA: &str = "stdio.html";

/// 🧬️ Artifact schema descriptor id.
pub const HTML_ARTIFACT_SCHEMA_ID: &str = "s.stdio.html";

//#region 🔖️Dialect
/// 🪪️ Surface coordinate(s) for this artifact — `artifact_kind` matches the schema descriptor
/// id above verbatim (never guessed); `standard`/`subset` match this file's own on-disk
/// `🏅️standards/🔖️.../🪆️subsets/✳️...` location. Lives at the artifact root (not under
/// `editor`/`viewer`) so a viewer file can read it without ever importing through the
/// sibling `editor` module.
pub const HTML_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.html", standard: StandardId("5"), subset: SubsetId("*") };
//#endregion 🔖️Dialect

//#region 🔖️ArtifactKind
/// 🗂️ This artifact's `ArtifactKindSpec`.
pub fn assembly(definition: semio_framework_plugin::ArtifactDefinition) -> Result<crate::registry::ArtifactAssembly, semio_framework_plugin::PluginAssemblyError> {
    crate::registry::definition_only_assembly("html", definition)
}

pub fn artifact_kind() -> ArtifactKindSpec {
    ArtifactKindSpec {
        id: "stdio.html".into(),
        name: "Html".into(),
        source_format: STDIO_HTML_DOCUMENT_SCHEMA.into(),
        component_kind: "stdio".into(),
        dimension: "data".into(),
        media_capability: OsMediaCapability::MeshOnly,
        media_type: MediaType { class: MediaClass::Data, form: MediaForm::Value },
        schema: STDIO_HTML_DOCUMENT_SCHEMA.into(),
        export_formats: vec![],
        import_formats: vec![],
        export_stdio_kinds: vec![],
        import_stdio_kinds: vec![],
    }
}
//#endregion 🔖️ArtifactKind
//#region 🚪️DerivedIoRegistry
pub mod io_registry {
    use crate::artifacts::html::standards::v5::subsets::any::io::io_registry as std_composer;
    use semio_framework_plugin::{register_composer_entries, ComposeError, ComposedArtifact, ComposerEntry, Dialect, ErasedComposeSource};
    use std::sync::OnceLock;

    static ENTRIES: OnceLock<Vec<&'static ComposerEntry>> = OnceLock::new();

    pub fn entries() -> &'static [&'static ComposerEntry] {
        ENTRIES.get_or_init(|| std_composer::entries().iter().collect()).as_slice()
    }

    pub fn compose(target: Dialect, sources: &[ErasedComposeSource]) -> Result<ComposedArtifact, ComposeError> {
        let entry = entries().iter().find(|e| e.writes == target).ok_or_else(|| ComposeError { message: format!("HtmlComposer: no entry writes {:?}", target), diagnostics: Vec::new() })?;
        (entry.compose)(sources)
    }

    pub fn register() {
        let _ = register_composer_entries(std_composer::entries());
    }
}
//#endregion 🚪️DerivedIoRegistry
