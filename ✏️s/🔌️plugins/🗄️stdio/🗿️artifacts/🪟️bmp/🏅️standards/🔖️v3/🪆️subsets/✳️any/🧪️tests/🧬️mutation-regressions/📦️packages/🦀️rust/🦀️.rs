//! 🧬️ mutation-regressions of `semio-s-artifact-stdio-bmp` — a path-preserving carve of the artifact root mounts.

#![allow(async_fn_in_trait)]

extern crate semio_os_kernel as protocol;
extern crate semio_os_kernel as store;

pub use schema::snapshot::BmpSnapshot;
#[cfg(test)]
pub use semio_s_artifact_stdio_contract::apply_mutation;
pub mod schema {
    pub use super::standards::v_v3::subsets::any::schema::*;
}
pub use semio_s_artifact_stdio_bmp_snapshot::{STDIO_BMP_DOCUMENT_SCHEMA};

#[path = "🪞️mirror/🦀️.rs"]
mod mirror;
pub use mirror::*;
