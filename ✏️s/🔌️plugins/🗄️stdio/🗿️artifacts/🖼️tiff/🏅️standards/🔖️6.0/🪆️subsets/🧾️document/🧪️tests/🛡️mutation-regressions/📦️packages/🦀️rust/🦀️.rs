//! 🛡️ mutation-regressions of `semio-s-artifact-stdio-tiff` — a path-preserving carve of the artifact root mounts.

#![allow(async_fn_in_trait)]
#![allow(long_running_const_eval)]

extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_value_derive as value_derive;

pub use schema::snapshot::TiffSnapshot;
pub use schema::diff::TiffDiff;
pub use schema::mutations::TiffMutation;
pub use semio_s_artifact_stdio_contract::apply_mutation;
pub use standards::v6_0::subsets::document::examples;
pub use semio_s_artifact_stdio_contract::apply_mutation_checked;
pub use semio_s_artifact_stdio_contract::mutation_from_payload_json;
pub use semio_s_artifact_stdio_contract::mutation_inverse;
pub use semio_s_artifact_stdio_contract::mutation_payload_json;
pub use semio_s_artifact_stdio_contract::MutationRefusal;
pub mod schema {
    pub use super::standards::v6_0::subsets::document::schema::*;
}
pub use semio_s_artifact_stdio_tiff_snapshot::{STDIO_TIFF_DOCUMENT_SCHEMA};
pub use crate::editor::tiff_any::artifact_items_11::{TIFF_ANY_DIALECT, TIFF_BASELINE_DIALECT};

#[path = "🪞️mirror/🦀️.rs"]
mod mirror;
pub use mirror::*;
