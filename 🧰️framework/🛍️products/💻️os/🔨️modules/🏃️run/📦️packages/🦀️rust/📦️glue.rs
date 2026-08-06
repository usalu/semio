//! 🏃️ Headless OS workflow runner (Shape V2 entry).
extern crate semio_framework_os_kernel as workflow;
extern crate semio_framework_os_kernel as dsl_core;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
#[path = "../../🦀️component.rs"]
mod run_lib;
pub use run_lib::*;
