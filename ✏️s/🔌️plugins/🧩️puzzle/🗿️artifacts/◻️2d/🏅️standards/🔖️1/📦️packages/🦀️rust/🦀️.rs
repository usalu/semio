//! 🔖️ v1 of `semio-s-artifact-puzzle-2d` — a path-preserving carve of the artifact root mounts.

#![allow(clippy::result_large_err)]

extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;

pub use semio_s_plugin_puzzle_retained as retained_command;
pub use semio_s_plugin_puzzle_jobs as puzzle_job;
pub use crate::standards::v1::subsets::any::schema::mutations::Puzzle2dMutation;
pub use crate::standards::v1::subsets::any::schema::snapshot::Puzzle2dSnapshot;
pub use crate::standards::v1::artifact_items_2::{ArtifactApps, artifact, definition, pilot_languages};

#[path = "🪞️mirror/🦀️.rs"]
mod mirror;
pub use mirror::*;
