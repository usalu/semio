//! 🎪 `stdio.zip` artifact — stdio reference format.

#![allow(async_fn_in_trait)]
#![allow(long_running_const_eval)]

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_schema as framework_schema;
extern crate semio_framework_value_derive as value_derive;

pub use semio_s_artifact_stdio_contract::{apply_mutation, apply_mutation_checked, mutation_from_payload_json, mutation_inverse, mutation_payload_json, MutationRefusal};

pub use schema::diff::ZipDiff;
pub use schema::mutations::ZipMutation;
pub use schema::snapshot::ZipSnapshot;
pub use schema::ZipArtifact;
/// 🧬️ The wire contract a native test host decodes `🥒️.feature` witnesses through and inverts them with
/// (`Mutation::from_payload_value`/`Mutation::inverse`), re-exported because such a host links this crate alone.

pub use semio_s_artifact_stdio_zip_inferences::{STDIO_ZIP_DOCUMENT_SCHEMA};

pub use semio_s_artifact_stdio_zip_inferences::{ZIP_ARTIFACT_SCHEMA_ID};

pub use semio_s_artifact_stdio_zip_inferences::{ARTIFACT_DEFINITION_SCHEMA};

pub use semio_s_artifact_stdio_zip_schema::{definition};

pub use semio_s_artifact_stdio_zip_inferences::{formats};

pub use semio_s_artifact_stdio_zip_schema::{native_codec};

pub use semio_s_artifact_stdio_zip_schema::{native_codecs};

pub use semio_s_artifact_stdio_zip_editor::{contribution};

pub use semio_s_artifact_stdio_zip_editor::{assembly, declaration, pilot_languages, zip_subset_validators};

pub use semio_s_artifact_stdio_zip_inferences::{artifact_kind};
//#region 🚪️DerivedIoRegistry
pub use semio_s_artifact_stdio_zip_opc::io_registry;
//#endregion 🚪️DerivedIoRegistry

#[path = "."]
pub mod standards {
    #[path = "."]
    pub mod v2_0 {
        // 🐜️ `⚙️engine/` dissolved (ticket 26/08/12/ENGINELESS-ARTIFACTS-AND-APP-STATE-MACHINES):
        // CRC32 + CP437 + extra-fields + EOCD parsing + `decode_zip`/`encode_zip`/
        // `sniff_zip_bytes`/`SniffConfidence`/`ZipError` all moved into `subsets::any::io`
        // (rule 2: this is the codec); `empty_zip_snapshot`/`demo_zip_snapshot` moved to
        // `subsets::any::schema`; `ZipEngine` (zero construction sites repo-wide) deleted
        // outright; tests moved into `subsets::any::io` (codec tests) and
        // `subsets::any::schema::inferences` (conformance laws). `zip` is NOT one of stdio's
        // 10 protected imperative plugin-root calls (it already used the declarative
        // `ArtifactDeclaration` builder) — every call site (incl. `📜️docx`/`📕️xlsx`/`🎞️pptx`'s
        // own zip-wrap export paths and `💬️bcf`'s zip-container sniff) was repointed directly
        // at the new locations, and no `engine` shim survives here since nothing references
        // it anymore.
        #[path = "."]
        pub mod subsets {
            #[path = "."]
            pub mod base {
                #[path = "."]
                pub mod schema {
                    pub use semio_s_artifact_stdio_zip_schema_2::standards::v2_0::subsets::base::schema::*;

                    pub use semio_s_artifact_stdio_zip_inferences::standards::v2_0::subsets::base::schema::snapshot;
                    pub use semio_s_artifact_stdio_zip_inferences::standards::v2_0::subsets::base::schema::inferences;
                    pub use semio_s_artifact_stdio_zip_mutations::standards::v2_0::subsets::base::schema::diff;
                    pub use semio_s_artifact_stdio_zip_mutations::standards::v2_0::subsets::base::schema::mutations;
                }
                #[path = "."]
                pub mod io {
                    pub use semio_s_artifact_stdio_zip_schema::standards::v2_0::subsets::base::io::*;
pub mod binary {
    pub use semio_s_artifact_stdio_zip_schema::standards::v2_0::subsets::base::io::binary::*;
    pub use semio_s_artifact_stdio_zip_inferences::standards::v2_0::subsets::base::io::binary::inferences;
}
pub mod sqlite {
    pub use semio_s_artifact_stdio_zip_schema::standards::v2_0::subsets::base::io::sqlite::*;
    pub use semio_s_artifact_stdio_zip_inferences::standards::v2_0::subsets::base::io::sqlite::snapshot;
}
pub mod text {
    pub use semio_s_artifact_stdio_zip_schema::standards::v2_0::subsets::base::io::text::*;
    pub use semio_s_artifact_stdio_zip_inferences::standards::v2_0::subsets::base::io::text::inferences;
}

