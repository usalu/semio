// #region chrome
//! 🎛️ Bordered chrome primitives shared by widgets and shell renderers.

use crate::wgpu::draw::DrawList;
use crate::wgpu::draw::IconAtlas;
use crate::wgpu::geometry::Rect;
use crate::wgpu::text::FontAtlas;
use crate::wgpu::theme::{Rgba, Theme};

/// 🔣️ Inline icon box inside a control (button/toggle/select/tree chevron) — React paints the same
/// icons with `size-small` (`calc(5 × --ui-spacing)` = 16px), so this reads
/// `chrome.iconInlineUiSpacing` rather than carrying its own px literal.
pub const ICON_TINY: f32 = (ui_styling::metrics::chrome::UI_SPACING_COMPACT_PX * ui_styling::metrics::chrome::ICON_INLINE_UI_SPACING) as f32;

/// 📐️ CSS's `--size-tiny` (`calc(3 × --ui-spacing)` = 9.6px), the step BELOW the `size-small` box
/// inline control icons get. React draws a `Tree` row's fold toggle at this size
/// (`🌳️Tree/🟦️.tsx:4576`) and sizes a `Progress` track with it (`h-tiny`,
/// `🗣️Interpreter/🟦️.tsx:2114`); section/group/property headers keep [`ICON_TINY`]
/// (`🌳️Tree/🟦️.tsx:251, 2182, 2609, 4401`).
pub const SIZE_TINY: f32 = (ui_styling::metrics::chrome::UI_SPACING_COMPACT_PX * ui_styling::metrics::chrome::SIZE_TINY_UI_SPACING) as f32;

/// 🌳️ A `Tree` row's own leading icon and its row actions — the Interpreter asks for a literal
/// `12` there (`🗣️Interpreter/🟦️.tsx:251, 1861, 1874`), which is `dom.iconTinyUiSpacing`'s px value
/// and NOT the `size-small` box inline control icons use.
pub const ICON_TREE_ROW: f32 = (ui_styling::metrics::chrome::UI_SPACING_COMPACT_PX * ui_styling::metrics::dom::ICON_TINY_UI_SPACING) as f32;

/// 🫥️ Re-export of the "paint nothing" identity — see [`Rgba::TRANSPARENT`].
pub const TRANSPARENT: Rgba = Rgba::TRANSPARENT;

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
        if hovered {
            theme.accent_hover
        } else {
            theme.selected
        }
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

