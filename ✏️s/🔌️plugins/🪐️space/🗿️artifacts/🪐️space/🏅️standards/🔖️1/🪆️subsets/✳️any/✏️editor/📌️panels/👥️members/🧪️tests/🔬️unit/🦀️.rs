
use super::*;
use crate::editor::space_index::config::SpaceIndexMember;

fn wire_and_retire(node: semio_framework_plugin::BuiltNode) -> String {
    let wire = serde_json::to_string(&node);
    let mut retirement = semio_framework_ui_contract::BuiltTreeRetirement::new(node);
    while !retirement.terminal_is_empty() {
        let step = retirement.close_step(1, 4096).expect("members fixture tree remains valid");
        if !step.progressed {
            std::thread::yield_now();
        }
    }
    wire.expect("members fixture wire")
}

#[semio_framework_async_macros::async_test]
async fn empty_config_renders_the_empty_state() {
    let json = wire_and_retire(render(&SpaceIndexConfig::default()).expect("empty members panel"));
    assert!(json.contains("s-space-members-empty"));
    assert!(json.contains("s-space-invite"));
    assert!(json.contains("s-space-share"));
}

#[semio_framework_async_macros::async_test]
async fn members_render_with_a_remove_action_each() {
    let config = SpaceIndexConfig { members: vec![SpaceIndexMember { user_id: "u-1".into(), email: "a@example.com".into(), display_name: "Alice".into(), role: "author".into() }], ..Default::default() };
    let json = wire_and_retire(render(&config).expect("members panel"));
    assert!(json.contains("member:u-1"));
    assert!(json.contains("removeMember"));
    assert!(json.contains("author"));
}

#[semio_framework_async_macros::async_test]
async fn public_visibility_offers_make_private() {
    let config = SpaceIndexConfig { visibility: "public".into(), ..Default::default() };
    let json = wire_and_retire(render(&config).expect("public members panel"));
    assert!(json.contains("\"visibility\":\"private\""));
}
