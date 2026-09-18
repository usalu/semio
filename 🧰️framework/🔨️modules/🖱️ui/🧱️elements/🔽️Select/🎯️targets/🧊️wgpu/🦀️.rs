//! 🔎️ wgpu render functions for the Select element — extracted from `widgets` mod's inline body
//! (ticket 26/08/05/UI-ELEMENT-CO-LOCATION-RESTRUCTURE). Wired as a CRATE-ROOT sibling module of
//! `crate::wgpu::widgets` (declared `#[cfg(feature = "wgpu-engine")] #[path = "..."] mod select;` right before
//! `pub mod widgets` in lib.rs — deliberately NOT nested inside `widgets { }`, since rustc resolves a
//! nested inline-module's `#[path]` as if the parent had its own on-disk directory, which fails for a
//! genuinely inline `mod widgets { }` block). `widgets` mod pulls these back in via
//! `use crate::wgpu::select::{render_select, render_select_menu};` so its own unqualified call sites keep
//! working. `crate::wgpu::widgets::{...}` reaches the sibling items this needs (`WidgetContext`,
//! `SelectItem`, `draw_text`, `draw_text_on`); `crate::wgpu::chrome`/`crate::wgpu::input`/`crate::wgpu::theme` are the
//! other top-level engine mods `widgets` itself also depends on.
//!
//! 🎹️ Since ticket 26/09/17/WGPU-RENDERER-REACT-PARITY packet W1o this module also owns the
//! element's **behaviour** contract — popup geometry (`🔖️Geometry`) and the keyboard/typeahead state
//! machine (`🔖️Keyboard`) — as pure functions both renderers' call sites share: `paint`'s retained
//! popup rows, `events`' key routing, and the immediate-mode kit below all resolve the same numbers
//! from here, so none of the three can drift. Every rule is a direct port of React's own first-party
//! Select (`🧱️elements/🔽️Select/🟦️.tsx`: `resolveSelectPlacement` at :245-265, `SelectTrigger`'s
//! `onKeyDown` at :395-410, `SelectContent`'s at :641-672, `moveActive` at :435-452,
//! `findTypeaheadOption` at :455-466), not of Radix, which this element stopped wrapping.

use crate::wgpu::chrome::push_control_border;
use crate::wgpu::component::layout::MeasureSelectItem;
use crate::wgpu::geometry::Rect;
use crate::wgpu::input::{HitKind, HitTarget};
use crate::wgpu::theme::{Level, Theme};
use crate::wgpu::widgets::{draw_text, draw_text_on, SelectItem, WidgetContext};

pub(crate) trait SelectItemView {
    fn value(&self) -> &str;
    fn label(&self) -> &str;
}

impl SelectItemView for SelectItem {
    fn value(&self) -> &str {
        &self.value
    }

    fn label(&self) -> &str {
        &self.label
    }
}

impl SelectItemView for MeasureSelectItem {
    fn value(&self) -> &str {
        &self.value
    }

    fn label(&self) -> &str {
        &self.label
    }
}

//#region 🔖️Geometry
/// 📏️ `SelectContent`'s `sideOffset` default — the gap between trigger and popup (`🟦️.tsx:476`).
pub(crate) const SELECT_SIDE_OFFSET: f32 = 4.0;

/// 📏️ `SelectContent`'s `collisionPadding` default — the viewport inset the popup keeps clear on
/// every edge, and the floor a flipped popup's top is clamped to (`🟦️.tsx:477`).
pub(crate) const SELECT_COLLISION_PADDING: f32 = 8.0;

/// 📏️ One popup row's height: a `text-sm` line box plus `py-single` on both sides — React's
/// `SelectItem` is `py-single … text-sm` (`🟦️.tsx:729`), never a `--size-*` control height, which is
/// why this does NOT reuse `Theme::control_height` the way the pre-parity literal did.
pub(crate) fn select_row_height(theme: &Theme) -> f32 {
    crate::wgpu::text::line_height(theme.font_size_body) + theme.padding_standard * 2.0
}

