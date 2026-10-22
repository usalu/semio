//! 🎪 `stdio.tiff` artifact — stdio reference format.

#![allow(async_fn_in_trait)]
#![allow(long_running_const_eval)]

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_schema as framework_schema;
extern crate semio_framework_value_derive as value_derive;

pub use semio_s_artifact_stdio_contract::{apply_mutation, apply_mutation_checked, mutation_from_payload_json, mutation_inverse, mutation_payload_json, MutationRefusal};

pub(crate) use semio_s_artifact_stdio_contract::impl_serde_op_codec;

use {semio_framework_artifact_reference::Dialect, semio_framework_artifact_reference::StandardId, semio_framework_artifact_reference::SubsetId};

pub use schema::diff::TiffDiff;
pub use schema::mutations::TiffMutation;
pub use schema::snapshot::TiffSnapshot;
pub use schema::TiffArtifact;

pub use semio_s_artifact_stdio_tiff_snapshot::{STDIO_TIFF_DOCUMENT_SCHEMA};

pub use semio_s_artifact_stdio_tiff_snapshot::{TIFF_ARTIFACT_SCHEMA_ID};

pub use semio_s_artifact_stdio_tiff_snapshot::{ARTIFACT_DEFINITION_SCHEMA};

pub fn definition() -> Result<semio_framework_plugin::ArtifactDefinition, semio_framework_plugin::PluginAssemblyError> {
    let factories = native_codecs();
    let executables = semio_s_artifact_stdio_contract::native_codec_executables(ARTIFACT_DEFINITION_SCHEMA, &factories)?;
    semio_s_artifact_stdio_contract::definition_from_schema_with_executables(ARTIFACT_DEFINITION_SCHEMA, executables)
}

pub use semio_s_artifact_stdio_tiff_snapshot::{formats};

fn native_codec() -> store::os_store::serialization::ArtifactCodec {
    let mut codec = store::bare_artifact_codec::<TiffSnapshot, TiffMutation>(STDIO_TIFF_DOCUMENT_SCHEMA);
    codec.extension = "tiff";
    codec.pack_schema_hash = semio_framework_hash::Sha256::digest(include_bytes!("🏅️standards/🔖️6.0/🪆️subsets/🧾️document/🚪️io/💾️binary/📸️snapshot/📡️.protocol.semio"));
    codec
}

pub fn native_codecs() -> Vec<semio_s_artifact_stdio_contract::NativeCodecFactory> {
    vec![semio_s_artifact_stdio_contract::NativeCodecFactory { id: "stdio.native.tiff.v1", artifact: "tiff", kind: artifact_kind, codec: native_codec }]
}

pub fn contribution() -> semio_s_artifact_stdio_contract::ArtifactContribution {
    semio_s_artifact_stdio_contract::ArtifactContribution { definition_constraint: None, identity: "tiff", schema: ARTIFACT_DEFINITION_SCHEMA, definition, assembly, formats, native_codecs }
}

pub use semio_s_artifact_stdio_tiff_mutation_regressions::{TIFF_ANY_DIALECT, TIFF_BASELINE_DIALECT};

pub use semio_s_artifact_stdio_tiff_snapshot::{artifact_kind};

