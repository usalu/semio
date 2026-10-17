//! 🎪 `stdio.svg` artifact — stdio reference format.

#![allow(async_fn_in_trait)]
#![allow(long_running_const_eval)]

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_schema as framework_schema;
extern crate semio_framework_value_derive as value_derive;

pub use semio_s_artifact_stdio_contract::{apply_mutation_checked, mutation_from_payload_json, mutation_inverse, mutation_payload_json, MutationRefusal};

pub(crate) use semio_s_artifact_stdio_contract::base64_standard;
pub(crate) use semio_s_artifact_stdio_contract::impl_serde_op_codec;

use {semio_framework_artifact_reference::Dialect, semio_framework_artifact_reference::StandardId, semio_framework_artifact_reference::SubsetId};

pub use schema::diff::SvgDiff;
pub use schema::mutations::SvgMutation;
pub use schema::snapshot::SvgSnapshot;
pub use schema::SvgArtifact;

pub use semio_s_artifact_stdio_svg_snapshot::{STDIO_SVG_DOCUMENT_SCHEMA};

pub use semio_s_artifact_stdio_svg_snapshot::{SVG_ARTIFACT_SCHEMA_ID};

pub use semio_s_artifact_stdio_svg_snapshot::{ARTIFACT_DEFINITION_SCHEMA};

pub fn definition() -> Result<semio_framework_plugin::ArtifactDefinition, semio_framework_plugin::PluginAssemblyError> {
    let factories = native_codecs();
    let executables = semio_s_artifact_stdio_contract::native_codec_executables(ARTIFACT_DEFINITION_SCHEMA, &factories)?;
    semio_s_artifact_stdio_contract::definition_from_schema_with_executables(ARTIFACT_DEFINITION_SCHEMA, executables)
}

pub use semio_s_artifact_stdio_svg_snapshot::{formats};

fn native_codec() -> store::os_store::serialization::ArtifactCodec {
    let mut codec = store::artifact_codec::<SvgSnapshot, SvgMutation>(STDIO_SVG_DOCUMENT_SCHEMA);
    codec.extension = "svg";
    codec.pack_schema_hash = semio_framework_hash::Sha256::digest(include_bytes!("🏅️standards/🔖️1.1/🪆️subsets/🧱️base/🚪️io/💾️binary/📸️snapshot/📡️.protocol.semio"));
    codec
}

pub fn native_codecs() -> Vec<semio_s_artifact_stdio_contract::NativeCodecFactory> {
    vec![semio_s_artifact_stdio_contract::NativeCodecFactory { id: "stdio.native.svg.v1", artifact: "svg", kind: artifact_kind, codec: native_codec }]
}

pub fn contribution() -> semio_s_artifact_stdio_contract::ArtifactContribution {
    semio_s_artifact_stdio_contract::ArtifactContribution { definition_constraint: None, identity: "svg", schema: ARTIFACT_DEFINITION_SCHEMA, definition, assembly, formats, native_codecs }
}

pub use semio_s_artifact_stdio_svg_mut_family::{SVG_ANY_DIALECT, SVG_BASIC_DIALECT, SVG_TINY_DIALECT};

pub use semio_s_artifact_stdio_svg_snapshot::{artifact_kind};

