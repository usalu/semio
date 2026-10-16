//! 🔺️ diff of `semio-s-artifact-stdio-md` — a path-preserving carve of the artifact root mounts.

#![allow(async_fn_in_trait)]
#![allow(long_running_const_eval)]

extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_schema as framework_schema;
extern crate semio_framework_value_derive as value_derive;

pub use schema::snapshot::MdSnapshot;
pub mod schema {
    pub use super::standards::v_commonmark::subsets::any::schema::*;
}
pub use semio_s_artifact_stdio_md_snapshot::{STDIO_MD_DOCUMENT_SCHEMA};

#[path = "🪞️mirror/🦀️.rs"]
mod mirror;
pub use mirror::*;
