//! 📏️ Canonical typography and chrome metrics shared by all UI consumers.
/// 🪜️ The `(font size, line box)` ramp React's CSS declares as `--text-*` / `--text-*--line-height`
/// (`🎨️styling/🖌️ui/🎨️.css`), read from the ONE generated token source both targets share
/// (`ui_styling::metrics::typography`) — never a hand-picked multiplier. Ratios are
/// `1.5 / 1.4286 / 1.5 / 1.5556 / 1.6`, so a single hardcoded factor cannot express them.
const LINE_HEIGHT_RAMP: [(f32, f32); 6] = [
    (ui_styling::metrics::typography::TEXT2XS_PX as f32, ui_styling::metrics::typography::TEXT2XS_LINE_HEIGHT_PX as f32),
    (ui_styling::metrics::typography::TEXT_XS_PX as f32, ui_styling::metrics::typography::TEXT_XS_LINE_HEIGHT_PX as f32),
    (ui_styling::metrics::typography::TEXT_SM_PX as f32, ui_styling::metrics::typography::TEXT_SM_LINE_HEIGHT_PX as f32),
    (ui_styling::metrics::typography::TEXT_BASE_PX as f32, ui_styling::metrics::typography::TEXT_BASE_LINE_HEIGHT_PX as f32),
    (ui_styling::metrics::typography::TEXT_LG_PX as f32, ui_styling::metrics::typography::TEXT_LG_LINE_HEIGHT_PX as f32),
    (ui_styling::metrics::typography::TEXT2XL_PX as f32, ui_styling::metrics::typography::TEXT2XL_LINE_HEIGHT_PX as f32),
];

/// 🧮️ The line box `size` occupies — the ONE formula wrapped and single-line text both go through
/// (`measure_text`, `measure_text_wrapped`, `paint`'s per-line baseline advance,
/// `mounted_layout`'s glyph preview), so wgpu can no longer disagree with itself the way the
/// pre-parity `size * 1.35` (wrapped) versus raw-glyph-bbox (single line) pair did. A token size
/// resolves to exactly React's computed line box; anything between ramp steps borrows the nearest
/// step's ratio, matching how a browser applies the inherited `--text-*--line-height` unitless
/// ratio to whatever `font-size` an element actually ends up with.
pub fn line_height(size: f32) -> f32 {
    let mut ratio = LINE_HEIGHT_RAMP[0].1 / LINE_HEIGHT_RAMP[0].0;
    let mut best = f32::INFINITY;
    for (font_px, line_px) in LINE_HEIGHT_RAMP {
        let distance = (size - font_px).abs();
        if distance < best {
            best = distance;
            ratio = line_px / font_px;
        }
    }
    size * ratio
}

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


pub const TREE_ROW_HEIGHT: f32 = (ui_styling::metrics::chrome::UI_SPACING_COMPACT_PX * ui_styling::metrics::dom::TREE_ROW_UI_SPACING) as f32;
