use super::*;

#[test]
fn virtual_file_system_scene_chrome_uses_the_shared_english_and_german_labels() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/../../../../../../../../../🔨️modules/🖱️ui/🧫️fixtures/📁️virtual-file-system-interaction/🔣️.json"))).expect("shared VFS interaction fixture");
    for pack in fixture["chrome"].as_array().expect("chrome packs") {
        let labels = scene_chrome_labels(pack["locale"] == "de").virtual_file_system;
        assert_eq!(labels.name, pack["name"]);
        assert_eq!(labels.no_file_system_nodes, pack["empty"]);
        assert_eq!(labels.expand, pack["expand"]);
        assert_eq!(labels.collapse, pack["collapse"]);
    }
}
use crate::dock::DockNode;

#[test]
fn a_published_immediate_select_owns_wheel_without_parsing_its_option_id() {
    let fixture: Value = serde_json::from_str(include_str!("../../../../../../../../../🔨️modules/🖱️ui/🧫️fixtures/🔽️retained-select-overlay-raster/🔣️.json")).unwrap();
    let law = &fixture["select"]["wheel"];
    let id = "owner.item.embedded";
    let mut shell = super::panel_anchor_model_tests::host_test_shell();
    let mut input = InputState::default();
    let mut draw = ui_wgpu::wgpu::DrawList::default();
    let mut atlas = FontAtlas::builtin();
    let theme = Theme::default();
    shell.open_selects.insert(id.into(), true);
    input.register_hit(HitTarget { rect: Rect::new(0.0, 0.0, 160.0, 100.0), event: None, control_id: Some("outer".into()), kind: HitKind::ScrollRegion, drag_axis: None, drag_data: None });
    let widget = ui_wgpu::wgpu::WidgetNode::Select {
        id: id.into(), value: "0".into(), items: (0..20).map(|index| ui_wgpu::wgpu::SelectItem { value: format!("option.item.{index}"), label: index.to_string() }).collect(), placeholder: None, on_change: None,
    };
    ui_wgpu::wgpu::render_widget(&widget, Rect::new(8.0, 62.0, 120.0, 22.4), &mut ui_wgpu::wgpu::WidgetContext {
        draw: &mut draw, overlay: None, atlas: &mut atlas, icons: None, input: &mut input, theme: &theme,
        scroll_offsets: &mut shell.scroll_offsets, collapsed_sections: &mut shell.collapsed_sections, open_selects: &mut shell.open_selects,
        interaction_maps: Some(&mut shell.widget_maps), pick_clip: None, viewport_height: 100.0,
    });
    input.publish_hits();
    let x = law["point"]["x"].as_f64().unwrap() as f32;
    let y = law["point"]["y"].as_f64().unwrap() as f32;
    assert!(shell.handle_pointer_wheel(x, y, 0.0, law["delta"].as_f64().unwrap() as f32, &mut input));
    assert_eq!(shell.scroll_offsets.get(&format!("select.{id}.scroll")).copied(), Some(law["expectedScroll"].as_f64().unwrap() as f32));
    assert!(!shell.scroll_offsets.contains_key("outer"));
    shell.open_selects.clear();
    assert!(shell.handle_pointer_wheel(x, y, 0.0, 1.0, &mut input), "accepted popup remains authoritative until its replacement is presented");
    assert_eq!(shell.scroll_offsets.get(&format!("select.{id}.scroll")).copied(), Some(35.0));
    println!("[DEBUG] immediate Select exact owner={id} accepted wheel=35");
}

#[test]
fn the_topmost_retained_world_owns_the_press_and_release_outside_its_bounds() {
    retained_world_sequence_probe("captured");
}

#[test]
fn captured_world_movement_and_wheel_never_fan_out_to_an_overlapping_peer() {
    retained_world_sequence_probe("motion-wheel");
}

#[test]
fn cancelled_world_capture_cannot_manufacture_a_successful_release() {
    retained_world_sequence_probe("cancel");
}

#[test]
fn a_modal_layer_blocks_actual_world_pointer_ingress() {
    retained_world_sequence_probe("modal");
}

#[test]
fn a_scene_refresh_preserves_the_exact_captured_owner_until_outside_release() {
    retained_world_sequence_probe("refresh");
}

#[test]
fn an_accepted_sibling_insertion_rebases_the_exact_renderer_scene_capture() {
    let fixture: Value = serde_json::from_str(include_str!("../../../../🧫️fixtures/🪪️scene-pointer-owner/🔣️.json")).unwrap();
    let law = &fixture["siblingInsertion"];
    let window = "renderer-capture-arena-rebase";
    let body = Rect::new(0.0, 0.0, 600.0, 300.0);
    let pointer = ui_render::PointerId(77);
    let mut shell = super::panel_anchor_model_tests::host_test_shell();
    let mut documents = Vec::new();
    let mut original = None;
    let mut current = None;
    for (index, ids) in law["records"].as_array().unwrap().iter().enumerate() {
        let ids: Vec<_> = ids.as_array().unwrap().iter().map(|id| id.as_u64().unwrap()).collect();
        let mut root =
            tree_pointer_record(1, "root", ui_contract::Component::Container(ui_contract::ContainerProps { role: Default::default(), label: None, description: None, required: None, error: None, default_open: None, drop_overlay: None }), &ids, None);
        root.layout = ui_contract::LayoutSpec::Stack(ui_contract::StackLayout { axis: ui_contract::Axis::Horizontal, grow: true, ..Default::default() });
        let mut records = vec![root];
        for id in ids {
            let mut record = canvas_pointer_record(id, &format!("scene-{id}"), &ui_wgpu::wgpu::Canvas2dScene::base(0.0, 0.0, 1.0, "[]".into()));
            record.layout = ui_contract::LayoutSpec::Stack(ui_contract::StackLayout { grow: true, ..Default::default() });
            records.push(record);
        }
        let document = shell.publish_surface_records(window, records).unwrap();
        let input = paint_component_pointer_documents(&mut shell, &[(window, "capture-controller", &document, body)]);
        if index > 0 {
            let key = ui_wgpu::wgpu::NodeKey::Explicit(format!("scene-{}", law["receiver"].as_u64().unwrap()));
            let target = input
                .hits()
                .iter()
                .filter(|hit| hit.kind == HitKind::ComponentScene)
                .find_map(|hit| crate::interpreter::retained_scene_target_at(window, hit.rect.x + hit.rect.w * 0.5, hit.rect.y + hit.rect.h * 0.5).filter(|target| target.key == key))
                .expect("the same protocol record remains mounted");
            if index == 1 {
                assert!(crate::interpreter::claim_scene_pointer_owner(target.clone(), pointer));
                original = Some(target);
            } else {
                current = Some(target);
            }
        }
        documents.push(document);
    }
    let original = original.unwrap();
    let current = current.unwrap();
    let captured = crate::interpreter::captured_scene_pointer(pointer);
    let released = crate::interpreter::release_scene_pointer(pointer);
    let duplicate = crate::interpreter::release_scene_pointer(pointer);
    assert!(crate::interpreter::request_ui_document_close(window));
    for _ in 0..262_144 {
        if !crate::interpreter::ui_document_close_pending() {
            break;
        }
        crate::interpreter::close_ui_document_one();
    }
    for mut document in documents {
        while !document.close_step() {}
    }
    assert!(!crate::interpreter::ui_document_close_pending());
    println!("[DEBUG] retained sibling host={} presented={:?} accepted={:?} captured={:?}", current.host_id, original.node, current.node, captured.as_ref().map(|owner| owner.node));
    assert!(original.same_component_host(&current), "a sibling insertion preserves the mounted receiver");
    assert_ne!(original.node, current.node, "the fixture must exercise different arena histories");
    assert_eq!(captured, Some(current.clone()));
    assert_eq!(usize::from(released == Some(current)), law["terminalReceivers"].as_u64().unwrap() as usize);
    assert_eq!(duplicate, None);
}

/// 🕒️ Accepted removal invalidates every old camera before the one-slot retirement lane drains.
#[test]
fn renderer_canvas_multiple_accepted_removals_invalidate_every_checked_out_camera() {
    let window = "renderer-camera-multiple-retirement";
    let controller = "renderer-camera-multiple-retirement-controller";
    let body = Rect::new(0.0, 0.0, 600.0, 300.0);
    let scene = ui_wgpu::wgpu::Canvas2dScene::base(0.0, 0.0, 1.0, "[]".into());
    let root = |children: &[u64]| {
        let mut record = tree_pointer_record(
            1,
            "root",
            ui_contract::Component::Container(ui_contract::ContainerProps { role: Default::default(), label: None, description: None, required: None, error: None, default_open: None, drop_overlay: None }),
            children,
            None,
        );
        record.layout = ui_contract::LayoutSpec::Stack(ui_contract::StackLayout { axis: ui_contract::Axis::Horizontal, grow: true, ..Default::default() });
        record
    };
    let child = |id, key: &str| {
        let mut record = canvas_pointer_record(id, key, &scene);
        record.layout = ui_contract::LayoutSpec::Stack(ui_contract::StackLayout { grow: true, ..Default::default() });
        record
    };
    let mut shell = super::panel_anchor_model_tests::host_test_shell();
    let mut presented = shell.publish_surface_records(window, vec![root(&[2, 3]), child(2, "left-camera"), child(3, "right-camera")]).unwrap();
    let input = paint_component_pointer_documents(&mut shell, &[(window, controller, &presented, body)]);
    let mut rects: Vec<_> = input.hits().iter().filter(|hit| hit.kind == HitKind::ComponentScene).map(|hit| hit.rect).collect();
    rects.sort_by(|left, right| left.x.total_cmp(&right.x));
    assert_eq!(rects.len(), 2);
    let mut interaction = pointer_interaction(shell, input);
    for rect in &rects {
        semio_framework_async::block_on(crate::winit_app::dispatch_normalized_event(
            &mut interaction,
            ui_render::DispatchEvent::Scroll { x: rect.x + rect.w * 0.5, y: rect.y + rect.h * 0.5, delta_x: 0.0, delta_y: -120.0, modifiers: Default::default() },
        ));
    }
    let mut deadlines = crate::scenes::SceneCameraDispatchCursor::begin(crate::app_now_ms() + 400.0);
    let mut removed = interaction.shell.publish_surface_records(window, vec![root(&[])]).unwrap();
    interaction.input = paint_component_pointer_documents(&mut interaction.shell, &[(window, controller, &removed, body)]);
    let mut actions = Vec::new();
    loop {
        match deadlines.step() {
            crate::scenes::SceneCameraDispatchStep::Action(action) => actions.push(action),
            crate::scenes::SceneCameraDispatchStep::Pending => {}
            crate::scenes::SceneCameraDispatchStep::Complete => break,
            crate::scenes::SceneCameraDispatchStep::Fault(fault) => panic!("{fault}"),
        }
    }
    assert!(deadlines.terminal_is_empty());
    assert!(actions.is_empty(), "every camera removed by the accepted pixels becomes stale atomically");
    while !presented.close_step() {}
    while !removed.close_step() {}
}

/// 🕒️ A same-host sibling insertion preserves a checked-out camera across its arena rebase.
#[test]
fn renderer_canvas_same_host_sibling_rebase_preserves_checked_out_camera() {
    let fixture: Value = serde_json::from_str(include_str!("../../../../🧫️fixtures/🪪️scene-pointer-owner/🔣️.json")).unwrap();
    let law = &fixture["siblingInsertion"];
    let receiver = law["receiver"].as_u64().unwrap();
    let window = "renderer-camera-sibling-rebase";
    let controller = "renderer-camera-sibling-rebase-controller";
    let body = Rect::new(0.0, 0.0, 600.0, 300.0);
    let mut shell = super::panel_anchor_model_tests::host_test_shell();
    let mut documents = Vec::new();
    let mut deadline = None;
    let mut presented = None;
    let mut accepted = None;
    for (index, ids) in law["records"].as_array().unwrap().iter().enumerate() {
        let ids: Vec<_> = ids.as_array().unwrap().iter().map(|id| id.as_u64().unwrap()).collect();
        let mut root =
            tree_pointer_record(1, "root", ui_contract::Component::Container(ui_contract::ContainerProps { role: Default::default(), label: None, description: None, required: None, error: None, default_open: None, drop_overlay: None }), &ids, None);
        root.layout = ui_contract::LayoutSpec::Stack(ui_contract::StackLayout { axis: ui_contract::Axis::Horizontal, grow: true, ..Default::default() });
        let mut records = vec![root];
        for id in ids {
            let mut record = canvas_pointer_record(id, &format!("scene-{id}"), &ui_wgpu::wgpu::Canvas2dScene::base(0.0, 0.0, 1.0, "[]".into()));
            record.layout = ui_contract::LayoutSpec::Stack(ui_contract::StackLayout { grow: true, ..Default::default() });
            records.push(record);
        }
        let document = shell.publish_surface_records(window, records).unwrap();
        let input = paint_component_pointer_documents(&mut shell, &[(window, controller, &document, body)]);
        let key = ui_wgpu::wgpu::NodeKey::Explicit(format!("scene-{receiver}"));
        let target_and_rect = input
            .hits()
            .iter()
            .filter(|hit| hit.kind == HitKind::ComponentScene)
            .find_map(|hit| crate::interpreter::retained_scene_target_at(window, hit.rect.x + hit.rect.w * 0.5, hit.rect.y + hit.rect.h * 0.5).filter(|target| target.key == key).map(|target| (target, hit.rect)));
        if index == 1 {
            let (target, rect) = target_and_rect.expect("the receiver is presented before its sibling insertion");
            presented = Some(target);
            let mut mounted = pointer_interaction(shell, input);
            semio_framework_async::block_on(crate::winit_app::dispatch_normalized_event(
                &mut mounted,
                ui_render::DispatchEvent::Scroll { x: rect.x + rect.w * 0.5, y: rect.y + rect.h * 0.5, delta_x: 0.0, delta_y: -120.0, modifiers: Default::default() },
            ));
            deadline = Some(crate::scenes::SceneCameraDispatchCursor::begin(crate::app_now_ms() + 400.0));
            shell = mounted.shell;
        } else {
            if index == 2 {
                accepted = target_and_rect.map(|(target, _)| target);
            }
        }
        documents.push(document);
    }
    let presented = presented.expect("the receiver owns the checked-out camera");
    let accepted = accepted.expect("the accepted sibling candidate retains the receiver");
    assert!(presented.same_component_host(&accepted));
    assert_ne!(presented.node, accepted.node, "the fixture exercises a real arena-node rebase");
    let mut deadline = deadline.expect("the receiver camera is checked out before acknowledgement");
    let mut actions = Vec::new();
    loop {
        match deadline.step() {
            crate::scenes::SceneCameraDispatchStep::Action(action) => actions.push(action),
            crate::scenes::SceneCameraDispatchStep::Pending => {}
            crate::scenes::SceneCameraDispatchStep::Complete => break,
            crate::scenes::SceneCameraDispatchStep::Fault(fault) => panic!("{fault}"),
        }
    }
    assert!(deadline.terminal_is_empty());
    assert_eq!(actions.len(), 1, "the accepted same-host receiver keeps its checked-out camera");
    for mut document in documents {
        while !document.close_step() {}
    }
}

#[test]
fn a_replacement_scene_cannot_inherit_the_previous_keys_capture() {
    retained_world_sequence_probe("replacement");
}

#[test]
fn another_pointers_chrome_release_preserves_the_captured_scene_drag() {
    retained_world_sequence_probe("foreign-release");
}

#[test]
fn another_pointers_cancellation_preserves_the_captured_scene_drag() {
    retained_world_sequence_probe("foreign-cancel");
}

#[test]
fn replaced_scene_pointer_slots_release_the_fixed_capture_grant() {
    let fixture: Value = serde_json::from_str(include_str!("../../../../🧫️fixtures/🪪️scene-pointer-owner/🔣️.json")).unwrap();
    let capacity = fixture["captureCapacity"].as_u64().unwrap();
    let mut shell = ShellState::new(Vec::new(), String::new());
    shell.panel_anchors = std::array::from_fn(|_| PanelAnchorState::default());
    shell.dock_tabs = ShellDock::default();
    let scene = ui_wgpu::wgpu::World3dScene::base(serde_json::json!({ "position": [4.0, 4.0, 4.0], "target": [0.0, 0.0, 0.0], "projection": "perspective" }).to_string(), "[]".into(), "[]".into(), "{}".into());
    let window = "scene-capture-capacity";
    let bounds = Rect::new(0.0, 0.0, 300.0, 200.0);
    let mut first = shell.publish_surface_records(window, vec![world3d_pointer_record(1, "original", &scene)]).unwrap();
    let input = paint_tree_pointer_document(&mut shell, window, &first, bounds);
    let target = shell.scene_pointer_target_at(100.0, 100.0, &input, &Theme::default()).unwrap();
    let admitted: Vec<_> = (0..capacity).map(|pointer| crate::interpreter::claim_scene_pointer_owner(target.clone(), ui_render::PointerId(pointer + 100))).collect();
    let overflow = crate::interpreter::claim_scene_pointer_owner(target.clone(), ui_render::PointerId(999));
    let mut successor = shell.publish_surface_records(window, vec![world3d_pointer_record(1, "successor", &scene)]).unwrap();
    let input = paint_tree_pointer_document(&mut shell, window, &successor, bounds);
    let target = shell.scene_pointer_target_at(100.0, 100.0, &input, &Theme::default()).unwrap();
    let old_live: Vec<_> = (0..capacity).map(|pointer| crate::interpreter::captured_scene_pointer(ui_render::PointerId(pointer + 100)).is_some()).collect();
    let fresh = crate::interpreter::claim_scene_pointer_owner(target, ui_render::PointerId(999));
    for pointer in 0..capacity {
        crate::interpreter::release_scene_pointer(ui_render::PointerId(pointer + 100));
    }
    crate::interpreter::release_scene_pointer(ui_render::PointerId(999));
    shell.sync_engine_surface_states();
    for _ in 0..SHELL_WINDOW_PAINT_OPPORTUNITIES.min(1 << 20) {
        if shell.retired_world3d_states.is_empty() {
            break;
        }
        shell.advance_world3d_retirement_step();
    }
    while !first.close_step() {}
    while !successor.close_step() {}
    assert!(admitted.iter().all(|value| *value), "every funded pointer owns a slot");
    assert!(!overflow, "live captures cannot exceed the fixed grant");
    assert!(old_live.iter().all(|value| !value), "a replaced key retires every old receiver");
    assert!(fresh, "stale targets must release their fixed grant before a successor claims it");
    println!("[DEBUG] exact retained replacement reclaimed its fixed pointer capture grant");
}

fn pointer_interaction(shell: ShellState, input: InputState<ActionDescriptor>) -> crate::AppInteractionState {
    crate::AppInteractionState {
        shell,
        input,
        theme: Theme::default(),
        theme_dark: false,
        last_pointer_x: 0.0,
        last_pointer_y: 0.0,
        pointer_down: false,
        pointer_button: 0,
        pointer_capture: PointerCapture::default(),
        modifiers: PointerModifiers::default(),
        space_pressed: false,
        wheel_zoom_deadline_ms: 0.0,
        caret_blink_at_ms: 0.0,
        caret_blink_visible: true,
        text_streams: std::array::from_fn(|_| None),
        text_fault: None,
        frame_fault: None,
        text_cancel_pending: false,
        #[cfg(not(target_arch = "wasm32"))]
        last_sync_pump_ms: 0.0,
    }
}

