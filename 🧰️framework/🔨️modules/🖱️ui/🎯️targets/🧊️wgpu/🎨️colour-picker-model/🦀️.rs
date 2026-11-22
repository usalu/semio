//! 🎨️ The language-neutral colour picker model: the DOM `<input type="color">` value (`#rrggbb`, lower case), the swatch palette, the hex and channel fields and the popup geometry of a `color` input.

use semio_ui_theme::geometry::Rect;

/// 🎨️ The stored value of a colour input: `#` and six lower-case hex digits, alpha dropped, as the DOM input normalises any value it is given.
pub fn colour_value(text: &str) -> Option<String> {
    let rgba = ui_contract::parse_ui_color_hex(text)?;
    Some(ui_contract::ui_color_hex(&rgba, false))
}

/// 🌈️ The channels of a stored value.
pub fn colour_channels(text: &str) -> Option<[u8; 3]> {
    let value = colour_value(text)?;
    let digit = |range: std::ops::Range<usize>| u8::from_str_radix(&value[range], 16).ok();
    Some([digit(1..3)?, digit(3..5)?, digit(5..7)?])
}

pub fn colour_from_channels(channels: [u8; 3]) -> String {
    format!("#{:02x}{:02x}{:02x}", channels[0], channels[1], channels[2])
}

/// 🔢️ A typed channel: a whole number `0..=255`; anything else is refused and the draft kept.
pub fn parse_channel(text: &str) -> Option<u8> {
    let trimmed = text.trim();
    if trimmed.is_empty() || !trimmed.bytes().all(|byte| byte.is_ascii_digit()) {
        return None;
    }
    trimmed.parse::<u8>().ok()
}

/// 🌈️ The palette of the swatch row (the colour wheel of the native picker, reduced to its primaries, secondaries and greys).
pub const COLOUR_SWATCHES: [&str; 16] = ["#000000", "#404040", "#808080", "#c0c0c0", "#ffffff", "#ff0000", "#ff8000", "#ffff00", "#80ff00", "#00ff00", "#00ff80", "#00ffff", "#0080ff", "#0000ff", "#8000ff", "#ff00ff"];

pub const COLOUR_SWATCH_COLUMNS: usize = 8;
pub const COLOUR_SWATCH_ROWS: usize = COLOUR_SWATCHES.len() / COLOUR_SWATCH_COLUMNS;

/// 🖊️ A field of the popup: how a draft of it becomes the stored value.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ColourField {
    Hex,
    Red,
    Green,
    Blue,
}

/// 🖊️ The stored value a draft of `field` makes over `current`, `None` while the draft is not (yet) a value — the field keeps its text and nothing is offered.
pub fn colour_apply_field(current: &str, field: ColourField, draft: &str) -> Option<String> {
    if field == ColourField::Hex {
        return colour_value(draft);
    }
    let mut channels = colour_channels(current)?;
    channels[match field {
        ColourField::Red => 0,
        ColourField::Green => 1,
        _ => 2,
    }] = parse_channel(draft)?;
    Some(colour_from_channels(channels))
}

/// ⌨️ Arrow keys over the swatch row; `None` keeps the focus where it is.
pub fn swatch_key(key: &str, focused: usize) -> Option<usize> {
    let last = COLOUR_SWATCHES.len() - 1;
    Some(match key {
        "ArrowLeft" => focused.saturating_sub(1),
        "ArrowRight" => (focused + 1).min(last),
        "ArrowUp" => focused.checked_sub(COLOUR_SWATCH_COLUMNS).unwrap_or(focused),
        "ArrowDown" => (focused + COLOUR_SWATCH_COLUMNS).min(last),
        "Home" => 0,
        "End" => last,
        _ => return None,
    })
}

/// 📐️ Where a colour popup's parts sit; every rect is absolute.
#[derive(Clone, Debug, PartialEq)]
pub struct ColourPopupLayout {
    pub panel: Rect,
    pub preview: Rect,
    pub swatches: [Rect; COLOUR_SWATCHES.len()],
    pub hex: Rect,
    pub channels: [Rect; 3],
}

/// 📐️ The popup below `anchor` (above it when the viewport has no room): a preview band, the swatch grid, the hex field and the three channel fields, `cell` high rows inside a `pad` frame.
pub fn colour_popup_layout(anchor: Rect, viewport_h: f32, cell: f32, pad: f32, rtl: bool) -> ColourPopupLayout {
    let width = cell * COLOUR_SWATCH_COLUMNS as f32 + pad * 2.0;
    let height = pad * 2.0 + cell * (1.0 + COLOUR_SWATCH_ROWS as f32 + 2.0);
    let below = anchor.y + anchor.h;
    let y = if below + height <= viewport_h || anchor.y < height { below } else { anchor.y - height };
    let panel = Rect::new(anchor.x, y, width, height);
    let inner = width - pad * 2.0;
    let column_x = |column: usize, span: f32| if rtl { panel.x + pad + inner - (column as f32 + 1.0) * span } else { panel.x + pad + column as f32 * span };
    let row_y = |row: usize| panel.y + pad + row as f32 * cell;
    let channel = inner / 3.0;
    ColourPopupLayout {
        panel,
        preview: Rect::new(panel.x + pad, row_y(0), inner, cell),
        swatches: std::array::from_fn(|index| Rect::new(column_x(index % COLOUR_SWATCH_COLUMNS, cell), row_y(1 + index / COLOUR_SWATCH_COLUMNS), cell, cell)),
        hex: Rect::new(panel.x + pad, row_y(1 + COLOUR_SWATCH_ROWS), inner, cell),
        channels: std::array::from_fn(|index| Rect::new(column_x(index, channel), row_y(2 + COLOUR_SWATCH_ROWS), channel, cell)),
    }
}
