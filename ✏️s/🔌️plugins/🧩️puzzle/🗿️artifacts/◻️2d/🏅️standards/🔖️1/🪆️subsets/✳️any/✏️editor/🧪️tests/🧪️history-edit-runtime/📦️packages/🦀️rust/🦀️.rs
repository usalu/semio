//! 🧪️ history-edit-runtime-tests of `semio-s-artifact-puzzle-2d` — a path-preserving carve of the artifact root mounts.

#![allow(clippy::result_large_err)]

extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;

#[cfg(test)]
pub use semio_s_plugin_puzzle_jobs as puzzle_job;

#[path = "🪞️mirror/🦀️.rs"]
mod mirror;
pub use mirror::*;
