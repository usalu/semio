//! 🚪️ IO s.draw (1/✳️any) — `io() -> IoDeclaration` (design.md §2/§3): the native codec plus every
//! foreign hop, aggregated from the typed `Serializer<DrawSnapshot>`/`Deserializer<DrawSnapshot>`
//! leaves under `📥️import/🧩️deserializers`/`📤️export/🧵️serializers`. Replaces the old hand-rolled
//! `ArtifactComposition`/`ComposerEntry` dispatch chain (`derived_composition`/`io_registry`,
//! deleted outright) — all io now goes exclusively through the `io_mechanism` registry (design.md
//! rule 3). `import_stdio_kinds`/`export_stdio_kinds` (old, zero callers even before this pass)
//! deleted alongside them.

//#region 🔖️SemioBridge
/// 🌉️ Relocated verbatim from the `⚙️engine` directory (ticket
/// 26/08/12/ENGINELESS-ARTIFACTS-AND-APP-STATE-MACHINES, rule 5: sniff/codec dispatch and
/// cross-format bridge functions live in `🚪️io/`).
use base64::{engine::general_purpose::STANDARD as BASE64, Engine as _};
use crate::artifacts::draw::{DrawSnapshot, FillStyle, PathSegment};
use crate::artifacts::draw::schema::{draw_layer_world_bounds, flatten_draw_document_to_scene_nodes, flatten_draw_layers, DrawSceneNode};
use semio_s_plugin_stdio::artifacts::semio::standards::v1::subsets::any::schema::geometry::{SemioPoint2, SemioPoint3, SemioQuaternion, SemioRgba, SemioTransform};
use semio_s_plugin_stdio::artifacts::semio::standards::v1::subsets::drawing::schema::snapshot::{
    DrawCanvas as SemioDrawCanvas, DrawLayer as SemioDrawLayer, DrawNode as SemioDrawNode, DrawStyle as SemioDrawStyle, PathSegment as SemioPathSegment, SemioDrawingSnapshot,
    STDIO_SEMIODRAWING_DOCUMENT_SCHEMA,
};
use semio_s_plugin_stdio::artifacts::svg::standards::v1_1::subsets::any::schema::snapshot::write_svg_xml;
use semio_s_plugin_stdio::artifacts::svg::SvgSnapshot;

/// 🕳️ stdio_gap: `s.stdio.semio/v1/drawing` bridges only to svg/dxf/pdf (per the master plan's
/// format lattice — dwg lives under `s.stdio.semio/v1/cad`, standard `ac1024`, a different hub
/// entirely). There is no route from `SemioDrawingSnapshot` to DWG bytes today, so this plugin's
/// former ad-hoc `draw_document_json_to_dwg_bytes`/`draw_document_json_from_dwg` pair was deleted
/// outright rather than hand-rolling DWG again — see `w5b-w-report.md` `stdio_gaps`.
const SEMIO_DRAWING_DIALECT: semio_framework::Dialect = semio_framework::Dialect { artifact_kind: "s.stdio.semio", standard: semio_framework::StandardId("v1"), subset: semio_framework::SubsetId("drawing") };
const SVG_DIALECT: semio_framework::Dialect = semio_framework::Dialect { artifact_kind: "s.stdio.svg", standard: semio_framework::StandardId("1.1"), subset: semio_framework::SubsetId::ANY };

/// 📌️ W5b-close fix: registers stdio's semio/drawing subset composer (svg/dxf/pdf io entries)
/// into the process-global `io` registry exactly once, so `io_dispatch` below resolves the
/// drawing→svg bridge regardless of host-boot ordering — a bare `cargo test` process never runs
/// the plugin-host boot path that would normally call this. Mirrors 🗒️note's/📏️layout's/🌍️gis's
/// own `ensure_..._registered()` helper (w5b-verify-report.md §6b flagged draw as the one sibling
/// that had not added this, causing `draw_document_to_svg_bridges_shape_text_image_and_gradient_nodes_through_semio_drawing`
/// and `draw_io_declares_vector_out_and_export_media_covers_both_ports` to fail with "no composer
/// registered").
async fn ensure_semio_drawing_bridge_registered() {
    static ONCE: std::sync::Once = std::sync::Once::new();
    ONCE.call_once(semio_s_plugin_stdio::artifacts::semio::standards::v1::subsets::drawing::io::register);
}

