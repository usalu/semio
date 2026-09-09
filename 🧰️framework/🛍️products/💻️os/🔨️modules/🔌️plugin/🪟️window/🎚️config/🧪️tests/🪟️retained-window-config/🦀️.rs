//! 🧪️ Retained reducers bind the exact captured window-config owner and version.

use super::*;

#[test]
fn retained_window_config_context_identity_binds_owner_generation_and_revision() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("🔣️.json")).unwrap();
    let mut digests = std::collections::BTreeSet::new();
    for row in fixture["cases"].as_array().unwrap() {
        let generation = row["generation"].as_u64().unwrap();
        let revision = [row["revisionByte"].as_u64().unwrap() as u8; 32];
        let snapshot = WindowConfigSnapshot {
            window_id: row["windowId"].as_str().unwrap().into(),
            window_kind_id: "graph",
            generation,
            revision,
            snapshot: Arc::new(crate::app::NoConfig {}),
        };
        assert_eq!(snapshot.generation(), generation);
        assert_eq!(snapshot.revision(), revision);
        let digest = crate::app::test_window_config_context_identity(Some(&snapshot));
        assert_eq!(digest, crate::app::test_window_config_context_identity(Some(&snapshot.clone())));
        assert_ne!(digest, crate::app::test_window_config_context_identity(None));
        digests.insert(digest);
    }
    assert_eq!(digests.len(), fixture["expectedUniqueContexts"].as_u64().unwrap() as usize);
    eprintln!("[DEBUG] retained window config binds concrete owner, generation, and revision");
}
