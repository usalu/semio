//! 🎪 `stdio.tsv` artifact — new-format artifact (master plan "New format artifacts" table).

#![allow(async_fn_in_trait)]
#![allow(long_running_const_eval)]

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_schema as framework_schema;
extern crate semio_framework_value_derive as value_derive;

pub use semio_s_artifact_stdio_contract::{apply_mutation, apply_mutation_checked, mutation_from_payload_json, mutation_inverse, mutation_payload_json, MutationRefusal};

pub use standards::iana::subsets::any::schema::diff::TsvDiff;
pub use standards::iana::subsets::any::schema::mutations::TsvMutation;
pub use standards::iana::subsets::any::schema::snapshot::TsvSnapshot;
pub use standards::iana::subsets::any::schema::TsvArtifact;

pub use semio_s_artifact_stdio_tsv_mutations::{STDIO_TSV_DOCUMENT_SCHEMA};

pub use semio_s_artifact_stdio_tsv_snapshot::{TSV_ARTIFACT_SCHEMA_ID};

pub use semio_s_artifact_stdio_tsv_mutations::{ARTIFACT_DEFINITION_SCHEMA};

pub use semio_s_artifact_stdio_tsv_mutations::{artifact_kind, assembly, declaration, definition, native_codecs};

pub use semio_s_artifact_stdio_tsv_mutations::{formats};

pub use semio_s_artifact_stdio_tsv_mutations::{native_codec};

pub use semio_s_artifact_stdio_tsv_mutations::{contribution};

pub use semio_s_artifact_stdio_tsv_io_registry::{register, register_pilot_languages};

//#region 🚪️DerivedIoRegistry
pub use semio_s_artifact_stdio_tsv_io_registry::io_registry;
//#endregion 🚪️DerivedIoRegistry

#[path = "."]
pub mod standards {
    #[path = "."]
    pub mod iana {
        // 🐜️ `⚙️engine/` dissolved (ticket 26/08/12/ENGINELESS-ARTIFACTS-AND-APP-STATE-MACHINES):
        // `sniff_real_bytes`/`decode_tsv`/`encode_tsv` moved to `subsets::any::schema::snapshot`
        // (mirrors `json`'s own `parse_json_text`/`write_json_text` placement — this artifact's
        // codec is a pure text<->snapshot round trip, no cross-artifact bridging), `io_registry`
        // moved to `subsets::any::io::io_registry`. `register()` is one of stdio's 10 protected
        // imperative plugin-root calls; the root `📑️tsv/🦀️.rs`'s OWN `register()` now
        // covers it directly (`crate::register()` — see stdio's plugin root).
        #[path = "."]
        pub mod subsets {
            #[path = "."]
            pub mod any {
                pub mod io {
    pub use semio_s_artifact_stdio_tsv_io_registry::standards::iana::subsets::any::io::*;
    pub use semio_s_artifact_stdio_tsv_snapshot::standards::iana::subsets::any::io::sqlite;
}
                #[path = "."]
                pub mod schema {
                    pub use semio_s_artifact_stdio_tsv_schema::standards::iana::subsets::any::schema::*;
                    
                    pub use semio_s_artifact_stdio_tsv_snapshot::standards::iana::subsets::any::schema::snapshot;
                    pub use semio_s_artifact_stdio_tsv_inferences::standards::iana::subsets::any::schema::inferences;
                    pub use semio_s_artifact_stdio_tsv_inferences::standards::iana::subsets::any::schema::diff;
                    pub use semio_s_artifact_stdio_tsv_mutations::standards::iana::subsets::any::schema::mutations;
                }
            }
        }
    }
}
pub use semio_s_artifact_stdio_tsv_inferences::examples;

#[path = "."]
pub mod editor {
    #[path = "."]
    pub mod tsv {
        pub use semio_s_artifact_stdio_tsv_mutations::editor::tsv::*;
        
        pub use semio_s_artifact_stdio_tsv_inferences::editor::tsv::edit_rules;
        pub use semio_s_artifact_stdio_tsv_inferences::editor::tsv::presence;
        pub use semio_s_artifact_stdio_tsv_inferences::editor::tsv::config;
        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod edit {
                pub use semio_s_artifact_stdio_tsv_schema::editor::tsv::modes::edit::*;
                
                pub use semio_s_artifact_stdio_tsv_schema::editor::tsv::modes::edit::windows;
            }
        }
    }
}

#[path = "."]
pub mod viewer {
    #[path = "."]
    pub mod tsv {
        pub use semio_s_artifact_stdio_tsv_mutations::viewer::tsv::*;
        
        pub use semio_s_artifact_stdio_tsv_inferences::viewer::tsv::presence;
        pub use semio_s_artifact_stdio_tsv_inferences::viewer::tsv::config;
        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod view {
                pub use semio_s_artifact_stdio_tsv_mutations::viewer::tsv::modes::view::*;
                
                pub use semio_s_artifact_stdio_tsv_schema::viewer::tsv::modes::view::windows;
            }
        }
    }
}

pub use crate::standards::iana::subsets::any::io::{TsvBuilderConstruction, TsvParts, TsvAnalyzerAnalysis, TsvBuilderFacets, TsvBuilder, TsvAnalyzer, TsvComposer};

#[cfg(test)]
#[path = "🏅️standards/🔖️iana/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🦀️.rs"]
mod hoisted_snapshot_0;
