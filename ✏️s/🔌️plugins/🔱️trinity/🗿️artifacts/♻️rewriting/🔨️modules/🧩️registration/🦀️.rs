//! 🧩️ Rewriting registration: the `ArtifactApps` bound, the artifact kind, the pilot language specs, the artifact definition and the artifact declaration.


pub trait ArtifactApps:
    semio_framework_plugin::PluginApp
    + From<semio_framework_plugin::app::VcsArtifactApp<semio_framework_plugin::app::EditorApp<crate::editor::rewriting::TrinityRewritingPlayApp>, semio_s_artifact_stdio_semio::SemioMembers>>
    + From<semio_framework_plugin::app::VcsArtifactApp<semio_framework_plugin::app::ViewerApp<crate::viewer::rewriting::TrinityRewritingViewer>, semio_s_artifact_stdio_semio::SemioMembers>>
{
}

impl<PA> ArtifactApps for PA where
    PA: semio_framework_plugin::PluginApp
        + From<semio_framework_plugin::app::VcsArtifactApp<semio_framework_plugin::app::EditorApp<crate::editor::rewriting::TrinityRewritingPlayApp>, semio_s_artifact_stdio_semio::SemioMembers>>
        + From<semio_framework_plugin::app::VcsArtifactApp<semio_framework_plugin::app::ViewerApp<crate::viewer::rewriting::TrinityRewritingViewer>, semio_s_artifact_stdio_semio::SemioMembers>>
{
}



//#region 🔖️Register
/// 📌️ Handcrafted facet grammars (text) and protocols (binary) for in-process execution — built once
/// and leaked to a `&'static` slice since `dsl::passthrough_hooks` isn't `const fn`, mirroring
/// `io_registry::entries()`'s own `OnceLock` convention.
/// `pub` (ticket 26/08/17/MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME, fleet-trinity-recipe): the new
/// declaration tree's `🪆️subsets/✳️any/🦀️.rs` reads these same five `LanguageSpec`s to build
/// its `NativeCodecs` `LanguagePair`s (see that file's own doc for why it does not delegate to a
/// sibling `crate::standards::v1::subsets::any::io::io()` the way `🗒️note`/`🖍️draw` do).
pub fn pilot_languages() -> &'static [semio_framework_dsl::LanguageSpec] {
    static LANGUAGES: std::sync::OnceLock<Vec<semio_framework_dsl::LanguageSpec>> = std::sync::OnceLock::new();
    LANGUAGES
        .get_or_init(|| {
            vec![
                semio_framework_dsl::LanguageSpec {
                    id: "rewriting.document",
                    extension: Some("rewriting"),
                    role: semio_framework_dsl::LanguageRole::Document,
                    grammar: Some(crate::standards::v1::subsets::any::io::text::snapshot::COMPONENT_GRAMMAR_SEMIO),
                    grammar_path: Some(crate::standards::v1::subsets::any::io::text::snapshot::COMPONENT_GRAMMAR_PATH),
                    protocol: Some(crate::standards::v1::subsets::any::io::binary::snapshot::COMPONENT_PROTOCOL_SEMIO),
                    protocol_path: Some(crate::standards::v1::subsets::any::io::binary::snapshot::COMPONENT_PROTOCOL_PATH),
                    hooks: semio_framework_dsl::passthrough_hooks("rewriting.document"),
                },
                semio_framework_dsl::LanguageSpec {
                    id: "rewriting.op",
                    extension: None,
                    role: semio_framework_dsl::LanguageRole::Ops,
                    grammar: Some(crate::standards::v1::subsets::any::io::text::mutations::COMPONENT_GRAMMAR_SEMIO),
                    grammar_path: Some(crate::standards::v1::subsets::any::io::text::mutations::COMPONENT_GRAMMAR_PATH),
                    protocol: Some(crate::standards::v1::subsets::any::io::binary::mutations::COMPONENT_PROTOCOL_SEMIO),
                    protocol_path: Some(crate::standards::v1::subsets::any::io::binary::mutations::COMPONENT_PROTOCOL_PATH),
                    hooks: semio_framework_dsl::passthrough_hooks("rewriting.op"),
                },
                semio_framework_dsl::LanguageSpec {
                    id: "rewriting.diff",
                    extension: None,
                    role: semio_framework_dsl::LanguageRole::Diff,
                    grammar: Some(crate::standards::v1::subsets::any::io::text::diff::COMPONENT_GRAMMAR_SEMIO),
                    grammar_path: Some(crate::standards::v1::subsets::any::io::text::diff::COMPONENT_GRAMMAR_PATH),
                    protocol: None,
                    protocol_path: None,
                    hooks: semio_framework_dsl::passthrough_hooks("rewriting.diff"),
                },
                semio_framework_dsl::LanguageSpec {
                    id: "rewriting.pack",
                    extension: None,
                    role: semio_framework_dsl::LanguageRole::Pack,
                    grammar: None,
                    grammar_path: None,
                    protocol: Some(crate::standards::v1::subsets::any::io::binary::snapshot::COMPONENT_PROTOCOL_SEMIO),
                    protocol_path: Some(crate::standards::v1::subsets::any::io::binary::snapshot::COMPONENT_PROTOCOL_PATH),
                    hooks: semio_framework_dsl::passthrough_hooks("rewriting.pack"),
                },
                semio_framework_dsl::LanguageSpec {
                    id: "rewriting.spr",
                    extension: None,
                    role: semio_framework_dsl::LanguageRole::Spr,
                    grammar: None,
                    grammar_path: None,
                    protocol: Some(crate::standards::v1::subsets::any::io::binary::mutations::COMPONENT_PROTOCOL_SEMIO),
                    protocol_path: Some(crate::standards::v1::subsets::any::io::binary::mutations::COMPONENT_PROTOCOL_PATH),
                    hooks: semio_framework_dsl::passthrough_hooks("rewriting.spr"),
                },
            ]
        })
        .as_slice()
}

