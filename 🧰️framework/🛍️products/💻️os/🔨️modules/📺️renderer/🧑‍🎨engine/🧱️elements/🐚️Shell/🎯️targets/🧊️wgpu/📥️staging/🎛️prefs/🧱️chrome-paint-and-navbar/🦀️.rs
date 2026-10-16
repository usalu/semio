//! 🎛️ `shell-prefs` staging of `chrome-paint-and-navbar`: RENDERER-WP4 steps S0 and T1, the part of the former chunk that plan Appendix B and C (re-homed by the call graph) assign to this crate.
//!
//! Members of tier-1 crates are methods of their component struct with the lower components as parameters; use-case members stay `impl ShellState` and address the components as `self.<component>.<field>`; a private item that crossed a module boundary was widened to `pub(super)`.

use super::*;

pub(crate) struct ChromeGroupItem<'a> {
    pub(crate) control_id: &'a str,
    pub(crate) icon_id: Option<&'a str>,
    pub(crate) label: Option<&'a str>,
    pub(crate) active: bool,
    pub(crate) disabled: bool,
    pub(crate) kind: HitKind,
}

/// 📐️ The box one retained chrome item (navbar/footer chip, dropdown row, surface control) occupies
/// — its own padding, its optional leading icon, and its label's MEASURED paint width, read off the
/// very atlas [`render_retained_chrome_group_item_step`] then paints that label with.
///
/// ⚠️ It used to price the label at `label.len() × font_size_small × 0.6`, a per-byte guess that has
/// no relation to the advances the painter actually pens: at `--text-xs` (11.2px) the guess is
/// 6.72px per byte, while the painter walks `FontAtlas::measure_text`'s real advances (10px flat for
/// the 8×16 ASCII fallback face). Every chip therefore laid out NARROWER than its own label and
/// `paint_retained_glyph_step` broke the run mid-word inside the chip — the "Concrete F/orest",
/// "Edit/or", "Transf/orm" two-line chips of the first real wgpu boot. React's chip labels are
/// `whitespace-nowrap truncate` and their box hugs the measured text
/// (`🎛️chrome-control-presentation/🟦️.ts`), which is what measuring here restores. A multi-byte
/// scalar also no longer inflates its chip by its UTF-8 length.
pub(crate) fn retained_chrome_group_item_width(atlas: &mut FontAtlas, theme: &Theme, item: &ChromeGroupItem<'_>) -> Option<f32> {
    let label = item.label.unwrap_or("");
    if label.len() > ui_wgpu::wgpu::RETAINED_NODE_TEXT_MAX_BYTES {
        return None;
    }
    let icon_w = item.icon_id.map(|_| CHROME_ICON_TINY + theme.gap_standard).unwrap_or(0.0);
    let label_w = if label.is_empty() { 0.0 } else { atlas.measure_text(label, theme.font_size_small).0 };
    Some(theme.padding_standard * 2.0 + icon_w + label_w)
}

