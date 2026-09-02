//! 👥️ SpaceIndexEditor panel — members (worker-brief task 3). The space's members from the folded
//! directory read model (`SpaceIndexConfig.members`, populated by `📇fold-directory-events`),
//! invite-by-email + role, remove-member, a visibility toggle, and copy-invite-link — every mutating
//! affordance an `os.directory.*` relay (contract §C6), never a network call from the guest.

use crate::editor::space_index::config::SpaceIndexConfig;
use crate::editor::space_index::space_index_action;
use semio_framework_plugin::plugin_app_close_prelude::Label;
use semio_framework_plugin::{tree_item, tree_item_with_action, LocalizedLabel, PanelGroup, PanelTabDefinition, PanelTabKind, PanelTreeBuilder};

pub const SPACE_INDEX_BODY_MEMBERS: &str = "s.space.members";
pub const SPACE_INDEX_PANEL_MEMBERS: &str = "s-space-members";

//#region 🔖️Definition
pub fn definition() -> PanelTabDefinition {
    PanelTabDefinition { kind: PanelTabKind::App(SPACE_INDEX_PANEL_MEMBERS.into()), label: LocalizedLabel::native("Members", "Mitglieder"), group: PanelGroup::Details, body_key: Some(SPACE_INDEX_BODY_MEMBERS.into()), children: Vec::new() }
}
//#endregion 🔖️Definition

//#region 🔖️Render
fn action_button(id: &str, label: impl TryInto<Label>, icon: &str, action: &str, args: semio_framework_plugin::UiValue) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::BuiltNode> {
    let mut item = tree_item_with_action(id, label, None, space_index_action(action, Some(args))?)?;
    if let semio_framework_plugin::Component::TreeItem(props) = &mut item.component {
        props.icon = Some(semio_framework_plugin::UiText::try_from_str(icon).ok_or_else(|| semio_framework_plugin::PluginAssemblyError::new("ui.member.icon", "fixed member-row icon admission failed"))?);
    }
    Ok(item)
}

fn member_row(config: &SpaceIndexConfig, member: &crate::editor::space_index::config::SpaceIndexMember) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::BuiltNode> {
    let row_id = format!("member:{}", member.user_id);
    let label = if member.display_name.is_empty() { member.email.clone() } else { format!("{} ({})", member.display_name, member.email) };
    let _ = config;
    let args = crate::editor::space_index::ui_value_map([("userId", crate::editor::space_index::ui_value_text(&member.user_id)?)])?;
    let mut item = tree_item_with_action(row_id, label, None, space_index_action("removeMember", Some(args))?)?;
    if let semio_framework_plugin::Component::TreeItem(props) = &mut item.component {
        props.description = Some(semio_framework_plugin::UiText::try_from_str(&member.role).ok_or_else(|| semio_framework_plugin::PluginAssemblyError::new("ui.member.role", "fixed member role admission failed"))?);
        props.icon = Some(semio_framework_plugin::UiText::try_from_str("user").ok_or_else(|| semio_framework_plugin::PluginAssemblyError::new("ui.member.icon", "fixed member-row icon admission failed"))?);
    }
    Ok(item)
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
pub fn render(config: &SpaceIndexConfig) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::BuiltNode> {
    let visibility_action = if config.visibility == "public" {
        action_button("s-space-visibility", "Make Private", "lock", "setVisibility", crate::editor::space_index::ui_value_map([("visibility", crate::editor::space_index::ui_value_text("private")?)])?)
    } else {
        action_button("s-space-visibility", "Make Public", "globe", "setVisibility", crate::editor::space_index::ui_value_map([("visibility", crate::editor::space_index::ui_value_text("public")?)])?)
    };
    let action_items = crate::editor::space_index::ui_node_list([
        action_button("s-space-invite", "Invite Member", "user-plus", "requestInviteMember", semio_framework_plugin::UiValue::Map(Default::default())),
        action_button(
            "s-space-share",
            "Copy Invite Link",
            "link",
            "copyInviteLink",
            crate::editor::space_index::ui_value_map([
                ("role", crate::editor::space_index::ui_value_text("spectator")?),
                ("ttlSecs", crate::editor::space_index::ui_value_number(604800.0)),
            ])?,
        ),
        visibility_action,
    ])?;
    let member_items = if config.members.is_empty() {
        let mut empty = tree_item("s-space-members-empty", "No members yet")?;
        if let semio_framework_plugin::Component::TreeItem(props) = &mut empty.component {
            props.icon = Some(semio_framework_plugin::UiText::try_from_str("users").ok_or_else(|| semio_framework_plugin::PluginAssemblyError::new("ui.member.icon", "fixed member-row icon admission failed"))?);
        }
        crate::editor::space_index::ui_node_list([Ok(empty)])?
    } else {
        crate::editor::space_index::ui_node_list(config.members.iter().map(|member| member_row(config, member)))?
    };
    let mut items = semio_framework_plugin::UiFixedList::default();
    for item in action_items.into_iter().chain(member_items) {
        items.try_push(item).map_err(|_| semio_framework_plugin::PluginAssemblyError::new("ui.member.items", "fixed member panel admission failed"))?;
    }
    let section_label = Label::try_from("Members").map_err(|_| semio_framework_plugin::PluginAssemblyError::new("ui.member.section-label", "fixed member section label admission failed"))?;
    PanelTreeBuilder::new(SPACE_INDEX_PANEL_MEMBERS)?.section(SPACE_INDEX_PANEL_MEMBERS, Some(section_label), true, items)?.build()
}
//#endregion 🔖️Render

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::editor::space_index::config::SpaceIndexMember;

    #[semio_framework_async_macros::async_test]
    async fn empty_config_renders_the_empty_state() {
        let node = render(&SpaceIndexConfig::default());
        let json = pack::to_json_string(&node);
        assert!(json.contains("s-space-members-empty"));
        assert!(json.contains("s-space-invite"));
        assert!(json.contains("s-space-share"));
    }

    #[semio_framework_async_macros::async_test]
    async fn members_render_with_a_remove_action_each() {
        let config = SpaceIndexConfig { members: vec![SpaceIndexMember { user_id: "u-1".into(), email: "a@example.com".into(), display_name: "Alice".into(), role: "author".into() }], ..Default::default() };
        let node = render(&config);
        let json = pack::to_json_string(&node);
        assert!(json.contains("member:u-1"));
        assert!(json.contains("removeMember"));
        assert!(json.contains("author"));
    }

    #[semio_framework_async_macros::async_test]
    async fn public_visibility_offers_make_private() {
        let config = SpaceIndexConfig { visibility: "public".into(), ..Default::default() };
        let node = render(&config);
        let json = pack::to_json_string(&node);
        assert!(json.contains("\"visibility\":\"private\""));
    }
}
//#endregion 🧪️Tests