//#region 🔖️Declaration
/// 🔖️ This artifact's declaration (ticket 26/08/12/ARTIFACTS-ONLY-PLUGIN-ARCHITECTURE W6) —
/// replaces the old side-effecting `crate::engine::register()`, previously called
/// unconditionally from `🗄️stdio`'s plugin root. Mirrors `🗒️note`/`🔋️model`'s own `declaration()`
/// exemplars: `.composers(...)` reads the real `ComposerEntry` rows (✳️any + 🔬️tiny + 🔰️basic folded
/// into one list) from `io_registry::v1_1` below, which sits at the root so the ✳️any subset does not
/// have to name the 🔬️tiny and 🔰️basic composers that are built on top of it. Both subset dialects'
/// `SubsetValidator`s (`🔬️tiny`/`🔰️basic`'s own `SvgTinyValidator`/`SvgBasicValidator`, previously
/// registered via `⚙️engine::register()`'s two trailing `subsets::{tiny,basic}::io::register()`
/// calls) are re-derived here via `subset_validator_entry_of::<…>()` rather than reused from those
/// files' own private `validator_entry()` caches (neither is `pub`) — same erasure helper, fresh
/// instances, same registry effect. `⚙️engine` itself is untouched — this only REFERENCES what it
/// already exposes.
/// 🧩️ Binds this executable root to its sole schema-owned definition.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn assembly() -> Result<semio_s_artifact_stdio_contract::ArtifactAssembly, semio_framework_plugin::PluginAssemblyError> {
    semio_s_artifact_stdio_contract::runtime_assembly("svg", definition()?, declaration)
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn declaration(definition: semio_framework_plugin::ArtifactDefinition) -> Result<semio_framework_plugin::ArtifactDeclaration, semio_framework_plugin::ArtifactDefinitionError> {
    let formats = formats()?;
    semio_framework_plugin::ArtifactDeclaration::builder(definition)
        .schema(standards::v1_1::subsets::base::schema::svg_artifact_schema_descriptor())
        .formats(formats)
        .schema_documents(semio_framework_schema_registry::ScopeSchemaExports { scope: "s.stdio.svg", exports: &[semio_framework_schema_registry::SchemaExport { id: "xml-document", leaves: semio_s_artifact_stdio_xml::schema::XML_DOCUMENT_SCHEMA_LEAVES }] })
        .inferences([standards::v1_1::subsets::base::schema::inferences::svg_artifact_inference_descriptor()])
        .composers(io_registry::v1_1::entries())
        .subset_validators(declared_subset_validators())
        .languages(pilot_languages())
        .document_codec_bare::<SvgSnapshot, SvgMutation>(STDIO_SVG_DOCUMENT_SCHEMA, Dialect { artifact_kind: "s.stdio.svg", standard: StandardId("1.1"), subset: SubsetId("*") })
        .document_codec_bare::<SvgSnapshot, SvgMutation>(STDIO_SVG_DOCUMENT_SCHEMA, Dialect { artifact_kind: "s.stdio.svg", standard: StandardId("1.1"), subset: SubsetId("tiny") })
        .document_codec_bare::<SvgSnapshot, SvgMutation>(STDIO_SVG_DOCUMENT_SCHEMA, Dialect { artifact_kind: "s.stdio.svg", standard: StandardId("1.1"), subset: SubsetId("basic") })
        .try_build()
}

/// 🛡️ Re-derives the 🔬️tiny and 🔰️basic subsets' `SubsetValidatorEntry`s — see `declaration()`'s own
/// doc for why this calls `subset_validator_entry_of` directly instead of reusing either subset's
/// own private cache.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn declared_subset_validators() -> &'static [semio_framework_plugin::io::SubsetValidatorEntry] {
    static ENTRIES: std::sync::OnceLock<Vec<semio_framework_plugin::io::SubsetValidatorEntry>> = std::sync::OnceLock::new();
    ENTRIES
        .get_or_init(|| vec![semio_framework_plugin::io::subset_validator_entry_of::<standards::v1_1::subsets::tiny::io::SvgTinyValidator>(), semio_framework_plugin::io::subset_validator_entry_of::<standards::v1_1::subsets::basic::io::SvgBasicValidator>()])
        .as_slice()
}

