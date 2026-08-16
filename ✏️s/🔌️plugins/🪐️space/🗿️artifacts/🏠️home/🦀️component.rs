//! 🏠️ S Home launcher artifact — document entity (constitutional: general).

use semio_framework_plugin::{ArtifactKindSpec, MediaClass, MediaForm, MediaType, OsMediaCapability};

pub use crate::artifacts::home::schema::mutations::SHomeMutation;

pub use crate::artifacts::home::schema::diff::SHomeDiff;

pub const S_HOME_DOCUMENT_SCHEMA: &str = "s.home";
pub use crate::artifacts::home::schema::SHomeArtifact;
pub use crate::artifacts::home::snapshot::schema::SHomeSnapshot;

//#region 🔖️ArtifactKind
/// 🗂️ OS artifact kind for this document.
pub fn artifact_kind() -> ArtifactKindSpec {
    ArtifactKindSpec {
        id: "space.shome".into(),
        name: "S Home".into(),
        source_format: S_HOME_DOCUMENT_SCHEMA.into(),
        component_kind: "home".into(),
        dimension: "data".into(),
        media_capability: OsMediaCapability::MeshOnly,
        media_type: MediaType { class: MediaClass::Data, form: MediaForm::Value },
        schema: S_HOME_DOCUMENT_SCHEMA.into(),
        export_formats: vec![],
        import_formats: vec![],
            export_stdio_kinds: vec!["stdio.csv", "stdio.json", "stdio.xlsx", "stdio.zip"],
        import_stdio_kinds: vec!["stdio.csv", "stdio.json", "stdio.xlsx", "stdio.zip"],
    }
}
//#endregion 🔖️ArtifactKind

