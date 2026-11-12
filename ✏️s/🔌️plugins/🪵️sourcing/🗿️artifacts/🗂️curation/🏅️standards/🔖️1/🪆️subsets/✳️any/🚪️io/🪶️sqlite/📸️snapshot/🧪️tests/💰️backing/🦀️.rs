//! 🔬️ Curation test requests are read from the one testing allocator.
pub(super) fn observe<T>(operation:impl FnOnce()->T)->(T,usize){let(result,(requested,_))=semio_framework_value_testing_allocator::observe_retirement_allocations(operation);(result,requested)}
