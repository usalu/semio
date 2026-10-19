/// 🏷️ Scene labels, dock tabs and select wheel: locale chrome labels, one tab stop per dock stack and the immediate select owning the wheel.

#[test]
fn icon_render_scene_chrome_uses_the_selected_locale() {
    let _camera_owner = semio_framework_os_renderer_wgpu_scene_state::camera_fixture_lease(semio_framework_os_renderer_wgpu_scene_state::camera_fixture_grant());
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../../../🖼️IconRenderHost/🧪️tests/🧫️fixtures/🏷️status/🔣️.json")).unwrap();
    for pack in fixture["locales"].as_array().unwrap() {
        let labels = scene_chrome_labels(pack["locale"] == "de").icon_render;
        assert_eq!(labels.empty_scene, pack["labels"]["empty"]);
        assert_eq!(labels.rendering, pack["labels"]["rendering"]);
        assert_eq!(labels.failed, pack["labels"]["failed"]);
    }
}

#[test]
fn virtual_file_system_scene_chrome_uses_the_shared_english_and_german_labels() {
    let _camera_owner = semio_framework_os_renderer_wgpu_scene_state::camera_fixture_lease(semio_framework_os_renderer_wgpu_scene_state::camera_fixture_grant());
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../../../../../../../../../🔨️modules/🖱️ui/🧪️tests/🧫️fixtures/📁️virtual-file-system-interaction/🔣️.json")).expect("shared VFS interaction fixture");
    for pack in fixture["chrome"].as_array().expect("chrome packs") {
        let labels = scene_chrome_labels(pack["locale"] == "de").virtual_file_system;
        assert_eq!(labels.name, pack["name"]);
        assert_eq!(labels.no_file_system_nodes, pack["empty"]);
        assert_eq!(labels.expand, pack["expand"]);
        assert_eq!(labels.collapse, pack["collapse"]);
    }
}
use semio_framework_os_renderer_wgpu_dock::{DockControlName, DockNode, DockStackTab};

#[test]
fn accepted_dock_tabs_publish_one_tab_stop_per_stack_and_activate_only_an_exact_address() {
    let _camera_owner = semio_framework_os_renderer_wgpu_scene_state::camera_fixture_lease(semio_framework_os_renderer_wgpu_scene_state::camera_fixture_grant());
    let fixture: Value = serde_json::from_str(include_str!("../../../../../🧪️tests/🧫️fixtures/⌨️dock-tab-keyboard/🔣️.json")).expect("Dock tab keyboard fixture");
    let stacks = fixture["stacks"].as_array().expect("Dock stacks");
    let mut shell = host_test_shell();
    shell.dock.dock.root = DockNode::Row(
        stacks
            .iter()
            .map(|stack| {
                let active = stack["active"].as_str().expect("active Dock tab").to_string();
                let windows = stack["tabs"].as_array().expect("Dock tabs").iter().map(|id| DockStackTab::new(id.as_str().expect("Dock tab id"))).collect();
                (DockNode::Stack { windows, active }, 1.0)
            })
            .collect(),
    );
    let initial = stacks[0]["active"].as_str().expect("initial Dock tab");
    shell.dock.dock.sync_active_window(initial);
    shell.windows.active_window_id = Some(initial.to_string());
    let mut input = InputState::default();
    with_chrome_control_names(|names| names.clear());
    let mut x = 0.0;
    for (stack_index, stack) in stacks.iter().enumerate() {
        let panel = stack["panel"].as_str().expect("Dock panel");
        note_dock_control_name(panel, DockControlName::Body(panel), false);
        input.register_hit(HitTarget { rect: Rect::new(x, 24.0, 120.0, 80.0), event: None, control_id: Some(panel.to_string()), kind: HitKind::ScrollRegion, drag_axis: None, drag_data: None });
        for tab in stack["tabs"].as_array().expect("Dock tabs") {
            let id = tab.as_str().expect("Dock tab id");
            let key = format!("dock.tab.{stack_index}.{id}");
            note_dock_control_name(&key, DockControlName::Tab { title: id, selected: Some(id) == stack["active"].as_str(), panel }, false);
            input.register_hit(HitTarget { rect: Rect::new(x, 0.0, 24.0, 24.0), event: None, control_id: Some(key), kind: HitKind::Window, drag_axis: None, drag_data: None });
            x += 24.0;
        }
    }
    PanelsViewCtx::of(&mut shell).publish_retained_hit_registry(&mut input);
    let expected = fixture["projection"]["nodes"].as_array().expect("projection nodes");
    for row in expected {
        let key = row["key"].as_str().expect("projection key");
        let node = shell.chrome.presented_chrome_accessibility.iter().find(|node| node.key == key).unwrap_or_else(|| panic!("missing accepted Dock node {key}"));
        assert_eq!(node.role.as_str(), row["role"].as_str().expect("projection role"));
        assert_eq!(node.focusable, row["focusable"].as_bool().expect("projection focusable"));
        assert_eq!(node.tabbable, row["tabbable"].as_bool().expect("projection tabbable"));
        assert_eq!(node.actionable, row["actionable"].as_bool().expect("projection actionable"));
        assert_eq!(node.selected, row.get("selected").and_then(Value::as_bool));
        assert_eq!(node.controls.as_deref(), row.get("controls").and_then(Value::as_str));
    }
    for stack in stacks {
        let panel = stack["panel"].as_str().unwrap();
        assert_eq!(shell.chrome.presented_chrome_accessibility.iter().filter(|node| node.role == "tab" && node.controls.as_deref() == Some(panel) && node.tabbable).count(), 1);
    }

    let node = shell.chrome.presented_chrome_accessibility.iter().find(|node| node.key == "dock.tab.0.b").expect("second left Dock tab");
    let target =
        ui_render::AccessibilityTarget { window_id: semio_framework_os_renderer_wgpu_ui_engine_dump::chrome_ledger::SHELL_CHROME_ACCESSIBILITY_WINDOW_ID.to_string(), window_generation: shell.chrome.presented_chrome_accessibility_generation, node_id: node.node_id, node_key: node.key.clone() };
    assert!(semio_framework_async::block_on(PointerCtx::of(&mut shell).handle_accessibility_event(&target, &ui_render::AccessibilityEvent::Focus, &mut input)).expect("Dock focus"));
    assert_eq!(shell.windows.active_window_id.as_deref(), Some(initial), "roving focus does not select");
    let stale = ui_render::AccessibilityTarget { window_generation: target.window_generation.saturating_sub(1), ..target.clone() };
    assert!(!semio_framework_async::block_on(PointerCtx::of(&mut shell).handle_accessibility_event(&stale, &ui_render::AccessibilityEvent::Activate, &mut input)).expect("stale Dock activation"));
    assert_eq!(shell.windows.active_window_id.as_deref(), Some(initial), "a stale tab address cannot select");
    assert!(semio_framework_async::block_on(PointerCtx::of(&mut shell).handle_accessibility_event(&target, &ui_render::AccessibilityEvent::Activate, &mut input)).expect("current Dock activation"));
    assert_eq!(shell.windows.active_window_id.as_deref(), Some("b"));
    assert_eq!(shell.dock.dock.active_window_id.as_deref(), Some("b"));
}

