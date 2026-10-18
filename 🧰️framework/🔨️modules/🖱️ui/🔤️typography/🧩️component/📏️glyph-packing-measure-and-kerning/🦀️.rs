//! 📏️ Glyph packing, measure and kerning: the atlas packing, the text and range measurement, the kerning lookups and the pen advance. See [`super`].

use super::*;

// #region text
impl FontAtlas {
    /// 📐️ Bin-packs one rasterized glyph into the alpha (`pixels`) or color (`color_pixels`)
    /// atlas page, per `RasterizedGlyph::is_color`, and records the resulting `GlyphEntry`.
    pub(super) fn pack_glyph(&mut self, key: (TextFace, char, u32), glyph: RasterizedGlyph) {
        let RasterizedGlyph { bitmap, width, height, bearing_x, bearing_y, advance, raster_scale, is_color } = glyph;
        let (atlas_x, atlas_y) = if is_color {
            if self.color_cursor_x + width + 2 >= self.color_width {
                self.color_cursor_x = 1;
                self.color_cursor_y += self.color_row_height + 2;
                self.color_row_height = 0;
            }
            let x = self.color_cursor_x;
            let y = self.color_cursor_y;
            for row in 0..height {
                let dst = (((y + row) * self.color_width + x) * 4) as usize;
                let src = (row * width * 4) as usize;
                if !bitmap.is_empty() && width > 0 {
                    self.color_pixels[dst..dst + (width * 4) as usize].copy_from_slice(&bitmap[src..src + (width * 4) as usize]);
                }
            }
            self.color_cursor_x += width + 2;
            self.color_row_height = self.color_row_height.max(height);
            self.color_dirty = true;
            (x, y)
        } else {
            if self.cursor_x + width + 2 >= self.width {
                self.cursor_x = 1;
                self.cursor_y += self.row_height + 2;
                self.row_height = 0;
            }
            let x = self.cursor_x;
            let y = self.cursor_y;
            for row in 0..height {
                let dst = ((y + row) * self.width + x) as usize;
                let src = (row * width) as usize;
                if !bitmap.is_empty() && width > 0 {
                    self.pixels[dst..dst + width as usize].copy_from_slice(&bitmap[src..src + width as usize]);
                }
            }
            self.cursor_x += width + 2;
            self.row_height = self.row_height.max(height);
            self.dirty = true;
            (x, y)
        };
        let logical_size = Self::grid_px(key.2) / self.raster_scale;
        let advance_em = if logical_size > 0.0 { advance / logical_size } else { 0.0 };
        self.glyphs.insert(key, GlyphEntry { atlas_x, atlas_y, width, height, advance, advance_em, bearing_x, bearing_y, raster_scale, is_color });
    }

    /// 📏️ Advance-and-kerning-summed width and the CSS line box height (`line_height`) — a single line of text
    /// occupies its whole line box in React, exactly as a wrapped paragraph's first line does.
    pub fn measure_text(&mut self, text: &str, size: f32) -> (f32, f32) {
        self.measure_text_face(TextFace::Sans, text, size)
    }

    /// 📏️ Measures one run with the same face-specific advances and pair kerning its painter pens.
    pub fn measure_text_face(&mut self, face: TextFace, text: &str, size: f32) -> (f32, f32) {
        let mut width = 0.0f32;
        let mut max_height = 0.0f32;
        let mut previous = None;
        for ch in text.chars() {
            width += self.pen_kerning(face, previous, width, ch, size);
            previous = Some(ch);
            let glyph = self.ensure_glyph_for(face, ch, size);
            width += glyph.advance;
            max_height = max_height.max(glyph.logical_height() + glyph.bearing_y);
        }
        (width, max_height.max(line_height(size)))
    }

    /// 📏️ The advance sum of one run of `text`, from `byte` up to `end` — the width a greedy wrap
    /// prices an unbreakable run at, off the very advances the painter then pens.
    pub fn measure_range(&mut self, text: &str, byte: usize, end: usize, size: f32) -> f32 {
        self.measure_range_face(TextFace::Sans, text, byte, end, size)
    }

    /// 📏️ Measures a byte range with one face-specific glyph stream, kerned within the range only — the range's own width.
    pub fn measure_range_face(&mut self, face: TextFace, text: &str, byte: usize, end: usize, size: f32) -> f32 {
        let Some(run) = text.get(byte..end) else { return 0.0 };
        self.measure_text_face(face, run, size).0
    }

    /// 🤝️ The pair kerning between `left` and `right` in `face` at `size` — [`PairKerning::em`] cached per face and pair. The
    /// fixed-pitch bitmap fallback never kerns.
    pub fn kerning_for(&mut self, face: TextFace, left: char, right: char, size: f32) -> f32 {
        if !matches!(self.mode, AtlasMode::Shaped) {
            return 0.0;
        }
        let key = (face, left, right);
        let em = match self.kerning.get(&key).copied() {
            Some(em) => em,
            None => {
                let em = self.pair_kerning.em(face, left, right);
                if self.kerning.len() >= KERNING_PAIR_CAPACITY {
                    self.kerning.clear();
                }
                self.kerning.insert(key, em);
                em
            }
        };
        em * size
    }

    /// 🤝️ The kerning a line pens before `ch`: its pair with the `previous` scalar once the pen has left the line start, nothing
    /// at a line start — the ONE rule measure, wrap, caret and paint all apply.
    pub fn pen_kerning(&mut self, face: TextFace, previous: Option<char>, pen: f32, ch: char, size: f32) -> f32 {
        match previous {
            Some(left) if pen > 0.0 => self.kerning_for(face, left, ch, size),
            _ => 0.0,
        }
    }

    /// 🤝️ The pair kerning at `byte` of `text`: between the scalar ending there and the scalar starting there, nothing at either
    /// end of the text.
    pub fn kerning_at(&mut self, text: &str, byte: usize, size: f32) -> f32 {
        let (Some(before), Some(after)) = (text.get(..byte), text.get(byte..)) else { return 0.0 };
        match (before.chars().next_back(), after.chars().next()) {
            (Some(left), Some(right)) => self.kerning_for(TextFace::Sans, left, right, size),
            _ => 0.0,
        }
    }

    /// ✒️ How far the scalar at `byte` moves a line's pen: its advance plus its pair kerning with the scalar after it — summed over
    /// a run, exactly [`Self::measure_text`]; summed up to a caret, exactly [`Self::pen_at`].
    pub fn pen_advance(&mut self, text: &str, byte: usize, size: f32) -> f32 {
        let Some(ch) = text.get(byte..).and_then(|rest| rest.chars().next()) else { return 0.0 };
        let advance = self.ensure_glyph(ch, size).advance;
        advance + self.kerning_at(text, byte + ch.len_utf8(), size)
    }

    /// 📍️ Where the scalar at `byte` is penned on a single line of `text` — the caret's x: the run before it plus that run's pair
    /// kerning with it, where Chromium places a caret after a kerned pair.
    pub fn pen_at(&mut self, text: &str, byte: usize, size: f32) -> f32 {
        let pen = self.measure_range(text, 0, byte, size);
        pen + if pen > 0.0 { self.kerning_at(text, byte, size) } else { 0.0 }
    }

}
// #endregion text
