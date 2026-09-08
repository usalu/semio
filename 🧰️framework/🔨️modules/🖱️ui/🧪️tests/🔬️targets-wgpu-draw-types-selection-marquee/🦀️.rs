
use super::*;
use crate::wgpu::theme::Theme;

#[test]
fn dashed_line_segments_emit_dashes_along_segment() {
    let segments = dashed_line_segments(0.0, 0.0, 20.0, 0.0, 5.0, 4.0);
    assert!(!segments.is_empty());
    let span: f32 = segments.iter().map(|(x0, _, x1, _)| x1 - x0).sum();
    assert!(span > 0.0 && span <= 20.0);
}

#[test]
fn selection_marquee_colors_use_active_token_only() {
    let theme = Theme::default();
    assert_eq!(selection_marquee_stroke(&theme), theme.selected);
    assert_eq!(selection_marquee_fill(&theme).a, SELECTION_MARQUEE_FILL_ALPHA);
}
