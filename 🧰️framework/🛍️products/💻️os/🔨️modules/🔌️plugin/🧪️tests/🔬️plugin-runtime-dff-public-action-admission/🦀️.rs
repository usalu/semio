mod dff_public_action_admission_tests {
    use super::*;

    fn padded_action(controller_id: &str, action_id: &str, bytes: usize) -> String {
        let mut body = format!(r#"{{"address":{{"pluginId":"fixture","appId":"{controller_id}","modeId":"mode","windowKindId":"window","windowInstanceId":"instance","actionId":"{action_id}"}},"arguments":{{}}}}"#);
        body.extend(std::iter::repeat(' ').take(bytes - body.len()));
        body
    }

    fn padded_command(controller_id: &str, command_id: &str, bytes: usize) -> String {
        let mut body = format!(r#"{{"address":{{"owner":{{"app":{{"pluginId":"fixture","appId":"{controller_id}"}}}},"commandId":"{command_id}"}},"arguments":{{}}}}"#);
        body.extend(std::iter::repeat(' ').take(bytes - body.len()));
        body
    }

    fn owner(controller_id: &str) -> crate::app::ToolOwnerWitness {
        let owner_type_name = match controller_id {
            "s.draw.draw@1/*#editor" => "semio_framework_plugin::app::EditorApp<semio_s_plugin_draw::editor::draw::DrawPlayApp>",
            "s.flow.flow@1/*#editor" => "semio_framework_plugin::app::EditorApp<semio_s_plugin_flow::editor::flow::FlowPlayApp>",
            "s.forms.forms@1/*#editor" => "semio_framework_plugin::app::EditorApp<semio_s_plugin_forms::editor::forms::FormsPlayApp>",
            "s.remodel.remodel@1/*#editor" => "semio_framework_plugin::app::EditorApp<semio_s_plugin_remodel::editor::remodel::RemodelPlayApp>",
            _ => "unproved::Owner",
        };
        crate::app::ToolOwnerWitness::test_with_name(owner_type_name)
    }

    fn admit_action(body: &str, runtime_controller_id: &str) -> Result<(), Fault> {
        validate_public_action_envelope(body, owner(runtime_controller_id), runtime_controller_id, &[])
    }

    fn admit_command(body: &str, runtime_controller_id: &str) -> Result<(), Fault> {
        validate_public_command_envelope(body, owner(runtime_controller_id), runtime_controller_id, &[])
    }

    #[test]
    fn action_and_command_specific_dff_wire_caps_are_enforced_before_deserialization() {
        for (controller_id, command_id, limit) in [
            ("s.forms.forms@1/*#editor", "setTryValue", 16_384),
            ("s.forms.forms@1/*#editor", "setTryValueStep", 16_384),
            ("s.draw.draw@1/*#editor", "canvasPointerDown", 8_192),
            ("s.flow.flow@1/*#editor", "duplicateWidget", 8_192),
            ("s.flow.flow@1/*#editor", "duplicateWidgetStep", 8_192),
        ] {
            assert!(admit_action(&padded_action(controller_id, command_id, limit), controller_id).is_ok(), "{command_id} action must admit its exact maximum");
            assert!(admit_action(&padded_action(controller_id, command_id, limit + 1), controller_id).is_err(), "{command_id} action must reject one byte beyond its maximum");
            assert!(admit_command(&padded_command(controller_id, command_id, limit), controller_id).is_ok(), "{command_id} command must admit its exact maximum");
            assert!(admit_command(&padded_command(controller_id, command_id, limit + 1), controller_id).is_err(), "{command_id} command must reject one byte beyond its maximum");
        }
    }

    #[test]
    fn same_schema_and_id_cannot_inherit_another_controllers_public_limit() {
        let limit = 8_192;
        assert!(admit_action(&padded_action("s.draw.draw@1/*#editor", "canvasPointerDown", limit + 1), "s.draw.draw@1/*#editor").is_err());
        assert!(admit_action(&padded_action("s.other.draw@1/*#editor", "canvasPointerDown", limit + 1), "s.other.draw@1/*#editor").is_ok());
        assert!(admit_command(&padded_command("s.draw.draw@1/*#editor", "canvasPointerDown", limit + 1), "s.draw.draw@1/*#editor").is_err());
        assert!(admit_command(&padded_command("s.other.draw@1/*#editor", "canvasPointerDown", limit + 1), "s.other.draw@1/*#editor").is_ok());
    }

    #[test]
    fn malformed_and_hostile_strings_are_rejected_by_predecode_admission() {
        assert!(admit_action(r#"{"address":{"actionId":"canvasPointerDown""#, "s.draw.draw@1/*#editor").is_err());
        assert!(admit_command(r#"{"address":{"commandId":"canvasPointerDown""#, "s.draw.draw@1/*#editor").is_err());
        let hostile_action = format!(r#"{{"address":{{"actionId":"setTryValue"}},"arguments":{{"valueJson":"{}"}}}}"#, "x".repeat(MAX_PUBLIC_ACTION_STRING_BYTES + 1));
        let hostile_command = format!(r#"{{"address":{{"commandId":"setTryValue"}},"arguments":{{"valueJson":"{}"}}}}"#, "x".repeat(MAX_PUBLIC_ACTION_STRING_BYTES + 1));
        assert!(admit_action(&hostile_action, "s.draw.draw@1/*#editor").is_err());
        assert!(admit_command(&hostile_command, "s.draw.draw@1/*#editor").is_err());
        assert!(admit_command(r#"{"addr\u0065ss":{"commandId":"canvasPointerDown"},"arguments":{}}"#, "s.draw.draw@1/*#editor").is_err());
        assert!(admit_command(r#"{"address":{"command\u0049d":"canvasPointerDown"},"arguments":{}}"#, "s.draw.draw@1/*#editor").is_err());
        assert!(admit_command(r#"{"address":{"commandId":"canvasPointer\u0044own"},"arguments":{}}"#, "s.draw.draw@1/*#editor").is_err());
    }

    #[test]
    fn set_contributions_pack_keeps_body_cap_and_skips_string_cap() {
        let json = "x".repeat(200_000);
        let body = format!(r#"{{"address":{{"owner":{{"app":{{"pluginId":"procedural","appId":"generation3d"}}}},"commandId":"setContributions"}},"arguments":{{"json":"{json}","page":0,"pageCount":1}}}}"#);
        assert!(body.len() <= MAX_PUBLIC_ACTION_BODY_BYTES, "scoped pack fixture must fit the public body");
        assert!(admit_command(&body, "s.flow.flow@1/*#editor").is_ok(), "setContributions must cross as one body-bounded pack");
        let over = format!(r#"{{"address":{{"owner":{{"app":{{"pluginId":"procedural","appId":"generation3d"}}}},"commandId":"setContributions"}},"arguments":{{"json":"{}","page":0,"pageCount":1}}}}"#, "x".repeat(MAX_PUBLIC_ACTION_BODY_BYTES));
        assert!(over.len() > MAX_PUBLIC_ACTION_BODY_BYTES);
        assert!(admit_command(&over, "s.flow.flow@1/*#editor").is_err(), "setContributions must still refuse an over-body pack");
    }

    #[test]
    fn command_classifier_uses_only_the_exact_address_command_id() {
        let mut decoy = r#"{"address":{"owner":"os","commandId":"ordinary"},"arguments":{"commandId":"canvasPointerDown"}}"#.to_string();
        decoy.extend(std::iter::repeat(' ').take(8_193 - decoy.len()));
        assert!(admit_command(&decoy, "s.draw.draw@1/*#editor").is_ok());
    }

    #[semio_framework_async_macros::async_test]
    async fn public_action_and_command_entry_points_require_a_live_instance_before_decode() {
        let runtime = PluginRuntime::<crate::app::NoPluginApp>::new();
        let action = plugin_handle_action(&runtime, 1, &padded_action("s.draw.draw@1/*#editor", "canvasPointerDown", 8_192), "{}").await.expect_err("an action without a live instance must reject before decoding");
        assert!(action.message.contains("unknown instance"));
        let command = plugin_handle_command(&runtime, 1, &padded_command("s.draw.draw@1/*#editor", "canvasPointerDown", 8_192), "{}").await.expect_err("a command without a live instance must reject before decoding");
        assert!(command.message.contains("unknown instance"));

        let malformed = plugin_handle_command(&runtime, 1, r#"{"address":{"commandId":"setTryValue""#, "{}").await.expect_err("a malformed DFF command must be rejected before decoding");
        assert!(malformed.message.contains("structurally incomplete"));
        let hostile = format!(r#"{{"address":{{"owner":"os","commandId":"setTryValue"}},"arguments":{{"valueJson":"{}"}}}}"#, "x".repeat(MAX_PUBLIC_ACTION_STRING_BYTES + 1));
        let hostile = plugin_handle_command(&runtime, 1, &hostile, "{}").await.expect_err("a hostile DFF command string must be rejected before decoding");
        assert!(hostile.message.contains("oversized string"));
    }

    #[test]
    fn action_entry_points_drive_spawn_admit_instead_of_awaiting_the_retained_job() {
        let source = include_str!("../../🦀️.rs");
        let public_start = source.find("pub async fn plugin_handle_action").expect("public action entry point");
        let public_end = source[public_start..].find("pub async fn plugin_handle_command").map(|offset| public_start + offset).expect("public action boundary");
        let public_action = &source[public_start..public_end];
        assert!(public_action.contains("runtime_instance_cell(runtime, instance_id)?"));
        assert!(public_action.contains("drive_self_waking_ready(instance.app.handle_action_invocation"));
        assert!(!public_action.contains("resolve_ready(instance.app.handle_action_invocation"));
        assert!(!public_action.contains("run_framework_reserved_job"));

        let exchange_start = source.find("protocol::AppCommand::Command { seq, command, view_state }").expect("reactor command exchange");
        let exchange_end = source[exchange_start..].find("protocol::AppCommand::CommandText").map(|offset| exchange_start + offset).expect("reactor command exchange boundary");
        let exchange = &source[exchange_start..exchange_end];
        assert!(exchange.contains("runtime_instance_cell(runtime, instance_id)?"));
        assert!(exchange.contains("drive_self_waking_ready(instance.app.handle_action_invocation"));
        assert!(!exchange.contains("resolve_ready(instance.app.handle_action_invocation"));
        assert!(!exchange.contains("run_framework_reserved_job"));
    }
}
