//! 🪪️ Native document identity and paid lifecycle acceptance.
#[cfg(test)]
#[global_allocator]
static HEAP: semio_framework_trace::HeapWitness = semio_framework_trace::HeapWitness;
#[cfg(test)]
pub(crate) use semio_framework_plugin_instance_initialization::stores as original_instance_stores;
#[cfg(test)]
#[path = "🦀️.rs"]
pub(crate) mod fixture;
#[cfg(test)]
#[path = "../../🎟️registration/🧪️tests/🦀️.rs"]
mod identity;