async fn resolve_draw_document_artboard(doc: &DrawSnapshot) -> (u32, u32) {
    if let Some(artboard) = &doc.artboard {
        return (artboard.width.max(1.0).round() as u32, artboard.height.max(1.0).round() as u32);
    }
    let mut max_x: f64 = 1024.0;
    let mut max_y: f64 = 1024.0;
    for layer in flatten_draw_layers(&doc.layers) {
        if let Some((x, y, width, height)) = draw_layer_world_bounds(layer) {
            max_x = max_x.max(x + width);
            max_y = max_y.max(y + height);
        }
    }
    (max_x.max(1.0).round() as u32, max_y.max(1.0).round() as u32)
}

/// 🌉️ [DrawTransform]'s 6-value affine matrix → semio's [SemioTransform] (Z-only rotation
/// quaternion, axis scale, zero-z translation) — the same decomposition stdio's own svg↔drawing
/// bridge applies on its side (`matrix_to_semio_transform` in that leaf).
async fn matrix_to_semio_transform(matrix: [f64; 6]) -> SemioTransform {
    let transform = crate::artifacts::draw::schema::draw_matrix_to_transform(matrix);
    SemioTransform {
        translation: SemioPoint3 { x: transform.x, y: transform.y, z: 0.0 },
        rotation: SemioQuaternion { x: 0.0, y: 0.0, z: (transform.rotation / 2.0).sin(), w: (transform.rotation / 2.0).cos() },
        scale: SemioPoint3 { x: transform.scale_x, y: transform.scale_y, z: 1.0 },
    }
}

/// ✏️ Draw's own [PathSegment] → semio's [SemioPathSegment] — same SVG-command grammar, field
/// renames only (no geometry recomputed).
async fn to_semio_path_segment(segment: &PathSegment) -> SemioPathSegment {
    match *segment {
        PathSegment::Move { to } => SemioPathSegment::MoveTo { to: SemioPoint2 { x: to[0], y: to[1] } },
        PathSegment::Line { to } => SemioPathSegment::LineTo { to: SemioPoint2 { x: to[0], y: to[1] } },
        PathSegment::Quad { ctrl, to } => SemioPathSegment::QuadTo { c: SemioPoint2 { x: ctrl[0], y: ctrl[1] }, to: SemioPoint2 { x: to[0], y: to[1] } },
        PathSegment::Cubic { ctrl1, ctrl2, to } => SemioPathSegment::CubicTo { c1: SemioPoint2 { x: ctrl1[0], y: ctrl1[1] }, c2: SemioPoint2 { x: ctrl2[0], y: ctrl2[1] }, to: SemioPoint2 { x: to[0], y: to[1] } },
        PathSegment::Arc { rx, ry, rotation, large_arc, sweep, to } => SemioPathSegment::ArcTo { rx, ry, x_rotation: rotation, large_arc, sweep, to: SemioPoint2 { x: to[0], y: to[1] } },
        PathSegment::Close => SemioPathSegment::Close,
    }
}

/// 🎨️ [FillStyle::Solid]/[StrokeStyle] → [SemioRgba] — `DrawStyle` is solid-color-only, so
/// gradients have no representable equivalent and are honestly dropped (matching the pre-migration
/// SVG renderer's own gradient fallback: no fill, not a fabricated flat color).
async fn solid_fill_to_semio_rgba(fill: &FillStyle) -> Option<SemioRgba> {
    match fill {
        FillStyle::Solid { color } => Some(SemioRgba { r: color[0] as f32, g: color[1] as f32, b: color[2] as f32, a: color[3] as f32 }),
        FillStyle::LinearGradient { .. } | FillStyle::RadialGradient { .. } => None,
    }
}

/// 🎨️ Interns one [DrawSceneNode]'s fill/stroke/opacity as a named [SemioDrawStyle] and returns
/// its name, or `None` when the node carries no representable presentation at all. 🕳️ stdio_gap:
/// `blend_mode`/`fill_rule` have no `DrawStyle` field and `Group`/`Image` nodes have no opacity
/// slot at all (only `Path`/`Text` reference a style) — both honestly dropped, not fabricated.
async fn intern_semio_style(styles: &mut Vec<SemioDrawStyle>, node: &DrawSceneNode) -> Option<String> {
    let fill = node.fill.as_ref().and_then(solid_fill_to_semio_rgba);
    let stroke = node.stroke.as_ref().map(|style| SemioRgba { r: style.color[0] as f32, g: style.color[1] as f32, b: style.color[2] as f32, a: style.color[3] as f32 });
    let stroke_width = node.stroke.as_ref().map(|style| style.width);
    let opacity = if (node.opacity - 1.0).abs() > f64::EPSILON { Some(node.opacity as f32) } else { None };
    if fill.is_none() && stroke.is_none() && opacity.is_none() {
        return None;
    }
    let name = format!("style{}", styles.len());
    styles.push(SemioDrawStyle { name: name.clone(), fill, stroke, stroke_width, opacity });
    Some(name)
}

