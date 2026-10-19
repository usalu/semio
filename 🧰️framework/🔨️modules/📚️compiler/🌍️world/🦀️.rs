//! 🌍️ `compiler_world` — the compiler's font port. The font set it names is Libertinus
//! Math/Serif/Mono (OFL 1.1, ~1.3 MB) plus the checked-in Noto Color Emoji CBDT subset (~150 KB),
//! well under `typst-assets`' ~15 MB. The files live beside this module in `🔤️fonts/` with their
//! license texts; the crate never embeds them. The host owns delivery (file system, fetch,
//! packaged asset) and hands the bytes in as a [`FontSet`], keyed by [`FontRole`] and named on
//! disk by [`FontRole::asset_file_name`].
//!
//! Scope: [`FontSet::from_directory`] is the native/host delivery path. The wasm-guest delivery
//! through the plugin host's `read-asset` seam builds the same [`FontSet`] from fetched bytes via
//! [`FontSet::try_from_loader`]; no wasm guest paints glyphs today, so none needs it yet.

use std::{fs, io, path::Path};

//#region 🔖️Fonts
/// 🔤️ One font's stable role — the role is what the layout layer (`compiler_math`) and shaper
/// (`compiler_text`) key off, never a file path or index.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum FontRole {
    /// ➗️ Libertinus Math — variables, symbols, and everything laid out via the OpenType MATH table.
    Math,
    /// 🧮️ Libertinus Serif Regular — upright text-in-math, numbers, function names.
    Serif,
    /// 🔖️ Libertinus Serif Italic — reserved for future prose/emphasis use; unused by math layout
    /// today (the Math font's own glyphs are already italic for Latin/Greek letters).
    SerifItalic,
    /// 🅿️ Libertinus Mono — reserved for future `Code` snippet rendering.
    Mono,
    /// 🎨️ Noto Color Emoji (CBDT bitmap subset) — `:shortcode:` atoms.
    Emoji,
}

impl FontRole {
    /// 📋️ Every role, in the field order of [`FontSet`].
    pub const ALL: [FontRole; 5] = [FontRole::Math, FontRole::Serif, FontRole::SerifItalic, FontRole::Mono, FontRole::Emoji];

    /// 🏷️ The asset's file name inside the `🔤️fonts/` directory — the identity a host resolves
    /// through its own delivery channel.
    pub const fn asset_file_name(self) -> &'static str {
        match self {
            FontRole::Math => "🧮️LibertinusMath-Regular.otf",
            FontRole::Serif => "📖️LibertinusSerif-Regular.otf",
            FontRole::SerifItalic => "🖋️LibertinusSerif-Italic.otf",
            FontRole::Mono => "⌨️LibertinusMono-Regular.otf",
            FontRole::Emoji => "🔤️NotoColorEmoji-subset.ttf",
        }
    }
}

/// 📦️ The full font set the compiler needs, keyed by [`FontRole`] — borrowed bytes the host owns.
#[derive(Clone, Copy, Debug)]
pub struct FontSet<'a> {
    pub math: &'a [u8],
    pub serif: &'a [u8],
    pub serif_italic: &'a [u8],
    pub mono: &'a [u8],
    pub emoji: &'a [u8],
}

impl<'a> FontSet<'a> {
    pub fn get(&self, role: FontRole) -> &'a [u8] {
        match role {
            FontRole::Math => self.math,
            FontRole::Serif => self.serif,
            FontRole::SerifItalic => self.serif_italic,
            FontRole::Mono => self.mono,
            FontRole::Emoji => self.emoji,
        }
    }

    /// 🔌️ Builds the set by asking `load` for every role in [`FontRole::ALL`] order and stops at
    /// the first failure — the host's delivery channel decides where the bytes come from.
    pub fn try_from_loader<E>(mut load: impl FnMut(FontRole) -> Result<&'a [u8], E>) -> Result<Self, E> {
        Ok(Self { math: load(FontRole::Math)?, serif: load(FontRole::Serif)?, serif_italic: load(FontRole::SerifItalic)?, mono: load(FontRole::Mono)?, emoji: load(FontRole::Emoji)? })
    }
}

impl FontSet<'static> {
    /// 📂️ Reads every [`FontRole::asset_file_name`] file from `directory` and leaks the buffers,
    /// so the set lives for the process. Call it once per process.
    pub fn from_directory(directory: &Path) -> io::Result<Self> {
        Self::try_from_loader(|role| {
            let path = directory.join(role.asset_file_name());
            let bytes = fs::read(&path).map_err(|error| io::Error::new(error.kind(), format!("{}: {error}", path.display())))?;
            Ok(&*Box::leak(bytes.into_boxed_slice()))
        })
    }
}
//#endregion 🔖️Fonts

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
pub(crate) mod tests;
//#endregion 🧪️Tests
