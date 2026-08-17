//! 🎪 `stdio.mp3` artifact — new-format artifact (master plan "New format artifacts" table).

use semio_framework_plugin::{ArtifactKindSpec, Dialect, MediaClass, MediaForm, MediaType, OsMediaCapability, StandardId, SubsetId};

pub use crate::artifacts::mp3::standards::mpeg1_layer3::subsets::any::schema::diff::Mp3Diff;
pub use crate::artifacts::mp3::standards::mpeg1_layer3::subsets::any::schema::mutations::Mp3Mutation;
pub use crate::artifacts::mp3::standards::mpeg1_layer3::subsets::any::schema::snapshot::Mp3Snapshot;
pub use crate::artifacts::mp3::standards::mpeg1_layer3::subsets::any::schema::Mp3Artifact;

/// 🏷️ Document schema / DSL envelope id.
pub const STDIO_MP3_DOCUMENT_SCHEMA: &str = "stdio.mp3";

/// 🧬️ Artifact schema descriptor id.
pub const MP3_ARTIFACT_SCHEMA_ID: &str = "s.stdio.mp3";

//#region 🔖️Dialect
/// 🪪️ Surface coordinate(s) for this artifact — `artifact_kind` matches the schema descriptor
/// id above verbatim (never guessed); `standard`/`subset` match this file's own on-disk
/// `🏅️standards/🔖️.../🪆️subsets/✳️...` location. Lives at the artifact root (not under
/// `editor`/`viewer`) so a viewer file can read it without ever importing through the
/// sibling `editor` module.
pub const MP3_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.mp3", standard: StandardId("mpeg1-layer3"), subset: SubsetId("*") };
//#endregion 🔖️Dialect

//#region 🔖️ArtifactKind
/// 🗂️ This artifact's `ArtifactKindSpec`.
pub fn artifact_kind() -> ArtifactKindSpec {
    ArtifactKindSpec {
        id: "stdio.mp3".into(),
        name: "Mp3".into(),
        source_format: STDIO_MP3_DOCUMENT_SCHEMA.into(),
        component_kind: "stdio".into(),
        dimension: "data".into(),
        media_capability: OsMediaCapability::MeshOnly,
        media_type: MediaType { class: MediaClass::Data, form: MediaForm::Value },
        schema: STDIO_MP3_DOCUMENT_SCHEMA.into(),
        export_formats: vec![],
        import_formats: vec![],
        export_stdio_kinds: vec![],
        import_stdio_kinds: vec![],
    }
}
//#endregion 🔖️ArtifactKind
//#region 🔖️Declaration
/// 🔖️ This artifact's declaration (ticket 26/08/12/ARTIFACTS-ONLY-PLUGIN-ARCHITECTURE g5) — replaces
/// the side-effecting `crate::artifacts::mp3::standards::mpeg1_layer3::engine::register()` call the
/// plugin root used to make imperatively. `.composers(...)` reaches this subset's `🚪️io`-level
/// `io_registry` (ticket 26/08/12/ENGINELESS-ARTIFACTS-AND-APP-STATE-MACHINES dissolved the old
/// `⚙️engine`; the registry moved to `🚪️io`, distinct from this file's own `🚪️DerivedIoRegistry`
/// shadow, whose `entries()` returns `&[&ComposerEntry]` — the wrong type for `.composers()`, which
/// wants `&'static [ComposerEntry]`) by its fully qualified path, per the silent-rebind hazard this
/// ticket calls out. `.document_codec_bare::<Mp3Snapshot, Mp3Mutation>(...)` folds in what
/// `subsets::any::io::register()`'s `store::register_document_codec(store::ArtifactCodec::of::<..>())`
/// call did — mp3 is a headless stdio artifact with no `ArtifactApp` to bind `.document_codec::<A>()`
/// to.
/// 🧩️ Binds this executable root to its sole schema-owned definition.
pub fn assembly(definition: semio_framework_plugin::ArtifactDefinition) -> Result<crate::registry::ArtifactAssembly, semio_framework_plugin::PluginAssemblyError> {
    crate::registry::runtime_assembly("mp3", definition, declaration)
}

pub fn declaration(definition: semio_framework_plugin::ArtifactDefinition) -> Result<semio_framework_plugin::ArtifactDeclaration, semio_framework_plugin::ArtifactDefinitionError> {
    let formats = crate::registry::format_descriptors_for("mp3")?;
    semio_framework_plugin::ArtifactDeclaration::builder(definition)
        .schema(crate::artifacts::mp3::standards::mpeg1_layer3::subsets::any::schema::mp3_artifact_schema_descriptor())
        .formats(formats)
        .inferences([crate::artifacts::mp3::standards::mpeg1_layer3::subsets::any::schema::inferences::mp3_artifact_inference_descriptor()])
        .composers(crate::artifacts::mp3::standards::mpeg1_layer3::subsets::any::io::io_registry::entries())
        .document_codec_bare::<Mp3Snapshot, Mp3Mutation>(STDIO_MP3_DOCUMENT_SCHEMA)
        .try_build()
}
//#endregion 🔖️Declaration
//#region 🚪️DerivedIoRegistry
pub mod io_registry {
    use crate::artifacts::mp3::standards::mpeg1_layer3::subsets::any::io::io_registry as std_composer;
    use semio_framework_plugin::{register_composer_entries, ComposeError, ComposedArtifact, ComposerEntry, Dialect, ErasedComposeSource};
    use std::sync::OnceLock;

    static ENTRIES: OnceLock<Vec<&'static ComposerEntry>> = OnceLock::new();

    pub fn entries() -> &'static [&'static ComposerEntry] {
        ENTRIES.get_or_init(|| std_composer::entries().iter().collect()).as_slice()
    }

    pub fn compose(target: Dialect, sources: &[ErasedComposeSource]) -> Result<ComposedArtifact, ComposeError> {
        let entry = entries().iter().find(|e| e.writes == target).ok_or_else(|| ComposeError { message: format!("Mp3Composer: no entry writes {:?}", target), diagnostics: Vec::new() })?;
        (entry.compose)(sources)
    }

    pub fn register() {
        register_composer_entries(std_composer::entries());
    }
}
//#endregion 🚪️DerivedIoRegistry
