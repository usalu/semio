//! 🧱️ Chrome paint steps, navbar/footer layout, utilities and window pane chrome.
//!
//! Immediate-mode chrome text and icons, group painting, bands, navbar/footer layouts, the sync pill, hub
//! projections, utility rendering, keybinding formatting, navbar controls, World3d status, the safe area and the
//! window pane chrome.
//!
//! One chunk of the former `shell` monolith (ticket CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT); [`ShellState`] and the
//! shared imports stay in the parent module and are reached through `use super::*;`.

use super::*;

/// ✍️ Immediate-mode Shell chrome text — the non-retained sibling of {@link chrome_text_step}, used by
/// the eleven overlay/dropdown/status painters below that draw a whole scalar in one pass. It was
/// `#[cfg(test)]`-gated, which made every one of those production call sites `E0425` on every target
/// (the reason this crate compiled on neither `wasm32-unknown-unknown` nor natively).
pub(crate) fn chrome_text(target: &mut DrawList, atlas: &mut FontAtlas, input: &mut InputState<ActionDescriptor>, theme: &Theme, text: &str, x: f32, y: f32, size: f32, color: Rgba) {
    let mut scroll = HashMap::new();
    let mut collapsed = HashMap::new();
    let mut selects = HashMap::new();
    let mut ctx = framework_widget_context(target, None, atlas, None, input, theme, &mut scroll, &mut collapsed, &mut selects, None, 0.0);
    draw_text(&mut ctx, text, x, y, size, color);
}

/// ✍️ Advances one Shell text scalar and at most one pre-admitted glyph. Shell chrome text flows
/// [`RetainedTextFlow::Clip`] — React's chrome labels are all `whitespace-nowrap truncate`
/// (`🎛️chrome-control-presentation/🟦️.ts`, `🏛️ShellHost/🟦️.tsx`'s chip/tab/pill labels), so a label
/// that outgrows its box loses its tail rather than dropping a second line inside a 22px chip.
fn chrome_text_step(target: &mut DrawList, atlas: &mut FontAtlas, text: &str, x: f32, y: f32, width: f32, size: f32, color: Rgba, cursor: &mut RetainedGlyphCursor) -> RetainedGlyphStep {
    paint_retained_glyph_step_flowed(text, Rect::new(x, y - size, width.max(1.0), size), size, color, RetainedTextFlow::Clip, atlas, target, cursor)
}

pub(crate) fn chrome_text_complete_step(target: &mut DrawList, atlas: &mut FontAtlas, text: &str, x: f32, y: f32, width: f32, size: f32, color: Rgba, cursor: &mut RetainedGlyphCursor) -> Result<bool, ()> {
    match chrome_text_step(target, atlas, text, x, y, width, size, color, cursor) {
        RetainedGlyphStep::Pending => Ok(false),
        RetainedGlyphStep::Complete => {
            cursor.reset();
            Ok(true)
        }
        RetainedGlyphStep::Fault => Err(()),
    }
}

/// ✍️ The WRAPPING twin of [`chrome_text_complete_step`], for the one piece of shell chrome whose copy
/// is prose rather than a chip label: the introduction card's body paragraphs, which React renders as
/// `<p class="whitespace-pre-line">` inside a `max-w-sm` box and therefore wraps. `rect` is the flow
/// box — its own top, not a baseline.
pub(crate) fn chrome_text_wrapped_complete_step(target: &mut DrawList, atlas: &mut FontAtlas, text: &str, rect: Rect, size: f32, color: Rgba, cursor: &mut RetainedGlyphCursor) -> Result<bool, ()> {
    match paint_retained_glyph_step_flowed(text, rect, size, color, RetainedTextFlow::Wrap, atlas, target, cursor) {
        RetainedGlyphStep::Pending => Ok(false),
        RetainedGlyphStep::Complete => {
            cursor.reset();
            Ok(true)
        }
        RetainedGlyphStep::Fault => Err(()),
    }
}

#[cfg(test)]
#[path = "../../../🧪️tests/🔬️wgpu-retained-chrome-text-laws/🦀️.rs"]
mod retained_chrome_text_laws;

#[cfg(test)]
#[path = "../../../🧪️tests/🔬️wgpu-shell-chrome-parity/🦀️.rs"]
mod shell_chrome_parity_tests;

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

pub(crate) fn chrome_icon(draw: &mut DrawList, icons: &IconAtlas, icon_id: &str, x: f32, y: f32, size: f32, color: Rgba) {
    if let Some(uv) = icons.icon_uv(icon_id) {
        draw.push_textured([x, y, size, size], uv, color);
    }
}

/// 🪧️ The identity tint for a FULL-COLOUR atlas cell. `UI_SHADER`'s textured branch is
/// `icon.rgb * in.color.rgb` (`🖱️ui/🎯️targets/🧊️wgpu/🎨️shaders/🦀️.rs:144`), which is what makes a
/// Lucide glyph wear `theme.text`: every Lucide cell is rasterised as a WHITE tint mask
/// (`rasterize_svg(svg, tint_mask = id != "semio-logo", …)`, `🖼️IconRenderHost/🎯️targets/🧊️wgpu/🦀️.rs:111`).
///
/// 🩸️ The brand mark is the one cell rasterised in its real colours — navy disc, orange/red/teal
/// bars — and the navbar multiplied it by `theme.text` anyway, which collapsed all four hues into the
/// text colour and painted the BLACK DISC of `📓️audit-visual-parity-puzzle3d.md` §2/§4. React paints
/// `<SemioLogo>` as an untinted SVG, so the identity is what parity means here.
pub(crate) const LOGO_UNTINTED: Rgba = Rgba::new(1.0, 1.0, 1.0, 1.0);

/// 🪧️ React's `size-workbench` on the navbar brand mark (`🏛️ShellHost/🟦️.tsx:10056`), in `--ui-spacing`
/// units: `--size-workbench` is `1.5 × --size-small` and `--size-small` is `5 × --ui-spacing`
/// (`🖱️ui/🎨️styling/🖌️ui/🎨️.css:736-737`), so the mark is 7.5 spacings — TALLER than a 7-spacing chip,
/// which is why it used to be sized off `control_height` and read visibly small.
pub(crate) const SHELL_NAVBAR_LOGO_UI_SPACING: f32 = 7.5;

#[cfg(test)]
fn chrome_group_border(draw: &mut DrawList, rect: Rect, theme: &Theme) {
    push_chrome_group_border(draw, rect, theme);
}

pub(crate) struct ChromeGroupItem<'a> {
    pub(crate) control_id: &'a str,
    pub(crate) icon_id: Option<&'a str>,
    pub(crate) label: Option<&'a str>,
    pub(crate) active: bool,
    pub(crate) disabled: bool,
    pub(crate) kind: HitKind,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum RetainedChromeGroupStep {
    Pending,
    Complete,
    Fault,
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

pub(crate) fn retained_panel_chrome_item_width(atlas: &mut FontAtlas, theme: &Theme, item: &ChromeGroupItem<'_>) -> Option<f32> {
    let label = item.label.unwrap_or("");
    if label.len() > ui_wgpu::wgpu::RETAINED_NODE_TEXT_MAX_BYTES {
        return None;
    }
    let leading = item.icon_id.map_or(0.0, |_| PANEL_CHROME_ICON_TINY + theme.gap_standard);
    let label_width = if label.is_empty() { 0.0 } else { atlas.measure_text(label, theme.font_size_small).0 };
    Some(theme.padding_standard * 2.0 + leading + label_width + theme.gap_standard + PANEL_CHROME_GRIP_SIZE)
}

/// 🧱️ Emits at most one retained chrome item scalar, glyph, hit, or border per grant.
pub(crate) fn render_retained_chrome_group_item_step(
    group_phase: &mut u8,
    glyph: &mut RetainedGlyphCursor,
    draw: &mut DrawList,
    atlas: &mut FontAtlas,
    icons: &IconAtlas,
    input: &mut InputState<ActionDescriptor>,
    theme: &Theme,
    rect: Rect,
    item: &ChromeGroupItem<'_>,
    register_hit: bool,
) -> RetainedChromeGroupStep {
    render_retained_chrome_group_item_step_with_trailing(group_phase, glyph, draw, atlas, icons, input, theme, rect, item, register_hit, CHROME_ICON_TINY, None)
}

#[allow(clippy::too_many_arguments, reason = "the retained painter's existing render context plus one optional trailing icon")]
fn render_retained_chrome_group_item_step_with_trailing(
    group_phase: &mut u8,
    glyph: &mut RetainedGlyphCursor,
    draw: &mut DrawList,
    atlas: &mut FontAtlas,
    icons: &IconAtlas,
    input: &mut InputState<ActionDescriptor>,
    theme: &Theme,
    rect: Rect,
    item: &ChromeGroupItem<'_>,
    register_hit: bool,
    leading_icon_size: f32,
    trailing_icon: Option<(&str, f32)>,
) -> RetainedChromeGroupStep {
    let hovered = !item.disabled && rect.contains(input.pointer_x, input.pointer_y);
    let color = if item.disabled { theme.text_muted } else { chrome_item_text(theme, item.active, hovered) };
    match *group_phase {
        0 => {
            let background = if item.disabled { theme.overlay_shadow } else { chrome_item_bg(theme, item.active, hovered) };
            if background.a > 0.0 {
                draw.push_solid([rect.x, rect.y, rect.w, rect.h], background);
            }
            *group_phase = 1;
        }
        1 => {
            if let Some(icon_id) = item.icon_id {
                chrome_icon(draw, icons, icon_id, rect.x + theme.padding_standard, rect.y + (rect.h - leading_icon_size) * 0.5, leading_icon_size, color);
            }
            *group_phase = 2;
        }
        2 => {
            if let Some(label) = item.label {
                if label.len() > ui_wgpu::wgpu::RETAINED_NODE_TEXT_MAX_BYTES {
                    *group_phase = 0;
                    glyph.reset();
                    return RetainedChromeGroupStep::Fault;
                }
                let icon_w = item.icon_id.map(|_| leading_icon_size + theme.gap_standard).unwrap_or(0.0);
                let x = rect.x + theme.padding_standard + icon_w;
                let trailing = trailing_icon.map_or(0.0, |(_, size)| theme.gap_standard + size);
                let width = (rect.x + rect.w - theme.padding_standard - trailing - x).max(1.0);
                match chrome_text_complete_step(draw, atlas, label, x, rect.y + (rect.h + theme.font_size_small) * 0.5 - 1.0, width, theme.font_size_small, color, glyph) {
                    Ok(false) => return RetainedChromeGroupStep::Pending,
                    Ok(true) => {}
                    Err(()) => {
                        *group_phase = 0;
                        glyph.reset();
                        return RetainedChromeGroupStep::Fault;
                    }
                }
            }
            *group_phase = 3;
        }
        3 => {
            draw.push_solid([rect.x, rect.y, rect.w, theme.stroke_hairline], theme.border_normal);
            *group_phase = 4;
        }
        4 => {
            draw.push_solid([rect.x, rect.y + rect.h - theme.stroke_hairline, rect.w, theme.stroke_hairline], theme.border_normal);
            *group_phase = 5;
        }
        5 => {
            draw.push_solid([rect.x, rect.y, theme.stroke_hairline, rect.h], theme.border_normal);
            *group_phase = 6;
        }
        6 => {
            draw.push_solid([rect.x + rect.w - theme.stroke_hairline, rect.y, theme.stroke_hairline, rect.h], theme.border_normal);
            *group_phase = 7;
        }
        7 => {
            if register_hit && !item.disabled {
                note_chrome_group_item(item);
                input.register_hit(HitTarget { rect, event: None, control_id: Some(item.control_id.into()), kind: item.kind.clone(), drag_axis: None, drag_data: None });
            }
            *group_phase = if trailing_icon.is_some() { 8 } else { 9 };
        }
        8 => {
            if let Some((icon_id, size)) = trailing_icon {
                chrome_icon(draw, icons, icon_id, rect.x + rect.w - theme.padding_standard - size, rect.y + (rect.h - size) * 0.5, size, color);
            }
            *group_phase = 9;
        }
        _ => {
            *group_phase = 0;
            glyph.reset();
            return RetainedChromeGroupStep::Complete;
        }
    }
    RetainedChromeGroupStep::Pending
}

#[allow(clippy::too_many_arguments, reason = "the retained painter's existing render context")]
pub(crate) fn render_retained_panel_chrome_item_step(
    group_phase: &mut u8,
    glyph: &mut RetainedGlyphCursor,
    draw: &mut DrawList,
    atlas: &mut FontAtlas,
    icons: &IconAtlas,
    input: &mut InputState<ActionDescriptor>,
    theme: &Theme,
    rect: Rect,
    item: &ChromeGroupItem<'_>,
    register_hit: bool,
) -> RetainedChromeGroupStep {
    render_retained_chrome_group_item_step_with_trailing(group_phase, glyph, draw, atlas, icons, input, theme, rect, item, register_hit, PANEL_CHROME_ICON_TINY, Some(("grip-vertical", PANEL_CHROME_GRIP_SIZE)))
}

#[cfg(test)]
fn measure_chrome_group_item(atlas: &mut FontAtlas, theme: &Theme, item: &ChromeGroupItem<'_>) -> f32 {
    let icon_w = item.icon_id.map(|_| CHROME_ICON_TINY + theme.gap_standard).unwrap_or(0.0);
    let text_w = item.label.map(|label| atlas.measure_text(label, theme.font_size_small).0).unwrap_or(0.0);
    theme.padding_standard * 2.0 + icon_w + text_w
}

fn floating_panel_available_width(body: Rect, theme: &Theme) -> f32 {
    (body.w - theme.panel_inset * 2.0).max(theme.panel_min_width)
}

pub(crate) fn floating_panel_max_width(body: Rect, theme: &Theme) -> f32 {
    theme.panel_max_width.min(floating_panel_available_width(body, theme)).max(theme.panel_min_width)
}

pub(crate) fn floating_panel_width(width: f32, body: Rect, theme: &Theme) -> f32 {
    width.clamp(theme.panel_min_width, floating_panel_max_width(body, theme))
}

/// 📑️ The window id behind a dock tab's LABEL target (`dock.tab.<path>.<windowId>`) — `None` for the
/// action chips (`….focus`/`….close`/`….drag`), which carry one dot more. React's label is its own
/// `<button>` beside those chips (`🖱️ui/🧱️elements/🎨️Canvas/🟦️.tsx:1049-1071`).
pub(crate) fn dock_tab_select_window_id(control_id: &str) -> Option<&str> {
    let rest = control_id.strip_prefix("dock.tab.")?;
    let (_, window_id) = rest.split_once('.')?;
    (!window_id.contains('.')).then_some(window_id)
}

/// 📑️ Whether this dock tab subtree carries `tab_id` anywhere — root row or drilled-down child.
pub(crate) fn dock_tab_node_carries(node: &DockTabNode, tab_id: &str) -> bool {
    node.id == tab_id || node.children.iter().any(|child| dock_tab_node_carries(child, tab_id))
}

/// ↔ Which anchor a `panel.resize.<anchor>.<inner|outer>` control belongs to, and whether it is the
/// inner (canvas-facing) handle — the one id shape both the registration and the drag arm parse.
pub(crate) fn panel_resize_anchor(control_id: &str) -> Option<(PanelAnchor, bool)> {
    let rest = control_id.strip_prefix("panel.resize.")?;
    let (anchor_id, side) = rest.rsplit_once('.')?;
    Some((PanelAnchor::from_str(anchor_id)?, side == "inner"))
}

/// 🧭️ Top-to-bottom order the anchors of one column band themselves in — the row order React's
/// `anchorVertical` names, used to slot an open anchor within its column.
pub(crate) const ANCHOR_COLUMN_ORDER: [PanelAnchor; 8] = [PanelAnchor::TopLeft, PanelAnchor::LeftMiddle, PanelAnchor::BottomLeft, PanelAnchor::TopMiddle, PanelAnchor::BottomMiddle, PanelAnchor::TopRight, PanelAnchor::RightMiddle, PanelAnchor::BottomRight];

/// 🧭️ One anchor's floating-panel box inside the body band, the Rust twin of React's
/// `anchorPositionStyle` (`🔨️modules/🖱️ui/🎯️targets/⚛️react/🟦️.tsx`): a left/right column insets by
/// `--spacing-single` (`theme.panel_inset`) from that edge, a middle column centres on the body, a
/// top/bottom row insets from that edge and the middle row centres — with React's
/// `calc(100% - spacing*2)` clamp applied to both axes.
///
/// 📐️ The one honest divergence: React's panel HUGS ITS CONTENT up to that clamp (`maxHeight`, never a
/// fixed `bottom`), which needs a measured retained-document height this renderer has no read of yet.
/// A column's visible anchors therefore share its height in equal bands instead — identical to the old
/// full-height single panel whenever a column carries exactly one open anchor, and never overlapping
/// when it carries two. See `📓️w1h-panel-anchors-and-command-dock.md`'s remaining gaps.
///
/// 📐️ Content hug: a measured document takes exactly its own height, clamped to React's
/// `calc(100% - spacing*2)` — and, when a column carries more than one open anchor, to that
/// anchor's share of the column so two hugging panels can never overlap. No measurement yet (the
/// surface's first frame) keeps the full band, which is what this renderer did before.
///
/// 📐️ One BONDED vertical edge, never a `top`+`bottom` pair (`anchorPositionStyle`): a top anchor
/// grows down from its own inset, a bottom anchor grows up from the body's bottom inset, and a
/// middle anchor centres on its band.
///
/// 📐️ The slack is computed BEFORE it is added to the origin: `slot_top + band - height` loses a
/// float step on a full-band panel (693.6 added then subtracted), which is visible as a one-thousandth
/// drift off the declared inset.
pub(crate) fn anchor_panel_rect(anchor: PanelAnchor, size: f32, content_h: Option<f32>, column_slots: usize, column_index: usize, body: Rect, theme: &Theme) -> Rect {
    let inset = theme.panel_inset;
    let width = floating_panel_width(size, body, theme);
    let available_h = (body.h - inset * 2.0).max(0.0);
    let slots = column_slots.max(1) as f32;
    let gap = if column_slots > 1 { theme.gap_standard } else { 0.0 };
    let band = ((available_h - gap * (slots - 1.0)) / slots).max(0.0);
    let height = content_h.map_or(band, |content| content.clamp(0.0, band));
    let x = match anchor.horizontal() {
        "left" => body.x + inset,
        "right" => body.x + body.w - inset - width,
        _ => body.x + (body.w - width) * 0.5,
    };
    let slot_top = body.y + inset + (band + gap) * column_index as f32;
    let slack = (band - height).max(0.0);
    let y = match anchor.vertical() {
        "top" => slot_top,
        "bottom" => slot_top + slack,
        _ => slot_top + slack * 0.5,
    };
    Rect::new(x, y, width, height)
}

#[cfg(test)]
fn render_chrome_group(draw: &mut DrawList, atlas: &mut FontAtlas, icons: &IconAtlas, input: &mut InputState<ActionDescriptor>, theme: &Theme, rect: Rect, items: &[ChromeGroupItem<'_>], register_hits: bool) {
    if items.is_empty() {
        return;
    }
    let hair = theme.stroke_hairline;
    let inner_y = rect.y + hair;
    let inner_h = (rect.h - hair * 2.0).max(0.0);
    let mut x = rect.x;
    for (index, item) in items.iter().enumerate() {
        let item_w = measure_chrome_group_item(atlas, theme, item);
        let item_rect = Rect::new(x, inner_y, item_w, inner_h);
        let hovered = !item.disabled && item_rect.contains(input.pointer_x, input.pointer_y);
        let bg = if item.disabled { theme.overlay_shadow } else { chrome_item_bg(theme, item.active, hovered) };
        if bg.a > 0.0 {
            draw.push_solid([item_rect.x, item_rect.y, item_rect.w, item_rect.h], bg);
        }
        let text_color = if item.disabled { theme.text_muted } else { chrome_item_text(theme, item.active, hovered) };
        let mut content_x = item_rect.x + theme.padding_standard;
        if let Some(icon_id) = item.icon_id {
            chrome_icon(draw, icons, icon_id, content_x, item_rect.y + (item_rect.h - CHROME_ICON_TINY) * 0.5, CHROME_ICON_TINY, text_color);
            content_x += CHROME_ICON_TINY + theme.gap_standard;
        }
        #[cfg(test)]
        {
            if let Some(label) = item.label {
                chrome_text(draw, atlas, input, theme, label, content_x, item_rect.y + (item_rect.h + theme.font_size_small) * 0.5 - 1.0, theme.font_size_small, text_color);
            }
        }
        if register_hits && !item.disabled {
            note_chrome_group_item(item);
            input.register_hit(HitTarget { rect: item_rect, event: None, control_id: Some(item.control_id.into()), kind: item.kind.clone(), drag_axis: None, drag_data: None });
        }
        x += item_w;
        if index + 1 < items.len() {
            draw.push_solid([x, inner_y, hair, inner_h], theme.border_normal);
        }
    }
    chrome_group_border(draw, rect, theme);
}

fn utility_node_label<'a>(label: &'a Option<String>, text: &'a Option<String>, title: &'a Option<String>, id: &'a str) -> &'a str {
    title.as_deref().or(label.as_deref()).or(text.as_deref()).unwrap_or(id)
}

/// 📏️ One horizontal interval in a shell chrome band.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub(crate) struct ShellChromeBandSpan {
    pub left: f32,
    pub right: f32,
}

impl ShellChromeBandSpan {
    pub(crate) fn width(self) -> f32 {
        (self.right - self.left).max(0.0)
    }
}

/// 🎯️ The widest unoccupied band and the viewport-centered placement clamped into it.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub(crate) struct ShellChromeCenteredBand {
    pub free: ShellChromeBandSpan,
    pub centered: ShellChromeBandSpan,
}

