//! 🎪 `stdio.png` artifact — stdio reference format.

#![allow(async_fn_in_trait)]
#![allow(long_running_const_eval)]

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_schema as framework_schema;
extern crate semio_framework_value_derive as value_derive;

pub use semio_s_artifact_stdio_contract::{apply_mutation, apply_mutation_checked, mutation_from_payload_json, mutation_inverse, mutation_payload_json, MutationRefusal};

pub(crate) use semio_s_artifact_stdio_contract::base64_standard;

pub use schema::diff::PngDiff;
pub use schema::mutations::PngMutation;
pub use schema::snapshot::PngSnapshot;
pub use schema::PngArtifact;
/// 🧾️ The DSL contract `PngSnapshot` implements, re-exported for clients that print or parse it.
pub use pack::snapshot::ArtifactDsl;

pub use semio_s_artifact_stdio_png_snapshot::{STDIO_PNG_DOCUMENT_SCHEMA};

pub use semio_s_artifact_stdio_png_snapshot::{PNG_ARTIFACT_SCHEMA_ID};

pub use semio_s_artifact_stdio_png_snapshot::{ARTIFACT_DEFINITION_SCHEMA};

pub use semio_s_artifact_stdio_png_tests::{definition};

pub use semio_s_artifact_stdio_png_snapshot::{formats};

pub use semio_s_artifact_stdio_png_mutations::{native_codec};

pub use semio_s_artifact_stdio_png_tests::{native_codecs};

pub use semio_s_artifact_stdio_png_tests::{contribution};

pub use semio_s_artifact_stdio_png_tests::{PNG_DIALECT};

pub use semio_s_artifact_stdio_png_tests::{artifact_kind};

pub use semio_s_artifact_stdio_png_tests::{assembly, declaration, pilot_languages};

pub use semio_s_artifact_stdio_png_tests::{register};

//#region 🚪️DerivedIoRegistry
pub use semio_s_artifact_stdio_png_tests::io_registry;
//#endregion 🚪️DerivedIoRegistry

#[path = "."]
pub mod standards {
    #[path = "."]
    pub mod v1_2 {
        #[path = "."]
        pub mod subsets {
            #[path = "."]
            pub mod any {
                #[path = "."]
                pub mod schema {
                    pub use semio_s_artifact_stdio_png_schema::standards::v1_2::subsets::any::schema::*;

                    pub use semio_s_artifact_stdio_png_snapshot::standards::v1_2::subsets::any::schema::snapshot;
                    pub use semio_s_artifact_stdio_png_schema::standards::v1_2::subsets::any::schema::inferences;
                    pub use semio_s_artifact_stdio_png_diff::standards::v1_2::subsets::any::schema::diff;
                    pub use semio_s_artifact_stdio_png_mutations::standards::v1_2::subsets::any::schema::mutations;
                    pub use semio_s_artifact_stdio_png_diff::standards::v1_2::subsets::any::schema::operations;
                }
                #[path = "."]
                pub mod io {
                    pub use semio_s_artifact_stdio_png_mutations::standards::v1_2::subsets::any::io::*;
pub mod binary {
    pub use semio_s_artifact_stdio_png_mutations::standards::v1_2::subsets::any::io::binary::*;
    pub use semio_s_artifact_stdio_png_diff::standards::v1_2::subsets::any::io::binary::inferences;
}
pub mod text {
    pub use semio_s_artifact_stdio_png_mutations::standards::v1_2::subsets::any::io::text::*;
    pub use semio_s_artifact_stdio_png_diff::standards::v1_2::subsets::any::io::text::inferences;
}

                    #[path = "."]
                    pub mod import {
                        #[path = "."]
                        pub mod deserializers {
                            #[path = "."]
                            pub mod artifacts {
                                pub use semio_s_artifact_stdio_png_tests::standards::v1_2::subsets::any::io::import::deserializers::artifacts::binary;
                                pub use semio_s_artifact_stdio_png_tests::standards::v1_2::subsets::any::io::import::deserializers::artifacts::deflate;
                            }
                        }
                    }
                    #[path = "."]
                    pub mod export {
                        #[path = "."]
                        pub mod serializers {
                            #[path = "."]
                            pub mod artifacts {
                                pub use semio_s_artifact_stdio_png_tests::standards::v1_2::subsets::any::io::export::serializers::artifacts::binary;
                                pub use semio_s_artifact_stdio_png_tests::standards::v1_2::subsets::any::io::export::serializers::artifacts::deflate;
                            }
                        }
                    }
                }
            }
        }
    }
}

// ---- Shims: keep pre-migration module paths resolving for external callers ----
pub mod schema {
    pub use super::standards::v1_2::subsets::any::schema::*;
}

pub use semio_s_artifact_stdio_png_diff::examples;

#[path = "."]
pub mod editor {
    #[path = "."]
    pub mod png {
        pub use semio_s_artifact_stdio_png_tests::editor::png::*;

        pub use semio_s_artifact_stdio_png_diff::editor::png::edit_rules;
        pub use semio_s_artifact_stdio_png_diff::editor::png::presence;
        pub use semio_s_artifact_stdio_png_diff::editor::png::config;
        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod edit {
                pub use semio_s_artifact_stdio_png_tests::editor::png::modes::edit::*;

                pub use semio_s_artifact_stdio_png_tests::editor::png::modes::edit::windows;
            }
        }
    }
}

#[path = "."]
pub mod viewer {
    #[path = "."]
    pub mod png {
        pub use semio_s_artifact_stdio_png_tests::viewer::png::*;

        pub use semio_s_artifact_stdio_png_diff::viewer::png::presence;
        pub use semio_s_artifact_stdio_png_diff::viewer::png::config;
        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod view {
                pub use semio_s_artifact_stdio_png_tests::viewer::png::modes::view::*;

                pub use semio_s_artifact_stdio_png_tests::viewer::png::modes::view::windows;
            }
        }
    }
}

pub use crate::standards::v1_2::subsets::any::io::facets::{PngBuilderConstruction, PngParts, PngAnalyzerAnalysis, PngBuilderFacets, PngBuilder, PngAnalyzer, PngComposer};

#[cfg(test)]
#[path = "🏅️standards/🔖️1.2/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🦀️.rs"]
mod hoisted_snapshot_0;
#[cfg(test)]
#[path = "🏅️standards/🔖️1.2/🪆️subsets/✳️any/🚪️io/🧪️tests/🧬️owned-native-oracle/🦀️.rs"]
mod hoisted_tests_1;
