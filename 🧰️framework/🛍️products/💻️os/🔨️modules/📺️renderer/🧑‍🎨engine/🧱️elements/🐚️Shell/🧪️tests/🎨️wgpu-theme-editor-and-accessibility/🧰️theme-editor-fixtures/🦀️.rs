/// 🧰️ Theme editor fixtures: the accessibility and publication fixtures, the nested settings shell, the control-id walk and the retained control state the theme laws read.

fn accessibility_fixture() -> Value {
    serde_json::from_str(include_str!("../../../../../🧪️tests/🧫️fixtures/♿️wgpu-accessibility-interaction/🔣️.json")).expect("accessibility fixture")
}

fn shell_with_nested_app_settings() -> ShellState {
    let fixture = accessibility_fixture();
    let selection = &fixture["nestedPanelSelection"];
    let mut shell = split_pane_shell();
    SessionCtx::of(&mut shell).sync_dock_tabs();
    shell.docs.session = None;
    let branch = shell.dock.dock_tabs.tabs_mut(PanelAnchor::BottomRight).iter_mut().find(|tab| tab.id == selection["rootId"].as_str().expect("root id")).expect("settings branch");
    branch.children.insert(0, DockTabNode::leaf(selection["appFirstLeafId"].as_str().expect("app settings id"), "App Settings", "settings", -1));
    shell.dock.anchor_state_mut(PanelAnchor::BottomRight).visible = false;
    shell
}

/// 🎨️ The control ids one theme-editor build offers, in build order — the wgpu twin of reading
/// `TreeDataItem.id` off React's rendered tree.
fn theme_editor_control_ids(shell: &ShellState) -> Vec<String> {
    fn control(control: &UiControlNode, out: &mut Vec<String>) {
        match control {
            UiControlNode::Button(node) => out.extend(node.id.clone()),
            UiControlNode::Input(node) => out.push(node.id.clone()),
            UiControlNode::Select(node) => out.push(node.id.clone()),
            _ => {}
        }
    }
    fn item(row: &UiTreeItemNode, out: &mut Vec<String>) {
        if let Some(value) = row.control.as_ref() {
            control(value, out);
        } else {
            out.push(row.id.clone());
        }
        row.items.iter().flatten().for_each(|child| item(child, out));
    }
    fn walk(node: &UiNode, out: &mut Vec<String>) {
        match node {
            UiNode::Button(button) => out.extend(button.id.clone()),
            UiNode::Select(select) => out.push(select.id.clone()),
            UiNode::Input(input) => out.push(input.id.clone()),
            UiNode::Field(field) => walk(&field.child, out),
            UiNode::Stack(stack) => stack.children.iter().for_each(|child| walk(child, out)),
            UiNode::Section(section) => section.children.iter().for_each(|child| walk(child, out)),
            UiNode::Tree(tree) => tree.sections.iter().for_each(|section| {
                out.push(section.id.clone());
                section.items.iter().for_each(|row| item(row, out));
            }),
            _ => {}
        }
    }
    let mut out = Vec::new();
    walk(&shell.input.build_settings_theme_ui(&shell.chrome, &shell.prefs), &mut out);
    out
}

/// 🎨️ A shell with one theme-editor section open.
fn shell_with_open_theme_section(section: &str) -> ShellState {
    let mut shell = ShellState::new(Vec::new(), String::new(), semio_framework_ui_locale::Locale::En, semio_framework_ui_locale::Terminology::Native);
    let parts: Vec<_> = section.split('.').collect();
    let root = parts.iter().take(4).copied().collect::<Vec<_>>().join(".");
    shell.input.set_canonical_tree_open(&root, true);
    for end in 5..=parts.len() {
        shell.input.set_canonical_tree_open(&parts[..end].join("."), true);
    }
    shell
}

fn theme_publication_fixture() -> Value {
    serde_json::from_str(include_str!("../../../../../🧪️tests/🧫️fixtures/🎨️settings-theme-publication/🔣️.json")).expect("theme publication fixture")
}

fn retained_theme_control_state(shell: &ShellState, surface: &str, control_id: &str) -> String {
    let document = shell.docs.panel_documents.get(surface).expect("Theme retained document");
    let read = document.try_read().expect("Theme retained document is readable");
    let key = format!("{surface}/{control_id}");
    for ordinal in 0..read.len() {
        let Some(record) = read.node_at(ordinal) else { continue };
        if record.key.as_str() != key {
            continue;
        }
        match &record.component {
            ui_contract::Component::Select(select) => return format!("select:{}", select.value.as_str()),
            ui_contract::Component::Input(input) => return format!("input:{}", input.value.as_str()),
            ui_contract::Component::Button(_) => return format!("button:{}", if record.disabled { "disabled" } else { "enabled" }),
            _ => continue,
        }
    }
    "absent".into()
}