/// 📏️ The popup's own height for `items` rows — rows plus the viewport's `p-single` inset
/// (`🟦️.tsx`'s `select-viewport`).
pub(crate) fn select_menu_height(items: usize, theme: &Theme) -> f32 {
    items as f32 * select_row_height(theme) + theme.padding_standard * 2.0
}

/// 📐️ The popup's top edge **relative to the trigger's own top-left**, flipping above the trigger
/// when the space below cannot hold it and the space above is larger — the port of
/// `resolveSelectPlacement` (`🟦️.tsx:245-265`) restricted to this target's `side: "bottom"`,
/// `align: "start"` case. A zero/unknown `viewport_h` never flips, so a caller without a measured
/// viewport (the immediate-mode kit below) degrades to the plain below-the-trigger placement rather
/// than guessing.
pub(crate) fn select_menu_top(trigger_abs_y: f32, trigger_h: f32, menu_h: f32, viewport_h: f32) -> f32 {
    if viewport_h <= 0.0 {
        return trigger_h + SELECT_SIDE_OFFSET;
    }
    let below = (viewport_h - SELECT_COLLISION_PADDING - (trigger_abs_y + trigger_h) - SELECT_SIDE_OFFSET).max(0.0);
    let above = (trigger_abs_y - SELECT_COLLISION_PADDING - SELECT_SIDE_OFFSET).max(0.0);
    if menu_h > below && above > below {
        let absolute = (trigger_abs_y - menu_h.min(above) - SELECT_SIDE_OFFSET).max(SELECT_COLLISION_PADDING);
        absolute - trigger_abs_y
    } else {
        trigger_h + SELECT_SIDE_OFFSET
    }
}

/// 📏️ React's `availableHeight` (`🟦️.tsx:255-258`): the room the popup actually has on the side it
/// settled on, which `SelectContent` applies as `max-h-(--semio-select-content-available-height)`
/// and the viewport scrolls inside. A zero/unknown `viewport_h` answers `f32::INFINITY` — an
/// unmeasured caller must not clamp the popup to nothing.
pub(crate) fn select_available_height(trigger_abs_y: f32, trigger_h: f32, menu_h: f32, viewport_h: f32) -> f32 {
    if viewport_h <= 0.0 {
        return f32::INFINITY;
    }
    let below = (viewport_h - SELECT_COLLISION_PADDING - (trigger_abs_y + trigger_h) - SELECT_SIDE_OFFSET).max(0.0);
    let above = (trigger_abs_y - SELECT_COLLISION_PADDING - SELECT_SIDE_OFFSET).max(0.0);
    if menu_h > below && above > below {
        above
    } else {
        below
    }
}

/// 📏️ The popup's PAINTED height: its natural height clamped to React's `availableHeight`. This is
/// the half of `resolveSelectPlacement` that `select_menu_top` alone never applied, which is why a
/// long `Select` used to draw past the surface edge instead of scrolling.
pub(crate) fn select_menu_painted_height(items: usize, theme: &Theme, trigger_abs_y: f32, trigger_h: f32, viewport_h: f32) -> f32 {
    let natural = select_menu_height(items, theme);
    natural.min(select_available_height(trigger_abs_y, trigger_h, natural, viewport_h))
}

/// 🔼️ How many rows fit inside a popup of `painted_height`. Fewer than `items` means both scroll
/// buttons are live and the viewport scrolls — React always MOUNTS both buttons
/// (`🟦️.tsx:675-679`), so this is a scroll-extent question, never a mount question.
pub(crate) fn select_visible_rows(items: usize, theme: &Theme, painted_height: f32) -> usize {
    let row = select_row_height(theme);
    if row <= 0.0 {
        return items;
    }
    let inner = (painted_height - theme.padding_standard * 2.0).max(0.0);
    ((inner / row).floor() as usize).min(items)
}

