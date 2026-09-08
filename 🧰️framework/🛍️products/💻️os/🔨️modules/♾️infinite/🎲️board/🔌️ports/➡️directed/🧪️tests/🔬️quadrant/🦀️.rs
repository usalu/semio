
use super::canvas::Color;
use super::*;

#[test]
fn canvas_theme_default_uses_centralized_board_light_tokens() {
    let p = CanvasPalette::default();
    let t = &ui_styling::BOARD_LIGHT;
    assert_eq!(p.raster_clear, Color::new(t.raster_clear));
    assert_eq!(p.edge_stroke_selected, Color::new(t.edge_stroke_selected));
    assert_eq!(p.handle_fill.to_rgba8().a, Color::new(t.handle_fill).to_rgba8().a);
}

#[test]
fn board_icon_paint_colors_use_canvas_tokens_not_node_chrome() {
    let theme = CanvasPalette::default();
    let (fg, bg) = IconPaintCache::board_icon_paint_colors(&theme);
    assert_eq!(fg, Color::new(ui_styling::CANVAS_LIGHT.icon_fg));
    assert_eq!(bg, Color::new(ui_styling::CANVAS_LIGHT.icon_bg));
    assert_ne!(fg.to_rgba8(), theme.node_stroke.to_rgba8());
}

#[test]
fn icon_cache_epoch_invalidation_retains_owner_until_cursor_close() {
    let cache = IconPaintCache::new();
    let svg = "<svg xmlns='http://www.w3.org/2000/svg' viewBox='0 0 1 1'><path d='M0 0L1 0L1 1Z'/></svg>";
    let fg = Color::new([1.0, 1.0, 1.0, 1.0]);
    let bg = Color::new([0.0, 0.0, 0.0, 1.0]);
    assert!(cache.get_or_build(svg, fg, bg, true).is_some());
    assert_eq!(cache.occupied_slots(), 1);
    cache.clear();
    assert!(cache.get_or_build(svg, fg, bg, true).is_some());
    assert_eq!(cache.occupied_slots(), 2);
    let mut retained_scene_seen = false;
    while !cache.close_step() {
        retained_scene_seen |= cache.has_retiring_scene();
    }
    assert!(retained_scene_seen, "vector icon scene remains retained until its exact retirement cursor drains");
    assert!(cache.terminal_is_empty());
}

#[test]
fn icon_cache_rejects_oversized_source_before_owner_construction() {
    let cache = IconPaintCache::new();
    let source = "x".repeat(16 * 1024 + 1);
    assert!(cache.get_or_build(&source, Color::new([1.0; 4]), Color::new([0.0; 4]), false).is_none());
    assert!(cache.faulted());
    assert_eq!(cache.occupied_slots(), 0);
    while !cache.close_step() {}
}

/// 🪶️ A cache that never admitted an icon owns nothing requiring incremental retirement.
#[test]
fn empty_icon_cache_releases_without_entering_close() {
    let cache = IconPaintCache::new();
    assert_eq!(cache.occupied_slots(), 0);
    drop(cache);
}

#[test]
fn board_engine_alias_works() {
    let mut engine = BoardEngine::new();
    engine.create_node(1, 0.0, 0.0, 40.0, true);
    engine.create_handle(10, 1, 0.0);
    engine.create_handle(11, 1, 3.14);
    engine.create_edge(100, 10, 11);
    assert_eq!(engine.render_snapshot().edges.len(), 1);
}
