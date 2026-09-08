//! 🔤️ Markup/math typesetting → SVG (Typst) and SVG → first-party vector paths (usvg).
//!
//! Tier split (ticket 26/09/01/RUNTIME-DEPENDENCY-ELIMINATION-FOR-S-PLUGINS-AND-ARTIFACTS):
//! [`MarkupTypesetter`] and [`TypstTypesetter`]/[`default_typesetter`] are target-neutral — their
//! public signatures name no `typst::*`/`usvg::*` type — but every fn body naming `typst`/
//! `typst-svg`/`typst-assets`/`usvg` directly is native-only
//! (`#[cfg(not(all(target_arch = "wasm32", target_env = "p2")))]`); the shipped `wasm32-wasip2`
//! guest component links none of the four crates (moved to the matching
//! `[target.'cfg(...)'.dependencies]` table in this crate's `Cargo.toml`). This mirrors
//! `semio-framework-raster`'s `SceneRasterizer`/`wgpu`/`vello` split: `render_svg` and
//! `svg_outline_paths`/`svg_natural_size` are called (via `🎞️animate`'s `⚙️engine/🔤️text`) from
//! `Scene::construct` Sobject builders (`Text`/`MathText`/...), reachable from wasip2 guest command
//! dispatch through `export-video-from-deck` → `Editor::handle` — the same chain
//! `raster-tier-split.md` already traced hop by hop — so a bare `cfg`-gate deleting these fns
//! outright would break that chain's compilation. On `wasm32-wasip2` both fns honestly return
//! `None`, which is already the trait's documented "compile failure" outcome and drives the same
//! fallback (`svg_to_vobject`'s empty-svg branch) a real Typst compile error already produces
//! natively; the video-export command that reaches this path fails anyway for the unrelated,
//! already-established reason (`semio-framework-raster`'s `RasterError::Adapter` — no GPU on
//! wasip2), so this fallback never changes an export's outcome. Unlike raster's `wgpu`, there is no
//! WASI capability gap forcing this: it is a deliberate CLAUDE.md "no third-party runtime
//! dependency in the shipped component" elimination, not a technical impossibility, and is
//! documented as such rather than implied.

use semio_framework_geometry::BezPath;
#[cfg(not(all(target_arch = "wasm32", target_env = "p2")))]
use semio_framework_geometry::Point;
#[cfg(not(all(target_arch = "wasm32", target_env = "p2")))]
use std::path::PathBuf;
#[cfg(not(all(target_arch = "wasm32", target_env = "p2")))]
use std::sync::OnceLock;
#[cfg(not(all(target_arch = "wasm32", target_env = "p2")))]
use typst::foundations::{Bytes, Datetime};
#[cfg(not(all(target_arch = "wasm32", target_env = "p2")))]
use typst::layout::{Abs, PagedDocument};
#[cfg(not(all(target_arch = "wasm32", target_env = "p2")))]
use typst::syntax::{FileId, Source, VirtualPath};
#[cfg(not(all(target_arch = "wasm32", target_env = "p2")))]
use typst::text::{Font, FontBook};
#[cfg(not(all(target_arch = "wasm32", target_env = "p2")))]
use typst::utils::LazyHash;
#[cfg(not(all(target_arch = "wasm32", target_env = "p2")))]
use typst::LibraryExt;
#[cfg(not(all(target_arch = "wasm32", target_env = "p2")))]
use typst::{Library, World};

//#region 🔖️MarkupTypesetter

/// 🖨️ Small first-party interface isolating the external Typst library (CLAUDE.md: "external libs
/// behind an interface"). A single real implementor ([`TypstTypesetter`]) exists; the trait exists
/// so the isolation itself is a named, documented contract rather than an implicit convention.
pub trait MarkupTypesetter {
    /// 🖊️ Compiles `markup` (Typst syntax) to a single merged SVG string, or `None` on a compile
    /// failure, an empty document, or (on `wasm32-wasip2`, where no third-party typesetting engine
    /// is linked into the shipped component) unconditionally.
    fn render_svg(&self, markup: &str) -> Option<String>;
}

