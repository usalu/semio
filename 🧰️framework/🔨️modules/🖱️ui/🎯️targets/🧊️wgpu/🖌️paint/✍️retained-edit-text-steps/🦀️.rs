//! ✍️ Retained edit text steps: the live view of an edit buffer (preedit spliced in), the text rectangle of a field, and the probe, highlight, text and caret steps a text input paints. See [`super`].

use super::*;
use crate::retained_glyph_and_node_cursors::RetainedWrapProbe;
use std::borrow::Cow;
use semio_ui_retained_tree::tree::{EditTextLayout, EditTextStop};

const EDIT_LAYOUT_MAX_BYTES: usize = 64 * 1024;

/// 🪟️ What a field shows: its text with any IME preedit spliced in at the caret, the caret, the selection and the preedit as byte ranges of that text.
#[derive(Debug)]
pub(super) struct EditView<'a> {
    pub(super) display: Cow<'a, str>,
    pub(super) caret: usize,
    pub(super) selection: Option<(usize, usize)>,
    pub(super) preedit: Option<(usize, usize)>,
    pub(super) editing: bool,
}

/// 🔭️ The view of `edit` (or of `fallback`, the published text, while no buffer is live); mid-composition the preedit replaces the selection and the caret sits at the input method's own cursor inside it.
pub(super) fn edit_view<'a>(edit: Option<&'a EditState>, composition_cursor: Option<usize>, fallback: &'a str) -> EditView<'a> {
    let Some(edit) = edit else { return EditView { display: Cow::Borrowed(fallback), caret: 0, selection: None, preedit: None, editing: false } };
    let sound = edit.caret <= edit.text.len() && edit.text.is_char_boundary(edit.caret) && edit.anchor <= edit.text.len() && edit.text.is_char_boundary(edit.anchor);
    let composition = edit.composition.as_deref().filter(|composition| !composition.is_empty() && sound);
    if let Some(composition) = composition {
        let mut at = composition_cursor.unwrap_or(composition.len()).min(composition.len());
        while !composition.is_char_boundary(at) {
            at -= 1;
        }
        let display = format!("{}{composition}{}", &edit.text[..edit.caret], &edit.text[edit.caret..]);
        return EditView { display: Cow::Owned(display), caret: edit.caret + at, selection: None, preedit: Some((edit.caret, edit.caret + composition.len())), editing: true };
    }
    let (start, end) = (edit.anchor.min(edit.caret), edit.anchor.max(edit.caret));
    EditView { display: Cow::Borrowed(if edit.text.is_empty() { fallback } else { edit.text.as_str() }), caret: if sound { edit.caret } else { 0 }, selection: (sound && start != end).then_some((start, end)), preedit: None, editing: true }
}

/// 📐️ Where a field's text lives: its rectangle, the font size and whether it wraps over several lines.
#[derive(Clone, Copy, Debug)]
pub(super) struct EditGeometry {
    pub(super) inner: Rect,
    pub(super) size: f32,
    pub(super) multi: bool,
    /// 🔠️ React's `font-mono` field (the icon selector's editor): measured, wrapped and painted in the mono face.
    pub(super) mono: bool,
}

impl EditGeometry {
    fn line_height(&self) -> f32 {
        semio_ui_text::line_height(self.size)
    }

    /// 🔝️ The top edge of `line`, the field scrolled by `scroll`: one centred row, or the stacked lines of a textarea.
    fn line_top(&self, line: usize, scroll: (f32, f32)) -> f32 {
        if self.multi {
            self.inner.y - scroll.1 + self.line_height() * line as f32
        } else {
            self.inner.y + (self.inner.h - self.line_height()) * 0.5
        }
    }
}

/// 🔲️ React's `p-single` inset of a text input (a textarea keeps its `py-single` too), centred in the row of a single-line field.
pub(super) fn input_text_rect(bounds: Rect, multi: bool, theme: &Theme) -> Rect {
    let pad = theme.padding_standard;
    let vertical = if multi { pad } else { 0.0 };
    Rect::new(bounds.x + pad, bounds.y + vertical, (bounds.w - pad * 2.0).max(0.0), (bounds.h - vertical * 2.0).max(0.0))
}

/// 🚨️ The row a field with a live refusal message reserves under its box, zero otherwise; layout reserves the very same row.
pub(super) fn input_refusal_row(node: &Node, theme: &Theme) -> f32 {
    if matches!(node.spec.0, UiNode::Input(_)) && node.state.edit.is_some() && node.state.number_refusal.as_deref().is_some_and(|message| !message.is_empty()) {
        semio_ui_wgpu_layout::layout::input_refusal_row_height(theme.font_size_small)
    } else {
        0.0
    }
}

