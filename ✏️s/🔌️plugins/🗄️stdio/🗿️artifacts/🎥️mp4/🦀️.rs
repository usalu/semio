//! 🎪 `stdio.mp4` artifact — new-format artifact (master plan "New format artifacts" table).

#![allow(async_fn_in_trait)]
#![allow(long_running_const_eval)]

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_schema as framework_schema;
extern crate semio_framework_value_derive as value_derive;

pub use semio_s_artifact_stdio_contract::{apply_mutation, apply_mutation_checked, mutation_from_payload_json, mutation_inverse, mutation_payload_json, MutationRefusal};

pub use standards::isobmff::subsets::any::schema::diff::Mp4Diff;
pub use standards::isobmff::subsets::any::schema::mutations::Mp4Mutation;
pub use standards::isobmff::subsets::any::schema::snapshot::Mp4Snapshot;
pub use standards::isobmff::subsets::any::schema::Mp4Artifact;

pub use semio_s_artifact_stdio_mp4_tests::{STDIO_MP4_DOCUMENT_SCHEMA};

pub use semio_s_artifact_stdio_mp4_boxes::{MP4_ARTIFACT_SCHEMA_ID};

pub use semio_s_artifact_stdio_mp4_boxes::{ARTIFACT_DEFINITION_SCHEMA};

pub use semio_s_artifact_stdio_mp4_tests::{definition};

pub use semio_s_artifact_stdio_mp4_boxes::{formats};

pub use semio_s_artifact_stdio_mp4_tests::{native_codec};

pub use semio_s_artifact_stdio_mp4_tests::{native_codecs};

pub use semio_s_artifact_stdio_mp4_tests::{contribution};

pub use semio_s_artifact_stdio_mp4_tests::{MP4_DIALECT};

pub use semio_s_artifact_stdio_mp4_tests::{artifact_kind};
pub use semio_s_artifact_stdio_mp4_tests::{assembly, declaration};
//#region 🚪️DerivedIoRegistry
pub use semio_s_artifact_stdio_mp4_tests::io_registry;
//#endregion 🚪️DerivedIoRegistry

#[path = "."]
pub mod standards {
    #[path = "."]
    pub mod isobmff {
        #[path = "."]
        pub mod subsets {
            #[path = "."]
            pub mod any {
                pub mod io {
    pub use semio_s_artifact_stdio_mp4_tests::standards::isobmff::subsets::any::io::*;
    pub mod binary {
        pub use semio_s_artifact_stdio_mp4_boxes::standards::isobmff::subsets::any::io::binary::*;
        pub use semio_s_artifact_stdio_mp4_schema::standards::isobmff::subsets::any::io::binary::diff;
        pub use semio_s_artifact_stdio_mp4_inferences::standards::isobmff::subsets::any::io::binary::inferences;
        pub use semio_s_artifact_stdio_mp4_mutations::standards::isobmff::subsets::any::io::binary::mutations;
        pub use semio_s_artifact_stdio_mp4_inferences::standards::isobmff::subsets::any::io::binary::snapshot;
    }
    pub mod sqlite {
        pub use semio_s_artifact_stdio_mp4_boxes::standards::isobmff::subsets::any::io::sqlite::*;
        pub use semio_s_artifact_stdio_mp4_inferences::standards::isobmff::subsets::any::io::sqlite::snapshot;
    }
}
                #[path = "."]
                pub mod schema {
                    pub use semio_s_artifact_stdio_mp4_schema::standards::isobmff::subsets::any::schema::*;
                    
                    pub use semio_s_artifact_stdio_mp4_inferences::standards::isobmff::subsets::any::schema::snapshot;
                    pub use semio_s_artifact_stdio_mp4_inferences::standards::isobmff::subsets::any::schema::inferences;
                    pub use semio_s_artifact_stdio_mp4_schema::standards::isobmff::subsets::any::schema::diff;
                    pub use semio_s_artifact_stdio_mp4_mutations::standards::isobmff::subsets::any::schema::mutations;
                }
            }
        }
    }
}
pub use semio_s_artifact_stdio_mp4_inferences::examples;

#[path = "."]
pub mod editor {
    #[path = "."]
    pub mod mp4 {
        pub use semio_s_artifact_stdio_mp4_tests::editor::mp4::*;
        
        pub use semio_s_artifact_stdio_mp4_inferences::editor::mp4::edit_rules;
        pub use semio_s_artifact_stdio_mp4_inferences::editor::mp4::presence;
        pub use semio_s_artifact_stdio_mp4_inferences::editor::mp4::config;
        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod edit {
                pub use semio_s_artifact_stdio_mp4_mutations::editor::mp4::modes::edit::*;
                
                pub use semio_s_artifact_stdio_mp4_mutations::editor::mp4::modes::edit::windows;
            }
        }
    }
}

#[path = "."]
pub mod viewer {
    #[path = "."]
    pub mod mp4 {
        pub use semio_s_artifact_stdio_mp4_viewer::viewer::mp4::*;
        
        pub use semio_s_artifact_stdio_mp4_inferences::viewer::mp4::presence;
        pub use semio_s_artifact_stdio_mp4_inferences::viewer::mp4::config;
        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod view {
                pub use semio_s_artifact_stdio_mp4_mutations::viewer::mp4::modes::view::*;
                
                pub use semio_s_artifact_stdio_mp4_mutations::viewer::mp4::modes::view::windows;
            }
        }
    }
}

pub use crate::standards::isobmff::subsets::any::io::{Mp4BuilderConstruction, Mp4Parts, Mp4AnalyzerAnalysis, Mp4BuilderFacets, Mp4Builder, Mp4Analyzer, Mp4Composer};

#[cfg(test)]
#[path = "🏅️standards/🔖️isobmff/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🦀️.rs"]
mod hoisted_snapshot_0;
