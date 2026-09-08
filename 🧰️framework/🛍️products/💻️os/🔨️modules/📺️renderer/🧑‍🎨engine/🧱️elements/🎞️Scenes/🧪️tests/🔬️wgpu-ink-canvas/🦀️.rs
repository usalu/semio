
use super::*;
use ui_wgpu::wgpu::UiPresence;

fn sample_block(id: &str, x: f64, y: f64, w: f64, h: f64) -> Value {
    json!({
        "id": id, "name": "Text", "kind": "text", "x": x, "y": y, "width": w, "height": h,
        "rotation": 0.0, "visible": true, "locked": false,
        "paragraphs": [], "fontSize": 18.0, "fontWeight": "normal", "align": "left",
    })
}

#[test]
fn hit_test_prefers_topmost_block() {
    let blocks = vec![sample_block("a", 0.0, 0.0, 100.0, 100.0), sample_block("b", 20.0, 20.0, 100.0, 100.0)];
    let overrides = HashMap::new();
    let hits = ink_items_at_point(&blocks, &overrides, 50.0, 50.0);
    assert_eq!(ink_item_id(hits[0]), "b");
}

#[test]
fn hit_test_misses_outside_bounds() {
    let blocks = vec![sample_block("a", 0.0, 0.0, 10.0, 10.0)];
    let overrides = HashMap::new();
    assert!(ink_items_at_point(&blocks, &overrides, 50.0, 50.0).is_empty());
}

#[test]
fn resize_bounds_east_handle_grows_width_only() {
    let from = InkBoundsF { x: 0.0, y: 0.0, w: 100.0, h: 50.0 };
    let to = ink_resize_bounds(from, "e", 20.0, 0.0, 8.0);
    assert_eq!(to, InkBoundsF { x: 0.0, y: 0.0, w: 120.0, h: 50.0 });
}

#[test]
fn resize_bounds_northwest_handle_moves_origin() {
    let from = InkBoundsF { x: 10.0, y: 10.0, w: 100.0, h: 100.0 };
    let to = ink_resize_bounds(from, "nw", -10.0, -10.0, 8.0);
    assert_eq!(to, InkBoundsF { x: 0.0, y: 0.0, w: 110.0, h: 110.0 });
}

#[test]
fn resize_bounds_respects_minimum_size() {
    let from = InkBoundsF { x: 0.0, y: 0.0, w: 20.0, h: 20.0 };
    let to = ink_resize_bounds(from, "e", -100.0, 0.0, 8.0);
    assert_eq!(to.w, 8.0);
}

#[test]
fn screen_world_roundtrip() {
    let camera = InkCameraF { x: 12.0, y: -8.0, zoom: 1.5 };
    let inner = Rect::new(100.0, 40.0, 400.0, 300.0);
    let (wx, wy) = ink_screen_to_world(camera, inner, 250.0, 150.0);
    let (sx, sy) = ink_world_to_screen(camera, inner, wx, wy);
    assert!((sx - 250.0).abs() < 0.01);
    assert!((sy - 150.0).abs() < 0.01);
}

#[test]
fn snap_rounds_to_nearest_grid_cell() {
    assert_eq!(ink_snap_coordinate(13.0, 8.0), 16.0);
    assert_eq!(ink_snap_coordinate(3.0, 8.0), 0.0);
}

#[test]
fn ink_block_bounds_from_points() {
    let block = json!({
        "id": "i1", "kind": "stroke", "x": 10.0, "y": 10.0, "width": 1.0, "height": 1.0,
        "points": [[0.0, 0.0], [5.0, 10.0], [-5.0, 2.0]], "strokeWidth": 3.0, "color": [0, 0, 0, 1],
    });
    let bounds = ink_item_bounds(&block);
    assert_eq!(bounds.x, 5.0);
    assert_eq!(bounds.y, 10.0);
    assert_eq!(bounds.w, 10.0);
    assert_eq!(bounds.h, 10.0);
}

//#region InkCanvasPaintTests
#[test]
fn item_card_background_uses_the_background_token_not_panel() {
    let block = json!({
        "id": "i1", "kind": "text", "x": 0.0, "y": 0.0, "width": 100.0, "height": 40.0,
        "paragraphs": [], "fontSize": 16.0,
    });
    let doc: InkDocumentJson = serde_json::from_str("{}").unwrap();
    let scene = UiComponentSceneNode {
        surface_id: "ink-paint-test".into(),
        controller_id: "controller".into(),
        component_kind: SurfaceKind::InkCanvas,
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
    };
    let mut draw = ui_wgpu::wgpu::DrawList::default();
    let mut atlas = ui_wgpu::wgpu::FontAtlas::builtin();
    let mut input = ui_wgpu::wgpu::InputState::<ActionDescriptor>::default();
    let theme = Theme::default();
    let mut scroll = HashMap::new();
    let mut collapsed = HashMap::new();
    let mut selects = HashMap::new();
    {
        let mut ctx = crate::interpreter::framework_widget_context(&mut draw, None, &mut atlas, None, &mut input, &theme, &mut scroll, &mut collapsed, &mut selects, None);
        let camera = InkCameraF { x: 0.0, y: 0.0, zoom: 1.0 };
        let inner = Rect::new(0.0, 0.0, 400.0, 300.0);
        draw_ink_item(&mut ctx, &scene, &block, camera, inner, &doc, false, false);
    }
    // 🎨️ `bg-background/90` in `ink-canvas-host.tsx` — the card's fill must resolve to
    // `theme.background`, not `theme.panel` (the app-chrome surface token).
    let expected = theme.background.with_alpha(0.9);
    let colors: Vec<[f32; 4]> = draw.layers.iter().flat_map(|layer| layer.ui_instances.iter()).map(|i| i.color).collect();
    assert!(colors.contains(&[expected.r, expected.g, expected.b, expected.a]), "expected an item card fill at theme.background@0.9, got {colors:?}");
    let stale = theme.panel.with_alpha(0.92);
    assert!(!colors.contains(&[stale.r, stale.g, stale.b, stale.a]), "the item card must no longer fill with the stale theme.panel@0.92 token");
}
//#endregion InkCanvasPaintTests
