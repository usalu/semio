//! 🧭️ Native original path rows share only neutral control and canonical native prefix storage.
#[cfg(test)]
#[global_allocator]
static ORIGINAL_NATIVE_ROWS_HEAP:semio_framework_trace::HeapWitness=semio_framework_trace::HeapWitness;
#[path="../../🦀️.rs"]mod definition;
pub use definition::{NativePathSet,NativeModifiedSet,NativePathInput,NATIVE_IO_PATH_CAPACITY};
