//! 🚪️ Exercises the original command's production retirement provider at its public boundary.
pub use semio_framework_os_kernel::os_store;

#[global_allocator]
static ORIGINAL_OWNER_HEAP: semio_framework_trace::HeapWitness = semio_framework_trace::HeapWitness;

#[path = "../🦀️.rs"]
mod command;

#[path="../../../../🧩️composition/📬️publication/🤝️group/🔏️digest/🧪️tests/🦀️.rs"]
mod group_digest;

#[cfg(feature="sync")]
#[path="../../../../🔄️sync/🪪️actor-identity/🧪️tests/🦀️.rs"]
mod actor_identity;
