//! 🧪️ Retained work reads the exact captured window transient generation.

use super::*;

#[test]
fn retained_window_input_preserves_owner_generation() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("🔣️.json")).unwrap();
    let mut identities = std::collections::BTreeSet::new();
    let mut digests = std::collections::BTreeSet::new();
    for row in fixture["cases"].as_array().unwrap() {
        let snapshot = WindowTransientSnapshot {
            window_id: row["windowId"].as_str().unwrap().into(),
            window_kind_id: if row["windowKindId"] == "canvas" { "canvas" } else { "world" },
            generation: row["generation"].as_u64().unwrap(),
            snapshot: Arc::new(crate::app::NoTransient {}),
        };
        assert_eq!(snapshot.generation(), row["generation"].as_u64().unwrap());
        let digest = crate::app::test_window_transient_context_identity(Some(&snapshot));
        assert_eq!(digest, crate::app::test_window_transient_context_identity(Some(&snapshot.clone())));
        assert_ne!(digest, crate::app::test_window_transient_context_identity(None));
        digests.insert(digest);
        identities.insert((snapshot.window_id().to_owned(), snapshot.window_kind_id().to_owned(), snapshot.generation()));
    }
    assert_eq!(identities.len(), fixture["expectedUniqueOwners"].as_u64().unwrap() as usize);
    assert_eq!(digests.len(), identities.len());
    eprintln!("[DEBUG] retained window input keeps distinct concrete owner and generation tuples");
}
