use super::{G, StepContext, unquoted_receiving_grant, unquoted_receiving_work};

#[test]
fn unquoted_decoder_leaves_lifetime_stock_for_other_owners() {
    let cases: serde_json::Value = serde_json::from_str(include_str!("🧫️fixtures/receiving-copy.json")).unwrap();
    for case in cases.as_array().unwrap() {
        let amount = |field: &str| usize::try_from(case[field].as_u64().unwrap()).unwrap();
        let whole = G {
            maximum_items: amount("issuedItems"),
            maximum_copy_bytes: amount("issued"),
            maximum_capacity_bytes: 65_536,
            maximum_release_bytes: 65_536,
            maximum_depth: 64,
        };
        let reserved = unquoted_receiving_grant(whole);
        assert_eq!(reserved.maximum_copy_bytes, amount("reserved"));
        assert_eq!(whole.maximum_copy_bytes - reserved.maximum_copy_bytes, amount("available"));
        assert_eq!(reserved.maximum_items, amount("reservedItems"));
        assert_eq!(whole.maximum_items - reserved.maximum_items, amount("availableItems"));
        assert_eq!(reserved.maximum_capacity_bytes, 32_768);
        assert_eq!(reserved.maximum_release_bytes, 32_768);
        assert_eq!(reserved.maximum_depth, 64);
    }
}

#[test]
fn unquoted_decoder_leaves_work_and_release_for_other_owners() {
    let cases: serde_json::Value = serde_json::from_str(include_str!("🧫️fixtures/receiving-work-release.json")).unwrap();
    for case in cases.as_array().unwrap() {
        let amount = |field: &str| case[field].as_u64().unwrap();
        let whole = G {
            maximum_items: 0,
            maximum_copy_bytes: 0,
            maximum_capacity_bytes: 0,
            maximum_release_bytes: usize::try_from(amount("issuedRelease")).unwrap(),
            maximum_depth: 64,
        };
        assert_eq!(unquoted_receiving_work(amount("issuedWork")), amount("reservedWork"));
        assert_eq!(unquoted_receiving_grant(whole).maximum_release_bytes as u64, amount("reservedRelease"));
    }
}

/// 🧱️ An exact receiving quote is indivisible in capacity and release: a held scope may never shrink it below those amounts, whatever stock it holds.
#[test]
fn an_exact_quote_floors_capacity_and_release_at_its_own_amounts() {
    let quote = G { maximum_items: 1, maximum_copy_bytes: 2368, maximum_capacity_bytes: 2384, maximum_release_bytes: 96, maximum_depth: 1 };
    let floor = StepContext::quote_floor(quote);
    assert_eq!((floor.maximum_capacity_bytes, floor.maximum_release_bytes), (2384, 96));
    assert_eq!((floor.maximum_items, floor.maximum_copy_bytes, floor.maximum_depth), (0, 0, 0));
}
