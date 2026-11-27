//! 🌉️ The action-bridge and clipboard laws of the sourcing curation editor: every declared UI action reaches its command, mounted through the shared harness.
#![allow(unused_imports)]
extern crate semio_os_kernel as protocol;
extern crate semio_os_kernel as store;
pub use semio_s_artifact_sourcing_curation::*;
pub mod editor_unit_tests {
    pub(crate) mod context {
        pub use semio_s_artifact_sourcing_curation_harness_actions::*;
        pub use semio_s_artifact_sourcing_curation_harness_dispatch::*;
        pub use semio_s_artifact_sourcing_curation_harness_mount::*;
        pub use semio_s_artifact_sourcing_curation_harness_windows::*;
    }
}
#[path = "../../🌉️actions/🦀️.rs"]
mod action_tests;
#[path = "../../../📋️clipboard/🧪️tests/🔬️unit/🦀️.rs"]
mod editing_tests;
