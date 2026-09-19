
use super::*;
use ui_wgpu::wgpu::{DrawList, FontAtlas, IconAtlas, InputState, TableScene};

fn table_scene(surface_id: &str, table: TableScene) -> UiComponentSceneNode {
    UiComponentSceneNode {
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
        render_table(node, Rect::new(0.0, 0.0, 400.0, 300.0), &mut ctx);
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
    table_hit(node, Rect::new(0.0, 0.0, 400.0, 300.0), x, y, &Theme::default(), modifiers)
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
        press_with(&node, 40.0, row_center_y(0), modifiers)
            .expect("row hit")
            .action
            .expect("interactionSelect")
            .args
            .as_ref()
            .and_then(|args| args.get("merge"))
            .and_then(semio_framework::DslValue::as_str)
            .map(str::to_string)
            .expect("merge")
    };
    assert_eq!(merge_for(SceneModifiers::default()), "replace");
    assert_eq!(merge_for(SceneModifiers { shift: true, ..SceneModifiers::default() }), "range");
    assert_eq!(merge_for(SceneModifiers { ctrl: true, ..SceneModifiers::default() }), "invertive");
    assert_eq!(merge_for(SceneModifiers { meta: true, ..SceneModifiers::default() }), "invertive");
    assert_eq!(merge_for(SceneModifiers { alt: true, ..SceneModifiers::default() }), "subtractive");
    assert_eq!(merge_for(SceneModifiers { shift: true, ctrl: true, ..SceneModifiers::default() }), "range", "shift wins a range pick even with ctrl also held");
}

//#region TableRowTransferTests
/// 🫳️ React `📊️Table/🟦️.tsx` `rowDragProps`: a row is `draggable` only when the scene names a
/// `rowDragMime` AND the row itself carries a `_drag` record, and the transfer payload is that
/// record serialized.
#[test]
fn a_row_is_a_drag_source_only_with_both_a_row_drag_mime_and_a_drag_record() {
    let rows = json!([{ "id": "r1", "name": "Alpha", "_drag": { "partId": "p1" } }, { "id": "r2", "name": "Beta" }]).to_string();
    let bounds = Rect::new(0.0, 0.0, 400.0, 300.0);
    let theme = Theme::default();
    let without_mime = table_scene("table-drag-none", TableScene::base(columns_json(&[("name", "Name", false)]), rows.clone()));
    assert!(scene_transfer_drag_source(&without_mime, bounds, 40.0, row_center_y(0), &theme).is_none(), "a table with no rowDragMime declares no drag source");

    let mut table = TableScene::base(columns_json(&[("name", "Name", false)]), rows);
    table.row_drag_mime = Some("application/x-semio-part".into());
    let node = table_scene("table-drag-src", table);
    let (mime, payload) = scene_transfer_drag_source(&node, bounds, 40.0, row_center_y(0), &theme).expect("row 0 carries _drag");
    assert_eq!(mime, "application/x-semio-part");
    assert_eq!(serde_json::from_str::<Value>(&payload).expect("payload json"), json!({ "partId": "p1" }));
    assert!(scene_transfer_drag_source(&node, bounds, 40.0, row_center_y(1), &theme).is_none(), "a row without a _drag record is not draggable");
}

/// 🫴️ React `📊️Table/🟦️.tsx` `onDrop`: the first `application/x-semio-*` entry on the transfer is
/// JSON-parsed and SPREAD over `dropActionJson`'s own args (`dispatchCellAction`), so the drop action
/// keeps everything it declared and gains the dragged record's keys.
#[test]
fn a_drop_spreads_the_transfer_payload_over_the_declared_drop_action_args() {
    let mut table = TableScene::base(columns_json(&[("name", "Name", false)]), json!([{ "id": "r1", "name": "Alpha" }]).to_string());
    table.drop_action_json = Some(json!({ "controllerId": "controller", "action": "acceptDrop", "args": { "surfaceId": "table-drop", "slot": "inbox" } }).to_string());
    let node = table_scene("table-drop", table);
    let bounds = Rect::new(0.0, 0.0, 400.0, 300.0);
    let theme = Theme::default();
    let action = scene_transfer_drop_action(&node, bounds, 40.0, row_center_y(0), &theme, "application/x-semio-part", &json!({ "partId": "p1" }).to_string()).expect("drop action");
    assert_eq!(action.action, "acceptDrop");
    let args = action.args.as_ref().expect("args");
    assert_eq!(args.get("slot").and_then(semio_framework::DslValue::as_str), Some("inbox"), "the declared args survive the spread");
    assert_eq!(args.get("partId").and_then(semio_framework::DslValue::as_str), Some("p1"), "the dragged record's keys are merged in");
    assert!(
        scene_transfer_drop_action(&node, bounds, 40.0, row_center_y(0), &theme, "text/plain", &json!({ "partId": "p1" }).to_string()).is_none(),
        "React filters the transfer types to `application/x-semio-*`; anything else is not a drop"
    );
}
//#endregion TableRowTransferTests
