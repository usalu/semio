//! ⚙️ LAW: General uses the shared Tree grammar and bottom panels reserve footer chrome below content.

use super::*;
use serde_json::Value;

fn fixture() -> Value {
    serde_json::from_str(include_str!("../../../../🧫️fixtures/⚙️settings-general-layout/🔣️.json")).expect("settings layout fixture")
}

fn locale_refresh_fixture() -> Value {
    serde_json::from_str(include_str!("../../../../🧪️fixtures/🌐️settings-locale-panel-refresh/🔣️.json")).expect("locale panel refresh fixture")
}

fn control_identity(control: &UiControlNode) -> (&str, &str) {
    match control {
        UiControlNode::Select(node) => (&node.id, &node.on_change.action),
        UiControlNode::Input(node) => (&node.id, &node.on_change.action),
        UiControlNode::Button(node) => (node.id.as_deref().unwrap_or_default(), &node.action.action),
        _ => ("", ""),
    }
}

fn retained_select_value(shell: &ShellState, surface: &str, control_id: &str) -> String {
    let document = shell.panel_documents.get(surface).expect("General retained document");
    let read = document.try_read().expect("General retained document is readable");
    let key = format!("{surface}/{control_id}");
    (0..read.len())
        .find_map(|ordinal| {
            let record = read.node_at(ordinal)?;
            if record.key.as_str() != key {
                return None;
            }
            match &record.component {
                ui_contract::Component::Select(select) => {
                    let value = select.value.as_str().to_string();
                    assert_eq!(ui_contract::accessibility_value(&record.component).text.as_deref(), Some(value.as_str()), "retained Select value drives AX valueText");
                    Some(value)
                }
                _ => None,
            }
        })
        .unwrap_or_else(|| panic!("retained Select {key}"))
}

fn retained_control_state(shell: &ShellState, surface: &str, control_id: &str) -> String {
    let document = shell.panel_documents.get(surface).expect("General retained document");
    let read = document.try_read().expect("General retained document is readable");
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

fn retained_has_record_label(shell: &ShellState, surface: &str, expected: &str) -> bool {
    let read = shell.panel_documents.get(surface).expect("mounted localized document").try_read().expect("localized document remains readable");
    (0..read.len()).any(|ordinal| {
        let Some(record) = read.node_at(ordinal) else { return false };
        matches!(&record.component, ui_contract::Component::Container(container) if container.label.as_ref().is_some_and(|label| label.0.as_str() == expected))
            || matches!(&record.component, ui_contract::Component::TreeSection(section) if section.label.as_ref().is_some_and(|label| label.0.as_str() == expected))
    })
}

fn dock_tab_label<'a>(shell: &'a ShellState, surface: &str) -> &'a str {
    let (anchor, path) = shell.dock_tabs.locate(surface).expect("localized dock tab remains mounted");
    shell.dock_tabs.node_at(anchor, &path).expect("localized dock tab path remains valid").label.as_str()
}

fn retained_header_identity(shell: &ShellState, surface: &str) -> (u64, ui_contract::UiRevision) {
    let header = shell.panel_documents.get(surface).expect("mounted localized document").header().expect("localized document header");
    (header.generation, header.revision)
}

fn dock_roster(shell: &ShellState) -> Vec<(String, String)> {
    fn collect(nodes: &[DockTabNode], roster: &mut Vec<(String, String)>) {
        for node in nodes {
            roster.push((node.id.clone(), node.label.clone()));
            collect(&node.children, roster);
        }
    }
    let mut roster = Vec::new();
    for anchor in PanelAnchor::ALL {
        collect(shell.dock_tabs.tabs(anchor), &mut roster);
    }
    roster
}

fn mount_localized_fixture_documents(shell: &mut ShellState, contract: &Value) -> Vec<String> {
    shell.session = Some(ActiveSession { plugin_id: "test".into(), instance_id: 1, app: super::command_registry_tests::test_app(Vec::new(), Vec::new()), view_state: ViewModel::default() });
    shell.sync_dock_tabs();
    let mounted = contract["mountedSurfaceIds"].as_array().unwrap().iter().map(|value| value.as_str().unwrap().to_string()).collect::<Vec<_>>();
    let roster = dock_roster(shell);
    let missing = mounted.iter().filter(|surface| !roster.iter().any(|(id, _)| id == *surface)).collect::<Vec<_>>();
    assert!(missing.is_empty(), "the real test session must mount every localized fixture leaf: missing={missing:?} roster={roster:?}");
    for surface in &mounted {
        let document = shell.publish_shell_panel_document(surface).expect("localized panel publication").expect("fixture panel owns a retained document");
        shell.panel_documents.insert(surface.clone(), document);
    }
    mounted
}

#[test]
fn general_projection_is_the_react_tree_with_inline_controls() {
    let shell = ShellState::new(Vec::new(), String::new());
    let UiNode::Tree(tree) = shell.build_settings_general_ui() else { panic!("General must publish one Tree") };
    let fixture = fixture();
    let expected_sections = fixture["tree"]["sections"].as_array().expect("fixture sections");
    assert_eq!(tree.sections.iter().map(|section| section.id.as_str()).collect::<Vec<_>>(), vec!["framework.settings.general", "framework.settings.driver.editor"]);
    for section in &tree.sections {
        let expected = expected_sections.iter().find(|row| row["id"].as_str() == Some(&section.id)).expect("known section");
        assert_eq!(section.default_open, expected["defaultOpen"].as_bool());
        let expected_ids = expected["itemIds"].as_array().expect("item ids").iter().filter_map(Value::as_str).filter(|id| *id != "framework.settings.driver.delete.action").collect::<Vec<_>>();
        assert_eq!(section.items.iter().map(|item| item.id.as_str()).collect::<Vec<_>>(), expected_ids);
    }
    let controls = tree.sections.iter().flat_map(|section| &section.items).filter_map(|item| item.control.as_ref().map(|control| (item.id.as_str(), control_identity(control)))).collect::<Vec<_>>();
    for expected in fixture["tree"]["controls"].as_array().expect("fixture controls") {
        let row_id = expected["rowId"].as_str().expect("row id");
        if row_id == "framework.settings.driver.delete.action" {
            continue;
        }
        let actual = controls.iter().find(|(id, _)| *id == row_id).unwrap_or_else(|| panic!("missing inline control row {row_id}"));
        assert_eq!(actual.1 .0, expected["controlId"].as_str().unwrap());
        assert_eq!(actual.1 .1, expected["action"].as_str().unwrap());
    }
}

