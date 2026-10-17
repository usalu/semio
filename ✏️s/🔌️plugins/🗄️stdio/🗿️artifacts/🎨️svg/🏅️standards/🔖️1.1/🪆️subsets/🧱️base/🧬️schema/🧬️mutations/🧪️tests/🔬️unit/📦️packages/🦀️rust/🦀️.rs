//! 🔬️ tests of `semio-s-artifact-stdio-svg` — a path-preserving carve of the artifact root mounts.

#![allow(async_fn_in_trait)]
#![allow(long_running_const_eval)]

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_value_derive as value_derive;

pub use schema::snapshot::SvgSnapshot;
pub mod schema {
    pub use super::standards::v1_1::subsets::base::schema::*;
}

#[path = "🪞️mirror/🦀️.rs"]
mod mirror;
pub use mirror::*;
