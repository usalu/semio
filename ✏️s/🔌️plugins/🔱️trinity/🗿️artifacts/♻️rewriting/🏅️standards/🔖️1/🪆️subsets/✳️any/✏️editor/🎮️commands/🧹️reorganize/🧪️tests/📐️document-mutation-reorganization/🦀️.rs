//! 📐️ Reorganizing a rule changes its document layout through reversible mutations.

use super::reorganize;
use crate::{apply_rewrite_rule_mutation, inverse_rewrite_rule_mutation, RewritingSnapshot};

#[semio_framework_async_macros::async_test]
async fn reorganize_uses_document_mutations() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("🔣️.json")).unwrap();
    for case in fixture["cases"].as_array().unwrap() {
        let mut before = RewritingSnapshot::default();
        before.rule_layout = pack::from_json_str(&case["before"].to_string()).unwrap();
        let emitted = reorganize(&before);
        assert_eq!(emitted.artifact_mutations.len(), case["artifactMutationCount"].as_u64().unwrap() as usize);
        assert_eq!(emitted.config_mutations.len(), case["configMutationCount"].as_u64().unwrap() as usize);
        let mut after = before.clone();
        let mut inverse = Vec::new();
        for mutation in &emitted.artifact_mutations {
            inverse.push(inverse_rewrite_rule_mutation(&after, mutation));
            apply_rewrite_rule_mutation(&mut after, mutation).unwrap();
        }
        let mut oracle = case["before"].clone();
        oracle.as_object_mut().unwrap().clear();
        let actual: serde_json::Value = serde_json::from_str(&pack::to_json_string(&after.rule_layout)).unwrap();
        assert_eq!(actual, case["after"]);
        assert_eq!(actual, oracle);
        for group in inverse.iter().rev() {
            for mutation in group {
                apply_rewrite_rule_mutation(&mut after, mutation).unwrap();
            }
        }
        assert_eq!(after, before);
    }
}
