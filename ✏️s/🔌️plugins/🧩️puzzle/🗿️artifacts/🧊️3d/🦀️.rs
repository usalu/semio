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

pub use crate::editor::puzzle3d::precompute::Puzzle3dPrecomputeSession;

#[path = "../../🎮️commands/📤️jobs/🦀️.rs"]
pub mod puzzle_job;

#[path = "../../🎮️commands/🧵️retained/🦀️.rs"]
pub mod retained_command;

#[cfg(test)]
fn retained_command_test_catalog() -> (&'static str, &'static str, &'static [&'static str], &'static str) {
    ("puzzle3d", "puzzle.3d", editor::puzzle3d::PUZZLE3D_RETAINED_TOOL_IDS, include_str!("🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🧫️fixtures/🗄️retained-jobs/🔣️.json"))
}

//#region ⚠️ Errors







//#endregion ⚠️ Errors





//#region 📐️Scale


















//#endregion 📐️Scale











































//#region 🔖️Snapshot
//#endregion 🔖️Snapshot

//#region 🔖️ArtifactKind
/// 🗿️ The `3d.puzzle` artifact kind — lifted out of the pre-consolidation manifest builder chain so
/// the artifact, not the app, owns its own identity.
pub fn artifact_kind() -> semio_framework_plugin::ArtifactKindSpec {
    semio_framework_plugin::ArtifactKindSpec {
        id: "3d.puzzle".into(),
        label: semio_framework_ui_locale::LocalizedLabel::native("3D Puzzle", "3D-Puzzle"),
        source_format: "puzzle.3d".into(),
        component_kind: "puzzle3d".into(),
        dimension: "3d".into(),
        media_capability: semio_framework_plugin::OsMediaCapability::MeshOnly,
        media_type: semio_framework_plugin::MediaType { class: semio_framework_plugin::MediaClass::ThreeD, form: semio_framework_plugin::MediaForm::Design },
        schema: PUZZLE_3D_SCHEMA.into(),
        export_formats: vec![],
        import_formats: vec![],
        export_stdio_kinds: vec!["stdio.json".into()],
        import_stdio_kinds: vec!["stdio.json".into()],
    }
}

/// 🔌️ The `kit.catalog` artifact kind puzzle3d's `kit:in` port CONSUMES — kept as a spec puzzle can
/// read, but no longer registered as a declaration.
///
/// 🐛️ MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME (registrar, 2026-08-18): the old doc here called the
/// duplicate declaration "harmless if a producer, e.g. block3d, declares an identical spec". It was
/// not harmless — it was the last thing blocking `🎪️demonstrator`, which bundles six plugins and
/// therefore registered `kit.catalog` twice, and the definition registry correctly rejected it.
/// An artifact kind is a SCHEMA: N declarations are N sources of truth for one contract, and the
/// first divergence between them would be undetectable. Ownership resolved to `🧱️block`, the only
/// plugin that actually PRODUCES the kind (`catalog:out`, `KIT_CATALOG_ARTIFACT_ID` across its 3d
/// and 5d artifact trees); `🧩️puzzle` and `🪵️sourcing` are consumers and reference it by `kind_id`
/// on their ports, which is all a consumer ever needed. `🗄️stdio`'s own docstrings describe
/// absorbing this kind into the shared vocabulary later — that move stays open and is not
/// pre-empted here.
pub fn kit_catalog_artifact_kind() -> semio_framework_plugin::ArtifactKindSpec {
    semio_framework_plugin::ArtifactKindSpec {
        id: "kit.catalog".into(),
        label: semio_framework_ui_locale::LocalizedLabel::native("Kit Catalog", "Bausatzkatalog"),
        source_format: "kit.catalog".into(),
        component_kind: "kit".into(),
        dimension: "3d".into(),
        media_capability: semio_framework_plugin::OsMediaCapability::MeshOnly,
        media_type: semio_framework_plugin::MediaType { class: semio_framework_plugin::MediaClass::Kit, form: semio_framework_plugin::MediaForm::Type },
        schema: "kit.catalog".into(),
        export_formats: vec![],
        import_formats: vec![],
        export_stdio_kinds: vec!["stdio.dwg".into(), "stdio.gltf".into(), "stdio.json".into(), "stdio.las".into(), "stdio.obj".into(), "stdio.ply".into(), "stdio.png".into(), "stdio.stl".into()],
        import_stdio_kinds: vec!["stdio.dwg".into(), "stdio.gltf".into(), "stdio.json".into(), "stdio.las".into(), "stdio.obj".into(), "stdio.ply".into(), "stdio.png".into(), "stdio.stl".into()],
    }
}
//#endregion 🔖️ArtifactKind

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
    + From<semio_framework_plugin::app::VcsArtifactApp<semio_framework_plugin::app::EditorApp<editor::puzzle3d::Puzzle3dPlayApp>>>
    + From<semio_framework_plugin::app::VcsArtifactApp<semio_framework_plugin::app::ViewerApp<viewer::puzzle3d::Puzzle3dViewer>>>
{
}

