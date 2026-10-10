//! ♻️ Composes the actual operation and history fields retained by typed command jobs.

use crate::app::AppOperationContext;

semio_framework_value::artifact_retire_struct!(AppOperationContext{app_instance_id,parent_document_id,operation_id,generation,canonical_base_revision,retained,authoring_seed});

#[cfg(test)]
#[path="🧪️tests/🦀️.rs"]
mod tests;
