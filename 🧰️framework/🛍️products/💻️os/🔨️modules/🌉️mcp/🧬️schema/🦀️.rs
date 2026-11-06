//! 🧬️ The `os.mcp` schema registry — the ONE place this crate declares a schema. [Design notes](README.md#source-notes-1).

use schemars::JsonSchema;
use schemars::schema_for;
use semio_framework_schema_registry::FacetLeaves;
use semio_framework_schema_registry::SchemaExport;
use semio_framework_schema_registry::ScopeSchemaExports;
use semio_framework_schema_registry::register_scope_schema_exports as register_exports;
use serde::Deserialize;
use serde::Serialize;
use semio_os_mcp_foundation::errors::{GatewayError, GatewayErrorCode};
use semio_os_mcp_foundation::reports::{ContextSummary, InvocationReport, InvocationStatus, JobState, JobStatus, PreparedActionReport, RevisionStamp, SearchHit};
use semio_os_mcp_wire::protocol::{CallToolResult, ContentBlock, Prompt, PromptArgument, PromptGetResult, PromptMessage, Resource, ResourceContent, ResourceTemplate, Tool};
use semio_os_mcp_wire::tool_shapes::normalize_boolean_subschemas;

#[path = "🧭️core-and-mutation-tool-schemas/🦀️.rs"]
mod core_and_mutation_tool_schemas;
pub use core_and_mutation_tool_schemas::*;

#[path = "🛡️untrusted-content-schemas/🦀️.rs"]
mod untrusted_content_schemas;
pub use untrusted_content_schemas::*;

#[path = "📄️artifact-tool-schemas/🦀️.rs"]
mod artifact_tool_schemas;
pub use artifact_tool_schemas::*;

#[path = "🖥️ui-and-catalog-tool-schemas/🦀️.rs"]
mod ui_and_catalog_tool_schemas;
pub use ui_and_catalog_tool_schemas::*;

#[path = "💡️inference-tool-schemas/🦀️.rs"]
mod inference_tool_schemas;
pub use inference_tool_schemas::*;

#[path = "📡️artifact-inference-wire/🦀️.rs"]
mod artifact_inference_wire;
pub use artifact_inference_wire::*;

#[path = "📚️schema-catalog/🦀️.rs"]
mod schema_catalog;
pub use schema_catalog::*;

#[path = "🧾️scope-exports-and-validation/🦀️.rs"]
mod scope_exports_and_validation;
pub use scope_exports_and_validation::*;

//#region 🧪️Tests
//#endregion 🧪️Tests