/// 📌️ Handcrafted facet grammars (text) and protocols (binary) for in-process execution — moved
/// here verbatim from `⚙️engine::register_pilot_languages` (same 5-role Document/Ops/Diff/Pack/Spr
/// shape every stdio artifact uses), leaked to a `&'static` slice since `dsl::passthrough_hooks`
/// isn't `const fn`, mirroring the `🗒️note`/`🔋️model` exemplars' own helper of the same shape.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn pilot_languages() -> &'static [semio_framework_dsl::LanguageSpec] {
    static LANGUAGES: std::sync::OnceLock<Vec<semio_framework_dsl::LanguageSpec>> = std::sync::OnceLock::new();
    LANGUAGES
        .get_or_init(|| {
            vec![
                semio_framework_dsl::LanguageSpec {
                    id: "stdio.svg",
                    extension: Some("svg"),
                    role: semio_framework_dsl::LanguageRole::Document,
                    grammar: Some(standards::v1_1::subsets::base::io::text::snapshot::COMPONENT_GRAMMAR_SEMIO),
                    grammar_path: Some(standards::v1_1::subsets::base::io::text::snapshot::COMPONENT_GRAMMAR_PATH),
                    protocol: Some(standards::v1_1::subsets::base::io::binary::snapshot::COMPONENT_PROTOCOL_SEMIO),
                    protocol_path: Some(standards::v1_1::subsets::base::io::binary::snapshot::COMPONENT_PROTOCOL_PATH),
                    hooks: semio_framework_dsl::passthrough_hooks("stdio.svg"),
                },
                semio_framework_dsl::LanguageSpec {
                    id: "stdio.svg.op",
                    extension: None,
                    role: semio_framework_dsl::LanguageRole::Ops,
                    grammar: Some(standards::v1_1::subsets::base::io::text::mutations::COMPONENT_GRAMMAR_SEMIO),
                    grammar_path: Some(standards::v1_1::subsets::base::io::text::mutations::COMPONENT_GRAMMAR_PATH),
                    protocol: Some(standards::v1_1::subsets::base::io::binary::mutations::COMPONENT_PROTOCOL_SEMIO),
                    protocol_path: Some(standards::v1_1::subsets::base::io::binary::mutations::COMPONENT_PROTOCOL_PATH),
                    hooks: semio_framework_dsl::passthrough_hooks("stdio.svg.op"),
                },
                semio_framework_dsl::LanguageSpec {
                    id: "stdio.svg.diff",
                    extension: None,
                    role: semio_framework_dsl::LanguageRole::Diff,
                    grammar: Some(standards::v1_1::subsets::base::io::text::diff::COMPONENT_GRAMMAR_SEMIO),
                    grammar_path: Some(standards::v1_1::subsets::base::io::text::diff::COMPONENT_GRAMMAR_PATH),
                    protocol: None,
                    protocol_path: None,
                    hooks: semio_framework_dsl::passthrough_hooks("stdio.svg.diff"),
                },
                semio_framework_dsl::LanguageSpec {
                    id: "stdio.svg.pack",
                    extension: None,
                    role: semio_framework_dsl::LanguageRole::Pack,
                    grammar: None,
                    grammar_path: None,
                    protocol: Some(standards::v1_1::subsets::base::io::binary::snapshot::COMPONENT_PROTOCOL_SEMIO),
                    protocol_path: Some(standards::v1_1::subsets::base::io::binary::snapshot::COMPONENT_PROTOCOL_PATH),
                    hooks: semio_framework_dsl::passthrough_hooks("stdio.svg.pack"),
                },
                semio_framework_dsl::LanguageSpec {
                    id: "stdio.svg.spr",
                    extension: None,
                    role: semio_framework_dsl::LanguageRole::Spr,
                    grammar: None,
                    grammar_path: None,
                    protocol: Some(standards::v1_1::subsets::base::io::binary::mutations::COMPONENT_PROTOCOL_SEMIO),
                    protocol_path: Some(standards::v1_1::subsets::base::io::binary::mutations::COMPONENT_PROTOCOL_PATH),
                    hooks: semio_framework_dsl::passthrough_hooks("stdio.svg.spr"),
                },
            ]
        })
        .as_slice()
}
//#endregion 🔖️Declaration

//#region 🚪️DerivedIoRegistry
pub use semio_s_artifact_stdio_svg_mutations_2::io_registry;
//#endregion 🚪️DerivedIoRegistry

#[path = "."]
pub mod standards {
    #[path = "."]
    pub mod v1_1 {

        #[path = "."]
        pub mod subsets {
            #[path = "."]
            pub mod base {
                #[path = "."]
                pub mod schema {
                    pub use semio_s_artifact_stdio_svg_schema::standards::v1_1::subsets::base::schema::*;

