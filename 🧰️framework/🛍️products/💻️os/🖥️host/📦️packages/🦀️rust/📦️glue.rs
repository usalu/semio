//! 🖥️ Semio framework OS host — Shape V2 glue.
#![feature(linkage)]

extern crate semio_framework_os_kernel as store;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as vcs;
extern crate semio_framework_os_kernel as pack;
extern crate semio_framework_os_kernel as spr;

#[path = "../../../🦀️component.rs"]
mod host_core;
pub use host_core::*;