/// 🧱️ Rust twin of `navbarFreeBandV1`: normalize occupied spans, merge overlaps, and choose
/// the widest remaining interval. A fully occupied bar retains the whole width, matching the shared
/// language-neutral fixture and React helper.
pub(crate) fn shell_chrome_free_band(width: f32, occupied: &[ShellChromeBandSpan]) -> ShellChromeBandSpan {
    let width = width.max(0.0);
    let mut spans: Vec<ShellChromeBandSpan> = occupied.iter().map(|span| ShellChromeBandSpan { left: span.left.clamp(0.0, width), right: span.right.clamp(0.0, width) }).filter(|span| span.right > span.left).collect();
    spans.sort_by(|left, right| left.left.total_cmp(&right.left));
    let mut merged: Vec<ShellChromeBandSpan> = Vec::with_capacity(spans.len());
    for span in spans {
        if let Some(last) = merged.last_mut().filter(|last| span.left <= last.right) {
            last.right = last.right.max(span.right);
        } else {
            merged.push(span);
        }
    }
    let mut band = ShellChromeBandSpan::default();
    let mut cursor = 0.0;
    for span in merged.into_iter().chain(std::iter::once(ShellChromeBandSpan { left: width, right: width })) {
        if span.left - cursor > band.width() {
            band = ShellChromeBandSpan { left: cursor, right: span.left };
        }
        cursor = cursor.max(span.right);
    }
    if band.width() > 0.0 {
        band
    } else {
        ShellChromeBandSpan { left: 0.0, right: width }
    }
}

/// 🎯️ Rust twin of `navbarCenteredLeftV1`, including its pixel rounding — to the nearest whole pixel INSIDE the band, so a
/// sub-pixel band edge is never crossed (the unrounded clamp when no whole pixel fits).
pub(crate) fn shell_chrome_centered_band(width: f32, occupied: &[ShellChromeBandSpan], desired_width: f32) -> ShellChromeCenteredBand {
    let free = shell_chrome_free_band(width, occupied);
    let centered_width = desired_width.max(0.0).min(free.width());
    let latest = free.right - centered_width;
    let left = if latest <= free.left {
        free.left
    } else {
        let (first, last) = (free.left.ceil(), latest.floor());
        let left = ((width - centered_width) * 0.5).clamp(free.left, latest);
        if first > last {
            left
        } else {
            left.round().clamp(first, last)
        }
    };
    ShellChromeCenteredBand { free, centered: ShellChromeBandSpan { left, right: left + centered_width } }
}

pub(crate) fn shell_chrome_clip_horizontal(rect: Rect, span: ShellChromeBandSpan) -> Option<Rect> {
    let left = rect.x.max(span.left);
    let right = (rect.x + rect.w).min(span.right);
    (right > left).then(|| Rect::new(left, rect.y, right - left, rect.h))
}

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub(crate) struct ShellNavbarChromeLayout {
    pub(crate) center: ShellChromeCenteredBand,
    pub(crate) trailing_right: f32,
}

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub(crate) struct ShellFooterChromeLayout {
    pub(crate) center: ShellChromeCenteredBand,
    pub(crate) presence: Option<Rect>,
    pub(crate) hub: Rect,
    pub(crate) sign_in: Option<Rect>,
    pub(crate) bottom_right_left: f32,
}

#[cfg(test)]
fn partition_utilities_by_category(utilities: &[UtilityNode]) -> [Vec<UtilityNode>; 4] {
    let mut buckets: [Vec<UtilityNode>; 4] = [vec![], vec![], vec![], vec![]];
    for utility in utilities {
        let idx = match utility.category() {
            ui_wgpu::wgpu::UtilityCategory::Selection => 0,
            ui_wgpu::wgpu::UtilityCategory::Utilities => 1,
            ui_wgpu::wgpu::UtilityCategory::History => 2,
            ui_wgpu::wgpu::UtilityCategory::Sync => 3,
        };
        buckets[idx].push(utility.clone());
    }
    buckets
}

//#region 🚦️SyncPill
/// 🚦️ The remote half of the pill vocabulary — React's
/// `SyncPillState["remote"]` (`🛠️ShellHelpers/🟦️.tsx`).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum ShellSyncRemote {
    Connected,
    Connecting,
    Backoff,
    Detached,
}

/// 🚦️ The `#s-sync-status` pill's state — React's `SyncPillState`, plus this renderer's own
/// `Recovering` arm for the native bootstrap-progress readout React has no twin for (it is produced
/// only by `ShellState::sync_pill`'s native branch and is documented as a declared divergence).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum ShellSyncPill {
    Persisted,
    Pending(usize),
    Remote(ShellSyncRemote),
    Recovering { received_bytes: u64, total_bytes: u64, received_chunks: u32, total_chunks: u32 },
}

/// 🚦️ Localized pill text, term for term with `syncPillText` (`🛠️ShellHelpers/🟦️.tsx`) — including
/// its German column, which this side used to lack: the pill was English-only, so a `de` shell read
/// `Remote: detached` where React reads `Remote: getrennt`. The strings live in
/// [`shell_chrome_string`] with every other shell-owned label rather than inline here.
pub(crate) fn shell_sync_pill_text(pill: ShellSyncPill, is_de: bool) -> String {
    match pill {
        ShellSyncPill::Recovering { received_bytes, total_bytes, received_chunks, total_chunks } => {
            format!("{} {received_bytes}/{total_bytes} · {received_chunks}/{total_chunks}", shell_chrome_string("sync.recovering", is_de))
        }
        ShellSyncPill::Persisted => shell_chrome_string("sync.persisted", is_de).to_string(),
        ShellSyncPill::Pending(count) => format!("{} ({count})", shell_chrome_string("sync.pending", is_de)),
        ShellSyncPill::Remote(remote) => {
            let state = match remote {
                ShellSyncRemote::Connected => "sync.remote.connected",
                ShellSyncRemote::Connecting => "sync.remote.connecting",
                ShellSyncRemote::Backoff => "sync.remote.backoff",
                ShellSyncRemote::Detached => "sync.remote.detached",
            };
            format!("{}: {}", shell_chrome_string("sync.remote", is_de), shell_chrome_string(state, is_de))
        }
    }
}
//#endregion 🚦️SyncPill

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "camelCase")]
pub enum ShellHubRemoteV1 {
    Detached,
    Connecting,
    Live {
        #[serde(rename = "peerCount")]
        peer_count: usize,
    },
    Backoff {
        #[serde(rename = "retryInMs")]
        retry_in_ms: u64,
    },
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ShellHubDocumentV1 {
    pub document_key: String,
    pub remote: ShellHubRemoteV1,
}

/// 📶️ The target-neutral hub projection (`🧬️schema/🔗️hub-projection`'s `projection`): the shell's
/// session and link axes and every attached document's remote state.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ShellHubProjectionV1 {
    pub session: HubSessionPresence,
    pub link: HubLink,
    pub documents: Vec<ShellHubDocumentV1>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) struct ShellHubSummaryV1 {
    pub state: HubConnectionState,
    pub peer_count: usize,
    pub document_count: usize,
}

pub(crate) fn shell_hub_connection_summary_v1(projection: &ShellHubProjectionV1) -> ShellHubSummaryV1 {
    let document_count = projection.documents.len();
    let statuses = projection
        .documents
        .iter()
        .map(|document| match document.remote {
            ShellHubRemoteV1::Detached => HubDocumentRemote::Detached,
            ShellHubRemoteV1::Connecting => HubDocumentRemote::Connecting,
            ShellHubRemoteV1::Live { peer_count } => HubDocumentRemote::Live { peer_count },
            ShellHubRemoteV1::Backoff { .. } => HubDocumentRemote::Backoff,
        })
        .collect::<Vec<_>>();
    let state = crate::hub_connection::hub_connection_summary(&statuses, projection.session, projection.link).state;
    let peer_count = match state {
        HubConnectionState::Live { peer_count } => peer_count,
        _ => 0,
    };
    ShellHubSummaryV1 { state, peer_count, document_count }
}

pub(crate) fn shell_hub_document_key(document_key: &ArtifactDocumentKey) -> String {
    match document_key {
        ArtifactDocumentKey::Local { document_id } => format!("local:{document_id}"),
        ArtifactDocumentKey::Hub { space_id, document_id } => format!("hub:{space_id}/{document_id}"),
    }
}

/// 🔌️ The link status an actor's sync status implies: live relinks, a backoff means the link is short; a
/// terminal link stays terminal until the next open (only the actor's coded terminal message ends a link).
pub(crate) fn shell_sync_link_after_status(current: DocumentLinkStatus, remote: &RemoteState) -> DocumentLinkStatus {
    match (current, remote) {
        (DocumentLinkStatus::LinkExpired | DocumentLinkStatus::AccessRevoked, _) => current,
        (_, RemoteState::Live { .. }) => DocumentLinkStatus::Linked,
        (_, RemoteState::Backoff { .. }) => DocumentLinkStatus::Reconnecting,
        _ => current,
    }
}

/// 🔌️ The terminal link a document actor's coded conflict names, if it names one.
pub(crate) fn shell_sync_link_terminal(code: &str) -> Option<DocumentLinkStatus> {
    [DocumentLinkStatus::LinkExpired, DocumentLinkStatus::AccessRevoked].into_iter().find(|status| status.code() == code)
}

pub(crate) fn shell_hub_remote(remote: &RemoteState) -> ShellHubRemoteV1 {
    match remote {
        RemoteState::Detached => ShellHubRemoteV1::Detached,
        RemoteState::Connecting => ShellHubRemoteV1::Connecting,
        RemoteState::Live { peer_count } => ShellHubRemoteV1::Live { peer_count: *peer_count },
        RemoteState::Backoff { retry_in_ms } => ShellHubRemoteV1::Backoff { retry_in_ms: *retry_in_ms },
    }
}

/// 📶️ The footer hub badge's text for one fold state, en + de, term for term with React's
/// `HubConnectionIndicator` (`ui.sync.*`).
pub(crate) fn shell_hub_connection_text(state: HubConnectionState, is_de: bool) -> String {
    match state {
        HubConnectionState::Local => shell_chrome_string("hub.local", is_de).to_string(),
        HubConnectionState::SignedOut => shell_chrome_string("hub.signedOut", is_de).to_string(),
        HubConnectionState::Live { peer_count } => format!("{} · {peer_count} {}", shell_chrome_string("hub.live", is_de), if peer_count == 1 { shell_chrome_string("hub.peerOne", is_de) } else { shell_chrome_string("hub.peerMany", is_de) }),
        HubConnectionState::Online => shell_chrome_string("hub.online", is_de).to_string(),
        HubConnectionState::Connecting => shell_chrome_string("hub.connecting", is_de).to_string(),
        HubConnectionState::Reconnecting => shell_chrome_string("hub.reconnecting", is_de).to_string(),
    }
}

//#region 📌️HubCheckIn
/// ⏳️ How long a requested Check In waits for the document's edits to be acknowledged.
#[cfg(not(target_arch = "wasm32"))]
pub(crate) const HUB_CHECK_IN_QUIESCENCE_MS: u64 = 15_000;
/// ⏱️ The whole Check In, from request to a terminal status.
#[cfg(not(target_arch = "wasm32"))]
pub(crate) const HUB_CHECK_IN_DEADLINE_MS: u64 = 180_000;
#[cfg(not(target_arch = "wasm32"))]
pub(crate) const HUB_CHECK_IN_POLL_MS: u64 = 250;

/// 📌️ One native hub Check In owner (see `ShellState::hub_check_in`).
#[cfg(not(target_arch = "wasm32"))]
#[derive(Clone, Debug)]
pub struct ShellHubCheckInV1 {
    pub space_id: String,
    pub document_id: String,
    pub request_id: String,
    pub head: Option<semio_framework_os_kernel::os_directory::EditedArtifactFrontierV1>,
    pub quiescent_by_ms: u64,
    pub deadline_at_ms: u64,
    pub next_poll_at_ms: u64,
    pub submitted: bool,
    pub cancel_requested: bool,
    pub cancel_sent: bool,
    pub status: Option<semio_framework_os_kernel::os_directory::DocumentCheckInStatusV1>,
}

#[cfg(not(target_arch = "wasm32"))]
pub(crate) fn hub_check_in_local_status(
    request_id: &str,
    phase: semio_framework_os_kernel::os_directory::DocumentCheckInPhaseV1,
    refusal: Option<semio_framework_os_kernel::os_directory::DocumentCheckInRefusalV1>,
) -> semio_framework_os_kernel::os_directory::DocumentCheckInStatusV1 {
    semio_framework_os_kernel::os_directory::DocumentCheckInStatusV1 {
        schema: semio_framework_os_kernel::os_directory::DOCUMENT_CHECK_IN_STATUS_SCHEMA_V1.to_string(),
        request_id: request_id.to_string(),
        phase,
        progress: semio_framework_os_kernel::os_directory::DocumentCheckInProgressV1 { completed_units: 0, total_units: 8 },
        ready: None,
        refusal,
    }
}

