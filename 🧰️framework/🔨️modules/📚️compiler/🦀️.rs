//! 📚️ `compiler` — facade for semio's incremental document compiler, replacing Typst. Technologies
//! depend on this one crate to reach the compiler's sub-slots (`📖️syntax`, `🌍️world`, `🔤️text`,
//! `🧮️math`, `📤️svg`, and later `⚙️eval`, `📐️layout`, `🧊️wgpu`, …), added one slot at a time as
//! each wave lands.
//!
//! [`compile_snippet_to_svg`] is Wave 2's deliverable: the functional replacement for both Typst
//! call sites this module exists to evict — `board_typst_markup_to_svg` (infinite-canvas icon
//! codec) and `typst_markup_to_svg` (animate-core `MathText`/`Text`).

#[cfg(not(all(target_arch = "wasm32", target_env = "p2")))]
use crate::math::FontContext;
#[cfg(not(all(target_arch = "wasm32", target_env = "p2")))]
use crate::svg::{FontSet as SvgFontSet, SvgOptions};
#[cfg(not(all(target_arch = "wasm32", target_env = "p2")))]
use crate::text::Font;
use crate::world::{FontRole, FontSet};
//#region 🔖️Fonts
/// 🌍️ The host-supplied fonts, parsed once. Build it from the host's [`FontSet`] and share it:
/// every `compile_*_to_svg` call takes it by reference and parses nothing.
#[cfg(not(all(target_arch = "wasm32", target_env = "p2")))]
pub struct CompilerFonts<'a> {
    math: Font<'a>,
    serif: Font<'a>,
    mono: Font<'a>,
    emoji: Font<'a>,
}

/// 🔨️ `wasm32-wasip2` arm: no glyph engine is linked on this target, so there is nothing to
/// parse; [`CompilerFonts::estimate_only`] is the only value and the `compile_*_to_svg` arms
/// estimate size from character count.
#[cfg(all(target_arch = "wasm32", target_env = "p2"))]
pub struct CompilerFonts<'a> {
    bytes: std::marker::PhantomData<&'a [u8]>,
}

/// 🚫️ The host's bytes for `role` are not a parseable font face.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct FontParseError {
    pub role: FontRole,
}

impl std::fmt::Display for FontParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "compiler font {:?} ({}) does not parse", self.role, self.role.asset_file_name())
    }
}

impl std::error::Error for FontParseError {}

#[cfg(not(all(target_arch = "wasm32", target_env = "p2")))]
impl<'a> CompilerFonts<'a> {
    /// 📖️ Parses the Math, Serif, Mono and Emoji faces out of `set`; the host keeps the bytes alive.
    pub fn parse(set: &FontSet<'a>) -> Result<Self, FontParseError> {
        let face = |role: FontRole| Font::from_bytes(set.get(role), 0).ok_or(FontParseError { role });
        Ok(Self { math: face(FontRole::Math)?, serif: face(FontRole::Serif)?, mono: face(FontRole::Mono)?, emoji: face(FontRole::Emoji)? })
    }

    fn layout_context(&self) -> FontContext<'_> {
        FontContext { math: &self.math, serif: &self.serif, mono: &self.mono, emoji: &self.emoji }
    }
}

#[cfg(all(target_arch = "wasm32", target_env = "p2"))]
impl<'a> CompilerFonts<'a> {
    /// ✅️ Accepts any set and parses nothing — this target never paints glyphs.
    pub fn parse(_set: &FontSet<'a>) -> Result<Self, FontParseError> {
        Ok(Self::estimate_only())
    }

    pub const fn estimate_only() -> Self {
        Self { bytes: std::marker::PhantomData }
    }
}
//#endregion 🔖️Fonts

//#region 🔖️Snippet
#[derive(Clone, Copy, Debug)]
pub struct SnippetOptions {
    pub font_size_pt: f32,
    pub margin_pt: f32,
}

impl Default for SnippetOptions {
    fn default() -> Self {
        Self { font_size_pt: 28.0, margin_pt: 3.0 }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct SvgSnippet {
    pub svg: String,
}

#[derive(Clone, Debug, PartialEq)]
pub enum CompileError {
    Syntax(semio_framework_diagnostic::TextError),
}

impl std::fmt::Display for CompileError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CompileError::Syntax(err) => write!(f, "{err}"),
        }
    }
}

impl std::error::Error for CompileError {}

#[cfg(not(all(target_arch = "wasm32", target_env = "p2")))]
fn render_to_svg(fonts: &CompilerFonts<'_>, box_: &crate::math::MathBox, options: SnippetOptions) -> SvgSnippet {
    let svg_ctx = SvgFontSet { math: &fonts.math, serif: &fonts.serif, mono: &fonts.mono };
    let svg = crate::svg::render_svg(box_, &svg_ctx, SvgOptions { font_size_pt: options.font_size_pt, margin_pt: options.margin_pt });
    SvgSnippet { svg }
}