/// 📎️ A `file` input's transparent Choose button sized to its caption, and the room left for the picked name.
pub(super) fn file_button_chrome(frame: Rect, label: &str, theme: &Theme) -> (Rect, Rgba, Rect) {
    let width = (label.chars().count() as f32 * theme.font_size_small * 0.55 + theme.padding_standard * 2.0).min(frame.w);
    (Rect::new(frame.x + 1.0, frame.y + 1.0, width, (frame.h - 2.0).max(0.0)), Rgba::TRANSPARENT, Rect::new(frame.x + width, frame.y, (frame.w - width).max(0.0), frame.h))
}

/// 🧱️ The box of an input without its refusal row, its colour swatch and the text geometry inside it.
pub(super) fn input_geometry(input_node: &semio_ui_declaration::ui::UiInputNode, text: &str, file_label: &str, bounds: Rect, row: f32, theme: &Theme) -> (Rect, Option<(Rect, Rgba, Rect)>, EditGeometry) {
    let frame = Rect::new(bounds.x, bounds.y, bounds.w, (bounds.h - row).max(0.0));
    let multi = input_node.input_kind == "longText";
    let swatch = matches!(input_node.input_kind.as_str(), "color" | "colorSwatch").then(|| color_input_swatch(frame, text)).flatten().or_else(|| (input_node.input_kind == "file").then(|| file_button_chrome(frame, file_label, theme)));
    let text_area = swatch.map_or(frame, |(_, _, text_bounds)| text_bounds);
    (frame, swatch, EditGeometry { inner: input_text_rect(text_area, multi, theme), size: theme.font_size_body, multi, mono: input_node.uses_mono_face() })
}

/// 🚨️ Asks for a layout pass for every input whose refusal row appeared or vanished since its size was last laid out.
pub fn sync_input_refusal_rows(tree: &mut UiTree, root: NodeId) {
    let mut pending = vec![root];
    let mut changes = Vec::new();
    while let Some(id) = pending.pop() {
        pending.extend(tree.children(id));
        let Some(node) = tree.node(id) else { continue };
        let wants = matches!(node.spec.0, UiNode::Input(_)) && node.state.edit.is_some() && node.state.number_refusal.as_deref().is_some_and(|message| !message.is_empty());
        if matches!(node.spec.0, UiNode::Input(_)) && node.state.refusal_row_laid != wants {
            changes.push((id, wants));
        }
    }
    for (id, wants) in changes {
        if let Some(node) = tree.node_mut(id) {
            node.state.refusal_row_laid = wants;
        }
        tree.request_layout(id);
    }
}

