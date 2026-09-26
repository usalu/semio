use super::*;
use ui_wgpu::wgpu::{DrawList, FontAtlas, IconAtlas, InputState, TableScene};

fn table_scene(surface_id: &str, table: TableScene) -> UiComponentSceneNode {
    UiComponentSceneNode {
        host_id: surface_id.into(),
        surface_id: surface_id.into(),
        controller_id: "controller".into(),
        component_kind: SurfaceKind::Table,
        pane_id: None,
        binding_id: None,
        presence: UiPresence::default(),
        canvas_2d: None,
        world_3d: None,
        node_graph: None,
        text_editor: None,
        table: Some(table),
        paint_2d: None,
        virtual_file_system: None,
        tiled_map: None,
        board2d: None,
        icon_render: None,
        ink_canvas: None,
        graph_timeline: None,
        diff_view: None,
        event_feed: None,
        block_list: None,
        menu: None,
    }
}

/// 🧪️ Renders `node` and returns the `InputState` so tests can inspect registered hit targets.
fn render(node: &UiComponentSceneNode) -> InputState<ActionDescriptor> {
    let mut draw = DrawList::default();
    let mut atlas = FontAtlas::builtin();
    let icons = IconAtlas::default();
    let mut input = InputState::<ActionDescriptor>::default();
    let theme = Theme::default();
    let mut scroll = HashMap::new();
    let mut collapsed = HashMap::new();
    let mut selects = HashMap::new();
    {
        let mut ctx = crate::interpreter::framework_widget_context(&mut draw, None, &mut atlas, Some(&icons), &mut input, &theme, &mut scroll, &mut collapsed, &mut selects, None, 0.0);
        render_table(node, Rect::new(0.0, 0.0, 400.0, 300.0), &mut ctx, UiDriverDrag::Handle);
    }
    input
}

fn hit<'a>(input: &'a InputState<ActionDescriptor>, control_id: &str) -> &'a HitTarget<ActionDescriptor> {
    input.staged_hits().iter().find(|target| target.control_id.as_deref() == Some(control_id)).unwrap_or_else(|| panic!("no hit target registered for control_id {control_id:?}"))
}

fn columns_json(entries: &[(&str, &str, bool)]) -> String {
    json!(entries.iter().map(|(id, label, sortable)| json!({ "id": id, "label": label, "sortable": sortable })).collect::<Vec<_>>()).to_string()
}

#[test]
fn header_click_on_unsorted_sortable_column_requests_ascending() {
    let table = TableScene::base(columns_json(&[("name", "Name", true)]), "[]".to_string());
    let node = table_scene("s1", table);
    let input = render(&node);
    let target = hit(&input, "s1.header.name");
    let action = target.event.as_ref().expect("sortTable action");
    assert_eq!(action.action, "sortTable");
    assert_eq!(action.args.as_ref().and_then(|args| args.get("columnId")).and_then(semio_framework::DslValue::as_str), Some("name"));
    assert_eq!(action.args.as_ref().and_then(|args| args.get("direction")).and_then(semio_framework::DslValue::as_str), Some("asc"));
}

#[test]
fn header_click_toggles_ascending_to_descending() {
    let mut table = TableScene::base(columns_json(&[("name", "Name", true)]), "[]".to_string());
    table.sort_json = Some(json!({ "columnId": "name", "direction": "asc" }).to_string());
    let node = table_scene("s1", table);
    let input = render(&node);
    let target = hit(&input, "s1.header.name");
    let action = target.event.as_ref().expect("sortTable action");
    assert_eq!(action.args.as_ref().and_then(|args| args.get("direction")).and_then(semio_framework::DslValue::as_str), Some("desc"));
}

#[test]
fn header_click_cycles_descending_back_to_ascending() {
    let mut table = TableScene::base(columns_json(&[("name", "Name", true)]), "[]".to_string());
    table.sort_json = Some(json!({ "columnId": "name", "direction": "desc" }).to_string());
    let node = table_scene("s1", table);
    let input = render(&node);
    let target = hit(&input, "s1.header.name");
    let action = target.event.as_ref().expect("sortTable action");
    assert_eq!(action.args.as_ref().and_then(|args| args.get("direction")).and_then(semio_framework::DslValue::as_str), Some("asc"));
}

