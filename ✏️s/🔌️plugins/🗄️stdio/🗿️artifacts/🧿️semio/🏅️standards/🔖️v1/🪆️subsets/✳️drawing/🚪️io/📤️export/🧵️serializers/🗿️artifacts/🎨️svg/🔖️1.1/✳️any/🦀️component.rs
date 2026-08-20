//! 📤️ `s.stdio.semio/v1/drawing` → `svg` (1.1) — mirrors the import leaf: `DrawNode::Path`
//! always lowers to an SVG `<path>` (absolute commands only — a documented normal form, see the
//! import leaf's module doc), `Group` composes its `SemioTransform` back into one SVG
//! `matrix(...)`, `Text` becomes a `<text>` with one child text node, and `Image` becomes this
//! bridge's own `Unknown{name:"image", href: data URI}` convention (round-trips with the import
//! leaf, real bytes embedded, not a placeholder).
//!
//! Honest lossy points (documented): only `layers[0]`'s tree is exported flattened straight into
//! `<svg>` (SVG 1.1 has no first-class "layer" concept — every layer's content is written as a
//! sibling `<g id="layer-<id>">`, so MULTIPLE layers DO survive, just not as anything SVG itself
//! calls a layer); colors are emitted as `rgba(r,g,b,a)` (matches the import leaf's own parser).

use crate::artifacts::semio::standards::v1::subsets::any::schema::geometry::{SemioRgba, SemioTransform};
use crate::artifacts::semio::standards::v1::subsets::drawing::schema::snapshot::{DrawNode, DrawStyle, PathSegment, SemioDrawingSnapshot};
use crate::artifacts::svg::{
    schema::snapshot::{svg_element_to_xml_node, CommonAttrs, Matrix2D, PathCommand, PresentationAttrs, SvgElement, TransformOp, ViewBox},
    SvgSnapshot,
};
use crate::artifacts::xml::schema::snapshot::{XmlAttr, XmlDocument};
use semio_framework_plugin::{ArtifactSerializer, Dialect, StandardId, SubsetId};

const FROM_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.semio", standard: StandardId("v1"), subset: SubsetId("drawing") };
const INTO_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.svg", standard: StandardId("1.1"), subset: SubsetId::ANY };

//#region 🔖️PathBuild
async fn segments_to_commands(segs: &[PathSegment]) -> Vec<PathCommand> {
    segs.iter()
        .map(|s| match *s {
            PathSegment::MoveTo { to } => PathCommand::MoveTo { x: to.x, y: to.y, relative: false },
            PathSegment::LineTo { to } => PathCommand::LineTo { x: to.x, y: to.y, relative: false },
            PathSegment::CubicTo { c1, c2, to } => PathCommand::CurveTo { x1: c1.x, y1: c1.y, x2: c2.x, y2: c2.y, x: to.x, y: to.y, relative: false },
            PathSegment::QuadTo { c, to } => PathCommand::QuadraticCurveTo { x1: c.x, y1: c.y, x: to.x, y: to.y, relative: false },
            PathSegment::ArcTo { rx, ry, x_rotation, large_arc, sweep, to } => PathCommand::Arc { rx, ry, x_axis_rotation: x_rotation, large_arc, sweep, x: to.x, y: to.y, relative: false },
            PathSegment::Close => PathCommand::ClosePath,
        })
        .collect()
}
//#endregion 🔖️PathBuild

//#region 🔖️TransformCompose
async fn semio_transform_to_matrix(t: &SemioTransform) -> Matrix2D {
    let theta = 2.0 * t.rotation.z.atan2(t.rotation.w);
    let (sin, cos) = theta.sin_cos();
    Matrix2D { a: cos * t.scale.x, b: sin * t.scale.x, c: -sin * t.scale.y, d: cos * t.scale.y, e: t.translation.x, f: t.translation.y }
}
//#endregion 🔖️TransformCompose

//#region 🔖️Style
async fn color_to_css(c: &SemioRgba) -> String {
    format!("rgba({},{},{},{})", (c.r * 255.0).round(), (c.g * 255.0).round(), (c.b * 255.0).round(), c.a)
}