impl<PA> ArtifactApps for PA where
    PA: semio_framework_plugin::PluginApp
        + From<semio_framework_plugin::app::VcsArtifactApp<semio_framework_plugin::app::EditorApp<editor::puzzle3d::Puzzle3dPlayApp>>>
        + From<semio_framework_plugin::app::VcsArtifactApp<semio_framework_plugin::app::ViewerApp<viewer::puzzle3d::Puzzle3dViewer>>>
{
}

pub fn definition() -> Result<semio_framework_plugin::ArtifactDefinition, semio_framework_plugin::ArtifactDefinitionError> {
use crate::standards::v1::subsets::any::schema::PUZZLE_3D_SCHEMA;
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
                    grammar: Some(standards::v1::subsets::any::io::text::snapshot::COMPONENT_GRAMMAR_SEMIO),
                    grammar_path: Some(standards::v1::subsets::any::io::text::snapshot::COMPONENT_GRAMMAR_PATH),
                    protocol: Some(standards::v1::subsets::any::io::binary::snapshot::COMPONENT_PROTOCOL_SEMIO),
                    protocol_path: Some(standards::v1::subsets::any::io::binary::snapshot::COMPONENT_PROTOCOL_PATH),
                    hooks: semio_framework_dsl::passthrough_hooks("puzzle.puzzle3d"),
                },
                semio_framework_dsl::LanguageSpec {
                    id: "puzzle.puzzle3d.op",
                    extension: None,
                    role: semio_framework_dsl::LanguageRole::Ops,
                    grammar: Some(standards::v1::subsets::any::io::text::mutations::COMPONENT_GRAMMAR_SEMIO),
                    grammar_path: Some(standards::v1::subsets::any::io::text::mutations::COMPONENT_GRAMMAR_PATH),
                    protocol: Some(standards::v1::subsets::any::io::binary::mutations::COMPONENT_PROTOCOL_SEMIO),
                    protocol_path: Some(standards::v1::subsets::any::io::binary::mutations::COMPONENT_PROTOCOL_PATH),
                    hooks: semio_framework_dsl::passthrough_hooks("puzzle.puzzle3d.op"),
                },
                semio_framework_dsl::LanguageSpec {
                    id: "puzzle.puzzle3d.diff",
                    extension: None,
                    role: semio_framework_dsl::LanguageRole::Diff,
                    grammar: Some(standards::v1::subsets::any::io::text::diff::COMPONENT_GRAMMAR_SEMIO),
                    grammar_path: Some(standards::v1::subsets::any::io::text::diff::COMPONENT_GRAMMAR_PATH),
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
                    protocol: Some(standards::v1::subsets::any::io::binary::snapshot::COMPONENT_PROTOCOL_SEMIO),
                    protocol_path: Some(standards::v1::subsets::any::io::binary::snapshot::COMPONENT_PROTOCOL_PATH),
                    hooks: semio_framework_dsl::passthrough_hooks("3d.pack"),
                },
                semio_framework_dsl::LanguageSpec {
                    id: "3d.spr",
                    extension: None,
                    role: semio_framework_dsl::LanguageRole::Spr,
                    grammar: None,
                    grammar_path: None,
                    protocol: Some(standards::v1::subsets::any::io::binary::mutations::COMPONENT_PROTOCOL_SEMIO),
                    protocol_path: Some(standards::v1::subsets::any::io::binary::mutations::COMPONENT_PROTOCOL_PATH),
                    hooks: semio_framework_dsl::passthrough_hooks("3d.spr"),
                },
            ]
        })
        .as_slice()
}
//#endregion 🔖️Declaration

