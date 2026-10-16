//! 🎪 `stdio.binary` artifact — stdio reference format.

#![allow(async_fn_in_trait)]
#![allow(long_running_const_eval)]

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_schema as framework_schema;
extern crate semio_framework_value_derive as value_derive;

pub use semio_s_artifact_stdio_contract::{apply_mutation, apply_mutation_checked, mutation_from_payload_json, mutation_inverse, mutation_payload_json, MutationRefusal};

use semio_framework_dispatch_macros::dyn_enum_close;

use semio_framework_plugin::plugin_app_close_prelude::*;

dyn_enum_close! {
    pub enum BinaryApps: PluginApp {
        Editor(VcsArtifactApp<EditorApp<editor::binary::BinaryEditor>>),
        Viewer(VcsArtifactApp<ViewerApp<viewer::binary::BinaryViewer>>),
    }
}

pub use schema::diff::BinaryDiff;
pub use schema::mutations::BinaryMutation;
pub use schema::snapshot::BinarySnapshot;
pub use schema::BinaryArtifact;
/// 🧬️ The wire contract a native test host decodes `🥒️.feature` witnesses through and inverts them with
/// (`Mutation::from_payload_value`/`Mutation::inverse`), re-exported because such a host links this crate alone.

pub use semio_s_artifact_stdio_binary_snapshot::{STDIO_BINARY_DOCUMENT_SCHEMA};

pub use semio_s_artifact_stdio_binary_snapshot::{BINARY_ARTIFACT_SCHEMA_ID};

pub use semio_s_artifact_stdio_binary_snapshot::{ARTIFACT_DEFINITION_SCHEMA};

pub use semio_s_artifact_stdio_binary_snapshot::{definition};

pub use semio_s_artifact_stdio_binary_snapshot::{formats};

pub use semio_s_artifact_stdio_binary_snapshot::{native_codecs};

pub use semio_s_artifact_stdio_binary_io_registry::{contribution};

pub use semio_s_artifact_stdio_binary_io_registry::{artifact, artifact_kind, assembly, declaration};
//#region 🚪️DerivedIoRegistry
pub use semio_s_artifact_stdio_binary_io_registry::io_registry;
//#endregion 🚪️DerivedIoRegistry

#[path = "."]
pub mod standards {
    #[path = "."]
    pub mod v_raw {
        // 🌳️ Standard root (ticket 26/08/17/CLEAN-ARTIFACT-STANDARD-SUBSET-MECHANISM, W2-P
        // pilot): `standard() -> StandardDeclaration`, mounts subset `any` below.
        pub use semio_s_artifact_stdio_binary_io_registry::standards::v_raw::*;
        

        // 🐜️ `⚙️engine/` dissolved (ticket 26/08/12/ENGINELESS-ARTIFACTS-AND-APP-STATE-MACHINES):
        // codecs already lived beside `BinarySnapshot`'s `ArtifactDsl`/`ArtifactPack` impls in
        // `subsets::any::schema::snapshot` (untouched); `empty_binary_snapshot`/
        // `demo_binary_snapshot` moved to `subsets::any::schema`; `BinaryEngine` (zero
        // construction sites repo-wide) deleted outright; the register cluster + `io_registry`
        // moved to `subsets::any::io`; tests moved into `subsets::any::schema::inferences`.
        // `register()` is one of stdio's 10 protected imperative plugin-root calls
        // (`crate::engine::register()` in `🗄️stdio/🦀️.rs`, reached
        // via this artifact's own top-level `pub mod engine` shim below) — left callable at
        // this exact path via a pure re-export of `subsets::any::io::register` (itself
        // unchanged).
        pub use semio_s_artifact_stdio_binary_io_registry::standards::v_raw::engine;
        #[path = "."]
        pub mod subsets {
            #[path = "."]
            pub mod any {
                // 🪆️ Subset root (ticket 26/08/17/CLEAN-ARTIFACT-STANDARD-SUBSET-MECHANISM,
                // W2-P pilot): `subset() -> SubsetDeclaration`, assembles the schema/io/
                // viewer/editor/examples children mounted below (and `crate::editor::binary`/
                // `crate::viewer::binary`, mounted at the plugin's top-level `editor`/`viewer`
                // modules, not here — see that file's own doc comment).
                pub use semio_s_artifact_stdio_binary_io_registry::standards::v_raw::subsets::any::*;
                

