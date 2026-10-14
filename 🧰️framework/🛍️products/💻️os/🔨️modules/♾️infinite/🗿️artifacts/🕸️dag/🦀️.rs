//! 🕸️ Independently composable DAG data and mutation contracts.
#[cfg(all(target_arch="wasm32",not(target_env="p2")))]
extern crate semio_framework_async as wasm_bindgen_futures;
extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
pub use semio_framework_os_kernel::{os_dsl, os_pack, os_spr, os_store};
#[path = "🧵️retained/🦀️.rs"]
mod retained;
pub use retained::DagMemberStoreProducer;
#[path = "🖼️assets/🎬️demo/🦀️.rs"]
mod demo;
pub use demo::dag_demo_host_snapshot;
use semio_framework_os_infinite_directed_presentation::*;
#[path = "🌿️vcs/🦀️.rs"]
pub mod vcs;
pub use vcs::*;
#[cfg(test)]
#[path = "🧪️tests/🕸️dag/🦀️.rs"]
mod package_tests;

/// 🎬️ Canonical generic DAG example owned by the framework artifact and shared with its editors.
pub const DAG_DEMO_TEXT: &str = include_str!("🖼️assets/🎬️demo/🗣️.dsl.semio");

#[cfg(test)]
#[path = "./🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🦀️.rs"]
pub(crate) mod sqlite_snapshot_tests;


#[cfg(all(target_arch="wasm32",not(target_env="p2")))]
#[path="🌿️vcs/🕸️browser/🦀️.rs"]
mod browser;
