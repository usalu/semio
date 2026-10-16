//! 🎪 `stdio.md` artifact — stdio reference format.

#![allow(async_fn_in_trait)]
#![allow(long_running_const_eval)]

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_schema as framework_schema;
extern crate semio_framework_value_derive as value_derive;

pub use semio_s_artifact_stdio_contract::{apply_mutation, apply_mutation_checked, mutation_from_payload_json, mutation_inverse, mutation_payload_json, MutationRefusal};

pub use schema::diff::MdDiff;
pub use schema::mutations::MdMutation;
pub use schema::snapshot::MdSnapshot;
pub use schema::MdArtifact;

pub use semio_s_artifact_stdio_md_snapshot::{STDIO_MD_DOCUMENT_SCHEMA};

pub use semio_s_artifact_stdio_md_snapshot::{MD_ARTIFACT_SCHEMA_ID};

pub use semio_s_artifact_stdio_md_snapshot::{ARTIFACT_DEFINITION_SCHEMA};

pub use semio_s_artifact_stdio_md_tests::{definition};

pub use semio_s_artifact_stdio_md_snapshot::{formats};

pub use semio_s_artifact_stdio_md_mutations::{native_codec};

pub use semio_s_artifact_stdio_md_tests::{native_codecs};

pub use semio_s_artifact_stdio_md_tests::{contribution};

pub use semio_s_artifact_stdio_md_tests::{MD_DIALECT};

pub use semio_s_artifact_stdio_md_tests::{artifact_kind};

pub use semio_s_artifact_stdio_md_tests::{assembly, declaration, pilot_languages};

//#region 🚪️DerivedIoRegistry
pub use semio_s_artifact_stdio_md_tests::io_registry;
//#endregion 🚪️DerivedIoRegistry

#[path = "."]
pub mod standards {
    #[path = "."]
    pub mod v_commonmark {
        // 🐜️ `⚙️engine/` dissolved (ticket 26/08/12/ENGINELESS-ARTIFACTS-AND-APP-STATE-MACHINES):
        // `MdEngine` (zero construction sites) deleted outright; its orphaned `register()`/
        // `register_artifact_schema()`/`register_artifact_inferences()`/
        // `register_pilot_languages()` (zero callers, superseded by `md::declaration()`)
        // deleted outright too; `parse_markdown_blocks` + the block/inline parser moved to
        // `subsets::any::io::import::deserializers`; `render_markdown_blocks` + the
        // block/inline renderer moved to `subsets::any::io::export::serializers`;
        // `io_registry` moved to `subsets::any::io`; `empty_md_snapshot`/`demo_md_snapshot`
        // + tests moved to `subsets::any::schema`. md is NOT one of stdio's 10 protected
        // imperative plugin-root `engine::register()` calls, so no `engine` shim remains —
        // external callers (🔱️trinity's jack/rewrite, 📜️imperative) were repointed to the
        // new `subsets::any::io::{import::deserializers,export::serializers}` paths.
        #[path = "."]
        pub mod subsets {
            #[path = "."]
            pub mod any {
                #[path = "."]
                pub mod schema {
                    pub use semio_s_artifact_stdio_md_schema::standards::v_commonmark::subsets::any::schema::*;
                    
                    pub use semio_s_artifact_stdio_md_snapshot::standards::v_commonmark::subsets::any::schema::snapshot;
                    pub use semio_s_artifact_stdio_md_schema::standards::v_commonmark::subsets::any::schema::inferences;
                    pub use semio_s_artifact_stdio_md_diff::standards::v_commonmark::subsets::any::schema::diff;
                    pub use semio_s_artifact_stdio_md_mutations::standards::v_commonmark::subsets::any::schema::mutations;
                }
                #[path = "."]
                pub mod io {
                    pub use semio_s_artifact_stdio_md_tests::standards::v_commonmark::subsets::any::io::*;
pub mod sqlite {
    pub use semio_s_artifact_stdio_md_diff::standards::v_commonmark::subsets::any::io::sqlite::*;
    pub use semio_s_artifact_stdio_md_snapshot::standards::v_commonmark::subsets::any::io::sqlite::snapshot;
}
                    
                    #[path = "."]
                    pub mod import {
                        #[path = "."]
                        pub mod deserializers {
                            pub use semio_s_artifact_stdio_md_snapshot::standards::v_commonmark::subsets::any::io::import::deserializers::*;
                            
                            pub use semio_s_artifact_stdio_md_diff::standards::v_commonmark::subsets::any::io::import::deserializers::artifacts;
                        }
                    }
                    #[path = "."]
                    pub mod export {
                        #[path = "."]
                        pub mod serializers {
                            pub use semio_s_artifact_stdio_md_snapshot::standards::v_commonmark::subsets::any::io::export::serializers::*;
                            
                            pub use semio_s_artifact_stdio_md_diff::standards::v_commonmark::subsets::any::io::export::serializers::artifacts;
                        }
                    }
                }
            }
        }
    }
}

// ---- Shims: keep pre-migration module paths resolving for external callers ----
pub mod schema {
    pub use super::standards::v_commonmark::subsets::any::schema::*;
}

pub use semio_s_artifact_stdio_md_diff::examples;

#[path = "."]
pub mod editor {
    #[path = "."]
    pub mod md {
        pub use semio_s_artifact_stdio_md_tests::editor::md::*;
        
        pub use semio_s_artifact_stdio_md_tests::editor::md::edit_rules;
        pub use semio_s_artifact_stdio_md_diff::editor::md::presence;
        pub use semio_s_artifact_stdio_md_diff::editor::md::config;
        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod edit {
                pub use semio_s_artifact_stdio_md_diff::editor::md::modes::edit::*;
                
                pub use semio_s_artifact_stdio_md_diff::editor::md::modes::edit::windows;
            }
        }
    }
}

#[path = "."]
pub mod viewer {
    #[path = "."]
    pub mod md {
        pub use semio_s_artifact_stdio_md_tests::viewer::md::*;
        
        pub use semio_s_artifact_stdio_md_diff::viewer::md::presence;
        pub use semio_s_artifact_stdio_md_diff::viewer::md::config;
        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod view {
                pub use semio_s_artifact_stdio_md_diff::viewer::md::modes::view::*;
                
                pub use semio_s_artifact_stdio_md_diff::viewer::md::modes::view::windows;
            }
        }
    }
}

pub use crate::standards::v_commonmark::subsets::any::io::{MdBuilderConstruction, MdParts, MdAnalyzerAnalysis, MdBuilderFacets, MdBuilder, MdAnalyzer, MdComposer};

#[cfg(test)]
#[path = "🏅️standards/🔖️commonmark/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🦀️.rs"]
mod hoisted_snapshot_0;
