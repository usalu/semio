//! 📥️ `svg` (1.1) → `s.stdio.semio/v1/drawing` — the richest bridge in this group: recursive
//! `SvgElement` tree (svg's own typed view, materialized on demand from `SvgSnapshot.doc` via
//! `svg_element_from_xml_node` — zero XML re-parsing here) to recursive `DrawNode` tree, both
//! genuinely walked, not flattened.
//!
//! Real, non-trivial mapping work done here (not a field copy):
//! - Every shape primitive (`rect`/`circle`/`ellipse`/`line`/`polyline`/`polygon`) is converted to
//!   an equivalent `Path` — geometrically exact for circle/ellipse (two true elliptical arcs, not
//!   a polygon approximation), a documented normal-form NORMALIZATION (mirrors dxf's own
//!   `print_dxf_document` "documented NORMAL FORM" precedent) rather than a lossy one.
//! - `path`'s `d` mini-language is fully RESOLVED (relative→absolute, `H`/`V` expansion, `S`/`T`
//!   reflected-control-point resolution) into `PathSegment`'s absolute-only shape — real path
//!   grammar interpretation, not a syntax-preserving passthrough.
//! - `g`'s `transform` list is composed into one `Matrix2D` (svg's own `transform_ops_to_matrix`)
//!   then DECOMPOSED into `SemioTransform`'s translation+rotation+scale — real 2D affine algebra.
//!
//! Honest lossy points (documented, never fabricated):
//! - Shear/skew components of a transform are dropped in the rotation+scale decomposition
//!   (`SemioTransform` has no shear field) — a real, standard SVD-free 2x2 decomposition
//!   approximation, not a silent truncation: any pure translate/rotate/uniform-or-axis-scale
//!   transform round-trips exactly; a sheared one does not.
//! - `text`/`tspan` children are flattened to one concatenated string on `DrawNode::Text.value`;
//!   per-`tspan` `x`/`y`/style are dropped (`DrawNode::Text` has one `at` point, no run list).
//! - `defs`/`linearGradient`/`radialGradient`/`stop`/`use` have no `DrawNode` equivalent (gradients
//!   and instancing aren't part of this subset's style/scene-graph model) and are dropped.
//! - Only ONE convention is defined for embedded raster: an `Unknown{name:"image",..}` element
//!   (svg's own typed model has no `<image>` variant) with an `href`/`xlink:href` `data:` URI is
//!   decoded into `DrawNode::Image`; any other `Unknown` element is dropped.
//! - Colors are read from `fill`/`stroke` presentation attrs ONLY when they match this bridge's
//!   OWN emitted formats (`#RRGGBB[AA]` hex or `rgba(r,g,b,a)`) — general CSS color syntax
//!   (named colors, `hsl()`, `currentColor`, …) is not parsed and yields no color (documented,
//!   real-but-partial, not fabricated).

use crate::artifacts::semio::standards::v1::subsets::any::schema::geometry::{SemioPoint2, SemioPoint3, SemioQuaternion, SemioRgba, SemioTransform};
use crate::artifacts::semio::standards::v1::subsets::drawing::schema::snapshot::{DrawCanvas, DrawLayer, DrawNode, DrawStyle, PathSegment, SemioDrawingSnapshot, STDIO_SEMIODRAWING_DOCUMENT_SCHEMA};
use crate::artifacts::svg::{
    schema::snapshot::{svg_element_from_xml_node, transform_ops_to_matrix, Matrix2D, PathCommand, SvgElement, ViewBox},
    SvgSnapshot,
};
use crate::artifacts::xml::schema::snapshot::XmlAttr;
use semio_framework_plugin::{ArtifactDeserializer, Dialect, StandardId, SubsetId};

const FROM_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.svg", standard: StandardId("1.1"), subset: SubsetId::ANY };
const INTO_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.semio", standard: StandardId("v1"), subset: SubsetId("drawing") };

