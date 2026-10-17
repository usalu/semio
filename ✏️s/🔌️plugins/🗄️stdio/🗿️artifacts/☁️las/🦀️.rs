#![allow(async_fn_in_trait)]
#![allow(long_running_const_eval)]
//! 🎪 `stdio.las` artifact — stdio reference format.
use semio_framework_plugin::PluginBuilder;
#[cfg(test)] use semio_framework_artifact_reference::{Dialect,StandardId,SubsetId};

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_schema as framework_schema;
extern crate semio_framework_value_derive as value_derive;

pub use semio_s_artifact_stdio_contract::{apply_mutation, apply_mutation_checked, mutation_from_payload_json, mutation_inverse, mutation_payload_json, MutationRefusal};

pub use schema::diff::LasDiff;
pub use schema::mutations::LasMutation;
pub use schema::snapshot::LasSnapshot;
pub use schema::LasArtifact;

pub use semio_s_artifact_stdio_las_snapshot::{STDIO_LAS_DOCUMENT_SCHEMA};

pub use semio_s_artifact_stdio_las_snapshot::{LAS_ARTIFACT_SCHEMA_ID};

pub use semio_s_artifact_stdio_las_snapshot::{ARTIFACT_DEFINITION_SCHEMA};

pub use semio_s_artifact_stdio_las_mutations::{definition};

pub use semio_s_artifact_stdio_las_snapshot::{formats};

pub use semio_s_artifact_stdio_las_mutations::{native_codec};

pub use semio_s_artifact_stdio_las_mutations::{native_codecs};

pub use semio_s_artifact_stdio_las_tests_2::{contribution};

pub use semio_s_artifact_stdio_las_tests_2::{assembly, declaration, pilot_languages};

pub use semio_s_artifact_stdio_las_snapshot::{artifact_kind};
//#region 🚪️DerivedIoRegistry
pub use semio_s_artifact_stdio_las_tests_2::io_registry;
//#endregion 🚪️DerivedIoRegistry

#[path = "."]
pub mod standards {
    #[path = "."]
    pub mod v1_0 {
        // ⚙️→🚪️/🧬️ dissolved (ticket 26/08/12/ENGINELESS-ARTIFACTS-AND-APP-STATE-MACHINES):
        // real code now lives in `subsets::any::{io,schema}`; this stays an inline barrel
        // so every existing `standards::v1_0::engine::*`/root `engine::*` path still resolves.
        pub use semio_s_artifact_stdio_las_tests_2::standards::v1_0::engine;
        #[path = "."]
        pub mod subsets {
            #[path = "."]
            pub mod any {
                #[path = "."]
                pub mod schema {
                    pub use semio_s_artifact_stdio_las_schema::standards::v1_0::subsets::any::schema::*;

                    pub use semio_s_artifact_stdio_las_snapshot::standards::v1_0::subsets::any::schema::snapshot;
                    pub use semio_s_artifact_stdio_las_snapshot::standards::v1_0::subsets::any::schema::diff;
                    pub use semio_s_artifact_stdio_las_schema::standards::v1_0::subsets::any::schema::inferences;
                    pub use semio_s_artifact_stdio_las_mutations::standards::v1_0::subsets::any::schema::mutations;
                }
                #[path = "."]
                pub mod io {
                    pub use semio_s_artifact_stdio_las_tests_2::standards::v1_0::subsets::any::io::*;
pub mod binary {
    pub use semio_s_artifact_stdio_las_tests_2::standards::v1_0::subsets::any::io::binary::*;
    pub use semio_s_artifact_stdio_las_snapshot::standards::v1_0::subsets::any::io::binary::inferences;
}
pub mod sqlite {
    pub use semio_s_artifact_stdio_las_tests::standards::v1_0::subsets::any::io::sqlite::*;
    pub use semio_s_artifact_stdio_las_snapshot::standards::v1_0::subsets::any::io::sqlite::snapshot;
}
pub mod text {
    pub use semio_s_artifact_stdio_las_tests_2::standards::v1_0::subsets::any::io::text::*;
    pub use semio_s_artifact_stdio_las_snapshot::standards::v1_0::subsets::any::io::text::inferences;
}

                    pub use semio_s_artifact_stdio_las_tests_2::standards::v1_0::subsets::any::io::import;
                    pub use semio_s_artifact_stdio_las_tests_2::standards::v1_0::subsets::any::io::export;
                }
            }
        }
    }
}

// ---- Shims: keep pre-migration module paths resolving for external callers ----
pub mod schema {
    pub use super::standards::v1_0::subsets::any::schema::*;
}
pub mod engine {
    pub use super::standards::v1_0::engine::*;
}

pub use semio_s_artifact_stdio_las_tests::examples;

#[path = "."]
pub mod editor {
    #[path = "."]
    pub mod las {
        pub use semio_s_artifact_stdio_las_mutations::editor::las::*;

        pub use semio_s_artifact_stdio_las_tests::editor::las::edit_rules;
        pub use semio_s_artifact_stdio_las_tests::editor::las::presence;
        pub use semio_s_artifact_stdio_las_tests::editor::las::config;
        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod edit {
                pub use semio_s_artifact_stdio_las_mutations::editor::las::modes::edit::*;

                pub use semio_s_artifact_stdio_las_snapshot::editor::las::modes::edit::windows;
            }
        }
    }
}

#[path = "."]
pub mod viewer {
    #[path = "."]
    pub mod las {
        pub use semio_s_artifact_stdio_las_mutations::viewer::las::*;

        pub use semio_s_artifact_stdio_las_tests::viewer::las::presence;
        pub use semio_s_artifact_stdio_las_tests::viewer::las::config;
        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod view {
                pub use semio_s_artifact_stdio_las_mutations::viewer::las::modes::view::*;

                pub use semio_s_artifact_stdio_las_schema::viewer::las::modes::view::windows;
            }
        }
    }
}

pub use crate::standards::v1_0::subsets::any::io::{LasBuilderConstruction, LasParts, LasAnalyzerAnalysis, LasBuilderFacets, LasBuilder, LasAnalyzer, LasComposer};

#[cfg(test)]
#[path = "🏅️standards/🔖️1.0/🪆️subsets/🎩️header/🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🦀️.rs"]
mod hoisted_snapshot_0;
