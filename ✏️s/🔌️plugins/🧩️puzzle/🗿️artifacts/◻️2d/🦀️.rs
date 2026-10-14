//! 🧩️ Puzzle 2d artifact — the `board.ports.directed.v1` document schema: the `Puzzle2dSnapshot`
//! (schema/camera/nodes/edges/meta), its node/handle/edge/kind-compatibility records, and the
//! `artifact_kind()` spec the play app's manifest binds. Sibling nodes: `🔺️diff`, `🔧️op`, `🗣️dsl`,
//! `🎒️pack`, `📡️spr`. No `⚙️engine` sibling anymore (ticket
//! 26/08/12/ARTIFACTS-ONLY-PLUGIN-ARCHITECTURE W1e): an artifact is a `🧬️schema` plus a `🚪️io`
//! system, never an engine — the old `⚙️engine`'s pure/document-only pieces moved into `🧬️schema`
//! and `🚪️io` (this file's own `declaration()` and I/O registry), and its genuinely
//! stateful `BoardHost` facade moved to `🎛️apps/◻️2d/⚙️engine` (since relocated again, ticket
//! 26/08/16/ARTIFACT-VIEWERS-AND-EDITORS-PER-SUBSET, to `✏️editor/⚙️engine` under this artifact's own
//! `✏️editor` surface).

#![allow(clippy::result_large_err)]

#[path = "🤖️generated/📇️registry/🦀️.rs"]
pub mod graph_manifest;
extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_os_kernel as vcs;
extern crate semio_framework_value_derive as value_derive;

#[cfg(test)]
#[path="🧪️tests/🔗️source-custody/🦀️.rs"]
pub(crate) mod test_source_custody;

#[path = "../../🔨️modules/🎲️board/🎬️scene/🦀️.rs"]
pub mod board_scene;

#[path = "../../🎮️commands/📤️jobs/🦀️.rs"]
pub mod puzzle_job;

#[path = "../../🎮️commands/🧵️retained/🦀️.rs"]
pub mod retained_command;

#[cfg(test)]
fn retained_command_test_catalog() -> (&'static str, &'static str, &'static [&'static str], &'static str) {
    ("puzzle2d", "board.ports.directed.v1", editor::puzzle2d::PUZZLE2D_RETAINED_TOOL_IDS, include_str!("🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🧫️fixtures/🗄️retained-jobs/🔣️.json"))
}

use semio_framework_value::{list::PagedList, paged::PagedUtf8};



//#region 🔖️Document

















































//#endregion 🔖️Document

//#region 🔖️Snapshot
//#endregion 🔖️Snapshot

//#region 🔖️Dialect

//#endregion 🔖️Dialect

//#region 🔖️ArtifactKind

//#endregion 🔖️ArtifactKind

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
    + From<semio_framework_plugin::app::VcsArtifactApp<semio_framework_plugin::app::EditorApp<editor::puzzle2d::Puzzle2dPlayApp>>>
    + From<semio_framework_plugin::app::VcsArtifactApp<semio_framework_plugin::app::ViewerApp<viewer::puzzle2d::Puzzle2dViewer>>>
{
}

impl<PA> ArtifactApps for PA where
    PA: semio_framework_plugin::PluginApp
        + From<semio_framework_plugin::app::VcsArtifactApp<semio_framework_plugin::app::EditorApp<editor::puzzle2d::Puzzle2dPlayApp>>>
        + From<semio_framework_plugin::app::VcsArtifactApp<semio_framework_plugin::app::ViewerApp<viewer::puzzle2d::Puzzle2dViewer>>>
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
        // 🐛️ D2-capability-claim-repairs: `.document_codec::<EditorApp<Puzzle2dPlayApp>>()` derives
        // its extension claim from `<Puzzle2dPlaySnapshot as pack::snapshot::ArtifactDsl>::EXTENSION`
        // (`…/🧬️mutations/🦀️.rs`, the editor's real `Snapshot` type), which is
        // `"puzzle2d-play"`, not the base `Puzzle2dSnapshot`'s `"puzzle2d"`.
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
    ArtifactDeclaration { kind: ArtifactKindId::parse("s.puzzle.puzzle2d").expect("canonical puzzle2d kind"), localization: &[], standards: vec![standards::v1::standard::<PA>()] }
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
                    grammar: Some(standards::v1::subsets::any::io::text::snapshot::COMPONENT_GRAMMAR_SEMIO),
                    grammar_path: Some(standards::v1::subsets::any::io::text::snapshot::COMPONENT_GRAMMAR_PATH),
                    protocol: Some(standards::v1::subsets::any::io::binary::snapshot::COMPONENT_PROTOCOL_SEMIO),
                    protocol_path: Some(standards::v1::subsets::any::io::binary::snapshot::COMPONENT_PROTOCOL_PATH),
                    hooks: semio_framework_dsl::passthrough_hooks("puzzle.puzzle2d"),
                },
                semio_framework_dsl::LanguageSpec {
                    id: "puzzle.puzzle2d.op",
                    extension: None,
                    role: semio_framework_dsl::LanguageRole::Ops,
                    grammar: Some(standards::v1::subsets::any::io::text::mutations::COMPONENT_GRAMMAR_SEMIO),
                    grammar_path: Some(standards::v1::subsets::any::io::text::mutations::COMPONENT_GRAMMAR_PATH),
                    protocol: Some(standards::v1::subsets::any::io::binary::mutations::COMPONENT_PROTOCOL_SEMIO),
                    protocol_path: Some(standards::v1::subsets::any::io::binary::mutations::COMPONENT_PROTOCOL_PATH),
                    hooks: semio_framework_dsl::passthrough_hooks("puzzle.puzzle2d.op"),
                },
                semio_framework_dsl::LanguageSpec {
                    id: "puzzle.puzzle2d.diff",
                    extension: None,
                    role: semio_framework_dsl::LanguageRole::Diff,
                    grammar: Some(standards::v1::subsets::any::io::text::diff::COMPONENT_GRAMMAR_SEMIO),
                    grammar_path: Some(standards::v1::subsets::any::io::text::diff::COMPONENT_GRAMMAR_PATH),
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
                    protocol: Some(standards::v1::subsets::any::io::binary::snapshot::COMPONENT_PROTOCOL_SEMIO),
                    protocol_path: Some(standards::v1::subsets::any::io::binary::snapshot::COMPONENT_PROTOCOL_PATH),
                    hooks: semio_framework_dsl::passthrough_hooks("2d.pack"),
                },
                semio_framework_dsl::LanguageSpec {
                    id: "2d.spr",
                    extension: None,
                    role: semio_framework_dsl::LanguageRole::Spr,
                    grammar: None,
                    grammar_path: None,
                    protocol: Some(standards::v1::subsets::any::io::binary::mutations::COMPONENT_PROTOCOL_SEMIO),
                    protocol_path: Some(standards::v1::subsets::any::io::binary::mutations::COMPONENT_PROTOCOL_PATH),
                    hooks: semio_framework_dsl::passthrough_hooks("2d.spr"),
                },
            ]
        })
        .as_slice()
}
//#endregion 🔖️Declaration

