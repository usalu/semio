#![allow(async_fn_in_trait)]
#![allow(long_running_const_eval)]
//! 🎪 `stdio.dxf` artifact — stdio reference format.
use semio_framework_plugin::PluginBuilder;
#[cfg(test)] use semio_framework_artifact_reference::{Dialect,StandardId,SubsetId};

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_schema as framework_schema;
extern crate semio_framework_value_derive as value_derive;

pub use semio_s_artifact_stdio_contract::{apply_mutation, apply_mutation_checked, mutation_from_payload_json, mutation_inverse, mutation_payload_json, MutationRefusal};

pub use schema::diff::DxfDiff;
pub use schema::mutations::DxfMutation;
pub use schema::snapshot::{DxfArc, DxfCircle, DxfInsert, DxfLine, DxfOther, DxfPolyline, DxfSnapshot, DxfSolid, DxfText};
pub use schema::DxfArtifact;

pub use semio_s_artifact_stdio_dxf_snapshot::{STDIO_DXF_DOCUMENT_SCHEMA};

pub use semio_s_artifact_stdio_dxf_snapshot::{DXF_ARTIFACT_SCHEMA_ID};

pub use semio_s_artifact_stdio_dxf_snapshot::{ARTIFACT_DEFINITION_SCHEMA};

pub fn definition() -> Result<semio_framework_plugin::ArtifactDefinition, semio_framework_plugin::PluginAssemblyError> {
    let factories = native_codecs();
    let executables = semio_s_artifact_stdio_contract::native_codec_executables(ARTIFACT_DEFINITION_SCHEMA, &factories)?;
    semio_s_artifact_stdio_contract::definition_from_schema_with_executables(ARTIFACT_DEFINITION_SCHEMA, executables)
}

pub use semio_s_artifact_stdio_dxf_snapshot::{formats};

fn native_codec() -> store::os_store::serialization::ArtifactCodec {
    let mut codec = store::artifact_codec::<DxfSnapshot, DxfMutation>(STDIO_DXF_DOCUMENT_SCHEMA);
    codec.extension = "dxf";
    codec
}

pub fn native_codecs() -> Vec<semio_s_artifact_stdio_contract::NativeCodecFactory> {
    vec![semio_s_artifact_stdio_contract::NativeCodecFactory { id: "stdio.native.dxf.v1", artifact: "dxf", kind: artifact_kind, codec: native_codec }]
}

#[cfg(test)]
pub(crate) fn register_sqlite_test_declaration() {
    static REGISTERED: std::sync::OnceLock<()> = std::sync::OnceLock::new();
    REGISTERED.get_or_init(|| {
        semio_framework_plugin::PluginBuilder::<_, semio_framework_plugin::app::NoPluginApp>::new("stdio").label("DXF SQLite declaration").version("0.0.1").package_id("semio:stdio").artifact(declaration(definition().unwrap()).unwrap()).try_build().unwrap();
    });
}

#[test]
fn sqlite_snapshot_dxf_native_factory_publishes_owned_schema_hash() {
    let codec=native_codec();
    let actual=codec.pack_schema_hash.iter().map(|byte|format!("{byte:02x}")).collect::<String>();
    let source:serde_json::Value=serde_json::from_str(ARTIFACT_DEFINITION_SCHEMA).unwrap();
    assert_eq!(source["codecs"][0]["native_factory"]["pack_schema_hash"].as_str().unwrap(),actual,"owner structural hash {actual}");
}

pub fn contribution() -> semio_s_artifact_stdio_contract::ArtifactContribution {
    semio_s_artifact_stdio_contract::ArtifactContribution { definition_constraint: None, identity: "dxf", schema: ARTIFACT_DEFINITION_SCHEMA, definition, assembly, formats, native_codecs }
}

