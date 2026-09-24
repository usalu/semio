use super::foreground_on_fill;
use crate::wgpu::theme::Theme;

#[test]
fn hover_fill_lifts_resting_gray_to_emphasized_ink() {
    let theme = Theme::light();
    assert_eq!(foreground_on_fill(&theme, theme.text_muted, false, false), theme.text_muted);
    assert_eq!(foreground_on_fill(&theme, theme.text_element, false, true), theme.border_emphasized);
    assert_eq!(foreground_on_fill(&theme, theme.text_muted, true, true), theme.active_foreground);
    assert_ne!(theme.text_muted, theme.border_emphasized);
    assert_ne!(theme.text_element, theme.border_emphasized);
}