/// 📌️ One sentence for a hub Check In status (the React twin is `checkinStatusText`).
#[cfg(not(target_arch = "wasm32"))]
pub(crate) fn hub_check_in_status_text(status: &semio_framework_os_kernel::os_directory::DocumentCheckInStatusV1, is_de: bool) -> String {
    use semio_framework_os_kernel::os_directory::{DocumentCheckInPhaseV1 as Phase, DocumentCheckInRefusalV1 as Refusal};
    let key = match (status.phase, status.refusal) {
        (Phase::Ready, _) => "checkIn.ready",
        (Phase::Cancelled, _) => "checkIn.cancelled",
        (Phase::Failed, Some(Refusal::UnknownHead)) => "checkIn.unknownHead",
        (Phase::Failed, Some(Refusal::StaleHead)) => "checkIn.staleHead",
        (Phase::Failed, Some(Refusal::ActiveCheckpointChanged)) => "checkIn.activeCheckpointChanged",
        (Phase::Failed, Some(Refusal::LedgerNotReplayable)) => "checkIn.ledgerNotReplayable",
        (Phase::Failed, Some(Refusal::CodecRefused)) => "checkIn.codecRefused",
        (Phase::Failed, Some(Refusal::AuthorityChanged)) => "checkIn.authorityChanged",
        (Phase::Failed, _) => "checkIn.unavailable",
        (Phase::Accepted | Phase::Materializing | Phase::Publishing, _) => return format!("{} {}/{}", shell_chrome_string("checkIn.running", is_de), status.progress.completed_units, status.progress.total_units),
    };
    shell_chrome_string(key, is_de).to_string()
}
//#endregion 📌️HubCheckIn

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

fn render_utility_section_divider(draw: &mut DrawList, theme: &Theme, x: f32, btn_y: f32, btn_h: f32) -> f32 {
    draw.push_solid([x + theme.gap_standard * 0.5, btn_y + 4.0, theme.stroke_hairline, btn_h - 8.0], theme.border_normal);
    x + theme.gap_standard
}

fn utility_at_path<'utility>(roots: &'utility [UtilityNode], path: &[u16; 64], depth: usize) -> Option<&'utility UtilityNode> {
    let mut siblings = roots;
    for level in 0..=depth {
        let node = siblings.get(path[level] as usize)?;
        if level == depth {
            return Some(node);
        }
        let UtilityNode::Collection { children, .. } = node else { return None };
        siblings = children;
    }
    None
}

fn utility_sibling_count(roots: &[UtilityNode], path: &[u16; 64], depth: usize) -> Option<usize> {
    if depth == 0 {
        return Some(roots.len());
    }
    let parent = utility_at_path(roots, path, depth - 1)?;
    let UtilityNode::Collection { children, .. } = parent else { return None };
    Some(children.len())
}

pub(crate) fn render_utility_node_step(
    cursor: &mut ShellChromeChildCursor,
    chrome: &mut ShellChromeBuildState,
    draw: &mut DrawList,
    atlas: &mut FontAtlas,
    icons: &IconAtlas,
    input: &mut InputState<ActionDescriptor>,
    theme: &Theme,
    x: f32,
    btn_y: f32,
    btn_h: f32,
    utility: &UtilityNode,
    collection_expanded: &HashMap<String, bool>,
) -> Result<Option<(f32, bool)>, ()> {
    match utility {
        UtilityNode::Separator { .. } => Ok(Some((render_utility_section_divider(draw, theme, x, btn_y, btn_h), false))),
        UtilityNode::Button { id, icon_id, label, text, title, disabled, on_press, .. } if !disabled.unwrap_or(false) => {
            let control_id = format!("{WINDOW_UTILITY_RAIL_PARENT}button.{id}");
            let item = ChromeGroupItem { control_id: &control_id, icon_id: Some(icon_id.as_str()), label: Some(utility_node_label(label, text, title, id)), active: false, disabled: false, kind: HitKind::Button };
            if cursor.rect.is_none() {
                let width = retained_chrome_group_item_width(atlas, theme, &item).ok_or(())?;
                cursor.rect = Some(Rect::new(x, btn_y, width, btn_h));
            }
            let Some(rect) = cursor.rect else { return Err(()) };
            match render_retained_chrome_group_item_step(&mut cursor.group_phase, &mut cursor.glyph, draw, atlas, icons, input, theme, rect, &item, false) {
                RetainedChromeGroupStep::Pending => return Ok(None),
                RetainedChromeGroupStep::Fault => return Err(()),
                RetainedChromeGroupStep::Complete => {}
            }
            note_chrome_group_item(&item);
            chrome.register_element_rect(id.clone(), rect);
            input.register_hit(HitTarget { rect, event: Some(on_press.clone()), control_id: Some(control_id), kind: HitKind::Button, drag_axis: None, drag_data: None });
            cursor.rect = None;
            Ok(Some((x + rect.w + theme.gap_standard * 0.5, false)))
        }
        UtilityNode::Toggle { id, icon_id, label, text, title, pressed, disabled, on_change, .. } if !disabled.unwrap_or(false) => {
            let control_id = format!("{WINDOW_UTILITY_RAIL_PARENT}toggle.{id}");
            let item = ChromeGroupItem { control_id: &control_id, icon_id: Some(icon_id.as_str()), label: Some(utility_node_label(label, text, title, id)), active: pressed.unwrap_or(false), disabled: false, kind: HitKind::Toggle };
            if cursor.rect.is_none() {
                let width = retained_chrome_group_item_width(atlas, theme, &item).ok_or(())?;
                cursor.rect = Some(Rect::new(x, btn_y, width, btn_h));
            }
            let Some(rect) = cursor.rect else { return Err(()) };
            match render_retained_chrome_group_item_step(&mut cursor.group_phase, &mut cursor.glyph, draw, atlas, icons, input, theme, rect, &item, false) {
                RetainedChromeGroupStep::Pending => return Ok(None),
                RetainedChromeGroupStep::Fault => return Err(()),
                RetainedChromeGroupStep::Complete => {}
            }
            note_chrome_group_item(&item);
            chrome.register_element_rect(id.clone(), rect);
            input.register_hit(HitTarget { rect, event: Some(on_change.clone()), control_id: Some(control_id), kind: HitKind::Toggle, drag_axis: None, drag_data: None });
            cursor.rect = None;
            Ok(Some((x + rect.w + theme.gap_standard * 0.5, false)))
        }
        UtilityNode::Collection { id, icon_id, label, text, title, disabled, children, .. } if !disabled.unwrap_or(false) => {
            let expanded = collection_expanded.get(id).copied().unwrap_or(false);
            let control_id = format!("{WINDOW_UTILITY_RAIL_PARENT}collection.{id}");
            let item = ChromeGroupItem { control_id: &control_id, icon_id: Some(icon_id.as_str()), label: Some(utility_node_label(label, text, title, id)), active: expanded, disabled: false, kind: HitKind::Button };
            if cursor.rect.is_none() {
                let width = retained_chrome_group_item_width(atlas, theme, &item).ok_or(())?;
                cursor.rect = Some(Rect::new(x, btn_y, width, btn_h));
            }
            let Some(rect) = cursor.rect else { return Err(()) };
            match render_retained_chrome_group_item_step(&mut cursor.group_phase, &mut cursor.glyph, draw, atlas, icons, input, theme, rect, &item, false) {
                RetainedChromeGroupStep::Pending => return Ok(None),
                RetainedChromeGroupStep::Fault => return Err(()),
                RetainedChromeGroupStep::Complete => {}
            }
            note_chrome_group_item(&item);
            input.register_hit(HitTarget { rect, event: None, control_id: Some(control_id), kind: HitKind::Button, drag_axis: None, drag_data: None });
            cursor.rect = None;
            Ok(Some((x + rect.w + theme.gap_standard * 0.5, expanded && !children.is_empty())))
        }
        UtilityNode::Button { .. } | UtilityNode::Toggle { .. } | UtilityNode::Collection { .. } => Ok(Some((x, false))),
    }
}

/// 🧭️ Item 5 (ribbon nesting): a collapsed group still highlights when the active picker
/// segment lives somewhere inside it (recursively, through further nested `Collection`s) —
/// `active-path` tracking, ported from the React ribbon's recursive reconciliation.
///
/// 🧭️ Item 5: previously flattened one level (`.filter(|child| !matches!(child,
/// UtilityNode::Collection { .. }))` dropped nested `Collection`s outright) — recursing
/// on the full, unfiltered `children` supports arbitrary nesting depth; each nested
/// `Collection` gets its own `collection_expanded` entry (already a flat `id`-keyed map,
/// so no path-keying change needed) and paints its own active-path highlight in turn.
#[cfg(test)]
pub(crate) fn render_utility_nodes(
    chrome: &mut ShellChromeBuildState,
    draw: &mut DrawList,
    atlas: &mut FontAtlas,
    icons: &IconAtlas,
    input: &mut InputState<ActionDescriptor>,
    theme: &Theme,
    mut x: f32,
    btn_y: f32,
    btn_h: f32,
    utilities: &[UtilityNode],
    collection_expanded: &HashMap<String, bool>,
) -> f32 {
    for utility in utilities {
        match utility {
            UtilityNode::Separator { .. } => {
                x = render_utility_section_divider(draw, theme, x, btn_y, btn_h);
            }
            UtilityNode::Button { id, icon_id, label, text, title, disabled, on_press, .. } => {
                if disabled.unwrap_or(false) {
                    continue;
                }
                let label_text = utility_node_label(label, text, title, id);
                let control_id = format!("{WINDOW_UTILITY_RAIL_PARENT}button.{id}");
                let item = ChromeGroupItem { control_id: &control_id, icon_id: Some(icon_id.as_str()), label: Some(label_text), active: false, disabled: false, kind: HitKind::Button };
                let item_w = measure_chrome_group_item(atlas, theme, &item);
                let rect = Rect::new(x, btn_y, item_w, btn_h);
                render_chrome_group(draw, atlas, icons, input, theme, rect, &[item], true);
                chrome.register_element_rect(id.clone(), rect);
                input.register_hit(HitTarget { rect, event: Some(on_press.clone()), control_id: Some(control_id), kind: HitKind::Button, drag_axis: None, drag_data: None });
                x += item_w + theme.gap_standard * 0.5;
            }
            UtilityNode::Toggle { id, icon_id, label, text, title, pressed, disabled, on_change, .. } => {
                if disabled.unwrap_or(false) {
                    continue;
                }
                let label_text = utility_node_label(label, text, title, id);
                let control_id = format!("{WINDOW_UTILITY_RAIL_PARENT}toggle.{id}");
                let item = ChromeGroupItem { control_id: &control_id, icon_id: Some(icon_id.as_str()), label: Some(label_text), active: pressed.unwrap_or(false), disabled: false, kind: HitKind::Toggle };
                let item_w = measure_chrome_group_item(atlas, theme, &item);
                let rect = Rect::new(x, btn_y, item_w, btn_h);
                render_chrome_group(draw, atlas, icons, input, theme, rect, &[item], true);
                chrome.register_element_rect(id.clone(), rect);
                input.register_hit(HitTarget { rect, event: Some(on_change.clone()), control_id: Some(control_id), kind: HitKind::Toggle, drag_axis: None, drag_data: None });
                x += item_w + theme.gap_standard * 0.5;
            }
            UtilityNode::Collection { id, icon_id, label, text, title, disabled, children, .. } => {
                if disabled.unwrap_or(false) {
                    continue;
                }
                let expanded = collection_expanded.get(id).copied().unwrap_or(false);
                let on_active_path = expanded || utility_subtree_has_active_path(children);
                let label_text = utility_node_label(label, text, title, id);
                let control_id = format!("{WINDOW_UTILITY_RAIL_PARENT}collection.{id}");
                let item = ChromeGroupItem { control_id: &control_id, icon_id: Some(icon_id.as_str()), label: Some(label_text), active: on_active_path, disabled: false, kind: HitKind::Button };
                let item_w = measure_chrome_group_item(atlas, theme, &item);
                let rect = Rect::new(x, btn_y, item_w, btn_h);
                render_chrome_group(draw, atlas, icons, input, theme, rect, &[item], true);
                input.register_hit(HitTarget { rect, event: None, control_id: Some(control_id), kind: HitKind::Button, drag_axis: None, drag_data: None });
                x += item_w + theme.gap_standard * 0.5;
                if expanded {
                    x = render_utility_nodes(chrome, draw, atlas, icons, input, theme, x, btn_y, btn_h, children, collection_expanded);
                }
            }
        }
    }
    x
}

/// 🌱️ `tab.group == PanelGroup::Workbench` already covers every host-app catalogue tab (each such app
/// declares its catalogue tab under that group — see `s/plugin/rs`'s `App::builder(...).panel_tab(...)`)
/// so no separate app-specific tab-id literal is needed here.
///
/// 🎨️🎛️ Same icon choices as React's `createFrameworkSettingsPanelTab`/`buildCommandCategoryTabs`
/// (`shellTabIcon("paintbrush")` / `COMMAND_CATEGORY_ICON = shellTabIcon("wrench")`).
fn panel_tab_icon_id(tab: &PanelTabDefinition) -> &'static str {
    if tab.group == PanelGroup::Workbench {
        return "library";
    }
    if tab.id().contains("parameters") {
        return "settings";
    }
    if tab.id().contains("inspector") || tab.id().contains("inspection") || tab.id() == FRAMEWORK_PANEL_TAB_INSPECTION_ID {
        return "text-search";
    }
    if tab.id() == FRAMEWORK_PANEL_TAB_ARTIFACT_ID {
        return "file-text";
    }
    if tab.id() == FRAMEWORK_DISPLAY_WINDOWS_TAB_ID {
        return "layout-grid";
    }
    if tab.id() == FRAMEWORK_DISPLAY_LAYOUT_TAB_ID {
        return "layout";
    }
    if tab.id() == FRAMEWORK_SETTINGS_GENERAL_TAB_ID {
        return "settings-2";
    }
    if tab.id() == FRAMEWORK_SETTINGS_THEME_TAB_ID {
        return "paintbrush";
    }
    if tab.id() == FRAMEWORK_PANEL_TAB_CATALOGUE_ID {
        return "library";
    }
    if tab.id() == FRAMEWORK_PANEL_TAB_HISTORY_ID {
        return "undo";
    }
    "circle-dot"
}

/// 🧭️ One app-declared panel tab as a dock node, children included — the wgpu twin of React's
/// `panelTabDefinitionToNode`. The label comes off the manifest's own `LocalizedLabel` through the live
/// terminology and locale, so this carries no shell dictionary and no default language.
pub(crate) fn dock_tab_node_of(tab: &PanelTabDefinition, terminology: Terminology, locale: Locale, order: i32) -> DockTabNode {
    DockTabNode {
        id: tab.id().to_string(),
        label: tab.label.resolve(terminology, locale).to_string(),
        icon_id: panel_tab_icon_id(tab).to_string(),
        order,
        children: tab.children.iter().enumerate().map(|(index, child)| dock_tab_node_of(child, terminology, locale, index as i32)).collect(),
    }
}

//#region ActionPanelAndUtilities
/// 🧰️ Resolves the utilities a window kind presents in the utility bar — the utility mirror of
/// {@link semio_framework::resolve_window_actions}: explicit `window_kind.utilities` refs in declared
/// order, plus any app utility referenced by no window kind (an "orphan" appearing on every window — the
/// scoping fallback that prevents blank utility bars mid-migration, Architecture Decision 8).
pub(crate) fn resolve_window_utilities<'a>(app: &'a AppDefinition, window_kind: &semio_framework::WindowKindDefinition) -> Vec<&'a semio_framework::UtilityDefinition> {
    use std::collections::HashSet;
    let referenced: HashSet<&str> = app.window_kinds.iter().flat_map(|window| window.utilities.iter().map(|utility_ref| utility_ref.as_str())).collect();
    let mut resolved: Vec<&'a semio_framework::UtilityDefinition> = Vec::new();
    let mut seen: HashSet<&str> = HashSet::new();
    for utility_ref in &window_kind.utilities {
        if let Some(utility) = app.utilities.iter().find(|utility| utility.id == utility_ref.as_str()) {
            if seen.insert(utility.id.as_str()) {
                resolved.push(utility);
            }
        }
    }
    for utility in &app.utilities {
        if !referenced.contains(utility.id.as_str()) && seen.insert(utility.id.as_str()) {
            resolved.push(utility);
        }
    }
    resolved
}

/// 📇️ The first window kind whose resolved actions include `action_id` — the window the palette/keybinding
/// redirect focuses to open an arg-carrying action's form (Architecture Decision 8, P3/P4).
// 🎯️ Exercised today only by `🛰️Dock`'s own `action_host_window_id_finds_scoping_window` test — the
// palette/keybinding redirect call site itself hasn't landed yet, hence the matching test-only cfg.
#[cfg(all(test, not(target_arch = "wasm32")))]
pub(crate) fn action_host_window_id(app: &AppDefinition, action_id: &str) -> Option<String> {
    app.window_kinds.iter().find(|kind| semio_framework::resolve_window_actions(app, kind).iter().any(|action| action.id == action_id)).map(|kind| kind.id.clone())
}