//#region 🔖️Declaration
/// 🔖️ This artifact's declaration (ticket 26/08/12/ARTIFACTS-ONLY-PLUGIN-ARCHITECTURE W6, g4) —
/// replaces the old side-effecting `crate::engine::register()`, which the plugin
/// root used to call unconditionally before `PluginBuilder::new(...)` was even constructed. Mirrors
/// `🗜️deflate`'s own `s.stdio.deflate` exemplar exactly: a headless library artifact with zero
/// `ArtifactApp`s, so `.document_codec_bare::<Snapshot, Mutation>(schema)` stands in for
/// `store::os_store::codec_registry::register_document_codec(store::artifact_codec::<DxfSnapshot, DxfMutation>(...))`.
/// `.composers(...)` reaches the ENGINE's own `io_registry` (returns `&'static [ComposerEntry]`,
/// owned rows) by its full path through the `engine` re-export (`🦀️.rs`'s `pub mod engine { pub use
/// super::standards::v_r12::engine::*; }`) — deliberately NOT this file's own `io_registry` module
/// below, whose `entries()` returns `&'static [&'static ComposerEntry]` (references) and would
/// silently rebind under a bare call (this ticket's "SILENT REBIND" hazard). dxf's own
/// `register_pilot_languages()` had no `register_schema_specs()` call, so unlike `dwg`/`obj`/`las`
/// there is no `.setup()` survivor needed here — every registration `engine::register()` performed
/// is covered by a declaration field.
/// 🧩️ Binds this executable root to its sole schema-owned definition.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn assembly() -> Result<semio_s_artifact_stdio_contract::ArtifactAssembly, semio_framework_plugin::PluginAssemblyError> {
    semio_s_artifact_stdio_contract::runtime_assembly("dxf", definition()?, declaration)
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn declaration(definition: semio_framework_plugin::ArtifactDefinition) -> Result<semio_framework_plugin::ArtifactDeclaration, semio_framework_plugin::ArtifactDefinitionError> {
    let formats = formats()?;
    semio_framework_plugin::ArtifactDeclaration::builder(definition)
        .schema(schema::dxf_artifact_schema_descriptor())
        .formats(formats)
        .inferences([schema::inferences::dxf_artifact_inference_descriptor()])
        .composers(engine::io_registry::entries())
        .languages(pilot_languages())
        .document_codec_bare::<DxfSnapshot, DxfMutation>(STDIO_DXF_DOCUMENT_SCHEMA, semio_framework_artifact_reference::Dialect { artifact_kind: "s.stdio.dxf", standard: semio_framework_artifact_reference::StandardId("r12"), subset: semio_framework_artifact_reference::SubsetId("*") })
        .try_build()
}

