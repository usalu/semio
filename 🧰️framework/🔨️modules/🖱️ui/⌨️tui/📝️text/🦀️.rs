//! 📝️ Terminal text model: grapheme clusters (UAX #29), cell widths (UAX #11 plus emoji presentation,
//! VS15/VS16, ZWJ sequences, flags, keycaps) and ellipsis helpers. Every function measures and cuts
//! whole clusters, so a name such as `🧰️framework` is the same number of cells here, in the cell
//! buffer and on a terminal that implements grapheme clustering.
//! See <https://www.unicode.org/reports/tr29/#Grapheme_Cluster_Boundaries> and
//! <https://www.unicode.org/reports/tr11/>.

use std::borrow::Cow;

#[path = "🔤️tables/🦀️.rs"]
mod tables;

use tables::{Break, BREAKS, EMOJI_MODIFIER_BASE, EMOJI_ZWJ, INCB_EXTEND, INCB_LINKER, VS15_NARROW, VS16_BASE, WIDE, ZERO};

//#region 📏️Scalar Classes
fn in_ranges(codepoint: u32, ranges: &[(u32, u32)]) -> bool {
    let at = ranges.partition_point(|&(first, _)| first <= codepoint);
    at > 0 && codepoint <= ranges[at - 1].1
}

fn break_class(c: char) -> Break {
    let codepoint = c as u32;
    if (0x20..0x7f).contains(&codepoint) {
        return Break::Any;
    }
    let at = BREAKS.partition_point(|&(first, _, _)| first <= codepoint);
    match at.checked_sub(1).map(|index| BREAKS[index]) {
        Some((_, last, class)) if codepoint <= last => class,
        _ => Break::Any,
    }
}

/// 📏️ Terminal cell width of one scalar: 0 for controls, joiners, marks and variation selectors, 2 for wide, else 1.
pub(crate) fn char_cells(c: char) -> u8 {
    let codepoint = c as u32;
    if codepoint < 0x7f {
        return u8::from(codepoint >= 0x20);
    }
    if codepoint < 0xa0 || in_ranges(codepoint, ZERO) {
        0
    } else if in_ranges(codepoint, WIDE) {
        2
    } else {
        1
    }
}
//#endregion 📏️Scalar Classes

//#region 🧩️Cluster Segmentation
struct Scan {
    previous: Break,
    regional: u8,
    emoji: u8,
    conjunct: u8,
}

impl Scan {
    fn new(first: char) -> Self {
        let mut scan = Self { previous: Break::Any, regional: 0, emoji: 0, conjunct: 0 };
        scan.advance(break_class(first), first);
        scan
    }

    fn breaks_before(&self, current: Break) -> bool {
        use Break::*;
        match (self.previous, current) {
            (CR, LF) => false,
            (Control | CR | LF, _) | (_, Control | CR | LF) => true,
            (L, L | V | LV | Lvt) | (LV | V, V | T) | (Lvt | T, T) => false,
            (_, Extend | Zwj | SpacingMark) | (Prepend, _) => false,
            (RegionalIndicator, RegionalIndicator) => self.regional.is_multiple_of(2),
            (_, InCBConsonant) => self.conjunct != 2,
            (_, ExtendedPictographic) => self.emoji != 2,
            _ => true,
        }
    }

    fn advance(&mut self, current: Break, c: char) {
        use Break::*;
        let codepoint = c as u32;
        self.regional = if current == RegionalIndicator { self.regional.saturating_add(1) } else { 0 };
        self.emoji = match (self.emoji, current) {
            (_, ExtendedPictographic) => 1,
            (1, Extend) => 1,
            (1, Zwj) => 2,
            _ => 0,
        };
        self.conjunct = if current == InCBConsonant {
            1
        } else if INCB_LINKER.binary_search(&codepoint).is_ok() {
            if self.conjunct > 0 { 2 } else { 0 }
        } else if in_ranges(codepoint, INCB_EXTEND) {
            self.conjunct
        } else {
            0
        };
        self.previous = current;
    }
}

fn cluster_len(s: &str) -> usize {
    let bytes = s.as_bytes();
    let Some(&lead) = bytes.first() else { return 0 };
    let single = match bytes.get(1) {
        None => true,
        Some(&next) => lead < 0x80 && next < 0x80 && !(lead == b'\r' && next == b'\n'),
    };
    if single && lead < 0x80 {
        return 1;
    }
    let mut chars = s.char_indices();
    let Some((_, first)) = chars.next() else { return 0 };
    let mut scan = Scan::new(first);
    for (index, c) in chars {
        let class = break_class(c);
        if scan.breaks_before(class) {
            return index;
        }
        scan.advance(class, c);
    }
    s.len()
}