/// 📐️ Lays out the text of the focused or hovered input, and of every textarea, once per change and parks every caret position on its state, so pointer, wheel and vertical motion routing need no font; keeps the caret in view and asks the layout for the height a textarea's wrapped lines need (React's `field-sizing-content`).
pub fn refresh_input_text_layout(tree: &mut UiTree, id: NodeId, theme: &Theme, atlas: &mut FontAtlas) {
    let Some(node) = tree.node(id) else { return };
    let UiNode::Input(input) = &node.spec.0 else { return };
    let Some(accepted) = tree.accepted_layout(id) else { return };
    let editing = node.state.edit.is_some();
    let text = match node.state.edit.as_ref() {
        Some(edit) => edit.text.as_str(),
        None if node.flags.contains(NodeFlags::HOVERED) || input.input_kind == "longText" => input.value.as_str(),
        None => return,
    };
    if text.len() > EDIT_LAYOUT_MAX_BYTES {
        return;
    }
    let (_, _, geometry) = input_geometry(input, text, node.state.file_chrome.map_or("", |chrome| chrome.0), Rect::new(0.0, 0.0, accepted.width, accepted.height), input_refusal_row(node, theme), theme);
    let inner = [geometry.inner.x, geometry.inner.y, geometry.inner.w, geometry.inner.h];
    let stale = node.state.edit_layout.as_ref().is_none_or(|laid| laid.text != text || laid.inner != inner || laid.multi != geometry.multi || laid.line_height != geometry.line_height());
    let mut laid = if stale {
        let mut probe = RetainedWrapProbe { mono: geometry.mono, ..Default::default() };
        let mut stops = Vec::new();
        loop {
            match retained_wrap_probe_step(text, geometry.inner.w, geometry.size, geometry.multi, atlas, &mut probe) {
                RetainedProbeStep::Glyph => stops.push(EditTextStop { byte: probe.glyph.start, line: probe.glyph.line, x: probe.glyph.x0 }),
                RetainedProbeStep::Done => {
                    stops.push(EditTextStop { byte: text.len(), line: probe.line, x: probe.pen });
                    break;
                }
                RetainedProbeStep::Fault => return,
            }
        }
        EditTextLayout { text: text.to_string(), inner, line_height: geometry.line_height(), multi: geometry.multi, lines: probe.line + 1, stops, revealed_caret: None }
    } else {
        let Some(laid) = node.state.edit_layout.clone() else { return };
        laid
    };
    let caret = node.state.edit.as_ref().map(|edit| edit.caret);
    let before = node.state.scroll_offset;
    let current = if laid.multi { before.1 } else { before.0 };
    let wanted = match caret.filter(|caret| editing && laid.revealed_caret != Some(*caret)) {
        Some(caret) => {
            laid.revealed_caret = Some(caret);
            if laid.multi { laid.reveal(caret, (0.0, current)).1 } else { laid.reveal(caret, (current, 0.0)).0 }
        }
        None => current.clamp(0.0, if laid.multi { laid.max_scroll().1 } else { laid.max_scroll().0 }),
    };
    let after = if laid.multi { (before.0, wanted) } else { (wanted, before.1) };
    let grown = if geometry.multi { laid.lines as f32 * laid.line_height + 2.0 * theme.padding_standard } else { 0.0 };
    let Some(node) = tree.node_mut(id) else { return };
    node.state.edit_layout = Some(laid);
    let regrown = (node.state.textarea_grow - grown).abs() > 0.5;
    if regrown {
        node.state.textarea_grow = grown;
    }
    let scrolled = (editing || geometry.multi) && after != before;
    if scrolled {
        node.state.scroll_offset = after;
    }
    if regrown {
        tree.request_layout(id);
    }
    if scrolled {
        tree.mark_dirty(id, NodeFlags::DIRTY_PAINT);
    }
}

/// 🧭️ Walks the text once to find the caret's line and pen, kept in `cursor.edit_caret`.
pub(super) fn retained_edit_caret_probe_step(view: &EditView, geometry: &EditGeometry, atlas: &mut FontAtlas, cursor: &mut RetainedNodePaintCursor) -> RetainedNodePaintStep {
    cursor.probe.mono = geometry.mono;
    match retained_wrap_probe_step(&view.display, geometry.inner.w, geometry.size, geometry.multi, atlas, &mut cursor.probe) {
        RetainedProbeStep::Glyph => {
            if cursor.item == 0 && cursor.probe.glyph.start == view.caret {
                cursor.edit_caret = (cursor.probe.glyph.line, cursor.probe.glyph.x0);
                cursor.item = 1;
            }
            RetainedNodePaintStep::Pending
        }
        RetainedProbeStep::Done => {
            if cursor.item == 0 {
                cursor.edit_caret = (cursor.probe.line, cursor.probe.pen);
            }
            RetainedNodePaintStep::Complete
        }
        RetainedProbeStep::Fault => RetainedNodePaintStep::Fault,
    }
}

fn span_update(run: &mut RetainedSpanRun, range: Option<(usize, usize)>, glyph: &RetainedProbeGlyph, newline: f32) -> Option<RetainedSpanRun> {
    let inside = range.is_some_and(|(start, end)| glyph.start >= start && glyph.start < end);
    if !inside {
        let done = run.open.then_some(*run);
        run.open = false;
        return done;
    }
    let x1 = glyph.x1 + newline;
    if run.open && run.line == glyph.line {
        run.x1 = x1;
        return None;
    }
    let done = run.open.then_some(*run);
    *run = RetainedSpanRun { open: true, line: glyph.line, x0: glyph.x0, x1 };
    done
}

fn flush_selection(run: RetainedSpanRun, geometry: &EditGeometry, scroll: (f32, f32), theme: &Theme, draw: &mut DrawList) -> bool {
    let rect = [geometry.inner.x - scroll.0 + run.x0, geometry.line_top(run.line, scroll), (run.x1 - run.x0).max(1.0), geometry.line_height()];
    retained_fixed_output(draw, |draw| draw.push_solid(rect, theme.accent)).is_ok()
}

fn flush_preedit(run: RetainedSpanRun, geometry: &EditGeometry, scroll: (f32, f32), theme: &Theme, draw: &mut DrawList) -> bool {
    let thickness = theme.stroke_hairline.max(1.0);
    let rect = [geometry.inner.x - scroll.0 + run.x0, geometry.line_top(run.line, scroll) + geometry.line_height() - thickness, (run.x1 - run.x0).max(1.0), thickness];
    retained_fixed_output(draw, |draw| draw.push_solid(rect, theme.text)).is_ok()
}

