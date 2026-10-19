//! 🪟️ LAW: the Display palette is a targetable drag source and closed World3d owners retire.

use semio_framework_os_renderer_wgpu_fixtures::shell::host_test_shell;
use super::*;
use semio_framework_os_renderer_wgpu_dock::DockNode;
use serde_json::Value;

fn fixture() -> Value {
    serde_json::from_str(include_str!("../../../../🧪️tests/🧫️fixtures/🪟️window-lifecycle-template-drag/🔣️.json")).expect("window lifecycle fixture")
}

#[test]
fn an_emptied_dock_paints_the_localized_notice_without_registering_window_hits() {
    let mut caller=semio_framework_os_renderer_wgpu_ui_engine_tests::fixture_original_turn::FixtureTurnV1::new(ui_contract::UI_WORKER_RETIREMENT_POLICY);
    let fixture: Value = serde_json::from_str(include_str!("../../../🛠️ShellHelpers/🧪️tests/🧫️fixtures/🪟️empty-dock-notice/🔣️.json")).unwrap();
    for locale in fixture["locales"].as_array().unwrap() {
        for viewport in fixture["viewports"].as_array().unwrap() {
            let mut shell = shell();
            shell.prefs.locale_id = locale["id"].as_str().unwrap().into();
            shell.dock.dock.root = DockNode::Stack { windows: vec![], active: String::new() };
            shell.dock.dock.active_window_id = None;
            shell.windows.active_window_id = None;
            let theme = Theme::default();
            let bounds = Rect::new(20.0, 40.0, viewport["width"].as_f64().unwrap() as f32, viewport["height"].as_f64().unwrap() as f32);
            let mut draw = DrawList::default();
            let mut atlas = FontAtlas::builtin();
            let icons = IconAtlas::default();
            let mut input = InputState::<ActionDescriptor>::default();
            let mut cursor = ShellChromeChildCursor::default();
            let mut overlay = None;
            let mut resources = infinite_world::world::World3dBuildContext::new(infinite_world::world::WorldCursorWakeAuthority::new());
            let mut complete = false;
            for _ in 0..1_024 {
                let before = draw.layers.iter().flat_map(|layer| &layer.ui_instances).filter(|item| item.params[2] == semio_framework_ui_draw_contract::KIND_GLYPH).count();
                complete = caller.turn(|cx|PanelsViewCtx::of(&mut shell).render_main_window_step(&mut cursor, &mut draw, &mut overlay, &mut atlas, &icons, &mut input, &theme, bounds, &mut resources, cx));
                let after = draw.layers.iter().flat_map(|layer| &layer.ui_instances).filter(|item| item.params[2] == semio_framework_ui_draw_contract::KIND_GLYPH).count();
                assert!(after.saturating_sub(before) <= 1, "each grant paints at most one notice glyph");
                if complete { break; }
            }
            assert!(complete);
            assert!(shell.dock.dock_window_plan.is_empty());
            assert!(!shell.dock.dock_drop_bodies.is_empty());
            assert!(input.staged_hits().is_empty(), "prose adds no phantom window or scroll target");
            let glyphs: Vec<_> = draw.layers.iter().flat_map(|layer| &layer.ui_instances).filter(|item| item.params[2] == semio_framework_ui_draw_contract::KIND_GLYPH).collect();
            assert!(!glyphs.is_empty(), "the empty dock paints its localized instruction");
            assert!(glyphs.iter().all(|glyph| glyph.color == [theme.text_muted.r, theme.text_muted.g, theme.text_muted.b, theme.text_muted.a]));
            let text = locale["text"].as_str().unwrap();
            let notice_bounds = bounds.inset(theme.panel_inset);
            assert_eq!(text, shell_chrome_string("display.emptyShell", shell.prefs.locale_id == "de"));
            let layout = empty_dock_notice_lines(&mut atlas, notice_bounds, &theme, shell.prefs.locale_id == "de");
            let tolerance = fixture["centerTolerance"].as_f64().unwrap() as f32;
            for (_, rect) in &layout {
                assert!((rect.x + rect.w * 0.5 - notice_bounds.x - notice_bounds.w * 0.5).abs() <= tolerance);
            }
            let first = layout.first().unwrap().1;
            let last = layout.last().unwrap().1;
            assert!(((first.y + last.y + last.h) * 0.5 - notice_bounds.y - notice_bounds.h * 0.5).abs() <= tolerance);
            let lines = atlas.wrap_lines(text, notice_bounds.w, theme.font_size_body);
            assert_eq!(lines.len() > 1, viewport["wraps"].as_bool().unwrap());
            let glyph_count: usize = lines.iter().map(|range| text[range.clone()].trim_end_matches(semio_framework_ui_text::is_wrap_space).chars().count()).sum();
            assert_eq!(glyphs.len(), glyph_count);
            caller.retire_world(&mut resources);
        }
    }
    caller.finish();
}

