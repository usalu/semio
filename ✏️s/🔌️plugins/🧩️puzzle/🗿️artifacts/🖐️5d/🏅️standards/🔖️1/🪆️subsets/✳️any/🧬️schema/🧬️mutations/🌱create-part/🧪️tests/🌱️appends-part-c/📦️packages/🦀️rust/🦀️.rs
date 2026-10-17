//! 🌱️ mut-any-5 of `semio-s-artifact-puzzle-5d` — a path-preserving carve of the artifact root mounts.

#![allow(clippy::result_large_err)]

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_os_kernel as vcs;
extern crate semio_framework_value_derive as value_derive;

pub use crate::standards::v1::subsets::any::schema::snapshot::Puzzle5dSnapshot;
pub use crate::standards::v1::subsets::any::schema::mutations::Puzzle5dMutation;
pub use crate::standards::v1::subsets::any::schema::mutations::artifact_items_6::{apply_puzzle5d_mutation};

#[path = "🪞️mirror/🦀️.rs"]
mod mirror;
pub use mirror::*;
