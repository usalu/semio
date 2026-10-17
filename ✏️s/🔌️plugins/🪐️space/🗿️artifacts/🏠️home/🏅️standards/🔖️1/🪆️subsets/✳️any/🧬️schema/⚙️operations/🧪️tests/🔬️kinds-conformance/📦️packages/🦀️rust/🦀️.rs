//! 🔬️ kinds-conformance of `semio-s-artifact-space-home` — a path-preserving carve of the artifact root mounts.

#![allow(async_fn_in_trait)]

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_value_derive as value_derive;

pub use crate::standards::v1::subsets::any::schema::snapshot::SHomeSnapshot;
pub use semio_s_artifact_space_home_inferences::{S_HOME_DOCUMENT_SCHEMA};
pub use crate::editor::home::artifact_items_2::{HOME_DIALECT};
pub use crate::editor::home::artifact_items_4::{declaration, definition};

#[path = "🪞️mirror/🦀️.rs"]
mod mirror;
pub use mirror::*;