/// 🪟️ Resolves an action only from its addressed window kind owner.
pub(crate) fn window_action_definition<'a>(app: &'a AppDefinition, window_kind_id: &str, action_id: &str) -> Option<&'a semio_framework::ActionDefinition> {
    semio_framework::window_kind_actions(app, app.window_kinds.iter().find(|kind| kind.id == window_kind_id)?).into_iter().find(|action| action.id == action_id)
}

/// 🖱️ Maps a `UtilityDefinition.cursor` CSS/winit cursor name onto the shell's {@link ui_wgpu::wgpu::SemioCursor}.
pub(crate) fn semio_cursor_from_name(name: &str) -> ui_wgpu::wgpu::SemioCursor {
    use ui_wgpu::wgpu::SemioCursor;
    match name.trim().to_ascii_lowercase().as_str() {
        "pointer" => SemioCursor::Pointer,
        "text" => SemioCursor::Text,
        "grab" => SemioCursor::Grab,
        "grabbing" => SemioCursor::Grabbing,
        "move" => SemioCursor::Move,
        "crosshair" | "cross" => SemioCursor::Crosshair,
        "not-allowed" | "notallowed" | "forbidden" => SemioCursor::NotAllowed,
        "ew-resize" | "col-resize" | "e-resize" | "w-resize" => SemioCursor::EwResize,
        "ns-resize" | "row-resize" | "n-resize" | "s-resize" => SemioCursor::NsResize,
        "nwse-resize" | "nw-resize" | "se-resize" => SemioCursor::NwseResize,
        "nesw-resize" | "ne-resize" | "sw-resize" => SemioCursor::NeswResize,
        "cell" | "selectable" => SemioCursor::Selectable,
        _ => SemioCursor::Default,
    }
}

/// ⌨️ Formats the first chord of a keybinding for chip badges, palette rows and context-menu shortcut
/// labels — the port of `formatKeybindingShortcut`
/// (`🖱️ui/🔨️modules/🔤️keybinding-text-interpretation/🟦️.ts`), pinned glyph for glyph by
/// `🐚️Shell/🧫️fixtures/⌨️keybinding-glyphs/🔣️.json`, which React's own suite reads too.
///
/// 🩸️ `apple` used to be `cfg!(target_os = "macos")`. That is a COMPILE-time fact and it is FALSE on
/// `wasm32-unknown-unknown`, so every browser boot — macOS included — painted `Ctrl+Alt+E` where
/// React painted `⌘️⌥️E`. It now reads the host's own platform, published by whoever owns a
/// `navigator` (`🧊️renderer/🦀️.rs`, region ⌨️HostPlatform).
pub(crate) fn format_keybinding_shortcut(keys: &str) -> String {
    let chord = keys.split(',').next().unwrap_or(keys).trim().to_ascii_lowercase();
    if chord.is_empty() {
        return String::new();
    }
    let apple = crate::host_platform_uses_meta();
    let glyph = |part: &str| -> String {
        match part {
            "mod" if apple => "⌘️".into(),
            "mod" => "Ctrl".into(),
            "ctrl" if apple => "⌃️".into(),
            "ctrl" => "Ctrl".into(),
            "meta" => "⌘️".into(),
            "alt" if apple => "⌥️".into(),
            "alt" => "Alt".into(),
            "shift" if apple => "⇧️".into(),
            "shift" => "Shift".into(),
            "backspace" => "⌫️".into(),
            "delete" => "⌦️".into(),
            "enter" if apple => "↵️".into(),
            "enter" => "Enter".into(),
            "escape" if apple => "⎋️".into(),
            "escape" => "Esc".into(),
            "up" | "arrowup" => "↑".into(),
            "down" | "arrowdown" => "↓".into(),
            "left" | "arrowleft" => "←".into(),
            "right" | "arrowright" => "→".into(),
            other if other.len() == 1 => other.to_ascii_uppercase(),
            other => {
                let mut s = other.to_string();
                if let Some(first) = s.get_mut(0..1) {
                    first.make_ascii_uppercase();
                }
                s
            }
        }
    };
    let parts: Vec<String> = chord.split('+').map(str::trim).filter(|part| !part.is_empty()).map(glyph).collect();
    if apple {
        parts.join("")
    } else {
        parts.join("+")
    }
}

/// ⌨️ Whether a key event is one of the hardcoded shell chords (palette/find/panels/nav/surface-role/
/// mode-cycle) that must win over app-declared keybindings (P4 — "reserved shell chords still win").
///
/// The `mod+alt+e`/`mod+alt+v` and `mod+alt+←`/`mod+alt+→` rows are the wgpu twin of React's
/// `SHELL_KEYBINDINGS` entries for `playground.navbar.roles.{editor,viewer}` and
/// `ui.shell.mode.{next,previous}` — see `🧱️elements/🏛️ShellHost/🟦️.tsx` and
/// `🏛️ShellHost/🧫️fixtures/🔀️surface-switch/🔣️.json`'s `keybindings` rows, which pin both spellings.
#[cfg(test)]
pub(crate) fn is_reserved_shell_chord(action: &ui_wgpu::wgpu::KeyAction, modifiers: &PointerModifiers) -> bool {
    is_reserved_shell_chord_in(&shell_shortcut_table(&HashMap::new()), action, modifiers)
}

/// ⌨️ {@link is_reserved_shell_chord} against a RESOLVED table, so a REMAPPED shell chord is reserved
/// at its new spelling and the default it replaced becomes an app's to claim — React's
/// `reservedShellChordsV1(SHELL_KEYBINDINGS, uiKeybindingOverrides)`.
pub(crate) fn is_reserved_shell_chord_in(table: &[(String, String)], action: &ui_wgpu::wgpu::KeyAction, modifiers: &PointerModifiers) -> bool {
    shell_shortcut_for_in(table, action, modifiers).is_some()
}

//#region 🔀️ChromeParity
/// 🎛️ One navbar control of the shell's centre cluster, resolved before anything is painted so the
/// laws below can assert "a hit target exists exactly when the app declares it" without a
/// `ShellState` (90+ fields, several without `Default`). The control ids are React's `ShellHost`
/// ids verbatim — one product across both renderers, one id for every probe.
///
/// @see `🧱️elements/🏛️ShellHost/🟦️.tsx` — `exampleSelectElement`/`modeSwitcherElement`/`roleSwitcherElement`
/// @see `🧱️elements/🏛️ShellHost/🔀️surface-switch/🟦️.ts` — `SURFACE_ROLE_CONTROL_IDS`
#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct ShellNavbarControl {
    pub control_id: String,
    pub icon_id: Option<&'static str>,
    pub label: String,
    pub active: bool,
    pub width_policy: Option<ShellNavbarWidthPolicy>,
}

/// 📐️ A navbar control's authored responsive width contract. The bounds are expressed in root-em
/// units and resolved in logical pixels before both centred-band reservation and paint.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum ShellNavbarWidthPolicy {
    RootRemClamp { minimum_rem: u16, maximum_rem: u16 },
}

/// 📐️ Resolves one control's available width through its authored width policy. The caller supplies
/// logical pixels, so neither device density nor typography participates in root-rem conversion.
pub(crate) fn resolve_shell_navbar_control_width(available_width: f32, policy: Option<ShellNavbarWidthPolicy>, root_rem_pixels: f32) -> f32 {
    match policy {
        Some(ShellNavbarWidthPolicy::RootRemClamp { minimum_rem, maximum_rem }) => available_width.clamp(minimum_rem as f32 * root_rem_pixels, maximum_rem as f32 * root_rem_pixels),
        None => available_width,
    }
}

/// 🗺️ What the navbar centre reads with no session open. React's `navbarItems` answers `[]` in that
/// case and paints no cluster at all; this renderer's chrome walk has no "paint nothing" arm, so it
/// keeps the product name it has always shown.
pub(crate) const SHELL_NAVBAR_SESSIONLESS_TITLE: &str = "semio · os";

/// 🗺️ The navbar centre title — React's `appBreadcrumb(resolveAppBreadcrumb(session.app, uiTerminology))`
/// (`🏛️ShellHost/🟦️.tsx:10058`), through the SAME `semio_framework::manifest` pair React's
/// `🛠️ShellHelpers/🟦️.tsx:1348-1355` twins, so `" · "` is one separator constant and a terminology
/// override (`reuse` → `Entwerfen mit Bestand · Aggregator`) lands identically on both renderers.
pub(crate) fn shell_navbar_title(app: &AppDefinition, terminology_id: &str) -> String {
    semio_framework::manifest::app_breadcrumb(semio_framework::manifest::resolve_app_breadcrumb(app, terminology_id))
}

/// 👁️✏️ React's `surfaceRoleChipText` (`🛠️ShellHelpers/🟦️.tsx:2422`) — the frozen `{en, de}` pair of the
/// navbar's read-only role badge, term for term.
pub(crate) fn shell_surface_role_chip_text(role: semio_framework::manifest::AppRole, is_de: bool) -> &'static str {
    match (role, is_de) {
        (semio_framework::manifest::AppRole::Viewer, false) => "Viewer",
        (semio_framework::manifest::AppRole::Viewer, true) => "Betrachter",
        (semio_framework::manifest::AppRole::Editor, _) => "Editor",
    }
}

/// ⌨️ The inline hotkey badge React's `ControlHotkeyBadge` paints INSIDE a chrome control
/// (`🔨️modules/⌨️control-hotkey-presentation/🟦️.tsx:32`) whenever the active driver's `hotkeys` axis is
/// `"inline"` — which `DEFAULT_UI_DRIVER` sets (`🧱️elements/🚗️UiDriver/🟦️.tsx:41`), so the desktop
/// navbar's role buttons read `Editor ⌘️⌥️E` / `Viewer ⌘️⌥️V`. The chord comes from this session's own
/// remappable table, so an override is reflected in the badge exactly as React's
/// `useControlHotkey(id)` reads `controlKeybindings`.
pub(crate) fn shell_control_hotkey_badge(table: &[(String, String)], control_id: &str) -> Option<String> {
    table.iter().find(|(id, _)| id == control_id).map(|(_, keys)| format_keybinding_shortcut(keys))
}

/// 📚️ One row of the open `playground.navbar.fixture` dropdown. `control_id` is what the click
/// handler's `shell.example.<id>` arm already dispatches `setActiveExample` for.
#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct ShellExampleRow {
    pub control_id: String,
    pub label: String,
    pub selected: bool,
}

/// 📚️ The example rows one surface offers, by DIALECT — `manifest::examples_for_app` is THE
/// predicate, shared with React's `examplesForApp` and pinned by
/// `🛂️manifest/🧫️fixtures/📚️example-picker.json`. Viewers additionally declare the example-switch action.
pub(crate) fn shell_offered_examples<'a>(examples: &'a [semio_framework::manifest::ExampleDefinition], app: &AppDefinition) -> Vec<&'a semio_framework::manifest::ExampleDefinition> {
    if app.role != semio_framework::AppRole::Editor && !app.actions.iter().chain(app.window_kinds.iter().flat_map(|kind| kind.actions.iter())).any(|action| action.id == "setActiveExample") {
        return Vec::new();
    }
    semio_framework::manifest::examples_for_app(examples, app)
}

pub(crate) fn shell_example_rows(examples: &[semio_framework::manifest::ExampleDefinition], app: &AppDefinition, active_example_id: Option<&str>, terminology: Terminology, locale: Locale) -> Vec<ShellExampleRow> {
    shell_offered_examples(examples, app)
        .into_iter()
        .map(|example| ShellExampleRow { control_id: format!("shell.example.{}", example.id), label: example.label.resolve(terminology, locale).to_string(), selected: active_example_id == Some(example.id.as_str()) })
        .collect()
}

/// 📚️ The example a fresh session announces — the Rust twin of React's `resolveBootExampleId`
/// (`🐚️Shell/🟦️.tsx`), pinned row-for-row by the shared `🐚️Shell/🧫️fixtures/📚️boot-example/🔣️.json`.
///
/// A still-authored live selection outranks everything; then the declared default — which on this
/// target is `?example=`'s value and on React the `VITE_SEMIO_DEFAULT_EXAMPLE` seed or the same query;
/// then the dialect's first example, so a navbar never boots on "No example" while the guest's own
/// default document is already that first fixture. An id the open dialect does not author falls back
/// rather than faulting: a url is not a place to hard-fail a shell, the rule `?role=` and `?mode=`
/// already follow. A dialect that authored no example answers `""`.
pub(crate) fn resolve_boot_example_id(active_example_id: &str, example_ids: &[String], default_example_id: Option<&str>) -> String {
    let authored = |candidate: &str| !candidate.is_empty() && example_ids.iter().any(|id| id == candidate);
    if authored(active_example_id) {
        return active_example_id.to_string();
    }
    if let Some(default_example_id) = default_example_id.filter(|candidate| authored(candidate)) {
        return default_example_id.to_string();
    }
    example_ids.first().cloned().unwrap_or_default()
}

/// 📚️ The `playground.navbar.fixture` trigger, or `None` when the open dialect authored no example —
/// React's `exampleOptions.length > 0` render gate, verbatim. The label is the SELECTED row's own
/// label (a select shows its value), falling back to the localized noun when nothing is picked yet.
pub(crate) fn shell_example_control(rows: &[ShellExampleRow], open: bool, is_de: bool) -> Option<ShellNavbarControl> {
    if rows.is_empty() {
        return None;
    }
    let label = rows.iter().find(|row| row.selected).map_or_else(|| shell_chrome_string("example.picker", is_de).to_string(), |row| row.label.clone());
    Some(ShellNavbarControl { control_id: "playground.navbar.fixture".to_string(), icon_id: Some("file"), label, active: open, width_policy: Some(ShellNavbarWidthPolicy::RootRemClamp { minimum_rem: 12, maximum_rem: 28 }) })
}

/// 🎛️ The `playground.navbar.modes.<id>` group — empty for a single-mode surface, exactly as React
/// renders no mode switcher for one (the viewer of generation3d has one mode and shows none).
pub(crate) fn shell_mode_controls(app: &AppDefinition, active_mode_id: Option<&str>, terminology: Terminology, locale: Locale) -> Vec<ShellNavbarControl> {
    if app.modes.len() < 2 {
        return Vec::new();
    }
    app.modes
        .iter()
        .map(|mode| ShellNavbarControl { control_id: format!("playground.navbar.modes.{}", mode.id), icon_id: None, label: mode.label.resolve(terminology, locale).to_string(), active: active_mode_id == Some(mode.id.as_str()), width_policy: None })
        .collect()
}

/// 👁️✏️ Both surfaces of ONE dialect, in `editor → viewer` focus order, or `None` when the plugin
/// declares fewer than two — the roles group's whole render gate. Twin of `surfaceRoleAppsV1`
/// (`🏛️ShellHost/🔀️surface-switch/🟦️.ts`), pinned by that lane's `group` fixture rows.
pub(crate) fn surface_role_apps<'a>(apps: &'a [AppDefinition], dialect: &semio_framework_artifact_reference::ArtifactDialect) -> Option<(&'a AppDefinition, &'a AppDefinition)> {
    let editor = apps.iter().find(|app| &app.dialect == dialect && app.role == semio_framework::manifest::AppRole::Editor)?;
    let viewer = apps.iter().find(|app| &app.dialect == dialect && app.role == semio_framework::manifest::AppRole::Viewer)?;
    Some((editor, viewer))
}

/// 👁️✏️ The app a role switch would open, or `None` when the role is already mounted or the dialect
/// declares no such sibling. Twin of `roleSwitchTargetV1`.
pub(crate) fn role_switch_target<'a>(apps: &'a [AppDefinition], dialect: &semio_framework_artifact_reference::ArtifactDialect, current_role: semio_framework::manifest::AppRole, requested: semio_framework::manifest::AppRole) -> Option<&'a AppDefinition> {
    if current_role == requested {
        return None;
    }
    let (editor, viewer) = surface_role_apps(apps, dialect)?;
    Some(match requested {
        semio_framework::manifest::AppRole::Editor => editor,
        semio_framework::manifest::AppRole::Viewer => viewer,
    })
}

