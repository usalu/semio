
use super::*;

#[test]
fn identical_inputs_produce_only_equal_operations() {
    let before = vec!["a", "b", "c"];
    let after = vec!["a", "b", "c"];
    let operations = diff_lines(&before, &after);
    assert_eq!(operations.len(), 3);
    assert!(operations.iter().all(|line| line.operation == DiffLineOperation::Equal));
}

#[test]
fn pure_addition_is_all_added_after_the_shared_prefix() {
    let before = vec!["a"];
    let after = vec!["a", "b", "c"];
    let operations = diff_lines(&before, &after);
    assert_eq!(operations[0].operation, DiffLineOperation::Equal);
    assert_eq!(operations[1].operation, DiffLineOperation::Added);
    assert_eq!(operations[2].operation, DiffLineOperation::Added);
}

#[test]
fn pure_removal_is_all_removed_after_the_shared_prefix() {
    let before = vec!["a", "b", "c"];
    let after = vec!["a"];
    let operations = diff_lines(&before, &after);
    assert_eq!(operations[0].operation, DiffLineOperation::Equal);
    assert_eq!(operations[1].operation, DiffLineOperation::Removed);
    assert_eq!(operations[2].operation, DiffLineOperation::Removed);
}

#[test]
fn changed_line_shows_as_a_remove_add_pair() {
    let before = vec!["a", "old", "c"];
    let after = vec!["a", "new", "c"];
    let operations = diff_lines(&before, &after);
    assert_eq!(operations.iter().filter(|line| line.operation == DiffLineOperation::Removed).count(), 1);
    assert_eq!(operations.iter().filter(|line| line.operation == DiffLineOperation::Added).count(), 1);
    assert_eq!(operations.iter().filter(|line| line.operation == DiffLineOperation::Equal).count(), 2);
}

#[test]
fn empty_inputs_produce_no_operations() {
    assert!(diff_lines(&[], &[]).is_empty());
}

#[test]
fn oversized_inputs_use_the_positional_fallback_without_panicking() {
    let before: Vec<&str> = vec!["x"; 2000];
    let after: Vec<&str> = vec!["x"; 2000];
    let operations = diff_lines(&before, &after);
    assert_eq!(operations.len(), 2000);
    assert!(operations.iter().all(|line| line.operation == DiffLineOperation::Equal));
}

//#region DiffViewPaintTests
/// 🧰️ Renders a `render_diff_view` scene in `mode` and returns the `DrawList` so paint-level
/// assertions (glyph tint, absence of row-background fills) can inspect it directly, same
/// technique as `render_entry_tests::Fixture`.
fn render_diff(before: &str, after: &str, mode: Option<&str>) -> (ui_wgpu::wgpu::DrawList, Theme) {
    let scene = UiComponentSceneNode {
        surface_id: "diff-paint-test".into(),
        controller_id: "controller".into(),
        component_kind: SurfaceKind::DiffView,
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
        diff_view: Some(ui_wgpu::wgpu::DiffViewScene { before: before.into(), after: after.into(), language: None, mode: mode.map(str::to_string), domain_id: None }),
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
        render_diff_view(&scene, Rect::new(0.0, 0.0, 400.0, 300.0), &mut ctx);
    }
    (draw, theme)
}

fn glyph_colors(draw: &ui_wgpu::wgpu::DrawList) -> Vec<Rgba> {
    draw.layers.iter().flat_map(|layer| layer.ui_instances.iter()).map(|instance| Rgba::new(instance.color[0], instance.color[1], instance.color[2], instance.color[3])).collect()
}

#[test]
fn unified_added_line_text_is_tinted_accent_not_a_row_background() {
    let (draw, theme) = render_diff("a\n", "a\nnew\n", Some("unified"));
    let colors = glyph_colors(&draw);
    assert!(colors.contains(&theme.accent), "added line's glyph text should be tinted theme.accent, got {colors:?}");
    // 🚫️ No translucent full-row wash left over — the old background-fill mechanism pushed a
    // `push_solid` at `theme.accent.with_alpha(0.16)` for every added row.
    assert!(!colors.contains(&theme.accent.with_alpha(0.16)), "added rows must no longer paint a translucent background wash");
}

#[test]
fn unified_removed_line_text_is_tinted_error() {
    let (draw, theme) = render_diff("old\n", "\n", Some("unified"));
    let colors = glyph_colors(&draw);
    assert!(colors.contains(&theme.error), "removed line's glyph text should be tinted theme.error, got {colors:?}");
}

#[test]
fn unchanged_lines_stay_full_brightness_not_dimmed() {
    let (draw, theme) = render_diff("same\n", "same\n", Some("unified"));
    let colors = glyph_colors(&draw);
    assert!(colors.contains(&theme.text), "an unchanged line must render at full theme.text brightness (React never dims equal lines), got {colors:?}");
    assert!(!colors.contains(&theme.text_muted), "unchanged lines must not be dimmed to theme.text_muted, got {colors:?}");
}

#[test]
fn split_mode_added_and_removed_columns_use_accent_and_error_text() {
    let (draw, theme) = render_diff("old\n", "new\n", Some("split"));
    let colors = glyph_colors(&draw);
    assert!(colors.contains(&theme.accent), "split-mode added column text should be theme.accent");
    assert!(colors.contains(&theme.error), "split-mode removed column text should be theme.error");
}
//#endregion DiffViewPaintTests
