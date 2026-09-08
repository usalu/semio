
use super::*;
use protocol::{Mutation, MutationDiff, OpBinary, OpText};

#[test]
fn mutations_match_the_json_oracle_and_restore_the_base() {
    let vectors: serde_json::Value = serde_json::from_str(include_str!("../../🧪️fixtures/🔁️mutations.json")).unwrap();
    let base: RemodelingPresence = serde_json::from_value(vectors["base"].clone()).unwrap();
    for vector in vectors["cases"].as_array().unwrap() {
        let oracle: RemodelingPresenceMutation = serde_json::from_value(vector["mutation"].clone()).unwrap();
        let mutation: RemodelingPresenceMutation = protocol::os_pack::json::from_json_str(&vector["mutation"].to_string()).unwrap();
        assert_eq!(mutation, oracle);
        let expected: RemodelingPresence = serde_json::from_value(vector["expected"].clone()).unwrap();
        let next = mutation.diff(&base).diff().apply(&base).unwrap();
        assert_eq!(next, expected);
        let restored = mutation.inverse(&base).into_iter().fold(next, |state, inverse| inverse.diff(&state).diff().apply(&state).unwrap());
        assert_eq!(restored, base);
        assert_eq!(RemodelingPresenceMutation::parse_op(&mutation.print_op()).unwrap(), mutation);
        assert_eq!(RemodelingPresenceMutation::decode_op(&mutation.encode_op().unwrap()).unwrap(), mutation);
    }
}
