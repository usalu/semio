//! 🪟️ LAW: the Display palette is a targetable drag source and closed World3d owners retire.

use super::*;
use crate::dock::DockNode;
use serde_json::Value;

fn fixture() -> Value {
    serde_json::from_str(include_str!("../../../../🧫️fixtures/🪟️window-lifecycle-template-drag/🔣️.json")).expect("window lifecycle fixture")
}

fn shell() -> ShellState {
    super::panel_anchor_model_tests::host_test_shell()
}

fn find_id<'a>(value: &'a Value, id: &str) -> Option<&'a Value> {
    if value.get("id").and_then(Value::as_str) == Some(id) {
        return Some(value);
    }
    match value {
        Value::Array(items) => items.iter().find_map(|item| find_id(item, id)),
        Value::Object(fields) => fields.values().find_map(|item| find_id(item, id)),
        _ => None,
    }
}

#[test]
fn display_window_and_projection_rows_publish_reacts_drag_payload_without_click_actions() {
    let mut shell = shell();
    shell.session.as_mut().unwrap().app.window_kinds.first_mut().surface_kind = ui_wgpu::wgpu::SurfaceKind::World3d;
    let value = serde_json::to_value(shell.build_display_windows_ui()).expect("display ui json");
    let fixture = fixture();
    let mime = fixture["mime"].as_str().unwrap();
    for (id, template) in [("framework.display.windows.main.kind", None), ("framework.display.windows.main.projection.parallel", Some("parallel"))] {
        let row = find_id(&value, id).expect("display row");
        assert_eq!(row["draggable"], true);
        assert!(row.get("action").is_none());
        let payload: Value = serde_json::from_str(row["dragData"][mime].as_str().expect("template payload")).unwrap();
        assert_eq!(payload["windowKindId"], "main");
        assert_eq!(payload.get("templateId").and_then(Value::as_str), template);
    }
}

#[test]
fn encoded_layout_templates_resolve_the_same_initial_top_and_three_point_looks_as_react() {
    let top_id = r#"world-projection:{"mode":{"kind":"orthographic"},"orientation":{"type":"cardinal","view":"top"}}"#;
    let top = world_projection_initial_seed(top_id).expect("React's encoded Top template decodes");
    assert_eq!(world_projection_template(top_id).id, "orthographic");
    assert_eq!(top.family, ui_wgpu::wgpu::CameraProjection3d::Orthographic);
    assert_eq!(top.orientation, ui_wgpu::wgpu::WorldProjectionOrientation::Cardinal(ui_wgpu::wgpu::WorldCardinalView::Top));
    assert_eq!(top.direction, [0.0, 0.0, 1.0]);
    assert_eq!(top.up, [0.0, 1.0, 0.0]);

    let perspective_id = r#"world-projection:{"mode":{"kind":"threePoint","fov":50},"orientation":{"type":"free"}}"#;
    let perspective = world_projection_initial_seed(perspective_id).expect("React's encoded 3-Point template decodes");
    assert_eq!(world_projection_template(perspective_id).id, "three-point");
    assert_eq!(perspective.family, ui_wgpu::wgpu::CameraProjection3d::Perspective);
    assert_eq!(perspective.direction, [0.75, -0.75, 0.55]);
    assert_eq!(perspective.up, [0.0, 0.0, 1.0]);
    assert!(world_projection_initial_seed("world-projection:not-json").is_none(), "a malformed template never silently becomes a different camera");
}

#[test]
fn final_close_then_display_open_restores_an_active_persisted_window() {
    let mut shell = shell();
    shell.dock.root = DockNode::Stack { windows: vec![DockStackTab::new("main")], active: "main".into() };
    shell.dock.active_window_id = Some("main".into());
    shell.active_window_id = Some("main".into());
    assert!(shell.close_dock_window("main"));
    assert!(shell.dock.collect_window_ids().is_empty());
    let theme = Theme::default();
    let mut atlas = FontAtlas::builtin();
    shell.plan_dock_windows(Rect::new(0.0, 0.0, 800.0, 600.0), &theme, &mut atlas);
    assert_eq!(shell.dock_drop_bodies.len(), 1, "an empty dock remains a full drop target");
    assert!(shell.dock_window_plan.is_empty(), "an empty drop target never becomes an empty-id window");
    assert!(shell.open_display_window("main", Some("top")));
    assert_eq!(shell.active_window_id.as_deref(), Some("main-2"));
    assert_eq!(shell.dock.collect_window_ids(), vec!["main-2"]);
    assert_eq!(shell.world_projection_template.get("main-2").map(String::as_str), Some("top"));
    assert!(shell.window_topology_actions.is_empty(), "React's drag-only Display palette gives the direct host action no transfer journal");
    assert_eq!(shell.window_topology_publications.len(), 1);
    assert_eq!(shell.window_topology_publications[0].journal_token, None);
    shell.plan_dock_windows(Rect::new(0.0, 0.0, 800.0, 600.0), &theme, &mut atlas);
    assert_eq!(shell.dock_window_plan.len(), 1);
    assert!(shell.dock_drop_bodies[0].1.w > 0.0 && shell.dock_drop_bodies[0].1.h > 0.0);
    let persisted = serde_json::to_value(shell.layout_override.as_ref().expect("persisted layout")).unwrap();
    assert!(persisted.to_string().contains("\"instanceId\":\"main-2\""));
    assert!(persisted.to_string().contains("\"templateId\":\"top\""));
}