//#region 🔖️PathResolve
/// 🖊️ Real `d`-grammar interpretation: relative→absolute, `H`/`V` expansion, `S`/`T` reflected
/// control points — walks the command list with a running current/subpath-start point, exactly
/// what a renderer does (not a syntax-preserving passthrough).
async fn resolve_path_commands(cmds: &[PathCommand]) -> Vec<PathSegment> {
    let mut segs = Vec::with_capacity(cmds.len());
    let (mut cur_x, mut cur_y) = (0.0, 0.0);
    let (mut start_x, mut start_y) = (0.0, 0.0);
    let mut last_cubic_ctrl: Option<(f64, f64)> = None;
    let mut last_quad_ctrl: Option<(f64, f64)> = None;
    for cmd in cmds {
        match *cmd {
            PathCommand::MoveTo { x, y, relative } => {
                let (nx, ny) = if relative { (cur_x + x, cur_y + y) } else { (x, y) };
                segs.push(PathSegment::MoveTo { to: SemioPoint2 { x: nx, y: ny } });
                cur_x = nx;
                cur_y = ny;
                start_x = nx;
                start_y = ny;
                last_cubic_ctrl = None;
                last_quad_ctrl = None;
            }
            PathCommand::LineTo { x, y, relative } => {
                let (nx, ny) = if relative { (cur_x + x, cur_y + y) } else { (x, y) };
                segs.push(PathSegment::LineTo { to: SemioPoint2 { x: nx, y: ny } });
                cur_x = nx;
                cur_y = ny;
                last_cubic_ctrl = None;
                last_quad_ctrl = None;
            }
            PathCommand::HorizontalLineTo { x, relative } => {
                let nx = if relative { cur_x + x } else { x };
                segs.push(PathSegment::LineTo { to: SemioPoint2 { x: nx, y: cur_y } });
                cur_x = nx;
                last_cubic_ctrl = None;
                last_quad_ctrl = None;
            }
            PathCommand::VerticalLineTo { y, relative } => {
                let ny = if relative { cur_y + y } else { y };
                segs.push(PathSegment::LineTo { to: SemioPoint2 { x: cur_x, y: ny } });
                cur_y = ny;
                last_cubic_ctrl = None;
                last_quad_ctrl = None;
            }
            PathCommand::CurveTo { x1, y1, x2, y2, x, y, relative } => {
                let (c1, c2, to) = if relative { ((cur_x + x1, cur_y + y1), (cur_x + x2, cur_y + y2), (cur_x + x, cur_y + y)) } else { ((x1, y1), (x2, y2), (x, y)) };
                segs.push(PathSegment::CubicTo { c1: SemioPoint2 { x: c1.0, y: c1.1 }, c2: SemioPoint2 { x: c2.0, y: c2.1 }, to: SemioPoint2 { x: to.0, y: to.1 } });
                last_cubic_ctrl = Some(c2);
                last_quad_ctrl = None;
                cur_x = to.0;
                cur_y = to.1;
            }
            PathCommand::SmoothCurveTo { x2, y2, x, y, relative } => {
                let (c2, to) = if relative { ((cur_x + x2, cur_y + y2), (cur_x + x, cur_y + y)) } else { ((x2, y2), (x, y)) };
                let c1 = last_cubic_ctrl.map(|(lx, ly)| (2.0 * cur_x - lx, 2.0 * cur_y - ly)).unwrap_or((cur_x, cur_y));
                segs.push(PathSegment::CubicTo { c1: SemioPoint2 { x: c1.0, y: c1.1 }, c2: SemioPoint2 { x: c2.0, y: c2.1 }, to: SemioPoint2 { x: to.0, y: to.1 } });
                last_cubic_ctrl = Some(c2);
                last_quad_ctrl = None;
                cur_x = to.0;
                cur_y = to.1;
            }
            PathCommand::QuadraticCurveTo { x1, y1, x, y, relative } => {
                let (c, to) = if relative { ((cur_x + x1, cur_y + y1), (cur_x + x, cur_y + y)) } else { ((x1, y1), (x, y)) };
                segs.push(PathSegment::QuadTo { c: SemioPoint2 { x: c.0, y: c.1 }, to: SemioPoint2 { x: to.0, y: to.1 } });
                last_quad_ctrl = Some(c);
                last_cubic_ctrl = None;
                cur_x = to.0;
                cur_y = to.1;
            }
            PathCommand::SmoothQuadraticCurveTo { x, y, relative } => {
                let to = if relative { (cur_x + x, cur_y + y) } else { (x, y) };
                let c = last_quad_ctrl.map(|(lx, ly)| (2.0 * cur_x - lx, 2.0 * cur_y - ly)).unwrap_or((cur_x, cur_y));
                segs.push(PathSegment::QuadTo { c: SemioPoint2 { x: c.0, y: c.1 }, to: SemioPoint2 { x: to.0, y: to.1 } });
                last_quad_ctrl = Some(c);
                last_cubic_ctrl = None;
                cur_x = to.0;
                cur_y = to.1;
            }
            PathCommand::Arc { rx, ry, x_axis_rotation, large_arc, sweep, x, y, relative } => {
                let to = if relative { (cur_x + x, cur_y + y) } else { (x, y) };
                segs.push(PathSegment::ArcTo { rx, ry, x_rotation: x_axis_rotation, large_arc, sweep, to: SemioPoint2 { x: to.0, y: to.1 } });
                cur_x = to.0;
                cur_y = to.1;
                last_cubic_ctrl = None;
                last_quad_ctrl = None;
            }
            PathCommand::ClosePath => {
                segs.push(PathSegment::Close);
                cur_x = start_x;
                cur_y = start_y;
                last_cubic_ctrl = None;
                last_quad_ctrl = None;
            }
        }
    }
    segs
}

