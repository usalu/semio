//! 🧬️ schema of `semio-s-artifact-stdio-json` — a path-preserving carve of the artifact root mounts.

#![allow(async_fn_in_trait)]
#![allow(long_running_const_eval)]

extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_schema as framework_schema;
extern crate semio_framework_value_derive as value_derive;

pub use schema::snapshot::JsonSnapshot;
pub mod schema {
    pub use super::standards::v_rfc8259::subsets::base::schema::*;
}
pub use crate::standards::v_rfc8259::subsets::i_json::schema::conformance::artifact_items::{STDIO_JSON_DOCUMENT_SCHEMA};

#[path = "🪞️mirror/🦀️.rs"]
mod mirror;
pub use mirror::*;
