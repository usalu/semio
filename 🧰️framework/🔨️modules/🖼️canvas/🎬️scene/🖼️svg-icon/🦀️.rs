//! 🖼️ SVG icon painting: the `usvg` parse options, themed and literal paint of a parsed tree, content bounds, and the `wasm32-wasip2` declared-size arm.

#[cfg(not(all(target_arch = "wasm32", target_env = "p2")))]
use std::sync::{Arc, OnceLock};

#[cfg(not(all(target_arch = "wasm32", target_env = "p2")))]
use super::usvg;
#[cfg(not(all(target_arch = "wasm32", target_env = "p2")))]
use super::{Affine, BezPath, Color, FillRule, Point, Scene, ShapeRef, Stroke};

// #region 🔖️IconUsvgParseOptions

#[cfg(not(all(target_arch = "wasm32", target_env = "p2")))]
static ICON_USVG_OPTIONS: OnceLock<usvg::Options<'static>> = OnceLock::new();

#[cfg(not(all(target_arch = "wasm32", target_env = "p2")))]
static ICON_USVG_WITHOUT_FACE: OnceLock<usvg::Options<'static>> = OnceLock::new();

/// 🔤️ Shared `usvg` parse options with the installed Noto Color Emoji face (the compiler's emoji font, delivered through
/// the runtime asset port) so `<text>` in `emoji:` SVG matches the compiler's font set; avoids system fallback glyphs.
/// Until a host installs the runtime assets the options carry no face and such text paints no glyphs.
#[cfg(not(all(target_arch = "wasm32", target_env = "p2")))]
pub(super) fn usvg_options_icons() -> &'static usvg::Options<'static> {
    match semio_framework_canvas_assets::emoji_font() {
        Some(face) => ICON_USVG_OPTIONS.get_or_init(|| {
            let mut db = usvg::fontdb::Database::new();
            db.load_font_data(face.to_vec());
            usvg::Options { fontdb: Arc::new(db), font_family: ui_styling::canvas_fonts::NOTO_COLOR_EMOJI.into(), ..Default::default() }
        }),
        None => ICON_USVG_WITHOUT_FACE.get_or_init(|| usvg::Options { font_family: ui_styling::canvas_fonts::NOTO_COLOR_EMOJI.into(), ..Default::default() }),
    }
}

// #endregion 🔖️IconUsvgParseOptions

#[cfg(not(all(target_arch = "wasm32", target_env = "p2")))]
fn to_affine(ts: &usvg::Transform) -> Affine {
    let usvg::Transform { sx, kx, ky, sy, tx, ty } = *ts;
    Affine::new([sx, ky, kx, sy, tx, ty].map(f64::from))
}

#[cfg(not(all(target_arch = "wasm32", target_env = "p2")))]
fn to_bez_path(path: &usvg::Path) -> BezPath {
    let mut local_path = BezPath::new();
    let mut just_closed = false;
    let mut most_recent_initial = (0_f64, 0_f64);
    for elt in path.data().segments() {
        match elt {
            usvg::tiny_skia_path::PathSegment::MoveTo(p) => {
                if std::mem::take(&mut just_closed) {
                    local_path.move_to(most_recent_initial);
                }
                most_recent_initial = (p.x.into(), p.y.into());
                local_path.move_to(most_recent_initial);
            }
            usvg::tiny_skia_path::PathSegment::LineTo(p) => {
                if std::mem::take(&mut just_closed) {
                    local_path.move_to(most_recent_initial);
                }
                local_path.line_to(Point::new(p.x as f64, p.y as f64));
            }
            usvg::tiny_skia_path::PathSegment::QuadTo(p1, p2) => {
                if std::mem::take(&mut just_closed) {
                    local_path.move_to(most_recent_initial);
                }
                local_path.quad_to(Point::new(p1.x as f64, p1.y as f64), Point::new(p2.x as f64, p2.y as f64));
            }
            usvg::tiny_skia_path::PathSegment::CubicTo(p1, p2, p3) => {
                if std::mem::take(&mut just_closed) {
                    local_path.move_to(most_recent_initial);
                }
                local_path.curve_to(Point::new(p1.x as f64, p1.y as f64), Point::new(p2.x as f64, p2.y as f64), Point::new(p3.x as f64, p3.y as f64));
            }
            usvg::tiny_skia_path::PathSegment::Close => {
                just_closed = true;
                local_path.close_path();
            }
        }
    }
    local_path
}