//#region 🔖️Declaration
/// 🔖️ This artifact's declaration (ticket 26/08/12/ARTIFACTS-ONLY-PLUGIN-ARCHITECTURE W6) —
/// replaces the old side-effecting `crate::engine::register()`, previously called
/// unconditionally from `🗄️stdio`'s plugin root. Mirrors `🗒️note`/`🔋️model`'s own `declaration()`
/// exemplars: `.composers(...)` reaches `⚙️engine`'s OWN `io_registry` (the real `ComposerEntry`
/// rows — ✳️any + 🧱️baseline already folded into one list there) by its FULLY QUALIFIED path,
/// never the bare `io_registry::entries()` shortcut that would silently rebind to this file's own
/// shadowing `io_registry` module above/below (repo-wide "silent rebind" hazard this ticket
/// tracks — that module returns `&[&ComposerEntry]`, a different type, and is left in place as
/// orphaned dead code, matching `🔋️model`'s own precedent for its orphaned wrapper). The baseline
/// subset's `SubsetValidator` (`🧱️baseline/🚪️io`'s own `TiffBaselineValidator`, previously
/// registered via `⚙️engine::register()`'s trailing `subsets::baseline::io::register()` call) is
/// re-derived here via `subset_validator_entry_of::<TiffBaselineValidator>()` rather than reused
/// from that file's own private `validator_entry()` cache (not `pub`) — same erasure helper, fresh
/// instance, same registry effect. `⚙️engine` itself is untouched — this only REFERENCES what it
/// already exposes.
/// 🧩️ Binds this executable root to its sole schema-owned definition.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn assembly() -> Result<semio_s_artifact_stdio_contract::ArtifactAssembly, semio_framework_plugin::PluginAssemblyError> {
    semio_s_artifact_stdio_contract::runtime_assembly("tiff", definition()?, declaration)
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn declaration(definition: semio_framework_plugin::ArtifactDefinition) -> Result<semio_framework_plugin::ArtifactDeclaration, semio_framework_plugin::ArtifactDefinitionError> {
    let formats = formats()?;
    semio_framework_plugin::ArtifactDeclaration::builder(definition)
        .schema(standards::v6_0::subsets::document::schema::tiff_artifact_schema_descriptor())
        .formats(formats)
        .inferences([standards::v6_0::subsets::document::schema::inferences::tiff_artifact_inference_descriptor()])
        .composers(standards::v6_0::subsets::baseline::io::io_registry::entries())
        .subset_validators(declared_subset_validators())
        .languages(pilot_languages())
        .document_codec_bare::<TiffSnapshot, TiffMutation>(STDIO_TIFF_DOCUMENT_SCHEMA, Dialect { artifact_kind: "s.stdio.tiff", standard: StandardId("6.0"), subset: SubsetId("*") })
        .document_codec_bare::<TiffSnapshot, TiffMutation>(STDIO_TIFF_DOCUMENT_SCHEMA, Dialect { artifact_kind: "s.stdio.tiff", standard: StandardId("6.0"), subset: SubsetId("baseline") })
        .try_build()
}

/// 🛡️ Re-derives the 🧱️baseline subset's `SubsetValidatorEntry` — see `declaration()`'s own doc for
/// why this calls `subset_validator_entry_of` directly instead of reusing the private cache in
/// `🧱️baseline/🚪️io`.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn declared_subset_validators() -> &'static [semio_framework_plugin::io::SubsetValidatorEntry] {
    static ENTRIES: std::sync::OnceLock<Vec<semio_framework_plugin::io::SubsetValidatorEntry>> = std::sync::OnceLock::new();
    ENTRIES.get_or_init(|| vec![semio_framework_plugin::io::subset_validator_entry_of::<standards::v6_0::subsets::baseline::io::TiffBaselineValidator>()]).as_slice()
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
                    id: "stdio.tiff",
                    extension: Some("tiff"),
                    role: semio_framework_dsl::LanguageRole::Document,
                    grammar: Some(standards::v6_0::subsets::document::io::text::snapshot::COMPONENT_GRAMMAR_SEMIO),
                    grammar_path: Some(standards::v6_0::subsets::document::io::text::snapshot::COMPONENT_GRAMMAR_PATH),
                    protocol: Some(standards::v6_0::subsets::document::io::binary::snapshot::COMPONENT_PROTOCOL_SEMIO),
                    protocol_path: Some(standards::v6_0::subsets::document::io::binary::snapshot::COMPONENT_PROTOCOL_PATH),
                    hooks: semio_framework_dsl::passthrough_hooks("stdio.tiff"),
                },
                semio_framework_dsl::LanguageSpec {
                    id: "stdio.tiff.op",
                    extension: None,
                    role: semio_framework_dsl::LanguageRole::Ops,
                    grammar: Some(standards::v6_0::subsets::document::io::text::mutations::COMPONENT_GRAMMAR_SEMIO),
                    grammar_path: Some(standards::v6_0::subsets::document::io::text::mutations::COMPONENT_GRAMMAR_PATH),
                    protocol: Some(standards::v6_0::subsets::document::io::binary::mutations::COMPONENT_PROTOCOL_SEMIO),
                    protocol_path: Some(standards::v6_0::subsets::document::io::binary::mutations::COMPONENT_PROTOCOL_PATH),
                    hooks: semio_framework_dsl::passthrough_hooks("stdio.tiff.op"),
                },
                semio_framework_dsl::LanguageSpec {
                    id: "stdio.tiff.diff",
                    extension: None,
                    role: semio_framework_dsl::LanguageRole::Diff,
                    grammar: Some(standards::v6_0::subsets::document::io::text::diff::COMPONENT_GRAMMAR_SEMIO),
                    grammar_path: Some(standards::v6_0::subsets::document::io::text::diff::COMPONENT_GRAMMAR_PATH),
                    protocol: None,
                    protocol_path: None,
                    hooks: semio_framework_dsl::passthrough_hooks("stdio.tiff.diff"),
                },
                semio_framework_dsl::LanguageSpec {
                    id: "stdio.tiff.pack",
                    extension: None,
                    role: semio_framework_dsl::LanguageRole::Pack,
                    grammar: None,
                    grammar_path: None,
                    protocol: Some(standards::v6_0::subsets::document::io::binary::snapshot::COMPONENT_PROTOCOL_SEMIO),
                    protocol_path: Some(standards::v6_0::subsets::document::io::binary::snapshot::COMPONENT_PROTOCOL_PATH),
                    hooks: semio_framework_dsl::passthrough_hooks("stdio.tiff.pack"),
                },
                semio_framework_dsl::LanguageSpec {
                    id: "stdio.tiff.spr",
                    extension: None,
                    role: semio_framework_dsl::LanguageRole::Spr,
                    grammar: None,
                    grammar_path: None,
                    protocol: Some(standards::v6_0::subsets::document::io::binary::mutations::COMPONENT_PROTOCOL_SEMIO),
                    protocol_path: Some(standards::v6_0::subsets::document::io::binary::mutations::COMPONENT_PROTOCOL_PATH),
                    hooks: semio_framework_dsl::passthrough_hooks("stdio.tiff.spr"),
                },
            ]
        })
        .as_slice()
}
//#endregion 🔖️Declaration

