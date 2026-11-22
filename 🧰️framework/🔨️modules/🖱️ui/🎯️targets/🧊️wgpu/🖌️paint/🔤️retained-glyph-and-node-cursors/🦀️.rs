//! 🔤️ Retained glyph and node cursors: the tree and text limits, the glyph cursor and its weighted and flowed steps and the node paint cursor with its step. See [`super`].

use super::*;

// #region paint
pub const TREE_INDENT_PER_LEVEL: f32 = (ui_styling::metrics::chrome::UI_SPACING_COMPACT_PX * ui_styling::metrics::dom::TREE_INDENT_PER_LEVEL_UI_SPACING) as f32;

pub const TREE_TOGGLE_WIDTH: f32 = (ui_styling::metrics::chrome::UI_SPACING_COMPACT_PX * ui_styling::metrics::dom::TREE_TOGGLE_UI_SPACING) as f32;

pub const TREE_ICON_SIZE: f32 = semio_ui_wgpu_layout::chrome::ICON_TREE_ROW;

pub const RETAINED_NODE_TEXT_MAX_BYTES: usize = 4 * 1024 * 1024;

pub const RETAINED_NODE_COLLECTION_ITEMS: usize = 256;

pub(super) const RETAINED_NODE_FIXED_OUTPUT_ITEMS: usize = 16;

pub const RETAINED_NUMBER_STEPPER_CHROME_OUTPUT_ITEMS: usize = 8;

pub(super) const RETAINED_NODE_FIXED_OUTPUT_BYTES: usize = 64 * 1024;

pub(super) const RETAINED_TREE_DEPTH: usize = 64;

/// ✒️ Retained byte, glyph, line, and pen authority shared by node and Shell text.
#[derive(Default)]
pub struct RetainedGlyphCursor {
    pub(super) byte: usize,
    pub(super) line: usize,
    pen_x: f32,
    /// 🔠️ Whether the run is set in the mono face (React's `font-mono`); the sans face otherwise.
    pub(super) mono: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RetainedGlyphStep {
    Pending,
    Complete,
    Fault,
}

/// ✂️ How a retained run treats a scalar that does not fit `bounds.w`. `Wrap` is CSS's default
/// `white-space: normal` — the run drops to the next line box, which is what a document `Text`
/// node does. `Clip` is React's chrome pairing of `whitespace-nowrap` + `truncate`
/// (`🎛️chrome-control-presentation/🟦️.ts`, and every chip label in `🏛️ShellHost/🟦️.tsx`): a chip
/// label stays on ONE line and whatever overflows its box is simply not painted. Chrome must never
/// wrap — a two-line chip label is the visible symptom of a measure/paint disagreement, not a
/// layout the reference ever produces.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum RetainedTextFlow {
    #[default]
    Wrap,
    Clip,
}

impl RetainedGlyphCursor {
    pub fn reset(&mut self) {
        self.byte = 0;
        self.line = 0;
        self.pen_x = 0.0;
    }

    pub fn byte(&self) -> usize {
        self.byte
    }

    /// 🖊️ The line box the pen is currently on — zero for the first. What the wrap law reads to
    /// assert that the painter lands every scalar on the line `FontAtlas::wrap_lines` measured it onto.
    pub fn line(&self) -> usize {
        self.line
    }

    pub fn close_step(&mut self) -> bool {
        if self.byte != 0 || self.line != 0 || self.pen_x != 0.0 {
            self.reset();
            return false;
        }
        true
    }

    pub fn terminal_is_empty(&self) -> bool {
        self.byte == 0 && self.line == 0 && self.pen_x == 0.0
    }
}

/// 📏️ The sub-pixel slack a run's last glyph is allowed to exceed its box by before the run counts
/// as overflowing — one 64th of a logical pixel, the same `LayoutUnit` grain a browser resolves
/// fractional layout on. A box sized from `FontAtlas::measure_text` is the SUM of the very advances
/// this painter then re-accumulates, and f32 addition does not have to land on the same last bit:
/// without the slack a perfectly measured chip label loses its final glyph (`Chat` → `Cha`) or, in
/// [`RetainedTextFlow::Wrap`], drops that glyph onto a second line.
const RETAINED_TEXT_FIT_EPSILON: f32 = semio_ui_text::LINE_BREAK_FIT_EPSILON;