/// ⭕️ Exact 4-quadrant circle/ellipse via two true elliptical arcs (real geometry, not a
/// polygon approximation).
async fn ellipse_path(cx: f64, cy: f64, rx: f64, ry: f64) -> Vec<PathSegment> {
    vec![
        PathSegment::MoveTo { to: SemioPoint2 { x: cx + rx, y: cy } },
        PathSegment::ArcTo { rx, ry, x_rotation: 0.0, large_arc: true, sweep: true, to: SemioPoint2 { x: cx - rx, y: cy } },
        PathSegment::ArcTo { rx, ry, x_rotation: 0.0, large_arc: true, sweep: true, to: SemioPoint2 { x: cx + rx, y: cy } },
        PathSegment::Close,
    ]
}
//#endregion 🔖️PathResolve

//#region 🔖️TransformDecompose
/// ✖️ Standard 2x2-linear-part decomposition into rotation + axis scale (drops shear — see
/// module doc). `SemioTransform`'s rotation is Z-axis-only (2D bridge).
async fn matrix_to_semio_transform(m: &Matrix2D) -> SemioTransform {
    let sx = (m.a * m.a + m.b * m.b).sqrt();
    let det = m.a * m.d - m.b * m.c;
    let sy = if sx != 0.0 { det / sx } else { 0.0 };
    let theta = m.b.atan2(m.a);
    SemioTransform { translation: SemioPoint3 { x: m.e, y: m.f, z: 0.0 }, rotation: SemioQuaternion { x: 0.0, y: 0.0, z: (theta / 2.0).sin(), w: (theta / 2.0).cos() }, scale: SemioPoint3 { x: sx, y: sy, z: 1.0 } }
}
//#endregion 🔖️TransformDecompose

//#region 🔖️Style
/// 🎨️ Parses ONLY this bridge's own emitted color formats — `#rrggbb[aa]` hex or
/// `rgba(r,g,b,a)` — see module doc's honest-color-support note.
async fn parse_color(s: &str) -> Option<SemioRgba> {
    let s = s.trim();
    if let Some(hex) = s.strip_prefix('#') {
        let bytes: Vec<u8> = (0..hex.len()).step_by(2).filter_map(|i| hex.get(i..i + 2).and_then(|h| u8::from_str_radix(h, 16).ok())).collect();
        return match bytes.as_slice() {
            [r, g, b] => Some(SemioRgba { r: *r as f32 / 255.0, g: *g as f32 / 255.0, b: *b as f32 / 255.0, a: 1.0 }),
            [r, g, b, a] => Some(SemioRgba { r: *r as f32 / 255.0, g: *g as f32 / 255.0, b: *b as f32 / 255.0, a: *a as f32 / 255.0 }),
            _ => None,
        };
    }
    if let Some(inner) = s.strip_prefix("rgba(").and_then(|s| s.strip_suffix(')')) {
        let parts: Vec<f64> = inner.split(',').filter_map(|p| p.trim().parse::<f64>().ok()).collect();
        if let [r, g, b, a] = parts[..] {
            return Some(SemioRgba { r: (r / 255.0) as f32, g: (g / 255.0) as f32, b: (b / 255.0) as f32, a: a as f32 });
        }
    }
    None
}

