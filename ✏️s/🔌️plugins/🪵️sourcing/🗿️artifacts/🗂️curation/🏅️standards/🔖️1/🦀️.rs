//! 🏅️ Standard root for `s.sourcing.curation` standard `1` (ticket
//! 26/08/17/CLEAN-ARTIFACT-STANDARD-SUBSET-MECHANISM). Exports `standard() -> StandardDeclaration`,
//! mounting subset `any` (the only subset this standard has). No real mime registration exists for
//! this artifact anywhere in the codebase — the old `definition()` capability rows only ever claimed
//! a codec id (`sourcing.curation/v1:curation`) and an extension (`curation`), never a mime type, so
//! `mimes` here is a documented synthesis, not a literal carry-over (see
//! `📓️w4-sourcing-report.md` `## openQuestions`); `extensions` IS the real, carried-over value.

use crate::standards::v1::subsets;
use semio_framework_plugin::app::declarations::{MediaDeclaration, StandardDeclaration};
use {semio_framework_artifact_reference::StandardId};

pub fn standard<A: crate::SourcingApplication>() -> StandardDeclaration<A> {
    StandardDeclaration { id: StandardId("1"), media: MediaDeclaration { mimes: &["application/vnd.semio.sourcing.curation+json"], extensions: &["curation"] }, subsets: vec![subsets::any::subset()] }
}

//#region 🧩️ArtifactItems7
pub mod artifact_items_7 {
use crate::{SOURCING_CURATION_SCHEMA};
use semio_framework_plugin::{ArtifactKindSpec, MediaClass, MediaForm, MediaType, OsMediaCapability};

//#region 🔖️ArtifactKind
/// 🗂️ This artifact's `ArtifactKindSpec` — stitched into the app manifest by
/// `crate::editor::sourcing::create_sourcing_curation_app`'s `🔖️Manifest` region.
pub fn artifact_kind() -> ArtifactKindSpec {
    ArtifactKindSpec {
        id: "catalogue.sourcing".into(),
        label: semio_framework_ui_locale::LocalizedLabel::native("Sourcing Curation", "Beschaffungskuratierung"),
        source_format: "sourcing.curation".into(),
        component_kind: "catalogue".into(),
        dimension: "data".into(),
        media_capability: OsMediaCapability::MeshOnly,
        media_type: MediaType { class: MediaClass::Kit, form: MediaForm::Kit },
        schema: SOURCING_CURATION_SCHEMA.into(),
        export_formats: vec![],
        import_formats: vec![],
        export_stdio_kinds: vec!["stdio.json".into(), "stdio.zip".into()],
        import_stdio_kinds: vec!["stdio.json".into(), "stdio.zip".into()],
    }
}
//#endregion 🔖️ArtifactKind
}
//#endregion 🧩️ArtifactItems7

