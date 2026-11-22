//! 🪜️ wgpu Stepper chrome and value text shared by the retained painter and the event router.

use semio_ui_theme::geometry::Rect;
use semio_ui_retained_tree::tree::NodeFlags;
use semio_ui_wgpu_layout::chrome::{push_control_border, push_icon, ICON_TINY};
use semio_ui_draw_contract::{DrawList, IconAtlas};
use semio_ui_theme::Theme;

/// 🎯️ A stepper value as every wgpu surface prints it: its display value (`stored × display_factor`) at its precision when
/// it has one (the contract's `format_ui_number_fixed`, React's `toFixed` twin), otherwise twelve significant digits.
pub fn stepper_value_text(value: f64, precision: Option<u16>, display_factor: Option<f64>) -> String {
    ui_contract::ui_number_display_text(value, display_factor, precision)
}

/// 🪜️ React Stepper outer chrome and the two logical side-button dividers.
pub fn push_stepper_chrome(draw: &mut DrawList, bounds: Rect, segments: [Rect; 3], inline: ui_contract::FlowInline, flags: NodeFlags, hovered_segment: Option<i8>, theme: &Theme) {
    let border = if flags.contains(NodeFlags::FOCUSED) { theme.selected } else { theme.border_normal };
    push_control_border(draw, bounds, theme, border, theme.input_bg, if flags.contains(NodeFlags::FOCUSED) { semio_ui_wgpu_layout::chrome::SharedBorderState::Focus } else { semio_ui_wgpu_layout::chrome::SharedBorderState::Normal });
    let [minus, _, plus] = segments;
    if hovered_segment == Some(-1) {
        draw.push_solid([minus.x, minus.y, minus.w, minus.h], theme.button_hover);
    } else if hovered_segment == Some(1) {
        draw.push_solid([plus.x, plus.y, plus.w, plus.h], theme.button_hover);
    }
    let hair = theme.stroke_hairline;
    for (segment, sign) in [(minus, -1), (plus, 1)] {
        let hovered = hovered_segment == Some(sign);
        let state = if hovered { semio_ui_wgpu_layout::chrome::SharedBorderState::Hover } else { semio_ui_wgpu_layout::chrome::SharedBorderState::Normal };
        let left = (sign == -1) == inline.is_rtl();
        draw.push_shared_border_edges(segment, hair, if hovered { theme.border_emphasized } else { theme.border_normal }, state, [false, !left, false, left]);
    }
}

/// ➕️ Catalog glyph centered in the button content box after its inline divider.
pub fn push_stepper_icon(draw: &mut DrawList, icons: Option<&IconAtlas>, id: &str, bounds: Rect, inline: ui_contract::FlowInline, theme: &Theme) {
    if let Some(icons) = icons {
        let inset = if (id == "minus") != inline.is_rtl() { -theme.stroke_hairline } else { theme.stroke_hairline };
        push_icon(draw, icons, id, bounds.x + (bounds.w - ICON_TINY + inset) * 0.5, bounds.y + (bounds.h - ICON_TINY) * 0.5, ICON_TINY, theme.text);
    }
}
