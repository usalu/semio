//! 🏗️ IFC4x3 ADD2 declarations and the artifact-owned editor.
#![allow(async_fn_in_trait)]
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_value_derive as value_derive;

#[cfg(test)]
#[global_allocator]
static IFC_ORIGINAL_HEAP: semio_framework_trace::HeapWitness = semio_framework_trace::HeapWitness;

#[path = "../../🧬️schema/📚️catalog/🦀️.rs"]
pub mod catalog;
#[path = "../../🧬️schema/👁️mvd/🦀️.rs"]
pub mod mvd;
#[path = "../../🧬️schema/🏷️templates/🦀️.rs"]
pub mod templates;
#[path = "../../🧬️schema/⚖️validation/🦀️.rs"]
pub mod validation;
#[path = "../../✏️editor/details.rs"]
pub mod details;
#[path = "../../✏️editor/🦀️.rs"]
pub mod editor;
#[path = "../../🚪️io/step.rs"]
pub mod io;

#[path = "../../🚪️io/native_exchange.rs"]
pub mod native_exchange;
