//! 🎪 `stdio.bcf` artifact — stdio reference format.

#![allow(async_fn_in_trait)]
#![allow(long_running_const_eval)]

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_schema as framework_schema;
extern crate semio_framework_value_derive as value_derive;

pub use semio_s_artifact_stdio_contract::{apply_mutation, apply_mutation_checked, mutation_from_payload_json, mutation_inverse, mutation_payload_json, MutationRefusal};

pub use schema::diff::BcfDiff;
pub use schema::mutations::BcfMutation;
pub use schema::snapshot::BcfSnapshot;
pub use schema::BcfArtifact;

pub use semio_s_artifact_stdio_bcf_inferences::{STDIO_BCF_DOCUMENT_SCHEMA};

pub use semio_s_artifact_stdio_bcf_inferences::{BCF_ARTIFACT_SCHEMA_ID};

pub use semio_s_artifact_stdio_bcf_inferences::{ARTIFACT_DEFINITION_SCHEMA};

pub use semio_s_artifact_stdio_bcf_mutations::{definition};

pub use semio_s_artifact_stdio_bcf_inferences::{formats};

pub use semio_s_artifact_stdio_bcf_mutations::{native_codec};

pub use semio_s_artifact_stdio_bcf_mutations::{native_codecs};

pub use semio_s_artifact_stdio_bcf_io_registry::{contribution};

pub use semio_s_artifact_stdio_bcf_io_registry::{assembly, declaration, pilot_languages};

pub use semio_s_artifact_stdio_bcf_inferences::{artifact_kind};
//#region 🚪️DerivedIoRegistry
pub use semio_s_artifact_stdio_bcf_io_registry::io_registry;
//#endregion 🚪️DerivedIoRegistry

#[path = "."]
pub mod standards {
    #[path = "."]
    pub mod v2_1 {
        #[path = "."]
        pub mod subsets {
            #[path = "."]
            pub mod any {
                #[path = "."]
                pub mod schema {
                    pub use semio_s_artifact_stdio_bcf_schema::standards::v2_1::subsets::any::schema::*;

                    pub use semio_s_artifact_stdio_bcf_inferences::standards::v2_1::subsets::any::schema::snapshot;
                    pub use semio_s_artifact_stdio_bcf_inferences::standards::v2_1::subsets::any::schema::diff;
                    pub use semio_s_artifact_stdio_bcf_inferences::standards::v2_1::subsets::any::schema::inferences;
                    pub use semio_s_artifact_stdio_bcf_mutations::standards::v2_1::subsets::any::schema::mutations;
                }
                #[path = "."]
                pub mod io {
                    pub use semio_s_artifact_stdio_bcf_io_registry::standards::v2_1::subsets::any::io::*;
pub mod sqlite {
    pub use semio_s_artifact_stdio_bcf_mutations::standards::v2_1::subsets::any::io::sqlite::*;
    pub use semio_s_artifact_stdio_bcf_inferences::standards::v2_1::subsets::any::io::sqlite::snapshot;
}

                    pub use semio_s_artifact_stdio_bcf_io_registry::standards::v2_1::subsets::any::io::import;
                    pub use semio_s_artifact_stdio_bcf_io_registry::standards::v2_1::subsets::any::io::export;
                }
            }
        }
    }
}

// ---- Shims: keep pre-migration module paths resolving for external callers ----
pub mod schema {
    pub use super::standards::v2_1::subsets::any::schema::*;
}

pub use semio_s_artifact_stdio_bcf_mutations::examples;

#[path = "."]
pub mod editor {
    #[path = "."]
    pub mod bcf {
        pub use semio_s_artifact_stdio_bcf_io_registry::editor::bcf::*;

        pub use semio_s_artifact_stdio_bcf_mutations::editor::bcf::edit_rules;
        pub use semio_s_artifact_stdio_bcf_mutations::editor::bcf::presence;
        pub use semio_s_artifact_stdio_bcf_mutations::editor::bcf::config;
        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod edit {
                pub use semio_s_artifact_stdio_bcf_schema::editor::bcf::modes::edit::*;

                pub use semio_s_artifact_stdio_bcf_inferences::editor::bcf::modes::edit::windows;
            }
        }
    }
}

#[path = "."]
pub mod viewer {
    #[path = "."]
    pub mod bcf {
        pub use semio_s_artifact_stdio_bcf_io_registry::viewer::bcf::*;

        pub use semio_s_artifact_stdio_bcf_mutations::viewer::bcf::presence;
        pub use semio_s_artifact_stdio_bcf_mutations::viewer::bcf::config;
        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod view {
                pub use semio_s_artifact_stdio_bcf_schema::viewer::bcf::modes::view::*;

                pub use semio_s_artifact_stdio_bcf_inferences::viewer::bcf::modes::view::windows;
            }
        }
    }
}

pub use crate::standards::v2_1::subsets::any::io::{BcfBuilderConstruction, BcfParts, BcfAnalyzerAnalysis, BcfBuilderFacets, BcfBuilder, BcfAnalyzer, BcfComposer};

#[cfg(test)]
#[path = "🏅️standards/🔖️2.1/🪆️subsets/🖊️markup/🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🦀️.rs"]
mod hoisted_snapshot_0;
