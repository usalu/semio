//! 🧬️ The value schema export and its lossless spellings — wiring only. Domain lives at the owner `🦀️.rs` files.

use semio_framework_value_control_contract::{ValueError, ValueRefusalKind};
use semio_framework_value_intrinsic::{DslValue, FromValue, Number};

#[path = "../../🔗️graphql/🔢️int64/🦀️.rs"]
pub mod graphql_int64;
#[path = "../../🌳️intrinsic/🔣️json/🦀️.rs"]
pub mod intrinsic_json;

const VALUE_SCHEMA_EXPORTS: [semio_framework_schema_registry::SchemaExport; 1] = [semio_framework_schema_registry::SchemaExport { id: "schema", leaves: semio_framework_schema_registry::FacetLeaves { rust: "", typescript: include_str!("../../🟦️.ts"), graphql: include_str!("../../🔗️.graphql"), json_schema: include_str!("../../🔣️.json"), proto: include_str!("../../🛰️.proto") } }];

/// 📌️ Registers the framework value schema (`framework/value/schema.json`: `DslValue`, `Binary64`, `Binary64Transport`, the
/// integer carriers) as the `schema` export of the `framework.value` scope, so every contract that `$ref`s it resolves it.
// 🚫️async: E1 pure registration helper (no I/O) — see R9
pub fn register_value_schema_exports() -> Result<(), semio_framework_schema_registry::SchemaExportRegistryError> {
    semio_framework_schema_registry::register_scope_schema_exports(semio_framework_schema_registry::ScopeSchemaExports { scope: "framework.value", exports: &VALUE_SCHEMA_EXPORTS })
}
