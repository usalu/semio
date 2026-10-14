//! 🧩️ Puzzle 3d artifact — the `puzzle.3d` document schema: the `Puzzle3dSnapshot`
//! (schema/domain/meta/objects/attractions/targetVolumes/references), its object/vortex/attraction/
//! target-volume/reference/kind-catalog records, the `Puzzle3dScale` scalar-or-triple pose scale, the
//! shared `Puzzle3dError`, and the `artifact_kind()` spec the play app's manifest binds. Sibling
//! nodes: `🔺️diff`, `🔧️op`, `🗣️dsl`, `🎒️pack`, `📡️spr`, `⚙️engine`.

#![allow(clippy::result_large_err)]

#[path = "🤖️generated/📇️registry/🦀️.rs"]
pub mod graph_manifest;
extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_os_kernel as vcs;
extern crate semio_framework_value_derive as value_derive;

//#region 🔖️Declaration
/// 🔖️ Puzzle3d's declaration (ticket 26/08/12/ARTIFACTS-ONLY-PLUGIN-ARCHITECTURE M1, relocated off
/// the former `⚙️engine` to the artifact root — `declaration()` describes the artifact itself, never
/// engine/app behaviour) — replaces the `ComposerEntry` half of the old `register_io()`. The
/// `"3d.puzzle"` OS-host mesh export/import bridge (`crate::editor::puzzle3d::register_mesh_io()`, moved
/// app-side by ticket 26/08/12/ENGINELESS-ARTIFACTS-AND-APP-STATE-MACHINES) has NO `ArtifactDeclaration`
/// field — it belongs to the same OS media-host 14-function family flagged on puzzle2d's
/// `declaration()` doc, a different mechanism from the nine §6 registrars this struct covers — so it
/// stays wired through `🧩️puzzle/🦀️.rs`'s own `.setup()`, not here.
pub trait ArtifactApps:
    semio_framework_plugin::PluginApp
    + From<semio_framework_plugin::app::VcsArtifactApp<semio_framework_plugin::app::EditorApp<semio_s_artifact_puzzle_3d_editor::Puzzle3dPlayApp>>>
    + From<semio_framework_plugin::app::VcsArtifactApp<semio_framework_plugin::app::ViewerApp<semio_s_artifact_puzzle_3d_viewer::Puzzle3dViewer>>>
{
}

impl<PA> ArtifactApps for PA where
    PA: semio_framework_plugin::PluginApp
        + From<semio_framework_plugin::app::VcsArtifactApp<semio_framework_plugin::app::EditorApp<semio_s_artifact_puzzle_3d_editor::Puzzle3dPlayApp>>>
        + From<semio_framework_plugin::app::VcsArtifactApp<semio_framework_plugin::app::ViewerApp<semio_s_artifact_puzzle_3d_viewer::Puzzle3dViewer>>>
{
}

