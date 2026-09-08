mod tests {
    use super::*;
    use crate::standards::v_rfc8259::subsets::base::schema::snapshot::JsonMember;

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn obj(pairs: Vec<(&str, JsonValue)>) -> JsonValue {
        JsonValue::Object { members: pairs.into_iter().map(|(k, v)| JsonMember { key: k.into(), value: v }).collect() }
    }

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn snapshot(value: JsonValue) -> JsonSnapshot {
        JsonSnapshot { value, ..JsonSnapshot::default() }
    }

    #[semio_framework_async_macros::async_test]
    async fn conforming_object_reports_nothing() {
        let value = obj(vec![("a", JsonValue::Number { lexeme: "1".into() }), ("b", JsonValue::String { value: "hi".into() })]);
        let diagnostics = check_i_json_conformance(&snapshot(value));
        assert!(diagnostics.is_empty(), "got {diagnostics:?}");
    }

    #[semio_framework_async_macros::async_test]
    async fn duplicate_member_name_is_hard() {
        let value = JsonValue::Object { members: vec![JsonMember { key: "a".into(), value: JsonValue::Number { lexeme: "1".into() } }, JsonMember { key: "a".into(), value: JsonValue::Number { lexeme: "2".into() } }] };
        let diagnostics = check_i_json_conformance(&snapshot(value));
        assert!(diagnostics.iter().any(|d| d.code.0 == CODE_DUPLICATE_MEMBER && d.severity == Severity::Error), "got {diagnostics:?}");
    }

    #[semio_framework_async_macros::async_test]
    async fn nested_duplicate_member_name_is_detected_recursively() {
        let inner = JsonValue::Object { members: vec![JsonMember { key: "x".into(), value: JsonValue::Null }, JsonMember { key: "x".into(), value: JsonValue::Null }] };
        let value = obj(vec![("outer", inner)]);
        let diagnostics = check_i_json_conformance(&snapshot(value));
        assert!(diagnostics.iter().any(|d| d.code.0 == CODE_DUPLICATE_MEMBER), "got {diagnostics:?}");
    }

    #[semio_framework_async_macros::async_test]
    async fn integer_within_safe_bound_is_clean() {
        let value = obj(vec![("n", JsonValue::Number { lexeme: "9007199254740991".into() })]);
        let diagnostics = check_i_json_conformance(&snapshot(value));
        assert!(diagnostics.iter().all(|d| d.code.0 != CODE_UNSAFE_INTEGER), "got {diagnostics:?}");
    }

    #[semio_framework_async_macros::async_test]
    async fn integer_beyond_safe_bound_is_hard() {
        let value = obj(vec![("n", JsonValue::Number { lexeme: "9007199254740993".into() })]);
        let diagnostics = check_i_json_conformance(&snapshot(value));
        assert!(diagnostics.iter().any(|d| d.code.0 == CODE_UNSAFE_INTEGER && d.severity == Severity::Error), "got {diagnostics:?}");
    }

    #[semio_framework_async_macros::async_test]
    async fn negative_integer_beyond_safe_bound_is_hard() {
        let value = obj(vec![("n", JsonValue::Number { lexeme: "-9007199254740993".into() })]);
        let diagnostics = check_i_json_conformance(&snapshot(value));
        assert!(diagnostics.iter().any(|d| d.code.0 == CODE_UNSAFE_INTEGER && d.severity == Severity::Error), "got {diagnostics:?}");
    }

    #[semio_framework_async_macros::async_test]
    async fn astronomically_large_integer_is_hard() {
        let value = obj(vec![("n", JsonValue::Number { lexeme: "100000000000000000000000000000".into() })]);
        let diagnostics = check_i_json_conformance(&snapshot(value));
        assert!(diagnostics.iter().any(|d| d.code.0 == CODE_UNSAFE_INTEGER && d.severity == Severity::Error), "got {diagnostics:?}");
    }

    #[semio_framework_async_macros::async_test]
    async fn fractional_number_is_never_flagged_as_unsafe_integer() {
        let value = obj(vec![("n", JsonValue::Number { lexeme: "9007199254740993.5".into() })]);
        let diagnostics = check_i_json_conformance(&snapshot(value));
        assert!(diagnostics.iter().all(|d| d.code.0 != CODE_UNSAFE_INTEGER), "got {diagnostics:?}");
    }

    #[semio_framework_async_macros::async_test]
    async fn top_level_scalar_is_soft() {
        let diagnostics = check_i_json_conformance(&snapshot(JsonValue::String { value: "hi".into() }));
        assert!(diagnostics.iter().any(|d| d.code.0 == CODE_TOP_LEVEL_SCALAR && d.severity == Severity::Warning), "got {diagnostics:?}");
    }

    #[semio_framework_async_macros::async_test]
    async fn top_level_array_is_clean_on_that_check() {
        let diagnostics = check_i_json_conformance(&snapshot(JsonValue::Array { items: vec![] }));
        assert!(diagnostics.iter().all(|d| d.code.0 != CODE_TOP_LEVEL_SCALAR), "got {diagnostics:?}");
    }

    #[semio_framework_async_macros::async_test]
    async fn noncharacter_in_string_is_soft() {
        let value = obj(vec![("s", JsonValue::String { value: "abc\u{FFFE}def".into() })]);
        let diagnostics = check_i_json_conformance(&snapshot(value));
        assert!(diagnostics.iter().any(|d| d.code.0 == CODE_STRING_NONCHARACTER && d.severity == Severity::Warning), "got {diagnostics:?}");
    }

    #[semio_framework_async_macros::async_test]
    async fn reserved_bmp_noncharacter_range_is_detected() {
        let value = obj(vec![("s", JsonValue::String { value: "\u{FDD5}".into() })]);
        let diagnostics = check_i_json_conformance(&snapshot(value));
        assert!(diagnostics.iter().any(|d| d.code.0 == CODE_STRING_NONCHARACTER), "got {diagnostics:?}");
    }

    #[semio_framework_async_macros::async_test]
    async fn ordinary_string_has_no_noncharacter_diagnostic() {
        let value = obj(vec![("s", JsonValue::String { value: "hello world".into() })]);
        let diagnostics = check_i_json_conformance(&snapshot(value));
        assert!(diagnostics.iter().all(|d| d.code.0 != CODE_STRING_NONCHARACTER), "got {diagnostics:?}");
    }
}
