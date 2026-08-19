//! 🎪 `stdio.bcf` artifact — stdio reference format.

use semio_framework_plugin::{ArtifactKindSpec, MediaClass, MediaForm, MediaType, OsMediaCapability};

pub use crate::artifacts::bcf::schema::diff::BcfDiff;
pub use crate::artifacts::bcf::schema::mutations::BcfMutation;
pub use crate::artifacts::bcf::schema::snapshot::BcfSnapshot;
pub use crate::artifacts::bcf::schema::BcfArtifact;

/// 🏷️ Document schema / DSL envelope id.
pub const STDIO_BCF_DOCUMENT_SCHEMA: &str = "stdio.bcf";

/// 🧬️ Artifact schema descriptor id.
pub const BCF_ARTIFACT_SCHEMA_ID: &str = "s.stdio.bcf";

//#region 🔖️Declaration
/// 🔖️ This artifact's declaration (ticket 26/08/12/ARTIFACTS-ONLY-PLUGIN-ARCHITECTURE W6, g2) —
/// replaces the old side-effecting `register()` (dissolved out of the former `⚙️engine`, ticket
/// 26/08/12/ENGINELESS-ARTIFACTS-AND-APP-STATE-MACHINES — the codec now lives in `🚪️io`). Mirrors
/// `🔋️energy`'s `s.model` exemplar: headless library artifact, zero `ArtifactApp`s, so
/// `.document_codec_bare` stands in for the old `store::register_document_codec(store::
/// ArtifactCodec::of::<BcfSnapshot, BcfMutation>(...))` call. `.composers(...)` reaches the
/// subset io's own `io_registry` directly (fully qualified, not through the `io` shim), whose
/// `entries()` returns `&'static [ComposerEntry]` (owned rows) — NOT this file's own shadowing
/// `io_registry` below, whose `entries()` returns `&'static [&'static ComposerEntry]`
/// (references) and would silently rebind under a bare call (this ticket's "SILENT REBIND"
/// hazard).
///
/// **NOT covered by any field**: nothing — bcf's `register()` never called `register_schema_spec`
/// (`BcfSnapshot`/`BcfDiff`/`BcfMutation` are all hand-rolled, no derivable `RecordSpec` — see the
/// deleted `register_pilot_languages`' own doc comment), so this artifact converts cleanly with
/// zero residual `.setup()` calls.
/// 🧩️ Binds this executable root to its sole schema-owned definition.
pub fn assembly(definition: semio_framework_plugin::ArtifactDefinition) -> Result<crate::registry::ArtifactAssembly, semio_framework_plugin::PluginAssemblyError> {
    crate::registry::runtime_assembly("bcf", definition, declaration)
}

pub fn declaration(definition: semio_framework_plugin::ArtifactDefinition) -> Result<semio_framework_plugin::ArtifactDeclaration, semio_framework_plugin::ArtifactDefinitionError> {
    let formats = crate::registry::format_descriptors_for("bcf")?;
    semio_framework_plugin::ArtifactDeclaration::builder(definition)
        .schema(crate::artifacts::bcf::schema::bcf_artifact_schema_descriptor())
        .formats(formats)
        .inferences([crate::artifacts::bcf::standards::v2_1::subsets::any::schema::inferences::bcf_artifact_inference_descriptor()])
        .composers(crate::artifacts::bcf::standards::v2_1::subsets::any::io::io_registry::entries())
        .languages(pilot_languages())
        .document_codec_bare::<BcfSnapshot, BcfMutation>(STDIO_BCF_DOCUMENT_SCHEMA)
        .try_build()
}

