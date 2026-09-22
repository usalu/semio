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

/// 📏️ `min-w-32` at the canonical 16 px root used by the mounted React oracle.
pub(crate) const SELECT_CONTENT_MIN_WIDTH: f32 = 128.0;

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
    crate::wgpu::chrome::ICON_TINY + theme.padding_standard * 2.0
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

//#region 🔼️ScrollSlots
// 🔼️ The two `WidgetContext::scroll_offsets` slots one open popup owns. `select_scroll_step` and
// `select_clamped_scroll` above were tested arithmetic with ZERO callers until ticket 26/09/17
// packet W15a — the chevrons painted and did nothing, so a `Select` longer than its available height
// was stuck on its first page. The press handler cannot clamp (it has neither the item list nor the
// resolved popup height), so it writes a DIRECTION into the pending slot and this module's painter,
// which has both, converts it into `select_scroll_step` pixels and clamps the result. Two f32 slots
// per open select, cleared by `clear_select_scroll` when the popup closes — no new state type, and
// nothing that grows per frame.

/// 🔼️ The slot holding one popup's current scroll offset in pixels.
pub(crate) fn select_scroll_key(id: &str) -> String {
    format!("select.{id}.scroll")
}

/// 🔼️ The slot holding one popup's UNAPPLIED chevron direction (`-1.0` up, `+1.0` down, `0.0` idle).
pub(crate) fn select_scroll_pending_key(id: &str) -> String {
    format!("select.{id}.scrollStep")
}

/// 🔼️ The control id one scroll chevron registers its hit under — `SelectScrollUpButton` /
/// `SelectScrollDownButton` (`🟦️.tsx:790-818`).
pub(crate) fn select_scroll_control_id(id: &str, up: bool) -> String {
    if up {
        format!("{id}.scroll.up")
    } else {
        format!("{id}.scroll.down")
    }
}

/// 🔼️ Splits a `{select}.scroll.{up|down}` control id back into `(select id, direction)`.
pub(crate) fn select_scroll_control_parts(control_id: &str) -> Option<(&str, f32)> {
    if let Some(id) = control_id.strip_suffix(".scroll.up") {
        return Some((id, -1.0));
    }
    control_id.strip_suffix(".scroll.down").map(|id| (id, 1.0))
}

/// 🔼️ The scrollable viewport's own height inside a popup of `painted_height` — the popup minus its
/// `p-single` inset, which is the `clientHeight` React's `scrollSelectViewport` reads.
pub(crate) fn select_scroll_viewport_height(theme: &Theme, painted_height: f32) -> f32 {
    (painted_height - theme.padding_standard * 2.0).max(0.0)
}

/// 🔼️ Applies one pending chevron `direction` to `offset` and clamps the result — the ONE place both
/// scroll functions are consumed, shared by the immediate-mode kit below and any other popup painter.
pub(crate) fn select_scrolled_offset(items: usize, theme: &Theme, painted_height: f32, offset: f32, direction: f32) -> f32 {
    let stepped = offset + direction * select_scroll_step(select_scroll_viewport_height(theme, painted_height));
    select_clamped_scroll(items, theme, painted_height, stepped)
}

