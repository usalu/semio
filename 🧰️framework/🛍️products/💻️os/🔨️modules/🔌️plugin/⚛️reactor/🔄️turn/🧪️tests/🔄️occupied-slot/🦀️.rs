#[test]
fn captured_slots_follow_the_shared_round_robin_order() {
    let cases: Vec<serde_json::Value> = serde_json::from_str(include_str!("🔣️.json")).unwrap();
    for case in cases {
        let mut slots = semio_framework_async::LazySlots::<usize, 8>::new();
        for (index, value) in case["slots"].as_array().unwrap().iter().enumerate() {
            if value.as_bool().unwrap() {
                slots[index] = Some(index);
            }
        }
        let cursor = case["start"].as_u64().unwrap() as usize;
        let actual = slots.take_next_occupied_slot(cursor);
        let expected = case["selected"].as_u64().map(|n| (n as usize, n as usize));
        assert_eq!(actual, expected);
    }
}
