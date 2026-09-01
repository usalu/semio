//! 🔤️ Markup/math typesetting → SVG (Typst) and SVG → first-party vector paths (usvg).
//!
//! This is the ONLY place `typst`, `typst-svg`, `typst-assets` and `usvg` are named in this module
//! — every public type is first-party (`semio_framework_geometry::BezPath` plus plain Rust types),
//! so a caller never needs to import any of the four crates itself (CLAUDE.md: "use external
//! libraries behind an interface" / "MUST NOT export api that ... requires an interface/class/type
//! outside of this codebase"). Relocated from `🎞️animate`'s `⚙️engine/🔤️text` `TypstTextRenderer`
//! and `svg_to_vobject`/`collect_svg_paths` (ticket
//! 26/09/01/RUNTIME-DEPENDENCY-ELIMINATION-FOR-S-PLUGINS-AND-ARTIFACTS).

use semio_framework_geometry::{BezPath, Point};
use std::path::PathBuf;
use std::sync::OnceLock;
use typst::foundations::{Bytes, Datetime};
use typst::layout::{Abs, PagedDocument};
use typst::syntax::{FileId, Source, VirtualPath};
use typst::text::{Font, FontBook};
use typst::utils::LazyHash;
use typst::LibraryExt;
use typst::{Library, World};

//#region 🔖️MarkupTypesetter

/// 🖨️ Small first-party interface isolating the external Typst library (CLAUDE.md: "external libs
/// behind an interface"). A single real implementor ([`TypstTypesetter`]) exists; the trait exists
/// so the isolation itself is a named, documented contract rather than an implicit convention.
pub trait MarkupTypesetter {
    /// 🖊️ Compiles `markup` (Typst syntax) to a single merged SVG string, or `None` on a compile
    /// failure or an empty document.
    fn render_svg(&self, markup: &str) -> Option<String>;
}

/// 🖨️ The real Typst-backed implementation.
pub struct TypstTypesetter;

impl MarkupTypesetter for TypstTypesetter {
    fn render_svg(&self, markup: &str) -> Option<String> {
        typst_markup_to_svg(markup)
    }
}

/// 🏭️ Returns the (stateless, zero-sized) default typesetter.
pub fn default_typesetter() -> TypstTypesetter {
    TypstTypesetter
}

static TYPST_FONTS: OnceLock<Vec<Font>> = OnceLock::new();

fn typst_asset_font_list() -> Vec<Font> {
    let mut out = Vec::new();
    for bytes in typst_assets::fonts() {
        let blob = Bytes::new(bytes);
        let mut idx = 0u32;
        while let Some(f) = Font::new(blob.clone(), idx) {
            out.push(f);
            idx = idx.saturating_add(1);
        }
    }
    out
}

fn typst_compile_markup_to_svg(markup: &str, fonts: &'static [Font], book: &'static LazyHash<FontBook>) -> Option<String> {
    static LIB: OnceLock<LazyHash<Library>> = OnceLock::new();
    static MAIN: OnceLock<FileId> = OnceLock::new();
    let library = LIB.get_or_init(|| LazyHash::new(Library::default()));
    let main = *MAIN.get_or_init(|| FileId::new(None, VirtualPath::new("/typeset.typ")));
    let source = Source::new(main, markup.to_string());
    struct TypesetWorld<'a> {
        library: &'static LazyHash<Library>,
        book: &'static LazyHash<FontBook>,
        main: FileId,
        source: Source,
        fonts: &'a [Font],
    }
    impl World for TypesetWorld<'_> {
        fn library(&self) -> &LazyHash<Library> {
            self.library
        }
        fn book(&self) -> &LazyHash<FontBook> {
            self.book
        }
        fn main(&self) -> FileId {
            self.main
        }
        fn source(&self, id: FileId) -> typst::diag::FileResult<Source> {
            if id == self.main {
                Ok(self.source.clone())
            } else {
                Err(typst::diag::FileError::NotFound(PathBuf::from("typeset.typ")))
            }
        }
        fn file(&self, _id: FileId) -> typst::diag::FileResult<Bytes> {
            Err(typst::diag::FileError::NotFound(PathBuf::from("typeset.bin")))
        }
        fn font(&self, index: usize) -> Option<Font> {
            self.fonts.get(index).cloned()
        }
        fn today(&self, _offset: Option<i64>) -> Option<Datetime> {
            None
        }
    }
    let w = TypesetWorld { library, book, main, source, fonts };
    let warned = typst::compile::<PagedDocument>(&w);
    let doc = warned.output.ok()?;
    if doc.pages.is_empty() {
        return None;
    }
    Some(typst_svg::svg_merged(&doc, Abs::pt(4.0)))
}

