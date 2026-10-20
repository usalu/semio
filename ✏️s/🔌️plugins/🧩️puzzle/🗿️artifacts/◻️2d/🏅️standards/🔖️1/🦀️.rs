//! 🏔️ `📓️terra-fleet-trinity-recipe-report.md`), mounts subset `any`. `mimes`/`extensions`: no real MIME
//! 🏅️ Standard root — `standard() -> StandardDeclaration` (`terra-fleet-trinity-recipe` recipe,
//! registration exists for `s.puzzle2d` outside the old `ArtifactDefinition`'s capability rows
//! (`definition()`, kept — debt D1) — that channel only ever claimed a codec id
//! (`board.ports.directed.v1:puzzle2d-play`) and an extension (`puzzle2d-play`, the EDITOR's own
//! `Puzzle2dPlaySnapshot::EXTENSION`, not the base `Puzzle2dSnapshot`'s `"puzzle2d"` — see this
//! artifact root's own `definition()` D2 comment), never a mime type.
//! `mimes: ["application/vnd.semio.puzzle2d+json"]` is a documented synthesis (matches `🗒️note`/
//! `🖍️draw`/`🔱️trinity`'s identical documented deviation); `extensions: ["puzzle2d-play"]` is the
//! real, carried-over value from `definition()`'s `s.puzzle2d.codec.document-1` row.

use crate::standards::v1::subsets;
use semio_framework_plugin::app::declarations::MediaDeclaration;
use semio_framework_plugin::app::declarations::StandardDeclaration;
use {semio_framework_artifact_reference::StandardId};

pub fn standard<PA: crate::ArtifactApps>() -> StandardDeclaration<PA> {
    StandardDeclaration { id: StandardId("1"), media: MediaDeclaration { mimes: &["application/vnd.semio.puzzle2d+json"], extensions: &["puzzle2d-play"] }, subsets: vec![subsets::any::subset::<PA>()] }
}

