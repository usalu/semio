#![allow(async_fn_in_trait)]
#![allow(long_running_const_eval)]
//! 🎪 `stdio.ply` artifact — stdio reference format.
use semio_framework_plugin::PluginBuilder;
#[cfg(test)] use semio_framework_artifact_reference::{Dialect,StandardId,SubsetId};

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_schema as framework_schema;
extern crate semio_framework_value_derive as value_derive;

pub use semio_s_artifact_stdio_contract::{apply_mutation, apply_mutation_checked, mutation_from_payload_json, mutation_inverse, mutation_payload_json, MutationRefusal};

pub use schema::diff::PlyDiff;
pub use schema::mutations::PlyMutation;
pub use schema::snapshot::PlySnapshot;
pub use schema::PlyArtifact;

pub use semio_s_artifact_stdio_ply_inferences::{STDIO_PLY_DOCUMENT_SCHEMA};

pub use semio_s_artifact_stdio_ply_inferences::{PLY_ARTIFACT_SCHEMA_ID};

pub use semio_s_artifact_stdio_ply_inferences::{ARTIFACT_DEFINITION_SCHEMA};

pub use semio_s_artifact_stdio_ply_mutations::{definition};

pub use semio_s_artifact_stdio_ply_inferences::{formats};

pub use semio_s_artifact_stdio_ply_mutations::{native_codec};

pub use semio_s_artifact_stdio_ply_mutations::{native_codecs};

pub use semio_s_artifact_stdio_ply_io_registry::{contribution};

pub use semio_s_artifact_stdio_ply_io_registry::{assembly, declaration, pilot_languages};

pub use semio_s_artifact_stdio_ply_inferences::{artifact_kind};
//#region 🚪️DerivedIoRegistry
pub use semio_s_artifact_stdio_ply_io_registry::io_registry;
//#endregion 🚪️DerivedIoRegistry

#[path = "."]
pub mod standards {
    #[path = "."]
    pub mod v1_0 {
        // ⚙️→🚪️/🧬️ dissolved (ticket 26/08/12/ENGINELESS-ARTIFACTS-AND-APP-STATE-MACHINES):
        // real code now lives in `subsets::any::{io,schema}`; this stays an inline barrel
        // so every existing `standards::v1_0::engine::*`/root `engine::*` path still resolves.
        pub use semio_s_artifact_stdio_ply_io_registry::standards::v1_0::engine;
        #[path = "."]
        pub mod subsets {
            #[path = "."]
            pub mod any {
                #[path = "."]
                pub mod schema {
                    pub use semio_s_artifact_stdio_ply_schema::standards::v1_0::subsets::any::schema::*;
                    
                    pub use semio_s_artifact_stdio_ply_inferences::standards::v1_0::subsets::any::schema::snapshot;
                    pub use semio_s_artifact_stdio_ply_inferences::standards::v1_0::subsets::any::schema::diff;
                    pub use semio_s_artifact_stdio_ply_inferences::standards::v1_0::subsets::any::schema::inferences;
                    pub use semio_s_artifact_stdio_ply_mutations::standards::v1_0::subsets::any::schema::mutations;
                }
                #[path = "."]
                pub mod io {
                    pub use semio_s_artifact_stdio_ply_io_registry::standards::v1_0::subsets::any::io::*;
pub mod binary {
    pub use semio_s_artifact_stdio_ply_io_registry::standards::v1_0::subsets::any::io::binary::*;
    pub use semio_s_artifact_stdio_ply_inferences::standards::v1_0::subsets::any::io::binary::inferences;
}
pub mod sqlite {
    pub use semio_s_artifact_stdio_ply_mutations::standards::v1_0::subsets::any::io::sqlite::*;
    pub use semio_s_artifact_stdio_ply_inferences::standards::v1_0::subsets::any::io::sqlite::snapshot;
}
pub mod text {
    pub use semio_s_artifact_stdio_ply_io_registry::standards::v1_0::subsets::any::io::text::*;
    pub use semio_s_artifact_stdio_ply_inferences::standards::v1_0::subsets::any::io::text::inferences;
}
                    
                    pub use semio_s_artifact_stdio_ply_io_registry::standards::v1_0::subsets::any::io::import;
                    pub use semio_s_artifact_stdio_ply_io_registry::standards::v1_0::subsets::any::io::export;
                }
            }
        }
    }
}

// ---- Crate-root module paths: the crate's public surface ----
pub mod schema {
    pub use super::standards::v1_0::subsets::any::schema::*;
}
pub mod engine {
    pub use super::standards::v1_0::engine::*;
}

pub use semio_s_artifact_stdio_ply_mutations::examples;

#[path = "."]
pub mod editor {
    #[path = "."]
    pub mod ply {
        pub use semio_s_artifact_stdio_ply_mutations::editor::ply::*;
        
        pub use semio_s_artifact_stdio_ply_mutations::editor::ply::edit_rules;
        pub use semio_s_artifact_stdio_ply_mutations::editor::ply::presence;
        pub use semio_s_artifact_stdio_ply_mutations::editor::ply::config;
        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod edit {
                pub use semio_s_artifact_stdio_ply_inferences::editor::ply::modes::edit::*;
                
                pub use semio_s_artifact_stdio_ply_inferences::editor::ply::modes::edit::windows;
            }
        }
    }
}

#[path = "."]
pub mod viewer {
    #[path = "."]
    pub mod ply {
        pub use semio_s_artifact_stdio_ply_schema::viewer::ply::*;
        
        pub use semio_s_artifact_stdio_ply_mutations::viewer::ply::presence;
        pub use semio_s_artifact_stdio_ply_mutations::viewer::ply::config;
        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod view {
                pub use semio_s_artifact_stdio_ply_schema::viewer::ply::modes::view::*;
                
                pub use semio_s_artifact_stdio_ply_inferences::viewer::ply::modes::view::windows;
            }
        }
    }
}

pub use crate::standards::v1_0::subsets::any::io::{PlyBuilderConstruction, PlyParts, PlyAnalyzerAnalysis, PlyBuilderFacets, PlyBuilder, PlyAnalyzer, PlyComposer};

#[cfg(test)]
#[path = "🏅️standards/🔖️1.0/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🦀️.rs"]
mod hoisted_snapshot_0;
