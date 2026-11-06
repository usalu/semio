//! 🧪️ Original retained-command and view laws run in their own test owner.
#![cfg(test)]
#[global_allocator]
static TEST_ALLOCATION_OBSERVER:semio_framework_trace::HeapWitness=semio_framework_trace::HeapWitness;
#[path="♻️metadata/🦀️.rs"] mod metadata_laws;
#[path="📸️checkpoint/🦀️.rs"] mod checkpoint_laws;