#[cfg(not(all(target_arch = "wasm32", target_env = "p2")))]
fn map_solid_icon_paint(paint: &usvg::Paint, opacity: usvg::Opacity, fg: Color, bg: Color) -> Option<Color> {
    let usvg::Paint::Color(c) = paint else {
        return None;
    };
    let a = opacity.get();
    if c.red < 22 && c.green < 22 && c.blue < 22 {
        return Some(fg.multiply_alpha(a));
    }
    if c.red > 233 && c.green > 233 && c.blue > 233 {
        return Some(bg.multiply_alpha(a));
    }
    Some(Color::from_rgba8(c.red, c.green, c.blue, opacity.to_u8()))
}

#[cfg(not(all(target_arch = "wasm32", target_env = "p2")))]
fn stroke_path(scene: &mut Scene, path: &usvg::Path, transform: Affine, local_path: &BezPath, fg: Color, bg: Color) {
    if let Some(stroke) = path.stroke() {
        if let Some(color) = map_solid_icon_paint(stroke.paint(), stroke.opacity(), fg, bg) {
            let conv = Stroke::new(f64::from(stroke.width().get()));
            scene.stroke(&conv, transform, color, None, ShapeRef::BezPath(local_path));
        }
    }
}

#[cfg(not(all(target_arch = "wasm32", target_env = "p2")))]
fn fill_path(scene: &mut Scene, path: &usvg::Path, transform: Affine, local_path: &BezPath, fg: Color, bg: Color) {
    if let Some(fill) = path.fill() {
        if let Some(color) = map_solid_icon_paint(fill.paint(), fill.opacity(), fg, bg) {
            scene.fill(
                match fill.rule() {
                    usvg::FillRule::NonZero => FillRule::NonZero,
                    usvg::FillRule::EvenOdd => FillRule::EvenOdd,
                },
                transform,
                color,
                None,
                ShapeRef::BezPath(local_path),
            );
        }
    }
}

#[cfg(not(all(target_arch = "wasm32", target_env = "p2")))]
fn render_path(scene: &mut Scene, path: &usvg::Path, fg: Color, bg: Color, stroke_first: bool) {
    if !path.is_visible() {
        return;
    }
    let transform = to_affine(&path.abs_transform());
    let local_path = to_bez_path(path);
    if stroke_first {
        stroke_path(scene, path, transform, &local_path, fg, bg);
        fill_path(scene, path, transform, &local_path, fg, bg);
    } else {
        fill_path(scene, path, transform, &local_path, fg, bg);
        stroke_path(scene, path, transform, &local_path, fg, bg);
    }
}

#[cfg(not(all(target_arch = "wasm32", target_env = "p2")))]
fn render_group(scene: &mut Scene, group: &usvg::Group, fg: Color, bg: Color, stroke_first: bool) {
    for node in group.children() {
        match node {
            usvg::Node::Group(g) => render_group(scene, g, fg, bg, stroke_first),
            usvg::Node::Path(path) => render_path(scene, path, fg, bg, stroke_first),
            usvg::Node::Text(t) => render_group(scene, t.flattened(), fg, bg, true),
            _ => {}
        }
    }
}

#[cfg(not(all(target_arch = "wasm32", target_env = "p2")))]
fn literal_paint(paint: &usvg::Paint, opacity: usvg::Opacity) -> Option<Color> {
    let usvg::Paint::Color(c) = paint else {
        return None;
    };
    Some(Color::from_rgba8(c.red, c.green, c.blue, opacity.to_u8()))
}

#[cfg(not(all(target_arch = "wasm32", target_env = "p2")))]
fn stroke_path_literal(scene: &mut Scene, path: &usvg::Path, transform: Affine, local_path: &BezPath) {
    if let Some(stroke) = path.stroke() {
        if let Some(color) = literal_paint(stroke.paint(), stroke.opacity()) {
            let conv = Stroke::new(f64::from(stroke.width().get()));
            scene.stroke(&conv, transform, color, None, ShapeRef::BezPath(local_path));
        }
    }
}