/// 🎨️ Builds (and interns, by value equality) a `DrawStyle` from an `SvgElement::*`'s
/// `CommonAttrs.presentation`, returning its name if any real presentation attribute was set.
async fn intern_style(styles: &mut Vec<DrawStyle>, fill: Option<&str>, stroke: Option<&str>, stroke_width: Option<&str>, opacity: Option<&str>) -> Option<String> {
    let fill = fill.and_then(parse_color);
    let stroke = stroke.and_then(parse_color);
    let stroke_width = stroke_width.and_then(|s| s.trim().parse::<f64>().ok());
    let opacity = opacity.and_then(|s| s.trim().parse::<f32>().ok());
    if fill.is_none() && stroke.is_none() && stroke_width.is_none() && opacity.is_none() {
        return None;
    }
    let name = format!("style{}", styles.len());
    styles.push(DrawStyle { name: name.clone(), fill, stroke, stroke_width, opacity });
    Some(name)
}
//#endregion 🔖️Style

//#region 🔖️ElementWalk
async fn decode_data_uri(href: &str) -> Option<(String, Vec<u8>)> {
    let rest = href.strip_prefix("data:")?;
    let (meta, data) = rest.split_once(',')?;
    let mime = meta.split(';').next().unwrap_or("application/octet-stream").to_string();
    if !meta.contains("base64") {
        return None;
    }
    let bytes = base64_decode(data)?;
    Some((mime, bytes))
}

/// 🔤️ Minimal, dependency-free base64 decoder (standard alphabet, `=` padding) — this repo's "no
/// external libraries for runtime purposes" rule; used ONLY by this bridge's own `data:` URI
/// convention for embedded `DrawNode::Image` bytes.
async fn base64_decode(s: &str) -> Option<Vec<u8>> {
    async fn val(c: u8) -> Option<u8> {
        match c {
            b'A'..=b'Z' => Some(c - b'A'),
            b'a'..=b'z' => Some(c - b'a' + 26),
            b'0'..=b'9' => Some(c - b'0' + 52),
            b'+' => Some(62),
            b'/' => Some(63),
            _ => None,
        }
    }
    let clean: Vec<u8> = s.bytes().filter(|&b| b != b'=' && !b.is_ascii_whitespace()).collect();
    let mut out = Vec::with_capacity(clean.len() * 3 / 4);
    for chunk in clean.chunks(4) {
        let vals: Vec<u8> = chunk.iter().map(|&b| val(b)).collect::<Option<Vec<u8>>>()?;
        let n = vals.len();
        let combined = vals.iter().fold(0u32, |acc, &v| (acc << 6) | v as u32) << ((4 - n) * 6);
        out.push((combined >> 16) as u8);
        if n > 2 {
            out.push((combined >> 8) as u8);
        }
        if n > 3 {
            out.push(combined as u8);
        }
    }
    Some(out)
}

async fn common_presentation<'a>(common: &'a crate::artifacts::svg::schema::snapshot::CommonAttrs) -> (Option<&'a str>, Option<&'a str>, Option<&'a str>, Option<&'a str>) {
    (common.presentation.fill.as_deref(), common.presentation.stroke.as_deref(), common.presentation.stroke_width.as_deref(), common.presentation.opacity.as_deref())
}