                #[path = "."]
                pub mod schema {
                    pub use semio_s_artifact_stdio_binary_schema::standards::v_raw::subsets::any::schema::*;
                    
                    pub use semio_s_artifact_stdio_binary_snapshot::standards::v_raw::subsets::any::schema::snapshot;
                    pub use semio_s_artifact_stdio_binary_schema::standards::v_raw::subsets::any::schema::inferences;
                    pub use semio_s_artifact_stdio_binary_snapshot::standards::v_raw::subsets::any::schema::diff;
                    pub use semio_s_artifact_stdio_binary_mutations::standards::v_raw::subsets::any::schema::mutations;
                }
                #[path = "."]
                pub mod io {
                    pub use semio_s_artifact_stdio_binary_io_registry::standards::v_raw::subsets::any::io::*;
pub mod binary {
    pub use semio_s_artifact_stdio_binary_io_registry::standards::v_raw::subsets::any::io::binary::*;
    pub use semio_s_artifact_stdio_binary_snapshot::standards::v_raw::subsets::any::io::binary::inferences;
}
pub mod text {
    pub use semio_s_artifact_stdio_binary_io_registry::standards::v_raw::subsets::any::io::text::*;
    pub use semio_s_artifact_stdio_binary_snapshot::standards::v_raw::subsets::any::io::text::inferences;
}
                    
                    pub use semio_s_artifact_stdio_binary_snapshot::standards::v_raw::subsets::any::io::import;
                    pub use semio_s_artifact_stdio_binary_snapshot::standards::v_raw::subsets::any::io::export;
                }
            }
        }
    }
}

// ---- Shims: keep pre-migration module paths resolving for external callers ----
pub mod schema {
    pub use super::standards::v_raw::subsets::any::schema::*;
}
pub mod engine {
    pub use super::standards::v_raw::engine::*;
}

pub use semio_s_artifact_stdio_binary_snapshot::examples;

#[path = "."]
pub mod editor {
    #[path = "."]
    pub mod binary {
        pub use semio_s_artifact_stdio_binary_mutations::editor::binary::*;
        
        pub use semio_s_artifact_stdio_binary_snapshot::editor::binary::edit_rules;
        pub use semio_s_artifact_stdio_binary_snapshot::editor::binary::presence;
        pub use semio_s_artifact_stdio_binary_snapshot::editor::binary::config;
        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod edit {
                pub use semio_s_artifact_stdio_binary_mutations::editor::binary::modes::edit::*;
                
                pub use semio_s_artifact_stdio_binary_mutations::editor::binary::modes::edit::windows;
            }
        }
    }
}

#[path = "."]
pub mod viewer {
    #[path = "."]
    pub mod binary {
        pub use semio_s_artifact_stdio_binary_io_registry::viewer::binary::*;
        
        pub use semio_s_artifact_stdio_binary_snapshot::viewer::binary::presence;
        pub use semio_s_artifact_stdio_binary_snapshot::viewer::binary::config;
        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod view {
                pub use semio_s_artifact_stdio_binary_mutations::viewer::binary::modes::view::*;
                
                pub use semio_s_artifact_stdio_binary_mutations::viewer::binary::modes::view::windows;
            }
        }
    }
}

pub use crate::standards::v_raw::subsets::any::io::{BinaryBuilderConstruction, BinaryParts, BinaryAnalyzerAnalysis, BinaryBuilderFacets, BinaryBuilder, BinaryAnalyzer, BinaryComposer};

#[cfg(test)]
#[path = "🏅️standards/🔖️raw/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🦀️.rs"]
mod hoisted_snapshot_0;
