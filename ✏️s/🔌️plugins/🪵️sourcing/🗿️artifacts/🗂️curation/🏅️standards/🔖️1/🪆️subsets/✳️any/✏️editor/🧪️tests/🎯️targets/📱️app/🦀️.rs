//! 📱️ The mounted-app laws of the sourcing curation editor: the document, config and manifest behaviour of one app instance, mounted through the shared harness.
#![allow(unused_imports)]
extern crate semio_os_kernel as protocol;
extern crate semio_os_kernel as store;
extern crate semio_os_kernel as pack;
pub use semio_s_artifact_sourcing_curation::*;
pub mod editor_unit_tests {
    pub(crate) mod context {
        pub use semio_s_artifact_sourcing_curation_harness_actions::*;
        pub use semio_s_artifact_sourcing_curation_harness_dispatch::*;
        pub use semio_s_artifact_sourcing_curation_harness_mount::*;
        pub use semio_s_artifact_sourcing_curation_harness_windows::*;
    }
}
#[path = "../../🧬️app/🦀️.rs"]
mod app_tests;
