//! 🧪️ Verifies the mutation fixture leaf contract.
use super::*;
#[test]
fn direct_counter_leaf_contract() {
    super::super::super::assert_counter_leaf_descriptor::<AddCounterSequence>(include_str!("../../🧫️fixtures/🧬️mutation-laws/🧬️mutations/🔢️add-counter-sequence/🔣️.json"));
}