#[test]
fn bottom_flow_places_content_above_root_and_leaf_tab_rows() {
    let fixture = fixture();
    let panel = &fixture["bottomPanel"]["panel"];
    let panel = Rect::new(panel["x"].as_f64().unwrap() as f32, panel["y"].as_f64().unwrap() as f32, panel["w"].as_f64().unwrap() as f32, panel["h"].as_f64().unwrap() as f32);
    let rows = fixture["bottomPanel"]["tabRows"].as_u64().unwrap() as usize;
    let row_height = fixture["bottomPanel"]["rowHeight"].as_f64().unwrap() as f32;
    let inset = fixture["bottomPanel"]["contentInset"].as_f64().unwrap() as f32;
    let expected = &fixture["bottomPanel"]["expected"];
    let content = flowed_anchor_content_rect(ui_contract::FlowBlock::Up, panel, rows as f32 * row_height, inset);
    let expected_content = &expected["content"];
    assert_eq!(content, Rect::new(expected_content["x"].as_f64().unwrap() as f32, expected_content["y"].as_f64().unwrap() as f32, expected_content["w"].as_f64().unwrap() as f32, expected_content["h"].as_f64().unwrap() as f32));
    let root_y = flowed_anchor_tab_row_y(ui_contract::FlowBlock::Up, panel, 0, rows, row_height);
    let leaf_y = flowed_anchor_tab_row_y(ui_contract::FlowBlock::Up, panel, 1, rows, row_height);
    assert_eq!(root_y, expected["rootTabY"].as_f64().unwrap() as f32);
    assert_eq!(leaf_y, expected["leafTabY"].as_f64().unwrap() as f32);
    assert_eq!(flowed_anchor_tab_divider_y(ui_contract::FlowBlock::Up, panel, rows as f32 * row_height), expected["dividerY"].as_f64().unwrap() as f32);
    assert!(content.y + content.h <= leaf_y, "retained body hits end before bottom tab/footer chrome begins");
}

#[test]
fn chrome_hosted_panel_owns_only_nested_rows_above_the_footer_root_toggle() {
    let mut shell = ShellState::new(Vec::new(), String::new());
    let anchor = PanelAnchor::BottomRight;
    shell.dock_tabs.tabs_mut(anchor).clear();
    shell.dock_tabs.tabs_mut(anchor).push(DockTabNode::branch("framework.settings", "Settings", "settings", 0, vec![DockTabNode::leaf("framework.settings.general", "General", "settings", 0)]));
    shell.anchor_state_mut(anchor).path = vec!["framework.settings".into(), "framework.settings.general".into()];
    shell.anchor_state_mut(anchor).visible = true;
    let theme = Theme::default();
    let body = shell.body_rect(&theme);
    let panel = shell.anchor_rect(anchor, body, &theme);
    let all_rows = shell.anchor_tab_rows(anchor);
    let panel_rows = shell.anchor_panel_tab_rows(anchor);
    assert_eq!(all_rows.len(), 2, "root plus selected child form the canonical path");
    assert_eq!(panel_rows.len(), 1, "the footer retains root ownership; the panel owns only the selected child row");
    let nested_y = flowed_anchor_tab_row_y(anchor.flow().block, panel, 0, panel_rows.len(), theme.control_height);
    let footer_root_y = body.y + body.h + (theme.footer_height - theme.control_height) * 0.5;
    assert!(nested_y + theme.control_height <= footer_root_y + f32::EPSILON, "panel child row ends before the footer root toggle begins");
}

#[test]
fn rendered_footer_root_closes_settings_without_selecting_or_dragging_the_pending_panel_tab() {
    let mut shell = ShellState::new(Vec::new(), String::new());
    let anchor = PanelAnchor::BottomRight;
    shell.dock_tabs.tabs_mut(anchor).clear();
    shell.dock_tabs.tabs_mut(anchor).push(DockTabNode::branch(FRAMEWORK_SETTINGS_PANEL_ID, "Settings", "settings", 0, vec![DockTabNode::leaf(FRAMEWORK_SETTINGS_GENERAL_TAB_ID, "General", "settings", 0)]));
    shell.anchor_state_mut(anchor).path = vec![FRAMEWORK_SETTINGS_PANEL_ID.into(), FRAMEWORK_SETTINGS_GENERAL_TAB_ID.into()];
    shell.anchor_state_mut(anchor).visible = true;
    shell.screen_w = 1_440.0;
    shell.screen_h = 1_000.0;

    let records = panel_ui_records(FRAMEWORK_SETTINGS_GENERAL_TAB_ID, &shell.build_settings_general_ui()).expect("General retained records");
    let document = shell.publish_surface_records(FRAMEWORK_SETTINGS_GENERAL_TAB_ID, records).expect("fresh General document");
    shell.panel_documents.insert(FRAMEWORK_SETTINGS_GENERAL_TAB_ID.into(), document);

    let theme = Theme::default();
    let body = shell.body_rect(&theme);
    let mut draw = DrawList::default();
    let mut atlas = FontAtlas::builtin();
    let icons = IconAtlas::default();
    let mut input = InputState::<ActionDescriptor>::default();
    let mut world_resources = infinite_world::world::World3dBuildContext::new(infinite_world::world::WorldCursorWakeAuthority::new());
    let mut panel_cursor = ShellChromeChildCursor::default();
    for _ in 0..9 {
        assert!(!shell.render_panel_step(&mut panel_cursor, anchor, &mut draw, None, &mut atlas, &icons, &mut input, &theme, body, &mut world_resources), "a fresh retained panel remains stepped while its document layout is pending");
    }
    assert_eq!(panel_cursor.phase, 1, "the real panel walk waits for accepted document layout before committing panel geometry");
    assert!(!shell.render_panel_step(&mut panel_cursor, anchor, &mut draw, None, &mut atlas, &icons, &mut input, &theme, body, &mut world_resources));
    assert!(!panel_cursor.document.terminal_is_complete() && !panel_cursor.document.terminal_is_fault(), "the footer is rendered while the General document is still pending");

    let mut footer_cursor = ShellChromeChildCursor::default();
    for _ in 0..4_096 {
        if shell.render_footer_step(&mut footer_cursor, &mut draw, &mut atlas, &icons, &mut input, &theme, shell.screen_w, shell.screen_h) {
            break;
        }
    }
    assert!(input.staged_hits().iter().any(|hit| hit.kind == HitKind::Toggle && hit.control_id.as_deref() == Some(FRAMEWORK_SETTINGS_PANEL_ID)), "the complete footer remains staged while panel layout is pending");
    assert!(input.hits().is_empty(), "an incomplete frame cannot promote even its completed footer");
    assert!(input.staged_hits().iter().all(|hit| hit.kind != HitKind::PanelTab || hit.control_id.as_deref() != Some(FRAMEWORK_SETTINGS_GENERAL_TAB_ID)), "a pending panel stages no premature leaf hit");

    let panel_complete = (0..SHELL_WINDOW_PAINT_OPPORTUNITIES.min(1 << 20)).any(|_| shell.render_panel_step(&mut panel_cursor, anchor, &mut draw, None, &mut atlas, &icons, &mut input, &theme, body, &mut world_resources));
    assert!(panel_complete, "the same pending panel cursor reaches accepted layout and paints");
    let mut settled_footer_cursor = ShellChromeChildCursor::default();
    let footer_complete = (0..4_096).any(|_| shell.render_footer_step(&mut settled_footer_cursor, &mut draw, &mut atlas, &icons, &mut input, &theme, shell.screen_w, shell.screen_h));
    assert!(footer_complete, "the settled frame paints its footer");
    shell.publish_retained_hit_registry(&mut input);
    let root = input.hits().iter().find(|hit| hit.kind == HitKind::Toggle && hit.control_id.as_deref() == Some(FRAMEWORK_SETTINGS_PANEL_ID)).expect("rendered footer Settings toggle");
    let leaf = input.hits().iter().find(|hit| hit.kind == HitKind::PanelTab && hit.control_id.as_deref() == Some(FRAMEWORK_SETTINGS_GENERAL_TAB_ID)).expect("rendered General panel tab after accepted layout");
    assert!(root.rect.y + root.rect.h <= leaf.rect.y || leaf.rect.y + leaf.rect.h <= root.rect.y, "the actual rendered Toggle and PanelTab hit rectangles never overlap");
    let (x, y) = (root.rect.x + root.rect.w * 0.5, root.rect.y + root.rect.h * 0.5);

    semio_framework_async::block_on(shell.handle_pointer_button(x, y, true, 0, &mut input, &theme)).expect("footer Settings press");
    semio_framework_async::block_on(shell.handle_pointer_button(x, y, false, 0, &mut input, &theme)).expect("footer Settings release");
    assert!(!shell.anchor_open(anchor), "the physical footer root gesture closes its anchor");
    assert_eq!(shell.chrome_accessibility_selected(FRAMEWORK_SETTINGS_GENERAL_TAB_ID, &HitKind::PanelTab), Some(false), "closing the root leaves no selected panel tab");
    assert!(shell.dock_tab_drag.is_none() && !input.drag.active, "the footer root gesture never arms a dock-tab drag");
}

