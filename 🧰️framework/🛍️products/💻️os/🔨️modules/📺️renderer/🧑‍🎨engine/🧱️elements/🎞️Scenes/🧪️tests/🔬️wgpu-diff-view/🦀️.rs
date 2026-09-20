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
        let mut ctx = crate::interpreter::framework_widget_context(&mut draw, None, &mut atlas, None, &mut input, &theme, &mut scroll, &mut collapsed, &mut selects, None, 0.0);
        render_diff_view(&scene, Rect::new(0.0, 0.0, 400.0, 300.0), &mut ctx);
    }
    (draw, theme)
}

fn glyph_colors(draw: &ui_wgpu::wgpu::DrawList) -> Vec<Rgba> {
    draw.layers.iter().flat_map(|layer| layer.ui_instances.iter()).map(|instance| Rgba::new(instance.color[0], instance.color[1], instance.color[2], instance.color[3])).collect()
}

/// 🔢️ Colours of the glyphs painted INSIDE the line-number gutter band, and colours of the glyphs
/// painted after it. React's gutter is muted (`text-muted-foreground`) while the line text keeps its
/// own tint, so a palette-wide "no muted anywhere" assertion cannot tell the two apart — every
/// brightness law below scopes itself to the text band by x.
fn glyph_colors_split_at_gutter(draw: &ui_wgpu::wgpu::DrawList, gutter_right_x: f32) -> (Vec<Rgba>, Vec<Rgba>) {
    let mut gutter = Vec::new();
    let mut text = Vec::new();
    for instance in draw.layers.iter().flat_map(|layer| layer.ui_instances.iter()) {
        let color = Rgba::new(instance.color[0], instance.color[1], instance.color[2], instance.color[3]);
        if instance.rect[0] < gutter_right_x {
            gutter.push(color);
        } else {
            text.push(color);
        }
    }
    (gutter, text)
}

/// 📏️ Where the unified gutter band ends — `render_diff_view`'s own derivation (`pad` + two columns + `pad`), restated here so the law moves with the paint instead of hardcoding a pixel.
fn unified_gutter_right_x(theme: &Theme) -> f32 {
    theme.padding_standard + DIFF_GUTTER_COLUMN_W * 2.0 + theme.padding_standard
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
    // 🔢️ Scoped to the TEXT band: the gutter to its left is legitimately muted (React's
    // `text-muted-foreground` number spans), and this law is about the line text only. It used to
    // assert `theme.text_muted` was absent from the WHOLE palette, which is why the gutter could not
    // be painted at all (ticket 26/09/17/WGPU-RENDERER-REACT-PARITY,
    // `📓️audit-w14-scenes-residual.md` item 8).
    let (_, text_colors) = glyph_colors_split_at_gutter(&draw, unified_gutter_right_x(&theme));
    assert!(text_colors.contains(&theme.text), "an unchanged line must render at full theme.text brightness (React never dims equal lines), got {text_colors:?}");
    assert!(!text_colors.contains(&theme.text_muted), "unchanged line TEXT must not be dimmed to theme.text_muted, got {text_colors:?}");
}

/// 🔢️ React `🔺️DiffViewHost/🟦️.tsx` `UnifiedDiff`: every row opens with two muted, right-aligned
/// `tabular-nums` spans holding `line.beforeNo ?? ""` and `line.afterNo ?? ""`.
#[test]
fn unified_rows_open_with_a_muted_before_and_after_line_number_gutter() {
    let (draw, theme) = render_diff("a\nb\n", "a\nb\n", Some("unified"));
    let (gutter_colors, text_colors) = glyph_colors_split_at_gutter(&draw, unified_gutter_right_x(&theme));
    assert!(!gutter_colors.is_empty(), "a unified diff must paint line numbers in the gutter band");
    assert!(gutter_colors.iter().all(|color| *color == theme.text_muted), "every gutter glyph is theme.text_muted, like React's text-muted-foreground spans, got {gutter_colors:?}");
    assert!(text_colors.contains(&theme.text), "the line text still paints to the right of the gutter");
}

/// 🔢️ React assigns `beforeNo` to `equal`/`remove` lines and `afterNo` to `equal`/`add` lines
/// (`🔺️DiffViewHost/🟦️.tsx:40`/`:44`/`:47`), and prints `""` for the missing one.
#[test]
fn removed_lines_carry_only_a_before_number_and_added_lines_only_an_after_number() {
    let before = vec!["a", "old", "c"];
    let after = vec!["a", "new", "c"];
    let operations = diff_lines(&before, &after);
    let removed = operations.iter().find(|line| line.operation == DiffLineOperation::Removed).expect("a removed line");
    let added = operations.iter().find(|line| line.operation == DiffLineOperation::Added).expect("an added line");
    let equal = operations.iter().find(|line| line.operation == DiffLineOperation::Equal).expect("an equal line");
    assert_eq!((removed.before_no, removed.after_no), (Some(2), None), "a removed line numbers only the BEFORE side");
    assert_eq!((added.before_no, added.after_no), (None, Some(2)), "an added line numbers only the AFTER side");
    assert_eq!((equal.before_no, equal.after_no), (Some(1), Some(1)), "an equal line numbers both sides, 1-based");
}

#[test]
fn split_mode_added_and_removed_columns_use_accent_and_error_text() {
    let (draw, theme) = render_diff("old\n", "new\n", Some("split"));
    let colors = glyph_colors(&draw);
    assert!(colors.contains(&theme.accent), "split-mode added column text should be theme.accent");
    assert!(colors.contains(&theme.error), "split-mode removed column text should be theme.error");
}
//#endregion DiffViewPaintTests
