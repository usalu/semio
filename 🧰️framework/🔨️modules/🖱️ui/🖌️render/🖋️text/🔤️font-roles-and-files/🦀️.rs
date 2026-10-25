//! 🎭️ Font roles and files: the font families, the text font roles and set, the font errors and the native font files with their read errors. See [`super`].

use super::*;

//#region 🔖️Font
/// 🔤️ Fixed family names every built-in font is forced under via `FontInfoOverride`, so multi-file
/// families (Noto Emoji's 12 codepoint-range buckets) merge into one fontique family regardless of
/// what each file's own `name` table declares. Ported verbatim from the wgpu-old target.
pub(super) const FAMILY_SANS: &str = "Anta";

pub(super) const FAMILY_SERIF: &str = "Kelly Slab";

pub(super) const FAMILY_MONO: &str = "Share Tech Mono";

const FAMILY_EMOJI: &str = "Noto Emoji";

/// 🧩️ One built-in font file of the text system, named by its identity in the runtime asset channel
/// instead of by compiled-in bytes. A host resolves [`Self::asset_path`] (relative to the
/// `🖼️assets/🔤️fonts` catalog directory) through its own channel — file system, fetch, packaged asset —
/// and hands the bytes over as a [`TextFontSet`].
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum TextFontRole {
    Sans,
    Serif,
    Mono,
    EmojiRegions,
    EmojiFlags,
    EmojiSymbols,
    EmojiObjects,
    EmojiActivities,
    EmojiTravel,
    EmojiFood,
    EmojiNature,
    EmojiPeople,
    EmojiFaces,
    EmojiJoinedForms,
    EmojiSupplement,
}

impl TextFontRole {
    pub const ALL: [Self; 15] = [
        Self::Sans,
        Self::Serif,
        Self::Mono,
        Self::EmojiRegions,
        Self::EmojiFlags,
        Self::EmojiSymbols,
        Self::EmojiObjects,
        Self::EmojiActivities,
        Self::EmojiTravel,
        Self::EmojiFood,
        Self::EmojiNature,
        Self::EmojiPeople,
        Self::EmojiFaces,
        Self::EmojiJoinedForms,
        Self::EmojiSupplement,
    ];
    pub(super) const EMOJI: [Self; 12] =
        [Self::EmojiRegions, Self::EmojiFlags, Self::EmojiSymbols, Self::EmojiObjects, Self::EmojiActivities, Self::EmojiTravel, Self::EmojiFood, Self::EmojiNature, Self::EmojiPeople, Self::EmojiFaces, Self::EmojiJoinedForms, Self::EmojiSupplement];

    pub const fn asset_path(self) -> &'static str {
        match self {
            Self::Sans => "🚀️anta/🏛️latin/📖️regular/🔤️outline.ttf",
            Self::Serif => "🧱️kelly-slab/🏛️latin/📖️regular/🔤️outline.ttf",
            Self::Mono => "⌨️share-tech-mono/🏛️latin/📖️regular/🔤️outline.ttf",
            Self::EmojiRegions => "😀️noto-emoji/🌍️regions/📖️regular/🔤️outline.ttf",
            Self::EmojiFlags => "😀️noto-emoji/🚩️flags/📖️regular/🔤️outline.ttf",
            Self::EmojiSymbols => "😀️noto-emoji/🔣️symbols/📖️regular/🔤️outline.ttf",
            Self::EmojiObjects => "😀️noto-emoji/🧰️objects/📖️regular/🔤️outline.ttf",
            Self::EmojiActivities => "😀️noto-emoji/🎯️activities/📖️regular/🔤️outline.ttf",
            Self::EmojiTravel => "😀️noto-emoji/🧳️travel/📖️regular/🔤️outline.ttf",
            Self::EmojiFood => "😀️noto-emoji/🍽️food/📖️regular/🔤️outline.ttf",
            Self::EmojiNature => "😀️noto-emoji/🌿️nature/📖️regular/🔤️outline.ttf",
            Self::EmojiPeople => "😀️noto-emoji/🧑️people/📖️regular/🔤️outline.ttf",
            Self::EmojiFaces => "😀️noto-emoji/😀️faces/📖️regular/🔤️outline.ttf",
            Self::EmojiJoinedForms => "😀️noto-emoji/🔗️joined-forms/📖️regular/🔤️outline.ttf",
            Self::EmojiSupplement => "😀️noto-emoji/🪉️supplement/📖️regular/🔤️outline.ttf",
        }
    }

    pub(super) const fn family(self) -> &'static str {
        match self {
            Self::Sans => FAMILY_SANS,
            Self::Serif => FAMILY_SERIF,
            Self::Mono => FAMILY_MONO,
            _ => FAMILY_EMOJI,
        }
    }

    pub(super) const fn slot(self) -> usize {
        self as usize
    }
}

