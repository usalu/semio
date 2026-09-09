use super::RewritingDiff;
use crate::standards::v1::subsets::any::schema::mutations::RewriteRuleMutation;
use crate::RewritingSnapshot;
use protocol::{Mutation, MutationDiff};

/// 🗂️ Each persisted map edit preserves null and removal as distinct operations.
#[semio_framework_async_macros::async_test]
async fn rewriting_map_ownership_transport_preserves_entry_presence() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("🔣️.json")).unwrap();
    for row in fixture["cases"].as_array().unwrap() {
        let mut current = RewritingSnapshot { parameter_bindings: pack::from_json_str(&row["before"].to_string()).unwrap(), ..Default::default() };
        for input in row["mutations"].as_array().unwrap() {
            let mutation: RewriteRuleMutation = pack::from_json_str(&input.to_string()).unwrap();
            let outcome = mutation.diff(&current);
            let direct = outcome.diff().apply(&current).unwrap();
            let encoded = pack::to_json_string(outcome.diff());
            let decoded: RewritingDiff = pack::from_json_str(&encoded).unwrap();
            let restored = decoded.apply(&current);
            assert_eq!(restored.as_ref(), Ok(&direct), "{}: persisted diff {}", row["name"], encoded);
            current = restored.unwrap();
        }
        let actual: serde_json::Value = serde_json::from_str(&pack::to_json_string(&current.parameter_bindings)).unwrap();
        assert_eq!(actual, row["after"], "{}", row["name"]);
    }
    println!("[DEBUG] Rewriting persisted map edits preserved explicit null entries and removals");
}

/// 🪢 Coalesced map edits retain successful sequential application over the original base.
#[semio_framework_async_macros::async_test]
async fn rewriting_map_ownership_absorb_preserves_sequential_application() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("🔣️.json")).unwrap();
    for row in fixture["cases"].as_array().unwrap() {
        let base = RewritingSnapshot { parameter_bindings: pack::from_json_str(&row["before"].to_string()).unwrap(), ..Default::default() };
        let mut current = base.clone();
        let mut combined = RewritingDiff::default();
        for input in row["mutations"].as_array().unwrap() {
            let mutation: RewriteRuleMutation = pack::from_json_str(&input.to_string()).unwrap();
            let outcome = mutation.diff(&current);
            current = outcome.diff().apply(&current).unwrap();
            combined.absorb(outcome.diff().clone());
        }
        assert_eq!(combined.apply(&base).as_ref(), Ok(&current), "{}", row["name"]);
    }
    println!("[DEBUG] Rewriting map-edit composition matched sequential application");
}
