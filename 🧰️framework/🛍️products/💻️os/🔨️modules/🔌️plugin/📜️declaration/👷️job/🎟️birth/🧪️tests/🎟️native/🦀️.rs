//! 🎟️ Actual shared producer law uses the current production library and independent physical witness.
#[global_allocator]
static ORIGINAL_RESERVED_HEAP:semio_framework_trace::HeapWitness=semio_framework_trace::HeapWitness;
#[test]
fn original_optional_reserved_producer_birth(){semio_framework_plugin::original_reserved_producer_laws::original_optional_producer_birth_and_complete_physical_return()}
