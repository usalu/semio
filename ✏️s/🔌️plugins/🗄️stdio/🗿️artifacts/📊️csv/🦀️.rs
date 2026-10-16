//! 🎪 `stdio.csv` artifact — stdio reference format.

#![allow(async_fn_in_trait)]
#![allow(long_running_const_eval)]

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_schema as framework_schema;
extern crate semio_framework_value_derive as value_derive;

pub use semio_s_artifact_stdio_contract::{apply_mutation, apply_mutation_checked, mutation_from_payload_json, mutation_inverse, mutation_payload_json, MutationRefusal};

pub use schema::diff::CsvDiff;
pub use schema::mutations::CsvMutation;
pub use schema::snapshot::{CsvField, CsvRecord, CsvSnapshot};
pub use schema::CsvArtifact;

pub use semio_s_artifact_stdio_csv_snapshot::{STDIO_CSV_DOCUMENT_SCHEMA};

pub use semio_s_artifact_stdio_csv_editor::{CSV_ARTIFACT_SCHEMA_ID};

pub use semio_s_artifact_stdio_csv_io_registry::{ARTIFACT_DEFINITION_SCHEMA};

pub use semio_s_artifact_stdio_csv_io_registry::{definition};

pub use semio_s_artifact_stdio_csv_io_registry::{formats};

pub use semio_s_artifact_stdio_csv_io_registry::{native_codec};

pub use semio_s_artifact_stdio_csv_io_registry::{native_codecs};

pub use semio_s_artifact_stdio_csv_io_registry::{contribution};

pub use semio_s_artifact_stdio_csv_io_registry::{artifact_kind};

pub use semio_s_artifact_stdio_csv_io_registry::{assembly, declaration, pilot_languages};

//#region 🚪️DerivedIoRegistry
pub use semio_s_artifact_stdio_csv_io_registry::io_registry;
//#endregion 🚪️DerivedIoRegistry

#[path = "."]
pub mod standards {
    #[path = "."]
    pub mod v_rfc4180 {
        #[path = "."]
        pub mod subsets {
            #[path = "."]
            pub mod any {
                pub use semio_s_artifact_stdio_csv_editor::standards::v_rfc4180::subsets::any::examples;
                #[path = "."]
                pub mod schema {
                    pub use semio_s_artifact_stdio_csv_schema::standards::v_rfc4180::subsets::any::schema::*;
                    
                    pub use semio_s_artifact_stdio_csv_snapshot::standards::v_rfc4180::subsets::any::schema::snapshot;
                    pub use semio_s_artifact_stdio_csv_schema::standards::v_rfc4180::subsets::any::schema::inferences;
                    pub use semio_s_artifact_stdio_csv_mutations::standards::v_rfc4180::subsets::any::schema::diff;
                    pub use semio_s_artifact_stdio_csv_mutations::standards::v_rfc4180::subsets::any::schema::mutations;
                }
                #[path = "."]
                pub mod io {
                    pub use semio_s_artifact_stdio_csv_io_registry::standards::v_rfc4180::subsets::any::io::*;
pub mod binary {
    pub use semio_s_artifact_stdio_csv_io_registry::standards::v_rfc4180::subsets::any::io::binary::*;
    pub use semio_s_artifact_stdio_csv_editor::standards::v_rfc4180::subsets::any::io::binary::inferences;
}
pub use semio_s_artifact_stdio_csv_snapshot::standards::v_rfc4180::subsets::any::io::sqlite;
pub mod text {
    pub use semio_s_artifact_stdio_csv_io_registry::standards::v_rfc4180::subsets::any::io::text::*;
    pub use semio_s_artifact_stdio_csv_editor::standards::v_rfc4180::subsets::any::io::text::inferences;
}
                    
                    pub use semio_s_artifact_stdio_csv_mutations::standards::v_rfc4180::subsets::any::io::import;
                    pub use semio_s_artifact_stdio_csv_mutations::standards::v_rfc4180::subsets::any::io::export;
                }
            }
        }
    }
}

// ---- Shims: keep pre-migration module paths resolving for external callers ----
pub mod schema {
    pub use super::standards::v_rfc4180::subsets::any::schema::*;
}

pub use standards::v_rfc4180::subsets::any::examples;

#[path = "."]
pub mod editor {
    #[path = "."]
    pub mod csv {
        pub use semio_s_artifact_stdio_csv_io_registry::editor::csv::*;
        
        pub use semio_s_artifact_stdio_csv_editor::editor::csv::edit_rules;
        pub use semio_s_artifact_stdio_csv_editor::editor::csv::presence;
        pub use semio_s_artifact_stdio_csv_editor::editor::csv::config;
        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod edit {
                pub use semio_s_artifact_stdio_csv_mutations::editor::csv::modes::edit::*;
                
                pub use semio_s_artifact_stdio_csv_mutations::editor::csv::modes::edit::windows;
            }
        }
    }
}

#[path = "."]
pub mod viewer {
    #[path = "."]
    pub mod csv {
        pub use semio_s_artifact_stdio_csv_io_registry::viewer::csv::*;
        
        pub use semio_s_artifact_stdio_csv_editor::viewer::csv::presence;
        pub use semio_s_artifact_stdio_csv_editor::viewer::csv::config;
        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod view {
                pub use semio_s_artifact_stdio_csv_mutations::viewer::csv::modes::view::*;
                
                pub use semio_s_artifact_stdio_csv_mutations::viewer::csv::modes::view::windows;
            }
        }
    }
}

pub use crate::standards::v_rfc4180::subsets::any::io::{CsvBuilderConstruction, CsvParts, CsvAnalyzerAnalysis, CsvBuilderFacets, CsvBuilder, CsvAnalyzer, CsvComposer};