/// 🎨️ Secondary ink on a hover or active fill. Element gray and muted gray are the hover fill,
/// so a label, icon, or description that keeps its resting color disappears.
pub fn foreground_on_fill(theme: &Theme, resting: Rgba, active: bool, hovered: bool) -> Rgba {
    if active {
        theme.active_foreground
    } else if hovered {
        theme.border_emphasized
    } else {
        resting
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
//#region 🎙️DriverTooltips
/// 🎙️ React's `UiDriverLabels` axis — whether chrome paints icon+label captions or icons only
/// (`🧱️elements/🚗️UiDriver/🟦️.tsx:15`).
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum UiDriverLabels {
    #[default]
    Full,
    Icons,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum UiDriverLabelTier {
    Beginner,
    #[default]
    Normal,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum UiDriverDrag {
    #[default]
    Handle,
    Surface,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum UiDriverReveal {
    #[default]
    Always,
    Hover,
}

/// 🎙️ React's `UiDriverTooltips` axis — how rich a chrome control's hover tooltip may be
/// (`🧱️elements/🚗️UiDriver/🟦️.tsx:23`). `None` is what `COMPACT_UI_DRIVER` ships, and it is the axis
/// value `useControlTooltipText` reads FIRST (`🏷️Label/🟦️.tsx:188`: `if (driver.tooltips === "none")
/// return undefined`).
///
/// `Full` and `Minimal` differ on React only by the manual/tutorial links a full tooltip adds; this
/// target's tooltip surface paints a label (plus its declared shortcut) and nothing else, so the two
/// tiers are behaviourally identical HERE and only `None` changes what the user sees. That is the
/// whole of the axis a canvas can honour, and it was unhonoured entirely — zero `UiDriver` hits
/// anywhere in this target — until ticket 26/09/17 packet W15a.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum UiDriverTooltips {
    #[default]
    Full,
    Minimal,
    None,
}

impl UiDriverTooltips {
    /// 🎙️ `parseUiDriver`'s own allowed values (`🚗️UiDriver/🟦️.tsx:69`); an unknown string is not a
    /// driver axis value, and React throws on it — a renderer cannot, so it keeps the default.
    pub fn from_axis(value: &str) -> Option<Self> {
        match value {
            "full" => Some(UiDriverTooltips::Full),
            "minimal" => Some(UiDriverTooltips::Minimal),
            "none" => Some(UiDriverTooltips::None),
            _ => None,
        }
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum UiDriverHotkeys {
    #[default]
    Inline,
    Tooltip,
    None,
}

/// 🚗️ The seven axes of a resolved `UiDriver`. `resolveUiDriver`
/// (`🚗️UiDriver/🟦️.tsx:80-84`) resolves a custom driver first, then a builtin, then
/// `DEFAULT_UI_DRIVER` — [`UiDriverChrome::builtin`] is the builtin half of that ladder.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct UiDriverChrome {
    pub labels: UiDriverLabels,
    pub label_tier: UiDriverLabelTier,
    pub drag: UiDriverDrag,
    pub chrome: UiDriverReveal,
    pub gumball: UiDriverReveal,
    pub tooltips: UiDriverTooltips,
    pub hotkeys: UiDriverHotkeys,
}

impl UiDriverChrome {
    /// 🚗️ `DEFAULT_UI_DRIVER` (`🚗️UiDriver/🟦️.tsx:41`).
    pub const DEFAULT: Self =
        Self { labels: UiDriverLabels::Full, label_tier: UiDriverLabelTier::Normal, drag: UiDriverDrag::Handle, chrome: UiDriverReveal::Always, gumball: UiDriverReveal::Always, tooltips: UiDriverTooltips::Full, hotkeys: UiDriverHotkeys::Inline };
    /// 🚗️ `COMPACT_UI_DRIVER` (`🚗️UiDriver/🟦️.tsx:43`) — icon-only chrome and NO tooltips.
    pub const COMPACT: Self =
        Self { labels: UiDriverLabels::Icons, label_tier: UiDriverLabelTier::Normal, drag: UiDriverDrag::Surface, chrome: UiDriverReveal::Hover, gumball: UiDriverReveal::Hover, tooltips: UiDriverTooltips::None, hotkeys: UiDriverHotkeys::None };

    /// 🚗️ The builtin driver named by `id`, falling back to the default exactly as
    /// `resolveUiDriver` does for an id no driver carries.
    pub fn builtin(id: &str) -> Self {
        match id {
            "compact" => Self::COMPACT,
            _ => Self::DEFAULT,
        }
    }

    /// 🚗️ Overrides every axis declared by the shared driver document.
    pub fn with_axes(mut self, labels: Option<&str>, label_tier: Option<&str>, drag: Option<&str>, chrome: Option<&str>, gumball: Option<&str>, tooltips: Option<&str>, hotkeys: Option<&str>) -> Self {
        if let Some("icons") = labels {
            self.labels = UiDriverLabels::Icons;
        } else if let Some("full") = labels {
            self.labels = UiDriverLabels::Full;
        }
        if let Some(value) = label_tier {
            self.label_tier = match value {
                "beginner" => UiDriverLabelTier::Beginner,
                "normal" => UiDriverLabelTier::Normal,
                _ => self.label_tier,
            };
        }
        if let Some(value) = drag {
            self.drag = match value {
                "handle" => UiDriverDrag::Handle,
                "surface" => UiDriverDrag::Surface,
                _ => self.drag,
            };
        }
        if let Some(value) = chrome {
            self.chrome = match value {
                "always" => UiDriverReveal::Always,
                "hover" => UiDriverReveal::Hover,
                _ => self.chrome,
            };
        }
        if let Some(value) = gumball {
            self.gumball = match value {
                "always" => UiDriverReveal::Always,
                "hover" => UiDriverReveal::Hover,
                _ => self.gumball,
            };
        }
        if let Some(tooltips) = tooltips.and_then(UiDriverTooltips::from_axis) {
            self.tooltips = tooltips;
        }
        if let Some(value) = hotkeys {
            self.hotkeys = match value {
                "tooltip" => UiDriverHotkeys::Tooltip,
                "none" => UiDriverHotkeys::None,
                "inline" => UiDriverHotkeys::Inline,
                _ => self.hotkeys,
            };
        }
        self
    }

    /// 💡️ Whether a chrome control with `label_visible` inline caption gets a hover tooltip at all —
    /// the two guards `useControlTooltipText` applies before it composes any text
    /// (`🏷️Label/🟦️.tsx:186-190`): a driver with `tooltips: "none"` never shows one, and a control
    /// whose caption is ALREADY painted beside its icon does not repeat itself in a tooltip.
    pub fn tooltip_shows(&self, label_visible: bool) -> bool {
        if matches!(self.tooltips, UiDriverTooltips::None) {
            return false;
        }
        !(label_visible && matches!(self.labels, UiDriverLabels::Full))
    }
}
//#endregion 🎙️DriverTooltips

#[cfg(test)]
mod foreground_on_fill_tests {
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
}

// #endregion chrome
