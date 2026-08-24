// #region chrome
//! 🎛️ Bordered chrome primitives shared by widgets and shell renderers.

use crate::wgpu::draw::DrawList;
use crate::wgpu::draw::IconAtlas;
use crate::wgpu::geometry::Rect;
use crate::wgpu::text::FontAtlas;
use crate::wgpu::theme::{Rgba, Theme};

pub const ICON_TINY: f32 = 14.0;

pub const TRANSPARENT: Rgba = Rgba::new(0.0, 0.0, 0.0, 0.0);

pub fn push_chrome_group_border(draw: &mut DrawList, rect: Rect, theme: &Theme) {
    let hair = theme.stroke_hairline;
    push_chrome_border(draw, rect, hair, theme.border_normal, true, true, true, true);
}

#[allow(clippy::too_many_arguments, reason = "one arg per border edge/style flag; grouping into a struct is a T2 restructure, out of scope")]
pub fn push_chrome_border(draw: &mut DrawList, rect: Rect, stroke: f32, color: Rgba, top: bool, right: bool, bottom: bool, left: bool) {
    if top {
        draw.push_solid([rect.x, rect.y, rect.w, stroke], color);
    }
    if bottom {
        draw.push_solid([rect.x, rect.y + rect.h - stroke, rect.w, stroke], color);
    }
    if left {
        draw.push_solid([rect.x, rect.y, stroke, rect.h], color);
    }
    if right {
        draw.push_solid([rect.x + rect.w - stroke, rect.y, stroke, rect.h], color);
    }
}

pub fn push_window_cap_border(draw: &mut DrawList, rect: Rect, stroke: f32, color: Rgba) {
    push_chrome_border(draw, rect, stroke, color, true, true, false, true);
}

pub fn push_control_border(draw: &mut DrawList, rect: Rect, theme: &Theme, border: Rgba, bg: Rgba) {
    if bg.a > 0.0 {
        draw.push_solid([rect.x, rect.y, rect.w, rect.h], bg);
    }
    let hair = theme.stroke_hairline;
    draw.push_solid([rect.x, rect.y, rect.w, hair], border);
    draw.push_solid([rect.x, rect.y + rect.h - hair, rect.w, hair], border);
    draw.push_solid([rect.x, rect.y, hair, rect.h], border);
    draw.push_solid([rect.x + rect.w - hair, rect.y, hair, rect.h], border);
}

pub fn push_icon(draw: &mut DrawList, icons: &IconAtlas, icon_id: &str, x: f32, y: f32, size: f32, color: Rgba) {
    if let Some(uv) = icons.icon_uv(icon_id) {
        draw.push_textured([x, y, size, size], uv, color);
    }
}

pub fn measure_action_item(atlas: &mut FontAtlas, theme: &Theme, icon: bool, label: Option<&str>) -> f32 {
    let icon_w = if icon { ICON_TINY + theme.gap_standard } else { 0.0 };
    let text_w = label.map_or(0.0, |value| atlas.measure_text(value, theme.font_size_small).0);
    theme.padding_standard * 2.0 + icon_w + text_w
}

pub fn chrome_item_bg(theme: &Theme, active: bool, hovered: bool) -> Rgba {
    if active {
        if hovered { theme.accent_hover } else { theme.selected }
    } else if hovered {
        theme.button_hover
    } else {
        TRANSPARENT
    }
}

pub fn chrome_item_text(theme: &Theme, active: bool, hovered: bool) -> Rgba {
    if active {
        theme.active_foreground
    } else if hovered {
        theme.border_emphasized
    } else {
        theme.text_element
    }
}

pub fn item_bg(theme: &Theme, pressed: bool, hovered: bool) -> Rgba {
    chrome_item_bg(theme, pressed, hovered)
}

pub fn item_text(theme: &Theme, pressed: bool, hovered: bool) -> Rgba {
    chrome_item_text(theme, pressed, hovered)
}

/// 👁️✏️ Ticket 26/08/16/ARTIFACT-VIEWERS-AND-EDITORS-PER-SUBSET contract §5: the read-only badge a
/// viewer session's window chrome shows — a small lock-icon chip pinned to `rect`'s top-right
/// corner. Distinct from `shell.rs`'s `build_window` role-chrome handling (which swaps the WINDOW-
/// CAP/tab icon to `IconName::Lock` — that lives one level up, in the declarative `WindowLayout`
/// vocabulary, not raw draw calls): this fn is for whoever paints a window's own content chrome
/// (e.g. a title bar inside the canvas itself) and wants the same badge there. Pure paint helper —
/// it does not decide WHEN to show the badge (`role_chrome::ChromeRole::is_read_only`'s job).
pub fn push_read_only_badge(draw: &mut DrawList, icons: &IconAtlas, theme: &Theme, rect: Rect) {
    let size = ICON_TINY;
    let margin = theme.padding_standard;
    let x = rect.x + rect.w - size - margin;
    let y = rect.y + margin;
    push_control_border(draw, Rect::new(x - margin * 0.5, y - margin * 0.5, size + margin, size + margin), theme, theme.border_normal, theme.button_hover);
    push_icon(draw, icons, "lock", x, y, size, theme.text_element);
}
// #endregion chrome