#[test]
fn accepted_general_tree_hugs_the_bottom_anchor_on_the_next_shell_panel_walk() {
    const SURFACE: &str = "framework.settings.general.compact-law";
    let fixture = fixture();
    let vector = &fixture["stableGeneral"];
    let mut shell = ShellState::new(Vec::new(), String::new());
    let anchor = PanelAnchor::BottomRight;
    shell.screen_w = vector["screen"]["width"].as_f64().unwrap() as f32;
    shell.screen_h = vector["screen"]["height"].as_f64().unwrap() as f32;
    shell.dock_tabs.tabs_mut(anchor).clear();
    shell.dock_tabs.tabs_mut(anchor).push(DockTabNode::branch(FRAMEWORK_SETTINGS_PANEL_ID, "Settings", "settings", 0, vec![DockTabNode::leaf(SURFACE, "General", "settings", 0)]));
    shell.anchor_state_mut(anchor).path = vec![FRAMEWORK_SETTINGS_PANEL_ID.into(), SURFACE.into()];
    shell.anchor_state_mut(anchor).visible = true;
    let records = panel_ui_records(SURFACE, &shell.build_settings_general_ui()).expect("actual General producer records");
    assert!(matches!(records.first().map(|record| &record.component), Some(ui_contract::Component::Tree(_))), "the published General root record is Tree");
    let document = shell.publish_surface_records(SURFACE, records).expect("General production ingress lease");
    shell.panel_documents.insert(SURFACE.into(), document);

    let theme = Theme::default();
    let body = shell.body_rect(&theme);
    let full_band = ShellState::panel_band(anchor, body, &theme);
    let first_panel = shell.anchor_rect(anchor, body, &theme);
    assert!((first_panel.h - (full_band.h - theme.panel_inset * 2.0)).abs() < 0.01, "first-frame fallback owns the full available anchor band before accepted layout");

    let icons = IconAtlas::default();
    let mut atlas = FontAtlas::builtin();
    let mut first_draw = DrawList::default();
    first_draw.set_screen_height(shell.screen_h);
    let mut first_input = InputState::<ActionDescriptor>::default();
    let mut resources = infinite_world::world::World3dBuildContext::new(infinite_world::world::WorldCursorWakeAuthority::new());
    let mut first_cursor = ShellChromeChildCursor::default();
    let first_complete = (0..SHELL_WINDOW_PAINT_OPPORTUNITIES.min(1 << 20)).any(|_| shell.render_panel_step(&mut first_cursor, anchor, &mut first_draw, None, &mut atlas, &icons, &mut first_input, &theme, body, &mut resources));
    assert!(first_complete, "normal General ingress, reconcile, layout and paint complete");
    assert!(crate::interpreter::retained_root_is_tree(SURFACE), "the accepted production root remains Tree");
    let intrinsic = crate::interpreter::retained_content_height(SURFACE).expect("accepted General intrinsic height");
    let minimum_unused = vector["minimumUnusedBandHeight"].as_f64().unwrap() as f32;
    assert!(intrinsic + shell.anchor_tab_bar_height(anchor, &theme) + theme.gap_standard * 2.0 + minimum_unused < first_panel.h, "fixture keeps a material unused band below the compact General extent: intrinsic={intrinsic} band={}", first_panel.h);
    let first_compact = first_cursor.rect.expect("accepted first-walk panel rectangle");
    assert!(first_compact.h + minimum_unused < first_panel.h, "the ingress walk reflows before it commits visible panel geometry");
    assert_eq!(first_draw.glass_regions.first().expect("first-walk General glass").rect, [first_compact.x, first_compact.y, first_compact.w, first_compact.h]);

    shell.retained_hit_windows_staging.clear();
    let mut draw = DrawList::default();
    draw.set_screen_height(shell.screen_h);
    let mut input = InputState::<ActionDescriptor>::default();
    let mut cursor = ShellChromeChildCursor::default();
    let complete = (0..SHELL_WINDOW_PAINT_OPPORTUNITIES.min(1 << 20)).any(|_| shell.render_panel_step(&mut cursor, anchor, &mut draw, None, &mut atlas, &icons, &mut input, &theme, body, &mut resources));
    assert!(complete, "a new Shell panel cursor completes against the accepted General layout");
    let compact = cursor.rect.expect("second panel rectangle");
    let glass = draw.glass_regions.first().expect("General panel glass").rect;
    assert_eq!(glass, [compact.x, compact.y, compact.w, compact.h]);
    assert!(compact.h + minimum_unused < first_panel.h, "second Shell walk hugs accepted General content");
    assert!((compact.y + compact.h - (first_panel.y + first_panel.h)).abs() < 0.01, "bottom panel keeps the same bonded edge while it compacts upward");
    let content = shell.anchor_content_rect(anchor, compact, &theme);
    let owners = shell.retained_hit_windows_staging.values().filter(|(owner, _)| owner == SURFACE).collect::<Vec<_>>();
    assert!(!owners.is_empty() && owners.iter().all(|(_, rect)| *rect == content), "every retained General target is owned by the compact content rectangle");
    shell.publish_retained_hit_registry(&mut input);
    let accessibility = crate::interpreter::published_accessibility_nodes_for_test(SURFACE);
    for (key, label) in [
        ("framework.settings.appearance", "Appearance"),
        ("framework.settings.layout", "Layout"),
        ("framework.settings.driver", "Driver"),
        ("framework.settings.language", "Language"),
        ("framework.settings.terminology", "Terminology"),
        ("framework.settings.mergePolicy", "Merge policy"),
    ] {
        let retained_key = format!("{SURFACE}/{key}");
        assert!(accessibility.iter().any(|node| node.key == retained_key && node.role == "treeitem" && node.label.as_deref() == Some(label)), "accepted General TreeItem {key} projects its intrinsic label {label}");
    }
    let control_suffix = vector["controlId"].as_str().unwrap();
    let live = input.hits().iter().find(|hit| hit.control_id.as_deref().is_some_and(|id| id.ends_with(control_suffix))).expect("actual General control hit");
    assert!(content.contains(live.rect.x + live.rect.w * 0.5, live.rect.y + live.rect.h * 0.5));
    let unused = (first_panel.x + theme.gap_standard, first_panel.y + theme.gap_standard);
    assert!(!content.contains(unused.0, unused.1));
    assert!(input.hit_at(unused.0, unused.1).is_none(), "the unused former full-height band has no General target");
}