#[test]
fn sorting_a_column_does_not_reset_a_different_column_to_desc() {
    let mut table = TableScene::base(columns_json(&[("name", "Name", true), ("age", "Age", true)]), "[]".to_string());
    table.sort_json = Some(json!({ "columnId": "age", "direction": "asc" }).to_string());
    let node = table_scene("s1", table);
    let input = render(&node);
    let target = hit(&input, "s1.header.name");
    let action = target.event.as_ref().expect("sortTable action");
    // 🔀️ "name" isn't the currently-sorted column, so clicking it must start a fresh ascending sort, not toggle.
    assert_eq!(action.args.as_ref().and_then(|args| args.get("direction")).and_then(semio_framework::DslValue::as_str), Some("asc"));
}

#[test]
fn non_sortable_column_registers_no_header_hit() {
    let table = TableScene::base(columns_json(&[("name", "Name", false)]), "[]".to_string());
    let node = table_scene("s1", table);
    let input = render(&node);
    assert!(input.staged_hits().iter().all(|target| target.control_id.as_deref() != Some("s1.header.name")), "a non-sortable column must not register a header sort hit target");
}

#[test]
fn row_click_dispatches_select_row_with_full_row_payload() {
    let rows = json!([{ "id": "r1", "name": { "kind": "text", "value": "Alpha" } }]).to_string();
    let table = TableScene::base(columns_json(&[("name", "Name", false)]), rows);
    let node = table_scene("s1", table);
    let input = render(&node);
    let target = hit(&input, "s1.row.r1");
    let action = target.event.as_ref().expect("selectRow action");
    assert_eq!(action.action, "selectRow");
    assert_eq!(action.args.as_ref().and_then(|args| args.get("row")).and_then(|row| row.get("id")).and_then(semio_framework::DslValue::as_str), Some("r1"));
}

//#region TablePointerTests
/// 🧪️ Resolves a pointer point through the production input path (`table_hit`), the route a real
/// press takes now that a `ComponentScene` leaf's single retained hit target shadows every row the
/// paint stages.
fn press(node: &UiComponentSceneNode, x: f32, y: f32) -> Option<SceneListHit> {
    press_with(node, x, y, SceneModifiers::default())
}

/// 🧪️ `press` with the modifier set the pointer event carried.
fn press_with(node: &UiComponentSceneNode, x: f32, y: f32, modifiers: SceneModifiers) -> Option<SceneListHit> {
    table_hit(node, Rect::new(0.0, 0.0, 400.0, 300.0), x, y, &Theme::default(), modifiers, UiDriverDrag::Handle)
}

fn row_center_y(index: usize) -> f32 {
    let theme = Theme::default();
    theme.control_height * 1.33 + theme.control_height * (index as f32 + 0.5)
}

#[test]
fn row_press_dispatches_select_row_with_the_full_row_payload() {
    let rows = json!([{ "id": "r1", "name": { "kind": "text", "value": "Alpha" } }]).to_string();
    let node = table_scene("table-press-row", TableScene::base(columns_json(&[("name", "Name", false)]), rows));
    let hit = press(&node, 40.0, row_center_y(0)).expect("row hit");
    let action = hit.action.expect("selectRow action");
    assert_eq!(action.action, "selectRow");
    assert_eq!(action.args.as_ref().and_then(|args| args.get("row")).and_then(|row| row.get("id")).and_then(semio_framework::DslValue::as_str), Some("r1"));
}