#[cfg(not(all(target_arch = "wasm32", target_env = "p2")))]
fn fill_path_literal(scene: &mut Scene, path: &usvg::Path, transform: Affine, local_path: &BezPath) {
    if let Some(fill) = path.fill() {
        if let Some(color) = literal_paint(fill.paint(), fill.opacity()) {
            scene.fill(
                match fill.rule() {
                    usvg::FillRule::NonZero => FillRule::NonZero,
                    usvg::FillRule::EvenOdd => FillRule::EvenOdd,
                },
                transform,
                color,
                None,
                ShapeRef::BezPath(local_path),
            );
        }
    }
}

#[cfg(not(all(target_arch = "wasm32", target_env = "p2")))]
fn render_path_literal(scene: &mut Scene, path: &usvg::Path, stroke_first: bool) {
    if !path.is_visible() {
        return;
    }
    let transform = to_affine(&path.abs_transform());
    let local_path = to_bez_path(path);
    if stroke_first {
        stroke_path_literal(scene, path, transform, &local_path);
        fill_path_literal(scene, path, transform, &local_path);
    } else {
        fill_path_literal(scene, path, transform, &local_path);
        stroke_path_literal(scene, path, transform, &local_path);
    }
}

#[cfg(not(all(target_arch = "wasm32", target_env = "p2")))]
fn render_group_literal(scene: &mut Scene, group: &usvg::Group, stroke_first: bool) {
    for node in group.children() {
        match node {
            usvg::Node::Group(g) => render_group_literal(scene, g, stroke_first),
            usvg::Node::Path(path) => render_path_literal(scene, path, stroke_first),
            usvg::Node::Text(t) => render_group_literal(scene, t.flattened(), true),
            _ => {}
        }
    }
}

#[cfg(not(all(target_arch = "wasm32", target_env = "p2")))]
/// 🏷️ Renders SVG tree paints literally (no icon fg/bg remapping); used for map labels.
pub(super) fn render_svg_tree_literal(scene: &mut Scene, tree: &usvg::Tree) {
    render_group_literal(scene, tree.root(), false);
}

#[cfg(not(all(target_arch = "wasm32", target_env = "p2")))]
fn icon_rect_xywh(r: usvg::Rect) -> Option<(f64, f64, f64, f64)> {
    let w = f64::from(r.width());
    let h = f64::from(r.height());
    if !(w > 1e-6 && h > 1e-6 && w.is_finite() && h.is_finite()) {
        return None;
    }
    Some((f64::from(r.x()), f64::from(r.y()), w, h))
}

#[cfg(not(all(target_arch = "wasm32", target_env = "p2")))]
fn icon_rect_nonzero(r: usvg::tiny_skia_path::NonZeroRect) -> (f64, f64, f64, f64) {
    (f64::from(r.x()), f64::from(r.y()), f64::from(r.width()), f64::from(r.height()))
}

#[cfg(not(all(target_arch = "wasm32", target_env = "p2")))]
fn icon_union_xywh(a: (f64, f64, f64, f64), b: (f64, f64, f64, f64)) -> (f64, f64, f64, f64) {
    let ax1 = a.0 + a.2;
    let ay1 = a.1 + a.3;
    let bx1 = b.0 + b.2;
    let by1 = b.1 + b.3;
    let x0 = a.0.min(b.0);
    let y0 = a.1.min(b.1);
    let x1 = ax1.max(bx1);
    let y1 = ay1.max(by1);
    (x0, y0, x1 - x0, y1 - y0)
}

#[cfg(not(all(target_arch = "wasm32", target_env = "p2")))]
fn icon_union_rects_into(acc: &mut Option<(f64, f64, f64, f64)>, r: usvg::Rect) {
    if let Some(xy) = icon_rect_xywh(r) {
        *acc = Some(match acc.take() {
            None => xy,
            Some(a) => icon_union_xywh(a, xy),
        });
    }
}

#[cfg(not(all(target_arch = "wasm32", target_env = "p2")))]
fn icon_visit_node_bounds(node: &usvg::Node, acc: &mut Option<(f64, f64, f64, f64)>) {
    match node {
        usvg::Node::Group(g) => {
            for c in g.children() {
                icon_visit_node_bounds(c, acc);
            }
        }
        usvg::Node::Path(p) => {
            if !p.is_visible() {
                return;
            }
            icon_union_rects_into(acc, p.abs_bounding_box());
            icon_union_rects_into(acc, p.abs_stroke_bounding_box());
        }
        usvg::Node::Image(img) => {
            if !img.is_visible() {
                return;
            }
            icon_union_rects_into(acc, img.abs_bounding_box());
        }
        usvg::Node::Text(t) => {
            icon_union_rects_into(acc, t.abs_bounding_box());
            icon_union_rects_into(acc, t.abs_stroke_bounding_box());
        }
    }
}