/// 🔼️ One scroll button's height: `py-single` twice plus a `size-tiny` chevron — React's
/// `SelectScrollUpButton`/`SelectScrollDownButton` are `py-single` with a `size-tiny` icon
/// (`🟦️.tsx:790-795`, `:813-818`).
pub(crate) fn select_scroll_button_height(theme: &Theme) -> f32 {
    crate::wgpu::chrome::SIZE_TINY + theme.padding_standard * 2.0
}

/// 🔼️ The pixels one scroll-button press moves the viewport — React's `scrollSelectViewport`
/// (`🟦️.tsx:769-775`): `max(24, floor(clientHeight * 0.8))`, signed by the direction.
pub(crate) fn select_scroll_step(viewport_height: f32) -> f32 {
    (viewport_height * 0.8).floor().max(SELECT_SCROLL_MIN_STEP)
}

/// 🔼️ React's own floor for a scroll-button step (`🟦️.tsx:771`).
pub(crate) const SELECT_SCROLL_MIN_STEP: f32 = 24.0;

/// 🔼️ A scroll offset clamped to what the popup can actually scroll: `content - viewport`, never
/// negative. The viewport is `overflow-y-auto`, so the DOM clamps this for React.
pub(crate) fn select_clamped_scroll(items: usize, theme: &Theme, painted_height: f32, offset: f32) -> f32 {
    let content = items as f32 * select_row_height(theme);
    let inner = (painted_height - theme.padding_standard * 2.0).max(0.0);
    offset.clamp(0.0, (content - inner).max(0.0))
}

/// 📐️ One popup row's `(x, y, w, h)` relative to the trigger's own top-left, given the popup top
/// [`select_menu_top`] resolved. Shared by `paint`'s retained row layout, `paint_select`, and
/// [`render_select_menu`] so geometry and hit-testing can never disagree.
pub(crate) fn select_row_rect(trigger_w: f32, index: usize, menu_top: f32, theme: &Theme) -> Rect {
    let inset = theme.padding_standard;
    Rect::new(inset, menu_top + inset + index as f32 * select_row_height(theme), (trigger_w - inset * 2.0).max(0.0), select_row_height(theme))
}
//#endregion 🔖️Geometry

//#region 🔖️Keyboard
/// ⏱️ How long a typeahead query survives without another keystroke (`🟦️.tsx:663`'s 700 ms timer),
/// measured against `events::EventRouter`'s own monotonic clock.
pub(crate) const SELECT_TYPEAHEAD_RESET_SECONDS: f32 = 0.7;

/// 📄️ How many rows `PageUp`/`PageDown` jump (`🟦️.tsx:447-450`).
pub(crate) const SELECT_PAGE_STEP: usize = 10;

/// 🎬️ Which row a keystroke on the CLOSED trigger opens the popup at (`🟦️.tsx:399-408`).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum SelectOpenIntent {
    /// ⬇️ `ArrowDown`/`Enter`/`Space`: the currently selected row.
    Selected,
    /// ⬆️ `ArrowUp`: the last row.
    Last,
    /// 🔤️ A printable key: the first row matching it, as a one-character typeahead query.
    Typeahead(char),
}

/// 🧭️ Highlight movement inside the OPEN popup (`🟦️.tsx:644`'s `movement` map).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum SelectMove {
    Next,
    Previous,
    First,
    Last,
    PageNext,
    PagePrevious,
}

/// ⌨️ What one key does to a `Select` — the single decision table `events` routes through, so the
/// open and closed halves of React's two `onKeyDown` handlers stay together.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum SelectKey {
    Open(SelectOpenIntent),
    Move(SelectMove),
    /// ⏎️ `Enter`/`Space` over the open popup commits the highlighted row.
    Commit,
    /// ↹️ `Tab` closes the popup WITHOUT restoring focus to the trigger, so the same keystroke's
    /// focus move still lands (`🟦️.tsx:657-660`).
    Close,
    Typeahead(char),
    Ignored,
}

