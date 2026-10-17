//! 🔖️ v1 of `semio-s-artifact-puzzle-5d` — a path-preserving carve of the artifact root mounts.

#![allow(clippy::result_large_err)]

extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_os_kernel as vcs;

pub use crate::standards::v1::subsets::any::schema::snapshot::Puzzle5dSnapshot;
pub use crate::standards::v1::subsets::any::schema::mutations::Puzzle5dMutation;
#[cfg(test)]
pub use semio_s_plugin_puzzle_retained as retained_command;
pub use semio_s_artifact_puzzle_5d_snapshot::{PUZZLE_5D_SCHEMA};
pub use semio_s_artifact_puzzle_5d_mut_any_8::{PUZZLE5D_DIALECT};
pub use crate::standards::v1::artifact_items_5::{ArtifactApps, artifact, definition, pilot_languages};

#[path = "🪞️mirror/🦀️.rs"]
mod mirror;
pub use mirror::*;
