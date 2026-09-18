//! 🎹️ `Select`'s wgpu keyboard state machine and popup geometry, asserted against the numbers
//! React's own first-party `Select` (`🧱️elements/🔽️Select/🟦️.tsx`) produces — ticket
//! 26/09/17/WGPU-RENDERER-REACT-PARITY packet W1o.

use super::*;
use crate::wgpu::theme::Theme;

const LABELS: [&str; 5] = ["Alpha", "Beta", "Gamma", "Delta", "Épsilon"];

#[semio_framework_async_macros::async_test]
async fn a_closed_select_opens_on_the_same_keys_react_opens_on() {
    assert_eq!(select_key(false, "ArrowDown", false, false, false), SelectKey::Open(SelectOpenIntent::Selected));
    assert_eq!(select_key(false, "Enter", false, false, false), SelectKey::Open(SelectOpenIntent::Selected));
    assert_eq!(select_key(false, " ", false, false, false), SelectKey::Open(SelectOpenIntent::Selected));
    assert_eq!(select_key(false, "ArrowUp", false, false, false), SelectKey::Open(SelectOpenIntent::Last));
    assert_eq!(select_key(false, "g", false, false, false), SelectKey::Open(SelectOpenIntent::Typeahead('g')));
    assert_eq!(select_key(false, "g", false, true, false), SelectKey::Ignored, "a modifier chord is never typeahead");
    assert_eq!(select_key(false, "F2", false, false, false), SelectKey::Ignored);
}

#[semio_framework_async_macros::async_test]
async fn an_open_select_routes_navigation_commit_and_dismissal() {
    assert_eq!(select_key(true, "ArrowDown", false, false, false), SelectKey::Move(SelectMove::Next));
    assert_eq!(select_key(true, "ArrowUp", false, false, false), SelectKey::Move(SelectMove::Previous));
    assert_eq!(select_key(true, "Home", false, false, false), SelectKey::Move(SelectMove::First));
    assert_eq!(select_key(true, "End", false, false, false), SelectKey::Move(SelectMove::Last));
    assert_eq!(select_key(true, "PageDown", false, false, false), SelectKey::Move(SelectMove::PageNext));
    assert_eq!(select_key(true, "PageUp", false, false, false), SelectKey::Move(SelectMove::PagePrevious));
    assert_eq!(select_key(true, "Enter", false, false, false), SelectKey::Commit);
    assert_eq!(select_key(true, " ", false, false, false), SelectKey::Commit);
    assert_eq!(select_key(true, "Tab", false, false, false), SelectKey::Close);
    assert_eq!(select_key(true, "b", false, false, false), SelectKey::Typeahead('b'));
}

#[semio_framework_async_macros::async_test]
async fn arrow_movement_wraps_and_paged_movement_clamps_exactly_like_move_active() {
    assert_eq!(select_moved_index(Some(0), 5, SelectMove::Next), Some(1));
    assert_eq!(select_moved_index(Some(4), 5, SelectMove::Next), Some(0), "Next wraps past the last row");
    assert_eq!(select_moved_index(Some(0), 5, SelectMove::Previous), Some(4), "Previous wraps past the first row");
    assert_eq!(select_moved_index(None, 5, SelectMove::Next), Some(0), "an unhighlighted popup starts at the first row");
    assert_eq!(select_moved_index(Some(2), 5, SelectMove::First), Some(0));
    assert_eq!(select_moved_index(Some(2), 5, SelectMove::Last), Some(4));
    assert_eq!(select_moved_index(Some(0), 5, SelectMove::PageNext), Some(4), "a page jump clamps to the last row, it never wraps");
    assert_eq!(select_moved_index(Some(4), 5, SelectMove::PagePrevious), Some(0));
    assert_eq!(select_moved_index(Some(0), 32, SelectMove::PageNext), Some(SELECT_PAGE_STEP));
    assert_eq!(select_moved_index(None, 0, SelectMove::Next), None, "an empty popup has nothing to highlight");
}

#[semio_framework_async_macros::async_test]
async fn typeahead_scans_forward_from_the_highlighted_row_and_cycles() {
    assert_eq!(select_typeahead_index(LABELS, "b", None), Some(1));
    assert_eq!(select_typeahead_index(LABELS, "D", None), Some(3), "matching is case-insensitive");
    assert_eq!(select_typeahead_index(LABELS, "ga", Some(0)), Some(2), "a multi-character query is a prefix, not a repeat-cycle");
    assert_eq!(select_typeahead_index(LABELS, "a", Some(2)), Some(0), "the scan cycles past the end back to the first row");
    assert_eq!(select_typeahead_index(LABELS, "z", None), None);
    assert_eq!(select_typeahead_index(["Ärger", "Beta"], "a", None), Some(0), "case folding is Unicode-aware, not ASCII-only");
}

