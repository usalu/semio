//! ✂️ Line breaking: the fit epsilon, the wrap spaces, the no-break classes and the break opportunities and unbreakable runs. See [`super`].

use super::*;

// #region text
//#region ✂️LineBreak
/// 📏️ The sub-pixel slack a line is allowed to exceed its box by before it counts as overflowing —
/// one 64th of a logical pixel, the `LayoutUnit` grain a browser resolves fractional layout on. A box
/// sized from these very advances and a painter that re-accumulates them do not have to land on the
/// same last f32 bit, and without the slack a perfectly measured run loses its final glyph to a
/// spurious break.
pub const LINE_BREAK_FIT_EPSILON: f32 = 1.0 / 64.0;

/// ␣️ A scalar CSS `white-space: normal` treats as an inter-word space: it is a break opportunity, it
/// HANGS at the end of a line (a trailing space never pushes a line over its box) and it never starts
/// one. `U+00A0` is deliberately excluded — a no-break space is exactly the character an author
/// writes to forbid the break this predicate would otherwise allow.
pub fn is_wrap_space(ch: char) -> bool {
    ch != '\n' && ch != '\u{00A0}' && ch.is_whitespace()
}

/// ✂️ A scalar a line may break AFTER even with no space — UAX#14 class `BA`/`ZW`/`BB`: the hyphen and
/// dash family, the soft hyphen (`U+00AD`, which CSS breaks at and paints nothing here because the
/// atlas rasterises it as a default-ignorable) and the zero-width space.
fn breaks_after(ch: char) -> bool {
    matches!(ch, '-' | '\u{00AD}' | '\u{200B}' | '\u{2010}' | '\u{2012}' | '\u{2013}' | '\u{2014}')
}

/// 🀄️ A scalar CSS may break on BOTH sides of — UAX#14's `ID` class, the Han/Kana/Hangul/full-width
/// blocks a browser wraps character by character because those scripts write no inter-word space.
fn is_ideographic(ch: char) -> bool {
    matches!(ch as u32, 0x1100..=0x11FF | 0x2E80..=0x303E | 0x3041..=0x33FF | 0x3400..=0x4DBF | 0x4E00..=0x9FFF | 0xA000..=0xA4CF | 0xAC00..=0xD7AF | 0xF900..=0xFAFF | 0xFE30..=0xFE4F | 0xFF00..=0xFF60 | 0xFFE0..=0xFFE6 | 0x20000..=0x3FFFD)
}

/// 🀄️ UAX#14 `CL`/`NS` — a closing bracket, a full stop, a comma or a small kana never starts a line,
/// so the ideographic break opportunity before it is suppressed.
fn is_no_break_before(ch: char) -> bool {
    matches!(
        ch,
        '、' | '。'
            | '，'
            | '．'
            | '！'
            | '？'
            | '：'
            | '；'
            | '」'
            | '』'
            | '）'
            | '】'
            | '］'
            | '｝'
            | '〉'
            | '》'
            | '〕'
            | '〗'
            | '〙'
            | '〛'
            | 'ー'
            | '々'
            | 'ぁ'
            | 'ぃ'
            | 'ぅ'
            | 'ぇ'
            | 'ぉ'
            | 'っ'
            | 'ゃ'
            | 'ゅ'
            | 'ょ'
            | 'ァ'
            | 'ィ'
            | 'ゥ'
            | 'ェ'
            | 'ォ'
            | 'ッ'
            | 'ャ'
            | 'ュ'
            | 'ョ'
            | '・'
            | '゠'
    )
}

/// 🀄️ UAX#14 `OP` — an opening bracket never ends a line, so the ideographic break opportunity after
/// it is suppressed.
fn is_no_break_after(ch: char) -> bool {
    matches!(ch, '「' | '『' | '（' | '【' | '［' | '｛' | '〈' | '《' | '〔' | '〖' | '〘' | '〚')
}

/// ✂️ **THE** soft-wrap predicate of this target: may a line break between `prev` and `next`? It is
/// CSS `word-break: normal` / `overflow-wrap: normal`, term for term — after a run of spaces, after a
/// hyphen/dash/soft-hyphen/zero-width-space, and on either side of an ideograph except where a
/// closing or opening bracket forbids it. NOTHING else breaks, which is what keeps a word whole.
///
/// 🩸️ Every wrapped run in this target funnels through here — [`FontAtlas::wrap_lines`],
/// [`FontAtlas::measure_text_wrapped`], `widgets::wrap_text`, `paint`'s retained glyph stepper and
/// `mounted_layout`'s worker-side intrinsic measure. They used to disagree: the measures split on
/// whitespace while the PAINTER broke at whatever glyph happened to overflow, so a card measured for
/// word wrap was painted `comp|osing` (ticket 26/09/17/WGPU-RENDERER-REACT-PARITY packet W9a).
/// A break inside a word is now reachable only through [`FontAtlas::wrap_lines`]'s last-resort arm,
/// when one unbreakable run is wider than the whole box — CSS's `overflow-wrap` fallback.
pub fn may_break_between(prev: char, next: char) -> bool {
    if is_wrap_space(prev) {
        return !is_wrap_space(next);
    }
    if is_wrap_space(next) {
        return false;
    }
    if breaks_after(prev) {
        return true;
    }
    if is_ideographic(prev) || is_ideographic(next) {
        return !is_no_break_before(next) && !is_no_break_after(prev);
    }
    false
}

/// ✂️ Whether a soft wrap may occur immediately BEFORE the scalar at `byte`. `byte` must be a char
/// boundary; the start and the end of `text` are never break opportunities.
pub fn is_break_opportunity(text: &str, byte: usize) -> bool {
    if byte == 0 || byte >= text.len() || !text.is_char_boundary(byte) {
        return false;
    }
    let (Some(prev), Some(next)) = (text[..byte].chars().next_back(), text[byte..].chars().next()) else { return false };
    may_break_between(prev, next)
}

/// ✂️ The end of the unbreakable run starting at `byte` — the span a greedy wrap must fit WHOLE or
/// move down. Stops at the next break opportunity, at a space (spaces hang, so they are priced on the
/// line they end) and at a newline.
pub fn unbreakable_run_end(text: &str, byte: usize) -> usize {
    let mut end = byte;
    while let Some(ch) = text.get(end..).and_then(|rest| rest.chars().next()) {
        if ch == '\n' || is_wrap_space(ch) || (end > byte && is_break_opportunity(text, end)) {
            break;
        }
        end += ch.len_utf8();
    }
    end
}
//#endregion ✂️LineBreak
// #endregion text