pub use crate::editor::puzzle2d::snapshot::Puzzle2dPlaySnapshot;

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
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

                #[path = "."]
                pub mod schema {
                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🦀️.rs"]
                    mod component;
                    pub use component::*;
                    #[path = "."]
                    pub mod snapshot {
                        #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🦀️.rs"]
                        mod component;
                        pub use component::*;
                    }
                    #[path = "."]
                    pub mod inferences {
                        #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/🦀️.rs"]
                        mod component;
                        pub use component::*;
                        #[path = "."]
                        pub mod flat_position {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/🎛️flat-position/🦀️.rs"]
                            mod component;
                            pub use component::*;
                        }
                    }
                    #[path = "."]
                    pub mod diff {
                        #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔺️diff/🦀️.rs"]
                        mod component;
                        pub use component::*;
                    }
                    #[path = "."]
                    pub mod mutations {
                        #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️.rs"]
                        mod component;
                        pub use component::*;
                        #[path = "."]
                        pub mod create_node {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🟢️node/🌱create-node/🦀️.rs"]
                            mod component;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🟢️node/🌱create-node/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🟢️node/🌱create-node/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            pub use component::*;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🟢️node/🌱create-node/🧪️tests/🧱️zero-width-node/🦀️.rs"]
                            mod tests_zero_width_node;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🟢️node/🌱create-node/🧪️tests/🌱️appends/🦀️.rs"]
                            mod tests_appends_a_capsule_to_the_tower;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🟢️node/🌱create-node/🧪️tests/🌱️appends-node-c/🦀️.rs"]
                            mod tests_appends_node_c;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🟢️node/🌱create-node/🧪️tests/🚫️rejects/🦀️.rs"]
                            mod tests_rejects_a_capsule_id_the_tower_already_holds;
                        }
                        #[path = "."]
                        pub mod delete_node {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🟢️node/🗑️delete-node/🦀️.rs"]
                            mod component;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🟢️node/🗑️delete-node/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🟢️node/🗑️delete-node/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            pub use component::*;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🟢️node/🗑️delete-node/🧪️tests/🚫️deletes/🦀️.rs"]
                            mod tests_deletes_the_tambour_and_severs_its_ten_edges;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🟢️node/🗑️delete-node/🧪️tests/🚫️rejects/🦀️.rs"]
                            mod tests_rejects_deleting_a_capsule_the_board_never_held;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🟢️node/🗑️delete-node/🧪️tests/🚫️removes/🦀️.rs"]
                            mod tests_removes_node_a_and_severs_edge;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🟢️node/🗑️delete-node/🧪️tests/🗑️removes-middle-node/🦀️.rs"]
                            mod tests_removes_the_middle_node_and_severs_its_edges;
                        }
                        #[path = "."]
                        pub mod move_node {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧭️pose/📍move-node/🦀️.rs"]
                            mod component;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧭️pose/📍move-node/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧭️pose/📍move-node/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            pub use component::*;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧭️pose/📍move-node/🧪️tests/📍️moves/🦀️.rs"]
                            mod tests_moves_a_capsule_across_the_shaft;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧭️pose/📍move-node/🧪️tests/📍️moves-node-a/🦀️.rs"]
                            mod tests_moves_node_a;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧭️pose/📍move-node/🧪️tests/🚫️rejects/🦀️.rs"]
                            mod tests_rejects_moving_a_capsule_the_board_never_held;
                        }
                        #[path = "."]
                        pub mod replace_node_geometry {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🟢️node/🧊replace-node-geometry/🦀️.rs"]
                            mod component;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🟢️node/🧊replace-node-geometry/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🟢️node/🧊replace-node-geometry/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            pub use component::*;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🟢️node/🧊replace-node-geometry/🧪️tests/🧱️negative-radius/🦀️.rs"]
                            mod tests_negative_radius;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🟢️node/🧊replace-node-geometry/🧪️tests/🔳️circle/🦀️.rs"]
                            mod tests_circle_to_rectangle;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🟢️node/🧊replace-node-geometry/🧪️tests/🚫️rejects/🦀️.rs"]
                            mod tests_rejects_reshaping_a_capsule_the_board_never_held;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🟢️node/🧊replace-node-geometry/🧪️tests/🔳️squares/🦀️.rs"]
                            mod tests_squares_the_concrete_forest_seed;
                        }
                        #[path = "."]
                        pub mod change_node_kind {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🟢️node/🏗️change-node-kind/🦀️.rs"]
                            mod component;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🟢️node/🏗️change-node-kind/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🟢️node/🏗️change-node-kind/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            pub use component::*;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🟢️node/🏗️change-node-kind/🧪️tests/🏷️reassigns/🦀️.rs"]
                            mod tests_reassigns_node_a_kind;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🟢️node/🏗️change-node-kind/🧪️tests/🚫️rejects/🦀️.rs"]
                            mod tests_rejects_rekinding_a_capsule_the_board_never_held;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🟢️node/🏗️change-node-kind/🧪️tests/🏗️rekinds/🦀️.rs"]
                            mod tests_rekinds_a_capsule_j_as_a_capsule_l;
                        }
                        #[path = "."]
                        pub mod edit_node_text {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🟢️node/✏️edit-node-text/🦀️.rs"]
                            mod component;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🟢️node/✏️edit-node-text/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🟢️node/✏️edit-node-text/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            pub use component::*;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🟢️node/✏️edit-node-text/🧪️tests/✏️recodes/🦀️.rs"]
                            mod tests_recodes_a_capsule_id_code;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🟢️node/✏️edit-node-text/🧪️tests/🚫️rejects/🦀️.rs"]
                            mod tests_rejects_recoding_a_capsule_the_board_never_held;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🟢️node/✏️edit-node-text/🧪️tests/✏️retitles-node-a/🦀️.rs"]
                            mod tests_retitles_node_a;
                        }
                        #[path = "."]
                        pub mod change_node_icon {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🟢️node/🎨change-node-icon/🦀️.rs"]
                            mod component;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🟢️node/🎨change-node-icon/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🟢️node/🎨change-node-icon/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            pub use component::*;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🟢️node/🎨change-node-icon/🧪️tests/🚫️rejects/🦀️.rs"]
                            mod tests_rejects_reiconing_a_capsule_the_board_never_held;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🟢️node/🎨change-node-icon/🧪️tests/🎨️swaps-a-capsule-icon/🦀️.rs"]
                            mod tests_swaps_a_capsule_icon;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🟢️node/🎨change-node-icon/🧪️tests/🎨️swaps-node-a-icon/🦀️.rs"]
                            mod tests_swaps_node_a_icon;
                        }
                        #[path = "."]
                        pub mod scale_node {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧭️pose/📏scale-node/🦀️.rs"]
                            mod component;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧭️pose/📏scale-node/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧭️pose/📏scale-node/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            pub use component::*;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧭️pose/📏scale-node/🧪️tests/🧱️zero-scale/🦀️.rs"]
                            mod tests_zero_scale;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧭️pose/📏scale-node/🧪️tests/📏️doubles-node-a/🦀️.rs"]
                            mod tests_doubles_node_a;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧭️pose/📏scale-node/🧪️tests/🚫️rejects/🦀️.rs"]
                            mod tests_rejects_scaling_a_capsule_the_board_never_held;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧭️pose/📏scale-node/🧪️tests/📏️scales/🦀️.rs"]
                            mod tests_scales_a_capsule_by_three_halves;
                        }
                        #[path = "."]
                        pub mod change_node_visible {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🟢️node/👁️change-node-visible/🦀️.rs"]
                            mod component;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🟢️node/👁️change-node-visible/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🟢️node/👁️change-node-visible/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            pub use component::*;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🟢️node/👁️change-node-visible/🧪️tests/🙈️hides-a-capsule/🦀️.rs"]
                            mod tests_hides_a_capsule;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🟢️node/👁️change-node-visible/🧪️tests/🙈️hides-node-a/🦀️.rs"]
                            mod tests_hides_node_a;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🟢️node/👁️change-node-visible/🧪️tests/🚫️rejects/🦀️.rs"]
                            mod tests_rejects_hiding_a_capsule_the_board_never_held;
                        }
                        #[path = "."]
                        pub mod change_node_locked {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🟢️node/🔒change-node-locked/🦀️.rs"]
                            mod component;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🟢️node/🔒change-node-locked/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🟢️node/🔒change-node-locked/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            pub use component::*;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🟢️node/🔒change-node-locked/🧪️tests/🔒️locks-node-a/🦀️.rs"]
                            mod tests_locks_node_a;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🟢️node/🔒change-node-locked/🧪️tests/🔒️locks/🦀️.rs"]
                            mod tests_locks_the_first_storey_tambour;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🟢️node/🔒change-node-locked/🧪️tests/🚫️rejects/🦀️.rs"]
                            mod tests_rejects_locking_a_capsule_the_board_never_held;
                        }
                        #[path = "."]
                        pub mod change_node_root {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🟢️node/🌟change-node-root/🦀️.rs"]
                            mod component;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🟢️node/🌟change-node-root/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🟢️node/🌟change-node-root/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            pub use component::*;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🟢️node/🌟change-node-root/🧪️tests/🌳️promotes-node-a-to-root/🦀️.rs"]
                            mod tests_promotes_node_a_to_root;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🟢️node/🌟change-node-root/🧪️tests/🌳️promotes/🦀️.rs"]
                            mod tests_promotes_the_base_to_root;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🟢️node/🌟change-node-root/🧪️tests/🚫️rejects/🦀️.rs"]
                            mod tests_rejects_rooting_a_capsule_the_board_never_held;
                        }
                        #[path = "."]
                        pub mod change_node_anchor {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🟢️node/⚓change-node-anchor/🦀️.rs"]
                            mod component;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🟢️node/⚓change-node-anchor/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🟢️node/⚓change-node-anchor/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            pub use component::*;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🟢️node/⚓change-node-anchor/🧪️tests/⚓️derives/🦀️.rs"]
                            mod tests_derives_a_capsule_pose_from_its_door_edge;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🟢️node/⚓change-node-anchor/🧪️tests/⚓️fixed-to-derived/🦀️.rs"]
                            mod tests_fixed_to_derived;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🟢️node/⚓change-node-anchor/🧪️tests/🚫️rejects/🦀️.rs"]
                            mod tests_rejects_anchoring_a_capsule_the_board_never_held;
                        }
                        #[path = "."]
                        pub mod add_node_handle {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔌️handle/➕add-node-handle/🦀️.rs"]
                            mod component;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔌️handle/➕add-node-handle/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔌️handle/➕add-node-handle/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            pub use component::*;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔌️handle/➕add-node-handle/🧪️tests/🧱️zero-radius-handle/🦀️.rs"]
                            mod tests_zero_radius_handle;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔌️handle/➕add-node-handle/🧪️tests/➕️adds/🦀️.rs"]
                            mod tests_adds_a_third_slot_door_to_the_tambour;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔌️handle/➕add-node-handle/🧪️tests/➕️appends-handle-3-to-node-b/🦀️.rs"]
                            mod tests_appends_handle_3_to_node_b;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔌️handle/➕add-node-handle/🧪️tests/🚫️rejects/🦀️.rs"]
                            mod tests_rejects_adding_a_door_to_a_capsule_the_board_never_held;
                        }
                        #[path = "."]
                        pub mod remove_node_handle {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔌️handle/➖remove-node-handle/🦀️.rs"]
                            mod component;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔌️handle/➖remove-node-handle/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔌️handle/➖remove-node-handle/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            pub use component::*;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔌️handle/➖remove-node-handle/🧪️tests/🚫️rejects/🦀️.rs"]
                            mod tests_rejects_removing_a_door_the_tambour_never_had;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔌️handle/➖remove-node-handle/🧪️tests/🚫️removes/🦀️.rs"]
                            mod tests_removes_a_tambour_door_and_severs_its_capsule_edge;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔌️handle/➖remove-node-handle/🧪️tests/🚫️removes2/🦀️.rs"]
                            mod tests_removes_handle_2_and_severs_edge;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔌️handle/➖remove-node-handle/🧪️tests/➖removes-middle-handle/🦀️.rs"]
                            mod tests_removes_the_middle_handle_and_severs_its_edge;
                        }
                        #[path = "."]
                        pub mod replace_node_handle {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔌️handle/🔌replace-node-handle/🦀️.rs"]
                            mod component;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔌️handle/🔌replace-node-handle/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔌️handle/🔌replace-node-handle/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            pub use component::*;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔌️handle/🔌replace-node-handle/🧪️tests/🧱️negative-scale/🦀️.rs"]
                            mod tests_negative_scale;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔌️handle/🔌replace-node-handle/🧪️tests/🚫️rejects/🦀️.rs"]
                            mod tests_rejects_replacing_a_door_the_tambour_never_had;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔌️handle/🔌replace-node-handle/🧪️tests/🔌️rekinds/🦀️.rs"]
                            mod tests_rekinds_an_unconnected_tambour_door;
                        }
                        #[path = "."]
                        pub mod connect_handles {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔌️handle/🪢️connect-handles/🦀️.rs"]
                            mod component;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔌️handle/🪢️connect-handles/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔌️handle/🪢️connect-handles/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            pub use component::*;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔌️handle/🪢️connect-handles/🧪️tests/🪢️adds-second-edge/🦀️.rs"]
                            mod tests_adds_second_edge;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔌️handle/🪢️connect-handles/🧪️tests/⏸️keeps/🦀️.rs"]
                            mod tests_keeps_an_edge_the_tower_already_holds;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔌️handle/🪢️connect-handles/🧪️tests/🪢️rewires/🦀️.rs"]
                            mod tests_rewires_the_capsule_the_subgraph_left_loose;
                        }
                        #[path = "."]
                        pub mod disconnect_handles {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔌️handle/✂️disconnect-handles/🦀️.rs"]
                            mod component;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔌️handle/✂️disconnect-handles/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔌️handle/✂️disconnect-handles/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            pub use component::*;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔌️handle/✂️disconnect-handles/🧪️tests/🚫️rejects/🦀️.rs"]
                            mod tests_rejects_severing_an_edge_the_board_never_held;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔌️handle/✂️disconnect-handles/🧪️tests/🚫️removes-edge-1/🦀️.rs"]
                            mod tests_removes_edge_1;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔌️handle/✂️disconnect-handles/🧪️tests/✂️severs-middle-edge/🦀️.rs"]
                            mod tests_severs_the_middle_edge;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔌️handle/✂️disconnect-handles/🧪️tests/✂️severs/🦀️.rs"]
                            mod tests_severs_a_capsule_from_the_first_storey_tambour;
                        }
                        #[path = "."]
                        pub mod replace_edge_geometry {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/〰️edge/🧮replace-edge-geometry/🦀️.rs"]
                            mod component;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/〰️edge/🧮replace-edge-geometry/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/〰️edge/🧮replace-edge-geometry/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            pub use component::*;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/〰️edge/🧮replace-edge-geometry/🧪️tests/🚫️rejects/🦀️.rs"]
                            mod tests_rejects_reposing_an_edge_the_board_never_held;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/〰️edge/🧮replace-edge-geometry/🧪️tests/🧮️reposes/🦀️.rs"]
                            mod tests_reposes_a_capsule_door_edge;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/〰️edge/🧮replace-edge-geometry/🧪️tests/📍️repositions-edge-1/🦀️.rs"]
                            mod tests_repositions_edge_1;
                        }
                        #[path = "."]
                        pub mod change_edge_kind {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/〰️edge/🏷️change-edge-kind/🦀️.rs"]
                            mod component;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/〰️edge/🏷️change-edge-kind/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/〰️edge/🏷️change-edge-kind/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            pub use component::*;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/〰️edge/🏷️change-edge-kind/🧪️tests/🏷️kinds/🦀️.rs"]
                            mod tests_kinds_a_capsule_door_edge_as_a_link;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/〰️edge/🏷️change-edge-kind/🧪️tests/🚫️rejects/🦀️.rs"]
                            mod tests_rejects_kinding_an_edge_the_board_never_held;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/〰️edge/🏷️change-edge-kind/🧪️tests/🏷️rekinds-edge-1/🦀️.rs"]
                            mod tests_rekinds_edge_1;
                        }
                        #[path = "."]
                        pub mod change_edge_tips {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/〰️edge/🖇️change-edge-tips/🦀️.rs"]
                            mod component;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/〰️edge/🖇️change-edge-tips/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/〰️edge/🖇️change-edge-tips/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            pub use component::*;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/〰️edge/🖇️change-edge-tips/🧪️tests/🚫️rejects/🦀️.rs"]
                            mod tests_rejects_tipping_an_edge_the_board_never_held;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/〰️edge/🖇️change-edge-tips/🧪️tests/🔀️swaps-edge-1-tips/🦀️.rs"]
                            mod tests_swaps_edge_1_tips;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/〰️edge/🖇️change-edge-tips/🧪️tests/🖇️tips/🦀️.rs"]
                            mod tests_tips_a_capsule_door_edge;
                        }
                        #[path = "."]
                        pub mod change_edge_visible {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/〰️edge/👀change-edge-visible/🦀️.rs"]
                            mod component;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/〰️edge/👀change-edge-visible/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/〰️edge/👀change-edge-visible/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            pub use component::*;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/〰️edge/👀change-edge-visible/🧪️tests/🙈️hides/🦀️.rs"]
                            mod tests_hides_a_capsule_door_edge;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/〰️edge/👀change-edge-visible/🧪️tests/🙈️hides-edge-1/🦀️.rs"]
                            mod tests_hides_edge_1;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/〰️edge/👀change-edge-visible/🧪️tests/🚫️rejects/🦀️.rs"]
                            mod tests_rejects_hiding_an_edge_the_board_never_held;
                        }
                        #[path = "."]
                        pub mod change_edge_locked {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/〰️edge/🔐change-edge-locked/🦀️.rs"]
                            mod component;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/〰️edge/🔐change-edge-locked/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/〰️edge/🔐change-edge-locked/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            pub use component::*;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/〰️edge/🔐change-edge-locked/🧪️tests/🔒️locks-edge-1/🦀️.rs"]
                            mod tests_locks_edge_1;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/〰️edge/🔐change-edge-locked/🧪️tests/🔒️locks/🦀️.rs"]
                            mod tests_locks_the_base_to_tambour_edge;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/〰️edge/🔐change-edge-locked/🧪️tests/🚫️rejects/🦀️.rs"]
                            mod tests_rejects_locking_an_edge_the_board_never_held;
                        }
                        #[path = "."]
                        pub mod change_manifest_id {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📚️catalog/🆔change-manifest-id/🦀️.rs"]
                            mod component;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📚️catalog/🆔change-manifest-id/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📚️catalog/🆔change-manifest-id/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            pub use component::*;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📚️catalog/🆔change-manifest-id/🧪️tests/📦️repoints-manifest/🦀️.rs"]
                            mod tests_repoints_manifest;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📚️catalog/🆔change-manifest-id/🧪️tests/📦️repoints/🦀️.rs"]
                            mod tests_repoints_the_tower_at_its_example_manifest;
                        }
                        #[path = "."]
                        pub mod connect_kind_compatibility {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📚️catalog/🤝connect-kind-compatibility/🦀️.rs"]
                            mod component;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📚️catalog/🤝connect-kind-compatibility/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📚️catalog/🤝connect-kind-compatibility/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            pub use component::*;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📚️catalog/🤝connect-kind-compatibility/🧪️tests/🤝️adds/🦀️.rs"]
                            mod tests_adds_handle_kind_pair;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📚️catalog/🤝connect-kind-compatibility/🧪️tests/🤝️admits/🦀️.rs"]
                            mod tests_admits_the_reverse_tambour_circular_pair;
                        }
                        #[path = "."]
                        pub mod disconnect_kind_compatibility {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📚️catalog/💔disconnect-kind-compatibility/🦀️.rs"]
                            mod component;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📚️catalog/💔disconnect-kind-compatibility/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📚️catalog/💔disconnect-kind-compatibility/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            pub use component::*;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📚️catalog/💔disconnect-kind-compatibility/🧪️tests/🚫️rejects/🦀️.rs"]
                            mod tests_rejects_withdrawing_a_pair_the_relation_never_held;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📚️catalog/💔disconnect-kind-compatibility/🧪️tests/🚫️removes/🦀️.rs"]
                            mod tests_removes_handle_kind_pair;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📚️catalog/💔disconnect-kind-compatibility/🧪️tests/💔️withdraws-middle-row/🦀️.rs"]
                            mod tests_withdraws_the_middle_row;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📚️catalog/💔disconnect-kind-compatibility/🧪️tests/💔️withdraws/🦀️.rs"]
                            mod tests_withdraws_the_tambour_rectangular_pair;
                        }
                        #[path = "."]
                        pub mod replace_kind_catalogs {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📚️catalog/📚replace-kind-catalogs/🦀️.rs"]
                            mod component;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📚️catalog/📚replace-kind-catalogs/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📚️catalog/📚replace-kind-catalogs/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            pub use component::*;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📚️catalog/📚replace-kind-catalogs/🧪️tests/🧱️off-rim-template/🦀️.rs"]
                            mod tests_off_rim_template;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📚️catalog/📚replace-kind-catalogs/🧪️tests/🗑️clears/🦀️.rs"]
                            mod tests_clears_the_installed_handle_catalog;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📚️catalog/📚replace-kind-catalogs/🧪️tests/📇️installs/🦀️.rs"]
                            mod tests_installs_handle_kind_catalog;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📚️catalog/📚replace-kind-catalogs/🧪️tests/📇️installs2/🦀️.rs"]
                            mod tests_installs_the_tower_handle_catalog;
                        }
                        #[path = "."]
                        pub mod create_target_region {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌍️region/🌍create-target-region/🦀️.rs"]
                            mod component;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌍️region/🌍create-target-region/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌍️region/🌍create-target-region/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            pub use component::*;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌍️region/🌍create-target-region/🧪️tests/🌍️appends-region-2/🦀️.rs"]
                            mod tests_appends_region_2;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌍️region/🌍create-target-region/🧪️tests/🌍️paints/🦀️.rs"]
                            mod tests_paints_a_tower_footprint;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌍️region/🌍create-target-region/🧪️tests/🚫️rejects/🦀️.rs"]
                            mod tests_rejects_a_region_id_the_board_already_holds;
                        }
                        #[path = "."]
                        pub mod delete_target_region {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌍️region/🪦delete-target-region/🦀️.rs"]
                            mod component;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌍️region/🪦delete-target-region/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌍️region/🪦delete-target-region/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            pub use component::*;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌍️region/🪦delete-target-region/🧪️tests/🪦️removes-region-1/🦀️.rs"]
                            mod tests_removes_region_1;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌍️region/🪦delete-target-region/🧪️tests/🪦️removes-middle-region/🦀️.rs"]
                            mod tests_removes_the_middle_region;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌍️region/🪦delete-target-region/🧪️tests/🚫️rejects/🦀️.rs"]
                            mod tests_rejects_deleting_a_region_the_board_never_held;
                        }
                        #[path = "."]
                        pub mod move_target_region {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌍️region/🚀move-target-region/🦀️.rs"]
                            mod component;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌍️region/🚀move-target-region/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌍️region/🚀move-target-region/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            pub use component::*;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌍️region/🚀move-target-region/🧪️tests/🚀️slides-region-1/🦀️.rs"]
                            mod tests_slides_region_1;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌍️region/🚀move-target-region/🧪️tests/🚫️rejects/🦀️.rs"]
                            mod tests_rejects_moving_a_region_the_board_never_held;
                        }
                        #[path = "."]
                        pub mod resize_target_region {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌍️region/📐resize-target-region/🦀️.rs"]
                            mod component;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌍️region/📐resize-target-region/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌍️region/📐resize-target-region/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            pub use component::*;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌍️region/📐resize-target-region/🧪️tests/📐️widens-region-1/🦀️.rs"]
                            mod tests_widens_region_1;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌍️region/📐resize-target-region/🧪️tests/🚫️rejects/🦀️.rs"]
                            mod tests_rejects_resizing_a_region_the_board_never_held;
                        }
                        #[path = "."]
                        pub mod edit_target_region_label {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌍️region/🖋️edit-target-region-label/🦀️.rs"]
                            mod component;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌍️region/🖋️edit-target-region-label/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌍️region/🖋️edit-target-region-label/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            pub use component::*;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌍️region/🖋️edit-target-region-label/🧪️tests/🖋️renames/🦀️.rs"]
                            mod tests_renames_region_1;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌍️region/🖋️edit-target-region-label/🧪️tests/🚫️rejects/🦀️.rs"]
                            mod tests_rejects_renaming_a_region_the_board_never_held;
                        }
                        #[path = "."]
                        pub mod change_target_region_hidden {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌍️region/🙈change-target-region-hidden/🦀️.rs"]
                            mod component;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌍️region/🙈change-target-region-hidden/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌍️region/🙈change-target-region-hidden/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            pub use component::*;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌍️region/🙈change-target-region-hidden/🧪️tests/🙈️hides/🦀️.rs"]
                            mod tests_hides_region_1;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌍️region/🙈change-target-region-hidden/🧪️tests/🚫️rejects/🦀️.rs"]
                            mod tests_rejects_hiding_a_region_the_board_never_held;
                        }
                        #[path = "."]
                        pub mod change_target_region_locked {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌍️region/🔏change-target-region-locked/🦀️.rs"]
                            mod component;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌍️region/🔏change-target-region-locked/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌍️region/🔏change-target-region-locked/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            pub use component::*;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌍️region/🔏change-target-region-locked/🧪️tests/🔏️locks/🦀️.rs"]
                            mod tests_locks_region_1;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌍️region/🔏change-target-region-locked/🧪️tests/🚫️rejects/🦀️.rs"]
                            mod tests_rejects_locking_a_region_the_board_never_held;
                        }
                        #[path = "."]
                        pub mod drag_selection {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/✋️selection/✋️drag-selection/🦀️.rs"]
                            mod component;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/✋️selection/✋️drag-selection/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/✋️selection/✋️drag-selection/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            pub use component::*;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/✋️selection/✋️drag-selection/🧪️tests/🧱️no-targets/🦀️.rs"]
                            mod tests_no_targets;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/✋️selection/✋️drag-selection/🧪️tests/✋️drags-two-nodes/🦀️.rs"]
                            mod tests_drags_two_nodes;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/✋️selection/✋️drag-selection/🧪️tests/🎯️drags-node-and-region/🦀️.rs"]
                            mod tests_drags_node_and_region;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/✋️selection/✋️drag-selection/🧪️tests/⚠️skips-locked-ghost/🦀️.rs"]
                            mod tests_skips_locked_ghost;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/✋️selection/✋️drag-selection/🧪️tests/🚫️rejects-ghosts/🦀️.rs"]
                            mod tests_rejects_ghosts;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/✋️selection/✋️drag-selection/🧪️tests/⏸️keeps-a-zero-offset/🦀️.rs"]
                            mod tests_keeps_a_zero_offset;
                        }
                        #[path = "."]
                        pub mod rotate_selection {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/✋️selection/🔄️rotate-selection/🦀️.rs"]
                            mod component;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/✋️selection/🔄️rotate-selection/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/✋️selection/🔄️rotate-selection/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            pub use component::*;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/✋️selection/🔄️rotate-selection/🧪️tests/🧱️repeated-targets/🦀️.rs"]
                            mod tests_repeated_targets;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/✋️selection/🔄️rotate-selection/🧪️tests/🔄️turns-two-nodes/🦀️.rs"]
                            mod tests_turns_two_nodes;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/✋️selection/🔄️rotate-selection/🧪️tests/🎯️skips-the-region/🦀️.rs"]
                            mod tests_skips_the_region;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/✋️selection/🔄️rotate-selection/🧪️tests/⚠️skips-locked-ghost/🦀️.rs"]
                            mod tests_skips_locked_ghost;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/✋️selection/🔄️rotate-selection/🧪️tests/🚫️rejects-ghosts/🦀️.rs"]
                            mod tests_rejects_ghosts;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/✋️selection/🔄️rotate-selection/🧪️tests/⏸️keeps-a-zero-angle/🦀️.rs"]
                            mod tests_keeps_a_zero_angle;
                        }
                        #[path = "."]
                        pub mod scale_selection {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/✋️selection/🔍️scale-selection/🦀️.rs"]
                            mod component;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/✋️selection/🔍️scale-selection/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/✋️selection/🔍️scale-selection/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            pub use component::*;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/✋️selection/🔍️scale-selection/🧪️tests/⛔️negative-factor/🦀️.rs"]
                            mod tests_negative_factor;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/✋️selection/🔍️scale-selection/🧪️tests/🧱️zero-factor/🦀️.rs"]
                            mod tests_zero_factor;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/✋️selection/🔍️scale-selection/🧪️tests/🔍️doubles-two-nodes/🦀️.rs"]
                            mod tests_doubles_two_nodes;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/✋️selection/🔍️scale-selection/🧪️tests/🎯️halves-node-region/🦀️.rs"]
                            mod tests_halves_node_region;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/✋️selection/🔍️scale-selection/🧪️tests/⚠️skips-locked-ghost/🦀️.rs"]
                            mod tests_skips_locked_ghost;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/✋️selection/🔍️scale-selection/🧪️tests/🚫️rejects-ghosts/🦀️.rs"]
                            mod tests_rejects_ghosts;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/✋️selection/🔍️scale-selection/🧪️tests/⏸️keeps-a-unit-factor/🦀️.rs"]
                            mod tests_keeps_a_unit_factor;
                        }
                    }
                }
                #[path = "."]
                pub mod io {
                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🦀️.rs"]
                    mod component;
                    pub use component::*;
                    #[path = "."]
                    pub mod import {
                        #[path = "."]
                        pub mod deserializers {
                            #[path = "."]
                            pub mod artifacts {
                                #[path = "."]
                                pub mod txt {
                                    #[path = "."]
                                    pub mod v_utf_8 {
                                        #[path = "."]
                                        pub mod any {
                                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/🔤️txt/🔖️utf-8/✳️any/🦀️.rs"]
                                            mod component;
                                            pub use component::*;
                                        }
                                    }
                                }
                                #[path = "."]
                                pub mod json {
                                    #[path = "."]
                                    pub mod v_rfc8259 {
                                        #[path = "."]
                                        pub mod any {
                                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/🔣️json/🔖️rfc8259/✳️any/🦀️.rs"]
                                            mod component;
                                            pub use component::*;
                                        }
                                    }
                                }
                            }
                        }
                    }
                    #[path = "."]
                    pub mod export {
                        #[path = "."]
                        pub mod serializers {
                            #[path = "."]
                            pub mod artifacts {
                                #[path = "."]
                                pub mod svg {
                                    #[path = "."]
                                    pub mod v1_1 {
                                        #[path = "."]
                                        pub mod any {
                                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📤️export/🧵️serializers/🗿️artifacts/🎨️svg/🔖️1.1/✳️any/🦀️.rs"]
                                            mod component;
                                            pub use component::*;
                                        }
                                    }
                                }
                                #[path = "."]
                                pub mod txt {
                                    #[path = "."]
                                    pub mod v_utf_8 {
                                        #[path = "."]
                                        pub mod any {
                                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📤️export/🧵️serializers/🗿️artifacts/🔤️txt/🔖️utf-8/✳️any/🦀️.rs"]
                                            mod component;
                                            pub use component::*;
                                        }
                                    }
                                }
                                #[path = "."]
                                pub mod pdf {
                                    #[path = "."]
                                    pub mod v1_4 {
                                        #[path = "."]
                                        pub mod any {
                                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📤️export/🧵️serializers/🗿️artifacts/📖️pdf/🔖️1.4/✳️any/🦀️.rs"]
                                            mod component;
                                            pub use component::*;
                                        }
                                    }
                                }
                                #[path = "."]
                                pub mod png {
                                    #[path = "."]
                                    pub mod v1_2 {
                                        #[path = "."]
                                        pub mod any {
                                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📤️export/🧵️serializers/🗿️artifacts/📷️png/🔖️1.2/✳️any/🦀️.rs"]
                                            mod component;
                                            pub use component::*;
                                        }
                                    }
                                }
                                #[path = "."]
                                pub mod json {
                                    #[path = "."]
                                    pub mod v_rfc8259 {
                                        #[path = "."]
                                        pub mod any {
                                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📤️export/🧵️serializers/🗿️artifacts/🔣️json/🔖️rfc8259/✳️any/🦀️.rs"]
                                            mod component;
                                            pub use component::*;
                                        }
                                    }
                                }
                                #[path = "."]
                                pub mod dwg {
                                    #[path = "."]
                                    pub mod v_ac1018 {
                                        #[path = "."]
                                        pub mod any {
                                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📤️export/🧵️serializers/🗿️artifacts/🖊️dwg/🔖️ac1018/✳️any/🦀️.rs"]
                                            mod component;
                                            pub use component::*;
                                        }
                                    }
                                }
                                #[path = "."]
                                pub mod dxf {
                                    #[path = "."]
                                    pub mod v_r12 {
                                        #[path = "."]
                                        pub mod any {
                                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📤️export/🧵️serializers/🗿️artifacts/📐️dxf/🔖️r12/✳️any/🦀️.rs"]
                                            mod component;
                                            pub use component::*;
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

pub use crate::standards::v1::subsets::any::schema::diff::Puzzle2dDiff;
pub use crate::standards::v1::subsets::any::schema::mutations::Puzzle2dMutation;


pub use crate::standards::v1::subsets::any::schema::snapshot::Puzzle2dSnapshot;

#[path = "."]
pub mod examples {
    #[path = "."]
    pub mod puzzle2d {
        #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🌲️concrete-forest/🦀️.rs"]
        pub mod concrete_forest;
        #[cfg(test)]
        #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🌲️concrete-forest/🧪️tests/🧩️example/🦀️.rs"]
        mod concrete_forest_tests;
        #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🏗️nakagin-capsule-tower/🦀️.rs"]
        pub mod nakagin_capsule_tower;
        #[cfg(test)]
        #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🏗️nakagin-capsule-tower/🧪️tests/🧩️example/🦀️.rs"]
        mod nakagin_capsule_tower_tests;
    }
}

#[path = "."]
pub mod editor {
    #[path = "."]
    pub mod puzzle2d {
        #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs"]
        mod component;
        pub use component::*;

        #[path = "."]
        pub mod examples {
            #[path = "."]
            pub mod demo_session {
                #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📚️examples/🎬️demo-session/🦀️.rs"]
                mod component;
                pub use component::*;
                #[cfg(test)]
                #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📚️examples/🎬️demo-session/🧪️tests/🧩️example/🦀️.rs"]
                mod tests;
            }
        }

        #[path = "."]
        pub mod config {
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎚️config/🦀️.rs"]
            mod component;
            pub use component::*;

            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎚️config/🧬️schema/🦀️.rs"]
            pub mod schema;
        }

        #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🪟️window/🦀️.rs"]
        pub mod window;

        #[path = "."]
        pub mod presence {
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/👥️presence/🦀️.rs"]
            mod component;
            pub use component::*;

            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/👥️presence/🧬️schema/🦀️.rs"]
            pub mod schema;
        }
        #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🗣️terminology/🦀️.rs"]
        pub mod terminology;
        #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🌉️wasm/🦀️.rs"]
        pub mod wasm;

        #[path = "."]
        pub mod engine {
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🦀️.rs"]
            mod component;
            pub use component::*;
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🎲️board-host/🦀️.rs"]
            pub mod board_host;
            #[cfg(test)]
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🖌️brush/🧪️tests/🔬️unit/🦀️.rs"]
            mod brush;
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🔣️icons/🦀️.rs"]
            pub mod icons;
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/📐️layout/🦀️.rs"]
            pub mod layout;
            #[cfg(test)]
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🔗️linking/🧪️tests/🔬️unit/🦀️.rs"]
            mod linking;
        }

        #[path = "."]
        pub mod precompute {
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⏳️precompute/🪣️fill/🦀️.rs"]
            pub mod fill;
        }

        #[path = "."]
        pub mod commands {
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🧊️document/🌱️add-node/🦀️.rs"]
            pub mod add_node;
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🧊️document/🎲️apply-board-events/🦀️.rs"]
            pub mod apply_board_events;
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🎛️runtime/🔒️close-handle-suggestions/🦀️.rs"]
            pub mod close_handle_suggestions;
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🧊️document/✅️accept-suggestion/🦀️.rs"]
            pub mod accept_suggestion;
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🎛️runtime/🎣️target-brush-suggestions/🦀️.rs"]
            pub mod target_brush_suggestions;
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🎛️runtime/🔁️cycle-candidate/🦀️.rs"]
            pub mod cycle_candidate;
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🧊️document/💞️create-edge/🦀️.rs"]
            pub mod create_edge;
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🧊️document/💔️delete-edge/🦀️.rs"]
            pub mod delete_edge;
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🧊️document/🔗️proximity-connect/🦀️.rs"]
            pub mod proximity_connect;
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🎛️runtime/📡️set-proximity-radius/🦀️.rs"]
            pub mod set_proximity_radius;
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🧊️document/🗑️delete-selection/🦀️.rs"]
            pub mod delete_selection;
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🧊️document/👯️duplicate-selection/🦀️.rs"]
            pub mod duplicate_selection;
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🎛️runtime/🛑️engagement-abort/🦀️.rs"]
            pub mod engagement_abort;
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🎛️runtime/🎛️engagement-control-select/🦀️.rs"]
            pub mod engagement_control_select;
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🎛️runtime/⌨️engagement-input/🦀️.rs"]
            pub mod engagement_input;
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🎛️runtime/📨️engagement-submit/🦀️.rs"]
            pub mod engagement_submit;
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🧊️document/🎯️focus-selection/🦀️.rs"]
            pub mod focus_selection;
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🧊️document/⚛️force-layout/🦀️.rs"]
            pub mod force_layout;
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🎛️runtime/📊️lod-scale-json/🦀️.rs"]
            pub mod lod_scale_json;
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🎛️runtime/🔓️open-handle-suggestions/🦀️.rs"]
            pub mod open_handle_suggestions;
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🎛️runtime/🖱️hover-suggestion/🦀️.rs"]
            pub mod hover_suggestion;
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🧊️document/🩹️patch-inspector/🦀️.rs"]
            pub mod patch_inspector;
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🧊️document/🧬️select-same-kind/🦀️.rs"]
            pub mod select_same_kind;
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🧊️document/🛍️set-active-example/🦀️.rs"]
            pub mod set_active_example;
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🎛️runtime/⚖️set-brush-kind-weights/🦀️.rs"]
            pub mod set_brush_kind_weights;
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🎛️runtime/📏️set-brush-node-size/🦀️.rs"]
            pub mod set_brush_node_size;
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🎛️runtime/📷️set-camera/🦀️.rs"]
            pub mod set_camera;
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🎛️runtime/🧮️set-fill-count/🦀️.rs"]
            pub mod set_fill_count;
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🎛️runtime/📐️set-grid-factor/🦀️.rs"]
            pub mod set_grid_factor;
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🎛️runtime/🧲️set-grid-snap-enabled/🦀️.rs"]
            pub mod set_grid_snap_enabled;
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🎛️runtime/👁️set-grid-visible/🦀️.rs"]
            pub mod set_grid_visible;
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🎛️runtime/☑️set-selectable-kind/🦀️.rs"]
            pub mod set_selectable_kind;
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🎛️runtime/🚧️set-brush-placement-contact-tolerance/🦀️.rs"]
            pub mod set_brush_placement_contact_tolerance;
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🎛️runtime/🫂️set-brush-placement-overlap-budget/🦀️.rs"]
            pub mod set_brush_placement_overlap_budget;
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🎛️runtime/🔂️engagement-repeat-last/🦀️.rs"]
            pub mod engagement_repeat_last;
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🎛️runtime/🔭️set-lod-mode-for-pane/🦀️.rs"]
            pub mod set_lod_mode_for_pane;
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🧊️document/🚀️translate-selection/🦀️.rs"]
            pub mod translate_selection;
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🧊️document/🔄️rotate-selection/🦀️.rs"]
            pub mod rotate_selection;
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🧊️document/📏️scale-selection/🦀️.rs"]
            pub mod scale_selection;
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🧊️document/📤️export-snapshot/🦀️.rs"]
            pub mod export_snapshot;
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🧊️document/📥️import-snapshot/🦀️.rs"]
            pub mod import_snapshot;
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🧊️document/🗂️open-import-snapshot/🦀️.rs"]
            pub mod open_import_snapshot;
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🧊️document/🔖️set-selection-flag/🦀️.rs"]
            pub mod set_selection_flag;
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🎛️runtime/🧭️set-suggestion-offset/🦀️.rs"]
            pub mod set_suggestion_offset;
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🎛️runtime/🕹️set-transform-gumball-flag/🦀️.rs"]
            pub mod set_transform_gumball_flag;
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🧊️document/➕️add-target-region/🦀️.rs"]
            pub mod add_target_region;
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🧊️document/🪦️delete-target-region/🦀️.rs"]
            pub mod delete_target_region;
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🧊️document/🚚️relocate-target-region/🦀️.rs"]
            pub mod relocate_target_region;
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🧊️document/🚩️set-target-region-flag/🦀️.rs"]
            pub mod set_target_region_flag;
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🎛️runtime/🖍️set-area-brush-size/🦀️.rs"]
            pub mod set_area_brush_size;
        }

        #[path = "."]
        pub mod panels {
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📌️panels/🛍️catalogue/🦀️.rs"]
            pub mod catalogue;
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📌️panels/🗿️artifact/🦀️.rs"]
            pub mod artifact;
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📌️panels/🔍️inspection/🦀️.rs"]
            pub mod inspection;
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📌️panels/⚙️settings/🦀️.rs"]
            pub mod settings;
        }

        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod edit {
                #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🦀️.rs"]
                mod component;
                pub use component::*;

                #[path = "."]
                pub mod options {
                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/☑️options/🖌️brush/🦀️.rs"]
                    pub mod brush;
                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/☑️options/🌐️grid/🦀️.rs"]
                    pub mod grid;
                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/☑️options/🔄️transform/🦀️.rs"]
                    pub mod transform;
                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/☑️options/🔭️lod/🦀️.rs"]
                    pub mod lod;
                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/☑️options/🎯️select/🦀️.rs"]
                    pub mod select;
                }

                #[path = "."]
                pub mod tools {
                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🛠️tools/🪣️fill/🦀️.rs"]
                    pub mod fill;
                }

                #[path = "."]
                pub mod windows {
                    #[path = "."]
                    pub mod overview {
                        #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/👁️overview/🦀️.rs"]
                        mod component;
                        pub use component::*;

                        #[path = "."]
                        pub mod utilities {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/👁️overview/🪛️utilities/🖌️brush/🦀️.rs"]
                            pub mod brush;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/👁️overview/🪛️utilities/🖍️area-brush/🦀️.rs"]
                            pub mod area_brush;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/👁️overview/🪛️utilities/🖱️select/🦀️.rs"]
                            pub mod select;
                        }
                    }

                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/🔍️detail/🦀️.rs"]
                    pub mod detail;
                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/🎯️selection/🦀️.rs"]
                    pub mod selection;
                }
            }
        }
    }
}

#[path = "."]
pub mod viewer {
    #[path = "."]
    pub mod puzzle2d {
        #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/👁️viewer/🦀️.rs"]
        mod component;
        pub use component::*;
        #[path = "."]
        pub mod presence {
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/👁️viewer/👥️presence/🧬️schema/🦀️.rs"]
            pub mod schema;
        }
        #[path = "."]
        pub mod config {
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/👁️viewer/🎚️config/🧬️schema/🦀️.rs"]
            pub mod schema;
        }

        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod view {
                #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/👁️viewer/🎭️modes/👁️view/🦀️.rs"]
                mod component;
                pub use component::*;

                #[path = "."]
                pub mod windows {
                    #[path = "."]
                    pub mod board {
                        #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/👁️viewer/🎭️modes/👁️view/🪟️windows/🎲️board/🦀️.rs"]
                        mod component;
                        pub use component::*;
                    }
                }
            }
        }
    }
}

pub use crate::standards::v1::subsets::any::io::{Puzzle2dBuilderConstruction, Puzzle2dParts, Puzzle2dAnalyzerAnalysis, Puzzle2dBuilderFacets, Puzzle2dBuilder, Puzzle2dAnalyzer, Puzzle2dComposer};

#[path = "."]
pub mod host {
#[path = "🔨️modules/🏠️host/🧰️owned/🦀️.rs"]
pub mod owned;
}
