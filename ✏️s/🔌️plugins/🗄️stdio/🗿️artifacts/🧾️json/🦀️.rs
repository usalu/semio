//! 🎪 `stdio.json` artifact — stdio reference format.

#![allow(async_fn_in_trait)]
#![allow(long_running_const_eval)]

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_schema as framework_schema;
extern crate semio_framework_value_derive as value_derive;

pub use semio_s_artifact_stdio_contract::{apply_mutation, apply_mutation_checked, mutation_from_payload_json, mutation_inverse, mutation_payload_json, MutationRefusal};

pub use schema::diff::JsonDiff;
pub use schema::mutations::JsonMutation;
pub use schema::snapshot::JsonSnapshot;
pub use schema::JsonArtifact;

pub use semio_s_artifact_stdio_json_schema::{STDIO_JSON_DOCUMENT_SCHEMA};

pub use semio_s_artifact_stdio_json_number::{JSON_ARTIFACT_SCHEMA_ID};

pub use semio_s_artifact_stdio_json_tests::{ARTIFACT_DEFINITION_SCHEMA};

pub use semio_s_artifact_stdio_json_tests::{definition};

pub use semio_s_artifact_stdio_json_tests::{formats};

pub use semio_s_artifact_stdio_json_tests::{native_codec};

pub use semio_s_artifact_stdio_json_tests::{native_codecs};

pub use semio_s_artifact_stdio_json_tests::{contribution};

pub use semio_s_artifact_stdio_json_tests::{artifact_kind};

pub use semio_s_artifact_stdio_json_tests::{assembly, declaration, pilot_languages, pilot_subset_validators};

//#region 🚪️DerivedIoRegistry
pub use semio_s_artifact_stdio_json_tests::io_registry;
//#endregion 🚪️DerivedIoRegistry

#[path = "."]
pub mod standards {
    #[path = "."]
    pub mod v_rfc8259 {
        #[path = "."]
        pub mod subsets {
            #[path = "."]
            pub mod base {
                #[path = "."]
                pub mod schema {
                    pub use semio_s_artifact_stdio_json_schema_2::standards::v_rfc8259::subsets::base::schema::*;

                    pub use semio_s_artifact_stdio_json_mutations::standards::v_rfc8259::subsets::base::schema::mutation_support;
                    pub use semio_s_artifact_stdio_json_schema::standards::v_rfc8259::subsets::base::schema::snapshot;
                    pub use semio_s_artifact_stdio_json_schema::standards::v_rfc8259::subsets::base::schema::inferences;
                    pub use semio_s_artifact_stdio_json_mutations::standards::v_rfc8259::subsets::base::schema::diff;
                    pub use semio_s_artifact_stdio_json_mutations::standards::v_rfc8259::subsets::base::schema::mutations;
                }
                #[path = "."]
                pub mod io {
                    pub use semio_s_artifact_stdio_json_tests::standards::v_rfc8259::subsets::base::io::*;
pub mod binary {
    pub use semio_s_artifact_stdio_json_tests::standards::v_rfc8259::subsets::base::io::binary::*;
    pub use semio_s_artifact_stdio_json_number::standards::v_rfc8259::subsets::base::io::binary::inferences;
}
pub mod sqlite {
    pub use semio_s_artifact_stdio_json_number::standards::v_rfc8259::subsets::base::io::sqlite::*;
    pub use semio_s_artifact_stdio_json_schema::standards::v_rfc8259::subsets::base::io::sqlite::snapshot;
}

                    pub use semio_s_artifact_stdio_json_schema::standards::v_rfc8259::subsets::base::io::import;
                    pub use semio_s_artifact_stdio_json_schema::standards::v_rfc8259::subsets::base::io::export;
                }
            }
            #[path = "."]
            pub mod geojson {
                pub use semio_s_artifact_stdio_json_schema::standards::v_rfc8259::subsets::geojson::schema;
                pub use semio_s_artifact_stdio_json_tests::standards::v_rfc8259::subsets::geojson::io;
            }
            #[path = "."]
            pub mod i_json {
                pub use semio_s_artifact_stdio_json_number::standards::v_rfc8259::subsets::i_json::examples;
                #[path = "."]
                pub mod schema {
                    pub use semio_s_artifact_stdio_json_schema_2::standards::v_rfc8259::subsets::i_json::schema::*;