async fn style_to_common(style_name: Option<&str>, styles: &[DrawStyle]) -> CommonAttrs {
    let mut common = CommonAttrs::default();
    if let Some(name) = style_name {
        if let Some(s) = styles.iter().find(|s| s.name == name) {
            let mut p = PresentationAttrs::default();
            if let Some(fill) = &s.fill {
                p.fill = Some(color_to_css(fill).await);
            }
            if let Some(stroke) = &s.stroke {
                p.stroke = Some(color_to_css(stroke).await);
            }
            if let Some(sw) = s.stroke_width {
                p.stroke_width = Some(sw.to_string());
            }
            if let Some(op) = s.opacity {
                p.opacity = Some(op.to_string());
            }
            common.presentation = p;
        }
    }
    common
}
//#endregion 🔖️Style

//#region 🔖️NodeBuild
const IMAGE_DATA_URI_ALPHABET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";

/// 🔤️ Minimal, dependency-free base64 encoder (mirror of the import leaf's decoder) — same
/// no-external-libraries rule.
async fn base64_encode(bytes: &[u8]) -> String {
    let mut out = String::with_capacity((bytes.len() + 2) / 3 * 4);
    for chunk in bytes.chunks(3) {
        let b0 = chunk[0];
        let b1 = *chunk.get(1).unwrap_or(&0);
        let b2 = *chunk.get(2).unwrap_or(&0);
        let n = ((b0 as u32) << 16) | ((b1 as u32) << 8) | b2 as u32;
        out.push(IMAGE_DATA_URI_ALPHABET[(n >> 18 & 0x3f) as usize] as char);
        out.push(IMAGE_DATA_URI_ALPHABET[(n >> 12 & 0x3f) as usize] as char);
        out.push(if chunk.len() > 1 { IMAGE_DATA_URI_ALPHABET[(n >> 6 & 0x3f) as usize] as char } else { '=' });
        out.push(if chunk.len() > 2 { IMAGE_DATA_URI_ALPHABET[(n & 0x3f) as usize] as char } else { '=' });
    }
    out
}

async fn svg_element_from_draw_node(node: &DrawNode, styles: &[DrawStyle]) -> SvgElement {
    match node {
        DrawNode::Path { segments, style } => SvgElement::Path { common: style_to_common(style.as_deref(), styles).await, d: segments_to_commands(segments).await },
        DrawNode::Text { value, at, style } => SvgElement::Text { common: style_to_common(style.as_deref(), styles).await, x: Some(at.x), y: Some(at.y), children: vec![SvgElement::TextNode(value.clone())] },
        DrawNode::Group { transform, children } => {
            let m = semio_transform_to_matrix(transform).await;
            SvgElement::Group {
                common: CommonAttrs { transform: Some(vec![TransformOp::Matrix { a: m.a, b: m.b, c: m.c, d: m.d, e: m.e, f: m.f }]), ..Default::default() },
                children: children.iter().map(|c| svg_element_from_draw_node(c, styles)).collect(),
            }
        }
        DrawNode::Image { at, width, height, mime, bytes } => SvgElement::Unknown {
            name: "image".into(),
            attrs: vec![
                XmlAttr { name: "x".into(), value: at.x.to_string() },
                XmlAttr { name: "y".into(), value: at.y.to_string() },
                XmlAttr { name: "width".into(), value: width.to_string() },
                XmlAttr { name: "height".into(), value: height.to_string() },
                XmlAttr { name: "href".into(), value: format!("data:{mime};base64,{}", base64_encode(bytes)) },
            ],
            children: vec![],
        },
    }
}
//#endregion 🔖️NodeBuild

//#region 🔖️Serializer
pub struct SemioDrawingToSvg;

impl ArtifactSerializer for SemioDrawingToSvg {
    type From = SemioDrawingSnapshot;
    type Into = SvgSnapshot;
    const FROM: Dialect = FROM_DIALECT;
    const INTO: Dialect = INTO_DIALECT;

