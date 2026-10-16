//! 🎪 `stdio.mp3` artifact — new-format artifact (master plan "New format artifacts" table).

#![allow(async_fn_in_trait)]
#![allow(long_running_const_eval)]

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_schema as framework_schema;
extern crate semio_framework_value_derive as value_derive;

pub use semio_s_artifact_stdio_contract::{apply_mutation, apply_mutation_checked, mutation_from_payload_json, mutation_inverse, mutation_payload_json, MutationRefusal};

pub use standards::mpeg1_layer3::subsets::any::schema::diff::Mp3Diff;
pub use standards::mpeg1_layer3::subsets::any::schema::mutations::Mp3Mutation;
pub use standards::mpeg1_layer3::subsets::any::schema::snapshot::{Id3Content,Id3Frame,Id3v1Tag,Id3v2Tag,Mp3Snapshot};
pub use standards::mpeg1_layer3::subsets::any::schema::Mp3Artifact;

pub use semio_s_artifact_stdio_mp3_schema::{STDIO_MP3_DOCUMENT_SCHEMA};

pub use semio_s_artifact_stdio_mp3_schema::{MP3_ARTIFACT_SCHEMA_ID};

pub use semio_s_artifact_stdio_mp3_schema::{ARTIFACT_DEFINITION_SCHEMA};

pub use semio_s_artifact_stdio_mp3_schema::{definition};

pub use semio_s_artifact_stdio_mp3_schema::{formats};

pub use semio_s_artifact_stdio_mp3_schema::{native_codec};

pub use semio_s_artifact_stdio_mp3_schema::{native_codecs};

pub use semio_s_artifact_stdio_mp3_inferences::{contribution};

pub use semio_s_artifact_stdio_mp3_inferences::{MP3_DIALECT};

pub use semio_s_artifact_stdio_mp3_schema::{artifact_kind};
pub use semio_s_artifact_stdio_mp3_inferences::{assembly, declaration};
//#region 🚪️DerivedIoRegistry
pub use semio_s_artifact_stdio_mp3_inferences::io_registry;
//#endregion 🚪️DerivedIoRegistry

#[path = "."]
pub mod standards {
    #[path = "."]
    pub mod mpeg1_layer3 {
        #[path = "."]
        pub mod subsets {
            #[path = "."]
            pub mod any {
                pub mod io {
    pub use semio_s_artifact_stdio_mp3_inferences::standards::mpeg1_layer3::subsets::any::io::*;
    pub mod binary {
        pub use semio_s_artifact_stdio_mp3_schema::standards::mpeg1_layer3::subsets::any::io::binary::*;
        pub use semio_s_artifact_stdio_mp3_mutations::standards::mpeg1_layer3::subsets::any::io::binary::diff;
        pub use semio_s_artifact_stdio_mp3_metadata::standards::mpeg1_layer3::subsets::any::io::binary::inferences;
        pub use semio_s_artifact_stdio_mp3_mutations::standards::mpeg1_layer3::subsets::any::io::binary::mutations;
        pub use semio_s_artifact_stdio_mp3_mutations::standards::mpeg1_layer3::subsets::any::io::binary::snapshot;
    }
    pub mod sqlite {
        pub use semio_s_artifact_stdio_mp3_metadata::standards::mpeg1_layer3::subsets::any::io::sqlite::*;
        pub use semio_s_artifact_stdio_mp3_mutations::standards::mpeg1_layer3::subsets::any::io::sqlite::snapshot;
    }
    pub mod text {
        pub use semio_s_artifact_stdio_mp3_metadata::standards::mpeg1_layer3::subsets::any::io::text::*;
        pub use semio_s_artifact_stdio_mp3_mutations::standards::mpeg1_layer3::subsets::any::io::text::diff;
        pub use semio_s_artifact_stdio_mp3_schema::standards::mpeg1_layer3::subsets::any::io::text::inferences;
        pub use semio_s_artifact_stdio_mp3_mutations::standards::mpeg1_layer3::subsets::any::io::text::mutations;
        pub use semio_s_artifact_stdio_mp3_mutations::standards::mpeg1_layer3::subsets::any::io::text::snapshot;
    }
}
                #[path = "."]
                pub mod schema {
                    pub use semio_s_artifact_stdio_mp3_schema::standards::mpeg1_layer3::subsets::any::schema::*;

                    pub use semio_s_artifact_stdio_mp3_mutations::standards::mpeg1_layer3::subsets::any::schema::snapshot;
                    pub use semio_s_artifact_stdio_mp3_inferences::standards::mpeg1_layer3::subsets::any::schema::inferences;
                    pub use semio_s_artifact_stdio_mp3_mutations::standards::mpeg1_layer3::subsets::any::schema::diff;
                    pub use semio_s_artifact_stdio_mp3_mutations::standards::mpeg1_layer3::subsets::any::schema::mutations;
                }
            
}
        }
    }
}
pub use semio_s_artifact_stdio_mp3_metadata::examples;

#[path = "."]
pub mod editor {
    #[path = "."]
    pub mod mp3 {
        pub use semio_s_artifact_stdio_mp3_inferences::editor::mp3::*;

        pub use semio_s_artifact_stdio_mp3_metadata::editor::mp3::edit_rules;
        pub use semio_s_artifact_stdio_mp3_metadata::editor::mp3::presence;
        pub use semio_s_artifact_stdio_mp3_metadata::editor::mp3::config;
        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod edit {
                pub use semio_s_artifact_stdio_mp3_inferences::editor::mp3::modes::edit::*;

                pub use semio_s_artifact_stdio_mp3_inferences::editor::mp3::modes::edit::windows;
            }
        }
    }
}

#[path = "."]
pub mod viewer {
    #[path = "."]
    pub mod mp3 {
        pub use semio_s_artifact_stdio_mp3_inferences::viewer::mp3::*;

        pub use semio_s_artifact_stdio_mp3_metadata::viewer::mp3::presence;
        pub use semio_s_artifact_stdio_mp3_metadata::viewer::mp3::config;
        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod view {
                pub use semio_s_artifact_stdio_mp3_inferences::viewer::mp3::modes::view::*;

                pub use semio_s_artifact_stdio_mp3_inferences::viewer::mp3::modes::view::windows;
            }
        }
    }
}

pub use crate::standards::mpeg1_layer3::subsets::any::io::{Mp3BuilderConstruction, Mp3Parts, Mp3AnalyzerAnalysis, Mp3BuilderFacets, Mp3Builder, Mp3Analyzer, Mp3Composer};

#[cfg(test)]
#[path = "🏅️standards/🔖️mpeg1-layer3/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🦀️.rs"]
mod hoisted_snapshot_0;
