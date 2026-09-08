mod opening_command_relay_tests {
    use super::*;

    async fn replay(effect: Effect) -> (String, Value) {
        let Effect::ReplayShellCommand { action_id, args: Some(args) } = effect else { panic!("expected a shell command relay") };
        (action_id, Value::from(args))
    }

    #[semio_framework_async_macros::async_test]
    async fn open_artifact_relays_an_exactly_matched_surface() {
        let (action_id, args) = replay(relay_open_artifact("s.test.document@1/*#editor".into(), 1, "test-plugin".into(), "s.test.document@1/*#editor".into()).await.expect("matching surface relays")).await;
        assert_eq!(action_id, "os.open-artifact");
        assert_eq!(args, serde_json::json!({ "artifactRef": "s.test.document@1/*#editor", "role": 1, "pluginId": "test-plugin", "appId": "s.test.document@1/*#editor" }));
    }

    #[semio_framework_async_macros::async_test]
    async fn default_app_commands_relay_the_validated_wire_coordinates() {
        let (set_action_id, set_args) = replay(relay_set_default_app("s.test.document".into(), "1".into(), "*".into(), 0, "test-plugin".into(), "s.test.document@1/*#viewer".into()).await.expect("matching default surface relays")).await;
        assert_eq!(set_action_id, "os.set-default-app");
        assert_eq!(set_args, serde_json::json!({ "artifactKind": "s.test.document", "standard": "1", "subset": "*", "role": 0, "pluginId": "test-plugin", "appId": "s.test.document@1/*#viewer" }));

        let (clear_action_id, clear_args) = replay(relay_clear_default_app("s.test.document".into(), "1".into(), "*".into(), 1).await.expect("valid clear relays")).await;
        assert_eq!(clear_action_id, "os.clear-default-app");
        assert_eq!(clear_args, serde_json::json!({ "artifactKind": "s.test.document", "standard": "1", "subset": "*", "role": 1 }));
    }

    #[semio_framework_async_macros::async_test]
    async fn opening_relays_reject_invalid_or_inconsistent_addresses() {
        let invalid_role = relay_open_artifact("s.test.document@1/*#viewer".into(), 7, String::new(), String::new()).await.expect_err("unknown role must fail");
        assert_eq!(invalid_role.code.0, "opening.invalid-role");

        let partial_app = relay_open_artifact("s.test.document@1/*#viewer".into(), 0, "test-plugin".into(), String::new()).await.expect_err("partial app reference must fail");
        assert_eq!(partial_app.code.0, "opening.partial-app-ref");

        let mismatched_app = relay_set_default_app("s.test.document".into(), "1".into(), "*".into(), 0, "test-plugin".into(), "s.test.other@1/*#viewer".into()).await.expect_err("app dialect must match default coordinate");
        assert_eq!(mismatched_app.code.0, "opening.app-mismatch");
    }
}
