mod role_chrome_tests {
    use super::super::layout::ActionDescriptor;
    use super::super::utilities::{utility_button, utility_toggle};
    use super::*;
    use crate::wgpu::IconName;

    #[semio_framework_async_macros::async_test]
    async fn from_boot_env_accepts_viewer_and_falls_back_to_editor() {
        assert_eq!(ChromeRole::from_boot_env(Some("viewer")), ChromeRole::Viewer);
        assert_eq!(ChromeRole::from_boot_env(Some("editor")), ChromeRole::Editor);
        assert_eq!(ChromeRole::from_boot_env(Some("bogus")), ChromeRole::Editor);
        assert_eq!(ChromeRole::from_boot_env(Some("")), ChromeRole::Editor);
        assert_eq!(ChromeRole::from_boot_env(None), ChromeRole::Editor);
    }

    #[semio_framework_async_macros::async_test]
    async fn title_chip_text_covers_both_roles_in_both_locales() {
        assert_eq!(role_title_chip_text(ChromeRole::Viewer, false), "Viewer");
        assert_eq!(role_title_chip_text(ChromeRole::Viewer, true), "Betrachter");
        assert_eq!(role_title_chip_text(ChromeRole::Editor, false), "Editor");
        assert_eq!(role_title_chip_text(ChromeRole::Editor, true), "Editor");
    }

    fn entry(plugin_id: &str, app_id: &str, role: ChromeRole, is_default: bool) -> OpenWithEntry {
        OpenWithEntry { plugin_id: plugin_id.into(), app_id: app_id.into(), label: app_id.into(), role, is_default }
    }

    #[semio_framework_async_macros::async_test]
    async fn open_with_menu_item_groups_by_role_viewer_first_then_editor() {
        let entries = vec![entry("norm", "s.cad.cad@1/*#editor", ChromeRole::Editor, false), entry("cad", "s.cad.cad@1/*#viewer", ChromeRole::Viewer, true)];
        let menu = open_with_menu_item(&entries, false);
        assert_eq!(menu.id, "menu.open-with");
        assert_eq!(menu.label.as_deref(), Some("Open with…"));
        let children = menu.children.expect("submenu children");
        assert_eq!(children.len(), 4, "viewer header + viewer entry + editor header + editor entry");
        assert_eq!(children[0].label.as_deref(), Some("Viewer"));
        assert_eq!(children[0].separator, Some(true));
        assert_eq!(children[1].id, "menu.open-with.cad.s.cad.cad@1/*#viewer");
        assert_eq!(children[2].label.as_deref(), Some("Editor"));
        assert_eq!(children[3].id, "menu.open-with.norm.s.cad.cad@1/*#editor");
    }

    #[semio_framework_async_macros::async_test]
    async fn open_with_menu_item_toggle_sets_when_not_default_and_clears_when_default() {
        let entries = vec![entry("cad", "editor", ChromeRole::Editor, false), entry("norm", "editor-alt", ChromeRole::Editor, true)];
        let menu = open_with_menu_item(&entries, false);
        let editor_entries: Vec<_> = menu.children.unwrap().into_iter().filter(|item| item.separator != Some(true)).collect();
        let not_default_toggle = editor_entries[0].children.as_ref().unwrap()[0].clone();
        assert_eq!(not_default_toggle.action.as_deref(), Some(OS_SET_DEFAULT_EDITOR));
        assert_eq!(not_default_toggle.checked, Some(false));
        let already_default_toggle = editor_entries[1].children.as_ref().unwrap()[0].clone();
        assert_eq!(already_default_toggle.action.as_deref(), Some(OS_CLEAR_DEFAULT_APP));
        assert_eq!(already_default_toggle.checked, Some(true));
    }

    #[semio_framework_async_macros::async_test]
    async fn open_with_menu_item_localizes_headers_and_label_to_german() {
        let entries = vec![entry("cad", "viewer", ChromeRole::Viewer, false)];
        let menu = open_with_menu_item(&entries, true);
        assert_eq!(menu.label.as_deref(), Some("Öffnen mit…"));
        assert_eq!(menu.children.unwrap()[1].children.as_ref().unwrap()[0].label.as_deref(), Some("Als Standard festlegen"));
    }

    fn shell_action(id: &str, kind: &str) -> ShellMenuAction {
        ShellMenuAction { id: id.into(), label: id.into(), icon: None, keys: None, kind: kind.into(), category: None, in_palette: true, arg_carrying: false }
    }

    #[semio_framework_async_macros::async_test]
    async fn filter_shell_menu_actions_drops_mutation_kind_for_viewer_only() {
        let actions = vec![shell_action("shell.rename", "Mutation"), shell_action("shell.zoomIn", "View")];
        let viewer = filter_shell_menu_actions_for_role(&actions, ChromeRole::Viewer);
        assert_eq!(viewer.iter().map(|action| action.id.as_str()).collect::<Vec<_>>(), vec!["shell.zoomIn"]);
        let editor = filter_shell_menu_actions_for_role(&actions, ChromeRole::Editor);
        assert_eq!(editor.len(), 2, "editor chrome keeps every action");
    }

    #[semio_framework_async_macros::async_test]
    async fn apply_role_to_utilities_disables_history_only_for_viewer() {
        let press = ActionDescriptor { controller_id: "history".into(), action: "undo".into(), args: None };
        let toggle = ActionDescriptor { controller_id: "select".into(), action: "toggleSelect".into(), args: None };
        let utilities = vec![utility_button("undo", IconName::RotateCcw, "Undo", press).with_category(UtilityCategory::History), utility_toggle("select", IconName::MousePointer, "Select", false, toggle)];
        let viewer = apply_role_to_utilities(utilities.clone(), ChromeRole::Viewer);
        assert_eq!(viewer[0].category(), UtilityCategory::History);
        assert!(matches!(&viewer[0], UtilityNode::Button { disabled: Some(true), .. }), "undo must be disabled for a viewer");
        assert!(matches!(&viewer[1], UtilityNode::Toggle { disabled: None, .. }), "non-history utilities are untouched");
        let editor = apply_role_to_utilities(utilities, ChromeRole::Editor);
        assert!(matches!(&editor[0], UtilityNode::Button { disabled: None, .. }), "editor chrome never disables history utilities");
    }

    // 🌱️ Ticket 26/09/01/RUNTIME-DEPENDENCY-ELIMINATION-FOR-S-PLUGINS-AND-ARTIFACTS: round-trip
    // coverage for `ChromeRole`.
    #[semio_framework_async_macros::async_test]
    async fn chrome_role_round_trips() {
        for role in [ChromeRole::Viewer, ChromeRole::Editor] {
            assert_eq!(ChromeRole::from_value(role.to_value()).expect("valid DslValue decodes"), role);
        }
    }
}