/// 🎯️ Parses `src` as a semio math notation snippet ([`syntax::parse_formula`]) and renders
/// it to a standalone SVG string — the functional replacement for
/// `typst::compile::<PagedDocument>` + `typst_svg::svg_merged` at both existing Typst call sites.
#[cfg(not(all(target_arch = "wasm32", target_env = "p2")))]
pub fn compile_snippet_to_svg(fonts: &CompilerFonts<'_>, src: &str, options: SnippetOptions) -> Result<SvgSnippet, CompileError> {
    let node = crate::syntax::parse_formula(src).map_err(CompileError::Syntax)?;
    let box_ = crate::math::layout(&fonts.layout_context(), &node);
    Ok(render_to_svg(fonts, &box_, options))
}

/// 🔤️ Renders arbitrary `text` (not parsed as math notation — see
/// [`crate::math::layout_raw_text`]) to a standalone SVG string. For callers with a plain string
/// to render as an icon/label, where `text` isn't guaranteed to be valid math notation syntax.
#[cfg(not(all(target_arch = "wasm32", target_env = "p2")))]
pub fn compile_text_to_svg(fonts: &CompilerFonts<'_>, text: &str, options: SnippetOptions) -> SvgSnippet {
    let box_ = crate::math::layout_raw_text(&fonts.layout_context(), text);
    render_to_svg(fonts, &box_, options)
}

/// 😀️ Renders arbitrary emoji `text` (see [`crate::math::layout_raw_emoji`]) to a
/// standalone SVG string.
#[cfg(not(all(target_arch = "wasm32", target_env = "p2")))]
pub fn compile_emoji_to_svg(fonts: &CompilerFonts<'_>, text: &str, options: SnippetOptions) -> SvgSnippet {
    let box_ = crate::math::layout_raw_emoji(&fonts.layout_context(), text);
    render_to_svg(fonts, &box_, options)
}

/// 💻️ Renders arbitrary `code` (not parsed — see [`crate::math::layout_raw_code`]) via the
/// Mono font to a standalone SVG string. For callers rendering a monospace code/source snippet.
#[cfg(not(all(target_arch = "wasm32", target_env = "p2")))]
pub fn compile_code_to_svg(fonts: &CompilerFonts<'_>, code: &str, options: SnippetOptions) -> SvgSnippet {
    let box_ = crate::math::layout_raw_code(&fonts.layout_context(), code);
    render_to_svg(fonts, &box_, options)
}

/// ↔ `wasm32-wasip2` arm shared by all four `compile_*_to_svg` entry points: no
/// glyph-shaping engine (`rustybuzz`) is linked on this target (see this crate's Cargo.toml
/// docstring). Estimates the SVG's declared box from `char_count` using the same
/// character-width-to-font-size ratio already established for the identical class of heuristic
/// fallback elsewhere in this ticket (`♾️infinite`'s `label_advance`, ratio `0.62`) — a disclosed,
/// intentional precision difference, not a stub: the returned `<svg>` carries no glyph paths,
/// since nothing on this target ever paints one (icon painting is unconditionally host-only, see
/// `IconPaintCache::get_or_build`). The only real consumer of this arm's output is
/// `preview_media_natural_size`'s dimension query, which reads the declared `width`/`height`
/// attributes only (`semio-framework-intrinsic-size`'s `wasm32-wasip2` arm), never glyph geometry.
/// Ticket `26/09/01/RUNTIME-DEPENDENCY-ELIMINATION-FOR-S-PLUGINS-AND-ARTIFACTS`.
#[cfg(all(target_arch = "wasm32", target_env = "p2"))]
fn estimate_svg(char_count: usize, options: SnippetOptions) -> SvgSnippet {
    const CHAR_WIDTH_RATIO: f32 = 0.62;
    const HEIGHT_RATIO: f32 = 1.2;
    let width_pt = (char_count as f32 * options.font_size_pt * CHAR_WIDTH_RATIO + options.margin_pt * 2.0).max(1.0);
    let height_pt = (options.font_size_pt * HEIGHT_RATIO + options.margin_pt * 2.0).max(1.0);
    SvgSnippet { svg: format!(r#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 {width_pt:.3} {height_pt:.3}" width="{width_pt:.3}" height="{height_pt:.3}"></svg>"#) }
}

#[cfg(all(target_arch = "wasm32", target_env = "p2"))]
pub fn compile_snippet_to_svg(_fonts: &CompilerFonts<'_>, src: &str, options: SnippetOptions) -> Result<SvgSnippet, CompileError> {
    crate::syntax::parse_formula(src).map_err(CompileError::Syntax)?;
    Ok(estimate_svg(src.chars().count(), options))
}

#[cfg(all(target_arch = "wasm32", target_env = "p2"))]
pub fn compile_text_to_svg(_fonts: &CompilerFonts<'_>, text: &str, options: SnippetOptions) -> SvgSnippet {
    estimate_svg(text.chars().count(), options)
}

#[cfg(all(target_arch = "wasm32", target_env = "p2"))]
pub fn compile_emoji_to_svg(_fonts: &CompilerFonts<'_>, text: &str, options: SnippetOptions) -> SvgSnippet {
    estimate_svg(text.chars().count(), options)
}

#[cfg(all(target_arch = "wasm32", target_env = "p2"))]
pub fn compile_code_to_svg(_fonts: &CompilerFonts<'_>, code: &str, options: SnippetOptions) -> SvgSnippet {
    estimate_svg(code.chars().count(), options)
}
//#endregion 🔖️Snippet

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
