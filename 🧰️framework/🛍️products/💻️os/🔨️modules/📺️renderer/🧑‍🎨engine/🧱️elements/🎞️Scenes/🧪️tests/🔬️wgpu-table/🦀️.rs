
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
        let mut ctx = crate::interpreter::framework_widget_context(&mut draw, None, &mut atlas, Some(&icons), &mut input, &theme, &mut scroll, &mut collapsed, &mut selects, None);
        render_table(node, Rect::new(0.0, 0.0, 400.0, 300.0), &mut ctx);
    }
    input
}

fn hit<'a>(input: &'a InputState<ActionDescriptor>, control_id: &str) -> &'a HitTarget<ActionDescriptor> {
    input.hit_targets.iter().find(|target| target.control_id.as_deref() == Some(control_id)).unwrap_or_else(|| panic!("no hit target registered for control_id {control_id:?}"))
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
    assert!(input.hit_targets.iter().all(|target| target.control_id.as_deref() != Some("s1.header.name")), "a non-sortable column must not register a header sort hit target");
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
