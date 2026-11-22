use super::*;
use semio_ui_draw_contract::DrawList;

fn hover_at(draw: &mut DrawList, theme: &Theme, hovered: bool, now: f32) -> f32 {
    chrome_hover_amount(draw, theme, Rect::new(5.0, 5.0, 40.0, 20.0), hovered, now)
}

#[test]
fn hover_fades_over_the_token_duration_and_flags_the_frame_as_moving() {
    let theme = Theme::light();
    let mut draw = DrawList::default();
    assert_eq!(hover_at(&mut draw, &theme, false, 1.0), 0.0);
    assert_eq!(hover_at(&mut draw, &theme, true, 2.0), 0.0);
    assert!(draw.motion_pending());
    let middle = hover_at(&mut draw, &theme, true, 2.0 + theme.transition_duration_seconds * 0.5);
    assert!((middle - 0.775561).abs() < 1e-3);
    draw.clear();
    assert_eq!(hover_at(&mut draw, &theme, true, 2.0 + theme.transition_duration_seconds * 2.0), 1.0);
    assert!(!draw.motion_pending());
}

#[test]
fn reduced_motion_snaps_hover_and_never_flags_motion() {
    let theme = Theme::light();
    let mut draw = DrawList::default();
    draw.set_reduced_motion(true);
    assert_eq!(hover_at(&mut draw, &theme, false, 1.0), 0.0);
    assert_eq!(hover_at(&mut draw, &theme, true, 1.01), 1.0);
    assert!(!draw.motion_pending());
}

#[test]
fn the_blend_is_premultiplied_exact_at_both_ends_and_fades_a_transparent_rest() {
    let theme = Theme::light();
    assert_eq!(chrome_item_bg_at(&theme, false, 0.0), chrome_item_bg(&theme, false, false));
    assert_eq!(chrome_item_bg_at(&theme, false, 1.0), chrome_item_bg(&theme, false, true));
    assert_eq!(chrome_item_text_at(&theme, false, 1.0), chrome_item_text(&theme, false, true));
    let half = chrome_item_bg_at(&theme, false, 0.5);
    let hover = chrome_item_bg(&theme, false, true);
    assert!((half.a - hover.a * 0.5).abs() < 1e-6);
    assert!((half.r - hover.r).abs() < 1e-6, "a fade from transparent keeps the hover hue");
}