pub use crate::editor::puzzle3d::snapshot::Puzzle3dPlaySnapshot;

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️design-parity-schema/🦀️.rs"]
mod design_parity_schema_tests;
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
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/📍️flat-position/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                }
                                #[path = "."]
                                pub mod flatten {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/🗜️flatten/🦀️.rs"]
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
                                pub mod create_object {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧊️object/🌱create-object/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧊️object/🌱create-object/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧊️object/🌱create-object/🦀️.rs"]
                                    pub mod mutation;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧊️object/🌱create-object/🧪️tests/🌱️appends-object-c/🦀️.rs"]
                                    mod tests_appends_object_c;
                                }
                                #[path = "."]
                                pub mod delete_object {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧊️object/🗑️delete-object/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧊️object/🗑️delete-object/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧊️object/🗑️delete-object/🦀️.rs"]
                                    pub mod mutation;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧊️object/🗑️delete-object/🧪️tests/🚫️removes/🦀️.rs"]
                                    mod tests_removes_object_a_and_severs_attraction;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧊️object/🗑️delete-object/🧪️tests/🗑️removes-middle-object/🦀️.rs"]
                                    mod tests_removes_the_middle_object_and_its_attractions;
                                }
                                #[path = "."]
                                pub mod move_object {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧭️pose/📍move-object/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧭️pose/📍move-object/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧭️pose/📍move-object/🦀️.rs"]
                                    pub mod mutation;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧭️pose/📍move-object/🧪️tests/📍️moves-object-a/🦀️.rs"]
                                    mod tests_moves_object_a;
                                }
                                #[path = "."]
                                pub mod rotate_object {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧭️pose/🔃rotate-object/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧭️pose/🔃rotate-object/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧭️pose/🔃rotate-object/🦀️.rs"]
                                    pub mod mutation;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧭️pose/🔃rotate-object/🧪️tests/🔄️half-turn-about-z/🦀️.rs"]
                                    mod tests_half_turn_about_z;
                                }
                                #[path = "."]
                                pub mod scale_object {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧭️pose/📏scale-object/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧭️pose/📏scale-object/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧭️pose/📏scale-object/🦀️.rs"]
                                    pub mod mutation;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧭️pose/📏scale-object/🧪️tests/📐️uniform-to-per-axis/🦀️.rs"]
                                    mod tests_uniform_to_per_axis;
                                }
                                #[path = "."]
                                pub mod change_object_mesh {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧊️object/🧱change-object-mesh/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧊️object/🧱change-object-mesh/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧊️object/🧱change-object-mesh/🦀️.rs"]
                                    pub mod mutation;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧊️object/🧱change-object-mesh/🧪️tests/🕸️repoints/🦀️.rs"]
                                    mod tests_repoints_object_a_mesh;
                                }
                                #[path = "."]
                                pub mod edit_object_label {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧊️object/🖋️edit-object-label/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧊️object/🖋️edit-object-label/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧊️object/🖋️edit-object-label/🦀️.rs"]
                                    pub mod mutation;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧊️object/🖋️edit-object-label/🧪️tests/🔤️relabels-object-a/🦀️.rs"]
                                    mod tests_relabels_object_a;
                                }
                                #[path = "."]
                                pub mod change_object_kind {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧊️object/🏗️change-object-kind/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧊️object/🏗️change-object-kind/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧊️object/🏗️change-object-kind/🦀️.rs"]
                                    pub mod mutation;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧊️object/🏗️change-object-kind/🧪️tests/🏷️reassigns/🦀️.rs"]
                                    mod tests_reassigns_object_a_kind;
                                }
                                #[path = "."]
                                pub mod change_object_anchor {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧊️object/⚓change-object-anchor/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧊️object/⚓change-object-anchor/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧊️object/⚓change-object-anchor/🦀️.rs"]
                                    pub mod mutation;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧊️object/⚓change-object-anchor/🧪️tests/⚓️fixed-to-derived/🦀️.rs"]
                                    mod tests_fixed_to_derived;
                                }
                                #[path = "."]
                                pub mod change_object_hidden {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧊️object/👁️change-object-hidden/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧊️object/👁️change-object-hidden/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧊️object/👁️change-object-hidden/🦀️.rs"]
                                    pub mod mutation;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧊️object/👁️change-object-hidden/🧪️tests/🙈️hides-object-a/🦀️.rs"]
                                    mod tests_hides_object_a;
                                }
                                #[path = "."]
                                pub mod change_object_locked {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧊️object/🔒change-object-locked/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧊️object/🔒change-object-locked/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧊️object/🔒change-object-locked/🦀️.rs"]
                                    pub mod mutation;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧊️object/🔒change-object-locked/🧪️tests/🔒️locks-object-a/🦀️.rs"]
                                    mod tests_locks_object_a;
                                }
                                #[path = "."]
                                pub mod add_object_vortex {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌀️vortex/➕add-object-vortex/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌀️vortex/➕add-object-vortex/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌀️vortex/➕add-object-vortex/🦀️.rs"]
                                    pub mod mutation;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌀️vortex/➕add-object-vortex/🧪️tests/🌀️appends/🦀️.rs"]
                                    mod tests_appends_vortex_3_to_object_b;
                                }
                                #[path = "."]
                                pub mod remove_object_vortex {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌀️vortex/➖remove-object-vortex/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌀️vortex/➖remove-object-vortex/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌀️vortex/➖remove-object-vortex/🦀️.rs"]
                                    pub mod mutation;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌀️vortex/➖remove-object-vortex/🧪️tests/🚫️removes/🦀️.rs"]
                                    mod tests_removes_vortex_2_and_severs_attraction;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌀️vortex/➖remove-object-vortex/🧪️tests/➖removes-middle-vortex/🦀️.rs"]
                                    mod tests_removes_the_middle_vortex_and_its_attraction;
                                }
                                #[path = "."]
                                pub mod replace_object_vortex {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌀️vortex/🔌replace-object-vortex/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌀️vortex/🔌replace-object-vortex/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌀️vortex/🔌replace-object-vortex/🦀️.rs"]
                                    pub mod mutation;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌀️vortex/🔌replace-object-vortex/🧪️tests/⏸️rekind/🦀️.rs"]
                                    mod tests_rekind_vortex_1_is_noop;
                                }
                                #[path = "."]
                                pub mod connect_vortices {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌀️vortex/🪢️connect-vortices/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌀️vortex/🪢️connect-vortices/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌀️vortex/🪢️connect-vortices/🦀️.rs"]
                                    pub mod mutation;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌀️vortex/🪢️connect-vortices/🧪️tests/🧲️adds/🦀️.rs"]
                                    mod tests_adds_second_attraction;
                                }
                                #[path = "."]
                                pub mod disconnect_vortices {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌀️vortex/✂️disconnect-vortices/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌀️vortex/✂️disconnect-vortices/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌀️vortex/✂️disconnect-vortices/🦀️.rs"]
                                    pub mod mutation;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌀️vortex/✂️disconnect-vortices/🧪️tests/🚫️removes/🦀️.rs"]
                                    mod tests_removes_attraction_1;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌀️vortex/✂️disconnect-vortices/🧪️tests/✂️severs-middle-attraction/🦀️.rs"]
                                    mod tests_severs_the_middle_attraction;
                                }
                                #[path = "."]
                                pub mod replace_attraction_geometry {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌀️vortex/🧮replace-attraction-geometry/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌀️vortex/🧮replace-attraction-geometry/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌀️vortex/🧮replace-attraction-geometry/🦀️.rs"]
                                    pub mod mutation;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌀️vortex/🧮replace-attraction-geometry/🧪️tests/📍️repositions/🦀️.rs"]
                                    mod tests_repositions_attraction_1;
                                }
                                #[path = "."]
                                pub mod create_target_volume {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌍️volume/🌍create-target-volume/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌍️volume/🌍create-target-volume/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌍️volume/🌍create-target-volume/🦀️.rs"]
                                    pub mod mutation;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌍️volume/🌍create-target-volume/🧪️tests/🧊️appends-volume-2/🦀️.rs"]
                                    mod tests_appends_volume_2;
                                }
                                #[path = "."]
                                pub mod delete_target_volume {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌍️volume/🪦delete-target-volume/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌍️volume/🪦delete-target-volume/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌍️volume/🪦delete-target-volume/🦀️.rs"]
                                    pub mod mutation;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌍️volume/🪦delete-target-volume/🧪️tests/🚫️removes-volume-1/🦀️.rs"]
                                    mod tests_removes_volume_1;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌍️volume/🪦delete-target-volume/🧪️tests/🪦️removes-middle-volume/🦀️.rs"]
                                    mod tests_removes_the_middle_volume;
                                }
                                #[path = "."]
                                pub mod move_target_volume {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌍️volume/🚀move-target-volume/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌍️volume/🚀move-target-volume/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌍️volume/🚀move-target-volume/🦀️.rs"]
                                    pub mod mutation;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌍️volume/🚀move-target-volume/🧪️tests/⬆️lifts-volume-1/🦀️.rs"]
                                    mod tests_lifts_volume_1;
                                }
                                #[path = "."]
                                pub mod rotate_target_volume {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌍️volume/🌀rotate-target-volume/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌍️volume/🌀rotate-target-volume/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌍️volume/🌀rotate-target-volume/🦀️.rs"]
                                    pub mod mutation;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌍️volume/🌀rotate-target-volume/🧪️tests/🔄️half-turn-about-z/🦀️.rs"]
                                    mod tests_half_turn_about_z;
                                }
                                #[path = "."]
                                pub mod scale_target_volume {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌍️volume/📐scale-target-volume/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌍️volume/📐scale-target-volume/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌍️volume/📐scale-target-volume/🦀️.rs"]
                                    pub mod mutation;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌍️volume/📐scale-target-volume/🧪️tests/📏️per-axis-to-uniform/🦀️.rs"]
                                    mod tests_per_axis_to_uniform;
                                }
                                #[path = "."]
                                pub mod change_target_volume_hidden {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌍️volume/🙈change-target-volume-hidden/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌍️volume/🙈change-target-volume-hidden/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌍️volume/🙈change-target-volume-hidden/🦀️.rs"]
                                    pub mod mutation;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌍️volume/🙈change-target-volume-hidden/🧪️tests/🙈️hides/🦀️.rs"]
                                    mod tests_hides_volume_1;
                                }
                                #[path = "."]
                                pub mod change_target_volume_locked {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌍️volume/🔐change-target-volume-locked/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌍️volume/🔐change-target-volume-locked/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌍️volume/🔐change-target-volume-locked/🦀️.rs"]
                                    pub mod mutation;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌍️volume/🔐change-target-volume-locked/🧪️tests/🔒️locks/🦀️.rs"]
                                    mod tests_locks_volume_1;
                                }
                                #[path = "."]
                                pub mod create_reference {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🖼️reference/🖼️create-reference/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🖼️reference/🖼️create-reference/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🖼️reference/🖼️create-reference/🦀️.rs"]
                                    pub mod mutation;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🖼️reference/🖼️create-reference/🧪️tests/🖼️appends-reference-2/🦀️.rs"]
                                    mod tests_appends_reference_2;
                                }
                                #[path = "."]
                                pub mod delete_reference {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🖼️reference/🚮delete-reference/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🖼️reference/🚮delete-reference/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🖼️reference/🚮delete-reference/🦀️.rs"]
                                    pub mod mutation;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🖼️reference/🚮delete-reference/🧪️tests/🚫️removes-reference-1/🦀️.rs"]
                                    mod tests_removes_reference_1;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🖼️reference/🚮delete-reference/🧪️tests/🚮️removes-middle-reference/🦀️.rs"]
                                    mod tests_removes_the_middle_reference;
                                }
                                #[path = "."]
                                pub mod move_reference {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🖼️reference/🎯move-reference/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🖼️reference/🎯move-reference/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🖼️reference/🎯move-reference/🦀️.rs"]
                                    pub mod mutation;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🖼️reference/🎯move-reference/🧪️tests/↔️slides-reference-1/🦀️.rs"]
                                    mod tests_slides_reference_1;
                                }
                                #[path = "."]
                                pub mod resize_reference {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🖼️reference/📎resize-reference/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🖼️reference/📎resize-reference/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🖼️reference/📎resize-reference/🦀️.rs"]
                                    pub mod mutation;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🖼️reference/📎resize-reference/🧪️tests/↔️widens-reference-1/🦀️.rs"]
                                    mod tests_widens_reference_1;
                                }
                                #[path = "."]
                                pub mod replace_reference_source {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🖼️reference/🖇️replace-reference-source/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🖼️reference/🖇️replace-reference-source/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🖼️reference/🖇️replace-reference-source/🦀️.rs"]
                                    pub mod mutation;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🖼️reference/🖇️replace-reference-source/🧪️tests/🖇️repoints/🦀️.rs"]
                                    mod tests_repoints_reference_1_source;
                                }
                                #[path = "."]
                                pub mod change_reference_hidden {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🖼️reference/👀change-reference-hidden/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🖼️reference/👀change-reference-hidden/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🖼️reference/👀change-reference-hidden/🦀️.rs"]
                                    pub mod mutation;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🖼️reference/👀change-reference-hidden/🧪️tests/🙈️hides/🦀️.rs"]
                                    mod tests_hides_reference_1;
                                }
                                #[path = "."]
                                pub mod change_reference_locked {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🖼️reference/🗝️change-reference-locked/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🖼️reference/🗝️change-reference-locked/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🖼️reference/🗝️change-reference-locked/🦀️.rs"]
                                    pub mod mutation;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🖼️reference/🗝️change-reference-locked/🧪️tests/🔒️locks/🦀️.rs"]
                                    mod tests_locks_reference_1;
                                }
                                #[path = "."]
                                pub mod change_domain {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📚️catalog/🌐change-domain/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📚️catalog/🌐change-domain/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📚️catalog/🌐change-domain/🦀️.rs"]
                                    pub mod mutation;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📚️catalog/🌐change-domain/🧪️tests/⚙️architecture/🦀️.rs"]
                                    mod tests_architecture_to_engineering;
                                }
                                #[path = "."]
                                pub mod connect_kind_compatibility {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📚️catalog/🤝connect-kind-compatibility/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📚️catalog/🤝connect-kind-compatibility/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📚️catalog/🤝connect-kind-compatibility/🦀️.rs"]
                                    pub mod mutation;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📚️catalog/🤝connect-kind-compatibility/🧪️tests/🤝️adds/🦀️.rs"]
                                    mod tests_adds_vortex_kind_pair;
                                }
                                #[path = "."]
                                pub mod disconnect_kind_compatibility {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📚️catalog/💔disconnect-kind-compatibility/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📚️catalog/💔disconnect-kind-compatibility/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📚️catalog/💔disconnect-kind-compatibility/🦀️.rs"]
                                    pub mod mutation;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📚️catalog/💔disconnect-kind-compatibility/🧪️tests/🚫️removes/🦀️.rs"]
                                    mod tests_removes_vortex_kind_pair;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📚️catalog/💔disconnect-kind-compatibility/🧪️tests/💔️withdraws-middle-row/🦀️.rs"]
                                    mod tests_withdraws_the_middle_row;
                                }
                                #[path = "."]
                                pub mod replace_kind_catalogs {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📚️catalog/📚replace-kind-catalogs/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📚️catalog/📚replace-kind-catalogs/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📚️catalog/📚replace-kind-catalogs/🦀️.rs"]
                                    pub mod mutation;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📚️catalog/📚replace-kind-catalogs/🧪️tests/📇️installs/🦀️.rs"]
                                    mod tests_installs_vortex_kind_catalog;
                                }
                                #[path = "."]
                                pub mod drag_selection {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/✋️selection/✋️drag-selection/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/✋️selection/✋️drag-selection/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/✋️selection/✋️drag-selection/🦀️.rs"]
                                    pub mod mutation;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/✋️selection/✋️drag-selection/🧪️tests/✋️drags-two-objects/🦀️.rs"]
                                    mod tests_drags_two_objects;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/✋️selection/✋️drag-selection/🧪️tests/🎯️drags/🦀️.rs"]
                                    mod tests_drags_object_and_volume;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/✋️selection/✋️drag-selection/🧪️tests/⚠️skips-locked-ghost/🦀️.rs"]
                                    mod tests_skips_locked_ghost;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/✋️selection/✋️drag-selection/🧪️tests/🚫️rejects-ghosts/🦀️.rs"]
                                    mod tests_rejects_ghosts;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/✋️selection/✋️drag-selection/🧪️tests/⏸️keeps-a-zero-offset/🦀️.rs"]
                                    mod tests_keeps_a_zero_offset;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/✋️selection/✋️drag-selection/🧪️tests/🧱️no-targets/🦀️.rs"]
                                    mod tests_no_targets;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/✋️selection/✋️drag-selection/🧪️tests/⛓️follow/🦀️.rs"]
                                    mod tests_follow;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/✋️selection/✋️drag-selection/🧪️tests/🪢️cross/🦀️.rs"]
                                    mod tests_cross;
                                }
                                #[path = "."]
                                pub mod rotate_selection {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/✋️selection/🔄️rotate-selection/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/✋️selection/🔄️rotate-selection/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/✋️selection/🔄️rotate-selection/🦀️.rs"]
                                    pub mod mutation;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/✋️selection/🔄️rotate-selection/🧪️tests/🔄️turns-two-objects/🦀️.rs"]
                                    mod tests_turns_two_objects;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/✋️selection/🔄️rotate-selection/🧪️tests/🎯️turns/🦀️.rs"]
                                    mod tests_turns_object_and_volume;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/✋️selection/🔄️rotate-selection/🧪️tests/⚠️skips-locked-ghost/🦀️.rs"]
                                    mod tests_skips_locked_ghost;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/✋️selection/🔄️rotate-selection/🧪️tests/🚫️rejects-ghosts/🦀️.rs"]
                                    mod tests_rejects_ghosts;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/✋️selection/🔄️rotate-selection/🧪️tests/⏸️keeps-a-zero-angle/🦀️.rs"]
                                    mod tests_keeps_a_zero_angle;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/✋️selection/🔄️rotate-selection/🧪️tests/🧱️repeated-targets/🦀️.rs"]
                                    mod tests_repeated_targets;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/✋️selection/🔄️rotate-selection/🧪️tests/⛓️orbits/🦀️.rs"]
                                    mod tests_orbits;
                                }
                                #[path = "."]
                                pub mod scale_selection {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/✋️selection/🔍️scale-selection/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/✋️selection/🔍️scale-selection/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/✋️selection/🔍️scale-selection/🦀️.rs"]
                                    pub mod mutation;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/✋️selection/🔍️scale-selection/🧪️tests/🔍️scales-two-objects/🦀️.rs"]
                                    mod tests_scales_two_objects;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/✋️selection/🔍️scale-selection/🧪️tests/🎯️scales/🦀️.rs"]
                                    mod tests_scales_object_and_volume;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/✋️selection/🔍️scale-selection/🧪️tests/⚠️skips-locked-ghost/🦀️.rs"]
                                    mod tests_skips_locked_ghost;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/✋️selection/🔍️scale-selection/🧪️tests/🚫️rejects-ghosts/🦀️.rs"]
                                    mod tests_rejects_ghosts;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/✋️selection/🔍️scale-selection/🧪️tests/⏸️keeps-unit-factors/🦀️.rs"]
                                    mod tests_keeps_unit_factors;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/✋️selection/🔍️scale-selection/🧪️tests/🧱️zero-factor/🦀️.rs"]
                                    mod tests_zero_factor;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/✋️selection/🔍️scale-selection/🧪️tests/⛔️negative-factor/🦀️.rs"]
                                    mod tests_negative_factor;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/✋️selection/🔍️scale-selection/🧪️tests/⛓️stays/🦀️.rs"]
                                    mod tests_stays;
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
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }

                pub use crate::standards::v1::subsets::any::schema::diff::Puzzle3dDiff;
pub use crate::standards::v1::subsets::any::schema::mutations::Puzzle3dMutation;


pub use crate::standards::v1::subsets::any::schema::snapshot::Puzzle3dSnapshot;
/// 🧩️ Cross-artifact solver inputs and outcomes used by Puzzle 2D and Puzzle 5D.
pub use crate::standards::v1::subsets::any::schema::{BrushPlacePayload, Puzzle3dEngineCommand, Puzzle3dEngineOutcome};
/// 📐️ Cross-artifact flattening primitives used by Puzzle 2D and Puzzle 5D.
pub use crate::standards::v1::subsets::any::schema::inferences::flatten::{flatten_objects, FlattenPlane, FlattenPose, DIAGRAM_HORIZONTAL_SCALE, DIAGRAM_RADIUS, DIAGRAM_VERTICAL_V_EXTRA};

#[path = "."]
pub mod examples {
    #[path = "."]
    pub mod puzzle3d {
        #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🌲️concrete-forest/🦀️.rs"]
        pub mod concrete_forest;
        #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🏗️nakagin-capsule-tower/🦀️.rs"]
        pub mod nakagin_capsule_tower;
        #[cfg(test)]
        #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🌲️concrete-forest/🧪️tests/🧩️example/🦀️.rs"]
        mod concrete_forest_tests;
        #[cfg(test)]
        #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🏗️nakagin-capsule-tower/🧪️tests/🧩️example/🦀️.rs"]
        mod nakagin_capsule_tower_tests;
    }
}

#[path = "."]
pub mod editor {
    #[path = "."]
    pub mod puzzle3d {
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
        pub mod precompute {
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⏳️precompute/🦀️.rs"]
            mod component;
            pub use component::*;
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⏳️precompute/🖌️brush/🦀️.rs"]
            pub mod brush;
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⏳️precompute/🪣️fill/🦀️.rs"]
            pub mod fill;
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⏳️precompute/📐️geometry/🦀️.rs"]
            pub mod geometry;
        }

        #[path = "."]
        pub mod config {
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎚️config/🦀️.rs"]
            mod component;
            pub use component::*;

            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎚️config/🧬️schema/🦀️.rs"]
            pub mod schema;
        }

        #[path = "."]
        pub mod window {
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🪟️window/🦀️.rs"]
            mod component;
            pub use component::*;
        }

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
        pub mod commands {
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🧊️document/➕️add-target-volume/🦀️.rs"]
            pub mod add_target_volume;
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🎛️runtime/☀️apply-sun/🦀️.rs"]
            pub mod apply_sun;
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🎛️runtime/🔒️close-vortex-suggestions/🦀️.rs"]
            pub mod close_vortex_suggestions;
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🎛️runtime/🔁️cycle-candidate/🦀️.rs"]
            pub mod cycle_candidate;
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🧊️document/💔️delete-attraction/🦀️.rs"]
            pub mod delete_attraction;
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🧊️document/🗑️delete-selection/🦀️.rs"]
            pub mod delete_selection;
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🧊️document/🪦️delete-target-volume/🦀️.rs"]
            pub mod delete_target_volume;
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🧊️document/👯️duplicate-selection/🦀️.rs"]
            pub mod duplicate_selection;
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🎛️runtime/🛑️engagement-abort/🦀️.rs"]
            pub mod engagement_abort;
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🎛️runtime/🎛️engagement-control-select/🦀️.rs"]
            pub mod engagement_control_select;
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🎛️runtime/⌨️engagement-input/🦀️.rs"]
            pub mod engagement_input;
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🎛️runtime/🔂️engagement-repeat-last/🦀️.rs"]
            pub mod engagement_repeat_last;
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🎛️runtime/📨️engagement-submit/🦀️.rs"]
            pub mod engagement_submit;
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🧊️document/📤️export-snapshot/🦀️.rs"]
            pub mod export_snapshot;
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🧊️document/🎯️focus-selection/🦀️.rs"]
            pub mod focus_selection;
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🎛️runtime/🖱️hover-suggestion/🦀️.rs"]
            pub mod hover_suggestion;
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🧊️document/📥️import-snapshot/🦀️.rs"]
            pub mod import_snapshot;
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🎛️runtime/🔓️open-vortex-suggestions/🦀️.rs"]
            pub mod open_vortex_suggestions;
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🧊️document/🗂️open-import-snapshot/🦀️.rs"]
            pub mod open_import_snapshot;
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🎛️runtime/📋️register-brush-mesh/🦀️.rs"]
            pub mod register_brush_mesh;
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🧊️document/🚚️relocate-target-volume/🦀️.rs"]
            pub mod relocate_target_volume;
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🧊️document/🔄️rotate-selection/🦀️.rs"]
            pub mod rotate_selection;
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🧊️document/📏️scale-selection/🦀️.rs"]
            pub mod scale_selection;
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🧊️document/🧬️select-same-kind/🦀️.rs"]
            pub mod select_same_kind;
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🎛️runtime/🤖️set-automatic/🦀️.rs"]
            pub mod set_automatic;
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🎛️runtime/🚧️set-brush-placement-contact-tolerance/🦀️.rs"]
            pub mod set_brush_placement_contact_tolerance;
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🎛️runtime/📷️set-camera/🦀️.rs"]
            pub mod set_camera;
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🎛️runtime/🧩️set-chunk-size/🦀️.rs"]
            pub mod set_chunk_size;
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🎛️runtime/📉️set-depth-variable/🦀️.rs"]
            pub mod set_depth_variable;
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🎛️runtime/🧮️set-fill-count/🦀️.rs"]
            pub mod set_fill_count;
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🎛️runtime/⚖️set-kind-weight/🦀️.rs"]
            pub mod set_kind_weight;
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🎛️runtime/✋️set-manual/🦀️.rs"]
            pub mod set_manual;
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🎛️runtime/📽️set-projection/🦀️.rs"]
            pub mod set_projection;
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🎛️runtime/📡️set-proximity-radius/🦀️.rs"]
            pub mod set_proximity_radius;
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🎛️runtime/☑️set-selectable-kind/🦀️.rs"]
            pub mod set_selectable_kind;
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🧊️document/🔖️set-selection-flag/🦀️.rs"]
            pub mod set_selection_flag;
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🎛️runtime/🧲️set-snap-enabled/🦀️.rs"]
            pub mod set_snap_enabled;
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🎛️runtime/↔️set-spacing/🦀️.rs"]
            pub mod set_spacing;
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🧊️document/🚩️set-target-volume-flag/🦀️.rs"]
            pub mod set_target_volume_flag;
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🎛️runtime/🕹️set-transform-gumball-flag/🦀️.rs"]
            pub mod set_transform_gumball_flag;
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🧊️document/👁️set-visible/🦀️.rs"]
            pub mod set_visible;
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🎛️runtime/🧭️set-vortex-direction/🦀️.rs"]
            pub mod set_vortex_direction;
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🎛️runtime/🌀️set-vortex-show/🦀️.rs"]
            pub mod set_vortex_show;
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🎛️runtime/📐️set-voxel-dims/🦀️.rs"]
            pub mod set_voxel_dims;
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🎛️runtime/🎣️target-brush-suggestions/🦀️.rs"]
            pub mod target_brush_suggestions;
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🧊️document/🚀️translate-selection/🦀️.rs"]
            pub mod translate_selection;
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🧊️document/🌍️world-relocate/🦀️.rs"]
            pub mod world_relocate;
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
                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/☑️options/🌐️grid/🦀️.rs"]
                    pub mod grid;
                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/☑️options/🔭️lod/🦀️.rs"]
                    pub mod lod;
                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/☑️options/🎥️projection/🦀️.rs"]
                    pub mod projection;
                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/☑️options/🎯️select/🦀️.rs"]
                    pub mod select;
                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/☑️options/☀️sun/🦀️.rs"]
                    pub mod sun;
                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/☑️options/🌀️vortex/🦀️.rs"]
                    pub mod vortex;
                }

                #[path = "."]
                pub mod tools {
                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🛠️tools/🪣️fill/🦀️.rs"]
                    pub mod fill;
                }

                #[path = "."]
                pub mod windows {
                    #[path = "."]
                    pub mod main {
                        #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/🧊️main/🦀️.rs"]
                        mod component;
                        pub use component::*;

                        #[path = "."]
                        pub mod utilities {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/🧊️main/🪛️utilities/🖌️brush/🦀️.rs"]
                            pub mod brush;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/🧊️main/🪛️utilities/🔄️transform/🦀️.rs"]
                            pub mod transform;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/🧊️main/🪛️utilities/🧊️volume-brush/🦀️.rs"]
                            pub mod volume_brush;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/🧊️main/🪛️utilities/🚚️world-relocate/🦀️.rs"]
                            pub mod world_relocate;
                        }
                    }
                }
            }
        }
    }
}

#[path = "."]
pub mod viewer {
    #[path = "."]
    pub mod puzzle3d {
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
                    pub mod main {
                        #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/👁️viewer/🎭️modes/👁️view/🪟️windows/🧊️main/🦀️.rs"]
                        mod component;
                        pub use component::*;
                    }
                }
            }
        }
    }
}

pub use crate::standards::v1::subsets::any::io::{Puzzle3dBuilderConstruction, Puzzle3dParts, Puzzle3dAnalyzerAnalysis, Puzzle3dBuilderFacets, Puzzle3dBuilder, Puzzle3dAnalyzer, Puzzle3dComposer};

#[path = "."]
pub mod host {
#[path = "🔨️modules/🏠️host/🧰️owned/🦀️.rs"]
pub mod owned;
}
