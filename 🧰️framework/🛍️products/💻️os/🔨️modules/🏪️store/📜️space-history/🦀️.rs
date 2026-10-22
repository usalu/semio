//! 📜️ Space-history ownership namespaces.

/// 🧪️ Library unit tests observe every physical request and release through the shared requested-byte observer.
#[cfg(test)]
#[global_allocator]
static SPACE_HISTORY_LIBRARY_HEAP:semio_framework_trace_memory_testing::RequestedAllocator=semio_framework_trace_memory_testing::RequestedAllocator;

#[path = "../📦️codec/🪶️snapshot-capability/🪆️workspace-receiving/🦀️.rs"]
pub mod sqlite_snapshot_workspace;
#[path = "📥️lifted/🚪️io/🦀️.rs"]
pub mod lifted_serialization;
#[path="🚪️io/🦀️.rs"]
pub mod io;
