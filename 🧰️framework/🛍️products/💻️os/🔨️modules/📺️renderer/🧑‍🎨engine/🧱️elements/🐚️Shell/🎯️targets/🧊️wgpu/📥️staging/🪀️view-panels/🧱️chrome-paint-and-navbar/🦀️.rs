//! 🪀️ `shell-view-panels` staging of `chrome-paint-and-navbar`: RENDERER-WP4 steps S0 and T1, the part of the former chunk that plan Appendix B and C (re-homed by the call graph) assign to this crate.
//!
//! Members of tier-1 crates are methods of their component struct with the lower components as parameters; use-case members stay `impl ShellState` and address the components as `self.<component>.<field>`; a private item that crossed a module boundary was widened to `pub(super)`.

use super::*;

/// 🪟️ Centers the localized Mode notice using the same line boxes as the React empty dock.
pub fn empty_dock_notice_lines(atlas: &mut FontAtlas, bounds: Rect, theme: &Theme, german: bool) -> Vec<(std::ops::Range<usize>, Rect)> {
    let text = shell_chrome_string("display.emptyShell", german);
    let lines = atlas.wrap_lines(text, bounds.w, theme.font_size_body);
    let height = semio_ui_text::line_height(theme.font_size_body);
    let top = bounds.y + (bounds.h - height * lines.len() as f32) * 0.5;
    lines.into_iter().enumerate().map(|(index, mut range)| {
        range.end = range.start + text[range.clone()].trim_end_matches(semio_ui_text::is_wrap_space).len();
        let width = atlas.measure_text(&text[range.clone()], theme.font_size_body).0;
        (range, Rect::new(bounds.x + (bounds.w - width) * 0.5, top + index as f32 * height + (height - theme.font_size_body) * 0.5, width, theme.font_size_body))
    }).collect()
}

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct ShellFooterChromeLayout {
    pub center: ShellChromeCenteredBand,
    pub presence: Option<Rect>,
    pub hub: Rect,
    pub sign_in: Option<Rect>,
    pub bottom_right_left: f32,
}

#[allow(clippy::too_many_arguments, reason = "one retained footer status item")]
/// 👥️ React `PresenceBar` in the footer: one shared-border row of square cells, an avatar per visible peer (initials inside a ring in the peer's presence colour, a robot badge bottom-right for an agent, the activity badge top-right) and the `+N` chip past the cap.
/// The whole row is one hit, `s-presence-peers`.
pub fn paint_footer_presence_cells(draw: &mut DrawList, atlas: &mut FontAtlas, icons: &IconAtlas, input: &mut InputState<ActionDescriptor>, theme: &Theme, rect: Rect, cells: &[semio_ui_wgpu_light::presence_bar::PresenceCell]) {
    use semio_ui_wgpu_light::presence_bar::PresenceCell;
    let hair = theme.stroke_hairline;
    let cell_w = if cells.is_empty() { 0.0 } else { (rect.w - hair * (cells.len() + 1) as f32) / cells.len() as f32 };
    let size = theme.font_size_small;
    let ring = hair * 2.0;
    draw.push_solid([rect.x, rect.y, rect.w, rect.h], theme.input_bg);
    for edge in [[rect.x, rect.y, rect.w, hair], [rect.x, rect.y + rect.h - hair, rect.w, hair], [rect.x, rect.y, hair, rect.h], [rect.x + rect.w - hair, rect.y, hair, rect.h]] {
        draw.push_solid(edge, theme.border_normal);
    }
    for (index, cell) in cells.iter().enumerate() {
        let x = rect.x + hair + index as f32 * (cell_w + hair);
        if index > 0 {
            draw.push_solid([x - hair, rect.y, hair, rect.h], theme.border_normal);
        }
        let inner = Rect::new(x, rect.y + hair, cell_w, rect.h - hair * 2.0);
        let center = |text: &str, atlas: &mut FontAtlas, font: f32| (inner.x + (inner.w - atlas.measure_text(text, font).0) * 0.5, inner.y + (inner.h + font) * 0.5 - 1.0);
        match cell {
            PresenceCell::Peer { initials, color, is_agent, badge, .. } => {
                let diameter = (inner.w.min(inner.h) - theme.gap_standard).max(0.0);
                let disc = Rect::new(inner.x + (inner.w - diameter) * 0.5, inner.y + (inner.h - diameter) * 0.5, diameter, diameter);
                draw.push_rounded([disc.x, disc.y, disc.w, disc.h], theme.presence_color(*color), diameter * 0.5);
                draw.push_rounded([disc.x + ring, disc.y + ring, (disc.w - ring * 2.0).max(0.0), (disc.h - ring * 2.0).max(0.0)], theme.button, (diameter * 0.5 - ring).max(0.0));
                let (tx, ty) = center(initials, atlas, size);
                chrome_text(draw, atlas, input, theme, initials, tx, ty, size, theme.text);
                let badge_d = inner.h * 0.4;
                if *is_agent {
                    let badge_rect = Rect::new(inner.x + inner.w - badge_d - hair, inner.y + inner.h - badge_d - hair, badge_d, badge_d);
                    draw.push_rounded([badge_rect.x, badge_rect.y, badge_rect.w, badge_rect.h], theme.presence_color(*color), badge_d * 0.5);
                    draw.push_rounded([badge_rect.x + hair, badge_rect.y + hair, badge_rect.w - hair * 2.0, badge_rect.h - hair * 2.0], theme.input_bg, badge_d * 0.5);
                    chrome_icon(draw, icons, "bot", badge_rect.x + hair, badge_rect.y + hair, (badge_d - hair * 2.0).max(0.0), theme.text);
                }
                if let Some(badge) = badge {
                    let badge_rect = Rect::new(inner.x + inner.w - badge_d - hair, inner.y + hair, badge_d, badge_d);
                    draw.push_rounded([badge_rect.x, badge_rect.y, badge_rect.w, badge_rect.h], theme.presence_color(*color), badge_d * 0.5);
                    draw.push_rounded([badge_rect.x + hair, badge_rect.y + hair, badge_rect.w - hair * 2.0, badge_rect.h - hair * 2.0], theme.input_bg, badge_d * 0.5);
                    let badge_size = badge_d * 0.7;
                    let bx = badge_rect.x + (badge_rect.w - atlas.measure_text(badge, badge_size).0) * 0.5;
                    chrome_text(draw, atlas, input, theme, badge, bx, badge_rect.y + (badge_rect.h + badge_size) * 0.5 - 1.0, badge_size, theme.text);
                }
            }
            PresenceCell::Overflow { count } => {
                let text = format!("+{count}");
                let (tx, ty) = center(&text, atlas, size);
                chrome_text(draw, atlas, input, theme, &text, tx, ty, size, theme.text_muted);
            }
        }
    }
    input.register_hit(HitTarget { rect, event: None, control_id: Some("s-presence-peers".to_string()), kind: HitKind::Button, drag_axis: None, drag_data: None });
}

pub fn render_footer_status_step(
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