/// 🪜️ Advances one UTF-8 scalar and at most one exactly admitted glyph, wrapping at `bounds.w`.
pub fn paint_retained_glyph_step(value: &str, bounds: Rect, size: f32, color: Rgba, atlas: &mut FontAtlas, draw: &mut DrawList, cursor: &mut RetainedGlyphCursor) -> RetainedGlyphStep {
    paint_retained_glyph_step_flowed(value, bounds, size, color, RetainedTextFlow::Wrap, atlas, draw, cursor)
}

/// 🅰️ [`paint_retained_glyph_step_flowed`] at an explicit [`TextWeight`] — the ONE retained entry a
/// caller reaches for React's `font-semibold`. A [`TextWeight::Semibold`] scalar is struck twice,
/// the second strike offset by [`crate::wgpu::text::faux_bold_offset`], and the step's own retained
/// reservation is priced at `weight.strikes()` items so the second strike can never overrun a budget
/// sized for one (see `TextWeight::strikes`'s own doc comment, and the law
/// `a_semibold_glyph_strike_prices_and_paints_two_instances`).
#[allow(clippy::too_many_arguments, reason = "one arg per retained text input; a struct here is a T2 restructure of every glyph call site")]
pub fn paint_retained_glyph_step_weighted(
    value: &str,
    bounds: Rect,
    size: f32,
    color: Rgba,
    flow: RetainedTextFlow,
    weight: semio_ui_text::TextWeight,
    atlas: &mut FontAtlas,
    draw: &mut DrawList,
    cursor: &mut RetainedGlyphCursor,
) -> RetainedGlyphStep {
    paint_retained_glyph_step_inner(value, bounds, size, color, flow, weight, 0.0, atlas, draw, cursor)
}

/// 🔠️ [`paint_retained_glyph_step_weighted`] with letter spacing: `tracking` extra logical pixels after every glyph (React's `tracking-wide`, `Theme::tracking_wide` × the font size).
#[allow(clippy::too_many_arguments, reason = "one arg per retained text input; a struct here is a T2 restructure of every glyph call site")]
pub fn paint_retained_glyph_step_tracked(value: &str, bounds: Rect, size: f32, color: Rgba, flow: RetainedTextFlow, weight: semio_ui_text::TextWeight, tracking: f32, atlas: &mut FontAtlas, draw: &mut DrawList, cursor: &mut RetainedGlyphCursor) -> RetainedGlyphStep {
    paint_retained_glyph_step_inner(value, bounds, size, color, flow, weight, tracking, atlas, draw, cursor)
}

/// 🌊️ [`paint_retained_glyph_step`] with an explicit [`RetainedTextFlow`] — the entry chrome text
/// takes so a chip label can never break onto a second line.
///
/// ✂️ **A [`RetainedTextFlow::Wrap`] run breaks where CSS breaks it.** Before the glyph at a
/// [`crate::wgpu::text::is_break_opportunity`] is penned, the WHOLE unbreakable run it starts is
/// priced ([`crate::wgpu::text::unbreakable_run_end`]) and moved down as one if it does not fit; the
/// per-glyph overflow arm below is only reachable once that run is alone on its line and still wider
/// than the box, which is CSS's `overflow-wrap` last resort. A space never triggers either arm — a
/// trailing space hangs. This stepper used to break at whatever glyph happened to overflow, which is
/// why a card measured for word wrap painted `comp|osing`
/// (ticket 26/09/17/WGPU-RENDERER-REACT-PARITY packet W9a).
#[allow(clippy::too_many_arguments, reason = "one arg per retained text input; a struct here is a T2 restructure of every glyph call site")]
pub fn paint_retained_glyph_step_flowed(value: &str, bounds: Rect, size: f32, color: Rgba, flow: RetainedTextFlow, atlas: &mut FontAtlas, draw: &mut DrawList, cursor: &mut RetainedGlyphCursor) -> RetainedGlyphStep {
    paint_retained_glyph_step_inner(value, bounds, size, color, flow, semio_ui_text::TextWeight::Regular, 0.0, atlas, draw, cursor)
}

