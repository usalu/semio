use super::*;
use semio_renderer_wgpu_fixtures::shell::paint_tree_pointer_documents;

/// 📐️ The retained tree paints the same insertion edge that its drop action uses.
#[test]
fn retained_tree_reorder_marker_follows_the_physical_pointer_and_clears() {
    let _camera_owner = semio_renderer_wgpu_scene_state::camera_fixture_lease(semio_renderer_wgpu_scene_state::camera_fixture_grant());
    let surface = "tree-reorder-marker-boundary";
    let body = Rect::new(17.0, 31.0, 360.0, 240.0);
    let theme = Theme::default();
    let mut shell = ShellState::new(Vec::new(), String::new(), semio_ui_locale::Locale::En, semio_ui_locale::Terminology::Native);
    let component = |wire: Value| serde_json::from_value(wire).expect("tree reorder component");
    let row = |id, key: &str| {
        let payload = serde_json::json!({ "documentId": "surface-owner", "stepId": key }).to_string();
        let mut record = tree_pointer_record(id, key, component(serde_json::json!({
            "type": "treeItem",
            "label": key,
            "draggable": true,
            "dragData": { (ui_contract::TREE_ROW_REORDER_MIME): payload },
        })), &[], None);
        record.bindings.try_push(ui_contract::ActionBinding {
            trigger: ui_contract::Trigger::Drop,
            action: ui_contract::ActionId::try_v1("s.test.tree", "dropRow").expect("bounded row drop action"),
            args: None,
            capability: None,
        }).expect("row drop binding");
        record
    };
    let records = vec![
        tree_pointer_record(1, "root", component(serde_json::json!({ "type": "tree" })), &[2], None),
        tree_pointer_record(2, "rows", component(serde_json::json!({ "type": "treeSection", "defaultOpen": true })), &[3, 4], None),
        row(3, "first"),
        row(4, "second"),
    ];
    let mut document = shell.windows.publish_surface_records(surface, records).expect("row reorder document publishes");
    let mut input = paint_tree_pointer_documents(&mut shell, &[(surface, &document, body)]);
    let hit = |key: &str| input.hits().iter().find(|hit| hit.control_id.as_deref() == Some(key)).expect("presented row hit").rect;
    let handle = hit("tree.drag.transfer.first");
    let target = hit("tree.label.second");
    let source = (handle.x + handle.w * 0.5, handle.y + handle.h * 0.5);
    let x = target.x + target.w * 0.5;
    let paint = |shell: &mut ShellState, input: &InputState<ActionDescriptor>| {
        let mut overlay = DrawList::default();
        PanelsViewCtx::of(shell).render_tree_drag_overlay_node(&mut overlay, input, &theme);
        overlay.layers.iter().flat_map(|layer| layer.ui_instances.iter()).map(|instance| instance.rect).collect::<Vec<_>>()
    };
    semio_framework_async::block_on(ChromeViewCtx::of(&mut shell).handle_pointer_button(source.0, source.1, true, 0, &mut input, &theme)).expect("handle down");
    for (fraction, edge) in [(0.1, Some(target.y)), (0.5, None), (0.9, Some(target.y + target.h - 2.0))] {
        PointerCtx::of(&mut shell).handle_pointer_move(x, target.y + target.h * fraction, true, &mut input, &theme);
        let markers = paint(&mut shell, &input);
        if let Some(edge) = edge {
            let [marker] = markers.as_slice() else { panic!("one positional marker, got {markers:?}") };
            assert!((marker[1] - edge).abs() < 0.01);
            assert_eq!(marker[3], 2.0);
        } else {
            assert!(markers.is_empty());
        }
    }
    PointerCtx::of(&mut shell).handle_pointer_move(body.x + body.w + 8.0, target.y, true, &mut input, &theme);
    assert!(paint(&mut shell, &input).is_empty());
    let y = target.y + target.h * 0.9;
    PointerCtx::of(&mut shell).handle_pointer_move(x, y, true, &mut input, &theme);
    semio_framework_async::block_on(ChromeViewCtx::of(&mut shell).handle_pointer_button(x, y, false, 0, &mut input, &theme)).expect("release accepts the lower edge");
    let actions = semio_renderer_wgpu_frame_actions::collect_fixture_actions(&mut input);
    assert_eq!(actions.len(), 1);
    let args = actions[0].args.as_ref().expect("row reorder args");
    assert_eq!(args.get("targetId").and_then(DslValue::as_str), Some("second"));
    assert_eq!(args.get("dropPosition").and_then(DslValue::as_str), Some("after"));
    assert!(paint(&mut shell, &input).is_empty());
    semio_framework_async::block_on(ChromeViewCtx::of(&mut shell).handle_pointer_button(source.0, source.1, true, 0, &mut input, &theme)).expect("second handle down");
    PointerCtx::of(&mut shell).handle_pointer_move(x, y, true, &mut input, &theme);
    assert_eq!(paint(&mut shell, &input).len(), 1);
    PointerCtx::of(&mut shell).handle_pointer_cancel(&mut input);
    assert!(paint(&mut shell, &input).is_empty());
    assert!(semio_renderer_wgpu_frame_actions::collect_fixture_actions(&mut input).is_empty());
    while !document.close_step() {}
}