    async fn serialize(from: &Self::From) -> Result<Self::Into, store::PackError> {
        let layer_groups: Vec<SvgElement> = from
            .layers
            .iter()
            .map(|layer| SvgElement::Group {
                common: CommonAttrs { id: Some(format!("layer-{}", layer.id)), ..Default::default() },
                children: match &layer.root {
                    DrawNode::Group { children, .. } => children.iter().map(|c| svg_element_from_draw_node(c, &from.styles)).collect(),
                    other => vec![svg_element_from_draw_node(other, &from.styles)],
                },
            })
            .collect();
        let root = SvgElement::Svg {
            common: CommonAttrs::default(),
            view_box: Some(ViewBox { min_x: 0.0, min_y: 0.0, width: from.canvas.width, height: from.canvas.height }),
            width: Some(from.canvas.width.to_string()),
            height: Some(from.canvas.height.to_string()),
            xmlns: Some("http://www.w3.org/2000/svg".into()),
            children: layer_groups,
        };
        Ok(SvgSnapshot { schema: crate::artifacts::svg::STDIO_SVG_DOCUMENT_SCHEMA.into(), doc: XmlDocument { root: Some(svg_element_to_xml_node(&root).await), doctype: None, declaration: None, prolog: Vec::new() } })
    }
}
//#endregion 🔖️Serializer

//#region 🔖️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::artifacts::semio::standards::v1::subsets::any::schema::geometry::SemioPoint2;
    use crate::artifacts::semio::standards::v1::subsets::drawing::schema::snapshot::{DrawCanvas, DrawLayer};

    async fn sample_drawing() -> SemioDrawingSnapshot {
        SemioDrawingSnapshot {
            canvas: DrawCanvas { width: 100.0, height: 50.0, background: None },
            styles: vec![DrawStyle { name: "s0".into(), fill: Some(SemioRgba { r: 1.0, g: 0.0, b: 0.0, a: 1.0 }), stroke: None, stroke_width: None, opacity: None }],
            layers: vec![DrawLayer {
                id: "0".into(),
                name: "root".into(),
                visible: true,
                root: DrawNode::Group {
                    transform: SemioTransform::identity(),
                    children: vec![
                        DrawNode::Path { segments: vec![PathSegment::MoveTo { to: SemioPoint2 { x: 0.0, y: 0.0 } }, PathSegment::LineTo { to: SemioPoint2 { x: 10.0, y: 10.0 } }, PathSegment::Close], style: Some("s0".into()) },
                        DrawNode::Image { at: SemioPoint2 { x: 1.0, y: 2.0 }, width: 3.0, height: 4.0, mime: "image/png".into(), bytes: vec![1, 2, 3, 4, 5] },
                    ],
                },
            }],
            ..SemioDrawingSnapshot::default()
        }
    }

    /// 🧪️ Real round trip through svg's own real XML text codec.
    #[semio_framework_async_macros::async_test]
    async fn real_text_round_trip_through_svg_codec() {
        let drawing = sample_drawing();
        let svg = semio_framework_plugin::resolve_ready(SemioDrawingToSvg::serialize(&drawing)).expect("serialize");
        let text = <SvgSnapshot as store::ArtifactDsl>::print_dsl(&svg);
        let reparsed = <SvgSnapshot as store::ArtifactDsl>::parse_dsl(&text).expect("reparse real svg text");
        let root = crate::artifacts::svg::schema::snapshot::svg_element_from_xml_node(reparsed.doc.root.as_ref().unwrap()).expect("typed view");
        match &root {
            SvgElement::Svg { view_box, .. } => assert_eq!(*view_box, Some(ViewBox { min_x: 0.0, min_y: 0.0, width: 100.0, height: 50.0 })),
            other => panic!("expected <svg>, got {other:?}"),
        }
    }

    #[semio_framework_async_macros::async_test]
    async fn image_node_round_trips_through_data_uri_convention() {
        let drawing = sample_drawing();
        let svg = semio_framework_plugin::resolve_ready(SemioDrawingToSvg::serialize(&drawing)).expect("serialize");
        let root = crate::artifacts::svg::schema::snapshot::svg_element_from_xml_node(svg.doc.root.as_ref().unwrap()).expect("typed view");
        let layer_group = match &root {
            SvgElement::Svg { children, .. } => &children[0],
            _ => panic!("expected svg root"),
        };
        let image_el = match layer_group {
            SvgElement::Group { children, .. } => &children[1],
            _ => panic!("expected layer group"),
        };
        match image_el {
            SvgElement::Unknown { name, attrs, .. } => {
                assert_eq!(name, "image");
                let href = attrs.iter().find(|a| a.name == "href").expect("href attr");
                assert!(href.value.starts_with("data:image/png;base64,"));
            }
            other => panic!("expected Unknown(image), got {other:?}"),
        }
    }
}
//#endregion 🔖️Tests