/// 🎯️ `TableHost`'s `onRowClick` picks into the framework interaction domain when the scene declares
/// one, with `targets` a JSON STRING of `[{granularity, id}]` — never the plugin-private `selectRow`.
#[test]
fn row_press_dispatches_interaction_select_when_the_scene_declares_a_domain() {
    let rows = json!([{ "id": "r1", "name": "Alpha" }]).to_string();
    let mut table = TableScene::base(columns_json(&[("name", "Name", false)]), rows);
    table.domain_id = Some("catalogue".into());
    table.domain_granularity_id = Some("part".into());
    let node = table_scene("table-press-domain", table);
    let hit = press(&node, 40.0, row_center_y(0)).expect("row hit");
    let action = hit.action.expect("interactionSelect action");
    assert_eq!(action.action, "interactionSelect");
    let args = action.args.as_ref().expect("args");
    assert_eq!(args.get("domainId").and_then(semio_framework::DslValue::as_str), Some("catalogue"));
    assert_eq!(args.get("merge").and_then(semio_framework::DslValue::as_str), Some("replace"));
    assert_eq!(args.get("method").and_then(semio_framework::DslValue::as_str), Some("pick"));
    assert_eq!(args.get("targets").and_then(semio_framework::DslValue::as_str), Some(r#"[{"granularity":"part","id":"r1"}]"#));
}

#[test]
fn header_press_dispatches_sort_table_with_the_next_direction() {
    let mut table = TableScene::base(columns_json(&[("name", "Name", true)]), "[]".to_string());
    table.sort_json = Some(json!({ "columnId": "name", "direction": "asc" }).to_string());
    let node = table_scene("table-press-header", table);
    let hit = press(&node, 40.0, 4.0).expect("header hit");
    let action = hit.action.expect("sortTable action");
    assert_eq!(action.action, "sortTable");
    assert_eq!(action.args.as_ref().and_then(|args| args.get("direction")).and_then(semio_framework::DslValue::as_str), Some("desc"));
}

/// ➖️➕️ A stepper cell's own segments dispatch the cell action with `{ delta: ±step }` merged into
/// its existing args, and its read-only centre swallows the press exactly as React's
/// `event.stopPropagation()` does — the row's `selectRow` must not fire underneath it.
#[test]
fn stepper_cell_segments_dispatch_the_merged_delta_and_the_centre_swallows_the_row_click() {
    let cell = json!({ "kind": "stepper", "value": 3.0, "min": 0.0, "max": 10.0, "step": 2.0, "action": { "controllerId": "controller", "action": "setCount", "args": { "objectId": "o1" } } });
    let rows = json!([{ "id": "r1", "count": cell }]).to_string();
    let node = table_scene("table-press-stepper", TableScene::base(columns_json(&[("count", "Count", false)]), rows));
    let y = row_center_y(0);
    let minus = press(&node, 20.0, y).expect("minus hit").action.expect("stepper decrement");
    assert_eq!(minus.action, "setCount");
    assert_eq!(minus.args.as_ref().and_then(|args| args.get("delta")).and_then(semio_framework::DslValue::as_f64), Some(-2.0));
    assert_eq!(minus.args.as_ref().and_then(|args| args.get("objectId")).and_then(semio_framework::DslValue::as_str), Some("o1"));
    let plus = press(&node, 380.0, y).expect("plus hit").action.expect("stepper increment");
    assert_eq!(plus.args.as_ref().and_then(|args| args.get("delta")).and_then(semio_framework::DslValue::as_f64), Some(2.0));
    assert!(press(&node, 200.0, y).expect("centre hit").action.is_none(), "a stepper's read-only centre must swallow the row click");
}

fn table_key_action(key: &str) -> ui_wgpu::wgpu::KeyAction {
    match key {
        "ArrowUp" => ui_wgpu::wgpu::KeyAction::ArrowUp,
        "ArrowRight" => ui_wgpu::wgpu::KeyAction::ArrowRight,
        "ArrowDown" => ui_wgpu::wgpu::KeyAction::ArrowDown,
        "ArrowLeft" => ui_wgpu::wgpu::KeyAction::ArrowLeft,
        "PageUp" => ui_wgpu::wgpu::KeyAction::PageUp,
        "PageDown" => ui_wgpu::wgpu::KeyAction::PageDown,
        "Home" => ui_wgpu::wgpu::KeyAction::Home,
        "End" => ui_wgpu::wgpu::KeyAction::End,
        "Enter" => ui_wgpu::wgpu::KeyAction::Enter,
        other => panic!("fixture key {other}"),
    }
}

/// ⌨️ The schema-owned React oracle fixture drives the WGPU centre focus geometry and live key
/// resolver. Every action keeps the authored args, Page/Home/End clamp at current bounds, and a row
/// whose addressed cell changes kind invalidates the retained focus address.
#[test]
fn table_stepper_keyboard_matches_the_shared_react_oracle_and_revalidates_the_live_cell() {
    let fixture: Value = serde_json::from_str(include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/../../../../../../../../../🔨️modules/🖱️ui/🧫️fixtures/⌨️table-stepper-keyboard/🔣️.json"))).expect("Table stepper keyboard fixture");
    let cell = &fixture["cell"];
    let action = &fixture["action"];
    let bounds = Rect::new(0.0, 0.0, 400.0, 300.0);
    remember_scene_theme(&Theme::default());
    for keyboard_case in fixture["cases"].as_array().expect("cases") {
        let payload = json!({
            "kind": "stepper",
            "value": keyboard_case["value"],
            "min": cell["min"],
            "max": cell["max"],
            "step": cell["step"],
            "action": action,
        });
        let mut row = serde_json::Map::new();
        row.insert("id".into(), cell["rowId"].clone());
        row.insert(cell["columnId"].as_str().unwrap().into(), payload);
        let rows = Value::Array(vec![Value::Object(row)]).to_string();
        let node = table_scene("window:sourcing-pool", TableScene::base(columns_json(&[(cell["columnId"].as_str().unwrap(), cell["columnLabel"].as_str().unwrap(), false)]), rows));
        let focus = table_stepper_focus_target(&node, bounds, 200.0, row_center_y(0), UiDriverDrag::Handle).expect("centre focus");
        assert_eq!(focus.row_id, cell["rowId"].as_str().unwrap());
        assert_eq!(focus.column_id, cell["columnId"].as_str().unwrap());
        assert!(table_stepper_focus_target(&node, bounds, 20.0, row_center_y(0), UiDriverDrag::Handle).is_none(), "the decrement button is not the spinbutton focus target");
        let accessible = table_stepper_accessibility_cells(&node, bounds, UiDriverDrag::Handle);
        assert_eq!(accessible.len(), 1);
        assert_eq!(
            (accessible[0].label.as_str(), accessible[0].min, accessible[0].value, accessible[0].max),
            (cell["columnLabel"].as_str().unwrap(), cell["min"].as_f64().unwrap(), keyboard_case["value"].as_f64().unwrap(), cell["max"].as_f64().unwrap())
        );
        assert!(accessible[0].rect.contains(200.0, row_center_y(0)), "the virtual spinbutton owns the painted centre third");
        let mut input = InputState::<ActionDescriptor>::default();
        let outcome = table_stepper_apply_key(&node, &focus.row_id, &focus.column_id, &table_key_action(keyboard_case["key"].as_str().unwrap()), &mut input).expect("live stepper").expect("bounded action");
        assert_eq!(outcome == TableStepperKeyOutcome::Consumed, keyboard_case["consumed"].as_bool().unwrap(), "{}", keyboard_case["id"]);
        let actions = drain_actions(&mut input);
        match keyboard_case["expectedDelta"].as_f64() {
            Some(expected) => {
                assert_eq!(actions.len(), 1, "{}", keyboard_case["id"]);
                let args = actions[0].args.as_ref().expect("args");
                assert_eq!(args.get("objectId").and_then(semio_framework::DslValue::as_str), action["args"]["objectId"].as_str());
                assert_eq!(args.get("delta").and_then(semio_framework::DslValue::as_f64), Some(expected));
            }
            None => assert!(actions.is_empty(), "{}", keyboard_case["id"]),
        }
    }

    let mut changed_row = serde_json::Map::new();
    changed_row.insert("id".into(), cell["rowId"].clone());
    changed_row.insert(cell["columnId"].as_str().unwrap().into(), json!({ "kind": "text", "value": "retired" }));
    let changed_rows = Value::Array(vec![Value::Object(changed_row)]).to_string();
    let changed = table_scene("window:sourcing-pool", TableScene::base(columns_json(&[(cell["columnId"].as_str().unwrap(), cell["columnLabel"].as_str().unwrap(), false)]), changed_rows));
    let mut input = InputState::<ActionDescriptor>::default();
    assert!(table_stepper_apply_key(&changed, cell["rowId"].as_str().unwrap(), cell["columnId"].as_str().unwrap(), &ui_wgpu::wgpu::KeyAction::ArrowUp, &mut input).is_none());
    assert!(table_stepper_accessibility_cells(&changed, bounds, UiDriverDrag::Handle).is_empty());
    assert!(drain_actions(&mut input).is_empty());
}

/// 🔘️ `renderTableCell` draws only `placement: "row"` buttons in the row; a `"menu"` button belongs
/// to the row's context menu and must not take a press meant for the row.
#[test]
fn row_buttons_dispatch_their_own_action_and_menu_placement_buttons_are_not_row_targets() {
    let cell = json!({ "kind": "buttons", "buttons": [
        { "iconId": "trash-2", "action": { "controllerId": "controller", "action": "removeRow", "args": { "id": "r1" } } },
        { "iconId": "pencil", "placement": "menu", "action": { "controllerId": "controller", "action": "renameRow", "args": { "id": "r1" } } }
    ] });
    let rows = json!([{ "id": "r1", "actions": cell }]).to_string();
    let node = table_scene("table-press-buttons", TableScene::base(columns_json(&[("actions", "Actions", false)]), rows));
    let y = row_center_y(0);
    let action = press(&node, 40.0, y).expect("button hit").action.expect("row button action");
    assert_eq!(action.action, "removeRow");
    let far = press(&node, 380.0, y).expect("hit").action.expect("action");
    assert_eq!(far.action, "removeRow", "the single row-placement button spans the whole cell; the menu button must not claim a segment");
}

/// 🪪️ `TableHost`'s `rowIds` falls back to the row's ordinal when it carries neither `id` nor
/// `pluginId` — without it every such row collapsed onto the empty id.
#[test]
fn row_without_an_id_is_keyed_by_its_ordinal() {
    let rows = json!([{ "name": "Alpha" }, { "name": "Beta" }]).to_string();
    let node = table_scene("table-press-ordinal", TableScene::base(columns_json(&[("name", "Name", false)]), rows));
    assert_eq!(press(&node, 40.0, row_center_y(1)).expect("row hit").control_id, "table-press-ordinal.row.1");
}
//#endregion TablePointerTests

/// 🎯️ React `🖱️ui/🎯️targets/⚛️react/🟦️.tsx` `interactionMergeFromModifiers`, pinned by its own vitest
/// law "interactionMergeFromModifiers: shift wins Range even with ctrl/meta also held; alt is
/// Subtractive; no modifier is Replace". A row pick carries the merge its MODIFIERS select — before
/// `UiEvent`'s pointer variants carried `EventModifiers` the wgpu side could only ever write
/// `"replace"` (ticket 26/09/17/WGPU-RENDERER-REACT-PARITY, `📓️audit-w14-scenes-residual.md` C1).
#[test]
fn row_press_merge_mode_follows_the_pointer_modifiers() {
    let rows = json!([{ "id": "r1", "name": "Alpha" }]).to_string();
    let mut table = TableScene::base(columns_json(&[("name", "Name", false)]), rows);
    table.domain_id = Some("catalogue".into());
    table.domain_granularity_id = Some("part".into());
    let node = table_scene("table-press-merge", table);
    let merge_for = |modifiers: SceneModifiers| {
        press_with(&node, 40.0, row_center_y(0), modifiers).expect("row hit").action.expect("interactionSelect").args.as_ref().and_then(|args| args.get("merge")).and_then(semio_framework::DslValue::as_str).map(str::to_string).expect("merge")
    };
    assert_eq!(merge_for(SceneModifiers::default()), "replace");
    assert_eq!(merge_for(SceneModifiers { shift: true, ..SceneModifiers::default() }), "range");
    assert_eq!(merge_for(SceneModifiers { ctrl: true, ..SceneModifiers::default() }), "invertive");
    assert_eq!(merge_for(SceneModifiers { meta: true, ..SceneModifiers::default() }), "invertive");
    assert_eq!(merge_for(SceneModifiers { alt: true, ..SceneModifiers::default() }), "subtractive");
    assert_eq!(merge_for(SceneModifiers { shift: true, ctrl: true, ..SceneModifiers::default() }), "range", "shift wins a range pick even with ctrl also held");
}

//#region TableRowTransferTests
fn drain_actions(input: &mut InputState<ActionDescriptor>) -> Vec<ActionDescriptor> {
    let mut actions = Vec::new();
    while let Some(action) = input.take_action_step().expect("action authority live") {
        actions.push(action.into_descriptor().expect("bounded action materializes"));
    }
    actions
}

fn transfer_table(surface_id: &str, controller_id: &str, row_drag_mime: Option<&str>, drop_action: Option<Value>) -> UiComponentSceneNode {
    let rows = json!([{ "id": "asset-7", "name": "Asset", "_drag": { "artifactId": "asset-7", "revision": 3 } }]).to_string();
    let mut table = TableScene::base(columns_json(&[("name", "Name", false)]), rows);
    table.row_drag_mime = row_drag_mime.map(str::to_string);
    table.drop_action_json = drop_action.map(|value| value.to_string());
    let mut node = table_scene(surface_id, table);
    node.controller_id = controller_id.to_string();
    node
}

#[test]
fn shared_fixture_drivers_gate_table_transfer_to_handle_or_surface_geometry() {
    let fixture: Value = serde_json::from_str(include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/../../../../../../../../../🔨️modules/🖱️ui/🧫️fixtures/🔀️scene-list-transfer/🔣️.json"))).expect("shared transfer fixture");
    let source = &fixture["table"]["source"];
    let node = transfer_table(source["surfaceId"].as_str().unwrap(), "controller.table-a", source["mime"].as_str(), None);
    let bounds = Rect::new(0.0, 0.0, 400.0, 300.0);
    let theme = Theme::default();
    let metrics = table_metrics(bounds, 1, &theme);
    let row = Rect::new(metrics.body.x, metrics.body.y, metrics.body.w, metrics.row_h);
    let handle = table_transfer_handle_rect(row, &theme);
    let handle_point = (handle.x + handle.w * 0.5, handle.y + handle.h * 0.5);
    let label_point = (handle.x + handle.w + theme.gap_standard * 2.0, handle_point.1);

    let handle_start = table_transfer_start(&node, bounds, handle_point.0, handle_point.1, &theme, UiDriverDrag::Handle).expect("Handle driver arms the semantic transfer handle");
    assert!(matches!(handle_start.source, SceneListTransferSource::TableRow { ref row_id, .. } if row_id == "asset-7"));
    assert!(table_transfer_start(&node, bounds, label_point.0, label_point.1, &theme, UiDriverDrag::Handle).is_none(), "Handle labels do not arm");
    assert!(table_transfer_start(&node, bounds, label_point.0, label_point.1, &theme, UiDriverDrag::Surface).is_some(), "Surface rows arm");
}

#[test]
fn shared_fixture_table_transfer_crosses_windows_and_revalidates_mime_payload_and_generation() {
    cancel_scene_list_transfer();
    let fixture: Value = serde_json::from_str(include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/../../../../../../../../../🔨️modules/🖱️ui/🧫️fixtures/🔀️scene-list-transfer/🔣️.json"))).expect("shared transfer fixture");
    let source_fixture = &fixture["table"]["source"];
    let destination_fixture = &fixture["table"]["destination"];
    let source = transfer_table(source_fixture["surfaceId"].as_str().unwrap(), "controller.table-a", source_fixture["mime"].as_str(), None);
    let destination = transfer_table(destination_fixture["surfaceId"].as_str().unwrap(), destination_fixture["dropAction"]["controllerId"].as_str().unwrap(), None, Some(destination_fixture["dropAction"].clone()));
    let bounds = Rect::new(0.0, 0.0, 400.0, 300.0);
    let theme = Theme::default();
    remember_scene_theme(&theme);
    let metrics = table_metrics(bounds, 1, &theme);
    let handle = table_transfer_handle_rect(Rect::new(metrics.body.x, metrics.body.y, metrics.body.w, metrics.row_h), &theme);
    let (x, y) = (handle.x + handle.w * 0.5, handle.y + handle.h * 0.5);
    let mut input = InputState::<ActionDescriptor>::default();

    passive_scene_pointer_button(&source, bounds, ui_render::PointerId(1), x, y, true, 0, SceneModifiers::default(), "window-a", 7, UiDriverDrag::Handle, &mut input).expect("source down");
    assert!(!cancel_scene_list_transfer_for_pointer(ui_render::PointerId(2)), "a foreign pointer cannot retire the transfer owner");
    passive_scene_pointer_move(&source, bounds, ui_render::PointerId(1), x + theme.control_height, y, "window-a", 7, UiDriverDrag::Handle);
    passive_scene_pointer_button(&destination, bounds, ui_render::PointerId(1), 20.0, row_center_y(0), false, 0, SceneModifiers::default(), "window-b", 3, UiDriverDrag::Handle, &mut input).expect("destination up");
    let actions = drain_actions(&mut input);
    assert_eq!(actions.len(), 1);
    assert_eq!(serde_json::to_value(&actions[0]).expect("action json"), fixture["table"]["expectedAction"]);

    passive_scene_pointer_button(&source, bounds, ui_render::PointerId(1), x, y, true, 0, SceneModifiers::default(), "window-a", 8, UiDriverDrag::Handle, &mut input).expect("source down");
    passive_scene_pointer_move(&source, bounds, ui_render::PointerId(1), x + theme.control_height, y, "window-a", 9, UiDriverDrag::Handle);
    passive_scene_pointer_button(&destination, bounds, ui_render::PointerId(1), 20.0, row_center_y(0), false, 0, SceneModifiers::default(), "window-b", 3, UiDriverDrag::Handle, &mut input).expect("stale destination up");
    assert!(drain_actions(&mut input).is_empty(), "a changed source document generation retires the transfer before release");
}
//#endregion TableRowTransferTests
