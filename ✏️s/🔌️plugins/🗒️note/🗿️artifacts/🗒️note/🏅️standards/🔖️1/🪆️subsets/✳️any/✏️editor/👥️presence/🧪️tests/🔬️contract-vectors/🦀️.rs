use semio_framework_test_fixtures::fixture_text;

#[allow(unused_imports)]
use semio_framework_value_derive::{FromValue, ToValue};
#[allow(unused_imports)]
use serde::{Deserialize, Serialize};
#[allow(unused_imports)]
use pack::snapshot::ArtifactPack;
use crate::editor::note::presence::*;
use dsl::os_pack as pack;
use protocol::{ColdMutation, Mutation, OpBinary, OpText};

#[semio_framework_async_macros::async_test]
async fn note_presence_contract_vectors_match_the_json_oracle() {
    let vectors: serde_json::Value = serde_json::from_str(fixture_text!("../../../../✏️editor/👥️presence/🧪️tests/🧫️fixtures/🔁️mutation-contracts.json")).expect("neutral contract vectors");
    let base: NotePresence = semio_framework_pack_json::from_json_str(&vectors["base"].to_string(), semio_framework_pack_json::JsonMemberPolicy::Reject).expect("owned base decoder");
    assert_eq!(<NotePresenceMutation as Mutation<NotePresence>>::DESCRIPTORS.len(), vectors["cases"].as_array().expect("cases").len());
    for vector in vectors["cases"].as_array().expect("cases") {
        let mutation: NotePresenceMutation = semio_framework_pack_json::from_json_str(&vector["mutation"].to_string(), semio_framework_pack_json::JsonMemberPolicy::Reject).expect("owned operation decoder");
        assert_eq!(serde_json::from_str::<serde_json::Value>(&semio_framework_pack_json::to_json_string(&mutation)).expect("independent operation oracle"), vector["mutation"]);
        assert_eq!(mutation.descriptor().semantic_kind, vector["kind"].as_str().expect("semantic kind"));
        assert_eq!(NotePresenceMutation::parse_op(&mutation.print_op()).expect("operation text"), mutation);
        assert_eq!(NotePresenceMutation::decode_op(&mutation.encode_op().expect("operation binary")).expect("binary decode"), mutation);
        let outcome = mutation.diff(&base);
        assert!(outcome.messages().is_empty());
        let next = protocol::apply_diff_cold(outcome.diff(), &base).expect("apply diff");
        assert_eq!(serde_json::from_str::<serde_json::Value>(&semio_framework_pack_json::to_json_string(&next)).expect("independent state oracle"), vector["expected"]);
        let restored = mutation.inverse(&base).expect("valid retained mutation inverse fixture").into_iter().fold(next, |state, inverse| protocol::apply_diff_cold(inverse.diff(&state).diff(), &state).expect("apply inverse"));
        assert_eq!(restored, base);
        protocol::os_spr::protocol_laws::assert_mutation_inverse_sum_law(&mutation, &base).await;
    }
}
