//! 🧷️ Declared tool registration capture returns every original alias on each cancelled prefix.
#[cfg(test)]
#[global_allocator]
static HEAP: semio_framework_trace::HeapWitness = semio_framework_trace::HeapWitness;
#[cfg(test)]
pub(crate) use semio_plugin_instance_initialization::stores as original_instance_stores;
#[cfg(test)]
#[path = "../../../🌱️initialization/🧪️tests/🪟️native-window/🦀️.rs"]
pub(crate) mod fixture;
#[cfg(test)]
#[path = "../../📥️staging/🧱️components/🏛️declarations/🌱️capture/🧪️tests/🦀️.rs"]
mod capture;