/// 👁️✏️ The `playground.navbar.roles.{editor,viewer}` group. Icons are fixed per role (`pencil`/`eye`),
/// deliberately NOT each app's own `icon_id`: both surfaces of one artifact carry the same artifact
/// icon, which would make the two buttons indistinguishable. Labels are each target
/// `AppDefinition`'s own `LocalizedLabel` — no shell dictionary, no default language.
///
/// ⌨️ Each label carries its chord INLINE (`Editor ⌘️⌥️E`), because React's `ButtonGroupItem` paints a
/// `ControlHotkeyBadge` beside its own inline label whenever the driver's `hotkeys` axis is `"inline"`
/// — which the default driver sets — and `playground.navbar.roles.*` are the two `SHELL_KEYBINDINGS`
/// rows that can be per-button at all (`🔨️modules/🕹️control-keybinding-context/🟦️.tsx:162`). The chord
/// is read off this session's own remappable table, never off the frozen defaults.
pub(crate) fn shell_role_controls(apps: &[AppDefinition], app: &AppDefinition, terminology: Terminology, locale: Locale, shortcuts: &[(String, String)]) -> Vec<ShellNavbarControl> {
    let Some((editor, viewer)) = surface_role_apps(apps, &app.dialect) else {
        return Vec::new();
    };
    [(editor, semio_framework::manifest::AppRole::Editor, "pencil"), (viewer, semio_framework::manifest::AppRole::Viewer, "eye")]
        .into_iter()
        .map(|(target, role, icon_id)| {
            let control_id = format!("playground.navbar.roles.{}", role.as_str());
            let label = target.label.resolve(terminology, locale).to_string();
            let label = shell_control_hotkey_badge(shortcuts, &control_id).map_or_else(|| label.clone(), |badge| format!("{label} {badge}"));
            ShellNavbarControl { control_id, icon_id: Some(icon_id), label, active: app.role == role, width_policy: None }
        })
        .collect()
}

/// 🔁️ The wrapping mode cycle `mod+alt+→`/`mod+alt+←` steps through — `None` for a single-mode app
/// (nothing to step to). An active id the app no longer declares steps from the first. Twin of
/// `stepModeIdV1`, pinned by the `modeSteps` fixture rows.
pub(crate) fn step_mode_id(mode_ids: &[String], active_mode_id: Option<&str>, step: i32) -> Option<String> {
    if mode_ids.len() < 2 {
        return None;
    }
    let index = active_mode_id.and_then(|id| mode_ids.iter().position(|candidate| candidate == id)).unwrap_or(0) as i32;
    let len = mode_ids.len() as i32;
    Some(mode_ids[(index + step).rem_euclid(len) as usize].clone())
}

/// 👁️✏️🔁️ **Dedup note**: the surface-role (`mod+alt+e`/`mod+alt+v`) and mode-cycle
/// (`mod+alt+→`/`mod+alt+←`) chords used to be hand-rolled predicates run AHEAD of the shortcut
/// table, which made both rows unremappable by construction (an override could add a chord but never
/// retire the hardcoded one) and matched a chord held with Shift that React's own
/// `keyboardEventMatchesChord` refuses. They are now ordinary [`SHELL_SHORTCUT_ROWS`] rows resolved
/// by [`shell_shortcut_for_in`] like every other verb.

/// 🛑️ The declared shape of a `World3dScene.statusJson` cancel affordance — the projection of
/// [`World3dComputeStatus`] the overlay control layer needs and nothing more.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub(crate) struct World3dCancelAffordance {
    pub cancellable: bool,
    pub cancel_action: String,
    pub cancel_args: serde_json::Map<String, Value>,
}

/// ⏳️ The FULL declared shape of a `World3dScene.statusJson` — the Rust twin of
/// `World3dComputeStatusV1`/`world3dComputeStatusV1` (`🔨️modules/🖱️ui/🎬️scene/🟦️.ts`), field for
/// field, so the wgpu World3d surface reads exactly what React's `WorldComputeStatusPane`
/// (`🌐️World3dHost/🟦️.tsx`) reads. Total by construction: malformed JSON, a missing field or a
/// hostile type degrades to the neutral status rather than throwing inside a render — a status
/// overlay must never be able to blank the viewport it annotates.
///
/// 🌍️ `phase_label` is the PRODUCER's own `{en, de}` pair; the shell never invents phase text and
/// never picks a default language — an absent pair falls back to the shell's own bilingual
/// `common.loading`, exactly as React's pane does.
#[derive(Clone, Debug, PartialEq)]
pub(crate) struct World3dComputeStatus {
    pub computing: bool,
    pub phase: String,
    pub phase_label: Option<(String, String)>,
    pub units_done: f64,
    pub units_total: f64,
    pub faces_done: f64,
    pub faces_total: f64,
    pub in_flight: f64,
    pub ratio: f64,
    pub cancellable: bool,
    pub cancel_action: String,
    pub cancel_args: serde_json::Map<String, Value>,
}

impl Default for World3dComputeStatus {
    fn default() -> Self {
        Self {
            computing: false,
            phase: "idle".to_string(),
            phase_label: None,
            units_done: 0.0,
            units_total: 0.0,
            faces_done: 0.0,
            faces_total: 0.0,
            in_flight: 0.0,
            ratio: 1.0,
            cancellable: false,
            cancel_action: String::new(),
            cancel_args: serde_json::Map::new(),
        }
    }
}

/// ⏳️🖼️ What the World3d compute-status pill says on ONE surface: the phase in the reader's own
/// language, the producer's own `unitsDone/unitsTotal` count, and the fraction its bar fills to.
/// `ratio` is `None` for an indeterminate producer (one reporting no `unitsTotal`), which paints a
/// bare track rather than a lie about how far along the work is.
#[derive(Clone, Debug, PartialEq)]
pub(crate) struct World3dStatusPill {
    pub surface_id: String,
    pub phase: String,
    pub phase_text: String,
    pub progress_text: Option<String>,
    pub ratio: Option<f32>,
    pub computing: bool,
}

impl World3dStatusPill {
    /// 🏷️ The one text run the pill paints — React renders the same two pieces as two adjacent
    /// spans, this target has one glyph run per label, so they join with a separator.
    pub fn label(&self) -> String {
        match self.progress_text.as_deref() {
            Some(progress) => format!("{} · {progress}", self.phase_text),
            None => self.phase_text.clone(),
        }
    }
}

/// ⏳️ Whether a status is worth annotating a viewport with at all — byte-for-byte React's own
/// `WorldComputeStatusPane` gate (`if (!computing && !cancellable && phase !== "cancelled") return
/// null`): a settled producer paints nothing, so an idle preview is never covered by chrome.
pub(crate) fn world3d_status_is_visible(status: &World3dComputeStatus) -> bool {
    status.computing || status.cancellable || status.phase == "cancelled"
}

/// ⏳️🖼️ The pill one surface's published status asks for, or `None` when the status is settled.
pub(crate) fn world3d_status_pill_for(surface_id: &str, status: &World3dComputeStatus, is_de: bool) -> Option<World3dStatusPill> {
    if !world3d_status_is_visible(status) {
        return None;
    }
    let phase_text = match (status.phase_label.as_ref(), is_de) {
        (Some((_, de)), true) => de.clone(),
        (Some((en, _)), false) => en.clone(),
        (None, _) => shell_chrome_string("common.loading", is_de).to_string(),
    };
    let (progress_text, ratio) = if status.units_total > 0.0 {
        let percent = (status.ratio * 100.0).round() as i64;
        (Some(format!("{}/{} ({percent}%)", status.units_done as i64, status.units_total as i64)), Some(status.ratio as f32))
    } else {
        (None, None)
    };
    Some(World3dStatusPill { surface_id: surface_id.to_string(), phase: status.phase.clone(), phase_text, progress_text, ratio, computing: status.computing })
}

/// 📏️ How wide a pill's own row is, on the same metric `retained_chrome_group_item_width` prices a
/// chrome control's label at — so the cancel control that follows it on the same row can be anchored
/// without measuring a glyph.
pub(crate) fn world3d_status_pill_width(theme: &Theme, pill: &World3dStatusPill) -> f32 {
    theme.padding_standard * 2.0 + pill.label().len() as f32 * theme.font_size_small * 0.6
}

/// ⌨️ One stop of the native shell's keyboard ring ([`ShellState::keyboard_ring`]): a focusable chrome control by
/// its accessibility key, or a dock window by its stack path and id.
#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) enum ShellKeyboardStop {
    Chrome(String),
    Window(Vec<usize>, String),
}

/// 👥️🖼️ One painted piece of board peer presence: a peer's cursor (with its viewport and name chip) or a
/// peer mark chip, each with the board it is clipped to.
#[derive(Clone, Debug)]
pub(crate) enum ShellBoardPresencePaint {
    Cursor { bounds: Rect, peer: crate::canvas_presence::BoardPeerCursor },
    Mark { bounds: Rect, rect: Rect, mark: crate::canvas_presence::BoardPeerMark },
}

/// 👥️🖼️ A presence chip's width for its text — the same estimate the status pills use.
pub(crate) fn board_presence_chip_width(theme: &Theme, text: &str) -> f32 {
    8.0 + text.chars().count() as f32 * theme.font_size_small * 0.6
}

/// 🖼️ Whether a surface is big enough to carry an overlay row at all — a collapsed dock pane paints
/// no chrome over its whole body.
pub(crate) fn surface_fits_overlay(theme: &Theme, bounds: Rect) -> bool {
    bounds.w >= theme.control_height * 4.0 && bounds.h >= theme.control_height * 2.0
}

/// 🛟️ Which axis an in-surface affordance is free to yield on — the Rust twin of React's `SafeAreaYield`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum SafeAreaYield {
    Inline,
    Block,
    Either,
}

/// 🛟️ How far an affordance must move off its anchor's own two edges to clear the chrome panels painted
/// over it — at most one axis is ever non-zero. The Rust twin of React's `ChromePanelSafeArea`.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub(crate) struct ChromePanelSafeArea {
    pub inline: f32,
    pub block: f32,
}

fn rects_overlap(a: Rect, b: Rect) -> bool {
    a.x < b.x + b.w && a.x + a.w > b.x && a.y < b.y + b.h && a.y + a.h > b.y
}

/// 🧭️ Sweep forbidden displacement intervals while preserving the axis gap and actual host bounds.
fn chrome_panel_axis_reserve(affordance: Rect, host: Rect, panels: &[Rect], inline: bool, direction: f32, gap: f32) -> f32 {
    if direction == 0.0 { return 0.0; }
    let axes = |rect: Rect| if inline { [rect.x, rect.x + rect.w, rect.y, rect.y + rect.h] } else { [rect.y, rect.y + rect.h, rect.x, rect.x + rect.w] };
    let a = axes(affordance);
    let h = axes(host);
    if a[2] < h[2] || a[3] > h[3] { return 0.0; }
    let mut intervals: Vec<(f32, f32)> = panels.iter().filter_map(|panel| {
        let p = axes(*panel);
        if a[2] >= p[3] || a[3] <= p[2] { return None; }
        Some(if direction > 0.0 { (p[0] - a[1] - gap, p[1] - a[0] + gap) } else { (a[0] - p[1] - gap, a[1] - p[0] + gap) })
    }).collect();
    intervals.sort_by(|a, b| a.0.total_cmp(&b.0));
    let mut offset = 0.0;
    for (start, end) in intervals {
        if start >= offset { break; }
        if end > offset { offset = end.ceil(); }
    }
    if offset > 0.0 && a[0] + direction * offset >= h[0] && a[1] + direction * offset <= h[1] { offset } else { 0.0 }
}

/// 🛟️ Clear every shell panel along one authored anchor axis, including cascaded obstacles. Preserve the
/// axis gap, round outward, and remain entirely inside the host. The least viable displacement wins;
/// block wins ties. No overlap or no fit leaves placement unchanged. React consumes the same corpus.
pub(crate) fn chrome_panel_safe_area(affordance: Rect, host: Rect, anchor: PanelAnchor, panels: &[Rect], yield_axis: SafeAreaYield, gap: f32) -> ChromePanelSafeArea {
    if !panels.iter().any(|panel| rects_overlap(*panel, affordance)) { return ChromePanelSafeArea::default(); }
    let block = if yield_axis == SafeAreaYield::Inline { 0.0 } else {
        chrome_panel_axis_reserve(affordance, host, panels, false, match anchor.vertical() { "top" => 1.0, "bottom" => -1.0, _ => 0.0 }, gap)
    };
    let inline = if yield_axis == SafeAreaYield::Block { 0.0 } else {
        chrome_panel_axis_reserve(affordance, host, panels, true, match anchor.horizontal() { "left" => 1.0, "right" => -1.0, _ => 0.0 }, gap)
    };
    if block > 0.0 && (inline == 0.0 || block <= inline) { return ChromePanelSafeArea { inline: 0.0, block }; }
    if inline > 0.0 { return ChromePanelSafeArea { inline, block: 0.0 }; }
    ChromePanelSafeArea::default()
}

/// 🛟️ The box a live surface's overlay row claims at its own top-left corner — the smallest row
/// `surface_fits_overlay` ever admits (`surfaceControlMinimum` in `🛑️surface-controls/🔣️.json`), so the
/// claim is stated before a single label has been measured. A chrome panel covering THAT box reserves.
pub(crate) fn surface_overlay_row_box(theme: &Theme, bounds: Rect) -> Rect {
    Rect::new(bounds.x + theme.gap_standard, bounds.y + theme.gap_standard, theme.control_height * 4.0, theme.control_height)
}

/// 🛟️ Where a live surface's overlay row actually starts once the open floating panels over it are
/// reserved — the wgpu twin of the React world pane's `useChromePanelSafeArea` rail offset. The status
/// pill and the cancel control that follows it share this one origin, so the row moves as one affordance.
pub(crate) fn surface_overlay_row_origin(theme: &Theme, bounds: Rect, panels: &[Rect]) -> [f32; 2] {
    let row = surface_overlay_row_box(theme, bounds);
    let safe_area = chrome_panel_safe_area(row, bounds, PanelAnchor::TopLeft, panels, SafeAreaYield::Either, theme.gap_standard);
    [row.x + safe_area.inline, row.y + safe_area.block]
}

/// ⏳️🖼️ The compute-status pills a set of live World3d surfaces asks for, each with the rect it is
/// painted in — pure over the surfaces' own bounds and status, so the laws drive it without a
/// `ShellState`. The pill leads the surface's overlay row; `surface_overlay_controls_for` anchors
/// the cancel control after it, which is the same left-to-right order React's pane lays out.
pub(crate) fn surface_status_pills_for(worlds: &[(&str, Rect, Option<&str>)], panels: &[Rect], theme: &Theme, is_de: bool) -> Vec<(World3dStatusPill, Rect)> {
    worlds
        .iter()
        .filter(|(_, bounds, _)| surface_fits_overlay(theme, *bounds))
        .filter_map(|(surface_id, bounds, status_json)| {
            let pill = world3d_status_pill_for(surface_id, &world3d_compute_status(*status_json), is_de)?;
            let origin = surface_overlay_row_origin(theme, *bounds, panels);
            let room = (bounds.x + bounds.w - theme.gap_standard - origin[0]).max(theme.control_height);
            let width = world3d_status_pill_width(theme, &pill).min(room);
            Some((pill, Rect::new(origin[0], origin[1], width, theme.control_height)))
        })
        .collect()
}

/// 🛑️🖼️ The overlay controls a set of live engine surfaces offers, with the anchor each is painted
/// at — pure over the surfaces' own bounds and status, so the laws drive it without a `ShellState`.
/// A surface too small to carry a control offers none, which is what keeps a collapsed dock pane from
/// painting chrome over its whole body.
pub(crate) fn surface_overlay_controls_for(_graphs: &[(&str, Rect)], worlds: &[(&str, Rect, Option<&str>)], panels: &[Rect], theme: &Theme, is_de: bool) -> Vec<(ShellNavbarControl, [f32; 2])> {
    let mut controls = Vec::new();
    for (surface_id, bounds, status_json) in worlds.iter().filter(|(_, bounds, _)| surface_fits_overlay(theme, *bounds)) {
        let status = world3d_compute_status(*status_json);
        if !status.cancellable {
            continue;
        }
        let lead = world3d_status_pill_for(surface_id, &status, is_de).map(|pill| world3d_status_pill_width(theme, &pill) + theme.gap_standard).unwrap_or(0.0);
        let control = ShellNavbarControl { control_id: format!("shell.world3d.cancel::{surface_id}"), icon_id: Some("x"), label: shell_chrome_string("common.cancel", is_de).to_string(), active: false, width_policy: None };
        let origin = surface_overlay_row_origin(theme, *bounds, panels);
        controls.push((control, [origin[0] + lead, origin[1]]));
    }
    controls
}

/// 🛑️ Reads the cancel contract out of one `statusJson`. The shell learns no domain verb from code:
/// whatever id `cancelAction` names is what the control dispatches.
pub(crate) fn world3d_cancel_affordance(status_json: Option<&str>) -> World3dCancelAffordance {
    let status = world3d_compute_status(status_json);
    World3dCancelAffordance { cancellable: status.cancellable, cancel_action: status.cancel_action, cancel_args: status.cancel_args }
}

/// ⏳️ A producer's own non-negative finite count, or `0` — the Rust twin of the TypeScript
/// `world3dComputeNumber`.
fn world3d_status_number(value: Option<&Value>) -> f64 {
    value.and_then(Value::as_f64).filter(|number| number.is_finite() && *number >= 0.0).unwrap_or(0.0)
}

/// 🌍️ A producer's `{en, de}` phase label, honoured only when BOTH tongues carry text — a pair with
/// one empty half would make one language silently fall back to the other, which is exactly the
/// default-language rule this codebase forbids.
fn world3d_status_label(value: Option<&Value>) -> Option<(String, String)> {
    let row = value?.as_object()?;
    let en = row.get("en").and_then(Value::as_str).filter(|text| !text.is_empty())?;
    let de = row.get("de").and_then(Value::as_str).filter(|text| !text.is_empty())?;
    Some((en.to_string(), de.to_string()))
}