#[semio_framework_async_macros::async_test]
async fn normalize_matches_the_typescript_normalizer_on_marks_case_and_whitespace() {
    assert_eq!(normalize_select_text("  Two   Words  "), "two words");
    assert_eq!(normalize_select_text("E\u{0301}psilon"), "epsilon", "decomposed combining marks are dropped");
    assert_eq!(normalize_select_text(""), "");
}

#[semio_framework_async_macros::async_test]
async fn opening_highlights_the_selected_last_or_first_matching_row() {
    assert_eq!(select_open_index(SelectOpenIntent::Selected, LABELS, Some(3)), Some(3));
    assert_eq!(select_open_index(SelectOpenIntent::Selected, LABELS, None), Some(0), "nothing selected opens on the first row");
    assert_eq!(select_open_index(SelectOpenIntent::Last, LABELS, Some(1)), Some(4));
    assert_eq!(select_open_index(SelectOpenIntent::Typeahead('d'), LABELS, None), Some(3));
    assert_eq!(select_open_index(SelectOpenIntent::Typeahead('z'), LABELS, None), Some(0), "an unmatched key still opens with a highlighted row");
    assert_eq!(select_open_index(SelectOpenIntent::Selected, [] as [&str; 0], None), None);
}

#[semio_framework_async_macros::async_test]
async fn row_metrics_are_the_css_line_box_plus_py_single_not_a_control_height() {
    let theme = Theme::light();
    let expected_row = crate::wgpu::text::line_height(theme.font_size_body) + theme.padding_standard * 2.0;
    assert!((select_row_height(&theme) - expected_row).abs() < f32::EPSILON);
    assert!((select_row_height(&theme) - 25.6).abs() < 0.01, "text-sm line box (19.2) + 2 × --spacing-single (3.2) = 25.6 px, got {}", select_row_height(&theme));
    assert!((select_menu_height(3, &theme) - (expected_row * 3.0 + theme.padding_standard * 2.0)).abs() < f32::EPSILON);
    assert!((select_menu_height(0, &theme) - theme.padding_standard * 2.0).abs() < f32::EPSILON);
}

#[semio_framework_async_macros::async_test]
async fn the_popup_sits_below_the_trigger_until_the_viewport_runs_out_then_flips_above() {
    let theme = Theme::light();
    let menu_h = select_menu_height(4, &theme);
    let trigger_h = theme.control_height;
    assert!((select_menu_top(10.0, trigger_h, menu_h, 800.0) - (trigger_h + SELECT_SIDE_OFFSET)).abs() < f32::EPSILON, "room below places below");
    let near_bottom = 800.0 - trigger_h - 10.0;
    let flipped = select_menu_top(near_bottom, trigger_h, menu_h, 800.0);
    assert!(flipped < 0.0, "no room below and plenty above must flip the popup over the trigger, got {flipped}");
    assert!((near_bottom + flipped + menu_h + SELECT_SIDE_OFFSET - near_bottom).abs() < 0.001, "a flipped popup's bottom edge keeps the sideOffset gap");
    assert!(
        (select_menu_top(0.0, trigger_h, menu_h, trigger_h + 1.0) - (trigger_h + SELECT_SIDE_OFFSET)).abs() < f32::EPSILON,
        "with no room on EITHER side React keeps the requested side rather than flipping"
    );
    assert!((select_menu_top(600.0, trigger_h, menu_h, 0.0) - (trigger_h + SELECT_SIDE_OFFSET)).abs() < f32::EPSILON, "an unmeasured viewport never flips");
}

#[semio_framework_async_macros::async_test]
async fn row_rects_stack_from_the_resolved_popup_top_and_inset_by_the_viewport_padding() {
    let theme = Theme::light();
    let menu_top = select_menu_top(0.0, theme.control_height, select_menu_height(3, &theme), 800.0);
    let first = select_row_rect(120.0, 0, menu_top, &theme);
    let second = select_row_rect(120.0, 1, menu_top, &theme);
    assert!((first.x - theme.padding_standard).abs() < f32::EPSILON);
    assert!((first.w - (120.0 - theme.padding_standard * 2.0)).abs() < f32::EPSILON);
    assert!((first.y - (menu_top + theme.padding_standard)).abs() < f32::EPSILON);
    assert!((second.y - first.y - select_row_height(&theme)).abs() < f32::EPSILON, "rows are exactly one row pitch apart");
    assert!(select_row_rect(1.0, 0, menu_top, &theme).w >= 0.0, "a narrower-than-its-own-inset trigger clamps to a non-negative width");
}

//#region 🔖️AvailableHeight
// 📏️ W2k: React clamps the popup to `availableHeight` and scrolls inside it
// (`🟦️.tsx:255-258`, `:618`, `:631`, `:676`), and mounts both scroll buttons whose press moves the
// viewport by `max(24, floor(clientHeight * 0.8))` (`:769-775`). None of that existed on wgpu.