#[cfg(not(all(target_arch = "wasm32", target_env = "p2")))]
/// 📐️ Union of visible paint bounds (paths, raster images, text) in absolute SVG space for uniform scale-and-center fits.
pub(super) fn svg_icon_content_bounds(tree: &usvg::Tree) -> (f64, f64, f64, f64) {
    let mut acc = None::<(f64, f64, f64, f64)>;
    for c in tree.root().children() {
        icon_visit_node_bounds(c, &mut acc);
    }
    if let Some(u) = acc {
        let (_, _, bw, bh) = u;
        if bw > 1e-6 && bh > 1e-6 {
            return u;
        }
    }
    let root = tree.root();
    let mut u = icon_rect_nonzero(root.abs_layer_bounding_box());
    if let Some(r) = icon_rect_xywh(root.abs_stroke_bounding_box()) {
        u = icon_union_xywh(u, r);
    }
    if let Some(r) = icon_rect_xywh(root.abs_bounding_box()) {
        u = icon_union_xywh(u, r);
    }
    let (_, _, bw, bh) = u;
    if bw > 1e-6 && bh > 1e-6 {
        return u;
    }
    let w = f64::from(tree.size().width());
    let h = f64::from(tree.size().height());
    (0.0, 0.0, w.max(1.0), h.max(1.0))
}

#[cfg(not(all(target_arch = "wasm32", target_env = "p2")))]
pub(super) fn render_svg_tree_themed(scene: &mut Scene, tree: &usvg::Tree, fg: Color, bg: Color) {
    render_group(scene, tree.root(), fg, bg, false);
}

#[cfg(not(all(target_arch = "wasm32", target_env = "p2")))]
/// 🎨️ Parses SVG source and renders it themed into `scene`.
pub fn append_svg_str_themed(scene: &mut Scene, svg: &str, fg: Color, bg: Color) -> Result<(), super::CanvasError> {
    let tree = usvg::Tree::from_str(svg, usvg_options_icons()).map_err(|e| super::CanvasError::SvgParse(e.to_string()))?;
    render_svg_tree_themed(scene, &tree, fg, bg);
    Ok(())
}

#[cfg(not(all(target_arch = "wasm32", target_env = "p2")))]
/// ➕️ Parses SVG source and renders it with the default icon theme into `scene`.
pub fn append_svg_str(scene: &mut Scene, svg: &str) -> Result<(), super::CanvasError> {
    append_svg_str_themed(scene, svg, super::theme::default_icon_fg(), super::theme::default_icon_bg())
}

/// 📏️ Parses SVG and returns visible content bounds in absolute SVG space.
#[cfg(not(all(target_arch = "wasm32", target_env = "p2")))]
pub fn svg_icon_content_bounds_from_str(svg: &str) -> Result<(f64, f64, f64, f64), super::CanvasError> {
    let tree = usvg::Tree::from_str(svg, usvg_options_icons()).map_err(|e| super::CanvasError::SvgParse(e.to_string()))?;
    Ok(svg_icon_content_bounds(&tree))
}

/// 🔲️ `wasm32-wasip2` arm: declared `width`/`height`/`viewBox` box
/// (`semio-framework-intrinsic-size`), NOT the painted-ink content bbox the native/browser
/// arm above computes via `usvg` — a disclosed, intentional behavior difference (no `usvg`
/// renderer is linked on this target). `x`/`y` are always `0.0`: a declared box has no
/// painted-content offset the way a bounding box of rendered ink can. Its only caller is
/// `preview_media_natural_size`'s dimension query (never icon painting — see
/// `IconPaintCache::get_or_build`'s own `wasm32-wasip2` arm). Ticket
/// `26/09/01/RUNTIME-DEPENDENCY-ELIMINATION-FOR-S-PLUGINS-AND-ARTIFACTS`.
#[cfg(all(target_arch = "wasm32", target_env = "p2"))]
pub fn svg_icon_content_bounds_from_str(svg: &str) -> Result<(f64, f64, f64, f64), super::CanvasError> {
    let (w, h) = super::intrinsic_size::svg_intrinsic_size(svg).map_err(|e| super::CanvasError::SvgParse(e.to_string()))?;
    Ok((0.0, 0.0, w, h))
}
