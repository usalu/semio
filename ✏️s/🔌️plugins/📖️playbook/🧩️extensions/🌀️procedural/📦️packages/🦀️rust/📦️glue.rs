//! 📦️ Package glue — wiring only. Domain lives at owner 🦀️component.rs.

extern crate flow_core;
extern crate semio_framework_os_kernel as vcs;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as dsl;
#[path = "../../🦀️component.rs"]
mod component;
pub use component::*;
