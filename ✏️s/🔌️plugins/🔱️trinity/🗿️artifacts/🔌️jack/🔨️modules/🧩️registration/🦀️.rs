//! 🧩️ Jack registration: the pilot language specs, the `ArtifactApps` bound, the artifact definition and the artifact declaration.

use crate::standards::v1::subsets::any::schema::composition::JackComposition;

//#region 🔖️Register
/// 📌️ Handcrafted facet grammars (text) and protocols (binary) for in-process execution — built once
/// and leaked to a `&'static` slice since `dsl::passthrough_hooks` isn't `const fn`, mirroring
/// `io_registry::entries()`'s own `OnceLock` convention. `pub` (ticket
/// 26/08/17/MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME, fleet-trinity-recipe): the new declaration
/// tree's `🪆️subsets/✳️any/🦀️.rs` reads these same five `LanguageSpec`s to build its
/// `NativeCodecs` `LanguagePair`s (see that file's own doc for why it does not delegate to a sibling
/// `crate::standards::v1::subsets::any::io::io()` the way `🗒️note`/`🖍️draw` do).
pub fn pilot_languages() -> &'static [semio_framework_dsl::LanguageSpec] {
    static LANGUAGES: std::sync::OnceLock<Vec<semio_framework_dsl::LanguageSpec>> = std::sync::OnceLock::new();
    LANGUAGES
        .get_or_init(|| {
            vec![
                semio_framework_dsl::LanguageSpec {
                    id: "jack.document",
                    extension: Some("trinity"),
                    role: semio_framework_dsl::LanguageRole::Document,
                    grammar: Some(crate::standards::v1::subsets::any::io::text::snapshot::COMPONENT_GRAMMAR_SEMIO),
                    grammar_path: Some(crate::standards::v1::subsets::any::io::text::snapshot::COMPONENT_GRAMMAR_PATH),
                    protocol: Some(crate::standards::v1::subsets::any::io::binary::snapshot::COMPONENT_PROTOCOL_SEMIO),
                    protocol_path: Some(crate::standards::v1::subsets::any::io::binary::snapshot::COMPONENT_PROTOCOL_PATH),
                    hooks: semio_framework_dsl::passthrough_hooks("jack.document"),
                },
                semio_framework_dsl::LanguageSpec {
                    id: "jack.op",
                    extension: None,
                    role: semio_framework_dsl::LanguageRole::Ops,
                    grammar: Some(crate::standards::v1::subsets::any::io::text::mutations::COMPONENT_GRAMMAR_SEMIO),
                    grammar_path: Some(crate::standards::v1::subsets::any::io::text::mutations::COMPONENT_GRAMMAR_PATH),
                    protocol: Some(crate::standards::v1::subsets::any::io::binary::mutations::COMPONENT_PROTOCOL_SEMIO),
                    protocol_path: Some(crate::standards::v1::subsets::any::io::binary::mutations::COMPONENT_PROTOCOL_PATH),
                    hooks: semio_framework_dsl::passthrough_hooks("jack.op"),
                },
                semio_framework_dsl::LanguageSpec {
                    id: "jack.diff",
                    extension: None,
                    role: semio_framework_dsl::LanguageRole::Diff,
                    grammar: Some(crate::standards::v1::subsets::any::io::text::diff::COMPONENT_GRAMMAR_SEMIO),
                    grammar_path: Some(crate::standards::v1::subsets::any::io::text::diff::COMPONENT_GRAMMAR_PATH),
                    protocol: None,
                    protocol_path: None,
                    hooks: semio_framework_dsl::passthrough_hooks("jack.diff"),
                },
                semio_framework_dsl::LanguageSpec {
                    id: "jack.pack",
                    extension: None,
                    role: semio_framework_dsl::LanguageRole::Pack,
                    grammar: None,
                    grammar_path: None,
                    protocol: Some(crate::standards::v1::subsets::any::io::binary::snapshot::COMPONENT_PROTOCOL_SEMIO),
                    protocol_path: Some(crate::standards::v1::subsets::any::io::binary::snapshot::COMPONENT_PROTOCOL_PATH),
                    hooks: semio_framework_dsl::passthrough_hooks("jack.pack"),
                },
                semio_framework_dsl::LanguageSpec {
                    id: "jack.spr",
                    extension: None,
                    role: semio_framework_dsl::LanguageRole::Spr,
                    grammar: None,
                    grammar_path: None,
                    protocol: Some(crate::standards::v1::subsets::any::io::binary::mutations::COMPONENT_PROTOCOL_SEMIO),
                    protocol_path: Some(crate::standards::v1::subsets::any::io::binary::mutations::COMPONENT_PROTOCOL_PATH),
                    hooks: semio_framework_dsl::passthrough_hooks("jack.spr"),
                },
            ]
        })
        .as_slice()
}

/// 🔖️ This artifact's OLD-channel definition (ticket 26/08/12/ARTIFACTS-ONLY-PLUGIN-ARCHITECTURE M1).
/// KEPT unread by the new declaration tree (ticket 26/08/17/CLEAN-ARTIFACT-STANDARD-SUBSET-MECHANISM,
/// debt D1 — deleted repo-wide only once every plugin has migrated, not this pass — `🗒️note`/`🖍️draw`
/// precedent, `📓️terra-fleet-trinity-recipe-report.md`): the real en/de localized names
/// (`"Jack"`/`"Buchse"`) still live only on these `ArtifactCapability` rows.
/// Jack query source and execution output now register through their exact window owners.
pub trait ArtifactApps<C:JackComposition>:
    semio_framework_plugin::PluginApp
    + From<semio_framework_plugin::app::VcsArtifactApp<semio_framework_plugin::app::EditorApp<crate::editor::jack::TrinityJackPlayApp<C>>, C::Members>>
    + From<semio_framework_plugin::app::VcsArtifactApp<semio_framework_plugin::app::ViewerApp<crate::viewer::jack::TrinityJackViewer<C>>, C::Members>>
{
}

