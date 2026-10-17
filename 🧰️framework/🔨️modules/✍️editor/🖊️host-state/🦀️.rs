//! 🖊️ Host state of the editor: the editor host record, its retirement custody with the drop and default implementations. See [`super`].

use super::*;

// #region 🔖️EditorState
pub struct EditorHost {
    pub(super) text: String,
    pub(super) caret: usize,
    pub(super) anchor: usize,
    pub(super) camera: Camera,
    pub(super) viewport: Viewport,
    pub(super) semantic_tokens: Vec<SemanticTokenJson>,
    pub(super) selectable_spans: Vec<SelectableSpanJson>,
    pub(super) diagnostics: Vec<DiagnosticJson>,
    pub(super) placeholders: Vec<PlaceholderJson>,
    pub(super) hover_occurrences: Vec<ByteRangeJson>,
    pub(super) selection_occurrences: Vec<ByteRangeJson>,
    pub(super) extra_carets: Vec<usize>,
    pub(super) font_px: f64,
    pub(super) line_height: f64,
    pub(super) show_line_numbers: bool,
    pub(super) tab_size: usize,
    pub(super) drag_selecting: bool,
    pub(super) hover_token_start: Option<usize>,
    pub(super) hover_token_end: Option<usize>,
    pub(super) theme: EditorCanvasTheme,
    pub(super) caret_visible: bool,
    pub(super) dead_line_y: f64,
    pub(super) chrome_edgeless_scroll: bool,
}

/// ♻️ The bytes of text (or of a scalar list) one close grant releases — a page, never a scalar.
const EDITOR_RETIREMENT_PAGE_BYTES: usize = 64 * 1024;

/// 🧹️ Retained text-editor owner that releases one page of its text or caret list, or one string-owning item, per close grant.
pub struct EditorHostRetirement {
    pub(super) text: String,
    pub(super) semantic_tokens: Vec<SemanticTokenJson>,
    pub(super) selectable_spans: Vec<SelectableSpanJson>,
    pub(super) diagnostics: Vec<DiagnosticJson>,
    pub(super) placeholders: Vec<PlaceholderJson>,
    pub(super) hover_occurrences: Vec<ByteRangeJson>,
    pub(super) selection_occurrences: Vec<ByteRangeJson>,
    pub(super) extra_carets: Vec<usize>,
    released: bool,
}

impl EditorHostRetirement {
    pub fn new(host: EditorHost) -> Self {
        let EditorHost {
            text,
            caret: _,
            anchor: _,
            camera: _,
            viewport: _,
            semantic_tokens,
            selectable_spans,
            diagnostics,
            placeholders,
            hover_occurrences,
            selection_occurrences,
            extra_carets,
            font_px: _,
            line_height: _,
            show_line_numbers: _,
            tab_size: _,
            drag_selecting: _,
            hover_token_start: _,
            hover_token_end: _,
            theme: _,
            caret_visible: _,
            dead_line_y: _,
            chrome_edgeless_scroll: _,
        } = host;
        Self { text, semantic_tokens, selectable_spans, diagnostics, placeholders, hover_occurrences, selection_occurrences, extra_carets, released: false }
    }

    pub fn close_step(&mut self) -> bool {
        if self.released {
            return true;
        }
        if !self.text.is_empty() {
            let mut cut = self.text.len().saturating_sub(EDITOR_RETIREMENT_PAGE_BYTES);
            while !self.text.is_char_boundary(cut) {
                cut += 1;
            }
            self.text.truncate(cut);
            return false;
        }
        if !self.extra_carets.is_empty() {
            let keep = self.extra_carets.len().saturating_sub(EDITOR_RETIREMENT_PAGE_BYTES / size_of::<usize>());
            self.extra_carets.truncate(keep);
            return false;
        }
        if self.semantic_tokens.pop().is_some() || self.selectable_spans.pop().is_some() || self.diagnostics.pop().is_some() || self.placeholders.pop().is_some() || self.hover_occurrences.pop().is_some() || self.selection_occurrences.pop().is_some()
        {
            return false;
        }
        self.released = true;
        true
    }

    pub fn terminal_is_empty(&self) -> bool {
        self.released
            && self.text.is_empty()
            && self.semantic_tokens.is_empty()
            && self.selectable_spans.is_empty()
            && self.diagnostics.is_empty()
            && self.placeholders.is_empty()
            && self.hover_occurrences.is_empty()
            && self.selection_occurrences.is_empty()
            && self.extra_carets.is_empty()
    }
}

impl Drop for EditorHostRetirement {
    fn drop(&mut self) {
        debug_assert!(self.terminal_is_empty(), "EditorHostRetirement must reach terminal-empty before release");
    }
}

impl Default for EditorHost {
    fn default() -> Self {
        Self::new()
    }
}
// #endregion 🔖️EditorState
