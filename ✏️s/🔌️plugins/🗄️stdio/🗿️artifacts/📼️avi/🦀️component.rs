//! 🎪 `stdio.avi` artifact — new-format artifact (master plan "New format artifacts" table).

use semio_framework_plugin::{ArtifactKindSpec, MediaClass, MediaForm, MediaType, OsMediaCapability};

pub use crate::artifacts::avi::standards::v1_0::subsets::any::schema::diff::AviDiff;
pub use crate::artifacts::avi::standards::v1_0::subsets::any::schema::mutations::AviMutation;
pub use crate::artifacts::avi::standards::v1_0::subsets::any::schema::snapshot::AviSnapshot;
pub use crate::artifacts::avi::standards::v1_0::subsets::any::schema::AviArtifact;

/// 🏷️ Document schema / DSL envelope id.
pub const STDIO_AVI_DOCUMENT_SCHEMA: &str = "stdio.avi";

/// 🧬️ Artifact schema descriptor id.
pub const AVI_ARTIFACT_SCHEMA_ID: &str = "s.stdio.avi";

//#region 🔖️ArtifactKind
/// 🗂️ This artifact's `ArtifactKindSpec`.
pub fn artifact_kind() -> ArtifactKindSpec {
    ArtifactKindSpec {
        id: "stdio.avi".into(),
        name: "Avi".into(),
        source_format: STDIO_AVI_DOCUMENT_SCHEMA.into(),
        component_kind: "stdio".into(),
        dimension: "data".into(),
        media_capability: OsMediaCapability::MeshOnly,
        media_type: MediaType { class: MediaClass::Data, form: MediaForm::Value },
        schema: STDIO_AVI_DOCUMENT_SCHEMA.into(),
        export_formats: vec![],
        import_formats: vec![],
        export_stdio_kinds: vec![],
        import_stdio_kinds: vec![],
    }
}
//#endregion 🔖️ArtifactKind
//#region 🔖️Declaration
/// 🔖️ This artifact's declaration (ticket 26/08/12/ARTIFACTS-ONLY-PLUGIN-ARCHITECTURE g5) — replaces
/// the side-effecting `crate::artifacts::avi::standards::v1_0::engine::register()` call the plugin
/// root used to make imperatively. `.composers(...)` reaches this standard's ENGINE-level
/// `io_registry` (below `⚙️engine`, distinct from this file's own `🚪️DerivedIoRegistry` shadow, whose
/// `entries()` returns `&[&ComposerEntry]` — the wrong type for `.composers()`, which wants
/// `&'static [ComposerEntry]`) by its fully qualified path, per the silent-rebind hazard this ticket
/// calls out. `.document_codec_bare::<AviSnapshot, AviMutation>(...)` folds in what
/// `subsets::any::io::register()`'s `store::register_document_codec(store::ArtifactCodec::of::<..>())`
/// call did — avi is a headless stdio artifact with no `ArtifactApp` to bind `.document_codec::<A>()`
/// to.
/// 🧩️ Binds this executable root to its sole schema-owned definition.
pub fn assembly(definition: semio_framework_plugin::ArtifactDefinition) -> Result<crate::registry::ArtifactAssembly, semio_framework_plugin::PluginAssemblyError> {
    crate::registry::runtime_assembly("avi", definition, declaration)
}

pub fn declaration(definition: semio_framework_plugin::ArtifactDefinition) -> Result<semio_framework_plugin::ArtifactDeclaration, semio_framework_plugin::ArtifactDefinitionError> {
    let formats = crate::registry::format_descriptors_for("avi")?;
    semio_framework_plugin::ArtifactDeclaration::builder(definition)
        .schema(crate::artifacts::avi::standards::v1_0::subsets::any::schema::avi_artifact_schema_descriptor())
        .formats(formats)
        .inferences([crate::artifacts::avi::standards::v1_0::subsets::any::schema::inferences::avi_artifact_inference_descriptor()])
        .composers(crate::artifacts::avi::standards::v1_0::subsets::any::io::io_registry::entries())
        .document_codec_bare::<AviSnapshot, AviMutation>(STDIO_AVI_DOCUMENT_SCHEMA)
        .try_build()
}
//#endregion 🔖️Declaration
//#region 🚪️DerivedIoRegistry
pub mod io_registry {
    use crate::artifacts::avi::standards::v1_0::subsets::any::io::io_registry as std_composer;
    use semio_framework_plugin::{register_composer_entries, ComposeError, ComposedArtifact, ComposerEntry, Dialect, ErasedComposeSource};
    use std::sync::OnceLock;

    static ENTRIES: OnceLock<Vec<&'static ComposerEntry>> = OnceLock::new();

    pub fn entries() -> &'static [&'static ComposerEntry] {
        ENTRIES.get_or_init(|| std_composer::entries().iter().collect()).as_slice()
    }

    pub fn compose(target: Dialect, sources: &[ErasedComposeSource]) -> Result<ComposedArtifact, ComposeError> {
        let entry = entries().iter().find(|e| e.writes == target).ok_or_else(|| ComposeError { message: format!("AviComposer: no entry writes {:?}", target), diagnostics: Vec::new() })?;
        (entry.compose)(sources)
    }

    pub fn register() {
        register_composer_entries(std_composer::entries());
    }
}
//#endregion 🚪️DerivedIoRegistry
