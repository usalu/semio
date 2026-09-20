//! 📐️ Per-widget metric parity: every size this target paints must be the generated token's px
//! value, and the same one React's class or prop resolves to. One assertion per drifted number the
//! ticket 26/09/17/WGPU-RENDERER-REACT-PARITY interpreter/element audit found (§5.2 "Icon size",
//! "Line height", and the widget rows), so a hand-typed literal creeping back in fails here rather
//! than in a screenshot diff.

use crate::wgpu::chrome::{ICON_TINY, ICON_TREE_ROW, SIZE_TINY};
use crate::wgpu::component::ui::{UiPresence, UiProgressNode};
use crate::wgpu::geometry::Rect;
use crate::wgpu::theme::Theme;
use crate::wgpu::Label;

/// 📶️ One progress node; `total: None` is the indeterminate case (`ui_contract::progress_fraction`).
fn progress(completed: f64, total: Option<f64>) -> UiProgressNode {
    UiProgressNode { id: "progress".into(), completed, total, value_text: Label::data("25%"), presence: UiPresence::default(), menu: None }
}

/// 🔣️ React's three icon boxes, at the compact 16px root: `size-tiny` 9.6, `size-small` 16 (what
/// `<Icon size="small">`, i.e. `renderControlIcon`'s default, resolves its CLASS to), and the
/// literal `12` the Interpreter hands a `Tree` row's icon and row actions.
#[semio_framework_async_macros::async_test]
async fn icon_boxes_resolve_to_reacts_own_three_sizes() {
    assert!((SIZE_TINY - 9.6).abs() < 0.001, "--size-tiny is calc(3 × --ui-spacing), got {SIZE_TINY}");
    assert!((ICON_TINY - 16.0).abs() < 0.001, "--size-small is calc(5 × --ui-spacing), got {ICON_TINY}");
    assert!((ICON_TREE_ROW - 12.0).abs() < 0.001, "a tree row icon is the Interpreter's literal 12, got {ICON_TREE_ROW}");
    assert!(SIZE_TINY < ICON_TREE_ROW && ICON_TREE_ROW < ICON_TINY, "the three boxes must stay distinct — collapsing them is the drift this test exists for");
}

/// 🌳️ The tree row metrics paint reads are the `dom.tree*UiSpacing` tokens, the same ones
/// `Theme`/`layout` already resolve — no second hand-typed copy in `paint`.
#[semio_framework_async_macros::async_test]
async fn tree_row_metrics_match_the_theme_tokens() {
    let theme = Theme::light();
    assert!((crate::wgpu::widgets::TREE_ROW_HEIGHT - theme.tree_row_height).abs() < 0.001);
    assert!((crate::wgpu::widgets::TREE_INDENT_PER_LEVEL - theme.tree_indent_per_level).abs() < 0.001);
    assert!((crate::wgpu::widgets::TREE_TOGGLE_WIDTH - theme.tree_toggle_width).abs() < 0.001);
    assert!((crate::wgpu::widgets::TREE_ICON_SIZE - ICON_TREE_ROW).abs() < 0.001);
}

/// 📶️ `Progress` is `h-tiny w-full` with the theme's own corner radius — not a one-spacing-unit
/// sliver, and not a hardcoded pill (`--radius-full` is `0` in this design system).
#[semio_framework_async_macros::async_test]
async fn progress_track_is_a_size_tiny_bar_across_the_full_width() {
    let theme = Theme::light();
    let node = progress(1.0, Some(4.0));
    let bounds = Rect::new(10.0, 20.0, 200.0, 40.0);
    let (track, fill) = crate::wgpu::paint::progress_bar_rects(&node, bounds, &theme);
    assert!((track[3] - SIZE_TINY).abs() < 0.001, "track height is --size-tiny, got {}", track[3]);
    assert!((track[2] - bounds.w).abs() < f32::EPSILON, "the track spans the node's full width");
    assert!((track[1] - (bounds.y + (bounds.h - SIZE_TINY) * 0.5)).abs() < 0.001, "the track is vertically centred in its node");
    assert!((fill[2] - track[2] * 0.25).abs() < 0.001, "a quarter-complete bar fills a quarter of the track");
    assert!((fill[3] - track[3]).abs() < f32::EPSILON);
}

