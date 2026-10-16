//! 🪀️ `shell-view-panels` staging of `chrome-paint-and-navbar`: RENDERER-WP4 steps S0 and T1, the part of the former chunk that plan Appendix B and C (re-homed by the call graph) assign to this crate.
//!
//! Members of tier-1 crates are methods of their component struct with the lower components as parameters; use-case members stay `impl ShellState` and address the components as `self.<component>.<field>`; a private item that crossed a module boundary was widened to `pub(super)`.

use super::*;

/// 🪟️ Centers the localized Mode notice using the same line boxes as the React empty dock.
pub(crate) fn empty_dock_notice_lines(atlas: &mut FontAtlas, bounds: Rect, theme: &Theme, german: bool) -> Vec<(std::ops::Range<usize>, Rect)> {
    let text = shell_chrome_string("display.emptyShell", german);
    let lines = atlas.wrap_lines(text, bounds.w, theme.font_size_body);
    let height = ui_wgpu::wgpu::text::line_height(theme.font_size_body);
    let top = bounds.y + (bounds.h - height * lines.len() as f32) * 0.5;
    lines.into_iter().enumerate().map(|(index, mut range)| {
        range.end = range.start + text[range.clone()].trim_end_matches(ui_wgpu::wgpu::text::is_wrap_space).len();
        let width = atlas.measure_text(&text[range.clone()], theme.font_size_body).0;
        (range, Rect::new(bounds.x + (bounds.w - width) * 0.5, top + index as f32 * height + (height - theme.font_size_body) * 0.5, width, theme.font_size_body))
    }).collect()
}

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub(crate) struct ShellFooterChromeLayout {
    pub(crate) center: ShellChromeCenteredBand,
    pub(crate) presence: Option<Rect>,
    pub(crate) hub: Rect,
    pub(crate) sign_in: Option<Rect>,
    pub(crate) bottom_right_left: f32,
}

#[allow(clippy::too_many_arguments, reason = "one retained footer status item")]
pub(crate) fn render_footer_status_step(
    cursor: &mut ShellChromeChildCursor,
    draw: &mut DrawList,
    atlas: &mut FontAtlas,
    icons: &IconAtlas,
    input: &mut InputState<ActionDescriptor>,
    theme: &Theme,
    rect: Rect,
    control_id: &str,
    icon_id: Option<&str>,
    label: &str,
    interactive: bool,
) -> Result<bool, ()> {
    let item = ChromeGroupItem { control_id, icon_id, label: Some(label), active: false, disabled: false, kind: HitKind::Button };
    match render_retained_chrome_group_item_step(&mut cursor.group_phase, &mut cursor.glyph, draw, atlas, icons, input, theme, rect, &item, interactive) {
        RetainedChromeGroupStep::Pending => Ok(false),
        RetainedChromeGroupStep::Fault => Err(()),
        RetainedChromeGroupStep::Complete => Ok(true),
    }
}

