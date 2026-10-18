//! 🎪 `stdio.html` artifact — new-format artifact (master plan "New format artifacts" table).

#![allow(async_fn_in_trait)]
#![allow(long_running_const_eval)]

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_schema as framework_schema;
extern crate semio_framework_value_derive as value_derive;

pub use semio_s_artifact_stdio_contract::{apply_mutation, apply_mutation_checked, mutation_from_payload_json, mutation_inverse, mutation_payload_json, MutationRefusal};

pub use semio_s_artifact_stdio_html_inferences::standards::v5::subsets::any::schema::diff::HtmlDiff;
pub use semio_s_artifact_stdio_html_mutations::standards::v5::subsets::any::schema::mutations::HtmlMutation;
pub use semio_s_artifact_stdio_html_snapshot::standards::v5::subsets::any::schema::snapshot::HtmlSnapshot;
pub use semio_s_artifact_stdio_html_schema::standards::v5::subsets::any::schema::HtmlArtifact;

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


pub use semio_s_artifact_stdio_html_inferences::examples;





pub use semio_s_artifact_stdio_html_io_registry::standards::v5::subsets::any::io::{HtmlBuilderConstruction, HtmlParts, HtmlAnalyzerAnalysis, HtmlBuilderFacets, HtmlBuilder, HtmlAnalyzer, HtmlComposer};

#[cfg(test)]
#[path = "🏅️standards/🔖️5/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🦀️.rs"]
mod hoisted_snapshot_0;
