//! ⚙️ operations of `semio-s-artifact-trinity-rewriting` — a path-preserving carve of the artifact root mounts.

#![allow(clippy::unnecessary_wraps)]

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_os_kernel as vcs;
extern crate protocol as replication;
extern crate semio_framework_value_derive as value_derive;

#[path = "🪞️mirror/🦀️.rs"]
mod mirror;
pub use mirror::*;
