//! 🧾️ Flow document: the persisted host snapshot, its mutation catalogue and every native representation codec (text, binary pack, SQLite).
extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as store;
pub use semio_framework_artifact_flow_flow_model::*;
use semio_framework_ui_locale::LocalizedLabel;
#[path = "../🧬️schema/📸️snapshot/🧾️host/🦀️.rs"]
pub mod snapshot;
pub use snapshot::*;
#[path = "../🌿️vcs/🦀️.rs"]
pub mod vcs;
pub use vcs::*;

#[cfg(test)]
#[path = "../🧪️tests/🧬️dsl-native-snapshot/🦀️.rs"]
mod dsl_native_snapshot_tests;
