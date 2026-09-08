
use super::*;

// 🔤️ These six assert real glyph-shaped output (`<path>`/`<rect>`/`<image>`), which only the
// native/host arm produces — the `wasm32-wasip2` arm's `estimate_svg` deliberately emits an
// empty `<svg>` (see its docstring). Native-only. RUNTIME-DEPENDENCY-ELIMINATION-FOR-S-PLUGINS-
// AND-ARTIFACTS (26/09/01).
#[cfg(not(all(target_arch = "wasm32", target_env = "p2")))]
#[test]
fn compiles_a_superscript_snippet_to_a_well_formed_svg() {
    let result = compile_snippet_to_svg("x^2", SnippetOptions::default()).expect("compile x^2");
    assert!(result.svg.starts_with("<svg "));
    assert!(result.svg.contains("<path"));
}

#[cfg(not(all(target_arch = "wasm32", target_env = "p2")))]
#[test]
fn compiles_a_fraction_snippet() {
    let result = compile_snippet_to_svg("frac(a, b)", SnippetOptions::default()).expect("compile frac(a, b)");
    assert!(result.svg.contains("<rect"));
}

#[cfg(not(all(target_arch = "wasm32", target_env = "p2")))]
#[test]
fn compiles_an_emoji_shortcode() {
    let result = compile_snippet_to_svg(":rocket:", SnippetOptions::default()).expect("compile :rocket:");
    assert!(result.svg.contains("<image"));
}

#[cfg(not(all(target_arch = "wasm32", target_env = "p2")))]
#[test]
fn compile_text_to_svg_renders_arbitrary_strings_including_notation_special_characters() {
    let result = compile_text_to_svg("a_b < c!", SnippetOptions::default());
    assert!(result.svg.starts_with("<svg "));
    assert!(result.svg.contains("<path"));
}

#[cfg(not(all(target_arch = "wasm32", target_env = "p2")))]
#[test]
fn compile_emoji_to_svg_renders_a_known_emoji_character() {
    let result = compile_emoji_to_svg("🚀", SnippetOptions::default());
    assert!(result.svg.contains("<image"));
}

#[cfg(not(all(target_arch = "wasm32", target_env = "p2")))]
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
