use super::*;
use semio_renderer_wgpu_fixtures::shell::{paint_tree_pointer_documents_in, PointerFixtureTurns};

/// 📐️ The retained tree tints every accepting row as an available drop place and the quarter band its drop action uses as the active one.
#[test]
fn retained_tree_reorder_bands_follow_the_physical_pointer_and_clears() {
    let _arena = semio_ui_contract_values::testkit::UiArenaFixture::new();
    let _camera_owner = semio_renderer_wgpu_scene_state::camera_fixture_lease(semio_renderer_wgpu_scene_state::camera_fixture_grant());
    let surface = "tree-reorder-marker-boundary";
    let body = Rect::new(17.0, 31.0, 360.0, 240.0);
    let theme = Theme::default();
    let mut shell = ShellState::new(Vec::new(), String::new(), semio_ui_locale::Locale::En, semio_ui_locale::Terminology::Native);
    let component = |wire: Value| serde_json::from_value(wire).expect("tree reorder component");
    let row = |id, key: &str| {
        let payload = serde_json::json!({ "documentId": "surface-owner", "stepId": key }).to_string();
        tree_pointer_record(id, key, component(serde_json::json!({
            "type": "treeItem",
            "label": key,
            "draggable": true,
            "dragData": { (ui_contract::TREE_ROW_REORDER_MIME): payload },
        })), &[], None)
    };
    let mut root = tree_pointer_record(1, "root", component(serde_json::json!({ "type": "tree" })), &[2], None);
    root.bindings.try_push(ui_contract::ActionBinding {
        trigger: ui_contract::Trigger::Drop,
        action: ui_contract::ActionId::try_v1("s.test.tree", "dropRow").expect("bounded tree drop action"),
        args: None,
        capability: None,
    }).expect("tree drop binding");
    let records = vec![
        root,
        tree_pointer_record(2, "rows", component(serde_json::json!({ "type": "treeSection", "defaultOpen": true })), &[3, 4], None),
        row(3, "first"),
        row(4, "second"),
    ];
    let mut document = shell.windows.publish_surface_records(surface, records).expect("row reorder document publishes");
    let mut turns = PointerFixtureTurns::default();
    let mut input = paint_tree_pointer_documents_in(&mut turns, &mut shell, &[(surface, &document, body)]);
    let hit = |key: &str| input.hits().iter().find(|hit| hit.control_id.as_deref() == Some(key)).expect("presented row hit").rect;
    let handle = hit("tree.drag.transfer.first");
    let target = hit("tree.label.second");
    let source = (handle.x + handle.w * 0.5, handle.y + handle.h * 0.5);
    let x = target.x + target.w * 0.5;
    let paint = |shell: &mut ShellState, input: &mut InputState<ActionDescriptor>| {
        let mut overlay = DrawList::default();
        PanelsViewCtx::of(shell).render_tree_drag_overlay_node(&mut overlay, &mut FontAtlas::builtin(), &IconAtlas::default(), input, &theme);
        overlay.layers.iter().flat_map(|layer| layer.ui_instances.iter()).map(|instance| instance.rect).collect::<Vec<_>>()
    };
    semio_framework_async::block_on(ChromeViewCtx::of(&mut shell).handle_pointer_button(source.0, source.1, true, 0, &mut input, &theme)).expect("handle down");
    let quarter = target.h * 0.25;
    for (fraction, band) in [(0.1, Some(target.y)), (0.5, None), (0.9, Some(target.y + target.h - quarter))] {
        PointerCtx::of(&mut shell).handle_pointer_move(x, target.y + target.h * fraction, true, &mut input, &theme);
        let places = paint(&mut shell, &mut input);
        let place = places.first().expect("the accepting row is an available drop place");
        assert!((place[1] - target.y).abs() < 0.01 && (place[3] - target.h).abs() < 0.01, "the place is the row: {place:?}");
        if let Some(edge) = band {
            let [_, active] = places.as_slice() else { panic!("the row place and the active band, got {places:?}") };
            assert!((active[1] - edge).abs() < 0.01);
            assert!((active[3] - quarter).abs() < 0.01, "the band is a quarter of the row, not a line");
        } else {
            assert_eq!(places.len(), 1, "a row reorder has no inside band: only the available place stays");
        }
    }
    PointerCtx::of(&mut shell).handle_pointer_move(body.x + body.w + 8.0, target.y, true, &mut input, &theme);
    assert!(paint(&mut shell, &mut input).is_empty());
    let y = target.y + target.h * 0.9;
    PointerCtx::of(&mut shell).handle_pointer_move(x, y, true, &mut input, &theme);
    semio_framework_async::block_on(ChromeViewCtx::of(&mut shell).handle_pointer_button(x, y, false, 0, &mut input, &theme)).expect("release accepts the lower edge");
    let actions = semio_renderer_wgpu_frame_actions::collect_fixture_actions(&mut input);
    assert_eq!(actions.len(), 1);
    let args = actions[0].args.as_ref().expect("row reorder args");
    assert_eq!(args.get("targetId").and_then(DslValue::as_str), Some("second"));
    assert_eq!(args.get("dropPosition").and_then(DslValue::as_str), Some("after"));
    assert!(paint(&mut shell, &mut input).is_empty());
    semio_framework_async::block_on(ChromeViewCtx::of(&mut shell).handle_pointer_button(source.0, source.1, true, 0, &mut input, &theme)).expect("second handle down");
    PointerCtx::of(&mut shell).handle_pointer_move(x, y, true, &mut input, &theme);
    assert_eq!(paint(&mut shell, &mut input).len(), 2);
    PointerCtx::of(&mut shell).handle_pointer_cancel(&mut input);
    assert!(paint(&mut shell, &mut input).is_empty());
    assert!(semio_renderer_wgpu_frame_actions::collect_fixture_actions(&mut input).is_empty());
    while !document.close_step() {}
    turns.finish();
}