/// 🧩️ Iterator over the extended grapheme clusters of a string, in order.
#[derive(Clone)]
pub struct Clusters<'a> {
    rest: &'a str,
}

impl<'a> Iterator for Clusters<'a> {
    type Item = &'a str;

    fn next(&mut self) -> Option<&'a str> {
        let len = cluster_len(self.rest);
        if len == 0 {
            return None;
        }
        let (cluster, rest) = self.rest.split_at(len);
        self.rest = rest;
        Some(cluster)
    }
}

/// 🔪️ Splits `s` into extended grapheme clusters.
pub fn clusters(s: &str) -> Clusters<'_> {
    Clusters { rest: s }
}
//#endregion 🧩️Cluster Segmentation

//#region 📐️Cluster Widths
/// 🎚️ How a terminal counts text: `Cluster` understands grapheme clusters, VS16 and ZWJ sequences; `Scalar`
/// is the wcwidth view that draws every visible scalar on its own and drops joiners and variation selectors.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum WidthMode {
    #[default]
    Cluster,
    Scalar,
}

fn emoji_modifier(c: char) -> bool {
    ('\u{1f3fb}'..='\u{1f3ff}').contains(&c)
}

fn emoji_element(s: &str) -> Option<(usize, bool)> {
    let mut chars = s.chars();
    let base = chars.next()?;
    let code = base as u32;
    let after = chars.as_str();
    if in_ranges(code, EMOJI_MODIFIER_BASE) {
        if let Some(modifier) = after.chars().next().filter(|&c| emoji_modifier(c)) {
            return Some((base.len_utf8() + modifier.len_utf8(), true));
        }
    }
    if in_ranges(code, VS16_BASE) {
        return after.starts_with('\u{fe0f}').then_some((base.len_utf8() + '\u{fe0f}'.len_utf8(), true));
    }
    in_ranges(code, EMOJI_ZWJ).then_some((base.len_utf8(), false))
}

fn emoji_sequence(cluster: &str) -> Option<usize> {
    let (mut end, mut featured) = emoji_element(cluster)?;
    while let Some((len, _)) = cluster[end..].strip_prefix('\u{200d}').and_then(emoji_element) {
        end += '\u{200d}'.len_utf8() + len;
        featured = true;
    }
    if cluster[end..].starts_with('\u{fe0f}') {
        end += '\u{fe0f}'.len_utf8();
    }
    featured.then_some(end)
}

fn scalar_sum(s: &str) -> u8 {
    s.chars().fold(0u8, |total, c| total.saturating_add(char_cells(c)))
}

thread_local! {
    static ACTIVE: std::cell::Cell<WidthMode> = const { std::cell::Cell::new(WidthMode::Cluster) };
}

/// 🧭 The width mode every width computation of this thread follows; a `CellBuffer` configured with
/// `set_width_mode` makes its mode the active one, so painting, truncation, cursor columns and hit tests agree.
pub fn active_width_mode() -> WidthMode {
    ACTIVE.with(std::cell::Cell::get)
}

pub fn set_active_width_mode(mode: WidthMode) {
    ACTIVE.with(|active| active.set(mode));
}

/// 📐️ Terminal cell width of one grapheme cluster: well-formed emoji sequences, flags and presentation
/// sequences take two cells, text presentation of a wide emoji one, everything else the sum of its scalars.
fn cluster_cells_unicode(cluster: &str) -> u8 {
    let mut chars = cluster.chars();
    let Some(base) = chars.next() else { return 0 };
    let base_cells = char_cells(base);
    let rest = chars.as_str();
    if rest.is_empty() {
        return base_cells;
    }
    if break_class(base) == Break::RegionalIndicator {
        if let Some(second) = rest.chars().next().filter(|&c| break_class(c) == Break::RegionalIndicator) {
            return 2u8.saturating_add(scalar_sum(&rest[second.len_utf8()..]));
        }
    }
    if let Some(end) = emoji_sequence(cluster) {
        return 2u8.saturating_add(scalar_sum(&cluster[end..]));
    }
    let sum = scalar_sum(cluster);
    let base_code = base as u32;
    if base_cells == 1 && rest.starts_with('\u{fe0f}') && in_ranges(base_code, VS16_BASE) {
        return sum.saturating_add(1);
    }
    if base_cells == 2 && rest.starts_with('\u{fe0e}') && in_ranges(base_code, VS15_NARROW) {
        return sum.saturating_sub(1);
    }
    sum
}

