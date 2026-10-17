//! 🚫️ mut-any-8 of `semio-s-artifact-puzzle-5d` — a path-preserving carve of the artifact root mounts.

#![allow(clippy::result_large_err)]

extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_value_derive as value_derive;

pub use crate::standards::v1::subsets::any::schema::snapshot::Puzzle5dSnapshot;
pub use crate::standards::v1::subsets::any::schema::mutations::Puzzle5dMutation;
pub use semio_s_artifact_puzzle_5d_snapshot::{PUZZLE_5D_SCHEMA};
pub use semio_s_artifact_puzzle_5d_mut_any_5::{apply_puzzle5d_mutation};
pub use crate::viewer::puzzle5d::artifact_items_3::{PUZZLE5D_DIALECT};

#[path = "🪞️mirror/🦀️.rs"]
mod mirror;
pub use mirror::*;
