//! 🖼️ Glyph rasterization: the glyph ensure, the swash rasterization, the symbol, single char and resolved glyph rendering and the bitmap glyphs. See [`super`].

use super::*;

// #region text
impl FontAtlas {
    /// 🔍️ Fetches (rasterizing on first use) the glyph for `ch` at LOGICAL `size_px`, keyed by
    /// `(char, device_size_px)` so the same character rasterized at two different sizes never
    /// returns the wrong bitmap (the pre-fix bug: a `char`-only key meant later sizes reused the
    /// first size's rasterization, blurring text at any size other than whichever was cached
    /// first). The key is the DEVICE size — `size_px * raster_scale` — so 16 logical px at 2× and
    /// 32 logical px at 1× stay distinct cache rows even though both rasterise 32 device px.
    pub fn ensure_glyph(&mut self, ch: char, size_px: f32) -> &GlyphEntry {
        self.ensure_glyph_for(TextFace::Sans, ch, size_px)
    }

    /// 🔤️ Resolves and caches one glyph under its authored face and device size.
    /// The raster comes from the quarter-pixel grid; the answered entry's `advance` is the face's advance at the EXACT
    /// `size_px` (`advance_em × size_px`) — the one width measure, wrap and paint all pen by.
    pub fn ensure_glyph_for(&mut self, face: TextFace, ch: char, size_px: f32) -> &GlyphEntry {
        let device = size_px * self.raster_scale;
        let exact = (face, ch, device.to_bits());
        if !self.sized.contains_key(&exact) {
            let key = (face, ch, Self::quantize_size(device));
            if !self.glyphs.contains_key(&key) {
                self.rasterize_glyph(key);
            }
            let mut glyph = *self.glyphs.get(&key).expect("glyph inserted");
            glyph.advance = glyph.advance_em * size_px;
            if self.sized.len() >= SIZED_GLYPH_CAPACITY {
                self.sized.clear();
            }
            self.sized.insert(exact, glyph);
        }
        self.sized.get(&exact).expect("sized glyph inserted")
    }

    /// 🪜️ Resolution order, and the ONE place it is written down: a default-ignorable format control
    /// is blank and zero-advance; a [`SYMBOL_FACE`] codepoint is drawn from this crate's own outlines
    /// (no shipped face carries one, in EITHER mode, on native or wasm); everything else goes to the
    /// atlas mode's own pipeline.
    fn rasterize_glyph(&mut self, key: (TextFace, char, u32)) {
        let (face, ch, device_size_px) = key;
        if is_zero_width_format_char(ch) {
            let glyph = self.blank_glyph();
            self.pack_glyph(key, glyph);
            return;
        }
        let raster_px = Self::grid_px(device_size_px);
        if let Some(glyph) = symbol_face_path(ch).and_then(|path| self.rasterize_symbol_glyph(path, raster_px)) {
            self.pack_glyph(key, glyph);
            return;
        }
        let glyph = match self.mode {
            AtlasMode::Bitmap => self.rasterize_bitmap_glyph(ch, raster_px),
            AtlasMode::Shaped => self.rasterize_shaped_glyph(face, ch, raster_px),
        };
        self.pack_glyph(key, glyph);
    }

    /// 🫥 A glyph that occupies no atlas texels and moves the pen by nothing.
    fn blank_glyph(&self) -> RasterizedGlyph {
        RasterizedGlyph { bitmap: Vec::new(), width: 0, height: 0, bearing_x: 0.0, bearing_y: 0.0, advance: 0.0, raster_scale: self.raster_scale, is_color: false }
    }

    /// 🔣️ Rasterizes one [`SYMBOL_FACE`] outline at DEVICE `size_px` through `zeno` — swash's own
    /// path rasterizer, already in this target's graph, so owning these glyphs adds no dependency.
    /// `zeno`'s `Placement::top` is the bitmap's top edge in the path's y-DOWN space (negative above
    /// the baseline), which is the sign-flipped twin of swash's; both land in `GlyphEntry` as the
    /// baseline-to-bitmap-bottom bearing every paint call site reads.
    fn rasterize_symbol_glyph(&self, path: &str, size_px: f32) -> Option<RasterizedGlyph> {
        let scale = size_px / SYMBOL_EM_UNITS;
        if !scale.is_finite() || scale <= 0.0 {
            return None;
        }
        let mut stroke = ZenoStroke::new(SYMBOL_STROKE_UNITS);
        stroke.join(ZenoJoin::Round);
        stroke.cap(ZenoCap::Round);
        let (bitmap, placement) = ZenoMask::new(path).style(ZenoStyle::Stroke(stroke)).transform(Some(ZenoTransform::scale(scale, scale))).format(SwashFormat::Alpha).render();
        if placement.width == 0 || placement.height == 0 {
            return None;
        }
        let raster_scale = self.raster_scale;
        let bearing = (SYMBOL_SIDE_BEARING_UNITS * scale).round().max(1.0);
        Some(RasterizedGlyph {
            bitmap,
            width: placement.width,
            height: placement.height,
            bearing_x: bearing / raster_scale,
            bearing_y: -(placement.top as f32 + placement.height as f32) / raster_scale,
            advance: (placement.width as f32 + bearing * 2.0) / raster_scale,
            raster_scale,
            is_color: false,
        })
    }