#[test]
fn template_pointer_drag_promotes_and_escape_cancels_without_mutating_the_dock() {
    let mut shell = shell();
    shell.dock.root = DockNode::Stack { windows: vec![DockStackTab::new("main")], active: "main".into() };
    shell.dock.active_window_id = Some("main".into());
    let before = shell.dock.root.clone();
    let mut input = InputState::<ActionDescriptor>::default();
    input.register_hit(HitTarget {
        rect: Rect::new(0.0, 0.0, 200.0, 30.0),
        event: None,
        control_id: Some("tree.label.framework.display.windows.main.kind".into()),
        kind: HitKind::TreeItem,
        drag_axis: None,
        drag_data: Some(window_template_drag_data("main", None)),
    });
    input.publish_hits();
    semio_framework_async::block_on(shell.handle_pointer_button(20.0, 15.0, true, 0, &mut input, &Theme::default())).unwrap();
    assert!(shell.pending_dock_drag.as_ref().is_some_and(|(payload, _)| payload.kind == DockDragKind::NewWindow));
    shell.handle_pointer_move(40.0, 15.0, true, &mut input, &Theme::default());
    assert!(shell.dock_drag.is_some());
    shell.handle_keyboard(ui_wgpu::wgpu::KeyAction::Escape, &PointerModifiers::default(), &mut input);
    assert!(shell.pending_dock_drag.is_none() && shell.dock_drag.is_none());
    assert_eq!(shell.dock.root, before);
}

#[test]
fn closed_world3d_retires_every_input_and_scene_owner_before_id_reuse() {
    let mut shell = shell();
    shell.dock_window_plan = vec![("world".into(), Rect::new(0.0, 0.0, 100.0, 100.0)), ("world-2".into(), Rect::new(100.0, 0.0, 100.0, 100.0))];
    let mut retired_token = None;
    for id in ["world", "world-2"] {
        let mut state = World3dState::new(id.into(), "controller".into());
        state.bounds = if id == "world" { Rect::new(0.0, 0.0, 100.0, 100.0) } else { Rect::new(100.0, 0.0, 100.0, 100.0) };
        let token = shell.world3d_states.try_insert(id.into(), state).unwrap_or_else(|_| panic!("world admitted"));
        shell.world3d_window_ids.insert(id.into(), id.into());
        if id == "world" {
            retired_token = Some(token);
        }
        shell.world3d_status.insert(id.into(), "status".into());
        shell.world3d_status_pill_trace.insert(id.into(), "pill".into());
        shell.active_utility_by_window.insert(id.into(), "utility".into());
        shell.action_panel_folded.insert(id.into(), false);
        shell.utility_bar_folded.insert(id.into(), false);
        shell.projection_pane_folded.insert(id.into(), false);
        shell.world_projection_template.insert(id.into(), "top".into());
        shell.search_possibles_open.insert(id.into(), true);
        shell.settle_pump.watches.insert(id.into(), ShellSettleWatch::default());
    }
    assert!(shell.scene_surface_contains(50.0, 50.0));
    shell.dock_window_plan.remove(0);
    shell.sync_engine_surface_states();
    assert!(!shell.world3d_states.contains_key("world"));
    assert_eq!(shell.retired_world3d_states.len(), 1, "scene ownership drains off the input path through bounded maintenance");
    assert!(shell.world3d_states.get_token(retired_token.unwrap()).is_none(), "late work cannot recover the retired generation");
    assert!(!shell.scene_surface_contains(50.0, 50.0));
    assert!(shell.scene_surface_contains(150.0, 50.0));
    for absent in [
        shell.world3d_status.contains_key("world"),
        shell.world3d_status_pill_trace.contains_key("world"),
        shell.active_utility_by_window.contains_key("world"),
        shell.action_panel_folded.contains_key("world"),
        shell.utility_bar_folded.contains_key("world"),
        shell.projection_pane_folded.contains_key("world"),
        shell.world_projection_template.contains_key("world"),
        shell.search_possibles_open.contains_key("world"),
        shell.settle_pump.watches.contains_key("world"),
    ] {
        assert!(!absent, "retired window state survived");
    }
    let reopened = World3dState::new("world".into(), "controller-new".into());
    let token = shell.world3d_states.try_insert("world".into(), reopened).unwrap_or_else(|_| panic!("reopened id is a fresh generation"));
    assert_ne!(token, retired_token.unwrap());
    assert!(shell.world3d_states.get_token(token).is_some());
    for _ in 0..SHELL_WINDOW_PAINT_OPPORTUNITIES.min(1 << 20) {
        if shell.retired_world3d_states.is_empty() {
            break;
        }
        shell.advance_world3d_retirement_step();
    }
    assert!(shell.retired_world3d_states.is_empty(), "the test drives the retired owner to terminal empty before teardown");
}

#[test]
fn retired_owners_apply_fixed_pressure_to_new_scene_admission() {
    let fixture = fixture();
    let capacity = fixture["retirement"]["capacity"].as_u64().unwrap() as usize;
    assert_eq!(capacity, crate::scenes::SCENE_SURFACE_CAPACITY);
    let mut states = AdmittedSurfaceMap::<usize>::default();
    assert!(states.set_external_reservations(capacity));
    assert!(states.get_or_insert_with("blocked".into(), || 1).is_none());
    assert_eq!(states.take_fault(), Some("scene surface item credits exceeded"));
    assert!(states.set_external_reservations(capacity - 1));
    assert_eq!(states.get_or_insert_with("admitted".into(), || 2).map(|value| *value), Some(2));
    assert!(states.get_or_insert_with("overflow".into(), || 3).is_none());
}