//#region 🧩️ArtifactItems8
pub mod artifact_items_8 {
//#region 🔖️Register
/// 🔖️ This artifact's declaration (ticket 26/08/12/ARTIFACTS-ONLY-PLUGIN-ARCHITECTURE M1) — replaces
/// the old side-effecting `register()`, which called four different global registries directly from
/// a plugin `.setup()` callback (a fifth, `crate::io_registry::register()`, was a
/// pure duplicate of what `.composers(...)` now does and was deleted rather than ported — see the
/// mechanism report's `register_all` composer-registration step). `crate::editor::sourcing::config::
/// schema::register_app_schema()` is the one exception, still called from `🪵️sourcing/🦀️.rs`'s
/// own `.setup()`: it registers the `SourcingCurationApp` CONFIG/PRESENCE schema, an app-scope concern
/// `ArtifactDeclaration` deliberately has no field for (see that struct's own doc). Relocated from
/// `⚙️engine` (ticket 26/08/12/ARTIFACTS-ONLY-PLUGIN-ARCHITECTURE reloc-g2): `declaration()` describes
/// the artifact (kind, schema, io ports, ownership), which is not engine behaviour.
pub fn definition() -> Result<semio_framework_plugin::ArtifactDefinition, semio_framework_plugin::ArtifactDefinitionError> {
    use semio_framework_plugin::{ArtifactCapability, ArtifactCapabilityKind, ArtifactDefinition, ArtifactIdentity, ArtifactIdentityClaim, ArtifactIdentityNamespace, ArtifactLocale, ArtifactLocalization};
    let rows: &[semio_framework_plugin::ArtifactCapabilityRow<'_>] = &[
        ("s.sourcing.curation.standard.v1", "standard", "1", &[], None),
        ("s.sourcing.curation.standard.v1.profile.any", "profile", "any", &[], None),
        ("s.sourcing.curation.schema.artifact", "schema", "s.sourcing.curation", &[("schema", "s.sourcing.curation")], None),
        ("s.sourcing.curation.inference.artifact", "inference", "s.sourcing.curation.inference", &[("schema", "s.sourcing.curation.inference")], None),
        ("s.sourcing.curation.composer.zip", "composer", "s.stdio.zip@2.0/*", &[("dialect", "s.stdio.zip@2.0/*")], None),
        ("s.sourcing.curation.composer.json", "composer", "s.stdio.json@rfc8259/*", &[("dialect", "s.stdio.json@rfc8259/*")], None),
        ("s.sourcing.curation.grammar.document", "grammar", "sourcing.curation", &[("grammar", "sourcing.curation")], None),
        ("s.sourcing.curation.grammar.op", "grammar", "sourcing.curation.op", &[("grammar", "sourcing.curation.op")], None),
        ("s.sourcing.curation.grammar.diff", "grammar", "sourcing.curation.diff", &[("grammar", "sourcing.curation.diff")], None),
        ("s.sourcing.curation.grammar.pack", "grammar", "curation.pack", &[("grammar", "curation.pack")], None),
        ("s.sourcing.curation.grammar.spr", "grammar", "curation.spr", &[("grammar", "curation.spr")], None),
        ("s.sourcing.curation.codec.document.v1", "codec", "sourcing.curation/v1:curation", &[("codec", "sourcing.curation/v1"), ("codec-extension", "20:sourcing.curation/v1:curation")], None),
        ("s.sourcing.curation.localization.en", "localization", "Sourcing", &[], Some(("en", "Sourcing"))),
        ("s.sourcing.curation.localization.de", "localization", "Beschaffung", &[], Some(("de", "Beschaffung"))),
    ];
    let mut definition = ArtifactDefinition::new(ArtifactIdentity::parse("s.sourcing.curation")?);
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
/// MECHANISM, design.md §2) — one standard (`1`), one subset (`any`). Replaces the OLD
/// `declaration()`/`ArtifactDeclaration::builder(...)` channel outright (atomic cutover with the
/// plugin root edit — no dual registration). `localization: &[]` is a documented shortfall: the
/// real en/de localized names still live on `definition()`'s kept capability rows (debt D1).
pub fn artifact<A: SourcingApplication>() -> semio_framework_plugin::app::declarations::ArtifactDeclaration<A> {
    use semio_framework_plugin::app::declarations::ArtifactDeclaration;
    use {semio_framework_artifact_reference::ArtifactKindId};
    ArtifactDeclaration { kind: ArtifactKindId::parse("s.sourcing.curation").expect("canonical sourcing.curation kind"), localization: &[], standards: vec![crate::standards::v1::standard()] }
}

/// 🧩️ App fleet capable of hosting this artifact's editor and viewer.
pub trait SourcingApplication:
    semio_framework_plugin::PluginApp
    + From<semio_framework_plugin::VcsArtifactApp<semio_framework_plugin::EditorApp<crate::editor::sourcing::SourcingCurationApp>, semio_s_artifact_stdio_semio::SemioMembers>>
    + From<semio_framework_plugin::VcsArtifactApp<semio_framework_plugin::ViewerApp<crate::viewer::sourcing::SourcingViewer>, semio_s_artifact_stdio_semio::SemioMembers>>
{
}

impl<A> SourcingApplication for A where
    A: semio_framework_plugin::PluginApp
        + From<semio_framework_plugin::VcsArtifactApp<semio_framework_plugin::EditorApp<crate::editor::sourcing::SourcingCurationApp>, semio_s_artifact_stdio_semio::SemioMembers>>
        + From<semio_framework_plugin::VcsArtifactApp<semio_framework_plugin::ViewerApp<crate::viewer::sourcing::SourcingViewer>, semio_s_artifact_stdio_semio::SemioMembers>>
{
}


/// 📌️ Handcrafted facet grammars (text) and protocols (binary) — built once and leaked to a
/// `&'static` slice since `dsl::passthrough_hooks` isn't `const fn`. Consumed by
/// `🚪️io/🦀️.rs`'s `io()` (via `language_spec`) to populate `NativeCodecs`'s
/// `LanguagePair`s — the new declaration tree's home for what the OLD `declaration()`'s
/// `.languages(...)` call used to register.
pub fn pilot_languages() -> &'static [semio_framework_dsl::LanguageSpec] {
    static LANGUAGES: std::sync::OnceLock<Vec<semio_framework_dsl::LanguageSpec>> = std::sync::OnceLock::new();
    LANGUAGES
        .get_or_init(|| {
            vec![
                semio_framework_dsl::LanguageSpec {
                    id: "sourcing.curation",
                    extension: Some("curation"),
                    role: semio_framework_dsl::LanguageRole::Document,
                    grammar: Some(crate::standards::v1::subsets::any::io::text::snapshot::COMPONENT_GRAMMAR_SEMIO),
                    grammar_path: Some(crate::standards::v1::subsets::any::io::text::snapshot::COMPONENT_GRAMMAR_PATH),
                    protocol: Some(crate::standards::v1::subsets::any::io::binary::snapshot::COMPONENT_PROTOCOL_SEMIO),
                    protocol_path: Some(crate::standards::v1::subsets::any::io::binary::snapshot::COMPONENT_PROTOCOL_PATH),
                    hooks: semio_framework_dsl::passthrough_hooks("sourcing.curation"),
                },
                semio_framework_dsl::LanguageSpec {
                    id: "sourcing.curation.op",
                    extension: None,
                    role: semio_framework_dsl::LanguageRole::Ops,
                    grammar: Some(crate::standards::v1::subsets::any::io::text::mutations::COMPONENT_GRAMMAR_SEMIO),
                    grammar_path: Some(crate::standards::v1::subsets::any::io::text::mutations::COMPONENT_GRAMMAR_PATH),
                    protocol: Some(crate::standards::v1::subsets::any::io::binary::mutations::COMPONENT_PROTOCOL_SEMIO),
                    protocol_path: Some(crate::standards::v1::subsets::any::io::binary::mutations::COMPONENT_PROTOCOL_PATH),
                    hooks: semio_framework_dsl::passthrough_hooks("sourcing.curation.op"),
                },
                semio_framework_dsl::LanguageSpec {
                    id: "sourcing.curation.diff",
                    extension: None,
                    role: semio_framework_dsl::LanguageRole::Diff,
                    grammar: Some(crate::standards::v1::subsets::any::io::text::diff::COMPONENT_GRAMMAR_SEMIO),
                    grammar_path: Some(crate::standards::v1::subsets::any::io::text::diff::COMPONENT_GRAMMAR_PATH),
                    protocol: None,
                    protocol_path: None,
                    hooks: semio_framework_dsl::passthrough_hooks("sourcing.curation.diff"),
                },
                semio_framework_dsl::LanguageSpec {
                    id: "curation.pack",
                    extension: None,
                    role: semio_framework_dsl::LanguageRole::Pack,
                    grammar: None,
                    grammar_path: None,
                    protocol: Some(crate::standards::v1::subsets::any::io::binary::snapshot::COMPONENT_PROTOCOL_SEMIO),
                    protocol_path: Some(crate::standards::v1::subsets::any::io::binary::snapshot::COMPONENT_PROTOCOL_PATH),
                    hooks: semio_framework_dsl::passthrough_hooks("curation.pack"),
                },
                semio_framework_dsl::LanguageSpec {
                    id: "curation.spr",
                    extension: None,
                    role: semio_framework_dsl::LanguageRole::Spr,
                    grammar: None,
                    grammar_path: None,
                    protocol: Some(crate::standards::v1::subsets::any::io::binary::mutations::COMPONENT_PROTOCOL_SEMIO),
                    protocol_path: Some(crate::standards::v1::subsets::any::io::binary::mutations::COMPONENT_PROTOCOL_PATH),
                    hooks: semio_framework_dsl::passthrough_hooks("curation.spr"),
                },
            ]
        })
        .as_slice()
}

/// 🔎️ Finds `pilot_languages()`'s entry for one `dsl::LanguageRole` — the lookup `io()` uses to
/// populate each `NativeCodecs` facet's `LanguagePair`.
pub fn language_spec(role: semio_framework_dsl::LanguageRole) -> Option<&'static semio_framework_dsl::LanguageSpec> {
    pilot_languages().iter().find(|spec| spec.role == role)
}
//#endregion 🔖️Register
}
//#endregion 🧩️ArtifactItems8