/// 🖼️ Decodes one `data:<mime>;base64,<data>` URI (as built by
/// [flatten_draw_document_to_scene_nodes] for image scene nodes) into real mime + bytes.
async fn decode_data_uri_bytes(uri: &str) -> Option<(String, Vec<u8>)> {
    let rest = uri.strip_prefix("data:")?;
    let (meta, data) = rest.split_once(',')?;
    let mime = meta.split(';').next().unwrap_or("application/octet-stream").to_string();
    let bytes = BASE64.decode(data).ok()?;
    Some((mime, bytes))
}

/// 🖍️ One [DrawSceneNode] → semio's recursive [SemioDrawNode]: each becomes its own `Group`
/// carrying the node's baked world transform, wrapping exactly one Path/Text/Image leaf (mirrors
/// the pre-migration SVG renderer's own `<g transform="matrix(...)"><path/></g>` shape).
async fn semio_draw_node_from_scene_node(node: &DrawSceneNode, styles: &mut Vec<SemioDrawStyle>) -> Option<SemioDrawNode> {
    let style = intern_semio_style(styles, node);
    let leaf = if let Some(text) = &node.text {
        SemioDrawNode::Text { value: text.content.clone(), at: SemioPoint2 { x: 0.0, y: text.size }, style }
    } else if let Some(image) = &node.image {
        let (mime, bytes) = decode_data_uri_bytes(&image.src).unwrap_or_default();
        SemioDrawNode::Image { at: SemioPoint2 { x: 0.0, y: 0.0 }, width: image.width, height: image.height, mime, bytes }
    } else {
        let segments: Vec<SemioPathSegment> = node.segments.iter().map(to_semio_path_segment).collect();
        if segments.is_empty() {
            return None;
        }
        SemioDrawNode::Path { segments, style }
    };
    Some(SemioDrawNode::Group { transform: matrix_to_semio_transform(node.transform), children: vec![leaf] })
}

/// 🌉️ Builds a real [SemioDrawingSnapshot] from this plugin's own domain document — the semio hub
/// side of draw's domain↔semio bridge. [flatten_draw_document_to_scene_nodes] has already resolved
/// booleans/traces/curve-flattening, so every scene node here is a concrete leaf.
pub async fn draw_document_to_semio_drawing(doc: &DrawSnapshot) -> SemioDrawingSnapshot {
    let (width, height) = resolve_draw_document_artboard(doc);
    let mut styles = Vec::new();
    let children: Vec<SemioDrawNode> = flatten_draw_document_to_scene_nodes(doc).iter().filter_map(|node| semio_draw_node_from_scene_node(node, &mut styles)).collect();
    SemioDrawingSnapshot {
        schema: STDIO_SEMIODRAWING_DOCUMENT_SCHEMA.into(),
        canvas: SemioDrawCanvas { width: width as f64, height: height as f64, background: None },
        styles,
        layers: vec![SemioDrawLayer { id: "root".into(), name: doc.title.clone().unwrap_or_else(|| "root".into()), visible: true, root: SemioDrawNode::Group { transform: SemioTransform::identity(), children } }],
    }
}

/// @emoji 🌉️ Serializes a draw document to SVG markup and raster dimensions by building a real
/// [SemioDrawingSnapshot] and dispatching through stdio's real semio/drawing↔svg bridge
/// (`io_dispatch`) — replaces the deleted hand-rolled SVG string builder.
pub async fn draw_document_to_svg(doc: &DrawSnapshot) -> Result<(String, u32, u32), String> {
    ensure_semio_drawing_bridge_registered();
    let (width, height) = resolve_draw_document_artboard(doc);
    let semio_drawing = draw_document_to_semio_drawing(doc);
    let key = semio_framework::IoKey {
        artifact_kind: SEMIO_DRAWING_DIALECT.artifact_kind.into(),
        standard: SEMIO_DRAWING_DIALECT.standard.0.into(),
        subset: SEMIO_DRAWING_DIALECT.subset.0.into(),
        direction: semio_framework::IoDirection::Export,
        format_kind: SVG_DIALECT.artifact_kind.into(),
        format_standard: SVG_DIALECT.standard.0.into(),
        format_subset: SVG_DIALECT.subset.0.into(),
    };
    let sources = [semio_framework::ErasedComposeSource { dialect: SEMIO_DRAWING_DIALECT, payload: semio_framework::IoPayload::Binary(<SemioDrawingSnapshot as store::ArtifactPack>::encode_pack(&semio_drawing)) }];
    let composed = semio_framework::resolve_ready(semio_framework::io_dispatch(&key, &sources)).map_err(|error| error.message)?;
    let bytes = match composed.payload {
        semio_framework::IoPayload::Binary(bytes) => bytes,
        semio_framework::IoPayload::Text(text) => text.into_bytes(),
    };
    let svg = <SvgSnapshot as store::ArtifactPack>::decode_pack(&bytes).map_err(|error| format!("{error:?}"))?;
    Ok((write_svg_xml(&svg.doc), width, height))
}

