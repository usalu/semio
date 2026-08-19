//! 🎪 `stdio.wav` artifact — new-format artifact (master plan "New format artifacts" table).

use semio_framework_plugin::{ArtifactKindSpec, Dialect, MediaClass, MediaForm, MediaType, OsMediaCapability, StandardId, SubsetId};

pub use crate::artifacts::wav::standards::riff_pcm::subsets::any::schema::diff::WavDiff;
pub use crate::artifacts::wav::standards::riff_pcm::subsets::any::schema::mutations::WavMutation;
pub use crate::artifacts::wav::standards::riff_pcm::subsets::any::schema::snapshot::WavSnapshot;
pub use crate::artifacts::wav::standards::riff_pcm::subsets::any::schema::WavArtifact;

/// 🏷️ Document schema / DSL envelope id.
pub const STDIO_WAV_DOCUMENT_SCHEMA: &str = "stdio.wav";

/// 🧬️ Artifact schema descriptor id.
pub const WAV_ARTIFACT_SCHEMA_ID: &str = "s.stdio.wav";

//#region 🔖️Dialect
/// 🪪️ Surface coordinate(s) for this artifact — `artifact_kind` matches the schema descriptor
/// id above verbatim (never guessed); `standard`/`subset` match this file's own on-disk
/// `🏅️standards/🔖️.../🪆️subsets/✳️...` location. Lives at the artifact root (not under
/// `editor`/`viewer`) so a viewer file can read it without ever importing through the
/// sibling `editor` module.
pub const WAV_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.wav", standard: StandardId("riff-pcm"), subset: SubsetId("*") };
//#endregion 🔖️Dialect

//#region 🔖️ArtifactKind
/// 🗂️ This artifact's `ArtifactKindSpec`.
pub async fn assembly(definition: semio_framework_plugin::ArtifactDefinition) -> Result<crate::registry::ArtifactAssembly, semio_framework_plugin::PluginAssemblyError> {
    crate::registry::definition_only_assembly("wav", definition)
}

pub async fn artifact_kind() -> ArtifactKindSpec {
    ArtifactKindSpec {
        id: "stdio.wav".into(),
        name: "Wav".into(),
        source_format: STDIO_WAV_DOCUMENT_SCHEMA.into(),
        component_kind: "stdio".into(),
        dimension: "data".into(),
        media_capability: OsMediaCapability::MeshOnly,
        media_type: MediaType { class: MediaClass::Data, form: MediaForm::Value },
        schema: STDIO_WAV_DOCUMENT_SCHEMA.into(),
        export_formats: vec![],
        import_formats: vec![],
        export_stdio_kinds: vec![],
        import_stdio_kinds: vec![],
    }
}
//#endregion 🔖️ArtifactKind
//#region 🚪️DerivedIoRegistry
pub mod io_registry {
    use crate::artifacts::wav::standards::riff_pcm::subsets::any::io::io_registry as std_composer;
    use semio_framework_plugin::{register_composer_entries, ComposeError, ComposedArtifact, ComposerEntry, Dialect, ErasedComposeSource};
    use std::sync::OnceLock;

    static ENTRIES: OnceLock<Vec<&'static ComposerEntry>> = OnceLock::new();

    pub async fn entries() -> &'static [&'static ComposerEntry] {
        ENTRIES.get_or_init(|| std_composer::entries().iter().collect()).as_slice()
    }

    pub async fn compose(target: Dialect, sources: &[ErasedComposeSource]) -> Result<ComposedArtifact, ComposeError> {
        let entry = entries().iter().find(|e| e.writes == target).ok_or_else(|| ComposeError { message: format!("WavComposer: no entry writes {:?}", target), diagnostics: Vec::new() })?;
        semio_framework_plugin::resolve_ready((entry.compose)(sources))
    }

    pub async fn register() {
        let _ = register_composer_entries(std_composer::entries());
    }
}
//#endregion 🚪️DerivedIoRegistry
