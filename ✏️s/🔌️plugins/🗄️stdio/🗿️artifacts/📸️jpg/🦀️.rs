//! 🎪 `stdio.jpg` artifact — stdio reference format.

#![allow(async_fn_in_trait)]
#![allow(long_running_const_eval)]

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_schema as framework_schema;
extern crate semio_framework_value_derive as value_derive;

pub use semio_s_artifact_stdio_contract::{apply_mutation, apply_mutation_checked, mutation_from_payload_json, mutation_inverse, mutation_payload_json, MutationRefusal};

pub use schema::diff::JpgDiff;
pub use schema::mutations::JpgMutation;
pub use schema::snapshot::{JpgSnapshot,JpgImage};
pub use schema::JpgArtifact;

pub use semio_s_artifact_stdio_jpg_inferences::{STDIO_JPG_DOCUMENT_SCHEMA};

pub use semio_s_artifact_stdio_jpg_mutations::{JPG_ARTIFACT_SCHEMA_ID};

pub use semio_s_artifact_stdio_jpg_operations::{ARTIFACT_DEFINITION_SCHEMA};

pub use semio_s_artifact_stdio_jpg_schema_2::{definition};

pub use semio_s_artifact_stdio_jpg_operations::{formats};

pub use semio_s_artifact_stdio_jpg_mutations::{native_codec};

pub use semio_s_artifact_stdio_jpg_mutations::{native_codecs};

pub use semio_s_artifact_stdio_jpg_io_registry::{contribution};

pub use semio_s_artifact_stdio_jpg_mutations::{JPG_ANY_DIALECT, JPG_BASELINE_DIALECT};

pub use semio_s_artifact_stdio_jpg_mutations::{artifact_kind};

pub use semio_s_artifact_stdio_jpg_io_registry::{assembly, declaration, declared_subset_validators, pilot_languages};

//#region 🚪️DerivedIoRegistry
pub use semio_s_artifact_stdio_jpg_io_registry::io_registry;
//#endregion 🚪️DerivedIoRegistry

#[path = "."]
pub mod standards {
    #[path = "."]
    pub mod v_jfif_1_01 {
        // 🐜️ `⚙️engine/` dissolved (ticket 26/08/12/ENGINELESS-ARTIFACTS-AND-APP-STATE-MACHINES):
        // real code now lives in `subsets::document::io` (codec/io_registry) and
        // `subsets::document::schema` (document helpers); this stays an inline barrel so every
        // existing `standards::v_jfif_1_01::engine::*`/root `engine::*` path still resolves
        // (`📸️remodel`'s own `jpg::engine::decode_jpg`/`encode_jpg`/`JpgError` consumer
        // included).

        #[path = "."]
        pub mod subsets {
            #[path = "."]
            pub mod document {
                #[path = "."]
                pub mod schema {
                    pub use semio_s_artifact_stdio_jpg_schema::standards::v_jfif_1_01::subsets::document::schema::*;

                    pub use semio_s_artifact_stdio_jpg_inferences::standards::v_jfif_1_01::subsets::document::schema::snapshot;
                    pub use semio_s_artifact_stdio_jpg_inferences::standards::v_jfif_1_01::subsets::document::schema::inferences;
                    pub use semio_s_artifact_stdio_jpg_mutations::standards::v_jfif_1_01::subsets::document::schema::diff;
                    pub use semio_s_artifact_stdio_jpg_mutations::standards::v_jfif_1_01::subsets::document::schema::mutations;

                    pub use semio_s_artifact_stdio_jpg_operations::standards::v_jfif_1_01::subsets::document::schema::operations;
                }
                #[path = "."]
                pub mod io {
                    pub use semio_s_artifact_stdio_jpg_inferences::standards::v_jfif_1_01::subsets::document::io::*;

                    pub use semio_s_artifact_stdio_jpg_schema_2::standards::v_jfif_1_01::subsets::document::io::facets;
                    #[path = "."]
                    pub mod binary {
                        #[path = "."]
                        pub mod snapshot {
                            pub use semio_s_artifact_stdio_jpg_inferences::standards::v_jfif_1_01::subsets::document::io::binary::snapshot::*;

