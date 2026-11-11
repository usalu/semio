//! 🫒️ Companion test crate tests-05 of the plugin SDK partition: hosts the test root `reactor_turn_unit_3` (part 3).
//!
//! 🧭️ The test sources stay where they are and are mounted here with `#[path]`; every name they use from the former facade root is spelled with the crate that owns it.

#[cfg(test)]
extern crate semio_os_kernel as store;
#[cfg(test)]
extern crate semio_os_kernel as protocol;
#[cfg(test)]
extern crate semio_os_kernel as pack;

#[cfg(test)]
#[path = "../../🪝️mounts/⚛️wp4-reactor/🧪️test-hosts/🔂️reactor-turn/🧱️unit-3/🦀️.rs"]
pub mod reactor_turn_unit_3;


#[cfg(test)]
#[test]
fn staged_receipt_laws_observe_a_real_heap_allocation_and_release(){
    let (_,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||{let mut value=Vec::<u8>::with_capacity(73);value.resize(73,1);std::hint::black_box(value.as_slice());drop(value);});
    assert_eq!(heap.requested_bytes,73);
    assert_eq!(heap.released_bytes,73);
}