/// 📌️ Handcrafted facet grammars (text) and protocols (binary) for in-process execution — built
/// once and leaked to a `&'static` slice since `dsl::passthrough_hooks` isn't `const fn`, copied
/// verbatim (five `LanguageSpec` rows, one per role) from `crate::standards::v_r12::
/// engine::register_pilot_languages`'s own `dsl::register_language(...)` call bodies.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn pilot_languages() -> &'static [semio_framework_dsl::LanguageSpec] {
    static LANGUAGES: std::sync::OnceLock<Vec<semio_framework_dsl::LanguageSpec>> = std::sync::OnceLock::new();
    LANGUAGES
        .get_or_init(|| {
            vec![
                semio_framework_dsl::LanguageSpec {
                    id: "stdio.dxf",
                    extension: Some("dxf"),
                    role: semio_framework_dsl::LanguageRole::Document,
                    grammar: Some(standards::v_r12::subsets::any::io::text::snapshot::COMPONENT_GRAMMAR_SEMIO),
                    grammar_path: Some(standards::v_r12::subsets::any::io::text::snapshot::COMPONENT_GRAMMAR_PATH),
                    protocol: Some(standards::v_r12::subsets::any::io::binary::snapshot::COMPONENT_PROTOCOL_SEMIO),
                    protocol_path: Some(standards::v_r12::subsets::any::io::binary::snapshot::COMPONENT_PROTOCOL_PATH),
                    hooks: semio_framework_dsl::passthrough_hooks("stdio.dxf"),
                },
                semio_framework_dsl::LanguageSpec {
                    id: "stdio.dxf.op",
                    extension: None,
                    role: semio_framework_dsl::LanguageRole::Ops,
                    grammar: Some(standards::v_r12::subsets::any::io::text::mutations::COMPONENT_GRAMMAR_SEMIO),
                    grammar_path: Some(standards::v_r12::subsets::any::io::text::mutations::COMPONENT_GRAMMAR_PATH),
                    protocol: Some(standards::v_r12::subsets::any::io::binary::mutations::COMPONENT_PROTOCOL_SEMIO),
                    protocol_path: Some(standards::v_r12::subsets::any::io::binary::mutations::COMPONENT_PROTOCOL_PATH),
                    hooks: semio_framework_dsl::passthrough_hooks("stdio.dxf.op"),
                },
                semio_framework_dsl::LanguageSpec {
                    id: "stdio.dxf.diff",
                    extension: None,
                    role: semio_framework_dsl::LanguageRole::Diff,
                    grammar: Some(standards::v_r12::subsets::any::io::text::diff::COMPONENT_GRAMMAR_SEMIO),
                    grammar_path: Some(standards::v_r12::subsets::any::io::text::diff::COMPONENT_GRAMMAR_PATH),
                    protocol: None,
                    protocol_path: None,
                    hooks: semio_framework_dsl::passthrough_hooks("stdio.dxf.diff"),
                },
                semio_framework_dsl::LanguageSpec {
                    id: "stdio.dxf.pack",
                    extension: None,
                    role: semio_framework_dsl::LanguageRole::Pack,
                    grammar: None,
                    grammar_path: None,
                    protocol: Some(standards::v_r12::subsets::any::io::binary::snapshot::COMPONENT_PROTOCOL_SEMIO),
                    protocol_path: Some(standards::v_r12::subsets::any::io::binary::snapshot::COMPONENT_PROTOCOL_PATH),
                    hooks: semio_framework_dsl::passthrough_hooks("stdio.dxf.pack"),
                },
                semio_framework_dsl::LanguageSpec {
                    id: "stdio.dxf.spr",
                    extension: None,
                    role: semio_framework_dsl::LanguageRole::Spr,
                    grammar: None,
                    grammar_path: None,
                    protocol: Some(standards::v_r12::subsets::any::io::binary::mutations::COMPONENT_PROTOCOL_SEMIO),
                    protocol_path: Some(standards::v_r12::subsets::any::io::binary::mutations::COMPONENT_PROTOCOL_PATH),
                    hooks: semio_framework_dsl::passthrough_hooks("stdio.dxf.spr"),
                },
            ]
        })
        .as_slice()
}
//#endregion 🔖️Declaration

pub use semio_s_artifact_stdio_dxf_snapshot::{artifact_kind};
//#region 🚪️DerivedIoRegistry
pub use semio_s_artifact_stdio_dxf_tests::io_registry;
//#endregion 🚪️DerivedIoRegistry

#[path = "."]
pub mod standards {
    #[path = "."]
    pub mod v_r12 {
        // ⚙️→🚪️/🧬️ dissolved (ticket 26/08/12/ENGINELESS-ARTIFACTS-AND-APP-STATE-MACHINES):
        // real code now lives in `subsets::any::{io,schema}`; this stays an inline barrel
        // so every existing `standards::v_r12::engine::*`/root `engine::*` path still resolves.
        pub use semio_s_artifact_stdio_dxf_tests::standards::v_r12::engine;
        #[path = "."]
        pub mod subsets {
            #[path = "."]
            pub mod any {
                #[path = "."]
                pub mod schema {
                    pub use semio_s_artifact_stdio_dxf_schema::standards::v_r12::subsets::any::schema::*;