    /// 🧵️ Resolves `ch` to a font + glyph id by running a single-codepoint `parley::Layout`. This
    /// is what performs family resolution and (via parley's built-in emoji-cluster detection)
    /// automatic fallback into the registered `GenericFamily::Emoji` family. Returns `None` when
    /// no registered font (including the emoji fallback) could shape the codepoint at all.
    fn shape_single_char(&mut self, face: TextFace, ch: char, size_px: f32) -> Option<ResolvedGlyph> {
        let text = ch.to_string();
        let mut builder = self.layout_cx.ranged_builder(&mut self.font_cx, &text, 1.0, true);
        builder.push_default(StyleProperty::FontStack(FontStack::Source(Cow::Borrowed(if face==TextFace::Emoji{EMOJI_BUCKET_STACK}else{face.family()}))));
        builder.push_default(StyleProperty::FontSize(size_px));
        let mut layout: parley::Layout<[u8; 4]> = builder.build(&text);
        layout.break_all_lines(None);
        for line in layout.lines() {
            for item in line.items() {
                let PositionedLayoutItem::GlyphRun(run) = item else { continue };
                let Some(glyph) = run.glyphs().next() else { continue };
                let font = run.run().font();
                return Some(ResolvedGlyph { glyph_id: glyph.id, advance: glyph.advance, font_data: font.data.clone(), font_index: font.index });
            }
        }
        None
    }

    /// 🖌️ Rasterizes a resolved glyph via swash, preferring color bitmap/outline sources (COLR,
    /// embedded color bitmaps — what carries Noto Emoji's color) over the plain scalable outline,
    /// so any glyph the resolved font can render in color comes back as `Content::Color` RGBA and
    /// everything else comes back as an 8-bit alpha mask.
    fn render_resolved(&mut self, resolved: &ResolvedGlyph, size_px: f32) -> Option<RasterizedGlyph> {
        let data = resolved.font_data.data();
        let font_ref = SwashFontRef::from_index(data, resolved.font_index as usize)?;
        let mut scaler = self.scale_cx.builder(font_ref).size(size_px).hint(true).build();
        let image = Render::new(&[Source::ColorBitmap(StrikeWith::BestFit), Source::ColorOutline(0), Source::Outline]).format(SwashFormat::Alpha).render(&mut scaler, resolved.glyph_id)?;
        let is_color = matches!(image.content, SwashContent::Color);
        let raster_scale = self.raster_scale;
        Some(RasterizedGlyph {
            bitmap: image.data,
            width: image.placement.width,
            height: image.placement.height,
            bearing_x: image.placement.left as f32 / raster_scale,
            bearing_y: (image.placement.top - image.placement.height as i32) as f32 / raster_scale,
            advance: resolved.advance / raster_scale,
            raster_scale,
            is_color,
        })
    }

    /// 🪶️ `shape_single_char` + `render_resolved`, falling back to the plain ASCII bitmap glyph
    /// (see `rasterize_bitmap_glyph`) when no registered font — including the emoji fallback —
    /// can shape `ch` at all (e.g. scripts none of Anta/Kelly Slab/Share Tech Mono/Noto Emoji
    /// cover, such as CJK or Arabic; a pre-existing limitation this atlas doesn't newly regress).
    fn rasterize_shaped_glyph(&mut self, face: TextFace, ch: char, size_px: f32) -> RasterizedGlyph {
        let Some(resolved) = self.shape_single_char(face, ch, size_px) else {
            return self.rasterize_bitmap_glyph(ch, size_px);
        };
        if let Some(glyph) = self.render_resolved(&resolved, size_px) {
            return glyph;
        }
        RasterizedGlyph { bitmap: Vec::new(), width: 0, height: 0, bearing_x: 0.0, bearing_y: 0.0, advance: resolved.advance / self.raster_scale, raster_scale: self.raster_scale, is_color: false }
    }

    fn rasterize_bitmap_glyph(&self, ch: char, device_size_px: f32) -> RasterizedGlyph {
        let index = ch as u32;
        let glyph_index = if (32..127).contains(&index) { (index - 32) as usize } else { 0 };
        let pattern = &BITMAP_FONT[glyph_index.min(BITMAP_FONT.len() - 1)];
        let mut bitmap = vec![0u8; (BITMAP_GLYPH_W * BITMAP_GLYPH_H) as usize];
        for (row, row_bits) in pattern.iter().enumerate() {
            for col in 0..BITMAP_GLYPH_W {
                if (row_bits >> (7 - col)) & 1 == 1 {
                    bitmap[row * BITMAP_GLYPH_W as usize + col as usize] = 255;
                }
            }
        }
        RasterizedGlyph { bitmap, width: BITMAP_GLYPH_W, height: BITMAP_GLYPH_H, bearing_x: 0.0, bearing_y: 0.0, advance: device_size_px / self.raster_scale * 0.625, raster_scale: 1.0, is_color: false }
    }

}
// #endregion text