fn shell() -> ShellState {
    host_test_shell()
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
    shell.docs.session.as_mut().unwrap().app.window_kinds.first_mut().surface_kind = semio_framework_ui_declaration::ui::SurfaceKind::World3d;
    let value = serde_json::to_value(shell.docs.build_display_windows_ui(&shell.prefs)).expect("display ui json");
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
    assert_eq!(semio_framework_ui_scene_camera::projection_spec_family(top), semio_framework_ui_scene_camera::CameraProjection3d::Orthographic);
    let (direction, up) = semio_framework_ui_scene_camera::projection_spec_orientation_look(top);
    assert_eq!(direction.to_array(), [0.0, 0.0, 1.0]);
    assert_eq!(up.to_array(), [0.0, 1.0, 0.0]);

    let perspective_id = r#"world-projection:{"mode":{"kind":"threePoint","fov":50},"orientation":{"type":"free"}}"#;
    let perspective = world_projection_initial_seed(perspective_id).expect("React's encoded 3-Point template decodes");
    assert_eq!(world_projection_template(perspective_id).id, "three-point");
    assert_eq!(semio_framework_ui_scene_camera::projection_spec_family(perspective), semio_framework_ui_scene_camera::CameraProjection3d::Perspective);
    let (direction, up) = semio_framework_ui_scene_camera::projection_spec_orientation_look(perspective);
    assert!((direction.length() - 1.0).abs() < 1e-6);
    assert_eq!(up.to_array(), [0.0, 0.0, 1.0]);
    assert!(world_projection_initial_seed("world-projection:not-json").is_none(), "a malformed template never silently becomes a different camera");
}

#[test]
fn final_close_then_display_open_restores_an_active_persisted_window() {
    let mut shell = shell();
    shell.dock.dock.root = DockNode::Stack { windows: vec![DockStackTab::new("main")], active: "main".into() };
    shell.dock.dock.active_window_id = Some("main".into());
    shell.windows.active_window_id = Some("main".into());
    assert!(SessionCtx::of(&mut shell).close_dock_window("main"));
    assert!(shell.dock.dock.collect_window_ids().is_empty());
    let theme = Theme::default();
    let mut atlas = FontAtlas::builtin();
    ChromeViewCtx::of(&mut shell).plan_dock_windows(Rect::new(0.0, 0.0, 800.0, 600.0), &theme, &mut atlas);
    assert_eq!(shell.dock.dock_drop_bodies.len(), 1, "an empty dock remains a full drop target");
    assert!(shell.dock.dock_window_plan.is_empty(), "an empty drop target never becomes an empty-id window");
    assert!(shell.dock.open_display_window("main", Some("top"), &mut shell.settle, &shell.docs, &mut shell.windows));
    assert_eq!(shell.windows.active_window_id.as_deref(), Some("main-2"));
    assert_eq!(shell.dock.dock.collect_window_ids(), vec!["main-2"]);
    assert_eq!(shell.windows.world_projection_template.get("main-2").map(String::as_str), Some("top"));
    assert!(shell.windows.window_topology_actions.is_empty(), "React's drag-only Display palette gives the direct host action no transfer journal");
    assert_eq!(shell.windows.window_topology_publications.len(), 1);
    assert_eq!(shell.windows.window_topology_publications[0].journal_token, None);
    ChromeViewCtx::of(&mut shell).plan_dock_windows(Rect::new(0.0, 0.0, 800.0, 600.0), &theme, &mut atlas);
    assert_eq!(shell.dock.dock_window_plan.len(), 1);
    assert!(shell.dock.dock_drop_bodies[0].1.w > 0.0 && shell.dock.dock_drop_bodies[0].1.h > 0.0);
    let persisted = serde_json::to_value(shell.dock.layout_override.as_ref().expect("persisted layout")).unwrap();
    assert!(persisted.to_string().contains("\"instanceId\":\"main-2\""));
    assert!(persisted.to_string().contains("\"templateId\":\"top\""));
}

#[test]
fn template_pointer_drag_promotes_and_escape_cancels_without_mutating_the_dock() {
    let mut shell = shell();
    shell.dock.dock.root = DockNode::Stack { windows: vec![DockStackTab::new("main")], active: "main".into() };
    shell.dock.dock.active_window_id = Some("main".into());
    let before = shell.dock.dock.root.clone();
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
    semio_framework_async::block_on(ChromeViewCtx::of(&mut shell).handle_pointer_button(20.0, 15.0, true, 0, &mut input, &Theme::default())).unwrap();
    assert!(shell.dock.pending_dock_drag.as_ref().is_some_and(|(payload, _)| payload.kind == DockDragKind::NewWindow));
    PointerCtx::of(&mut shell).handle_pointer_move(40.0, 15.0, true, &mut input, &Theme::default());
    assert!(shell.dock.dock_drag.is_some());
    PointerCtx::of(&mut shell).handle_keyboard(semio_framework_ui_wgpu_input::KeyAction::Escape, &PointerModifiers::default(), &mut input);
    assert!(shell.dock.pending_dock_drag.is_none() && shell.dock.dock_drag.is_none());
    assert_eq!(shell.dock.dock.root, before);
}

