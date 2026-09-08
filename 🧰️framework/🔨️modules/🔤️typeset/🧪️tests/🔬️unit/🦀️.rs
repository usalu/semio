
use super::*;

#[test]
#[cfg(not(all(target_arch = "wasm32", target_env = "p2")))]
fn typst_plain_text_compiles_to_svg() {
    let svg = default_typesetter().render_svg("#set page(width: 100pt, height: 100pt, margin: 4pt, fill: none)\n\"hello\"");
    assert!(svg.is_some());
    assert!(svg.unwrap().contains("svg"));
}

#[test]
fn typst_empty_markup_is_none_or_svg() {
    let svg = default_typesetter().render_svg("");
    if let Some(svg) = svg {
        assert!(svg.contains("svg"));
    }
}

#[test]
#[cfg(not(all(target_arch = "wasm32", target_env = "p2")))]
fn svg_outline_paths_extracts_at_least_one_path() {
    let svg = default_typesetter().render_svg("#set page(width: 100pt, height: 100pt, margin: 4pt, fill: none)\n#set text(size: 36pt)\n\"A\"").expect("compiled svg");
    let (_, height) = svg_natural_size(&svg).expect("natural size");
    assert!(height > 0.0);
    let paths = svg_outline_paths(&svg, 1.0, height).expect("outline paths");
    assert!(!paths.is_empty());
}

#[test]
fn svg_outline_paths_none_on_garbage_input() {
    assert!(svg_outline_paths("not an svg document", 1.0, 0.0).is_none());
}

/// 🔬️ Language-agnostic fixture: a hand-authored SVG (not Typst output) with a known 10×10
/// square path, so the expected extracted geometry is exact and independent of Typst/usvg
/// internals — this is the test that actually exercises our coordinate-flip math
/// (`map_svg_point`), not just "usvg parsed something". Native-only: the wasip2 stub never
/// parses anything, so there is nothing for this fixture to exercise there.
#[cfg(not(all(target_arch = "wasm32", target_env = "p2")))]
const FIXTURE_SQUARE_SVG: &str = r#"<svg xmlns="http://www.w3.org/2000/svg" width="10" height="10"><path d="M0 0L10 0L10 10L0 10Z" fill="black"/></svg>"#;

#[test]
#[cfg(not(all(target_arch = "wasm32", target_env = "p2")))]
fn svg_natural_size_matches_fixture_dimensions() {
    let (width, height) = svg_natural_size(FIXTURE_SQUARE_SVG).expect("fixture parses");
    assert!((width - 10.0).abs() < 1e-6);
    assert!((height - 10.0).abs() < 1e-6);
}

#[test]
#[cfg(not(all(target_arch = "wasm32", target_env = "p2")))]
fn svg_outline_paths_flips_y_and_scales_exactly() {
    let paths = svg_outline_paths(FIXTURE_SQUARE_SVG, 2.0, 20.0).expect("fixture parses");
    assert_eq!(paths.len(), 1);
    let points: Vec<(f64, f64)> = paths[0]
        .elements()
        .into_iter()
        .filter_map(|el| match el {
            semio_framework_geometry::PathEl::MoveTo(p) => Some(p.into()),
            semio_framework_geometry::PathEl::LineTo(p) => Some(p.into()),
            _ => None,
        })
        .collect();
    // 🔢️ scale=2.0, flip_y_offset=20.0: (x,y) -> (2x, 20 - 2y). Source corners (0,0) (10,0)
    // (10,10) (0,10) become exactly (0,20) (20,20) (20,0) (0,0).
    assert_eq!(points, vec![(0.0, 20.0), (20.0, 20.0), (20.0, 0.0), (0.0, 0.0)]);
}
