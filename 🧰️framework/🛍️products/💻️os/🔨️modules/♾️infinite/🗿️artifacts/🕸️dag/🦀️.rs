//! 🕸️ Independently composable DAG data and mutation contracts.
extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
pub use semio_framework_os_kernel::{os_dsl, os_pack, os_spr, os_store};
#[path = "🧵️retained/🦀️.rs"]
mod retained;
pub use retained::DagSnapshotRetirementFactory;
#[path = "🧬️schema/📸️snapshot/🦀️.rs"]
pub mod snapshot;
pub use snapshot::*;
#[path = "🌿️vcs/🦀️.rs"]
pub mod vcs;
pub use vcs::*;
#[cfg(test)]
#[path = "🧪️tests/🕸️dag/🦀️.rs"]
mod package_tests;

/// 🎬️ Canonical generic DAG example owned by the framework artifact and shared with its editors.
pub const DAG_DEMO_TEXT: &str = include_str!("🖼️assets/🎬️demo/🗣️.dsl.semio");
