//! 📋️ Package glue for schema derivation and validation.

#[allow(unused_extern_crates)]
extern crate self as semio_framework_schema;

#[path = "../../⚛️component/🦀️.rs"]
mod component;

pub use component::*;

#[path = "../../🦀️.rs"]
mod schema_facet;
pub use schema_facet::*;
pub use semio_framework_schema_validator::*;
