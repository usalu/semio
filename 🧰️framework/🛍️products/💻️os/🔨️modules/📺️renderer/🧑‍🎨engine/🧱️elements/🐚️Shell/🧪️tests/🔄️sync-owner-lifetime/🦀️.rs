use super::{shell_sync_owner_matches, ShellSyncOwner};

const BINDING_FIXTURE: &str = include_str!("../../../../../../🔌️plugin/📡️backbone/🔗️binding/🧪️tests/🧫️fixtures/🔣️.json");
#[allow(non_snake_case)]
fn SHELL_SOURCE() -> &'static str {
    semio_framework_os_renderer_wgpu_test_support::shell_source()
}
#[allow(non_snake_case)]
fn BRIDGE_SOURCE() -> &'static str {
    semio_framework_os_renderer_wgpu_test_support::repo_text("🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🌉️ProgramBridge/🎯️targets/🧊️wgpu/🦀️.rs")
}
#[allow(non_snake_case)]
fn KERNEL_SOURCE() -> &'static str {
    semio_framework_os_renderer_wgpu_test_support::renderer_source()
}

fn owner(generation: u64, uri: &str) -> ShellSyncOwner {
    ShellSyncOwner { plugin_id: "s.plugin.puzzle".into(), instance_id: 7, binding_generation: generation, actor_uri: uri.into(), document_key: "space-a/map".into() }
}

#[test]
fn exact_sync_owner_rejects_late_and_foreign_completions() {
    let active = owner(3, "actor://space-a/map");
    assert!(shell_sync_owner_matches(&active, &owner(3, "actor://space-a/map")));
    assert!(!shell_sync_owner_matches(&active, &owner(2, "actor://space-a/map")));
    assert!(!shell_sync_owner_matches(&active, &owner(3, "actor://space-b/map")));
    let foreign_instance = ShellSyncOwner { instance_id: 8, ..owner(3, "actor://space-a/map") };
    assert!(!shell_sync_owner_matches(&active, &foreign_instance));
}

#[test]
fn renderer_uses_owned_event_binding_without_retired_commands() {
    let fixture: serde_json::Value = serde_json::from_str(BINDING_FIXTURE).expect("neutral binding fixture parses");
    assert_eq!(fixture["cases"].as_array().map(Vec::len), Some(8));
    assert!(BRIDGE_SOURCE().contains("Event::Message { source: MessageEndpoint::Shell"));
    assert!(BRIDGE_SOURCE().contains("Event::Message { source: MessageEndpoint::Backbone"));
    assert!(KERNEL_SOURCE().contains("QueuedKernelEventKind::MessageShell"));
    assert!(KERNEL_SOURCE().contains("QueuedKernelEventKind::MessageBackbone"));
    assert!(!SHELL_SOURCE().contains("plugin.attach_backbone("));
    assert!(!SHELL_SOURCE().contains("plugin.detach_backbone("));
}

#[test]
fn hub_overlay_has_footer_and_command_openers_after_dock_removal() {
    assert!(SHELL_SOURCE().contains("FRAMEWORK_HUB_PANEL_ID"));
    assert!(SHELL_SOURCE().contains("\"framework.hub.signIn\""));
    assert!(SHELL_SOURCE().contains("\"os.openHub\""));
    assert!(SHELL_SOURCE().matches("self.push_uri(\"/hub\".to_string());").count() >= 2);
    assert!(SHELL_SOURCE().matches("self.apply_shell_uri(\"/hub\").await").count() >= 2);
    assert!(SHELL_SOURCE().contains("if path.trim_end_matches('/') == \"/hub\""));
    assert!(SHELL_SOURCE().contains("ShellChromeFramePhase::HubWorkspace"));
}