#[allow(clippy::too_many_arguments, reason = "one arg per retained text input; a struct here is a T2 restructure of every glyph call site")]
fn paint_retained_glyph_step_inner(value: &str, bounds: Rect, size: f32, color: Rgba, flow: RetainedTextFlow, weight: semio_ui_text::TextWeight, tracking: f32, atlas: &mut FontAtlas, draw: &mut DrawList, cursor: &mut RetainedGlyphCursor) -> RetainedGlyphStep {
    if value.len() > RETAINED_NODE_TEXT_MAX_BYTES || !value.is_char_boundary(cursor.byte) {
        return RetainedGlyphStep::Fault;
    }
    if cursor.byte == value.len() {
        return RetainedGlyphStep::Complete;
    }
    let Some(ch) = value[cursor.byte..].chars().next() else { return RetainedGlyphStep::Fault };
    let Some(next_byte) = cursor.byte.checked_add(ch.len_utf8()) else { return RetainedGlyphStep::Fault };
    if ch == '\n' {
        if matches!(flow, RetainedTextFlow::Clip) {
            cursor.byte = next_byte;
            return RetainedGlyphStep::Pending;
        }
        let Some(next_line) = cursor.line.checked_add(1) else { return RetainedGlyphStep::Fault };
        cursor.byte = next_byte;
        cursor.line = next_line;
        cursor.pen_x = 0.0;
        return RetainedGlyphStep::Pending;
    }
    let face = retained_face(cursor.mono);
    let mut kerning = if cursor.pen_x > 0.0 { retained_kerning(atlas, face, value, cursor.byte, size) } else { 0.0 };
    if matches!(flow, RetainedTextFlow::Wrap) && cursor.pen_x > 0.0 && semio_ui_text::is_break_opportunity(value, cursor.byte) {
        let run = atlas.measure_range_face(face, value, cursor.byte, semio_ui_text::unbreakable_run_end(value, cursor.byte), size);
        if cursor.pen_x + kerning + run > bounds.w.max(1.0) + RETAINED_TEXT_FIT_EPSILON {
            let Some(next_line) = cursor.line.checked_add(1) else { return RetainedGlyphStep::Fault };
            cursor.line = next_line;
            cursor.pen_x = 0.0;
            kerning = 0.0;
        }
    }
    let strikes = weight.strikes();
    if draw.begin_retained_output(strikes, strikes * size_of::<semio_ui_draw_contract::UiInstance>()).is_err() {
        return RetainedGlyphStep::Fault;
    }
    let atlas_w = atlas.width as f32;
    let atlas_h = atlas.height as f32;
    let glyph = atlas.ensure_glyph_for(face, ch, size);
    let (atlas_x, atlas_y, width, height, advance, bearing_x, bearing_y) = (glyph.atlas_x, glyph.atlas_y, glyph.width, glyph.height, glyph.advance, glyph.bearing_x, glyph.bearing_y);
    let (logical_w, logical_h) = (glyph.logical_width(), glyph.logical_height());
    let hangs = matches!(flow, RetainedTextFlow::Wrap) && semio_ui_text::is_wrap_space(ch);
    let overflows = !hangs && cursor.pen_x > 0.0 && cursor.pen_x + kerning + advance > bounds.w.max(1.0) + RETAINED_TEXT_FIT_EPSILON;
    if overflows && matches!(flow, RetainedTextFlow::Clip) {
        if draw.finish_retained_output().is_err() {
            return RetainedGlyphStep::Fault;
        }
        cursor.byte = next_byte;
        cursor.pen_x += kerning + advance + tracking;
        return RetainedGlyphStep::Pending;
    }
    if overflows {
        let Some(next_line) = cursor.line.checked_add(1) else {
            let _ = draw.finish_retained_output();
            return RetainedGlyphStep::Fault;
        };
        cursor.line = next_line;
        cursor.pen_x = 0.0;
        kerning = 0.0;
    }
    cursor.pen_x += kerning;
    let line_height = semio_ui_text::line_height(size);
    let baseline = match flow {
        RetainedTextFlow::Wrap => bounds.y + line_height * (cursor.line as f32 + 1.0),
        RetainedTextFlow::Clip => bounds.y + size + line_height * cursor.line as f32,
    };
    let x = bounds.x + cursor.pen_x + bearing_x;
    let y = baseline - logical_h - bearing_y;
    let uv = [atlas_x as f32 / atlas_w, atlas_y as f32 / atlas_h, (atlas_x + width) as f32 / atlas_w, (atlas_y + height) as f32 / atlas_h];
    draw.push_glyph([x, y, logical_w.max(1.0), logical_h.max(1.0)], color, uv);
    if let Some(offset) = weight.synthetic_offset(size) {
        draw.push_glyph([x + offset, y, logical_w.max(1.0), logical_h.max(1.0)], color, uv);
    }
    if draw.finish_retained_output().is_err() {
        return RetainedGlyphStep::Fault;
    }
    cursor.byte = next_byte;
    cursor.pen_x += advance + tracking;
    RetainedGlyphStep::Pending
}