/// 🔢 Width of a unit of text in `mode`: a whole cluster in `Cluster` mode, the sum of its scalars in `Scalar` mode.
pub fn cluster_cells_in(cluster: &str, mode: WidthMode) -> u8 {
    match mode {
        WidthMode::Cluster => cluster_cells_unicode(cluster),
        WidthMode::Scalar => scalar_sum(cluster),
    }
}

/// 🔁 `cluster_cells_in` for the active width mode.
pub fn cluster_cells(cluster: &str) -> u8 {
    cluster_cells_in(cluster, active_width_mode())
}

/// 🧮 Total display width of a string in terminal cells under `mode`.
pub fn display_width_in(s: &str, mode: WidthMode) -> u16 {
    if s.is_ascii() {
        return s.bytes().filter(|byte| (0x20..0x7f).contains(byte)).count().min(usize::from(u16::MAX)) as u16;
    }
    match mode {
        WidthMode::Scalar => s.chars().fold(0u16, |total, c| total.saturating_add(u16::from(char_cells(c)))),
        WidthMode::Cluster => clusters(s).fold(0u16, |total, cluster| total.saturating_add(u16::from(cluster_cells_unicode(cluster)))),
    }
}

/// 🔂 `display_width_in` for the active width mode.
pub fn display_width(s: &str) -> u16 {
    display_width_in(s, active_width_mode())
}

/// 🧱️ The units a width mode counts: grapheme clusters, or single scalars.
#[derive(Clone)]
struct Units<'a> {
    rest: &'a str,
    mode: WidthMode,
}

impl<'a> Iterator for Units<'a> {
    type Item = &'a str;

    fn next(&mut self) -> Option<&'a str> {
        let len = match self.mode {
            WidthMode::Cluster => cluster_len(self.rest),
            WidthMode::Scalar => self.rest.chars().next().map_or(0, char::len_utf8),
        };
        if len == 0 {
            return None;
        }
        let (unit, rest) = self.rest.split_at(len);
        self.rest = rest;
        Some(unit)
    }
}

fn units(s: &str, mode: WidthMode) -> Units<'_> {
    Units { rest: s, mode }
}

//#endregion 📐️Cluster Widths

//#region ✂️Truncation And Ellipsis
/// 🔠️ The ellipsis glyph; one cell wide.
pub const ELLIPSIS: &str = "\u{2026}";

/// ✂️ Truncates `s` to at most `max_cells` display cells without splitting a unit of `mode`; returns the slice and its width.
pub fn truncate_to_in(s: &str, max_cells: u16, mode: WidthMode) -> (&str, u16) {
    let mut used = 0u16;
    let mut end = 0usize;
    for unit in units(s, mode) {
        let width = u16::from(cluster_cells_in(unit, mode));
        if used.saturating_add(width) > max_cells {
            break;
        }
        used += width;
        end += unit.len();
    }
    (&s[..end], used)
}

/// 🪚 `truncate_to_in` for the active width mode.
pub fn truncate_to(s: &str, max_cells: u16) -> (&str, u16) {
    truncate_to_in(s, max_cells, active_width_mode())
}

fn suffix_within(s: &str, max_cells: u16, mode: WidthMode) -> (&str, u16) {
    let mut remaining = display_width_in(s, mode);
    let mut start = 0usize;
    for unit in units(s, mode) {
        if remaining <= max_cells {
            break;
        }
        remaining -= u16::from(cluster_cells_in(unit, mode));
        start += unit.len();
    }
    (&s[start..], remaining)
}

/// 🪡️ Where an over-long text loses its cells.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Elision {
    End,
    Start,
    Middle { tail: u16 },
}

/// 🩹 Fits `s` into `max_cells` under `mode` by replacing the cut part with `ellipsis`; text that fits is returned as is.
/// A `Middle` cut keeps up to `tail` trailing cells (the discriminating end of a name) and as much head as fits.
pub fn elide_in<'a>(s: &'a str, max_cells: u16, elision: Elision, ellipsis: &str, mode: WidthMode) -> Cow<'a, str> {
    if display_width_in(s, mode) <= max_cells {
        return Cow::Borrowed(s);
    }
    let mark = display_width_in(ellipsis, mode);
    if max_cells < mark {
        return Cow::Owned(truncate_to_in(ellipsis, max_cells, mode).0.to_string());
    }
    let room = max_cells - mark;
    let text = match elision {
        Elision::End => format!("{}{ellipsis}", truncate_to_in(s, room, mode).0),
        Elision::Start => format!("{ellipsis}{}", suffix_within(s, room, mode).0),
        Elision::Middle { tail } => {
            let (tail_text, tail_width) = suffix_within(s, tail.min(room.saturating_sub(1)), mode);
            format!("{}{ellipsis}{tail_text}", truncate_to_in(s, room - tail_width, mode).0)
        }
    };
    Cow::Owned(text)
}

