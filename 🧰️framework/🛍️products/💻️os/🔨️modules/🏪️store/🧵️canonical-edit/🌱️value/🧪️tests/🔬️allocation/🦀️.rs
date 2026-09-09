use super::*;

#[test]
fn shared_value_canonical_json_rejects_overcapacity_before_admitting_key_storage() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/🔣️.json")).unwrap();
    for case in fixture["reservationCases"].as_array().unwrap() {
        let requested = case["requested"].as_u64().unwrap() as usize;
        let allocated = case["allocated"].as_u64().unwrap() as usize;
        let budget = case["budget"].as_u64().unwrap() as usize * size_of::<Option<KeySlot>>();
        let mut called = false;
        let result = reserve_table_with(requested, budget, |value, actual_request| {
            called = true;
            assert_eq!(actual_request, requested);
            assert!(value.is_empty());
            value.try_reserve_exact(allocated)
        });
        assert_eq!(called, requested * size_of::<Option<KeySlot>>() <= budget);
        assert_eq!(result.is_ok(), case["accepted"].as_bool().unwrap());
        if let Ok(value) = result { assert!(value.is_empty()); assert!(value.capacity() * size_of::<Option<KeySlot>>() <= budget); }
    }
    println!("[DEBUG] canonical key admission rejects injected allocator overcapacity before retaining a numeric table");
}