#[test]
fn appearance_option_commit_uses_the_retained_router_and_retires_its_popup() {
    const SURFACE: &str = "framework.settings.general.appearance-commit-law";
    let fixture = fixture();
    let vector = &fixture["selectCommit"];
    let control_id = vector["controlId"].as_str().unwrap();
    let option_id = vector["optionControlId"].as_str().unwrap();
    let initial_value = vector["initialValue"].as_str().unwrap();
    let next_value = vector["nextValue"].as_str().unwrap();
    let expected_action = vector["action"].as_str().unwrap();
    let mut shell = ShellState::new(Vec::new(), String::new());
    let anchor = PanelAnchor::BottomRight;
    shell.screen_w = 1_440.0;
    shell.screen_h = 1_000.0;
    shell.dock_tabs.tabs_mut(anchor).clear();
    shell.dock_tabs.tabs_mut(anchor).push(DockTabNode::branch(FRAMEWORK_SETTINGS_PANEL_ID, "Settings", "settings", 0, vec![DockTabNode::leaf(SURFACE, "General", "settings", 0)]));
    shell.anchor_state_mut(anchor).path = vec![FRAMEWORK_SETTINGS_PANEL_ID.into(), SURFACE.into()];
    shell.anchor_state_mut(anchor).visible = true;
    let initial = shell.build_settings_general_ui();
    let UiNode::Tree(initial_tree) = &initial else { panic!("General root remains Tree") };
    let initial_select = initial_tree
        .sections
        .iter()
        .flat_map(|section| &section.items)
        .filter_map(|item| item.control.as_ref())
        .find_map(|control| match control {
            UiControlNode::Select(select) if select.id == control_id => Some(select),
            _ => None,
        })
        .expect("appearance Select producer");
    assert_eq!(initial_select.value, initial_value);
    let records = panel_ui_records(SURFACE, &initial).expect("General retained records");
    let document = shell.publish_surface_records(SURFACE, records).expect("General production ingress");
    shell.panel_documents.insert(SURFACE.into(), document);

    let icons = IconAtlas::default();
    let mut atlas = FontAtlas::builtin();
    let mut resources = infinite_world::world::World3dBuildContext::new(infinite_world::world::WorldCursorWakeAuthority::new());
    let mut input = InputState::<ActionDescriptor>::default();
    let body = shell.body_rect(&Theme::default());
    let mut paint = |shell: &mut ShellState, input: &mut InputState<ActionDescriptor>, theme: &Theme| {
        while input.retire_hit_step() {}
        let mut draw = DrawList::default();
        draw.set_screen_height(shell.screen_h);
        let mut cursor = ShellChromeChildCursor::default();
        let complete = (0..SHELL_WINDOW_PAINT_OPPORTUNITIES.min(1 << 20)).any(|_| shell.render_panel_step(&mut cursor, anchor, &mut draw, None, &mut atlas, &icons, input, theme, body, &mut resources));
        assert!(complete, "General panel ingress/layout/paint completes");
        shell.publish_retained_hit_registry(input);
        draw
    };

    let theme = Theme::default();
    let _ = paint(&mut shell, &mut input, &theme);
    let retained_control_id = format!("{SURFACE}/{control_id}");
    let trigger = input.hits().iter().find(|hit| hit.kind == HitKind::Select && hit.control_id.as_deref() == Some(retained_control_id.as_str())).cloned().expect("published appearance Select trigger");
    let inline = &fixture["inlineControls"];
    let select_geometry = inline["cases"].as_array().and_then(|cases| cases.iter().find(|case| case["kind"] == "select")).expect("neutral Select geometry");
    assert!((trigger.rect.w - select_geometry["expectedWidthPx"].as_f64().unwrap() as f32).abs() < 0.01, "the retained Select fills React's token-backed Tree value column");
    assert!((trigger.rect.h - select_geometry["expectedHeightPx"].as_f64().unwrap() as f32).abs() < 0.01, "the retained Select keeps React's authored small height");
    assert_eq!(shell.retained_hit_window(&trigger).map(|(owner, _)| owner), Some(SURFACE.to_string()), "the qualified Select address resolves to its General surface owner");
    let (trigger_x, trigger_y) = (trigger.rect.x + trigger.rect.w * 0.5, trigger.rect.y + trigger.rect.h * 0.5);
    semio_framework_async::block_on(shell.handle_pointer_button(trigger_x, trigger_y, true, 0, &mut input, &theme)).expect("appearance trigger press");
    semio_framework_async::block_on(shell.handle_pointer_button(trigger_x, trigger_y, false, 0, &mut input, &theme)).expect("appearance trigger release");

    let _ = paint(&mut shell, &mut input, &theme);
    let option = input.hits().iter().find(|hit| hit.kind == HitKind::Button && hit.control_id.as_deref() == Some(option_id)).cloned().expect("open appearance popup publishes the dark option as one retained Button");
    assert_eq!(shell.retained_hit_window(&option).map(|(owner, _)| owner), Some(SURFACE.to_string()), "the popup option stays owned by the same General surface");
    let (option_x, option_y) = (option.rect.x + option.rect.w * 0.5, option.rect.y + option.rect.h * 0.5);
    let underlying = input.hits().iter().find(|hit| hit.kind == HitKind::Select && hit.rect.contains(option_x, option_y)).expect("the fixture keeps a real underlying control beneath the popup option");
    assert_ne!(underlying.control_id.as_deref(), Some(option_id));
    assert_eq!(input.hit_at(option_x, option_y).and_then(|hit| hit.control_id.as_deref()), Some(option_id), "the overlay option outranks the overlapping underlying Select in the host registry");
    semio_framework_async::block_on(shell.handle_pointer_button(option_x, option_y, true, 0, &mut input, &theme)).expect("appearance option press");

    let refreshed = shell.build_settings_general_ui();
    let refreshed_records = panel_ui_records(SURFACE, &refreshed).expect("General refresh records while the option owns pointer capture");
    let refreshed_document = shell.publish_surface_records(SURFACE, refreshed_records).expect("normal General refresh while the option owns pointer capture");
    shell.panel_documents.insert(SURFACE.into(), refreshed_document);
    let _ = paint(&mut shell, &mut input, &theme);
    let captured_option = input.hits().iter().find(|hit| hit.kind == HitKind::Button && hit.control_id.as_deref() == Some(option_id)).cloned().expect("the pressed option remains published until its captured release");
    assert_eq!(captured_option.rect, option.rect, "the interleaved Shell walk retains the exact pressed option geometry");

    semio_framework_async::block_on(shell.handle_pointer_button(option_x, option_y, false, 0, &mut input, &theme)).expect("appearance option release");

    assert_eq!(shell.appearance_id, initial_value, "the retained gesture queues its app command before Shell mutation");
    let actions = crate::collect_fixture_actions(&mut input);
    assert_eq!(actions.len(), vector["expectedQueuedActions"].as_u64().unwrap() as usize, "one retained option activation publishes one bounded action");
    let action = actions.into_iter().next().unwrap();
    assert_eq!(action.controller_id, "framework");
    assert_eq!(action.action, expected_action);
    assert_eq!(action.args.as_ref().and_then(|args| args.get("value")).and_then(DslValue::as_str), Some(next_value));
    assert_eq!(action.args.as_ref().and_then(|args| args.get("windowId")).and_then(DslValue::as_str), Some(SURFACE));
    semio_framework_async::block_on(shell.dispatch_action(action)).expect("the normal frame action funnel applies appearance");
    assert_eq!(shell.appearance_id, next_value, "Shell root state accepts the appearance action");
    let next_theme = resolve_theme_for_ids("semio", &shell.appearance_id);
    assert_eq!(next_theme.background, Theme::dark().background, "the next paint resolves through the newly accepted appearance");

    let updated = shell.build_settings_general_ui();
    let UiNode::Tree(updated_tree) = &updated else { panic!("updated General root remains Tree") };
    let selected = updated_tree.sections.iter().flat_map(|section| &section.items).filter_map(|item| item.control.as_ref()).find_map(|control| match control {
        UiControlNode::Select(select) if select.id == control_id => Some(select.value.as_str()),
        _ => None,
    });
    assert_eq!(selected, Some(next_value), "the next General projection reflects the committed appearance");
    let records = panel_ui_records(SURFACE, &updated).expect("updated General records");
    let document = shell.publish_surface_records(SURFACE, records).expect("updated General ingress");
    shell.panel_documents.insert(SURFACE.into(), document);
    let _ = paint(&mut shell, &mut input, &next_theme);
    let open_options = input.hits().iter().filter(|hit| hit.kind == HitKind::Button && matches!(hit.control_id.as_deref(), Some("system" | "light" | "dark"))).count();
    assert_eq!(open_options, vector["expectedOpenOptionsAfterCommit"].as_u64().unwrap() as usize, "the same commit retires every popup option before the next paint");

    let ordinary_control_id = format!("{SURFACE}/framework.settings.resetDock");
    let ordinary = input.hits().iter().find(|hit| hit.kind == HitKind::Button && hit.control_id.as_deref() == Some(ordinary_control_id.as_str())).cloned().expect("ordinary retained General Button");
    assert_eq!(shell.retained_hit_window(&ordinary).map(|(owner, _)| owner), Some(SURFACE.to_string()), "the qualified ordinary Button address resolves to its General surface owner");
    let (ordinary_x, ordinary_y) = (ordinary.rect.x + ordinary.rect.w * 0.5, ordinary.rect.y + ordinary.rect.h * 0.5);
    semio_framework_async::block_on(shell.handle_pointer_button(ordinary_x, ordinary_y, true, 0, &mut input, &next_theme)).expect("ordinary retained Button press");
    semio_framework_async::block_on(shell.handle_pointer_button(ordinary_x, ordinary_y, false, 0, &mut input, &next_theme)).expect("ordinary retained Button release");
    let ordinary_actions = crate::collect_fixture_actions(&mut input);
    assert_eq!(ordinary_actions.len(), 1, "ordinary retained Buttons keep one EventRouter action path");
    assert_eq!(ordinary_actions[0].action, "runOsCommand");
}

