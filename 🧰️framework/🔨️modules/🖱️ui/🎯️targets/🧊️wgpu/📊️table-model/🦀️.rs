//! 📊️ The language-neutral table model: fixed column tracks, horizontal scroll, row and column keys, selection merge, sort and the empty label.

pub const TABLE_COLUMN_WIDTH_PX: f32 = 192.0;

/// 📏️ One column track `minmax(min, max)`; a fixed column has `min == max`. Mirrors `tableWindowColumnTemplate` in `🗣️Interpreter/📊️table-window/🟦️.ts`.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct TableTrack {
    pub min: f32,
    pub max: f32,
}

impl TableTrack {
    pub const FIXED: Self = Self { min: TABLE_COLUMN_WIDTH_PX, max: TABLE_COLUMN_WIDTH_PX };

    pub fn width(self) -> f32 {
        self.max.max(self.min)
    }
}

fn spacer(count: usize) -> String {
    let px = count as f32 * TABLE_COLUMN_WIDTH_PX;
    format!("minmax({px}px, {px}px)")
}

/// 🧾️ The CSS grid template React renders for a header and its rows.
pub fn table_column_template(columns: usize, actions: usize, leading: usize, trailing: usize) -> String {
    let mut tracks = Vec::new();
    if leading > 0 {
        tracks.push(spacer(leading));
    }
    tracks.extend((0..columns).map(|_| format!("minmax({TABLE_COLUMN_WIDTH_PX}px, {TABLE_COLUMN_WIDTH_PX}px)")));
    if actions > 0 {
        tracks.push(format!("calc({actions} * var(--size-medium) + {actions} * var(--spacing-single))"));
    }
    if trailing > 0 {
        tracks.push(spacer(trailing));
    }
    tracks.join(" ")
}

/// 📐️ The actions track in pixels: `actions × size-medium + actions × spacing-single`.
pub fn table_actions_track_px(actions: usize, size_medium: f32, spacing_single: f32) -> f32 {
    actions as f32 * (size_medium + spacing_single)
}

/// 🧮️ The x of every track start and the full content width, left to right.
pub fn table_track_offsets(tracks: &[f32]) -> (Vec<f32>, f32) {
    let mut x = 0.0;
    let starts = tracks.iter().map(|width| { let start = x; x += width; start }).collect();
    (starts, x)
}

/// 🗺️ The surface table's column track: React's `TableHost` passes no column widths, so its `<table class="w-full">` shares the available width equally between the columns; a zero-column table owns the whole width.
pub fn table_fill_track_px(width: f32, columns: usize) -> f32 {
    if columns == 0 { width } else { width / columns as f32 }
}

/// 📏️ The width a row (and the header) needs: `columns` fixed tracks plus the actions track.
pub fn table_content_width(columns: usize, actions_px: f32) -> f32 {
    columns as f32 * TABLE_COLUMN_WIDTH_PX + actions_px
}

/// 🗄️ A column's text rect inside a row `row_width` wide, `gap` inset on both sides, `scroll_left` already subtracted, mirrored for a right-to-left row.
pub fn table_column_text_span(row_width: f32, column: usize, gap: f32, rtl: bool) -> (f32, f32) {
    let width = (TABLE_COLUMN_WIDTH_PX - gap * 2.0).max(0.0);
    let offset = gap + column as f32 * TABLE_COLUMN_WIDTH_PX;
    (if rtl { row_width - offset - width } else { offset }, width)
}

/// ↔️ The largest horizontal scroll a viewport of `viewport` pixels reaches over `content` pixels.
pub fn table_max_scroll_left(content: f32, viewport: f32) -> f32 {
    (content - viewport).max(0.0)
}

/// ↔️ Mirrors `tableWindowScrollLeftForColumnV1`.
pub fn table_scroll_left_for_column(index: usize, column_px: f32, scroll_left: f32, width: f32) -> f32 {
    let left = index as f32 * column_px;
    if left < scroll_left {
        left
    } else if left + column_px > scroll_left + width {
        (left + column_px - width).max(0.0)
    } else {
        scroll_left
    }
}

/// ⌨️ Mirrors `tableWindowScrollTopForRowV1`: the scroll top that brings row `index` fully into a viewport of `height` pixels.
pub fn table_scroll_top_for_row(index: usize, row_px: f32, scroll_top: f32, height: f32) -> f32 {
    let top = index as f32 * row_px;
    if top < scroll_top {
        top
    } else if top + row_px > scroll_top + height {
        (top + row_px - height).max(0.0)
    } else {
        scroll_top
    }
}

/// ⌨️ Mirrors `tableWindowNextRowV1`.
pub fn table_next_row(key: &str, current: usize, total: usize, page: usize) -> Option<usize> {
    if total == 0 {
        return None;
    }
    let last = total - 1;
    let page = page.max(1);
    match key {
        "ArrowDown" => Some((current + 1).min(last)),
        "ArrowUp" => Some(current.saturating_sub(1).min(last)),
        "PageDown" => Some((current + page).min(last)),
        "PageUp" => Some(current.saturating_sub(page).min(last)),
        "Home" => Some(0),
        "End" => Some(last),
        _ => None,
    }
}