#[semio_framework_async_macros::async_test]
async fn the_available_height_is_the_room_on_the_side_the_popup_settled_on() {
    let theme = Theme::light();
    let menu_h = select_menu_height(3, &theme);
    let below = select_available_height(100.0, theme.control_height, menu_h, 800.0);
    assert!((below - (800.0 - SELECT_COLLISION_PADDING - (100.0 + theme.control_height) - SELECT_SIDE_OFFSET)).abs() < 0.001);
    let tall = select_menu_height(40, &theme);
    let flipped = select_available_height(700.0, theme.control_height, tall, 800.0);
    assert!((flipped - (700.0 - SELECT_COLLISION_PADDING - SELECT_SIDE_OFFSET)).abs() < 0.001, "a flipped popup measures the room ABOVE");
    assert_eq!(select_available_height(100.0, theme.control_height, menu_h, 0.0), f32::INFINITY, "an unmeasured viewport must not clamp the popup to nothing");
}

#[semio_framework_async_macros::async_test]
async fn a_long_popup_is_clamped_to_the_available_height_and_a_short_one_is_not() {
    let theme = Theme::light();
    let natural = select_menu_height(40, &theme);
    let painted = select_menu_painted_height(40, &theme, 100.0, theme.control_height, 400.0);
    assert!(painted < natural, "40 rows cannot fit a 400px surface");
    assert!((painted - select_available_height(100.0, theme.control_height, natural, 400.0)).abs() < 0.001);
    let short = select_menu_painted_height(2, &theme, 100.0, theme.control_height, 800.0);
    assert!((short - select_menu_height(2, &theme)).abs() < f32::EPSILON, "a popup that fits keeps its natural height");
    assert!((select_menu_painted_height(40, &theme, 100.0, theme.control_height, 0.0) - natural).abs() < f32::EPSILON, "unmeasured means unclamped");
}

#[semio_framework_async_macros::async_test]
async fn the_visible_row_count_follows_the_painted_height_and_decides_whether_it_scrolls() {
    let theme = Theme::light();
    let full = select_menu_height(4, &theme);
    assert_eq!(select_visible_rows(4, &theme, full), 4, "a popup at its natural height shows every row");
    let clipped = theme.padding_standard * 2.0 + select_row_height(&theme) * 2.5;
    assert_eq!(select_visible_rows(4, &theme, clipped), 2, "a partial row is not a visible row");
    assert_eq!(select_visible_rows(4, &theme, 0.0), 0);
    assert_eq!(select_visible_rows(2, &theme, full), 2, "the count never exceeds the item count");
}

#[semio_framework_async_macros::async_test]
async fn a_scroll_button_moves_the_viewport_by_reacts_own_step_and_never_past_the_content() {
    let theme = Theme::light();
    assert_eq!(select_scroll_step(10.0), SELECT_SCROLL_MIN_STEP, "React floors the step at 24px");
    assert_eq!(select_scroll_step(100.0), 80.0);
    assert_eq!(select_scroll_step(101.0), 80.0, "the step is floored, not rounded");
    let painted = theme.padding_standard * 2.0 + select_row_height(&theme) * 2.0;
    let extent = select_row_height(&theme) * 8.0 - select_row_height(&theme) * 2.0;
    assert_eq!(select_clamped_scroll(8, &theme, painted, -10.0), 0.0);
    assert!((select_clamped_scroll(8, &theme, painted, 10_000.0) - extent).abs() < 0.001);
    assert_eq!(select_clamped_scroll(2, &theme, painted, 10_000.0), 0.0, "a popup that fits cannot scroll");
}

#[semio_framework_async_macros::async_test]
async fn a_scroll_button_is_one_tiny_chevron_between_two_single_paddings() {
    let theme = Theme::light();
    assert!((select_scroll_button_height(&theme) - (crate::wgpu::chrome::SIZE_TINY + theme.padding_standard * 2.0)).abs() < f32::EPSILON);
}

#[semio_framework_async_macros::async_test]
async fn the_popups_inline_edge_mirrors_under_rtl_exactly_as_reacts_own_positioner_does() {
    let trigger = ui_contract::OverlayRect::new(100.0, 0.0, 60.0, 22.4);
    assert_eq!(ui_contract::resolve_select_inline_left(trigger, 30.0, ui_contract::OverlayAlign::Start, ui_contract::FlowInline::Ltr), 100.0);
    assert_eq!(ui_contract::resolve_select_inline_left(trigger, 30.0, ui_contract::OverlayAlign::Start, ui_contract::FlowInline::Rtl), 130.0);
}
//#endregion 🔖️AvailableHeight
