//! 🧪️ Unit laws of `semio-s-artifact-stdio-ply-inferences`, mounted from their places beside the inference sources and run as a companion crate, so the inferences crate's test closure stays inside the source gate.

extern crate semio_framework_os_kernel as protocol;

pub use semio_s_artifact_stdio_ply_inferences::{schema, standards, PlySnapshot, STDIO_PLY_DOCUMENT_SCHEMA};

#[cfg(test)]
#[path = "../../../../📦bounds/🧪️tests/🔬️unit/🦀️.rs"]
mod bounds_tests;

#[cfg(test)]
#[path = "../../🦀️.rs"]
mod inferences_tests;
