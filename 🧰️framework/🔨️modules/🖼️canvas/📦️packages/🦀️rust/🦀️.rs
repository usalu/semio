//! 🖼️ General Canvas defining package.
#[cfg(all(target_arch = "wasm32", not(target_env = "p2")))]
extern crate semio_framework_async as wasm_bindgen_futures;
#[path = "../../🦀️.rs"]
mod component;
pub use component::*;
#[cfg(test)]
#[path = "../../../⏱️trace/🧮️memory/🧪️testing/📥️requests/🦀️.rs"]
mod test_allocation;
#[cfg(test)]
#[global_allocator]
static TEST_ALLOCATOR: test_allocation::RequestedAllocator = test_allocation::RequestedAllocator;