#[test]
fn an_open_up_flow_disclosure_keeps_its_header_below_its_children_and_retires_them_when_closed() {
    const SURFACE: &str = "framework.settings.general.driver-retirement-law";
    let neutral = fixture();
    let geometry = &neutral["upFlowDisclosure"];
    let section = &geometry["sectionRect"];
    let expected = &geometry["expectedHeaderRect"];
    let expected_y = section["y"].as_f64().unwrap() + section["h"].as_f64().unwrap() - geometry["headerHeight"].as_f64().unwrap();
    assert_eq!(expected["y"].as_f64(), Some(expected_y), "the neutral Up-flow oracle bonds the disclosure header to the section bottom");
    let mut shell = ShellState::new(Vec::new(), String::new());
    let anchor = PanelAnchor::BottomRight;
    shell.screen_w = 1_440.0;
    shell.screen_h = 1_000.0;
    shell.dock_tabs.tabs_mut(anchor).clear();
    shell.dock_tabs.tabs_mut(anchor).push(DockTabNode::branch(FRAMEWORK_SETTINGS_PANEL_ID, "Settings", "settings", 0, vec![DockTabNode::leaf(SURFACE, "General", "settings", 0)]));
    shell.anchor_state_mut(anchor).path = vec![FRAMEWORK_SETTINGS_PANEL_ID.into(), SURFACE.into()];
    shell.anchor_state_mut(anchor).visible = true;
    let records = panel_ui_records(SURFACE, &shell.build_settings_general_ui()).expect("General retained records");
    let document = shell.publish_surface_records(SURFACE, records).expect("General production ingress");
    shell.panel_documents.insert(SURFACE.into(), document);

    let icons = IconAtlas::default();
    let mut atlas = FontAtlas::builtin();
    let mut resources = infinite_world::world::World3dBuildContext::new(infinite_world::world::WorldCursorWakeAuthority::new());
    let mut input = InputState::<ActionDescriptor>::default();
    let theme = Theme::default();
    let body = shell.body_rect(&theme);
    let mut paint = |shell: &mut ShellState, input: &mut InputState<ActionDescriptor>| {
        while input.retire_hit_step() {}
        shell.retained_hit_windows_staging.clear();
        let mut draw = DrawList::default();
        draw.set_screen_height(shell.screen_h);
        let mut cursor = ShellChromeChildCursor::default();
        let complete = (0..SHELL_WINDOW_PAINT_OPPORTUNITIES.min(1 << 20)).any(|_| shell.render_panel_step(&mut cursor, anchor, &mut draw, None, &mut atlas, &icons, input, &theme, body, &mut resources));
        assert!(complete, "General panel ingress/layout/paint completes");
        shell.publish_retained_hit_registry(input);
    };
    let section_id = format!("section.chevron.{SURFACE}/framework.settings.driver.editor");
    let child_id = format!("{SURFACE}/framework.settings.driver.saveLabel");
    let section_hit = |input: &InputState<ActionDescriptor>| input.hits().iter().find(|hit| hit.control_id.as_deref() == Some(section_id.as_str())).cloned().expect("Drivers disclosure hit");

    paint(&mut shell, &mut input);
    assert!(!input.hits().iter().any(|hit| hit.control_id.as_deref() == Some(child_id.as_str())), "Drivers starts closed with no child hit");
    let header = section_hit(&input);
    let point = (header.rect.x + header.rect.w * 0.5, header.rect.y + header.rect.h * 0.5);
    semio_framework_async::block_on(shell.handle_pointer_button(point.0, point.1, true, 0, &mut input, &theme)).expect("Drivers open press");
    semio_framework_async::block_on(shell.handle_pointer_button(point.0, point.1, false, 0, &mut input, &theme)).expect("Drivers open release");
    paint(&mut shell, &mut input);
    paint(&mut shell, &mut input);
    let child = input.hits().iter().find(|hit| hit.control_id.as_deref() == Some(child_id.as_str())).cloned().expect("opening Drivers publishes its real save-label child");

    let header = section_hit(&input);
    assert!(header.rect.y >= child.rect.y + child.rect.h, "an expanded Up-flow section keeps its disclosure header below every child: header={:?} child={:?}", header.rect, child.rect);
    let point = (header.rect.x + header.rect.w * 0.5, header.rect.y + header.rect.h * 0.5);
    let top = input.hit_at(point.0, point.1).expect("the painted Drivers header center remains targetable");
    assert_eq!(top.control_id.as_deref(), Some(section_id.as_str()), "the disclosure header, rather than an expanded child, owns its painted center");
    semio_framework_async::block_on(shell.handle_pointer_button(point.0, point.1, true, 0, &mut input, &theme)).expect("Drivers close press");
    semio_framework_async::block_on(shell.handle_pointer_button(point.0, point.1, false, 0, &mut input, &theme)).expect("Drivers close release");
    paint(&mut shell, &mut input);
    paint(&mut shell, &mut input);
    assert!(!input.hits().iter().any(|hit| hit.control_id.as_deref() == Some(child_id.as_str())), "closing Drivers retires its child from the host hit registry");
    assert!(!shell.retained_hit_windows.contains_key(&child_id), "the atomic retained owner map retires the same child generation");
}

