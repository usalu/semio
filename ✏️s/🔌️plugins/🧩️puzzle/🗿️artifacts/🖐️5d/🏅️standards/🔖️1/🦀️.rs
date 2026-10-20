//! 🏔️ `📓️terra-fleet-trinity-recipe-report.md`), mounts subset `any`. `mimes`/`extensions`: no real MIME
//! 🏅️ Standard root — `standard() -> StandardDeclaration` (`terra-fleet-trinity-recipe` recipe,
//! registration exists for `s.puzzle5d` outside the old `ArtifactDefinition`'s capability rows
//! (`definition()`, kept — debt D1) — that channel only ever claimed a codec id (`puzzle.5d:
//! puzzle5d-play`) and an extension (`puzzle5d-play`, the EDITOR's own play snapshot extension, not
//! the base `Puzzle5dSnapshot`'s own extension), never a mime type.
//! `mimes: ["application/vnd.semio.puzzle5d+json"]` is a documented synthesis (matches `🗒️note`/
//! `🖍️draw`/`🔱️trinity`'s identical documented deviation); `extensions: ["puzzle5d-play"]` is the
//! real, carried-over value from `definition()`'s `s.puzzle5d.codec.document-1` row.

use crate::standards::v1::subsets;
use semio_framework_plugin::app::declarations::MediaDeclaration;
use semio_framework_plugin::app::declarations::StandardDeclaration;
use {semio_framework_artifact_reference::StandardId};

pub fn standard<PA: crate::ArtifactApps>() -> StandardDeclaration<PA> {
    StandardDeclaration { id: StandardId("1"), media: MediaDeclaration { mimes: &["application/vnd.semio.puzzle5d+json"], extensions: &["puzzle5d-play"] }, subsets: vec![subsets::any::subset::<PA>()] }
}