/// ↔️ Mirrors `tableWindowNextColumnV1`.
pub fn table_next_column(key: &str, current: usize, total: usize) -> Option<usize> {
    if total == 0 {
        return None;
    }
    match key {
        "ArrowRight" => Some((current + 1).min(total - 1)),
        "ArrowLeft" => Some(current.saturating_sub(1)),
        "Home" => Some(0),
        "End" => Some(total - 1),
        _ => None,
    }
}

/// 🎯️ What one key does to a table's row focus and selection.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TableKeyOutcome {
    pub focus: Option<usize>,
    pub select: Option<&'static str>,
    pub activate: bool,
    pub escape: bool,
}

/// ⌨️ The merge mode a modifier chord asks for, as `interactionMergeFromModifiers`: shift range, ctrl or meta invertive, alt subtractive.
pub fn table_merge_for_modifiers(shift: bool, ctrl_or_meta: bool, alt: bool) -> &'static str {
    if shift {
        "range"
    } else if ctrl_or_meta {
        "invertive"
    } else if alt {
        "subtractive"
    } else {
        "replace"
    }
}

/// ⌨️ Row keys: Up/Down/Page/Home/End move focus (Shift extends a range), Enter and Space activate (Ctrl or Meta toggles), Escape returns.
pub fn table_key(key: &str, shift: bool, ctrl_or_meta: bool, current: usize, total: usize, page: usize) -> TableKeyOutcome {
    if let Some(next) = table_next_row(key, current, total, page) {
        return TableKeyOutcome { focus: Some(next), select: shift.then_some("range"), activate: false, escape: false };
    }
    match key {
        "Enter" | " " => TableKeyOutcome { focus: None, select: Some(table_merge_for_modifiers(false, ctrl_or_meta, false)), activate: true, escape: false },
        "Escape" => TableKeyOutcome { focus: None, select: None, activate: false, escape: true },
        _ => TableKeyOutcome { focus: None, select: None, activate: false, escape: false },
    }
}

/// 🔀️ Mirrors `tableSortNextDirectionV1`.
pub fn table_sort_next(column: &str, sort_column: Option<&str>, sort_direction: Option<&str>) -> &'static str {
    if sort_column == Some(column) && sort_direction == Some("asc") { "desc" } else { "asc" }
}

/// ♿️ Mirrors `tableSortAriaV1`.
pub fn table_aria_sort(column: &str, sort_column: Option<&str>, sort_direction: Option<&str>) -> Option<&'static str> {
    (sort_column == Some(column)).then(|| if sort_direction == Some("desc") { "descending" } else { "ascending" })
}

/// 🏷️ The label key React's `Table` resolves an empty body to.
pub const TABLE_EMPTY_LABEL_KEY: &str = "ui.common.noData";

/// 🌍️ The empty-state text for a locale, from the `ui.common.noData` row of the i18n catalogue; `None` for an unknown locale.
pub fn table_empty_label(locale: &str) -> Option<&'static str> {
    match locale {
        "en" => Some("No data"),
        "de" => Some("Keine Daten"),
        _ => None,
    }
}

/// 📻️ The initials `TableAvatar` shows: the first letters of the first two words, upper-cased.
pub fn table_avatar_initials(name: &str) -> String {
    name.trim().split(' ').take(2).filter_map(|word| word.chars().next()).collect::<String>().to_uppercase().chars().take(2).collect()
}

/// 📍️ ⋯ The side of a row's context-menu trigger: React's `ContextMenuTriggerButton` is `size-6` (1.5rem).
pub const TABLE_ROW_MENU_TRIGGER_PX: f32 = 24.0;

/// 📏️ A surface-table row's height: `h-medium`, grown to hold the trigger button every `TableHost` row carries (`onRowContextMenu` is always bound).
pub fn table_row_height(size_medium: f32) -> f32 {
    size_medium.max(TABLE_ROW_MENU_TRIGGER_PX)
}

/// 🪢️ ⋯ The trigger inside `row`: the last cell's content end (`ml-auto`, `px-single` inset), centred in the row.
pub fn table_row_menu_trigger(row: (f32, f32, f32, f32), spacing_single: f32, rtl: bool) -> (f32, f32, f32, f32) {
    let (x, y, w, h) = row;
    let side = TABLE_ROW_MENU_TRIGGER_PX;
    (if rtl { x + spacing_single } else { x + w - spacing_single - side }, y + (h - side) / 2.0, side, side)
}

/// ⚓️ Where the trigger opens the menu: React's `contextMenuAnchorPointV1` over the row (`<tr>`) box, `x = max(left, right - 4)`, `y = min(bottom, top + 24)`.
pub fn table_row_menu_anchor(row: (f32, f32, f32, f32)) -> (f32, f32) {
    let (x, y, w, h) = row;
    ((x + w - 4.0).max(x), (y + h).min(y + 24.0))
}