pub(super) fn retained_world_sequence_probe(scenario: &str) {
    let fixture: Value = serde_json::from_str(include_str!("../../../../🧫️fixtures/🪪️scene-pointer-owner/🔣️.json")).unwrap();
    let mut shell = ShellState::new(Vec::new(), String::new());
    shell.panel_anchors = std::array::from_fn(|_| PanelAnchorState::default());
    shell.dock_tabs = ShellDock::default();
    let mut active_documents = Vec::new();
    let mut retired_documents = Vec::new();
    let mut row_sources = Vec::new();
    let scene = ui_wgpu::wgpu::World3dScene::base(serde_json::json!({ "position": [4.0, 4.0, 4.0], "target": [0.0, 0.0, 0.0], "projection": "perspective" }).to_string(), "[]".into(), "[]".into(), "{}".into());
    for row in fixture["surfaces"].as_array().unwrap() {
        let window = row["windowId"].as_str().unwrap();
        let surface = row["surfaceId"].as_str().unwrap();
        let bounds = row["bounds"].as_array().unwrap();
        let bounds = Rect::new(bounds[0].as_f64().unwrap() as f32, bounds[1].as_f64().unwrap() as f32, bounds[2].as_f64().unwrap() as f32, bounds[3].as_f64().unwrap() as f32);
        let document = shell.publish_surface_records(window, vec![world3d_pointer_record(1, "world", &scene)]).unwrap();
        active_documents.push(document);
        row_sources.push((window.to_string(), surface.to_string(), bounds));
    }
    shell.dock_window_plan = row_sources.iter().map(|(window, _, bounds)| (window.clone(), *bounds)).collect();
    let paint_rows: Vec<_> = row_sources.iter().zip(active_documents.iter()).map(|((window, _, bounds), document)| (window.as_str(), document, *bounds)).collect();
    let input = paint_tree_pointer_documents(&mut shell, &paint_rows);
    let mut rows = Vec::new();
    for (window, surface, bounds) in row_sources {
        let host_id = shell.world3d_states.iter().find_map(|(host_id, state)| (state.surface_id == surface).then(|| host_id.clone())).expect("painted world admits one exact runtime host");
        let state = shell.world3d_states.get_mut(&host_id).expect("painted world retains its exact runtime interaction owner");
        assert_eq!(state.surface_id, surface, "runtime host preserves the document wire surface");
        let before = infinite_world::world::enqueue_world3d_event(state, WorldInteractionIntent::pointer_leave(-1.0, -1.0)).unwrap();
        rows.push((window, surface, host_id, bounds, before));
    }
    let mut interaction = pointer_interaction(shell, input);
    let case = fixture["cases"].as_array().unwrap().iter().find(|case| case["name"] == scenario).unwrap();
    let point = |name: &str| (fixture[name][0].as_f64().unwrap() as f32, fixture[name][1].as_f64().unwrap() as f32);
    for step in case["steps"].as_array().unwrap() {
        match step.as_str().unwrap() {
            "press" | "release" | "secondaryPress" | "secondaryRelease" => {
                let down = matches!(step.as_str().unwrap(), "press" | "secondaryPress");
                let (x, y) = point(if down { "press" } else { "release" });
                let button = if step.as_str().unwrap().starts_with("secondary") { 2 } else { 0 };
                semio_framework_async::block_on(interaction.handle_pointer_button(ui_render::PointerId(77), x, y, down, button, PointerModifiers::default()));
            }
            "move" => {
                let (x, y) = point("release");
                semio_framework_async::block_on(interaction.handle_pointer_move(ui_render::PointerId(77), x, y, true, 0, PointerModifiers::default()));
            }
            "wheel" => {
                let (x, y) = point("press");
                let target = interaction.shell.scene_pointer_target_at(x, y, &interaction.input, &interaction.theme).unwrap();
                interaction.apply_scene_wheel(&target, x, y, 1.0, &PointerModifiers::default()).unwrap();
            }
            "chrome" => {
                let hits = interaction.input.hits().to_vec();
                for hit in hits {
                    interaction.input.register_hit(hit);
                }
                interaction.input.register_hit(HitTarget { rect: rows.last().unwrap().3, event: None, control_id: Some("shell.fixture.chrome".into()), kind: HitKind::Button, drag_axis: None, drag_data: None });
                interaction.input.publish_hits();
            }
            "modal" => {
                interaction.shell.context_menu = Some(ContextMenuState::default());
                let paint_rows: Vec<_> = rows.iter().zip(active_documents.iter()).map(|((window, _, _, bounds, _), document)| (window.as_str(), document, *bounds)).collect();
                interaction.input = paint_tree_pointer_documents(&mut interaction.shell, &paint_rows);
            }
            "cancel" => interaction.handle_pointer_cancel(ui_render::PointerId(77)),
            "foreignPress" | "foreignRelease" => {
                let (x, y) = point("release");
                semio_framework_async::block_on(interaction.handle_pointer_button(ui_render::PointerId(78), x, y, step == "foreignPress", 0, PointerModifiers::default()));
            }
            "foreignCancel" => interaction.handle_pointer_cancel(ui_render::PointerId(78)),
            "refresh" | "replace" => {
                let window = rows.last().expect("the refreshed row remains mounted").0.clone();
                let key = if step == "replace" { "replacement-world" } else { "world" };
                let refreshed = ui_wgpu::wgpu::World3dScene::base(serde_json::json!({ "position": [5.0, 5.0, 5.0], "target": [0.0, 0.0, 0.0], "projection": "perspective" }).to_string(), "[]".into(), "[]".into(), "{}".into());
                let document = interaction.shell.publish_surface_records(&window, vec![world3d_pointer_record(1, key, &refreshed)]).unwrap();
                retired_documents.push(std::mem::replace(active_documents.last_mut().expect("the refreshed row owns a document"), document));
                let paint_rows: Vec<_> = rows.iter().zip(active_documents.iter()).map(|((window, _, _, bounds, _), document)| (window.as_str(), document, *bounds)).collect();
                interaction.input = paint_tree_pointer_documents(&mut interaction.shell, &paint_rows);
            }
            _ => unreachable!(),
        }
    }
    let menu_open = interaction.shell.context_menu.is_some();
    let active_window = interaction.shell.active_window_id.clone();
    let mut observed = Vec::new();
    for (_, surface, host_id, _, before) in &rows {
        let state = interaction.shell.world3d_states.get_mut(host_id).expect("the exact pre-replacement runtime host remains available through captured release");
        let after = infinite_world::world::enqueue_world3d_event(state, WorldInteractionIntent::pointer_leave(-1.0, -1.0)).unwrap();
        observed.push((surface.clone(), after - before - 1));
    }
    let fault = interaction.input.take_action_step().err();
    interaction.shell.dock_window_plan.clear();
    interaction.shell.sync_engine_surface_states();
    for _ in 0..SHELL_WINDOW_PAINT_OPPORTUNITIES.min(1 << 20) {
        if interaction.shell.retired_world3d_states.is_empty() {
            break;
        }
        interaction.shell.advance_world3d_retirement_step();
    }
    for mut document in retired_documents.into_iter().chain(active_documents) {
        while !document.close_step() {}
    }
    assert!(fault.is_none(), "pointer admission fault: {fault:?}");
    for (surface, count) in observed {
        let expected = if Some(surface.as_str()) == fixture["expectedOwner"].as_str() { case["events"].as_array().unwrap().len() as u64 } else { 0 };
        assert_eq!(count, expected, "{surface}: exact retained owner receives the actual press and captured release");
    }
    assert_eq!(menu_open, case["menuOpen"].as_bool().unwrap(), "{scenario} context menu state");
    let expected_window = if case["steps"].as_array().unwrap().iter().any(|step| step == "modal") { None } else { fixture["expectedOwner"].as_str() };
    assert_eq!(active_window.as_deref(), expected_window, "{scenario} activates the exact receiving window");
    assert!(crate::interpreter::captured_scene_pointer(ui_render::PointerId(77)).is_none(), "{scenario} released its captured owner");
    println!("[DEBUG] real retained World sequence {scenario} preserved exact owner, edge count, capture close, and menu state");
}

#[test]
fn retained_pane_actions_address_the_concrete_window_before_guest_admission() {
    let fixture = pane_owner_fixture();
    for case in fixture["cases"].as_array().unwrap() {
        let owner = case["window"].as_str().unwrap();
        let surface = case["surface"].as_str().unwrap();
        let mut shell = super::panel_anchor_model_tests::host_test_shell();
        let controller_id = shell.session.as_ref().unwrap().app.controller_id.clone();
        shell.plugins.clear();
        let result =
            semio_framework_async::block_on(shell.dispatch_action(ActionDescriptor { controller_id, action: semio_framework::SET_ACTIVE_UTILITY_ACTION_ID.into(), args: crate::action_args_json!({ "windowId": surface, "utilityId": "owner-brush" }) }));
        assert_eq!(result.unwrap_err(), "action program missing", "fixture stops at guest admission");
        assert_eq!(shell.active_utility_for_window(owner), Some("owner-brush"), "{} canonical host utility scope", case["kind"]);
        assert_eq!(shell.active_utility_by_window.len(), 1, "{} one concrete owner", case["kind"]);
        if surface != owner {
            assert!(!shell.active_utility_by_window.contains_key(surface), "{} pane identity never becomes an application window", case["kind"]);
        }
    }
    println!("[DEBUG] pane actions reached the concrete host window before guest admission");
}

fn pane_owner_select_records() -> Vec<ui_contract::UiNodeRecord> {
    pane_owner_select_records_with("pane-owner-select", "system")
}

fn pane_owner_select_records_with(key: &str, value: &str) -> Vec<ui_contract::UiNodeRecord> {
    serde_json::from_value(serde_json::json!([{
        "id": 1, "key": key,
        "component": { "type": "select", "value": value, "items": [{ "value": "system", "label": "System" }, { "value": "dark", "label": "Dark" }] },
        "layout": { "kind": "leaf", "width": "fill", "height": "hug" },
        "style": {}, "activity": "idle", "accessibility": { "label": "Pane selection" },
        "bindings": [{ "trigger": "change", "action": { "scope": "framework", "name": "setAppearance", "version": 1 } }]
    }]))
    .unwrap()
}

fn pane_owner_input_records_with(key: &str) -> Vec<ui_contract::UiNodeRecord> {
    serde_json::from_value(serde_json::json!([{
        "id": 1, "key": key,
        "component": { "type": "input", "kind": "text", "value": "", "commit": "enter" },
        "layout": { "kind": "leaf", "width": "fill", "height": "hug" },
        "style": {}, "activity": "idle", "accessibility": { "label": "Replacement input" },
        "bindings": [{ "trigger": "commit", "action": { "scope": "framework", "name": "setReplacementInput", "version": 1 } }]
    }]))
    .unwrap()
}

fn retained_focus_panel_shell(surface: &str) -> ShellState {
    let mut shell = super::panel_anchor_model_tests::host_test_shell();
    shell.dock_tabs = ShellDock::default();
    shell.panel_anchors = std::array::from_fn(|_| PanelAnchorState::default());
    shell.dock_tabs.tabs_mut(PanelAnchor::BottomRight).push(DockTabNode::leaf(surface, "Preferences", "settings", 0));
    *shell.anchor_state_mut(PanelAnchor::BottomRight) = PanelAnchorState { visible: true, size: 300.0, path: vec![surface.into()] };
    shell
}

#[test]
fn retained_panel_focus_routes_keyboard_without_activating_an_application_window() {
    let fixture = pane_owner_fixture();
    let surface = fixture["panel"]["surface"].as_str().unwrap();
    let previous = fixture["previousWindow"].as_str().unwrap();
    let mut shell = retained_focus_panel_shell(surface);
    shell.active_window_id = Some(previous.into());
    let mut document = shell.publish_surface_records(surface, pane_owner_select_records()).unwrap();
    let mut input = paint_tree_pointer_document(&mut shell, surface, &document, Rect::new(0.0, 0.0, 320.0, 120.0));
    let target = crate::interpreter::published_accessibility_target_for_test(surface, "pane-owner-select").expect("the published pane exposes its exact accessible address");
    let focused = semio_framework_async::block_on(shell.handle_accessibility_event(&target, &ui_render::AccessibilityEvent::Focus, &mut input)).unwrap();
    let active = shell.active_window_id.clone();
    let open_focus = shell.retained_keyboard_focus().map(|(surface, _)| surface);
    shell.anchor_state_mut(PanelAnchor::BottomRight).visible = false;
    let hidden_focus = shell.retained_keyboard_focus();
    semio_framework_async::block_on(shell.handle_keyboard_async(ui_wgpu::wgpu::KeyAction::Space(true), &PointerModifiers::default(), &mut input)).unwrap();
    let hidden_actions = crate::collect_fixture_actions(&mut input);
    shell.anchor_state_mut(PanelAnchor::BottomRight).visible = true;
    assert!(shell.retained_keyboard_focus().is_some(), "the same visible retained owner recovers without a synthetic focus event");
    crate::interpreter::begin_accessibility_visible_documents();
    crate::interpreter::publish_accessibility_visible_documents();
    let closed_focus = shell.retained_keyboard_focus();
    semio_framework_async::block_on(shell.handle_keyboard_async(ui_wgpu::wgpu::KeyAction::Space(true), &PointerModifiers::default(), &mut input)).unwrap();
    let closed_actions = crate::collect_fixture_actions(&mut input);
    while !document.close_step() {}
    assert!(focused);
    assert_eq!(active.as_deref(), Some(previous));
    assert_eq!(open_focus.as_deref(), Some(surface));
    assert!(hidden_focus.is_none(), "a hidden panel cannot receive keyboard input");
    assert!(hidden_actions.is_empty(), "a hidden panel dispatches no retained keyboard action");
    assert!(closed_focus.is_none(), "a closed surface cannot receive keyboard input");
    assert!(closed_actions.is_empty(), "a closed surface dispatches no retained keyboard action");
    println!("[DEBUG] open panel owned keyboard without app activation and relinquished it when hidden or closed");
}

#[test]
fn same_surface_keyed_successor_retains_keyboard_focus_without_a_synthetic_focus_event() {
    let surface = "focus-lifecycle-keyed-successor";
    let mut shell = retained_focus_panel_shell(surface);
    let mut initial = shell.publish_surface_records(surface, pane_owner_select_records_with("stable-select", "system")).expect("initial keyed document publishes");
    let mut input = paint_tree_pointer_document(&mut shell, surface, &initial, Rect::new(0.0, 0.0, 320.0, 120.0));
    let target = crate::interpreter::published_accessibility_target_for_test(surface, "stable-select").expect("the published pane exposes its exact accessible address");
    assert!(semio_framework_async::block_on(shell.handle_accessibility_event(&target, &ui_render::AccessibilityEvent::Focus, &mut input)).unwrap());
    let before = shell.retained_keyboard_focus().expect("the initial keyed node owns keyboard focus");

    let mut successor = shell.publish_surface_records(surface, pane_owner_select_records_with("stable-select", "dark")).expect("same-surface successor publishes");
    input = paint_tree_pointer_document(&mut shell, surface, &successor, Rect::new(0.0, 0.0, 320.0, 120.0));
    let after = shell.retained_keyboard_focus().expect("the keyed successor retains keyboard focus");
    assert_eq!(after, before, "key reconciliation preserves the exact focused node identity");
    semio_framework_async::block_on(shell.handle_keyboard_async(ui_wgpu::wgpu::KeyAction::Space(true), &PointerModifiers::default(), &mut input)).unwrap();
    assert!(crate::collect_fixture_actions(&mut input).is_empty(), "the retained Select opens before commit");
    semio_framework_async::block_on(shell.handle_keyboard_async(ui_wgpu::wgpu::KeyAction::Space(true), &PointerModifiers::default(), &mut input)).unwrap();
    assert_eq!(crate::collect_fixture_actions(&mut input).iter().map(|action| action.action.as_str()).collect::<Vec<_>>(), ["setAppearance"]);
    while !initial.close_step() {}
    while !successor.close_step() {}
}

#[test]
fn successor_that_removes_the_focused_key_cannot_receive_retained_keyboard_input() {
    let surface = "focus-lifecycle-removed-key";
    let mut shell = retained_focus_panel_shell(surface);
    let mut initial = shell.publish_surface_records(surface, pane_owner_select_records_with("retired-select", "system")).expect("initial removable document publishes");
    let mut input = paint_tree_pointer_document(&mut shell, surface, &initial, Rect::new(0.0, 0.0, 320.0, 120.0));
    let target = crate::interpreter::published_accessibility_target_for_test(surface, "retired-select").expect("the published pane exposes its exact accessible address");
    assert!(semio_framework_async::block_on(shell.handle_accessibility_event(&target, &ui_render::AccessibilityEvent::Focus, &mut input)).unwrap());
    assert!(shell.retained_keyboard_focus().is_some());

    let mut successor = shell.publish_surface_records(surface, pane_owner_select_records_with("replacement-select", "dark")).expect("replacement document publishes");
    input = paint_tree_pointer_document(&mut shell, surface, &successor, Rect::new(0.0, 0.0, 320.0, 120.0));
    assert!(shell.chrome_build.content_has_focus(surface), "the shell receives no invented blur command");
    assert!(shell.retained_keyboard_focus().is_none(), "the removed focused node generation cannot route into its replacement");
    semio_framework_async::block_on(shell.handle_keyboard_async(ui_wgpu::wgpu::KeyAction::Space(true), &PointerModifiers::default(), &mut input)).unwrap();
    assert!(crate::collect_fixture_actions(&mut input).is_empty(), "the replacement receives no keyboard action without its own focus event");
    while !initial.close_step() {}
    while !successor.close_step() {}
}

#[test]
fn same_key_changed_control_kind_cannot_inherit_retained_keyboard_focus() {
    let surface = "focus-lifecycle-changed-kind";
    let mut shell = retained_focus_panel_shell(surface);
    let mut initial = shell.publish_surface_records(surface, pane_owner_select_records_with("polymorphic-control", "system")).expect("initial Select document publishes");
    let mut input = paint_tree_pointer_document(&mut shell, surface, &initial, Rect::new(0.0, 0.0, 320.0, 120.0));
    let target = crate::interpreter::published_accessibility_target_for_test(surface, "polymorphic-control").expect("the published pane exposes its exact accessible address");
    assert!(semio_framework_async::block_on(shell.handle_accessibility_event(&target, &ui_render::AccessibilityEvent::Focus, &mut input)).unwrap());
    assert!(shell.retained_keyboard_focus().is_some());

    let mut successor = shell.publish_surface_records(surface, pane_owner_input_records_with("polymorphic-control")).expect("same-key Input successor publishes");
    input = paint_tree_pointer_document(&mut shell, surface, &successor, Rect::new(0.0, 0.0, 320.0, 120.0));
    assert!(shell.chrome_build.content_has_focus(surface), "document reconciliation emits no synthetic blur");
    assert!(shell.retained_keyboard_focus().is_none(), "a same-key replacement control kind cannot inherit its predecessor's focus");
    semio_framework_async::block_on(shell.handle_keyboard_async(ui_wgpu::wgpu::KeyAction::Enter, &PointerModifiers::default(), &mut input)).unwrap();
    assert!(crate::collect_fixture_actions(&mut input).is_empty(), "the replacement Input receives no Enter commit without its own focus event");
    while !initial.close_step() {}
    while !successor.close_step() {}
}

#[test]
fn retained_pane_focus_follows_the_last_focus_event_and_ignores_an_old_surface_blur() {
    let fixture = pane_owner_fixture();
    let cases = fixture["cases"].as_array().unwrap();
    let mut shell = super::panel_anchor_model_tests::host_test_shell();
    let mut documents = Vec::new();
    for row in cases {
        let surface = row["surface"].as_str().unwrap();
        let document = shell.publish_surface_records(surface, pane_owner_select_records()).unwrap();
        documents.push(document);
    }
    let paint_rows: Vec<_> = cases.iter().zip(documents.iter()).map(|(row, document)| (row["surface"].as_str().unwrap(), document, Rect::new(0.0, 0.0, 320.0, 120.0))).collect();
    let mut input = paint_tree_pointer_documents(&mut shell, &paint_rows);
    let mut observations = Vec::new();
    let mut previous: Option<String> = None;
    for kind in fixture["focusSequence"].as_array().unwrap() {
        let row = cases.iter().find(|row| &row["kind"] == kind).unwrap();
        let surface = row["surface"].as_str().unwrap();
        let target = crate::interpreter::published_accessibility_target_for_test(surface, "pane-owner-select").expect("the published pane exposes its exact accessible address");
        let accepted = semio_framework_async::block_on(shell.handle_accessibility_event(&target, &ui_render::AccessibilityEvent::Focus, &mut input)).unwrap();
        observations.push((accepted, shell.active_window_id.clone(), shell.retained_keyboard_focus().map(|(surface, _)| surface), previous.as_ref().is_some_and(|old| shell.chrome_build.content_has_focus(old))));
        previous = Some(surface.into());
    }
    let blurred = cases.iter().find(|row| row["kind"] == fixture["lateBlur"]).unwrap()["surface"].as_str().unwrap();
    let target = crate::interpreter::published_accessibility_target_for_test(blurred, "pane-owner-select").expect("the published pane exposes its exact accessible address");
    semio_framework_async::block_on(shell.handle_accessibility_event(&target, &ui_render::AccessibilityEvent::Blur, &mut input)).unwrap();
    let final_focus = shell.retained_keyboard_focus().map(|(surface, _)| surface);
    for mut document in documents {
        while !document.close_step() {}
    }
    for (kind, (accepted, owner, focus, old_has_focus)) in fixture["focusSequence"].as_array().unwrap().iter().zip(observations) {
        let row = cases.iter().find(|row| &row["kind"] == kind).unwrap();
        assert!(accepted);
        assert_eq!(owner.as_deref(), row["window"].as_str(), "{kind} concrete owner");
        assert_eq!(focus.as_deref(), row["surface"].as_str(), "{kind} latest focused surface");
        assert!(!old_has_focus, "{kind} clears its previously focused sibling");
    }
    assert_eq!(final_focus, previous, "an old surface blur cannot clear the newest focused surface");
    println!("[DEBUG] retained focus followed the newest pane event and ignored late sibling blur");
}

fn pane_owner_fixture() -> Value {
    serde_json::from_str(include_str!("../../🧫️fixtures/🪪️window-surface-owner/🔣️.json")).expect("window surface ownership fixture")
}

#[test]
fn retained_pane_pointer_activation_preserves_the_concrete_window_owner() {
    let fixture = pane_owner_fixture();
    for case in fixture["cases"].as_array().unwrap() {
        let mut shell = ShellState::new(Vec::new(), String::new());
        let owner = case["window"].as_str().unwrap();
        let surface = case["surface"].as_str().unwrap();
        let previous = fixture["previousWindow"].as_str().unwrap();
        shell.dock.root = DockNode::Row(vec![(DockNode::Stack { windows: vec![DockStackTab::new(previous)], active: previous.into() }, 1.0), (DockNode::Stack { windows: vec![DockStackTab::new(owner)], active: owner.into() }, 1.0)]);
        shell.dock.sync_active_window(previous);
        shell.active_window_id = Some(previous.into());
        let mut input = InputState::<ActionDescriptor>::default();
        semio_framework_async::block_on(shell.route_retained_pointer_press(surface, Rect::new(0.0, 0.0, 320.0, 120.0), 15.0, 15.0, true, 0, HitKind::Input, None, ui_render::PointerId(1), &mut input)).unwrap();
        assert_eq!(shell.active_window_id.as_deref(), Some(owner), "{} shell owner", case["kind"]);
        assert_eq!(shell.dock.active_window_id.as_deref(), Some(owner), "{} dock owner", case["kind"]);
        assert_eq!(shell.dock.active_stack, Some(vec![1]), "{} active stack", case["kind"]);
    }
    println!("[DEBUG] all retained pane pointer presses activated their concrete window and dock stack");
}

#[test]
fn retained_pane_accessibility_and_keyboard_focus_keep_surface_and_window_distinct() {
    let fixture = pane_owner_fixture();
    for case in fixture["cases"].as_array().unwrap() {
        let owner = case["window"].as_str().unwrap();
        let surface = case["surface"].as_str().unwrap();
        let records = pane_owner_select_records();
        let mut shell = super::panel_anchor_model_tests::host_test_shell();
        let mut document = shell.publish_surface_records(surface, records).expect("pane document publishes");
        let mut input = paint_tree_pointer_document(&mut shell, surface, &document, Rect::new(0.0, 0.0, 320.0, 120.0));
        shell.active_window_id = Some(fixture["previousWindow"].as_str().unwrap().into());
        let target = crate::interpreter::published_accessibility_target_for_test(surface, "pane-owner-select").expect("the published pane exposes its exact accessible address");
        let focused = semio_framework_async::block_on(shell.handle_accessibility_event(&target, &ui_render::AccessibilityEvent::Focus, &mut input)).unwrap();
        let active = shell.active_window_id.clone();
        let keyboard_surface = shell.retained_keyboard_focus().map(|(surface, _)| surface);
        let select_owns_keyboard = shell.retained_select_owns_keyboard();
        while !document.close_step() {}
        assert!(focused, "{} accessibility focus dispatch", case["kind"]);
        assert_eq!(active.as_deref(), Some(owner), "{} active window", case["kind"]);
        assert_eq!(keyboard_surface.as_deref(), Some(surface), "{} keyboard surface", case["kind"]);
        assert!(select_owns_keyboard, "{} Select keyboard ownership", case["kind"]);
    }
    println!("[DEBUG] pane accessibility focus preserved concrete window activation and exact keyboard surface");
}