#[test]
fn closed_world3d_retires_every_input_and_scene_owner_before_id_reuse() {
    let mut shell = shell();
    for id in ["world", "world-2"] {
        assert!(shell.dock.dock.split_root_with_window(id, semio_framework_os_renderer_wgpu_dock::DockSide::Right), "the committed dock holds {id}");
    }
    shell.dock.dock_window_plan = vec![("world".into(), Rect::new(0.0, 0.0, 100.0, 100.0)), ("world-2".into(), Rect::new(100.0, 0.0, 100.0, 100.0))];
    let mut retired_token = None;
    for id in ["world", "world-2"] {
        let mut state = World3dState::new(id.into(), "controller".into());
        state.session.bounds = if id == "world" { Rect::new(0.0, 0.0, 100.0, 100.0) } else { Rect::new(100.0, 0.0, 100.0, 100.0) };
        let token = shell.surfaces.world3d_states.try_insert(id.into(), state).unwrap_or_else(|_| panic!("world admitted"));
        shell.surfaces.world3d_window_ids.insert(id.into(), id.into());
        if id == "world" {
            retired_token = Some(token);
        }
        shell.surfaces.world3d_status.insert(id.into(), "status".into());
        shell.surfaces.world3d_status_pill_trace.insert(id.into(), "pill".into());
        shell.windows.active_utility_by_window.insert(id.into(), "utility".into());
        shell.windows.action_panel_folded.insert(id.into(), false);
        shell.windows.utility_bar_folded.insert(id.into(), false);
        shell.windows.projection_pane_folded.insert(id.into(), false);
        shell.windows.world_projection_template.insert(id.into(), "top".into());
        shell.commands.search_possibles_open.insert(id.into(), true);
        shell.settle.settle_pump.watches.insert(id.into(), ShellSettleWatch::default());
    }
    assert!(shell.surfaces.scene_surface_contains(50.0, 50.0));
    assert!(shell.dock.dock.close_window("world"), "the dock — the retention authority — closes the window");
    shell.dock.dock_window_plan.remove(0);
    SessionCtx::of(&mut shell).sync_engine_surface_states();
    assert!(!shell.surfaces.world3d_states.contains_key("world"));
    assert_eq!(shell.surfaces.retired_world3d_states.len(), 1, "scene ownership drains off the input path through bounded maintenance");
    assert!(shell.surfaces.world3d_states.get_token(retired_token.unwrap()).is_none(), "late work cannot recover the retired generation");
    assert!(!shell.surfaces.scene_surface_contains(50.0, 50.0));
    assert!(shell.surfaces.scene_surface_contains(150.0, 50.0));
    for absent in [
        shell.surfaces.world3d_status.contains_key("world"),
        shell.surfaces.world3d_status_pill_trace.contains_key("world"),
        shell.windows.active_utility_by_window.contains_key("world"),
        shell.windows.action_panel_folded.contains_key("world"),
        shell.windows.utility_bar_folded.contains_key("world"),
        shell.windows.projection_pane_folded.contains_key("world"),
        shell.windows.world_projection_template.contains_key("world"),
        shell.commands.search_possibles_open.contains_key("world"),
        shell.settle.settle_pump.watches.contains_key("world"),
    ] {
        assert!(!absent, "retired window state survived");
    }
    let reopened = World3dState::new("world".into(), "controller-new".into());
    let token = shell.surfaces.world3d_states.try_insert("world".into(), reopened).unwrap_or_else(|_| panic!("reopened id is a fresh generation"));
    assert_ne!(token, retired_token.unwrap());
    assert!(shell.surfaces.world3d_states.get_token(token).is_some());
    for _ in 0..SHELL_WINDOW_PAINT_OPPORTUNITIES.min(1 << 20) {
        if shell.surfaces.retired_world3d_states.is_empty() {
            break;
        }
        shell.surfaces.advance_world3d_retirement_step();
    }
    assert!(shell.surfaces.retired_world3d_states.is_empty(), "the test drives the retired owner to terminal empty before teardown");
}

#[test]
fn retired_owners_apply_fixed_pressure_to_new_scene_admission() {
    let fixture = fixture();
    let capacity = fixture["retirement"]["capacity"].as_u64().unwrap() as usize;
    assert_eq!(capacity, semio_framework_os_renderer_wgpu_contracts::SCENE_SURFACE_CAPACITY);
    let mut states = AdmittedSurfaceMap::<usize>::default();
    assert!(states.set_external_reservations(capacity));
    assert!(states.get_or_insert_with("blocked".into(), || 1).is_none());
    assert_eq!(states.take_fault(), Some("scene surface item credits exceeded"));
    assert!(states.set_external_reservations(capacity - 1));
    assert_eq!(states.get_or_insert_with("admitted".into(), || 2).map(|value| *value), Some(2));
    assert!(states.get_or_insert_with("overflow".into(), || 3).is_none());
}
