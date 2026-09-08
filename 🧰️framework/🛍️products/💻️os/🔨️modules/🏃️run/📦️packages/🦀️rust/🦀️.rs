//! 🏃️ Headless OS workflow runner (Shape V2 entry).
extern crate semio_framework_artifact_workflow_workflow as workflow;
extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
#[path = "../../🦀️.rs"]
mod run_lib;
pub use run_lib::*;