//#region 🔖️Declaration
/// 🔖️ This artifact's declaration (ticket 26/08/12/ARTIFACTS-ONLY-PLUGIN-ARCHITECTURE M1, relocated
/// off `⚙️engine` to the artifact root — `declaration()` describes the artifact itself, not engine
/// behaviour) — replaces the old side-effecting `register_artifact_schema()`/
/// `register_artifact_inference()`/`register_io()` trio (each a lone-call wrapper around one global
/// registry) plus the root's old `register_pilot_languages()` 5-language block, folded here into one
/// declarative table. `kind` is `"s.home"`, matching `HOME_DIALECT.artifact_kind`/
/// `S_HOME_DOCUMENT_SCHEMA` above — NOT `space.shome` (the OS-level `ArtifactKindSpec.id`, a
/// different namespace) or `s.space.home` (the schema-descriptor id) — see
/// `ArtifactDeclaration::register_all`'s ownership check, which is enforced against the composer
/// table's own dialects. Both apps' own config/presence schema (`apps::home::config::schema::
/// app_schema_descriptor()`/`apps::space::config::schema::app_schema_descriptor()`) moved off
/// `.setup()` onto `ArtifactApp::app_schema()` overrides (ticket W1c) — `ArtifactDeclaration`
/// deliberately has no field for app-scope schema (see that struct's own doc); it is registered by
/// `.register_document_app::<A>()` instead, keyed off `A` the same way this declaration is keyed off
/// `kind`.
pub fn definition() -> Result<semio_framework_plugin::ArtifactDefinition, semio_framework_plugin::ArtifactDefinitionError> {
    use semio_framework_plugin::{ArtifactCapability, ArtifactCapabilityKind, ArtifactDefinition, ArtifactIdentity, ArtifactIdentityClaim, ArtifactIdentityNamespace, ArtifactLocale, ArtifactLocalization};
    let rows: &[(&str, &str, &str, &[(&str, &str)], Option<(&str, &str)>)] = &[
        ("s.home.schema.artifact", "schema", "s.space.home", &[("schema", "s.space.home")], None),
        ("s.home.inference.artifact", "inference", "s.space.home.inference", &[("schema", "s.space.home.inference")], None),
        ("s.home.composer.zip", "composer", "s.stdio.zip@2.0/*", &[("dialect", "s.stdio.zip@2.0/*")], None),
        ("s.home.composer.csv", "composer", "s.stdio.csv@rfc4180/*", &[("dialect", "s.stdio.csv@rfc4180/*")], None),
        ("s.home.composer.xlsx", "composer", "s.stdio.xlsx@ecma-376/*", &[("dialect", "s.stdio.xlsx@ecma-376/*")], None),
        ("s.home.composer.json", "composer", "s.stdio.json@rfc8259/*", &[("dialect", "s.stdio.json@rfc8259/*")], None),
        ("s.home.grammar.document", "grammar", "space.home.document", &[("grammar", "space.home.document")], None),
        ("s.home.grammar.op", "grammar", "space.home.op", &[("grammar", "space.home.op")], None),
        ("s.home.grammar.diff", "grammar", "space.home.diff", &[("grammar", "space.home.diff")], None),
        ("s.home.grammar.pack", "grammar", "home.pack", &[("grammar", "home.pack")], None),
        ("s.home.grammar.spr", "grammar", "home.spr", &[("grammar", "home.spr")], None),
        ("s.home.codec.document", "codec", "s.home:shome", &[("codec", "s.home"), ("extension", "shome")], None),
        ("s.home.localization.en", "localization", "Home", &[], Some(("en", "Home"))),
        ("s.home.localization.de", "localization", "Startseite", &[], Some(("de", "Startseite"))),
    ];
    let mut definition = ArtifactDefinition::new(ArtifactIdentity::parse("s.home")?);
    for (identity, kind, descriptor, claims, localization) in rows {
        let mut capability = ArtifactCapability::new(ArtifactIdentity::parse(*identity)?, ArtifactCapabilityKind::parse(*kind)?).descriptor(descriptor.as_bytes())?;
        for (namespace, value) in *claims {
            capability = capability.claim(ArtifactIdentityClaim::new(ArtifactIdentityNamespace::parse(*namespace)?, *value)?)?;
        }
        if let Some((locale, text)) = localization {
            capability = capability.localization(ArtifactLocalization::new(ArtifactLocale::parse(*locale)?, *text)?)?;
        }
        definition = definition.capability(capability)?;
    }
    Ok(definition)
}

pub fn declaration() -> Result<semio_framework_plugin::ArtifactDeclaration, semio_framework_plugin::ArtifactDefinitionError> {
    semio_framework_plugin::ArtifactDeclaration::builder(definition()?)
        .schema(crate::artifacts::home::schema::home_artifact_schema_descriptor())
        .inferences([crate::artifacts::home::standards::v1::subsets::any::schema::inferences::home_artifact_inference_descriptor()])
        .composers(crate::artifacts::home::standards::v1::subsets::any::io::io_registry::entries())
        .languages(pilot_languages())
        .document_codec::<crate::apps::home::HomeApp>()
        .try_build()
}

