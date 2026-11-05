//! 🧪️ The borrowed native input frontier preserves its source and refuses unfunded construction.
use super::*;
#[test]
fn constructor_quotes_native_owner_and_rejects_zero_input_extent() {
    let demand=JsonNativeInputCursor::<DslValue>::constructor_demand();
    assert!(demand.copy_bytes>=std::mem::size_of::<JsonNativeInputCursor<'_,DslValue>>());
    assert_eq!(demand.capacity_bytes,0);
    assert_eq!(demand.depth,1);
}