/// 🖨️ Compile Typst markup to merged SVG.
fn typst_markup_to_svg(markup: &str) -> Option<String> {
    let fonts = TYPST_FONTS.get_or_init(typst_asset_font_list);
    static FONT_BOOK: OnceLock<LazyHash<FontBook>> = OnceLock::new();
    let book = FONT_BOOK.get_or_init(|| LazyHash::new(FontBook::from_fonts(fonts.iter())));
    typst_compile_markup_to_svg(markup, fonts.as_slice(), book)
}

//#endregion 🔖️MarkupTypesetter

//#region 🔖️SvgOutline

/// 📐️ The natural size (`width`, `height`) of an SVG document in its own user-space units, or
/// `None` if `svg` fails to parse.
pub fn svg_natural_size(svg: &str) -> Option<(f64, f64)> {
    let tree = usvg::Tree::from_str(svg, &usvg::Options::default()).ok()?;
    Some((tree.size().width() as f64, tree.size().height() as f64))
}

/// 🖼️ Parses `svg` and returns its vector path outlines, mapping each point
/// `(x, y) -> (x * scale, flip_y_offset - y * scale)` (SVG is y-down; this is the "flip to y-up
/// and rescale" step every caller needs). Returns `None` if `svg` fails to parse.
pub fn svg_outline_paths(svg: &str, scale: f64, flip_y_offset: f64) -> Option<Vec<BezPath>> {
    let tree = usvg::Tree::from_str(svg, &usvg::Options::default()).ok()?;
    let mut paths = Vec::new();
    for child in tree.root().children() {
        collect_svg_paths(child, scale, flip_y_offset, &mut paths);
    }
    Some(paths)
}

fn map_svg_point(x: f32, y: f32, scale: f64, flip_y_offset: f64) -> Point {
    Point::new(x as f64 * scale, flip_y_offset - y as f64 * scale)
}

fn collect_svg_paths(node: &usvg::Node, scale: f64, flip_y_offset: f64, out: &mut Vec<BezPath>) {
    match node {
        usvg::Node::Group(group) => {
            for child in group.children() {
                collect_svg_paths(child, scale, flip_y_offset, out);
            }
        }
        usvg::Node::Path(path) => {
            let mut p = BezPath::new();
            for segment in path.data().segments() {
                match segment {
                    usvg::tiny_skia_path::PathSegment::MoveTo(pt) => {
                        p.move_to(map_svg_point(pt.x, pt.y, scale, flip_y_offset));
                    }
                    usvg::tiny_skia_path::PathSegment::LineTo(pt) => {
                        p.line_to(map_svg_point(pt.x, pt.y, scale, flip_y_offset));
                    }
                    usvg::tiny_skia_path::PathSegment::QuadTo(c, pt) => {
                        p.quad_to(map_svg_point(c.x, c.y, scale, flip_y_offset), map_svg_point(pt.x, pt.y, scale, flip_y_offset));
                    }
                    usvg::tiny_skia_path::PathSegment::CubicTo(c1, c2, pt) => {
                        p.curve_to(map_svg_point(c1.x, c1.y, scale, flip_y_offset), map_svg_point(c2.x, c2.y, scale, flip_y_offset), map_svg_point(pt.x, pt.y, scale, flip_y_offset));
                    }
                    usvg::tiny_skia_path::PathSegment::Close => p.close_path(),
                }
            }
            if !p.elements().is_empty() {
                out.push(p);
            }
        }
        _ => {}
    }
}

//#endregion 🔖️SvgOutline

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
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
    /// (`map_svg_point`), not just "usvg parsed something".
    const FIXTURE_SQUARE_SVG: &str = r#"<svg xmlns="http://www.w3.org/2000/svg" width="10" height="10"><path d="M0 0L10 0L10 10L0 10Z" fill="black"/></svg>"#;

    #[test]
    fn svg_natural_size_matches_fixture_dimensions() {
        let (width, height) = svg_natural_size(FIXTURE_SQUARE_SVG).expect("fixture parses");
        assert!((width - 10.0).abs() < 1e-6);
        assert!((height - 10.0).abs() < 1e-6);
    }

    #[test]
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
}