/// ⌨️ `key` (a DOM `KeyboardEvent.key` name) against a `Select` whose popup is `open`. `alt`/`ctrl`/
/// `meta` suppress the printable-key typeahead exactly as React's guards do; `Escape` is deliberately
/// absent — the overlay stack already dismisses the topmost popup before per-widget routing runs.
pub(crate) fn select_key(open: bool, key: &str, alt: bool, ctrl: bool, meta: bool) -> SelectKey {
    let printable = (!alt && !ctrl && !meta).then(|| single_char(key)).flatten();
    if open {
        return match key {
            "ArrowDown" => SelectKey::Move(SelectMove::Next),
            "ArrowUp" => SelectKey::Move(SelectMove::Previous),
            "Home" => SelectKey::Move(SelectMove::First),
            "End" => SelectKey::Move(SelectMove::Last),
            "PageDown" => SelectKey::Move(SelectMove::PageNext),
            "PageUp" => SelectKey::Move(SelectMove::PagePrevious),
            "Enter" | "NumpadEnter" | " " => SelectKey::Commit,
            "Tab" => SelectKey::Close,
            _ => printable.map_or(SelectKey::Ignored, SelectKey::Typeahead),
        };
    }
    match key {
        "ArrowDown" | "Enter" | "NumpadEnter" | " " => SelectKey::Open(SelectOpenIntent::Selected),
        "ArrowUp" => SelectKey::Open(SelectOpenIntent::Last),
        _ => printable.map_or(SelectKey::Ignored, |char| SelectKey::Open(SelectOpenIntent::Typeahead(char))),
    }
}

/// 🔤️ A `KeyboardEvent.key` that names one printable character (React's `event.key.length === 1`).
fn single_char(key: &str) -> Option<char> {
    let mut chars = key.chars();
    let first = chars.next()?;
    chars.next().is_none().then_some(first).filter(|char| !char.is_control())
}

/// 🧭️ The row `movement` highlights next, wrapping on `Next`/`Previous` and clamping on the paged
/// steps — arithmetic ported verbatim from `moveActive` (`🟦️.tsx:435-452`), including its treatment
/// of "nothing highlighted yet" as index `-1`.
pub(crate) fn select_moved_index(active: Option<usize>, len: usize, movement: SelectMove) -> Option<usize> {
    if len == 0 {
        return None;
    }
    let active = active.map_or(-1_i64, |index| index as i64);
    let len_i = len as i64;
    let next = match movement {
        SelectMove::First => 0,
        SelectMove::Last => len_i - 1,
        SelectMove::Next => (active + 1 + len_i).rem_euclid(len_i),
        SelectMove::Previous => (active - 1 + len_i).rem_euclid(len_i),
        SelectMove::PageNext => (len_i - 1).min(active.max(0) + SELECT_PAGE_STEP as i64),
        SelectMove::PagePrevious => 0.max(if active < 0 { len_i } else { active } - SELECT_PAGE_STEP as i64),
    };
    usize::try_from(next).ok().filter(|index| *index < len)
}

/// 🔤️ The Latin-1 Supplement's canonical decompositions, `U+00C0..=U+00FF` in order — the only block
/// [`normalize_select_text`] needs a table for, since it carries every accented letter German
/// ("Ärger") and the other shipped locales actually use. Characters with no decomposition (`Æ`, `Ø`,
/// `ß`, `×`, `÷`, `Ð`, `Þ`) map to themselves, exactly as NFKD leaves them.
const LATIN1_BASE: &str = "AAAAAAÆCEEEEIIIIÐNOOOOO×ØUUUUYÞßaaaaaaæceeeeiiiiðnooooo÷øuuuuyþy";