                    pub use semio_s_artifact_stdio_svg_schema::standards::v1_1::subsets::base::schema::mutation_support;
                    pub use semio_s_artifact_stdio_svg_snapshot::standards::v1_1::subsets::base::schema::snapshot;
                    pub use semio_s_artifact_stdio_svg_schema::standards::v1_1::subsets::base::schema::inferences;
                    pub use semio_s_artifact_stdio_svg_snapshot::standards::v1_1::subsets::base::schema::diff;
                    pub use semio_s_artifact_stdio_svg_mutations::standards::v1_1::subsets::base::schema::mutations;
                }
                #[path = "."]
                pub mod io {
                    pub use semio_s_artifact_stdio_svg_mutations_2::standards::v1_1::subsets::base::io::*;
pub mod sqlite {
    pub use semio_s_artifact_stdio_svg_handcrafted_diff_codec_tests::standards::v1_1::subsets::base::io::sqlite::*;
    pub use semio_s_artifact_stdio_svg_snapshot::standards::v1_1::subsets::base::io::sqlite::snapshot;
}
pub mod text {
    pub use semio_s_artifact_stdio_svg_mutations_2::standards::v1_1::subsets::base::io::text::*;
    pub use semio_s_artifact_stdio_svg_handcrafted_diff_codec_tests::standards::v1_1::subsets::base::io::text::inferences;
}

                    pub use semio_s_artifact_stdio_svg_handcrafted_diff_codec_tests::standards::v1_1::subsets::base::io::import;
                    pub use semio_s_artifact_stdio_svg_handcrafted_diff_codec_tests::standards::v1_1::subsets::base::io::export;
                }
            }
            #[path = "."]
            pub mod tiny {
                // 🏅️ SVG Tiny 1.1 (W3C Mobile SVG Profiles, REC-SVGMobile-20030114 §SVG
                // Tiny 1.1). `schema` re-exports the ✳️any subset's `SvgSnapshot`
                // verbatim (same Rust type, same `s.stdio.svg` schema id); `io` reuses
                // the ✳️any subset's xml import/export leaves rather than duplicating
                // them. Ticket 26/08/11/ARTIFACT-STANDARD-SUBSETS-REAL-VOCABULARIES.
                pub use semio_s_artifact_stdio_svg_mutations_2::standards::v1_1::subsets::tiny::io;
                pub use semio_s_artifact_stdio_svg_schema_2::standards::v1_1::subsets::tiny::schema;
            }
            #[path = "."]
            pub mod basic {
                // 🏅️ SVG Basic 1.1 (W3C Mobile SVG Profiles, REC-SVGMobile-20030114 §SVG
                // Basic 1.1). Same shape as `✳️tiny` above.
                pub use semio_s_artifact_stdio_svg_mutations_2::standards::v1_1::subsets::basic::io;
                pub use semio_s_artifact_stdio_svg_schema_3::standards::v1_1::subsets::basic::schema;
            }
        }
    }
}

pub mod schema {
    pub use super::standards::v1_1::subsets::base::schema::*;
}

pub use semio_s_artifact_stdio_svg_handcrafted_diff_codec_tests::examples;

#[path = "."]
pub mod editor {
    #[path = "."]
    pub mod svg_any {
        pub use semio_s_artifact_stdio_svg_mut_family::editor::svg_any::*;