impl<PA,C:JackComposition> ArtifactApps<C> for PA where
    PA: semio_framework_plugin::PluginApp
        + From<semio_framework_plugin::app::VcsArtifactApp<semio_framework_plugin::app::EditorApp<crate::editor::jack::TrinityJackPlayApp<C>>, C::Members>>
        + From<semio_framework_plugin::app::VcsArtifactApp<semio_framework_plugin::app::ViewerApp<crate::viewer::jack::TrinityJackViewer<C>>, C::Members>>
{
}

pub fn definition() -> Result<semio_framework_plugin::ArtifactDefinition, semio_framework_plugin::ArtifactDefinitionError> {
    use semio_framework_plugin::{ArtifactCapability, ArtifactCapabilityKind, ArtifactDefinition, ArtifactIdentity, ArtifactIdentityClaim, ArtifactIdentityNamespace, ArtifactLocale, ArtifactLocalization};

    let rows: &[semio_framework_plugin::ArtifactCapabilityRow<'_>] = &[
        ("s.trinity.jack.standard.v1", "standard", "1", &[], None),
        ("s.trinity.jack.standard.v1.profile.any", "profile", "any", &[], None),
        ("s.trinity.jack.schema.artifact", "schema", "s.trinity.jack", &[("schema", "s.trinity.jack")], None),
        ("s.trinity.jack.inference.artifact", "inference", "s.trinity.jack.inference", &[("schema", "s.trinity.jack.inference")], None),
        ("s.trinity.jack.composer.native", "composer", "s.trinity.jack@1/*", &[("dialect", "s.trinity.jack@1/*")], None),
        ("s.trinity.jack.grammar.1", "grammar", "jack.document", &[("grammar", "jack.document")], None),
        ("s.trinity.jack.grammar.2", "grammar", "jack.op", &[("grammar", "jack.op")], None),
        ("s.trinity.jack.grammar.3", "grammar", "jack.diff", &[("grammar", "jack.diff")], None),
        ("s.trinity.jack.grammar.4", "grammar", "jack.pack", &[("grammar", "jack.pack")], None),
        ("s.trinity.jack.grammar.5", "grammar", "jack.spr", &[("grammar", "jack.spr")], None),
        // 🐛️ D2-capability-claim-repairs: `.document_codec::<EditorApp<TrinityJackPlayApp>>()`
        // derives its extension claim from `<JackSnapshot as pack::snapshot::ArtifactDsl>::EXTENSION`
        // (`…/🚪️io/📝️text/📸️snapshot/🦀️.rs`), which is `"trinity"`, not `"jack"`.
        ("s.trinity.jack.codec.document-1", "codec", "trinity.graph:trinity", &[("codec", "trinity.graph"), ("codec-extension", "13:trinity.graph:trinity")], None),
        ("s.trinity.jack.localization.en", "localization", "Jack", &[], Some(("en", "Jack"))),
        ("s.trinity.jack.localization.de", "localization", "Buchse", &[], Some(("de", "Buchse"))),
    ];
    let mut definition = ArtifactDefinition::new(ArtifactIdentity::parse("s.trinity.jack")?);
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

/// 🌳️ This artifact's declaration tree root (ticket 26/08/17/CLEAN-ARTIFACT-STANDARD-SUBSET-
/// MECHANISM design.md §2, fleet-trinity-recipe) — replaces the old `declaration()`
/// (`ArtifactDeclaration::builder(...).schema(...).inferences(...).composers(...).languages(...)
/// .document_codec(...)` chain, deleted outright, no dual channel) as the ONLY registration channel
/// for schema/io/viewer/editor rows. `definition()` (old `ArtifactDefinition`/capability rows, above)
/// is kept per debt D1, and `artifact_kind()` is kept because this crate's own plugin-root
/// `.activation(...)` still reads `artifact_kind().id`; neither has any caller left in this function.
pub fn artifact<PA: ArtifactApps<C>,C:JackComposition>() -> semio_framework_plugin::app::declarations::ArtifactDeclaration<PA> {
    use semio_framework_plugin::app::declarations::ArtifactDeclaration;
    use {semio_framework_artifact_reference::ArtifactKindId};
    ArtifactDeclaration { kind: ArtifactKindId::parse("s.trinity.jack").expect("canonical jack kind"), localization: &[], standards: vec![crate::standards::v1::standard::<PA,C>()] }
}
/// 🎯️ `ArtifactKindSpec` identity shared by every `jack`-family app that mounts this artifact.
pub fn artifact_kind() -> semio_framework_plugin::ArtifactKindSpec {
    semio_framework_plugin::ArtifactKindSpec {
        id: "graph.trinity".into(),
        label: semio_framework_ui_locale::LocalizedLabel::native("Trinity Graph", "Trinity-Graph"),
        source_format: "trinity.graph".into(),
        component_kind: "trinity".into(),
        dimension: "graph".into(),
        media_capability: semio_framework_plugin::OsMediaCapability::MeshOnly,
        media_type: semio_framework_plugin::MediaType { class: semio_framework_plugin::MediaClass::Graph, form: semio_framework_plugin::MediaForm::Trinity },
        schema: "trinity.graph".into(),
        export_formats: vec![],
        import_formats: vec![],
        export_stdio_kinds: Vec::new(),
        import_stdio_kinds: Vec::new(),
    }
}
//#endregion 🔖️Register