/// 🖨️ The real Typst-backed implementation on every target except `wasm32-wasip2`; a zero-sized
/// marker whose `render_svg` honestly reports "no engine" there. One definition serves both targets
/// since the type itself is zero-sized either way — only the trait impl below is per-target.
pub struct TypstTypesetter;

/// 🏭️ Returns the (stateless, zero-sized) default typesetter.
pub fn default_typesetter() -> TypstTypesetter {
    TypstTypesetter
}

#[cfg(not(all(target_arch = "wasm32", target_env = "p2")))]
impl MarkupTypesetter for TypstTypesetter {
    fn render_svg(&self, markup: &str) -> Option<String> {
        typst_markup_to_svg(markup)
    }
}

/// 🧊️ `wasm32-wasip2` links none of `typst`/`typst-svg`/`typst-assets` (see the module docstring);
/// this reports the same outcome the trait already documents for any other compile failure.
#[cfg(all(target_arch = "wasm32", target_env = "p2"))]
impl MarkupTypesetter for TypstTypesetter {
    fn render_svg(&self, _markup: &str) -> Option<String> {
        None
    }
}

#[cfg(not(all(target_arch = "wasm32", target_env = "p2")))]
static TYPST_FONTS: OnceLock<Vec<Font>> = OnceLock::new();

#[cfg(not(all(target_arch = "wasm32", target_env = "p2")))]
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

#[cfg(not(all(target_arch = "wasm32", target_env = "p2")))]
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
#[cfg(not(all(target_arch = "wasm32", target_env = "p2")))]
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
#[cfg(not(all(target_arch = "wasm32", target_env = "p2")))]
pub fn svg_natural_size(svg: &str) -> Option<(f64, f64)> {
    let tree = usvg::Tree::from_str(svg, &usvg::Options::default()).ok()?;
    Some((tree.size().width() as f64, tree.size().height() as f64))
}

/// 🧊️ `wasm32-wasip2` links no `usvg` (see the module docstring); reports the same "failed to
/// parse" outcome the native path reports for genuinely malformed SVG.
#[cfg(all(target_arch = "wasm32", target_env = "p2"))]
pub fn svg_natural_size(_svg: &str) -> Option<(f64, f64)> {
    None
}

/// 🖼️ Parses `svg` and returns its vector path outlines, mapping each point
/// `(x, y) -> (x * scale, flip_y_offset - y * scale)` (SVG is y-down; this is the "flip to y-up
/// and rescale" step every caller needs). Returns `None` if `svg` fails to parse.
#[cfg(not(all(target_arch = "wasm32", target_env = "p2")))]
pub fn svg_outline_paths(svg: &str, scale: f64, flip_y_offset: f64) -> Option<Vec<BezPath>> {
    let tree = usvg::Tree::from_str(svg, &usvg::Options::default()).ok()?;
    let mut paths = Vec::new();
    for child in tree.root().children() {
        collect_svg_paths(child, scale, flip_y_offset, &mut paths);
    }
    Some(paths)
}

/// 🧊️ See [`svg_natural_size`]'s wasip2 docstring — same reasoning.
#[cfg(all(target_arch = "wasm32", target_env = "p2"))]
pub fn svg_outline_paths(_svg: &str, _scale: f64, _flip_y_offset: f64) -> Option<Vec<BezPath>> {
    None
}

#[cfg(not(all(target_arch = "wasm32", target_env = "p2")))]
fn map_svg_point(x: f32, y: f32, scale: f64, flip_y_offset: f64) -> Point {
    Point::new(x as f64 * scale, flip_y_offset - y as f64 * scale)
}

#[cfg(not(all(target_arch = "wasm32", target_env = "p2")))]
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
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