                    pub use semio_s_artifact_stdio_json_mutations_2::standards::v_rfc8259::subsets::i_json::schema::mutations;
                }
                pub use semio_s_artifact_stdio_json_tests::standards::v_rfc8259::subsets::i_json::io;
            }
        }
    }
}

// ---- Crate-root module paths: the crate's public surface ----
pub mod schema {
    pub use super::standards::v_rfc8259::subsets::base::schema::*;
}

pub use semio_s_artifact_stdio_json_number::examples;

#[path = "."]
pub mod editor {
    #[path = "."]
    pub mod json_any {
        pub use semio_s_artifact_stdio_json_mutations_2::editor::json_any::*;

        pub use semio_s_artifact_stdio_json_mutations_2::editor::json_any::edit_rules;
        pub use semio_s_artifact_stdio_json_number::editor::json_any::presence;
        pub use semio_s_artifact_stdio_json_number::editor::json_any::config;
        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod edit {
                pub use semio_s_artifact_stdio_json_mutations::editor::json_any::modes::edit::*;

                pub use semio_s_artifact_stdio_json_schema::editor::json_any::modes::edit::windows;
            }
        }
    }
    #[path = "."]
    pub mod json_i_json {
        pub use semio_s_artifact_stdio_json_mutations_2::editor::json_i_json::*;

        pub use semio_s_artifact_stdio_json_mutations_2::editor::json_i_json::edit_rules;
        pub use semio_s_artifact_stdio_json_number::editor::json_i_json::presence;
        pub use semio_s_artifact_stdio_json_number::editor::json_i_json::config;
        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod edit {
                pub use semio_s_artifact_stdio_json_mutations::editor::json_i_json::modes::edit::*;

                pub use semio_s_artifact_stdio_json_schema::editor::json_i_json::modes::edit::windows;
            }
        }
    }
}

#[path = "."]
pub mod viewer {
    #[path = "."]
    pub mod json_any {
        pub use semio_s_artifact_stdio_json_mutations_2::viewer::json_any::*;

        pub use semio_s_artifact_stdio_json_number::viewer::json_any::presence;
        pub use semio_s_artifact_stdio_json_number::viewer::json_any::config;
        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod view {
                pub use semio_s_artifact_stdio_json_mutations::viewer::json_any::modes::view::*;

                pub use semio_s_artifact_stdio_json_schema::viewer::json_any::modes::view::windows;
            }
        }
    }
    #[path = "."]
    pub mod json_i_json {
        pub use semio_s_artifact_stdio_json_mutations_2::viewer::json_i_json::*;

        pub use semio_s_artifact_stdio_json_number::viewer::json_i_json::presence;
        pub use semio_s_artifact_stdio_json_number::viewer::json_i_json::config;
        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod view {
                pub use semio_s_artifact_stdio_json_schema_2::viewer::json_i_json::modes::view::*;

                pub use semio_s_artifact_stdio_json_schema_2::viewer::json_i_json::modes::view::windows;
            }
        }
    }
}

pub use crate::standards::v_rfc8259::subsets::i_json::io::{JsonIJsonBuilderConstruction, JsonIJsonAnalyzerAnalysis, JsonIJsonBuilderFacets, JsonIJsonBuilder, JsonIJsonAnalyzer, JsonIJsonComposer};

pub use crate::standards::v_rfc8259::subsets::geojson::io::{JsonGeoJsonBuilderConstruction, JsonGeoJsonAnalyzerAnalysis, JsonGeoJsonBuilderFacets, JsonGeoJsonBuilder, JsonGeoJsonAnalyzer, JsonGeoJsonComposer};

pub use crate::standards::v_rfc8259::subsets::base::io::{JsonBuilderConstruction, JsonParts, JsonAnalyzerAnalysis, JsonBuilderFacets, JsonBuilder, JsonAnalyzer, JsonComposer};

#[cfg(test)]
#[path = "🏅️standards/🔖️rfc8259/🪆️subsets/🧱️base/🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🦀️.rs"]
mod hoisted_snapshot_0;
