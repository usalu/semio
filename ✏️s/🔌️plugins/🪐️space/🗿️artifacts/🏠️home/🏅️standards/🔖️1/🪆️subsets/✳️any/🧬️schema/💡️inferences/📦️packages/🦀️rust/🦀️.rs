//! 💡️ inferences of `semio-s-artifact-space-home` — a path-preserving carve of the artifact root mounts.

#![allow(async_fn_in_trait)]

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_value_derive as value_derive;

pub use crate::standards::v1::subsets::any::schema::snapshot::SHomeSnapshot;
pub use crate::standards::v1::subsets::any::io::binary::snapshot::artifact_items::{S_HOME_DOCUMENT_SCHEMA};
pub use crate::standards::v1::subsets::any::io::binary::snapshot::artifact_items_3::{artifact_kind};

#[path = "🪞️mirror/🦀️.rs"]
mod mirror;
pub use mirror::*;