pub async fn draw_document_json_to_svg(value: &serde_json::Value) -> Result<(String, u32, u32), String> {
    let doc: DrawSnapshot = serde_json::from_value(value.clone()).map_err(|error| error.to_string())?;
    draw_document_to_svg(&doc)
}
//#endregion 🔖️SemioBridge

//#region 🔖️IoDeclaration
pub async fn io() -> semio_framework_plugin::app::declarations::IoDeclaration {
    use crate::artifacts::draw::standards::v1::subsets::any::io::export::serializers::artifacts as export;
    use crate::artifacts::draw::standards::v1::subsets::any::io::import::deserializers::artifacts as import;
    use crate::artifacts::draw::{DrawMutation, DrawSnapshot, DRAW_DIALECT, DRAW_DOCUMENT_SCHEMA};
    use semio_framework::io::io_mechanism::{deserializer_entry, serializer_entry, IoEntry};
    use semio_framework_plugin::app::declarations::{IoDeclaration, LanguagePair, NativeCodecs};
    use std::sync::OnceLock;

    async fn entries() -> &'static [IoEntry] {
        static ENTRIES: OnceLock<Vec<IoEntry>> = OnceLock::new();
        ENTRIES
            .get_or_init(|| {
                vec![
                    serializer_entry::<DrawSnapshot, export::svg::v1_1::any::DrawIntoSvg>(DRAW_DIALECT),
                    deserializer_entry::<DrawSnapshot, import::svg::v1_1::any::SvgIntoDraw>(DRAW_DIALECT),
                    serializer_entry::<DrawSnapshot, export::pdf::v1_4::any::DrawIntoPdf>(DRAW_DIALECT),
                    deserializer_entry::<DrawSnapshot, import::pdf::v1_4::any::PdfIntoDraw>(DRAW_DIALECT),
                    serializer_entry::<DrawSnapshot, export::png::v1_2::any::DrawIntoPng>(DRAW_DIALECT),
                    deserializer_entry::<DrawSnapshot, import::png::v1_2::any::PngIntoDraw>(DRAW_DIALECT),
                    serializer_entry::<DrawSnapshot, export::json::v_rfc8259::any::DrawIntoJson>(DRAW_DIALECT),
                    deserializer_entry::<DrawSnapshot, import::json::v_rfc8259::any::JsonIntoDraw>(DRAW_DIALECT),
                    serializer_entry::<DrawSnapshot, export::dwg::v_ac1018::any::DrawIntoDwg>(DRAW_DIALECT),
                    deserializer_entry::<DrawSnapshot, import::dwg::v_ac1018::any::DwgIntoDraw>(DRAW_DIALECT),
                    serializer_entry::<DrawSnapshot, export::dxf::v_r12::any::DrawIntoDxf>(DRAW_DIALECT),
                    deserializer_entry::<DrawSnapshot, import::dxf::v_r12::any::DxfIntoDraw>(DRAW_DIALECT),
                ]
            })
            .as_slice()
    }

    IoDeclaration {
        native: NativeCodecs {
            // 🎯️ `LanguagePair { text: None, binary: None }` for every facet: a documented,
            // deliberate scope-narrowing matching every other subset already on the new tree
            // (stdio binary/txt, sequence) — `NativeCodecs`'s own doc calls this a legal, supported
            // shape. The underlying `ArtifactDsl`/`ArtifactPack`/`OpText`/`OpBinary` codecs these
            // would point at are unchanged, independently implemented (see `📸️snapshot/`,
            // `🧬️mutations/` siblings), and independently tested either way.
            snapshot: LanguagePair { text: None, binary: None },
            diff: LanguagePair { text: None, binary: None },
            mutations: LanguagePair { text: None, binary: None },
            inferences: None,
            codec: store::ArtifactCodec::of::<DrawSnapshot, DrawMutation>(DRAW_DOCUMENT_SCHEMA.to_string()),
        },
        entries: entries(),
    }
}
//#endregion 🔖️IoDeclaration

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::artifacts::draw::schema::{create_draw_image_layer, create_draw_shape_layer_rect, default_draw_document, default_layer_base};
    use crate::artifacts::draw::{DrawImageAsset, DrawLayerNode, DrawTextBody, StrokeStyle};

    /// 🌉️ Ported from the pre-migration `draw_document_to_svg_renders_shape_text_image_and_gradient_nodes`
    /// (same shape/text/image/gradient coverage) onto the new `SemioDrawingSnapshot`→`io_dispatch`
    /// bridge — decodes the real bridged SVG back into stdio's own typed `SvgElement` tree instead
    /// of substring-matching hand-rolled markup, since the markup is no longer hand-rolled.
    #[test]
    async fn draw_document_to_svg_bridges_shape_text_image_and_gradient_nodes_through_semio_drawing() {
        use semio_s_plugin_stdio::artifacts::svg::standards::v1_1::subsets::any::schema::snapshot::{parse_svg_xml, svg_element_from_xml_node, SvgElement};

        let mut rect = create_draw_shape_layer_rect("Rect");
        if let DrawLayerNode::Shape(shape) = &mut rect {
            shape.base.attributes.fill = Some(FillStyle::Solid { color: [1.0, 0.0, 0.0, 0.5] });
            shape.base.attributes.stroke = Some(StrokeStyle { color: [0.0, 0.0, 0.0, 1.0], width: 2.0, cap: "round".into(), join: "round".into(), dash: None });
        }
        let mut gradient_rect = create_draw_shape_layer_rect("Gradient");
        if let DrawLayerNode::Shape(shape) = &mut gradient_rect {
            shape.base.attributes.fill = Some(FillStyle::LinearGradient { x1: 0.0, y1: 0.0, x2: 1.0, y2: 1.0, stops: Vec::new() });
        }
        let text = DrawLayerNode::Text(DrawTextBody { base: default_layer_base("T"), x: 0.0, y: 0.0, content: "<a & b>".into(), size: 12.0 });
        let mut assets = std::collections::BTreeMap::new();
        assets.insert("img".to_string(), DrawImageAsset { mime: "image/png".into(), data: "aGVsbG8=".into(), width: Some(4), height: Some(4) });
        let image = create_draw_image_layer("Image", "img");

        let mut doc = default_draw_document("svg-test", None);
        doc.layers = vec![rect, gradient_rect, text, image];
        doc.assets = assets;
        doc.artboard = None;

        let (svg_text, width, height) = draw_document_to_svg(&doc).expect("svg export via semio/drawing bridge");
        assert!(width >= 1 && height >= 1);

        let reparsed = parse_svg_xml(&svg_text).expect("bridged svg reparses");
        let root = svg_element_from_xml_node(reparsed.root.as_ref().expect("svg root")).expect("typed svg root");
        let layer_children = match &root {
            SvgElement::Svg { children, .. } => match &children[0] {
                SvgElement::Group { children, .. } => children,
                other => panic!("expected layer group, got {other:?}"),
            },
            other => panic!("expected <svg> root, got {other:?}"),
        };
        assert_eq!(layer_children.len(), 4, "rect, gradient rect, text, image");
        let leaf = |index: usize| match &layer_children[index] {
            SvgElement::Group { children, .. } => &children[0],
            other => panic!("expected node wrapper group, got {other:?}"),
        };

        match leaf(0) {
            SvgElement::Path { common, .. } => assert!(common.presentation.fill.as_deref().is_some_and(|fill| fill.starts_with("rgba(255,")), "{:?}", common.presentation.fill),
            other => panic!("expected filled rect path, got {other:?}"),
        }
        match leaf(1) {
            SvgElement::Path { common, .. } => assert!(common.presentation.fill.is_none(), "gradients have no semio/drawing equivalent — dropped, not fabricated"),
            other => panic!("expected gradient rect path, got {other:?}"),
        }
        match leaf(2) {
            SvgElement::Text { children, .. } => assert_eq!(children, &vec![SvgElement::TextNode("<a & b>".into())]),
            other => panic!("expected text node, got {other:?}"),
        }
        match leaf(3) {
            SvgElement::Unknown { name, attrs, .. } => {
                assert_eq!(name, "image");
                let href = attrs.iter().find(|attr| attr.name == "href").expect("image href attr");
                assert!(href.value.starts_with("data:image/png;base64,"));
            }
            other => panic!("expected image node, got {other:?}"),
        }

        let json_error = draw_document_json_to_svg(&serde_json::json!({"bad": true}));
        assert!(json_error.is_err());
    }
}
//#endregion 🧪️Tests
