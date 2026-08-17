//! 👥️ SpaceIndexEditor panel — members (worker-brief task 3). The space's members from the folded
//! directory read model (`SpaceIndexConfig.members`, populated by `📇fold-directory-events`),
//! invite-by-email + role, remove-member, a visibility toggle, and copy-invite-link — every mutating
//! affordance an `os.directory.*` relay (contract §C6), never a network call from the guest.

use crate::editor::space_index::config::SpaceIndexConfig;
use crate::editor::space_index::space_index_action;
use semio_framework_plugin::{tree_item, tree_item_with_action, Label, LocalizedLabel, PanelGroup, PanelTabDefinition, PanelTabKind, PanelTreeBuilder, UiNode, UiTreeItemNode};
use serde_json::json;

pub const SPACE_INDEX_BODY_MEMBERS: &str = "s.space.members";
pub const SPACE_INDEX_PANEL_MEMBERS: &str = "s-space-members";

//#region 🔖️Definition
pub fn definition() -> PanelTabDefinition {
    PanelTabDefinition {
        kind: PanelTabKind::App(SPACE_INDEX_PANEL_MEMBERS.into()),
        label: LocalizedLabel::native("Members", "Mitglieder"),
        group: PanelGroup::Details,
        body_key: Some(SPACE_INDEX_BODY_MEMBERS.into()),
        children: Vec::new(),
    }
}
//#endregion 🔖️Definition

//#region 🔖️Render
fn action_button(id: &str, label: impl Into<Label>, icon: &str, action: &str, args: serde_json::Value) -> UiTreeItemNode {
    UiTreeItemNode { icon_id: Some(icon.into()), menu: None, ..tree_item_with_action(id, label, None, space_index_action(action, Some(args))) }
}

fn member_row(config: &SpaceIndexConfig, member: &crate::editor::space_index::config::SpaceIndexMember) -> UiTreeItemNode {
    let row_id = format!("member:{}", member.user_id);
    let label = if member.display_name.is_empty() { member.email.clone() } else { format!("{} ({})", member.display_name, member.email) };
    let _ = config;
    UiTreeItemNode { description: Some(member.role.clone()), icon_id: Some("user".into()), menu: None, ..tree_item_with_action(row_id, Label::data(label), None, space_index_action("removeMember", Some(json!({ "userId": member.user_id }))))
    }
}

/// 👥️ `#s-space-share` (contract §C0 id grammar) is the copy-invite-link action's element id; every
/// other action here follows the same `#s-space-<verb>` shape.
///
/// 🌐️ **Known limitation, documented (`📓️w2-b-report.md`)**: `UiTreeItemNode`/`PanelTreeBuilder`
/// take a plain `Label` (`impl Into<Label>`, gated to `Label::data`/a locale-already-resolved
/// `LabelText` — `🧰️framework/🔨️modules/🖱️ui/📦️packages/🦀️rust/🎯️targets/🧊️wgpu/🦀️label.rs`, no
/// `From<LocalizedLabel>`), never a bare `LocalizedLabel` — unlike `PanelTabDefinition.label`/
/// `ActionDefinition.label`/`DialogDefinition`/`ActionArgDef` (all genuinely bilingual below). A real
/// `LabelText` needs a resolved locale, which means a per-app `app_labels!` terminology struct plus a
/// `locale` field threaded through `SpaceIndexConfig` (`🖍️draw`'s `DrawPlayLabels`/`DrawConfig.locale`
/// precedent) — a real facet, not a one-line fix, deferred rather than half-built at this effort
/// level. Every tree-content string below is English-only until that lands; every STATIC manifest
/// string (panel tab, dialogs, action labels) is already en+de.
pub fn render(config: &SpaceIndexConfig) -> UiNode {
    let visibility_action = if config.visibility == "public" { action_button("s-space-visibility", Label::data("Make Private"), "lock", "setVisibility", json!({ "visibility": "private" })) } else { action_button("s-space-visibility", Label::data("Make Public"), "globe", "setVisibility", json!({ "visibility": "public" })) };
    let action_items = vec![
        action_button("s-space-invite", Label::data("Invite Member"), "user-plus", "requestInviteMember", json!({})),
        action_button("s-space-share", Label::data("Copy Invite Link"), "link", "copyInviteLink", json!({ "role": "spectator", "ttlSecs": 604800u64 })),
        visibility_action,
    ];
    let member_items = if config.members.is_empty() {
        vec![UiTreeItemNode { icon_id: Some("users".into()), menu: None, ..tree_item("s-space-members-empty", Label::data("No members yet")) }]
    } else {
        config.members.iter().map(|member| member_row(config, member)).collect()
    };
    PanelTreeBuilder::new(SPACE_INDEX_PANEL_MEMBERS)
        .section(SPACE_INDEX_PANEL_MEMBERS, Some(Label::data("Members")), true, action_items.into_iter().chain(member_items).collect())
        .build()
}
//#endregion 🔖️Render

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::editor::space_index::config::SpaceIndexMember;

    #[test]
    fn empty_config_renders_the_empty_state() {
        let node = render(&SpaceIndexConfig::default());
        let json = serde_json::to_string(&node).unwrap();
        assert!(json.contains("s-space-members-empty"));
        assert!(json.contains("s-space-invite"));
        assert!(json.contains("s-space-share"));
    }

    #[test]
    fn members_render_with_a_remove_action_each() {
        let config = SpaceIndexConfig { members: vec![SpaceIndexMember { user_id: "u-1".into(), email: "a@example.com".into(), display_name: "Alice".into(), role: "author".into() }], ..Default::default() };
        let node = render(&config);
        let json = serde_json::to_string(&node).unwrap();
        assert!(json.contains("member:u-1"));
        assert!(json.contains("removeMember"));
        assert!(json.contains("author"));
    }

    #[test]
    fn public_visibility_offers_make_private() {
        let config = SpaceIndexConfig { visibility: "public".into(), ..Default::default() };
        let node = render(&config);
        let json = serde_json::to_string(&node).unwrap();
        assert!(json.contains("\"visibility\":\"private\""));
    }
}
//#endregion 🧪️Tests