/// 🚀️ The shell tree drag starts at the shared threshold: every case of `places.cases.json` decides the same way here as in the React twin.
#[test]
fn the_shell_tree_drag_starts_at_the_shared_threshold() {
    let cases: serde_json::Value = serde_json::from_str(include_str!("../../../../../../../../../🔨️modules/🖱️ui/🔨️modules/🫳️pointer-drag/places.cases.json")).expect("the shared drag cases parse");
    let theme = Theme::default();
    let mut shell = ShellState::new(Vec::new(), String::new(), semio_ui_locale::Locale::En, semio_ui_locale::Terminology::Native);
    for case in cases["threshold"].as_array().expect("threshold") {
        let point = |key: &str| (case[key]["x"].as_f64().unwrap() as f32, case[key]["y"].as_f64().unwrap() as f32);
        let (from, to) = (point("from"), point("to"));
        let mut input = InputState::<ActionDescriptor>::default();
        shell.input.tree_drag = None;
        shell.input.tree_drag_origin = from;
        shell.input.pending_tree_drag = Some(("row".to_string(), std::collections::HashMap::new()));
        PointerCtx::of(&mut shell).handle_pointer_move(to.0, to.1, true, &mut input, &theme);
        assert_eq!(shell.input.tree_drag.is_some(), case["begins"].as_bool().unwrap(), "{case}");
    }
}

/// 👻️ The tree drag ghost paints its label and icon inside its own glass content layer, sized by its label.
#[test]
fn the_tree_drag_ghost_paints_its_label_and_icon_inside_its_glass_content() {
    let theme = Theme::default();
    let mut shell = ShellState::new(Vec::new(), String::new(), semio_ui_locale::Locale::En, semio_ui_locale::Terminology::Native);
    let mut input = InputState::<ActionDescriptor>::default();
    let payload = |label: &str| std::collections::HashMap::from([(semio_ui_wgpu_input::TREE_DRAG_GHOST_LABEL_KEY.to_string(), label.to_string()), (semio_ui_wgpu_input::TREE_DRAG_GHOST_ICON_KEY.to_string(), "folder".to_string())]);
    let icons = IconAtlas::from_packed(1, 1, vec![255; 4], vec![("folder".to_string(), [0.0, 0.0, 1.0, 1.0])]);
    let paint = |shell: &mut ShellState, input: &mut InputState<ActionDescriptor>| {
        let mut overlay = DrawList::default();
        PanelsViewCtx::of(shell).render_tree_drag_overlay_node(&mut overlay, &mut FontAtlas::builtin(), &icons, input, &theme);
        overlay
    };
    let mut ghost = |label: &str| {
        shell.input.tree_drag_origin = (10.0, 10.0);
        shell.input.tree_drag = None;
        shell.input.pending_tree_drag = Some(("row".to_string(), payload(label)));
        PointerCtx::of(&mut shell).handle_pointer_move(40.0, 40.0, true, &mut input, &theme);
        paint(&mut shell, &mut input)
    };
    let short = ghost("Row");
    let long = ghost("A considerably longer row label");
    for overlay in [&short, &long] {
        assert_eq!(overlay.glass_regions.len(), 1, "one glass ghost");
        let content = overlay.layers.iter().filter(|layer| layer.foreground_of == Some(0)).flat_map(|layer| layer.ui_instances.iter()).count();
        assert!(content > 1, "the label glyphs and the icon sit in the ghost's content layer, got {content}");
        assert!(overlay.layers.iter().filter(|layer| layer.foreground_of.is_none()).all(|layer| layer.ui_instances.is_empty()), "nothing of the ghost leaks outside its content layer");
    }
    let width = |overlay: &DrawList| overlay.glass_regions.iter().next().expect("ghost region").rect[2];
    assert!(width(&long) > width(&short), "the ghost grows with its label");
}
