//! 🗿️ First-party artifact identity owner and explicitly imported physical text extensions.

#[path="../../../🧬️schema/🗿️artifact-reference/🦀️.rs"]
pub mod schema;
pub use schema::*;

const SCHEMA_EXPORTS:[semio_framework_schema_registry::SchemaExport;1]=[semio_framework_schema_registry::SchemaExport{id:"artifact-reference",leaves:semio_framework_schema_registry::FacetLeaves{rust:include_str!("../../../🧬️schema/🗿️artifact-reference/🦀️.rs"),typescript:include_str!("../../../🧬️schema/🗿️artifact-reference/🟦️.ts"),graphql:include_str!("../../../🧬️schema/🗿️artifact-reference/🔗️.graphql"),json_schema:include_str!("../../../🧬️schema/🗿️artifact-reference/🔣️.json"),proto:include_str!("../../../🧬️schema/🗿️artifact-reference/🛰️.proto")}}];

/// 📇️ Registers the canonical semantic reference contract for all native and artifact clients.
pub fn register_artifact_reference_schema_exports()->Result<(),semio_framework_schema_registry::SchemaExportRegistryError>{
 semio_framework_schema_registry::register_scope_schema_exports(semio_framework_schema_registry::ScopeSchemaExports{scope:"framework.schema",exports:&SCHEMA_EXPORTS})
}

#[path="."]
pub mod io {
 #[path="."]
 pub mod text {
  #[path="../../../🚪️io/📝️text/🗿️artifact-reference/🦀️.rs"]
  pub mod artifact_reference;
 }
}

#[cfg(test)]
#[path="../../🧪️tests/🦀️.rs"]
mod tests;

#[path="../../../🚪️io/📝️text/🗿️artifact-reference/🪆️binding/🦀️.rs"]
mod reference_binding;

#[cfg(test)]
#[path="../../../🚪️io/📝️text/🗿️artifact-reference/🪆️binding/🧪️tests/🏛️ownership/🦀️.rs"]
mod binding_owner_tests;
