//! 📏️ Metrics of one standard-14 font and the descriptor values derived from them.

/// 📏 Metrics of one standard-14 font (AFM units, 1/1000 em).
pub struct StandardFontMetrics {
    pub name: &'static str,
    pub family: &'static str,
    pub bbox: [f64; 4],
    pub ascender: f64,
    pub descender: f64,
    pub cap_height: f64,
    pub x_height: f64,
    pub italic_angle: f64,
    pub stem_v: f64,
    pub fixed_pitch: bool,
    pub bold: bool,
    pub symbolic: bool,
    /// Glyph name → advance width, sorted by name for binary search.
    pub widths: &'static [(&'static str, u16)],
    /// Built-in encoding: character code → glyph name (Symbol/ZapfDingbats only carry their own).
    pub builtin_encoding: &'static [(u8, &'static str)],
}

impl StandardFontMetrics {
    /// 📏 Advance width of a glyph name (AFM units).
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn width(&self, glyph: &str) -> Option<f64> {
        self.widths.binary_search_by(|(name, _)| (*name).cmp(glyph)).ok().map(|index| self.widths[index].1 as f64)
    }
    /// 🏳️ `/Flags` of a font descriptor for this face (§9.8.2).
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn flags(&self) -> u32 {
        let mut flags = 0u32;
        if self.fixed_pitch {
            flags |= 1;
        }
        if self.family.starts_with("Times") {
            flags |= 2;
        }
        flags |= if self.symbolic { 4 } else { 32 };
        if self.italic_angle != 0.0 {
            flags |= 64;
        }
        if self.bold {
            flags |= 1 << 18;
        }
        flags
    }
}