                            pub use semio_s_artifact_stdio_jpg_inferences::standards::v_jfif_1_01::subsets::document::io::binary::snapshot::decoded_components;
                            pub use semio_s_artifact_stdio_jpg_inferences::standards::v_jfif_1_01::subsets::document::io::binary::snapshot::encode_options;
                        }
                        pub use semio_s_artifact_stdio_jpg_mutations::standards::v_jfif_1_01::subsets::document::io::binary::mutations;
                        pub use semio_s_artifact_stdio_jpg_mutations::standards::v_jfif_1_01::subsets::document::io::binary::diff;
                        pub use semio_s_artifact_stdio_jpg_operations::standards::v_jfif_1_01::subsets::document::io::binary::inferences;
                    }
                    #[path = "."]
                    pub mod text {
                        pub use semio_s_artifact_stdio_jpg_inferences::standards::v_jfif_1_01::subsets::document::io::text::snapshot;
                        pub use semio_s_artifact_stdio_jpg_mutations::standards::v_jfif_1_01::subsets::document::io::text::mutations;
                        pub use semio_s_artifact_stdio_jpg_mutations::standards::v_jfif_1_01::subsets::document::io::text::diff;
                        pub use semio_s_artifact_stdio_jpg_operations::standards::v_jfif_1_01::subsets::document::io::text::inferences;
                    }
                    pub use semio_s_artifact_stdio_jpg_inferences::standards::v_jfif_1_01::subsets::document::io::sqlite;
                    pub use semio_s_artifact_stdio_jpg_schema_2::standards::v_jfif_1_01::subsets::document::io::import;
                    pub use semio_s_artifact_stdio_jpg_schema_2::standards::v_jfif_1_01::subsets::document::io::export;
                }
            }
            #[path = "."]
            pub mod baseline {
                // 🏅️ ITU-T T.81 / ISO 10918-1 Annex F baseline sequential DCT (JFIF 1.01
                // container) -- ticket 26/08/11/ARTIFACT-STANDARD-SUBSETS-REAL-VOCABULARIES.
                // `schema` re-exports the ✳️any subset's `JpgSnapshot` verbatim (same Rust
                // type, same `s.stdio.jpg` schema id); `io` reuses the ✳️any subset's
                // `binary` DAG leaf rather than duplicating it. `JpgSnapshot` gained
                // `frame`/`sof_marker`/`arithmetic`/`dc_huffman_table_count`/
                // `ac_huffman_table_count` fields as part of this subset landing --
                // `⚙️engine::decode_jpg` now persists what it already computed transiently.
                pub use semio_s_artifact_stdio_jpg_io_registry::standards::v_jfif_1_01::subsets::baseline::io;
                pub use semio_s_artifact_stdio_jpg_schema_2::standards::v_jfif_1_01::subsets::baseline::schema;
            
}
        }
    }
}

pub mod schema {
    pub use super::standards::v_jfif_1_01::subsets::document::schema::*;
}

pub use semio_s_artifact_stdio_jpg_operations::examples;

#[path = "."]
pub mod editor {
    #[path = "."]
    pub mod jpg_any {
        pub use semio_s_artifact_stdio_jpg_schema_2::editor::jpg_any::*;

        pub use semio_s_artifact_stdio_jpg_operations::editor::jpg_any::edit_rules;
        pub use semio_s_artifact_stdio_jpg_operations::editor::jpg_any::presence;
        pub use semio_s_artifact_stdio_jpg_operations::editor::jpg_any::config;
        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod edit {
                pub use semio_s_artifact_stdio_jpg_schema_2::editor::jpg_any::modes::edit::*;

                pub use semio_s_artifact_stdio_jpg_schema_2::editor::jpg_any::modes::edit::windows;
            }
        }
    }
    #[path = "."]
    pub mod jpg_baseline {
        pub use semio_s_artifact_stdio_jpg_io_registry::editor::jpg_baseline::*;

        pub use semio_s_artifact_stdio_jpg_operations::editor::jpg_baseline::presence;
        pub use semio_s_artifact_stdio_jpg_operations::editor::jpg_baseline::config;
        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod edit {
                pub use semio_s_artifact_stdio_jpg_schema_2::editor::jpg_baseline::modes::edit::*;

                pub use semio_s_artifact_stdio_jpg_schema_2::editor::jpg_baseline::modes::edit::windows;
            }
        }
    }
}

#[path = "."]
pub mod viewer {
    #[path = "."]
    pub mod jpg_any {
        pub use semio_s_artifact_stdio_jpg_schema_2::viewer::jpg_any::*;

        pub use semio_s_artifact_stdio_jpg_operations::viewer::jpg_any::presence;
        pub use semio_s_artifact_stdio_jpg_operations::viewer::jpg_any::config;
        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod view {
                pub use semio_s_artifact_stdio_jpg_schema_2::viewer::jpg_any::modes::view::*;

                pub use semio_s_artifact_stdio_jpg_schema_2::viewer::jpg_any::modes::view::windows;
            }
        }
    }
    #[path = "."]
    pub mod jpg_baseline {
        pub use semio_s_artifact_stdio_jpg_schema_2::viewer::jpg_baseline::*;

        pub use semio_s_artifact_stdio_jpg_operations::viewer::jpg_baseline::presence;
        pub use semio_s_artifact_stdio_jpg_operations::viewer::jpg_baseline::config;
        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod view {
                pub use semio_s_artifact_stdio_jpg_schema_2::viewer::jpg_baseline::modes::view::*;

                pub use semio_s_artifact_stdio_jpg_schema_2::viewer::jpg_baseline::modes::view::windows;
            }
        }
    }
}

pub use crate::standards::v_jfif_1_01::subsets::baseline::io::{JpgBaselineBuilderConstruction, JpgBaselineAnalyzerAnalysis, JpgBaselineBuilderFacets, JpgBaselineBuilder, JpgBaselineAnalyzer, JpgBaselineComposer};

pub use crate::standards::v_jfif_1_01::subsets::document::io::facets::{JpgBuilderConstruction, JpgParts, JpgAnalyzerAnalysis, JpgBuilderFacets, JpgBuilder, JpgAnalyzer, JpgComposer};

#[cfg(test)]
#[path = "🏅️standards/🔖️jfif-1.01/🪆️subsets/🧾️document/🚪️io/🪶️sqlite/📸️snapshot/🫴️receiving/🧪️tests/🦀️.rs"]
mod hoisted_receiving_0;
#[cfg(test)]
#[path = "🏅️standards/🔖️jfif-1.01/🪆️subsets/🧾️document/🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🦀️.rs"]
mod hoisted_snapshot_1;