/// 🗿️ Declares the Rewriting artifact capabilities and localized identity.
pub fn definition() -> Result<semio_framework_plugin::ArtifactDefinition, semio_framework_plugin::ArtifactDefinitionError> {
    use semio_framework_plugin::{ArtifactCapability, ArtifactCapabilityKind, ArtifactDefinition, ArtifactIdentity, ArtifactIdentityClaim, ArtifactIdentityNamespace, ArtifactLocale, ArtifactLocalization};

    let rows: &[semio_framework_plugin::ArtifactCapabilityRow<'_>] = &[
        ("s.trinity.rewriting.standard.v1", "standard", "1", &[], None),
        ("s.trinity.rewriting.standard.v1.profile.any", "profile", "any", &[], None),
        ("s.trinity.rewriting.schema.artifact", "schema", "s.trinity.rewriting", &[("schema", "s.trinity.rewriting")], None),
        ("s.trinity.rewriting.inference.artifact", "inference", "s.trinity.rewriting.inference", &[("schema", "s.trinity.rewriting.inference")], None),
        ("s.trinity.rewriting.composer.native", "composer", "s.trinity.rewriting@1/*", &[("dialect", "s.trinity.rewriting@1/*")], None),
        ("s.trinity.rewriting.composer.format-1", "composer", "s.stdio.txt@utf-8/*", &[("dialect", "s.stdio.txt@utf-8/*")], None),
        ("s.trinity.rewriting.composer.format-5", "composer", "s.stdio.json@rfc8259/*", &[("dialect", "s.stdio.json@rfc8259/*")], None),
        ("s.trinity.rewriting.grammar.1", "grammar", "rewriting.document", &[("grammar", "rewriting.document")], None),
        ("s.trinity.rewriting.grammar.2", "grammar", "rewriting.op", &[("grammar", "rewriting.op")], None),
        ("s.trinity.rewriting.grammar.3", "grammar", "rewriting.diff", &[("grammar", "rewriting.diff")], None),
        ("s.trinity.rewriting.grammar.4", "grammar", "rewriting.pack", &[("grammar", "rewriting.pack")], None),
        ("s.trinity.rewriting.grammar.5", "grammar", "rewriting.spr", &[("grammar", "rewriting.spr")], None),
        ("s.trinity.rewriting.codec.document-1", "codec", "trinity.rewrite.rule:rewriting", &[("codec", "trinity.rewrite.rule"), ("codec-extension", "20:trinity.rewrite.rule:rewriting")], None),
        ("s.trinity.rewriting.localization.en", "localization", "Rewriting", &[], Some(("en", "Rewriting"))),
        ("s.trinity.rewriting.localization.de", "localization", "Umschreiben", &[], Some(("de", "Umschreiben"))),
    ];
    let mut definition = ArtifactDefinition::new(ArtifactIdentity::parse("s.trinity.rewriting")?);
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
pub fn artifact<PA: ArtifactApps>() -> semio_framework_plugin::app::declarations::ArtifactDeclaration<PA> {
    use semio_framework_plugin::app::declarations::ArtifactDeclaration;
    use {semio_framework_artifact_reference::ArtifactKindId};
    ArtifactDeclaration { kind: ArtifactKindId::parse("s.trinity.rewriting").expect("canonical rewriting kind"), localization: &[], standards: vec![crate::standards::v1::standard::<PA>()] }
}
//#endregion 🔖️Register
