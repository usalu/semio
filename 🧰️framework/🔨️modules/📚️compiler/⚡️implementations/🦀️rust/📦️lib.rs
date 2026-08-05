//! 📚️ `compiler` — facade for semio's incremental document compiler, replacing Typst. Technologies
//! depend on this one crate to reach the compiler's sub-slots (`📖️syntax`, `🌍️world`, `🔤️text`,
//! `🧮️math`, `📤️svg`, and later `⚙️eval`, `📐️layout`, `🧊️wgpu`, …), added one slot at a time as
//! each wave lands.
//!
//! [`compile_snippet_to_svg`] is Wave 2's deliverable: the functional replacement for both Typst
//! call sites this module exists to evict — `board_typst_markup_to_svg` (infinite-canvas icon
//! codec) and `typst_markup_to_svg` (animate-core `MathText`/`Text`).

pub use compiler_syntax as syntax;

use compiler_math::FontContext;
use compiler_svg::{FontSet as SvgFontSet, SvgOptions};
use compiler_text::Font;
use std::sync::OnceLock;

//#region 🔖️Fonts
struct Fonts {
    math: Font<'static>,
    serif: Font<'static>,
    mono: Font<'static>,
    emoji: Font<'static>,
}

static FONTS: OnceLock<Fonts> = OnceLock::new();

/// @emoji 🌍️ Lazily parses the embedded font set once per process — every `compile_snippet_to_svg`
/// call after the first reuses the same parsed `Font`s.
fn fonts() -> &'static Fonts {
    FONTS.get_or_init(|| {
        let embedded = compiler_world::embedded_fonts();
        Fonts {
            math: Font::from_bytes(embedded.math, 0).expect("embedded Math font must parse"),
            serif: Font::from_bytes(embedded.serif, 0).expect("embedded Serif font must parse"),
            mono: Font::from_bytes(embedded.mono, 0).expect("embedded Mono font must parse"),
            emoji: Font::from_bytes(embedded.emoji, 0).expect("embedded Emoji font must parse"),
        }
    })
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
    Syntax(dsl_core::TextError),
}

impl std::fmt::Display for CompileError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CompileError::Syntax(err) => write!(f, "{err}"),
        }
    }
}

impl std::error::Error for CompileError {}

fn render_to_svg(box_: &compiler_math::MathBox, options: SnippetOptions) -> SvgSnippet {
    let f = fonts();
    let svg_ctx = SvgFontSet { math: &f.math, serif: &f.serif, mono: &f.mono };
    let svg = compiler_svg::render_svg(box_, &svg_ctx, SvgOptions { font_size_pt: options.font_size_pt, margin_pt: options.margin_pt });
    SvgSnippet { svg }
}

/// @emoji 🎯️ Parses `src` as a semio math notation snippet ([`syntax::parse_formula`]) and renders
/// it to a standalone SVG string — the functional replacement for
/// `typst::compile::<PagedDocument>` + `typst_svg::svg_merged` at both existing Typst call sites.
pub fn compile_snippet_to_svg(src: &str, options: SnippetOptions) -> Result<SvgSnippet, CompileError> {
    let node = compiler_syntax::parse_formula(src).map_err(CompileError::Syntax)?;
    let f = fonts();
    let layout_ctx = FontContext { math: &f.math, serif: &f.serif, mono: &f.mono, emoji: &f.emoji };
    let box_ = compiler_math::layout(&layout_ctx, &node);
    Ok(render_to_svg(&box_, options))
}

/// @emoji 🔤️ Renders arbitrary `text` (not parsed as math notation — see
/// [`compiler_math::layout_raw_text`]) to a standalone SVG string. For callers with a plain string
/// to render as an icon/label, where `text` isn't guaranteed to be valid math notation syntax.
pub fn compile_text_to_svg(text: &str, options: SnippetOptions) -> SvgSnippet {
    let f = fonts();
    let layout_ctx = FontContext { math: &f.math, serif: &f.serif, mono: &f.mono, emoji: &f.emoji };
    let box_ = compiler_math::layout_raw_text(&layout_ctx, text);
    render_to_svg(&box_, options)
}

/// @emoji 😀️ Renders arbitrary emoji `text` (see [`compiler_math::layout_raw_emoji`]) to a
/// standalone SVG string.
pub fn compile_emoji_to_svg(text: &str, options: SnippetOptions) -> SvgSnippet {
    let f = fonts();
    let layout_ctx = FontContext { math: &f.math, serif: &f.serif, mono: &f.mono, emoji: &f.emoji };
    let box_ = compiler_math::layout_raw_emoji(&layout_ctx, text);
    render_to_svg(&box_, options)
}

/// @emoji 💻️ Renders arbitrary `code` (not parsed — see [`compiler_math::layout_raw_code`]) via the
/// Mono font to a standalone SVG string. For callers rendering a monospace code/source snippet.
pub fn compile_code_to_svg(code: &str, options: SnippetOptions) -> SvgSnippet {
    let f = fonts();
    let layout_ctx = FontContext { math: &f.math, serif: &f.serif, mono: &f.mono, emoji: &f.emoji };
    let box_ = compiler_math::layout_raw_code(&layout_ctx, code);
    render_to_svg(&box_, options)
}
//#endregion 🔖️Snippet

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn compiles_a_superscript_snippet_to_a_well_formed_svg() {
        let result = compile_snippet_to_svg("x^2", SnippetOptions::default()).expect("compile x^2");
        assert!(result.svg.starts_with("<svg "));
        assert!(result.svg.contains("<path"));
    }

    #[test]
    fn compiles_a_fraction_snippet() {
        let result = compile_snippet_to_svg("frac(a, b)", SnippetOptions::default()).expect("compile frac(a, b)");
        assert!(result.svg.contains("<rect"));
    }

    #[test]
    fn compiles_an_emoji_shortcode() {
        let result = compile_snippet_to_svg(":rocket:", SnippetOptions::default()).expect("compile :rocket:");
        assert!(result.svg.contains("<image"));
    }

    #[test]
    fn compile_text_to_svg_renders_arbitrary_strings_including_notation_special_characters() {
        let result = compile_text_to_svg("a_b < c!", SnippetOptions::default());
        assert!(result.svg.starts_with("<svg "));
        assert!(result.svg.contains("<path"));
    }

    #[test]
    fn compile_emoji_to_svg_renders_a_known_emoji_character() {
        let result = compile_emoji_to_svg("🚀", SnippetOptions::default());
        assert!(result.svg.contains("<image"));
    }

    #[test]
    fn compile_code_to_svg_renders_via_the_mono_font() {
        let result = compile_code_to_svg("fn main() {}", SnippetOptions::default());
        assert!(result.svg.contains("<path"));
    }

    #[test]
    fn invalid_syntax_is_a_syntax_error_not_a_panic() {
        let err = compile_snippet_to_svg("frac(a, b", SnippetOptions::default()).expect_err("unclosed call must fail to parse");
        assert!(matches!(err, CompileError::Syntax(_)));
    }

    #[test]
    fn repeated_calls_reuse_the_lazily_parsed_fonts() {
        // Not a behavioral assertion beyond "doesn't panic/reparsing corrupt state" — the OnceLock
        // is the actual mechanism under test; this just exercises it more than once.
        for src in ["x", "y^2", "frac(1, 2)"] {
            compile_snippet_to_svg(src, SnippetOptions::default()).unwrap_or_else(|e| panic!("compile {src:?} failed: {e}"));
        }
    }
}
//#endregion 🧪️Tests