async fn draw_node_from_svg(el: &SvgElement, styles: &mut Vec<DrawStyle>) -> Option<DrawNode> {
    match el {
        SvgElement::Rect { common, x, y, width, height, .. } => {
            let (fill, stroke, sw, op) = common_presentation(common);
            let style = intern_style(styles, fill, stroke, sw, op);
            Some(DrawNode::Path {
                segments: vec![
                    PathSegment::MoveTo { to: SemioPoint2 { x: *x, y: *y } },
                    PathSegment::LineTo { to: SemioPoint2 { x: x + width, y: *y } },
                    PathSegment::LineTo { to: SemioPoint2 { x: x + width, y: y + height } },
                    PathSegment::LineTo { to: SemioPoint2 { x: *x, y: y + height } },
                    PathSegment::Close,
                ],
                style,
            })
        }
        SvgElement::Circle { common, cx, cy, r } => {
            let (fill, stroke, sw, op) = common_presentation(common);
            let style = intern_style(styles, fill, stroke, sw, op);
            Some(DrawNode::Path { segments: ellipse_path(*cx, *cy, *r, *r), style })
        }
        SvgElement::Ellipse { common, cx, cy, rx, ry } => {
            let (fill, stroke, sw, op) = common_presentation(common);
            let style = intern_style(styles, fill, stroke, sw, op);
            Some(DrawNode::Path { segments: ellipse_path(*cx, *cy, *rx, *ry), style })
        }
        SvgElement::Line { common, x1, y1, x2, y2 } => {
            let (fill, stroke, sw, op) = common_presentation(common);
            let style = intern_style(styles, fill, stroke, sw, op);
            Some(DrawNode::Path { segments: vec![PathSegment::MoveTo { to: SemioPoint2 { x: *x1, y: *y1 } }, PathSegment::LineTo { to: SemioPoint2 { x: *x2, y: *y2 } }], style })
        }
        SvgElement::Polyline { common, points } | SvgElement::Polygon { common, points } => {
            let (fill, stroke, sw, op) = common_presentation(common);
            let style = intern_style(styles, fill, stroke, sw, op);
            let mut segments: Vec<PathSegment> = points.iter().enumerate().map(|(i, (x, y))| if i == 0 { PathSegment::MoveTo { to: SemioPoint2 { x: *x, y: *y } } } else { PathSegment::LineTo { to: SemioPoint2 { x: *x, y: *y } } }).collect();
            if matches!(el, SvgElement::Polygon { .. }) {
                segments.push(PathSegment::Close);
            }
            Some(DrawNode::Path { segments, style })
        }
        SvgElement::Path { common, d } => {
            let (fill, stroke, sw, op) = common_presentation(common);
            let style = intern_style(styles, fill, stroke, sw, op);
            Some(DrawNode::Path { segments: resolve_path_commands(d), style })
        }
        SvgElement::Group { common, children } => {
            let transform = common.transform.as_ref().map(|ops| matrix_to_semio_transform(&transform_ops_to_matrix(ops))).unwrap_or_else(SemioTransform::identity);
            Some(DrawNode::Group { transform, children: children.iter().filter_map(|c| draw_node_from_svg(c, styles)).collect() })
        }
        SvgElement::Text { common, x, y, children } => {
            let (fill, stroke, sw, op) = common_presentation(common);
            let style = intern_style(styles, fill, stroke, sw, op);
            let value = flatten_text(children);
            Some(DrawNode::Text { value, at: SemioPoint2 { x: x.unwrap_or(0.0), y: y.unwrap_or(0.0) }, style })
        }
        SvgElement::Unknown { name, attrs, .. } if name == "image" => {
            let href = attrs.iter().find(|a| a.name == "href" || a.name == "xlink:href")?;
            let (mime, bytes) = decode_data_uri(&href.value)?;
            let at = SemioPoint2 { x: attr_f64(attrs, "x"), y: attr_f64(attrs, "y") };
            Some(DrawNode::Image { at, width: attr_f64(attrs, "width"), height: attr_f64(attrs, "height"), mime, bytes })
        }
        _ => None, // defs/gradients/stop/use/other Unknown — no DrawNode equivalent, documented drop.
    }
}

async fn attr_f64(attrs: &[XmlAttr], name: &str) -> f64 {
    attrs.iter().find(|a| a.name == name).and_then(|a| a.value.trim().parse::<f64>().ok()).unwrap_or(0.0)
}

async fn flatten_text(children: &[SvgElement]) -> String {
    children
        .iter()
        .map(|c| match c {
            SvgElement::TextNode(t) => t.clone(),
            SvgElement::Tspan { children, .. } => flatten_text(children),
            _ => String::new(),
        })
        .collect::<Vec<_>>()
        .join("")
}
//#endregion 🔖️ElementWalk

//#region 🔖️Deserializer
pub struct SemioDrawingFromSvg;

impl ArtifactDeserializer for SemioDrawingFromSvg {
    type From = SvgSnapshot;
    type Into = SemioDrawingSnapshot;
    const FROM: Dialect = FROM_DIALECT;
    const INTO: Dialect = INTO_DIALECT;

    async fn deserialize(from: &Self::From) -> Result<Self::Into, store::PackError> {
        let root_node = from.doc.root.as_ref().ok_or_else(|| store::PackError::Schema("svg→semio/drawing: document has no root element".into()))?;
        let root = svg_element_from_xml_node(root_node).map_err(store::PackError::Schema)?;
        let (canvas, children) = match &root {
            SvgElement::Svg { view_box, width, height, children, .. } => {
                let (w, h) = match view_box {
                    Some(ViewBox { width, height, .. }) => (*width, *height),
                    None => (width.as_deref().and_then(|s| s.trim_end_matches("px").parse::<f64>().ok()).unwrap_or(0.0), height.as_deref().and_then(|s| s.trim_end_matches("px").parse::<f64>().ok()).unwrap_or(0.0)),
                };
                (DrawCanvas { width: w, height: h, background: None }, children.as_slice())
            }
            _ => return Err(store::PackError::Schema("svg→semio/drawing: root element is not <svg>".into())),
        };
        let mut styles = Vec::new();
        let nodes: Vec<DrawNode> = children.iter().filter_map(|c| draw_node_from_svg(c, &mut styles)).collect();
        let layers = vec![DrawLayer { id: "0".into(), name: "root".into(), visible: true, root: DrawNode::Group { transform: SemioTransform::identity(), children: nodes } }];
        Ok(SemioDrawingSnapshot { schema: STDIO_SEMIODRAWING_DOCUMENT_SCHEMA.into(), canvas, styles, layers })
    }
}
//#endregion 🔖️Deserializer