/// 📦️ The bytes of every [`TextFontRole`], borrowed from whatever the host loaded them into. Only read
/// while [`TextSystem::new`] registers them; the system copies what it keeps.
#[derive(Clone, Copy, Debug)]
pub struct TextFontSet<'a> {
    pub(super) fonts: [&'a [u8]; 15],
}

impl<'a> TextFontSet<'a> {
    /// 🔌️ Host-neutral constructor: asks `load` for every role in [`TextFontRole::ALL`] order and stops
    /// at the first failure.
    pub fn try_from_loader<E>(mut load: impl FnMut(TextFontRole) -> Result<&'a [u8], E>) -> Result<Self, E> {
        let mut fonts: [&'a [u8]; 15] = [&[]; 15];
        for role in TextFontRole::ALL {
            fonts[role.slot()] = load(role)?;
        }
        Ok(Self { fonts })
    }

    pub fn get(&self, role: TextFontRole) -> &'a [u8] {
        self.fonts[role.slot()]
    }
}

/// ⚠️ A built-in font whose bytes fontique refused to register as a font.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct TextFontError {
    pub role: TextFontRole,
}

impl std::fmt::Display for TextFontError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "built-in text font {:?} ({}) is not a registrable font", self.role, self.role.asset_path())
    }
}

impl std::error::Error for TextFontError {}

/// 💾️ Native delivery of the built-in fonts: the fifteen files read from a catalog directory into
/// buffers this value owns. Absent on `wasm32`, where a host fetches the files itself and builds a
/// [`TextFontSet`] through [`TextFontSet::try_from_loader`] — this crate never links a file system into
/// a guest.
#[cfg(not(target_arch = "wasm32"))]
pub struct TextFontFiles {
    buffers: Vec<Vec<u8>>,
}

/// ❌️ The first font file [`TextFontFiles::read`] could not read.
#[cfg(not(target_arch = "wasm32"))]
#[derive(Debug)]
pub struct TextFontReadError {
    pub role: TextFontRole,
    pub path: std::path::PathBuf,
    pub cause: std::io::Error,
}

#[cfg(not(target_arch = "wasm32"))]
impl std::fmt::Display for TextFontReadError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "cannot read built-in text font {:?} at {}: {}", self.role, self.path.display(), self.cause)
    }
}

#[cfg(not(target_arch = "wasm32"))]
impl std::error::Error for TextFontReadError {}

#[cfg(not(target_arch = "wasm32"))]
impl TextFontFiles {
    pub fn read(fonts_root: &std::path::Path) -> Result<Self, TextFontReadError> {
        let mut buffers = Vec::with_capacity(TextFontRole::ALL.len());
        for role in TextFontRole::ALL {
            let path = fonts_root.join(role.asset_path());
            match std::fs::read(&path) {
                Ok(bytes) => buffers.push(bytes),
                Err(cause) => return Err(TextFontReadError { role, path, cause }),
            }
        }
        Ok(Self { buffers })
    }

    pub fn set(&self) -> TextFontSet<'_> {
        match TextFontSet::try_from_loader::<std::convert::Infallible>(|role| Ok(self.buffers[role.slot()].as_slice())) {
            Ok(set) => set,
        }
    }
}

/// 🧭️ Which family a [`TextStyle`] resolves to. `Custom` names a host-provided font requested via
/// [`TextSystem::request_font`] — never a `parley`/`fontique` type, so a caller never needs those
/// crates in scope just to pick a style.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum FontFamilyChoice {
    SansSerif,
    Serif,
    Monospace,
    Custom(FontDependencyId),
}
//#endregion 🔖️Font