/// 📶️ An indeterminate bar keeps the same track and centres its sweep, matching React's
/// `mx-auto w-1/3` fill.
#[semio_framework_async_macros::async_test]
async fn an_indeterminate_progress_bar_centres_a_fixed_share() {
    let theme = Theme::light();
    let node = progress(0.0, None);
    let bounds = Rect::new(0.0, 0.0, 300.0, 30.0);
    let (track, fill) = crate::wgpu::paint::progress_bar_rects(&node, bounds, &theme);
    assert!(fill[2] > 0.0 && fill[2] < track[2], "an indeterminate sweep is a share of the track, not all of it");
    let left = fill[0] - track[0];
    let right = track[0] + track[2] - (fill[0] + fill[2]);
    assert!((left - right).abs() < 0.001, "the sweep is centred");
}

/// 🧭️ Real gizmo paint stays bounded after translating a pane, and its circular heads match the neutral Three sprite corpus.
#[semio_framework_async_macros::async_test]
async fn orbit_gizmo_heads_keep_diameter_when_the_viewport_moves() {
    use crate::wgpu::draw_types::gizmo::{orbit_view_gizmo_head_radius, orbit_view_gizmo_tips};
    use crate::wgpu::draw_types::{DrawList, KIND_ROUNDED};
    use crate::wgpu::widgets::{gizmo::paint_orbit_view_gizmo, WidgetContext};
    use crate::wgpu::{Camera3d, FontAtlas, InputState};
    use std::collections::HashMap;

    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/🧭️gizmo-tip-bounds/🔣️.json")).unwrap();
    for row in fixture["cases"].as_array().unwrap() {
        let radius = orbit_view_gizmo_head_radius(row["prominent"].as_bool().unwrap(), row["hovered"].as_bool().unwrap());
        assert!((f64::from(radius) - row["radius"].as_f64().unwrap()).abs() < 0.00001);
    }
    let camera = Camera3d::default();
    let theme = Theme::light();
    let mut atlas = FontAtlas::builtin();
    for row in fixture["viewports"].as_array().unwrap() {
        let values: Vec<f32> = row.as_array().unwrap().iter().map(|value| value.as_f64().unwrap() as f32).collect();
        let viewport = Rect::new(values[0], values[1], values[2], values[3]);
        let tips = orbit_view_gizmo_tips(&camera, viewport);
        let center = &tips[14];
        for (tip, row) in tips.iter().take(6).zip(fixture["axisOffsets"].as_array().unwrap()) {
            let expected: Vec<f32> = row.as_array().unwrap().iter().map(|value| value.as_f64().unwrap() as f32).collect();
            assert!((tip.screen_x - center.screen_x - expected[0]).abs() < 0.001);
            assert!((tip.screen_y - center.screen_y - expected[1]).abs() < 0.001);
            assert!((tip.depth - expected[2]).abs() < 0.00001);
        }
        for hovered in [None, Some(0)] {
            let mut draw = DrawList::default();
            let mut input = InputState::<()>::default();
            let mut scroll = HashMap::new();
            let mut collapsed = HashMap::new();
            let mut selects = HashMap::new();
            let mut ctx = WidgetContext { draw: &mut draw, overlay: None, atlas: &mut atlas, icons: None, input: &mut input, theme: &theme, scroll_offsets: &mut scroll, collapsed_sections: &mut collapsed, open_selects: &mut selects, interaction_maps: None, pick_clip: None, viewport_height: viewport.h };
            paint_orbit_view_gizmo(&mut ctx, &camera, viewport, hovered);
            let heads: Vec<_> = draw.layers.iter().flat_map(|layer| &layer.overlay_ui_instances).collect();
            assert_eq!(heads.len(), 15);
            for head in heads {
                assert_eq!(head.params[2], KIND_ROUNDED);
                assert!((head.rect[2] - head.rect[3]).abs() < 0.00001);
                assert!(f64::from(head.rect[2]) <= fixture["maxHeadDiameter"].as_f64().unwrap() + 0.00001);
                assert!((head.params[0] * 2.0 - head.rect[2]).abs() < 0.00001);
                let center = [head.rect[0] + head.rect[2] * 0.5, head.rect[1] + head.rect[3] * 0.5];
                assert!(tips.iter().any(|tip| (tip.screen_x - center[0]).abs() < 0.001 && (tip.screen_y - center[1]).abs() < 0.001));
            }
        }
    }
}
