//! 👥️ Copy-on-write population overlays, receivers and admissions every retained EXPRESS machine reads and the validation callers publish.
pub use semio_s_artifact_stdio_ifc4x3_express::{ast, ifc4x3_express_program, metadata, program_index};
#[path = "../retained.rs"]
pub mod retained;
#[path = "../overlay.rs"]
pub mod overlay;
#[path = "../receive.rs"]
pub mod receive;
#[path = "../admission.rs"]
pub mod admission;

#[cfg(test)]
#[global_allocator]
static IFC_ORIGINAL_HEAP: semio_framework_trace::HeapWitness = semio_framework_trace::HeapWitness;

#[cfg(test)]
#[path = "../overlay-tests.rs"]
mod overlay_tests;
