//! 🎪 `stdio.epw` artifact — new-format artifact (master plan "New format artifacts" table).

#![allow(async_fn_in_trait)]
#![allow(long_running_const_eval)]

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_schema as framework_schema;
extern crate semio_framework_value_derive as value_derive;

pub use semio_s_artifact_stdio_contract::{apply_mutation, apply_mutation_checked, mutation_from_payload_json, mutation_inverse, mutation_payload_json, MutationRefusal};

pub use standards::energyplus::subsets::any::schema::diff::EpwDiff;
pub use standards::energyplus::subsets::any::schema::mutations::EpwMutation;
pub use standards::energyplus::subsets::any::schema::snapshot::EpwSnapshot;
pub use standards::energyplus::subsets::any::schema::EpwArtifact;

pub use semio_s_artifact_stdio_epw_mutations::{STDIO_EPW_DOCUMENT_SCHEMA};

pub use semio_s_artifact_stdio_epw_snapshot::{EPW_ARTIFACT_SCHEMA_ID};

pub use semio_s_artifact_stdio_epw_io_registry::{ARTIFACT_DEFINITION_SCHEMA};

pub use semio_s_artifact_stdio_epw_io_registry::{ARTIFACT_DEFINITION_CONSTRAINT};

pub use semio_s_artifact_stdio_epw_io_registry::{validate_definition_schema};

pub use semio_s_artifact_stdio_epw_io_registry::{definition};

pub use semio_s_artifact_stdio_epw_io_registry::{formats};

pub use semio_s_artifact_stdio_epw_snapshot::{native_codecs};

pub use semio_s_artifact_stdio_epw_io_registry::{contribution};

pub use semio_s_artifact_stdio_epw_io_registry::{artifact_kind, assembly, declaration};

pub use semio_s_artifact_stdio_epw_io_registry::{register, register_pilot_languages};

//#region 🚪️DerivedIoRegistry
pub use semio_s_artifact_stdio_epw_io_registry::io_registry;
//#endregion 🚪️DerivedIoRegistry

#[path = "."]
pub mod standards {
    #[path = "."]
    pub mod energyplus {
        #[path = "."]
        pub mod subsets {
            #[path = "."]
            pub mod any {
                pub use semio_s_artifact_stdio_epw_io_registry::standards::energyplus::subsets::any::io;
                #[path = "."]
                pub mod schema {
                    pub use semio_s_artifact_stdio_epw_schema::standards::energyplus::subsets::any::schema::*;

                    pub use semio_s_artifact_stdio_epw_snapshot::standards::energyplus::subsets::any::schema::snapshot;
                    pub use semio_s_artifact_stdio_epw_snapshot::standards::energyplus::subsets::any::schema::diff;
                    pub use semio_s_artifact_stdio_epw_schema::standards::energyplus::subsets::any::schema::inferences;
                    pub use semio_s_artifact_stdio_epw_mutations::standards::energyplus::subsets::any::schema::mutations;
                }
            }
        }
    }
}
pub use semio_s_artifact_stdio_epw_snapshot::examples;

#[path = "."]
pub mod editor {
    #[path = "."]
    pub mod epw {
        pub use semio_s_artifact_stdio_epw_editor::editor::epw::*;

        pub use semio_s_artifact_stdio_epw_snapshot::editor::epw::edit_rules;
        pub use semio_s_artifact_stdio_epw_snapshot::editor::epw::presence;
        pub use semio_s_artifact_stdio_epw_snapshot::editor::epw::config;
        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod edit {
                pub use semio_s_artifact_stdio_epw_editor::editor::epw::modes::edit::*;

                pub use semio_s_artifact_stdio_epw_editor::editor::epw::modes::edit::windows;
            }
        }
    }
}

#[path = "."]
pub mod viewer {
    #[path = "."]
    pub mod epw {
        pub use semio_s_artifact_stdio_epw_mutations::viewer::epw::*;

        pub use semio_s_artifact_stdio_epw_snapshot::viewer::epw::presence;
        pub use semio_s_artifact_stdio_epw_snapshot::viewer::epw::config;
        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod view {
                pub use semio_s_artifact_stdio_epw_mutations::viewer::epw::modes::view::*;

                pub use semio_s_artifact_stdio_epw_snapshot::viewer::epw::modes::view::windows;
            }
        }
    }
}

#[cfg(test)]
#[path = "🧪️tests/📜️definition/🦀️.rs"]
mod definition_tests;

pub use crate::standards::energyplus::subsets::any::io::{EpwBuilderConstruction, EpwParts, EpwAnalyzerAnalysis, EpwBuilderFacets, EpwBuilder, EpwAnalyzer, EpwComposer};

#[cfg(test)]
#[path = "🏅️standards/🔖️energyplus/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🦀️.rs"]
mod hoisted_snapshot_0;
