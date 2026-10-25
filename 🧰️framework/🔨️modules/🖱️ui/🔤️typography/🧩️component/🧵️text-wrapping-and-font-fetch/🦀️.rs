//! 🧵️ Text wrapping and font fetch: the pre-wrapped and wrapped lines, the wrapped measurement and the font byte fetch. See [`super`].

use super::*;

// #region text
impl FontAtlas {
    /// ↩️ Returns CSS `white-space: pre-wrap` line ranges: every space is kept, and the spaces ending a line HANG (CSS Text 3
    /// §4.1.3) — they never push the line over its box, so the break falls after them and only a word opens the next line.
    pub fn pre_wrap_lines(&mut self, face: TextFace, text: &str, max_width: f32, size: f32) -> Vec<std::ops::Range<usize>> {
        let limit = max_width.max(1.0);
        let mut lines = Vec::new();
        let mut byte = 0usize;
        loop {
            let line_start = byte;
            let mut pen = 0.0f32;
            let mut last_break = None;
            let mut previous = None;
            let mut completed = false;
            while let Some(ch) = text.get(byte..).and_then(|rest| rest.chars().next()) {
                let next = byte + ch.len_utf8();
                if ch == '\n' {
                    lines.push(line_start..byte);
                    byte = next;
                    completed = true;
                    break;
                }
                let kerning = self.pen_kerning(face, previous, pen, ch, size);
                let advance = self.ensure_glyph_for(face, ch, size).advance;
                if !is_wrap_space(ch) && pen + kerning + advance > limit + LINE_BREAK_FIT_EPSILON && byte > line_start {
                    if let Some(split) = last_break.filter(|split| *split > line_start) {
                        lines.push(line_start..split);
                        byte = split;
                        completed = true;
                        break;
                    }
                }
                pen += kerning;
                pen += advance;
                previous = Some(ch);
                byte = next;
                if is_wrap_space(ch) {
                    last_break = Some(next);
                }
            }
            if !completed {
                lines.push(line_start..text.len());
                break;
            }
            if byte == text.len() {
                lines.push(byte..byte);
                break;
            }
        }
        lines
    }

    /// ✂️ CSS greedy line breaking over `text` at `max_width`, as BYTE RANGES into `text` — the one
    /// wrap this target has. A line ends at a [`may_break_between`] opportunity, never inside a word,
    /// and a hard `\n` always ends one. Trailing spaces HANG: they join the line they end and never
    /// push it over the box, exactly as `white-space: normal` does.
    ///
    /// 🩸️ The single last-resort arm is CSS's `overflow-wrap` fallback: once a line is empty and ONE
    /// unbreakable run is still wider than the whole box, the run breaks at the glyph that overflows,
    /// because the alternative is a word painted outside its own card.
    pub fn wrap_lines(&mut self, text: &str, max_width: f32, size: f32) -> Vec<std::ops::Range<usize>> {
        self.wrap_lines_face(TextFace::Sans, text, max_width, size)
    }

    /// 🌊️ Applies the shared greedy wrap using the selected face's glyph advances.
    pub fn wrap_lines_face(&mut self, face: TextFace, text: &str, max_width: f32, size: f32) -> Vec<std::ops::Range<usize>> {
        let limit = max_width.max(1.0);
        let mut lines = Vec::new();
        let (mut line_start, mut pen, mut byte, mut previous) = (0usize, 0.0f32, 0usize, None);
        while let Some(ch) = text.get(byte..).and_then(|rest| rest.chars().next()) {
            let next = byte + ch.len_utf8();
            if ch == '\n' {
                lines.push(line_start..byte);
                (line_start, pen, byte, previous) = (next, 0.0, next, None);
                continue;
            }
            let mut kerning = self.pen_kerning(face, previous, pen, ch, size);
            if pen > 0.0 && is_break_opportunity(text, byte) {
                let run = self.measure_range_face(face, text, byte, unbreakable_run_end(text, byte), size);
                if pen + kerning + run > limit + LINE_BREAK_FIT_EPSILON {
                    lines.push(line_start..byte);
                    (line_start, pen, kerning) = (byte, 0.0, 0.0);
                }
            }
            let advance = self.ensure_glyph_for(face, ch, size).advance;
            if pen > 0.0 && !is_wrap_space(ch) && pen + kerning + advance > limit + LINE_BREAK_FIT_EPSILON {
                lines.push(line_start..byte);
                (line_start, pen, kerning) = (byte, 0.0, 0.0);
            }
            pen += kerning;
            pen += advance;
            previous = Some(ch);
            byte = next;
        }
        lines.push(line_start..text.len());
        lines
    }

    /// 📏️ Wrapped bounding box at `max_width` — [`Self::wrap_lines`]'s own lines, each priced without
    /// its hanging trailing spaces, by `line_height(size)` per line.
    pub fn measure_text_wrapped(&mut self, text: &str, max_width: f32, size: f32) -> (f32, f32) {
        let lines = self.wrap_lines(text, max_width, size);
        let height = lines.len().max(1) as f32 * line_height(size);
        let mut width = 0.0f32;
        for line in &lines {
            let trimmed = text[line.clone()].trim_end_matches(is_wrap_space);
            width = width.max(self.measure_text(trimmed, size).0);
        }
        (width.min(max_width), height)
    }
}

pub async fn fetch_font_bytes(url: &str) -> Result<Vec<u8>, String> {
    #[cfg(all(target_arch = "wasm32", not(target_env = "p2")))]
    {
        use js_sys::Uint8Array;
        use semio_framework_async::browser::JsFuture;
        use wasm_bindgen::JsCast;
        use web_sys::{Request, RequestInit, RequestMode, Response};

        let opts = RequestInit::new();
        opts.set_method("GET");
        opts.set_mode(RequestMode::Cors);
        let request = Request::new_with_str_and_init(url, &opts).map_err(|_| "request failed")?;
        let window = web_sys::window().ok_or("no window")?;
        let resp_value = JsFuture::from(window.fetch_with_request(&request)).await.map_err(|_| "fetch failed")?;
        let resp: Response = resp_value.dyn_into().map_err(|_| "response cast failed")?;
        if !resp.ok() {
            return Ok(Vec::new());
        }
        let buffer = JsFuture::from(resp.array_buffer().map_err(|_| "array_buffer failed")?).await.map_err(|_| "buffer failed")?;
        let array = Uint8Array::new(&buffer);
        let mut bytes = vec![0u8; array.length() as usize];
        array.copy_to(&mut bytes);
        Ok(bytes)
    }
    #[cfg(not(all(target_arch = "wasm32", not(target_env = "p2"))))]
    {
        let _ = url;
        Ok(Vec::new())
    }
}
// #endregion text
