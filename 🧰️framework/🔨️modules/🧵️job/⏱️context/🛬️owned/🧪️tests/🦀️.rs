use super::{G, unquoted_receiving_grant};

#[test]
fn unquoted_decoder_leaves_lifetime_copy_stock_for_other_owners() {
    let cases: serde_json::Value = serde_json::from_str(include_str!("🧫️fixtures/receiving-copy.json")).unwrap();
    for case in cases.as_array().unwrap() {
        let amount = |field: &str| usize::try_from(case[field].as_u64().unwrap()).unwrap();
        let whole = G {
            maximum_items: 64,
            maximum_copy_bytes: amount("issued"),
            maximum_capacity_bytes: 65_536,
            maximum_release_bytes: 65_536,
            maximum_depth: 64,
        };
        let reserved = unquoted_receiving_grant(whole);
        assert_eq!(reserved.maximum_copy_bytes, amount("reserved"));
        assert_eq!(whole.maximum_copy_bytes - reserved.maximum_copy_bytes, amount("available"));
        assert_eq!(reserved.maximum_items, 32);
        assert_eq!(reserved.maximum_capacity_bytes, 32_768);
        assert_eq!(reserved.maximum_release_bytes, 32_768);
        assert_eq!(reserved.maximum_depth, 64);
    }
}
