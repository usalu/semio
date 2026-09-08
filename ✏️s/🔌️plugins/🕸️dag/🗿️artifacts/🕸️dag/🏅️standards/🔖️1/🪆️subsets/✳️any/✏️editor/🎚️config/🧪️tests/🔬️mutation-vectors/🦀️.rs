
use super::*;
use protocol::{Mutation, MutationDiff, OpBinary, OpText};

#[test]
fn language_neutral_mutations_match_json_oracle_and_restore_base() {
    let vectors: serde_json::Value = serde_json::from_str(include_str!("../../🧪️fixtures/🔁️mutations.json")).unwrap();
    for vector in vectors.as_array().unwrap() {
        let base: DagConfig = dsl::json::from_json_str(&vector["base"].to_string()).unwrap();
        let mutation: DagConfigMutation = dsl::json::from_json_str(&vector["mutation"].to_string()).unwrap();
        let oracle: DagConfigMutation = serde_json::from_value(vector["mutation"].clone()).unwrap();
        assert_eq!(mutation, oracle);
        assert_eq!(mutation.descriptor().semantic_kind, vector["kind"].as_str().unwrap());
        let next = mutation.diff(&base).diff().apply(&base).unwrap();
        assert_eq!(serde_json::to_value(&next).unwrap(), vector["after"]);
        let encoded = mutation.encode_op().unwrap();
        assert_eq!(DagConfigMutation::decode_op(&encoded).unwrap(), mutation);
        assert_eq!(DagConfigMutation::parse_op(&mutation.print_op()).unwrap(), mutation);
        let mut restored = next;
        for inverse in mutation.inverse(&base) {
            restored = inverse.diff(&restored).diff().apply(&restored).unwrap();
        }
        assert_eq!(restored, base);
    }
}
