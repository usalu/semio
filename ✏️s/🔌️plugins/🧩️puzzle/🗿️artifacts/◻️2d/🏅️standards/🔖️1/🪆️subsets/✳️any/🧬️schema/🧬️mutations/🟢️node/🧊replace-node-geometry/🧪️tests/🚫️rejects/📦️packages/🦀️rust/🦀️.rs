//! 🚫️ mut-any-6 of `semio-s-artifact-puzzle-2d` — a path-preserving carve of the artifact root mounts.

#![allow(clippy::result_large_err)]

extern crate semio_framework_os_kernel as protocol;

#[cfg(test)]
pub use crate::standards::v1::subsets::any::schema::snapshot::Puzzle2dSnapshot;

#[path = "🪞️mirror/🦀️.rs"]
mod mirror;
pub use mirror::*;