#[test]
fn host_preference_dispatch_republishes_general_without_a_guest_refresh() {
    let fixture = fixture();
    let contract = &fixture["retainedPreferencePublication"];
    let surface = contract["surfaceId"].as_str().unwrap();
    assert!(!contract["requiresGuestRefresh"].as_bool().unwrap());
    let mut shell = ShellState::new(Vec::new(), String::new());
    shell.session = Some(ActiveSession { plugin_id: "test".into(), instance_id: 1, app: super::command_registry_tests::test_app(Vec::new(), Vec::new()), view_state: ViewModel::default() });
    shell.chrome_present.maintenance.load_requested = false;
    let initial = shell.publish_shell_panel_document(surface).expect("initial General publication").expect("General owns a retained document");
    shell.panel_documents.insert(surface.to_string(), initial);

    for vector in contract["cases"].as_array().unwrap() {
        let action = vector["action"].as_str().unwrap();
        let publication_lane = vector["publicationLane"].as_str().unwrap();
        let control_id = vector["controlId"].as_str().unwrap();
        let initial_value = vector["initialValue"].as_str().unwrap();
        let next_value = vector["nextValue"].as_str().unwrap();
        assert_eq!(retained_select_value(&shell, surface, control_id), initial_value);
        let before = shell.panel_documents.get(surface).unwrap().header().expect("initial General header");

        semio_framework_async::block_on(shell.dispatch_action(ActionDescriptor { controller_id: "framework".into(), action: action.into(), args: crate::action_args_json!({ "value": next_value }) })).expect("host preference dispatch");

        let after_dispatch = shell.panel_documents.get(surface).expect("dispatch keeps General published").header().expect("General header after dispatch");
        if publication_lane == "maintenance" {
            assert_eq!(after_dispatch, before, "{action} keeps the exact readable owner until bounded maintenance");
            let cursor = shell.chrome_present.maintenance.locale_refresh.as_ref().expect("locale dispatch arms mounted-owner maintenance");
            assert_eq!(cursor.pending.front().map(String::as_str), Some(surface));
            shell.advance_chrome_maintenance_step();
        } else {
            assert!(after_dispatch.revision != before.revision && after_dispatch.generation > before.generation, "{action} republishes in dispatch");
        }
        let after = shell.panel_documents.get(surface).expect("preference keeps General published").header().expect("updated General header");
        assert!(after.revision != before.revision && after.generation > before.generation, "{action} publishes through its declared lane");
        assert_eq!(retained_select_value(&shell, surface, control_id), next_value, "{action} publishes the accepted value without guest refresh");
        assert!(shell.closing_documents.terminal_is_empty(), "{action} retires the replaced General source lease");
        assert!(shell.owed_refresh_scope.asks_for_nothing(), "{action} does not broaden into a guest refresh");
    }
}

