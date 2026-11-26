//! 📱️ The mounted-app laws of the sourcing curation editor: the document, config and manifest behaviour of one app instance, mounted through the shared harness.
#![allow(unused_imports)]
extern crate semio_os_kernel as protocol;
extern crate semio_os_kernel as store;
pub use semio_s_artifact_sourcing_curation::*;
#[path = "../../🧰️harness/🦀️.rs"]
pub mod harness;
pub mod editor_unit_tests {
    pub(crate) use crate::harness as context;
}
#[path = "../../🧬️app/🦀️.rs"]
mod app_tests;
