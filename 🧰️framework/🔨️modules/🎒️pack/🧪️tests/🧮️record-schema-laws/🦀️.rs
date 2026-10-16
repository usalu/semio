//! 🔬️ Record schema laws that drive the canonical schema hash and its scratch accounting through the public `pack::record` paths.
use pack::record;
use semio_framework_value::{ValueError, ValueRefusalKind};
use semio_framework_dsl_record::{FieldSpec, RecordLayout, RecordSpec, RecordSpecProducer, Shape};

pub(crate) use semio_framework_trace_memory_testing as test_allocation;
#[global_allocator]
static REQUESTED_ALLOCATOR: test_allocation::RequestedAllocator = test_allocation::RequestedAllocator;

#[path = "../../🌱️value/🧪️tests/🔬️schema-hash/💰️storage/🦀️.rs"]
mod schema_storage_tests;