#[test]
fn driver_draft_save_and_reset_republish_every_retained_general_field() {
    let fixture = fixture();
    let contract = &fixture["retainedPreferencePublication"];
    let surface = contract["surfaceId"].as_str().unwrap();
    let mut shell = ShellState::new(Vec::new(), String::new());
    shell.driver_id = "compact".into();
    shell.clear_driver_draft();
    let initial = shell.publish_shell_panel_document(surface).expect("initial General publication").expect("General owns a retained document");
    shell.panel_documents.insert(surface.to_string(), initial);
    let mut missing = Vec::new();

    for vector in contract["driverWorkflow"].as_array().unwrap() {
        let action = vector["action"].as_str().unwrap();
        let before = shell.panel_documents.get(surface).unwrap().header().expect("General header before driver mutation");
        semio_framework_async::block_on(shell.dispatch_action(ActionDescriptor { controller_id: "framework".into(), action: action.into(), args: semio_framework::optional_json_to_dsl(Some(vector["arguments"].clone())) }))
            .expect("driver preference dispatch");
        let after = shell.panel_documents.get(surface).unwrap().header().expect("General header after driver mutation");
        if after.revision == before.revision || after.generation <= before.generation {
            missing.push(action.to_string());
            shell.republish_shell_panel_document(surface).expect("test continuation republishes the missing driver successor");
        }
        for observation in vector["observations"].as_array().unwrap() {
            let control_id = observation["controlId"].as_str().unwrap();
            assert_eq!(retained_control_state(&shell, surface, control_id), observation["expectedState"].as_str().unwrap(), "{action}:{control_id}");
        }
        assert!(shell.closing_documents.terminal_is_empty(), "{action} retires the exact replaced General owner");
        assert!(shell.owed_refresh_scope.asks_for_nothing(), "{action} stays host-owned");
    }
    assert!(missing.is_empty(), "driver mutations missing retained General publication: {missing:?}");
}

#[test]
fn general_republication_refusal_preserves_the_exact_readable_owner_and_retries() {
    let mut shell = ShellState::new(Vec::new(), String::new());
    let surface = FRAMEWORK_SETTINGS_GENERAL_TAB_ID;
    let initial = shell.publish_shell_panel_document(surface).expect("initial General publication").expect("General retained document");
    let before = initial.header().expect("initial General header");
    shell.panel_documents.insert(surface.into(), initial);
    let vacancy = shell.closing_documents.first_vacant_index().expect("retirement vacancy");
    shell.closing_documents.epochs[vacancy] = u64::MAX;

    let error = semio_framework_async::block_on(shell.dispatch_action(ActionDescriptor { controller_id: "framework".into(), action: "setAppearance".into(), args: crate::action_args_json!({ "value": "dark" }) }))
        .expect_err("exhausted retirement admission refuses replacement");
    assert!(error.contains("retirement registry refused"));
    let retained = shell.panel_documents.get(surface).expect("refusal keeps the readable General owner");
    assert_eq!(retained.header().expect("retained General header"), before, "refusal preserves exact generation and revision");
    assert_eq!(retained_select_value(&shell, surface, "framework.settings.appearance"), "system");

    shell.closing_documents.epochs[vacancy] = 0;
    semio_framework_async::block_on(shell.dispatch_action(ActionDescriptor { controller_id: "framework".into(), action: "setAppearance".into(), args: crate::action_args_json!({ "value": "dark" }) }))
        .expect("the exact refused preference retries after retirement admission returns");
    let successor = shell.panel_documents.get(surface).unwrap().header().expect("successor General header");
    assert!(successor.generation > before.generation);
    assert_ne!(successor.revision, before.revision);
    assert_eq!(retained_select_value(&shell, surface, "framework.settings.appearance"), "dark");
    assert!(shell.closing_documents.terminal_is_empty());
}