        pub use semio_s_artifact_stdio_svg_handcrafted_diff_codec_tests::editor::svg_any::presence;
        pub use semio_s_artifact_stdio_svg_handcrafted_diff_codec_tests::editor::svg_any::config;
        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod edit {
                pub use semio_s_artifact_stdio_svg_mutations::editor::svg_any::modes::edit::*;

                pub use semio_s_artifact_stdio_svg_mutations::editor::svg_any::modes::edit::windows;
            }
        }
    }
    #[path = "."]
    pub mod svg_basic {
        pub use semio_s_artifact_stdio_svg_schema_3::editor::svg_basic::*;

        pub use semio_s_artifact_stdio_svg_handcrafted_diff_codec_tests::editor::svg_basic::presence;
        pub use semio_s_artifact_stdio_svg_handcrafted_diff_codec_tests::editor::svg_basic::config;
        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod edit {
                pub use semio_s_artifact_stdio_svg_schema_3::editor::svg_basic::modes::edit::*;

                pub use semio_s_artifact_stdio_svg_schema_3::editor::svg_basic::modes::edit::windows;
            }
        }
    }
    #[path = "."]
    pub mod svg_tiny {
        pub use semio_s_artifact_stdio_svg_schema_2::editor::svg_tiny::*;

        pub use semio_s_artifact_stdio_svg_handcrafted_diff_codec_tests::editor::svg_tiny::presence;
        pub use semio_s_artifact_stdio_svg_handcrafted_diff_codec_tests::editor::svg_tiny::config;
        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod edit {
                pub use semio_s_artifact_stdio_svg_schema_2::editor::svg_tiny::modes::edit::*;

                pub use semio_s_artifact_stdio_svg_schema_2::editor::svg_tiny::modes::edit::windows;
            }
        }
    }
}

#[path = "."]
pub mod viewer {
    #[path = "."]
    pub mod svg_any {
        pub use semio_s_artifact_stdio_svg_mut_family::viewer::svg_any::*;

        pub use semio_s_artifact_stdio_svg_handcrafted_diff_codec_tests::viewer::svg_any::presence;
        pub use semio_s_artifact_stdio_svg_handcrafted_diff_codec_tests::viewer::svg_any::config;
        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod view {
                pub use semio_s_artifact_stdio_svg_tests::viewer::svg_any::modes::view::*;

                pub use semio_s_artifact_stdio_svg_mutations::viewer::svg_any::modes::view::windows;
            }
        }
    }
    #[path = "."]
    pub mod svg_basic {
        pub use semio_s_artifact_stdio_svg_schema_3::viewer::svg_basic::*;

        pub use semio_s_artifact_stdio_svg_handcrafted_diff_codec_tests::viewer::svg_basic::presence;
        pub use semio_s_artifact_stdio_svg_handcrafted_diff_codec_tests::viewer::svg_basic::config;
        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod view {
                pub use semio_s_artifact_stdio_svg_schema_3::viewer::svg_basic::modes::view::*;

                pub use semio_s_artifact_stdio_svg_schema_3::viewer::svg_basic::modes::view::windows;
            }
        }
    }
    #[path = "."]
    pub mod svg_tiny {
        pub use semio_s_artifact_stdio_svg_schema_2::viewer::svg_tiny::*;

        pub use semio_s_artifact_stdio_svg_handcrafted_diff_codec_tests::viewer::svg_tiny::presence;
        pub use semio_s_artifact_stdio_svg_handcrafted_diff_codec_tests::viewer::svg_tiny::config;
        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod view {
                pub use semio_s_artifact_stdio_svg_schema_2::viewer::svg_tiny::modes::view::*;

                pub use semio_s_artifact_stdio_svg_schema_2::viewer::svg_tiny::modes::view::windows;
            }
        }
    }
}

pub use crate::standards::v1_1::subsets::tiny::io::{SvgTinyBuilderConstruction, SvgTinyAnalyzerAnalysis, SvgTinyBuilderFacets, SvgTinyBuilder, SvgTinyAnalyzer, SvgTinyComposer};

pub use crate::standards::v1_1::subsets::base::io::{PathBuilder, GradientStopSpec, ElementBuilder, SvgBuilderConstruction, SvgParts, SvgAnalyzerAnalysis, SvgBuilderFacets, SvgBuilder, SvgAnalyzer, SvgComposer};

pub use crate::standards::v1_1::subsets::basic::io::{SvgBasicBuilderConstruction, SvgBasicAnalyzerAnalysis, SvgBasicBuilderFacets, SvgBasicBuilder, SvgBasicAnalyzer, SvgBasicComposer};

#[cfg(test)]
#[path = "🏅️standards/🔖️1.1/🪆️subsets/🧱️base/🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🦀️.rs"]
mod hoisted_snapshot_0;
