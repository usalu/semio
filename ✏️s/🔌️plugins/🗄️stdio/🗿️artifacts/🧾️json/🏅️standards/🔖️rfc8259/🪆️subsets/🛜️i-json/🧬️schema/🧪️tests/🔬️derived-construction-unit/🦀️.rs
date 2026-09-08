mod tests {
    use super::*;

    #[semio_framework_async_macros::async_test]
    async fn conforming_snapshot_builds_clean() {
        let snapshot = JsonIJsonBuilderConstruction::from_text("{\"a\":1}").expect("parses").build().expect("conforming construction must build");
        assert!(matches!(snapshot.value, crate::standards::v_rfc8259::subsets::base::schema::snapshot::JsonValue::Object { .. }));
    }

    #[semio_framework_async_macros::async_test]
    async fn duplicate_member_name_fails_build() {
        let err = JsonIJsonBuilderConstruction::from_text("{\"a\":1,\"a\":2}").expect("parses").build().expect_err("a duplicate member name must fail build()");
        assert!(err.iter().any(|d| d.code.0 == "stdio.json.i-json.duplicate-member-name"));
    }

    #[semio_framework_async_macros::async_test]
    async fn unsafe_integer_injected_via_raw_mutate_still_fails_build() {
        use crate::standards::v_rfc8259::subsets::base::schema::snapshot::{JsonMember, JsonValue};
        let violating = JsonSnapshot { value: JsonValue::Object { members: vec![JsonMember { key: "n".into(), value: JsonValue::Number { lexeme: "9007199254740993".into() } }] }, ..JsonSnapshot::default() };
        let (mutated, _diff) =
            JsonIJsonBuilderConstruction::from_snapshot(JsonSnapshot::default()).mutate(JsonIJsonMutation::SetSnapshot(crate::standards::v_rfc8259::subsets::i_json::schema::mutations::set_snapshot::SetSnapshot { snapshot: violating }));
        let err = mutated.build().expect_err("an unsafe integer must fail build()");
        assert!(err.iter().any(|d| d.code.0 == "stdio.json.i-json.unsafe-integer"));
    }
}
