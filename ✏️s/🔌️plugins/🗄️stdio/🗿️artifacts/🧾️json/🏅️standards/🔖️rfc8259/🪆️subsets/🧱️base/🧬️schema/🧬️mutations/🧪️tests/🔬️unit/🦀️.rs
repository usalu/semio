use super::*;
use protocol::SemanticMutation;
#[test]
fn aggregate_roster_is_exact() {
    assert_eq!(JsonMutation::kinds().len(), 5);
}

/// ⚖️ `mutation_inverse_sum_law`: for every leaf the inverse diffs sum to the negative forward diff.
#[semio_framework_async_macros::async_test]
async fn mutation_inverse_sum_law_holds_for_every_leaf() {
    use crate::schema::snapshot::{JsonMember, JsonValue};
    let member = |key: &str, value: JsonValue| JsonMember { key: key.into(), value };
    let base = JsonSnapshot {
        schema: "stdio.json".into(),
        value: JsonValue::Object {
            members: vec![
                member("title", JsonValue::String { value: "Old".into() }),
                member("tags", JsonValue::Array { items: vec![JsonValue::Bool { value: true }, JsonValue::Null] }),
                member("enabled", JsonValue::Bool { value: false }),
            ],
        },
    };
    let key = |name: &str| JsonPathSegment::Key(name.into());
    for mutation in [
        JsonMutation::SetMember(SetMemberPayload { path: Vec::new(), key: "title".into(), value: JsonValue::String { value: "New".into() }, index: None }),
        JsonMutation::SetMember(SetMemberPayload { path: Vec::new(), key: "fresh".into(), value: JsonValue::Null, index: Some(1) }),
        JsonMutation::SetMember(SetMemberPayload { path: Vec::new(), key: "title".into(), value: JsonValue::Null, index: None }),
        JsonMutation::RemoveMember(RemoveMemberPayload { path: Vec::new(), key: "tags".into() }),
        JsonMutation::InsertArrayElement(InsertArrayElementPayload { path: vec![key("tags")], index: 1, value: JsonValue::Bool { value: false } }),
        JsonMutation::RemoveArrayElement(RemoveArrayElementPayload { path: vec![key("tags")], index: 0 }),
        JsonMutation::SetScalar(SetScalarPayload { path: vec![key("enabled")], value: JsonValue::Bool { value: true } }),
    ] {
        protocol::protocol_laws::assert_mutation_inverse_sum_law(&mutation, &base).await;
    }
}