/// 🔼️ The first row index a popup scrolled by `offset` draws, and how many rows cover the viewport
/// (one extra so a partially scrolled band is never short a row at its bottom edge).
pub(crate) fn select_scrolled_row_window(items: usize, theme: &Theme, painted_height: f32, offset: f32) -> (usize, usize) {
    let row = select_row_height(theme);
    if row <= 0.0 {
        return (0, items);
    }
    let first = ((offset / row).floor().max(0.0) as usize).min(items);
    let visible = select_visible_rows(items, theme, painted_height).saturating_add(1);
    (first, (first + visible).min(items))
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub(crate) struct SelectPopupGeometry {
    pub menu: Rect,
    pub up: Option<Rect>,
    pub down: Option<Rect>,
    pub scroll: f32,
    pub max_scroll: f32,
    pub first_row: usize,
    pub last_row: usize,
}

impl SelectPopupGeometry {
    /// 🧭️ Projects window-local popup geometry into a frame without changing router authority.
    pub(crate) fn translated(self, x: f32, y: f32) -> Self {
        let translate = |rect: Rect| Rect::new(rect.x + x, rect.y + y, rect.w, rect.h);
        Self { menu: translate(self.menu), up: self.up.map(translate), down: self.down.map(translate), ..self }
    }
}

pub(crate) fn select_popup_geometry(trigger: Rect, items: usize, theme: &Theme, viewport_h: f32, offset: f32, direction: f32) -> SelectPopupGeometry {
    let border = theme.stroke_hairline;
    let band = select_scroll_button_height(theme);
    let natural_viewport = select_menu_height(items, theme);
    let natural = natural_viewport + border * 2.0 + band * 2.0;
    let menu_h = natural.min(select_available_height(trigger.y, trigger.h, natural, viewport_h));
    let menu_top = select_menu_top(trigger.y, trigger.h, natural, viewport_h);
    let menu = Rect::new(trigger.x, trigger.y + menu_top, trigger.w.max(SELECT_CONTENT_MIN_WIDTH), menu_h);
    let inner_height = (menu.h - border * 2.0).max(0.0);
    let band = band.min(inner_height * 0.5);
    let inner_width = (menu.w - border * 2.0).max(0.0);
    let up = Some(Rect::new(menu.x + border, menu.y + border, inner_width, band));
    let down = Some(Rect::new(menu.x + border, menu.y + menu.h - border - band, inner_width, band));
    let viewport_height = (inner_height - band * 2.0).max(0.0);
    let scroll = select_scrolled_offset(items, theme, viewport_height, offset, direction);
    let (first_row, last_row) = select_scrolled_row_window(items, theme, viewport_height, scroll);
    let max_scroll = select_clamped_scroll(items, theme, viewport_height, f32::MAX);
    SelectPopupGeometry { menu, up, down, scroll, max_scroll, first_row, last_row }
}

pub(crate) fn select_popup_viewport_rect(popup: SelectPopupGeometry) -> Rect {
    let top = popup.up.map_or(popup.menu.y, |button| button.y + button.h);
    let bottom = popup.down.map_or(popup.menu.y + popup.menu.h, |button| button.y);
    let left = popup.up.or(popup.down).map_or(popup.menu.x, |button| button.x);
    let width = popup.up.or(popup.down).map_or(popup.menu.w, |button| button.w);
    Rect::new(left, top, width, (bottom - top).max(0.0))
}

pub(crate) fn select_revealed_scroll(popup: SelectPopupGeometry, index: usize, theme: &Theme) -> f32 {
    let viewport = select_popup_viewport_rect(popup);
    let row_top = theme.padding_standard + index as f32 * select_row_height(theme);
    let row_bottom = row_top + select_row_height(theme);
    if row_top < popup.scroll {
        row_top.clamp(0.0, popup.max_scroll)
    } else if row_bottom > popup.scroll + viewport.h {
        (row_bottom - viewport.h).clamp(0.0, popup.max_scroll)
    } else {
        popup.scroll
    }
}

pub(crate) fn select_popup_row_rect(_trigger: Rect, index: usize, popup: SelectPopupGeometry, theme: &Theme) -> Rect {
    let viewport = select_popup_viewport_rect(popup);
    let inset = theme.padding_standard;
    Rect::new(viewport.x + inset, viewport.y + inset + index as f32 * select_row_height(theme) - popup.scroll, (viewport.w - inset * 2.0).max(0.0), select_row_height(theme))
}

pub(crate) fn select_popup_row_hit_rect(trigger: Rect, index: usize, popup: SelectPopupGeometry, theme: &Theme) -> Rect {
    let row = select_popup_row_rect(trigger, index, popup, theme);
    let viewport = select_popup_viewport_rect(popup);
    let top = viewport.y;
    let bottom = viewport.y + viewport.h;
    let y = row.y.max(top);
    let edge = (row.y + row.h).min(bottom);
    Rect::new(row.x.max(popup.menu.x), y, (row.x + row.w).min(popup.menu.x + popup.menu.w).max(row.x.max(popup.menu.x)) - row.x.max(popup.menu.x), (edge - y).max(0.0))
}

pub(crate) fn select_scroll_direction_at(popup: SelectPopupGeometry, x: f32, y: f32) -> Option<f32> {
    if popup.up.is_some_and(|rect| rect.contains(x, y)) {
        return Some(-1.0);
    }
    popup.down.filter(|rect| rect.contains(x, y)).map(|_| 1.0)
}
//#endregion 🔼️ScrollSlots

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

fn is_normalization_mark(value: char) -> bool {
    matches!(value, '\u{0300}'..='\u{036f}' | '\u{0483}'..='\u{0489}' | '\u{1ab0}'..='\u{1aff}' | '\u{1dc0}'..='\u{1dff}' | '\u{20d0}'..='\u{20f0}' | '\u{fe20}'..='\u{fe2f}')
}

fn push_nfkd_base(value: char, out: &mut String) {
    match value {
        '\u{ff01}'..='\u{ff5e}' => out.extend(char::from_u32(value as u32 - 0xfee0).expect("fullwidth ASCII maps into ASCII").to_lowercase()),
        '\u{fb00}' => out.push_str("ff"),
        '\u{fb01}' => out.push_str("fi"),
        '\u{fb02}' => out.push_str("fl"),
        '\u{fb03}' => out.push_str("ffi"),
        '\u{fb04}' => out.push_str("ffl"),
        '\u{fb05}' | '\u{fb06}' => out.push_str("st"),
        _ => out.extend((value as u32).checked_sub(0xc0).filter(|_| value <= '\u{00ff}').and_then(|index| LATIN1_BASE.chars().nth(index as usize)).unwrap_or(value).to_lowercase()),
    }
}

/// 🔤️ The shared bounded normalization owner for Select typeahead and the shell palettes. It
/// projects the shipped EN/DE NFKD repertoire, strips Unicode mark ranges, folds case, optionally
/// collapses whitespace, and applies the consumer's scalar limit. The neutral ShellSearch fixture
/// pins precomposed/decomposed German, fullwidth compatibility forms and presentation ligatures.
pub fn normalize_nfkd_text(value: &str, collapse_whitespace: bool, limit: usize) -> String {
    let mut out = String::with_capacity(value.len());
    let mut pending_space = false;
    for value in value.chars() {
        if is_normalization_mark(value) {
            continue;
        }
        if collapse_whitespace && value.is_whitespace() {
            pending_space = !out.is_empty();
            continue;
        }
        if pending_space {
            out.push(' ');
            pending_space = false;
        }
        push_nfkd_base(value, &mut out);
    }
    out.trim().chars().take(limit).collect()
}

/// 🔤️ React's `normalizeSelectText` (`🟦️.tsx:194-200`).
pub(crate) fn normalize_select_text(value: &str) -> String {
    normalize_nfkd_text(value, true, usize::MAX)
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
    let inset = ctx.theme.padding_standard;
    let font_size = ctx.theme.font_size_body;
    // 🔼️ Resolve this frame's scroll offset BEFORE anything paints: take whatever direction the
    // press handler left pending, price it in `select_scroll_step` pixels against the popup's own
    // viewport, clamp it to what the popup can actually scroll, and write both slots back. A popup
    // that fits clamps straight to `0.0`, so the chevrons stay inert exactly as React's do.
    let pending = ctx.scroll_offsets.remove(&select_scroll_pending_key(id)).unwrap_or(0.0);
    let scroll_key = select_scroll_key(id);
    let stored = ctx.scroll_offsets.get(&scroll_key).copied().unwrap_or(0.0);
    let popup = select_popup_geometry(bounds, items.len(), ctx.theme, viewport_h, stored, pending);
    if popup.scroll == 0.0 {
        ctx.scroll_offsets.remove(&scroll_key);
    } else {
        ctx.scroll_offsets.insert(scroll_key, popup.scroll);
    }
    let mut render_rows = |draw: &mut crate::wgpu::draw::DrawList| {
        let glass = draw.push_glass([popup.menu.x, popup.menu.y, popup.menu.w, popup.menu.h], ctx.theme.border_radius, ctx.theme.glass(Level::Menu));
        draw.begin_glass_content(glass);
        crate::wgpu::chrome::push_chrome_border(draw, popup.menu, ctx.theme.stroke_hairline, ctx.theme.border_normal, true, true, true, true);
        draw.push_scissor(popup.menu);
        for (index, item) in items.iter().enumerate().take(popup.last_row).skip(popup.first_row) {
            let row = select_popup_row_rect(bounds, index, popup, ctx.theme);
            let row_hovered = ctx.input.hit_at(ctx.input.pointer_x, ctx.input.pointer_y).and_then(|h| h.control_id.as_deref()) == Some(&format!("{id}.item.{}", item.value()));
            if row_hovered || item.value() == value {
                draw.push_rounded([row.x, row.y, row.w, row.h], ctx.theme.row_hover, ctx.theme.border_radius);
            }
            draw_text_on(draw, ctx.atlas, item.label(), row.x + inset, row.y + (row.h + font_size) * 0.5 - 2.0, font_size, ctx.theme.text);
            ctx.input.register_hit(HitTarget { rect: select_popup_row_hit_rect(bounds, index, popup, ctx.theme), event: None, control_id: Some(format!("{id}.item.{}", item.value())), kind: HitKind::DropdownItem, drag_axis: None, drag_data: None });
        }
        draw.pop_scissor();
        // 🔼️ The chevrons paint and register LAST so their bands win the hit resolve (`HitRegistry`
        // resolves the most recently registered target first) over the rows they sit on top of.
        if let (Some(up), Some(down)) = (popup.up, popup.down) {
            let chevron = crate::wgpu::chrome::SIZE_TINY;
            let center_x = popup.menu.x + (popup.menu.w - chevron) * 0.5;
            if let Some(icons) = ctx.icons {
                crate::wgpu::chrome::push_icon(draw, icons, "chevron-up", center_x, up.y + (up.h - chevron) * 0.5, chevron, ctx.theme.text_muted);
                crate::wgpu::chrome::push_icon(draw, icons, "chevron-down", center_x, down.y + (down.h - chevron) * 0.5, chevron, ctx.theme.text_muted);
            }
            ctx.input.register_hit(HitTarget { rect: up, event: None, control_id: Some(select_scroll_control_id(id, true)), kind: HitKind::DropdownItem, drag_axis: None, drag_data: None });
            ctx.input.register_hit(HitTarget { rect: down, event: None, control_id: Some(select_scroll_control_id(id, false)), kind: HitKind::DropdownItem, drag_axis: None, drag_data: None });
        }
        draw.end_glass_content();
    };
    if let Some(overlay) = ctx.overlay.as_deref_mut() {
        render_rows(overlay);
    } else {
        render_rows(ctx.draw);
    }
    if let Some(maps) = ctx.interaction_maps.as_deref_mut() {
        maps.register_select_popup_wheel(id, popup.menu, popup.max_scroll, ctx.input.staged_hits().len());
    }
}

/// 🔼️ Arms one chevron press: the direction is stashed for [`render_select_menu`] to price and
/// clamp on the next paint, which is the only place the item count and the resolved popup height are
/// both in hand. Answers `false` for a control id that is not a scroll chevron.
pub fn arm_select_scroll(scroll_offsets: &mut std::collections::HashMap<String, f32>, control_id: &str) -> bool {
    let Some((id, direction)) = select_scroll_control_parts(control_id) else { return false };
    scroll_offsets.insert(select_scroll_pending_key(id), direction);
    true
}

pub(crate) fn arm_retained_select_scroll_at(tree: &mut crate::wgpu::tree::UiTree, id: crate::wgpu::arena::NodeId, x: f32, y: f32) -> bool {
    let direction = tree.node(id).and_then(|node| node.state.select_popup).and_then(|popup| select_scroll_direction_at(popup, x, y));
    let Some(direction) = direction else { return false };
    let Some(node) = tree.node_mut(id) else { return false };
    node.state.scroll_offset.0 = direction;
    tree.mark_dirty(id, crate::wgpu::tree::NodeFlags::DIRTY_PAINT);
    true
}

pub(crate) fn scroll_retained_select_at(tree: &mut crate::wgpu::tree::UiTree, id: crate::wgpu::arena::NodeId, x: f32, y: f32, delta: f32) -> bool {
    let Some(popup) = tree.node(id).filter(|node| node.state.open).and_then(|node| node.state.select_popup).filter(|popup| popup.menu.contains(x, y)) else { return false };
    if delta.is_finite() && delta != 0.0 {
        if let Some(node) = tree.node_mut(id) {
            node.state.scroll_offset.1 = (node.state.scroll_offset.1 + delta).clamp(0.0, popup.max_scroll);
        }
        tree.mark_dirty(id, crate::wgpu::tree::NodeFlags::DIRTY_PAINT);
    }
    true
}

/// 🧹️ Drops both of one popup's scroll slots — called when the popup closes so a reopened `Select`
/// starts at its first page, like a freshly mounted `SelectContent`, and neither slot outlives it.
pub fn clear_select_scroll(scroll_offsets: &mut std::collections::HashMap<String, f32>, id: &str) {
    scroll_offsets.remove(&select_scroll_key(id));
    scroll_offsets.remove(&select_scroll_pending_key(id));
}

#[cfg(test)]
#[path = "../../🧪️tests/🔬️wgpu-select-keyboard/🦀️.rs"]
mod keyboard_tests;
