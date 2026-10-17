//! 📦️ opc of `semio-s-artifact-stdio-zip` — a path-preserving carve of the artifact root mounts.

#![allow(async_fn_in_trait)]
#![allow(long_running_const_eval)]

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_value_derive as value_derive;

pub use schema::snapshot::ZipSnapshot;
pub use semio_s_artifact_stdio_contract::apply_mutation;
pub use semio_s_artifact_stdio_contract::apply_mutation_checked;
pub use semio_s_artifact_stdio_contract::mutation_from_payload_json;
pub use semio_s_artifact_stdio_contract::mutation_inverse;
pub use semio_s_artifact_stdio_contract::mutation_payload_json;
pub use semio_s_artifact_stdio_contract::MutationRefusal;
#[cfg(test)]
pub use schema::diff::ZipDiff;
#[cfg(test)]
pub use schema::mutations::ZipMutation;
pub mod schema {
    pub use super::standards::v2_0::subsets::base::schema::*;
}
pub use semio_s_artifact_stdio_zip_inferences::{STDIO_ZIP_DOCUMENT_SCHEMA};

#[path = "🪞️mirror/🦀️.rs"]
mod mirror;
pub use mirror::*;