pub fn definition() -> Result<semio_framework_plugin::ArtifactDefinition, semio_framework_plugin::ArtifactDefinitionError> {
use semio_framework_plugin::{ArtifactCapability, ArtifactCapabilityKind, ArtifactDefinition, ArtifactIdentity, ArtifactIdentityClaim, ArtifactIdentityNamespace, ArtifactLocale, ArtifactLocalization};

    let rows: &[semio_framework_plugin::ArtifactCapabilityRow<'_>] = &[
        ("s.puzzle.puzzle3d.standard.v1", "standard", "1", &[], None),
        ("s.puzzle.puzzle3d.standard.v1.profile.any", "profile", "any", &[], None),
        ("s.puzzle.puzzle3d.schema.artifact", "schema", "s.puzzle.puzzle3d", &[("schema", "s.puzzle.puzzle3d")], None),
        ("s.puzzle.puzzle3d.inference.artifact", "inference", "s.puzzle.puzzle3d.inference", &[("schema", "s.puzzle.puzzle3d.inference")], None),
        ("s.puzzle.puzzle3d.composer.native", "composer", "s.puzzle.puzzle3d@1/*", &[("dialect", "s.puzzle.puzzle3d@1/*")], None),
        ("s.puzzle.puzzle3d.composer.format-4", "composer", "s.stdio.json@rfc8259/*", &[("dialect", "s.stdio.json@rfc8259/*")], None),
        ("s.puzzle.puzzle3d.grammar.1", "grammar", "puzzle.puzzle3d", &[("grammar", "puzzle.puzzle3d")], None),
        ("s.puzzle.puzzle3d.grammar.2", "grammar", "puzzle.puzzle3d.op", &[("grammar", "puzzle.puzzle3d.op")], None),
        ("s.puzzle.puzzle3d.grammar.3", "grammar", "puzzle.puzzle3d.diff", &[("grammar", "puzzle.puzzle3d.diff")], None),
        ("s.puzzle.puzzle3d.grammar.4", "grammar", "3d.pack", &[("grammar", "3d.pack")], None),
        ("s.puzzle.puzzle3d.grammar.5", "grammar", "3d.spr", &[("grammar", "3d.spr")], None),
        // 🐛️ D2-capability-claim-repairs: `.document_codec::<EditorApp<Puzzle3dPlayApp>>()` derives
        // its extension claim from `<Puzzle3dPlaySnapshot as pack::snapshot::ArtifactDsl>::EXTENSION`
        // (`…/🧬️mutations/🦀️.rs`, the editor's real `Snapshot` type), which is
        // `"puzzle3d-play"`, not the base `Puzzle3dSnapshot`'s `"puzzle3d"`.
        ("s.puzzle.puzzle3d.codec.document-1", "codec", "puzzle.3d:puzzle3d-play", &[("codec", "puzzle.3d"), ("codec-extension", "9:puzzle.3d:puzzle3d-play")], None),
        ("s.puzzle.puzzle3d.localization.en", "localization", "3D Puzzle", &[], Some(("en", "3D Puzzle"))),
        ("s.puzzle.puzzle3d.localization.de", "localization", "3D-Puzzle", &[], Some(("de", "3D-Puzzle"))),
    ];
    let mut definition = ArtifactDefinition::new(ArtifactIdentity::parse("s.puzzle.puzzle3d")?);
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
    ArtifactDeclaration { kind: ArtifactKindId::parse("s.puzzle.puzzle3d").expect("canonical puzzle3d kind"), localization: &[], standards: vec![standards::v1::standard::<PA>()] }
}