//#region 🧩️ArtifactItems5
pub mod artifact_items_5 {
//#region 🔖️Declaration
/// 🔖️ Puzzle5d's declaration (ticket 26/08/12/ARTIFACTS-ONLY-PLUGIN-ARCHITECTURE M1, relocated off
/// the former `⚙️engine` to the artifact root — `declaration()` describes the artifact itself, not
/// engine behaviour) — replaces the `ComposerEntry` half of the old `register_io()`. The `"5d.puzzle"`
/// OS-host mesh export/import bridge (`register_mesh_io()`, now `crate::editor::puzzle5d::register_mesh_io`
/// per ticket 26/08/12/ENGINELESS-ARTIFACTS-AND-APP-STATE-MACHINES) has NO `ArtifactDeclaration`
/// field — same OS media-host 14-function family flagged on puzzle2d's `declaration()` doc — so it
/// stays wired through `🧩️puzzle/🦀️.rs`'s own `.setup()`, not here.
pub trait ArtifactApps:
    semio_framework_plugin::PluginApp
    + From<semio_framework_plugin::app::VcsArtifactApp<semio_framework_plugin::app::EditorApp<crate::editor::puzzle5d::Puzzle5dPlayApp>>>
    + From<semio_framework_plugin::app::VcsArtifactApp<semio_framework_plugin::app::ViewerApp<crate::viewer::puzzle5d::Puzzle5dViewer>>>
{
}

impl<PA> ArtifactApps for PA where
    PA: semio_framework_plugin::PluginApp
        + From<semio_framework_plugin::app::VcsArtifactApp<semio_framework_plugin::app::EditorApp<crate::editor::puzzle5d::Puzzle5dPlayApp>>>
        + From<semio_framework_plugin::app::VcsArtifactApp<semio_framework_plugin::app::ViewerApp<crate::viewer::puzzle5d::Puzzle5dViewer>>>
{
}

pub fn definition() -> Result<semio_framework_plugin::ArtifactDefinition, semio_framework_plugin::ArtifactDefinitionError> {
use semio_framework_plugin::{ArtifactCapability, ArtifactCapabilityKind, ArtifactDefinition, ArtifactIdentity, ArtifactIdentityClaim, ArtifactIdentityNamespace, ArtifactLocale, ArtifactLocalization};

    let rows: &[semio_framework_plugin::ArtifactCapabilityRow<'_>] = &[
        ("s.puzzle.puzzle5d.standard.v1", "standard", "1", &[], None),
        ("s.puzzle.puzzle5d.standard.v1.profile.any", "profile", "any", &[], None),
        ("s.puzzle.puzzle5d.schema.artifact", "schema", "s.puzzle.puzzle5d", &[("schema", "s.puzzle.puzzle5d")], None),
        ("s.puzzle.puzzle5d.inference.artifact", "inference", "s.puzzle.puzzle5d.inference", &[("schema", "s.puzzle.puzzle5d.inference")], None),
        ("s.puzzle.puzzle5d.composer.native", "composer", "s.puzzle.puzzle5d@1/*", &[("dialect", "s.puzzle.puzzle5d@1/*")], None),
        ("s.puzzle.puzzle5d.composer.format-1", "composer", "s.stdio.zip@2.0/*", &[("dialect", "s.stdio.zip@2.0/*")], None),
        ("s.puzzle.puzzle5d.composer.format-2", "composer", "s.stdio.png@1.2/*", &[("dialect", "s.stdio.png@1.2/*")], None),
        ("s.puzzle.puzzle5d.composer.format-3", "composer", "s.stdio.json@rfc8259/*", &[("dialect", "s.stdio.json@rfc8259/*")], None),
        ("s.puzzle.puzzle5d.grammar.1", "grammar", "puzzle.puzzle5d", &[("grammar", "puzzle.puzzle5d")], None),
        ("s.puzzle.puzzle5d.grammar.2", "grammar", "puzzle.puzzle5d.op", &[("grammar", "puzzle.puzzle5d.op")], None),
        ("s.puzzle.puzzle5d.grammar.3", "grammar", "puzzle.puzzle5d.diff", &[("grammar", "puzzle.puzzle5d.diff")], None),
        ("s.puzzle.puzzle5d.grammar.4", "grammar", "5d.pack", &[("grammar", "5d.pack")], None),
        ("s.puzzle.puzzle5d.grammar.5", "grammar", "5d.spr", &[("grammar", "5d.spr")], None),
        ("s.puzzle.puzzle5d.codec.document-1", "codec", "puzzle.5d:puzzle5d-play", &[("codec", "puzzle.5d"), ("codec-extension", "9:puzzle.5d:puzzle5d-play")], None),
        ("s.puzzle.puzzle5d.localization.en", "localization", "5D Puzzle", &[], Some(("en", "5D Puzzle"))),
        ("s.puzzle.puzzle5d.localization.de", "localization", "5D-Puzzle", &[], Some(("de", "5D-Puzzle"))),
    ];
    let mut definition = ArtifactDefinition::new(ArtifactIdentity::parse("s.puzzle.puzzle5d")?);
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

/// 🌳️ This artifact's declaration tree root (ticket `26/08/17/MICROKERNEL-POOLED-ACTOR-PLUGIN-
/// RUNTIME`, `terra-descriptors` packet, following the `terra-fleet-trinity-recipe` recipe) —
/// replaces the old `declaration()` (`ArtifactDeclaration::builder(...).schema(...).inferences(...)
/// .composers(...).languages(...).document_codec(...)` chain, deleted outright, no dual channel) as
/// the ONLY registration channel for schema/io/viewer/editor rows. `definition()` (old
/// `ArtifactDefinition`/capability rows, above) is kept per debt D1.
pub fn artifact<PA: ArtifactApps>() -> semio_framework_plugin::app::declarations::ArtifactDeclaration<PA> {
    use semio_framework_plugin::app::declarations::ArtifactDeclaration;
    use {semio_framework_artifact_reference::ArtifactKindId};
    ArtifactDeclaration { kind: ArtifactKindId::parse("s.puzzle.puzzle5d").expect("canonical puzzle5d kind"), localization: &[], standards: vec![crate::standards::v1::standard::<PA>()] }
}

/// 📌️ Handcrafted facet grammars (text) and protocols (binary) for in-process execution — built once
/// and leaked to a `&'static` slice since `dsl::passthrough_hooks` isn't `const fn`. This function
/// existed as a side-effecting `register_pilot_languages()` before M1 but was never called from
/// anywhere (dead code, confirmed by grep) — wiring it into `declaration()`'s `.languages(...)` is
/// this conversion's one real bug fix: puzzle5d's own grammars were never actually registered.
pub fn pilot_languages() -> &'static [semio_framework_dsl::LanguageSpec] {
    static LANGUAGES: std::sync::OnceLock<Vec<semio_framework_dsl::LanguageSpec>> = std::sync::OnceLock::new();
    LANGUAGES
        .get_or_init(|| {
            vec![
                semio_framework_dsl::LanguageSpec {
                    id: "puzzle.puzzle5d",
                    extension: Some("puzzle5d"),
                    role: semio_framework_dsl::LanguageRole::Document,
                    grammar: Some(crate::standards::v1::subsets::any::io::text::snapshot::COMPONENT_GRAMMAR_SEMIO),
                    grammar_path: Some(crate::standards::v1::subsets::any::io::text::snapshot::COMPONENT_GRAMMAR_PATH),
                    protocol: Some(crate::standards::v1::subsets::any::io::binary::snapshot::COMPONENT_PROTOCOL_SEMIO),
                    protocol_path: Some(crate::standards::v1::subsets::any::io::binary::snapshot::COMPONENT_PROTOCOL_PATH),
                    hooks: semio_framework_dsl::passthrough_hooks("puzzle.puzzle5d"),
                },
                semio_framework_dsl::LanguageSpec {
                    id: "puzzle.puzzle5d.op",
                    extension: None,
                    role: semio_framework_dsl::LanguageRole::Ops,
                    grammar: Some(crate::standards::v1::subsets::any::io::text::mutations::COMPONENT_GRAMMAR_SEMIO),
                    grammar_path: Some(crate::standards::v1::subsets::any::io::text::mutations::COMPONENT_GRAMMAR_PATH),
                    protocol: Some(crate::standards::v1::subsets::any::io::binary::mutations::COMPONENT_PROTOCOL_SEMIO),
                    protocol_path: Some(crate::standards::v1::subsets::any::io::binary::mutations::COMPONENT_PROTOCOL_PATH),
                    hooks: semio_framework_dsl::passthrough_hooks("puzzle.puzzle5d.op"),
                },
                semio_framework_dsl::LanguageSpec {
                    id: "puzzle.puzzle5d.diff",
                    extension: None,
                    role: semio_framework_dsl::LanguageRole::Diff,
                    grammar: Some(crate::standards::v1::subsets::any::io::text::diff::COMPONENT_GRAMMAR_SEMIO),
                    grammar_path: Some(crate::standards::v1::subsets::any::io::text::diff::COMPONENT_GRAMMAR_PATH),
                    protocol: None,
                    protocol_path: None,
                    hooks: semio_framework_dsl::passthrough_hooks("puzzle.puzzle5d.diff"),
                },
                semio_framework_dsl::LanguageSpec {
                    id: "5d.pack",
                    extension: None,
                    role: semio_framework_dsl::LanguageRole::Pack,
                    grammar: None,
                    grammar_path: None,
                    protocol: Some(crate::standards::v1::subsets::any::io::binary::snapshot::COMPONENT_PROTOCOL_SEMIO),
                    protocol_path: Some(crate::standards::v1::subsets::any::io::binary::snapshot::COMPONENT_PROTOCOL_PATH),
                    hooks: semio_framework_dsl::passthrough_hooks("5d.pack"),
                },
                semio_framework_dsl::LanguageSpec {
                    id: "5d.spr",
                    extension: None,
                    role: semio_framework_dsl::LanguageRole::Spr,
                    grammar: None,
                    grammar_path: None,
                    protocol: Some(crate::standards::v1::subsets::any::io::binary::mutations::COMPONENT_PROTOCOL_SEMIO),
                    protocol_path: Some(crate::standards::v1::subsets::any::io::binary::mutations::COMPONENT_PROTOCOL_PATH),
                    hooks: semio_framework_dsl::passthrough_hooks("5d.spr"),
                },
            ]
        })
        .as_slice()
}
//#endregion 🔖️Declaration
}
//#endregion 🧩️ArtifactItems5
