//! 🌳️ Canonical IFC4x3 ADD2 EXPRESS program, released declaration metadata and owner indices: the immutable data every validation, query and editing crate reads.
pub mod ast;
pub mod metadata;
mod program;
#[path = "program-index.rs"]
pub mod program_index;
pub use program::{decode_program, ifc4x3_express_program};

pub const IFC4X3_NATIVE_EXPRESS_EXECUTION: bool = false;
pub const IFC4X3_NATIVE_EXPRESS_IMPLEMENTATION: bool = true;
pub const IFC4X3_RETAINED_EXPRESS_RECEIVING_IMPLEMENTATION: bool = true;
pub const IFC4X3_RETAINED_EXPRESS_IMPLEMENTATION: bool = false;
pub use semio_s_artifact_stdio_ifc_json_pack::JsonPackError;
#[cfg(test)]
#[path = "🧪️tests/🦀️.rs"]
mod laws;