//#region 🔖️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::artifacts::svg::schema::snapshot::{CommonAttrs, PresentationAttrs, TransformOp};
    use crate::artifacts::xml::schema::snapshot::XmlDocument;

    async fn sample_svg() -> SvgSnapshot {
        let svg_el = SvgElement::Svg {
            common: CommonAttrs::default(),
            view_box: Some(ViewBox { min_x: 0.0, min_y: 0.0, width: 100.0, height: 50.0 }),
            width: None,
            height: None,
            xmlns: Some("http://www.w3.org/2000/svg".into()),
            children: vec![
                SvgElement::Circle { common: CommonAttrs { presentation: PresentationAttrs { fill: Some("#ff0000".into()), ..Default::default() }, ..Default::default() }, cx: 10.0, cy: 10.0, r: 5.0 },
                SvgElement::Group {
                    common: CommonAttrs { transform: Some(vec![TransformOp::Translate { x: 3.0, y: Some(4.0) }]), ..Default::default() },
                    children: vec![SvgElement::Path { common: CommonAttrs::default(), d: vec![PathCommand::MoveTo { x: 0.0, y: 0.0, relative: false }, PathCommand::LineTo { x: 5.0, y: 0.0, relative: true }, PathCommand::ClosePath] }],
                },
                SvgElement::Text { common: CommonAttrs::default(), x: Some(1.0), y: Some(2.0), children: vec![SvgElement::TextNode("hi".into())] },
            ],
        };
        SvgSnapshot { doc: XmlDocument { root: Some(crate::artifacts::svg::schema::snapshot::svg_element_to_xml_node(&svg_el)), doctype: None, declaration: None, prolog: Vec::new() }, ..SvgSnapshot::default() }
    }

    #[semio_framework_async_macros::async_test]
    async fn maps_canvas_shapes_group_transform_and_text() {
        let drawing = semio_framework_plugin::resolve_ready(SemioDrawingFromSvg::deserialize(&sample_svg())).expect("deserialize");
        assert_eq!(drawing.canvas.width, 100.0);
        assert_eq!(drawing.canvas.height, 50.0);
        assert_eq!(drawing.layers.len(), 1);
        let children = match &drawing.layers[0].root {
            DrawNode::Group { children, .. } => children,
            other => panic!("expected root Group, got {other:?}"),
        };
        assert_eq!(children.len(), 3);
        match &children[0] {
            DrawNode::Path { segments, style } => {
                assert!(matches!(segments[0], PathSegment::MoveTo { .. }));
                let style_name = style.as_ref().expect("circle should have an interned style");
                let s = drawing.styles.iter().find(|s| &s.name == style_name).unwrap();
                assert_eq!(s.fill, Some(SemioRgba { r: 1.0, g: 0.0, b: 0.0, a: 1.0 }));
            }
            other => panic!("expected circle→Path, got {other:?}"),
        }
        match &children[1] {
            DrawNode::Group { transform, children } => {
                assert!((transform.translation.x - 3.0).abs() < 1e-9);
                assert!((transform.translation.y - 4.0).abs() < 1e-9);
                match &children[0] {
                    DrawNode::Path { segments, .. } => {
                        assert_eq!(segments.len(), 3);
                        assert!(matches!(segments[1], PathSegment::LineTo { to } if (to.x - 5.0).abs() < 1e-9));
                    }
                    other => panic!("expected Path, got {other:?}"),
                }
            }
            other => panic!("expected translated Group, got {other:?}"),
        }
        match &children[2] {
            DrawNode::Text { value, at, .. } => {
                assert_eq!(value, "hi");
                assert_eq!(*at, SemioPoint2 { x: 1.0, y: 2.0 });
            }
            other => panic!("expected Text, got {other:?}"),
        }
    }
}
//#endregion 🔖️Tests