//#region 🧩️ArtifactItems2
pub mod artifact_items_2 {
//#region 🔖️Declaration
/// 🔖️ Puzzle2d's declaration (ticket 26/08/12/ARTIFACTS-ONLY-PLUGIN-ARCHITECTURE M1, relocated off
/// `⚙️engine` to the artifact root — `declaration()` describes the artifact itself, not engine
/// behaviour) — replaces the puzzle2d slice of the old umbrella `register()`, which also drove
/// puzzle3d's and puzzle5d's own registration (see their sibling `declaration()`s).
///
/// **W1d update.** `register_app_schemas()` is GONE — it was never a genuine coverage gap, just
/// category-1 app-scope schema under a different name; `Puzzle2dPlayApp::app_schema()` (see that
/// impl's own doc) now covers it, declared automatically the moment `Puzzle2dPlayApp` is bound to the
/// plugin root via `.editor::<crate::editor::puzzle2d::Puzzle2dPlayApp>(…)` (ticket
/// 26/08/16/ARTIFACT-VIEWERS-AND-EDITORS-PER-SUBSET: `.document_app()` is retired in favor of the
/// role-split `.editor()`/`.viewer()` builder methods).
/// `register_media_io()` (`register_2d_export_handlers`/`register_dwg_import_handler`, now on
/// `crate::editor::puzzle2d::register_media_io` — ticket 26/08/12/ARTIFACTS-ONLY-PLUGIN-ARCHITECTURE
/// W1e moved it off the deleted `⚙️engine` to the app that owns its callback bodies) is a genuinely
/// DIFFERENT case and is still kept on `🧩️puzzle/🦀️.rs`'s own `.setup()` — it is the OS
/// media-host registry, an entirely separate 14-function family
/// (`register_2d_export_handlers`/`register_mesh_exporter`/…) from the nine §6 registrars
/// `ArtifactDeclaration` covers, keyed by a legacy OS-kind string this declaration's own `kind` isn't
/// — see `🧩️puzzle/🦀️.rs`'s `plugin()` doc for the full judgement.
pub trait ArtifactApps:
    semio_framework_plugin::PluginApp
    + From<semio_framework_plugin::app::VcsArtifactApp<semio_framework_plugin::app::EditorApp<crate::editor::puzzle2d::Puzzle2dPlayApp>>>
    + From<semio_framework_plugin::app::VcsArtifactApp<semio_framework_plugin::app::ViewerApp<crate::viewer::puzzle2d::Puzzle2dViewer>>>
{
}

impl<PA> ArtifactApps for PA where
    PA: semio_framework_plugin::PluginApp
        + From<semio_framework_plugin::app::VcsArtifactApp<semio_framework_plugin::app::EditorApp<crate::editor::puzzle2d::Puzzle2dPlayApp>>>
        + From<semio_framework_plugin::app::VcsArtifactApp<semio_framework_plugin::app::ViewerApp<crate::viewer::puzzle2d::Puzzle2dViewer>>>
{
}

pub fn definition() -> Result<semio_framework_plugin::ArtifactDefinition, semio_framework_plugin::ArtifactDefinitionError> {
    use semio_framework_plugin::{ArtifactCapability, ArtifactCapabilityKind, ArtifactDefinition, ArtifactIdentity, ArtifactIdentityClaim, ArtifactIdentityNamespace, ArtifactLocale, ArtifactLocalization};

    let rows: &[semio_framework_plugin::ArtifactCapabilityRow<'_>] = &[
        ("s.puzzle.puzzle2d.standard.v1", "standard", "1", &[], None),
        ("s.puzzle.puzzle2d.standard.v1.profile.any", "profile", "any", &[], None),
        ("s.puzzle.puzzle2d.schema.artifact", "schema", "s.puzzle.puzzle2d", &[("schema", "s.puzzle.puzzle2d")], None),
        ("s.puzzle.puzzle2d.inference.artifact", "inference", "s.puzzle.puzzle2d.inference", &[("schema", "s.puzzle.puzzle2d.inference")], None),
        ("s.puzzle.puzzle2d.composer.native", "composer", "s.puzzle.puzzle2d@1/*", &[("dialect", "s.puzzle.puzzle2d@1/*")], None),
        ("s.puzzle.puzzle2d.composer.format-1", "composer", "s.stdio.svg@1.1/*", &[("dialect", "s.stdio.svg@1.1/*")], None),
        ("s.puzzle.puzzle2d.composer.format-2", "composer", "s.stdio.pdf@1.4/*", &[("dialect", "s.stdio.pdf@1.4/*")], None),
        ("s.puzzle.puzzle2d.composer.format-3", "composer", "s.stdio.png@1.2/*", &[("dialect", "s.stdio.png@1.2/*")], None),
        ("s.puzzle.puzzle2d.composer.format-4", "composer", "s.stdio.json@rfc8259/*", &[("dialect", "s.stdio.json@rfc8259/*")], None),
        ("s.puzzle.puzzle2d.composer.format-5", "composer", "s.stdio.dwg@ac1018/*", &[("dialect", "s.stdio.dwg@ac1018/*")], None),
        ("s.puzzle.puzzle2d.composer.format-6", "composer", "s.stdio.dxf@r12/*", &[("dialect", "s.stdio.dxf@r12/*")], None),
        ("s.puzzle.puzzle2d.grammar.1", "grammar", "puzzle.puzzle2d", &[("grammar", "puzzle.puzzle2d")], None),
        ("s.puzzle.puzzle2d.grammar.2", "grammar", "puzzle.puzzle2d.op", &[("grammar", "puzzle.puzzle2d.op")], None),
        ("s.puzzle.puzzle2d.grammar.3", "grammar", "puzzle.puzzle2d.diff", &[("grammar", "puzzle.puzzle2d.diff")], None),
        ("s.puzzle.puzzle2d.grammar.4", "grammar", "2d.pack", &[("grammar", "2d.pack")], None),
        ("s.puzzle.puzzle2d.grammar.5", "grammar", "2d.spr", &[("grammar", "2d.spr")], None),
        ("s.puzzle.puzzle2d.codec.document-1", "codec", "board.ports.directed.v1:puzzle2d-play", &[("codec", "board.ports.directed.v1"), ("codec-extension", "17:board.ports.directed.v1:puzzle2d-play")], None),
        ("s.puzzle.puzzle2d.localization.en", "localization", "2D Puzzle", &[], Some(("en", "2D Puzzle"))),
        ("s.puzzle.puzzle2d.localization.de", "localization", "2D-Puzzle", &[], Some(("de", "2D-Puzzle"))),
    ];
    let mut definition = ArtifactDefinition::new(ArtifactIdentity::parse("s.puzzle.puzzle2d")?);
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
    ArtifactDeclaration { kind: ArtifactKindId::parse("s.puzzle.puzzle2d").expect("canonical puzzle2d kind"), localization: &[], standards: vec![crate::standards::v1::standard::<PA>()] }
}

/// 📌️ Handcrafted facet grammars (text) and protocols (binary) for in-process execution — built once
/// and leaked to a `&'static` slice since `dsl::passthrough_hooks` isn't `const fn`.
pub fn pilot_languages() -> &'static [semio_framework_dsl::LanguageSpec] {
    static LANGUAGES: std::sync::OnceLock<Vec<semio_framework_dsl::LanguageSpec>> = std::sync::OnceLock::new();
    LANGUAGES
        .get_or_init(|| {
            vec![
                semio_framework_dsl::LanguageSpec {
                    id: "puzzle.puzzle2d",
                    extension: Some("puzzle2d"),
                    role: semio_framework_dsl::LanguageRole::Document,
                    grammar: Some(crate::standards::v1::subsets::any::io::text::snapshot::COMPONENT_GRAMMAR_SEMIO),
                    grammar_path: Some(crate::standards::v1::subsets::any::io::text::snapshot::COMPONENT_GRAMMAR_PATH),
                    protocol: Some(crate::standards::v1::subsets::any::io::binary::snapshot::COMPONENT_PROTOCOL_SEMIO),
                    protocol_path: Some(crate::standards::v1::subsets::any::io::binary::snapshot::COMPONENT_PROTOCOL_PATH),
                    hooks: semio_framework_dsl::passthrough_hooks("puzzle.puzzle2d"),
                },
                semio_framework_dsl::LanguageSpec {
                    id: "puzzle.puzzle2d.op",
                    extension: None,
                    role: semio_framework_dsl::LanguageRole::Ops,
                    grammar: Some(crate::standards::v1::subsets::any::io::text::mutations::COMPONENT_GRAMMAR_SEMIO),
                    grammar_path: Some(crate::standards::v1::subsets::any::io::text::mutations::COMPONENT_GRAMMAR_PATH),
                    protocol: Some(crate::standards::v1::subsets::any::io::binary::mutations::COMPONENT_PROTOCOL_SEMIO),
                    protocol_path: Some(crate::standards::v1::subsets::any::io::binary::mutations::COMPONENT_PROTOCOL_PATH),
                    hooks: semio_framework_dsl::passthrough_hooks("puzzle.puzzle2d.op"),
                },
                semio_framework_dsl::LanguageSpec {
                    id: "puzzle.puzzle2d.diff",
                    extension: None,
                    role: semio_framework_dsl::LanguageRole::Diff,
                    grammar: Some(crate::standards::v1::subsets::any::io::text::diff::COMPONENT_GRAMMAR_SEMIO),
                    grammar_path: Some(crate::standards::v1::subsets::any::io::text::diff::COMPONENT_GRAMMAR_PATH),
                    protocol: None,
                    protocol_path: None,
                    hooks: semio_framework_dsl::passthrough_hooks("puzzle.puzzle2d.diff"),
                },
                semio_framework_dsl::LanguageSpec {
                    id: "2d.pack",
                    extension: None,
                    role: semio_framework_dsl::LanguageRole::Pack,
                    grammar: None,
                    grammar_path: None,
                    protocol: Some(crate::standards::v1::subsets::any::io::binary::snapshot::COMPONENT_PROTOCOL_SEMIO),
                    protocol_path: Some(crate::standards::v1::subsets::any::io::binary::snapshot::COMPONENT_PROTOCOL_PATH),
                    hooks: semio_framework_dsl::passthrough_hooks("2d.pack"),
                },
                semio_framework_dsl::LanguageSpec {
                    id: "2d.spr",
                    extension: None,
                    role: semio_framework_dsl::LanguageRole::Spr,
                    grammar: None,
                    grammar_path: None,
                    protocol: Some(crate::standards::v1::subsets::any::io::binary::mutations::COMPONENT_PROTOCOL_SEMIO),
                    protocol_path: Some(crate::standards::v1::subsets::any::io::binary::mutations::COMPONENT_PROTOCOL_PATH),
                    hooks: semio_framework_dsl::passthrough_hooks("2d.spr"),
                },
            ]
        })
        .as_slice()
}
//#endregion 🔖️Declaration
}
//#endregion 🧩️ArtifactItems2
