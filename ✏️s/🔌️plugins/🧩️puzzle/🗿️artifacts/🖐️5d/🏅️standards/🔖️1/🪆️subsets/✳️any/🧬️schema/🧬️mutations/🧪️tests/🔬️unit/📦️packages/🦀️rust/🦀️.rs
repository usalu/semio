//! 🔬️ tests of `semio-s-artifact-puzzle-5d` — a path-preserving carve of the artifact root mounts.

#![allow(clippy::result_large_err)]

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_value_derive as value_derive;

pub use crate::standards::v1::subsets::any::schema::snapshot::Puzzle5dSnapshot;
pub use crate::standards::v1::subsets::any::schema::diff::Puzzle5dDiff;
pub use crate::standards::v1::subsets::any::schema::mutations::Puzzle5dMutation;
pub use semio_s_plugin_puzzle_jobs as puzzle_job;
pub use semio_s_plugin_puzzle_retained as retained_command;
#[cfg(test)]
pub use crate::editor::puzzle5d::snapshot::Puzzle5dPlaySnapshot;
pub use semio_s_artifact_puzzle_5d_snapshot::{PUZZLE_5D_SCHEMA};
pub use semio_s_artifact_puzzle_5d_mut_any_5::{apply_puzzle5d_mutation};
pub use semio_s_artifact_puzzle_5d_mut_any_8::{PUZZLE5D_DIALECT};

#[path = "🪞️mirror/🦀️.rs"]
mod mirror;
pub use mirror::*;
