//! 🌍️ `compiler_world` — the compiler's vendored font set, replacing `typst-assets`. Libertinus
//! Math/Serif/Mono (OFL 1.1, ~1.3 MB) plus the existing checked-in Noto Color Emoji CBDT subset
//! (~150 KB) — well under `typst-assets`' ~15 MB. License texts ship alongside in `🔤️fonts/`.
//!
//! Scope (Wave 2): only the native `EmbeddedFonts` provider — bytes baked in via `include_bytes!`.
//! The wasm-guest `HostAssetFonts` provider (serving these bytes through the plugin host's
//! `read-asset` seam, mirroring the "guestslim" pipeline this replaces) is real follow-up work,
//! not yet built — landing in the call-site-swap ticket once a wasm guest actually needs it.

//#region 🔖️Fonts
/// @emoji 🔤️ One embedded font's raw bytes plus a stable role — the role is what the layout layer
/// (`compiler_math`) and shaper (`compiler_text`) key off, never a file path or index.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum FontRole {
    /// Libertinus Math — variables, symbols, and everything laid out via the OpenType MATH table.
    Math,
    /// Libertinus Serif Regular — upright text-in-math, numbers, function names.
    Serif,
    /// Libertinus Serif Italic — reserved for future prose/emphasis use; unused by math layout
    /// today (the Math font's own glyphs are already italic for Latin/Greek letters).
    SerifItalic,
    /// Libertinus Mono — reserved for future `Code` snippet rendering.
    Mono,
    /// Noto Color Emoji (CBDT bitmap subset) — `:shortcode:` atoms.
    Emoji,
}

/// @emoji 📦️ The full embedded font set, keyed by [`FontRole`].
#[derive(Clone, Copy, Debug)]
pub struct FontSet {
    pub math: &'static [u8],
    pub serif: &'static [u8],
    pub serif_italic: &'static [u8],
    pub mono: &'static [u8],
    pub emoji: &'static [u8],
}

impl FontSet {
    pub fn get(&self, role: FontRole) -> &'static [u8] {
        match role {
            FontRole::Math => self.math,
            FontRole::Serif => self.serif,
            FontRole::SerifItalic => self.serif_italic,
            FontRole::Mono => self.mono,
            FontRole::Emoji => self.emoji,
        }
    }
}

/// @emoji 🏗️ The native/host font provider — bytes baked into the binary at compile time.
pub fn embedded_fonts() -> FontSet {
    FontSet {
        math: include_bytes!("🔤️fonts/🧮️LibertinusMath-Regular.otf"),
        serif: include_bytes!("🔤️fonts/📖️LibertinusSerif-Regular.otf"),
        serif_italic: include_bytes!("🔤️fonts/🖋️LibertinusSerif-Italic.otf"),
        mono: include_bytes!("🔤️fonts/⌨️LibertinusMono-Regular.otf"),
        emoji: include_bytes!("🔤️fonts/🔤️NotoColorEmoji-subset.ttf"),
    }
}
//#endregion 🔖️Fonts

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
