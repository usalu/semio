//! 🎪 `stdio.avi` artifact — new-format artifact (master plan "New format artifacts" table).

#![allow(async_fn_in_trait)]
#![allow(long_running_const_eval)]

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_schema as framework_schema;
extern crate semio_framework_value_derive as value_derive;

pub use semio_s_artifact_stdio_contract::{apply_mutation, apply_mutation_checked, mutation_from_payload_json, mutation_inverse, mutation_payload_json, MutationRefusal};

pub use standards::v1_0::subsets::any::schema::diff::AviDiff;
pub use standards::v1_0::subsets::any::schema::mutations::AviMutation;
pub use standards::v1_0::subsets::any::schema::snapshot::AviSnapshot;
pub use standards::v1_0::subsets::any::schema::AviArtifact;

pub use semio_s_artifact_stdio_avi_tests::{STDIO_AVI_DOCUMENT_SCHEMA};

pub use semio_s_artifact_stdio_avi_schema::{AVI_ARTIFACT_SCHEMA_ID};

pub use semio_s_artifact_stdio_avi_schema::{ARTIFACT_DEFINITION_SCHEMA};

pub use semio_s_artifact_stdio_avi_tests::{definition};

pub use semio_s_artifact_stdio_avi_schema::{formats};

pub use semio_s_artifact_stdio_avi_tests::{native_codec};

pub use semio_s_artifact_stdio_avi_tests::{native_codecs};

pub use semio_s_artifact_stdio_avi_tests::{contribution};

pub use semio_s_artifact_stdio_avi_tests::{AVI_DIALECT};

pub use semio_s_artifact_stdio_avi_tests::{artifact_kind};
pub use semio_s_artifact_stdio_avi_tests::{assembly, declaration};
//#region 🚪️DerivedIoRegistry
pub use semio_s_artifact_stdio_avi_tests::io_registry;
//#endregion 🚪️DerivedIoRegistry

#[path = "."]
pub mod standards {
    #[path = "."]
    pub mod v1_0 {
        #[path = "."]
        pub mod subsets {
            #[path = "."]
            pub mod any {
                pub mod io {
    pub use semio_s_artifact_stdio_avi_tests::standards::v1_0::subsets::any::io::*;
    pub mod binary {
        pub use semio_s_artifact_stdio_avi_schema::standards::v1_0::subsets::any::io::binary::*;
        pub use semio_s_artifact_stdio_avi_snapshot::standards::v1_0::subsets::any::io::binary::diff;
        pub use semio_s_artifact_stdio_avi_snapshot::standards::v1_0::subsets::any::io::binary::inferences;
        pub use semio_s_artifact_stdio_avi_mutations::standards::v1_0::subsets::any::io::binary::mutations;
        pub use semio_s_artifact_stdio_avi_snapshot::standards::v1_0::subsets::any::io::binary::snapshot;
    }
    pub mod sqlite {
        pub use semio_s_artifact_stdio_avi_schema::standards::v1_0::subsets::any::io::sqlite::*;
        pub use semio_s_artifact_stdio_avi_snapshot::standards::v1_0::subsets::any::io::sqlite::snapshot;
    }
    pub mod text {
        pub use semio_s_artifact_stdio_avi_schema::standards::v1_0::subsets::any::io::text::*;
        pub use semio_s_artifact_stdio_avi_snapshot::standards::v1_0::subsets::any::io::text::diff;
        pub use semio_s_artifact_stdio_avi_snapshot::standards::v1_0::subsets::any::io::text::inferences;
        pub use semio_s_artifact_stdio_avi_mutations::standards::v1_0::subsets::any::io::text::mutations;
        pub use semio_s_artifact_stdio_avi_snapshot::standards::v1_0::subsets::any::io::text::snapshot;
    }
}
                #[path = "."]
                pub mod schema {
                    pub use semio_s_artifact_stdio_avi_schema::standards::v1_0::subsets::any::schema::*;
                    
                    pub use semio_s_artifact_stdio_avi_snapshot::standards::v1_0::subsets::any::schema::snapshot;
                    pub use semio_s_artifact_stdio_avi_mutations::standards::v1_0::subsets::any::schema::inferences;
                    pub use semio_s_artifact_stdio_avi_snapshot::standards::v1_0::subsets::any::schema::diff;
                    pub use semio_s_artifact_stdio_avi_mutations::standards::v1_0::subsets::any::schema::mutations;
                }
            }
        }
    }
}
pub use semio_s_artifact_stdio_avi_snapshot::examples;

#[path = "."]
pub mod editor {
    #[path = "."]
    pub mod avi {
        pub use semio_s_artifact_stdio_avi_tests::editor::avi::*;
        
        pub use semio_s_artifact_stdio_avi_snapshot::editor::avi::edit_rules;
        pub use semio_s_artifact_stdio_avi_snapshot::editor::avi::presence;
        pub use semio_s_artifact_stdio_avi_snapshot::editor::avi::config;
        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod edit {
                pub use semio_s_artifact_stdio_avi_tests::editor::avi::modes::edit::*;
                
                pub use semio_s_artifact_stdio_avi_mutations::editor::avi::modes::edit::windows;
            }
        }
    }
}

#[path = "."]
pub mod viewer {
    #[path = "."]
    pub mod avi {
        pub use semio_s_artifact_stdio_avi_tests::viewer::avi::*;
        
        pub use semio_s_artifact_stdio_avi_snapshot::viewer::avi::presence;
        pub use semio_s_artifact_stdio_avi_snapshot::viewer::avi::config;
        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod view {
                pub use semio_s_artifact_stdio_avi_tests::viewer::avi::modes::view::*;
                
                pub use semio_s_artifact_stdio_avi_mutations::viewer::avi::modes::view::windows;
            }
        }
    }
}

pub use crate::standards::v1_0::subsets::any::io::{AviBuilderConstruction, AviParts, AviAnalyzerAnalysis, AviBuilderFacets, AviBuilder, AviAnalyzer, AviComposer};

#[cfg(test)]
#[path = "🏅️standards/🔖️1.0/🪆️subsets/🎛️hdrl/🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🦀️.rs"]
mod hoisted_snapshot_0;
