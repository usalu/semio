
use super::*;
use crate::wgpu::flex::LayoutEngine;
use crate::wgpu::text::FontAtlas;
use crate::wgpu::theme::Theme;

fn single_window_layout(window_kind_id: &str) -> WindowLayout {
    crate::wgpu::even_window_layout(&[window_kind_id.to_string()])
}

fn run_layout(shell: &mut Shell) {
    let root = shell.tree().root.expect("set_window_layout must produce a root");
    let mut engine = LayoutEngine::new();
    let mut atlas = FontAtlas::builtin();
    let theme = Theme::default();
    engine.compute(shell.tree_mut(), root, &mut atlas, &theme, 400.0, 400.0);
}

fn count_nodes(tree: &UiTree, id: NodeId) -> usize {
    1 + tree.children(id).map(|child| count_nodes(tree, child)).sum::<usize>()
}

#[test]
fn set_window_layout_with_one_window_produces_the_expected_retained_tree_shape() {
    let mut shell = Shell::new();
    shell.set_window_layout(single_window_layout("app.viewport"));

    let root = shell.tree().root.expect("expected a root node");
    // shell.root -> tab-group Stack -> one Button window leaf = 3 nodes.
    assert_eq!(count_nodes(shell.tree(), root), 3);
    let tab_group = shell.tree().children(root).next().expect("expected a tab-group child");
    assert!(shell.tree().node(tab_group).unwrap().flags.contains(NodeFlags::CLIPS_CHILDREN));
    let window_leaf = shell.tree().children(tab_group).next().expect("expected a window leaf");
    assert!(matches!(shell.tree().node(window_leaf).unwrap().spec.0, UiNode::Button(_)));
}

#[test]
fn set_window_layout_called_twice_with_the_same_layout_is_idempotent_and_does_not_panic() {
    let mut shell = Shell::new();
    shell.set_window_layout(single_window_layout("app.viewport"));
    let first_count = count_nodes(shell.tree(), shell.tree().root.unwrap());

    shell.set_window_layout(single_window_layout("app.viewport"));
    let second_count = count_nodes(shell.tree(), shell.tree().root.unwrap());

    assert_eq!(first_count, second_count);
    assert_eq!(shell.window_layout(), Some(&single_window_layout("app.viewport")));
}

#[test]
fn pointer_down_and_up_on_the_same_window_cap_activates_its_tab() {
    let mut shell = Shell::new();
    shell.set_window_layout(single_window_layout("app.viewport"));
    run_layout(&mut shell);

    let down = shell.dispatch(&UiEvent::PointerDown { x: 10.0, y: 10.0, button: crate::wgpu::events::PointerButton::Primary });
    assert!(down.is_empty(), "press alone must not activate a tab");

    let up = shell.dispatch(&UiEvent::PointerUp { x: 10.0, y: 10.0, button: crate::wgpu::events::PointerButton::Primary });
    assert_eq!(up, vec![ShellEvent::TabActivated { window_id: "app.viewport".into() }]);
}

#[test]
fn pointer_down_then_up_outside_the_pressed_window_cap_does_not_activate_a_tab() {
    let mut shell = Shell::new();
    shell.set_window_layout(single_window_layout("app.viewport"));
    run_layout(&mut shell);

    shell.dispatch(&UiEvent::PointerDown { x: 10.0, y: 10.0, button: crate::wgpu::events::PointerButton::Primary });
    let up = shell.dispatch(&UiEvent::PointerUp { x: -50.0, y: -50.0, button: crate::wgpu::events::PointerButton::Primary });
    assert!(up.is_empty(), "releasing outside every hit target must not activate a tab");
}

fn only_window_button(shell: &Shell) -> UiButtonNode {
    let root = shell.tree().root.expect("expected a root node");
    let tab_group = shell.tree().children(root).next().expect("expected a tab-group child");
    let window_leaf = shell.tree().children(tab_group).next().expect("expected a window leaf");
    match &shell.tree().node(window_leaf).unwrap().spec.0 {
        UiNode::Button(button) => button.clone(),
        other => panic!("expected a Button window leaf, got {other:?}"),
    }
}

#[test]
fn a_window_with_no_role_paints_no_role_chrome() {
    let mut shell = Shell::new();
    shell.set_window_layout(single_window_layout("app.viewport"));
    let button = only_window_button(&shell);
    assert_eq!(button.label.as_str(), "app.viewport");
    assert_eq!(button.icon_id, IconName::AppWindow);
}

#[test]
fn set_window_role_viewer_appends_the_title_chip_and_swaps_to_the_lock_icon() {
    let mut shell = Shell::new();
    shell.set_window_role("app.viewport", ChromeRole::Viewer);
    shell.set_window_layout(single_window_layout("app.viewport"));
    let button = only_window_button(&shell);
    assert_eq!(button.label.as_str(), "app.viewport · Viewer");
    assert_eq!(button.icon_id, IconName::Lock, "the read-only badge stands in for the window-kind icon");
}

#[test]
fn set_window_role_editor_appends_the_title_chip_but_keeps_the_window_kind_icon() {
    let mut shell = Shell::new();
    shell.set_window_kind_icons(std::collections::HashMap::from([("app.viewport".to_string(), IconName::Folder)]));
    shell.set_window_role("app.viewport", ChromeRole::Editor);
    shell.set_window_layout(single_window_layout("app.viewport"));
    let button = only_window_button(&shell);
    assert_eq!(button.label.as_str(), "app.viewport · Editor");
    assert_eq!(button.icon_id, IconName::Folder, "an editor session is not read-only, so its own window-kind icon survives");
}

#[test]
fn set_window_role_after_the_layout_is_already_set_repaints_immediately() {
    let mut shell = Shell::new();
    shell.set_window_layout(single_window_layout("app.viewport"));
    assert_eq!(only_window_button(&shell).label.as_str(), "app.viewport", "no role yet");

    shell.set_window_role("app.viewport", ChromeRole::Viewer);
    assert_eq!(only_window_button(&shell).label.as_str(), "app.viewport · Viewer", "role chrome appears without a caller re-supplying the layout");

    shell.clear_window_role("app.viewport");
    assert_eq!(only_window_button(&shell).label.as_str(), "app.viewport", "clearing the role reverts the chrome");
}

#[test]
fn set_locale_de_resolves_the_german_title_chip() {
    let mut shell = Shell::new();
    shell.set_window_role("app.viewport", ChromeRole::Viewer);
    shell.set_locale(Locale::De);
    shell.set_window_layout(single_window_layout("app.viewport"));
    assert_eq!(only_window_button(&shell).label.as_str(), "app.viewport · Betrachter");
}