/// 📌️ Handcrafted facet grammars (text) and protocols (binary), copied verbatim (five
/// `LanguageSpec` rows) from the former `⚙️engine::register_pilot_languages`'s own
/// `dsl::register_language(...)` call bodies (dissolved, ticket 26/08/12/ENGINELESS-ARTIFACTS-
/// AND-APP-STATE-MACHINES).
fn pilot_languages() -> &'static [dsl::LanguageSpec] {
    static LANGUAGES: std::sync::OnceLock<Vec<dsl::LanguageSpec>> = std::sync::OnceLock::new();
    LANGUAGES
        .get_or_init(|| {
            vec![
                dsl::LanguageSpec {
                    id: "stdio.bcf",
                    extension: Some("bcf"),
                    role: dsl::LanguageRole::Document,
                    grammar: Some(crate::artifacts::bcf::schema::snapshot::text::COMPONENT_GRAMMAR_SEMIO),
                    grammar_path: Some(crate::artifacts::bcf::schema::snapshot::text::COMPONENT_GRAMMAR_PATH),
                    protocol: Some(crate::artifacts::bcf::schema::snapshot::binary::COMPONENT_PROTOCOL_SEMIO),
                    protocol_path: Some(crate::artifacts::bcf::schema::snapshot::binary::COMPONENT_PROTOCOL_PATH),
                    hooks: dsl::passthrough_hooks("stdio.bcf"),
                },
                dsl::LanguageSpec {
                    id: "stdio.bcf.op",
                    extension: None,
                    role: dsl::LanguageRole::Ops,
                    grammar: Some(crate::artifacts::bcf::schema::mutations::text::COMPONENT_GRAMMAR_SEMIO),
                    grammar_path: Some(crate::artifacts::bcf::schema::mutations::text::COMPONENT_GRAMMAR_PATH),
                    protocol: Some(crate::artifacts::bcf::schema::mutations::binary::COMPONENT_PROTOCOL_SEMIO),
                    protocol_path: Some(crate::artifacts::bcf::schema::mutations::binary::COMPONENT_PROTOCOL_PATH),
                    hooks: dsl::passthrough_hooks("stdio.bcf.op"),
                },
                dsl::LanguageSpec {
                    id: "stdio.bcf.diff",
                    extension: None,
                    role: dsl::LanguageRole::Diff,
                    grammar: Some(crate::artifacts::bcf::schema::diff::text::COMPONENT_GRAMMAR_SEMIO),
                    grammar_path: Some(crate::artifacts::bcf::schema::diff::text::COMPONENT_GRAMMAR_PATH),
                    protocol: None,
                    protocol_path: None,
                    hooks: dsl::passthrough_hooks("stdio.bcf.diff"),
                },
                dsl::LanguageSpec {
                    id: "stdio.bcf.pack",
                    extension: None,
                    role: dsl::LanguageRole::Pack,
                    grammar: None,
                    grammar_path: None,
                    protocol: Some(crate::artifacts::bcf::schema::snapshot::binary::COMPONENT_PROTOCOL_SEMIO),
                    protocol_path: Some(crate::artifacts::bcf::schema::snapshot::binary::COMPONENT_PROTOCOL_PATH),
                    hooks: dsl::passthrough_hooks("stdio.bcf.pack"),
                },
                dsl::LanguageSpec {
                    id: "stdio.bcf.spr",
                    extension: None,
                    role: dsl::LanguageRole::Spr,
                    grammar: None,
                    grammar_path: None,
                    protocol: Some(crate::artifacts::bcf::schema::mutations::binary::COMPONENT_PROTOCOL_SEMIO),
                    protocol_path: Some(crate::artifacts::bcf::schema::mutations::binary::COMPONENT_PROTOCOL_PATH),
                    hooks: dsl::passthrough_hooks("stdio.bcf.spr"),
                },
            ]
        })
        .as_slice()
}
//#endregion 🔖️Declaration

//#region 🔖️ArtifactKind
/// 🗂️ This artifact's `ArtifactKindSpec`.
pub fn artifact_kind() -> ArtifactKindSpec {
    ArtifactKindSpec {
        id: "stdio.bcf".into(),
        name: "Bcf".into(),
        source_format: STDIO_BCF_DOCUMENT_SCHEMA.into(),
        component_kind: "stdio".into(),
        dimension: "data".into(),
        media_capability: OsMediaCapability::MeshOnly,
        media_type: MediaType { class: MediaClass::Data, form: MediaForm::Value },
        schema: STDIO_BCF_DOCUMENT_SCHEMA.into(),
        export_formats: vec![],
        import_formats: vec![],
        export_stdio_kinds: vec![],
        import_stdio_kinds: vec![],
    }
}
//#endregion 🔖️ArtifactKind
//#region 🚪️DerivedIoRegistry
pub mod io_registry {
    use crate::artifacts::bcf::standards::v2_1::subsets::any::io::io_registry as v2_1;
    use semio_framework_plugin::{register_composer_entries, ComposeError, ComposedArtifact, ComposerEntry, Dialect, ErasedComposeSource};
    use std::sync::OnceLock;

    static ENTRIES: OnceLock<Vec<&'static ComposerEntry>> = OnceLock::new();

    pub fn entries() -> &'static [&'static ComposerEntry] {
        ENTRIES.get_or_init(|| v2_1::entries().iter().collect()).as_slice()
    }

    pub fn compose(target: Dialect, sources: &[ErasedComposeSource]) -> Result<ComposedArtifact, ComposeError> {
        let entry = entries().iter().find(|e| e.writes == target).ok_or_else(|| ComposeError { message: format!("BcfComposer: no entry writes {:?}", target), diagnostics: Vec::new() })?;
        semio_framework_plugin::resolve_ready((entry.compose)(sources))
    }

    pub fn register() {
        let _ = register_composer_entries(v2_1::entries());
    }
}
//#endregion 🚪️DerivedIoRegistry
