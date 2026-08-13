//! 🎪 `stdio.epw` artifact — new-format artifact (master plan "New format artifacts" table).

use semio_framework_plugin::{ArtifactKindSpec, MediaClass, MediaForm, MediaType, OsMediaCapability};

pub use crate::artifacts::epw::standards::energyplus::subsets::any::schema::snapshot::EpwSnapshot;
pub use crate::artifacts::epw::standards::energyplus::subsets::any::schema::EpwArtifact;
pub use crate::artifacts::epw::standards::energyplus::subsets::any::schema::diff::EpwDiff;
pub use crate::artifacts::epw::standards::energyplus::subsets::any::schema::mutations::EpwMutation;

/// 🏷️ Document schema / DSL envelope id.
pub const STDIO_EPW_DOCUMENT_SCHEMA: &str = "stdio.epw";

/// 🧬️ Artifact schema descriptor id.
pub const EPW_ARTIFACT_SCHEMA_ID: &str = "s.stdio.epw";

//#region 🔖️ArtifactKind
/// 🗂️ This artifact's `ArtifactKindSpec`.
pub fn artifact_kind() -> ArtifactKindSpec {
    ArtifactKindSpec {
        id: "stdio.epw".into(),
        name: "Epw".into(),
        source_format: STDIO_EPW_DOCUMENT_SCHEMA.into(),
        component_kind: "stdio".into(),
        dimension: "data".into(),
        media_capability: OsMediaCapability::MeshOnly,
        media_type: MediaType { class: MediaClass::Data, form: MediaForm::Value },
        schema: STDIO_EPW_DOCUMENT_SCHEMA.into(),
        export_formats: vec![],
        import_formats: vec![],
        export_stdio_kinds: vec![],
        import_stdio_kinds: vec![],
    }
}
//#endregion 🔖️ArtifactKind

//#region 🔖️Register
/// 🗂️ Registers this artifact's IO composer + the handcrafted grammar/protocol `LanguageSpec` —
/// dissolved out of the former `⚙️engine` (ticket 26/08/12/ENGINELESS-ARTIFACTS-AND-APP-STATE-
/// MACHINES). `epw` is one of stdio's 10 deliberate imperative-`register()` artifacts (never
/// converted to the `ArtifactDeclaration` builder pattern, per `crate::plugin()`'s own call —
/// unchanged in call order/behavior, only the function's file moved with the deleted directory).
pub fn register() {
    crate::artifacts::epw::standards::energyplus::subsets::any::io::register();
    register_pilot_languages();
}

/// 📌️ Registers handcrafted facet grammars (text) and protocols (binary).
pub fn register_pilot_languages() {
    use crate::artifacts::epw::standards::energyplus::subsets::any::schema::snapshot;
    dsl::register_language(dsl::LanguageSpec {
        id: "stdio.epw",
        extension: Some("epw"),
        role: dsl::LanguageRole::Document,
        grammar: Some(snapshot::text::COMPONENT_GRAMMAR_SEMIO),
        grammar_path: Some(snapshot::text::COMPONENT_GRAMMAR_PATH),
        protocol: Some(snapshot::binary::COMPONENT_PROTOCOL_SEMIO),
        protocol_path: Some(snapshot::binary::COMPONENT_PROTOCOL_PATH),
        hooks: dsl::passthrough_hooks("stdio.epw"),
    });
}
//#endregion 🔖️Register

//#region 🚪️DerivedIoRegistry
pub mod io_registry {
    use std::sync::OnceLock;
    use semio_framework_plugin::{ComposerEntry, Dialect, ErasedComposeSource, ComposedArtifact, ComposeError, register_composer_entries};
    use crate::artifacts::epw::standards::energyplus::subsets::any::io::io_registry as std_composer;

    static ENTRIES: OnceLock<Vec<&'static ComposerEntry>> = OnceLock::new();

    pub fn entries() -> &'static [&'static ComposerEntry] {
        ENTRIES.get_or_init(|| std_composer::entries().iter().collect()).as_slice()
    }

    pub fn compose(target: Dialect, sources: &[ErasedComposeSource]) -> Result<ComposedArtifact, ComposeError> {
        let entry = entries()
            .iter()
            .find(|e| e.writes == target)
            .ok_or_else(|| ComposeError { message: format!("EpwComposer: no entry writes {:?}", target), diagnostics: Vec::new() })?;
        (entry.compose)(sources)
    }

    pub fn register() {
        register_composer_entries(std_composer::entries());
    }
}
//#endregion 🚪️DerivedIoRegistry
