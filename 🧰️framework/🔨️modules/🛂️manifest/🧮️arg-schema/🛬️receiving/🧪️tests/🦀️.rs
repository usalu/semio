//! 🛬️ Recursive input declarations retain real native receiving ports before staged preparation can publish them.
use super::*;
use semio_framework_value_native_owned::native_decoding::{NativeReceive, NativeReceiver, owned::tree::NativeValueSource};

#[test]
fn recursive_action_inputs_have_a_controlled_native_receiver() {
    let mut receiver = <Vec<ActionArgDef> as NativeReceive<dyn NativeValueSource>>::receiver();
    receiver.begin_close();
    assert!(receiver.terminal_is_empty());
}

#[test]
fn heap_witness_observes_actual_allocation_before_receiver_claims() {
    let (_, heap) = semio_framework_trace::observe_heap_allocations_on_this_thread(|| {
        let allocation = vec![std::hint::black_box(7u8); std::hint::black_box(73)];
        std::hint::black_box(&allocation);
        drop(allocation);
    });
    assert!(heap.requested_bytes >= 73 && heap.released_bytes >= 73);
}


#[path = "native-input-cases.rs"]
mod input_cases;
