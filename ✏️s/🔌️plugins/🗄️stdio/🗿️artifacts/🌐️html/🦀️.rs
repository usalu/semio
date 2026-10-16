//! 🎪 `stdio.html` artifact — new-format artifact (master plan "New format artifacts" table).

#![allow(async_fn_in_trait)]
#![allow(long_running_const_eval)]

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_schema as framework_schema;
extern crate semio_framework_value_derive as value_derive;

pub use semio_s_artifact_stdio_contract::{apply_mutation, apply_mutation_checked, mutation_from_payload_json, mutation_inverse, mutation_payload_json, MutationRefusal};

pub use standards::v5::subsets::any::schema::diff::HtmlDiff;
pub use standards::v5::subsets::any::schema::mutations::HtmlMutation;
pub use standards::v5::subsets::any::schema::snapshot::HtmlSnapshot;
pub use standards::v5::subsets::any::schema::HtmlArtifact;

pub use semio_s_artifact_stdio_html_mutations::{STDIO_HTML_DOCUMENT_SCHEMA};

pub use semio_s_artifact_stdio_html_schema::{HTML_ARTIFACT_SCHEMA_ID};

pub use semio_s_artifact_stdio_html_mutations::{ARTIFACT_DEFINITION_SCHEMA};

pub use semio_s_artifact_stdio_html_mutations::{artifact_kind, assembly, declaration, definition, native_codecs};

pub use semio_s_artifact_stdio_html_mutations::{formats};

pub use semio_s_artifact_stdio_html_mutations::{native_codec};

pub use semio_s_artifact_stdio_html_mutations::{contribution};

pub use semio_s_artifact_stdio_html_mutations::{HTML_DIALECT};

//#region 🚪️DerivedIoRegistry
pub use semio_s_artifact_stdio_html_io_registry::io_registry;
//#endregion 🚪️DerivedIoRegistry

#[path = "."]
pub mod standards {
    #[path = "."]
    pub mod v5 {
        // 🐜️ `⚙️engine/` dissolved (ticket 26/08/12/ENGINELESS-ARTIFACTS-AND-APP-STATE-MACHINES):
        // `sniff_real_bytes` moved to `subsets::any::io::import::deserializers`, `io_registry`
        // moved to `subsets::any::io::io_registry`. `register()` is one of stdio's 10 protected
        // imperative plugin-root calls (`crate::standards::v5::engine::
        // register()` in `🗄️stdio/🦀️.rs`) — left callable at this exact path via a
        // pure re-export of `subsets::any::io::register` (itself unchanged).
        pub use semio_s_artifact_stdio_html_io_registry::standards::v5::engine;
        #[path = "."]
        pub mod subsets {
            #[path = "."]
            pub mod any {
                pub mod io {
    pub use semio_s_artifact_stdio_html_io_registry::standards::v5::subsets::any::io::*;
    pub mod binary {
        pub use semio_s_artifact_stdio_html_schema::standards::v5::subsets::any::io::binary::*;
        pub use semio_s_artifact_stdio_html_inferences::standards::v5::subsets::any::io::binary::diff;
        pub use semio_s_artifact_stdio_html_inferences::standards::v5::subsets::any::io::binary::inferences;
        pub use semio_s_artifact_stdio_html_mutations::standards::v5::subsets::any::io::binary::mutations;
        pub use semio_s_artifact_stdio_html_snapshot::standards::v5::subsets::any::io::binary::snapshot;
    }
    pub mod sqlite {
        pub use semio_s_artifact_stdio_html_inferences::standards::v5::subsets::any::io::sqlite::*;
        pub use semio_s_artifact_stdio_html_snapshot::standards::v5::subsets::any::io::sqlite::snapshot;
    }
}
                #[path = "."]
                pub mod schema {
                    pub use semio_s_artifact_stdio_html_schema::standards::v5::subsets::any::schema::*;
                    
                    pub use semio_s_artifact_stdio_html_snapshot::standards::v5::subsets::any::schema::snapshot;
                    pub use semio_s_artifact_stdio_html_inferences::standards::v5::subsets::any::schema::inferences;
                    pub use semio_s_artifact_stdio_html_inferences::standards::v5::subsets::any::schema::diff;
                    pub use semio_s_artifact_stdio_html_mutations::standards::v5::subsets::any::schema::mutations;
                }
            }
        }
    }
}
pub use semio_s_artifact_stdio_html_inferences::examples;

#[path = "."]
pub mod editor {
    #[path = "."]
    pub mod html {
        pub use semio_s_artifact_stdio_html_mutations::editor::html::*;
        
        pub use semio_s_artifact_stdio_html_mutations::editor::html::edit_rules;
        pub use semio_s_artifact_stdio_html_inferences::editor::html::presence;
        pub use semio_s_artifact_stdio_html_inferences::editor::html::config;
        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod edit {
                pub use semio_s_artifact_stdio_html_mutations::editor::html::modes::edit::*;
                
                pub use semio_s_artifact_stdio_html_inferences::editor::html::modes::edit::windows;
            }
        }
    }
}

#[path = "."]
pub mod viewer {
    #[path = "."]
    pub mod html {
        pub use semio_s_artifact_stdio_html_mutations::viewer::html::*;
        
        pub use semio_s_artifact_stdio_html_inferences::viewer::html::presence;
        pub use semio_s_artifact_stdio_html_inferences::viewer::html::config;
        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod view {
                pub use semio_s_artifact_stdio_html_mutations::viewer::html::modes::view::*;
                
                pub use semio_s_artifact_stdio_html_inferences::viewer::html::modes::view::windows;
            }
        }
    }
}

pub use crate::standards::v5::subsets::any::io::{HtmlBuilderConstruction, HtmlParts, HtmlAnalyzerAnalysis, HtmlBuilderFacets, HtmlBuilder, HtmlAnalyzer, HtmlComposer};

#[cfg(test)]
#[path = "🏅️standards/🔖️5/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🦀️.rs"]
mod hoisted_snapshot_0;