/// ⏳️ The ONE reader of `World3dScene.statusJson` on this target — the Rust twin of
/// `world3dComputeStatusV1`, answering the same shared `🛑️surface-controls/🔣️.json` fixture.
pub(crate) fn world3d_compute_status(status_json: Option<&str>) -> World3dComputeStatus {
    let Some(raw) = status_json.filter(|json| !json.is_empty()) else {
        return World3dComputeStatus::default();
    };
    let Ok(Value::Object(row)) = serde_json::from_str::<Value>(raw) else {
        return World3dComputeStatus::default();
    };
    let progress = row.get("progress").and_then(Value::as_object);
    let field = |name: &str| progress.and_then(|progress| progress.get(name));
    let cancel_action = row.get("cancelAction").and_then(Value::as_str).unwrap_or_default().to_string();
    let units_total = world3d_status_number(field("unitsTotal"));
    let units_done = if units_total == 0.0 { world3d_status_number(field("unitsDone")) } else { world3d_status_number(field("unitsDone")).min(units_total) };
    let declared_ratio = field("ratio").and_then(Value::as_f64).filter(|ratio| ratio.is_finite());
    World3dComputeStatus {
        computing: row.get("computing") == Some(&Value::Bool(true)),
        phase: row.get("phase").and_then(Value::as_str).filter(|phase| !phase.is_empty()).unwrap_or("idle").to_string(),
        phase_label: world3d_status_label(row.get("phaseLabel")),
        units_done,
        units_total,
        faces_done: world3d_status_number(field("facesDone")),
        faces_total: world3d_status_number(field("facesTotal")),
        in_flight: world3d_status_number(field("inFlight")),
        ratio: match declared_ratio {
            Some(ratio) => ratio.clamp(0.0, 1.0),
            None if units_total > 0.0 => units_done / units_total,
            None => 1.0,
        },
        cancellable: row.get("cancellable") == Some(&Value::Bool(true)) && !cancel_action.is_empty(),
        cancel_action,
        cancel_args: row
            .get("cancelArgs")
            .and_then(Value::as_object)
            .map(|args| args.iter().filter(|(_, value)| value.is_string() || value.as_f64().is_some_and(f64::is_finite)).map(|(key, value)| (key.clone(), value.clone())).collect())
            .unwrap_or_default(),
    }
}
//#endregion 🔀️ChromeParity

//#region 🪟️WindowPaneChrome
/// 🪟️ One chip a window pane's own overlay rows carry. React mounts each as a FOLDED `Pane` inside
/// `window-body` at its own anchor (`🪟️Window/🟦️.tsx:320-412`), plus the world's projection pane
/// portalled into the same host (`🌐️World3dHost/🟦️.tsx:5188`): a top row `Actions` (leading) ·
/// `Search` (centre) · `Window Options` (trailing) and a bottom row `Utilities` (leading) ·
/// `Projection` (trailing).
///
/// 🩸️ wgpu painted NONE of them. Its only per-pane chrome was the Measures fold chip — an icon-only
/// square with no label — and the app's utilities sat on a FOOTER rail that React has no counterpart
/// for at all (`Transform · Brush · Volume Brush · Relocate` in the first real boot).
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum WindowPaneChip {
    Actions,
    Search,
    WindowOptions,
    Utilities,
    Projection,
}

impl WindowPaneChip {
    /// 🪟️ Every chip, in React's own mount order inside `window-body`.
    pub(crate) const ALL: [Self; 5] = [Self::Actions, Self::Search, Self::WindowOptions, Self::Utilities, Self::Projection];

    /// 🧭️ The `Pane` anchor React mounts this chip at.
    pub(crate) fn anchor(self) -> PanelAnchor {
        match self {
            Self::Actions => PanelAnchor::TopLeft,
            Self::Search => PanelAnchor::TopMiddle,
            Self::WindowOptions => PanelAnchor::TopRight,
            Self::Utilities => PanelAnchor::BottomLeft,
            Self::Projection => PanelAnchor::BottomRight,
        }
    }

    /// 🖼️ React's FIXED semantic pane icon, never a fold-direction chevron
    /// (`WINDOW_PANE_*_ICON`, `🖱️ui/🎯️targets/⚛️react/🟦️.tsx:8201-8204`).
    ///
    /// 🔀️ `Projection` is React's one exception: its `Pane` takes `icon={worldProjectionSpecIconId(spec)}`
    /// (`🌐️World3dHost/🟦️.tsx:5202`), so the chip wears the SELECTED template's icon. This answers the
    /// icon of [`WORLD_PROJECTION_DEFAULT_TEMPLATE_ID`] — the spec React synthesises for a pane whose
    /// camera declares none — and [`ShellState::window_pane_chip_icon_id`] resolves the live one.
    pub(crate) fn icon_id(self) -> &'static str {
        match self {
            Self::Actions => "play",
            Self::Search => "search",
            Self::WindowOptions => "settings-2",
            Self::Utilities => "hammer",
            Self::Projection => world_projection_template(WORLD_PROJECTION_DEFAULT_TEMPLATE_ID).icon_id,
        }
    }

    pub(crate) fn label_key(self) -> &'static str {
        match self {
            Self::Actions => "common.actions",
            Self::Search => "window.search.title",
            Self::WindowOptions => "common.windowOptions",
            Self::Utilities => "common.utilities",
            Self::Projection => "common.projection",
        }
    }

    /// 🎯️ The control id this chip's press dispatches — React's `toggleId` VERBATIM, carrying the same
    /// fold/unfold split. `Actions` and `Search` each drive the state of their own pane.
    ///
    /// 🩸️ This said "the wgpu twin of React's `toggleId`" while minting `shell.action.fold.<windowId>`
    /// and four siblings — a shell-private grammar with the window at the END. React mints
    /// `framework.window.<windowIdSegment>.engagement.toggle`, so NOTHING resolved the same chip on
    /// the two renderers, and a behavioural journey addressed by control id could only report every
    /// pane chip as absent on one side (ticket 26/09/17/WGPU-RENDERER-REACT-PARITY,
    /// `📓️w9c-behaviour-parity-run-2.md` steps 9–12). The id is the integration key across i18n,
    /// tooltips, hotkeys, tutorials and introduction anchors — one spelling, or none of them meet.
    pub(crate) fn control_id(self, window_id: &str, folded: bool) -> String {
        match self {
            Self::Actions => semio_framework::child_element_id(WINDOW_PANE_CHIP_PARENT, &[window_id, "engagement", "toggle"]),
            Self::Search => semio_framework::child_element_id(WINDOW_PANE_CHIP_PARENT, &[window_id, "search", "toggle"]),
            Self::WindowOptions if folded => semio_framework::child_element_id(WINDOW_PANE_CHIP_PARENT, &[window_id, "measures", "unfold"]),
            Self::WindowOptions => semio_framework::child_element_id(WINDOW_PANE_CHIP_PARENT, &[window_id, "measures", "fold"]),
            Self::Utilities if folded => semio_framework::child_element_id(WINDOW_PANE_CHIP_PARENT, &[window_id, "utilityBar", "unfold"]),
            Self::Utilities => semio_framework::child_element_id(WINDOW_PANE_CHIP_PARENT, &[window_id, "utilityBar", "fold"]),
            Self::Projection => semio_framework::child_element_id(WORLD_PROJECTION_PANE_PARENT, &[window_id, "pane", "fold"]),
        }
    }

    /// 🎯️ The chip a control id names, and the `(pane, verb)` pair it is spelled with — the exact
    /// inverse of [`Self::control_id`], so one table states both directions.
    fn from_pane_and_verb(pane: &str, verb: &str) -> Option<Self> {
        match (pane, verb) {
            ("engagement", "toggle") => Some(Self::Actions),
            ("search", "toggle") => Some(Self::Search),
            ("measures", "fold" | "unfold") => Some(Self::WindowOptions),
            ("utilityBar", "fold" | "unfold") => Some(Self::Utilities),
            ("pane", "fold") => Some(Self::Projection),
            _ => None,
        }
    }
}

/// 🆔️ React's parent id for every window-scoped pane chip — `childElementId("framework.window", id, …)`
/// in `🪟️Window/🟦️.tsx`, where `id` is the window INSTANCE id.
pub(crate) const WINDOW_PANE_CHIP_PARENT: &str = "framework.window";

/// 🆔️ React's parent id for the world's projection pane — `world3dProjectionPaneElementId`
/// (`🌐️World3dHost/🟦️.tsx:5218`), which is NOT under `framework.window`, and whose `Pane` declares no
/// `toggleId` and therefore wears the default `childElementId(id, "pane", "fold")`.
pub(crate) const WORLD_PROJECTION_PANE_PARENT: &str = "framework.worldOrbit.projection";

/// 🧰️ The routing namespace every row of a pane's UNFOLDED Utilities rail carries, its TAIL being the
/// utility's own framework element id — the id React renders on the leaf (`UtilityTree`'s
/// `id={entry.id}`, `🎛️UtilityTree/🟦️.tsx:237`) and the id an introduction step addresses it by, which
/// this renderer also publishes through `ShellChromeBuildState::register_element_rect`. React needs no
/// namespace because its rail is DOM above the canvas; this renderer routes a press by id, and the rail
/// is painted INSIDE a world pane's rect, so without a claimable prefix every utility press was handed
/// to the world surface instead of the shell — the same gap the pane chips had
/// (`📓️w12d-pane-chip-ids-and-projection-toggle.md` §3).
pub(crate) const WINDOW_UTILITY_RAIL_PARENT: &str = "framework.utility.";

/// 🔀️ One selectable row of the Projection pane — the flattened Rust twin of React's
/// `createWorldProjectionTemplates` taxonomy (`♾️infinite/🌍️world/🎨️r3f/🟦️.tsx:3097`):
/// `Parallel > Orthographic | Axonometric (Isometric/Dimetric/Trimetric) | Oblique
/// (Cabinet/Cavalier/Military)` and `Perspective > 1-Point/2-Point/3-Point/Curvilinear`, in that
/// order, each carrying its own `projection-*` icon.
///
/// 🌍️ The labels are FROZEN English on both renderers: React's templates hard-code them
/// (`leaf("orthographic", "Orthographic", …)`) rather than resolving a `LocalizedLabel`, so
/// translating them here would make the two shells read differently for the same row.
#[derive(Clone, Copy)]
pub(crate) struct WorldProjectionTemplate {
    pub id: &'static str,
    pub label: &'static str,
    pub icon_id: &'static str,
    /// 🌲️ Depth in React's template tree — a branch is 0, its leaves are 1.
    pub depth: u8,
    pub mode: Viewport3dProjectionMode,
}

impl WorldProjectionTemplate {
    /// 📐️ React's complete template spec for a newly opened pane.
    pub(crate) fn spec(self) -> Viewport3dProjectionSpec {
        let orientation = match self.mode {
            Viewport3dProjectionMode::Orthographic {} => Viewport3dProjectionOrientation::Cardinal { view: Viewport3dOrthographicView::Plan },
            Viewport3dProjectionMode::Axonometric { .. } => Viewport3dProjectionOrientation::Corner { quadrant: Viewport3dAxonometricQuadrant::Ne, hemisphere: Some(Viewport3dAxonometricHemisphere::Upper) },
            Viewport3dProjectionMode::Oblique { variant: Viewport3dObliqueVariant::Military, .. } => Viewport3dProjectionOrientation::Cardinal { view: Viewport3dOrthographicView::Plan },
            Viewport3dProjectionMode::Oblique { .. } | Viewport3dProjectionMode::OnePoint { .. } => Viewport3dProjectionOrientation::Cardinal { view: Viewport3dOrthographicView::Front },
            _ => Viewport3dProjectionOrientation::Free {},
        };
        Viewport3dProjectionSpec { mode: self.mode, orientation }
    }
}

/// 🔀️ React's projection taxonomy, depth-first, branch before its own leaves.
pub(crate) const WORLD_PROJECTION_TEMPLATES: &[WorldProjectionTemplate] = &[
    WorldProjectionTemplate { id: "parallel", label: "Parallel", icon_id: "projection-parallel", depth: 0, mode: Viewport3dProjectionMode::Orthographic {} },
    WorldProjectionTemplate { id: "orthographic", label: "Orthographic", icon_id: "projection-orthographic", depth: 1, mode: Viewport3dProjectionMode::Orthographic {} },
    WorldProjectionTemplate { id: "axonometric", label: "Axonometric", icon_id: "projection-axonometric", depth: 1, mode: Viewport3dProjectionMode::Axonometric { variant: Viewport3dAxonometricVariant::Isometric, angle_a: 30.0, angle_b: 30.0 } },
    WorldProjectionTemplate { id: "axonometric-isometric", label: "Isometric", icon_id: "projection-isometric", depth: 2, mode: Viewport3dProjectionMode::Axonometric { variant: Viewport3dAxonometricVariant::Isometric, angle_a: 15.0, angle_b: 30.0 } },
    WorldProjectionTemplate { id: "axonometric-dimetric", label: "Dimetric", icon_id: "projection-dimetric", depth: 2, mode: Viewport3dProjectionMode::Axonometric { variant: Viewport3dAxonometricVariant::Dimetric, angle_a: 15.0, angle_b: 15.0 } },
    WorldProjectionTemplate { id: "axonometric-trimetric", label: "Trimetric", icon_id: "projection-trimetric", depth: 2, mode: Viewport3dProjectionMode::Axonometric { variant: Viewport3dAxonometricVariant::Trimetric, angle_a: 12.0, angle_b: 42.0 } },
    WorldProjectionTemplate { id: "oblique", label: "Oblique", icon_id: "projection-oblique", depth: 1, mode: Viewport3dProjectionMode::Oblique { variant: Viewport3dObliqueVariant::Cavalier, angle: 45.0, depth_scale: 1.0 } },
    WorldProjectionTemplate { id: "oblique-cabinet", label: "Cabinet", icon_id: "projection-oblique-cabinet", depth: 2, mode: Viewport3dProjectionMode::Oblique { variant: Viewport3dObliqueVariant::Cabinet, angle: 45.0, depth_scale: 0.5 } },
    WorldProjectionTemplate { id: "oblique-cavalier", label: "Cavalier", icon_id: "projection-oblique-cavalier", depth: 2, mode: Viewport3dProjectionMode::Oblique { variant: Viewport3dObliqueVariant::Cavalier, angle: 45.0, depth_scale: 1.0 } },
    WorldProjectionTemplate { id: "oblique-military", label: "Military", icon_id: "projection-oblique-military", depth: 2, mode: Viewport3dProjectionMode::Oblique { variant: Viewport3dObliqueVariant::Military, angle: 45.0, depth_scale: 1.0 } },
    WorldProjectionTemplate { id: "perspective", label: "Perspective", icon_id: "projection-perspective", depth: 0, mode: Viewport3dProjectionMode::ThreePoint { fov: 50.0 } },
    WorldProjectionTemplate { id: "one-point", label: "1-Point", icon_id: "projection-one-point", depth: 1, mode: Viewport3dProjectionMode::OnePoint { fov: 50.0 } },
    WorldProjectionTemplate { id: "two-point", label: "2-Point", icon_id: "projection-two-point", depth: 1, mode: Viewport3dProjectionMode::TwoPoint { fov: 50.0, vertical_shift: 0.0 } },
    WorldProjectionTemplate { id: "three-point", label: "3-Point", icon_id: "projection-three-point", depth: 1, mode: Viewport3dProjectionMode::ThreePoint { fov: 50.0 } },
    WorldProjectionTemplate { id: "curvilinear", label: "Curvilinear", icon_id: "projection-curvilinear", depth: 1, mode: Viewport3dProjectionMode::Curvilinear { fov: 120.0, strength: 1.0, mapping: Viewport3dCurvilinearMapping::Fisheye } },
];

/// 🔀️ The template a pane with no explicit selection reads. React resolves
/// `cameraState.projectionSpec ?? worldProjectionDefaults("threePoint")` for a non-orthographic
/// camera (`🌐️World3dHost/🟦️.tsx`'s `worldProjectionSpec`), and this renderer's orbit is
/// perspective-only (`♾️infinite/🌍️world/🦀️.rs`'s `WORLD3D_PERSPECTIVE_CAMERA_ZOOM`), so both
/// shells open on 3-Point.
pub(crate) const WORLD_PROJECTION_DEFAULT_TEMPLATE_ID: &str = "three-point";

/// ⏱️ The ceiling one unfolded projection body may spend inside the chrome walk — its own paint opportunities, not the whole
/// walk's, and only its own WORK (`document_opportunity_remains`): polls of the layout worker pool and steps of other surfaces'
/// layouts are not the pane's. The pane's content is React's fixed taxonomy (18 retained nodes), and its own work is priced per
/// grant: two layout passes (the pane re-solves once it hugs its measured content) of ~240 own steps each — every worker job
/// step costs two opportunities on this thread (its outcome is taken, then applied), plus node collection and session close —
/// and ~800 paint grants of one glyph or node phase each: 1 405 in WG11's overlay build 7, where a 1 024 ceiling faulted the
/// converging pane at paint progress 418. Nearly three times that fits; a body that reaches it has stopped converging, which is
/// a fault, not a slow frame.
pub(crate) const WORLD_PROJECTION_PANE_PAINT_OPPORTUNITIES: usize = 4096;

