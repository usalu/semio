
use super::*;
use crate::editor::space_index::config::SpaceIndexMember;
use semio_framework_plugin::{TreeWindowRequest, ViewModel, TREE_WINDOW_DEFAULT_ROWS};

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
    let json = wire_and_retire(render(&SpaceIndexConfig::default(), &TreeWindows::unhosted()).expect("empty members panel"));
    assert!(json.contains("s-space-members.empty"));
    assert!(json.contains("s-space-invite"));
    assert!(json.contains("s-space-share"));
}

#[semio_framework_async_macros::async_test]
async fn members_render_with_a_remove_action_each() {
    let config = SpaceIndexConfig { members: vec![SpaceIndexMember { user_id: "u-1".into(), email: "a@example.com".into(), display_name: "Alice".into(), role: "author".into() }], ..Default::default() };
    let json = wire_and_retire(render(&config, &TreeWindows::unhosted()).expect("members panel"));
    assert!(json.contains("member:u-1"));
    assert!(json.contains("removeMember"));
    assert!(json.contains("author"));
}

#[semio_framework_async_macros::async_test]
async fn public_visibility_offers_make_private() {
    let config = SpaceIndexConfig { visibility: "public".into(), ..Default::default() };
    let json = wire_and_retire(render(&config, &TreeWindows::unhosted()).expect("public members panel"));
    assert!(json.contains("\"visibility\":\"private\""));
}

//#region 🪟️WindowLaws
/// 🪟️ A collaboration space an order of magnitude past one viewport — the subject of every law below.
fn oversized_config(members: usize) -> SpaceIndexConfig {
    SpaceIndexConfig {
        members: (0..members)
            .map(|index| SpaceIndexMember { user_id: format!("u-{index}"), email: format!("member-{index}@example.com"), display_name: format!("Member {index}"), role: "author".into() })
            .collect(),
        ..Default::default()
    }
}

/// 🪟️ The panel body exactly as the host reads it, for the host-known windows in `requests`.
fn window_body(config: &SpaceIndexConfig, requests: Vec<TreeWindowRequest>) -> String {
    let view = ViewModel { tree_windows: requests, ..Default::default() };
    let node = render(config, &TreeWindows::for_body(&view, SPACE_INDEX_BODY_MEMBERS)).expect("render the members panel");
    semio_framework_plugin::artifact_app_laws::project_and_retire_fixture_tree(semio_framework_plugin::built_to_component_tree(node)).expect("project the members panel")
}

/// 🪟️ Law (a): the members container stamps its FULL extent and materialises at most its slice — no
/// `+N`, and the three fixed action rows sit in their own unwindowed section.
#[semio_framework_async_macros::async_test]
async fn oversized_membership_stamps_totals_and_never_a_continuation_row() {
    let config = oversized_config(300);
    let json = window_body(&config, Vec::new());
    assert!(json.contains("\"total\":300"), "the members section stamps its full extent: {json}");
    assert!(!json.contains(".more"), "no continuation row survives: {json}");
    assert!(!json.contains("\"+"), "no `+N` label survives: {json}");
    assert!(json.matches("\"member:").count() <= TREE_WINDOW_DEFAULT_ROWS as usize, "first paint materialises about one viewport: {json}");
    assert!(json.contains("s-space-invite") && json.contains("s-space-share") && json.contains("s-space-visibility"), "the three fixed action rows always render: {json}");
}

/// 🪟️ Law (b): a container the host closed stamps its total and materialises nothing.
#[semio_framework_async_macros::async_test]
async fn closed_section_stamps_total_and_materialises_no_members() {
    let config = oversized_config(300);
    let json = window_body(&config, vec![TreeWindowRequest { body_key: SPACE_INDEX_BODY_MEMBERS.into(), node_key: SPACE_INDEX_PANEL_MEMBERS.into(), open: Some(false), offset: 0, rows: 0 }]);
    assert!(json.contains("\"total\":300"), "a closed section still stamps its extent: {json}");
    assert!(!json.contains("\"member:"), "a closed section materialises no rows: {json}");
    assert!(json.contains("s-space-invite"), "the fixed action section is unaffected: {json}");
}

/// 🪟️ Law (c): a host window materialises exactly `[offset, offset + rows)`, keyed by the raw row id.
#[semio_framework_async_macros::async_test]
async fn host_window_materialises_exactly_its_slice() {
    let config = oversized_config(300);
    let json = window_body(&config, vec![TreeWindowRequest { body_key: SPACE_INDEX_BODY_MEMBERS.into(), node_key: SPACE_INDEX_PANEL_MEMBERS.into(), open: Some(true), offset: 120, rows: 6 }]);
    assert!(json.contains("\"offset\":120"), "the section reports its offset: {json}");
    for index in 120..126 {
        assert!(json.contains(&format!("\"member:u-{index}\"")), "row {index} is inside the window: {json}");
    }
    assert!(!json.contains("\"member:u-119\""), "the row before the window stays out: {json}");
    assert!(!json.contains("\"member:u-126\""), "the row after the window stays out: {json}");
}

/// 🪟️ Law (d) for a deliberately UNBOUND tree: membership rows are directory identities, not targets
/// of any app interaction domain, so this tree declares none and stamps no granularity — every member
/// row keeps its own `removeMember` binding instead.
#[semio_framework_async_macros::async_test]
async fn unbound_tree_keeps_per_row_actions_and_declares_no_domain() {
    let config = oversized_config(3);
    let json = window_body(&config, Vec::new());
    assert!(!json.contains("interactionDomain"), "the members tree binds no domain: {json}");
    assert!(!json.contains("granularity"), "an unbound tree stamps no pick granularity: {json}");
    assert_eq!(json.matches("removeMember").count(), 3, "every member row keeps its own action: {json}");
}
//#endregion 🪟️WindowLaws
