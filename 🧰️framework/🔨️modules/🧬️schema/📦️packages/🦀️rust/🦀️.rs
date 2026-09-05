//! 📋️ Package glue for schema derivation and validation.

#[allow(unused_extern_crates)]
extern crate self as semio_framework_schema;

#[path = "../../⚛️component.rs"]
mod component;
#[path = "../../✅️validator.rs"]
mod validator;

pub use component::*;
pub use validator::*;
