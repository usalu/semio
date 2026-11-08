//! 📬️ Batch publication and member emission of one one-item Store preparation.

#[cfg(test)]
#[path = "../📦️prepared/♻️retirement/🧪️tests/🦀️.rs"]
pub mod tests;
pub use semio_os_store_paged_one_item as paged_one_item;
#[path = "../🧩️composition/📨️emission/📦️owned/🦀️.rs"]
pub mod member_owned_batch;
pub use semio_os_store_ephemeral::{ephemeral_preparation, ephemeral_transfer};
#[path = "📥️lifted/📬️publication/🦀️.rs"]
pub mod lifted_one_item_publication;
#[path = "📥️lifted/🚦️native/🦀️.rs"]
pub mod lifted_native;
#[path = "📥️lifted/🤝️group/🦀️.rs"]
pub mod lifted_member_group;
#[cfg(test)]
#[global_allocator]
static TEST_ALLOCATION_OBSERVER: semio_framework_trace_memory_testing::RequestedAllocator = semio_framework_trace_memory_testing::RequestedAllocator;
