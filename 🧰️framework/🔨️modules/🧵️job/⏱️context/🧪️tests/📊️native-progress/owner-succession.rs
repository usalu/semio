//! 🔁️ Each owner retires one escrow row, so owners succeeding one another on one Source never fill the fixed table with unacknowledged tombstones.
use super::{caller,checkpoint,quantum};

#[test]
fn successive_owners_on_one_source_never_exhaust_the_escrow_table() {
    let mut caller = caller();
    for owner in 0..128 {
        let mut decoder = caller.admitted_decoder();
        checkpoint(&mut caller, &mut decoder);
        for _ in 0..4096 {
            if decoder.terminal_is_empty() {
                break;
            }
            caller.bare_turn(quantum(), |cx| cx.with_original_retirement(|cx| { decoder.close_on_source(cx)?; Ok(()) })).unwrap_or_else(|error| panic!("owner {owner}: {error:?}"));
        }
        assert!(decoder.terminal_is_empty(), "owner {owner} never retired");
    }
    caller.retire(quantum());
}