#[test]
fn a_published_immediate_select_owns_wheel_without_parsing_its_option_id() {
    let _camera_owner = semio_framework_os_renderer_wgpu_scene_state::camera_fixture_lease(semio_framework_os_renderer_wgpu_scene_state::camera_fixture_grant());
    let fixture: Value = serde_json::from_str(include_str!("../../../../../../../../../../🔨️modules/🖱️ui/🧪️tests/🧫️fixtures/🔽️retained-select-overlay-raster/🔣️.json")).unwrap();
    let law = &fixture["select"]["wheel"];
    let id = "owner.item.embedded";
    let mut shell = host_test_shell();
    let mut input = InputState::default();
    let mut draw = semio_framework_ui_draw_contract::DrawList::default();
    let mut atlas = FontAtlas::builtin();
    let theme = Theme::default();
    shell.input.open_selects.insert(id.into(), true);
    input.register_hit(HitTarget { rect: Rect::new(0.0, 0.0, 160.0, 100.0), event: None, control_id: Some("outer".into()), kind: HitKind::ScrollRegion, drag_axis: None, drag_data: None });
    let widget =
        semio_framework_ui_wgpu_widget_kit::WidgetNode::Select { id: id.into(), value: "0".into(), items: (0..20).map(|index| semio_framework_ui_wgpu_widget_kit::SelectItem { value: format!("option.item.{index}"), label: index.to_string() }).collect(), placeholder: None, on_change: None };
    semio_framework_ui_wgpu_widgets::render_widget(
        &widget,
        Rect::new(8.0, 62.0, 120.0, 22.4),
        &mut semio_framework_ui_wgpu_widget_kit::WidgetContext {
            draw: &mut draw,
            overlay: None,
            atlas: &mut atlas,
            icons: None,
            input: &mut input,
            theme: &theme,
            scroll_offsets: &mut shell.input.scroll_offsets,
            collapsed_sections: &mut shell.input.collapsed_sections,
            open_selects: &mut shell.input.open_selects,
            interaction_maps: Some(&mut shell.input.widget_maps),
            pick_clip: None,
            viewport_height: 100.0,
        },
    );
    input.publish_hits();
    let x = law["point"]["x"].as_f64().unwrap() as f32;
    let y = law["point"]["y"].as_f64().unwrap() as f32;
    assert!(shell.input.handle_pointer_wheel(x, y, 0.0, law["delta"].as_f64().unwrap() as f32, &mut input));
    assert_eq!(shell.input.scroll_offsets.get(&format!("select.{id}.scroll")).copied(), Some(law["expectedScroll"].as_f64().unwrap() as f32));
    assert!(!shell.input.scroll_offsets.contains_key("outer"));
    shell.input.open_selects.clear();
    assert!(shell.input.handle_pointer_wheel(x, y, 0.0, 1.0, &mut input), "accepted popup remains authoritative until its replacement is presented");
    assert_eq!(shell.input.scroll_offsets.get(&format!("select.{id}.scroll")).copied(), Some(35.0));
}
