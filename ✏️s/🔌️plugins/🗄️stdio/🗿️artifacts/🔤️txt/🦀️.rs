//! 🎪 `stdio.txt` artifact — stdio reference format.

#![allow(async_fn_in_trait)]
#![allow(long_running_const_eval)]

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_schema as framework_schema;
extern crate semio_framework_value_derive as value_derive;

pub use semio_s_artifact_stdio_contract::{apply_mutation, apply_mutation_checked, mutation_from_payload_json, mutation_inverse, mutation_payload_json, MutationRefusal};

pub use semio_s_artifact_stdio_txt_tests::TxtApps;

pub use schema::diff::TxtDiff;
pub use schema::mutations::TxtMutation;
pub use schema::snapshot::TxtSnapshot;
pub use schema::TxtArtifact;

pub use semio_s_artifact_stdio_txt_schema::{STDIO_TXT_DOCUMENT_SCHEMA};

pub use semio_s_artifact_stdio_txt_examples::{TXT_ARTIFACT_SCHEMA_ID};

pub use semio_s_artifact_stdio_txt_tests::{ARTIFACT_DEFINITION_SCHEMA};

pub use semio_s_artifact_stdio_txt_tests::{artifact, artifact_kind, assembly, declaration, definition, native_codecs};

pub use semio_s_artifact_stdio_txt_tests::{formats};

pub use semio_s_artifact_stdio_txt_tests::{native_codec};

pub use semio_s_artifact_stdio_txt_tests::{contribution};

//#region 🚪️DerivedIoRegistry
pub use semio_s_artifact_stdio_txt_tests::io_registry;
//#endregion 🚪️DerivedIoRegistry

#[path = "."]
pub mod standards {
    #[path = "."]
    pub mod v_utf_8 {
        // 🌳️ Standard root (ticket 26/08/17/CLEAN-ARTIFACT-STANDARD-SUBSET-MECHANISM, W2-P
        // pilot): `standard() -> StandardDeclaration`, mounts subset `any` below.
        pub use semio_s_artifact_stdio_txt_tests::standards::v_utf_8::*;

        // 🐜️ `⚙️engine/` dissolved (ticket 26/08/12/ENGINELESS-ARTIFACTS-AND-APP-STATE-MACHINES):
        // `TxtEngine` (zero construction sites) deleted outright; `empty_txt_snapshot`/
        // `demo_txt_snapshot` moved to `subsets::any::schema`; register cluster + `io_registry`
        // moved to `subsets::any::io`; tests moved beside what they test in `subsets::any::
        // schema`. `register()` is one of stdio's 10 protected imperative plugin-root calls
        // (`crate::engine::register()` in `🗄️stdio/🦀️.rs`, reached via
        // this artifact's own top-level `pub mod engine` shim) — left callable at this exact
        // path via a pure re-export of `subsets::any::io::register` (itself unchanged).
        pub use semio_s_artifact_stdio_txt_tests::standards::v_utf_8::engine;
        #[path = "."]
        pub mod subsets {
            #[path = "."]
            pub mod any {
                // 🪆️ Subset root (ticket 26/08/17/CLEAN-ARTIFACT-STANDARD-SUBSET-MECHANISM,
                // W2-P pilot): `subset() -> SubsetDeclaration` — see `💾️binary`'s mirrored
                // mount above for the shared reasoning.
                pub use semio_s_artifact_stdio_txt_tests::standards::v_utf_8::subsets::any::*;

                #[path = "."]
                pub mod schema {
                    pub use semio_s_artifact_stdio_txt_schema::standards::v_utf_8::subsets::any::schema::*;

                    pub use semio_s_artifact_stdio_txt_inferences::standards::v_utf_8::subsets::any::schema::mutation_support;
                    pub use semio_s_artifact_stdio_txt_schema::standards::v_utf_8::subsets::any::schema::snapshot;
                    pub use semio_s_artifact_stdio_txt_inferences::standards::v_utf_8::subsets::any::schema::inferences;
                    pub use semio_s_artifact_stdio_txt_inferences::standards::v_utf_8::subsets::any::schema::diff;
                    pub use semio_s_artifact_stdio_txt_mutations::standards::v_utf_8::subsets::any::schema::mutations;
                }
                #[path = "."]
                pub mod io {
                    pub use semio_s_artifact_stdio_txt_tests::standards::v_utf_8::subsets::any::io::*;
pub mod binary {
    pub use semio_s_artifact_stdio_txt_tests::standards::v_utf_8::subsets::any::io::binary::*;
    pub use semio_s_artifact_stdio_txt_examples::standards::v_utf_8::subsets::any::io::binary::inferences;
}
pub mod sqlite {
    pub use semio_s_artifact_stdio_txt_examples::standards::v_utf_8::subsets::any::io::sqlite::*;
    pub use semio_s_artifact_stdio_txt_schema::standards::v_utf_8::subsets::any::io::sqlite::snapshot;
}
pub mod text {
    pub use semio_s_artifact_stdio_txt_tests::standards::v_utf_8::subsets::any::io::text::*;
    pub use semio_s_artifact_stdio_txt_examples::standards::v_utf_8::subsets::any::io::text::inferences;
}

                    pub use semio_s_artifact_stdio_txt_inferences::standards::v_utf_8::subsets::any::io::import;
                    pub use semio_s_artifact_stdio_txt_inferences::standards::v_utf_8::subsets::any::io::export;
                }
            }
        }
    }
}

// ---- Shims: keep pre-migration module paths resolving for external callers ----
pub mod schema {
    pub use super::standards::v_utf_8::subsets::any::schema::*;
}
pub mod engine {
    pub use super::standards::v_utf_8::engine::*;
}

pub use semio_s_artifact_stdio_txt_examples::examples;

#[path = "."]
pub mod editor {
    #[path = "."]
    pub mod txt {
        pub use semio_s_artifact_stdio_txt_tests::editor::txt::*;

        pub use semio_s_artifact_stdio_txt_examples::editor::txt::edit_rules;
        pub use semio_s_artifact_stdio_txt_examples::editor::txt::presence;
        pub use semio_s_artifact_stdio_txt_examples::editor::txt::config;
        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod edit {
                pub use semio_s_artifact_stdio_txt_inferences::editor::txt::modes::edit::*;

                pub use semio_s_artifact_stdio_txt_inferences::editor::txt::modes::edit::windows;
            }
        }
    }
}

#[path = "."]
pub mod viewer {
    #[path = "."]
    pub mod txt {
        pub use semio_s_artifact_stdio_txt_tests::viewer::txt::*;

        pub use semio_s_artifact_stdio_txt_examples::viewer::txt::presence;
        pub use semio_s_artifact_stdio_txt_examples::viewer::txt::config;
        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod view {
                pub use semio_s_artifact_stdio_txt_inferences::viewer::txt::modes::view::*;

                pub use semio_s_artifact_stdio_txt_inferences::viewer::txt::modes::view::windows;
            }
        }
    }
}

pub use crate::standards::v_utf_8::subsets::any::io::{TxtBuilderConstruction, TxtParts, TxtAnalyzerAnalysis, TxtBuilderFacets, TxtBuilder, TxtAnalyzer, TxtComposer};

#[cfg(test)]
#[path = "🏅️standards/🔖️utf-8/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🦀️.rs"]
mod hoisted_snapshot_0;