/// 🧭️ One scalar of a field's text as the wrap law places it: its byte range, the line it lands on and the pen interval it spans there.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub(super) struct RetainedProbeGlyph {
    pub(super) start: usize,
    pub(super) end: usize,
    pub(super) line: usize,
    pub(super) x0: f32,
    pub(super) x1: f32,
}

/// 🔠️ The face a retained text run is set in: the mono face for React's `font-mono` fields, the sans face otherwise.
fn retained_face(mono: bool) -> semio_ui_text::TextFace {
    if mono { semio_ui_text::TextFace::Mono } else { semio_ui_text::TextFace::Sans }
}

/// 🤝️ The pair kerning before the scalar at `byte` in `face`.
fn retained_kerning(atlas: &mut FontAtlas, face: semio_ui_text::TextFace, value: &str, byte: usize, size: f32) -> f32 {
    if face == semio_ui_text::TextFace::Sans {
        return atlas.kerning_at(value, byte, size);
    }
    match (value.get(..byte).and_then(|before| before.chars().next_back()), value.get(byte..).and_then(|after| after.chars().next())) {
        (Some(left), Some(right)) => atlas.kerning_for(face, left, right, size),
        _ => 0.0,
    }
}

/// 📐️ The incremental line-and-pen walk over a field's text: one scalar per step, the same breaks `FontAtlas::wrap_lines` and the glyph painter take.
#[derive(Clone, Copy, Debug, Default)]
pub(super) struct RetainedWrapProbe {
    pub(super) mono: bool,
    pub(super) byte: usize,
    pub(super) line: usize,
    pub(super) pen: f32,
    pub(super) glyph: RetainedProbeGlyph,
    pub(super) selection: RetainedSpanRun,
    pub(super) preedit: RetainedSpanRun,
}

/// 🖍️ The open rectangle of one highlighted span on the line the probe is on.
#[derive(Clone, Copy, Debug, Default)]
pub(super) struct RetainedSpanRun {
    pub(super) open: bool,
    pub(super) line: usize,
    pub(super) x0: f32,
    pub(super) x1: f32,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) enum RetainedProbeStep {
    Glyph,
    Done,
    Fault,
}

/// 🪜️ Advances `probe` by one scalar of `value` wrapped at `width` (`multi`) or laid on one unbounded line, leaving the scalar in `probe.glyph`.
pub(super) fn retained_wrap_probe_step(value: &str, width: f32, size: f32, multi: bool, atlas: &mut FontAtlas, probe: &mut RetainedWrapProbe) -> RetainedProbeStep {
    if value.len() > RETAINED_NODE_TEXT_MAX_BYTES || !value.is_char_boundary(probe.byte) {
        return RetainedProbeStep::Fault;
    }
    let Some(ch) = value[probe.byte..].chars().next() else { return RetainedProbeStep::Done };
    let start = probe.byte;
    let end = start + ch.len_utf8();
    if ch == '\n' && multi {
        probe.glyph = RetainedProbeGlyph { start, end, line: probe.line, x0: probe.pen, x1: probe.pen };
        probe.line += 1;
        probe.pen = 0.0;
        probe.byte = end;
        return RetainedProbeStep::Glyph;
    }
    let limit = width.max(1.0) + RETAINED_TEXT_FIT_EPSILON;
    let face = retained_face(probe.mono);
    let mut kerning = if probe.pen > 0.0 { retained_kerning(atlas, face, value, start, size) } else { 0.0 };
    if multi && probe.pen > 0.0 && semio_ui_text::is_break_opportunity(value, start) {
        let run = atlas.measure_range_face(face, value, start, semio_ui_text::unbreakable_run_end(value, start), size);
        if probe.pen + kerning + run > limit {
            probe.line += 1;
            probe.pen = 0.0;
            kerning = 0.0;
        }
    }
    let advance = atlas.ensure_glyph_for(face, ch, size).advance;
    if multi && probe.pen > 0.0 && !semio_ui_text::is_wrap_space(ch) && probe.pen + kerning + advance > limit {
        probe.line += 1;
        probe.pen = 0.0;
        kerning = 0.0;
    }
    let x0 = probe.pen + kerning;
    probe.glyph = RetainedProbeGlyph { start, end, line: probe.line, x0, x1: x0 + advance };
    probe.pen = x0 + advance;
    probe.byte = end;
    RetainedProbeStep::Glyph
}

