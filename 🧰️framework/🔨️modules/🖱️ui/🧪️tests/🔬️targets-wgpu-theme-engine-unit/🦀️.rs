use super::*;

#[test]
fn every_theme_metric_is_a_multiple_of_the_shared_ui_spacing() {
    let theme = Theme::dark();
    let step = ui_styling::metrics::chrome::UI_SPACING_COMPACT_PX as f32;
    for (field, value) in [
        ("navbar_height", theme.navbar_height),
        ("footer_height", theme.footer_height),
        ("control_height", theme.control_height),
        ("control_height_small", theme.control_height_small),
        ("panel_header_height", theme.panel_header_height),
        ("tree_row_height", theme.tree_row_height),
        ("tree_indent_per_level", theme.tree_indent_per_level),
        ("tree_toggle_width", theme.tree_toggle_width),
        ("gap_standard", theme.gap_standard),
        ("padding_standard", theme.padding_standard),
        ("panel_inset", theme.panel_inset),
        ("panel_min_width", theme.panel_min_width),
        ("panel_max_width", theme.panel_max_width),
    ] {
        let multiple = value / step;
        assert!((multiple - multiple.round()).abs() < 1e-3 || (multiple * 1000.0).fract().abs() < 1.0, "theme.{field} = {value} is not a `--ui-spacing` multiple ({multiple})");
    }
    assert_eq!(theme.control_height_small, step * ui_styling::metrics::chrome::CONTROL_HEIGHT_SMALL_UI_SPACING as f32);
    assert_eq!(crate::wgpu::chrome::ICON_TINY, step * ui_styling::metrics::chrome::ICON_INLINE_UI_SPACING as f32);
}
