//! 📏️ Measurement and utf16: the text measurement results, the `TextSystem` measure and wrap operations and the utf8 to utf16 offset conversions. See [`super`].

use super::*;

//#region 🔖️Measure
//#region 📏️Measurement
/// 📏️ The outcome of measuring one leaf's intrinsic content against the available space taffy offers
/// it. `Pending`/`Failed` are how master.md's decision #1 ("a dependency that is not ready is reported
/// as a pending measurement with a placeholder size — never awaited") is expressed in this crate: an
/// unshaped `TextSystem` run or an undecoded image resource is a `Pending` result carrying whatever
/// placeholder size the caller judges reasonable (e.g. the last known size, or a fixed minimum), never
/// a blocked call. Whoever completes that dependency (the text system's shaping, the resource
/// registry's decode) is responsible for invalidating the window once it lands — this type only
/// records that the frame just built proceeded on a placeholder.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Measurement {
    Ready { width: f32, height: f32 },
    Pending { placeholder_width: f32, placeholder_height: f32 },
    Failed,
}
//#endregion 📏️Measurement

/// 🧮️ `measure`/`measure_wrapped`/`wrap` are the layout-phase surface `layout.rs`'s `MeasureFn` calls
/// into. `next_grapheme`/`previous_grapheme`/`selection_geometry` are the input/edit-phase surface —
/// all three funnel through `parley::Cursor`/`Selection`/`Cluster`, which are grapheme-cluster-safe by
/// construction: per swash's own module docs, a shaping cluster is "equivalent to Unicode grapheme
/// clusters" for the vast majority of scripts, and a complex-script cluster that spans multiple
/// graphemes is still a *safe* (never mid-grapheme) place to land, matching what a real text editor
/// does for those scripts anyway. This is what lets this crate ship grapheme-aware cursor movement
/// without adding a `unicode-segmentation` dependency this packet is forbidden from introducing.
impl TextSystem {
    /// 📏️ Unwrapped intrinsic size. Short-circuits to `Measurement::Pending` without shaping at all
    /// when `style.family` names a [`FontDependencyId`] that has not resolved yet — the placeholder is
    /// a crude `chars × size_px × 0.6` estimate, deliberately not a real shape with the wrong font.
    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    pub fn measure(&mut self, text: &str, style: &TextStyle) -> Measurement {
        if !self.family_ready(style.family) {
            return Self::placeholder_measurement(text, style.size_px);
        }
        let shaped = self.shape(text, style);
        Measurement::Ready { width: shaped.width, height: shaped.height }
    }

    /// 📏️ Wrapped bounding size at `max_width`. Uncached (unlike [`Self::measure`]/[`Self::shape`]) —
    /// wrapping is keyed on a float `max_width` a caller typically varies continuously during resize,
    /// which would fragment a cache rather than serve it.
    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    pub fn measure_wrapped(&mut self, text: &str, style: &TextStyle, max_width: f32) -> Measurement {
        if !self.family_ready(style.family) {
            return Self::placeholder_measurement(text, style.size_px);
        }
        let layout = self.layout_for(text, style, Some(max_width));
        Measurement::Ready { width: layout.width(), height: layout.height() }
    }

    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    pub(super) fn placeholder_measurement(text: &str, size_px: f32) -> Measurement {
        let placeholder_width = text.chars().count() as f32 * size_px * 0.6;
        let placeholder_height = size_px * 1.35;
        Measurement::Pending { placeholder_width, placeholder_height }
    }

    /// ✂️ Line byte-ranges at `max_width`, via parley's own UAX#14 line breaking — never mid-cluster,
    /// unlike the wgpu-old target's whitespace-split `wrap_text`. Returns byte ranges into `text`
    /// rather than owned `String` copies (the old API's shape): a caller that wants owned lines slices
    /// `text` itself, and a caller building glyph runs per line avoids a copy entirely.
    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    pub fn wrap(&mut self, text: &str, style: &TextStyle, max_width: f32) -> Vec<Range<usize>> {
        let layout = self.layout_for(text, style, Some(max_width));
        let mut lines: Vec<Range<usize>> = layout.lines().map(|line| line.text_range()).collect();
        if lines.is_empty() {
            lines.push(0..0);
        }
        lines
    }

    /// ➡️ The next cursor-safe byte offset after `byte_index`, in visual (bidi-correct) order —
    /// `parley::Cursor::next_visual`, which is what arrow-key movement should use in mixed-direction
    /// text. Returns `byte_index` itself at the end of `text` (parley's own `Cursor` clamps rather than
    /// panicking).
    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    pub fn next_grapheme(&mut self, text: &str, style: &TextStyle, byte_index: usize) -> usize {
        let layout = self.layout_for(text, style, None);
        Cursor::from_byte_index(&layout, byte_index, Affinity::Downstream).next_visual(&layout).index()
    }

    /// ⬅️ The previous cursor-safe byte offset before `byte_index` — see [`Self::next_grapheme`].
    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    pub fn previous_grapheme(&mut self, text: &str, style: &TextStyle, byte_index: usize) -> usize {
        let layout = self.layout_for(text, style, None);
        Cursor::from_byte_index(&layout, byte_index, Affinity::Downstream).previous_visual(&layout).index()
    }

    /// 🖍️ Highlight rectangles for `range`, multi-line-safe, via `parley::Selection::geometry`. Returns
    /// `[x, y, width, height]` — `peniko::kurbo::Rect` never crosses this file's public boundary.
    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    pub fn selection_geometry(&mut self, text: &str, style: &TextStyle, range: Range<usize>) -> Vec<[f32; 4]> {
        let layout = self.layout_for(text, style, None);
        let anchor = Cursor::from_byte_index(&layout, range.start, Affinity::Downstream);
        let focus = Cursor::from_byte_index(&layout, range.end, Affinity::Downstream);
        Selection::new(anchor, focus).geometry(&layout).into_iter().map(|(rect, _line)| [rect.x0 as f32, rect.y0 as f32, (rect.x1 - rect.x0) as f32, (rect.y1 - rect.y0) as f32]).collect()
    }
}

/// 🔡️ UTF-8 byte index → UTF-16 code unit index, for platform (IME/accessibility) boundaries. `text`
/// must be sliced at `byte_index`, a char boundary — every caller in this crate reaches one only via
/// `next_grapheme`/`previous_grapheme`/`Cluster::text_range`, all of which only ever produce char
/// boundaries, so the `unwrap_or_else` fallback (treat an invalid index as "end of string") is a
/// defensive floor, not an expected path.
// 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
pub fn utf8_to_utf16(text: &str, byte_index: usize) -> u32 {
    match text.get(..byte_index) {
        Some(prefix) => prefix.encode_utf16().count() as u32,
        None => text.encode_utf16().count() as u32,
    }
}

/// 🔡️ UTF-16 code unit index → UTF-8 byte index — the inverse of [`utf8_to_utf16`]. A non-BMP
/// character (e.g. most emoji) is 2 UTF-16 units and 1 `char`; `ch.len_utf16()` accounts for that so
/// the round trip lands exactly on the character's start, never mid-surrogate-pair.
// 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
pub fn utf16_to_utf8(text: &str, utf16_index: u32) -> usize {
    let mut utf16_count = 0u32;
    for (byte_index, ch) in text.char_indices() {
        if utf16_count >= utf16_index {
            return byte_index;
        }
        utf16_count += ch.len_utf16() as u32;
    }
    text.len()
}
//#endregion 🔖️Measure
