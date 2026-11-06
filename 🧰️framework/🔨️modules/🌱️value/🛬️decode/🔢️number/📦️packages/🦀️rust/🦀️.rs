//! 🔢️ Bounded original decimal conversion owns its lexical and exact-rounding frontiers.
pub use semio_framework_value_control_contract::*;
#[path="../../🦀️.rs"]
mod number;
pub use number::OriginalFloatWord;
#[cfg(test)]
pub(crate) use semio_framework_value_testing_allocator::observe_retirement_allocations;
#[cfg(test)]
#[test]
fn numeric_laws_observe_a_real_allocation_and_release(){
 let (_,heap)=observe_retirement_allocations(||{let mut probe=Vec::<u8>::with_capacity(73);probe.resize(73,7);std::hint::black_box(&probe);drop(probe);});
 assert_eq!(heap,(73,73));
}