const WORLD_PROJECTION_TEMPLATE_PREFIX: &str = "world-projection:";

fn decoded_world_projection_template(template_id: &str) -> Option<Viewport3dProjectionSpec> {
    serde_json::from_str(template_id.strip_prefix(WORLD_PROJECTION_TEMPLATE_PREFIX)?).ok()
}

pub(crate) fn world_projection_template_selection_id(template_id: &str) -> Option<&'static str> {
    if let Some(template) = WORLD_PROJECTION_TEMPLATES.iter().find(|template| template.id == template_id) {
        return Some(template.id);
    }
    Some(match decoded_world_projection_template(template_id)?.mode {
        Viewport3dProjectionMode::Orthographic {} => "orthographic",
        Viewport3dProjectionMode::Axonometric { variant: Viewport3dAxonometricVariant::Isometric, .. } => "axonometric-isometric",
        Viewport3dProjectionMode::Axonometric { variant: Viewport3dAxonometricVariant::Dimetric, .. } => "axonometric-dimetric",
        Viewport3dProjectionMode::Axonometric { variant: Viewport3dAxonometricVariant::Trimetric, .. } => "axonometric-trimetric",
        Viewport3dProjectionMode::Oblique { variant: Viewport3dObliqueVariant::Cabinet, .. } => "oblique-cabinet",
        Viewport3dProjectionMode::Oblique { variant: Viewport3dObliqueVariant::Cavalier, .. } => "oblique-cavalier",
        Viewport3dProjectionMode::Oblique { variant: Viewport3dObliqueVariant::Military, .. } => "oblique-military",
        Viewport3dProjectionMode::OnePoint { .. } => "one-point",
        Viewport3dProjectionMode::TwoPoint { .. } => "two-point",
        Viewport3dProjectionMode::ThreePoint { .. } => "three-point",
        Viewport3dProjectionMode::Curvilinear { .. } => "curvilinear",
    })
}

fn world_projection_icon_id_v1(template_id: &str) -> Option<&'static str> {
    world_projection_template_selection_id(template_id).and_then(|id| WORLD_PROJECTION_TEMPLATES.iter().find(|template| template.id == id)).map(|template| template.icon_id)
}

/// 🖼️ Resolves one accepted dock tab through React's `windowIconsById[id] ?? projection ?? kind`
/// precedence. The projection arm remains a fallback for a newly mounted World pane whose local
/// `SetWindowIconContext` publication has not run yet.
pub(crate) fn dock_tab_icon_id_v1(kind_icon_id: &str, override_icon_id: Option<&str>, template_id: Option<&str>) -> String {
    override_icon_id.filter(|icon_id| !icon_id.is_empty()).or_else(|| template_id.and_then(world_projection_icon_id_v1)).unwrap_or(kind_icon_id).to_string()
}

pub(crate) fn world_projection_initial_seed(template_id: &str) -> Option<Viewport3dProjectionSpec> {
    WORLD_PROJECTION_TEMPLATES.iter().find(|template| template.id == template_id).copied().map(WorldProjectionTemplate::spec).or_else(|| decoded_world_projection_template(template_id))
}

/// 🔀️ The template row `id` names, or the default one for an id no template declares.
pub(crate) fn world_projection_template(id: &str) -> &'static WorldProjectionTemplate {
    let id = world_projection_template_selection_id(id).unwrap_or(WORLD_PROJECTION_DEFAULT_TEMPLATE_ID);
    WORLD_PROJECTION_TEMPLATES.iter().find(|template| template.id == id).or_else(|| WORLD_PROJECTION_TEMPLATES.iter().find(|template| template.id == WORLD_PROJECTION_DEFAULT_TEMPLATE_ID)).expect("the default projection template is declared")
}

/// 🔀️ Materializes React's nested Projection Tree for one exact window pane. Row ids follow
/// `childElementId(paneId, template.id)` at every level. Nesting is structural; React deliberately
/// keeps every row id directly qualified by the pane rather than encoding its ancestor path.
fn world_projection_pane_rows(prefix: &str, window_id: &str, selected_id: &str, depth: u8, cursor: &mut usize) -> Vec<UiTreeItemNode> {
    let mut rows = Vec::new();
    while let Some(template) = WORLD_PROJECTION_TEMPLATES.get(*cursor) {
        if template.depth < depth {
            break;
        }
        if template.depth > depth {
            *cursor += 1;
            continue;
        }
        let id = semio_framework::child_element_id(prefix, &[template.id]);
        *cursor += 1;
        let children = world_projection_pane_rows(prefix, window_id, selected_id, depth + 1, cursor);
        rows.push(UiTreeItemNode {
            id,
            label: Label::data(template.label),
            icon_id: IconName::from_str(template.icon_id),
            presence: UiPresence { selected: template.id == selected_id, ..UiPresence::default() },
            default_open: Some(true),
            action: Some(ActionDescriptor {
                controller_id: "framework".into(),
                action: "setWorldProjectionTemplate".into(),
                args: crate::action_args_json!({ "windowId": window_id.to_string(), "templateId": template.id }),
            }),
            items: (!children.is_empty()).then_some(children),
            ..UiTreeItemNode::base(String::new(), Label::data(String::new()))
        });
    }
    rows
}

/// 📐️ The box one pane overlay chip occupies inside a window body — React's `anchorPositionStyle`
/// (`🖱️ui/🎯️targets/⚛️react/🟦️.tsx:6864`) on this target's own token: a corner anchor insets
/// `--spacing-single` (`PANEL_INSET_UI_SPACING`, the same 1 × ui-spacing `theme.panel_inset` carries)
/// on BOTH its edges, a middle anchor centres on that axis. One chip per anchor, so a band is one box.
pub(crate) fn window_pane_chip_rect(theme: &Theme, body: Rect, anchor: PanelAnchor, width: f32) -> Rect {
    let x = match anchor.horizontal() {
        "left" => body.x + theme.panel_inset,
        "right" => body.x + body.w - theme.panel_inset - width,
        _ => body.x + (body.w - width) * 0.5,
    };
    let y = match anchor.vertical() {
        "top" => body.y + theme.panel_inset,
        "bottom" => body.y + body.h - theme.panel_inset - theme.control_height,
        _ => body.y + (body.h - theme.control_height) * 0.5,
    };
    Rect::new(x, y, width, theme.control_height)
}

impl ShellState {
    pub(crate) fn world3d_host_id_for_window(&self, window_id: &str) -> Option<&str> {
        self.world3d_window_ids.iter().find_map(|(host_id, owner_window_id)| (owner_window_id == window_id).then_some(host_id.as_str()))
    }

    pub(crate) fn world3d_state_for_window(&self, window_id: &str) -> Option<&World3dState> {
        self.world3d_states.get(self.world3d_host_id_for_window(window_id)?)
    }

    pub(crate) fn world3d_state_for_window_mut(&mut self, window_id: &str) -> Option<&mut World3dState> {
        let host_id = self.world3d_host_id_for_window(window_id)?.to_string();
        self.world3d_states.get_mut(&host_id)
    }

    pub(crate) fn engine_host_id_for_window(&self, window_id: &str) -> Option<&str> {
        self.world3d_host_id_for_window(window_id)
            .or_else(|| self.node_graph_states.iter().find_map(|(host_id, surface)| (surface.window_id == window_id).then_some(host_id.as_str())))
            .or_else(|| self.tiled_map_states.iter().find_map(|(host_id, surface)| (surface.window_id == window_id).then_some(host_id.as_str())))
            .or_else(|| self.board2d_states.iter().find_map(|(host_id, surface)| (surface.window_id == window_id).then_some(host_id.as_str())))
    }

    /// 📐️ Hands a dock template to its World surface once the producer's delivered camera is
    /// resident. The map records template identity before scene paint creates the surface owner, so
    /// this boundary retries harmlessly until both halves exist.
    pub(crate) fn apply_initial_world_projection_template(&mut self, window_id: &str) {
        let Some(template_id) = self.world_projection_template.get(window_id) else { return };
        let Some(seed) = world_projection_initial_seed(template_id) else { return };
        let Some(world) = self.world3d_state_for_window_mut(window_id) else { return };
        let _ = infinite_world::world::apply_world3d_initial_projection_seed(world, seed);
    }

    /// 🎛️ React's `Window` `utilityBarFolded` — `useState(true)`, so a pane's Utilities rail starts
    /// FOLDED and is opened by its own chip (`🪟️Window/🟦️.tsx:176-177`).
    pub(crate) fn utility_bar_folded(&self, window_id: &str) -> bool {
        self.utility_bar_folded.get(window_id).copied().unwrap_or(true)
    }

    /// 🔀️ React's `WorldOrbitProjectionSwitchPane` `folded` — `useState(true)`
    /// (`🌐️World3dHost/🟦️.tsx:5190`), so absent means folded.
    pub(crate) fn projection_pane_folded(&self, window_id: &str) -> bool {
        self.projection_pane_folded.get(window_id).copied().unwrap_or(true)
    }

    /// 🔀️ The projection template this pane shows as selected — React's `worldProjectionSpec`
    /// fallback chain (`cameraState.projectionSpec ?? (cameraState.projection === "orthographic" ?
    /// worldProjectionDefaults("orthographic") : worldProjectionDefaults("threePoint"))`,
    /// `🌐️World3dHost/🟦️.tsx`), collapsed onto this renderer's flat template ids: an explicit press
    /// wins, otherwise the pane's LIVE camera family answers, which is what makes a `Top` pane open
    /// on Orthographic instead of the static default.
    pub(crate) fn world_projection_template_id(&self, window_id: &str) -> &str {
        if let Some(selected) = self.world_projection_template.get(window_id) {
            return world_projection_template(selected).id;
        }
        match self.world3d_state_for_window(window_id).map(infinite_world::world::world3d_camera_projection) {
            Some(ui_wgpu::wgpu::CameraProjection3d::Orthographic) => "orthographic",
            _ => WORLD_PROJECTION_DEFAULT_TEMPLATE_ID,
        }
    }

    /// 🎛️ React's `Window` Actions fold; absent means folded.
    pub(crate) fn window_actions_folded(&self, window_id: &str) -> bool {
        self.action_panel_folded.get(window_id).copied().unwrap_or(true)
    }

    /// 🔎️ React's `Window` Search fold; absent means folded.
    pub(crate) fn window_search_folded(&self, window_id: &str) -> bool {
        self.search_panel_folded.get(window_id).copied().unwrap_or(true)
    }

    /// 🪟️ The overlay chips ONE pane paints this frame, each with its fold state and whether its
    /// toggle is live. React mounts the projection pane only from inside a world surface
    /// (`🌐️World3dHost/🟦️.tsx:5210`) and mounts every other pane unconditionally, DISABLING the
    /// toggle of one with no body to open (`toggleDisabled={!utilityBar}`, `🪟️Window/🟦️.tsx:403`) —
    /// which is the rule this follows for the Utilities rail and the Measures rail alike.
    ///
    /// 🔀️ `Projection` is mounted ENABLED for every world pane, because React's never is anything
    /// else: `WorldOrbitProjectionSwitchPane` passes no `toggleDisabled` at all and is handed
    /// `cameraState.projectionSpec ?? worldProjectionDefaults(…)` (`🌐️World3dHost/🟦️.tsx:6598`), so
    /// the spec is SYNTHESISED whenever the camera carries none and the chip is always pressable.
    ///
    /// 🩸️ It used to be `(chip, true, true)` — permanently disabled — on the reading that a pane
    /// with no `WorldProjectionSpec` is React's disabled-toggle case. That reading was wrong, and
    /// because `paint_window_pane_chips_step` registers no hit for a disabled chip it made the ONLY
    /// painted chip in the whole shell with no hit target: every sibling registered one, this one
    /// did not (`📓️audit-visual-parity-puzzle3d.md` §3 P0).
    ///
    /// 🎬️ React mounts the Actions `Pane` only for `engagementVisible = !!(engagement || actionPane)`
    /// and the Search `Pane` only for a `search` spec (`🪟️Window/🟦️.tsx:205`/`:214`/`:379`/`:401`),
    /// and BOTH specs are `undefined` when their payload is empty (`windowEngagementToSpec`/
    /// `windowEngagementToSearchSpec` answer `undefined` for `!hasContent`,
    /// `🛠️ShellHelpers/🟦️.tsx:1852`/`:1874`; `windowActionPaneNode` answers `undefined` for a kind
    /// with no panel-eligible action, `:4260`). A chip with no body behind it is not React's
    /// disabled-toggle case — that case is the Utilities/Measures rail, which React mounts
    /// unconditionally — so these two do not mount at all.
    pub(crate) fn window_pane_chips(&self, window_id: &str) -> Vec<(WindowPaneChip, bool, bool)> {
        let has_utilities = self.session.as_ref().is_some_and(|session| !self.derive_window_utility_nodes(session, window_id).is_empty());
        WindowPaneChip::ALL
            .into_iter()
            .filter_map(|chip| match chip {
                WindowPaneChip::WindowOptions => Some((chip, self.measures_rail_folded(window_id), !self.window_measures_documents.contains_key(window_id))),
                WindowPaneChip::Utilities => Some((chip, self.utility_bar_folded(window_id), !has_utilities)),
                WindowPaneChip::Projection => self.world3d_host_id_for_window(window_id).is_some().then_some((chip, self.projection_pane_folded(window_id), false)),
                WindowPaneChip::Actions => (self.window_has_engagement_spec(window_id) || self.window_has_action_pane(window_id)).then_some((chip, self.window_actions_folded(window_id), false)),
                WindowPaneChip::Search => self.window_has_search_spec(window_id).then_some((chip, self.window_search_folded(window_id), false)),
            })
            .collect()
    }

    /// 🎯️ The live window instance and the chip a pane-chip control id addresses — the read side of
    /// [`WindowPaneChip::control_id`].
    ///
    /// ⚖️ The window is RESOLVED, never parsed back out: `element_id_segment` is lossy
    /// (`puzzle3d-main-top` → `puzzle3dMainTop`), so the segment is matched against the instances this
    /// frame actually carries — which is also what refuses a chip addressed to a window that has since
    /// closed.
    pub(crate) fn window_pane_chip_target(&self, control_id: &str) -> Option<(String, WindowPaneChip)> {
        let (parent, rest) = [WINDOW_PANE_CHIP_PARENT, WORLD_PROJECTION_PANE_PARENT].into_iter().find_map(|parent| control_id.strip_prefix(parent).and_then(|rest| rest.strip_prefix('.')).map(|rest| (parent, rest)))?;
        let (segment, tail) = rest.split_once('.')?;
        let (pane, verb) = tail.split_once('.')?;
        let chip = WindowPaneChip::from_pane_and_verb(pane, verb)?;
        if matches!(chip, WindowPaneChip::Projection) != (parent == WORLD_PROJECTION_PANE_PARENT) {
            return None;
        }
        self.dock.window_instances().into_iter().map(|(window, _)| window).chain(self.live_window_ids()).find(|window| semio_framework::element_id_segment(window) == segment).map(|window| (window, chip))
    }

    /// 🎛️ Flips one pane chip's own fold state, and NOTHING else — React's chips are fold toggles.
    ///
    /// ⚖️ Every chip first activates its own window, because React activates a window on any pointer
    /// that lands in it (`GhostRegionShell`'s `onPointerDownCapture` → `onActivate`,
    /// `🪟️Window/🟦️.tsx:302`), and a chip is painted inside the window body.
    ///
    /// ⚖️ The five folds are independent. No chip closes another pane's fold, and none of them
    /// opens or closes a SURFACE — every `Pane` holds its own `useState(true)`
    /// (`🪟️Window/🟦️.tsx:169`/`:176`/`:183`, `WorldOrbitProjectionSwitchPane`'s at
    /// `🌐️World3dHost/🟦️.tsx:5225`).
    ///
    /// 🩸️ `Search` used to also flip `search_open`/`overlay_state`, i.e. open the SHELL's own
    /// centred command palette (the `ui.search.toggle` / `⌘K` overlay). React's Search chip opens a
    /// window-scoped `Pane` and focuses its input; it never opens the shell overlay, so the wgpu chip
    /// raised a surface React does not raise (`📓️w12d-pane-chip-ids-and-projection-toggle.md` §2).
    pub(crate) fn toggle_window_pane_chip(&mut self, window_id: &str, chip: WindowPaneChip) -> Result<(), String> {
        if self.dock.activate_window(window_id) {
            self.active_window_id = Some(window_id.to_string());
        }
        match chip {
            WindowPaneChip::WindowOptions => {
                let folded = self.measures_rail_folded(window_id);
                self.measures_folded.insert(window_id.to_string(), !folded);
            }
            WindowPaneChip::Utilities => {
                let folded = self.utility_bar_folded(window_id);
                self.utility_bar_folded.insert(window_id.to_string(), !folded);
            }
            WindowPaneChip::Projection => {
                let folded = self.projection_pane_folded(window_id);
                self.projection_pane_folded.insert(window_id.to_string(), !folded);
                if folded {
                    self.refresh_window_projection_document(window_id)?;
                } else {
                    let previous = self.window_projection_documents.remove(window_id);
                    self.retire_one_surface_document(previous)?;
                }
            }
            WindowPaneChip::Actions => {
                let folded = self.window_actions_folded(window_id);
                self.action_panel_folded.insert(window_id.to_string(), !folded);
            }
            WindowPaneChip::Search => {
                let folded = self.window_search_folded(window_id);
                self.search_panel_folded.insert(window_id.to_string(), !folded);
            }
        }
        Ok(())
    }

