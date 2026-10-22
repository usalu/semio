#![allow(async_fn_in_trait)]
#![allow(long_running_const_eval)]
//! 🎪 `stdio.stl` artifact — stdio reference format.
use semio_framework_plugin::PluginBuilder;
#[cfg(test)] use semio_framework_artifact_reference::{Dialect,StandardId,SubsetId};

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_schema as framework_schema;
extern crate semio_framework_value_derive as value_derive;

pub use semio_s_artifact_stdio_contract::{apply_mutation, apply_mutation_checked, mutation_from_payload_json, mutation_inverse, mutation_payload_json, MutationRefusal};

pub use schema::diff::StlDiff;
pub use schema::mutations::StlMutation;
pub use schema::snapshot::StlSnapshot;
pub use schema::StlArtifact;

pub use semio_s_artifact_stdio_stl_inferences::{STDIO_STL_DOCUMENT_SCHEMA};

pub use semio_s_artifact_stdio_stl_inferences::{STL_ARTIFACT_SCHEMA_ID};

pub use semio_s_artifact_stdio_stl_viewer::{ARTIFACT_DEFINITION_SCHEMA};

pub use semio_s_artifact_stdio_stl_viewer::{definition};

pub use semio_s_artifact_stdio_stl_viewer::{formats};

pub use semio_s_artifact_stdio_stl_viewer::{native_codec};

pub use semio_s_artifact_stdio_stl_viewer::{native_codecs};

pub use semio_s_artifact_stdio_stl_examples::{contribution};

pub use semio_s_artifact_stdio_stl_viewer::{assembly, declaration, pilot_languages};

pub use semio_s_artifact_stdio_stl_viewer::{artifact_kind};
//#region 🚪️DerivedIoRegistry
pub use semio_s_artifact_stdio_stl_examples::io_registry;
//#endregion 🚪️DerivedIoRegistry

#[path = "."]
pub mod standards {
    #[path = "."]
    pub mod v_ascii {
        // ⚙️→🚪️/🧬️ dissolved (ticket 26/08/12/ENGINELESS-ARTIFACTS-AND-APP-STATE-MACHINES):
        // real code now lives in `subsets::any::{io,schema}`; this stays an inline barrel
        // so every existing `standards::v_ascii::engine::*`/root `engine::*` path still resolves.
        pub use semio_s_artifact_stdio_stl_mutations::standards::v_ascii::engine;
        #[path = "."]
        pub mod subsets {
            #[path = "."]
            pub mod any {
                #[path = "."]
                pub mod schema {
                    pub use semio_s_artifact_stdio_stl_schema::standards::v_ascii::subsets::any::schema::*;
                    
                    pub use semio_s_artifact_stdio_stl_inferences::standards::v_ascii::subsets::any::schema::snapshot;
                    pub use semio_s_artifact_stdio_stl_mutations::standards::v_ascii::subsets::any::schema::diff;
                    pub use semio_s_artifact_stdio_stl_inferences::standards::v_ascii::subsets::any::schema::inferences;
                    pub use semio_s_artifact_stdio_stl_mutations::standards::v_ascii::subsets::any::schema::mutations;
                }
                #[path = "."]
                pub mod io {
                    pub use semio_s_artifact_stdio_stl_mutations::standards::v_ascii::subsets::any::io::*;
pub mod binary {
    pub use semio_s_artifact_stdio_stl_mutations::standards::v_ascii::subsets::any::io::binary::*;
    pub use semio_s_artifact_stdio_stl_inferences::standards::v_ascii::subsets::any::io::binary::inferences;
}
pub mod sqlite {
    pub use semio_s_artifact_stdio_stl_examples::standards::v_ascii::subsets::any::io::sqlite::*;
    pub use semio_s_artifact_stdio_stl_inferences::standards::v_ascii::subsets::any::io::sqlite::snapshot;
}
pub mod text {
    pub use semio_s_artifact_stdio_stl_mutations::standards::v_ascii::subsets::any::io::text::*;
    pub use semio_s_artifact_stdio_stl_inferences::standards::v_ascii::subsets::any::io::text::inferences;
}
                    
                    #[path = "."]
                    pub mod import {
                        #[path = "."]
                        pub mod deserializers {
                            #[path = "."]
                            pub mod artifacts {
                                pub use semio_s_artifact_stdio_stl_viewer::standards::v_ascii::subsets::any::io::import::deserializers::artifacts::binary;
                                pub use semio_s_artifact_stdio_stl_viewer::standards::v_ascii::subsets::any::io::import::deserializers::artifacts::txt;
                            }
                        }
                    }
                    #[path = "."]
                    pub mod export {
                        #[path = "."]
                        pub mod serializers {
                            #[path = "."]
                            pub mod artifacts {
                                pub use semio_s_artifact_stdio_stl_viewer::standards::v_ascii::subsets::any::io::export::serializers::artifacts::binary;
                                pub use semio_s_artifact_stdio_stl_viewer::standards::v_ascii::subsets::any::io::export::serializers::artifacts::txt;
                            }
                        }
                    }
                }
            }
        }
    }
}

// ---- Crate-root module paths: the crate's public surface ----
pub mod schema {
    pub use super::standards::v_ascii::subsets::any::schema::*;
}
pub mod engine {
    pub use super::standards::v_ascii::engine::*;
}

pub use semio_s_artifact_stdio_stl_examples::examples;

#[path = "."]
pub mod editor {
    #[path = "."]
    pub mod stl {
        pub use semio_s_artifact_stdio_stl_examples::editor::stl::*;
        
        pub use semio_s_artifact_stdio_stl_examples::editor::stl::edit_rules;
        pub use semio_s_artifact_stdio_stl_examples::editor::stl::presence;
        pub use semio_s_artifact_stdio_stl_examples::editor::stl::config;
        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod edit {
                pub use semio_s_artifact_stdio_stl_inferences::editor::stl::modes::edit::*;
                
                pub use semio_s_artifact_stdio_stl_inferences::editor::stl::modes::edit::windows;
            }
        }
    }
}

#[path = "."]
pub mod viewer {
    #[path = "."]
    pub mod stl {
        pub use semio_s_artifact_stdio_stl_viewer::viewer::stl::*;
        
        pub use semio_s_artifact_stdio_stl_examples::viewer::stl::presence;
        pub use semio_s_artifact_stdio_stl_examples::viewer::stl::config;
        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod view {
                pub use semio_s_artifact_stdio_stl_inferences::viewer::stl::modes::view::*;
                
                pub use semio_s_artifact_stdio_stl_inferences::viewer::stl::modes::view::windows;
            }
        }
    }
}

pub use crate::standards::v_ascii::subsets::any::io::{StlBuilderConstruction, StlParts, StlAnalyzerAnalysis, StlBuilderFacets, StlBuilder, StlAnalyzer, StlComposer};

#[cfg(test)]
#[path = "🏅️standards/🔖️ascii/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🦀️.rs"]
mod hoisted_snapshot_0;
