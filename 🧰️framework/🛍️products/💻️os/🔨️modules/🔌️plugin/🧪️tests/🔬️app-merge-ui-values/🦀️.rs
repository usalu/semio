mod merge_ui_values_tests {
    use super::*;

    fn ui_text(value: &str) -> UiText {
        UiText::try_from_str(value).expect("bounded fixture text")
    }

    fn ui_list(values: impl IntoIterator<Item = UiValue>) -> UiList {
        let mut builder = UiListBuilder::try_new().expect("fixed list builder");
        for value in values {
            builder.push(value).expect("fixed list page");
        }
        builder.finish()
    }

    fn ui_map(entries: impl IntoIterator<Item = (String, UiValue)>) -> UiMap {
        let mut builder = UiMapBuilder::try_new().expect("fixed map builder");
        for (key, value) in entries {
            builder.push(key, value).expect("ascending fixed map page");
        }
        builder.finish()
    }

    #[semio_framework_async_macros::async_test]
    async fn retained_ui_value_bridge_advances_every_shape_to_the_matching_json_candidate() {
        assert_eq!(ui_value_to_dsl_retained(&UiValue::Null).await.expect("retained null"), DslValue::Null);
        assert_eq!(ui_value_to_dsl_retained(&UiValue::Bool(true)).await.expect("retained bool"), DslValue::Bool(true));
        assert_eq!(ui_value_to_dsl_retained(&UiValue::Number(-2.5)).await.expect("retained number"), serde_json::json!(-2.5));
        assert_eq!(ui_value_to_dsl_retained(&UiValue::Text(ui_text("hi"))).await.expect("retained text"), DslValue::String("hi".into()));
        assert_eq!(ui_value_to_dsl_retained(&UiValue::List(ui_list([UiValue::Number(1.0), UiValue::Bool(false)]))).await.expect("retained list"), DslValue::from(&serde_json::from_str::<serde_json::Value>("[1,false]").expect("independent JSON.stringify fixture")));
        let mut map = BTreeMap::new();
        map.insert("id".to_string(), UiValue::Text(ui_text("w1")));
        assert_eq!(ui_value_to_dsl_retained(&UiValue::Map(ui_map(map))).await.expect("retained map"), serde_json::json!({ "id": "w1" }));
    }

    #[semio_framework_async_macros::async_test]
    async fn merge_ui_values_returns_none_when_neither_side_is_set() {
        assert_eq!(merge_ui_values(None, None).await.expect("empty merge"), None);
    }

    #[semio_framework_async_macros::async_test]
    async fn merge_ui_values_falls_back_to_whichever_single_side_is_set() {
        assert_eq!(merge_ui_values(Some(&UiValue::Text(ui_text("a"))), None).await.expect("args merge"), Some(DslValue::from(&serde_json::json!("a"))));
        assert_eq!(merge_ui_values(None, Some(&UiValue::Number(3.0))).await.expect("input merge"), Some(DslValue::from(&serde_json::json!(3))));
    }

    /// 🔀️ The decided merge rule: `input` wins on key collision when both are objects.
    #[semio_framework_async_macros::async_test]
    async fn merge_ui_values_prefers_input_on_key_collision_between_two_maps() {
        let mut args_map = BTreeMap::new();
        args_map.insert("value".to_string(), UiValue::Number(1.0));
        args_map.insert("scope".to_string(), UiValue::Text(ui_text("keep")));
        let mut input_map = BTreeMap::new();
        input_map.insert("value".to_string(), UiValue::Number(2.0));
        let merged = merge_ui_values(Some(&UiValue::Map(ui_map(args_map))), Some(&UiValue::Map(ui_map(input_map)))).await.expect("retained merge").expect("both sides set");
        assert_eq!(merged, serde_json::json!({ "value": 2, "scope": "keep" }));
    }

    /// 🔀️ A scalar `input` has no field to merge INTO — it replaces `args` wholesale (e.g.
    /// `Trigger::Delta`'s signed step count next to a non-object `args`).
    #[semio_framework_async_macros::async_test]
    async fn merge_ui_values_replaces_a_non_object_args_wholesale_when_input_is_also_set() {
        let merged = merge_ui_values(Some(&UiValue::Text(ui_text("stale"))), Some(&UiValue::Number(-2.0))).await.expect("retained merge");
        assert_eq!(merged, Some(DslValue::from(&serde_json::json!(-2))));
    }

}
