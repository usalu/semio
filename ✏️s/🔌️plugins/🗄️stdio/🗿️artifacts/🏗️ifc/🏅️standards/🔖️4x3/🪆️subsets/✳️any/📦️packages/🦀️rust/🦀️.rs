//! 🏗️ IFC4x3 ADD2 declarations and the artifact-owned editor, assembled from the express, schema, evaluator, machine, templates, mvd, step, exchange and fem crates.
#![allow(async_fn_in_trait)]
extern crate semio_os_kernel as protocol;
extern crate semio_framework_value_derive as value_derive;

#[cfg(test)]
#[global_allocator]
static IFC_ORIGINAL_HEAP: semio_framework_trace::HeapWitness = semio_framework_trace::HeapWitness;

pub mod catalog {
    pub use semio_s_artifact_stdio_ifc4x3_schema::*;
}
#[path = "../../🧬️schema/👁️mvd/🧰️facade/🦀️.rs"]
pub mod mvd;
#[path = "../../🧬️schema/🏷️templates/🧰️facade/🦀️.rs"]
pub mod templates;
#[path = "../../🧬️schema/⚖️validation/🦀️.rs"]
pub mod validation;
#[path = "../../✏️editor/details.rs"]
pub mod details;
#[path = "../../✏️editor/🦀️.rs"]
pub mod editor;
#[path = "../../👁️viewer/🦀️.rs"]
pub mod viewer;
pub use semio_s_artifact_stdio_ifc4x3_step as io;
#[path = "../../🚪️io/🧰️facade/🦀️.rs"]
pub mod native_exchange;
