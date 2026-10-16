//! 🔬️ Record laws that drive the codec stack through the public `pack::record` paths: operation pages, intrinsic media, borrowed preflight and intrinsic boundaries.
use pack::record;
use pack::record::*;
use semio_framework_value::{DslValue, Number, ValueError, ValueRefusalKind};
use semio_framework_pack_error::PackRefusal;
use semio_framework_dsl_record::{FieldSpec, FieldValue, RecordLayout, RecordSpec, RecordSpecProducer, RecordValue, Shape, WireEdgeLabel, WireNode, WireValue};
use semio_framework_replication_codec::PackLimits;

pub(crate) use semio_framework_trace_memory_testing as test_allocation;
#[global_allocator]
static REQUESTED_ALLOCATOR: test_allocation::RequestedAllocator = test_allocation::RequestedAllocator;

#[path = "../../🌱️value/🧪️tests/📦️operation-pages/🦀️.rs"]
mod operation_page_tests;

mod intrinsic_media_tests {
    semio_framework_pack_intrinsic_media_laws::intrinsic_media_wire_laws!(
        include_str!("../../🌱️value/🧪️tests/🧫️fixtures/🧮️wire-materialization/🔣️.json"),
        include_str!("../../🌱️value/🧪️tests/🧫️fixtures/🎞️intrinsic-media/🔣️.json"),
        include_str!("../../🌱️value/🧪️tests/🧫️fixtures/🎞️intrinsic-media/🌲️depth/🔣️.json"));
}

#[path = "../../🌱️value/🧪️tests/🫳️preflight/🦀️.rs"]
mod borrowed_preflight_tests;

#[path = "../../🌱️value/🌳️intrinsic/🧪️tests/🦀️.rs"]
mod intrinsic_boundary_laws;
