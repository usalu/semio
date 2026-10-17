//! 🎪 `stdio.xml` artifact — stdio reference format.

#![allow(async_fn_in_trait)]
#![allow(long_running_const_eval)]

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_schema as framework_schema;
extern crate semio_framework_value_derive as value_derive;

pub use semio_s_artifact_stdio_contract::{apply_mutation, apply_mutation_checked, mutation_from_payload_json, mutation_inverse, mutation_payload_json, MutationRefusal};

pub(crate) use semio_s_artifact_stdio_contract::impl_serde_op_codec;

pub use schema::diff::XmlDiff;
pub use schema::mutations::XmlMutation;
pub use schema::snapshot::XmlSnapshot;
pub use schema::XmlArtifact;

pub use semio_s_artifact_stdio_xml_snapshot::{STDIO_XML_DOCUMENT_SCHEMA};

pub use semio_s_artifact_stdio_xml_inferences::{XML_ARTIFACT_SCHEMA_ID};

pub use semio_s_artifact_stdio_xml_inferences::{ARTIFACT_DEFINITION_SCHEMA};

pub use semio_s_artifact_stdio_xml_mutations::{definition};

pub use semio_s_artifact_stdio_xml_inferences::{formats};

pub use semio_s_artifact_stdio_xml_mutations::{native_codec};

pub use semio_s_artifact_stdio_xml_mutations::{native_codecs};

pub use semio_s_artifact_stdio_xml_schema_2::{contribution};

pub use semio_s_artifact_stdio_xml_inferences::{artifact_kind};

pub use semio_s_artifact_stdio_xml_schema_2::{assembly, declaration, pilot_languages, pilot_subset_validators};

//#region 🚪️DerivedIoRegistry
pub use semio_s_artifact_stdio_xml_schema_2::io_registry;
//#endregion 🚪️DerivedIoRegistry

#[path = "."]
pub mod standards {
    #[path = "."]
    pub mod v1_0 {
        #[path = "."]
        pub mod subsets {
            #[path = "."]
            pub mod base {
                // 🐜️ `⚙️engine/` dissolved (ticket 26/08/12/ENGINELESS-ARTIFACTS-AND-APP-STATE-MACHINES):
                // `XmlEngine` (zero construction sites) deleted outright; its orphaned
                // `register()`/`register_artifact_schema()`/`register_artifact_inferences()`/
                // `register_pilot_languages()` (zero callers, superseded by `xml::declaration()`)
                // deleted outright too; `io_registry` moved to `subsets::base::io`;
                // `empty_xml_snapshot`/`demo_xml_snapshot` + tests moved to `subsets::base::schema`.
                // xml has no dedicated codec of its own to move -- the real text codec
                // (`xml_document_from_text`/`xml_document_to_text`) already lives in
                // `subsets::base::schema::snapshot`, unmoved. xml is NOT one of stdio's 10
                // protected imperative plugin-root `engine::register()` calls, so no `engine`
                // shim remains — no external caller ever reached `xml::…::engine::` (confirmed
                // repo-wide).
                pub use semio_s_artifact_stdio_xml_inferences::standards::v1_0::subsets::base::examples;
                #[path = "."]
                pub mod schema {
                    pub use semio_s_artifact_stdio_xml_schema::standards::v1_0::subsets::base::schema::*;

                    pub use semio_s_artifact_stdio_xml_snapshot::standards::v1_0::subsets::base::schema::mutation_support;
                    pub use semio_s_artifact_stdio_xml_snapshot::standards::v1_0::subsets::base::schema::snapshot;
                    pub use semio_s_artifact_stdio_xml_inferences::standards::v1_0::subsets::base::schema::inferences;
                    pub use semio_s_artifact_stdio_xml_schema::standards::v1_0::subsets::base::schema::diff;
                    pub use semio_s_artifact_stdio_xml_mutations::standards::v1_0::subsets::base::schema::mutations;
                }
                #[path = "."]
                pub mod io {
                    pub use semio_s_artifact_stdio_xml_schema_2::standards::v1_0::subsets::base::io::*;
pub mod sqlite {
    pub use semio_s_artifact_stdio_xml_inferences::standards::v1_0::subsets::base::io::sqlite::*;
    pub use semio_s_artifact_stdio_xml_snapshot::standards::v1_0::subsets::base::io::sqlite::snapshot;
}
pub mod text {
    pub use semio_s_artifact_stdio_xml_schema_2::standards::v1_0::subsets::base::io::text::*;
    pub use semio_s_artifact_stdio_xml_inferences::standards::v1_0::subsets::base::io::text::inferences;
}