/// 🖍️ Paints the selection highlight (React's `::selection`, the accent fill) and the underline of the IME preedit, one scalar per step, beneath the text.
pub(super) fn retained_edit_span_step(view: &EditView, geometry: &EditGeometry, scroll: (f32, f32), theme: &Theme, atlas: &mut FontAtlas, draw: &mut DrawList, cursor: &mut RetainedNodePaintCursor) -> RetainedNodePaintStep {
    cursor.probe.mono = geometry.mono;
    let step = retained_wrap_probe_step(&view.display, geometry.inner.w, geometry.size, geometry.multi, atlas, &mut cursor.probe);
    let (done, glyph) = match step {
        RetainedProbeStep::Fault => return RetainedNodePaintStep::Fault,
        RetainedProbeStep::Glyph => (false, cursor.probe.glyph),
        RetainedProbeStep::Done => (true, RetainedProbeGlyph { start: usize::MAX, ..Default::default() }),
    };
    let newline = if view.display[glyph.start.min(view.display.len())..].starts_with('\n') && !done { geometry.size * 0.35 } else { 0.0 };
    let selected = span_update(&mut cursor.probe.selection, view.selection, &glyph, newline).is_none_or(|run| flush_selection(run, geometry, scroll, theme, draw));
    let preedit = span_update(&mut cursor.probe.preedit, view.preedit, &glyph, 0.0).is_none_or(|run| flush_preedit(run, geometry, scroll, theme, draw));
    if !(selected && preedit) {
        return RetainedNodePaintStep::Fault;
    }
    if done { RetainedNodePaintStep::Complete } else { RetainedNodePaintStep::Pending }
}

/// 🅰️ Paints the field's text one scalar per step; scalars inside the selection take `selected` (React's selection text colour), the rest `color`.
#[allow(clippy::too_many_arguments, reason = "one retained paint context plus the arm payload")]
pub(super) fn retained_edit_text_step(view: &EditView, geometry: &EditGeometry, scroll: (f32, f32), color: Rgba, selected: Rgba, atlas: &mut FontAtlas, draw: &mut DrawList, cursor: &mut RetainedNodePaintCursor) -> RetainedNodePaintStep {
    let byte = cursor.glyph.byte();
    let tint = if view.selection.is_some_and(|(start, end)| byte >= start && byte < end) { selected } else { color };
    let (bounds, flow) = if geometry.multi {
        (Rect::new(geometry.inner.x - scroll.0, geometry.inner.y - scroll.1, geometry.inner.w, geometry.inner.h), RetainedTextFlow::Wrap)
    } else {
        (Rect::new(geometry.inner.x - scroll.0, geometry.inner.y + (geometry.inner.h - geometry.size) * 0.5, geometry.inner.w + scroll.0, geometry.inner.h), RetainedTextFlow::Clip)
    };
    cursor.glyph.mono = geometry.mono;
    match paint_retained_glyph_step_flowed(&view.display, bounds, geometry.size, tint, flow, atlas, draw, &mut cursor.glyph) {
        RetainedGlyphStep::Pending => RetainedNodePaintStep::Pending,
        RetainedGlyphStep::Complete => RetainedNodePaintStep::Complete,
        RetainedGlyphStep::Fault => RetainedNodePaintStep::Fault,
    }
}

/// 🖊️ ▏ Draws the caret at the place the probe found, in the text colour like a native input's caret.
pub(super) fn retained_edit_caret_draw(geometry: &EditGeometry, scroll: (f32, f32), color: Rgba, draw: &mut DrawList, cursor: &RetainedNodePaintCursor) -> RetainedNodePaintStep {
    let (line, pen) = cursor.edit_caret;
    let height = if geometry.multi { geometry.line_height() } else { (geometry.size * 1.2).min(geometry.inner.h) };
    let top = if geometry.multi { geometry.line_top(line, scroll) } else { geometry.inner.y + (geometry.inner.h - height) * 0.5 };
    let x = (geometry.inner.x - scroll.0 + pen).min(geometry.inner.x + geometry.inner.w - 1.0);
    if retained_fixed_output(draw, |draw| draw.push_solid([x, top, 1.0, height], color)).is_err() {
        RetainedNodePaintStep::Fault
    } else {
        RetainedNodePaintStep::Complete
    }
}