/// 🖌️ Generation-qualified cursor for one retained node's byte, glyph, line, and chrome output.
pub struct RetainedNodePaintCursor {
    pub(super) node: Option<NodeId>,
    pub glyph: RetainedGlyphCursor,
    pub(super) measure_byte: usize,
    pub(super) measure_width: f32,
    pub(super) caret_width: f32,
    pub(super) probe: RetainedWrapProbe,
    pub(super) edit_caret: (usize, f32),
    pub(super) origin_x: f32,
    pub(super) origin_y: f32,
    pub(super) chrome: bool,
    pub(super) phase: u16,
    pub(super) item: usize,
    pub(super) section: usize,
    pub(super) depth: usize,
    pub(super) path: [usize; RETAINED_TREE_DEPTH],
    pub(super) row_y: f32,
    pub(super) visited: usize,
    pub(super) ascending: bool,
    pub(super) selected: Option<usize>,
    pub popup_route: bool,
}

impl Default for RetainedNodePaintCursor {
    fn default() -> Self {
        Self {
            node: None,
            glyph: RetainedGlyphCursor::default(),
            measure_byte: 0,
            measure_width: 0.0,
            caret_width: 0.0,
            probe: RetainedWrapProbe::default(),
            edit_caret: (0, 0.0),
            origin_x: 0.0,
            origin_y: 0.0,
            chrome: false,
            phase: 0,
            item: 0,
            section: 0,
            depth: 0,
            path: [0; RETAINED_TREE_DEPTH],
            row_y: 0.0,
            visited: 0,
            ascending: false,
            selected: None,
            popup_route: false,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RetainedNodePaintStep {
    Pending,
    Complete,
    Fault,
}

impl RetainedNodePaintCursor {
    pub fn is_active(&self) -> bool {
        self.node.is_some()
    }

    pub fn census(&self) -> String {
        format!(
            "node={:?} phase={} section={} item={} depth={} measure-byte={} glyph-byte={} glyph-line={} visited={} chrome={} popup={}",
            self.node, self.phase, self.section, self.item, self.depth, self.measure_byte, self.glyph.byte, self.glyph.line, self.visited, self.chrome, self.popup_route
        )
    }

    #[cfg(any(test, feature = "testkit"))]
    pub fn terminal_is_empty(&self) -> bool {
        self.node.is_none()
    }

    pub fn close_step(&mut self) -> bool {
        if self.node.take().is_some() {
            self.reset_progress();
            return false;
        }
        self.glyph.close_step()
    }

    pub(super) fn begin(&mut self, node: NodeId, origin_x: f32, origin_y: f32) {
        self.node = Some(node);
        self.origin_x = origin_x;
        self.origin_y = origin_y;
        self.reset_progress();
    }

    pub(super) fn finish(&mut self) -> RetainedNodePaintStep {
        self.node = None;
        self.reset_progress();
        RetainedNodePaintStep::Complete
    }

    pub fn cancel_draw_route(&mut self, draw: &mut DrawList) {
        let _ = retained_select_close_route(draw, self);
    }

    fn reset_progress(&mut self) {
        self.glyph.reset();
        self.measure_byte = 0;
        self.measure_width = 0.0;
        self.caret_width = 0.0;
        self.probe = RetainedWrapProbe::default();
        self.edit_caret = (0, 0.0);
        self.chrome = false;
        self.phase = 0;
        self.item = 0;
        self.section = 0;
        self.depth = 0;
        self.path = [0; RETAINED_TREE_DEPTH];
        self.row_y = 0.0;
        self.visited = 0;
        self.ascending = false;
        self.selected = None;
        self.popup_route = false;
    }

    pub(super) fn advance(&mut self, phase: u16) {
        self.phase = phase;
        self.glyph.reset();
        self.measure_byte = 0;
        self.measure_width = 0.0;
        self.caret_width = 0.0;
        self.probe = RetainedWrapProbe::default();
        self.chrome = false;
        self.item = 0;
    }

    /// 🧮️ Moves to the next part of the SAME phase — a table's next column label or cell: the glyph and measure state restart,
    /// the part index advances.
    pub(super) fn next_part(&mut self) {
        self.glyph.reset();
        self.measure_byte = 0;
        self.measure_width = 0.0;
        self.item += 1;
    }
}
// #endregion paint