//#region 🚪️DerivedIoRegistry
pub use semio_s_artifact_stdio_tiff_mutation_regressions::io_registry;
//#endregion 🚪️DerivedIoRegistry

#[path = "."]
pub mod standards {
    #[path = "."]
    pub mod v6_0 {
        #[path = "."]
        pub mod subsets {
            #[path = "."]
            pub mod document {
                // 🐜️ `⚙️engine/` dissolved (ticket 26/08/12/ENGINELESS-ARTIFACTS-AND-APP-STATE-MACHINES):
                // real code now lives in `io` (codec/io_registry) and `schema` (document
                // helpers), both siblings within this same `any` module — this stays an
                // inline barrel so every existing `subsets::document::engine::*` path (reached
                // from the `v6_0::engine`/root `engine::*` barrels above it) still resolves.

                pub use semio_s_artifact_stdio_tiff_snapshot::standards::v6_0::subsets::document::examples;
                #[path = "."]
                pub mod schema {
                    pub use semio_s_artifact_stdio_tiff_schema::standards::v6_0::subsets::document::schema::*;

                    pub use semio_s_artifact_stdio_tiff_snapshot::standards::v6_0::subsets::document::schema::snapshot;
                    pub use semio_s_artifact_stdio_tiff_inferences::standards::v6_0::subsets::document::schema::inferences;
                    pub use semio_s_artifact_stdio_tiff_schema::standards::v6_0::subsets::document::schema::diff;
                    pub use semio_s_artifact_stdio_tiff_mutations::standards::v6_0::subsets::document::schema::mutations;

                    pub use semio_s_artifact_stdio_tiff_snapshot::standards::v6_0::subsets::document::schema::operations;
                }
                #[path = "."]
                pub mod io {
                    pub use semio_s_artifact_stdio_tiff_inferences::standards::v6_0::subsets::document::io::*;

                    pub use semio_s_artifact_stdio_tiff_mutation_regressions::standards::v6_0::subsets::document::io::facets;
                    #[path = "."]
                    pub mod binary {
                        pub use semio_s_artifact_stdio_tiff_snapshot::standards::v6_0::subsets::document::io::binary::snapshot;
                        pub use semio_s_artifact_stdio_tiff_mutations::standards::v6_0::subsets::document::io::binary::mutations;
                        pub use semio_s_artifact_stdio_tiff_schema::standards::v6_0::subsets::document::io::binary::diff;
                        pub use semio_s_artifact_stdio_tiff_snapshot::standards::v6_0::subsets::document::io::binary::inferences;
                    }
                    #[path = "."]
                    pub mod text {
                        pub use semio_s_artifact_stdio_tiff_snapshot::standards::v6_0::subsets::document::io::text::snapshot;
                        pub use semio_s_artifact_stdio_tiff_mutations::standards::v6_0::subsets::document::io::text::mutations;
                        pub use semio_s_artifact_stdio_tiff_schema::standards::v6_0::subsets::document::io::text::diff;
                        pub use semio_s_artifact_stdio_tiff_snapshot::standards::v6_0::subsets::document::io::text::inferences;
                    }
                    pub use semio_s_artifact_stdio_tiff_snapshot::standards::v6_0::subsets::document::io::sqlite;
                    pub use semio_s_artifact_stdio_tiff_inferences::standards::v6_0::subsets::document::io::import;
                    pub use semio_s_artifact_stdio_tiff_inferences::standards::v6_0::subsets::document::io::export;
                }
            }
            #[path = "."]
            pub mod baseline {
                // 🏅️ Baseline TIFF (6.0) -- Adobe TIFF 6.0 Part 1 "Baseline TIFF", the
                // honestly-scope-limited case: `TiffSnapshot`(6.0) retains only a decoded
                // `RasterImage{width,height,rgba}`, no IFD. Added in ticket
                // 26/08/11/ARTIFACT-STANDARD-SUBSETS-REAL-VOCABULARIES W3.
                pub use semio_s_artifact_stdio_tiff_mutation_regressions::standards::v6_0::subsets::baseline::io;
                pub use semio_s_artifact_stdio_tiff_schema_2::standards::v6_0::subsets::baseline::schema;
            
}
        }
    }
}

