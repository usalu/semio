use super::*;
use std::sync::OnceLock;

fn fonts() -> &'static CompilerFonts<'static> {
    static FONTS: OnceLock<CompilerFonts<'static>> = OnceLock::new();
    FONTS.get_or_init(|| CompilerFonts::parse(&crate::world::tests::test_fonts()).expect("compiler font assets must parse"))
}

// 🔤️ These six assert real glyph-shaped output (`<path>`/`<rect>`/`<image>`), which only the
// native/host arm produces — the `wasm32-wasip2` arm's `estimate_svg` deliberately emits an
// empty `<svg>` (see its docstring). Native-only. RUNTIME-DEPENDENCY-ELIMINATION-FOR-S-PLUGINS-
// AND-ARTIFACTS (26/09/01).
#[cfg(not(all(target_arch = "wasm32", target_env = "p2")))]
#[test]
fn compiles_a_superscript_snippet_to_a_well_formed_svg() {
    let result = compile_snippet_to_svg(fonts(), "x^2", SnippetOptions::default()).expect("compile x^2");
    assert!(result.svg.starts_with("<svg "));
    assert!(result.svg.contains("<path"));
}

#[cfg(not(all(target_arch = "wasm32", target_env = "p2")))]
#[test]
fn compiles_a_fraction_snippet() {
    let result = compile_snippet_to_svg(fonts(), "frac(a, b)", SnippetOptions::default()).expect("compile frac(a, b)");
    assert!(result.svg.contains("<rect"));
}

#[cfg(not(all(target_arch = "wasm32", target_env = "p2")))]
#[test]
fn compiles_an_emoji_shortcode() {
    let result = compile_snippet_to_svg(fonts(), ":rocket:", SnippetOptions::default()).expect("compile :rocket:");
    assert!(result.svg.contains("<image"));
}

#[cfg(not(all(target_arch = "wasm32", target_env = "p2")))]
#[test]
fn compile_text_to_svg_renders_arbitrary_strings_including_notation_special_characters() {
    let result = compile_text_to_svg(fonts(), "a_b < c!", SnippetOptions::default());
    assert!(result.svg.starts_with("<svg "));
    assert!(result.svg.contains("<path"));
}

#[cfg(not(all(target_arch = "wasm32", target_env = "p2")))]
#[test]
fn compile_emoji_to_svg_renders_a_known_emoji_character() {
    let result = compile_emoji_to_svg(fonts(), "🚀", SnippetOptions::default());
    assert!(result.svg.contains("<image"));
}

#[cfg(not(all(target_arch = "wasm32", target_env = "p2")))]
#[test]
fn compile_code_to_svg_renders_via_the_mono_font() {
    let result = compile_code_to_svg(fonts(), "fn main() {}", SnippetOptions::default());
    assert!(result.svg.contains("<path"));
}

#[test]
fn invalid_syntax_is_a_syntax_error_not_a_panic() {
    let err = compile_snippet_to_svg(fonts(), "frac(a, b", SnippetOptions::default()).expect_err("unclosed call must fail to parse");
    assert!(matches!(err, CompileError::Syntax(_)));
}

#[test]
fn repeated_calls_share_one_parsed_font_set() {
    for src in ["x", "y^2", "frac(1, 2)"] {
        compile_snippet_to_svg(fonts(), src, SnippetOptions::default()).unwrap_or_else(|e| panic!("compile {src:?} failed: {e}"));
    }
}

#[cfg(not(all(target_arch = "wasm32", target_env = "p2")))]
#[test]
fn independently_parsed_font_sets_compile_identical_svg() {
    let first = CompilerFonts::parse(&crate::world::tests::test_fonts()).expect("first parse");
    let second = CompilerFonts::parse(&crate::world::tests::test_fonts()).expect("second parse");
    for src in ["x^2", "frac(a, b)", ":rocket:"] {
        let a = compile_snippet_to_svg(&first, src, SnippetOptions::default()).expect("first compile");
        let b = compile_snippet_to_svg(&second, src, SnippetOptions::default()).expect("second compile");
        assert_eq!(a, b, "{src}");
    }
}

#[cfg(not(all(target_arch = "wasm32", target_env = "p2")))]
#[test]
fn parse_names_the_role_whose_bytes_are_not_a_font() {
    let real = crate::world::tests::test_fonts();
    let garbage: &[u8] = &[0u8; 16];
    let failure = CompilerFonts::parse(&FontSet { serif: garbage, ..real }).err().expect("garbage serif must be refused");
    assert_eq!(failure, FontParseError { role: FontRole::Serif });
    assert!(failure.to_string().contains(FontRole::Serif.asset_file_name()), "{failure}");
    let failure = CompilerFonts::parse(&FontSet { emoji: garbage, ..real }).err().expect("garbage emoji must be refused");
    assert_eq!(failure.role, FontRole::Emoji);
}
