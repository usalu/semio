//! 🔬️ host-admission-tests of `semio-s-artifact-puzzle-5d` — a path-preserving carve of the artifact root mounts.

#![allow(clippy::result_large_err)]

extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;

pub use crate::standards::v1::subsets::any::schema::snapshot::Puzzle5dSnapshot;
#[cfg(test)]
pub use semio_s_plugin_puzzle_jobs as puzzle_job;
pub use semio_s_artifact_puzzle_5d_snapshot::{PUZZLE_5D_SCHEMA};

#[path = "🪞️mirror/🦀️.rs"]
mod mirror;
pub use mirror::*;