// ---- Crate-root module paths: the crate's public surface ----
pub mod schema {
    pub use super::standards::v6_0::subsets::document::schema::*;
}

pub use standards::v6_0::subsets::document::examples;

#[path = "."]
pub mod editor {
    #[path = "."]
    pub mod tiff_any {
        pub use semio_s_artifact_stdio_tiff_mutation_regressions::editor::tiff_any::*;

        pub use semio_s_artifact_stdio_tiff_snapshot::editor::tiff_any::edit_rules;
        pub use semio_s_artifact_stdio_tiff_snapshot_surfaces::editor::tiff_any::presence;
        pub use semio_s_artifact_stdio_tiff_snapshot_surfaces::editor::tiff_any::config;
        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod edit {
                pub use semio_s_artifact_stdio_tiff_inferences::editor::tiff_any::modes::edit::*;

                pub use semio_s_artifact_stdio_tiff_inferences::editor::tiff_any::modes::edit::windows;
            }
        }
    }
    #[path = "."]
    pub mod tiff_baseline {
        pub use semio_s_artifact_stdio_tiff_mutation_regressions::editor::tiff_baseline::*;

        pub use semio_s_artifact_stdio_tiff_snapshot_surfaces::editor::tiff_baseline::edit_rules;
        pub use semio_s_artifact_stdio_tiff_snapshot_surfaces::editor::tiff_baseline::presence;
        pub use semio_s_artifact_stdio_tiff_snapshot_surfaces::editor::tiff_baseline::config;
        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod edit {
                pub use semio_s_artifact_stdio_tiff_mutation_regressions::editor::tiff_baseline::modes::edit::*;

                pub use semio_s_artifact_stdio_tiff_mutation_regressions::editor::tiff_baseline::modes::edit::windows;
            }
        }
    }
}

#[path = "."]
pub mod viewer {
    #[path = "."]
    pub mod tiff_any {
        pub use semio_s_artifact_stdio_tiff_mutation_regressions::viewer::tiff_any::*;

        pub use semio_s_artifact_stdio_tiff_snapshot_surfaces::viewer::tiff_any::presence;
        pub use semio_s_artifact_stdio_tiff_schema::viewer::tiff_any::config;
        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod view {
                pub use semio_s_artifact_stdio_tiff_inferences::viewer::tiff_any::modes::view::*;

                pub use semio_s_artifact_stdio_tiff_inferences::viewer::tiff_any::modes::view::windows;
            }
        }
    }
    #[path = "."]
    pub mod tiff_baseline {
        pub use semio_s_artifact_stdio_tiff_mutation_regressions::viewer::tiff_baseline::*;

        pub use semio_s_artifact_stdio_tiff_schema::viewer::tiff_baseline::presence;
        pub use semio_s_artifact_stdio_tiff_schema::viewer::tiff_baseline::config;
        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod view {
                pub use semio_s_artifact_stdio_tiff_mutation_regressions::viewer::tiff_baseline::modes::view::*;

                pub use semio_s_artifact_stdio_tiff_mutation_regressions::viewer::tiff_baseline::modes::view::windows;
            }
        }
    }
}

pub use crate::standards::v6_0::subsets::baseline::io::{TiffBaselineBuilderConstruction, TiffBaselineAnalyzerAnalysis, TiffBaselineBuilderFacets, TiffBaselineBuilder, TiffBaselineAnalyzer, TiffBaselineComposer};

pub use crate::standards::v6_0::subsets::document::io::facets::{TiffBuilderConstruction, TiffParts, TiffAnalyzerAnalysis, TiffBuilderFacets, TiffBuilder, TiffAnalyzer, TiffComposer};

#[cfg(test)]
#[path = "🏅️standards/🔖️6.0/🪆️subsets/🧾️document/🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🦀️.rs"]
mod hoisted_snapshot_0;
