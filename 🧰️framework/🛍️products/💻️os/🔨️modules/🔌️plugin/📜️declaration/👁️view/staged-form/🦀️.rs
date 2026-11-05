//! 📝️ Original source-owned ordinary controls share exact typed bindings across renderer families.
#[path="payload.rs"]
mod payload;
pub use payload::*;
#[path="control.rs"]
mod control;
pub use control::*;
#[path="status.rs"]
mod status;
pub use status::*;
#[path="body.rs"]
mod body;
pub use body::*;
#[path="binding.rs"]
mod binding;
pub use binding::*;
#[path="output.rs"]
mod output;
pub use output::*;

#[path = "binding-source.rs"]
mod binding_source;
pub use binding_source::*;

#[cfg(test)]
#[global_allocator]
static TEST_HEAP: semio_framework_trace::HeapWitness = semio_framework_trace::HeapWitness;

#[cfg(test)]
#[path="page-source-tests.rs"]
mod page_source_tests;

#[path="page-source.rs"]
mod page_source;
pub use page_source::*;