    /// 🖼️ The icon ONE pane chip wears this frame — [`WindowPaneChip::icon_id`] for every fixed one,
    /// and the SELECTED projection template's icon for `Projection`, which is the one React resolves
    /// per pane (`icon={worldProjectionSpecIconId(spec)}`, `🌐️World3dHost/🟦️.tsx:5202`).
    pub(crate) fn window_pane_chip_icon_id(&self, chip: WindowPaneChip, window_id: &str) -> &'static str {
        match chip {
            WindowPaneChip::Projection => world_projection_template(self.world_projection_template_id(window_id)).icon_id,
            _ => chip.icon_id(),
        }
    }

    /// 🖌️ One paint opportunity of ONE pane's overlay chip row. Steps chip by chip through
    /// `cursor.scalar` so a long label never blocks the chrome walk, and registers each chip's hit
    /// above the window body's own `ScrollRegion`. `true` once the whole row is off the critical path.
    ///
    /// 🫧️ ONE glass region per chip, pushed on its first opportunity and RE-ENTERED on every later
    /// one — pushing a region per paint opportunity would stack the same frosted band until the
    /// fixed layer filled, and painting the chip outside its own content layer is precisely the
    /// defect W3c §3 found (the glass samples the label away again).
    ///
    /// ⏪️ After the pane's own chips, an open history-edit session adds its time-travel indicator
    /// ([`Self::paint_window_time_travel_indicator_step`]).
    #[allow(clippy::too_many_arguments, reason = "the chrome walk's own paint context, forwarded unchanged")]
    pub(crate) fn paint_window_pane_chips_step(&mut self, cursor: &mut ShellChromeChildCursor, draw: &mut DrawList, atlas: &mut FontAtlas, icons: &IconAtlas, input: &mut InputState<ActionDescriptor>, theme: &Theme, window_id: &str, window_rect: Rect) -> bool {
        if !surface_fits_overlay(theme, window_rect) {
            return true;
        }
        let chips = self.window_pane_chips(window_id);
        let Some((chip, folded, disabled)) = chips.get(cursor.scalar).copied() else {
            return self.paint_window_time_travel_indicator_step(cursor, draw, atlas, icons, input, theme, window_id, window_rect, chips.len());
        };
        let is_de = self.locale_id == "de";
        let label = shell_chrome_string(chip.label_key(), is_de);
        let control_id = chip.control_id(window_id, folded);
        let item = ChromeGroupItem { control_id: control_id.as_str(), icon_id: Some(self.window_pane_chip_icon_id(chip, window_id)), label: Some(label), active: !folded, disabled, kind: HitKind::Toggle };
        if cursor.rect.is_none() {
            let Some(width) = retained_chrome_group_item_width(atlas, theme, &item) else {
                self.error = Some("Shell pane chip exceeded the retained chrome boundary".to_string());
                return true;
            };
            cursor.rect = Some(window_pane_chip_rect(theme, window_rect, chip.anchor(), width));
        }
        let Some(rect) = cursor.rect else { return true };
        if cursor.depth == 0 {
            cursor.depth = draw.push_glass([rect.x, rect.y, rect.w, rect.h], 0.0, theme.glass(Level::Window)).saturating_add(1);
        }
        draw.begin_glass_content(cursor.depth - 1);
        let step = render_retained_chrome_group_item_step(&mut cursor.group_phase, &mut cursor.glyph, draw, atlas, icons, input, theme, rect, &item, false);
        draw.end_glass_content();
        match step {
            RetainedChromeGroupStep::Pending => return false,
            RetainedChromeGroupStep::Complete => {}
            RetainedChromeGroupStep::Fault => self.error = Some("Shell pane chip exceeded the retained glyph boundary".to_string()),
        }
        if !disabled {
            note_chrome_group_item(&item);
            with_chrome_control_names(|names| {
                if let Some(presentation) = names.get_mut(item.control_id) {
                    presentation.role = Some("button");
                }
            });
            self.pane_overlay_hits.push(HitTarget { rect, event: None, control_id: Some(control_id), kind: HitKind::Toggle, drag_axis: None, drag_data: None });
        }
        cursor.rect = None;
        cursor.depth = 0;
        cursor.scalar += 1;
        false
    }

    /// 🧰️ One paint opportunity of ONE pane's UNFOLDED Utilities rail — the app's own utility tree,
    /// walked with the very stepper the footer rail used, laid out to the RIGHT of the `Utilities`
    /// chip on the pane's bottom-left row. This is where React puts them (`utilityBarNode(utilities,
    /// instance.id, …)` handed to the `bottom-left` `Pane`, `🏛️ShellHost/🟦️.tsx:10263`).
    ///
    /// 🖼️ The rail's own surface, pushed ONCE — React's unfolded `Pane` body carries a panel
    /// background, and a rail painted straight onto a 3D scene is unreadable.
    #[allow(clippy::too_many_arguments, reason = "the chrome walk's own paint context, forwarded unchanged")]
    pub(crate) fn paint_window_utilities_step(&mut self, cursor: &mut ShellChromeChildCursor, draw: &mut DrawList, atlas: &mut FontAtlas, icons: &IconAtlas, input: &mut InputState<ActionDescriptor>, theme: &Theme, window_id: &str, window_rect: Rect) -> bool {
        if self.utility_bar_folded(window_id) || !surface_fits_overlay(theme, window_rect) {
            return true;
        }
        let Some(session) = self.session.clone() else { return true };
        let utilities = self.derive_window_utility_nodes(&session, window_id);
        if utilities.is_empty() {
            return true;
        }
        let btn_y = window_rect.y + window_rect.h - theme.panel_inset - theme.control_height;
        if cursor.x <= 0.0 {
            let chip = ChromeGroupItem {
                control_id: "",
                icon_id: Some(WindowPaneChip::Utilities.icon_id()),
                label: Some(shell_chrome_string(WindowPaneChip::Utilities.label_key(), self.locale_id == "de")),
                active: true,
                disabled: false,
                kind: HitKind::Toggle,
            };
            let chip_w = retained_chrome_group_item_width(atlas, theme, &chip).unwrap_or(theme.control_height);
            cursor.x = window_pane_chip_rect(theme, window_rect, PanelAnchor::BottomLeft, chip_w).x + chip_w + theme.gap_standard;
            draw.push_solid([cursor.x, btn_y, (window_rect.x + window_rect.w - theme.panel_inset - cursor.x).max(0.0), theme.control_height], theme.panel);
        }
        if cursor.flag {
            let Some(next_depth) = cursor.depth.checked_add(1).filter(|depth| *depth < cursor.path.0.len()) else { return true };
            cursor.depth = next_depth;
            cursor.path.0[next_depth] = 0;
            cursor.flag = false;
            return false;
        }
        let Some(utility) = utility_at_path(&utilities, &cursor.path.0, cursor.depth) else {
            let siblings = utility_sibling_count(&utilities, &cursor.path.0, cursor.depth).unwrap_or(0);
            let next = cursor.path.0[cursor.depth] as usize + 1;
            if next < siblings {
                cursor.path.0[cursor.depth] = next as u16;
                return false;
            }
            if cursor.depth > 0 {
                cursor.depth -= 1;
                return false;
            }
            return true;
        };
        let x = cursor.x;
        let result = render_utility_node_step(cursor, &mut self.chrome_build, draw, atlas, icons, input, theme, x, btn_y, theme.control_height, utility, &self.utility_collection_expanded);
        let advanced = match result {
            Ok(result) => result,
            Err(()) => {
                self.error = Some("Shell pane utility exceeded the retained chrome boundary".to_string());
                return true;
            }
        };
        let Some((next_x, descend)) = advanced else { return false };
        cursor.x = next_x;
        cursor.flag = descend;
        if descend {
            return false;
        }
        let siblings = utility_sibling_count(&utilities, &cursor.path.0, cursor.depth).unwrap_or(0);
        let next = cursor.path.0[cursor.depth] as usize + 1;
        if next < siblings {
            cursor.path.0[cursor.depth] = next as u16;
            return false;
        }
        if cursor.depth > 0 {
            cursor.depth -= 1;
            return false;
        }
        true
    }

    /// 🔀️ React's `WorldProjectionKindSwitch` as one retained Tree document. The generic Tree owns
    /// the full-width rows, icons, disclosure chevrons and AX hierarchy; Projection adds only the
    /// canonical taxonomy and exact per-window action payload.
    fn build_window_projection_ui(&self, window_id: &str) -> UiNode {
        let pane_id = format!("{WORLD_PROJECTION_PANE_PARENT}.{}", semio_framework::element_id_segment(window_id));
        let selected = self.world_projection_template_id(window_id);
        let mut cursor = 0;
        let items = world_projection_pane_rows(&pane_id, window_id, selected, 0, &mut cursor);
        UiNode::Stack(UiStackNode {
            direction: "column".into(),
            gap: None,
            padding: None,
            id: Some(format!("{pane_id}.body")),
            children: vec![UiNode::Tree(UiTreeNode {
                presentation: Default::default(),
                sections: vec![UiTreeSectionNode {
                    header_toolbar: None,
                    id: format!("{pane_id}.projection-modes"),
                    label: None,
                    default_open: Some(true),
                    presence: UiPresence::default(),
                    items,
                    window: None,
                }],
                presence: UiPresence::default(),
                drop_action: None,
                menu: None,
                interaction_domain: None,
            })],
            presence: UiPresence::default(),
            activate: None,
            drop_action: None,
            drop_overlay: None,
            menu: None,
        })
    }

    /// 🔀️ Rebuilds one Projection document through the same revision-keyed shell ingress as Actions,
    /// Search and Measures. The prior lease retires before the replacement is admitted.
    fn refresh_window_projection_document(&mut self, window_id: &str) -> Result<(), String> {
        let surface = window_projection_surface_id(window_id);
        let records = panel_ui_scroll_records(&surface, &self.build_window_projection_ui(window_id))?;
        let previous = self.window_projection_documents.remove(window_id);
        self.retire_one_surface_document(previous)?;
        let document = self.publish_surface_records(&surface, records)?;
        self.window_projection_documents.insert(window_id.to_string(), document);
        Ok(())
    }

    /// 🔀️ Applies one live Projection Tree row through the same view-state and camera path as the
    /// former bespoke row, then republishes the retained Tree so selection and AX stay atomic.
    pub(crate) fn select_world_projection_template(&mut self, window_id: &str, template_id: &str) -> Result<(), String> {
        if self.world3d_host_id_for_window(window_id).is_none() {
            return Ok(());
        }
        let effective_id = match template_id {
            "parallel" => "orthographic",
            "axonometric" => "axonometric-isometric",
            "oblique" => "oblique-cavalier",
            "perspective" => "three-point",
            other => other,
        };
        let template = world_projection_template(effective_id);
        if !self.apply_window_title_host_command(window_id, template.label) {
            return Ok(());
        }
        self.world_projection_template.insert(window_id.to_string(), template.id.to_string());
        let _ = self.apply_window_icon_host_command(window_id, template.icon_id);
        if let Some(world) = self.world3d_state_for_window_mut(window_id) {
            infinite_world::world::apply_world3d_projection_mode(world, template.mode);
        }
        self.refresh_window_projection_document(window_id)
    }

    /// 📐️ The Projection pane's 300px bottom-right body, ending above its own chip and growing up.
    fn window_projection_rect(window_id: &str, body: Rect, theme: &Theme) -> Rect {
        let width = WINDOW_PANE_BODY_WIDTH_PX.min((body.w - theme.panel_inset * 2.0).max(0.0));
        let bottom = body.y + body.h - theme.panel_inset - theme.control_height - theme.gap_standard;
        let available = (bottom - body.y - theme.panel_inset).max(0.0);
        let height = crate::interpreter::retained_content_height(&window_projection_surface_id(window_id)).unwrap_or(available).min(available).max(0.0);
        Rect::new(body.x + body.w - theme.panel_inset - width, bottom - height, width, height)
    }

    /// 🔀️ One paint opportunity of ONE pane's UNFOLDED retained Projection Tree.
    ///
    /// 🧭️ Selecting a row is VIEW STATE and dispatches nothing, which is React's own rule for this
    /// pane verbatim: `handleProjectionKindChange` only sets `externalPendingProjectionSpec`, because
    /// the single `setProjection` consumer takes granular `{field, value}` pairs and has no lossless
    /// mapping from a whole spec (`🌐️World3dHost/🟦️.tsx:6574`). Here the selection is observable in
    /// the chip's own icon; making the ORBIT follow it needs an orthographic frustum this renderer's
    /// `OrbitController` does not carry yet (`♾️infinite/🌍️world/🦀️.rs`'s
    /// `WORLD3D_PERSPECTIVE_CAMERA_ZOOM`) — the World3d lane's gap, not this one's.
    #[allow(clippy::too_many_arguments, reason = "the chrome walk's own paint context, forwarded unchanged")]
    pub(crate) fn paint_window_projection_step(
        &mut self,
        cursor: &mut ShellChromeChildCursor,
        draw: &mut DrawList,
        overlay: &mut Option<&mut DrawList>,
        atlas: &mut FontAtlas,
        icons: &IconAtlas,
        input: &mut InputState<ActionDescriptor>,
        theme: &Theme,
        window_id: &str,
        window_rect: Rect,
        world_resources: &mut infinite_world::world::World3dBuildContext,
        cx:&mut semio_framework_job::StepContext<'_>,
    ) -> bool {
        if self.projection_pane_folded(window_id) || self.world3d_host_id_for_window(window_id).is_none() || !surface_fits_overlay(theme, window_rect) {
            return true;
        }
        if !self.window_projection_documents.contains_key(window_id) {
            if let Err(error) = self.refresh_window_projection_document(window_id) {
                self.error = Some(error);
                return true;
            }
        }
        let Some(document) = self.window_projection_documents.remove(window_id) else { return true };
        let surface = window_projection_surface_id(window_id);
        crate::interpreter::set_ui_document_flow(&surface, ui_contract::UiFlow { inline: ui_contract::FlowInline::Rtl, block: ui_contract::FlowBlock::Down });
        if cursor.document.phase_name() == "viewport" {
            let selected_key = format!(
                "{surface}/{WORLD_PROJECTION_PANE_PARENT}.{}.{}",
                semio_framework::element_id_segment(window_id),
                semio_framework::element_id_segment(self.world_projection_template_id(window_id))
            );
            if let Ok(header) = document.header() {
                let _ = crate::interpreter::stamp_ui_document_tree_selected_item(&surface, header.generation, &selected_key);
            }
        }
        let rect = Self::window_projection_rect(window_id, window_rect, theme);
        if !cursor.flag && cursor.document.layout_is_accepted() && cursor.rect.is_some_and(|previous| previous != rect) {
            cursor.document.restart_viewport_after_host_reflow();
        }
        cursor.rect = Some(rect);
        if !cursor.flag && cursor.document.phase_name() == "paint" {
            draw.push_solid([rect.x, rect.y, rect.w, rect.h], theme.panel);
            cursor.flag = true;
        }
        let controller = self.document_controller_id();
        let complete = {
            let scroll_offsets = &mut self.scroll_offsets;
            let collapsed_sections = &mut self.collapsed_sections;
            let open_selects = &mut self.open_selects;
            let widget_maps = &mut self.widget_maps_staging;
            let mut hosts = crate::scenes::SceneEngineHosts { chrome_labels: scene_chrome_labels(self.locale_id == "de"), world3d_states: &mut self.world3d_states, world_resources, window_id: surface.as_str() };
            let viewport_height_for_widgets = draw.screen_height();
            let mut ctx = framework_widget_context(draw, overlay.as_deref_mut(), atlas, Some(icons), input, theme, scroll_offsets, collapsed_sections, open_selects, Some(widget_maps), viewport_height_for_widgets);
            ctx.pick_clip = Some(rect);
            render_ui_document_step(&mut cursor.document, &document, rect, &mut ctx, surface.as_str(), controller.as_str(), self.chrome_build.driver.drag, &mut hosts, cx)
        };
        self.window_projection_documents.insert(window_id.to_string(), document);
        if complete {
            self.note_retained_body_painted(&surface);
        } else {
            if document_opportunity_remains(cursor, WORLD_PROJECTION_PANE_PAINT_OPPORTUNITIES) {
                return false;
            }
            Self::debug_log(&format!(
                "[TRACE] wgpu-shell projection paint {surface} exhausted {WORLD_PROJECTION_PANE_PAINT_OPPORTUNITIES} opportunities parked-in={}",
                cursor.document.phase_name()
            ));
            self.record_document_paint_fault(&surface);
        }
        push_chrome_group_border(draw, rect, theme);
        self.register_retained_body_hits(&surface, rect, input);
        true
    }
}

#[cfg(all(test, not(target_arch = "wasm32")))]
#[path = "../../../🧪️tests/🧭️wgpu-navbar-footer-parity/🦀️.rs"]
mod navbar_footer_parity_tests;
//#endregion 🪟️WindowPaneChrome
//#endregion ActionPanelAndUtilities
