use super::*;

fn drain(mut owner: Box<dyn store::ErasedSnapshotRetirement>, items: usize, bytes: usize) -> usize {
    let mut released = 0;
    for _ in 0..10_000 {
        match owner.close_step(items, bytes).unwrap() {
            store::SnapshotRetirementStep::Pending { released_items, released_bytes } => {
                assert!(released_items <= items && released_bytes <= bytes);
                released += released_bytes;
            }
            store::SnapshotRetirementStep::Complete => {
                assert!(owner.terminal_is_empty());
                return released;
            }
            store::SnapshotRetirementStep::Blocked => panic!("unshared retirement blocked"),
        }
    }
    panic!("document retirement did not reach terminal emptiness");
}

#[test]
fn rewriting_window_config_document_retirement_respects_exact_grants() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/🔣️.json")).unwrap();
    for budget in fixture["budgets"].as_array().unwrap() {
        let items = budget["items"].as_u64().unwrap() as usize;
        let bytes = budget["bytes"].as_u64().unwrap() as usize;
        for row in fixture["snapshots"].as_array().unwrap() {
            let value: RewritingSnapshot = pack::from_json_str(&row["value"].to_string()).unwrap();
            let expected = row["bytes"].as_u64().unwrap() as usize;
            let root = std::sync::Arc::new(value.clone());
            let mut shared = store::retirement::shared_retirement(std::sync::Arc::clone(&root));
            assert!(matches!(shared.close_step(items, bytes).unwrap(), store::SnapshotRetirementStep::Blocked));
            drop(root);
            assert_eq!(drain(shared, items, bytes), expected);
            assert_eq!(drain(store::retirement::owned_retirement(value), items, bytes), expected);
        }
        for row in fixture["mutations"].as_array().unwrap() {
            let value: RewriteRuleMutation = pack::from_json_str(&row["value"].to_string()).unwrap();
            assert_eq!(drain(store::retirement::owned_retirement(value), items, bytes), row["bytes"].as_u64().unwrap() as usize);
        }
    }
    eprintln!("[DEBUG] Rewriting document roots and all seven mutations retired under exact item/byte grants, with retained-reader blocking");
}