/// 📌️ Handcrafted facet grammars (text) and protocols (binary) for in-process execution — built once
/// and leaked to a `&'static` slice since `dsl::passthrough_hooks` isn't `const fn`. This function
/// existed as a side-effecting `register_pilot_languages()` before M1 but was never called from
/// anywhere (dead code, confirmed by grep) — wiring it into `declaration()`'s `.languages(...)` is
/// this conversion's one real bug fix: puzzle3d's own grammars were never actually registered.
pub fn pilot_languages() -> &'static [semio_framework_dsl::LanguageSpec] {
    static LANGUAGES: std::sync::OnceLock<Vec<semio_framework_dsl::LanguageSpec>> = std::sync::OnceLock::new();
    LANGUAGES
        .get_or_init(|| {
            vec![
                semio_framework_dsl::LanguageSpec {
                    id: "puzzle.puzzle3d",
                    extension: Some("puzzle3d"),
                    role: semio_framework_dsl::LanguageRole::Document,
                    grammar: Some(semio_s_artifact_puzzle_3d_model::io::text::snapshot::COMPONENT_GRAMMAR_SEMIO),
                    grammar_path: Some(semio_s_artifact_puzzle_3d_model::io::text::snapshot::COMPONENT_GRAMMAR_PATH),
                    protocol: Some(semio_s_artifact_puzzle_3d_model::io::binary::snapshot::COMPONENT_PROTOCOL_SEMIO),
                    protocol_path: Some(semio_s_artifact_puzzle_3d_model::io::binary::snapshot::COMPONENT_PROTOCOL_PATH),
                    hooks: semio_framework_dsl::passthrough_hooks("puzzle.puzzle3d"),
                },
                semio_framework_dsl::LanguageSpec {
                    id: "puzzle.puzzle3d.op",
                    extension: None,
                    role: semio_framework_dsl::LanguageRole::Ops,
                    grammar: Some(semio_s_artifact_puzzle_3d_mutations::io::text::mutations::COMPONENT_GRAMMAR_SEMIO),
                    grammar_path: Some(semio_s_artifact_puzzle_3d_mutations::io::text::mutations::COMPONENT_GRAMMAR_PATH),
                    protocol: Some(semio_s_artifact_puzzle_3d_mutations::io::binary::mutations::COMPONENT_PROTOCOL_SEMIO),
                    protocol_path: Some(semio_s_artifact_puzzle_3d_mutations::io::binary::mutations::COMPONENT_PROTOCOL_PATH),
                    hooks: semio_framework_dsl::passthrough_hooks("puzzle.puzzle3d.op"),
                },
                semio_framework_dsl::LanguageSpec {
                    id: "puzzle.puzzle3d.diff",
                    extension: None,
                    role: semio_framework_dsl::LanguageRole::Diff,
                    grammar: Some(semio_s_artifact_puzzle_3d_diff::io::text::diff::COMPONENT_GRAMMAR_SEMIO),
                    grammar_path: Some(semio_s_artifact_puzzle_3d_diff::io::text::diff::COMPONENT_GRAMMAR_PATH),
                    protocol: None,
                    protocol_path: None,
                    hooks: semio_framework_dsl::passthrough_hooks("puzzle.puzzle3d.diff"),
                },
                semio_framework_dsl::LanguageSpec {
                    id: "3d.pack",
                    extension: None,
                    role: semio_framework_dsl::LanguageRole::Pack,
                    grammar: None,
                    grammar_path: None,
                    protocol: Some(semio_s_artifact_puzzle_3d_model::io::binary::snapshot::COMPONENT_PROTOCOL_SEMIO),
                    protocol_path: Some(semio_s_artifact_puzzle_3d_model::io::binary::snapshot::COMPONENT_PROTOCOL_PATH),
                    hooks: semio_framework_dsl::passthrough_hooks("3d.pack"),
                },
                semio_framework_dsl::LanguageSpec {
                    id: "3d.spr",
                    extension: None,
                    role: semio_framework_dsl::LanguageRole::Spr,
                    grammar: None,
                    grammar_path: None,
                    protocol: Some(semio_s_artifact_puzzle_3d_mutations::io::binary::mutations::COMPONENT_PROTOCOL_SEMIO),
                    protocol_path: Some(semio_s_artifact_puzzle_3d_mutations::io::binary::mutations::COMPONENT_PROTOCOL_PATH),
                    hooks: semio_framework_dsl::passthrough_hooks("3d.spr"),
                },
            ]
        })
        .as_slice()
}
//#endregion 🔖️Declaration

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️design-parity-schema/🦀️.rs"]
mod design_parity_schema_tests;
#[cfg(test)]
#[path = "🧪️tests/🔬️manifest-catalog/🦀️.rs"]
mod manifest_catalog_tests;
//#endregion 🧪️Tests

#[path = "."]
pub mod standards {
    #[path = "."]
    pub mod v1 {
        #[path = "🏅️standards/🔖️1/🦀️.rs"]
        mod component;
        pub use component::*;
        #[path = "."]
        pub mod subsets {
            #[path = "."]
            pub mod any {
                #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🦀️.rs"]
                mod component;
                pub use component::*;
            }
        }
    }
}

pub use semio_s_artifact_puzzle_3d_io::{Puzzle3dBuilderConstruction, Puzzle3dParts, Puzzle3dAnalyzerAnalysis, Puzzle3dBuilderFacets, Puzzle3dBuilder, Puzzle3dAnalyzer, Puzzle3dComposer};