/// 🔤️ React's `normalizeSelectText` (`🟦️.tsx:194-200`): case-folded, whitespace-collapsed, combining
/// marks dropped. Full NFKD would need a Unicode table this crate deliberately does not depend on,
/// so this drops already-decomposed marks (`U+0300..=U+036F`) and folds the precomposed Latin-1
/// letters through [`LATIN1_BASE`]; anything outside those two cases compares as itself, which is
/// also what NFKD does for it.
pub(crate) fn normalize_select_text(value: &str) -> String {
    let mut out = String::with_capacity(value.len());
    let mut pending_space = false;
    for char in value.chars() {
        if matches!(char, '\u{0300}'..='\u{036f}') {
            continue;
        }
        if char.is_whitespace() {
            pending_space = !out.is_empty();
            continue;
        }
        if pending_space {
            out.push(' ');
            pending_space = false;
        }
        let folded = (char as u32).checked_sub(0xC0).filter(|_| char <= '\u{00ff}').and_then(|index| LATIN1_BASE.chars().nth(index as usize)).unwrap_or(char);
        out.extend(folded.to_lowercase());
    }
    out
}

/// 🔤️ The first row at or after the one following `active` whose label starts with `query` — the
/// cyclic scan `findTypeaheadOption` performs (`🟦️.tsx:455-466`).
pub(crate) fn select_typeahead_index<'a>(labels: impl IntoIterator<Item = &'a str>, query: &str, active: Option<usize>) -> Option<usize> {
    let normalized: Vec<String> = labels.into_iter().map(normalize_select_text).collect();
    if normalized.is_empty() {
        return None;
    }
    let needle = normalize_select_text(query);
    let start = active.map_or(normalized.len().saturating_sub(1), |index| index);
    for offset in 1..=normalized.len() {
        let index = (start + offset) % normalized.len();
        if normalized[index].starts_with(&needle) {
            return Some(index);
        }
    }
    None
}

/// 🎬️ Where the highlight lands when the popup OPENS: the selected row, the last row, or the first
/// typeahead match (`🟦️.tsx:538-546`). Falls back to the first row when nothing matches, so an open
/// popup always has exactly one highlighted row for `Enter` to commit.
pub(crate) fn select_open_index<'a>(intent: SelectOpenIntent, labels: impl IntoIterator<Item = &'a str>, selected: Option<usize>) -> Option<usize> {
    let labels: Vec<&str> = labels.into_iter().collect();
    if labels.is_empty() {
        return None;
    }
    match intent {
        SelectOpenIntent::Selected => Some(selected.unwrap_or(0)),
        SelectOpenIntent::Last => Some(labels.len() - 1),
        SelectOpenIntent::Typeahead(char) => Some(select_typeahead_index(labels.iter().copied(), &char.to_string(), None).unwrap_or(0)),
    }
}
//#endregion 🔖️Keyboard

pub(crate) fn render_select<E: Clone, T: SelectItemView>(id: &str, value: &str, items: &[T], placeholder: Option<&str>, bounds: Rect, ctx: &mut WidgetContext<'_, E>) {
    let open = *ctx.open_selects.get(id).unwrap_or(&false);
    let hovered = ctx.input.hovered_id.as_deref() == Some(id);
    let bg = if hovered { ctx.theme.button_hover } else { ctx.theme.input_bg };
    push_control_border(ctx.draw, bounds, ctx.theme, ctx.theme.border_normal, bg);
    let label = items.iter().find(|item| item.value() == value).map_or(placeholder.unwrap_or("Select…"), |item| item.label());
    draw_text(ctx, label, bounds.x + ctx.theme.padding_standard, bounds.y + (bounds.h + ctx.theme.font_size_body) * 0.5 - 2.0, ctx.theme.font_size_body, ctx.theme.text);
    if let Some(icons) = ctx.icons {
        crate::wgpu::chrome::push_icon(
            ctx.draw,
            icons,
            "chevron-down",
            bounds.x + bounds.w - ctx.theme.padding_standard - crate::wgpu::chrome::ICON_TINY,
            bounds.y + (bounds.h - crate::wgpu::chrome::ICON_TINY) * 0.5,
            crate::wgpu::chrome::ICON_TINY,
            ctx.theme.text_element,
        );
    }
    ctx.input.register_hit(HitTarget { rect: bounds, event: None, control_id: Some(id.to_string()), kind: HitKind::Select, drag_axis: None, drag_data: None });
    if open {
        render_select_menu(id, value, items, bounds, ctx);
    }
}

