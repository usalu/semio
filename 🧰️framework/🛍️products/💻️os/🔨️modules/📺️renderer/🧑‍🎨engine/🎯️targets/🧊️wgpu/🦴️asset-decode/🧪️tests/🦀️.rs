//! 🦴️ Unit tests of asset-decode retain an independent physical allocator witness.

#[global_allocator]
static ALLOCATOR:semio_framework_trace::HeapWitness=semio_framework_trace::HeapWitness;
