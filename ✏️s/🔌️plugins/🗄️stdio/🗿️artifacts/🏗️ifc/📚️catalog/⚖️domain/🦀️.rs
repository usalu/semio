//! ⚖️ Owned IFC4 graph access to the schema-neutral borrowed declaration core.
use super::{IfcCatalog,IfcTypeExpression};
#[path="core.rs"]mod core;
#[path="ifc4.rs"]mod values;
pub use core::*;