/// 📌️ Handcrafted facet grammars (text) and protocols (binary) for in-process execution — moved
/// verbatim from the root's old `register_pilot_languages()`, built once and leaked to a `&'static`
/// slice since `dsl::passthrough_hooks` isn't `const fn` (mirrors note's own `pilot_languages()`).
/// Uses the fully-qualified `std::sync::OnceLock` (deviation: the `⚙️engine` source file relied on a
/// local `use std::sync::{Mutex, OnceLock};` import that does not travel with the move; every sibling
/// artifact's own `pilot_languages()` already spells this fully-qualified, so this matches that
/// convention rather than adding a new top-level `use` here).
fn pilot_languages() -> &'static [dsl::LanguageSpec] {
    static LANGUAGES: std::sync::OnceLock<Vec<dsl::LanguageSpec>> = std::sync::OnceLock::new();
    LANGUAGES
        .get_or_init(|| {
            vec![
                dsl::LanguageSpec {
                    id: "space.shome",
                    extension: Some("shome"),
                    role: dsl::LanguageRole::Document,
                    grammar: Some(crate::artifacts::home::dsl::COMPONENT_GRAMMAR_SEMIO),
                    grammar_path: Some(crate::artifacts::home::dsl::COMPONENT_GRAMMAR_PATH),
                    protocol: Some(crate::artifacts::home::snapshot::pack::COMPONENT_PROTOCOL_SEMIO),
                    protocol_path: Some(crate::artifacts::home::snapshot::pack::COMPONENT_PROTOCOL_PATH),
                    hooks: dsl::passthrough_hooks("space.shome"),
                },
                dsl::LanguageSpec {
                    id: "space.shome.op",
                    extension: None,
                    role: dsl::LanguageRole::Ops,
                    grammar: Some(crate::artifacts::home::op::COMPONENT_GRAMMAR_SEMIO),
                    grammar_path: Some(crate::artifacts::home::op::COMPONENT_GRAMMAR_PATH),
                    protocol: Some(crate::artifacts::home::spr::COMPONENT_PROTOCOL_SEMIO),
                    protocol_path: Some(crate::artifacts::home::spr::COMPONENT_PROTOCOL_PATH),
                    hooks: dsl::passthrough_hooks("space.shome.op"),
                },
                dsl::LanguageSpec {
                    id: "space.shome.diff",
                    extension: None,
                    role: dsl::LanguageRole::Diff,
                    grammar: Some(crate::artifacts::home::diff::COMPONENT_GRAMMAR_SEMIO),
                    grammar_path: Some(crate::artifacts::home::diff::COMPONENT_GRAMMAR_PATH),
                    protocol: None,
                    protocol_path: None,
                    hooks: dsl::passthrough_hooks("space.shome.diff"),
                },
                dsl::LanguageSpec {
                    id: "home.pack",
                    extension: None,
                    role: dsl::LanguageRole::Pack,
                    grammar: None,
                    grammar_path: None,
                    protocol: Some(crate::artifacts::home::snapshot::pack::COMPONENT_PROTOCOL_SEMIO),
                    protocol_path: Some(crate::artifacts::home::snapshot::pack::COMPONENT_PROTOCOL_PATH),
                    hooks: dsl::passthrough_hooks("home.pack"),
                },
                dsl::LanguageSpec {
                    id: "home.spr",
                    extension: None,
                    role: dsl::LanguageRole::Spr,
                    grammar: None,
                    grammar_path: None,
                    protocol: Some(crate::artifacts::home::spr::COMPONENT_PROTOCOL_SEMIO),
                    protocol_path: Some(crate::artifacts::home::spr::COMPONENT_PROTOCOL_PATH),
                    hooks: dsl::passthrough_hooks("home.spr"),
                },
            ]
        })
        .as_slice()
}
//#endregion 🔖️Declaration

//#region 🚪️DerivedIoRegistry
pub mod io_registry {
    use std::sync::OnceLock;
    use semio_framework_plugin::{ComposerEntry, Dialect, ErasedComposeSource, ComposedArtifact, ComposeError, register_composer_entries};
    use crate::artifacts::home::standards::v1::subsets::any::io::io_registry as v1;

    static ENTRIES: OnceLock<Vec<&'static ComposerEntry>> = OnceLock::new();

    pub fn entries() -> &'static [&'static ComposerEntry] {
        ENTRIES.get_or_init(|| v1::entries().iter().collect()).as_slice()
    }

    pub fn compose(target: Dialect, sources: &[ErasedComposeSource]) -> Result<ComposedArtifact, ComposeError> {
        let entry = entries()
            .iter()
            .find(|e| e.writes == target)
            .ok_or_else(|| ComposeError { message: format!("SHomeComposer: no entry writes {:?}", target), diagnostics: Vec::new() })?;
        (entry.compose)(sources)
    }

    pub fn register() {
        register_composer_entries(v1::entries());
    }
}
//#endregion 🚪️DerivedIoRegistry