#[test]
fn retained_key_mapper_separates_select_typeahead_and_space_from_input_text() {
    let modifiers = PointerModifiers::default();
    assert!(matches!(ui_event_from_key_action(&ui_wgpu::wgpu::KeyAction::Char("d".into()), &modifiers, true), Some(ui_wgpu::wgpu::UiEvent::KeyDown { key, .. }) if key == "d"));
    assert!(matches!(ui_event_from_key_action(&ui_wgpu::wgpu::KeyAction::Space(true), &modifiers, true), Some(ui_wgpu::wgpu::UiEvent::KeyDown { key, .. }) if key == " "));
    assert!(ui_event_from_key_action(&ui_wgpu::wgpu::KeyAction::Space(false), &modifiers, true).is_none());
    assert!(matches!(ui_event_from_key_action(&ui_wgpu::wgpu::KeyAction::Char("d".into()), &modifiers, false), Some(ui_wgpu::wgpu::UiEvent::TextInput { text }) if text == "d"));
}

#[test]
fn accessibility_focus_arms_shell_select_space_and_typeahead_routing() {
    let surface = "control-focus-select";
    let records: Vec<ui_contract::UiNodeRecord> = serde_json::from_value(serde_json::json!([
        {
            "id": 1,
            "key": "framework.settings.appearance",
            "component": { "type": "select", "value": "system", "items": [{ "value": "system", "label": "System" }, { "value": "dark", "label": "Dark" }] },
            "layout": { "kind": "leaf", "width": "fill", "height": "hug" },
            "style": {}, "activity": "idle", "accessibility": { "label": "Appearance" },
            "bindings": [{ "trigger": "change", "action": { "scope": "framework", "name": "setAppearance", "version": 1 } }]
        }
    ]))
    .unwrap();
    let mut shell = super::panel_anchor_model_tests::host_test_shell();
    let mut document = shell.publish_surface_records(surface, records).expect("Select surface publishes");
    let mut input = paint_tree_pointer_document(&mut shell, surface, &document, Rect::new(0.0, 0.0, 320.0, 120.0));
    shell.active_window_id = Some(surface.to_string());
    let target = crate::interpreter::published_accessibility_target_for_test(surface, "framework.settings.appearance").expect("the published pane exposes its exact accessible address");
    assert!(semio_framework_async::block_on(shell.handle_accessibility_event(&target, &ui_render::AccessibilityEvent::Focus, &mut input)).unwrap());
    assert!(shell.chrome_build.content_has_focus(surface));
    assert!(shell.retained_select_owns_keyboard());

    semio_framework_async::block_on(shell.handle_keyboard_async(ui_wgpu::wgpu::KeyAction::Space(true), &PointerModifiers::default(), &mut input)).unwrap();
    assert!(crate::collect_fixture_actions(&mut input).is_empty(), "opening with Space commits nothing");
    semio_framework_async::block_on(shell.handle_keyboard_async(ui_wgpu::wgpu::KeyAction::Space(true), &PointerModifiers::default(), &mut input)).unwrap();
    let space = crate::collect_fixture_actions(&mut input);
    assert_eq!(space.iter().map(|action| action.action.as_str()).collect::<Vec<_>>(), ["setAppearance"]);

    semio_framework_async::block_on(shell.handle_keyboard_async(ui_wgpu::wgpu::KeyAction::Char("d".into()), &PointerModifiers::default(), &mut input)).unwrap();
    assert!(crate::collect_fixture_actions(&mut input).is_empty(), "typeahead opens and highlights before commit");
    semio_framework_async::block_on(shell.handle_keyboard_async(ui_wgpu::wgpu::KeyAction::Enter, &PointerModifiers::default(), &mut input)).unwrap();
    let typed = crate::collect_fixture_actions(&mut input);
    assert_eq!(typed.iter().map(|action| action.action.as_str()).collect::<Vec<_>>(), ["setAppearance"]);
    assert_eq!(typed[0].args.as_ref().and_then(|args| args.get("value")).and_then(DslValue::as_str), Some("dark"));
    while !document.close_step() {}
}

pub(super) fn tree_pointer_record(id: u64, key: &str, component: ui_contract::Component, children: &[u64], action: Option<&str>) -> ui_contract::UiNodeRecord {
    let mut child_ids = ui_contract::UiNodeChildren::default();
    for child in children {
        child_ids.try_push(ui_contract::UiNodeId(*child)).expect("tree pointer fixture child");
    }
    let mut bindings = ui_contract::UiNodeBindings::default();
    if let Some(action) = action {
        bindings
            .try_push(ui_contract::ActionBinding { trigger: ui_contract::Trigger::Activate, action: ui_contract::ActionId::try_v1("s.test.tree", action).expect("bounded tree pointer action"), args: None, capability: None })
            .expect("tree pointer fixture binding");
    }
    ui_contract::UiNodeRecord {
        id: ui_contract::UiNodeId(id),
        key: ui_contract::UiText::try_from_str(key).expect("bounded tree pointer key"),
        component,
        layout: PanelProjection::stack_layout(ui_contract::Axis::Vertical, ui_contract::SpaceToken::None),
        style: Default::default(),
        activity: Default::default(),
        disabled: false,
        transition: None,
        accessibility: Default::default(),
        bindings,
        menu: None,
        children: child_ids,
    }
}

fn published_tree_pointer_document(shell: &mut ShellState, surface: &str) -> (UiDocumentLease, String) {
    let mut item = UiTreeItemNode::base("transfer", Label::data("Transfer"));
    item.action = Some(ActionDescriptor { controller_id: "s.test.tree".into(), action: "selectTreeItem".into(), args: None });
    item.draggable = Some(true);
    item.drag_data = Some(HashMap::from([("application/x-semio-window-template".into(), "{\"windowKindId\":\"world\",\"templateId\":\"default\"}".into())]));
    let node = UiNode::Tree(UiTreeNode { presentation: Default::default(),
        sections: vec![UiTreeSectionNode { id: "section".into(), label: None, default_open: Some(true), presence: UiPresence::default(), items: vec![item], window: None }],
        presence: UiPresence::default(),
        drop_action: None,
        menu: None,
        interaction_domain: None,
    });
    let records = panel_ui_records(surface, &node).expect("the authored Tree projects through the production panel assembler");
    let item_key = records.iter().find(|record| matches!(&record.component, ui_contract::Component::TreeItem(_))).map(|record| record.key.as_str().to_string()).expect("the projected Tree carries its transfer row");
    (shell.publish_surface_records(surface, records).expect("tree pointer document publishes"), item_key)
}

fn published_canvas_catalogue_document(shell: &mut ShellState, surface: &str, raw_payload: &str) -> UiDocumentLease {
    let component = |wire: Value| serde_json::from_value(wire).expect("catalogue pointer component wire");
    let records = vec![
        tree_pointer_record(1, "root", component(serde_json::json!({ "type": "tree" })), &[2], None),
        tree_pointer_record(2, "section", component(serde_json::json!({ "type": "treeSection", "defaultOpen": true })), &[3], None),
        tree_pointer_record(
            3,
            "catalogue",
            component(serde_json::json!({
                "type": "treeItem",
                "label": "Catalogue item",
                "draggable": true,
                "dragData": {
                    "application/x-semio-catalogue-item": raw_payload,
                    "text/plain": "Catalogue item"
                }
            })),
            &[],
            None,
        ),
    ];
    shell.publish_surface_records(surface, records).expect("catalogue pointer document publishes")
}

fn paint_tree_pointer_documents(shell: &mut ShellState, documents: &[(&str, &UiDocumentLease, Rect)]) -> InputState<ActionDescriptor> {
    let mut draw = DrawList::default();
    let mut atlas = FontAtlas::builtin();
    let icons = IconAtlas::default();
    let mut input = InputState::<ActionDescriptor>::default();
    let theme = Theme::default();
    let (mut scroll, mut collapsed, mut selects) = (HashMap::new(), HashMap::new(), HashMap::new());
    let mut world3d_states = std::mem::take(&mut shell.world3d_states);
    let mut world_resources = infinite_world::world::World3dBuildContext::new(infinite_world::world::WorldCursorWakeAuthority::new());
    crate::interpreter::begin_accessibility_visible_documents();
    for (surface, document, body) in documents {
        let mut cursor = UiDocumentFrameCursor::default();
        let complete = (0..SHELL_WINDOW_PAINT_OPPORTUNITIES.min(1 << 20)).any(|_| {
            let mut ctx = framework_widget_context(&mut draw, None, &mut atlas, Some(&icons), &mut input, &theme, &mut scroll, &mut collapsed, &mut selects, None, body.h);
            let mut hosts = crate::scenes::SceneEngineHosts { chrome_labels: crate::scenes::SceneChromeLabels::english(), world3d_states: &mut world3d_states, world_resources: &mut world_resources, window_id: surface };
            let done = render_ui_document_step(&mut cursor, document, *body, &mut ctx, surface, "s.test.tree", ui_wgpu::wgpu::UiDriverDrag::Handle, &mut hosts);
            assert!(done || !cursor.terminal_is_fault(), "the tree pointer document faulted in phase {}", cursor.phase_name());
            done
        });
        assert!(complete, "the tree pointer document painted within its opportunity ceiling");
        shell.register_retained_body_hits(surface, *body, &mut input);
    }
    shell.world3d_states = world3d_states;
    shell.publish_retained_hit_registry(&mut input);
    input
}

fn paint_tree_pointer_document(shell: &mut ShellState, surface: &str, document: &UiDocumentLease, body: Rect) -> InputState<ActionDescriptor> {
    paint_tree_pointer_documents(shell, &[(surface, document, body)])
}

fn table_pointer_record(id: u64, key: &str, scene: &ui_wgpu::wgpu::TableScene) -> ui_contract::UiNodeRecord {
    let surface = ui_wgpu::wgpu::encode_surface_doc(ui_contract::SurfaceKind::Table, scene).expect("bounded Table scene encodes");
    tree_pointer_record(id, key, ui_contract::Component::Surface(surface), &[], None)
}

fn canvas_pointer_record(id: u64, key: &str, scene: &ui_wgpu::wgpu::Canvas2dScene) -> ui_contract::UiNodeRecord {
    let surface = ui_wgpu::wgpu::encode_surface_doc(ui_contract::SurfaceKind::Canvas2d, scene).expect("bounded Canvas2d scene encodes");
    tree_pointer_record(id, key, ui_contract::Component::Surface(surface), &[], None)
}

fn world3d_pointer_record(id: u64, key: &str, scene: &ui_wgpu::wgpu::World3dScene) -> ui_contract::UiNodeRecord {
    let surface = ui_wgpu::wgpu::encode_surface_doc(ui_contract::SurfaceKind::World3d, scene).expect("bounded World3d scene encodes");
    tree_pointer_record(id, key, ui_contract::Component::Surface(surface), &[], None)
}

fn runnable_window_body_fixture(_instance_id: u32, surface_id: &str, body_key: &str, _view_state: &ViewModel, _document_dsl: Option<&str>, _refresh_effects: Option<&mut Vec<semio_framework::kernel::Effect>>) -> Result<UiDocumentLease, String> {
    for section in [semio_framework::UiRefreshSection::Catalogue, semio_framework::UiRefreshSection::Engagements, semio_framework::UiRefreshSection::Measures, semio_framework::UiRefreshSection::Tools] {
        if body_key == section.body_key() {
            return Ok(crate::program_bridge::window_measures_section_tests::section_document(&serde_json::json!({}), 1, section));
        }
    }
    let scene = ui_wgpu::wgpu::World3dScene::base(serde_json::json!({ "position": [4.0, 4.0, 4.0], "target": [0.0, 0.0, 0.0], "projection": "perspective" }).to_string(), "[]".into(), "[]".into(), "{}".into());
    let records = vec![world3d_pointer_record(1, "initial-world", &scene)];
    let identity = ui_contract::UiDocumentAssemblyIdentity { generation: 1, revision: ui_contract::UiRevision(1), root: Some(ui_contract::UiNodeId(1)), layout_epoch: 0 };
    UiDocumentLease::try_publish(SurfaceId::try_from(surface_id).map_err(|_| "fixture surface exceeds the retained contract")?, identity, records).map_err(|error| crate::program_bridge::retained_publication_refusal(surface_id, error))
}

std::thread_local! {
    static WINDOW_JOURNAL_FIXTURE_REFUSALS: std::cell::Cell<usize> = const { std::cell::Cell::new(0) };
    static WINDOW_BODY_FIXTURE_REFUSALS: std::cell::Cell<usize> = const { std::cell::Cell::new(0) };
    static WINDOW_BODY_FIXTURE_ATTEMPTS: std::cell::Cell<usize> = const { std::cell::Cell::new(0) };
    static WINDOW_PUBLICATION_FIXTURE_EVENTS: std::cell::RefCell<Vec<String>> = const { std::cell::RefCell::new(Vec::new()) };
}

fn scripted_window_body_fixture(instance_id: u32, surface_id: &str, body_key: &str, view_state: &ViewModel, document_dsl: Option<&str>, refresh_effects: Option<&mut Vec<semio_framework::kernel::Effect>>) -> Result<UiDocumentLease, String> {
    if surface_id == "main-2" {
        WINDOW_BODY_FIXTURE_ATTEMPTS.with(|attempts| attempts.set(attempts.get() + 1));
        WINDOW_PUBLICATION_FIXTURE_EVENTS.with(|events| events.borrow_mut().push("render:main-2".into()));
        let refused = WINDOW_BODY_FIXTURE_REFUSALS.with(|refusals| {
            let remaining = refusals.get();
            if remaining > 0 {
                refusals.set(remaining - 1);
                true
            } else {
                false
            }
        });
        if refused {
            return Err("fixture refuses the required initial window body".to_string());
        }
    } else if surface_id == "main-3" {
        WINDOW_PUBLICATION_FIXTURE_EVENTS.with(|events| events.borrow_mut().push("render:main-3".into()));
    }
    runnable_window_body_fixture(instance_id, surface_id, body_key, view_state, document_dsl, refresh_effects)
}

fn refuse_window_body_fixture_action(_instance_id: u32, action_json: &str, _view_state: &ViewModel) -> Result<semio_framework::kernel::InvocationResult, String> {
    if action_json.contains("shell.windowSplit") || action_json.contains("shell.windowMove") {
        WINDOW_JOURNAL_FIXTURE_REFUSALS.with(|count| count.set(count.get() + 1));
        let instance = if action_json.contains("main-2") {
            "main-2"
        } else if action_json.contains("main-3") {
            "main-3"
        } else {
            "unknown"
        };
        WINDOW_PUBLICATION_FIXTURE_EVENTS.with(|events| events.borrow_mut().push(format!("journal:{instance}")));
    }
    Err("fixture guest refuses the journal action".into())
}

fn shell_with_scripted_window_publication(refusals: usize) -> ShellState {
    let mut shell = super::panel_anchor_model_tests::host_test_shell();
    shell.dock.root = DockNode::Stack { windows: vec![DockStackTab::instance("main-2", "main", WindowStackCorner::TopLeft)], active: "main-2".into() };
    shell.dock.active_window_id = Some("main-2".into());
    shell.active_window_id = Some("main-2".into());
    shell.persist_dock_layout();
    let program = shell.plugins.iter_mut().find(|program| program.plugin_id == "space").expect("the host fixture has its guest program");
    program.install_fixture_render(scripted_window_body_fixture);
    program.install_fixture_action(refuse_window_body_fixture_action);
    WINDOW_BODY_FIXTURE_REFUSALS.with(|remaining| remaining.set(refusals));
    WINDOW_BODY_FIXTURE_ATTEMPTS.with(|attempts| attempts.set(0));
    WINDOW_JOURNAL_FIXTURE_REFUSALS.with(|count| count.set(0));
    WINDOW_PUBLICATION_FIXTURE_EVENTS.with(|events| events.borrow_mut().clear());
    let controller_id = shell.shell_command_controller_id().expect("the fixture session journals transfers");
    let note = ShellState::note_shell_command_action(&controller_id, "shell.windowSplit", "Split Window", Some(serde_json::json!({ "windowKindId": "main", "instanceId": "main-2" })));
    let token = shell.reserve_window_topology_action(note).expect("the fixture transfer journal is admitted");
    shell.commit_window_topology_publication("main-2", "main.body", Some(token));
    shell
}

fn retire_scripted_window_publication_documents(shell: &mut ShellState) {
    shell.retire_documents_outside(&[], true).expect("scripted window documents retire");
    shell.retire_documents_outside(&[], false).expect("scripted panel documents retire");
    let auxiliary =
        std::mem::take(&mut shell.window_actions_documents).into_values().chain(std::mem::take(&mut shell.window_search_documents).into_values()).chain(std::mem::take(&mut shell.window_measures_documents).into_values()).collect::<Vec<_>>();
    for document in auxiliary {
        shell.retire_one_surface_document(Some(document)).expect("scripted auxiliary document retires");
    }
    shell.drain_retained_document_arenas();
}

pub(super) fn paint_component_pointer_documents(shell: &mut ShellState, documents: &[(&str, &str, &UiDocumentLease, Rect)]) -> InputState<ActionDescriptor> {
    let mut draw = DrawList::default();
    let mut atlas = FontAtlas::builtin();
    let icons = IconAtlas::default();
    let mut input = InputState::<ActionDescriptor>::default();
    let theme = Theme::default();
    let (mut scroll, mut collapsed, mut selects) = (HashMap::new(), HashMap::new(), HashMap::new());
    let mut world3d_states = AdmittedSurfaceMap::default();
    let mut world_resources = infinite_world::world::World3dBuildContext::new(infinite_world::world::WorldCursorWakeAuthority::new());
    crate::interpreter::begin_accessibility_visible_documents();
    for (window_id, controller_id, document, body) in documents {
        let mut cursor = UiDocumentFrameCursor::default();
        let complete = (0..SHELL_WINDOW_PAINT_OPPORTUNITIES.min(1 << 20)).any(|_| {
            let mut ctx = framework_widget_context(&mut draw, None, &mut atlas, Some(&icons), &mut input, &theme, &mut scroll, &mut collapsed, &mut selects, None, body.h);
            let mut hosts = crate::scenes::SceneEngineHosts { chrome_labels: crate::scenes::SceneChromeLabels::english(), world3d_states: &mut world3d_states, world_resources: &mut world_resources, window_id };
            let done = render_ui_document_step(&mut cursor, document, *body, &mut ctx, window_id, controller_id, ui_wgpu::wgpu::UiDriverDrag::Handle, &mut hosts);
            assert!(done || !cursor.terminal_is_fault(), "the component pointer document faulted in phase {}", cursor.phase_name());
            done
        });
        assert!(complete, "the component pointer document painted within its opportunity ceiling");
        shell.register_retained_body_hits(window_id, *body, &mut input);
    }
    shell.publish_retained_hit_registry(&mut input);
    input
}

