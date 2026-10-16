//! 💡️ inferences of `semio-s-artifact-imperative-procedure` — a path-preserving carve of the artifact root mounts.

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_schema as framework_schema;

pub use crate::schema::snapshot::ProcedureSnapshot;
pub use imperative_engine::Path;
pub use neural_engine::Dictionary;
pub use neural_engine::Registry;
pub use imperative_engine::Step;
pub use neural_engine::Value;
#[cfg(test)]
pub use crate::schema::diff::ProcedureDiff;
pub mod mutations {
    pub use crate::standards::v1::subsets::any::schema::mutations::*;
}
pub mod schema {
    pub use super::standards::v1::subsets::any::schema::*;
}
pub use semio_s_artifact_imperative_procedure_snapshot::{PROCEDURE_DIALECT, PROCEDURE_DOCUMENT_SCHEMA, PathRef, procedure_flow_child_handle, procedure_snapshot_naming, seed_dictionary};
pub use semio_s_artifact_imperative_procedure_schema::{ProcedureScene, procedure_derivable_scene, procedure_scene};
pub use crate::viewer::procedure::artifact_items_5::{procedure_child_restore_projection};

#[path = "🪞️mirror/🦀️.rs"]
mod mirror;
pub use mirror::*;
