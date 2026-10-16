//! 🎪 `stdio.bmp` artifact — stdio reference format.

#![allow(async_fn_in_trait)]
#![allow(long_running_const_eval)]

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_schema as framework_schema;
extern crate semio_framework_value_derive as value_derive;

pub use semio_s_artifact_stdio_contract::{apply_mutation, apply_mutation_checked, mutation_from_payload_json, mutation_inverse, mutation_payload_json, MutationRefusal};

pub use schema::diff::BmpDiff;
pub use schema::mutations::BmpMutation;
pub use schema::snapshot::BmpSnapshot;
pub use schema::BmpArtifact;
/// 🧾️ The DSL contract `BmpSnapshot` implements, re-exported for clients that print or parse it.
pub use pack::snapshot::ArtifactDsl;

pub use semio_s_artifact_stdio_bmp_snapshot::{STDIO_BMP_DOCUMENT_SCHEMA};

pub use semio_s_artifact_stdio_bmp_snapshot::{BMP_ARTIFACT_SCHEMA_ID};

pub use semio_s_artifact_stdio_bmp_editor_2::{ARTIFACT_DEFINITION_SCHEMA};

pub use semio_s_artifact_stdio_bmp_editor_2::{definition};

pub use semio_s_artifact_stdio_bmp_editor_2::{formats};

pub use semio_s_artifact_stdio_bmp_snapshot::{native_codecs};

pub use semio_s_artifact_stdio_bmp_editor_2::{contribution};

pub use semio_s_artifact_stdio_bmp_editor_2::{BMP_DIALECT};

pub use semio_s_artifact_stdio_bmp_editor_2::{artifact_kind, assembly, declaration};
//#region 🚪️DerivedIoRegistry
pub use semio_s_artifact_stdio_bmp_mutations::io_registry;
//#endregion 🚪️DerivedIoRegistry

#[path = "."]
pub mod standards {
    #[path = "."]
    pub mod v_v3 {
        // 🐜️ `⚙️engine/` dissolved (ticket 26/08/12/ENGINELESS-ARTIFACTS-AND-APP-STATE-MACHINES):
        // real code now lives in `subsets::any::io` (codec/register/io_registry —
        // `register` stays reachable since bmp is one of stdio's 10 deliberate imperative
        // `engine::register()` plugin-root calls) and `subsets::any::schema` (document
        // helpers); this stays an inline barrel so every existing
        // `standards::v_v3::engine::*`/root `engine::*` path still resolves.

        #[path = "."]
        pub mod subsets {
            #[path = "."]
            pub mod any {
                #[path = "."]
                pub mod schema {
                    pub use semio_s_artifact_stdio_bmp_schema::standards::v_v3::subsets::any::schema::*;

                    pub use semio_s_artifact_stdio_bmp_snapshot::standards::v_v3::subsets::any::schema::snapshot;
                    pub use semio_s_artifact_stdio_bmp_inferences::standards::v_v3::subsets::any::schema::inferences;
                    pub use semio_s_artifact_stdio_bmp_inferences::standards::v_v3::subsets::any::schema::diff;
                    pub use semio_s_artifact_stdio_bmp_mutations::standards::v_v3::subsets::any::schema::mutations;

                    pub use semio_s_artifact_stdio_bmp_inferences::standards::v_v3::subsets::any::schema::operations;
                }
                #[path = "."]
                pub mod io {
                    pub use semio_s_artifact_stdio_bmp_mutations::standards::v_v3::subsets::any::io::*;
pub mod binary {
    pub use semio_s_artifact_stdio_bmp_mutations::standards::v_v3::subsets::any::io::binary::*;
    pub use semio_s_artifact_stdio_bmp_inferences::standards::v_v3::subsets::any::io::binary::diff;
    pub use semio_s_artifact_stdio_bmp_schema::standards::v_v3::subsets::any::io::binary::inferences;
}

                    pub use semio_s_artifact_stdio_bmp_mutation_regressions::standards::v_v3::subsets::any::io::import;
                    pub use semio_s_artifact_stdio_bmp_mutation_regressions::standards::v_v3::subsets::any::io::export;
                }
            }
        }
    }
}

// ---- Shims: keep pre-migration module paths resolving for external callers ----
pub mod schema {
    pub use super::standards::v_v3::subsets::any::schema::*;
}

pub use semio_s_artifact_stdio_bmp_inferences::examples;

#[path = "."]
pub mod editor {
    #[path = "."]
    pub mod bmp {
        pub use semio_s_artifact_stdio_bmp_editor_2::editor::bmp::*;

        pub use semio_s_artifact_stdio_bmp_inferences::editor::bmp::edit_rules;
        pub use semio_s_artifact_stdio_bmp_inferences::editor::bmp::presence;
        pub use semio_s_artifact_stdio_bmp_inferences::editor::bmp::config;
        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod edit {
                pub use semio_s_artifact_stdio_bmp_mutation_regressions::editor::bmp::modes::edit::*;

                pub use semio_s_artifact_stdio_bmp_mutation_regressions::editor::bmp::modes::edit::windows;
            }
        }
    }
}

#[path = "."]
pub mod viewer {
    #[path = "."]
    pub mod bmp {
        pub use semio_s_artifact_stdio_bmp_editor_2::viewer::bmp::*;

        pub use semio_s_artifact_stdio_bmp_inferences::viewer::bmp::presence;
        pub use semio_s_artifact_stdio_bmp_inferences::viewer::bmp::config;
        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod view {
                pub use semio_s_artifact_stdio_bmp_mutation_regressions::viewer::bmp::modes::view::*;

                pub use semio_s_artifact_stdio_bmp_mutation_regressions::viewer::bmp::modes::view::windows;
            }
        }
    }
}

pub use crate::standards::v_v3::subsets::any::io::{BmpBuilderConstruction, BmpParts, BmpAnalyzerAnalysis, BmpBuilderFacets, BmpBuilder, BmpAnalyzer, BmpComposer};

#[cfg(test)]
#[path = "🏅️standards/🔖️v3/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🦀️.rs"]
mod hoisted_snapshot_0;