#[test]
fn required_window_body_refusal_holds_the_transfer_journal_then_recovers_once() {
    let fixture: Value = serde_json::from_str(include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/../../../../🧫️fixtures/🪟️window-lifecycle-template-drag/🔣️.json"))).expect("window publication fixture");
    let recovery = fixture["publicationOutcomes"]["recovery"].as_array().expect("recovery sequence");
    let mut shell = shell_with_scripted_window_publication(1);

    assert_eq!(semio_framework_async::block_on(shell.settle_pump_step_inner()), ShellSettleStep::Drained);
    assert_eq!(WINDOW_BODY_FIXTURE_ATTEMPTS.with(|attempts| attempts.get()), 1);
    assert!(!shell.window_ui.contains_key("main-2"));
    assert!(shell.window_topology_refresh_owed);
    assert_eq!(shell.window_topology_actions.len(), 1);
    assert!(shell.deferred_actions.is_empty());
    assert_eq!(shell.surface_faults.iter().any(|fault| fault.surface_id == "main-2"), recovery[0]["surfaceFault"].as_bool().unwrap());

    assert_eq!(semio_framework_async::block_on(shell.settle_pump_step_inner()), ShellSettleStep::Drained);
    assert_eq!(WINDOW_BODY_FIXTURE_ATTEMPTS.with(|attempts| attempts.get()), 2);
    assert!(shell.window_ui.contains_key("main-2"));
    assert_eq!(shell.window_topology_refresh_owed, recovery[1]["topologyOwed"].as_bool().unwrap());
    assert!(shell.window_topology_actions.is_empty());
    assert!(shell.window_topology_publications.is_empty());
    assert_eq!(shell.deferred_actions.iter().filter(|action| action.action == "noteShellCommand").count(), 1);
    assert_eq!(WINDOW_JOURNAL_FIXTURE_REFUSALS.with(|count| count.get()), 0);

    assert_eq!(semio_framework_async::block_on(shell.settle_pump_step_inner()), ShellSettleStep::Drained);
    assert_eq!(WINDOW_JOURNAL_FIXTURE_REFUSALS.with(|count| count.get()), 1, "recovery releases exactly one journal on the following settle step");
    assert!(shell.deferred_actions.is_empty());
    retire_scripted_window_publication_documents(&mut shell);
}

#[test]
fn permanent_window_body_refusal_retires_the_journal_at_the_fixture_ceiling() {
    let fixture: Value = serde_json::from_str(include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/../../../../🧫️fixtures/🪟️window-lifecycle-template-drag/🔣️.json"))).expect("window publication fixture");
    let outcomes = &fixture["publicationOutcomes"];
    let retry_ceiling = outcomes["retryCeiling"].as_u64().expect("retry ceiling") as usize;
    assert_eq!(retry_ceiling, usize::from(WINDOW_TOPOLOGY_PUBLICATION_ATTEMPTS));
    let mut shell = shell_with_scripted_window_publication(retry_ceiling);

    for attempt in 0..retry_ceiling {
        assert_eq!(semio_framework_async::block_on(shell.settle_pump_step_inner()), ShellSettleStep::Drained);
        if attempt + 1 < retry_ceiling {
            assert!(shell.window_topology_refresh_owed);
            assert_eq!(shell.window_topology_actions.len(), 1);
        }
    }
    assert_eq!(WINDOW_BODY_FIXTURE_ATTEMPTS.with(|attempts| attempts.get()), retry_ceiling);
    assert!(!shell.window_topology_refresh_owed);
    assert!(shell.window_topology_actions.is_empty());
    assert!(shell.window_topology_publications.is_empty());
    assert!(shell.deferred_actions.is_empty());
    assert_eq!(WINDOW_JOURNAL_FIXTURE_REFUSALS.with(|count| count.get()), 0, "a permanently refused body never emits a success-shaped journal");
    let fault = shell.surface_faults.iter().find(|fault| fault.surface_id == "main-2").expect("terminal publication fault");
    assert!(fault.detail.contains(&format!("after {retry_ceiling} attempts")));
    assert_eq!(outcomes["permanentFault"]["journal"], "terminal-refusal");
    assert_eq!(outcomes["permanentFault"]["surfaceFault"], true);
    retire_scripted_window_publication_documents(&mut shell);
}

#[test]
fn topology_journal_credit_refusal_is_returned_to_the_transfer_caller() {
    let mut shell = super::panel_anchor_model_tests::host_test_shell();
    for _ in 0..ui_wgpu::wgpu::action::ACTION_QUEUE_ITEM_CAPACITY {
        shell.reserve_window_topology_action(ActionDescriptor { controller_id: "fixture".into(), action: "occupied".into(), args: None }).expect("the occupied fixture journal publishes");
    }
    let error = shell.reserve_window_topology_action(ActionDescriptor { controller_id: "test".into(), action: "noteShellCommand".into(), args: None }).expect_err("a full action queue returns its exact refusal");
    assert_eq!(error, ui_wgpu::wgpu::BoundedActionFault::ItemCredits);
    assert!(shell.window_topology_publications.is_empty(), "refusal precedes every topology owner");
    assert!(shell.deferred_actions.is_empty());
}

fn canvas_input_fixture() -> Value {
    serde_json::from_str(include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/../../../../🧱️elements/📐️Canvas2dHost/🧫️fixtures/🖱️input-contract/🔣️.json"))).expect("shared Canvas2d input fixture")
}

fn assert_json_number_eq(actual: &Value, expected: &Value, field: &str) {
    assert_eq!(actual.as_f64().unwrap_or_else(|| panic!("{field} actual number")), expected.as_f64().unwrap_or_else(|| panic!("{field} expected number")), "{field}");
}

fn canvas_action_args(action: &ActionDescriptor) -> Value {
    dsl_value_as_json(action.args.as_ref().expect("Canvas2d action args"))
}
#[test]
fn standalone_multi_app_variants_resolve_their_declared_app() {
    assert_eq!(resolve_playground_app_id("puzzle2d"), Some("s.puzzle.puzzle2d@1/*#editor"));
    assert_eq!(resolve_playground_app_id("puzzle3d"), Some("s.puzzle.puzzle3d@1/*#editor"));
    assert_eq!(resolve_playground_app_id("3d"), Some("s.puzzle.puzzle3d@1/*#editor"));
    assert_eq!(resolve_playground_app_id("puzzle5d"), Some("s.puzzle.puzzle5d@1/*#editor"));
    assert_eq!(resolve_registry_plugin_id("generation3d"), "procedural");
    assert_eq!(resolve_playground_app_id("generation3d"), Some("s.procedural.generation3d@1/*#editor"));
}

//#region SilhouetteContentTests

#[test]
fn silhouette_hit_intersections_leave_the_cap_gap_empty() {
    let silhouette = WindowSilhouette::from_measured_top(Rect::new(0.0, 0.0, 300.0, 200.0), 80.0, 60.0, 32.0);
    let hits: Vec<Rect> = silhouette.content_clip_rects().iter().filter_map(|clip| ShellState::intersect_content_rect(*clip, silhouette.bounds)).collect();
    assert_eq!(hits.len(), 3);
    assert!(!hits.iter().any(|rect| rect.contains(150.0, 16.0)));
    assert!(hits.iter().any(|rect| rect.contains(150.0, 100.0)));
}

//#endregion SilhouetteContentTests

/// 🧪️ `dock_window_order` is a `Self`-less associated fn, so it's callable without constructing a
/// full `ShellState` fixture (impractically large: 90+ fields, several without `Default`).
fn window_ids(node: &crate::dock::DockNode) -> Vec<String> {
    let mut out = Vec::new();
    ShellState::dock_window_order(node, &mut Vec::new(), &mut out);
    out.into_iter().map(|(_, window_id)| window_id).collect()
}

#[test]
fn dock_window_order_flattens_a_single_stack_in_tab_order() {
    let node = crate::dock::DockNode::Stack { windows: vec![DockStackTab::new("a"), DockStackTab::new("b"), DockStackTab::new("c")], active: "a".into() };
    assert_eq!(window_ids(&node), vec!["a", "b", "c"]);
}

#[test]
fn dock_window_order_walks_row_and_column_children_depth_first() {
    let left = crate::dock::DockNode::Stack { windows: vec![DockStackTab::new("left")], active: "left".into() };
    let right_top = crate::dock::DockNode::Stack { windows: vec![DockStackTab::new("top")], active: "top".into() };
    let right_bottom = crate::dock::DockNode::Stack { windows: vec![DockStackTab::new("bottom")], active: "bottom".into() };
    let right = crate::dock::DockNode::Column(vec![(right_top, 0.5), (right_bottom, 0.5)]);
    let root = crate::dock::DockNode::Row(vec![(left, 0.5), (right, 0.5)]);
    assert_eq!(window_ids(&root), vec!["left", "top", "bottom"]);
}

#[test]
fn dock_window_order_pairs_each_window_with_its_own_stack_path() {
    let a = crate::dock::DockNode::Stack { windows: vec![DockStackTab::new("a")], active: "a".into() };
    let b = crate::dock::DockNode::Stack { windows: vec![DockStackTab::new("b")], active: "b".into() };
    let root = crate::dock::DockNode::Row(vec![(a, 0.5), (b, 0.5)]);
    let mut out = Vec::new();
    ShellState::dock_window_order(&root, &mut Vec::new(), &mut out);
    assert_eq!(out, vec![(vec![0], "a".to_string()), (vec![1], "b".to_string())]);
}

// 🎯️🕹️ w2-input-wiring: key mapping is pure and focus tracking is explicit owned build state,
// so both remain testable without a full `ShellState` fixture.

#[test]
fn ui_event_from_key_action_maps_plain_char_to_text_input() {
    let modifiers = PointerModifiers::default();
    let event = ui_event_from_key_action(&ui_wgpu::wgpu::KeyAction::Char("a".into()), &modifiers, false);
    assert_eq!(event, Some(ui_wgpu::wgpu::UiEvent::TextInput { text: "a".into() }));
}

#[test]
fn ui_event_from_key_action_routes_ctrl_char_as_key_down_for_clipboard_chords() {
    let modifiers = PointerModifiers { ctrl: true, ..Default::default() };
    let event = ui_event_from_key_action(&ui_wgpu::wgpu::KeyAction::Char("c".into()), &modifiers, false);
    assert_eq!(event, Some(ui_wgpu::wgpu::UiEvent::KeyDown { key: "c".into(), modifiers: ui_wgpu::wgpu::EventModifiers { shift: false, ctrl: true, alt: false, meta: false } }));
}

#[test]
fn ui_event_from_key_action_maps_editing_and_tab_keys_to_matching_key_down_strings() {
    let modifiers = PointerModifiers::default();
    let cases = [
        (ui_wgpu::wgpu::KeyAction::Backspace, "Backspace"),
        (ui_wgpu::wgpu::KeyAction::Delete, "Delete"),
        (ui_wgpu::wgpu::KeyAction::Enter, "Enter"),
        (ui_wgpu::wgpu::KeyAction::Escape, "Escape"),
        (ui_wgpu::wgpu::KeyAction::ArrowLeft, "ArrowLeft"),
        (ui_wgpu::wgpu::KeyAction::ArrowRight, "ArrowRight"),
        (ui_wgpu::wgpu::KeyAction::ArrowUp, "ArrowUp"),
        (ui_wgpu::wgpu::KeyAction::ArrowDown, "ArrowDown"),
        (ui_wgpu::wgpu::KeyAction::Home, "Home"),
        (ui_wgpu::wgpu::KeyAction::End, "End"),
        (ui_wgpu::wgpu::KeyAction::PageUp, "PageUp"),
        (ui_wgpu::wgpu::KeyAction::PageDown, "PageDown"),
        (ui_wgpu::wgpu::KeyAction::Tab, "Tab"),
    ];
    for (action, key) in cases {
        let event = ui_event_from_key_action(&action, &modifiers, false);
        assert_eq!(event, Some(ui_wgpu::wgpu::UiEvent::KeyDown { key: key.into(), modifiers: ui_wgpu::wgpu::EventModifiers::default() }), "KeyAction {action:?} should map to KeyDown{{{key}}}");
    }
}

#[test]
fn ui_event_from_key_action_has_no_mapping_for_space() {
    let event = ui_event_from_key_action(&ui_wgpu::wgpu::KeyAction::Space(true), &PointerModifiers::default(), false);
    assert_eq!(event, None);
}

#[test]
fn content_focus_tracker_defaults_unfocused_and_clears_a_typed_focus_record() {
    let mut chrome = ShellChromeBuildState::default();
    let window_id = "w2-input-wiring-test-window-a";
    assert!(!chrome.content_has_focus(window_id));
    let mut arena: ui_wgpu::wgpu::Arena<()> = ui_wgpu::wgpu::Arena::new();
    let node_id = arena.insert(());
    chrome.content_focus.insert(window_id.into(), Some(RetainedContentFocus { node: node_id, key: ui_wgpu::wgpu::NodeKey::Explicit("typed-focus".into()), kind: RetainedNodeFocusKind::Input }));
    chrome.focused_retained_surface = Some(window_id.into());
    assert!(chrome.content_has_focus(window_id));
    chrome.note_content_focus_commands(&[ui_wgpu::wgpu::UiCommand::FocusChanged { window_id: window_id.to_string(), node: None }]);
    assert!(!chrome.content_has_focus(window_id));
}

#[test]
fn content_focus_tracker_keeps_a_typed_record_scoped_to_its_window() {
    let mut chrome = ShellChromeBuildState::default();
    let window_id = "w2-input-wiring-test-window-b";
    let other_window_id = "w2-input-wiring-test-window-c";
    let mut arena: ui_wgpu::wgpu::Arena<()> = ui_wgpu::wgpu::Arena::new();
    let node_id = arena.insert(());
    chrome.content_focus.insert(other_window_id.into(), Some(RetainedContentFocus { node: node_id, key: ui_wgpu::wgpu::NodeKey::Explicit("other-focus".into()), kind: RetainedNodeFocusKind::Input }));
    assert!(!chrome.content_has_focus(window_id));
    assert!(chrome.content_has_focus(other_window_id));
}

#[test]
fn content_focus_tracker_ignores_non_focus_commands() {
    let mut chrome = ShellChromeBuildState::default();
    let window_id = "w2-input-wiring-test-window-d";
    chrome.note_content_focus_commands(&[ui_wgpu::wgpu::UiCommand::ClipboardPasteRequested { window_id: window_id.to_string() }]);
    assert!(!chrome.content_has_focus(window_id));
}

/// 🕒️ `finish_dock_drag`'s successful-drop branch persists the new layout and clears the drag —
/// unchanged behavior this ticket's `noteShellCommand` dispatch is appended after, not instead of.
/// No host app is configured (`ShellState::new`'s bare fixture, same "impractically large" 90+-field
/// constraint as `dock_window_order`'s own fixture note above), so `host_controller_id()` is `None`
/// and the new dispatch is a documented no-op here — this pins the "must still complete without a
/// host to log against" half of that behavior; `note_shell_command_action`'s own shape (the other
/// half) is covered directly in `command_registry_tests`.
///
/// 🎬️ A live drag never edits the committed tree, so the drop zone is a path into
/// `DockState::render_view`'s derivation — lifting `a` out of the lone axis child hoists the stack
/// to the ROOT, exactly as React's `collapseLayout` does.
#[test]
fn finish_dock_drag_persists_layout_and_clears_drag_state_on_successful_drop() {
    let mut shell = ShellState::new(Vec::new(), String::new());
    shell.dock.root = crate::dock::DockNode::Row(vec![(crate::dock::DockNode::Stack { windows: vec![DockStackTab::new("a"), DockStackTab::new("b"), DockStackTab::new("c")], active: "a".into() }, 1.0)]);
    let payload = DockDragPayload { kind: DockDragKind::Tab, window_id: "a".into(), window_kind_id: "a".into(), template_id: None, source_path: vec![0], tab_index: 0, ghost_label: "a".into() };
    let zone = DockDropZone::Tab { stack_path: vec![], corner: WindowStackCorner::TopLeft, index: 2 };
    shell.dock_drag = Some(DockDragState { payload, x: 10.0, y: 10.0, drop_zone: Some(zone) });
    assert!(shell.layout_override.is_none(), "sanity: nothing persisted yet");
    let input = InputState::<ActionDescriptor>::default();
    let result = semio_framework_async::block_on(shell.finish_dock_drag(10.0, 10.0, &input));
    assert!(result.is_ok(), "finish_dock_drag must not error even without a host app to log a shell.windowMove against");
    assert!(shell.layout_override.is_some(), "a successful drop persists the new dock layout");
    assert!(shell.dock_drag.is_none(), "the transient drag state is always taken");
}

/// 🛰️ A browser-normalized press and move preserve Chrome capture and promote the exact dock-tab
/// grip after the five-pixel threshold, before any release mutates the committed dock.
#[test]
fn normalized_dock_tab_pointer_sequence_promotes_the_drag_before_release() {
    let stack = |window_id: &str| DockNode::Stack { windows: vec![DockStackTab::instance(window_id, "main", WindowStackCorner::TopLeft)], active: window_id.to_string() };
    let mut shell = super::panel_anchor_model_tests::host_test_shell();
    shell.dock.root = DockNode::Row(vec![(stack("top"), 0.5), (stack("perspective"), 0.5)]);
    shell.dock.mobile = false;
    let rect = Rect::new(80.0, 35.0, 18.0, 23.0);
    let mut input = InputState::<ActionDescriptor>::default();
    input.register_hit(HitTarget { rect, event: None, control_id: Some("dock.tab.0.top.drag".into()), kind: HitKind::Button, drag_axis: None, drag_data: None });
    input.publish_hits();
    let mut interaction = pointer_interaction(shell, input);
    let pointer = ui_render::PointerInfo { id: ui_render::PointerId(1), kind: ui_render::PointerKind::Mouse, pressure: Some(0.5), tilt: None };
    let start = (rect.x + rect.w * 0.5, rect.y + rect.h * 0.5);
    semio_framework_async::block_on(crate::winit_app::dispatch_normalized_event(
        &mut interaction,
        ui_render::DispatchEvent::PointerDown { pointer, x: start.0, y: start.1, button: ui_render::PointerButton::Primary, modifiers: ui_render::EventModifiers::default() },
    ));
    assert!(interaction.shell.pending_dock_drag.as_ref().is_some_and(|(payload, origin)| payload.kind == DockDragKind::Tab && payload.window_id == "top" && *origin == start));
    assert_eq!(interaction.pointer_capture.holder(pointer.id), Some(PointerHitOwner::Chrome));

    semio_framework_async::block_on(crate::winit_app::dispatch_normalized_event(&mut interaction, ui_render::DispatchEvent::PointerMove { pointer, x: start.0 + 6.0, y: start.1, modifiers: ui_render::EventModifiers::default() }));
    assert!(interaction.shell.pending_dock_drag.is_none());
    assert!(interaction.shell.dock_drag.as_ref().is_some_and(|drag| drag.payload.kind == DockDragKind::Tab && drag.payload.window_id == "top"));
    assert_eq!(interaction.shell.dock.collect_window_ids(), vec!["top".to_string(), "perspective".to_string()], "promotion derives a ghost without mutating the committed dock");

    interaction.handle_pointer_cancel(pointer.id);
    assert!(interaction.shell.pending_dock_drag.is_none() && interaction.shell.dock_drag.is_none());
}

#[test]
fn context_menu_point_resolves_the_exact_concrete_window_instance() {
    let mut shell = ShellState::new(Vec::new(), String::new());
    shell.dock_drop_bodies = vec![(Vec::new(), Rect::new(0.0, 0.0, 100.0, 100.0), "canvas".into()), (vec![1], Rect::new(100.0, 0.0, 100.0, 100.0), "canvas-copy".into())];
    let mut input = InputState::<ActionDescriptor>::default();
    assert_eq!(shell.context_window_instance_id(25.0, 25.0), None, "candidate geometry does not route input before presentation");
    shell.publish_retained_hit_registry(&mut input);
    assert_eq!(shell.context_window_instance_id(25.0, 25.0), Some("canvas"));
    assert_eq!(shell.context_window_instance_id(125.0, 25.0), Some("canvas-copy"));
    assert_eq!(shell.context_window_instance_id(250.0, 25.0), None);
    println!("[DEBUG] native context menu resolved its exact concrete window and preserved panel scope outside dock bodies");
}

/// ⚖️ LAW: pressing a window's retained BODY activates that window, so the keyboard follows the
/// window the user actually clicked into rather than whichever one the session opened with.
///
/// 🩸️ `handle_keyboard_async` routes real keys into retained content on exactly one predicate —
/// `content_has_focus(active_window_id)` — and a retained body press was the one way into a window
/// that never set `active_window_id`. Measured on 6118: pressing the Generations window's inline
/// rename editor logged `content focus window=generation3d-generations node=Some(..)` and then every
/// keystroke logged `key routing window=procedural-main contentFocus=false`, so the editor opened,
/// took focus, and could not be typed into (ticket 26/09/09/PROCEDURAL-3D-END-TO-END,
/// `📓️wgpu-generation-publication-2026-09-13.md`).
#[test]
fn a_retained_body_press_activates_its_own_window_so_the_keyboard_follows_it() {
    let mut shell = ShellState::new(Vec::new(), String::new());
    let opened_with = "procedural-main";
    let pressed = "generation3d-generations";
    shell.active_window_id = Some(opened_with.into());
    let mut document = shell.publish_surface_records(pressed, pane_owner_select_records_with("body-focus", "system")).expect("pressed window document publishes");
    let body = Rect::new(3.0, 54.0, 315.0, 814.0);
    let mut input = paint_tree_pointer_document(&mut shell, pressed, &document, body);
    let target = crate::interpreter::published_accessibility_target_for_test(pressed, "body-focus").expect("the published pane exposes its exact accessible address");
    assert!(semio_framework_async::block_on(shell.handle_accessibility_event(&target, &ui_render::AccessibilityEvent::Focus, &mut input)).unwrap());
    assert!(!shell.chrome_build.content_has_focus(opened_with), "sanity: the window the keyboard used to follow never had content focus");

    semio_framework_async::block_on(shell.route_retained_pointer_press(pressed, body, 255.0, 90.0, true, 0, HitKind::Input, None, ui_render::PointerId(1), &mut input)).expect("a retained body press routes");

    assert_eq!(shell.active_window_id.as_deref(), Some(pressed), "the pressed window is the active one");
    assert!(shell.chrome_build.content_has_focus(shell.active_window_id.as_deref().expect("an active window")), "…so the keyboard's own predicate now answers for the window whose content holds focus");
    while !document.close_step() {}
    println!("[DEBUG] retained body press moved the active window {opened_with} -> {pressed}");
}

/// ⚖️ LAW: the RELEASE half of the same click changes nothing about activation — a click activates on
/// the press, the way every window manager and every browser does, and the action fires on release
/// (`route_retained_pointer_press`'s own `else if !down` arm).
#[test]
fn only_the_press_half_of_a_body_click_moves_the_active_window() {
    let mut shell = ShellState::new(Vec::new(), String::new());
    shell.active_window_id = Some("procedural-main".into());
    let mut input = InputState::<ActionDescriptor>::default();
    let body = Rect::new(3.0, 54.0, 315.0, 814.0);
    semio_framework_async::block_on(shell.route_retained_pointer_press("generation3d-generations", body, 255.0, 90.0, false, 0, HitKind::Input, None, ui_render::PointerId(1), &mut input)).expect("a retained body release routes");
    assert_eq!(shell.active_window_id.as_deref(), Some("procedural-main"), "a release alone never activates — the press already did, or nothing did");
}

/// ⚖️ LAW: the production Shell host routes BOTH semantic targets of a published handle-driven
/// Tree through the retained router. The trailing handle arms and promotes the retained drag across
/// the real down/move/up ingress without firing the row selection, while the remaining label band
/// still commits that row's published `Activate` binding on a later click.
#[test]
fn published_tree_handle_routes_through_shell_and_preserves_label_selection() {
    let surface = "tree-pointer-host-boundary";
    let body = Rect::new(17.0, 31.0, 360.0, 240.0);
    let theme = Theme::default();
    let mut shell = ShellState::new(Vec::new(), String::new());
    let (mut document, item_key) = published_tree_pointer_document(&mut shell, surface);
    let mut input = paint_tree_pointer_document(&mut shell, surface, &document, body);
    assert_eq!(crate::interpreter::accessibility_visible_window_ids(), vec![surface.to_string()], "the same completed retained-body publication owns the unnamed accessibility surface set");

    let handle_id = format!("tree.drag.transfer.{item_key}");
    let handle = input.hits().iter().find(|hit| hit.control_id.as_deref() == Some(handle_id.as_str())).expect("the published Handle driver exposes the transfer affordance").rect;
    let row_id = format!("tree.label.{item_key}");
    let row = input.hits().iter().find(|hit| hit.control_id.as_deref() == Some(row_id.as_str())).expect("the same published row keeps its label target").rect;
    let (handle_x, handle_y) = (handle.x + handle.w * 0.5, handle.y + handle.h * 0.5);
    semio_framework_async::block_on(shell.handle_pointer_button(handle_x, handle_y, true, 0, &mut input, &theme)).expect("Shell routes handle down");
    shell.handle_pointer_move(handle_x - 8.0, handle_y, true, &mut input, &theme);
    let sessions = crate::interpreter::active_retained_drag_sessions();
    assert_eq!(sessions.len(), 1, "the Shell move promotes the retained Tree press to one drag session");
    assert_eq!(sessions[0].0, surface);
    assert_eq!(sessions[0].3.get("application/x-semio-window-template").map(String::as_str), Some("{\"windowKindId\":\"world\",\"templateId\":\"default\"}"));
    semio_framework_async::block_on(shell.handle_pointer_button(handle_x - 8.0, handle_y, false, 0, &mut input, &theme)).expect("Shell routes handle up");
    assert!(crate::interpreter::active_retained_drag_sessions().is_empty(), "release retires the retained drag session");
    assert!(crate::collect_fixture_actions(&mut input).is_empty(), "a handle gesture never aliases the row-label selection");

    let (label_x, label_y) = (row.x + theme.gap_standard.max(1.0), row.y + row.h * 0.5);
    semio_framework_async::block_on(shell.handle_pointer_button(label_x, label_y, true, 0, &mut input, &theme)).expect("Shell routes label down");
    semio_framework_async::block_on(shell.handle_pointer_button(label_x, label_y, false, 0, &mut input, &theme)).expect("Shell routes label up");
    let actions = crate::collect_fixture_actions(&mut input);
    assert_eq!(actions.len(), 1, "the label click still commits exactly one selection action");
    assert_eq!(actions[0].controller_id, "s.test.tree");
    assert_eq!(actions[0].action, "selectTreeItem");
    let args = dsl_value_as_json(actions[0].args.as_ref().expect("the retained host scopes the selection"));
    assert_eq!(args["windowId"], surface);
    crate::interpreter::begin_accessibility_visible_documents();
    crate::interpreter::publish_accessibility_visible_documents();
    assert!(crate::interpreter::accessibility_visible_window_ids().is_empty(), "a later complete walk with no retained body removes the live document from production accessibility without retiring it");
    while !document.close_step() {}
}

struct DisplayTransferHostFixture {
    shell: ShellState,
    document: UiDocumentLease,
    input: InputState<ActionDescriptor>,
    theme: Theme,
    source: (f32, f32),
}

fn display_transfer_host_fixture() -> DisplayTransferHostFixture {
    let surface = "display-transfer-negative-boundary";
    let body = Rect::new(19.0, 37.0, 420.0, 300.0);
    let theme = Theme::default();
    let mut shell = super::panel_anchor_model_tests::host_test_shell();
    let mut node = shell.build_display_windows_ui();
    let UiNode::Stack(stack) = &mut node else { panic!("Display Windows publishes a stack") };
    let Some(UiNode::Tree(tree)) = stack.children.first_mut() else { panic!("Display Windows stack publishes a tree") };
    tree.sections.iter_mut().find(|section| section.id == "framework.display.windows.main").expect("the real app's main window-kind section").default_open = Some(true);
    let records = panel_ui_records(surface, &node).expect("the real Display producer projects");
    let projected_item_key = records
        .iter()
        .find(|record| record.key.as_str().ends_with("framework.display.windows.main.kind") && matches!(&record.component, ui_contract::Component::TreeItem(_)))
        .map(|record| record.key.as_str().to_string())
        .expect("the real Display producer projects its transfer row");
    shell.dock = DockState::default();
    shell.dock_canvas_bounds = Rect::new(500.0, 0.0, 400.0, 600.0);
    shell.dock_drop_tab_bars.clear();
    shell.dock_drop_bodies.clear();
    *shell.dock_tabs.tabs_mut(PanelAnchor::TopLeft) = vec![DockTabNode::leaf(surface, "Windows", "panels-top-left", 0)];
    shell.anchor_state_mut(PanelAnchor::TopLeft).visible = true;
    shell.anchor_state_mut(PanelAnchor::TopLeft).path = vec![surface.to_string()];
    shell.active_window_id = Some("previous-app-window".into());
    let mut document = shell.publish_surface_records(surface, records).expect("the projected Display document acquires a retained lease");
    let input = paint_tree_pointer_document(&mut shell, surface, &document, body);
    let transfer_control_id = format!("tree.drag.transfer.{projected_item_key}");
    let handle = input.hits().iter().find(|hit| hit.control_id.as_deref() == Some(transfer_control_id.as_str())).expect("the real Display producer paints a transfer handle");
    let source = (handle.rect.x + handle.rect.w * 0.5, handle.rect.y + handle.rect.h * 0.5);
    DisplayTransferHostFixture { shell, document, input, theme, source }
}

fn perform_display_transfer(fixture: &mut DisplayTransferHostFixture, drop: (f32, f32)) -> Result<(), String> {
    let (source_x, source_y) = fixture.source;
    semio_framework_async::block_on(fixture.shell.handle_pointer_button(source_x, source_y, true, 0, &mut fixture.input, &fixture.theme))?;
    fixture.shell.handle_pointer_move(drop.0, drop.1, true, &mut fixture.input, &fixture.theme);
    semio_framework_async::block_on(fixture.shell.handle_pointer_button(drop.0, drop.1, false, 0, &mut fixture.input, &fixture.theme))
}

fn close_display_transfer_host_fixture(mut fixture: DisplayTransferHostFixture) {
    while !fixture.document.close_step() {}
}

#[test]
fn cancelled_display_transfer_retires_capture_without_creating_a_window() {
    let mut fixture = display_transfer_host_fixture();
    let before = fixture.shell.dock.window_instances().len();
    let (x, y) = fixture.source;
    semio_framework_async::block_on(fixture.shell.handle_pointer_button(x, y, true, 0, &mut fixture.input, &fixture.theme)).expect("Display transfer press");
    fixture.shell.handle_pointer_move(800.0, 500.0, true, &mut fixture.input, &fixture.theme);
    fixture.shell.handle_pointer_cancel(&mut fixture.input);
    assert!(crate::interpreter::retained_pointer_capture_window(ui_render::PointerId(1)).is_none());
    assert!(fixture.shell.dock_drag.is_none() && fixture.shell.pending_dock_drag.is_none());
    assert!(!fixture.input.pointer_down && !fixture.input.drag.active);
    semio_framework_async::block_on(fixture.shell.handle_pointer_button(800.0, 500.0, false, 0, &mut fixture.input, &fixture.theme)).expect("late release stays inert");
    assert_eq!(fixture.shell.dock.window_instances().len(), before);
    assert!(fixture.shell.window_topology_actions.is_empty());
    close_display_transfer_host_fixture(fixture);
}

#[test]
fn refused_window_publication_does_not_block_a_ready_display_peer() {
    let contract: Value = serde_json::from_str(include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/../../../../🧫️fixtures/🪟️window-lifecycle-template-drag/🔣️.json"))).expect("window publication fixture");
    let cohort = &contract["publicationOutcomes"]["cohort"];
    let mut fixture = display_transfer_host_fixture();
    let program = fixture.shell.plugins.iter_mut().find(|program| program.plugin_id == "space").expect("the actual Display fixture has its guest ProgramBridge");
    program.install_fixture_render(scripted_window_body_fixture);
    program.install_fixture_action(refuse_window_body_fixture_action);
    WINDOW_BODY_FIXTURE_REFUSALS.with(|remaining| remaining.set(1));
    WINDOW_BODY_FIXTURE_ATTEMPTS.with(|attempts| attempts.set(0));
    WINDOW_JOURNAL_FIXTURE_REFUSALS.with(|count| count.set(0));
    WINDOW_PUBLICATION_FIXTURE_EVENTS.with(|events| events.borrow_mut().clear());

    perform_display_transfer(&mut fixture, (800.0, 500.0)).expect("the first actual Display transfer commits");
    perform_display_transfer(&mut fixture, (750.0, 300.0)).expect("the second actual Display transfer commits");
    assert_eq!(fixture.shell.dock.window_instances().len(), 2);
    assert_eq!(fixture.shell.window_topology_actions.len(), 2);
    assert_eq!(fixture.shell.window_topology_publications.len(), 2);

    assert_eq!(semio_framework_async::block_on(fixture.shell.settle_pump_step_inner()), ShellSettleStep::Drained);
    assert!(!fixture.shell.window_ui.contains_key(cohort["refused"].as_str().unwrap()));
    assert!(fixture.shell.window_ui.contains_key(cohort["ready"].as_str().unwrap()));
    assert_eq!(fixture.shell.window_topology_actions.len(), 1, "the ready peer releases its token during the first cohort refresh");
    assert_eq!(fixture.shell.deferred_actions.len(), 1);
    let ready_journal = dsl_value_as_json(fixture.shell.deferred_actions[0].args.as_ref().expect("the ready journal carries its instance"));
    assert_eq!(ready_journal["detail"]["instanceId"], cohort["ready"]);

    assert_eq!(semio_framework_async::block_on(fixture.shell.settle_pump_step_inner()), ShellSettleStep::Drained);
    let events = WINDOW_PUBLICATION_FIXTURE_EVENTS.with(|events| events.borrow().clone());
    let ready_dispatch = events.iter().position(|event| event == "journal:main-3").expect("the ready peer dispatches");
    let refused_retry = events.iter().enumerate().filter(|(_, event)| event.as_str() == "render:main-2").nth(1).map(|(index, _)| index).expect("the refused peer retries");
    assert!(ready_dispatch < refused_retry, "the ready journal dispatches before the unrelated retry");
    assert_eq!(semio_framework_async::block_on(fixture.shell.settle_pump_step_inner()), ShellSettleStep::Drained);
    let events = WINDOW_PUBLICATION_FIXTURE_EVENTS.with(|events| events.borrow().clone());
    for instance in ["main-2", "main-3"] {
        assert_eq!(events.iter().filter(|event| event.as_str() == format!("journal:{instance}")).count(), 1, "{instance} dispatches exactly once");
        assert_eq!(contract["publicationOutcomes"]["cohort"]["afterRetryDispatches"][instance], 1);
    }
    assert!(fixture.shell.window_topology_publications.is_empty());
    assert!(fixture.shell.window_topology_actions.is_empty());
    assert!(fixture.shell.window_topology_action_tokens.is_empty());
    retire_scripted_window_publication_documents(&mut fixture.shell);
    close_display_transfer_host_fixture(fixture);
}

#[test]
fn actual_display_release_refuses_item_and_byte_credit_before_dock_mutation() {
    let contract: Value = serde_json::from_str(include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/../../../../🧫️fixtures/🪟️window-lifecycle-template-drag/🔣️.json"))).expect("window publication fixture");
    let refusals = contract["publicationOutcomes"]["atomicCreditRefusals"].as_array().expect("credit vectors");

    let mut item = display_transfer_host_fixture();
    let item_root = item.shell.dock.root.clone();
    for _ in 0..ui_wgpu::wgpu::action::ACTION_QUEUE_ITEM_CAPACITY {
        item.shell.reserve_window_topology_action(ActionDescriptor { controller_id: "fixture".into(), action: "occupied".into(), args: None }).expect("item-credit fixture admission");
    }
    let item_error = perform_display_transfer(&mut item, (800.0, 500.0)).expect_err("the physical release observes item refusal");
    assert!(item_error.contains("ItemCredits"));
    assert_eq!(item.shell.dock.root, item_root);
    assert!(item.shell.window_topology_publications.is_empty());
    assert!(crate::interpreter::retained_pointer_capture_window(ui_render::PointerId(1)).is_none(), "a refused physical release still retires capture");
    assert!(item.shell.pending_dock_drag.is_none() && item.shell.dock_drag.is_none());
    assert_eq!(refusals[0]["dockMutation"], "none");
    close_display_transfer_host_fixture(item);

    let mut bytes = display_transfer_host_fixture();
    let byte_root = bytes.shell.dock.root.clone();
    let chunk = "x".repeat(3_900);
    let mut admitted = 0usize;
    loop {
        let action = ActionDescriptor { controller_id: "fixture".into(), action: "occupied".into(), args: crate::action_args_json!({ "a": chunk.clone(), "b": chunk.clone(), "c": chunk.clone(), "d": chunk.clone() }) };
        match bytes.shell.reserve_window_topology_action(action) {
            Ok(_) => admitted += 1,
            Err(ui_wgpu::wgpu::BoundedActionFault::ByteCredits) => break,
            Err(error) => panic!("byte fixture reached the wrong admission fault: {error:?}"),
        }
    }
    assert!(admitted < ui_wgpu::wgpu::action::ACTION_QUEUE_ITEM_CAPACITY, "byte credits exhaust before item credits");
    let byte_error = perform_display_transfer(&mut bytes, (800.0, 500.0)).expect_err("the physical release observes byte refusal");
    assert!(byte_error.contains("ByteCredits"));
    assert_eq!(bytes.shell.dock.root, byte_root);
    assert!(bytes.shell.window_topology_publications.is_empty());
    assert!(crate::interpreter::retained_pointer_capture_window(ui_render::PointerId(1)).is_none());
    assert!(bytes.shell.pending_dock_drag.is_none() && bytes.shell.dock_drag.is_none());
    assert_eq!(refusals[1]["dockMutation"], "none");
    close_display_transfer_host_fixture(bytes);
}

/// ⚖️ LAW: Display's real window-kind producer keeps its transfer payload through panel projection,
/// retained reconciliation and paint, so the published Shell ingress starts a NewWindow dock drag.
#[test]
fn display_window_kind_reaches_shell_as_a_transfer_handle_and_new_window_drag() {
    let surface = "display-transfer-host-boundary";
    let body = Rect::new(19.0, 37.0, 420.0, 300.0);
    let theme = Theme::default();
    let mut shell = super::panel_anchor_model_tests::host_test_shell();
    let mut node = shell.build_display_windows_ui();
    let UiNode::Stack(stack) = &mut node else { panic!("Display Windows publishes a stack") };
    let Some(UiNode::Tree(tree)) = stack.children.first_mut() else { panic!("Display Windows stack publishes a tree") };
    let section = tree.sections.iter_mut().find(|section| section.id == "framework.display.windows.main").expect("the real app's main window-kind section");
    section.default_open = Some(true);

    let records = panel_ui_records(surface, &node).expect("the real Display producer projects");
    let projected_item_key = records
        .iter()
        .find(|record| record.key.as_str().ends_with("framework.display.windows.main.kind") && matches!(&record.component, ui_contract::Component::TreeItem(_)))
        .map(|record| record.key.as_str().to_string())
        .expect("the real Display producer projects its transfer row");
    assert!(projected_item_key.ends_with("framework.display.windows.main.kind"));
    let transfer_control_id = format!("tree.drag.transfer.{projected_item_key}");
    let sort_control_id = format!("tree.drag.sort.{projected_item_key}");
    shell.dock = DockState::default();
    shell.dock_canvas_bounds = Rect::new(500.0, 0.0, 400.0, 600.0);
    shell.dock_drop_tab_bars.clear();
    shell.dock_drop_bodies.clear();
    *shell.dock_tabs.tabs_mut(PanelAnchor::TopLeft) = vec![DockTabNode::leaf(surface, "Windows", "panels-top-left", 0)];
    shell.anchor_state_mut(PanelAnchor::TopLeft).visible = true;
    shell.anchor_state_mut(PanelAnchor::TopLeft).path = vec![surface.to_string()];
    shell.active_window_id = Some("previous-app-window".into());
    let mut document = shell.publish_surface_records(surface, records).expect("the projected Display document acquires a retained lease");
    let mut input = paint_tree_pointer_document(&mut shell, surface, &document, body);
    let handle = input.hits().iter().find(|hit| hit.control_id.as_deref() == Some(transfer_control_id.as_str())).expect("the reconciled and painted window kind publishes its transfer handle");
    assert!(!input.hits().iter().any(|hit| hit.control_id.as_deref() == Some(sort_control_id.as_str())), "a payload-bearing row never degrades to a sort handle");
    let drag_data = handle.drag_data.as_ref().expect("the transfer handle owns the producer's payload");
    let payload = decode_window_template_drag(drag_data).expect("the retained MIME payload stays decodable");
    assert_eq!(payload.window_kind_id, "main");
    assert_eq!(payload.template_id, None);

    let (x, y) = (handle.rect.x + handle.rect.w * 0.5, handle.rect.y + handle.rect.h * 0.5);
    semio_framework_async::block_on(shell.handle_pointer_button(x, y, true, 0, &mut input, &theme)).expect("Shell routes the published transfer handle");
    assert_eq!(shell.active_window_id.as_deref(), Some("previous-app-window"), "a retained panel press never invents the panel as an application window");
    let (pending, _) = shell.pending_dock_drag.as_ref().expect("the host ingress arms a dock drag");
    assert_eq!(pending.kind, DockDragKind::NewWindow);
    assert_eq!(pending.window_kind_id, "main");
    assert_eq!(pending.template_id, None);

    shell.handle_pointer_move(800.0, 500.0, true, &mut input, &theme);
    assert!(shell.pending_dock_drag.is_none());
    assert!(shell.dock_drag.is_some(), "the captured retained move promotes the transfer into the dock authority");
    assert!(input.hit_at(800.0, 500.0).is_none(), "the runtime failure's release point owns no current retained hit");
    semio_framework_async::block_on(shell.handle_pointer_button(800.0, 500.0, false, 0, &mut input, &theme)).expect("a hitless release commits through retained capture");
    assert_eq!(shell.dock.window_instances(), vec![("main-2".to_string(), "main".to_string())], "the empty dock receives exactly one new window");
    assert!(shell.window_topology_refresh_owed, "the successful topology mutation owes the new roster's first Full refresh");
    assert!(matches!(shell.owed_refresh_scope, UiDirtyScope::Full));
    assert_eq!(shell.window_topology_actions.len(), 1, "the gesture receives exactly one bounded journal admission");
    assert_eq!(shell.window_topology_publications.len(), 1);
    assert!(shell.window_topology_publications[0].journal_token.is_some());
    assert!(shell.deferred_actions.is_empty(), "the guest journal cannot run before the new window body is published");
    assert!(crate::interpreter::retained_pointer_capture_window(ui_render::PointerId(1)).is_none(), "the hitless release retires retained capture");
    assert!(shell.pending_dock_drag.is_none() && shell.dock_drag.is_none());
    semio_framework_async::block_on(shell.handle_pointer_button(800.0, 500.0, false, 0, &mut input, &theme)).expect("a duplicate release is inert");
    assert_eq!(shell.dock.window_instances().len(), 1, "one gesture cannot create a second window");

    semio_framework_async::block_on(shell.handle_pointer_button(x, y, true, 0, &mut input, &theme)).expect("a second transfer can arm after capture retirement");
    shell.handle_pointer_move(1000.0, 700.0, true, &mut input, &theme);
    semio_framework_async::block_on(shell.handle_pointer_button(1000.0, 700.0, false, 0, &mut input, &theme)).expect("an outside release cancels cleanly");
    assert_eq!(shell.dock.window_instances().len(), 1, "an outside drop does not create a window");
    assert!(crate::interpreter::retained_pointer_capture_window(ui_render::PointerId(1)).is_none());
    assert!(shell.pending_dock_drag.is_none() && shell.dock_drag.is_none());

    let scene = ui_wgpu::wgpu::World3dScene::base(serde_json::json!({ "position": [4.0, 4.0, 4.0], "target": [0.0, 0.0, 0.0], "projection": "perspective" }).to_string(), "[]".into(), "[]".into(), "{}".into());
    let body_document = shell.publish_surface_records("main-2", vec![world3d_pointer_record(1, "initial-world", &scene)]).expect("the guest producer publishes the created instance's retained body");
    shell.window_ui.insert("main-2".into(), body_document);
    shell.complete_window_topology_refresh();
    assert!(!shell.window_topology_refresh_owed);
    assert!(shell.window_topology_actions.is_empty());
    assert!(shell.window_topology_publications.is_empty());
    assert_eq!(shell.deferred_actions.len(), 1, "one admitted topology journal is released after body publication");
    let journal = &shell.deferred_actions[0];
    assert_eq!(journal.action, "noteShellCommand");
    let journal_args = dsl_value_as_json(journal.args.as_ref().expect("the topology journal carries its command"));
    assert_eq!(journal_args["commandId"], "shell.windowSplit");
    assert_eq!(journal_args["detail"]["windowKindId"], "main");
    assert_eq!(journal_args["detail"]["instanceId"], "main-2");
    shell.complete_window_topology_refresh();
    assert_eq!(shell.deferred_actions.len(), 1, "completion acknowledgement cannot duplicate the journal");

    let bounds = Rect::new(0.0, 0.0, 900.0, 600.0);
    shell.screen_w = bounds.w;
    shell.screen_h = bounds.h;
    let mut draw = DrawList::default();
    draw.set_screen_height(bounds.h);
    let mut overlay_draw = DrawList::default();
    overlay_draw.set_screen_height(bounds.h);
    let mut overlay = Some(&mut overlay_draw);
    let mut atlas = FontAtlas::builtin();
    let icons = IconAtlas::default();
    let mut body_input = InputState::<ActionDescriptor>::default();
    let mut cursor = ShellChromeChildCursor::default();
    let mut world_resources = infinite_world::world::World3dBuildContext::new(infinite_world::world::WorldCursorWakeAuthority::new());
    crate::interpreter::begin_accessibility_visible_documents();
    let painted = (0..SHELL_WINDOW_PAINT_OPPORTUNITIES.min(1 << 20)).any(|_| shell.render_main_window_step(&mut cursor, &mut draw, &mut overlay, &mut atlas, &icons, &mut body_input, &theme, bounds, &mut world_resources));
    assert!(painted, "the canonical Shell window walk consumes the created instance's initial body");
    assert!(body_input.staged_hits().iter().any(|hit| hit.kind == HitKind::World3d), "the canonical window walk stages the new instance's physical World3d hit target");
    shell.publish_retained_hit_registry(&mut body_input);
    shell.sync_engine_surface_states();
    let world_host = shell.world3d_host_id_for_window("main-2").expect("the accepted body registers its World3d host").to_string();
    assert!(shell.world3d_states.contains_key(&world_host), "the published body registers a live World3d owner before any example switch");
    assert!(body_input.hits().iter().any(|hit| hit.kind == HitKind::World3d), "the new instance publishes a physical World3d hit target");
    let fixture: Value = serde_json::from_str(include_str!("../../🧫️fixtures/🛟️panel-window-reservation/🔣️.json")).unwrap();
    let world_hit = body_input.hits().iter().find(|hit| hit.kind == HitKind::World3d).unwrap();
    assert!(shell.retained_hit_window(world_hit).is_some(), "the actual retained publication registers this scene's body ownership");
    let point = (world_hit.rect.x + world_hit.rect.w * 0.5, world_hit.rect.y + world_hit.rect.h * 0.5);
    let owner = shell.pointer_owner_at(point.0, point.1, &body_input, &theme);
    assert_eq!(if owner == PointerHitOwner::Surface { "surface" } else { "chrome" }, fixture["retainedWorld"]["owner"].as_str().unwrap());
    assert_eq!(shell.wheel_reaches_scene_surface(point.0, point.1, &body_input, &theme), fixture["retainedWorld"]["wheel"].as_bool().unwrap());

    let world_token = shell.world3d_states.token(&world_host).expect("the live World3d owner has generation identity");
    assert!(shell.close_dock_window("main-2"));
    shell.plan_dock_windows(bounds, &theme, &mut atlas);
    shell.sync_engine_surface_states();
    assert!(!shell.world3d_states.contains_key(&world_host));
    assert!(shell.world3d_states.get_token(world_token).is_none(), "the closed instance's generation cannot be recovered");
    assert_eq!(shell.retired_world3d_states.len(), 1, "the closed owner enters bounded progressive retirement");
    shell.retire_documents_outside(&[], true).expect("the closed retained body enters document retirement");
    assert!(!shell.window_ui.contains_key("main-2"));
    shell.deferred_actions.clear();
    for _ in 0..SHELL_WINDOW_PAINT_OPPORTUNITIES.min(1 << 20) {
        if shell.retired_world3d_states.is_empty() {
            break;
        }
        shell.advance_world3d_retirement_step();
    }
    assert!(shell.retired_world3d_states.is_empty(), "the retired World3d owner reaches terminal empty under maintenance");
    for _ in 0..SHELL_WINDOW_PAINT_OPPORTUNITIES.min(1 << 20) {
        if !crate::interpreter::ui_document_close_pending_for("main-2") {
            break;
        }
        assert!(crate::interpreter::close_ui_document_one());
        shell = crate::os_host::finish_world_fixture_component_close(shell);
    }
    assert!(!crate::interpreter::ui_document_close_pending_for("main-2"), "the presenter maintenance lane returns the closed retained surface before its identity is reused");

    let program = shell.plugins.iter_mut().find(|program| program.plugin_id == "space").expect("the host fixture has its guest program");
    program.install_fixture_render(runnable_window_body_fixture);
    program.install_fixture_action(refuse_window_body_fixture_action);
    WINDOW_JOURNAL_FIXTURE_REFUSALS.with(|count| count.set(0));
    semio_framework_async::block_on(shell.handle_pointer_button(x, y, true, 0, &mut input, &theme)).expect("the real Display producer arms a fresh transfer");
    shell.handle_pointer_move(800.0, 500.0, true, &mut input, &theme);
    semio_framework_async::block_on(shell.handle_pointer_button(800.0, 500.0, false, 0, &mut input, &theme)).expect("the real Display producer creates the fresh instance");
    assert!(shell.window_topology_refresh_owed);
    assert!(!shell.window_ui.contains_key("main-2"), "the new topology has no body before the settle lane runs");
    let settle = semio_framework_async::block_on(shell.settle_pump_step_inner());
    assert_eq!(settle, ShellSettleStep::Drained, "the topology debt consumes one bounded settle step");
    assert!(!shell.window_topology_refresh_owed);
    assert!(shell.window_ui.contains_key("main-2"), "refresh_ui fetches the new roster's body through the runnable ProgramBridge fixture");
    assert_eq!(shell.deferred_actions.iter().filter(|action| action.action == "noteShellCommand").count(), 1, "the first body refresh releases exactly one admitted topology journal");
    assert_eq!(WINDOW_JOURNAL_FIXTURE_REFUSALS.with(|count| count.get()), 0, "the journal cannot overtake initial body publication");
    assert_eq!(semio_framework_async::block_on(shell.settle_pump_step_inner()), ShellSettleStep::Drained, "the following bounded step owns guest dispatch");
    assert_eq!(WINDOW_JOURNAL_FIXTURE_REFUSALS.with(|count| count.get()), 1, "the topology journal is refused exactly once");
    assert!(shell.deferred_actions.is_empty(), "the refused journal retains no stale action owner");

    shell.plan_dock_windows(bounds, &theme, &mut atlas);
    let mut refreshed_draw = DrawList::default();
    refreshed_draw.set_screen_height(bounds.h);
    let mut refreshed_overlay_draw = DrawList::default();
    refreshed_overlay_draw.set_screen_height(bounds.h);
    let mut refreshed_overlay = Some(&mut refreshed_overlay_draw);
    let mut refreshed_input = InputState::<ActionDescriptor>::default();
    let mut refreshed_cursor = ShellChromeChildCursor::default();
    crate::interpreter::begin_accessibility_visible_documents();
    let refreshed =
        (0..SHELL_WINDOW_PAINT_OPPORTUNITIES.min(1 << 20)).any(|_| shell.render_main_window_step(&mut refreshed_cursor, &mut refreshed_draw, &mut refreshed_overlay, &mut atlas, &icons, &mut refreshed_input, &theme, bounds, &mut world_resources));
    assert!(refreshed, "the body returned by refresh_ui enters the canonical Shell paint walk");
    assert!(refreshed_input.staged_hits().iter().any(|hit| hit.kind == HitKind::World3d));
    shell.publish_retained_hit_registry(&mut refreshed_input);
    assert!(refreshed_input.hits().iter().any(|hit| hit.kind == HitKind::World3d), "the actual refreshed body publishes a physical World3d hit before any example switch");

    assert!(shell.close_dock_window("main-2"));
    shell.plan_dock_windows(bounds, &theme, &mut atlas);
    shell.sync_engine_surface_states();
    shell.retire_documents_outside(&[], true).expect("the refreshed window body retires");
    shell.retire_documents_outside(&[], false).expect("the runnable fixture's panel bodies retire");
    let auxiliary_documents =
        std::mem::take(&mut shell.window_actions_documents).into_values().chain(std::mem::take(&mut shell.window_search_documents).into_values()).chain(std::mem::take(&mut shell.window_measures_documents).into_values()).collect::<Vec<_>>();
    for auxiliary in auxiliary_documents {
        shell.retire_one_surface_document(Some(auxiliary)).expect("the refreshed auxiliary body retires");
    }
    shell.drain_retained_document_arenas();
    for _ in 0..SHELL_WINDOW_PAINT_OPPORTUNITIES.min(1 << 20) {
        if shell.retired_world3d_states.is_empty() {
            break;
        }
        shell.advance_world3d_retirement_step();
    }
    assert!(shell.retired_world3d_states.is_empty(), "the refreshed owner also reaches terminal retirement");
    while !document.close_step() {}
}

/// ⚖️ LAW: two real `Component::Surface(Table)` documents publish into one Shell registry,
/// and the host's retained down/move/up ingress preserves the generation-owned transfer between them.
#[test]
fn two_published_table_surfaces_transfer_through_the_shell_host() {
    table_transfer_through_the_shell_host(false);
}

#[test]
fn a_published_table_transfer_survives_a_same_host_document_refresh() {
    table_transfer_through_the_shell_host(true);
}

fn table_transfer_through_the_shell_host(refresh: bool) {
    crate::scenes::cancel_scene_list_transfer();
    let fixture: Value = serde_json::from_str(include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/../../../../../../../../../🔨️modules/🖱️ui/🧫️fixtures/🔀️scene-list-transfer/🔣️.json"))).expect("shared transfer fixture");
    let source_fixture = &fixture["table"]["source"];
    let destination_fixture = &fixture["table"]["destination"];
    let source_id = "table-shell-source";
    let destination_id = "table-shell-destination";
    let columns = serde_json::json!([{ "id": "name", "label": "Name", "sortable": false }]).to_string();
    let mut source_scene = ui_wgpu::wgpu::TableScene::base(
        columns.clone(),
        serde_json::json!([{
            "id": source_fixture["rowId"],
            "name": "Asset",
            "_drag": source_fixture["payload"]
        }])
        .to_string(),
    );
    source_scene.row_drag_mime = source_fixture["mime"].as_str().map(str::to_string);
    let mut destination_scene = ui_wgpu::wgpu::TableScene::base(columns, serde_json::json!([{ "id": "destination", "name": "Destination" }]).to_string());
    destination_scene.drop_action_json = Some(destination_fixture["dropAction"].to_string());

    let mut shell = super::panel_anchor_model_tests::host_test_shell();
    let mut source_document = shell.publish_surface_records(source_id, vec![table_pointer_record(1, "source-table", &source_scene)]).expect("source Table document publishes");
    let mut destination_document = shell.publish_surface_records(destination_id, vec![table_pointer_record(1, "destination-table", &destination_scene)]).expect("destination Table document publishes");
    let source_body = Rect::new(11.0, 17.0, 360.0, 240.0);
    let destination_body = Rect::new(411.0, 17.0, 360.0, 240.0);
    let mut input = paint_component_pointer_documents(
        &mut shell,
        &[(source_id, "controller.table-a", &source_document, source_body), (destination_id, destination_fixture["dropAction"]["controllerId"].as_str().expect("destination controller"), &destination_document, destination_body)],
    );
    let source_owner = crate::interpreter::retained_scene_target_at(source_id, source_body.w * 0.5, source_body.h * 0.5).expect("source component host");
    let destination_owner = crate::interpreter::retained_scene_target_at(destination_id, destination_body.w * 0.5, destination_body.h * 0.5).expect("destination component host");
    let source_control = format!("{}.row.asset-7", source_owner.host_id);
    let destination_control = format!("{}.row.destination", destination_owner.host_id);
    let source_row = input.hits().iter().find(|hit| hit.control_id.as_deref() == Some(source_control.as_str())).expect("source row paints from the decoded Component::Surface").rect;
    let destination_row = input.hits().iter().find(|hit| hit.control_id.as_deref() == Some(destination_control.as_str())).expect("destination row paints from the decoded Component::Surface").rect;
    let theme = Theme::default();
    let handle_size = theme.control_height_small.min(source_row.h).min(source_row.w.max(0.0));
    let source_point = (source_row.x + theme.padding_standard + handle_size * 0.5, source_row.y + source_row.h * 0.5);
    let destination_point = (destination_row.x + destination_row.w * 0.5, destination_row.y + destination_row.h * 0.5);
    assert_eq!(input.hit_at(source_point.0, source_point.1).map(|hit| hit.kind), Some(HitKind::ComponentScene), "the published semantic surface owns the physical handle point");
    assert_eq!(input.hit_at(destination_point.0, destination_point.1).map(|hit| hit.kind), Some(HitKind::ComponentScene), "the published semantic surface owns the physical drop point");

    semio_framework_async::block_on(shell.handle_pointer_button(source_point.0, source_point.1, true, 0, &mut input, &theme)).expect("Shell routes Table source down");
    let mut refreshed_document = if refresh {
        let document = shell.publish_surface_records(source_id, vec![table_pointer_record(1, "source-table", &source_scene)]).expect("the source refresh publishes");
        input =
            paint_component_pointer_documents(&mut shell, &[(source_id, "controller.table-a", &document, source_body), (destination_id, destination_fixture["dropAction"]["controllerId"].as_str().unwrap(), &destination_document, destination_body)]);
        let refreshed_owner = crate::interpreter::retained_scene_target_at(source_id, source_body.w * 0.5, source_body.h * 0.5).expect("refreshed component host");
        assert!(source_owner.same_component_host(&refreshed_owner), "ordinary publication preserves the mounted Table owner");
        Some(document)
    } else {
        None
    };
    shell.handle_pointer_move(destination_point.0, destination_point.1, true, &mut input, &theme);
    semio_framework_async::block_on(shell.handle_pointer_button(destination_point.0, destination_point.1, false, 0, &mut input, &theme)).expect("Shell routes Table destination up");
    let actions = crate::collect_fixture_actions(&mut input);
    assert_eq!(actions.len(), 1, "one physical transfer emits exactly one destination action");
    assert_eq!(serde_json::to_value(&actions[0]).expect("action wire"), fixture["table"]["expectedAction"]);
    assert!(crate::interpreter::retained_pointer_capture_window(ui_render::PointerId(1)).is_none(), "release retires retained capture");
    assert!(!crate::scenes::cancel_scene_list_transfer(), "release retires the scene transfer authority");
    while !source_document.close_step() {}
    while !destination_document.close_step() {}
    if let Some(document) = refreshed_document.as_mut() {
        while !document.close_step() {}
    }
}

/// ⚖️ LAW: the physical Shell route preserves Canvas2d's full modifier chord and gives every
/// captured gesture exactly one cancelled terminal action on Escape or a hitless release.
#[test]
fn published_canvas_pointer_capture_preserves_modifiers_and_cancels_exactly_once() {
    crate::scenes::cancel_canvas_pointer_gesture(&mut InputState::default());
    let fixture = canvas_input_fixture();
    let surface = fixture["surface"]["id"].as_str().expect("surface id");
    let controller = fixture["surface"]["controllerId"].as_str().expect("controller id");
    let width = fixture["surface"]["width"].as_f64().expect("surface width") as f32;
    let height = fixture["surface"]["height"].as_f64().expect("surface height") as f32;
    let body = Rect::new(17.0, 31.0, width, height);
    let scene = ui_wgpu::wgpu::Canvas2dScene::base(0.0, 0.0, 1.0, "[]".into());
    let mut shell = super::panel_anchor_model_tests::host_test_shell();
    let mut document = shell.publish_surface_records(surface, vec![canvas_pointer_record(1, "canvas", &scene)]).expect("Canvas2d document publishes");
    let mut input = paint_component_pointer_documents(&mut shell, &[(surface, controller, &document, body)]);
    let down = &fixture["pointer"]["down"];
    let x = body.x + down["x"].as_f64().expect("pointer x") as f32;
    let y = body.y + down["y"].as_f64().expect("pointer y") as f32;
    input.modifiers = PointerModifiers { shift: true, ctrl: true, alt: true, meta: true };
    let theme = Theme::default();

    semio_framework_async::block_on(shell.handle_pointer_button(x, y, true, 0, &mut input, &theme)).expect("Shell routes Canvas2d down");
    shell.handle_keyboard(ui_wgpu::wgpu::KeyAction::Escape, &PointerModifiers::default(), &mut input);
    let actions = crate::collect_fixture_actions(&mut input);
    assert_eq!(actions.iter().map(|action| action.action.as_str()).collect::<Vec<_>>(), ["canvasPointerDown", "canvasPointerUp"]);
    let down_args = canvas_action_args(&actions[0]);
    assert_json_number_eq(&down_args["x"], &down["x"], "pointer down x");
    assert_json_number_eq(&down_args["y"], &down["y"], "pointer down y");
    for modifier in ["shift", "ctrl", "meta", "alt"] {
        assert_eq!(down_args[modifier], down["modifiers"][modifier]);
    }
    let cancelled = canvas_action_args(&actions[1]);
    assert_json_number_eq(&cancelled["x"], &fixture["pointer"]["cancel"]["x"], "cancel x");
    assert_json_number_eq(&cancelled["y"], &fixture["pointer"]["cancel"]["y"], "cancel y");
    assert_eq!(cancelled["cancelled"], true);
    for modifier in ["shift", "ctrl", "meta", "alt"] {
        assert_eq!(cancelled[modifier], false);
    }

    semio_framework_async::block_on(shell.handle_pointer_button(x, y, true, 0, &mut input, &theme)).expect("a fresh Canvas2d gesture arms");
    let outside = (body.x + body.w + 20.0, body.y + body.h + 20.0);
    semio_framework_async::block_on(shell.handle_pointer_button(outside.0, outside.1, false, 0, &mut input, &theme)).expect("captured outside release routes");
    let outside_actions = crate::collect_fixture_actions(&mut input);
    assert_eq!(outside_actions.iter().map(|action| action.action.as_str()).collect::<Vec<_>>(), ["canvasPointerDown", "canvasPointerUp"]);
    assert_eq!(canvas_action_args(&outside_actions[1])["cancelled"], true);
    assert!(crate::interpreter::retained_pointer_capture_window(ui_render::PointerId(1)).is_none());
    semio_framework_async::block_on(shell.handle_pointer_button(outside.0, outside.1, false, 0, &mut input, &theme)).expect("duplicate release stays inert");
    assert!(crate::collect_fixture_actions(&mut input).is_empty(), "a retired gesture cannot emit a second terminal action");
    semio_framework_async::block_on(shell.handle_pointer_button(x, y, true, 0, &mut input, &theme)).expect("explicit cancellation gesture arms");
    shell.handle_pointer_cancel(&mut input);
    shell.handle_pointer_cancel(&mut input);
    let actions = crate::collect_fixture_actions(&mut input);
    assert_eq!(actions.iter().map(|action| action.action.as_str()).collect::<Vec<_>>(), ["canvasPointerDown", "canvasPointerUp"]);
    assert_eq!(canvas_action_args(&actions[1])["cancelled"], true);
    assert!(crate::interpreter::retained_pointer_capture_window(ui_render::PointerId(1)).is_none());
    assert!(!input.pointer_down && !input.drag.active);
    while !document.close_step() {}
}

#[test]
fn renderer_canvas_surface_routes_the_retained_pointer_sequence() {
    renderer_canvas_pointer_sequence(false);
}

#[test]
fn renderer_foreign_pointer_cancel_preserves_the_retained_canvas_sequence() {
    renderer_canvas_pointer_sequence(true);
}

fn renderer_canvas_pointer_sequence(foreign_cancel: bool) {
    crate::scenes::cancel_canvas_pointer_gesture(&mut InputState::default());
    let fixture = canvas_input_fixture();
    let surface = fixture["surface"]["id"].as_str().unwrap();
    let controller = fixture["surface"]["controllerId"].as_str().unwrap();
    let body = Rect::new(17.0, 31.0, fixture["surface"]["width"].as_f64().unwrap() as f32, fixture["surface"]["height"].as_f64().unwrap() as f32);
    let scene = ui_wgpu::wgpu::Canvas2dScene::base(0.0, 0.0, 1.0, "[]".into());
    let mut shell = super::panel_anchor_model_tests::host_test_shell();
    let mut document = shell.publish_surface_records(surface, vec![canvas_pointer_record(1, "canvas", &scene)]).unwrap();
    let input = paint_component_pointer_documents(&mut shell, &[(surface, controller, &document, body)]);
    let mut interaction = pointer_interaction(shell, input);
    let x = body.x + fixture["pointer"]["down"]["x"].as_f64().unwrap() as f32;
    let y = body.y + fixture["pointer"]["down"]["y"].as_f64().unwrap() as f32;
    semio_framework_async::block_on(interaction.handle_pointer_button(ui_render::PointerId(77), x, y, true, 0, PointerModifiers::default()));
    let down = crate::collect_fixture_actions(&mut interaction.input);
    if foreign_cancel {
        interaction.handle_pointer_cancel(ui_render::PointerId(78));
    }
    let after_foreign_cancel = crate::collect_fixture_actions(&mut interaction.input);
    semio_framework_async::block_on(interaction.handle_pointer_button(ui_render::PointerId(77), body.x + body.w + 20.0, body.y + body.h + 20.0, false, 0, PointerModifiers::default()));
    let up = crate::collect_fixture_actions(&mut interaction.input);
    crate::scenes::cancel_canvas_pointer_gesture(&mut interaction.input);
    crate::interpreter::release_scene_pointer(ui_render::PointerId(77));
    while !document.close_step() {}
    assert_eq!(down.iter().map(|action| action.action.as_str()).collect::<Vec<_>>(), ["canvasPointerDown"], "actual renderer dispatch reaches the retained Canvas2d owner");
    assert!(after_foreign_cancel.is_empty(), "another pointer must not finish the captured Canvas gesture");
    assert_eq!(up.iter().map(|action| action.action.as_str()).collect::<Vec<_>>(), ["canvasPointerUp"]);
    assert_eq!(canvas_action_args(&up[0])["cancelled"], true);
    println!("[DEBUG] actual renderer Canvas sequence foreign_cancel={foreign_cancel} delivered one down and one terminal outside release");
}

#[test]
fn renderer_canvas_secondary_drag_reaches_the_document_gesture_before_opening_its_context_menu() {
    crate::scenes::cancel_canvas_pointer_gesture(&mut InputState::default());
    let fixture: Value = serde_json::from_str(include_str!("../../../../../../../../../🔨️modules/🖱️ui/🧫️fixtures/🧭️canvas2d-camera-gestures/🔣️.json")).unwrap();
    let row = fixture["panCases"].as_array().unwrap().iter().find(|row| row["id"] == "right-button-gesture").unwrap();
    let surface = fixture["surface"]["id"].as_str().unwrap();
    let controller = fixture["surface"]["controllerId"].as_str().unwrap();
    let body = Rect::new(17.0, 31.0, fixture["surface"]["width"].as_f64().unwrap() as f32, fixture["surface"]["height"].as_f64().unwrap() as f32);
    let initial = &fixture["wheel"]["initialCamera"];
    let layers = serde_json::json!([{ "role": "meta", "utility": row["activeUtility"] }]).to_string();
    let scene = ui_wgpu::wgpu::Canvas2dScene::base(initial["x"].as_f64().unwrap(), initial["y"].as_f64().unwrap(), initial["zoom"].as_f64().unwrap(), layers);
    let mut shell = super::panel_anchor_model_tests::host_test_shell();
    let mut document = shell.publish_surface_records(surface, vec![canvas_pointer_record(1, "canvas", &scene)]).unwrap();
    let input = paint_component_pointer_documents(&mut shell, &[(surface, controller, &document, body)]);
    let mut interaction = pointer_interaction(shell, input);
    let start = (body.x + row["start"]["x"].as_f64().unwrap() as f32, body.y + row["start"]["y"].as_f64().unwrap() as f32);
    let end = (body.x + row["end"]["x"].as_f64().unwrap() as f32, body.y + row["end"]["y"].as_f64().unwrap() as f32);
    let button = row["button"].as_i64().unwrap() as i16;
    semio_framework_async::block_on(interaction.handle_pointer_button(ui_render::PointerId(77), start.0, start.1, true, button, PointerModifiers::default()));
    semio_framework_async::block_on(interaction.handle_pointer_move(ui_render::PointerId(77), end.0, end.1, true, button, PointerModifiers::default()));
    semio_framework_async::block_on(interaction.handle_pointer_button(ui_render::PointerId(77), end.0, end.1, false, button, PointerModifiers::default()));
    let actions = crate::collect_fixture_actions(&mut interaction.input);
    let action_ids = actions.iter().map(|action| action.action.as_str()).collect::<Vec<_>>();
    let context_menu_open = interaction.shell.context_menu.is_some();
    crate::scenes::cancel_canvas_pointer_gesture(&mut interaction.input);
    crate::interpreter::release_scene_pointer(ui_render::PointerId(77));
    while !document.close_step() {}
    assert_eq!(action_ids, row["expectedActions"].as_array().unwrap().iter().map(|action| action.as_str().unwrap()).collect::<Vec<_>>(), "the actual App/Shell route preserves the neutral Canvas2d secondary document gesture");
    assert!(context_menu_open, "the document gesture preserves Canvas2d's secondary-click context menu");
}

#[test]
fn a_published_canvas_hit_cannot_retarget_a_same_key_successor() {
    let surface = "canvas-secondary-successor";
    let controller = "controller.canvas-secondary-successor";
    let body = Rect::new(17.0, 31.0, 100.0, 100.0);
    let point = (body.x + body.w * 0.5, body.y + body.h * 0.5);
    let mut shell = super::panel_anchor_model_tests::host_test_shell();
    let initial_scene = ui_wgpu::wgpu::Canvas2dScene::base(0.0, 0.0, 1.0, "[]".into());
    let mut initial = shell.publish_surface_records(surface, vec![canvas_pointer_record(1, "canvas-owner", &initial_scene)]).unwrap();
    let input = paint_component_pointer_documents(&mut shell, &[(surface, controller, &initial, body)]);
    let hit = input.hit_at(point.0, point.1).expect("the initial Canvas2d publication owns its body point").clone();
    let old_target = shell.retained_scene_hits.get(surface).expect("the published generic Canvas hit carries exact scene provenance").1.clone();
    let old_scene_hits = shell.retained_scene_hits.clone();
    let old_hit_windows = shell.retained_hit_windows.clone();

    let removed_record = tree_pointer_record(
        1,
        "canvas-owner",
        ui_contract::Component::Container(ui_contract::ContainerProps { role: Default::default(), label: None, description: None, required: None, error: None, default_open: None, drop_overlay: None }),
        &[],
        None,
    );
    let mut removed = shell.publish_surface_records(surface, vec![removed_record]).unwrap();
    let _removed_input = paint_component_pointer_documents(&mut shell, &[(surface, controller, &removed, body)]);
    assert!(crate::interpreter::retained_scene_target_at(surface, point.0 - body.x, point.1 - body.y).is_none(), "the intermediate publication retires the mounted Canvas owner");

    let successor_scene = ui_wgpu::wgpu::Canvas2dScene::base(0.0, 0.0, 2.0, "[]".into());
    let mut successor = shell.publish_surface_records(surface, vec![canvas_pointer_record(1, "canvas-owner", &successor_scene)]).unwrap();
    let _successor_input = paint_component_pointer_documents(&mut shell, &[(surface, controller, &successor, body)]);
    let successor_target = crate::interpreter::retained_scene_target_at(surface, point.0 - body.x, point.1 - body.y).expect("the successor Canvas2d scene is live");
    assert_eq!(old_target.surface_id, successor_target.surface_id);
    assert_eq!(old_target.key, successor_target.key, "the regression keeps the exact same retained key");
    assert_ne!(old_target, successor_target, "the successor still has a distinct generation-qualified identity");

    shell.retained_scene_hits = old_scene_hits;
    shell.retained_hit_windows = old_hit_windows;
    assert!(shell.retained_canvas2d_hit_window(&hit, point.0, point.1).is_none(), "an old published hit cannot inherit a same-surface same-key successor");
    while !initial.close_step() {}
    while !removed.close_step() {}
    while !successor.close_step() {}
}

#[test]
fn renderer_canvas_wheel_burst_preserves_each_zoom_in_event() {
    renderer_canvas_wheel_burst("two-zoom-in-events");
}

#[test]
fn renderer_canvas_first_wheel_uses_the_authored_camera() {
    renderer_canvas_wheel_burst("authored-camera");
}

#[test]
fn renderer_canvas_opposite_wheel_events_preserve_the_react_camera() {
    renderer_canvas_wheel_burst("zoom-in-then-out-events");
}

fn renderer_canvas_wheel_burst(case_id: &str) {
    let fixture: Value = serde_json::from_str(include_str!("../../../../../../../../../🔨️modules/🖱️ui/🧫️fixtures/🧭️canvas2d-camera-gestures/🔣️.json")).unwrap();
    let case = if case_id == "authored-camera" {
        serde_json::json!({ "deltas": [fixture["wheel"]["zoomInDeltaY"]], "expectedFactors": [fixture["wheel"]["zoomInFactor"]], "expectedActionCount": 1 })
    } else {
        fixture["wheelBursts"].as_array().unwrap().iter().find(|case| case["id"] == case_id).unwrap().clone()
    };
    let surface = format!("renderer-canvas-{case_id}");
    let controller = fixture["surface"]["controllerId"].as_str().unwrap();
    let body = Rect::new(17.0, 31.0, fixture["surface"]["width"].as_f64().unwrap() as f32, fixture["surface"]["height"].as_f64().unwrap() as f32);
    let initial = &fixture["wheel"]["initialCamera"];
    let scene = ui_wgpu::wgpu::Canvas2dScene::base(initial["x"].as_f64().unwrap(), initial["y"].as_f64().unwrap(), initial["zoom"].as_f64().unwrap(), "[]".into());
    let mut shell = super::panel_anchor_model_tests::host_test_shell();
    let mut document = shell.publish_surface_records(&surface, vec![canvas_pointer_record(1, "canvas", &scene)]).unwrap();
    let mut input = paint_component_pointer_documents(&mut shell, &[(&surface, controller, &document, body)]);
    let x = body.x + fixture["wheel"]["anchor"]["x"].as_f64().unwrap() as f32;
    let y = body.y + fixture["wheel"]["anchor"]["y"].as_f64().unwrap() as f32;
    let mut interaction = pointer_interaction(shell, input);
    for delta in case["deltas"].as_array().unwrap() {
        semio_framework_async::block_on(crate::winit_app::dispatch_normalized_event(&mut interaction, ui_render::DispatchEvent::Scroll { x, y, delta_x: 0.0, delta_y: delta.as_f64().unwrap() as f32, modifiers: ui_render::EventModifiers::default() }));
    }
    let actions = crate::scenes::sweep_expired_scene_camera_dispatches(crate::app_now_ms() + 400.0);
    let actions: Vec<_> = actions.iter().filter(|action| action.controller_id == controller && canvas_action_args(action)["surfaceId"] == surface).collect();
    let expected_zoom = case["expectedFactors"].as_array().unwrap().iter().fold(initial["zoom"].as_f64().unwrap() as f32, |zoom, factor| (zoom * factor.as_f64().unwrap() as f32).clamp(0.05, 32.0));
    while !document.close_step() {}
    assert_eq!(actions.len(), case["expectedActionCount"].as_u64().unwrap() as usize, "the burst publishes one settled camera");
    let camera = &canvas_action_args(actions[0])["camera"];
    let zoom = camera["zoom"].as_f64().unwrap() as f32;
    assert_eq!(zoom, expected_zoom, "every admitted wheel event contributes its own React factor");
    let relative_x = x - body.x - body.w * 0.5;
    let relative_y = y - body.y - body.h * 0.5;
    assert!((relative_x / zoom + camera["x"].as_f64().unwrap() as f32 - (relative_x / initial["zoom"].as_f64().unwrap() as f32 + initial["x"].as_f64().unwrap() as f32)).abs() < 0.0001);
    assert!((relative_y / zoom + camera["y"].as_f64().unwrap() as f32 - (relative_y / initial["zoom"].as_f64().unwrap() as f32 + initial["y"].as_f64().unwrap() as f32)).abs() < 0.0001);
    println!("[DEBUG] actual renderer wheel burst {case_id} published settled zoom {zoom}");
}

/// ⚖️ LAW: a real retained Tree drag reaches a real published Canvas2d leaf through Shell move/drop
/// ingress, preserves the raw catalogue MIME payload, and retires both drag authorities once.
#[test]
fn retained_catalogue_item_drags_and_drops_into_a_published_canvas() {
    let fixture = canvas_input_fixture();
    let catalogue = &fixture["catalogue"];
    let raw_payload = catalogue["rawPayload"].as_str().expect("raw catalogue payload");
    let source_id = "canvas-catalogue-source";
    let destination_id = fixture["surface"]["id"].as_str().expect("destination surface");
    let controller = fixture["surface"]["controllerId"].as_str().expect("destination controller");
    let mut shell = super::panel_anchor_model_tests::host_test_shell();
    let mut source_document = published_canvas_catalogue_document(&mut shell, source_id, raw_payload);
    let scene = ui_wgpu::wgpu::Canvas2dScene::base(0.0, 0.0, 1.0, "[]".into());
    let mut destination_document = shell.publish_surface_records(destination_id, vec![canvas_pointer_record(1, "canvas", &scene)]).expect("Canvas2d destination publishes");
    let source_body = Rect::new(11.0, 17.0, 320.0, 200.0);
    let destination_body = Rect::new(411.0, 17.0, 100.0, 100.0);
    let mut input = paint_component_pointer_documents(&mut shell, &[(source_id, "controller.catalogue", &source_document, source_body), (destination_id, controller, &destination_document, destination_body)]);
    let handle = input.hits().iter().find(|hit| hit.control_id.as_deref() == Some("tree.drag.transfer.catalogue")).expect("catalogue source publishes a transfer handle").rect;
    let point = &catalogue["point"];
    let source_point = (handle.x + handle.w * 0.5, handle.y + handle.h * 0.5);
    let destination_point = (destination_body.x + point["x"].as_f64().expect("drop x") as f32, destination_body.y + point["y"].as_f64().expect("drop y") as f32);
    let theme = Theme::default();
    semio_framework_async::block_on(shell.handle_pointer_button(source_point.0, source_point.1, true, 0, &mut input, &theme)).expect("catalogue source down routes");
    shell.handle_pointer_move(destination_point.0, destination_point.1, true, &mut input, &theme);
    semio_framework_async::block_on(shell.handle_pointer_button(destination_point.0, destination_point.1, false, 0, &mut input, &theme)).expect("Canvas2d drop routes");
    let actions = crate::collect_fixture_actions(&mut input);
    assert_eq!(actions.iter().map(|action| action.action.as_str()).collect::<Vec<_>>(), ["canvasDragOver", "canvasDragLeave", "canvasDrop"]);
    let over = canvas_action_args(&actions[0]);
    assert_json_number_eq(&over["x"], &point["x"], "drag over x");
    assert_json_number_eq(&over["y"], &point["y"], "drag over y");
    assert_json_number_eq(&over["width"], &fixture["surface"]["width"], "drag over width");
    assert_json_number_eq(&over["height"], &fixture["surface"]["height"], "drag over height");
    assert_eq!(over["types"], catalogue["types"]);
    let leave = canvas_action_args(&actions[1]);
    assert_eq!(leave["surfaceId"], destination_id);
    let drop = canvas_action_args(&actions[2]);
    assert_json_number_eq(&drop["x"], &point["x"], "drop x");
    assert_json_number_eq(&drop["y"], &point["y"], "drop y");
    assert_json_number_eq(&drop["width"], &fixture["surface"]["width"], "drop width");
    assert_json_number_eq(&drop["height"], &fixture["surface"]["height"], "drop height");
    assert_eq!(drop["dragData"], raw_payload);
    assert!(crate::interpreter::active_retained_drag_sessions().is_empty());
    assert!(crate::interpreter::retained_pointer_capture_window(ui_render::PointerId(1)).is_none());
    semio_framework_async::block_on(shell.handle_pointer_button(destination_point.0, destination_point.1, false, 0, &mut input, &theme)).expect("duplicate drop release stays inert");
    assert!(crate::collect_fixture_actions(&mut input).is_empty(), "a retired retained drag cannot drop twice");

    semio_framework_async::block_on(shell.handle_pointer_button(source_point.0, source_point.1, true, 0, &mut input, &theme)).expect("a second catalogue source down routes");
    shell.handle_pointer_move(destination_point.0, destination_point.1, true, &mut input, &theme);
    shell.handle_pointer_move(destination_body.x + destination_body.w + 40.0, destination_body.y + destination_body.h + 40.0, true, &mut input, &theme);
    semio_framework_async::block_on(shell.handle_pointer_button(destination_body.x + destination_body.w + 40.0, destination_body.y + destination_body.h + 40.0, false, 0, &mut input, &theme)).expect("outside catalogue release cancels cleanly");
    let cancelled = crate::collect_fixture_actions(&mut input);
    assert_eq!(cancelled.iter().map(|action| action.action.as_str()).collect::<Vec<_>>(), ["canvasDragOver", "canvasDragLeave"]);
    assert!(crate::interpreter::active_retained_drag_sessions().is_empty());
    assert!(crate::interpreter::retained_pointer_capture_window(ui_render::PointerId(1)).is_none());
    while !source_document.close_step() {}
    while !destination_document.close_step() {}
}

/// ⚖️ LAW: two complete primary clicks on the published Canvas2d leaf emit one React-shaped
/// `canvasDoubleClick` at the shared fixture point, after both release phases.
#[test]
fn published_canvas_double_click_emits_once_at_surface_coordinates() {
    let fixture = canvas_input_fixture();
    let surface = "canvas-double-click-surface";
    let controller = fixture["surface"]["controllerId"].as_str().expect("controller id");
    let body = Rect::new(13.0, 29.0, 100.0, 100.0);
    let point = &fixture["doubleClick"];
    let x = body.x + point["x"].as_f64().expect("double click x") as f32;
    let y = body.y + point["y"].as_f64().expect("double click y") as f32;
    let scene = ui_wgpu::wgpu::Canvas2dScene::base(0.0, 0.0, 1.0, "[]".into());
    let mut shell = super::panel_anchor_model_tests::host_test_shell();
    let mut document = shell.publish_surface_records(surface, vec![canvas_pointer_record(1, "canvas", &scene)]).expect("Canvas2d document publishes");
    let mut input = paint_component_pointer_documents(&mut shell, &[(surface, controller, &document, body)]);
    let theme = Theme::default();

    assert!(point["intervalMs"].as_f64().expect("double click interval") <= 400.0);
    for _ in 0..2 {
        semio_framework_async::block_on(shell.handle_pointer_button(x, y, true, 0, &mut input, &theme)).expect("Canvas2d click down routes");
        semio_framework_async::block_on(shell.handle_pointer_button(x, y, false, 0, &mut input, &theme)).expect("Canvas2d click up routes");
    }
    let actions = crate::collect_fixture_actions(&mut input);
    assert_eq!(actions.iter().map(|action| action.action.as_str()).collect::<Vec<_>>(), ["canvasPointerDown", "canvasPointerUp", "canvasPointerDown", "canvasPointerUp", "canvasDoubleClick"]);
    let double_click = canvas_action_args(actions.last().expect("double click action"));
    assert_json_number_eq(&double_click["x"], &point["x"], "double click x");
    assert_json_number_eq(&double_click["y"], &point["y"], "double click y");
    assert_json_number_eq(&double_click["width"], &fixture["surface"]["width"], "double click width");
    assert_json_number_eq(&double_click["height"], &fixture["surface"]["height"], "double click height");
    while !document.close_step() {}
}

//#region 🔎️QuickSearchPaletteKeyboard
/// ⌨️ `mod+p`, as the browser delivers it.
fn palette_chord() -> (ui_wgpu::wgpu::KeyAction, PointerModifiers) {
    (ui_wgpu::wgpu::KeyAction::Char("p".into()), PointerModifiers { shift: false, ctrl: true, alt: false, meta: false })
}

/// ⚖️ LAW: the shell's own overlay query fields are CHROME, not content.
///
/// The gate every hardcoded shell chord sits behind used to be a bare `focused_id.is_some()`, and
/// the chord that opens the quick-search palette focuses the palette's own query field — so opening
/// the palette disabled the chord that closes it (ticket 26/09/09/PROCEDURAL-3D-END-TO-END).
#[test]
fn the_shells_own_overlay_fields_do_not_count_as_the_user_typing() {
    assert!(!ShellState::content_is_editing(None, false), "nothing focused is not editing");
    assert!(!ShellState::content_is_editing(Some("ui.search.input"), false), "the palette's own query field is chrome");
    assert!(!ShellState::content_is_editing(Some("ui.find.input"), false), "so is the find overlay's");
    assert!(ShellState::content_is_editing(Some("generation3d.height"), false), "an app content field IS the user typing");
    assert!(ShellState::content_is_editing(None, true), "and so is the sync-attach draft buffer");
}

/// ⚖️ LAW: `mod+p` TOGGLES the quick-search palette, and the palette owns every key while it is open.
///
/// Measured on 6118 before this fix: the second `mod+p` did not close the palette, it fell through
/// to the open palette's own `Char` arm and typed a literal `p` into the query — which then made
/// every later keystroke search for `p<whatever the user meant>` and `Enter` activate nothing at all.
#[test]
fn the_palette_chord_toggles_and_never_types_itself_into_the_query() {
    let mut shell = ShellState::new(Vec::new(), String::new());
    let mut input = InputState::<ActionDescriptor>::default();
    let (action, modifiers) = palette_chord();

    shell.handle_keyboard(action.clone(), &modifiers, &mut input);
    assert_eq!(shell.overlay_state, OverlayState::Search, "the chord opens the palette");
    assert_eq!(input.focused_id.as_deref(), Some("ui.search.input"), "and focuses its query field");

    for key in ["D", "e"] {
        shell.handle_keyboard(ui_wgpu::wgpu::KeyAction::Char(key.into()), &PointerModifiers::default(), &mut input);
    }
    assert_eq!(shell.search_query, "De", "plain keys type into the palette's query");

    shell.handle_keyboard(action, &modifiers, &mut input);
    assert_eq!(shell.overlay_state, OverlayState::None, "the same chord closes it");
    assert!(!shell.search_open);
    assert_eq!(input.focused_id, None, "and hands focus back to the canvas");
    assert_eq!(shell.search_query, "De", "unactivated dismissal preserves React's owned query without appending a literal `p`");
    shell.handle_keyboard(ui_wgpu::wgpu::KeyAction::Char("x".into()), &PointerModifiers::default(), &mut input);
    assert_eq!(shell.search_query, "De", "closed palette state does not consume ordinary typing");
    shell.handle_keyboard(ui_wgpu::wgpu::KeyAction::Char("p".into()), &PointerModifiers::default(), &mut input);
    assert_eq!(shell.search_query, "De", "the chord's character never leaks into a closed retained query");
    println!("[DEBUG] wgpu-shell palette chord: open -> typed \"De\" -> closed, query preserved, focus released");
}

/// ⚖️ LAW: Escape closes the palette. It used to be claimed by the focused-input commit first, which
/// left the palette with no keyboard route out at all once the toggle was broken too.
#[test]
fn escape_closes_the_palette_rather_than_committing_its_query_field() {
    let mut shell = ShellState::new(Vec::new(), String::new());
    let mut input = InputState::<ActionDescriptor>::default();
    let (action, modifiers) = palette_chord();
    shell.handle_keyboard(action, &modifiers, &mut input);
    assert_eq!(shell.overlay_state, OverlayState::Search);

    semio_framework_async::block_on(shell.handle_keyboard_async(ui_wgpu::wgpu::KeyAction::Escape, &PointerModifiers::default(), &mut input)).expect("escape routes");
    assert_eq!(shell.overlay_state, OverlayState::None, "escape closes the topmost overlay");
    assert_eq!(input.focused_id, None);
}
//#endregion 🔎️QuickSearchPaletteKeyboard

/// 🪟️ Closing a Dock window retires its exact Canvas owner before a pending camera can settle.
#[test]
fn renderer_canvas_closed_window_retires_its_pending_camera_owner() {
    let fixture: Value = serde_json::from_str(include_str!("../../../../../../../../../🔨️modules/🖱️ui/🧫️fixtures/🧭️canvas2d-camera-gestures/🔣️.json")).unwrap();
    let surface = "renderer-canvas-closed-window";
    let controller = fixture["surface"]["controllerId"].as_str().unwrap();
    let body = Rect::new(17.0, 31.0, 400.0, 300.0);
    let mut shell = super::panel_anchor_model_tests::host_test_shell();
    shell.dock.root = DockNode::Stack { windows: vec![DockStackTab::new(surface)], active: surface.into() };
    shell.dock.sync_active_window(surface);
    shell.active_window_id = Some(surface.into());
    let scene = ui_wgpu::wgpu::Canvas2dScene::base(12.0, -8.0, 2.0, "[]".into());
    let document = shell.publish_surface_records(surface, vec![canvas_pointer_record(1, "canvas-owner", &scene)]).unwrap();
    let input = paint_component_pointer_documents(&mut shell, &[(surface, controller, &document, body)]);
    let owner = crate::interpreter::retained_scene_target_at(surface, 200.0, 150.0).unwrap();
    shell.window_ui.insert(surface.into(), document);
    let mut interaction = pointer_interaction(shell, input);
    semio_framework_async::block_on(crate::winit_app::dispatch_normalized_event(
        &mut interaction,
        ui_render::DispatchEvent::Scroll { x: body.x + 200.0, y: body.y + 150.0, delta_x: 0.0, delta_y: -120.0, modifiers: ui_render::EventModifiers::default() },
    ));

    assert!(interaction.shell.close_dock_window(surface));
    let actions: Vec<_> = crate::scenes::sweep_expired_scene_camera_dispatches(crate::app_now_ms() + 400.0).into_iter().filter(|action| canvas_action_args(action)["surfaceId"] == surface).collect();
    assert_eq!(actions.len(), fixture["sameKeySceneRemount"]["retiredDeadlineActions"].as_u64().unwrap() as usize, "a Dock close fences the pending camera before retained-layout reconciliation");
    interaction.shell.retire_documents_outside(&[], true).expect("the closed Dock body reaches its canonical document retirement lane");
    assert!(interaction.shell.settle_pump_pending(), "the retained close owns a runtime wake until its exact document is retired");
    for _ in 0..262_144 {
        if !crate::interpreter::ui_document_close_pending() {
            break;
        }
        assert!(crate::interpreter::close_ui_document_one(), "each admitted close opportunity advances one bounded unit");
    }
    assert!(!crate::interpreter::ui_document_close_pending(), "the bounded runtime close reaches terminal");
    let target_after_close = crate::interpreter::retained_scene_target_at(surface, 200.0, 150.0);

    let removed_record = tree_pointer_record(
        1,
        "canvas-owner",
        ui_contract::Component::Container(ui_contract::ContainerProps { role: Default::default(), label: None, description: None, required: None, error: None, default_open: None, drop_overlay: None }),
        &[],
        None,
    );
    let mut cleanup = interaction.shell.publish_surface_records(surface, vec![removed_record]).unwrap();
    let _ = paint_component_pointer_documents(&mut interaction.shell, &[(surface, controller, &cleanup, body)]);
    crate::scenes::retire_scene_identity(&owner);
    while !cleanup.close_step() {}

    assert!(target_after_close.is_none(), "the complete window close retires the retained component target");
}

#[test]
fn renderer_canvas_reopen_waits_for_the_exact_old_window_close() {
    let fixture: Value = serde_json::from_str(include_str!("../../../../../../../../../🔨️modules/🖱️ui/🧫️fixtures/🧭️canvas2d-camera-gestures/🔣️.json")).unwrap();
    let surface = "renderer-canvas-reopened-window";
    let controller = fixture["surface"]["controllerId"].as_str().unwrap();
    let body = Rect::new(17.0, 31.0, 400.0, 300.0);
    let scene = ui_wgpu::wgpu::Canvas2dScene::base(12.0, -8.0, 2.0, "[]".into());
    let mut shell = super::panel_anchor_model_tests::host_test_shell();
    shell.dock.root = DockNode::Stack { windows: vec![DockStackTab::new(surface)], active: surface.into() };
    shell.dock.sync_active_window(surface);
    let old = shell.publish_surface_records(surface, vec![canvas_pointer_record(1, "canvas-owner", &scene)]).unwrap();
    let _ = paint_component_pointer_documents(&mut shell, &[(surface, controller, &old, body)]);
    let old_owner = crate::interpreter::retained_scene_target_at(surface, 200.0, 150.0).unwrap();
    shell.window_ui.insert(surface.into(), old);
    assert!(shell.close_dock_window(surface));
    shell.retire_documents_outside(&[], true).unwrap();
    assert!(crate::interpreter::ui_document_close_pending());
    assert!(!crate::interpreter::scene_pointer_target_is_live(&old_owner), "a requested close is no longer an interactive live owner");

    shell.dock.root = DockNode::Stack { windows: vec![DockStackTab::new(surface)], active: surface.into() };
    shell.dock.sync_active_window(surface);
    let mut successor = shell.publish_surface_records(surface, vec![canvas_pointer_record(1, "canvas-owner", &scene)]).unwrap();
    let mut cursor = UiDocumentFrameCursor::default();
    let (mut draw, mut atlas, icons, mut input, theme) = (DrawList::default(), FontAtlas::builtin(), IconAtlas::default(), InputState::<ActionDescriptor>::default(), Theme::default());
    let (mut scroll, mut collapsed, mut selects) = (HashMap::new(), HashMap::new(), HashMap::new());
    let mut world3d_states = AdmittedSurfaceMap::default();
    let mut world_resources = infinite_world::world::World3dBuildContext::new(infinite_world::world::WorldCursorWakeAuthority::new());
    let mut ctx = framework_widget_context(&mut draw, None, &mut atlas, Some(&icons), &mut input, &theme, &mut scroll, &mut collapsed, &mut selects, None, body.h);
    let mut hosts = crate::scenes::SceneEngineHosts { chrome_labels: crate::scenes::SceneChromeLabels::english(), world3d_states: &mut world3d_states, world_resources: &mut world_resources, window_id: surface };
    assert!(!render_ui_document_step(&mut cursor, &successor, body, &mut ctx, surface, controller, ui_wgpu::wgpu::UiDriverDrag::Handle, &mut hosts));
    assert_eq!(cursor.phase_name(), "ingress", "the successor cannot enter the old retained generation while its close is pending");

    for _ in 0..262_144 {
        if !crate::interpreter::ui_document_close_pending() {
            break;
        }
        assert!(crate::interpreter::close_ui_document_one());
    }
    assert!(!crate::interpreter::ui_document_close_pending());
    let _ = paint_component_pointer_documents(&mut shell, &[(surface, controller, &successor, body)]);
    let successor_owner = crate::interpreter::retained_scene_target_at(surface, 200.0, 150.0).unwrap();
    assert_ne!(successor_owner, old_owner, "the reopened same-ID window receives a fresh exact generation");
    assert!(crate::interpreter::scene_pointer_target_is_live(&successor_owner));

    let removed_record = tree_pointer_record(
        1,
        "canvas-owner",
        ui_contract::Component::Container(ui_contract::ContainerProps { role: Default::default(), label: None, description: None, required: None, error: None, default_open: None, drop_overlay: None }),
        &[],
        None,
    );
    let mut cleanup = shell.publish_surface_records(surface, vec![removed_record]).unwrap();
    let _ = paint_component_pointer_documents(&mut shell, &[(surface, controller, &cleanup, body)]);
    while !successor.close_step() {}
    while !cleanup.close_step() {}
}

/// 🪟️ Sibling components retain independent cameras while sharing their document action address.
#[test]
fn sibling_canvas_components_mount_under_one_document_without_sharing_their_camera() {
    let fixture: Value = serde_json::from_str(include_str!("../../../../../../../../../🔨️modules/🖱️ui/🧫️fixtures/🧭️canvas2d-camera-gestures/🔣️.json")).unwrap();
    let law = &fixture["siblingWindowLifetime"];
    let surface = "sibling-canvas-document";
    let controller = "sibling-canvas-controller";
    let body = Rect::new(0.0, 0.0, 400.0, 300.0);
    let mut shell = super::panel_anchor_model_tests::host_test_shell();
    shell.dock.root = DockNode::Stack { windows: vec![DockStackTab::new(surface)], active: surface.into() };
    shell.dock.sync_active_window(surface);
    let scene = |index: usize| {
        let camera = &law["cameras"][index];
        ui_wgpu::wgpu::Canvas2dScene::base(camera["x"].as_f64().unwrap(), camera["y"].as_f64().unwrap(), camera["zoom"].as_f64().unwrap(), "[]".into())
    };
    let root = |children: &[u64]| {
        let mut record = tree_pointer_record(
            1,
            "root",
            ui_contract::Component::Container(ui_contract::ContainerProps { role: Default::default(), label: None, description: None, required: None, error: None, default_open: None, drop_overlay: None }),
            children,
            None,
        );
        record.layout = ui_contract::LayoutSpec::Stack(ui_contract::StackLayout { axis: ui_contract::Axis::Horizontal, grow: true, ..Default::default() });
        record
    };
    let child = |id, key: &str, scene: &ui_wgpu::wgpu::Canvas2dScene| {
        let mut record = canvas_pointer_record(id, key, scene);
        record.layout = ui_contract::LayoutSpec::Stack(ui_contract::StackLayout { grow: true, ..Default::default() });
        record
    };
    let mut document = shell.publish_surface_records(surface, vec![root(&[2, 3]), child(2, "left-canvas", &scene(0)), child(3, "right-canvas", &scene(1))]).unwrap();
    let input = paint_component_pointer_documents(&mut shell, &[(surface, controller, &document, body)]);
    let mut rects: Vec<_> = input.hits().iter().filter(|hit| hit.kind == ui_wgpu::wgpu::HitKind::ComponentScene).map(|hit| hit.rect).collect();
    assert_eq!(rects.len(), 2);
    rects.sort_by(|a, b| a.x.total_cmp(&b.x));
    assert!(rects.iter().all(|rect| rect.w > 0.0 && rect.h > 0.0));
    let targets: Vec<_> = rects.iter().map(|rect| crate::interpreter::retained_scene_target_at(surface, rect.x + rect.w * 0.5, rect.y + rect.h * 0.5).unwrap()).collect();
    assert_ne!(targets[0].host_id, targets[1].host_id);
    assert_eq!(targets[0].surface_id, targets[1].surface_id);
    let mut interaction = pointer_interaction(shell, input);
    for (index, rect) in rects.iter().enumerate() {
        semio_framework_async::block_on(crate::winit_app::dispatch_normalized_event(
            &mut interaction,
            ui_render::DispatchEvent::Scroll { x: rect.x + rect.w * 0.5, y: rect.y + rect.h * 0.5, delta_x: 0.0, delta_y: -120.0, modifiers: Default::default() },
        ));
        let actions = crate::scenes::sweep_expired_scene_camera_dispatches(crate::app_now_ms() + 400.0);
        let cameras: Vec<_> = actions.iter().filter(|action| action.controller_id == controller).map(canvas_action_args).collect();
        assert_eq!(cameras.len(), 1);
        assert_eq!(cameras[0]["surfaceId"], surface);
        assert!((cameras[0]["camera"]["zoom"].as_f64().unwrap() - law["expectedZooms"][index].as_f64().unwrap()).abs() < 0.0001);
    }
    let mut successor = interaction.shell.publish_surface_records(surface, vec![root(&[3]), child(3, "right-canvas", &scene(1))]).unwrap();
    interaction.input = paint_component_pointer_documents(&mut interaction.shell, &[(surface, controller, &successor, body)]);
    assert!(!crate::interpreter::scene_pointer_target_is_live(&targets[0]));
    let survivor = interaction.input.hits().iter().find(|hit| hit.kind == ui_wgpu::wgpu::HitKind::ComponentScene).unwrap().rect;
    semio_framework_async::block_on(crate::winit_app::dispatch_normalized_event(
        &mut interaction,
        ui_render::DispatchEvent::Scroll { x: survivor.x + survivor.w * 0.5, y: survivor.y + survivor.h * 0.5, delta_x: 0.0, delta_y: -120.0, modifiers: Default::default() },
    ));
    let actions = crate::scenes::sweep_expired_scene_camera_dispatches(crate::app_now_ms() + 400.0);
    let camera = actions.iter().find(|action| action.controller_id == controller).map(canvas_action_args).unwrap();
    assert!((camera["camera"]["zoom"].as_f64().unwrap() - law["survivingZoom"].as_f64().unwrap()).abs() < 0.0001);
    assert!(crate::interpreter::request_ui_document_close(surface));
    for _ in 0..262_144 {
        if !crate::interpreter::close_ui_document_one() {
            break;
        }
    }
    assert!(!crate::interpreter::ui_document_close_pending_for(surface));
    while !document.close_step() {}
    while !successor.close_step() {}
}

/// 🪟️ Closing each distinct window returns its retained capacity before the next window opens.
#[test]
fn renderer_canvas_sequential_window_close_reuses_retained_surface_capacity() {
    let fixture: Value = serde_json::from_str(include_str!("../../../../../../../../../🔨️modules/🖱️ui/🧫️fixtures/🧭️canvas2d-camera-gestures/🔣️.json")).unwrap();
    let law = &fixture["sequentialWindowLifetime"];
    let mount_count = law["mountCount"].as_u64().unwrap() as usize;
    assert!(mount_count > ui_wgpu::wgpu::engine::UI_LAYOUT_SURFACE_SLOTS);
    assert!(mount_count > crate::scenes::SCENE_SURFACE_CAPACITY);
    let controller = fixture["surface"]["controllerId"].as_str().unwrap();
    let body = Rect::new(0.0, 0.0, 400.0, 300.0);
    let scene = ui_wgpu::wgpu::Canvas2dScene::base(12.0, -8.0, 2.0, "[]".into());
    let mut interaction = pointer_interaction(super::panel_anchor_model_tests::host_test_shell(), InputState::default());
    for index in 0..mount_count {
        let surface = format!("sequential-canvas-window-{index}");
        interaction.shell.dock.root = DockNode::Stack { windows: vec![DockStackTab::new(&surface)], active: surface.clone() };
        interaction.shell.dock.sync_active_window(&surface);
        let document = interaction.shell.publish_surface_records(&surface, vec![canvas_pointer_record(1, "canvas-owner", &scene)]).unwrap();
        interaction.input = paint_component_pointer_documents(&mut interaction.shell, &[(&surface, controller, &document, body)]);
        assert!(crate::interpreter::retained_scene_target_at(&surface, 200.0, 150.0).is_some(), "window {index} mounts after earlier windows have closed");
        semio_framework_async::block_on(crate::winit_app::dispatch_normalized_event(&mut interaction, ui_render::DispatchEvent::Scroll { x: 200.0, y: 150.0, delta_x: 0.0, delta_y: -120.0, modifiers: Default::default() }));
        let actions = crate::scenes::sweep_expired_scene_camera_dispatches(crate::app_now_ms() + 400.0);
        let camera =
            actions.iter().find(|action| action.controller_id == controller && canvas_action_args(action)["surfaceId"] == surface).map(|action| canvas_action_args(action)["camera"].clone()).expect("each admitted mount retains a working camera");
        assert_eq!(camera["x"].as_f64(), Some(12.0));
        assert_eq!(camera["y"].as_f64(), Some(-8.0));
        assert!(camera["zoom"].as_f64().unwrap() > 2.0);
        interaction.shell.window_ui.insert(surface.clone(), document);
        assert_eq!(interaction.shell.window_ui.len(), law["maximumLiveWindows"].as_u64().unwrap() as usize);
        assert!(interaction.shell.close_dock_window(&surface));
        interaction.shell.retire_documents_outside(&[], true).unwrap();
        for _ in 0..262_144 {
            if !crate::interpreter::ui_document_close_pending() {
                break;
            }
            assert!(crate::interpreter::close_ui_document_one());
        }
        assert!(!crate::interpreter::ui_document_close_pending(), "window {index} closes within the bounded opportunity ceiling");
        assert!(crate::interpreter::retained_scene_target_at(&surface, 200.0, 150.0).is_none());
        assert_eq!(interaction.shell.window_ui.len(), law["liveWindowsAfterClose"].as_u64().unwrap() as usize);
    }
}

/// 🪪️ A retained camera survives an authored refresh and remounts with a replacement key.
#[test]
fn renderer_canvas_camera_obeys_the_mounted_component_identity() {
    let fixture: Value = serde_json::from_str(include_str!("../../../../../../../../../🔨️modules/🖱️ui/🧫️fixtures/🧭️canvas2d-camera-gestures/🔣️.json")).unwrap();
    let surface = "renderer-canvas-mount-lifetime";
    let controller = fixture["surface"]["controllerId"].as_str().unwrap();
    let body = Rect::new(17.0, 31.0, 400.0, 300.0);
    let mut interaction = pointer_interaction(super::panel_anchor_model_tests::host_test_shell(), InputState::default());
    let mut documents = Vec::new();
    let mut actual = Vec::new();
    for step in fixture["mountLifetime"]["steps"].as_array().unwrap() {
        let scene = ui_wgpu::wgpu::Canvas2dScene::base(12.0, -8.0, step["authoredZoom"].as_f64().unwrap(), "[]".into());
        let document = interaction.shell.publish_surface_records(surface, vec![canvas_pointer_record(1, step["key"].as_str().unwrap(), &scene)]).unwrap();
        interaction.input = paint_component_pointer_documents(&mut interaction.shell, &[(surface, controller, &document, body)]);
        semio_framework_async::block_on(crate::winit_app::dispatch_normalized_event(
            &mut interaction,
            ui_render::DispatchEvent::Scroll { x: body.x + body.w * 0.5, y: body.y + body.h * 0.5, delta_x: 0.0, delta_y: -120.0, modifiers: ui_render::EventModifiers::default() },
        ));
        let actions = crate::scenes::sweep_expired_scene_camera_dispatches(crate::app_now_ms() + 400.0);
        let camera = actions.iter().find(|action| action.controller_id == controller && canvas_action_args(action)["surfaceId"] == surface).map(|action| canvas_action_args(action)["camera"].clone());
        actual.push(camera);
        documents.push(document);
    }
    for document in &mut documents {
        while !document.close_step() {}
    }
    for (actual, expected) in actual.iter().zip(fixture["mountLifetime"]["steps"].as_array().unwrap()) {
        let camera = actual.as_ref().expect("each mounted wheel settles its camera");
        assert!((camera["zoom"].as_f64().unwrap() - expected["expectedZoom"].as_f64().unwrap()).abs() < 0.00001, "{actual:?}");
        assert_eq!(camera["x"].as_f64(), Some(12.0));
        assert_eq!(camera["y"].as_f64(), Some(-8.0));
    }
    println!("[DEBUG] mounted Canvas camera retained local refresh and seeded the replacement key: {actual:?}");
}

/// 🕒️ A checked-out old deadline cannot publish the replacement component's camera.
#[test]
fn renderer_canvas_retired_deadline_cannot_publish_a_successor() {
    renderer_canvas_retirement_probe(true, false, false);
    renderer_canvas_retirement_probe(true, true, false);
}

/// 🧹️ Removing a Canvas2d component retires its pending settled camera action.
#[test]
fn renderer_canvas_removal_retires_its_pending_camera() {
    renderer_canvas_retirement_probe(false, false, false);
    renderer_canvas_retirement_probe(false, true, false);
}

/// 🔁️ A removed and remounted Canvas owns its camera even when its retained key is reused.
#[test]
fn renderer_canvas_same_key_remount_retires_checked_out_camera() {
    renderer_canvas_retirement_probe(true, false, true);
    renderer_canvas_retirement_probe(true, true, true);
}

fn renderer_canvas_retirement_probe(replace: bool, close: bool, remount: bool) {
    let fixture: Value = serde_json::from_str(include_str!("../../../../../../../../../🔨️modules/🖱️ui/🧫️fixtures/🧭️canvas2d-camera-gestures/🔣️.json")).unwrap();
    let lifetime = &fixture["sameKeySceneRemount"];
    let key = if remount { lifetime["key"].as_str().unwrap() } else { "original" };
    let surface_id = format!("renderer-canvas-retirement-{replace}-{close}-{remount}");
    let surface = surface_id.as_str();
    let controller = fixture["surface"]["controllerId"].as_str().unwrap();
    let body = Rect::new(17.0, 31.0, 400.0, 300.0);
    let mut shell = super::panel_anchor_model_tests::host_test_shell();
    let scene = ui_wgpu::wgpu::Canvas2dScene::base(12.0, -8.0, 2.0, "[]".into());
    let mut original = shell.publish_surface_records(surface, vec![canvas_pointer_record(1, key, &scene)]).unwrap();
    let input = paint_component_pointer_documents(&mut shell, &[(surface, controller, &original, body)]);
    let original_owner = crate::interpreter::retained_scene_target_at(surface, 200.0, 150.0).unwrap();
    let mut interaction = pointer_interaction(shell, input);
    let scroll = || ui_render::DispatchEvent::Scroll { x: body.x + 200.0, y: body.y + 150.0, delta_x: 0.0, delta_y: -120.0, modifiers: ui_render::EventModifiers::default() };
    semio_framework_async::block_on(crate::winit_app::dispatch_normalized_event(&mut interaction, scroll()));
    let mut deadline = crate::scenes::SceneCameraDispatchCursor::begin(crate::app_now_ms() + 400.0);
    let container =
        |key| tree_pointer_record(1, key, ui_contract::Component::Container(ui_contract::ContainerProps { role: Default::default(), label: None, description: None, required: None, error: None, default_open: None, drop_overlay: None }), &[], None);
    let mut removed = if remount { Some(interaction.shell.publish_surface_records(surface, vec![container(key)]).unwrap()) } else { None };
    if let Some(removed) = removed.as_ref() {
        interaction.input = paint_component_pointer_documents(&mut interaction.shell, &[(surface, controller, removed, body)]);
        assert!(crate::interpreter::retained_scene_target_at(surface, 200.0, 150.0).is_none());
    }
    let next_scene = ui_wgpu::wgpu::Canvas2dScene::base(12.0, -8.0, if remount { lifetime["remountAuthoredZoom"].as_f64().unwrap() } else { 3.0 }, "[]".into());
    let record = if replace { canvas_pointer_record(1, if remount { key } else { "successor" }, &next_scene) } else { container("removed") };
    let mut successor = interaction.shell.publish_surface_records(surface, vec![record]).unwrap();
    interaction.input = paint_component_pointer_documents(&mut interaction.shell, &[(surface, controller, &successor, body)]);
    if replace {
        semio_framework_async::block_on(crate::winit_app::dispatch_normalized_event(&mut interaction, scroll()));
    }
    let mut actions = Vec::new();
    if close {
        while !deadline.close_step() {}
    } else {
        loop {
            match deadline.step() {
                crate::scenes::SceneCameraDispatchStep::Action(action) => {
                    if canvas_action_args(&action)["surfaceId"] == surface {
                        actions.push(action);
                    }
                }
                crate::scenes::SceneCameraDispatchStep::Pending => {}
                crate::scenes::SceneCameraDispatchStep::Complete => break,
                crate::scenes::SceneCameraDispatchStep::Fault(fault) => panic!("{fault}"),
            }
        }
    }
    assert!(deadline.terminal_is_empty());
    crate::scenes::retire_scene_identity(&original_owner);
    let settled: Vec<_> = crate::scenes::sweep_expired_scene_camera_dispatches(crate::app_now_ms() + 400.0).into_iter().filter(|action| canvas_action_args(action)["surfaceId"] == surface).collect();
    while !original.close_step() {}
    if let Some(removed) = removed.as_mut() {
        while !removed.close_step() {}
    }
    while !successor.close_step() {}
    let expected = if remount { lifetime["retiredDeadlineActions"].as_u64().unwrap() as usize } else { fixture["mountLifetime"][if replace { "retiredDeadlineActions" } else { "removedCameraActions" }].as_u64().unwrap() as usize };
    assert_eq!(actions.len(), expected, "a retired Canvas camera cannot publish after replacement/removal");
    assert_eq!(settled.len(), if remount { lifetime["remountedCameraActions"].as_u64().unwrap() as usize } else { usize::from(replace) }, "only the live successor retains its camera deadline");
    if replace {
        let camera = canvas_action_args(&settled[0])["camera"].clone();
        let expected_zoom = if remount { lifetime["expectedRemountZoom"].as_f64().unwrap() } else { fixture["mountLifetime"]["steps"][2]["expectedZoom"].as_f64().unwrap() };
        assert!((camera["zoom"].as_f64().unwrap() - expected_zoom).abs() < 0.00001);
    }
    println!("[DEBUG] Canvas retirement replace={replace} close={close} remount={remount} emitted {} stale actions and {} successor actions", actions.len(), settled.len());
}
