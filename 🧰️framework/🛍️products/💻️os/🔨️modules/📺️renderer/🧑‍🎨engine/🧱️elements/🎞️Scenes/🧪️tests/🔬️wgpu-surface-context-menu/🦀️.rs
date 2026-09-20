//! 🖱️ Standing law for the per-scene-kind context-menu SURFACE TARGET — the `surface` half of
//! React's `openSurfaceContextMenu({ menu, surface: { surfaceId, kind, hits, selection }, … })`.
//!
//! Every assertion here is keyed to one React host's own `onContextMenu`, because the `hits`/
//! `selection` shape is per-kind CONVENTION rather than a schema: a kind that tracks no pick state
//! reports no hits, a kind that tracks no selection reports no groups, and the domains are the exact
//! strings a plugin's menu resolver matches on (`row`, `entry`, `block`, `node`, `layer`, `position`,
//! `route`, `object`, `feature`). A drift in one of those strings is silent — the menu simply comes
//! back empty — so it is pinned here rather than left to a live boot.

use super::*;
use ui_wgpu::wgpu::{BlockListScene, Canvas2dScene, DiffViewScene, EventFeedScene, GraphTimelineScene, InkCanvasScene, TableScene, VirtualFileSystemScene};

fn scene(kind: SurfaceKind) -> UiComponentSceneNode {
    UiComponentSceneNode {
        surface_id: "surface-1".into(),
        controller_id: "controller".into(),
        component_kind: kind,
        pane_id: None,
        binding_id: None,
        presence: UiPresence::default(),
        canvas_2d: None,
        world_3d: None,
        node_graph: None,
        text_editor: None,
        table: None,
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

const SURFACE: Rect = Rect { x: 0.0, y: 0.0, w: 400.0, h: 300.0 };

/// 📊️ A body row answers `{domain:"row", id}` and the table's own `selectedIds` — `TableHost`'s
/// `onRowContextMenu` (`📊️Table/🟦️.tsx:240`).
#[test]
fn table_reports_the_row_under_the_pointer_and_its_selection() {
    let mut table = TableScene::base(json!([{ "id": "name", "label": "Name", "sortable": false }]).to_string(), json!([{ "id": "row-a" }, { "id": "row-b" }]).to_string());
    table.selection_json = Some(json!({ "selectedIds": ["row-b"] }).to_string());
    let mut node = scene(SurfaceKind::Table);
    node.table = Some(table);
    let theme = Theme::default();
    let metrics = table_metrics(SURFACE, 1, &theme);
    let target = scene_context_menu_target(&node, SURFACE, 10.0, metrics.body.y + metrics.row_h * 0.5);
    assert_eq!(target.hits, vec![ui_wgpu::wgpu::ContextMenuHit { domain: "row".into(), id: "row-a".into(), label: None }]);
    assert_eq!(target.selection, vec![ui_wgpu::wgpu::ContextMenuSelectionGroup { domain: "row".into(), ids: vec!["row-b".into()] }]);
    assert!(target.text.is_none());
}

/// 📊️ A press in the HEADER band reports no hit: React binds the menu on rows only, so the request
/// must travel as a whole-surface one rather than inventing a header domain.
#[test]
fn table_header_band_reports_no_hit() {
    let table = TableScene::base(json!([{ "id": "name", "label": "Name", "sortable": true }]).to_string(), json!([{ "id": "row-a" }]).to_string());
    let mut node = scene(SurfaceKind::Table);
    node.table = Some(table);
    let target = scene_context_menu_target(&node, SURFACE, 10.0, SURFACE.y + 1.0);
    assert!(target.hits.is_empty());
}

/// 🗂️ The VISIBLE row (expansion-aware) answers `{domain:"row"}`, and `selectedRowIdsJson` is the
/// selection group — `VirtualFileSystemHost`'s `onRowContextMenu` (`🗣️Interpreter/🟦️.tsx:820`).
#[test]
fn virtual_file_system_reports_the_visible_row_and_selected_rows() {
    let rows = json!([{ "id": "dir", "name": "dir", "children": [{ "id": "leaf", "name": "leaf" }] }]).to_string();
    let vfs = VirtualFileSystemScene { schema_json: json!({ "columns": [] }).to_string(), rows_json: rows, selected_row_ids_json: Some(json!(["leaf"]).to_string()), hovered_row_id: None, empty_message: None, drag_drop_enabled: None };
    let mut node = scene(SurfaceKind::VirtualFileSystem);
    node.virtual_file_system = Some(vfs);
    let theme = Theme::default();
    let metrics = vfs_metrics(SURFACE, &theme);
    let target = scene_context_menu_target(&node, SURFACE, 10.0, metrics.body.y + metrics.row_h * 0.5);
    assert_eq!(target.hits, vec![ui_wgpu::wgpu::ContextMenuHit { domain: "row".into(), id: "dir".into(), label: None }]);
    assert_eq!(target.selection, vec![ui_wgpu::wgpu::ContextMenuSelectionGroup { domain: "row".into(), ids: vec!["leaf".into()] }]);
}

/// 📜️ A feed row answers `{domain:"entry", id}` and NO selection — an `EventFeedScene` tracks none
/// (`📡️EventFeedHost/🟦️.tsx:89`).
#[test]
fn event_feed_reports_the_entry_under_the_pointer_and_never_a_selection() {
    let entries = json!([{ "id": "e1", "title": "first" }, { "id": "e2", "title": "second" }]).to_string();
    let mut node = scene(SurfaceKind::EventFeed);
    node.event_feed = Some(EventFeedScene { entries_json: entries, follow: None, activate_action: None, domain_id: None });
    let theme = Theme::default();
    let target = scene_context_menu_target(&node, SURFACE, 10.0, SURFACE.y + theme.control_height * 0.5);
    assert_eq!(target.hits, vec![ui_wgpu::wgpu::ContextMenuHit { domain: "entry".into(), id: "e1".into(), label: None }]);
    assert!(target.selection.is_empty());
}

/// 🖱️ The four WHOLE-SURFACE kinds report an empty target, byte-for-byte React's
/// `surface: { hits: [], selection: [] }` — a hit invented here would be a domain no resolver knows.
#[test]
fn whole_surface_kinds_report_no_hits_and_no_selection() {
    let mut timeline = scene(SurfaceKind::GraphTimeline);
    timeline.graph_timeline = Some(GraphTimelineScene { columns_json: json!([]).to_string() });
    let mut blocks = scene(SurfaceKind::BlockList);
    blocks.block_list = Some(BlockListScene { steps_json: json!([]).to_string(), palette_json: json!([]).to_string(), selected_id: None, dragging_id: None, domain_id: None });
    let mut diff = scene(SurfaceKind::DiffView);
    diff.diff_view = Some(DiffViewScene { before: "a".into(), after: "b".into(), mode: Some("unified".into()), language: None, domain_id: None });
    let mut canvas = scene(SurfaceKind::Canvas2d);
    canvas.canvas_2d = Some(Canvas2dScene::base(0.0, 0.0, 1.0, json!([]).to_string()));
    for node in [timeline, blocks, diff, canvas] {
        let target = scene_context_menu_target(&node, SURFACE, 40.0, 40.0);
        assert!(target.hits.is_empty(), "{:?} must report no hits", node.component_kind);
        assert!(target.selection.is_empty(), "{:?} must report no selection", node.component_kind);
        assert!(target.text.is_none(), "{:?} must carry no text context", node.component_kind);
    }
}

/// 🖋️ Ink answers every block under the pointer as `{domain:"block"}`, topmost first, and a
/// right-click OUTSIDE the painted selection targets the topmost hit instead — React's own
/// "select-then-menu" rule (`🖋️InkCanvasHost/🟦️.tsx:1355-1367`).
#[test]
fn ink_canvas_reports_blocks_under_the_pointer_topmost_first() {
    let document = json!({
        "camera": { "x": 0.0, "y": 0.0, "zoom": 1.0 },
        "blocks": [
            { "id": "under", "x": 0.0, "y": 0.0, "points": [[10.0, 10.0], [30.0, 10.0]], "strokeWidth": 2.0 },
            { "id": "over", "x": 0.0, "y": 0.0, "points": [[10.0, 10.0], [30.0, 10.0]], "strokeWidth": 2.0 }
        ]
    })
    .to_string();
    let mut node = scene(SurfaceKind::InkCanvas);
    node.ink_canvas = Some(InkCanvasScene::base(document, "select".into(), "content".into(), true));
    let target = scene_context_menu_target(&node, SURFACE, 20.0, 10.0);
    assert_eq!(target.hits.iter().map(|hit| hit.id.as_str()).collect::<Vec<_>>(), vec!["over", "under"]);
    assert_eq!(target.hits.iter().map(|hit| hit.domain.as_str()).collect::<Vec<_>>(), vec!["block", "block"]);
    assert_eq!(target.selection, vec![ui_wgpu::wgpu::ContextMenuSelectionGroup { domain: "block".into(), ids: vec!["over".into()] }]);
}

/// 🖋️ A right-click INSIDE the painted selection keeps that selection — React only replaces it when
/// the topmost hit is not already selected.
#[test]
fn ink_canvas_keeps_the_painted_selection_when_the_hit_is_already_selected() {
    let document = json!({
        "camera": { "x": 0.0, "y": 0.0, "zoom": 1.0 },
        "blocks": [{ "id": "over", "x": 0.0, "y": 0.0, "points": [[10.0, 10.0], [30.0, 10.0]], "strokeWidth": 2.0 }]
    })
    .to_string();
    let mut ink = InkCanvasScene::base(document, "select".into(), "content".into(), true);
    ink.selection_json = json!(["over", "other"]).to_string();
    let mut node = scene(SurfaceKind::InkCanvas);
    node.ink_canvas = Some(ink);
    let target = scene_context_menu_target(&node, SURFACE, 20.0, 10.0);
    assert_eq!(target.selection, vec![ui_wgpu::wgpu::ContextMenuSelectionGroup { domain: "block".into(), ids: vec!["over".into(), "other".into()] }]);
}

/// 🖋️ Empty canvas: no block under the pointer, and nothing selected — no groups at all.
#[test]
fn ink_canvas_reports_nothing_on_empty_canvas() {
    let document = json!({ "camera": { "x": 0.0, "y": 0.0, "zoom": 1.0 }, "blocks": [] }).to_string();
    let mut node = scene(SurfaceKind::InkCanvas);
    node.ink_canvas = Some(InkCanvasScene::base(document, "select".into(), "content".into(), true));
    let target = scene_context_menu_target(&node, SURFACE, 200.0, 200.0);
    assert!(target.hits.is_empty());
    assert!(target.selection.is_empty());
}

/// 🏷️ The menu VOCABULARY id is camelCase for all fifteen kinds — the string React sends as both
/// `menu.id` and `surface.kind`, and the key `contextMenuSurfaceTitleKeys` looks the menu title up
/// by (`🗣️Interpreter/🟦️.tsx:712`). It is deliberately NOT `SurfaceKind::as_str`'s kebab wire tag.
#[test]
fn context_menu_surface_kind_ids_are_react_camel_case() {
    let expected = [
        (SurfaceKind::BlockList, "blockList"),
        (SurfaceKind::Board2d, "board2d"),
        (SurfaceKind::Canvas2d, "canvas2d"),
        (SurfaceKind::DiffView, "diffView"),
        (SurfaceKind::EventFeed, "eventFeed"),
        (SurfaceKind::GraphTimeline, "graphTimeline"),
        (SurfaceKind::IconRender, "iconRender"),
        (SurfaceKind::InkCanvas, "inkCanvas"),
        (SurfaceKind::NodeGraph, "nodeGraph"),
        (SurfaceKind::Paint2d, "paint2d"),
        (SurfaceKind::Table, "table"),
        (SurfaceKind::TextEditor, "textEditor"),
        (SurfaceKind::TiledMap, "tiledMap"),
        (SurfaceKind::VirtualFileSystem, "virtualFileSystem"),
        (SurfaceKind::World3d, "world3d"),
    ];
    for (kind, id) in expected {
        assert_eq!(context_menu_surface_kind_id(kind), id);
        assert!(!id.contains('-'), "{id} must not be the kebab wire tag");
    }
}

/// 🕸️ `parseSelectionDomainsFromSession` (`🌐️World3dHost/🟦️.tsx:1801`) accepts BOTH shapes a node-graph
/// host answers with: the bare node-id array and the per-domain object (with the `edgeIds`/`handleIds`
/// aliases). A resolver that only read one of them dropped every edge/handle selection.
#[test]
fn node_graph_selection_domains_accept_both_wire_shapes() {
    assert_eq!(engine_canvas::selection_domains_for_test(&json!(["a", "b"]).to_string()), (vec!["a".to_string(), "b".to_string()], Vec::new(), Vec::new()));
    assert_eq!(engine_canvas::selection_domains_for_test(&json!({ "nodes": ["n"], "edgeIds": ["e"], "handleIds": ["h"] }).to_string()), (vec!["n".to_string()], vec!["e".to_string()], vec!["h".to_string()]));
    assert_eq!(engine_canvas::selection_domains_for_test(&json!({ "nodes": ["n"], "edges": ["e"], "handles": ["h"] }).to_string()), (vec!["n".to_string()], vec!["e".to_string()], vec!["h".to_string()]));
    assert_eq!(engine_canvas::selection_domains_for_test("not json"), (Vec::new(), Vec::new(), Vec::new()));
}

/// 🖱️ `hits` carry the host's `label` when it publishes one — React maps `{domain, id, label}` for
/// every canvas pick target, and a resolver labelling its rows off the hit relies on it.
#[test]
fn pick_target_hits_carry_the_optional_label() {
    let hits = engine_canvas::pick_target_hits_for_test(&json!([{ "domain": "node", "id": "n1", "label": "Node One" }, { "domain": "edge", "id": "e1" }]).to_string());
    assert_eq!(hits[0], ui_wgpu::wgpu::ContextMenuHit { domain: "node".into(), id: "n1".into(), label: Some("Node One".into()) });
    assert_eq!(hits[1], ui_wgpu::wgpu::ContextMenuHit { domain: "edge".into(), id: "e1".into(), label: None });
    assert!(engine_canvas::pick_target_hits_for_test("null").is_empty());
}