                    #[path = "."]
                    pub mod import {
                        #[path = "."]
                        pub mod deserializers {
                            #[path = "."]
                            pub mod artifacts {
                                pub use semio_s_artifact_stdio_zip_schema::standards::v2_0::subsets::base::io::import::deserializers::artifacts::binary;
                                pub use semio_s_artifact_stdio_zip_opc::standards::v2_0::subsets::base::io::import::deserializers::artifacts::deflate;
                            }
                        }
                    }
                    #[path = "."]
                    pub mod export {
                        #[path = "."]
                        pub mod serializers {
                            #[path = "."]
                            pub mod artifacts {
                                pub use semio_s_artifact_stdio_zip_opc::standards::v2_0::subsets::base::io::export::serializers::artifacts::binary;
                                pub use semio_s_artifact_stdio_zip_opc::standards::v2_0::subsets::base::io::export::serializers::artifacts::deflate;
                            }
                        }
                    }
                }
            }
            #[path = "."]
            pub mod iso21320 {
                pub use semio_s_artifact_stdio_zip_schema::standards::v2_0::subsets::iso21320::schema;
                pub use semio_s_artifact_stdio_zip_opc::standards::v2_0::subsets::iso21320::io;
            }
        }
    }
}

// ---- Shims: keep pre-migration module paths resolving for external callers ----
pub mod schema {
    pub use super::standards::v2_0::subsets::base::schema::*;
}

/// 📦️ Shared OPC (Open Packaging Conventions) layer — zip-and-XML container plumbing
/// that `docx`/`xlsx`/`pptx` import cross-artifact (`crate::opc::*`).
pub use semio_s_artifact_stdio_zip_opc::opc;

pub use semio_s_artifact_stdio_zip_schema::examples;

#[path = "."]
pub mod editor {
    pub(crate) use semio_s_artifact_stdio_zip_editor::editor::editing;
    #[path = "."]
    pub mod zip {
        #[path = "."]
        pub mod base {
            pub use semio_s_artifact_stdio_zip_editor::editor::zip::base::*;

            pub use semio_s_artifact_stdio_zip_schema::editor::zip::base::edit_rules;
            pub use semio_s_artifact_stdio_zip_schema::editor::zip::base::presence;
            pub use semio_s_artifact_stdio_zip_schema::editor::zip::base::config;
            #[path = "."]
            pub mod modes {
                #[path = "."]
                pub mod edit {
                    pub use semio_s_artifact_stdio_zip_editor::editor::zip::base::modes::edit::*;

                    pub use semio_s_artifact_stdio_zip_editor::editor::zip::base::modes::edit::windows;
                }
            }
        }
        #[path = "."]
        pub mod iso21320 {
            pub use semio_s_artifact_stdio_zip_editor::editor::zip::iso21320::*;

            pub use semio_s_artifact_stdio_zip_schema::editor::zip::iso21320::edit_rules;
            pub use semio_s_artifact_stdio_zip_schema::editor::zip::iso21320::presence;
            pub use semio_s_artifact_stdio_zip_schema::editor::zip::iso21320::config;
            #[path = "."]
            pub mod modes {
                #[path = "."]
                pub mod edit {
                    pub use semio_s_artifact_stdio_zip_editor::editor::zip::iso21320::modes::edit::*;

                    pub use semio_s_artifact_stdio_zip_editor::editor::zip::iso21320::modes::edit::windows;
                }
            }
        }
    }
}

#[path = "."]
pub mod viewer {
    #[path = "."]
    pub mod zip {
        #[path = "."]
        pub mod base {
            pub use semio_s_artifact_stdio_zip_schema::viewer::zip::base::*;

            pub use semio_s_artifact_stdio_zip_schema::viewer::zip::base::presence;
            pub use semio_s_artifact_stdio_zip_schema::viewer::zip::base::config;
            #[path = "."]
            pub mod modes {
                #[path = "."]
                pub mod view {
                    pub use semio_s_artifact_stdio_zip_inferences::viewer::zip::base::modes::view::*;

                    pub use semio_s_artifact_stdio_zip_inferences::viewer::zip::base::modes::view::windows;
                }
            }
        }
        #[path = "."]
        pub mod iso21320 {
            pub use semio_s_artifact_stdio_zip_schema::viewer::zip::iso21320::*;

            pub use semio_s_artifact_stdio_zip_schema::viewer::zip::iso21320::presence;
            pub use semio_s_artifact_stdio_zip_schema::viewer::zip::iso21320::config;
            #[path = "."]
            pub mod modes {
                #[path = "."]
                pub mod view {
                    pub use semio_s_artifact_stdio_zip_inferences::viewer::zip::iso21320::modes::view::*;

                    pub use semio_s_artifact_stdio_zip_inferences::viewer::zip::iso21320::modes::view::windows;
                }
            }
        }
    }
}

pub use crate::standards::v2_0::subsets::iso21320::io::{ZipIso21320BuilderConstruction, ZipIso21320AnalyzerAnalysis, ZipIso21320BuilderFacets, ZipIso21320Builder, ZipIso21320Analyzer, ZipIso21320Composer};

pub use crate::standards::v2_0::subsets::base::io::{ZipBuilderConstruction, ZipParts, ZipAnalyzerAnalysis, ZipBuilderFacets, ZipBuilder, ZipAnalyzer, ZipComposer};

#[cfg(test)]
#[path = "🏅️standards/🔖️2.0/🪆️subsets/🧱️base/🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🦀️.rs"]
mod hoisted_snapshot_0;
