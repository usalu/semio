use super::*;

fn drain(mut retirement: Box<dyn ErasedSnapshotRetirement>, items: usize, bytes: usize) -> usize {
    let mut released = 0;
    for _ in 0..100_000 {
        match retirement.close_step(items, bytes).unwrap() {
            SnapshotRetirementStep::Pending { released_items, released_bytes } => {
                assert!(released_items <= items);
                assert!(released_bytes <= bytes);
                released += released_bytes;
            }
            SnapshotRetirementStep::Complete => {
                assert!(retirement.terminal_is_empty());
                return released;
            }
            SnapshotRetirementStep::Blocked => panic!("unshared fixture unexpectedly blocked"),
        }
    }
    panic!("bounded fixture retirement did not finish");
}

#[test]
fn owned_retirement_matches_neutral_exact_byte_grants() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/🔣️.json")).unwrap();
    assert_eq!(fixture["cases"].as_array().unwrap().len(), 9);
    for row in fixture["cases"].as_array().unwrap() {
        for budget in fixture["budgets"].as_array().unwrap() {
            let retirement = match row["kind"].as_str().unwrap() {
                "string" => owned_retirement(row["value"].as_str().unwrap().to_owned()),
                "strings" => owned_retirement(serde_json::from_value::<Vec<String>>(row["value"].clone()).unwrap()),
                "optionalString" => owned_retirement(serde_json::from_value::<Option<String>>(row["value"].clone()).unwrap()),
                "pair" => owned_retirement(serde_json::from_value::<(String, String)>(row["value"].clone()).unwrap()),
                "stringMap" => owned_retirement(serde_json::from_value::<std::collections::BTreeMap<String, String>>(row["value"].clone()).unwrap()),
                "value" => owned_retirement(crate::os_pack::json::from_json_str::<crate::DslValue>(&row["value"].to_string()).unwrap()),
                _ => panic!("unknown neutral case"),
            };
            assert_eq!(drain(retirement, budget["items"].as_u64().unwrap() as usize, budget["bytes"].as_u64().unwrap() as usize), row["bytes"].as_u64().unwrap() as usize, "{}", row["id"]);
        }
    }
}

#[test]
fn owned_retirement_rejects_false_terminal_and_preserves_shared_roots() {
    let root = Arc::new("owned".to_string());
    let mut shared = shared_retirement(Arc::clone(&root));
    assert!(matches!(shared.close_step(0, 0).unwrap(), SnapshotRetirementStep::Pending { released_items: 0, released_bytes: 0 }));
    assert!(matches!(shared.close_step(1, 1).unwrap(), SnapshotRetirementStep::Blocked));
    assert_eq!(Arc::strong_count(&root), 2);
    drop(root);
    assert_eq!(drain(shared, 1, 1), 5);
    let mut value = owned_retirement("zero".to_string());
    for _ in 0..4 {
        assert!(matches!(value.close_step(1, 0).unwrap(), SnapshotRetirementStep::Pending { released_bytes: 0, .. }));
    }
    assert!(!value.terminal_is_empty());
    assert_eq!(drain(value, 1, 2), 4);
    struct Hostile {
        mode: u8,
    }
    impl RetirementCursor for Hostile {
        fn close_step(&mut self, maximum_bytes: usize) -> RetirementStep {
            match self.mode {
                1 => RetirementStep::Bytes(maximum_bytes + 1),
                _ => RetirementStep::Complete,
            }
        }
        fn terminal_is_empty(&self) -> bool {
            self.mode == 2
        }
    }
    for mode in [0, 1] {
        let mut stack = CursorStack(ManuallyDrop::new(vec![Box::new(Hostile { mode })]));
        assert!(stack.step(1, 1).is_err());
        assert_eq!(stack.0.len(), 1);
        stack.0.pop();
        assert!(matches!(stack.step(1, 1).unwrap(), SnapshotRetirementStep::Complete));
    }
}