/// 🪅 `elide_in` for the active width mode.
pub fn elide<'a>(s: &'a str, max_cells: u16, elision: Elision, ellipsis: &str) -> Cow<'a, str> {
    elide_in(s, max_cells, elision, ellipsis, active_width_mode())
}

/// 🔚️ Fits `s` into `max_cells`, cutting its end and marking the cut with `…`.
pub fn elide_end(s: &str, max_cells: u16) -> Cow<'_, str> {
    elide(s, max_cells, Elision::End, ELLIPSIS)
}

/// 🔛️ Fits `s` into `max_cells`, cutting its start and marking the cut with `…`.
pub fn elide_start(s: &str, max_cells: u16) -> Cow<'_, str> {
    elide(s, max_cells, Elision::Start, ELLIPSIS)
}

/// 🤏️ Fits `s` into `max_cells`, cutting its middle with `…` and keeping up to `tail_cells` trailing cells.
pub fn elide_middle(s: &str, max_cells: u16, tail_cells: u16) -> Cow<'_, str> {
    elide(s, max_cells, Elision::Middle { tail: tail_cells }, ELLIPSIS)
}
//#endregion ✂️Truncation And Ellipsis

//#region 🧵️Cursor Positions
/// 🧵️ The byte offset of the cluster boundary at or before `byte`.
pub fn boundary_at_or_before(s: &str, byte: usize) -> usize {
    let mut start = 0usize;
    for cluster in clusters(s) {
        if start + cluster.len() > byte {
            return start;
        }
        start += cluster.len();
    }
    s.len()
}

/// ◀️ The byte offset of the cluster boundary before `byte` (0 at the start).
pub fn previous_boundary(s: &str, byte: usize) -> usize {
    let mut previous = 0usize;
    let mut start = 0usize;
    for cluster in clusters(s) {
        if start >= byte {
            break;
        }
        previous = start;
        start += cluster.len();
    }
    previous
}

/// ▶️ The byte offset of the cluster boundary after `byte` (`s.len()` at the end).
pub fn next_boundary(s: &str, byte: usize) -> usize {
    let mut start = 0usize;
    for cluster in clusters(s) {
        start += cluster.len();
        if start > byte {
            return start;
        }
    }
    s.len()
}

/// 🎯️ The byte offset of the cluster that covers display column `cell` of `s` under `mode` (`s.len()` past the end).
pub fn boundary_at_cell_in(s: &str, cell: u16, mode: WidthMode) -> usize {
    let mut column = 0u16;
    let mut start = 0usize;
    for unit in units(s, mode) {
        let width = u16::from(cluster_cells_in(unit, mode));
        if width > 0 && cell >= column && cell < column + width {
            return boundary_at_or_before(s, start);
        }
        column = column.saturating_add(width);
        start += unit.len();
    }
    s.len()
}

/// 🧿 `boundary_at_cell_in` for the active width mode.
pub fn boundary_at_cell(s: &str, cell: u16) -> usize {
    boundary_at_cell_in(s, cell, active_width_mode())
}

/// 🪟️ The part of `s` visible when its first `start` cells scroll out of a view `width` cells wide under `mode`: the
/// slice, and how many blank cells lead it because a wide unit straddles the left edge.
pub fn window_cells_in(s: &str, start: u16, width: u16, mode: WidthMode) -> (&str, u16) {
    let mut column = 0u16;
    let mut from = s.len();
    let mut lead = 0u16;
    let mut offset = 0usize;
    for unit in units(s, mode) {
        let cells = u16::from(cluster_cells_in(unit, mode));
        if column >= start {
            from = offset;
            lead = column - start;
            break;
        }
        column = column.saturating_add(cells);
        offset += unit.len();
        if column > start {
            lead = column - start;
            from = offset;
            break;
        }
    }
    let (visible, _) = truncate_to_in(&s[from..], width.saturating_sub(lead), mode);
    (visible, lead.min(width))
}

/// 🔭 `window_cells_in` for the active width mode.
pub fn window_cells(s: &str, start: u16, width: u16) -> (&str, u16) {
    window_cells_in(s, start, width, active_width_mode())
}
//#endregion 🧵️Cursor Positions

#[cfg(test)]
#[path = "../🧪️tests/📏️text-width/🦀️.rs"]
mod tests;
