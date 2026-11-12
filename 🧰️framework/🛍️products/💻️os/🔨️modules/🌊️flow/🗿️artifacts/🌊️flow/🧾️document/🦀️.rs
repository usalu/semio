//! 🧾️ Flow document: the persisted host snapshot, its mutation catalogue and every native representation codec (text, binary pack, SQLite).
extern crate semio_os_kernel as dsl;
extern crate semio_os_kernel as store;
pub use semio_framework_artifact_flow_flow_model::*;
use semio_ui_locale::LocalizedLabel;
#[path = "../🧬️schema/📸️snapshot/🧾️host/🦀️.rs"]
pub mod snapshot;
pub use snapshot::*;
#[path = "../🌿️vcs/🦀️.rs"]
pub mod vcs;
pub use vcs::*;

#[cfg(test)]
#[path = "../🧪️tests/🧬️dsl-native-snapshot/🦀️.rs"]
mod dsl_native_snapshot_tests;

#[cfg(test)]
use semio_framework_value_testing_allocator as _;

#[cfg(test)]
#[path="../🧬️schema/📸️snapshot/🧪️tests/🧬️retained-clone/🦀️.rs"]
mod retained_snapshot_clone_laws;