                    pub use semio_s_artifact_stdio_xml_schema::standards::v1_0::subsets::base::io::import;
                    pub use semio_s_artifact_stdio_xml_mutations::standards::v1_0::subsets::base::io::export;
                }
            }
            #[path = "."]
            pub mod valid {
                pub use semio_s_artifact_stdio_xml_schema_2::standards::v1_0::subsets::valid::schema;
                pub use semio_s_artifact_stdio_xml_schema_2::standards::v1_0::subsets::valid::io;
                #[path = "."]
                pub mod examples {
                    pub use semio_s_artifact_stdio_xml_inferences::standards::v1_0::subsets::valid::examples::demo;
                    pub use semio_s_artifact_stdio_xml_inferences::standards::v1_0::subsets::valid::examples::no_doctype;
                }
            }
        }
    }
}

// ---- Shims: keep pre-migration module paths resolving for external callers ----
pub mod schema {
    pub use super::standards::v1_0::subsets::base::schema::*;
}

pub use standards::v1_0::subsets::base::examples;

#[path = "."]
pub mod editor {
    #[path = "."]
    pub mod xml_any {
        pub use semio_s_artifact_stdio_xml_tests_2::editor::xml_any::*;

        pub use semio_s_artifact_stdio_xml_schema_2::editor::xml_any::edit_rules;
        pub use semio_s_artifact_stdio_xml_inferences::editor::xml_any::presence;
        pub use semio_s_artifact_stdio_xml_inferences::editor::xml_any::config;
        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod edit {
                pub use semio_s_artifact_stdio_xml_mutations::editor::xml_any::modes::edit::*;

                pub use semio_s_artifact_stdio_xml_mutations::editor::xml_any::modes::edit::windows;
            }
        }
    }
    #[path = "."]
    pub mod xml_valid {
        pub use semio_s_artifact_stdio_xml_tests_2::editor::xml_valid::*;

        pub use semio_s_artifact_stdio_xml_inferences::editor::xml_valid::edit_rules;
        pub use semio_s_artifact_stdio_xml_inferences::editor::xml_valid::presence;
        pub use semio_s_artifact_stdio_xml_inferences::editor::xml_valid::config;
        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod edit {
                pub use semio_s_artifact_stdio_xml_mutations::editor::xml_valid::modes::edit::*;

                pub use semio_s_artifact_stdio_xml_mutations::editor::xml_valid::modes::edit::windows;
            }
        }
    }
}

#[path = "."]
pub mod viewer {
    #[path = "."]
    pub mod xml_any {
        pub use semio_s_artifact_stdio_xml_schema_2::viewer::xml_any::*;

        pub use semio_s_artifact_stdio_xml_inferences::viewer::xml_any::presence;
        pub use semio_s_artifact_stdio_xml_inferences::viewer::xml_any::config;
        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod view {
                pub use semio_s_artifact_stdio_xml_mutations::viewer::xml_any::modes::view::*;

                pub use semio_s_artifact_stdio_xml_mutations::viewer::xml_any::modes::view::windows;
            }
        }
    }
    #[path = "."]
    pub mod xml_valid {
        pub use semio_s_artifact_stdio_xml_tests_2::viewer::xml_valid::*;

        pub use semio_s_artifact_stdio_xml_inferences::viewer::xml_valid::presence;
        pub use semio_s_artifact_stdio_xml_inferences::viewer::xml_valid::config;
        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod view {
                pub use semio_s_artifact_stdio_xml_mutations::viewer::xml_valid::modes::view::*;

                pub use semio_s_artifact_stdio_xml_mutations::viewer::xml_valid::modes::view::windows;
            }
        }
    }
}

pub use crate::standards::v1_0::subsets::base::io::{XmlBuilderConstruction, XmlParts, XmlAnalyzerAnalysis, XmlBuilderFacets, XmlBuilder, XmlAnalyzer, XmlComposer};

pub use crate::standards::v1_0::subsets::valid::io::{XmlValidBuilderConstruction, XmlValidAnalyzerAnalysis, XmlValidBuilderFacets, XmlValidBuilder, XmlValidAnalyzer, XmlValidComposer};

#[cfg(test)]
#[path = "🏅️standards/🔖️1.0/🪆️subsets/🧱️base/🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🦀️.rs"]
mod hoisted_snapshot_0;