#[test]
fn locale_refresh_republishes_one_exact_mounted_shell_owner_per_maintenance_step() {
    let contract = locale_refresh_fixture();
    assert_eq!(contract["maxPanelsPerStep"].as_u64(), Some(1));
    assert_eq!(contract["requiresGuestRefresh"].as_bool(), Some(false));
    let mut shell = ShellState::new(Vec::new(), String::new());
    shell.chrome_present.maintenance.load_requested = false;
    shell.locale_id = contract["initialLocale"].as_str().unwrap().into();
    let mounted = mount_localized_fixture_documents(&mut shell, &contract);
    let unmounted = contract["unmountedSurfaceIds"].as_array().unwrap().iter().map(|value| value.as_str().unwrap()).collect::<Vec<_>>();
    let initial_headers = mounted.iter().map(|surface| (surface.clone(), retained_header_identity(&shell, surface))).collect::<HashMap<_, _>>();

    semio_framework_async::block_on(shell.dispatch_action(ActionDescriptor { controller_id: "framework".into(), action: "setLocale".into(), args: crate::action_args_json!({ "value": contract["nextLocale"].as_str().unwrap() }) }))
        .expect("locale host mutation arms retained refresh");

    assert_eq!(dock_tab_label(&shell, FRAMEWORK_SETTINGS_GENERAL_TAB_ID), "Allgemein");
    assert_eq!(dock_tab_label(&shell, FRAMEWORK_SETTINGS_THEME_TAB_ID), "Thema");
    assert_eq!(dock_tab_label(&shell, FRAMEWORK_SETTINGS_KEYBINDINGS_TAB_ID), "Tastenkürzel");

    let cursor = shell.chrome_present.maintenance.locale_refresh.as_ref().expect("mounted localized roster cursor");
    assert_eq!(cursor.locale_id, contract["nextLocale"].as_str().unwrap());
    assert_eq!(cursor.pending.iter().cloned().collect::<Vec<_>>(), mounted, "the cursor snapshots the exact mounted shell-owned inventory");
    assert!(unmounted.iter().all(|surface| !shell.panel_documents.contains_key(*surface)), "locale refresh never creates an unmounted leaf");
    assert!(mounted.iter().all(|surface| retained_header_identity(&shell, surface) == initial_headers[surface]), "dispatch itself performs no unbounded publication loop");

    for (index, surface) in mounted.iter().enumerate() {
        shell.advance_chrome_maintenance_step();
        for (candidate_index, candidate) in mounted.iter().enumerate() {
            let current = retained_header_identity(&shell, candidate);
            if candidate_index <= index {
                assert!(current.0 > initial_headers[candidate].0 && current.1 != initial_headers[candidate].1, "step {index} has refreshed {candidate}");
            } else {
                assert_eq!(current, initial_headers[candidate], "step {index} leaves later owner {candidate} untouched");
            }
        }
        assert!(retained_header_identity(&shell, surface).0 > initial_headers[surface].0);
        assert!(shell.closing_documents.terminal_is_empty(), "each replaced locale owner retires before the next step");
    }
    assert!(shell.chrome_present.maintenance.locale_refresh.is_none());
    assert!(retained_has_record_label(&shell, FRAMEWORK_SETTINGS_GENERAL_TAB_ID, "Allgemein"));
    assert!(retained_has_record_label(&shell, FRAMEWORK_SETTINGS_THEME_TAB_ID, "Design"));
    assert!(retained_has_record_label(&shell, FRAMEWORK_SETTINGS_KEYBINDINGS_TAB_ID, "Tastenkürzel"));
    assert!(shell.owed_refresh_scope.asks_for_nothing(), "host localization never requests a guest refresh");
}

#[test]
fn locale_and_terminology_changes_require_one_full_guest_refresh_and_settle() {
    let contract = locale_refresh_fixture();
    assert_eq!(contract["requiresGuestRefresh"].as_bool(), Some(true));
    for (action, value) in [("setLocale", contract["nextLocale"].as_str().unwrap()), ("setTerminology", "de")] {
        let mut shell = ShellState::new(Vec::new(), String::new());
        shell.session = Some(ActiveSession { plugin_id: "test".into(), instance_id: 1, app: super::command_registry_tests::test_app(Vec::new(), Vec::new()), view_state: ViewModel::default() });
        shell.chrome_present.maintenance.load_requested = false;
        semio_framework_async::block_on(shell.dispatch_action(ActionDescriptor { controller_id: "framework".into(), action: action.into(), args: crate::action_args_json!({ "value": value }) }))
            .expect("locale-bearing host mutation");
        assert!(matches!(shell.owed_refresh_scope, semio_framework::kernel::UiDirtyScope::Full), "{action} must rebuild guest bodies and Window Measures from the new ViewModel axes");
        assert!(shell.settle_pump_pending(), "{action} arms the bounded settle owner for the full guest refresh");
        assert!(shell.chrome_present.maintenance.locale_refresh.is_none(), "{action} must not schedule a duplicate shell-only locale publication lane");
    }
}

#[test]
fn locale_refresh_refusal_and_supersession_preserve_exact_owner_and_generation() {
    let contract = locale_refresh_fixture();
    let mut shell = ShellState::new(Vec::new(), String::new());
    shell.chrome_present.maintenance.load_requested = false;
    shell.locale_id = "en".into();
    let mounted = mount_localized_fixture_documents(&mut shell, &contract);
    let first = mounted.first().unwrap();
    let prior = retained_header_identity(&shell, first);
    semio_framework_async::block_on(shell.dispatch_action(ActionDescriptor { controller_id: "framework".into(), action: "setLocale".into(), args: crate::action_args_json!({ "value": "de" }) })).expect("German cursor");
    let first_generation = shell.chrome_present.maintenance.locale_refresh.as_ref().unwrap().generation;
    let vacancy = shell.closing_documents.first_vacant_index().expect("retirement vacancy");
    shell.closing_documents.epochs[vacancy] = u64::MAX;
    shell.advance_chrome_maintenance_step();
    assert_eq!(retained_header_identity(&shell, first), prior, "admission refusal keeps the exact readable prior owner");
    assert_eq!(shell.chrome_present.maintenance.locale_refresh.as_ref().unwrap().pending.front(), Some(first), "refusal retries the same cursor head");

    shell.closing_documents.epochs[vacancy] = 0;
    shell.advance_chrome_maintenance_step();
    assert!(retained_header_identity(&shell, first).0 > prior.0, "returned admission refreshes the refused owner");
    semio_framework_async::block_on(shell.dispatch_action(ActionDescriptor { controller_id: "framework".into(), action: "setLocale".into(), args: crate::action_args_json!({ "value": "en" }) })).expect("newer English cursor supersedes German work");
    let successor = shell.chrome_present.maintenance.locale_refresh.as_ref().unwrap();
    assert!(successor.generation > first_generation);
    assert_eq!(successor.locale_id, "en");
    assert_eq!(successor.pending.iter().cloned().collect::<Vec<_>>(), mounted, "supersession restarts the exact current mounted roster");
    while shell.chrome_present.maintenance.locale_refresh.is_some() {
        shell.advance_chrome_maintenance_step();
    }
    assert_eq!(dock_tab_label(&shell, FRAMEWORK_SETTINGS_GENERAL_TAB_ID), "General");
    assert_eq!(dock_tab_label(&shell, FRAMEWORK_SETTINGS_THEME_TAB_ID), "Theme");
    assert_eq!(dock_tab_label(&shell, FRAMEWORK_SETTINGS_KEYBINDINGS_TAB_ID), "Hotkeys");
    assert!(retained_has_record_label(&shell, FRAMEWORK_SETTINGS_GENERAL_TAB_ID, "General"));
    assert!(retained_has_record_label(&shell, FRAMEWORK_SETTINGS_THEME_TAB_ID, "Theme"));
    assert!(retained_has_record_label(&shell, FRAMEWORK_SETTINGS_KEYBINDINGS_TAB_ID, "Hotkeys"));
    assert!(shell.owed_refresh_scope.asks_for_nothing());
}