                    pub use semio_s_artifact_stdio_dxf_snapshot::standards::v_r12::subsets::any::schema::snapshot;
                    pub use semio_s_artifact_stdio_dxf_diff::standards::v_r12::subsets::any::schema::diff;
                    pub use semio_s_artifact_stdio_dxf_inferences::standards::v_r12::subsets::any::schema::inferences;
                    pub use semio_s_artifact_stdio_dxf_mutations::standards::v_r12::subsets::any::schema::mutations;
                }
                #[path = "."]
                pub mod io {
                    pub use semio_s_artifact_stdio_dxf_tests::standards::v_r12::subsets::any::io::*;
pub mod binary {
    pub use semio_s_artifact_stdio_dxf_tests::standards::v_r12::subsets::any::io::binary::*;
    pub use semio_s_artifact_stdio_dxf_inferences::standards::v_r12::subsets::any::io::binary::inferences;
}
pub mod sqlite {
    pub use semio_s_artifact_stdio_dxf_inferences::standards::v_r12::subsets::any::io::sqlite::*;
    pub use semio_s_artifact_stdio_dxf_snapshot::standards::v_r12::subsets::any::io::sqlite::snapshot;
}
pub mod text {
    pub use semio_s_artifact_stdio_dxf_tests::standards::v_r12::subsets::any::io::text::*;
    pub use semio_s_artifact_stdio_dxf_inferences::standards::v_r12::subsets::any::io::text::inferences;
}

                    pub use semio_s_artifact_stdio_dxf_inferences::standards::v_r12::subsets::any::io::import;
                    pub use semio_s_artifact_stdio_dxf_inferences::standards::v_r12::subsets::any::io::export;
                }
            }
        }
    }
}

// ---- Crate-root module paths: the crate's public surface ----
pub mod schema {
    pub use super::standards::v_r12::subsets::any::schema::*;
}
pub mod engine {
    pub use super::standards::v_r12::engine::*;
}

pub use semio_s_artifact_stdio_dxf_inferences::examples;

#[path = "."]
pub mod editor {
    #[path = "."]
    pub mod dxf {
        pub use semio_s_artifact_stdio_dxf_tests::editor::dxf::*;

        pub use semio_s_artifact_stdio_dxf_inferences::editor::dxf::edit_rules;
        pub use semio_s_artifact_stdio_dxf_inferences::editor::dxf::presence;
        pub use semio_s_artifact_stdio_dxf_inferences::editor::dxf::config;
        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod edit {
                pub use semio_s_artifact_stdio_dxf_mutations::editor::dxf::modes::edit::*;

                pub use semio_s_artifact_stdio_dxf_inferences::editor::dxf::modes::edit::windows;
            }
        }
    }
}

#[path = "."]
pub mod viewer {
    #[path = "."]
    pub mod dxf {
        pub use semio_s_artifact_stdio_dxf_tests::viewer::dxf::*;

        pub use semio_s_artifact_stdio_dxf_inferences::viewer::dxf::presence;
        pub use semio_s_artifact_stdio_dxf_inferences::viewer::dxf::config;
        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod view {
                pub use semio_s_artifact_stdio_dxf_mutations::viewer::dxf::modes::view::*;

                pub use semio_s_artifact_stdio_dxf_inferences::viewer::dxf::modes::view::windows;
            }
        }
    }
}

pub use crate::standards::v_r12::subsets::any::io::{DxfBuilderConstruction, DxfParts, DxfAnalyzerAnalysis, DxfBuilderFacets, DxfBuilder, DxfAnalyzer, DxfComposer};

#[cfg(test)]
pub(crate) use semio_framework_trace_memory_testing as test_allocation;
#[cfg(test)]
pub use semio_s_artifact_stdio_dxf_snapshot::{TEST_ALLOCATION_OBSERVER};
#[cfg(test)]
#[path="🏅️standards/🔖️r12/🪆️subsets/📰️header/🧬️schema/♻️retirement/🧪️tests/🦀️.rs"]
mod original_owner_tests;

#[cfg(test)]
#[path = "🏅️standards/🔖️r12/🪆️subsets/📰️header/🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🦀️.rs"]
mod hoisted_snapshot_0;
