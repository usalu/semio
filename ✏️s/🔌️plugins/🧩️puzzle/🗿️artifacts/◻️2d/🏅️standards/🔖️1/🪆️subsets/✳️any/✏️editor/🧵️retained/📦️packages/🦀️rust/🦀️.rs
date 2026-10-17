//! 🧵️ retained of `semio-s-artifact-puzzle-2d` — a path-preserving carve of the artifact root mounts.

#![allow(clippy::result_large_err)]

extern crate semio_framework_os_kernel as protocol;

pub use semio_s_plugin_puzzle_retained as retained_command;
pub use crate::standards::v1::subsets::any::schema::snapshot::Puzzle2dSnapshot;
pub use semio_s_plugin_puzzle_jobs as puzzle_job;

#[path = "🪞️mirror/🦀️.rs"]
mod mirror;
pub use mirror::*;
