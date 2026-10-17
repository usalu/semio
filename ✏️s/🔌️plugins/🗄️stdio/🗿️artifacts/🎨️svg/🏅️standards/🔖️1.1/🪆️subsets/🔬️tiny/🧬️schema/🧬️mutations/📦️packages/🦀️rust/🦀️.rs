//! 🧬️ mutations-2 of `semio-s-artifact-stdio-svg` — a path-preserving carve of the artifact root mounts.

#![allow(async_fn_in_trait)]
#![allow(long_running_const_eval)]

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_value_derive as value_derive;

pub use schema::snapshot::SvgSnapshot;
pub use schema::diff::SvgDiff;
pub use schema::mutations::SvgMutation;
pub(crate) use semio_s_artifact_stdio_contract::impl_serde_op_codec;
pub mod schema {
    pub use super::standards::v1_1::subsets::base::schema::*;
}
pub use semio_s_artifact_stdio_svg_snapshot::{STDIO_SVG_DOCUMENT_SCHEMA};

#[path = "🪞️mirror/🦀️.rs"]
mod mirror;
pub use mirror::*;