/// 🔽️ The open popup. `WidgetContext::viewport_height` is the measured surface the kit flips and
/// clamps against — it used to carry none, so this path passed `0.0` and could only ever place below
/// at full natural height (ticket 26/09/17 packet W2k threaded the viewport through
/// `framework_widget_context`). Everything else — row pitch, insets, hit rects, the clamp, the
/// scroll buttons — comes from `🔖️Geometry` above, shared with the retained path.
pub(crate) fn render_select_menu<E: Clone, T: SelectItemView>(id: &str, value: &str, items: &[T], bounds: Rect, ctx: &mut WidgetContext<'_, E>) {
    let viewport_h = ctx.viewport_height;
    let menu_h = select_menu_painted_height(items.len(), ctx.theme, bounds.y, bounds.h, viewport_h);
    let menu_top = select_menu_top(bounds.y, bounds.h, select_menu_height(items.len(), ctx.theme), viewport_h);
    let menu = Rect::new(bounds.x, bounds.y + menu_top, bounds.w, menu_h);
    let scrolls = select_visible_rows(items.len(), ctx.theme, menu_h) < items.len();
    let inset = ctx.theme.padding_standard;
    let font_size = ctx.theme.font_size_body;
    let mut render_rows = |draw: &mut crate::wgpu::draw::DrawList| {
        draw.push_glass([menu.x, menu.y, menu.w, menu.h], ctx.theme.border_radius, ctx.theme.glass(Level::Menu));
        if scrolls {
            let button_h = select_scroll_button_height(ctx.theme);
            let chevron = crate::wgpu::chrome::SIZE_TINY;
            let center_x = menu.x + (menu.w - chevron) * 0.5;
            if let Some(icons) = ctx.icons {
                crate::wgpu::chrome::push_icon(draw, icons, "chevron-up", center_x, menu.y + inset, chevron, ctx.theme.text_muted);
                crate::wgpu::chrome::push_icon(draw, icons, "chevron-down", center_x, menu.y + menu.h - button_h + inset, chevron, ctx.theme.text_muted);
            }
        }
        let visible = select_visible_rows(items.len(), ctx.theme, menu_h);
        for (index, item) in items.iter().enumerate().take(visible.max(1)) {
            let relative = select_row_rect(bounds.w, index, menu_top, ctx.theme);
            let row = Rect::new(bounds.x + relative.x, bounds.y + relative.y, relative.w, relative.h);
            let row_hovered = ctx.input.hit_at(ctx.input.pointer_x, ctx.input.pointer_y).and_then(|h| h.control_id.as_deref()) == Some(&format!("{id}.item.{}", item.value()));
            if row_hovered || item.value() == value {
                draw.push_rounded([row.x, row.y, row.w, row.h], ctx.theme.row_hover, ctx.theme.border_radius);
            }
            draw_text_on(draw, ctx.atlas, item.label(), row.x + inset, row.y + (row.h + font_size) * 0.5 - 2.0, font_size, ctx.theme.text);
            ctx.input.register_hit(HitTarget { rect: row, event: None, control_id: Some(format!("{id}.item.{}", item.value())), kind: HitKind::DropdownItem, drag_axis: None, drag_data: None });
        }
    };
    if let Some(overlay) = ctx.overlay.as_deref_mut() {
        render_rows(overlay);
    } else {
        render_rows(ctx.draw);
    }
}

#[cfg(test)]
#[path = "../../🧪️tests/🔬️wgpu-select-keyboard/🦀️.rs"]
mod keyboard_tests;
